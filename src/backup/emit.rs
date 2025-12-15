// src/recompiler/emit.rs
use anyhow::*;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::path::Path;

use crate::disasm::PpcCs;
use crate::function::Function;
use crate::image::SectionFlags;
use crate::log::Phase;
use crate::xlog;
use crate::xdebug;
use crate::xtrace;
use crate::recompiler::templates::{PPC_CONTEXT_RS, PPC_RT_RS};

use super::{CSRState, Recompiler, RecompilerLocalVariables};
use super::lower_one;

impl Recompiler {

    /// For the given function, emit comments describing any switch tables
    /// whose base address lies inside [fn_base .. fn_base+fn_size).
    fn emit_switch_comments_for_fn(&mut self, fnc: &Function) {
        let fn_start = fnc.base as u32;
        let fn_end   = fn_start.wrapping_add(fnc.size as u32);

        // (base, r, labels)
        let mut hits: Vec<(u32, u32, Vec<u32>)> = self
            .switch_tables
            .iter()
            .filter_map(|(base, sw)| {
                if *base >= fn_start && *base < fn_end {
                    Some((*base, sw.r, sw.labels.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Stable ordering by base address.
        hits.sort_by_key(|(base, _, _)| *base);

        for (base, r, labels) in hits {
            self.println(format!(
                "    //   switch @ 0x{base:08X}: r{} with {} label(s)",
                r,
                labels.len()
            ));

            for (idx, lbl) in labels.iter().enumerate() {
                self.println(format!(
                    "    //       case {idx:2} → 0x{lbl:08X}"
                ));
            }
        }
    }

    /// True if `addr` lies inside this Function's [base, end) range.
    #[inline]
    pub fn addr_in_function(&self, fnc: &Function, addr: u32) -> bool {
        let start = fnc.base as u32;
        let end   = start.wrapping_add(fnc.size as u32);
        addr >= start && addr < end
    }

    /// True if `addr` lies inside *any* known function range.
    #[inline]
    pub fn addr_in_any_function(&self, addr: u32) -> bool {
        self.functions.iter().any(|f| {
            let start = f.base as u32;
            let end   = start.wrapping_add(f.size as u32);
            addr >= start && addr < end
        })
    }

    /// If `addr` is inside any known function, or is an alias for one, return
    /// the Rust name we will have emitted (sub_XXXXXXXX of the **primary**).
    #[inline]
    pub fn resolve_func_name_at(&self, addr: u32) -> Option<String> {
        // 0) Canonicalize alias → primary if we have an alias record.
        let canonical = if let Some(al) = self.aliases.iter().find(|al| al.alias == addr) {
            al.primary
        } else {
            addr
        };

        // 1) Exact / in-range hit on a function, using the canonical address
        if let Some(f) = self.functions.iter().find(|f| {
            let start = f.base as u32;
            let end   = start.wrapping_add(f.size as u32);
            canonical >= start && canonical < end
        }) {
            return Some(Self::resolve_func_name(self, f));
        }

        // 2) We didn't find a function containing `canonical`, but if it *was*
        //    an alias we at least return a stable name sub_PRIMARY.
        if canonical != addr {
            return Some(format!("sub_{:08X}", canonical));
        }

        None
    }

    #[inline]
    pub fn resolve_extern_wrapper(&self, va: u32) -> Option<&str> {
        // 1) Exact thunk address match.
        if let Some(s) = self.extern_wrappers.get(&va) {
            return Some(s.as_str());
        }

        // 2) Fuzzy match: treat a thunk as a tiny 16-byte region.
        //    This covers cases where analysis treated the thunk as
        //    part of a bigger function and we only have the base.
        for (&thunk_base, name) in &self.extern_wrappers {
            if va >= thunk_base && va < thunk_base + 0x10 {
                return Some(name.as_str());
            }
        }

        None
    }

    /// Clamp an emission range to the containing CODE section to avoid runaway prints.
    #[inline]
    fn clamp_to_code_section(&self, start_va: usize, size: usize) -> usize {
        for s in &self.image.sections {
            if !s.flags.contains(SectionFlags::CODE) { continue; }
            let lo = s.base as usize;
            let hi = lo + s.data.len();
            if start_va >= lo && start_va < hi {
                return size.min(hi - start_va);
            }
        }
        size
    }

    // Resolve a function's printable name (Rust style: sub_XXXXXXXX)
    fn resolve_func_name(_rec: &Recompiler, f: &Function) -> String {
        format!("sub_{:08X}", f.base as u32)
    }

    // Emit locals as Rust `let mut` bindings (keeps your style explicit).
    fn emit_locals(buf: &mut String, locals: &RecompilerLocalVariables) {
        if locals.ctr      { buf.push_str("    let mut ctr: PPCRegister = Default::default();\n"); }
        if locals.xer      { buf.push_str("    let mut xer: PPCXERRegister = Default::default();\n"); }
        if locals.reserved { buf.push_str("    let mut reserved: PPCRegister = Default::default();\n"); }
        for i in 0..8   { if locals.cr[i] { buf.push_str(&format!("    let mut cr{i}: PPCCRRegister = Default::default();\n")); } }
        for i in 0..32  { if locals.r[i]  { buf.push_str(&format!("    let mut r{i}: PPCRegister = Default::default();\n")); } }
        for i in 0..32  { if locals.f[i]  { buf.push_str(&format!("    let mut f{i}: PPCRegister = Default::default();\n")); } }
        for i in 0..128 { if locals.v[i]  { buf.push_str(&format!("    let mut v{i}: PPCVRegister = Default::default();\n")); } }
        if locals.env      { buf.push_str("    let mut env0: PPCContext = Default::default();\n"); }
        if locals.temp     { buf.push_str("    let mut tmp: PPCRegister = Default::default();\n"); }
        if locals.vtemp    { buf.push_str("    let mut vtmp: PPCVRegister = Default::default();\n"); }
        if locals.ea       { buf.push_str("    let mut ea: u32 = 0;\n"); }
    }

    // --- small print helpers used throughout emission ---

    #[inline]
    pub(crate) fn print(&mut self, s: impl AsRef<str>) {
        self.out.push_str(s.as_ref());
    }
    #[inline]
    pub(crate) fn println(&mut self, s: impl AsRef<str>) {
        self.out.push_str(s.as_ref());
        self.out.push('\n');
    }
    #[inline]
    pub(crate) fn println_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        use std::fmt::Write as _;
        self.out.write_fmt(args).unwrap();
        self.out.push('\n');
    }

    /// Recompile (emit) one function: disassemble and lower every instruction.
    /// Emits **Rust style** functions with a `pc` dispatcher:
    ///   pub fn sub_XXXXXXXX(ctx: &mut PPCContext, base: *mut u8) { ... }
    pub fn recompile_fn(&mut self, fnc: &Function, cs: &PpcCs) -> Result<()> {
        let _phase = Phase::new("recomp::recompile_fn");

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
    ) -> anyhow::Result<()> {
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

    /// Copy out code bytes for [start_va .. start_va+size) into an owned Vec<u8>.
    fn get_code_bytes(&self, start_va: usize, size: usize) -> Result<Vec<u8>> {
        for s in &self.image.sections {
            let lo = s.base as usize;
            let hi = lo + s.data.len();
            if start_va >= lo && start_va + size <= hi {
                let off = start_va - lo;
                return Ok(s.data[off..off + size].to_vec());
            }
        }
        bail!(
            "address range [0x{start_va:08X}..0x{:08X}) not in any CODE section",
            start_va + size
        );
    }

    /// Batch-write every ~150 functions, with console progress.
    pub fn recompile_all(&mut self) -> Result<()> {
        use std::io::Write;

        let _phase = Phase::new("recomp::recompile_all");
        let cs = PpcCs::new()?;

        // Build extern wrappers first (xam/xboxkrnl shims etc.)
        self.emit_externs_if_any()?;

        let mut in_this_file = 0usize;
        let total = self.functions.len();

        println!("Starting static recompile of {total} functions...");

        for idx in 0..total {
            let fnc_ptr: *const Function = &self.functions[idx];
            let fnc: &Function = unsafe { &*fnc_ptr };

            println!("> lowering 0x{:08X} ({} / {})", fnc.base, idx + 1, total);

            // Current chunk module name (ppc_000, ppc_001, ...)
            let current_module = format!("ppc_{:03}", self.file_index);
            // Record where this function will live
            self.func_modules
                .insert(fnc.base as u32, current_module);

            self.recompile_fn(fnc, &cs)?;

            in_this_file += 1;

            if idx % 10 == 0 {
                print!(
                    "\r[{:5}/{:5}] current file index {:03}",
                    idx + 1,
                    total,
                    self.file_index
                );
                std::io::stdout().flush().ok();
            }

            if in_this_file == 100 {
                println!("\n→ wrote batch #{:03} (100 functions)", self.file_index);
                if !self.out.is_empty() {
                    self.save_current_out_data(None)?;
                }
                in_this_file = 0;
            }
        }

        if !self.out.is_empty() {
            println!(
                "\n→ wrote final batch #{:03} ({} remaining)",
                self.file_index, in_this_file
            );
            self.save_current_out_data(None)?;
        }

        println!("✅ Recompile complete: wrote {} file(s)", self.file_index);

        // Summary of any instructions we didn't know how to lower.
        self.report_unhandled_insns();

        Ok(())
    }

    /// Write `self.out` to a file (skips write if content unchanged).
    pub fn save_current_out_data(&mut self, name: Option<&str>) -> Result<()> {
        let _p = Phase::new("recomp::save_current_out_data");

        // Workspace root
        let root = if self.config.out_directory_path.is_empty() {
            ".".into()
        } else {
            self.config.out_directory_path.clone()
        };

        // Decide where to write:
        //   - If `name` is Some(..): root/<name>  (ppc_func_mapping.rs, etc.)
        //   - If `name` is None:     root/ppc_chunk_XXX/src/ppc_YYY.rs
        let (dir, fname) = if let Some(name) = name {
            (PathBuf::from(&root), name.to_string())
        } else {
            // Chunked ppc_###.rs file
            let file_idx = self.file_index;
            let crate_idx = file_idx / 10; // 10 ppc_###.rs per chunk crate

            let chunk_dir = PathBuf::from(&root)
                .join(format!("ppc_chunk_{:03}", crate_idx))
                .join("src");

            std::fs::create_dir_all(&chunk_dir).ok();

            (chunk_dir, format!("ppc_{:03}.rs", file_idx))
        };

        std::fs::create_dir_all(&dir).ok();
        let path = dir.join(&fname);

        // Build final file contents, with optional header for ppc_### files.
        let mut final_out = String::new();

        // Only the chunk files (ppc_###.rs) get the ppc_ctx prelude header.
        let is_chunk_file = name.is_none()
            && fname.starts_with("ppc_")
            && fname.ends_with(".rs")
            && fname != "ppc_func_mapping.rs";

        if is_chunk_file {
            // Everything in this file expects PPCContext / PPCRegister etc. from ppc_ctx.
            final_out.push_str("// @generated — Xenon Recompiler chunk\n");
            final_out.push_str("#![allow(dead_code, non_snake_case, non_camel_case_types, \
                clippy::too_many_arguments, clippy::needless_return)]\n\n");
            final_out.push_str("use ppc_ctx::recompiler::ppc_context::*;\n\n");
        }

        final_out.push_str(&self.out);

        // Note the fully-qualified Result::Ok here to avoid any Ok() shadowing.
        if let std::result::Result::Ok(existing) = std::fs::read(&path) {
            if existing.len() == final_out.len() && existing == final_out.as_bytes() {
                xlog!("RECOMP: unchanged '{}', skipping write", path.display());
                self.out.clear();

                // Track generated ppc_###.rs files, but exclude the mapping file.
                if is_chunk_file {
                    if !self.generated_files.contains(&fname) {
                        self.generated_files.push(fname.clone());
                    }
                }

                if name.is_none() {
                    self.file_index += 1;
                }
                return Ok(());
            }
        }

        std::fs::write(&path, final_out.as_bytes())?;
        xlog!("RECOMP: wrote '{}'", path.display());
        self.out.clear();

        // Track generated ppc_###.rs files, but exclude the mapping file.
        if is_chunk_file {
            if !self.generated_files.contains(&fname) {
                self.generated_files.push(fname.clone());
            }
        }

        if name.is_none() {
            self.file_index += 1;
        }
        Ok(())
    }

    /// Emit a **Rust** mapping table:
    ///   pub type FuncPtr = fn(&mut PPCContext, *mut u8);
    ///   pub static PPC_FUNC_MAPPINGS: &[(u32, FuncPtr)] =
    ///       &[(0xADDR, crate::ppc_000::sub_XXXXXXXX), ...];
    pub fn emit_rust_mapping(&mut self) -> Result<()> {
        let _p = Phase::new("recomp::emit_rust_mapping");

        self.out.clear();

        // Header
        self.println("// Auto-generated function mapping (Rust style)");
        self.println("use crate::recompiler::ppc_context::PPCContext;");
        // FuncPtr now takes an explicit entry_pc (guest VA).
        self.println("pub type FuncPtr = fn(&mut PPCContext, *mut u8, u32);");
        self.println("");

        // Stage entries without mut-borrowing self while iterating
        let mut entries = String::new();

        // 1) Primary functions: (base -> crate::ppc_###::sub_BASE)
        for f in &self.functions {
            let base = f.base as u32;
            let name = Self::resolve_func_name(self, f);

            if let Some(module) = self.func_modules.get(&base) {
                entries.push_str(&format!(
                    "    (0x{base:08X}, crate::{}::{}),\n",
                    module, name
                ));
            } else {
                // Fallback (shouldn't normally happen): keep old flat style.
                entries.push_str(&format!("    (0x{:08X}, {}),\n", base, name));
            }
        }

        // 2) Aliases: (alias_addr -> crate::ppc_###::sub_PRIMARY)
        //
        // NOTE: We intentionally map aliases directly to the primary function's
        // implementation, not to the tiny alias wrapper.
        for al in &self.aliases {
            if let Some(module) = self.func_modules.get(&al.primary) {
                entries.push_str(&format!(
                    "    (0x{:08X}, crate::{}::sub_{:08X}),\n",
                    al.alias,
                    module,
                    al.primary
                ));
            } else {
                // Fallback: no module info; keep older style
                entries.push_str(&format!(
                    "    (0x{:08X}, sub_{:08X}),\n",
                    al.alias,
                    al.primary
                ));
            }
        }

        self.println("pub static PPC_FUNC_MAPPINGS: &[(u32, FuncPtr)] = &[");
        self.print(&entries);
        self.println("];");

        self.save_current_out_data(Some("ppc_func_mapping.rs"))?;
        Ok(())
    }

    /// Write an index file that declares `mod ppc_###;` for every generated chunk.
    pub fn write_mod_index(&self, index_name: &str) -> Result<()> {
        let out_dir = if self.config.out_directory_path.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(&self.config.out_directory_path)
        };
        std::fs::create_dir_all(&out_dir).ok();
        let path = out_dir.join(index_name);

        // Check which helper files exist in the output dir
        // NOTE: we deliberately ignore any externs.rs here — it should be
        // user-provided in your host crate if you want one.
        let has_mapping = out_dir.join("ppc_func_mapping.rs").exists();
        let has_xam     = out_dir.join("xam.rs").exists();
        let has_krnl    = out_dir.join("xboxkrnl.rs").exists();

        let mut s = String::new();
        s.push_str("// @generated — Xenon Recompiler module index\n");
        s.push_str(
            "#![allow(dead_code, non_snake_case, non_camel_case_types, \
                clippy::too_many_arguments, clippy::needless_return)]\n\n",
        );

        // Recompiler support (PPCContext lives under `crate::recompiler`)
        s.push_str("pub mod recompiler {\n");
        s.push_str("    pub mod ppc_context { include!(\"ppc_context.rs\"); }\n");
        s.push_str("}\n\n");

        // Re-export PPCContext at the crate root for convenience
        s.push_str("pub use recompiler::ppc_context::PPCContext;\n\n");

        // Optional: re-export import wrappers if we generated them alongside ppc_ files.
        if has_xam {
            s.push_str("#[path = \"xam.rs\"] pub mod xam;\n");
        }
        if has_krnl {
            s.push_str("#[path = \"xboxkrnl.rs\"] pub mod xboxkrnl;\n");
        }
        if has_xam || has_krnl {
            s.push_str("\n");
        }

        // Chunk modules (public submodules; no global re-export)
        for f in &self.generated_files {
            if let Some(stem) = f.strip_suffix(".rs") {
                s.push_str(&format!("#[path = \"{}\"] pub mod {};\n", f, stem));
            }
        }

        // Optional helpers: mapping table only
        if has_mapping {
            s.push_str("\n#[path = \"ppc_func_mapping.rs\"] mod ppc_func_mapping;\n");
            s.push_str("pub use ppc_func_mapping::*;\n");
        }

        std::fs::write(&path, s)?;
        crate::xlog!("RECOMP: wrote module index '{}'", path.display());
        Ok(())
    }

    fn emit_shared_ppc_files(&self, root: &PathBuf) -> Result<()> {
        std::fs::create_dir_all(root)?;

        // Root copies (for single-crate mode and for ppc_ctx to copy if desired)
        std::fs::write(root.join("ppc_context.rs"), PPC_CONTEXT_RS)?;
        std::fs::write(root.join("rt.rs"),           PPC_RT_RS)?;
        Ok(())
    }

    /// Given a function base VA, return the chunk crate name, e.g. "ppc_chunk_000".
    pub(crate) fn chunk_crate_for_func(&self, base: u32) -> Option<String> {
        let module = self.func_modules.get(&base)?;
        if !module.starts_with("ppc_") {
            return None;
        }
        let idx_str = &module["ppc_".len()..];
        let file_idx: usize = idx_str.parse().ok()?;
        let crate_idx = file_idx / 10; // 20 ppc_###.rs per crate
        Some(format!("ppc_chunk_{:03}", crate_idx))
    }

    /// Generate a multi-crate Cargo workspace:
    ///   - ppc_ctx       : holds PPCContext / registers (from ppc_context.rs)
    ///   - ppc_chunk_### : each owns up to 20 ppc_###.rs files
    ///   - ppc_rt        : runtime crate with PPC_FUNC_MAPPINGS pointing into chunk crates
    ///
    /// NOTE: The old single-crate lib.rs/mod.rs + ppc_func_mapping.rs remain
    /// in the root output dir for backwards compatibility.
    pub fn write_workspace_multi_crate(&self) -> Result<()> {
        let _p = Phase::new("recomp::write_workspace_multi_crate");

        let root = if self.config.out_directory_path.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(&self.config.out_directory_path)
        };
        std::fs::create_dir_all(&root)?;

        let has_xam  = root.join("xam.rs").exists();
        let has_krnl = root.join("xboxkrnl.rs").exists();

        self.emit_shared_ppc_files(&root)?;

        // ------------------------------
        // 0) Build chunk map from generated_files
        // ------------------------------
        use std::collections::BTreeMap;
        let mut chunk_map: BTreeMap<usize, Vec<String>> = BTreeMap::new();

        for fname in &self.generated_files {
            if !fname.starts_with("ppc_") || !fname.ends_with(".rs") {
                continue;
            }
            // ppc_000.rs -> 0, ppc_001.rs -> 1, ...
            let idx_str = &fname["ppc_".len()..fname.len() - ".rs".len()];
            let file_idx: usize = idx_str.parse().unwrap_or(0);
            let crate_idx = file_idx / 10; // <= 10 ppc_### per crate

            chunk_map
                .entry(crate_idx)
                .or_default()
                .push(fname.clone());
        }

        // ------------------------------
        // 1) Workspace root Cargo.toml
        // ------------------------------
        let mut members: Vec<String> = Vec::new();        
        members.push("ppc_ctx".to_string());

        if has_xam {
            members.push("xam".to_string());
        }
        if has_krnl {
            members.push("xboxkrnl".to_string());
        }

        members.push("ppc_rt".to_string());
        for crate_idx in chunk_map.keys() {
            members.push(format!("ppc_chunk_{:03}", crate_idx));
        }

        let mut ws_toml = String::new();
        ws_toml.push_str("[workspace]\n");
        ws_toml.push_str("members = [\n");
        for m in &members {
            ws_toml.push_str(&format!("    \"{}\",\n", m));
        }
        ws_toml.push_str("]\n");

        std::fs::write(root.join("Cargo.toml"), ws_toml)?;

        // ------------------------------
        // 2) ppc_ctx crate
        // ------------------------------
        let ctx_dir = root.join("ppc_ctx");
        let ctx_src = ctx_dir.join("src");
        std::fs::create_dir_all(&ctx_src)?;

        let ctx_toml = r#"[package]
name = "ppc_ctx"
version = "0.1.0"
edition = "2021"
"#;
        std::fs::write(ctx_dir.join("Cargo.toml"), ctx_toml)?;

        // Emit canonical ppc_context.rs and rt.rs into the ppc_ctx crate.
        std::fs::write(ctx_src.join("ppc_context.rs"), PPC_CONTEXT_RS)?;
        std::fs::write(ctx_src.join("rt.rs"),          PPC_RT_RS)?;

        let mut ctx_lib = String::new();
        ctx_lib.push_str("// @generated — ppc_ctx\n");
        ctx_lib.push_str("#![allow(dead_code, non_snake_case, non_camel_case_types, \
            clippy::too_many_arguments, clippy::needless_return)]\n\n");

        // The real PPCContext definitions live in ppc_context.rs.
        // We include it as the *body of a module* so its inner `#![...]`
        // attributes are valid module-level attributes.
        ctx_lib.push_str("pub mod ppc_ctx_root {\n");
        ctx_lib.push_str("    include!(\"ppc_context.rs\");\n");
        ctx_lib.push_str("}\n\n");

        // Preserve the old path: ppc_ctx::recompiler::ppc_context::PPCContext
        ctx_lib.push_str("pub mod recompiler {\n");
        ctx_lib.push_str("    pub mod ppc_context {\n");
        ctx_lib.push_str("        // Re-export everything from ppc_ctx_root so existing\n");
        ctx_lib.push_str("        // paths like `recompiler::ppc_context::PPCContext` work.\n");
        ctx_lib.push_str("        pub use crate::ppc_ctx_root::*;\n");
        ctx_lib.push_str("    }\n");
        ctx_lib.push_str("}\n\n");

        // Re-export for convenience: ppc_ctx::PPCContext, etc.
        ctx_lib.push_str("pub use ppc_ctx_root::PPCContext;\n");
        ctx_lib.push_str("pub use ppc_ctx_root::*;\n");

        ctx_lib.push_str("pub mod rt;\n");

        std::fs::write(ctx_src.join("lib.rs"), ctx_lib)?;

                // ------------------------------
        // 2b) xam / xboxkrnl crates (import wrappers)
        // ------------------------------
        if has_xam {
            let xam_dir = root.join("xam");
            let xam_src = xam_dir.join("src");
            std::fs::create_dir_all(&xam_src)?;

            let xam_toml = r#"[package]
name = "xam"
version = "0.1.0"
edition = "2021"

[dependencies]
ppc_ctx = { path = "../ppc_ctx" }
"#;
            std::fs::write(xam_dir.join("Cargo.toml"), xam_toml)?;

            // Copy the generated stub file into this crate
            let root_xam = root.join("xam.rs");
            if root_xam.exists() {
                std::fs::copy(&root_xam, xam_src.join("xam.rs"))?;
            }

            let mut xam_lib = String::new();
            xam_lib.push_str("// @generated — XAM import shim crate\n");
            xam_lib.push_str("#![allow(dead_code, non_snake_case, non_camel_case_types, \
                clippy::too_many_arguments, clippy::needless_return)]\n\n");
            xam_lib.push_str("pub use ppc_ctx::PPCContext;\n\n");
            xam_lib.push_str("mod xam;\n");
            xam_lib.push_str("pub use xam::*;\n");

            std::fs::write(xam_src.join("lib.rs"), xam_lib)?;
        }

        if has_krnl {
            let krnl_dir = root.join("xboxkrnl");
            let krnl_src = krnl_dir.join("src");
            std::fs::create_dir_all(&krnl_src)?;

            let krnl_toml = r#"[package]
name = "xboxkrnl"
version = "0.1.0"
edition = "2021"

[dependencies]
ppc_ctx = { path = "../ppc_ctx" }
"#;
            std::fs::write(krnl_dir.join("Cargo.toml"), krnl_toml)?;

            // Copy the generated stub file into this crate
            let root_krnl = root.join("xboxkrnl.rs");
            if root_krnl.exists() {
                std::fs::copy(&root_krnl, krnl_src.join("xboxkrnl.rs"))?;
            }

            let mut krnl_lib = String::new();
            krnl_lib.push_str("// @generated — XboxKrnl import shim crate\n");
            krnl_lib.push_str("#![allow(dead_code, non_snake_case, non_camel_case_types, \
                clippy::too_many_arguments, clippy::needless_return)]\n\n");
            krnl_lib.push_str("pub use ppc_ctx::PPCContext;\n\n");
            krnl_lib.push_str("mod xboxkrnl;\n");
            krnl_lib.push_str("pub use xboxkrnl::*;\n");

            std::fs::write(krnl_src.join("lib.rs"), krnl_lib)?;
        }

        // ------------------------------
        // 3) ppc_chunk_### crates
        // ------------------------------
        for (crate_idx, files) in &chunk_map {
            let chunk_name = format!("ppc_chunk_{:03}", crate_idx);
            let chunk_dir = root.join(&chunk_name);
            let chunk_src = chunk_dir.join("src");
            std::fs::create_dir_all(&chunk_src)?;

            let mut chunk_toml = String::new();
            chunk_toml.push_str("[package]\n");
            chunk_toml.push_str(&format!("name = \"{}\"\n", chunk_name));
            chunk_toml.push_str("version = \"0.1.0\"\n");
            chunk_toml.push_str("edition = \"2021\"\n\n");
            chunk_toml.push_str("[dependencies]\n");
            chunk_toml.push_str("ppc_ctx = { path = \"../ppc_ctx\" }\n");
            if has_xam {
                chunk_toml.push_str("xam = { path = \"../xam\" }\n");
            }
            if has_krnl {
                chunk_toml.push_str("xboxkrnl = { path = \"../xboxkrnl\" }\n");
            }

            std::fs::write(chunk_dir.join("Cargo.toml"), chunk_toml)?;

            let mut lib = String::new();
            lib.push_str("// @generated — Xenon Recompiler chunk crate\n");
            lib.push_str("#![allow(dead_code, non_snake_case, non_camel_case_types, \
                clippy::too_many_arguments, clippy::needless_return)]\n\n");
            lib.push_str("// Bring PPC types into scope for recompiled functions.\n");
            lib.push_str("pub use ppc_ctx::recompiler::ppc_context::*;\n\n");

            lib.push_str("pub mod rt {\n");
            lib.push_str("    pub use ppc_ctx::rt::*;\n");
            lib.push_str("}\n\n");

            for f in files {
                // f is "ppc_000.rs" etc — we assume it's already in chunk_src.
                if let Some(stem) = f.strip_suffix(".rs") {
                    lib.push_str(&format!(
                        "#[path = \"{}\"] pub mod {};\n",
                        f, stem
                    ));
                    lib.push_str(&format!("pub use {}::*;\n\n", stem));
                }
            }

            std::fs::write(chunk_src.join("lib.rs"), lib)?;
        }

       // ------------------------------
        // 4) ppc_rt crate (runtime + mapping)
        // ------------------------------
        let rt_dir = root.join("ppc_rt");
        let rt_src = rt_dir.join("src");
        std::fs::create_dir_all(&rt_src)?;

        let mut rt_toml = String::new();
        rt_toml.push_str("[package]\n");
        rt_toml.push_str("name = \"ppc_rt\"\n");
        rt_toml.push_str("version = \"0.1.0\"\n");
        rt_toml.push_str("edition = \"2021\"\n\n");
        rt_toml.push_str("[dependencies]\n");
        rt_toml.push_str("ppc_ctx = { path = \"../ppc_ctx\" }\n");
        for crate_idx in chunk_map.keys() {
            rt_toml.push_str(&format!(
                "ppc_chunk_{:03} = {{ path = \"../ppc_chunk_{:03}\" }}\n",
                crate_idx, crate_idx
            ));
        }

        std::fs::write(rt_dir.join("Cargo.toml"), rt_toml)?;

// ppc_rt/src/lib.rs: runtime + mapping + C-ABI visible table
let rt_lib = r#"// @generated — Xenon Recompiler runtime crate
#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    clippy::too_many_arguments,
    clippy::needless_return
)]

use ppc_ctx::PPCContext;

/// Rust-side function pointer type used by the dispatcher and mapping.
/// NOTE: We keep this as a Rust `fn` type; C++ must NOT call these directly.
/// It only stores and forwards them back into Rust.
pub type FuncPtr = fn(&mut PPCContext, *mut u8, u32);

/// C-ABI-visible mapping entry for C++ (`rust_ppc_shim.h`).
///
/// C++ sees:
///   typedef void (*PPCFunc)(PPCContext* ctx, uint8_t* base, uint32_t entry_pc);
///   typedef struct PPCFuncMapping {
///       uint32_t guest;
///       PPCFunc  host;
///   } PPCFuncMapping;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PPCFuncMapping {
    pub guest: u32,
    pub host:  FuncPtr,
}

/// Generated tables live in func_mapping.rs
mod func_mapping;
pub use func_mapping::{PPC_FUNC_MAPPINGS, PPCFuncMappings};

/// Dispatcher entry point used by the C++ side (`rex_ppc_call_guest`).
///
/// First tries an exact match, then falls back to the last function whose
/// start address is <= guest_va (so internal BL targets inside a function
/// still work).
pub unsafe fn call_guest(guest_va: u32, ctx: &mut PPCContext, base: *mut u8) {
    let entry_pc = guest_va;

    // 1) Fast path: exact hit (includes alias entries).
    for (va, func) in PPC_FUNC_MAPPINGS.iter() {
        if *va == guest_va {
            (func)(ctx, base, entry_pc);
            return;
        }
    }

    // 2) Fallback: treat guest_va as lying inside the last function whose
    //    start address is <= guest_va.
    let mut candidate_fn: Option<FuncPtr> = None;
    let mut candidate_start: u32 = 0;

    for (va, func) in PPC_FUNC_MAPPINGS.iter() {
        let start = *va;
        if start <= guest_va && start >= candidate_start {
            candidate_start = start;
            candidate_fn = Some(*func);
        }
    }

    if let Some(f) = candidate_fn {
        f(ctx, base, entry_pc);
        return;
    }

    panic!("call_guest: no mapping for guest VA 0x{:08X}", guest_va);
}

/// ==== C ABI exports for rex_ppc.h ======================================

#[no_mangle]
pub extern "C" fn rex_ppc_create_context() -> *mut PPCContext {
    Box::into_raw(Box::new(PPCContext::default()))
}

#[no_mangle]
pub extern "C" fn rex_ppc_destroy_context(ctx: *mut PPCContext) {
    if !ctx.is_null() {
        unsafe { drop(Box::from_raw(ctx)); }
    }
}

#[no_mangle]
pub extern "C" fn rex_ppc_reset_context(ctx: *mut PPCContext) {
    if !ctx.is_null() {
        unsafe { *ctx = PPCContext::default(); }
    }
}

#[no_mangle]
pub extern "C" fn rex_ppc_call_guest(
    guest_va: u32,
    ctx: *mut PPCContext,
    base: *mut u8,
) {
    assert!(!ctx.is_null());
    unsafe { crate::call_guest(guest_va, &mut *ctx, base); }
}

/// Host-missing stub: matches legacy semantics (r3 = 0; return).
/// C++ can declare this in rust_ppc_shim.h but MUST NOT touch fields directly.
#[no_mangle]
pub extern "C" fn rex_host_missing_stub(
    ctx: *mut PPCContext,
    _base: *mut u8,
    _entry_pc: u32,
) {
    if ctx.is_null() {
        return;
    }
    unsafe {
        // Match guest stub semantics: r3 = 0; return
        (*ctx).r3.u32 = 0;
    }
}
"#;

std::fs::write(rt_src.join("lib.rs"), rt_lib)?;

        // Now emit the actual mapping table into ppc_rt/src/func_mapping.rs
        self.emit_rt_mapping(&rt_src)?;

        Ok(())
    }

    /// Emit the mapping table into the ppc_rt crate as `func_mapping.rs`.
    ///
    /// Layout:
    ///   ppc_rt/src/lib.rs          -> defines FuncPtr, PPCFuncMapping, C-ABI exports
    ///   ppc_rt/src/func_mapping.rs -> defines PPC_FUNC_MAPPINGS + PPCFuncMappings
    pub fn emit_rt_mapping(&self, rt_src: &Path) -> Result<()> {
        use std::fs;

        let mut out = String::new();

        out.push_str("// @generated — Xenon Recompiler function mapping\n");
        out.push_str("// This file is included from ppc_rt/src/lib.rs\n\n");
        out.push_str("use crate::{FuncPtr, PPCFuncMapping};\n\n");

        // ------------------------------
        // 1) Rust-side mapping (slice)
        // ------------------------------
        out.push_str("pub static PPC_FUNC_MAPPINGS: &[(u32, FuncPtr)] = &[\n");

        let mut c_entries: Vec<String> = Vec::new();

        // 1a) Primary functions
        for f in &self.functions {
            let base = f.base as u32;
            let name = Self::resolve_func_name(self, f);

            if let Some(chunk_name) = self.chunk_crate_for_func(base) {
                out.push_str(&format!(
                    "    (0x{base:08X}, {chunk_name}::{name}),\n"
                ));

                c_entries.push(format!(
                    "    PPCFuncMapping {{ guest: 0x{base:08X}, host: {chunk_name}::{name} }},\n"
                ));
            } else {
                eprintln!(
                    "⚠️  ppc_rt: no chunk crate found for function 0x{base:08X}; skipping"
                );
            }
        }

        // 1b) Aliases
        for al in &self.aliases {
            if let Some(chunk_name) = self.chunk_crate_for_func(al.primary) {
                out.push_str(&format!(
                    "    (0x{:08X}, {chunk_name}::sub_{:08X}),\n",
                    al.alias,
                    al.primary
                ));

                c_entries.push(format!(
                    "    PPCFuncMapping {{ guest: 0x{:08X}, host: {chunk_name}::sub_{:08X} }},\n",
                    al.alias,
                    al.primary
                ));
            } else {
                eprintln!(
                    "⚠️  ppc_rt: no chunk crate found for alias primary 0x{:08X}; \
                    skipping alias 0x{:08X}",
                    al.primary,
                    al.alias
                );
            }
        }

        out.push_str("];\n\n");

        // ------------------------------
        // 2) C-visible mapping (array + sentinel)
        // ------------------------------
        // Sentinel stub – never called in normal flow, but type-correct.
        out.push_str(
            "fn ppc_sentinel_stub(_ctx: &mut ppc_ctx::PPCContext, _base: *mut u8, _entry_pc: u32) {}\n\n",
        );

        out.push_str("#[no_mangle]\n");
        out.push_str(&format!(
            "pub static PPCFuncMappings: [PPCFuncMapping; {}] = [\n",
            c_entries.len() + 1
        ));
        for line in &c_entries {
            out.push_str(line);
        }
        // Sentinel: guest == 0 terminates the C++ loop.
        out.push_str("    PPCFuncMapping { guest: 0, host: ppc_sentinel_stub },\n");
        out.push_str("];\n");

        fs::create_dir_all(rt_src)?;
        fs::write(rt_src.join("func_mapping.rs"), out)?;
        Ok(())
    }

}
