// src/recompiler/emit/workspace.rs

use anyhow::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::log::Phase;
use crate::recompiler::templates::{PPC_CONTEXT_RS, PPC_RT_RS};
use crate::recompiler::Recompiler;

impl Recompiler {
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

            // 🔴 Do not map import thunks (or any addr that has an extern wrapper)
            if self.extern_wrappers.contains_key(&base) {
                // Optional: debug
                // eprintln!("ppc_rt: skip extern thunk 0x{:08X}", base);
                continue;
            }

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
            // If either side is an import, don't map it.
            if self.extern_wrappers.contains_key(&al.alias)
                || self.extern_wrappers.contains_key(&al.primary)
            {
                continue;
            }

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
