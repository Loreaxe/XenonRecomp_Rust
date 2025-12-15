// src/ppc.rs
#![allow(dead_code)]

#[inline]
pub fn bswap32(x: u32) -> u32 { x.swap_bytes() }

/// Host-endian raw PPC word + common field helpers.
#[derive(Copy, Clone)]
pub struct PpcRaw(u32); // host-endian raw word

impl PpcRaw {
    /// Construct from a big-endian on-disk word.
    #[inline]
    pub fn from_be_word(be: u32) -> Self { Self(u32::from_be(be)) }

    /// Construct from a host-endian word directly.
    #[inline]
    pub fn from_host_word(host: u32) -> Self { Self(host) }

    #[inline]
    pub fn word(self) -> u32 { self.0 }

    // ---- basic fields ----
    #[inline]
    pub fn op(self) -> u32  { (self.0 >> 26) & 0x3f }  // bits 26..31
    #[inline]
    pub fn xop(self) -> u32 { (self.0 >> 1) & 0x3ff }  // XO in many forms
    #[inline]
    pub fn bo(self) -> u32  { (self.0 >> 21) & 0x1f }
    #[inline]
    pub fn aa(self) -> bool { ((self.0 >> 1) & 1) != 0 }   // AA (abs)
    #[inline]
    pub fn lk(self) -> bool { (self.0 & 1) != 0 }          // LK

    #[inline]
    pub fn simm16(self) -> i32 { ((self.0 as i32) << 16) >> 16 }   // signed low16
    #[inline]
    pub fn uimm16(self) -> u32 { self.0 & 0xffff }

    // ---- branch displacements ----

    /// BC-form BD: 14-bit signed displacement (bits 16..29), scaled by 4.
    /// Sign-extend after the <<2 (i.e., treat as 16-bit signed).
    #[inline]
    pub fn bd(self) -> i32 {
        let bd14 = ((self.0 >> 2) & 0x3FFF) as i32;   // 14 bits
        let disp16 = (bd14 << 2) & 0xFFFF;            // (BD<<2) masked to 16 bits
        (disp16 as i16) as i32                         // sign-extend 16 -> 32
    }

    /// B/BL LI field: 24 bits (bits 6..29) scaled by 4.
    /// If AA=0, LI<<2 is a 26-bit signed displacement relative to PC.
    /// If AA=1, LI<<2 is absolute.
    #[inline]
    pub fn branch_target(self, pc: u32) -> u32 {
        let li = (self.0 >> 2) & 0x00FF_FFFF; // 24-bit
        if self.aa() {
            (li << 2) & 0xFFFF_FFFC
        } else {
            let disp = (((li << 8) as i32) >> 6) as i32; // sign-extended (LI<<2)
            pc.wrapping_add(disp as u32)
        }
    }

    // ---- helpers for LIS/ADDI rebuilding ----

    /// Rebuild a 32-bit constant from `lis` (upper) and `addi` (lower).
    ///   lis rX, imm16         => upper = imm16 << 16
    ///   addi rX, rX|r0, imm16 => lower adds sign-extended imm16
    #[inline]
    pub fn compose_lis_addi(upper: Self, lower: Self) -> u32 {
        let hi = (upper.uimm16() as u32) << 16;
        let lo = lower.simm16() as i32 as u32;
        hi.wrapping_add(lo)
    }

    // Primary opcodes we care about (matches C++ semantics)
    pub const OP_BC:  u32 = 16; // conditional branch
    pub const OP_B:   u32 = 18; // unconditional branch
    pub const OP_XL:  u32 = 19; // bclr/bcctr/etc (XL-form)

    // XO values for bclr / bcctr (what C++ checked with PPC_OP_CTR + xop test)
    pub const XO_BCLR:  u32 = 16;
    pub const XO_BCCTR: u32 = 528;

    #[inline]
    pub fn is_bc(self) -> bool {
        self.op() == Self::OP_BC
    }

    #[inline]
    pub fn is_b_or_bl(self) -> bool {
        self.op() == Self::OP_B
    }

    /// C++: `op == PPC_OP_CTR && (xop == 16 || xop == 528)`
    #[inline]
    pub fn is_ctr_form(self) -> bool {
        self.op() == Self::OP_XL && (self.xop() == Self::XO_BCLR || self.xop() == Self::XO_BCCTR)
    }

    /// C++: `PPC_BL(instruction)` (is it a *call*?)
    #[inline]
    pub fn is_link(self) -> bool {
        self.lk()
    }

    /// C++: `PPC_BO(instruction) & 0x10` (“ignore CTR” bit).
    /// If this bit is *clear*, the branch is conditional on CTR.
    #[inline]
    pub fn bo_ignores_ctr(self) -> bool {
        (self.bo() & 0x10) != 0
    }

    /// C++: `PPC_BA(instruction)` – AA bit (absolute branch)
    #[inline]
    pub fn is_absolute(self) -> bool {
        self.aa()
    }

    /// BC-form branch destination, matching C++: `addr + PPC_BD(instruction)`
    #[inline]
    pub fn bc_target(self, pc: u32) -> u32 {
        pc.wrapping_add(self.bd() as u32)
    }

    /// B/BL destination as in C++: `addr + PPC_BI(instruction)` for AA == 0,
    /// and “absolute” when AA == 1.
    #[inline]
    pub fn b_target(self, pc: u32) -> u32 {
        self.branch_target(pc)
    }

}
