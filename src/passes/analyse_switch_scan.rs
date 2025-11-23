// src/passes/analyse_switch_scans.rs

use anyhow::*;
use crate::pipeline::{Pass, Ctx};
use crate::switch::{SwitchType, run_switch_collect, write_switch_toml};
use crate::db::{SwitchEntry, SwitchKind};
use crate::log::Phase;
use crate::xlog;
use crate::xdebug;
use crate::image::SectionFlags;

pub struct AnalyseSwitchScan { pub dump_to: Option<String> }

impl Pass for AnalyseSwitchScan {
    fn name(&self) -> &'static str { "AnalyseSwitchScan" }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let _phase = Phase::new("pass::AnalyseSwitchScan");
        let mut tables = run_switch_collect(ctx.img, &ctx.cs)?; // make mutable

        // Helper: true if `va` lies in any CODE section.
        fn is_code_addr(ctx: &Ctx, va: u32) -> bool {
            for s in &ctx.img.sections {
                if !s.flags.contains(SectionFlags::CODE) {
                    continue;
                }
                let lo = s.base;
                let hi = s.base.wrapping_add(s.data.len() as u32);
                if va >= lo && va < hi {
                    return true;
                }
            }
            false
        }

        // Post-filter each table's labels/default using CODE range sanity.
        for t in &mut tables {
            // Default label sanity
            if t.default_label != 0 && !is_code_addr(ctx, t.default_label) {
                xdebug!(
                    "SW: dropping non-code default label 0x{:08X} for switch @ 0x{:08X}",
                    t.default_label,
                    t.base
                );
                t.default_label = 0;
            }

            // Label list sanity
            let old_len = t.labels.len();
            t.labels.retain(|&lbl| lbl != 0 && is_code_addr(ctx, lbl));
            let new_len = t.labels.len();

            if new_len != old_len {
                xdebug!(
                    "SW: pruned {} invalid labels for switch @ 0x{:08X}",
                    old_len - new_len,
                    t.base
                );
            }
        }

        xlog!("SW: found {} switch tables", tables.len());

        for t in &tables {
            xdebug!(
                "SW: base=0x{:08X} r={} labels={} type={:?}",
                t.base, t.r, t.labels.len(), t.ty
            );
            ctx.db.switches.push(SwitchEntry {
                base: t.base,
                r: t.r,
                default_label: t.default_label,
                labels: t.labels.clone(),
                kind: match t.ty {
                    SwitchType::Absolute    => SwitchKind::Absolute,
                    SwitchType::Computed    => SwitchKind::Computed,
                    SwitchType::ByteOffset  => SwitchKind::ByteOffset,
                    SwitchType::ShortOffset => SwitchKind::ShortOffset,
                },
            });
        }

        if let Some(path) = &self.dump_to {
            let _p = Phase::new("pass::AnalyseSwitchScan.dump_toml");
            write_switch_toml(&tables, path)?;
            xlog!("SW: wrote {}", path);
        }
        Ok(())
    }
}
