// src/passes/analyse_function.rs
use anyhow::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::ppc::PpcRaw;

use crate::{
    function::Function,
    image::{Section, SectionFlags},
    pipeline::{Ctx, Pass},
};
use crate::log::Phase;
use crate::xlog;
use crate::xdebug;

use rayon::prelude::*; // parallel BL analyze

pub struct AnalyseFunctions;

impl Pass for AnalyseFunctions {
    fn name(&self) -> &'static str {
        "AnalyseFunctions"
    }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let _phase = Phase::new("pass::AnalyseFunctions");
        xlog!(
            "FUN: sections={} entry=0x{:08X}",
            ctx.img.sections.len(),
            ctx.img.entry_point
        );

        // -------------- helpers --------------

        // We recompute aliases from scratch on each run.
        ctx.db.aliases.clear();

        // PPC_OP == 18 and LK == 1  => BL
        #[inline]
        fn is_bl(x: u32) -> bool {
            let opcode = (x >> 26) & 0x3F;
            let lk = x & 1;
            opcode == 18 && lk == 1
        }

        // This is the exact equivalent of the old PPC_BI() + PC logic:
        //   size_t address = base + (data - section.data) + PPC_BI(insn);
        //
        //   LI is 24-bit signed, displacement = sign_extend(LI<<2).
        fn branch_target(x: u32, pc: u32) -> u32 {
            // Reuse the canonical PPC branch logic (handles AA / sign-ext correctly)
            let raw = PpcRaw::from_host_word(x);
            raw.branch_target(pc)
        }

        fn find_subslice(hay: &[u8], needle: &[u8]) -> Option<usize> {
            if needle.is_empty() || needle.len() > hay.len() {
                return None;
            }
            for i in 0..=hay.len() - needle.len() {
                if &hay[i..i + needle.len()] == needle {
                    return Some(i);
                }
            }
            None
        }

        let find_sig = |image: &crate::image::Image, pat: &[u8]| -> Option<u32> {
            for sec in &image.sections {
                if !sec.flags.contains(SectionFlags::CODE) {
                    continue;
                }
                if sec.data.len() < pat.len() {
                    continue;
                }
                if let Some(pos) = find_subslice(&sec.data, pat) {
                    return Some(sec.base + pos as u32);
                }
            }
            None
        };

        // Small helper to push a function record.
        fn push_fn_if_new(seen: &mut HashSet<u32>, ctx: &mut Ctx, base: u32, size: u32) {
            if seen.insert(base) {
                ctx.db.functions.push(crate::db::FunctionInfo {
                    base,
                    size,
                    blocks: Vec::new(), // no CFG persisted
                });
            }
        }

        // Helper: check if an address lies inside any known (base,size) function range.
        #[inline]
        fn is_inside_existing_fn(ctx: &Ctx, addr: u32) -> bool {
            ctx.db.functions.iter().any(|f| {
                let start = f.base;
                let end = f.base.wrapping_add(f.size);
                addr >= start && addr < end
            })
        }

        // Helper: find a function whose base == addr, returning its size.
        #[inline]
        #[allow(dead_code)]
        fn existing_fn_size(ctx: &Ctx, addr: u32) -> Option<u32> {
            ctx.db
                .functions
                .iter()
                .find(|f| f.base == addr)
                .map(|f| f.size)
        }

        // Map all CODE sections.
        #[derive(Clone, Copy)]
        struct CodeSpan {
            base: u32,
            end: u32,
            idx: usize,
        }

        let mut code_spans: Vec<CodeSpan> = Vec::new();
        for (i, s) in ctx.img.sections.iter().enumerate() {
            if s.flags.contains(SectionFlags::CODE) && !s.data.is_empty() {
                xdebug!(
                    "FUN: span[{}] '{}' VA=0x{:08X}..0x{:08X} len={}",
                    i,
                    s.name,
                    s.base,
                    s.base.wrapping_add(s.data.len() as u32),
                    s.data.len()
                );
                code_spans.push(CodeSpan {
                    base: s.base,
                    end: s.base.wrapping_add(s.data.len() as u32),
                    idx: i,
                });
            }
        }
        code_spans.sort_by_key(|c| c.base);

        // Binary search used for mapping BL targets & .pdata.
        let find_code_span = |va: u32, spans: &[CodeSpan]| -> Option<CodeSpan> {
            let mut lo = 0usize;
            let mut hi = spans.len();
            while lo < hi {
                let mid = (lo + hi) / 2;
                let c = spans[mid];
                if va < c.base {
                    hi = mid;
                } else if va >= c.end {
                    lo = mid + 1;
                } else {
                    return Some(c);
                }
            }
            None
        };

        #[inline]
        fn words_for_section(sec: &Section) -> Vec<u32> {
            sec.data
                .chunks_exact(4)
                .map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]))
                .collect()
        }

        // -------- Precompute words for each CODE section (index by section idx) --------
        let _p_words = Phase::new("pass::AnalyseFunctions.precompute_words");
        let mut words_per_sec: Vec<Option<Vec<u32>>> = vec![None; ctx.img.sections.len()];
        for span in &code_spans {
            words_per_sec[span.idx] =
                Some(words_for_section(&ctx.img.sections[span.idx]));
        }
        drop(_p_words);

       // If `addr` is inside one or more .pdata function ranges, snap it to the
        // *nearest* function start: the one with the largest begin <= addr < end.
        //
        // Additionally, respect db.aliases created from .pdata EH landing-pad
        // pairs: if `addr` is exactly an alias entrypoint, canonicalise to its
        // primary.
        #[inline]
        fn canonicalize_to_pdata(ctx: &Ctx, addr: u32) -> u32 {
            if addr == 0 {
                return 0;
            }

            // First, if this address is explicitly known as an alias, canonicalise
            // directly to its primary function root.
            if let Some(a) = ctx.db.aliases.iter().find(|a| a.alias == addr) {
                return a.primary;
            }

            let mut best_start: u32 = 0;

            for p in &ctx.db.pdata {
                let start = p.begin;
                let end   = p.end;

                if addr >= start && addr < end {
                    if start > best_start {
                        best_start = start;
                    }
                }
            }

            if best_start != 0 {
                best_start
            } else {
                addr
            }
        }

        // True if `addr` lies inside any raw .pdata [begin, end) entry.
        #[inline]
        fn is_inside_pdata(ctx: &Ctx, addr: u32) -> bool {
            ctx.db
                .pdata
                .iter()
                .any(|p| addr >= p.begin && addr < p.end)
        }

        // True if the entire [start, end) interval is contained in a single .pdata entry.
        #[inline]
        fn gap_inside_single_pdata(ctx: &Ctx, start: u32, end: u32) -> bool {
            if start >= end {
                return false;
            }
            ctx.db
                .pdata
                .iter()
                .any(|p| start >= p.begin && end <= p.end)
        }

        // -------- signatures (big-endian bytes) --------
        // These correspond 1:1 to the patterns in label_regsaveloads().
        const SIG_SAVEGPRLR_CHAIN: &[u8] = &[
            0xF9, 0xC1, 0xFF, 0x68,
            0xF9, 0xE1, 0xFF, 0x70,
        ];
        const SIG_RESTGPRLR_CHAIN: &[u8] = &[
            0xE9, 0xC1, 0xFF, 0x68,
            0xE9, 0xE1, 0xFF, 0x70,
        ];

        const SIG_SAVEFPR_CHAIN: &[u8] = &[
            0xD9, 0xCC, 0xFF, 0x70,
            0xD9, 0xEC, 0xFF, 0x78,
        ];
        const SIG_RESTFPR_CHAIN: &[u8] = &[
            0xC9, 0xCC, 0xFF, 0x70,
            0xC9, 0xEC, 0xFF, 0x78,
        ];

        const SIG_SAVEVMX_CHAIN_14_32: &[u8] = &[
            0x39, 0x60, 0xFE, 0xE0,
            0x7D, 0xCB, 0x61, 0xCE,
        ];
        const SIG_RESTVMX_CHAIN_14_32: &[u8] = &[
            0x39, 0x60, 0xFE, 0xE0,
            0x7D, 0xCB, 0x60, 0xCE,
        ];

        const SIG_SAVEVMX_CHAIN_64_128: &[u8] = &[
            0x39, 0x60, 0xFC, 0x00,
            0x10, 0x0B, 0x61, 0xCB,
        ];
        const SIG_RESTVMX_CHAIN_64_128: &[u8] = &[
            0x39, 0x60, 0xFC, 0x00,
            0x10, 0x0B, 0x60, 0xCB,
        ];

        // Real longjmp entry
        const SIG_LONGJMP: &[u8] = &[
            0x7C, 0x08, 0x02, 0xA6,
            0x94, 0x21, 0xFF, 0xB0,
            0x90, 0x01, 0x00, 0x50,
        ];

        // Real setjmp entry
        const SIG_SETJMP: &[u8] = &[
            0x2C, 0x00, 0x00, 0x00,
            0x7C, 0x09, 0x03, 0xA6,
            0x4D, 0x82, 0x00, 0x20,
            0x7C, 0x08, 0x02, 0xA6,
        ];

        // -------- resolve thunk bases (signature only, then canonicalize via .pdata) --------
        let restgprlr_14_address =
            find_sig(ctx.img, SIG_RESTGPRLR_CHAIN).unwrap_or(0);
        let savegprlr_14_address =
            find_sig(ctx.img, SIG_SAVEGPRLR_CHAIN).unwrap_or(0);
        let restfpr_14_address =
            find_sig(ctx.img, SIG_RESTFPR_CHAIN).unwrap_or(0);
        let savefpr_14_address =
            find_sig(ctx.img, SIG_SAVEFPR_CHAIN).unwrap_or(0);
        let restvmx_14_address =
            find_sig(ctx.img, SIG_RESTVMX_CHAIN_14_32).unwrap_or(0);
        let savevmx_14_address =
            find_sig(ctx.img, SIG_SAVEVMX_CHAIN_14_32).unwrap_or(0);
        let restvmx_64_address =
            find_sig(ctx.img, SIG_RESTVMX_CHAIN_64_128).unwrap_or(0);
        let savevmx_64_address =
            find_sig(ctx.img, SIG_SAVEVMX_CHAIN_64_128).unwrap_or(0);

        // Resolve longjmp/setjmp (signature only, then canonicalize via .pdata).
        let mut longjmp_address: u32 =
            find_sig(ctx.img, SIG_LONGJMP).unwrap_or(0);
        let mut setjmp_address: u32 =
            find_sig(ctx.img, SIG_SETJMP).unwrap_or(0);

        // Snap them to their .pdata root if they lie inside a runtime function.
        longjmp_address = canonicalize_to_pdata(ctx, longjmp_address);
        setjmp_address = canonicalize_to_pdata(ctx, setjmp_address);

        xlog!(
            "FUN: thunk bases restgpr14=0x{:08X} savegpr14=0x{:08X} restfpr14=0x{:08X} \
             savefpr14=0x{:08X} restvmx14=0x{:08X} savevmx14=0x{:08X} restvmx64=0x{:08X} \
             savevmx64=0x{:08X} longjmp=0x{:08X} setjmp=0x{:08X}",
            restgprlr_14_address,
            savegprlr_14_address,
            restfpr_14_address,
            savefpr_14_address,
            restvmx_14_address,
            savevmx_14_address,
            restvmx_64_address,
            savevmx_64_address,
            longjmp_address,
            setjmp_address,
        );

        let mut seen: HashSet<u32> = HashSet::new();
        let mut inner_pdata_entries: Vec<u32> = Vec::new();

        #[inline]
        fn seed_thunk_chain(
            seen: &mut HashSet<u32>,
            ctx: &mut Ctx,
            base_start: u32, // address of the start_reg thunk (e.g. __savegprlr_14)
            start_reg: u32,
            end_reg: u32,    // exclusive, e.g. 32
            fn_size: u32,    // pattern.fn_size (per-reg stride in bytes)
            final_size: u32, // pattern.final_size (size of last thunk body)
        ) {
            if base_start == 0 {
                return;
            }
            if end_reg <= start_reg {
                return;
            }

            // Total bytes from start_reg entrypoint to final BLR.
            let reg_count = end_reg - start_reg;
            let total_len = (reg_count - 1) * fn_size + final_size;

            for r in start_reg..end_reg {
                let delta_regs = r - start_reg;
                let offset = delta_regs * fn_size;
                let base = base_start.wrapping_add(offset);

                // Body size for this entrypoint: from its label to the shared tail BLR.
                let size = total_len - offset;

                push_fn_if_new(seen, ctx, base, size);
            }
        }

        // -------- seed save/restore thunks --------
        {
            let _p_seed = Phase::new("pass::AnalyseFunctions.seed_thunks");

            // GPR chain: "__savegprlr_%d" / "__restgprlr_%d"
            //
            // IDA pattern:
            //   { "__savegprlr_%d", ..., true,  14, 32, 4, 12 }
            //   { "__restgprlr_%d", ..., false, 14, 32, 4, 16 }
            seed_thunk_chain(
                &mut seen,
                ctx,
                savegprlr_14_address,
                14,
                32,
                4,
                12,
            );
            seed_thunk_chain(
                &mut seen,
                ctx,
                restgprlr_14_address,
                14,
                32,
                4,
                16,
            );

            // FPR chain: "__savefpr_%d" / "__restfpr_%d"
            //
            // IDA pattern:
            //   { "__savefpr_%d", ..., true,  14, 32, 4, 8 }
            //   { "__restfpr_%d", ..., false, 14, 32, 4, 8 }
            seed_thunk_chain(
                &mut seen,
                ctx,
                savefpr_14_address,
                14,
                32,
                4,
                8,
            );
            seed_thunk_chain(
                &mut seen,
                ctx,
                restfpr_14_address,
                14,
                32,
                4,
                8,
            );

            // VMX 14..31/32: "__savevmx_%d" / "__restvmx_%d" (classic 32-reg VMX)
            //
            // IDA pattern:
            //   { "__savevmx_%d", ..., true,  14, 32, 8, 12 }
            //   { "__restvmx_%d", ..., false, 14, 32, 8, 12 }
            seed_thunk_chain(
                &mut seen,
                ctx,
                savevmx_14_address,
                14,
                32,
                8,
                12,
            );
            seed_thunk_chain(
                &mut seen,
                ctx,
                restvmx_14_address,
                14,
                32,
                8,
                12,
            );

            // VMX 64..127/128: "__savevmx_%d" / "__restvmx_%d" (vmx128 style)
            //
            // IDA pattern:
            //   { "__savevmx_%d", ..., true,  64, 128, 8, 12 }
            //   { "__restvmx_%d", ..., false, 64, 128, 8, 12 }
            seed_thunk_chain(
                &mut seen,
                ctx,
                savevmx_64_address,
                64,
                128,
                8,
                12,
            );
            seed_thunk_chain(
                &mut seen,
                ctx,
                restvmx_64_address,
                64,
                128,
                8,
                12,
            );

            // Seed setjmp/longjmp as normal functions so BL scan doesn't have to guess.
            if longjmp_address != 0 {
                push_fn_if_new(
                    &mut seen,
                    ctx,
                    longjmp_address,
                    SIG_LONGJMP.len() as u32,
                );
            }
            if setjmp_address != 0 {
                push_fn_if_new(
                    &mut seen,
                    ctx,
                    setjmp_address,
                    SIG_SETJMP.len() as u32,
                );
            }

            xlog!("FUN: seeded {} thunk funcs", seen.len());
        }

        // -------- seed from entry point (if present) --------
        if ctx.img.entry_point != 0 {
            if let Some(span) = find_code_span(ctx.img.entry_point, &code_spans) {
                let sec = &ctx.img.sections[span.idx];
                if let Some(words) = &words_per_sec[span.idx] {
                    let start_idx = ((ctx.img.entry_point - sec.base) / 4) as usize;
                    if start_idx < words.len() {
                        let f = Function::analyze(
                            &words[start_idx..],
                            ctx.img.entry_point as usize,
                            &ctx.cs,
                        );
                        if f.size != 0 {
                            if seen.insert(f.base as u32) {
                                ctx.db.functions.push(crate::db::FunctionInfo {
                                    base: f.base as u32,
                                    size: f.size as u32,
                                    blocks: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
        }

        // -------- range utilities & prologue heuristic for BL scan --------

        #[inline]
        fn build_ranges_from_db(ctx: &Ctx) -> Vec<(u32, u32)> {
            let mut ranges: Vec<(u32, u32)> = ctx
                .db
                .functions
                .iter()
                .map(|f| (f.base, f.base.wrapping_add(f.size)))
                .collect();
            ranges.sort_by_key(|r| r.0);
            ranges
        }

        #[inline]
        fn addr_in_ranges(ranges: &[(u32, u32)], addr: u32) -> bool {
            let mut lo = 0usize;
            let mut hi = ranges.len();
            while lo < hi {
                let mid = (lo + hi) / 2;
                let (start, end) = ranges[mid];
                if addr < start {
                    hi = mid;
                } else if addr >= end {
                    lo = mid + 1;
                } else {
                    return true;
                }
            }
            false
        }

        // Very cheap “does this look like a function prologue?” heuristic.
        #[inline]
        fn likely_function_start(words: &[u32], idx: usize) -> bool {
            if idx >= words.len() {
                return false;
            }
            let w0 = words[idx];
            let w1 = *words.get(idx + 1).unwrap_or(&0);
            let w2 = *words.get(idx + 2).unwrap_or(&0);

            // stwu r1, -imm(r1)
            if (w0 & 0xFFFF_0000) == 0x9421_0000 {
                return true;
            }

            // addi r1, r1, -imm  (frame setup without stwu)
            if (w0 & 0xFFFF_0000) == 0x3821_0000 {
                return true;
            }

            // mflr r0
            if w0 == 0x7C08_02A6 {
                return true;
            }

            // mflr r0; stw r0, imm(r1)
            if w0 == 0x7C08_02A6 && (w1 & 0xFFFF_0000) == 0x9001_0000 {
                return true;
            }

            // stwu r1, -imm(r1); mflr r0
            if (w0 & 0xFFFF_0000) == 0x9421_0000 && w1 == 0x7C08_02A6 {
                return true;
            }

            // Common GCC/MSVC pattern:
            //   stwu r1,-imm(r1); mflr r0; stw r0,imm(r1)
            if (w0 & 0xFFFF_0000) == 0x9421_0000
                && w1 == 0x7C08_02A6
                && (w2 & 0xFFFF_0000) == 0x9001_0000
            {
                return true;
            }

            // Tiny dispatchers: lis rX,imm; ori/addi rX,rX,imm
            // These often appear at "naked" functions that immediately call another fn.
            let op0 = (w0 >> 26) & 0x3F;
            if op0 == 15 /* lis */ {
                let op1 = (w1 >> 26) & 0x3F;
                if op1 == 24 /* ori */ || op1 == 14 /* addi */ {
                    return true;
                }
            }

            false
        }

        // Very fast single-path function boundary finder.
        //
        // Now takes:
        //   - `words`: instructions starting at `fn_base`
        //   - `fn_base`: VA of words[0]
        //   - `fn_limit`: upper bound VA; branching past this is considered "leaving"
        //
        // Returns (size_in_bytes, saw_terminator).
        #[inline]
        fn fast_function_size_with_term(
            words: &[u32],
            fn_base: u32,
            fn_limit: u32,
        ) -> (usize, bool) {
            if words.is_empty() {
                return (0, false);
            }

            let w0 = words[0];
            let w1 = *words.get(1).unwrap_or(&0);

            // Tiny frameless helpers: just "blr"
            if w0 == 0x4E80_0020 {
                // blr
                return (4, true);
            }

            if w1 == 0x4E80_0020 && w0 != 0 {
                // X; blr
                return (8, true);
            }

            let mut size: usize = 0;
            let mut saw_term = false;

            // Track the furthest forward *internal* branch target we've seen so far.
            // If a BLR/BCTR occurs before this target, it's an internal early-return,
            // not the true end of the function.
            let mut max_internal_target: u32 = fn_base;

            for &w in words {
                // Do not scan beyond the allowed [fn_base, fn_limit) envelope.
                if fn_base.wrapping_add(size as u32) >= fn_limit {
                    break;
                }

                let pc = fn_base.wrapping_add(size as u32);
                size += 4;

                if w == 0 {
                    // Treat 0 as padding / end of meaningful code.
                    break;
                }

                let op = (w >> 26) & 0x3F;
                let lk = w & 1;

                // Unconditional branch (B), non-link.
                //
                // We only treat it as a *function terminator* if:
                //   - it is a self-loop (b .), OR
                //   - it jumps outside [fn_base, fn_limit).
                //
                // Otherwise it's an internal branch and we keep scanning, and
                // remember the furthest forward target so BLR/BCTR before it
                // are not treated as final terminators.
                if op == 18 && lk == 0 {
                    let raw = PpcRaw::from_host_word(w);
                    let target = raw.branch_target(pc);

                    if target == pc || target < fn_base || target >= fn_limit {
                        saw_term = true;
                        break;
                    } else {
                        // Internal forward branch; remember the furthest target.
                        if target > max_internal_target {
                            max_internal_target = target;
                        }
                        continue;
                    }
                }

                // blr / bctr family, non-link.
                //
                // Normally these terminate the function, but if we've already seen
                // an internal forward branch whose target lies *past* this point,
                // then this BLR/BCTR is just an early-return on one path and the
                // function's code continues at that target.
                if w == 0x4E80_0020 /* blr */ || w == 0x4E80_0420 /* bctr */ {
                    if max_internal_target > pc {
                        // There is a path that jumps past this BLR/BCTR; keep scanning.
                        continue;
                    }

                    saw_term = true;
                    break;
                }
            }

            (size, saw_term)
        }

        #[inline]
        fn fast_function_size(words: &[u32], fn_base: u32, fn_limit: u32) -> usize {
            fast_function_size_with_term(words, fn_base, fn_limit).0
        }

        // Precompute how many instructions BL_scan will look at, for % reporting.
        let total_bl_insns: usize = code_spans
            .iter()
            .map(|span| {
                words_per_sec[span.idx]
                    .as_ref()
                    .map(|w| w.len())
                    .unwrap_or(0)
            })
            .sum();


        // -------- Decode .pdata once, stash in DB, and seed root functions --------
        {
            let _p = Phase::new("pass::AnalyseFunctions.decode_pdata");
            ctx.db.pdata.clear();

            let mut added = 0usize;

            // Track previous .pdata entry to detect EH landing pads
            let mut last_pdata_begin: Option<u32> = None;
            let mut last_pdata_end:   Option<u32> = None;
            let mut last_pdata_has_eh: bool      = false;

            if let Some(sec) = ctx.img.find_section(".pdata") {
                for entry in sec.data.chunks_exact(8) {
                    let begin = u32::from_be_bytes([
                        entry[0], entry[1], entry[2], entry[3],
                    ]);
                    let data  = u32::from_be_bytes([
                        entry[4], entry[5], entry[6], entry[7],
                    ]);

                    if begin == 0 {
                        continue; // unused slot
                    }

                    // Layout matching the XEX RUNTIME_FUNCTION bitfield used in the IDA plugin:
                    //
                    //   bits  0..7  : PrologLength
                    //   bits  8..29 : FunctionLength in 4-byte instructions (22 bits)
                    //   bits 30..31 : FunctionType (2 bits)
                    //
                    // NOTE: if your format differs, reconcile this with your previous "flags/extra"
                    // layout instead of blindly swapping it.
                    let prolog_len = (data & 0xFF) as u8;
                    let func_words = ((data >> 8) & 0x003F_FFFF) as u32; // 22 bits
                    if func_words == 0 {
                        continue;
                    }
                    let func_type  = ((data >> 30) & 0x3) as u8;

                    let size_bytes = func_words.saturating_mul(4);
                    if size_bytes == 0 {
                        continue;
                    }

                    let end    = begin.wrapping_add(size_bytes);

                    // For EH landing-pad detection we only care about whether EH is present.
                    // If you have a specific "EH flag" bit, you can derive has_eh from that.
                    // For now we keep your previous has_eh logic if it came from elsewhere.
                    let has_eh = (data & 0x8000_0000) != 0;

                    // Record the raw entry in db.pdata.
                    ctx.db.pdata.push(crate::db::PdataEntry {
                        begin,
                        end,
                        flags: prolog_len,
                        func_words: func_words.min(u16::MAX as u32) as u16,
                        extra: func_type as u16,
                        raw: data,
                    });

                    // If the previous .pdata entry had EH and this one starts exactly
                    // at its end, treat this as an EH landing pad alias.
                    if let (Some(prev_begin), Some(prev_end)) =
                        (last_pdata_begin, last_pdata_end)
                    {
                        if last_pdata_has_eh && begin == prev_end {
                            // Avoid duplicate aliases.
                            if !ctx.db.aliases.iter().any(|a|
                                a.primary == prev_begin && a.alias == begin
                            ) {
                                ctx.db.aliases.push(crate::db::FunctionAlias {
                                    primary: prev_begin,
                                    alias: begin,
                                });
                                xdebug!(
                                    "FUN: .pdata EH landing pad alias 0x{:08X} -> 0x{:08X}",
                                    begin,
                                    prev_begin
                                );
                            }
                        }
                    }

                    // Seed a root function for each .pdata entry, except for save/restore
                    // millicode (vmx thunks) which we handle via signature-based chains.
                    let is_millicode = match func_type {
                        // Adjust these discriminants to match your RuntimeFunctionType enum.
                        // Example mapping:
                        //   0 = None/Normal
                        //   1 = Export
                        //   2 = SaveMillicode
                        //   3 = RestoreMillicode
                        2 | 3 => true, // SaveMillicode / RestoreMillicode
                        _ => false,
                    };

                    if !is_millicode {
                        if seen.insert(begin) {
                            ctx.db.functions.push(crate::db::FunctionInfo {
                                base: begin,
                                size: size_bytes,
                                blocks: Vec::new(),
                            });
                            added += 1;
                        }
                    }

                    // Remember this entry for the next iteration.
                    last_pdata_begin = Some(begin);
                    last_pdata_end   = Some(end);
                    last_pdata_has_eh = has_eh;
                }
            }

            xlog!(
                "FUN: decoded {} .pdata entries into db.pdata, seeded {} root funcs",
                ctx.db.pdata.len(),
                added
            );
        }

        // ======================== BL scan (parallel analyze) ========================

        {
            let _p = Phase::new("pass::AnalyseFunctions.BL_scan");

            // Build ranges from *pre-BL* functions (thunks, .pdata, config, entry).
            let ranges = build_ranges_from_db(ctx);

            // Phase 1: collect candidate BL targets (sequential, cheap).
            // Each candidate: (tgt, span_idx, start_idx_in_words)
            let mut candidates: Vec<(u32, usize, usize)> = Vec::new();

            let mut processed_bl_insns: usize = 0;
            let mut next_report_pct: usize = 5; // 5%, 10%, 15%, ...

            for span in &code_spans {
                let caller_sec = &ctx.img.sections[span.idx];
                let Some(caller_words) = &words_per_sec[span.idx] else {
                    continue;
                };
                let caller_base = caller_sec.base;

                for (i, &insn) in caller_words.iter().enumerate() {
                    // ---- progress reporting for BL scan (phase 1) ----
                    processed_bl_insns += 1;
                    if total_bl_insns > 0 && next_report_pct <= 100 {
                        let pct = processed_bl_insns * 100 / total_bl_insns;
                        if pct >= next_report_pct {
                            xlog!("FUN: BL scan {:5}%", pct);
                            next_report_pct += 5;
                        }
                    }
                    // ---------------------------------------------------

                    if !is_bl(insn) {
                        continue;
                    }

                    let pc = caller_base + (i as u32) * 4;
                    let tgt = branch_target(insn, pc);

                    // Map target to *any* CODE span (cross-section OK).
                    let Some(tgt_span) = find_code_span(tgt, &code_spans) else {
                        continue;
                    };

                    // Only aligned targets make sense as code.
                    if (tgt & 3) != 0 {
                        continue;
                    }

                    // Is this BL target inside an existing .pdata function?
                    // If yes, canonicalize_to_pdata() returns that function base.
                    let pdata_base = canonicalize_to_pdata(ctx, tgt);
                    let is_inner_pdata = pdata_base != 0 && pdata_base != tgt;

                    // Skip targets that lie inside any known function (O(log N)),
                    // *except* for inner .pdata addresses we want to split out later.
                    if addr_in_ranges(&ranges, tgt) && !is_inner_pdata {
                        continue;
                    }

                    // Dedup exact bases against everything we've seen so far.
                    if !seen.insert(tgt) {
                        continue;
                    }

                    // Inner .pdata target: record it for cheap synthetic functions later,
                    // but do *not* run Function::analyze on it (too slow / redundant).
                    if is_inner_pdata {
                        inner_pdata_entries.push(tgt);
                        continue;
                    }

                    let tgt_sec = &ctx.img.sections[tgt_span.idx];
                    let Some(tgt_words) = &words_per_sec[tgt_span.idx] else {
                        continue;
                    };

                    let start_idx = ((tgt - tgt_sec.base) / 4) as usize;
                    if start_idx >= tgt_words.len() {
                        continue;
                    }

                    // Prologue heuristic optional – left disabled for maximum coverage.
                    // if !likely_function_start(tgt_words, start_idx) { continue; }

                    candidates.push((tgt, tgt_span.idx, start_idx));
                }
            }

            xlog!(
                "FUN: BL scan candidates={} (~{}% of insns had BL)",
                candidates.len(),
                if total_bl_insns > 0 {
                    (candidates.len().min(total_bl_insns) * 100) / total_bl_insns
                } else {
                    0
                }
            );

            // Phase 2: run fast_function_size in parallel over all *normal* candidates.
            let words_ref = &words_per_sec;

            let total_candidates = candidates.len();
            let processed_candidates = AtomicUsize::new(0);
            let next_report_pct_analyze = AtomicUsize::new(5);

            let mut funcs: Vec<crate::db::FunctionInfo> = candidates
                .par_iter()
                .filter_map(|&(tgt, span_idx, start_idx)| {
                    let words = words_ref[span_idx].as_ref().unwrap();

                    // ---- FAST FUNCTION SIZE (no CFG analysis) ----
                    let tgt_sec = &ctx.img.sections[span_idx];
                    let sec_end = tgt_sec.base.wrapping_add(tgt_sec.data.len() as u32);
                    let size = fast_function_size(&words[start_idx..], tgt, sec_end);

                    // ---- analysis-phase progress reporting ----
                    let done = processed_candidates.fetch_add(1, Ordering::Relaxed) + 1;

                    if total_candidates > 0 {
                        let pct = done * 100 / total_candidates;

                        loop {
                            let target = next_report_pct_analyze.load(Ordering::Relaxed);

                            // Up to 90%: report every 5%
                            if target < 90 {
                                if pct >= target {
                                    if next_report_pct_analyze
                                        .compare_exchange(
                                            target,
                                            target + 5,
                                            Ordering::Relaxed,
                                            Ordering::Relaxed,
                                        )
                                        .is_ok()
                                    {
                                        xlog!("FUN: BL analyze {:5}%", pct);
                                    }
                                }
                                break;
                            }

                            // From 90% onward: report every 1%
                            if pct >= target && target <= 100 {
                                if next_report_pct_analyze
                                    .compare_exchange(
                                        target,
                                        target + 1,
                                        Ordering::Relaxed,
                                        Ordering::Relaxed,
                                    )
                                    .is_ok()
                                {
                                    xlog!("FUN: BL analyze {:5}%", pct);
                                }
                            }
                            break;
                        }
                    }
                    // --------------------------------------------

                    if size == 0 {
                        None
                    } else {
                        Some(crate::db::FunctionInfo {
                            base: tgt,
                            size: size as u32,
                            blocks: Vec::new(),
                        })
                    }
                })
                .collect();

            // Dedup by base in case of weird repeated candidates landing at same entry.
            funcs.sort_by_key(|f| f.base);
            funcs.dedup_by_key(|f| f.base);

            let discovered = funcs.len();
            ctx.db.functions.extend(funcs.into_iter());

            xlog!(
                "FUN: BL scan discovered {} new funcs ({} inner .pdata BL targets recorded separately)",
                discovered,
                inner_pdata_entries.len()
            );
        }

                // --- Synthesise aliases for inner .pdata BL targets (cheap) ---
        {
            let _p = Phase::new("pass::AnalyseFunctions.inner_pdata_split");

            if !inner_pdata_entries.is_empty() {
                inner_pdata_entries.sort();
                inner_pdata_entries.dedup();

                // Helper: like AnalyseSwitchBind::find_pdata_range
                fn find_pdata_range_for(ctx: &Ctx, addr: u32) -> Option<(u32, u32)> {
                    ctx.db
                        .pdata
                        .iter()
                        .filter(|p| addr >= p.begin && addr < p.end)
                        .max_by_key(|p| p.begin)
                        .map(|p| (p.begin, p.end))
                }

                // Snapshot of current functions so we can test for “already a root”.
                let funcs_snapshot = ctx.db.functions.clone();
                let mut added_aliases = 0usize;

                for &addr in &inner_pdata_entries {
                    // Already have a function that starts exactly here? Skip.
                    if funcs_snapshot.iter().any(|f| f.base == addr) {
                        continue;
                    }

                    // Must lie inside some .pdata function.
                    let Some((start, end)) = find_pdata_range_for(ctx, addr) else {
                        continue;
                    };
                    if addr < start || addr >= end {
                        continue;
                    }

                    // Avoid duplicate alias records.
                    if ctx.db.aliases.iter().any(|a| a.primary == start && a.alias == addr) {
                        continue;
                    }

                    ctx.db.aliases.push(crate::db::FunctionAlias {
                        primary: start,
                        alias: addr,
                    });
                    added_aliases += 1;
                }

                if added_aliases != 0 {
                    xlog!(
                        "FUN: inner .pdata split added {} alias entrypoints",
                        added_aliases
                    );
                }
            }
        }

       // ======================== Gap rescan (aggressive leaf search) ========================
        //
        // After BL scan, we may still have gaps between known functions that contain tiny
        // frameless / leaf helpers (like 0x821DB5A0..0x821DB5E0 in your sample).
        // We:
        //   - build per-section function coverage from the current DB snapshot
        //   - look for gaps >= GAP_MIN_BYTES
        //   - for each gap, run a local sweep that:
        //       * respects existing function ranges
        //       * does NOT require a prologue
        //       * uses fast_function_size_with_term to carve functions
        {
            let _p = Phase::new("pass::AnalyseFunctions.gap_rescan");

            // Only bother with reasonably large gaps; tiny padding holes are rarely code.
            const GAP_MIN_BYTES: u32 = 0x40;

            let img = &ctx.img;

            // Work on a snapshot of the current functions, sorted by base, so that our
            // gap detection and "already covered" checks are stable for the whole pass.
            ctx.db.functions.sort_by_key(|f| f.base);
            let funcs_snapshot = ctx.db.functions.clone();

            #[derive(Clone, Copy)]
            struct GapJob {
                span_idx: usize,
                start: u32,
                end: u32,
            }

            let mut jobs: Vec<GapJob> = Vec::new();

            // Build gap jobs per CODE section.
            for span in &code_spans {
                let sec = &img.sections[span.idx];
                let sec_start = sec.base;
                let sec_end = sec.base.wrapping_add(sec.data.len() as u32);

                // Collect all functions that lie in this CODE section.
                let mut local_funcs: Vec<&crate::db::FunctionInfo> = funcs_snapshot
                    .iter()
                    .filter(|f| f.base >= sec_start && f.base < sec_end)
                    .collect();
                local_funcs.sort_by_key(|f| f.base);

                let mut prev_end = sec_start;

                for f in local_funcs {
                    let gap_start = prev_end;
                    let gap_end = f.base;

                    if gap_end > gap_start {
                        let gap_size = gap_end - gap_start;
                        // Do NOT try to carve new roots out of a gap that lies entirely
                        // inside a single .pdata-described function. .pdata "owns" that.
                        if gap_size >= GAP_MIN_BYTES
                            && !gap_inside_single_pdata(ctx, gap_start, gap_end)
                        {
                            jobs.push(GapJob {
                                span_idx: span.idx,
                                start: gap_start,
                                end: gap_end,
                            });
                        }
                    }

                    let end = f.base.wrapping_add(f.size);
                    if end > prev_end {
                        prev_end = end;
                    }
                }

                // Tail gap after the last function up to section end.
                if sec_end > prev_end {
                    let gap_start = prev_end;
                    let gap_end = sec_end;
                    let gap_size = gap_end - gap_start;

                    if gap_size >= GAP_MIN_BYTES
                        && !gap_inside_single_pdata(ctx, gap_start, gap_end)
                    {
                        jobs.push(GapJob {
                            span_idx: span.idx,
                            start: gap_start,
                            end: gap_end,
                        });
                    }
                }
            }

            if !jobs.is_empty() {
                let words_ref = &words_per_sec;
                let funcs_snapshot_ref = &funcs_snapshot;

                // Local helper: check coverage against the snapshot (no &Ctx inside Rayon).
                fn addr_in_snapshot(fns: &[crate::db::FunctionInfo], addr: u32) -> bool {
                    fns.iter().any(|f| {
                        let start = f.base;
                        let end = start.wrapping_add(f.size);
                        addr >= start && addr < end
                    })
                }

                let per_job_new: Vec<Vec<crate::db::FunctionInfo>> =
                    jobs.par_iter()
                        .map(|job| {
                            let mut out: Vec<crate::db::FunctionInfo> = Vec::new();

                            let sec = &img.sections[job.span_idx];
                            let Some(sec_words) = &words_ref[job.span_idx] else {
                                return out;
                            };

                            let mut cursor = job.start;
                            let job_end = job.end;

                            while cursor + 4 <= job_end {
                                // Require alignment and skip anything already covered by an
                                // existing function in the snapshot.
                                if (cursor & 3) != 0
                                    || addr_in_snapshot(funcs_snapshot_ref, cursor)
                                {
                                    cursor = cursor.wrapping_add(4);
                                    continue;
                                }

                                let start_idx = ((cursor - sec.base) / 4) as usize;
                                if start_idx >= sec_words.len() {
                                    break;
                                }

                                // Use the same terminator-aware boundary finder we use
                                // elsewhere (no CFG, just a straight-line walk).
                                let sec_end =
                                    sec.base.wrapping_add(sec.data.len() as u32);
                                let gap_limit = job_end.min(sec_end);

                                let (size_usize, has_term) =
                                    fast_function_size_with_term(&sec_words[start_idx..], cursor, gap_limit);
                                let mut size_u32 = size_usize as u32;

                                if size_u32 == 0 || !has_term {
                                    cursor = cursor.wrapping_add(4);
                                    continue;
                                }

                                // Clamp so we don't run past the gap or the section.
                                let sec_end =
                                    sec.base.wrapping_add(sec.data.len() as u32);
                                let max_size =
                                    job_end.min(sec_end).saturating_sub(cursor);
                                if size_u32 > max_size {
                                    size_u32 = max_size;
                                }
                                if size_u32 == 0 {
                                    cursor = cursor.wrapping_add(4);
                                    continue;
                                }

                                out.push(crate::db::FunctionInfo {
                                    base: cursor,
                                    size: size_u32,
                                    blocks: Vec::new(),
                                });

                                // Jump past the just-carved function to avoid overlaps.
                                cursor = cursor.wrapping_add(size_u32);
                            }

                            out
                        })
                        .collect();

                let mut added = 0usize;
                for mut v in per_job_new {
                    added += v.len();
                    ctx.db.functions.append(&mut v);
                }

                if added != 0 {
                    xlog!(
                        "FUN: gap rescan added {} extra (likely leaf) functions",
                        added
                    );
                }
            }
        }

        // --- Pointer-based roots: data-section code pointers (thunks, vtables, etc.) ---
        {
            let _p = Phase::new("pass::AnalyseFunctions.pointer_roots");

            use std::collections::HashSet;

            // Build a set of existing function bases so we don't spam duplicates.
            let mut base_set: HashSet<u32> =
                ctx.db.functions.iter().map(|f| f.base).collect();

            let mut added_funcs  = 0usize;
            let mut added_aliases = 0usize;

            // Scan all NON-CODE sections for 4-byte values that look like code pointers.
            for (sec_idx, sec) in ctx.img.sections.iter().enumerate() {
                // We only care about data-like sections here; CODE is handled elsewhere.
                if sec.flags.contains(SectionFlags::CODE) {
                    continue;
                }
                if sec.data.len() < 4 {
                    continue;
                }

                // Treat as big-endian u32 pointers.
                for (word_idx, chunk) in sec.data.chunks_exact(4).enumerate() {
                    let val = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

                    // Only aligned addresses make sense as code.
                    if (val & 3) != 0 {
                        continue;
                    }

                    // Already a function base? Skip creating a new one.
                    if base_set.contains(&val) {
                        continue;
                    }


                    // If this value points *inside* a .pdata-described function,
                    // we never want to create a brand new root function there.
                    // Instead, treat it as an alias of the .pdata primary.
                    if is_inside_pdata(ctx, val) {
                        // Prefer an existing alias mapping if one exists (e.g. EH pad).
                        if let Some(a) = ctx.db.aliases.iter().find(|a| a.alias == val) {
                            // Alias is already known; nothing more to do here.
                            // We *still* do not create a new function root.
                            let _ = a; // silence unused warning if you like
                        } else if let Some(p) = ctx
                            .db
                            .pdata
                            .iter()
                            .find(|p| val >= p.begin && val < p.end)
                        {
                            let primary = p.begin;
                            if primary != val
                                && !ctx
                                    .db
                                    .aliases
                                    .iter()
                                    .any(|a| a.primary == primary && a.alias == val)
                            {
                                ctx.db.aliases.push(crate::db::FunctionAlias {
                                    primary,
                                    alias: val,
                                });
                                added_aliases += 1;
                            }
                        }
                        continue;
                    }

                    // Must land inside some CODE span.
                    let Some(span) = find_code_span(val, &code_spans) else {
                        continue;
                    };

                    // If this address lies *inside* an existing function, treat it
                    // as an alias instead of a new function root.
                    if is_inside_existing_fn(ctx, val) {
                        if let Some(owner) = ctx.db.functions.iter().find(|f| {
                            let start = f.base;
                            let end   = start.wrapping_add(f.size);
                            val >= start && val < end
                        }) {
                            if !ctx.db.aliases.iter().any(|a| a.primary == owner.base && a.alias == val) {
                                ctx.db.aliases.push(crate::db::FunctionAlias {
                                    primary: owner.base,
                                    alias: val,
                                });
                                added_aliases += 1;
                            }
                        }
                        continue;
                    }

                    let code_sec = &ctx.img.sections[span.idx];
                    let Some(code_words) = &words_per_sec[span.idx] else {
                        continue;
                    };

                    let start_idx = ((val - code_sec.base) / 4) as usize;
                    if start_idx >= code_words.len() {
                        continue;
                    }

                    // Use the same fast, terminator-aware boundary finder we use elsewhere.
                    let sec_end = code_sec.base.wrapping_add(code_sec.data.len() as u32);
                    let (size_usize, has_term) =
                        fast_function_size_with_term(&code_words[start_idx..], val, sec_end);
                    let mut size_u32 = size_usize as u32;

                    if size_u32 == 0 || !has_term {
                        continue;
                    }

                    // Cheap sanity clamp: don't run past the section.
                    let sec_end = code_sec.base.wrapping_add(code_sec.data.len() as u32);
                    let max_size = sec_end.saturating_sub(val);
                    if size_u32 > max_size {
                        size_u32 = max_size;
                    }
                    if size_u32 == 0 {
                        continue;
                    }

                    ctx.db.functions.push(crate::db::FunctionInfo {
                        base: val,
                        size: size_u32,
                        blocks: Vec::new(),
                    });
                    base_set.insert(val);
                    added_funcs += 1;
                }
            }

            if added_funcs != 0 || added_aliases != 0 {
                xlog!(
                    "FUN: pointer_roots added {} functions and {} aliases from data-section code pointers",
                    added_funcs,
                    added_aliases
                );
            }
        }

        // --- Bind longjmp/setjmp to actual function bases via their signatures ---
        {
            let _p = Phase::new("pass::AnalyseFunctions.bind_jumps_to_funcs");

            // Closure: find a *unique* function whose body contains the given signature.
            let find_fn_by_sig = |pat: &[u8]| -> Option<u32> {
                let mut found: Option<u32> = None;

                'outer: for f in &ctx.db.functions {
                    // Find the CODE span / section containing this function.
                    let span = match find_code_span(f.base, &code_spans) {
                        Some(s) => s,
                        None => continue,
                    };

                    let sec = &ctx.img.sections[span.idx];

                    let start_off = (f.base - sec.base) as usize;
                    let end_off = start_off.saturating_add(f.size as usize);

                    if end_off > sec.data.len() || pat.len() > f.size as usize {
                        continue;
                    }

                    let body = &sec.data[start_off..end_off];

                    if find_subslice(body, pat).is_some() {
                        // If we already saw a match, it’s ambiguous – bail out.
                        if found.is_some() {
                            found = None;
                            break 'outer;
                        }
                        found = Some(f.base);
                    }
                }

                found
            };

            // Try to refine longjmp/setjmp using the function list + signatures.
            let longjmp_from_sig = find_fn_by_sig(SIG_LONGJMP);
            let setjmp_from_sig  = find_fn_by_sig(SIG_SETJMP);

            // Fall back to the earlier raw-scan addresses if this fails/ambiguous.
            let longjmp_address = longjmp_from_sig.unwrap_or(longjmp_address);
            let setjmp_address  = setjmp_from_sig.unwrap_or(setjmp_address);

            xlog!(
                "FUN: final longjmp=0x{:08X} setjmp=0x{:08X}",
                longjmp_address,
                setjmp_address
            );

            ctx.db.longjmp = if longjmp_address != 0 {
                Some(longjmp_address)
            } else {
                None
            };

            ctx.db.setjmp = if setjmp_address != 0 {
                Some(setjmp_address)
            } else {
                None
            };

            // If `ctx.cfg` is mutable (i.e. Ctx has `pub cfg: &'a mut RecompilerConfig`),
            // you can persist these for the recompiler / print_function_call:
            //
            // ctx.cfg.longJmpAddress = longjmp_address;
            // ctx.cfg.setJmpAddress  = setjmp_address;
        }

        xlog!("FUN: total functions={}", ctx.db.functions.len());
        Ok(())
    }
}
