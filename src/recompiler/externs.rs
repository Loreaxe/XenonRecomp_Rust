// src/recompiler/externs.rs
//
// Import wrapper / extern handling for the static recompiler.
//
// IMPORTANT:
// - xenon_recomp itself is *only* a generator.
// - This module is used at generation-time to:
//     * analyse the import thunks discovered from the XEX
//     * generate `xam.rs` and `xboxkrnl.rs` in the output folder
//     * populate `Recompiler::extern_wrappers` so lowering can emit
//       calls like `__imp__NtCreateEvent(ctx, base);`.
// - The generated `xam.rs` / `xboxkrnl.rs` are **not** compiled by
//   xenon_recomp itself; they are intended for your host/game crate.
//

use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::recompiler::ppc_context::PPCContext;
use super::Recompiler;

use crate::xex::collect_import_symbols;
use crate::xlog;

type Addr = u32;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ModuleKind {
    Xam,
    XboxKrnl,
}

impl Recompiler {
    /// Simple identifier sanitizer: keep [a-zA-Z0-9], turn everything else into `_`,
    /// and ensure the identifier does not start with a digit.
    #[inline]
    fn sanitize_ident(name: &str) -> String {
        let mut s = String::with_capacity(name.len());
        for ch in name.chars() {
            if ch.is_ascii_alphanumeric() {
                s.push(ch);
            } else {
                s.push('_');
            }
        }
        if s.is_empty() || s.as_bytes()[0].is_ascii_digit() {
            s.insert(0, '_');
        }
        s
    }

    /// Heuristic split: decide whether an import symbol belongs to XAM or XboxKrnl.
    /// Only affects which *generated file* we write it into.
    fn classify_module(sym: &str) -> ModuleKind {
        // XAM-ish things
        if sym.starts_with("NetDll_")
            || sym.starts_with("Xam")
            || sym.starts_with("xam_")
        {
            ModuleKind::Xam
        } else {
            // Everything else defaults to kernel for now.
            ModuleKind::XboxKrnl
        }
    }

    /// Helper: emit a single file (`xam.rs` or `xboxkrnl.rs`) with wrappers.
    fn emit_file_import_wrappers(
        &mut self,
        module: ModuleKind,
        entries: &[(Addr, String, String)], // (thunk_va, original_name, rust_ident)
    ) -> Result<()> {
        if entries.is_empty() {
            return Ok(());
        }

        let (filename, module_label) = match module {
            ModuleKind::Xam      => ("xam.rs",      "xam.xex"),
            ModuleKind::XboxKrnl => ("xboxkrnl.rs", "xboxkrnl.exe"),
        };

        self.println("// Auto-generated import wrappers for this title.");
        self.println(&format!("// module: {}", module_label));
        self.println("// Only functions actually imported by the XEX are listed here.");
        self.println("");
        self.println("#![allow(non_snake_case)]");
        self.println("#![allow(unused_variables)]");
        self.println("");
        self.println("use crate::recompiler::ppc_context::PPCContext;");
        self.println("");

        for (addr, sym_name, rust_ident) in entries {
            self.println_fmt(format_args!(
                "// thunk VA: 0x{addr:08X}  ({sym_name})"
            ));
            self.println_fmt(format_args!(
                "pub fn {rust_ident}(ctx: &mut PPCContext, base: *mut u8) {{"
            ));
            self.println("    // TODO: implement this import for this title.");
            self.println_fmt(format_args!(
                "    unimplemented!(\"{rust_ident} (VA 0x{addr:08X})\");"
            ));
            self.println("}");
            self.println("");
        }

        // Write the file next to the PPC chunks.
        self.save_current_out_data(Some(filename))?;
        Ok(())
    }

/// Main entry used by `Recompiler::emit_externs_if_any`.
    ///
    /// `imports` is `(thunk_va_or_impl_va, symbol_name)` from
    /// `xex::collect_import_symbols`.
    ///
    /// We:
    ///   * generate `xam.rs` and `xboxkrnl.rs` (one stub per symbol)
    ///   * fill `self.extern_wrappers` with *all* VAs that refer to that symbol,
    ///     so lowering can emit `crate::xam::Foo(ctx, base);` regardless
    ///     of whether the BL hits the thunk or the implementation address.
    pub fn emit_rust_externs(&mut self, imports: &[(u32, String)]) -> Result<()> {
        use std::collections::BTreeMap;

        // 1) For each symbol name, choose:
        //    - module (Xam / XboxKrnl)
        //    - a stable Rust identifier (Name)
        //    - a "primary" address (smallest VA) for comments in the stub file.
        //
        //    We use BTreeMap just to keep output deterministic.
        let mut sym_map: BTreeMap<String, (ModuleKind, String, u32)> = BTreeMap::new();

        for (addr, sym_name) in imports {
            let module = Self::classify_module(sym_name);
            let rust_ident = format!("{}", Self::sanitize_ident(sym_name));

            sym_map
                .entry(sym_name.clone())
                .and_modify(|entry| {
                    // Keep the smallest VA as the primary (good for comments).
                    if *addr < entry.2 {
                        entry.2 = *addr;
                    }
                })
                .or_insert((module, rust_ident, *addr));
        }

        // 2) Build per-module entry lists for file emission
        //    (one stub per symbol, using the primary address only for comments).
        let mut xam_entries: Vec<(Addr, String, String)> = Vec::new();
        let mut krnl_entries: Vec<(Addr, String, String)> = Vec::new();

        for (sym_name, (module, rust_ident, primary_addr)) in &sym_map {
            match module {
                ModuleKind::Xam => {
                    xam_entries.push((*primary_addr, sym_name.clone(), rust_ident.clone()));
                }
                ModuleKind::XboxKrnl => {
                    krnl_entries.push((*primary_addr, sym_name.clone(), rust_ident.clone()));
                }
            }
        }

        // Generate the two wrapper files in the output directory.
        let _ = self.emit_file_import_wrappers(ModuleKind::Xam, &xam_entries);
        let _ = self.emit_file_import_wrappers(ModuleKind::XboxKrnl, &krnl_entries);

        // 3) Build the VA -> wrapper-name map, used later by lowering to emit
        //    calls like `crate::xam::NtCreateEvent(ctx, base);`.
        //
        //    IMPORTANT: we insert *every* VA from `imports`, not just the
        //    primary one, so both the thunk address and the actual XAM/krnl
        //    implementation VA resolve to the same wrapper.
        let mut map = HashMap::<u32, String>::new();

        for (addr, sym_name) in imports {
            if let Some((module, rust_ident, _primary)) = sym_map.get(sym_name) {
                let qual = match module {
                    ModuleKind::Xam => format!("crate::xam::{}", rust_ident),
                    ModuleKind::XboxKrnl => format!("crate::xboxkrnl::{}", rust_ident),
                };
                map.insert(*addr, qual);
            }
        }

        self.extern_wrappers = map;

        Ok(())
    }

    pub fn emit_externs_if_any(&mut self) -> Result<()> {
        if self.xex_bytes.is_empty() {
            xlog!("RECOMP: no XEX bytes attached; skipping extern emission");
            return Ok(());
        }

        let imports = match collect_import_symbols(&self.xex_bytes, &self.image) {
            Ok(v) => v,
            Err(e) => {
                xlog!("RECOMP: collect_import_symbols failed: {e:?}");
                return Ok(()); // don't abort recompile just because imports failed
            }
        };

        if imports.is_empty() {
            xlog!("RECOMP: no import thunks found; skipping xam/xboxkrnl emission");
            return Ok(());
        }

        xlog!("RECOMP: emitting extern wrappers for {} thunk(s)", imports.len());
        self.emit_rust_externs(&imports)
    }

    // -------------------------------------------------------------------------
    // Optional runtime-style "catch-all" call helper
    // (only used as a last-resort fallback in generated code).
    // -------------------------------------------------------------------------

    /// Generation-time placeholder. In a pure static recompile workflow this
    /// function will never be called by the generated game code; it's only here
    /// so `crate::recompiler::externs::call` in generation-time logic compiles.
    pub fn externs_call_stub(_ctx: &mut PPCContext, _base: *mut u8, addr: u32) {
        panic!(
            "externs::call() stub hit for VA=0x{:08X} — missing import wrapper?",
            addr
        );
    }
}

// -----------------------------------------------------------------------------
// Runtime-facing stub `call`
// -----------------------------------------------------------------------------
//
// The generated code does things like:
//
//     crate::recompiler::externs::call(ctx, base, 0x82CA9408);
//
// This stub keeps both the generator and any naive host builds compiling.
// For a real game/host crate, you are expected to *override* this module
// with a proper implementation that dispatches to your generated xam.rs /
// xboxkrnl.rs wrappers.

/// Stub implementation of the runtime import dispatcher.
///
/// In a real host/game crate you should provide your own `externs::call`
/// that matches this signature and forwards to the generated wrappers.
pub fn call(ctx: &mut PPCContext, base: *mut u8, addr: u32) {
    // For now, just use the panic-y stub so we notice if anything actually
    // tries to call this when running the generator itself.
    Recompiler::externs_call_stub(ctx, base, addr);
}
