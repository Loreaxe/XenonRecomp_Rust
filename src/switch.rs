// src/switch.rs
#![allow(clippy::needless_return)]

use anyhow::*;
use capstone::arch::ppc::PpcInsn;

use crate::disasm::PpcCs;
use crate::image::{Image, SectionFlags};
use crate::ppc::PpcRaw;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchType {
    Absolute,
    Computed,
    ByteOffset,
    ShortOffset,
}

impl Default for SwitchType {
    fn default() -> Self {
        SwitchType::Absolute
    }
}

#[derive(Default, Debug, Clone)]
pub struct SwitchTable {
    pub labels: Vec<u32>,
    pub base: u32,          // address of the BCTR (jump site)
    pub default_label: u32,
    pub r: u32,
    pub ty: SwitchType,
}

// ---------- tiny helpers -----------------------------------------------------

#[inline]
fn is_any(id: u32, xs: &[PpcInsn]) -> bool {
    xs.iter().any(|&k| k as u32 == id)
}

#[inline]
fn is_default_guard_branch(id: u32) -> bool {
    is_any(
        id,
        &[
            PpcInsn::PPC_INS_BGT,
            PpcInsn::PPC_INS_BGTLR,
            PpcInsn::PPC_INS_BLE,
            PpcInsn::PPC_INS_BLELR,
            PpcInsn::PPC_INS_BGE,
            PpcInsn::PPC_INS_BGELR,
            PpcInsn::PPC_INS_BLT,
            PpcInsn::PPC_INS_BLTLR,
        ],
    )
}

#[inline]
fn is_cmp_imm(id: u32) -> bool {
    is_any(
        id,
        &[
            PpcInsn::PPC_INS_CMPLWI,
            PpcInsn::PPC_INS_CMPLI,
            PpcInsn::PPC_INS_CMPWI,
        ],
    )
}

// Build Capstone IDs once per section
fn build_ids(cs: &PpcCs, words: &[u32], base: u32) -> Vec<u32> {
    let mut out = Vec::with_capacity(words.len());
    for (i, &w) in words.iter().enumerate() {
        let pc = base + (i as u32 * 4);
        out.push(cs.disasm_once(w, pc).map(|d| d.id).unwrap_or(0));
    }
    out
}

// Straight window match over IDs (like the C++ SearchMask)
fn search_mask_idxs(ids: &[u32], pattern: &[u32]) -> Vec<usize> {
    let n = ids.len();
    let m = pattern.len();
    if m == 0 || n < m {
        return Vec::new();
    }
    let mut hits = Vec::new();
    for i in 0..=n - m {
        let mut ok = true;
        for k in 0..m {
            if ids[i + k] != pattern[k] {
                ok = false;
                break;
            }
        }
        if ok {
            hits.push(i);
        }
    }
    hits
}

// Backward probe to recover r / labels / default (C++ ScanTable)
//
// `at_index` is the index of the *BCTR* (or equivalent final jump) in the
// current section's word array, so that we can set `table.base` to its PC.
fn scan_table(
    ids: &[u32],
    words: &[u32],
    base: u32,
    at_index: usize,
    table: &mut SwitchTable,
) {
    let mut found_cr_gate = false;

    for back in 0..32 {
        if at_index < back {
            break;
        }
        let i = at_index - back;
        let pc = base + (i as u32 * 4);
        let raw = PpcRaw::from_host_word(words[i]);
        let id = *ids.get(i).unwrap_or(&0);

        if is_default_guard_branch(id) {
            if !found_cr_gate {
                found_cr_gate = true;
                // some encodings have no imm target -> compute from LI/AA
                table.default_label = raw.branch_target(pc);
            }
            continue;
        }

        if found_cr_gate && is_cmp_imm(id) {
            // PPC_INS_CMPLWI/CMPLI/CMPWI enc: r in bits 16..20, imm in low16
            let w = raw.word();
            table.r = ((w >> 16) & 0x1f) as u32;
            let max = (w & 0xffff) as u32;

            table.labels.resize((max + 1) as usize, 0);
            // IMPORTANT: record the *jump* site (BCTR) as the switch base,
            // not the LIS/ADDI sequence. `at_index` is the BCTR index.
            table.base = base + (at_index as u32 * 4);
            break;
        }
    }
}

// Compose (hi,lo) immediate pair at index s (LIS/ADDIS with ADDI/ORI)
fn compose_pair(ids: &[u32], words: &[u32], s: usize) -> Option<u32> {
    let up = *words.get(s)?;
    let lo = *words.get(s + 1)?;
    let up_id = *ids.get(s)?;
    let lo_id = *ids.get(s + 1)?;
    let up_raw = PpcRaw::from_host_word(up);
    let lo_raw = PpcRaw::from_host_word(lo);
    let hi = (up_raw.uimm16() as u32) << 16;

    if (up_id == PpcInsn::PPC_INS_LIS as u32 || up_id == PpcInsn::PPC_INS_ADDIS as u32)
        && lo_id == PpcInsn::PPC_INS_ADDI as u32
    {
        return Some(hi.wrapping_add(lo_raw.simm16() as i32 as u32));
    }
    if (up_id == PpcInsn::PPC_INS_LIS as u32 || up_id == PpcInsn::PPC_INS_ADDIS as u32)
        && lo_id == PpcInsn::PPC_INS_ORI as u32
    {
        return Some(hi | lo_raw.uimm16());
    }
    None
}

// Read labels using the same positional rules as C++ ReadTable()
// - start = the pattern start index (s)
// - base_addr = section base VA (currently unused but kept for parity)
fn read_table_at(
    image: &Image,
    words: &[u32],
    ids: &[u32],
    _base_addr: u32,
    start: usize,
    table: &mut SwitchTable,
) {
    // pOffset from first pair at s..s+1
    let p_offset = compose_pair(ids, words, start).unwrap_or(0);

    match table.ty {
        SwitchType::Absolute => {
            for i in 0..table.labels.len() {
                let ea = p_offset.wrapping_add((i * 4) as u32);
                if let Some(val) = image.read_be_u32(ea) {
                    table.labels[i] = val;
                }
            }
        }
        SwitchType::Computed => {
            // base from s+4,s+5 ; shift from RLWINM at s+3
            let base = compose_pair(ids, words, start + 4).unwrap_or(0);
            let rlw = PpcRaw::from_host_word(words[start + 3]).word();
            let shift = (rlw >> 11) & 0x1f;

            for i in 0..table.labels.len() {
                let ea = p_offset.wrapping_add(i as u32);
                let byte = image
                    .find(ea)
                    .and_then(|(s, k)| s.data.get(k))
                    .copied()
                    .unwrap_or(0) as u32;
                table.labels[i] = base.wrapping_add(byte << shift);
            }
        }
        SwitchType::ByteOffset => {
            // base from s+3,s+4 ; offsets are u8 at pOffset
            let base = compose_pair(ids, words, start + 3).unwrap_or(0);
            for i in 0..table.labels.len() {
                let ea = p_offset.wrapping_add(i as u32);
                let byte = image
                    .find(ea)
                    .and_then(|(s, k)| s.data.get(k))
                    .copied()
                    .unwrap_or(0) as u32;
                table.labels[i] = base.wrapping_add(byte);
            }
        }
        SwitchType::ShortOffset => {
            // base from s+4,s+5 ; offsets are be u16 at pOffset + 2*i
            let base = compose_pair(ids, words, start + 4).unwrap_or(0);
            for i in 0..table.labels.len() {
                let ea = p_offset.wrapping_add((i * 2) as u32);
                let val = if let Some((s, k)) = image.find(ea) {
                    if k + 2 <= s.data.len() {
                        u16::from_be_bytes([s.data[k], s.data[k + 1]]) as u32
                    } else {
                        0
                    }
                } else {
                    0
                };
                table.labels[i] = base.wrapping_add(val);
            }
        }
    }
}

// ---------- main collection (exact C++ patterns) -----------------------------
pub fn run_switch_collect(image: &Image, cs: &PpcCs) -> Result<Vec<SwitchTable>> {
    // Exact sequences (C++ parity)
    let absolute_variants: &[[u32; 6]] = &[
        [
            PpcInsn::PPC_INS_LIS as u32,
            PpcInsn::PPC_INS_ADDI as u32,
            PpcInsn::PPC_INS_RLWINM as u32, // rlwinm form
            PpcInsn::PPC_INS_LWZX as u32,
            PpcInsn::PPC_INS_MTCTR as u32,
            PpcInsn::PPC_INS_BCTR as u32,
        ],
        [
            PpcInsn::PPC_INS_LIS as u32,
            PpcInsn::PPC_INS_ADDI as u32,
            PpcInsn::PPC_INS_SLWI as u32,   // slwi alias form
            PpcInsn::PPC_INS_LWZX as u32,
            PpcInsn::PPC_INS_MTCTR as u32,
            PpcInsn::PPC_INS_BCTR as u32,
        ],
    ];
    let computed: [u32; 8] = [
        PpcInsn::PPC_INS_LIS as u32,
        PpcInsn::PPC_INS_ADDI as u32,
        PpcInsn::PPC_INS_LBZX as u32,
        PpcInsn::PPC_INS_RLWINM as u32,
        PpcInsn::PPC_INS_LIS as u32,
        PpcInsn::PPC_INS_ADDI as u32,
        PpcInsn::PPC_INS_ADD as u32,
        PpcInsn::PPC_INS_MTCTR as u32,
    ];
    let byte_off: [u32; 7] = [
        PpcInsn::PPC_INS_LIS as u32,
        PpcInsn::PPC_INS_ADDI as u32,
        PpcInsn::PPC_INS_LBZX as u32,
        PpcInsn::PPC_INS_LIS as u32,
        PpcInsn::PPC_INS_ADDI as u32,
        PpcInsn::PPC_INS_ADD as u32,
        PpcInsn::PPC_INS_MTCTR as u32,
    ];
    let short_off: [u32; 8] = [
        PpcInsn::PPC_INS_LIS as u32,
        PpcInsn::PPC_INS_ADDI as u32,
        PpcInsn::PPC_INS_RLWINM as u32,
        PpcInsn::PPC_INS_LHZX as u32,
        PpcInsn::PPC_INS_LIS as u32,
        PpcInsn::PPC_INS_ADDI as u32,
        PpcInsn::PPC_INS_ADD as u32,
        PpcInsn::PPC_INS_MTCTR as u32,
    ];

    let mut out: Vec<SwitchTable> = Vec::new();

    for sec in &image.sections {
        if !sec.flags.contains(SectionFlags::CODE) {
            continue;
        }
        if sec.data.len() < 4 {
            continue;
        }

        // Host-order words
        let words: Vec<u32> = sec
            .data
            .chunks_exact(4)
            .map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        // Capstone IDs once
        let ids = build_ids(cs, &words, sec.base);

        // scan one pattern family
        let mut scan_family = |pat: &[u32], ty: SwitchType| {
            let pat_len = pat.len();
            for s in search_mask_idxs(&ids, pat) {
                let bctr_idx = s + pat_len - 1;

                let mut table = SwitchTable {
                    ty,
                    ..Default::default()
                };
                scan_table(&ids, &words, sec.base, bctr_idx, &mut table);
                if table.base == 0 || table.labels.is_empty() {
                    continue;
                }

                read_table_at(image, &words, &ids, sec.base, s, &mut table);
                out.push(table);
            }
        };

        for pat in absolute_variants {
            scan_family(pat, SwitchType::Absolute);
        }
        scan_family(&computed, SwitchType::Computed);
        scan_family(&byte_off,   SwitchType::ByteOffset);
        scan_family(&short_off,  SwitchType::ShortOffset);
    }

    out.sort_by_key(|t| t.base);
    Ok(out)
}

// ---------- emitters (unchanged) --------------------------------------------
pub fn write_switch_toml(tables: &[SwitchTable], out_path: &str) -> Result<()> {
    use std::fmt::Write as _;
    let mut out = String::from("# Generated by XenonAnalyse (Rust)\n");

    let hdr = |ty: SwitchType| -> &'static str {
        match ty {
            SwitchType::Absolute => "# ---- ABSOLUTE JUMPTABLE ----",
            SwitchType::Computed => "# ---- COMPUTED JUMPTABLE ----",
            SwitchType::ByteOffset => "# ---- OFFSETED JUMPTABLE ----",
            SwitchType::ShortOffset => "# ---- OFFSETED WORD JUMPTABLE ----",
        }
    };

    let mut last: Option<SwitchType> = None;
    for t in tables {
        if last != Some(t.ty) {
            out.push('\n');
            out.push_str(hdr(t.ty));
            out.push('\n');
            last = Some(t.ty);
        }
        writeln!(out, "[[switch]]")?;
        writeln!(out, "base = 0x{:08X}", t.base)?;
        writeln!(out, "r = {}", t.r)?;
        writeln!(out, "default = 0x{:08X}", t.default_label)?;
        writeln!(out, "labels = [")?;
        for v in &t.labels {
            writeln!(out, "    0x{v:08X},")?;
        }
        writeln!(out, "]\n")?;
    }

    std::fs::write(out_path, out)?;
    Ok(())
}

pub fn write_switch_rs(tables: &[SwitchTable], out_path: &str) -> Result<()> {
    use std::fmt::Write as _;

    let mut out = String::new();
    out.push_str("//! Auto-generated by XenonAnalyse (Rust)\n");
    out.push_str("#![allow(clippy::upper_case_acronyms)]\n\n");

    out.push_str("#[derive(Copy, Clone, Debug, PartialEq, Eq)]\n");
    out.push_str("pub enum SwitchKind { Absolute, Computed, ByteOffset, ShortOffset }\n\n");

    out.push_str("#[derive(Debug)]\n");
    out.push_str("pub struct SwitchSpec {\n");
    out.push_str("    pub base: u32,\n");
    out.push_str("    pub r: u32,\n");
    out.push_str("    pub default_label: u32,\n");
    out.push_str("    pub kind: SwitchKind,\n");
    out.push_str("    pub labels: &'static [u32],\n");
    out.push_str("}\n\n");

    for (i, t) in tables.iter().enumerate() {
        writeln!(out, "static LABELS_{i}: [u32; {}] = [", t.labels.len())?;
        for v in &t.labels {
            writeln!(out, "    0x{v:08X},")?;
        }
        writeln!(out, "];\n")?;
    }

    out.push_str("pub static SWITCHES: &[SwitchSpec] = &[\n");
    for (i, t) in tables.iter().enumerate() {
        let kind = match t.ty {
            SwitchType::Absolute => "SwitchKind::Absolute",
            SwitchType::Computed => "SwitchKind::Computed",
            SwitchType::ByteOffset => "SwitchKind::ByteOffset",
            SwitchType::ShortOffset => "SwitchKind::ShortOffset",
        };
        writeln!(
            out,
            "    SwitchSpec {{ base: 0x{:08X}, r: {}, default_label: 0x{:08X}, kind: {kind}, labels: &LABELS_{i} }},",
            t.base, t.r, t.default_label
        )?;
    }
    out.push_str("];\n");

    std::fs::write(out_path, out)?;
    Ok(())
}

pub fn run_switch_scan(image: &Image, out_path: &str) -> Result<()> {
    let cs = PpcCs::new()?;
    let tables = run_switch_collect(image, &cs)?;
    write_switch_rs(&tables, out_path)
}
