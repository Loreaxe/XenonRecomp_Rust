// src/recompiler/emit/helpers.rs

use anyhow::*;
use crate::function::Function;
use crate::image::SectionFlags;
use crate::recompiler::{Recompiler, RecompilerLocalVariables};

impl Recompiler {
    /// For the given function, emit comments describing any switch tables
    /// whose base address lies inside [fn_base .. fn_base+fn_size).
    pub(crate) fn emit_switch_comments_for_fn(&mut self, fnc: &Function) {
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
        for (&thunk_base, name) in &self.extern_wrappers {
            if va >= thunk_base && va < thunk_base + 0x10 {
                return Some(name.as_str());
            }
        }

        None
    }

    /// Clamp an emission range to the containing CODE section to avoid runaway prints.
    #[inline]
    pub(crate) fn clamp_to_code_section(&self, start_va: usize, size: usize) -> usize {
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
    pub(crate) fn resolve_func_name(_rec: &Recompiler, f: &Function) -> String {
        format!("sub_{:08X}", f.base as u32)
    }

    // Emit locals as Rust `let mut` bindings.
    pub(crate) fn emit_locals(buf: &mut String, locals: &RecompilerLocalVariables) {
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

    /// Copy out code bytes for [start_va .. start_va+size) into an owned Vec<u8>.
    pub(crate) fn get_code_bytes(&self, start_va: usize, size: usize) -> Result<Vec<u8>> {
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
}
