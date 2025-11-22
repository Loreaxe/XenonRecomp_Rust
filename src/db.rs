// src/db.rs
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwitchKind {
    #[default]
    Absolute,
    Computed,
    ByteOffset,
    ShortOffset,
}

#[derive(Debug, Clone, Default)]
pub struct SwitchEntry {
    pub base: u32,
    pub r: u32,
    pub default_label: u32,
    pub labels: Vec<u32>,
    pub kind: SwitchKind,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionInfo {
    pub base: u32,
    pub size: u32,
    pub blocks: Vec<(u32, u32)>, // (rel_base, size)
}

/// Simple symbol record: address + preferred name
#[derive(Debug, Clone, Default)]
pub struct SymbolInfo {
    pub addr: u32,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct PdataEntry {
    pub begin: u32,      // BeginAddress (VA)
    pub end: u32,        // begin + func_words*4
    pub flags: u8,       // low 8 bits of Data
    pub func_words: u16, // bits 8..19 of Data
    pub extra: u16,      // high bits 20..31, raw (uninterpreted)
    pub raw: u32,        // original Data dword
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FunctionAlias {
    /// The “real” function entrypoint (usually a .pdata root).
    pub primary: u32,
    /// An alternate entrypoint that shares the same body.
    pub alias: u32,
}

#[derive(Default, Debug)]
pub struct AnalysisDb {
    pub switches: Vec<SwitchEntry>,
    pub functions: Vec<FunctionInfo>,
    pub invalid_insn: HashSet<u32>,

    /// C++/Rust-friendly symbol names (sub_XXXXXXXX, thunk names, etc.)
    pub symbols: Vec<SymbolInfo>,

    pub pdata:     Vec<PdataEntry>,
    pub longjmp: Option<u32>,
    pub setjmp: Option<u32>,


    /// Alternate entrypoints that share a body with a primary function.
    pub aliases: Vec<FunctionAlias>,
}
