// src/recompiler/analyse.rs
use anyhow::*;
use crate::config::RecompilerSwitchTable;
use crate::disasm::PpcCs;
use crate::log::Phase;
use crate::xlog;

use crate::function::{Block, Function};

use super::Recompiler;

impl Recompiler {
    /// Normalize possibly-RVA addresses to VA.
    #[inline]
    fn to_va(&self, addr: u32) -> u32 {
        // Heuristic: if it's smaller than the image size, treat as RVA.
        if addr < self.image.size as u32 {
            self.image.base.wrapping_add(addr)
        } else {
            addr
        }
    }

    /// Ensure all function bases and switch-table keys/labels are VA, not RVA.
    fn normalize_va_invariants(&mut self) {
        let base = self.image.base;
        let size = self.image.size;

        #[inline]
        fn to_va_with(base: u32, size: u32, addr: u32) -> u32 {
            if addr < size {
                base.wrapping_add(addr)
            } else {
                addr
            }
        }
        let mut_to_va = |addr: u32| to_va_with(base, size, addr);

        // --- Functions ---
        for f in &mut self.functions {
            let b = f.base as u32;
            if b < size {
                let va = mut_to_va(b);
                if va != b {
                    xlog!("RECOMP: fixup function base 0x{:08X} → 0x{:08X}", b, va);
                    f.base = va as usize;
                }
            }
        }

        // --- Switch tables (keys + labels) ---
        let mut remap: Vec<(u32, RecompilerSwitchTable)> = Vec::new();
        for (k, v) in std::mem::take(&mut self.switch_tables) {
            let new_k = mut_to_va(k);
            let new_labels: Vec<u32> = v.labels.into_iter().map(|x| mut_to_va(x)).collect();
            remap.push((new_k, RecompilerSwitchTable { r: v.r, labels: new_labels }));
        }
        for (k, v) in remap {
            self.switch_tables.insert(k, v);
        }
    }

    /// Mirrors analysis: function carve + switch-table ingest.
    ///
    /// NOTE: we now **only** trust explicit seeds:
    ///   - `[functions]` in the recompiler TOML, or
    ///   - functions already present (e.g. via `seed_from_analysis_db`).
    ///
    /// We no longer do our own `.pdata`-based carving here.
    pub fn analyse(&mut self) -> Result<()> {
        let _phase = Phase::new("recomp::analyse");
        ensure!(
            !self.image.sections.is_empty(),
            "image not initialised; load it before analyse()"
        );

        let cs = PpcCs::new()?;

        if self.functions.is_empty() {
            xlog!(
                "RECOMP: analyse(): no functions present; did you forget to seed_from_analysis_db()?"
            );
        }

        // Normalize RVAs → VAs for everything we have.
        self.normalize_va_invariants();
        xlog!("RECOMP: functions after normalisation = {}", self.functions.len());

        // Switch tables:
        if self.switch_tables.is_empty() {
            let _p_scan = Phase::new("recomp::scan_switch");
            let scanned = crate::switch::run_switch_collect(&self.image, &cs)?;
            for t in scanned {
                self.switch_tables
                    .entry(t.base)
                    .or_insert_with(|| RecompilerSwitchTable {
                        r: t.r,
                        labels: t.labels,
                    });
            }
            xlog!(
                "RECOMP: switch tables discovered by scan = {}",
                self.switch_tables.len()
            );
        } else {
            xlog!(
                "RECOMP: using {} switch table(s) from analysis DB",
                self.switch_tables.len()
            );
        }

        Ok(())
    }

    /// Seed functions using explicit (base,size) from config and analyse each span.
    /// Accepts either VA or RVA in the config; RVAs will be normalized to VA.
    pub fn ingest_functions_from_config(&mut self, cs: &PpcCs) -> Result<()> {
        let _p = Phase::new("recomp::ingest_functions_from_config");
        for (&base_raw, &size) in &self.config.functions {
            let va = self.to_va(base_raw);
            xlog!(
                "RECOMP: config fn raw=0x{:08X} → va=0x{:08X} size={}",
                base_raw,
                va,
                size
            );
            if let Some((sec, off)) = self.image.find(va) {
                let end = (off as u32).saturating_add(size) as usize;
                let end = end.min(sec.data.len());
                let words: Vec<u32> = sec.data[off..end]
                    .chunks_exact(4)
                    .map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]))
                    .collect();
                self.functions.push(Function::analyze(&words, va as usize, cs));
            }
        }
        Ok(())
    }

    /// Seed the recompiler directly from the analysis database instead of TOML.
    pub fn seed_from_analysis_db(&mut self, db: &crate::db::AnalysisDb) {
        self.functions.clear();
        self.switch_tables.clear();

        // ---- functions ----
        for f in &db.functions {
            let blocks: Vec<Block> = f
                .blocks
                .iter()
                .map(|(base, size)| Block {
                    base: *base as usize,
                    size: *size as usize,
                    projected_size: None,
                    #[cfg(debug_assertions)]
                    parent: 0,
                })
                .collect();

            self.functions.push(Function {
                base: f.base as usize,
                size: f.size as usize,
                blocks,
            });
        }

        // ---- switches ----
        for sw in &db.switches {
            self.switch_tables.insert(
                sw.base,
                RecompilerSwitchTable {
                    r: sw.r,
                    labels: sw.labels.clone(),
                },
            );
        }

        // Make sure everything is in VA space, not RVAs.
        self.normalize_va_invariants();

        xlog!(
            "RECOMP: seeded {} function(s), {} switch table(s) from analysis DB",
            self.functions.len(),
            self.switch_tables.len()
        );
    }

}
