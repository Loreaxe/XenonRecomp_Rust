// src/recompiler/emit/fn_emit.rs

use anyhow::*;
use std::collections::BTreeSet;

use crate::disasm::PpcCs;
use crate::function::Function;
use crate::log::Phase;
use crate::xtrace;
use crate::recompiler::{CSRState, Recompiler, RecompilerLocalVariables};

use super::super::lower_one;

impl Recompiler {
    /// Recompile (emit) one function: disassemble and lower every instruction.
    /// Emits **Rust style** functions with a `pc` dispatcher:
    ///   pub fn sub_XXXXXXXX(ctx: &mut PPCContext, base: *mut u8) { ... }
    pub fn recompile_fn(&mut self, fnc: &Function, cs: &PpcCs) -> Result<()> {
        let _phase = Phase::new("recomp::recompile_fn");

        // 🔴 Skip import thunks / extern targets completely.
        // Any VA that appears in `extern_wrappers` will be handled
        // by the generated xam.rs/xboxkrnl.rs wrappers instead.
        let fn_base_va = fnc.base as u32;
        if self.extern_wrappers.contains_key(&fn_base_va) {
            // Optional debug:
            // eprintln!(
            //     "RECOMP: skip extern thunk at 0x{:08X} (mapped to {})",
            //     fn_base_va,
            //     self.extern_wrappers.get(&fn_base_va).unwrap()
            // );
            return Ok(());
        }

        // -----------------------------------------------------
        // 0) Preserve existing file contents while we build fn
        // -----------------------------------------------------
        let mut file_prefix = String::new();
        std::mem::swap(&mut self.out, &mut file_prefix); // self.out now = scratch for this fn

        // Locals / FPSCR state accumulate across the whole function.
        let mut locals = RecompilerLocalVariables::default();
        let mut csr = CSRState::Unknown;

        // Use a section-clamped (and capped) size to prevent runaway prints.
        let fn_size = self
            .clamp_to_code_section(fnc.base, fnc.size)
            .min(Self::MAX_FUNC_BYTES);

        let fn_start = fnc.base;
        let fn_end   = fn_start + fn_size;
        let fn_start_va = fn_start as u32;
        let fn_end_va   = fn_end   as u32;

        if fnc.base as u32 == 0x821DB538 {
            eprintln!(
                "DEBUG: sub_821DB538: fn_size={} (clamped), original size={}",
                fn_size, fnc.size
            );
        }

        // --------------------------
        // 1) Build per-block ranges
        // --------------------------
        //
        // Instead of trusting only `fnc.blocks` (which might be a single big
        // block), we also seed block boundaries from switch-table labels that
        // lie inside this function. That guarantees a match-arm for each
        // `pc = 0xXXXXXXXX; continue 'dispatch;` target the lowering emits.

        let mut block_starts: BTreeSet<u32> = BTreeSet::new();

        // Always start with the function entry.
        block_starts.insert(fn_start_va);

        // 1a) NEW: direct branch targets inside this function.
        //
        // We do a quick disasm pass and grab the IMM operand of any branch
        // that has an immediate target. Capstone resolves the PC-relative
        // branch to an absolute VA for us.
        {
            use capstone::arch::ArchOperand;
            use capstone::arch::ppc::{PpcInsn, PpcOperand};

            let bytes = self.get_code_bytes(fn_start, fn_size)?;
            let insns = cs.disasm_all(&bytes, fn_start as u64)?;

            for insn in insns.iter() {
                let id = insn.id().0;

                // Only look at *direct* branch / branch-with-link opcodes.
                let is_branch = matches!(
                    id,
                    x if x == PpcInsn::PPC_INS_B as u32
                        || x == PpcInsn::PPC_INS_BL as u32
                        || x == PpcInsn::PPC_INS_BDNZ as u32
                        || x == PpcInsn::PPC_INS_BDNZF as u32
                        || x == PpcInsn::PPC_INS_BDNZT as u32
                        || x == PpcInsn::PPC_INS_BDZ as u32
                        || x == PpcInsn::PPC_INS_BEQ as u32
                        || x == PpcInsn::PPC_INS_BNE as u32
                        || x == PpcInsn::PPC_INS_BGE as u32
                        || x == PpcInsn::PPC_INS_BGT as u32
                        || x == PpcInsn::PPC_INS_BLE as u32
                        || x == PpcInsn::PPC_INS_BLT as u32
                        // ... add any other `b*` with an immediate target you care about ...
                );

                if !is_branch {
                    continue;
                }

                if let std::result::Result::Ok(detail) = cs.insn_detail(insn) {
                    let arch = detail.arch_detail();
                    for op in arch.operands() {
                        // `op` is &ArchOperand; pattern-match on the value.
                        if let ArchOperand::PpcOperand(PpcOperand::Imm(target)) = op {
                            let target = target as u32;
                            if target >= fn_start_va && target < fn_end_va {
                                block_starts.insert(target);
                            }
                        }
                    }
                }
            }
        }

        // 1b) Existing analysed blocks (if any).
        if !fnc.blocks.is_empty() {
            for b in &fnc.blocks {
                let start = fnc.base + b.base;
                if start >= fn_start && start < fn_end {
                    block_starts.insert(start as u32);
                }
            }
        }

        // 1c) Switch-table labels inside this function.
        for (sw_base, sw) in &self.switch_tables {
            if *sw_base < fn_start_va || *sw_base >= fn_end_va {
                continue;
            }
            for &lbl in &sw.labels {
                if lbl >= fn_start_va && lbl < fn_end_va {
                    block_starts.insert(lbl);
                }
            }
        }

        // 1d) Alias entrypoints that belong to this function.
        //
        // If both the primary and the alias live inside this function's
        // address range, treat `alias` as an alternate entrypoint. That
        // way `entry_pc == alias` gets its own match arm.
        for al in &self.aliases {
            let primary = al.primary;
            let alias   = al.alias;

            if self.addr_in_function(fnc, primary) && self.addr_in_function(fnc, alias) {
                block_starts.insert(alias);
            }
        }

        // If we somehow ended up with no starts, fall back to a single block.
        if block_starts.is_empty() {
            block_starts.insert(fn_start_va);
        }

        // Turn sorted starts into (start, size) ranges.
        let mut starts_vec: Vec<u32> = block_starts.into_iter().collect();
        starts_vec.sort_unstable();

        let mut block_ranges: Vec<(usize, usize)> = Vec::new();
        for (i, &s) in starts_vec.iter().enumerate() {
            let start = s as usize;
            let end = if i + 1 < starts_vec.len() {
                starts_vec[i + 1] as usize
            } else {
                fn_end
            };
            if start < end {
                let size = end - start;
                let size = self
                    .clamp_to_code_section(start, size)
                    .min(Self::MAX_FUNC_BYTES);
                if size != 0 {
                    block_ranges.push((start, size));
                }
            }
        }

        // For safety, keep blocks sorted by address.
        block_ranges.sort_by_key(|(start, _)| *start);

        // --------------------------------
        // 2) Lower each block into a body
        // --------------------------------

        let mut blocks: Vec<(u32, String)> = Vec::new(); // (block_pc, body)

        for (i, (bb_start, bb_size)) in block_ranges.iter().enumerate() {
            let bb_start = *bb_start;
            let bb_size  = *bb_size;
            let bb_end   = bb_start + bb_size;
            let bb_pc    = bb_start as u32;

            self.out.clear();

            self.println_fmt(format_args!(
                "    //   block [0x{:08X}..0x{:08X})",
                bb_start, bb_end
            ));

            // per-block termination flag
            let mut block_terminated = false;

            // Lower all instructions in this block; this fills `locals` / `csr`
            // and lets handlers mark `block_terminated = true` when appropriate.
            self.lower_range(
                cs,
                fnc,
                bb_start,
                bb_size,
                &mut locals,
                &mut csr,
                &mut block_terminated,
            )?;

            let mut block_body = std::mem::take(&mut self.out);

            // Decide whether we need an auto-fallthrough.
            let mut needs_fallthrough =
                block_ranges.get(i + 1).is_some() && !block_terminated;

            if needs_fallthrough {
                let trimmed = block_body.trim_end();
                if let Some(last_line) = trimmed.rsplit('\n').next() {
                    let last = last_line.trim_end();
                    if last.ends_with("continue 'dispatch;") || last.ends_with("return;") {
                        needs_fallthrough = false;
                    }
                }
            }

            if needs_fallthrough {
                if let Some((next_start, _)) = block_ranges.get(i + 1) {
                    let next_pc = *next_start as u32;
                    block_body.push_str(&format!(
                        "\tpc = 0x{next_pc:08X}; continue 'dispatch;\n"
                    ));
                }
            }

            blocks.push((bb_pc, block_body));
        }

        // -------------------------------------
        // 3) Emit function header + dispatcher
        // -------------------------------------

        self.out.clear(); // scratch now contains nothing

        let name = Self::resolve_func_name(self, fnc);

        // Function signature
        self.println_fmt(format_args!(
            "pub fn {}(ctx: &mut PPCContext, base: *mut u8, entry_pc: u32) {{",
            name
        ));

        // Emit locals (r0.., f0.., v0.. etc) based on `locals` usage.
        Self::emit_locals(&mut self.out, &locals);

        // Function-level debug header
        self.println_fmt(format_args!(
            "    // ---- function 0x{:08X} size={}",
            fnc.base as u32,
            fnc.size
        ));

        // Switch-table comments for this function (if any)
        self.emit_switch_comments_for_fn(fnc);

        // Initial PC = entry_pc supplied by the dispatcher. This may be the
        // function root or an internal alias (e.g. __savegprlr_28).
        self.println("    let mut pc: u32 = entry_pc;");
        self.println("    'dispatch: loop {");
        self.println("        match pc {");

        // One match arm per block (in address order).
        for (bb_pc, body) in &blocks {
            self.println_fmt(format_args!("            0x{bb_pc:08X} => {{"));
            // Insert pre-generated block body (already indented with tabs).
            self.print(body);
            self.println("            }");
        }

        // Any other PC is a logic error.
        self.println("            _ => unsafe { core::hint::unreachable_unchecked() },");
        self.println("        }");
        self.println("    }");
        self.println("}");
        self.println("");

        // -------------------------------------
        // 4) Merge fn-source back into file buf
        // -------------------------------------
        let mut fn_source = String::new();
        std::mem::swap(&mut self.out, &mut fn_source);   // fn_source = this function text

        std::mem::swap(&mut self.out, &mut file_prefix); // restore previous file contents
        self.print(fn_source);                           // append this fn
        self.println("");                                // extra newline (optional)

        Ok(())
    }

    /// Process a linear code range: disassemble and lower every instruction.
    fn lower_range(
        &mut self,
        cs: &PpcCs,
        fnc: &Function,
        range_start_va: usize,
        range_size: usize,
        locals: &mut RecompilerLocalVariables,
        csr: &mut CSRState,
        block_terminated: &mut bool,
    ) -> Result<()> {
        const CHUNK: usize = 256 * 1024; // 256 KiB
        xtrace!(
            "RECOMP: lower_range base=0x{:08X} size={} (fn=0x{:08X})",
            range_start_va as u32,
            range_size,
            fnc.base
        );

        let bytes = self.get_code_bytes(range_start_va, range_size)?;
        if range_start_va as u32 == 0x821DB538 {
            eprintln!(
                "DEBUG: lower_range for 0x821DB538: size={}, bytes_len={}",
                range_size,
                bytes.len()
            );
            if !bytes.is_empty() {
                for (i, b) in bytes.iter().take(16).enumerate() {
                    eprint!("{:02X} ", b);
                }
                eprintln!();
            }
        }

        // Only print per-instruction disasm comments when explicitly requested.
        let verbose = std::env::var_os("XENON_RECOMP_VERBOSE").is_some();

        let mut off = 0usize;
        while off < bytes.len() {
            let end = (off + CHUNK).min(bytes.len());
            let chunk = &bytes[off..end];
            let base_addr = (range_start_va + off) as u64;

            let insns = cs
                .disasm_all(chunk, base_addr)
                .with_context(|| format!("capstone failed at 0x{:08X}", base_addr as u32))?;

            if insns.is_empty() {
                off = end;
                continue;
            }

            for insn in insns.iter() {
                let ib = insn.bytes();
                if ib.len() < 4 {
                    continue;
                }
                let be_word = u32::from_be_bytes([ib[0], ib[1], ib[2], ib[3]]);

                let next_be_word = if ib.len() >= 8 {
                    Some(u32::from_be_bytes([ib[4], ib[5], ib[6], ib[7]]))
                } else {
                    let pc_off = (insn.address() as usize).saturating_sub(range_start_va);
                    if pc_off + 8 <= bytes.len() {
                        Some(u32::from_be_bytes([
                            bytes[pc_off + 4],
                            bytes[pc_off + 5],
                            bytes[pc_off + 6],
                            bytes[pc_off + 7],
                        ]))
                    } else {
                        None
                    }
                };

                if verbose {
                    self.println(format!(
                        "    // {:08X}: {}\t{}",
                        insn.address() as u32,
                        insn.mnemonic().unwrap_or(""),
                        insn.op_str().unwrap_or("")
                    ));
                }

                let _handled = lower_one(
                    self,
                    fnc,
                    insn.address() as u32,
                    be_word,
                    insn,
                    locals,
                    csr,
                    next_be_word,
                    &*cs,
                    block_terminated,
                );

                // 🔚 Stop decoding this block as soon as a terminator (blr/bctr/etc.) fires.
                if *block_terminated {
                    break;
                }
            }

            // If the block terminated, stop the whole range; don't walk into jump tables.
            if *block_terminated {
                break;
            }

            off = end;
        }

        Ok(())
    }
}
