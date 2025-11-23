// src/recompiler/emit.rs
use anyhow::*;
use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::disasm::PpcCs;
use crate::function::Function;
use crate::image::SectionFlags;
use crate::log::Phase;
use crate::xlog;
use crate::xdebug;
use crate::xtrace;

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
            "pub fn {}(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {{",
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

        // Initial PC = first block's start address.
        let entry_pc = blocks
            .first()
            .map(|(pc, _)| *pc)
            .unwrap_or(fnc.base as u32);
        self.println_fmt(format_args!("    let mut pc: u32 = 0x{entry_pc:08X};"));
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
        // 3b) Emit alias entrypoints (if any)
        // -------------------------------------
        //
        // For any alias whose primary == this function's base, emit a tiny wrapper:
        //
        //   pub fn sub_ALIAS(ctx, base) { sub_PRIMARY(ctx, base); }
        //
        // The mapping table will still map (ALIAS_ADDR -> sub_PRIMARY), but having a
        // named alias is useful for debug / potential direct calls.
        let primary_base = fnc.base as u32;
        if !self.aliases.is_empty() {
            // Take a snapshot of all aliases for this primary so we don't
            // hold an immutable borrow of `self` while emitting code.
            let aliases_for_this_fn: Vec<_> = self
                .aliases
                .iter()
                .copied() // FunctionAlias is Copy
                .filter(|al| al.primary == primary_base)
                .collect();

            for al in aliases_for_this_fn {
                let alias_name   = format!("sub_{:08X}", al.alias);
                let primary_name = &name; // already the "sub_XXXXXXXX" for this fn

                self.println_fmt(format_args!(
                    "pub fn {}(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {{",
                    alias_name
                ));
                self.println_fmt(format_args!("    {}(ctx, base);", primary_name));
                self.println("}");
                self.println("");
            }
        }

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

        // Build extern wrappers & mapping first
        self.emit_externs_if_any()?;

        // Emit a Rust mapping table up-front so you get a stable index.
        self.emit_rust_mapping()?;

        let mut in_this_file = 0usize;
        let total = self.functions.len();

        println!("Starting static recompile of {total} functions...");

        for idx in 0..total {
            let fnc_ptr: *const Function = &self.functions[idx];
            let fnc: &Function = unsafe { &*fnc_ptr };

            println!("> lowering 0x{:08X} ({} / {})", fnc.base, idx + 1, total);

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

            if in_this_file == 250 {
                println!("\n→ wrote batch #{:03} (250 functions)", self.file_index);
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

        // NEW: summary of any instructions we didn't know how to lower.
        self.report_unhandled_insns();

        Ok(())
    }

    /// Write `self.out` to a file (skips write if content unchanged).
    pub fn save_current_out_data(&mut self, name: Option<&str>) -> Result<()> {
        let _p = Phase::new("recomp::save_current_out_data");
        let dir = if self.config.out_directory_path.is_empty() {
            ".".into()
        } else {
            self.config.out_directory_path.clone()
        };
        std::fs::create_dir_all(&dir).ok();

        let fname = name
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("ppc_{:03}.rs", self.file_index));
        let path = PathBuf::from(&dir).join(&fname);

        // Note the fully-qualified Result::Ok here to avoid any Ok() shadowing.
        if let std::result::Result::Ok(existing) = std::fs::read(&path) {
            if existing.len() == self.out.len() && existing == self.out.as_bytes() {
                xlog!("RECOMP: unchanged '{}', skipping write", path.display());
                self.out.clear();
                if fname.starts_with("ppc_") && fname.ends_with(".rs") {
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

        std::fs::write(&path, &self.out)?;
        xlog!("RECOMP: wrote '{}'", path.display());
        self.out.clear();

        if fname.starts_with("ppc_") && fname.ends_with(".rs") {
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
    ///   pub static PPC_FUNC_MAPPINGS: &[(u32, FuncPtr)] = &[(0xADDR, sub_XXXXXXXX), ...];
    pub fn emit_rust_mapping(&mut self) -> Result<()> {
        let _p = Phase::new("recomp::emit_rust_mapping");

        // Header
        self.println("// Auto-generated function mapping (Rust style)");
        self.println("use crate::recompiler::ppc_context::PPCContext;");
        self.println("pub type FuncPtr = fn(&mut PPCContext, *mut u8);");
        self.println("");

        // Stage entries without mut-borrowing self while iterating
        let mut entries = String::new();

        // 1) Primary functions: (base -> sub_BASE)
        for f in &self.functions {
            let base = f.base as u32;
            let name = Self::resolve_func_name(self, f);
            entries.push_str(&format!("    (0x{:08X}, {}),\n", base, name));
        }

        // 2) Aliases: (alias_addr -> sub_PRIMARY)
        //
        // NOTE: We intentionally map aliases directly to the primary function's
        // implementation, not to the tiny alias wrapper. Semantically, a call
        // to the alias VA should execute the same body as the primary.
        for al in &self.aliases {
            entries.push_str(&format!(
                "    (0x{:08X}, sub_{:08X}),\n",
                al.alias,
                al.primary
            ));
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

        // Chunk modules (mod + pub use)
        for f in &self.generated_files {
            if let Some(stem) = f.strip_suffix(".rs") {
                s.push_str(&format!("#[path = \"{}\"] mod {};\n", f, stem));
                s.push_str(&format!("pub use {}::*;\n", stem));
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

}
