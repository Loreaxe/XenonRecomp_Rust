// src/passes/analyse_switch_bind.rs
#![allow(clippy::needless_return)]

use anyhow::*;
use crate::pipeline::{Pass, Ctx};
use crate::log::Phase;
use crate::xlog;
use crate::xdebug;
use crate::image::SectionFlags;

/// After we have:
///   - a function list (from .pdata + BL scan + signatures)
///   - switch tables (from AnalyseSwitchScan),
///
/// this pass makes sure that every switch jump site (`switch.base` / BCTR)
/// is covered by some function. If not, it uses `.pdata` as the single source
/// of truth to add or extend a function range.
///
/// This is the XEX-only analogue of Auto_Function_Parser.py.
pub struct AnalyseSwitchBind;

impl Pass for AnalyseSwitchBind {
    fn name(&self) -> &'static str {
        "AnalyseSwitchBind"
    }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let _phase = Phase::new("pass::AnalyseSwitchBind");

        // Helper: search db.pdata for the runtime function entry that covers `addr`.
        fn find_pdata_range(ctx: &Ctx, addr: u32) -> Option<(u32, u32)> {
            let mut best: Option<(u32, u32)> = None;

            for p in &ctx.db.pdata {
                if addr >= p.begin && addr < p.end {
                    if let Some((cur_start, _)) = best {
                        if p.begin > cur_start {
                            best = Some((p.begin, p.end));
                        }
                    } else {
                        best = Some((p.begin, p.end));
                    }
                }
            }

            best
        }

        // Helper: canonicalize an address to the .pdata "root" begin, or 0 if none.
        #[inline]
        fn canonicalize_to_pdata(ctx: &Ctx, addr: u32) -> u32 {
            if addr == 0 {
                return 0;
            }

            // Respect db.aliases produced by AnalyseFunctions / decode_pdata:
            // if this address is a known alias entrypoint (e.g. EH landing pad),
            // canonicalise to its primary runtime entry.
            if let Some(a) = ctx.db.aliases.iter().find(|a| a.alias == addr) {
                return a.primary;
            }

            if let Some((start, end)) = find_pdata_range(ctx, addr) {
                if addr >= start && addr < end {
                    start
                } else {
                    0
                }
            } else {
                0
            }
        }

        // Quick helper: is `addr` already inside some known function?
        fn addr_in_functions(funcs: &[crate::db::FunctionInfo], addr: u32) -> bool {
            funcs.iter().any(|f| {
                let start = f.base;
                let end   = f.base.wrapping_add(f.size);
                addr >= start && addr < end
            })
        }

        let mut added    = 0usize;
        let mut extended = 0usize;

        // ======================== Initial bind: make sure each switch site is inside some function ========================
        for sw in &ctx.db.switches {
            let addr = sw.base;

            // Already covered by some function? Done.
            if addr_in_functions(&ctx.db.functions, addr) {
                continue;
            }

            // Try to recover a function range from .pdata.
            let Some((start, end)) = find_pdata_range(ctx, addr) else {
                xdebug!(
                    "SWB: switch at 0x{:08X} not covered by any function or .pdata entry",
                    addr
                );
                continue;
            };

            let size = end.saturating_sub(start);
            if size == 0 {
                continue;
            }

            // See if we already have a function with this base.
            if let Some(f) = ctx.db.functions.iter_mut().find(|f| f.base == start) {
                let old_end = f.base.wrapping_add(f.size);
                if end > old_end {
                    f.size = end.wrapping_sub(f.base);
                    extended += 1;
                    xdebug!(
                        "SWB: extended func 0x{:08X} from 0x{:08X} to 0x{:08X} to cover switch at 0x{:08X}",
                        f.base,
                        old_end,
                        end,
                        addr
                    );
                } else {
                    // Function base already exists and is big enough; just log it.
                    xlog!(
                        "SWB: switch at 0x{:08X} now covered by existing func 0x{:08X}-0x{:08X}",
                        addr,
                        start,
                        end
                    );
                }
            } else {
                // New function discovered purely from .pdata.
                ctx.db.functions.push(crate::db::FunctionInfo {
                    base: start,
                    size,
                    blocks: Vec::new(),
                });
                added += 1;
                xdebug!(
                    "SWB: added func 0x{:08X}-0x{:08X} from .pdata for switch at 0x{:08X}",
                    start,
                    end,
                    addr
                );
            }
        }

        if added != 0 || extended != 0 {
            xlog!(
                "SWB: added {} new functions and extended {} via .pdata binding",
                added,
                extended
            );
            ctx.db.functions.sort_by_key(|f| f.base);
        } else {
            xlog!("SWB: all switch sites already covered by existing functions");
        }

        // ======================== .pdata / switch cluster stitching (post-bind) ========================
        //
        // Now that we've:
        //   - discovered functions (AnalyseFunctions)
        //   - bound switches to .pdata ranges (this pass)
        //
        // we can safely re-cluster .pdata-described functions so that:
        //   - each .pdata root function covers the entire [root .. max_end) cluster
        //   - any helper functions that start inside that range are dropped
        //
        // This is what prevents the "ERROR: 0x832C3818" style arms: all switch cases
        // become part of the dispatcher function's range again.
        {
            let _p = Phase::new("pass::AnalyseSwitchBind.cluster_pdata_switches");
            use std::collections::HashMap;

            // Work on a sorted snapshot so we can binary-search by base.
            let mut funcs_snapshot = ctx.db.functions.clone();
            funcs_snapshot.sort_by_key(|f| f.base);

            // root_base -> max_end
            let mut root_max_end: HashMap<u32, u32> = HashMap::new();
            // function base -> its .pdata root (if any)
            let mut base_to_root: HashMap<u32, u32> = HashMap::new();

            for f in &funcs_snapshot {
                if f.size == 0 {
                    continue;
                }

                let base = f.base;
                let end  = base.wrapping_add(f.size);

                let root = canonicalize_to_pdata(ctx, base);
                if root == 0 {
                    continue;
                }

                base_to_root.insert(base, root);

                let e = root_max_end.entry(root).or_insert(end);
                if end > *e {
                    *e = end;
                }
            }

            if !root_max_end.is_empty() {
                // 1) Extend root function sizes to cover the whole cluster.
                for f in &mut ctx.db.functions {
                    if let Some(&cluster_end) = root_max_end.get(&f.base) {
                        if cluster_end > f.base {
                            f.size = cluster_end.wrapping_sub(f.base);
                        }
                    }
                }

                // 2) Drop inner helpers that lie strictly inside any cluster [root .. cluster_end),
                //    keeping only the root function itself.
                let old_len = ctx.db.functions.len();

                ctx.db.functions.retain(|f| {
                    let base = f.base;

                    let Some(&root) = base_to_root.get(&base) else {
                        // not part of any .pdata cluster
                        return true;
                    };

                    if let Some(&cluster_end) = root_max_end.get(&root) {
                        if base == root {
                            // keep the cluster root itself
                            return true;
                        }
                        if base >= root && base < cluster_end {
                            // inner helper – now merged into the root cluster
                            return false;
                        }
                    }

                    true
                });

                let new_len = ctx.db.functions.len();
                let removed = old_len.saturating_sub(new_len);
                if removed != 0 {
                    xdebug!(
                        "SWB: clustered {} .pdata groups, removed {} inner helper funcs",
                        root_max_end.len(),
                        removed
                    );
                }
            } else {
                xdebug!("SWB: no .pdata clusters to stitch after switch bind");
            }
        }

        // ======================== Switch-based cluster stitching ========================
        //
        // The .pdata-based stitching groups functions that share a runtime entry, but
        // for patterns like:
        //
        //   base=0x832C2B00 (bctr switch) => labels:
        //     0x832C2B2C, 0x832C2BC4, 0x832C3818, 0x832C3D18, ...
        //
        // the "far" labels may end up as separate functions (or lie beyond the current
        // end of the dispatcher). Here we:
        //
        //   - Find the function that *contains* each switch site (sw.base)
        //   - Extend that function to cover all of its labels (using known functions
        //     when present, otherwise at least up to `label + 4`)
        //   - Drop functions whose base lies strictly inside that extended range
        //
        // BUT: we only merge labels that are plausibly part of the same logical function:
        //   - Prefer labels that share the same .pdata range as the owner
        //   - Fallback to labels that are within MAX_SWITCH_CLUSTER_BYTES of the owner
        {
            let _p = Phase::new("pass::AnalyseSwitchBind.switch_cluster");
            use std::collections::{HashMap, HashSet};

            // Hard cap on how far a switch cluster can stretch from its owner.
            // Tune this if needed.
            const MAX_SWITCH_CLUSTER_BYTES: u32 = 0x20000; // 128 KiB

            // Helper: find the function that contains `addr`.
            #[inline]
            fn find_func_containing(
                funcs: &[crate::db::FunctionInfo],
                addr: u32,
            ) -> Option<&crate::db::FunctionInfo> {
                funcs.iter().find(|f| {
                    let start = f.base;
                    let end   = start.wrapping_add(f.size);
                    addr >= start && addr < end
                })
            }

            // Work on a sorted snapshot so that binary_search_by_key is valid.
            let mut funcs_snapshot = ctx.db.functions.clone();
            funcs_snapshot.sort_by_key(|f| f.base);

            // We key clusters by the ".pdata root" of the owner function when available,
            // otherwise by the owner's own base. This ensures that multi-switch dispatchers
            // that have been split into several helper functions under one .pdata entry
            // are stitched back into a single logical function.
            //
            // owner_root_base -> max_end
            let mut cluster_end_by_owner: HashMap<u32, u32> = HashMap::new();

            for sw in &ctx.db.switches {
                let addr = sw.base;

                let Some(owner) = find_func_containing(&funcs_snapshot, addr) else {
                    // Switch site not inside any known function – nothing we can do here.
                    continue;
                };

                let mut owner_base = owner.base;

                // Canonicalise the owner to its .pdata root, if any; otherwise keep the
                // owner as-is. This is the logical "root" for the whole cluster.
                let mut owner_root = canonicalize_to_pdata(ctx, owner_base);
                if owner_root == 0 {
                    owner_root = owner_base;
                }

                // Start the cluster at the end of the *root* function if it exists,
                // otherwise at the end of the immediate owner.
                let initial_end = if let Some(root_fn) =
                    funcs_snapshot.iter().find(|f| f.base == owner_root)
                {
                    root_fn.base.wrapping_add(root_fn.size)
                } else {
                    owner_base.wrapping_add(owner.size)
                };

                let mut cluster_end = initial_end;
                let mut any_label   = false;

                // --- NEW: bubble the owner back to a previous contiguous function, if any.
                //
                // For patterns like:
                //
                //   82B3D5C0: ... (prologue, cmplwi/bgtlr, etc.)
                //   82B3D5E0: lis/addi/slwi/lwzx/mtctr
                //   82B3D5F4: bctr          ; switch jump
                //
                // our initial "owner" is the function starting at 0x82B3D5E0, but the
                // *logical* owner is the immediately preceding function at 0x82B3D5C0,
                // whose end == 0x82B3D5E0. In that case we treat 0x82B3D5C0 as the
                // cluster root so the entire dispatcher (prologue + switches + cases)
                // becomes a single function.
                //
                // We use the sorted snapshot to find the previous function in O(log N).
                // Find the index of the owner function (or its insertion point)
                let idx = funcs_snapshot
                    .binary_search_by_key(&owner_base, |f| f.base)
                    .unwrap_or_else(|i| i);

                if idx > 0 {
                    let prev = &funcs_snapshot[idx - 1];
                    let prev_base = prev.base;
                    let prev_end  = prev.base.wrapping_add(prev.size);

                    // If the previous function ends exactly at the owner's base,
                    // treat that previous function as the real "owner root".
                    if prev_end == owner_base {
                        owner_base = prev_base;

                        // Make sure our initial cluster_end covers both the previous
                        // function and the one that contained the switch.
                        let prev_cluster_end = prev_base.wrapping_add(prev.size);
                        if prev_cluster_end > cluster_end {
                            cluster_end = prev_cluster_end;
                        }
                    }
                }

                // Try to capture the .pdata range that owns this (possibly bubbled) owner.
                let owner_pdata = find_pdata_range(ctx, owner_base);

                // Iterate over all labels (including default, if non-zero)
                for &lbl in sw
                    .labels
                    .iter()
                    .chain(std::iter::once(&sw.default_label))
                {
                    if lbl == 0 {
                        continue;
                    }

                    // Decide whether this label is "eligible" for clustering with the owner.
                    let mut eligible = false;

                    // 1) Prefer labels that share the same .pdata entry as the owner.
                    if let Some((ob, oe)) = owner_pdata {
                        if let Some((lb, le)) = find_pdata_range(ctx, lbl) {
                            if ob == lb && oe == le {
                                eligible = true;
                            }
                        }
                    }

                    // 2) Fallback: allow labels that are simply "nearby" in VA space.
                    if !eligible {
                        let dist = if lbl >= owner_root {
                            lbl - owner_root
                        } else {
                            owner_root - lbl
                        };
                        if dist <= MAX_SWITCH_CLUSTER_BYTES {
                            eligible = true;
                        }
                    }

                    if !eligible {
                        continue;
                    }

                    any_label = true;

                    // Use the function that *contains* this label, not just one that starts at it.
                    let end = if let Some(child) = find_func_containing(&funcs_snapshot, lbl) {
                        child.base.wrapping_add(child.size)
                    } else {
                        // no function contains this label; just ensure we cover the label itself
                        lbl.wrapping_add(4)
                    };

                    // Cap cluster growth so one switch can't grab the world.
                    let max_end    = owner_root.saturating_add(MAX_SWITCH_CLUSTER_BYTES);
                    let end_capped = end.min(max_end);

                    if end_capped > cluster_end {
                        cluster_end = end_capped;
                    }
                }

                if !any_label {
                    continue;
                }

                let e = cluster_end_by_owner
                    .entry(owner_root)
                    .or_insert(cluster_end);
                if cluster_end > *e {
                    *e = cluster_end;
                }
            }

            // Collapse nested clusters: if one owner's [root .. end) is fully contained
            // inside another owner's [other_root .. other_end), we treat the inner one
            // as a helper and drop it as a root. This avoids patterns like:
            //
            //   root A = 0x82B3D5C0, end_A = 0x82B3D6D0
            //   root B = 0x82B3D61C, end_B = 0x82B3D6D0
            //
            // where both host switches but B is entirely within A's dispatcher.
            if cluster_end_by_owner.len() > 1 {
                let mut entries: Vec<(u32, u32)> = cluster_end_by_owner
                    .iter()
                    .map(|(&root, &end)| (root, end))
                    .collect();

                entries.sort_by_key(|&(root, _)| root);

                let mut keep_roots: HashSet<u32> = HashSet::new();

                'outer: for i in 0..entries.len() {
                    let (root_i, end_i) = entries[i];
                    for j in 0..entries.len() {
                        if i == j {
                            continue;
                        }
                        let (root_j, end_j) = entries[j];

                        // root_i cluster lies wholly inside root_j cluster?
                        if root_i > root_j && end_i <= end_j {
                            // Then root_i is just an inner helper cluster; drop it.
                            continue 'outer;
                        }
                    }
                    keep_roots.insert(root_i);
                }

                // Remove any owner whose cluster is fully contained in another.
                cluster_end_by_owner.retain(|root, _| keep_roots.contains(root));
            }

            if !cluster_end_by_owner.is_empty() {
                // 1) Extend owner functions to their cluster end.
                for f in &mut ctx.db.functions {
                    if let Some(&cluster_end) = cluster_end_by_owner.get(&f.base) {
                        if cluster_end > f.base {
                            f.size = cluster_end.wrapping_sub(f.base);
                        }
                    }
                }

                // 2) Drop ANY function whose base lies strictly inside an owner's cluster
                //    (except the owner itself). This merges mid-function stubs, jump-table
                //    loaders, etc. into their dispatcher.
                let old_len = ctx.db.functions.len();

                ctx.db.functions.retain(|f| {
                    let base = f.base;

                    // Keep any owner roots as-is.
                    if cluster_end_by_owner.contains_key(&base) {
                        return true;
                    }

                    // Drop helpers that start anywhere in a cluster interior.
                    for (&owner_root, &cluster_end) in &cluster_end_by_owner {
                        if base > owner_root && base < cluster_end {
                            return false;
                        }
                    }

                    true
                });

                let new_len = ctx.db.functions.len();
                let removed = old_len.saturating_sub(new_len);

                if removed != 0 {
                    xdebug!(
                        "SWB: switch_cluster merged {} case/helper funcs into dispatcher roots",
                        removed
                    );
                } else {
                    xdebug!(
                        "SWB: switch_cluster: no separate case/helper funcs needed merging"
                    );
                }
            } else {
                xdebug!("SWB: switch_cluster found no multi-function switch dispatchers");
            }
        }

        // ======================== Final post-processing (post-bind) ========================
        //
        // Now that:
        //   - .pdata clusters are stitched
        //   - switch-based ranges have been extended
        //
        // we clamp function ranges so they:
        //   - stay inside their CODE section
        //   - do not overlap the next function
        {
            let _p = Phase::new("pass::AnalyseSwitchBind.final_clamp");

            // Build CODE spans
            #[derive(Clone, Copy)]
            struct CodeSpan {
                base: u32,
                end:  u32,
                idx:  usize,
            }

            let mut code_spans: Vec<CodeSpan> = Vec::new();
            for (i, s) in ctx.img.sections.iter().enumerate() {
                if s.flags.contains(SectionFlags::CODE) && !s.data.is_empty() {
                    code_spans.push(CodeSpan {
                        base: s.base,
                        end:  s.base.wrapping_add(s.data.len() as u32),
                        idx:  i,
                    });
                }
            }
            code_spans.sort_by_key(|c| c.base);

            #[inline]
            fn find_code_span(va: u32, spans: &[CodeSpan]) -> Option<CodeSpan> {
                let mut lo = 0usize;
                let mut hi = spans.len();
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    let c   = spans[mid];
                    if va < c.base {
                        hi = mid;
                    } else if va >= c.end {
                        lo = mid + 1;
                    } else {
                        return Some(c);
                    }
                }
                None
            }

            ctx.db.functions.sort_by_key(|f| f.base);

            let old_len = ctx.db.functions.len();
            let len     = old_len;

            for i in 0..len {
                let (cur_slice, rest) = ctx.db.functions.split_at_mut(i + 1);
                let f = &mut cur_slice[i];

                let base = f.base;
                let mut end = base.wrapping_add(f.size);

                // Clamp to section end
                if let Some(span) = find_code_span(base, &code_spans) {
                    if end > span.end {
                        end = span.end;
                    }
                }

                // Clamp to next function base
                if i + 1 < len {
                    let next_base = rest[0].base;
                    if end > next_base {
                        end = next_base;
                    }
                }

                if end <= base {
                    f.size = 0;
                } else {
                    f.size = end.wrapping_sub(base);
                }
            }

            ctx.db.functions.retain(|f| f.size != 0);
            let new_len = ctx.db.functions.len();
            let removed = old_len.saturating_sub(new_len);
            if removed != 0 {
                xdebug!(
                    "SWB: final clamp culled {} zero-sized/overlapping funcs",
                    removed
                );
            }
        }

        // Final invariant check: every switch site + label must lie in some function.
        {
            fn addr_in_functions(funcs: &[crate::db::FunctionInfo], addr: u32) -> bool {
                funcs.iter().any(|f| {
                    let start = f.base;
                    let end   = f.base.wrapping_add(f.size);
                    addr >= start && addr < end
                })
            }

            for sw in &ctx.db.switches {
                let base_ok = addr_in_functions(&ctx.db.functions, sw.base);
                if !base_ok {
                    xlog!(
                        "SWB: INVARIANT FAIL: switch at 0x{:08X} not covered by any function",
                        sw.base
                    );
                }

                if sw.default_label != 0 && !addr_in_functions(&ctx.db.functions, sw.default_label) {
                    xlog!(
                        "SWB: INVARIANT FAIL: default label 0x{:08X} for switch 0x{:08X} not covered",
                        sw.default_label,
                        sw.base
                    );
                }

                for &lbl in &sw.labels {
                    if !addr_in_functions(&ctx.db.functions, lbl) {
                        xlog!(
                            "SWB: INVARIANT FAIL: label 0x{:08X} for switch 0x{:08X} not covered",
                            lbl,
                            sw.base
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
