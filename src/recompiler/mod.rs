// src/recompiler/mod.rs
use anyhow::*;
use std::collections::{HashMap, BTreeMap};
use std::path::PathBuf;


use crate::config::{RecompilerConfig, RecompilerSwitchTable};
use crate::function::Function;
use crate::image::Image;
use crate::xlog;
use crate::db::FunctionAlias;

mod instructions;
pub use instructions::lower_one;

mod ppc_context;
pub mod symbol;
mod externs;

// NEW: split big impl into separate modules
mod analyse;
mod emit;

/// Local variable tracking used by the lowering stage.
#[derive(Copy, Clone, Debug)]
pub struct RecompilerLocalVariables {
    pub ctr: bool,
    pub xer: bool,
    pub reserved: bool,
    pub cr: [bool; 8],
    pub r: [bool; 32],
    pub f: [bool; 32],
    pub v: [bool; 128],
    pub env: bool,
    pub temp: bool,
    pub vtemp: bool,
    pub ea: bool,
}

impl Default for RecompilerLocalVariables {
    fn default() -> Self {
        Self {
            ctr: false,
            xer: false,
            reserved: false,
            cr: [false; 8],
            r: [false; 32],
            f: [false; 32],
            v: [false; 128],
            env: false,
            temp: false,
            vtemp: false,
            ea: false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CSRState {
    Unknown,
    FPU,
    VMX,
}
impl Default for CSRState {
    fn default() -> Self {
        CSRState::Unknown
    }
}

#[inline]
pub const fn compute_mask(mut mstart: u32, mut mstop: u32) -> u64 {
    mstart &= 0x3F;
    mstop  &= 0x3F;

    let left  = u64::MAX >> mstart;
    let right = if mstop >= 63 { 0 } else { u64::MAX >> (mstop + 1) };

    let value = left ^ right;
    if mstart <= mstop { value } else { !value }
}

pub struct Recompiler {
    pub image: Image,
    pub functions: Vec<Function>,
    pub out: String,
    pub file_index: usize,
    pub config: RecompilerConfig,

    // Analysis products
    pub switch_tables: HashMap<u32, RecompilerSwitchTable>, // base -> (r, labels)

    pub extern_wrappers: HashMap<u32, String>,

    // Raw XEX bytes (needed for import thunk collection)
    pub xex_bytes: Vec<u8>,

    // Filenames like "ppc_000.rs" captured as we emit files
    pub generated_files: Vec<String>,

    // Instruction IDs we didn’t know how to lower (id → (mnemonic, first_addr))
    pub missing_insns: BTreeMap<u32, (String, u32)>,

    // Alternate entrypoints that share a body with a primary function.
    pub aliases: Vec<FunctionAlias>,
}

impl Recompiler {
    // EIEIO word used to detect MMIO stores
    pub const C_EIEIO: u32 = 0x7C00_06AC;

    /// (Optional) last-resort cap so a single bad function can’t explode output.
    pub const MAX_FUNC_BYTES: usize = 256 * 1024; // 256 KiB during emission

    /// Construct explicitly (avoids `Image: Default` requirement).
    pub fn new(image: Image, config: RecompilerConfig) -> Self {
        xlog!(
            "RECOMP: new image base=0x{:08X} size={} entry=0x{:08X}",
            image.base,
            image.size,
            image.entry_point
        );
        Self {
            image,
            functions: Vec::new(),
            out: String::new(),
            file_index: 0,
            config,
            switch_tables: HashMap::new(),
            extern_wrappers: HashMap::new(),
            aliases: Vec::new(),
            xex_bytes: Vec::new(),
            generated_files: Vec::new(),
            missing_insns: BTreeMap::new(),
        }
    }

    /// Record an unimplemented instruction (first time only).
    pub fn note_unhandled_insn(&mut self, id: u32, mnemonic: &str, addr: u32) {
        self.missing_insns
            .entry(id)
            .or_insert_with(|| (mnemonic.to_string(), addr));
    }

    /// Print a summary of unimplemented instructions after the recompile.
    pub fn report_unhandled_insns(&self) {
        if self.missing_insns.is_empty() {
            return;
        }

        println!();
        println!("⚠️  Unimplemented PPC instructions encountered during recompile:");
        for (id, (mnemonic, addr)) in &self.missing_insns {
            println!(
                "   {:>12}  (id = 0x{:04X}, first seen at 0x{:08X})",
                mnemonic, id, addr
            );
        }
        println!(
            "   → add handlers under src/recompiler/instructions/handle/* and wire them into dispatch.rs"
        );
    }

    /// Preferred constructor when you already have the raw XEX bytes.
    pub fn new_with_blob(image: Image, config: RecompilerConfig, xex_bytes: Vec<u8>) -> Self {
        let mut s = Self::new(image, config);
        s.xex_bytes = xex_bytes;
        s
    }

    /// Optional helper to attach the XEX bytes post-construction.
    pub fn attach_blob(&mut self, xex_bytes: Vec<u8>) {
        self.xex_bytes = xex_bytes;
    }

    pub fn load_config(&mut self, config_path: &str) -> Result<()> {
        xlog!("RECOMP: load_config '{}'", config_path);
        self.config = RecompilerConfig::load_from_file(config_path)?;
        Ok(())
    }
}
