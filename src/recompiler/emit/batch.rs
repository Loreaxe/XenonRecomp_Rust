// src/recompiler/emit/batch.rs

use anyhow::*;
use std::path::{Path, PathBuf};

use crate::disasm::PpcCs;
use crate::function::Function;
use crate::log::Phase;
use crate::xlog;
use crate::recompiler::Recompiler;

impl Recompiler {
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
}
