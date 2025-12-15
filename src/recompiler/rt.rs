// ppc_ctx/src/rt.rs

#![allow(dead_code)]

use core::ptr;
use crate::PPCContext;

extern "C" {
    /// Current image VA base (0 if no image loaded yet)
    fn rex_img_base() -> u32;
    /// Current image byte size
    fn rex_img_size() -> u32;
    /// Code base VA (falls back to image base if not set)
    fn rex_code_base() -> u32;
    /// Guest VA of the "missing import" stub
    fn rex_missing_import_stub_va() -> u32;
}

/// Big-endian 8/16/32/64-bit loads from `base + addr`.
/// These are the Rust equivalents of PPC_LOAD_U* macros.

#[inline(always)]
pub unsafe fn load_u8(base: *const u8, addr: u32) -> u8 {
    ptr::read_volatile(base.add(addr as usize))
}

#[inline(always)]
pub unsafe fn load_u16(base: *const u8, addr: u32) -> u16 {
    let p = base.add(addr as usize);
    let b0 = ptr::read_volatile(p);
    let b1 = ptr::read_volatile(p.add(1));
    u16::from_be_bytes([b0, b1])
}

#[inline(always)]
pub unsafe fn load_u32(base: *const u8, addr: u32) -> u32 {
    let p = base.add(addr as usize);
    let b0 = ptr::read_volatile(p);
    let b1 = ptr::read_volatile(p.add(1));
    let b2 = ptr::read_volatile(p.add(2));
    let b3 = ptr::read_volatile(p.add(3));
    u32::from_be_bytes([b0, b1, b2, b3])
}

#[inline(always)]
pub unsafe fn load_u64(base: *const u8, addr: u32) -> u64 {
    let p = base.add(addr as usize);
    let b0 = ptr::read_volatile(p);
    let b1 = ptr::read_volatile(p.add(1));
    let b2 = ptr::read_volatile(p.add(2));
    let b3 = ptr::read_volatile(p.add(3));
    let b4 = ptr::read_volatile(p.add(4));
    let b5 = ptr::read_volatile(p.add(5));
    let b6 = ptr::read_volatile(p.add(6));
    let b7 = ptr::read_volatile(p.add(7));
    u64::from_be_bytes([b0, b1, b2, b3, b4, b5, b6, b7])
}

/// Big-endian stores (PPC_STORE_U*).

#[inline(always)]
pub unsafe fn store_u8(base: *mut u8, addr: u32, val: u8) {
    ptr::write_volatile(base.add(addr as usize), val);
}

#[inline(always)]
pub unsafe fn store_u16(base: *mut u8, addr: u32, val: u16) {
    let p = base.add(addr as usize);
    let bytes = val.to_be_bytes();
    ptr::write_volatile(p, bytes[0]);
    ptr::write_volatile(p.add(1), bytes[1]);
}

#[inline(always)]
pub unsafe fn store_u32(base: *mut u8, addr: u32, val: u32) {
    let p = base.add(addr as usize);
    let bytes = val.to_be_bytes();
    ptr::write_volatile(p, bytes[0]);
    ptr::write_volatile(p.add(1), bytes[1]);
    ptr::write_volatile(p.add(2), bytes[2]);
    ptr::write_volatile(p.add(3), bytes[3]);
}

#[inline(always)]
pub unsafe fn store_u64(base: *mut u8, addr: u32, val: u64) {
    let p = base.add(addr as usize);
    let bytes = val.to_be_bytes();
    ptr::write_volatile(p, bytes[0]);
    ptr::write_volatile(p.add(1), bytes[1]);
    ptr::write_volatile(p.add(2), bytes[2]);
    ptr::write_volatile(p.add(3), bytes[3]);
    ptr::write_volatile(p.add(4), bytes[4]);
    ptr::write_volatile(p.add(5), bytes[5]);
    ptr::write_volatile(p.add(6), bytes[6]);
    ptr::write_volatile(p.add(7), bytes[7]);
}

#[inline(always)]
pub unsafe fn stwcx32(base: *mut u8, addr: u32, expected: u32, new: u32) -> bool {
    // Emulate PPC stwcx.: compare 32-bit value at EA with `expected`,
    // and if equal, store `new`. Returns true on success.
    //
    // Values are in host-endian u32; load_u32/store_u32 already handle BE layout.
    let current = load_u32(base as *const u8, addr);
    if current == expected {
        store_u32(base, addr, new);
        true
    } else {
        false
    }
}

#[inline(always)]
pub unsafe fn stdcx64(base: *mut u8, addr: u32, expected: u64, new: u64) -> bool {
    // Emulate PPC stdcx.: compare 64-bit value at EA with `expected`,
    // and if equal, store `new`. Returns true on success.
    let current = load_u64(base as *const u8, addr);
    if current == expected {
        store_u64(base, addr, new);
        true
    } else {
        false
    }
}

#[inline(always)]
pub unsafe fn memset_ea(base: *mut u8, addr: u32, val: u8, len: usize) {
    // Equivalent to: memset(base + addr, val, len), but volatile.
    let p = base.add(addr as usize);
    for i in 0..len {
        core::ptr::write_volatile(p.add(i), val);
    }
}

/// Helpful arch timers for MFTB-style timebase reads.
///
/// On x86_64 we use RDTSCP; on AArch64 we use CNTVCT_EL0.
/// Other targets just return 0.
#[inline(always)]
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

/// MMIO helpers can just alias to normal loads/stores for now.

#[inline(always)]
pub unsafe fn mm_load_u8(base: *const u8, addr: u32) -> u8 {
    load_u8(base, addr)
}
#[inline(always)]
pub unsafe fn mm_load_u16(base: *const u8, addr: u32) -> u16 {
    load_u16(base, addr)
}
#[inline(always)]
pub unsafe fn mm_load_u32(base: *const u8, addr: u32) -> u32 {
    load_u32(base, addr)
}
#[inline(always)]
pub unsafe fn mm_load_u64(base: *const u8, addr: u32) -> u64 {
    load_u64(base, addr)
}

#[inline(always)]
pub unsafe fn mm_store_u8(base: *mut u8, addr: u32, val: u8) {
    store_u8(base, addr, val)
}
#[inline(always)]
pub unsafe fn mm_store_u16(base: *mut u8, addr: u32, val: u16) {
    store_u16(base, addr, val)
}
#[inline(always)]
pub unsafe fn mm_store_u32(base: *mut u8, addr: u32, val: u32) {
    store_u32(base, addr, val)
}
#[inline(always)]
pub unsafe fn mm_store_u64(base: *mut u8, addr: u32, val: u64) {
    store_u64(base, addr, val)
}

fn sanitize_indirect_target(raw: u32) -> u32 {
    unsafe {
        let img_base = rex_img_base();
        let img_size = rex_img_size();
        let code_base = rex_code_base();
        let img_end  = img_base.wrapping_add(img_size);

        // Nothing loaded / obviously bogus
        if img_base == 0 || img_size == 0 {
            return 0;
        }

        // Reject NULL and misaligned targets immediately.
        if raw == 0 || (raw & 3) != 0 {
            return 0;
        }

        // If the raw looks like a normal VA inside the image, accept it.
        if raw >= code_base && raw < img_end {
            return raw;
        }

        // If it looks like an RVA relative to code base (e.g. low range),
        // you could normalise here. For now, be conservative and treat it
        // as unresolved.
        0
    }
}

/// Type of the guest-call dispatcher: given a guest VA, call that function.
pub type GuestDispatcher = unsafe fn(guest_va: u32, ctx: &mut PPCContext, base: *mut u8);

/// Global dispatcher used by `rt::call`.
///
/// This is set once by your host (e.g. in ppc_rt or your main) to a function
/// that knows how to map `guest_va` → the right generated Rust function.
static mut GUEST_DISPATCH: Option<GuestDispatcher> = None;

/// Install the guest dispatcher.
///
/// Call this once from your top-level crate, passing something like
/// `ppc_rt::call_guest` (see below).
pub fn install_guest_dispatcher(f: GuestDispatcher) {
    unsafe {
        GUEST_DISPATCH = Some(f);
    }
}

#[inline(always)]
pub fn call(ctx: &mut PPCContext, base: *mut u8, guest_va: u32) {
    unsafe { call_indirect(guest_va, ctx, base) }
}

/// Indirect guest call used by the generated chunks.
///
/// This just forwards to the installed dispatcher. If none is installed,
/// we panic so it’s obvious you forgot to wire it up.
pub unsafe fn call_indirect(guest_va: u32, ctx: &mut PPCContext, base: *mut u8) {
    if let Some(dispatch) = GUEST_DISPATCH {
        dispatch(guest_va, ctx, base);
    } else {
        panic!("rt::call_indirect: no guest dispatcher installed for 0x{guest_va:08X}");
    }
}

/// Basic host-side debug trap.
///
/// This is what the recompiled PPC `debugtrap` / `blrl` sites will call.
/// On supported hosts we trigger a real breakpoint instruction so a debugger
/// will catch it; otherwise we just panic.
#[inline(always)]
pub unsafe fn debugtrap() {
    // x86 / x86_64: INT3
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        core::arch::asm!("int3", options(nomem, nostack, preserves_flags));
    }

    // AArch64: BRK
    #[cfg(target_arch = "aarch64")]
    {
        core::arch::asm!("brk #0", options(nomem, nostack));
    }

    // Fallback: just panic so we get a backtrace / message.
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        panic!("PPC debugtrap hit");
    }
}