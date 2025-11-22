// src/recompiler/ppc_context.rs

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![cfg_attr(any(target_arch = "x86_64", target_arch = "aarch64"), allow(unused_unsafe))]

use core::mem::MaybeUninit;

pub const PPC_MEMORY_SIZE: u64 = 0x1_0000_0000;

// ---------- Function indirection plumbing (pure Rust) ----------

pub type FuncPtr = fn(&mut PPCContext, *mut u8);

// ---------- Context ----------

#[repr(C)]
pub struct PPCContext {
    // ---------- GPRs ----------
    pub r3: PPCRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub r0: PPCRegister,
    pub r1: PPCRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub r2: PPCRegister,
    pub r4: PPCRegister,
    pub r5: PPCRegister,
    pub r6: PPCRegister,
    pub r7: PPCRegister,
    pub r8: PPCRegister,
    pub r9: PPCRegister,
    pub r10: PPCRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub r11: PPCRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub r12: PPCRegister,
    pub r13: PPCRegister,

    // (non_volatile_as_local) removed → always present r14..r31
    pub r14: PPCRegister,
    pub r15: PPCRegister,
    pub r16: PPCRegister,
    pub r17: PPCRegister,
    pub r18: PPCRegister,
    pub r19: PPCRegister,
    pub r20: PPCRegister,
    pub r21: PPCRegister,
    pub r22: PPCRegister,
    pub r23: PPCRegister,
    pub r24: PPCRegister,
    pub r25: PPCRegister,
    pub r26: PPCRegister,
    pub r27: PPCRegister,
    pub r28: PPCRegister,
    pub r29: PPCRegister,
    pub r30: PPCRegister,
    pub r31: PPCRegister,

    #[cfg(not(feature = "skip_lr"))]
    pub lr: u64,

    // (ctr_as_local/xer_as_local/reserved_as_local) removed → always present
    pub ctr: PPCRegister,
    pub xer: PPCXERRegister,
    pub reserved: PPCRegister,

    #[cfg(not(feature = "skip_msr"))]
    pub msr: u32, // default set in Default impl

    #[cfg(not(feature = "cr_as_local"))]
    pub cr0: PPCCRRegister,
    #[cfg(not(feature = "cr_as_local"))]
    pub cr1: PPCCRRegister,
    #[cfg(not(feature = "cr_as_local"))]
    pub cr2: PPCCRRegister,
    #[cfg(not(feature = "cr_as_local"))]
    pub cr3: PPCCRRegister,
    #[cfg(not(feature = "cr_as_local"))]
    pub cr4: PPCCRRegister,
    #[cfg(not(feature = "cr_as_local"))]
    pub cr5: PPCCRRegister,
    #[cfg(not(feature = "cr_as_local"))]
    pub cr6: PPCCRRegister,
    #[cfg(not(feature = "cr_as_local"))]
    pub cr7: PPCCRRegister,

    pub fpscr: PPCFPSCRRegister,

    // ---------- FPRs ----------
    #[cfg(not(feature = "non_argument_as_local"))]
    pub f0: PPCRegister,
    pub f1: PPCRegister,
    pub f2: PPCRegister,
    pub f3: PPCRegister,
    pub f4: PPCRegister,
    pub f5: PPCRegister,
    pub f6: PPCRegister,
    pub f7: PPCRegister,
    pub f8: PPCRegister,
    pub f9: PPCRegister,
    pub f10: PPCRegister,
    pub f11: PPCRegister,
    pub f12: PPCRegister,
    pub f13: PPCRegister,

    // (non_volatile_as_local) removed → always present f14..f31
    pub f14: PPCRegister,
    pub f15: PPCRegister,
    pub f16: PPCRegister,
    pub f17: PPCRegister,
    pub f18: PPCRegister,
    pub f19: PPCRegister,
    pub f20: PPCRegister,
    pub f21: PPCRegister,
    pub f22: PPCRegister,
    pub f23: PPCRegister,
    pub f24: PPCRegister,
    pub f25: PPCRegister,
    pub f26: PPCRegister,
    pub f27: PPCRegister,
    pub f28: PPCRegister,
    pub f29: PPCRegister,
    pub f30: PPCRegister,
    pub f31: PPCRegister,

    // ---------- Vector registers (VMX/Altivec) ----------
    pub v0: PPCVRegister,
    pub v1: PPCVRegister,
    pub v2: PPCVRegister,
    pub v3: PPCVRegister,
    pub v4: PPCVRegister,
    pub v5: PPCVRegister,
    pub v6: PPCVRegister,
    pub v7: PPCVRegister,
    pub v8: PPCVRegister,
    pub v9: PPCVRegister,
    pub v10: PPCVRegister,
    pub v11: PPCVRegister,
    pub v12: PPCVRegister,
    pub v13: PPCVRegister,

    // (non_volatile_as_local) removed → always present v14..v31
    pub v14: PPCVRegister,
    pub v15: PPCVRegister,
    pub v16: PPCVRegister,
    pub v17: PPCVRegister,
    pub v18: PPCVRegister,
    pub v19: PPCVRegister,
    pub v20: PPCVRegister,
    pub v21: PPCVRegister,
    pub v22: PPCVRegister,
    pub v23: PPCVRegister,
    pub v24: PPCVRegister,
    pub v25: PPCVRegister,
    pub v26: PPCVRegister,
    pub v27: PPCVRegister,
    pub v28: PPCVRegister,
    pub v29: PPCVRegister,
    pub v30: PPCVRegister,
    pub v31: PPCVRegister,

    // v32..v63 keep non_argument_as_local gate (your current setup)
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v32: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v33: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v34: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v35: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v36: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v37: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v38: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v39: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v40: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v41: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v42: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v43: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v44: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v45: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v46: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v47: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v48: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v49: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v50: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v51: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v52: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v53: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v54: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v55: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v56: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v57: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v58: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v59: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v60: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v61: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v62: PPCVRegister,
    #[cfg(not(feature = "non_argument_as_local"))]
    pub v63: PPCVRegister,

    // (non_volatile_as_local) removed → always present v64..v127
    pub v64: PPCVRegister,
    pub v65: PPCVRegister,
    pub v66: PPCVRegister,
    pub v67: PPCVRegister,
    pub v68: PPCVRegister,
    pub v69: PPCVRegister,
    pub v70: PPCVRegister,
    pub v71: PPCVRegister,
    pub v72: PPCVRegister,
    pub v73: PPCVRegister,
    pub v74: PPCVRegister,
    pub v75: PPCVRegister,
    pub v76: PPCVRegister,
    pub v77: PPCVRegister,
    pub v78: PPCVRegister,
    pub v79: PPCVRegister,
    pub v80: PPCVRegister,
    pub v81: PPCVRegister,
    pub v82: PPCVRegister,
    pub v83: PPCVRegister,
    pub v84: PPCVRegister,
    pub v85: PPCVRegister,
    pub v86: PPCVRegister,
    pub v87: PPCVRegister,
    pub v88: PPCVRegister,
    pub v89: PPCVRegister,
    pub v90: PPCVRegister,
    pub v91: PPCVRegister,
    pub v92: PPCVRegister,
    pub v93: PPCVRegister,
    pub v94: PPCVRegister,
    pub v95: PPCVRegister,
    pub v96: PPCVRegister,
    pub v97: PPCVRegister,
    pub v98: PPCVRegister,
    pub v99: PPCVRegister,
    pub v100: PPCVRegister,
    pub v101: PPCVRegister,
    pub v102: PPCVRegister,
    pub v103: PPCVRegister,
    pub v104: PPCVRegister,
    pub v105: PPCVRegister,
    pub v106: PPCVRegister,
    pub v107: PPCVRegister,
    pub v108: PPCVRegister,
    pub v109: PPCVRegister,
    pub v110: PPCVRegister,
    pub v111: PPCVRegister,
    pub v112: PPCVRegister,
    pub v113: PPCVRegister,
    pub v114: PPCVRegister,
    pub v115: PPCVRegister,
    pub v116: PPCVRegister,
    pub v117: PPCVRegister,
    pub v118: PPCVRegister,
    pub v119: PPCVRegister,
    pub v120: PPCVRegister,
    pub v121: PPCVRegister,
    pub v122: PPCVRegister,
    pub v123: PPCVRegister,
    pub v124: PPCVRegister,
    pub v125: PPCVRegister,
    pub v126: PPCVRegister,
    pub v127: PPCVRegister,
}

// Conservative default init to match C++ default MSR and keep other fields zeroed.
impl Default for PPCContext {
    fn default() -> Self {
        // Zero-init, then set MSR if present.
        let mut ctx: Self = unsafe { MaybeUninit::zeroed().assume_init() };
        #[cfg(not(feature = "skip_msr"))]
        {
            ctx.msr = 0x0200A000;
        }
        ctx
    }
}

// ---------- Registers ----------

#[repr(C)]
pub union PPCRegister {
    pub s8: i8,
    pub u8: u8,
    pub s16: i16,
    pub u16: u16,
    pub s32: i32,
    pub u32: u32,
    pub s64: i64,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
}

#[repr(C)]
pub struct PPCXERRegister {
    pub so: u8,
    pub ov: u8,
    pub ca: u8,
}

#[repr(C)]
pub struct PPCCRRegister {
    pub lt: u8,
    pub gt: u8,
    pub eq: u8,
    pub so: u8, // alias for `un` in FP compare
}

impl PPCCRRegister {
    #[inline]
    pub fn compare_i64(&mut self, left: i64, right: i64, xer: &PPCXERRegister) {
        self.lt = (left < right) as u8;
        self.gt = (left > right) as u8;
        self.eq = (left == right) as u8;
        self.so = xer.so;
    }
    #[inline]
    pub fn compare_u64(&mut self, left: u64, right: u64, xer: &PPCXERRegister) {
        self.lt = (left < right) as u8;
        self.gt = (left > right) as u8;
        self.eq = (left == right) as u8;
        self.so = xer.so;
    }
    #[inline]
    pub fn compare_i32(&mut self, left: i32, right: i32, xer: &PPCXERRegister) {
        self.lt = (left < right) as u8;
        self.gt = (left > right) as u8;
        self.eq = (left == right) as u8;
        self.so = xer.so;
    }
    #[inline]
    pub fn compare_u32(&mut self, left: u32, right: u32, xer: &PPCXERRegister) {
        self.lt = (left < right) as u8;
        self.gt = (left > right) as u8;
        self.eq = (left == right) as u8;
        self.so = xer.so;
    }
    #[inline]
    pub fn compare_f64(&mut self, left: f64, right: f64) {
        let un = left.is_nan() || right.is_nan();
        self.lt = (!un && left < right) as u8;
        self.gt = (!un && left > right) as u8;
        self.eq = (!un && left == right) as u8;
        self.so = un as u8; // mirror original `un` aliasing into `so`
    }

    // SIMD mask helpers (x86_64 only)
    #[cfg(target_arch = "x86_64")]
    #[inline]
    pub unsafe fn set_from_mask_ps(&mut self, mask: core::arch::x86_64::__m128, imm: i32) {
        use core::arch::x86_64::_mm_movemask_ps;
        let m = _mm_movemask_ps(mask);
        self.lt = (m == imm) as u8;
        self.gt = 0;
        self.eq = (m == 0) as u8;
        self.so = 0;
    }
    #[cfg(target_arch = "x86_64")]
    #[inline]
    pub unsafe fn set_from_mask_epi8(&mut self, mask: core::arch::x86_64::__m128i, imm: i32) {
        use core::arch::x86_64::_mm_movemask_epi8;
        let m = _mm_movemask_epi8(mask);
        self.lt = (m == imm) as u8;
        self.gt = 0;
        self.eq = (m == 0) as u8;
        self.so = 0;
    }
}

#[repr(C, align(16))]
pub union PPCVRegister {
    pub s8: [i8; 16],
    pub u8: [u8; 16],
    pub s16: [i16; 8],
    pub u16: [u16; 8],
    pub s32: [i32; 4],
    pub u32: [u32; 4],
    pub s64: [i64; 2],
    pub u64: [u64; 2],
    pub f32: [f32; 4],
    pub f64: [f64; 2],
}

// ---------- FPSCR ----------

pub const PPC_ROUND_NEAREST: usize = 0x00;
pub const PPC_ROUND_TOWARD_ZERO: usize = 0x01;
pub const PPC_ROUND_UP: usize = 0x02;
pub const PPC_ROUND_DOWN: usize = 0x03;
pub const PPC_ROUND_MASK: usize = 0x03;

#[repr(C)]
pub struct PPCFPSCRRegister {
    pub csr: u32,
}

impl PPCFPSCRRegister {
    // Host<->Guest rounding map (shared)
    const HOST_TO_GUEST: [usize; 4] = [PPC_ROUND_NEAREST, PPC_ROUND_DOWN, PPC_ROUND_UP, PPC_ROUND_TOWARD_ZERO];

    // x86_64 path
    #[cfg(target_arch = "x86_64")]
    #[inline]
    fn getcsr() -> u32 {
        unsafe { core::arch::x86_64::_mm_getcsr() as u32 }
    }
    #[cfg(target_arch = "x86_64")]
    #[inline]
    fn setcsr(v: u32) {
        unsafe { core::arch::x86_64::_mm_setcsr(v as u32) }
    }
    #[cfg(target_arch = "x86_64")]
    const ROUND_SHIFT: usize = 13;
    #[cfg(target_arch = "x86_64")]
    const ROUND_MASK: u32 = 0b11 << Self::ROUND_SHIFT;
    // NOTE: Intel MXCSR: bit 15 (FZ) and bit 6 (DAZ).
    #[cfg(target_arch = "x86_64")]
    const FLUSH_MASK: u32 = (1 << 15) | (1 << 6);
    #[cfg(target_arch = "x86_64")]
    const GUEST_TO_HOST: [u32; 4] = {
        // nearest, toward-zero, up(+inf), down(-inf)
        const NEAREST: u32 = 0b00 << 13;
        const TOWARD_ZERO: u32 = 0b11 << 13;
        const UP: u32 = 0b10 << 13;
        const DOWN: u32 = 0b01 << 13;
        [NEAREST, TOWARD_ZERO, UP, DOWN]
    };

    // AArch64 path
    #[cfg(target_arch = "aarch64")]
    #[inline]
    fn getcsr() -> u32 {
        let v: u64;
        unsafe { core::arch::asm!("mrs {dst}, fpcr", dst = out(reg) v); }
        v as u32
    }
    #[cfg(target_arch = "aarch64")]
    #[inline]
    fn setcsr(v: u32) {
        let vv = v as u64;
        unsafe { core::arch::asm!("msr fpcr, {src}", src = in(reg) vv); }
    }
    #[cfg(target_arch = "aarch64")]
    const ROUND_SHIFT: usize = 22;
    #[cfg(target_arch = "aarch64")]
    const ROUND_MASK: u32 = 0b11 << Self::ROUND_SHIFT;
    // FZ (bit 24) and FZ16 (bit 19)
    #[cfg(target_arch = "aarch64")]
    const FLUSH_MASK: u32 = (1 << 24) | (1 << 19);
    #[cfg(target_arch = "aarch64")]
    const GUEST_TO_HOST: [u32; 4] = {
        // nearest=00, +inf=01, -inf=10, toward-zero=11 (per AArch64 FPCR)
        const NEAREST: u32 = 0b00 << 22;
        const UP: u32 = 0b01 << 22;
        const DOWN: u32 = 0b10 << 22;
        const TOWARD_ZERO: u32 = 0b11 << 22;
        [NEAREST, TOWARD_ZERO, UP, DOWN]
    };

    #[inline]
    pub fn load_from_host(&mut self) -> u32 {
        let csr = Self::getcsr();
        self.csr = csr;
        let idx = ((csr & Self::ROUND_MASK) as usize) >> Self::ROUND_SHIFT;
        Self::HOST_TO_GUEST[idx] as u32
    }

    #[inline]
    pub fn store_from_guest(&mut self, value: u32) {
        let mut csr = Self::getcsr();
        csr &= !Self::ROUND_MASK;
        let idx = (value as usize) & PPC_ROUND_MASK;
        csr |= Self::GUEST_TO_HOST[idx];
        Self::setcsr(csr);
        self.csr = csr;
    }

    #[inline]
    pub fn enable_flush_mode_unconditional(&mut self) {
        let mut csr = Self::getcsr();
        csr |= Self::FLUSH_MASK;
        Self::setcsr(csr);
        self.csr = csr;
    }

    #[inline]
    pub fn disable_flush_mode_unconditional(&mut self) {
        let mut csr = Self::getcsr();
        csr &= !Self::FLUSH_MASK;
        Self::setcsr(csr);
        self.csr = csr;
    }

    #[inline]
    pub fn enable_flush_mode(&mut self) {
        let mut csr = Self::getcsr();
        if (csr & Self::FLUSH_MASK) != Self::FLUSH_MASK {
            csr |= Self::FLUSH_MASK;
            Self::setcsr(csr);
        }
        self.csr = csr;
    }

    #[inline]
    pub fn disable_flush_mode(&mut self) {
        let mut csr = Self::getcsr();
        if (csr & Self::FLUSH_MASK) != 0 {
            csr &= !Self::FLUSH_MASK;
            Self::setcsr(csr);
        }
        self.csr = csr;
    }
}

// ---------- Macros to keep emitted code source-compatible ----------

/// Assumes `ctx` and `base` are in scope (your generated functions already take them).
#[macro_export]
macro_rules! PPC_CALL_INDIRECT_FUNC {
    ($ea:expr) => {{
        unsafe { crate::rt::call_indirect($ea as u32, ctx, base) }
    }};
}

// Vector path uses these in a few places; forward to runtime BE stores.
#[macro_export]
macro_rules! PPC_STORE_U8 {
    ($addr:expr, $val:expr) => {{
        unsafe { crate::rt::store_u8(base, $addr as u32, $val as u8) }
    }};
}
#[macro_export]
macro_rules! PPC_STORE_U16 {
    ($addr:expr, $val:expr) => {{
        unsafe { crate::rt::store_u16(base, $addr as u32, $val as u16) }
    }};
}
#[macro_export]
macro_rules! PPC_STORE_U32 {
    ($addr:expr, $val:expr) => {{
        unsafe { crate::rt::store_u32(base, $addr as u32, $val as u32) }
    }};
}
#[macro_export]
macro_rules! PPC_STORE_U64 {
    ($addr:expr, $val:expr) => {{
        unsafe { crate::rt::store_u64(base, $addr as u32, $val as u64) }
    }};
}

// If you need explicit MMIO variants later, alias for now (as in the C++ header).
pub use PPC_STORE_U8 as PPC_MM_STORE_U8;
pub use PPC_STORE_U16 as PPC_MM_STORE_U16;
pub use PPC_STORE_U32 as PPC_MM_STORE_U32;
pub use PPC_STORE_U64 as PPC_MM_STORE_U64;

// ---------- Vector masks / tables (public so the emitter can refer to them) ----------

#[rustfmt::skip]
pub static VectorMaskL: [u8; 16*16] = [
    0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,
    0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,
    0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,
    0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,
    0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,
    0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,0x0B,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,0x0C,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,0x0D,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0E,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,
];

#[rustfmt::skip]
pub static VectorMaskR: [u8; 16*16] = [
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,0xFF,
    0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,0xFF,
    0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,0xFF,
    0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,0xFF,
];

#[rustfmt::skip]
pub static VectorShiftTableL: [u8; 16*16] = [
    0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00,
    0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,
    0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,
    0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,
    0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,
    0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,
    0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,
    0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,
    0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,
    0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,
    0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,
    0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,
    0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,
    0x1C,0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,
    0x1D,0x1C,0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,
    0x1E,0x1D,0x1C,0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,
];

#[rustfmt::skip]
pub static VectorShiftTableR: [u8; 16*16] = [
    0x1F,0x1E,0x1D,0x1C,0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,
    0x1E,0x1D,0x1C,0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,
    0x1D,0x1C,0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,
    0x1C,0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,
    0x1B,0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,
    0x1A,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,
    0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,
    0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,
    0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,
    0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,
    0x15,0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,
    0x14,0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,
    0x13,0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,
    0x12,0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,
    0x11,0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,
    0x10,0x0F,0x0E,0x0D,0x0C,0x0B,0x0A,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,
];

// ---------- Helpful arch timers for MFTB (optional re-export) ----------

#[inline]
pub fn rdtsc_u64() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe { core::arch::x86_64::_rdtsc() }
    }
    #[cfg(target_arch = "aarch64")]
    {
        let v: u64;
        unsafe { core::arch::asm!("mrs {dst}, cntvct_el0", dst = out(reg) v); }
        v
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        0
    }
}
