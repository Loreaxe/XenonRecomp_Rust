// src/disasm.rs
use anyhow::*;
use capstone::prelude::*;
use capstone::{arch::ppc::ArchMode as PpcMode, Endian};

pub struct PpcCs(Capstone);

/// Small owned instruction we control.
#[derive(Clone, Debug)]
pub struct InsnLite {
    pub id: u32,
    pub address: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: Option<String>,
    pub op_str: Option<String>,
}

impl std::ops::Deref for PpcCs {
    type Target = capstone::Capstone;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl PpcCs {

    pub fn new() -> Result<Self> {
        let cs = Capstone::new()
            .ppc()
            .mode(PpcMode::Mode64)     // Xenon: 64-bit PPC
            .endian(Endian::Big)       // Big‐endian code
            .detail(true)              // enable operand details
            .build()?;
        Ok(Self(cs))
    }

    /// Disassemble a single BE 32-bit instruction at guest PC — return our InsnLite.
    pub fn disasm_once(&self, be_word: u32, pc: u32) -> Result<InsnLite> {
        let bytes = be_word.to_be_bytes();
        let insns = self.0.disasm_all(&bytes, pc as u64)?;
        let i = insns.iter().next().ok_or_else(|| anyhow!("no insn"))?;
        Ok(InsnLite {
            id: i.id().0,
            address: i.address(),
            bytes: i.bytes().to_vec(),
            mnemonic: i.mnemonic().map(|s| s.to_string()),
            op_str: i.op_str().map(|s| s.to_string()),
        })
    }

    /// Capstone IDs from BE words.
    pub fn ids_from_words(&self, words_be: &[u32], base_ea: u32) -> Result<Vec<u32>> {
        let mut bytes = Vec::with_capacity(words_be.len() * 4);
        for w in words_be { bytes.extend_from_slice(&w.to_be_bytes()); }
        let insns = self.0.disasm_all(&bytes, base_ea as u64)?;
        Ok(insns.iter().map(|i| i.id().0).collect())
    }

    /// Full instructions (owned lite form) for window scans.
    pub fn disasm_words(&self, words_be: &[u32], base_ea: u32) -> Result<Vec<InsnLite>> {
        let mut bytes = Vec::with_capacity(words_be.len() * 4);
        for w in words_be { bytes.extend_from_slice(&w.to_be_bytes()); }
        let insns = self.0.disasm_all(&bytes, base_ea as u64)?;
        Ok(insns.iter().map(|i| InsnLite {
            id: i.id().0,
            address: i.address(),
            bytes: i.bytes().to_vec(),
            mnemonic: i.mnemonic().map(|s| s.to_string()),
            op_str: i.op_str().map(|s| s.to_string()),
        }).collect())
    }
}
