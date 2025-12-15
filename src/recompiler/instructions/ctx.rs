// src/recompiler/instructions/ctx.rs

#![allow(clippy::needless_return)]
#![allow(clippy::useless_format)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use capstone::arch::ppc::{PpcOperand, PpcReg, PpcInsn};
use capstone::arch::DetailsArchInsn;
use capstone::{Capstone, Insn, RegId};
use std::fmt::Write as _;

use crate::function::Function;
use crate::recompiler::{CSRState, Recompiler, RecompilerLocalVariables};

/// Convenience to get Insn ID as u32
#[inline]
pub(crate) fn id_u32(i: &Insn<'_>) -> u32 {
    i.id().0
}

/// A lowering context shared by all handlers.
pub(crate) struct LowerCtx<'a> {
    pub rec: &'a mut Recompiler,
    pub fnc: &'a Function,
    pub base: u32,
    pub be_word: u32,
    pub insn: &'a Insn<'a>,
    pub cs: &'a Capstone,
    pub locals: &'a mut RecompilerLocalVariables,
    pub csr: &'a mut CSRState,
    pub mmio: bool,
    pub block_terminated: &'a mut bool,
}

#[inline]
fn normalize_mn(m: &str) -> &str {
    // Strip the optional condition-code '.' suffix: "vcmpneb." -> "vcmpneb"
    m.strip_suffix('.').unwrap_or(m)
}

#[inline]
pub(crate) fn has_mn(ctx: &LowerCtx<'_>, want: &str) -> bool {
    ctx.insn
        .mnemonic()
        .map(|m| normalize_mn(m).eq_ignore_ascii_case(want))
        .unwrap_or(false)
}

#[inline]
pub(crate) fn has_any_mn(ctx: &LowerCtx<'_>, wants: &[&str]) -> bool {
    let Some(m) = ctx.insn.mnemonic() else { return false; };
    let m = normalize_mn(m);
    wants.iter().any(|w| m.eq_ignore_ascii_case(w))
}


impl<'a> LowerCtx<'a> {

        /// Xbox 360 VMX128 alias decoder.
    ///
    /// Many VMX128 loads/stores are encoded using the primary opcode 4 and
    /// the VX form, but Capstone reports them as vector compares
    /// (vcmpequd/vcmpneb./vcmpneh./vcmpnezb./vcmpnezh.).
    ///
    /// This helper decodes the raw 32-bit word and returns:
    ///   (xo, vd_index, ra_index, rb_index)
    /// where:
    ///   - xo is the 10-bit XO field
    ///   - vd_index is a full 0..127 vector index (VMX128: v32+)
    ///   - ra_index / rb_index are 0..31 GPR indices
    #[inline]
    pub fn vmx128_decode(&self) -> Option<(u16, usize, usize, usize)> {
        let w = self.be_word;

        // Primary opcode must be 4 for these VX / VMX128 forms.
        let op = (w >> 26) & 0x3F;
        if op != 4 {
            return None;
        }

        let xo = ((w >> 1) & 0x3FF) as u16;

        // VX fields
        let vd = ((w >> 21) & 0x1F) as usize; // base vector index
        let va = ((w >> 16) & 0x1F) as usize; // we reinterpret as RA (GPR)
        let vb = ((w >> 11) & 0x1F) as usize; // we reinterpret as RB (GPR)

        // VMX128 uses the *upper* vector bank: v32..v63 etc.
        let vd_full = vd + 32;

        Some((xo, vd_full, va, vb))
    }

    /// Raw 32-bit instruction word as stored in the image
    /// (you already have this as `be_word` field).
    #[inline]
    pub fn raw_word(&self) -> u32 {
        self.be_word
    }

    /// Major opcode (top 6 bits).
    #[inline]
    pub fn prim_opcode(&self) -> u32 {
        (self.be_word >> 26) & 0x3f
    }

    /// If this instruction is *one of* the Xenon VMX128 load/store
    /// instructions, return its logical kind. Otherwise `None`.
    #[inline]
    pub fn vmx128_mem_op(&self) -> Option<Vmx128MemOp> {
        classify_vmx128_mem_word(self.be_word)
    }

    #[inline]
    pub fn print(&mut self, s: impl AsRef<str>) {
        self.rec.out.push_str(s.as_ref());
    }

    #[inline]
    pub fn println(&mut self, s: impl AsRef<str>) {
        self.rec.out.push_str(s.as_ref());
        self.rec.out.push('\n');
    }

    #[inline]
    pub fn println_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        self.rec.out.write_fmt(args).unwrap();
        self.rec.out.push('\n');
    }

    #[inline]
    pub fn mark_block_terminated(&mut self) {
        *self.block_terminated = true;
    }

    #[inline]
    pub fn target_in_current_function(&self, target: u32) -> bool {
        self.rec.addr_in_function(self.fnc, target)
    }

    #[inline]
    pub fn resolve_callee_name(&self, addr: u32) -> Option<String> {
        self.rec.resolve_func_name_at(addr)
    }

    #[inline]
    pub fn resolve_extern_name(&self, target: u32) -> Option<String> {
        self.rec
            .resolve_extern_wrapper(target)
            .map(|s| s.to_string())
    }

    #[inline]
    pub fn target_in_any_function(&self, target: u32) -> bool {
        self.rec.addr_in_any_function(target)
    }

    /// Emit a `// addr: BE_WORD  mnemonic (alias) operands` comment.
    pub fn emit_disasm_comment(&mut self) {
        let addr = self.base;
        let word = self.be_word;

        let mut line = String::new();
        line.push_str("\t// ");

        use std::fmt::Write;
        write!(&mut line, "{addr:08X}: {word:08X}  ").unwrap();

        let mn = self.insn.mnemonic().unwrap_or("");
        let ops = self.insn.op_str().unwrap_or("");

        // ---------------------------------------------------------
        // Xenon VMX128 alias pretty-printing for these 5 cases:
        //   vcmpequd  -> (lvx128)
        //   vcmpneb*  -> (lvlx128)
        //   vcmpneh*  -> (lvrx128)
        //   vcmpnezb* -> (stvlx128)
        //   vcmpnezh* -> (stvrx128)
        //
        // Only attach the alias if vmx128_decode() says this is a
        // VX/primary-opcode-4 word (i.e. a real VMX128 pattern).
        // ---------------------------------------------------------
        let mut alias: Option<&'static str> = None;

        if let Some((_xo, _vd, _ra, _rb)) = self.vmx128_decode() {
            let id = crate::recompiler::instructions::ctx::id_u32(self.insn);

            if id == PpcInsn::PPC_INS_VCMPEQUD as u32 || has_mn(self, "vcmpequd") {
                alias = Some("lvx128");
            } else if id == PpcInsn::PPC_INS_VCMPNEB as u32 || has_mn(self, "vcmpneb") {
                alias = Some("lvlx128");
            } else if id == PpcInsn::PPC_INS_VCMPNEH as u32 || has_mn(self, "vcmpneh") {
                alias = Some("lvrx128");
            } else if id == PpcInsn::PPC_INS_VCMPNEZB as u32 || has_mn(self, "vcmpnezb") {
                alias = Some("stvlx128");
            } else if id == PpcInsn::PPC_INS_VCMPNEZH as u32 || has_mn(self, "vcmpnezh") {
                alias = Some("stvrx128");
            }
        }

        // Print mnemonic + optional "(alias)"
        if !mn.is_empty() {
            line.push_str(mn);
            if let Some(a) = alias {
                line.push(' ');
                line.push('(');
                line.push_str(a);
                line.push(')');
            }
        }

        // Print operands if any
        if !ops.is_empty() {
            if !mn.is_empty() {
                line.push(' ');
            }
            line.push_str(ops);
        }

        self.println(line);
    }

    // ----------------------------------------------------------------------------------
    // Capstone operand helpers (correct GPR mapping + MEM base/disp)
    // ----------------------------------------------------------------------------------

    #[inline]
    fn map_ppc_index(reg: RegId) -> Option<usize> {
        let id = reg.0 as u16;
        Some(match id {
            // -------- GPRs r0..r31 --------
            x if x == PpcReg::PPC_REG_R0 as u16 => 0,
            x if x == PpcReg::PPC_REG_R1 as u16 => 1,
            x if x == PpcReg::PPC_REG_R2 as u16 => 2,
            x if x == PpcReg::PPC_REG_R3 as u16 => 3,
            x if x == PpcReg::PPC_REG_R4 as u16 => 4,
            x if x == PpcReg::PPC_REG_R5 as u16 => 5,
            x if x == PpcReg::PPC_REG_R6 as u16 => 6,
            x if x == PpcReg::PPC_REG_R7 as u16 => 7,
            x if x == PpcReg::PPC_REG_R8 as u16 => 8,
            x if x == PpcReg::PPC_REG_R9 as u16 => 9,
            x if x == PpcReg::PPC_REG_R10 as u16 => 10,
            x if x == PpcReg::PPC_REG_R11 as u16 => 11,
            x if x == PpcReg::PPC_REG_R12 as u16 => 12,
            x if x == PpcReg::PPC_REG_R13 as u16 => 13,
            x if x == PpcReg::PPC_REG_R14 as u16 => 14,
            x if x == PpcReg::PPC_REG_R15 as u16 => 15,
            x if x == PpcReg::PPC_REG_R16 as u16 => 16,
            x if x == PpcReg::PPC_REG_R17 as u16 => 17,
            x if x == PpcReg::PPC_REG_R18 as u16 => 18,
            x if x == PpcReg::PPC_REG_R19 as u16 => 19,
            x if x == PpcReg::PPC_REG_R20 as u16 => 20,
            x if x == PpcReg::PPC_REG_R21 as u16 => 21,
            x if x == PpcReg::PPC_REG_R22 as u16 => 22,
            x if x == PpcReg::PPC_REG_R23 as u16 => 23,
            x if x == PpcReg::PPC_REG_R24 as u16 => 24,
            x if x == PpcReg::PPC_REG_R25 as u16 => 25,
            x if x == PpcReg::PPC_REG_R26 as u16 => 26,
            x if x == PpcReg::PPC_REG_R27 as u16 => 27,
            x if x == PpcReg::PPC_REG_R28 as u16 => 28,
            x if x == PpcReg::PPC_REG_R29 as u16 => 29,
            x if x == PpcReg::PPC_REG_R30 as u16 => 30,
            x if x == PpcReg::PPC_REG_R31 as u16 => 31,

            // -------- FPRs f0..f31 --------
            x if x == PpcReg::PPC_REG_F0 as u16 => 0,
            x if x == PpcReg::PPC_REG_F1 as u16 => 1,
            x if x == PpcReg::PPC_REG_F2 as u16 => 2,
            x if x == PpcReg::PPC_REG_F3 as u16 => 3,
            x if x == PpcReg::PPC_REG_F4 as u16 => 4,
            x if x == PpcReg::PPC_REG_F5 as u16 => 5,
            x if x == PpcReg::PPC_REG_F6 as u16 => 6,
            x if x == PpcReg::PPC_REG_F7 as u16 => 7,
            x if x == PpcReg::PPC_REG_F8 as u16 => 8,
            x if x == PpcReg::PPC_REG_F9 as u16 => 9,
            x if x == PpcReg::PPC_REG_F10 as u16 => 10,
            x if x == PpcReg::PPC_REG_F11 as u16 => 11,
            x if x == PpcReg::PPC_REG_F12 as u16 => 12,
            x if x == PpcReg::PPC_REG_F13 as u16 => 13,
            x if x == PpcReg::PPC_REG_F14 as u16 => 14,
            x if x == PpcReg::PPC_REG_F15 as u16 => 15,
            x if x == PpcReg::PPC_REG_F16 as u16 => 16,
            x if x == PpcReg::PPC_REG_F17 as u16 => 17,
            x if x == PpcReg::PPC_REG_F18 as u16 => 18,
            x if x == PpcReg::PPC_REG_F19 as u16 => 19,
            x if x == PpcReg::PPC_REG_F20 as u16 => 20,
            x if x == PpcReg::PPC_REG_F21 as u16 => 21,
            x if x == PpcReg::PPC_REG_F22 as u16 => 22,
            x if x == PpcReg::PPC_REG_F23 as u16 => 23,
            x if x == PpcReg::PPC_REG_F24 as u16 => 24,
            x if x == PpcReg::PPC_REG_F25 as u16 => 25,
            x if x == PpcReg::PPC_REG_F26 as u16 => 26,
            x if x == PpcReg::PPC_REG_F27 as u16 => 27,
            x if x == PpcReg::PPC_REG_F28 as u16 => 28,
            x if x == PpcReg::PPC_REG_F29 as u16 => 29,
            x if x == PpcReg::PPC_REG_F30 as u16 => 30,
            x if x == PpcReg::PPC_REG_F31 as u16 => 31,

            // -------- Vector regs v0..v31 (Altivec-style) --------
            x if x == PpcReg::PPC_REG_V0 as u16 => 0,
            x if x == PpcReg::PPC_REG_V1 as u16 => 1,
            x if x == PpcReg::PPC_REG_V2 as u16 => 2,
            x if x == PpcReg::PPC_REG_V3 as u16 => 3,
            x if x == PpcReg::PPC_REG_V4 as u16 => 4,
            x if x == PpcReg::PPC_REG_V5 as u16 => 5,
            x if x == PpcReg::PPC_REG_V6 as u16 => 6,
            x if x == PpcReg::PPC_REG_V7 as u16 => 7,
            x if x == PpcReg::PPC_REG_V8 as u16 => 8,
            x if x == PpcReg::PPC_REG_V9 as u16 => 9,
            x if x == PpcReg::PPC_REG_V10 as u16 => 10,
            x if x == PpcReg::PPC_REG_V11 as u16 => 11,
            x if x == PpcReg::PPC_REG_V12 as u16 => 12,
            x if x == PpcReg::PPC_REG_V13 as u16 => 13,
            x if x == PpcReg::PPC_REG_V14 as u16 => 14,
            x if x == PpcReg::PPC_REG_V15 as u16 => 15,
            x if x == PpcReg::PPC_REG_V16 as u16 => 16,
            x if x == PpcReg::PPC_REG_V17 as u16 => 17,
            x if x == PpcReg::PPC_REG_V18 as u16 => 18,
            x if x == PpcReg::PPC_REG_V19 as u16 => 19,
            x if x == PpcReg::PPC_REG_V20 as u16 => 20,
            x if x == PpcReg::PPC_REG_V21 as u16 => 21,
            x if x == PpcReg::PPC_REG_V22 as u16 => 22,
            x if x == PpcReg::PPC_REG_V23 as u16 => 23,
            x if x == PpcReg::PPC_REG_V24 as u16 => 24,
            x if x == PpcReg::PPC_REG_V25 as u16 => 25,
            x if x == PpcReg::PPC_REG_V26 as u16 => 26,
            x if x == PpcReg::PPC_REG_V27 as u16 => 27,
            x if x == PpcReg::PPC_REG_V28 as u16 => 28,
            x if x == PpcReg::PPC_REG_V29 as u16 => 29,
            x if x == PpcReg::PPC_REG_V30 as u16 => 30,
            x if x == PpcReg::PPC_REG_V31 as u16 => 31,

            // -------- CR fields cr0..cr7 --------
            x if x == PpcReg::PPC_REG_CR0 as u16 => 0,
            x if x == PpcReg::PPC_REG_CR1 as u16 => 1,
            x if x == PpcReg::PPC_REG_CR2 as u16 => 2,
            x if x == PpcReg::PPC_REG_CR3 as u16 => 3,
            x if x == PpcReg::PPC_REG_CR4 as u16 => 4,
            x if x == PpcReg::PPC_REG_CR5 as u16 => 5,
            x if x == PpcReg::PPC_REG_CR6 as u16 => 6,
            x if x == PpcReg::PPC_REG_CR7 as u16 => 7,

            _ => return None,
        })
    }

    /// Returns a **GPR index 0..31** for Reg operands; 0 otherwise.
    #[inline]
    pub fn op_reg(&self, i: usize) -> usize {
        let detail = match self.cs.insn_detail(self.insn) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        let arch = detail.arch_detail();
        let ppc = match arch.ppc() {
            Some(p) => p,
            None => return 0,
        };
        let Some(op) = ppc.operands().nth(i) else { return 0; };
        match op {
            PpcOperand::Reg(rid) => Self::map_ppc_index(rid).unwrap_or(0),
            _ => 0,
        }
    }

    #[inline]
    pub fn op_imm(&self, i: usize) -> i64 {
        let detail = match self.cs.insn_detail(self.insn) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        let arch = detail.arch_detail();
        let ppc = match arch.ppc() {
            Some(p) => p,
            None => return 0,
        };
        let Some(op) = ppc.operands().nth(i) else { return 0; };
        match op {
            PpcOperand::Imm(v) => v,
            _ => 0,
        }
    }

    /// nth GPR operand (0 = first GPR), or 0 if none (falls back to r0).
    #[inline]
    pub fn op_gpr(&self, nth: usize) -> usize {
        use capstone::arch::ppc::PpcOperand;

        let detail = match self.cs.insn_detail(self.insn) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        let arch = detail.arch_detail();
        let ppc = match arch.ppc() {
            Some(p) => p,
            None => return 0,
        };

        let mut count = 0usize;
        for op in ppc.operands() {
            if let PpcOperand::Reg(rid) = op {
                let id = rid.0 as u16;

                // Only GPRs r0..r31
                if id >= capstone::arch::ppc::PpcReg::PPC_REG_R0 as u16
                    && id <= capstone::arch::ppc::PpcReg::PPC_REG_R31 as u16
                {
                    if count == nth {
                        // map to 0..31 like ctx.r0..ctx.r31
                        return (id - capstone::arch::ppc::PpcReg::PPC_REG_R0 as u16) as usize;
                    }
                    count += 1;
                }
            }
        }

        // Fallback to r0 if we didn't find enough GPRs
        0
    }

    /// nth immediate operand (0 = first IMM), or 0 if none.
    #[inline]
    pub fn op_imm_n(&self, nth: usize) -> i64 {
        use capstone::arch::ppc::PpcOperand;

        let detail = match self.cs.insn_detail(self.insn) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        let arch = detail.arch_detail();
        let ppc = match arch.ppc() {
            Some(p) => p,
            None => return 0,
        };

        let mut count = 0usize;
        for op in ppc.operands() {
            if let PpcOperand::Imm(v) = op {
                if count == nth {
                    return v;
                }
                count += 1;
            }
        }
        0
    }

    /// Parse MEM operand `i`: returns (base_reg_index, disp).
    #[inline]
    pub fn op_mem(&self, i: usize) -> (Option<usize>, i64) {
        let detail = match self.cs.insn_detail(self.insn) {
            Ok(d) => d,
            Err(_) => return (None, 0),
        };
        let arch = detail.arch_detail();
        let ppc = match arch.ppc() {
            Some(p) => p,
            None => return (None, 0),
        };
        let Some(op) = ppc.operands().nth(i) else { return (None, 0); };
        let PpcOperand::Mem(mem) = op else { return (None, 0); };

        let base = Self::map_ppc_index(mem.base());
        let disp = mem.disp() as i64;
        (base, disp)
    }

    /// First immediate operand interpreted as a branch target (Capstone-style).
    #[inline]
    pub fn branch_target(&self) -> u32 {
        let detail = match self.cs.insn_detail(self.insn) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        let arch = detail.arch_detail();
        let ppc = match arch.ppc() {
            Some(p) => p,
            None => return 0,
        };

        for op in ppc.operands() {
            if let PpcOperand::Imm(v) = op {
                return v as u32;
            }
        }
        0
    }

    /// Try to find a CRx operand; if none is present, default to CR0.
    #[inline]
    pub(crate) fn branch_cr_index(&self) -> usize {
        let detail = match self.cs.insn_detail(self.insn) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        let arch = detail.arch_detail();
        let ppc = match arch.ppc() {
            Some(p) => p,
            None => return 0,
        };

        for op in ppc.operands() {
            if let PpcOperand::Reg(rid) = op {
                let id = rid.0 as u16;
                if id == PpcReg::PPC_REG_CR0 as u16
                    || id == PpcReg::PPC_REG_CR1 as u16
                    || id == PpcReg::PPC_REG_CR2 as u16
                    || id == PpcReg::PPC_REG_CR3 as u16
                    || id == PpcReg::PPC_REG_CR4 as u16
                    || id == PpcReg::PPC_REG_CR5 as u16
                    || id == PpcReg::PPC_REG_CR6 as u16
                    || id == PpcReg::PPC_REG_CR7 as u16
                {
                    return Self::map_ppc_index(rid).unwrap_or(0);
                }
            }
        }

        0
    }

    /// Build an EA expression string using your runtime `(base, ea)` API.
    #[inline]
    pub fn ea_expr_u32(&mut self, base_reg: Option<usize>, disp: i64) -> String {
        let d = disp as i64;
        match base_reg {
            Some(r) => format!(
                "({}.u32 as u32).wrapping_add({:#x})",
                self.r(r),
                d as u32
            ),
            None => format!("{:#x}", d as u32),
        }
    }

    // -------- Register printers (index form for Rust) --------
    fn ctx_gpr_name(idx: usize) -> &'static str {
        match idx {
            0  => "ctx.r0",
            1  => "ctx.r1",
            2  => "ctx.r2",
            3  => "ctx.r3",
            4  => "ctx.r4",
            5  => "ctx.r5",
            6  => "ctx.r6",
            7  => "ctx.r7",
            8  => "ctx.r8",
            9  => "ctx.r9",
            10 => "ctx.r10",
            11 => "ctx.r11",
            12 => "ctx.r12",
            13 => "ctx.r13",
            14 => "ctx.r14",
            15 => "ctx.r15",
            16 => "ctx.r16",
            17 => "ctx.r17",
            18 => "ctx.r18",
            19 => "ctx.r19",
            20 => "ctx.r20",
            21 => "ctx.r21",
            22 => "ctx.r22",
            23 => "ctx.r23",
            24 => "ctx.r24",
            25 => "ctx.r25",
            26 => "ctx.r26",
            27 => "ctx.r27",
            28 => "ctx.r28",
            29 => "ctx.r29",
            30 => "ctx.r30",
            31 => "ctx.r31",
            _ => panic!("GPR index out of range: {}", idx),
        }
    }

    pub fn r(&mut self, idx: usize) -> String {
        let use_local = (cfg!(feature = "non_argument_as_local") && matches!(idx, 0 | 2 | 11 | 12))
            || (cfg!(feature = "non_volatile_as_local") && idx >= 14);

        if use_local {
            self.locals.r[idx] = true;
            format!("r[{idx}]")
        } else {
            Self::ctx_gpr_name(idx).to_string()
        }
    }

    fn ctx_fpr_name(idx: usize) -> &'static str {
        match idx {
            0  => "ctx.f0",
            1  => "ctx.f1",
            2  => "ctx.f2",
            3  => "ctx.f3",
            4  => "ctx.f4",
            5  => "ctx.f5",
            6  => "ctx.f6",
            7  => "ctx.f7",
            8  => "ctx.f8",
            9  => "ctx.f9",
            10 => "ctx.f10",
            11 => "ctx.f11",
            12 => "ctx.f12",
            13 => "ctx.f13",
            14 => "ctx.f14",
            15 => "ctx.f15",
            16 => "ctx.f16",
            17 => "ctx.f17",
            18 => "ctx.f18",
            19 => "ctx.f19",
            20 => "ctx.f20",
            21 => "ctx.f21",
            22 => "ctx.f22",
            23 => "ctx.f23",
            24 => "ctx.f24",
            25 => "ctx.f25",
            26 => "ctx.f26",
            27 => "ctx.f27",
            28 => "ctx.f28",
            29 => "ctx.f29",
            30 => "ctx.f30",
            31 => "ctx.f31",
            _ => panic!("FPR index out of range: {}", idx),
        }
    }

    pub fn f(&mut self, idx: usize) -> String {
        let use_local = (cfg!(feature = "non_argument_as_local") && idx == 0)
            || (cfg!(feature = "non_volatile_as_local") && idx >= 14);

        if use_local {
            self.locals.f[idx] = true;
            format!("f[{idx}]")
        } else {
            Self::ctx_fpr_name(idx).to_string()
        }
    }

    fn ctx_vr_name(idx: usize) -> &'static str {
        match idx {
             0 => "ctx.v0",
             1 => "ctx.v1",
             2 => "ctx.v2",
             3 => "ctx.v3",
             4 => "ctx.v4",
             5 => "ctx.v5",
             6 => "ctx.v6",
             7 => "ctx.v7",
             8 => "ctx.v8",
             9 => "ctx.v9",
            10 => "ctx.v10",
            11 => "ctx.v11",
            12 => "ctx.v12",
            13 => "ctx.v13",
            14 => "ctx.v14",
            15 => "ctx.v15",
            16 => "ctx.v16",
            17 => "ctx.v17",
            18 => "ctx.v18",
            19 => "ctx.v19",
            20 => "ctx.v20",
            21 => "ctx.v21",
            22 => "ctx.v22",
            23 => "ctx.v23",
            24 => "ctx.v24",
            25 => "ctx.v25",
            26 => "ctx.v26",
            27 => "ctx.v27",
            28 => "ctx.v28",
            29 => "ctx.v29",
            30 => "ctx.v30",
            31 => "ctx.v31",
            32 => "ctx.v32",
            33 => "ctx.v33",
            34 => "ctx.v34",
            35 => "ctx.v35",
            36 => "ctx.v36",
            37 => "ctx.v37",
            38 => "ctx.v38",
            39 => "ctx.v39",
            40 => "ctx.v40",
            41 => "ctx.v41",
            42 => "ctx.v42",
            43 => "ctx.v43",
            44 => "ctx.v44",
            45 => "ctx.v45",
            46 => "ctx.v46",
            47 => "ctx.v47",
            48 => "ctx.v48",
            49 => "ctx.v49",
            50 => "ctx.v50",
            51 => "ctx.v51",
            52 => "ctx.v52",
            53 => "ctx.v53",
            54 => "ctx.v54",
            55 => "ctx.v55",
            56 => "ctx.v56",
            57 => "ctx.v57",
            58 => "ctx.v58",
            59 => "ctx.v59",
            60 => "ctx.v60",
            61 => "ctx.v61",
            62 => "ctx.v62",
            63 => "ctx.v63",
            64 => "ctx.v64",
            65 => "ctx.v65",
            66 => "ctx.v66",
            67 => "ctx.v67",
            68 => "ctx.v68",
            69 => "ctx.v69",
            70 => "ctx.v70",
            71 => "ctx.v71",
            72 => "ctx.v72",
            73 => "ctx.v73",
            74 => "ctx.v74",
            75 => "ctx.v75",
            76 => "ctx.v76",
            77 => "ctx.v77",
            78 => "ctx.v78",
            79 => "ctx.v79",
            80 => "ctx.v80",
            81 => "ctx.v81",
            82 => "ctx.v82",
            83 => "ctx.v83",
            84 => "ctx.v84",
            85 => "ctx.v85",
            86 => "ctx.v86",
            87 => "ctx.v87",
            88 => "ctx.v88",
            89 => "ctx.v89",
            90 => "ctx.v90",
            91 => "ctx.v91",
            92 => "ctx.v92",
            93 => "ctx.v93",
            94 => "ctx.v94",
            95 => "ctx.v95",
            96 => "ctx.v96",
            97 => "ctx.v97",
            98 => "ctx.v98",
            99 => "ctx.v99",
           100 => "ctx.v100",
           101 => "ctx.v101",
           102 => "ctx.v102",
           103 => "ctx.v103",
           104 => "ctx.v104",
           105 => "ctx.v105",
           106 => "ctx.v106",
           107 => "ctx.v107",
           108 => "ctx.v108",
           109 => "ctx.v109",
           110 => "ctx.v110",
           111 => "ctx.v111",
           112 => "ctx.v112",
           113 => "ctx.v113",
           114 => "ctx.v114",
           115 => "ctx.v115",
           116 => "ctx.v116",
           117 => "ctx.v117",
           118 => "ctx.v118",
           119 => "ctx.v119",
           120 => "ctx.v120",
           121 => "ctx.v121",
           122 => "ctx.v122",
           123 => "ctx.v123",
           124 => "ctx.v124",
           125 => "ctx.v125",
           126 => "ctx.v126",
           127 => "ctx.v127",
           _ => panic!("VR index out of range: {}", idx),
        }
    }

    pub fn v(&mut self, idx: usize) -> String {
        let non_arg = (32..=63).contains(&idx);
        let nv = (14..=31).contains(&idx) || (64..=127).contains(&idx);
        let use_local = (cfg!(feature = "non_argument_as_local") && non_arg)
            || (cfg!(feature = "non_volatile_as_local") && nv);

        if use_local {
            self.locals.v[idx] = true;
            format!("v[{idx}]")
        } else {
            Self::ctx_vr_name(idx).to_string()
        }
    }

    fn ctx_cr_name(idx: usize) -> &'static str {
        match idx {
            0 => "ctx.cr0",
            1 => "ctx.cr1",
            2 => "ctx.cr2",
            3 => "ctx.cr3",
            4 => "ctx.cr4",
            5 => "ctx.cr5",
            6 => "ctx.cr6",
            7 => "ctx.cr7",
            _ => panic!("CR index out of range: {}", idx),
        }
    }

    pub fn cr(&mut self, idx: usize) -> String {
        if cfg!(feature = "cr_as_local") {
            self.locals.cr[idx] = true;
            format!("cr[{idx}]")
        } else {
            Self::ctx_cr_name(idx).to_string()
        }
    }

    pub fn ctr(&mut self) -> String {
        if cfg!(feature = "ctr_as_local") {
            self.locals.ctr = true;
            "ctr".into()
        } else {
            "ctx.ctr".into()
        }
    }

    pub fn xer(&mut self) -> String {
        if cfg!(feature = "xer_as_local") {
            self.locals.xer = true;
            "xer".into()
        } else {
            "ctx.xer".into()
        }
    }

    pub fn reserved(&mut self) -> String {
        if cfg!(feature = "reserved_as_local") {
            self.locals.reserved = true;
            "reserved".into()
        } else {
            "ctx.reserved".into()
        }
    }

    pub fn temp(&mut self) -> String {
        self.locals.temp = true;
        "tmp".into()
    }

    pub fn vtemp(&mut self) -> String {
        self.locals.vtemp = true;
        "vtmp".into()
    }

    pub fn env(&mut self) -> String {
        self.locals.env = true;
        "env".into()
    }

    pub fn ea(&mut self) -> String {
        self.locals.ea = true;
        "ea".into()
    }

    // -------- Control-flow emission helpers (Rust style) --------
    pub fn goto(&mut self, target: u32) {
        self.println_fmt(format_args!(
            "\tpc = 0x{target:08X}; continue 'dispatch;"
        ));
    }

    pub fn call_external(&mut self, addr: u32) {
        self.println_fmt(format_args!(
            "\tcrate::rt::call(ctx, base, 0x{addr:08X});"
        ));
    }

    pub fn print_function_call(&mut self, addr: u32) {
        let cfg = &self.rec.config;

        // --- special longjmp / setjmp handling stays as-is ---
        if addr == cfg.longJmpAddress {
            self.println("\t// TODO: longjmp model");
            self.println("\tunsafe { core::intrinsics::abort(); }");
            return;
        }

        if addr == cfg.setJmpAddress {
            let env = self.env();
            let t   = self.temp();
            let r3  = self.r(3);
            self.println_fmt(format_args!("\t{env} = ctx.clone();"));
            self.println_fmt(format_args!(
                "\t{t}.s64 = 0; // TODO setjmp(&mut {env}, {r3}/*env addr*/);"
            ));
            self.println_fmt(format_args!(
                "\tif {t}.s64 != 0 {{ ctx = {env}.clone(); }}"
            ));
            self.println_fmt(format_args!("\t{r3} = {t};"));
            return;
        }

        // 1) Prefer extern wrappers (xam/xboxkrnl shims etc.)
        if let Some(ext) = self.resolve_extern_name(addr) {
            self.println_fmt(format_args!("\t{ext}(ctx, base);"));
            return;
        }

        // 2) In multi-crate mode, only direct-call if we KNOW the callee
        //    lives in the same chunk crate as the caller. Otherwise,
        //    fall back to the external dispatcher.
        if self.target_in_any_function(addr) {
            if let Some(name) = self.resolve_callee_name(addr) {
                let caller_base = self.fnc.base as u32;

                if let (Some(caller_chunk), Some(callee_chunk)) = (
                    self.rec.chunk_crate_for_func(caller_base),
                    self.rec.chunk_crate_for_func(addr),
                ) {
                    if caller_chunk == callee_chunk {
                        // Same chunk crate => direct Rust call is valid
                        self.println_fmt(format_args!(
                            "\tcrate::{name}(ctx, base, 0x{addr:08X});"
                        ));
                        return;
                    }
                }
            }
        }

        // 3) Fallback: unknown or cross-crate target => go through the
        //    address-based external dispatcher (host/runtime decides).
        self.call_external(addr);
    }

    pub fn print_conditional_branch(&mut self, invert: bool, cond_field: &str, target: u32) {
        let cr_idx = self.branch_cr_index();
        let cond_bit = {
            let cr = self.cr(cr_idx);
            format!("{cr}.{cond_field}")
        };

        // CR bits are u8 (0/1). We interpret:
        //   - non-inverted: branch if bit != 0
        //   - inverted:     branch if bit == 0
        let cmp_expr = if invert {
            format!("{cond_bit} == 0")
        } else {
            format!("{cond_bit} != 0")
        };

        // if (condition) { ... }
        self.println_fmt(format_args!("\tif {cmp} {{", cmp = cmp_expr));

        // 1) Intra-function branch → just a goto
        if self.target_in_current_function(target) {
            self.goto(target);
            self.println("\t}");
            return;
        }

        // 2) Cross-function / external:
        //    delegate to print_function_call(), which already:
        //      - handles longjmp/setjmp
        //      - prefers extern wrappers
        //      - only direct-calls same-chunk functions
        //      - falls back to rt::call(ctx, base, addr)
        self.print_function_call(target);
        self.println("\treturn;");
        self.println("\t}");
    }

    pub fn set_flush_mode(&mut self, enable: bool) {
        let new_state = if enable {
            CSRState::VMX
        } else {
            CSRState::FPU
        };
        if *self.csr != new_state {
            if enable {
                self.println("\tctx.fpscr.enable_flush_mode_unconditional();");
            } else {
                self.println("\tctx.fpscr.disable_flush_mode_unconditional();");
            }
            *self.csr = new_state;
        }
    }

    pub fn emit_mid_asm_hook_if_any(&mut self, after_instruction: bool) {
        let copied = if let Some(h) = self.rec.config.mid_asm_hooks.get(&self.base) {
            if h.afterInst != after_instruction {
                return;
            }
            Some((
                h.name.clone(),
                h.registers.clone(),
                h.ret,
                h.returnOnTrue,
                h.returnOnFalse,
                h.jumpAddressOnTrue,
                h.jumpAddressOnFalse,
            ))
        } else {
            None
        };

        let Some((name, registers, ret, ret_on_true, ret_on_false, jmp_true, jmp_false)) =
            copied
        else {
            return;
        };

        let returns_bool =
            ret_on_false || ret_on_true || jmp_false != 0 || jmp_true != 0;

        let mut call = String::new();
        write!(&mut call, "{}(", name).unwrap();
        let mut first = true;
        for reg in &registers {
            if !first {
                call.push_str(", ");
            } else {
                first = false;
            }
            let s = reg.as_str();
            if s == "ctr" {
                call.push_str(&self.ctr());
            } else if s == "reserved" {
                call.push_str(&self.reserved());
            } else if s == "fpscr" {
                call.push_str("ctx.fpscr");
            } else if s.starts_with("cr") {
                let idx: usize = s[2..].parse().unwrap_or(0);
                call.push_str(&self.cr(idx));
            } else if s.starts_with('r') {
                let idx: usize = s[1..].parse().unwrap_or(0);
                call.push_str(&self.r(idx));
            } else if s.starts_with('f') {
                let idx: usize = s[1..].parse().unwrap_or(0);
                call.push_str(&self.f(idx));
            } else if s.starts_with('v') {
                let idx: usize = s[1..].parse().unwrap_or(0);
                call.push_str(&self.v(idx));
            } else {
                call.push_str("/*unsupported-reg*/");
            }
        }
        call.push(')');

        if returns_bool {
            self.println_fmt(format_args!("\tif {call} {{"));
            if ret_on_true {
                self.println("\t\treturn;");
            } else if jmp_true != 0 {
                self.goto(jmp_true);
            }
            self.println("\t} else {");
            if ret_on_false {
                self.println("\t\treturn;");
            } else if jmp_false != 0 {
                self.goto(jmp_false);
            }
            self.println("\t}");
        } else {
            self.println_fmt(format_args!("\t{call};"));
            if ret {
                self.println("\treturn;");
            }
        }
    }

    pub fn is_mmio_store(&self, next_be_word: u32) -> bool {
        next_be_word == Recompiler::C_EIEIO
    }
}

/// VMX128 (Xenon) vector load/store opcodes we care about.
/// These are *not* visible as distinct Capstone IDs; Capstone
/// currently mislabels some of them as vcmpequd / vcmpne*.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Vmx128MemOp {
    Lvsl128,
    Lvsr128,
    Lvewx128,
    Lvx128,
    Stvewx128,
    Stvx128,
    Lvxl128,
    Stvxl128,
    Lvlx128,
    Lvrx128,
    Stvlx128,
    Stvrx128,
    Lvlxl128,
    Lvrxl128,
    Stvlxl128,
    Stvrxl128,
}

// ---- VMX128 opcode pattern helpers (mirroring binutils ppc-opc.c) ----

const fn op_field(x: u32) -> u32 {
    // OP(x) from ppc-opc.c: (((x) & 0x3f) << 26)
    (x & 0x3f) << 26
}

const fn vx_mask(xop_mask: u32) -> u32 {
    // VX(op, xop_mask) with op=0x3f for mask building:
    // OP(0x3f) | (((xop_mask) & 0x3ff) << 1)
    op_field(0x3f) | ((xop_mask & 0x3ff) << 1)
}

const fn vx128_1_pattern(op: u32, xop: u32) -> u32 {
    // VX128_1(op, xop): OP(op) | (xop & 0x7f3)
    op_field(op) | (xop & 0x7f3)
}

// Common mask for all VX128_1 instructions.
const VX128_1_MASK: u32 = vx_mask(0x7f3);

// Concrete VMX128 op patterns (using the xop values from binutils)
const LVSL128_OP:   u32 = vx128_1_pattern(4,    3);
const LVSR128_OP:   u32 = vx128_1_pattern(4,   67);
const LVEWX128_OP:  u32 = vx128_1_pattern(4,  131);
const LVX128_OP:    u32 = vx128_1_pattern(4,  195);
const STVEWX128_OP: u32 = vx128_1_pattern(4,  387);
const STVX128_OP:   u32 = vx128_1_pattern(4,  451);
const LVXL128_OP:   u32 = vx128_1_pattern(4,  707);
const STVXL128_OP:  u32 = vx128_1_pattern(4,  963);
const LVLX128_OP:   u32 = vx128_1_pattern(4, 1027);
const LVRX128_OP:   u32 = vx128_1_pattern(4, 1091);
const STVLX128_OP:  u32 = vx128_1_pattern(4, 1283);
const STVRX128_OP:  u32 = vx128_1_pattern(4, 1347);
const LVLXL128_OP:  u32 = vx128_1_pattern(4, 1539);
const LVRXL128_OP:  u32 = vx128_1_pattern(4, 1603);
const STVLXL128_OP: u32 = vx128_1_pattern(4, 1795);
const STVRXL128_OP: u32 = vx128_1_pattern(4, 1859);

#[inline]
fn classify_vmx128_mem_word(word: u32) -> Option<Vmx128MemOp> {
    let sig = word & VX128_1_MASK;

    Some(match sig {
        LVSL128_OP   => Vmx128MemOp::Lvsl128,
        LVSR128_OP   => Vmx128MemOp::Lvsr128,
        LVEWX128_OP  => Vmx128MemOp::Lvewx128,
        LVX128_OP    => Vmx128MemOp::Lvx128,
        STVEWX128_OP => Vmx128MemOp::Stvewx128,
        STVX128_OP   => Vmx128MemOp::Stvx128,
        LVXL128_OP   => Vmx128MemOp::Lvxl128,
        STVXL128_OP  => Vmx128MemOp::Stvxl128,
        LVLX128_OP   => Vmx128MemOp::Lvlx128,
        LVRX128_OP   => Vmx128MemOp::Lvrx128,
        STVLX128_OP  => Vmx128MemOp::Stvlx128,
        STVRX128_OP  => Vmx128MemOp::Stvrx128,
        LVLXL128_OP  => Vmx128MemOp::Lvlxl128,
        LVRXL128_OP  => Vmx128MemOp::Lvrxl128,
        STVLXL128_OP => Vmx128MemOp::Stvlxl128,
        STVRXL128_OP => Vmx128MemOp::Stvrxl128,
        _ => return None,
    })
}

pub const fn compute_mask64(mut mstart: u32, mut mstop: u32) -> u64 {
    mstart &= 0x3F;
    mstop &= 0x3F;

    let left = u64::MAX >> mstart;
    let right = if mstop >= 63 {
        0
    } else {
        u64::MAX >> (mstop + 1)
    };

    let value = left ^ right;
    if mstart <= mstop {
        value
    } else {
        !value
    }
}

// 32-bit RLWINM / RLWIMI / RLWNM mask helper.
// MB/ME are in the usual 0..31 range.
pub const fn compute_mask32(mb: u32, me: u32) -> u32 {
    let all: u32 = !0;
    let mb = mb & 31;
    let me = me & 31;

    if mb <= me {
        // Bits mb..me set (MSR-style numbering but this combination
        // matches the RLWINM examples from the IBM docs when used
        // with a standard u32.rotate_left).
        let left = all >> mb;
        let right = if me == 31 { 0 } else { all >> (me + 1) };
        left ^ right
    } else {
        // Wrap-around case: mb..31 and 0..me
        let t = (all >> (me + 1)) ^ (all >> mb);
        !t
    }
}

#[allow(dead_code)]
pub(crate) fn handle_link_if_needed(ctx: &mut LowerCtx<'_>) {
    if !ctx.rec.config.skip_lr {
        ctx.println_fmt(format_args!("\tctx.lr = 0x{:X};", ctx.base + 4));
    }
}
