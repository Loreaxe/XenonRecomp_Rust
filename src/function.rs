// src/function.rs
#![allow(clippy::needless_return)]
#![allow(clippy::too_many_arguments)]

use crate::disasm::PpcCs;

/// Basic block information for a discovered function.
#[derive(Clone, Debug, Default)]
pub struct Block {
    /// Offset (in bytes) from the function base.
    pub base: usize,
    /// Size of this block in bytes.
    pub size: usize,
    /// Projected size (in bytes) for this block, if any.
    /// In the C++ version this was `size_t(-1)` when unset; here we use `None`.
    pub projected_size: Option<usize>,
    /// Debug-only “parent” block base, useful for diagnostics.
    #[cfg(debug_assertions)]
    pub parent: usize,
}

impl Block {
    pub fn new(base: usize, size: usize) -> Self {
        Self {
            base,
            size,
            projected_size: None,
            #[cfg(debug_assertions)]
            parent: 0,
        }
    }

    pub fn new_with_projection(base: usize, size: usize, projected_size: Option<usize>) -> Self {
        Self {
            base,
            size,
            projected_size,
            #[cfg(debug_assertions)]
            parent: 0,
        }
    }
}

/// A function discovered in code, consisting of one or more basic blocks.
#[derive(Clone, Debug, Default)]
pub struct Function {
    /// Absolute VA of the function start.
    pub base: usize,
    /// Total function size in bytes (from base to furthest block end).
    pub size: usize,
    /// Basic blocks (sorted later).
    pub blocks: Vec<Block>,
}

impl Function {
    pub fn new(base: usize, size: usize) -> Self {
        Self {
            base,
            size,
            blocks: Vec::new(),
        }
    }

    /// Search for the index of the block that contains `address`.
    ///
    /// Mirrors the C++ `Function::SearchBlock(size_t address) const`:
    /// - If `address` lies within `[begin, end)` for a block → that block.
    /// - If block is “fresh” (begin == end) and `address == begin` → that block.
    /// - Otherwise returns `None`.
    pub fn search_block(&self, address: usize) -> Option<usize> {
        if address < self.base {
            return None;
        }

        for (i, block) in self.blocks.iter().enumerate() {
            let begin = self.base + block.base;
            let end = begin + block.size;

            if begin != end {
                if address >= begin && address < end {
                    return Some(i);
                }
            } else {
                // Fresh block
                if address == begin {
                    return Some(i);
                }
            }
        }

        None
    }

    // ---- PPC helpers (mirroring the old macros) -----------------------------

    #[inline]
    fn ppc_op(insn: u32) -> u32 {
        (insn >> 26) & 0x3F
    }

    #[inline]
    fn ppc_xop(insn: u32) -> u32 {
        (insn >> 1) & 0x3FF
    }

    /// Link bit (LK) – true if this is a call (BL/BLR/etc.).
    #[inline]
    fn ppc_bl(insn: u32) -> bool {
        (insn & 0x1) != 0
    }

    /// Absolute address bit (AA).
    #[inline]
    fn ppc_ba(insn: u32) -> bool {
        (insn & 0x2) != 0
    }

    /// BO field for branch-on-CTR.
    #[inline]
    fn ppc_bo(insn: u32) -> u32 {
        (insn >> 21) & 0x1F
    }

    /// BD displacement for `bc` (conditional branch), sign-extended and
    /// already multiplied by 4, matching the C++ `PPC_BD()` behaviour.
    #[inline]
    fn ppc_bd(insn: u32) -> isize {
        // Equivalent to:
        // #define PPC_BD(i) ((signed int)((((i) & 0xFFFC) ^ 0x8000) - 0x8000))
        let bd = ((insn >> 2) & 0x3FFF) as i32; // 14 bits
        let bd_signed = if (bd & 0x2000) != 0 {
            bd | !0x3FFF
        } else {
            bd
        };
        (bd_signed << 2) as isize
    }

    /// LI displacement for `b` (unconditional branch), sign-extended and
    /// multiplied by 4, matching the C++ `PPC_BI()` behaviour.
    #[inline]
    fn ppc_bi(insn: u32) -> isize {
        // Equivalent to:
        // #define PPC_BI(i) ((signed int)((((i) & 0x3FFFFFC) ^ 0x2000000) - 0x2000000))
        let li = ((insn >> 2) & 0x00FF_FFFF) as i32; // 24 bits
        let li_signed = if (li & 0x0080_0000) != 0 {
            li | !0x00FF_FFFF
        } else {
            li
        };
        (li_signed << 2) as isize
    }

    /// Port of the C++:
    ///
    ///   Function Function::Analyze(const void* code, size_t size, size_t base)
    ///
    /// In this Rust version:
    /// * `words` is a slice of big-endian PPC instructions already decoded into host
    ///   `u32` (like your `u32::from_be_bytes(..)` pipeline).
    /// * `base` is the starting VA of `words[0]`.
    /// * `_cs` is retained for signature compatibility. We don't use it here because
    ///   the original C++ version also treated undecodable instructions as “just code”
    ///   and advanced by 4 bytes, only logging failures.
    pub fn analyze(words: &[u32], base: usize, _cs: &PpcCs) -> Self {
        let mut fnc = Function {
            base,
            size: 0,
            blocks: Vec::new(),
        };

        if words.is_empty() {
            return fnc;
        }

        // === shifted ptr tail call special case ==============================
        //
        // C++:
        //   if (*((uint32_t*)code + 1) == 0x04000048) {
        //       fn.size = 0x8;
        //       return fn;
        //   }
        //
        // Our `words` are already u32::from_be_bytes(..), so the compare matches.
        if words.len() >= 2 && words[1] == 0x0400_0048 {
            fnc.size = 0x8;
            return fnc;
        }

        // Initial block + block stack.
        fnc.blocks.reserve(8);
        fnc.blocks.push(Block::new(0, 0));

        let mut block_stack: Vec<usize> = Vec::with_capacity(32);
        block_stack.push(0);

        // We work in word indices instead of raw pointers.
        const PPC_OP_BC: u32 = 16;
        const PPC_OP_B: u32 = 18;
        const PPC_OP_CTR: u32 = 19; // xop 16/528 for blr/bctr family

        let mut idx: usize = 0;
        let total_words = words.len();

        // RESTORE_DATA equivalent:
        //   data = dataStart + ((block.base + block.size)/4) - 1
        #[inline]
        fn restore_idx(blocks: &Vec<Block>, stack: &Vec<usize>) -> Option<usize> {
            if let Some(&bi) = stack.last() {
                let b = &blocks[bi];
                let mut new_idx = (b.base + b.size) / 4;
                if new_idx > 0 {
                    new_idx -= 1;
                }
                Some(new_idx)
            } else {
                None
            }
        }

        // Main analysis loop.
        while idx < total_words {
            if block_stack.is_empty() {
                break;
            }

            let addr = base + (idx << 2);

            // Current block index
            let cur_index = *block_stack.last().unwrap();

            {
                let cur_block = &mut fnc.blocks[cur_index];

                debug_assert_eq!(addr, fnc.base + cur_block.base + cur_block.size);

                // projectedSize check (fallthrough cap)
                if let Some(projected) = cur_block.projected_size {
                    if cur_block.size >= projected {
                        // Fallthrough exceeded the projected size: end this block.
                        block_stack.pop();
                        if let Some(new_idx) = restore_idx(&fnc.blocks, &block_stack) {
                            idx = new_idx;
                        } else {
                            // Nothing left to process.
                            break;
                        }
                        idx += 1;
                        continue;
                    }
                }

                // Consume this instruction into the current block.
                cur_block.size += 4;
            }

            let instruction = words[idx];
            let op = Self::ppc_op(instruction);
            let xop = Self::ppc_xop(instruction);
            let is_link = Self::ppc_bl(instruction);

            // ---------------- conditional branch (BC) -------------------------
            if op == PPC_OP_BC {
                if is_link {
                    // Conditional call, nothing special for basic-block carving.
                    idx += 1;
                    continue;
                }

                // True/false branch: terminate current block and create left/right.
                {
                    let cur_block = &mut fnc.blocks[cur_index];
                    cur_block.projected_size = None;
                }

                block_stack.pop();

                debug_assert!(
                    !Self::ppc_ba(instruction),
                    "Function::analyze: absolute BC not expected"
                );

                let branch_dest =
                    (addr as isize).wrapping_add(Self::ppc_bd(instruction)) as usize;

                // false = fallthrough, true = taken
                let l_base = (addr - base) + 4;
                let r_base = branch_dest.saturating_sub(base);

                // left (false) block
                let l_block_idx = if let Some(idx) = fnc.search_block(base + l_base) {
                    idx
                } else {
                    let projected = r_base.checked_sub(l_base);
                    let mut block = Block::new_with_projection(l_base, 0, projected);
                    #[cfg(debug_assertions)]
                    {
                        block.parent = fnc.blocks[cur_index].base;
                    }
                    fnc.blocks.push(block);
                    fnc.blocks.len() - 1
                };

                // push false case first (like original)
                block_stack.push(l_block_idx);

                // right (true) block
                let r_block_idx = if let Some(idx) = fnc.search_block(branch_dest) {
                    idx
                } else {
                    let mut block = Block::new(r_base, 0);
                    #[cfg(debug_assertions)]
                    {
                        block.parent = fnc.blocks[cur_index].base;
                    }
                    fnc.blocks.push(block);
                    fnc.blocks.len() - 1
                };

                // Right case goes on top of the stack (further away).
                block_stack.push(r_block_idx);

                if let Some(new_idx) = restore_idx(&fnc.blocks, &block_stack) {
                    idx = new_idx;
                }
                idx += 1;
                continue;
            }
            // --------------- unconditional branch / return / padding ----------
            else if op == PPC_OP_B
                || instruction == 0
                || (op == PPC_OP_CTR && (xop == 16 || xop == 528))
            {
                if !is_link {
                    // non-link: this really terminates the block
                    let (cur_base, cur_size, cur_proj) = {
                        let b = &fnc.blocks[cur_index];
                        (b.base, b.size, b.projected_size)
                    };

                    block_stack.pop();

                    if op == PPC_OP_B {
                        debug_assert!(
                            !Self::ppc_ba(instruction),
                            "Function::analyze: absolute B not expected"
                        );
                        let branch_dest =
                            (addr as isize).wrapping_add(Self::ppc_bi(instruction)) as usize;
                        let branch_base = branch_dest.saturating_sub(base);

                        if branch_dest < base {
                            // tail call before function base → ignore.
                            if let Some(new_idx) = restore_idx(&fnc.blocks, &block_stack) {
                                idx = new_idx;
                            }
                            idx += 1;
                            continue;
                        }

                        let branch_block = fnc.search_block(branch_dest);

                        // carry over our projection if blocks are next to each other
                        let is_continuous =
                            branch_base == cur_base.saturating_add(cur_size);
                        let size_projection = if let Some(proj) = cur_proj {
                            if is_continuous && proj > cur_size {
                                Some(proj - cur_size)
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if branch_block.is_none() {
                            let mut block =
                                Block::new_with_projection(branch_base, 0, size_projection);
                            #[cfg(debug_assertions)]
                            {
                                block.parent = cur_base;
                            }
                            fnc.blocks.push(block);
                            let new_idx_block = fnc.blocks.len() - 1;
                            block_stack.push(new_idx_block);

                            if let Some(new_idx) = restore_idx(&fnc.blocks, &block_stack) {
                                idx = new_idx;
                            }
                            idx += 1;
                            continue;
                        }
                    } else if op == PPC_OP_CTR {
                        // 5th bit of BO tells CPU to ignore CTR.
                        let conditional = (Self::ppc_bo(instruction) & 0x10) == 0;
                        if conditional {
                            // Right block just returns; left case is fallthrough.
                            let l_base = (addr - base) + 4;
                            let l_block_idx = if let Some(idx) =
                                fnc.search_block(base + l_base)
                            {
                                idx
                            } else {
                                let mut block = Block::new(l_base, 0);
                                #[cfg(debug_assertions)]
                                {
                                    block.parent = cur_base;
                                }
                                fnc.blocks.push(block);
                                fnc.blocks.len() - 1
                            };

                            block_stack.push(l_block_idx);
                            if let Some(new_idx) = restore_idx(&fnc.blocks, &block_stack) {
                                idx = new_idx;
                            }
                            idx += 1;
                            continue;
                        }
                    }

                    if let Some(new_idx) = restore_idx(&fnc.blocks, &block_stack) {
                        idx = new_idx;
                    } else {
                        break;
                    }
                }
            }
            // ---------------- default: normal instruction ---------------------
            //
            // In the original C++ version, invalid opcodes were only logged and
            // the analyzer still advanced by 4 bytes. For XEX/Xenon code this
            // is fine: we simply treat every non-branch, non-zero instruction
            // as valid. Any special skip patterns are handled by the outer
            // AnalyseFunctions pass via cfg.invalid_instructions.
            else {
                // nothing to do; we already extended the current block by 4 bytes
            }

            idx += 1;
        }

        // Sort blocks and cut off at first discontinuity, like the C++ code.
        if fnc.blocks.len() > 1 {
            fnc.blocks.sort_by_key(|b| b.base);

            let mut discontinuity: Option<usize> = None;
            for i in 0..(fnc.blocks.len() - 1) {
                let cur_end = fnc.blocks[i].base + fnc.blocks[i].size;
                if cur_end >= fnc.blocks[i + 1].base {
                    continue;
                }
                discontinuity = Some(i + 1);
                break;
            }

            if let Some(idx) = discontinuity {
                fnc.blocks.truncate(idx);
            }
        }

        // Function size = furthest block end.
        fnc.size = 0;
        for b in &fnc.blocks {
            fnc.size = fnc.size.max(b.base + b.size);
        }

        fnc
    }
}
