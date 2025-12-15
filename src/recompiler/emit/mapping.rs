// src/recompiler/emit/mapping.rs

use anyhow::*;
use std::path::PathBuf;

use crate::log::Phase;
use crate::xlog;
use crate::recompiler::Recompiler;

impl Recompiler {
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

            // 🔴 Do not map import thunks (or any addr that has an extern wrapper)
            if self.extern_wrappers.contains_key(&base) {
                // Optional debug:
                // xlog!("MAPPING: skip import thunk 0x{:08X}", base);
                continue;
            }

            let name = Self::resolve_func_name(self, f);

            if let Some(module) = self.func_modules.get(&base) {
                entries.push_str(&format!(
                    "    (0x{base:08X}, crate::{}::{}),\n",
                    module, name
                ));
            } else {
                entries.push_str(&format!("    (0x{:08X}, {}),\n", base, name));
            }
        }

        // 2) Aliases: (alias_addr -> crate::ppc_###::sub_PRIMARY)
        for al in &self.aliases {
            // If either side is an import, don't map it.
            if self.extern_wrappers.contains_key(&al.alias)
                || self.extern_wrappers.contains_key(&al.primary)
            {
                continue;
            }

            if let Some(module) = self.func_modules.get(&al.primary) {
                entries.push_str(&format!(
                    "    (0x{:08X}, crate::{}::sub_{:08X}),\n",
                    al.alias,
                    module,
                    al.primary
                ));
            } else {
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
}
