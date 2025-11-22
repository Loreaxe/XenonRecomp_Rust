pub fn sub_82B97600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97600 size=16
    let mut pc: u32 = 0x82B97600;
    'dispatch: loop {
        match pc {
            0x82B97600 => {
    //   block [0x82B97600..0x82B97610)
	// 82B97600: C0032E74  lfs f0, 0x2e74(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11892 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97604: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82B97608: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B9760C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97610 size=16
    let mut pc: u32 = 0x82B97610;
    'dispatch: loop {
        match pc {
            0x82B97610 => {
    //   block [0x82B97610..0x82B97620)
	// 82B97610: C0032E78  lfs f0, 0x2e78(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11896 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97614: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82B97618: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B9761C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97620 size=16
    let mut pc: u32 = 0x82B97620;
    'dispatch: loop {
        match pc {
            0x82B97620 => {
    //   block [0x82B97620..0x82B97630)
	// 82B97620: C0032E7C  lfs f0, 0x2e7c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11900 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97624: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82B97628: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B9762C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97630 size=12
    let mut pc: u32 = 0x82B97630;
    'dispatch: loop {
        match pc {
            0x82B97630 => {
    //   block [0x82B97630..0x82B9763C)
	// 82B97630: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B97634: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B97638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97640 size=12
    let mut pc: u32 = 0x82B97640;
    'dispatch: loop {
        match pc {
            0x82B97640 => {
    //   block [0x82B97640..0x82B9764C)
	// 82B97640: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B97644: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B97648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97650 size=12
    let mut pc: u32 = 0x82B97650;
    'dispatch: loop {
        match pc {
            0x82B97650 => {
    //   block [0x82B97650..0x82B9765C)
	// 82B97650: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B97654: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B97658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97660 size=12
    let mut pc: u32 = 0x82B97660;
    'dispatch: loop {
        match pc {
            0x82B97660 => {
    //   block [0x82B97660..0x82B9766C)
	// 82B97660: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B97664: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B97668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97670 size=12
    let mut pc: u32 = 0x82B97670;
    'dispatch: loop {
        match pc {
            0x82B97670 => {
    //   block [0x82B97670..0x82B9767C)
	// 82B97670: A163292C  lhz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B97674: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B97678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97680 size=12
    let mut pc: u32 = 0x82B97680;
    'dispatch: loop {
        match pc {
            0x82B97680 => {
    //   block [0x82B97680..0x82B9768C)
	// 82B97680: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B97684: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82B97688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97690 size=12
    let mut pc: u32 = 0x82B97690;
    'dispatch: loop {
        match pc {
            0x82B97690 => {
    //   block [0x82B97690..0x82B9769C)
	// 82B97690: 8963292C  lbz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B97694: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B97698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B976A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B976A0 size=12
    let mut pc: u32 = 0x82B976A0;
    'dispatch: loop {
        match pc {
            0x82B976A0 => {
    //   block [0x82B976A0..0x82B976AC)
	// 82B976A0: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82B976A4: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B976A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B976B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B976B0 size=12
    let mut pc: u32 = 0x82B976B0;
    'dispatch: loop {
        match pc {
            0x82B976B0 => {
    //   block [0x82B976B0..0x82B976BC)
	// 82B976B0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B976B4: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B976B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B976C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B976C0 size=12
    let mut pc: u32 = 0x82B976C0;
    'dispatch: loop {
        match pc {
            0x82B976C0 => {
    //   block [0x82B976C0..0x82B976CC)
	// 82B976C0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B976C4: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B976C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B976D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B976D0 size=12
    let mut pc: u32 = 0x82B976D0;
    'dispatch: loop {
        match pc {
            0x82B976D0 => {
    //   block [0x82B976D0..0x82B976DC)
	// 82B976D0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B976D4: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B976D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B976E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B976E0 size=12
    let mut pc: u32 = 0x82B976E0;
    'dispatch: loop {
        match pc {
            0x82B976E0 => {
    //   block [0x82B976E0..0x82B976EC)
	// 82B976E0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B976E4: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B976E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B976F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B976F0 size=12
    let mut pc: u32 = 0x82B976F0;
    'dispatch: loop {
        match pc {
            0x82B976F0 => {
    //   block [0x82B976F0..0x82B976FC)
	// 82B976F0: A1632930  lhz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B976F4: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B976F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97700 size=12
    let mut pc: u32 = 0x82B97700;
    'dispatch: loop {
        match pc {
            0x82B97700 => {
    //   block [0x82B97700..0x82B9770C)
	// 82B97700: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B97704: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82B97708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97710 size=12
    let mut pc: u32 = 0x82B97710;
    'dispatch: loop {
        match pc {
            0x82B97710 => {
    //   block [0x82B97710..0x82B9771C)
	// 82B97710: 89632930  lbz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B97714: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B97718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97720 size=12
    let mut pc: u32 = 0x82B97720;
    'dispatch: loop {
        match pc {
            0x82B97720 => {
    //   block [0x82B97720..0x82B9772C)
	// 82B97720: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82B97724: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B97728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97730 size=64
    let mut pc: u32 = 0x82B97730;
    'dispatch: loop {
        match pc {
            0x82B97730 => {
    //   block [0x82B97730..0x82B97770)
	// 82B97730: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B97734: 3960043F  li r11, 0x43f
	ctx.r[11].s64 = 1087;
	// 82B97738: 409A0008  bne cr6, 0x82b97740
	if !ctx.cr[6].eq {
	pc = 0x82B97740; continue 'dispatch;
	}
	// 82B9773C: 39600400  li r11, 0x400
	ctx.r[11].s64 = 1024;
	// 82B97740: 9163294C  stw r11, 0x294c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10572 as u32), ctx.r[11].u32 ) };
	// 82B97744: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 82B97748: 81432944  lwz r10, 0x2944(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 82B9774C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B97750: 516A83DE  rlwimi r10, r11, 0x10, 0xf, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x0000000000010000) | (ctx.r[10].u64 & 0xFFFFFFFFFFFEFFFF);
	// 82B97754: 91432944  stw r10, 0x2944(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10564 as u32), ctx.r[10].u32 ) };
	// 82B97758: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B9775C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82B97760: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B97764: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82B97768: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B9776C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97770 size=12
    let mut pc: u32 = 0x82B97770;
    'dispatch: loop {
        match pc {
            0x82B97770 => {
    //   block [0x82B97770..0x82B9777C)
	// 82B97770: 8163294C  lwz r11, 0x294c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10572 as u32) ) } as u64;
	// 82B97774: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82B97778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97780 size=8
    let mut pc: u32 = 0x82B97780;
    'dispatch: loop {
        match pc {
            0x82B97780 => {
    //   block [0x82B97780..0x82B97788)
	// 82B97780: 80632EFC  lwz r3, 0x2efc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12028 as u32) ) } as u64;
	// 82B97784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97788 size=16
    let mut pc: u32 = 0x82B97788;
    'dispatch: loop {
        match pc {
            0x82B97788 => {
    //   block [0x82B97788..0x82B97798)
	// 82B97788: 8163309C  lwz r11, 0x309c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82B9778C: 90832F00  stw r4, 0x2f00(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12032 as u32), ctx.r[4].u32 ) };
	// 82B97790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B97794: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97798 size=40
    let mut pc: u32 = 0x82B97798;
    'dispatch: loop {
        match pc {
            0x82B97798 => {
    //   block [0x82B97798..0x82B977C0)
	// 82B97798: 8103288C  lwz r8, 0x288c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10380 as u32) ) } as u64;
	// 82B9779C: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82B977A0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82B977A4: 419A001C  beq cr6, 0x82b977c0
	if ctx.cr[6].eq {
		sub_82B977C0(ctx, base);
		return;
	}
	// 82B977A8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82B977AC: 419A0014  beq cr6, 0x82b977c0
	if ctx.cr[6].eq {
		sub_82B977C0(ctx, base);
		return;
	}
	// 82B977B0: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82B977B4: 419A000C  beq cr6, 0x82b977c0
	if ctx.cr[6].eq {
		sub_82B977C0(ctx, base);
		return;
	}
	// 82B977B8: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82B977BC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B977C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B977C0 size=12
    let mut pc: u32 = 0x82B977C0;
    'dispatch: loop {
        match pc {
            0x82B977C0 => {
    //   block [0x82B977C0..0x82B977CC)
	// 82B977C0: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82B977C4: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B977C8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B977CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B977CC size=76
    let mut pc: u32 = 0x82B977CC;
    'dispatch: loop {
        match pc {
            0x82B977CC => {
    //   block [0x82B977CC..0x82B97818)
	// 82B977CC: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B977D0: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82B977D4: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82B977D8: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82B977DC: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82B977E0: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B977E4: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B977E8: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82B977EC: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82B977F0: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B977F4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B977F8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B977FC: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 82B97800: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82B97804: 9163288C  stw r11, 0x288c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10380 as u32), ctx.r[11].u32 ) };
	// 82B97808: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B9780C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82B97810: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B97814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97818 size=8
    let mut pc: u32 = 0x82B97818;
    'dispatch: loop {
        match pc {
            0x82B97818 => {
    //   block [0x82B97818..0x82B97820)
	// 82B97818: 80632F00  lwz r3, 0x2f00(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12032 as u32) ) } as u64;
	// 82B9781C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97820 size=16
    let mut pc: u32 = 0x82B97820;
    'dispatch: loop {
        match pc {
            0x82B97820 => {
    //   block [0x82B97820..0x82B97830)
	// 82B97820: 816330A0  lwz r11, 0x30a0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82B97824: 90832F04  stw r4, 0x2f04(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12036 as u32), ctx.r[4].u32 ) };
	// 82B97828: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9782C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97830 size=40
    let mut pc: u32 = 0x82B97830;
    'dispatch: loop {
        match pc {
            0x82B97830 => {
    //   block [0x82B97830..0x82B97858)
	// 82B97830: 81032890  lwz r8, 0x2890(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10384 as u32) ) } as u64;
	// 82B97834: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82B97838: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82B9783C: 419A001C  beq cr6, 0x82b97858
	if ctx.cr[6].eq {
		sub_82B97858(ctx, base);
		return;
	}
	// 82B97840: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82B97844: 419A0014  beq cr6, 0x82b97858
	if ctx.cr[6].eq {
		sub_82B97858(ctx, base);
		return;
	}
	// 82B97848: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82B9784C: 419A000C  beq cr6, 0x82b97858
	if ctx.cr[6].eq {
		sub_82B97858(ctx, base);
		return;
	}
	// 82B97850: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82B97854: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97858 size=12
    let mut pc: u32 = 0x82B97858;
    'dispatch: loop {
        match pc {
            0x82B97858 => {
    //   block [0x82B97858..0x82B97864)
	// 82B97858: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82B9785C: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B97860: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97864 size=76
    let mut pc: u32 = 0x82B97864;
    'dispatch: loop {
        match pc {
            0x82B97864 => {
    //   block [0x82B97864..0x82B978B0)
	// 82B97864: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B97868: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82B9786C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82B97870: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82B97874: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82B97878: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9787C: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B97880: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82B97884: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82B97888: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9788C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B97890: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B97894: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 82B97898: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82B9789C: 91632890  stw r11, 0x2890(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10384 as u32), ctx.r[11].u32 ) };
	// 82B978A0: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B978A4: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82B978A8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B978AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B978B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B978B0 size=8
    let mut pc: u32 = 0x82B978B0;
    'dispatch: loop {
        match pc {
            0x82B978B0 => {
    //   block [0x82B978B0..0x82B978B8)
	// 82B978B0: 80632F04  lwz r3, 0x2f04(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12036 as u32) ) } as u64;
	// 82B978B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B978B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B978B8 size=16
    let mut pc: u32 = 0x82B978B8;
    'dispatch: loop {
        match pc {
            0x82B978B8 => {
    //   block [0x82B978B8..0x82B978C8)
	// 82B978B8: 816330A4  lwz r11, 0x30a4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82B978BC: 90832F08  stw r4, 0x2f08(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12040 as u32), ctx.r[4].u32 ) };
	// 82B978C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B978C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B978C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B978C8 size=40
    let mut pc: u32 = 0x82B978C8;
    'dispatch: loop {
        match pc {
            0x82B978C8 => {
    //   block [0x82B978C8..0x82B978F0)
	// 82B978C8: 81032894  lwz r8, 0x2894(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10388 as u32) ) } as u64;
	// 82B978CC: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82B978D0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82B978D4: 419A001C  beq cr6, 0x82b978f0
	if ctx.cr[6].eq {
		sub_82B978F0(ctx, base);
		return;
	}
	// 82B978D8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82B978DC: 419A0014  beq cr6, 0x82b978f0
	if ctx.cr[6].eq {
		sub_82B978F0(ctx, base);
		return;
	}
	// 82B978E0: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82B978E4: 419A000C  beq cr6, 0x82b978f0
	if ctx.cr[6].eq {
		sub_82B978F0(ctx, base);
		return;
	}
	// 82B978E8: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82B978EC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B978F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B978F0 size=12
    let mut pc: u32 = 0x82B978F0;
    'dispatch: loop {
        match pc {
            0x82B978F0 => {
    //   block [0x82B978F0..0x82B978FC)
	// 82B978F0: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82B978F4: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B978F8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B978FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B978FC size=76
    let mut pc: u32 = 0x82B978FC;
    'dispatch: loop {
        match pc {
            0x82B978FC => {
    //   block [0x82B978FC..0x82B97948)
	// 82B978FC: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B97900: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82B97904: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82B97908: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82B9790C: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82B97910: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B97914: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B97918: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82B9791C: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82B97920: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B97924: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B97928: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9792C: 798CA7E6  rldicr r12, r12, 0x34, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(52) & 0xFFFFFFFFFFFFFFFF;
	// 82B97930: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82B97934: 91632894  stw r11, 0x2894(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10388 as u32), ctx.r[11].u32 ) };
	// 82B97938: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B9793C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82B97940: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B97944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97948 size=8
    let mut pc: u32 = 0x82B97948;
    'dispatch: loop {
        match pc {
            0x82B97948 => {
    //   block [0x82B97948..0x82B97950)
	// 82B97948: 80632F08  lwz r3, 0x2f08(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12040 as u32) ) } as u64;
	// 82B9794C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97950 size=36
    let mut pc: u32 = 0x82B97950;
    'dispatch: loop {
        match pc {
            0x82B97950 => {
    //   block [0x82B97950..0x82B97974)
	// 82B97950: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82B97954: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97958: D0032980  stfs f0, 0x2980(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10624 as u32), tmp.u32 ) };
	// 82B9795C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B97960: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B97964: 798C7FE6  rldicr r12, r12, 0x2f, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(47) & 0xFFFFFFFFFFFFFFFF;
	// 82B97968: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B9796C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B97970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97978 size=16
    let mut pc: u32 = 0x82B97978;
    'dispatch: loop {
        match pc {
            0x82B97978 => {
    //   block [0x82B97978..0x82B97988)
	// 82B97978: C0032980  lfs f0, 0x2980(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10624 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B9797C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82B97980: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B97984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97988 size=16
    let mut pc: u32 = 0x82B97988;
    'dispatch: loop {
        match pc {
            0x82B97988 => {
    //   block [0x82B97988..0x82B97998)
	// 82B97988: C003297C  lfs f0, 0x297c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10620 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B9798C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82B97990: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B97994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97998 size=12
    let mut pc: u32 = 0x82B97998;
    'dispatch: loop {
        match pc {
            0x82B97998 => {
    //   block [0x82B97998..0x82B979A4)
	// 82B97998: 81632978  lwz r11, 0x2978(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10616 as u32) ) } as u64;
	// 82B9799C: 556307BE  clrlwi r3, r11, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B979A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B979A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B979A8 size=12
    let mut pc: u32 = 0x82B979A8;
    'dispatch: loop {
        match pc {
            0x82B979A8 => {
    //   block [0x82B979A8..0x82B979B4)
	// 82B979A8: 816329C0  lwz r11, 0x29c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10688 as u32) ) } as u64;
	// 82B979AC: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82B979B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B979B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B979B8 size=28
    let mut pc: u32 = 0x82B979B8;
    'dispatch: loop {
        match pc {
            0x82B979B8 => {
    //   block [0x82B979B8..0x82B979D4)
	// 82B979B8: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82B979BC: 508BAA94  rlwimi r11, r4, 0x15, 0xa, 0xa
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(21) as u64) & 0x0000000000200000) | (ctx.r[11].u64 & 0xFFFFFFFFFFDFFFFF);
	// 82B979C0: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 82B979C4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B979C8: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82B979CC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B979D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B979D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B979D8 size=12
    let mut pc: u32 = 0x82B979D8;
    'dispatch: loop {
        match pc {
            0x82B979D8 => {
    //   block [0x82B979D8..0x82B979E4)
	// 82B979D8: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82B979DC: 55635FFE  rlwinm r3, r11, 0xb, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	// 82B979E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B979E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B979E8 size=28
    let mut pc: u32 = 0x82B979E8;
    'dispatch: loop {
        match pc {
            0x82B979E8 => {
    //   block [0x82B979E8..0x82B97A04)
	// 82B979E8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B979EC: 908328D8  stw r4, 0x28d8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10456 as u32), ctx.r[4].u32 ) };
	// 82B979F0: E9430010  ld r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B979F4: 798C37E6  rldicr r12, r12, 0x26, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(38) & 0xFFFFFFFFFFFFFFFF;
	// 82B979F8: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B979FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B97A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97A08 size=8
    let mut pc: u32 = 0x82B97A08;
    'dispatch: loop {
        match pc {
            0x82B97A08 => {
    //   block [0x82B97A08..0x82B97A10)
	// 82B97A08: 806328D8  lwz r3, 0x28d8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10456 as u32) ) } as u64;
	// 82B97A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97A10 size=12
    let mut pc: u32 = 0x82B97A10;
    'dispatch: loop {
        match pc {
            0x82B97A10 => {
    //   block [0x82B97A10..0x82B97A1C)
	// 82B97A10: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82B97A14: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B97A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97A20 size=20
    let mut pc: u32 = 0x82B97A20;
    'dispatch: loop {
        match pc {
            0x82B97A20 => {
    //   block [0x82B97A20..0x82B97A34)
	// 82B97A20: 8963293C  lbz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82B97A24: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82B97A28: 514B063A  rlwimi r11, r10, 0, 0x18, 0x1d
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x00000000000000FC) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF03);
	// 82B97A2C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B97A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97A38 size=36
    let mut pc: u32 = 0x82B97A38;
    'dispatch: loop {
        match pc {
            0x82B97A38 => {
    //   block [0x82B97A38..0x82B97A5C)
	// 82B97A38: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82B97A3C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97A40: D00329CC  stfs f0, 0x29cc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10700 as u32), tmp.u32 ) };
	// 82B97A44: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B97A48: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B97A4C: 798C07E6  rldicr r12, r12, 0x20, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 82B97A50: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B97A54: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B97A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97A60 size=8
    let mut pc: u32 = 0x82B97A60;
    'dispatch: loop {
        match pc {
            0x82B97A60 => {
    //   block [0x82B97A60..0x82B97A68)
	// 82B97A60: 806329CC  lwz r3, 0x29cc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10700 as u32) ) } as u64;
	// 82B97A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97A68 size=36
    let mut pc: u32 = 0x82B97A68;
    'dispatch: loop {
        match pc {
            0x82B97A68 => {
    //   block [0x82B97A68..0x82B97A8C)
	// 82B97A68: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82B97A6C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97A70: D00329C4  stfs f0, 0x29c4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10692 as u32), tmp.u32 ) };
	// 82B97A74: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B97A78: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B97A7C: 798C17E6  rldicr r12, r12, 0x22, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(34) & 0xFFFFFFFFFFFFFFFF;
	// 82B97A80: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B97A84: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B97A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97A90 size=8
    let mut pc: u32 = 0x82B97A90;
    'dispatch: loop {
        match pc {
            0x82B97A90 => {
    //   block [0x82B97A90..0x82B97A98)
	// 82B97A90: 806329C4  lwz r3, 0x29c4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10692 as u32) ) } as u64;
	// 82B97A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97A98 size=28
    let mut pc: u32 = 0x82B97A98;
    'dispatch: loop {
        match pc {
            0x82B97A98 => {
    //   block [0x82B97A98..0x82B97AB4)
	// 82B97A98: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82B97A9C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97AA0: D00329D0  stfs f0, 0x29d0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10704 as u32), tmp.u32 ) };
	// 82B97AA4: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B97AA8: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82B97AAC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B97AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97AB8 size=8
    let mut pc: u32 = 0x82B97AB8;
    'dispatch: loop {
        match pc {
            0x82B97AB8 => {
    //   block [0x82B97AB8..0x82B97AC0)
	// 82B97AB8: 806329D0  lwz r3, 0x29d0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10704 as u32) ) } as u64;
	// 82B97ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97AC0 size=36
    let mut pc: u32 = 0x82B97AC0;
    'dispatch: loop {
        match pc {
            0x82B97AC0 => {
    //   block [0x82B97AC0..0x82B97AE4)
	// 82B97AC0: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82B97AC4: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97AC8: D00329C8  stfs f0, 0x29c8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10696 as u32), tmp.u32 ) };
	// 82B97ACC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B97AD0: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B97AD4: 798C0FE6  rldicr r12, r12, 0x21, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(33) & 0xFFFFFFFFFFFFFFFF;
	// 82B97AD8: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B97ADC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B97AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97AE8 size=8
    let mut pc: u32 = 0x82B97AE8;
    'dispatch: loop {
        match pc {
            0x82B97AE8 => {
    //   block [0x82B97AE8..0x82B97AF0)
	// 82B97AE8: 806329C8  lwz r3, 0x29c8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10696 as u32) ) } as u64;
	// 82B97AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97AF0 size=12
    let mut pc: u32 = 0x82B97AF0;
    'dispatch: loop {
        match pc {
            0x82B97AF0 => {
    //   block [0x82B97AF0..0x82B97AFC)
	// 82B97AF0: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82B97AF4: 5563677E  rlwinm r3, r11, 0xc, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82B97AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B00 size=12
    let mut pc: u32 = 0x82B97B00;
    'dispatch: loop {
        match pc {
            0x82B97B00 => {
    //   block [0x82B97B00..0x82B97B0C)
	// 82B97B00: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82B97B04: 55637F7E  rlwinm r3, r11, 0xf, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 82B97B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B10 size=12
    let mut pc: u32 = 0x82B97B10;
    'dispatch: loop {
        match pc {
            0x82B97B10 => {
    //   block [0x82B97B10..0x82B97B1C)
	// 82B97B10: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82B97B14: 5563EFFE  rlwinm r3, r11, 0x1d, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B97B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B20 size=12
    let mut pc: u32 = 0x82B97B20;
    'dispatch: loop {
        match pc {
            0x82B97B20 => {
    //   block [0x82B97B20..0x82B97B2C)
	// 82B97B20: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82B97B24: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B97B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B30 size=12
    let mut pc: u32 = 0x82B97B30;
    'dispatch: loop {
        match pc {
            0x82B97B30 => {
    //   block [0x82B97B30..0x82B97B3C)
	// 82B97B30: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82B97B34: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B97B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B40 size=8
    let mut pc: u32 = 0x82B97B40;
    'dispatch: loop {
        match pc {
            0x82B97B40 => {
    //   block [0x82B97B40..0x82B97B48)
	// 82B97B40: 88632942  lbz r3, 0x2942(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10562 as u32) ) } as u64;
	// 82B97B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B48 size=8
    let mut pc: u32 = 0x82B97B48;
    'dispatch: loop {
        match pc {
            0x82B97B48 => {
    //   block [0x82B97B48..0x82B97B50)
	// 82B97B48: 9083351C  stw r4, 0x351c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13596 as u32), ctx.r[4].u32 ) };
	// 82B97B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B50 size=8
    let mut pc: u32 = 0x82B97B50;
    'dispatch: loop {
        match pc {
            0x82B97B50 => {
    //   block [0x82B97B50..0x82B97B58)
	// 82B97B50: 8063351C  lwz r3, 0x351c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13596 as u32) ) } as u64;
	// 82B97B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B58 size=16
    let mut pc: u32 = 0x82B97B58;
    'dispatch: loop {
        match pc {
            0x82B97B58 => {
    //   block [0x82B97B58..0x82B97B68)
	// 82B97B58: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82B97B5C: 508BB890  rlwimi r11, r4, 0x17, 2, 8
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(23) as u64) & 0x000000003F800000) | (ctx.r[11].u64 & 0xFFFFFFFFC07FFFFF);
	// 82B97B60: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82B97B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B68 size=12
    let mut pc: u32 = 0x82B97B68;
    'dispatch: loop {
        match pc {
            0x82B97B68 => {
    //   block [0x82B97B68..0x82B97B74)
	// 82B97B68: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82B97B6C: 55634E7E  rlwinm r3, r11, 9, 0x19, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 82B97B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97B78 size=52
    let mut pc: u32 = 0x82B97B78;
    'dispatch: loop {
        match pc {
            0x82B97B78 => {
    //   block [0x82B97B78..0x82B97BAC)
	// 82B97B78: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B97B7C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B97B80: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B97B84: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B97B88: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B97B8C: 554BAFFE  rlwinm r11, r10, 0x15, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000007FFu64;
	// 82B97B90: 552A5D7E  srwi r10, r9, 0x15
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(21);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B97B94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82B97B98: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82B97B9C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B97BA0: 516A077A  rlwimi r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x0000000000000004) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82B97BA4: 5543077E  clrlwi r3, r10, 0x1d
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82B97BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97BB0 size=148
    let mut pc: u32 = 0x82B97BB0;
    'dispatch: loop {
        match pc {
            0x82B97BB0 => {
    //   block [0x82B97BB0..0x82B97C44)
	// 82B97BB0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82B97BB4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82B97BB8: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97BBC: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82B97BC0: 54A8083C  slwi r8, r5, 1
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B97BC4: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82B97BC8: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82B97BCC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82B97BD0: 552907FA  rlwinm r9, r9, 0, 0x1f, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B97BD4: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82B97BD8: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82B97BDC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B97BE0: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B97BE4: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B97BE8: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B97BEC: 78A5FFE6  rldicr r5, r5, 0x3f, 0x3f
	ctx.r[5].u64 = (ctx.r[5].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B97BF0: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82B97BF4: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82B97BF8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82B97BFC: 53FEFB7E  rlwimi r30, r31, 0x1f, 0xd, 0x1f
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[30].u64 & 0xFFFFFFFFFFF80000);
	// 82B97C00: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 82B97C04: 53FEF856  rlwimi r30, r31, 0x1f, 1, 0xb
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[30].u64 & 0xFFFFFFFF800FFFFF);
	// 82B97C08: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82B97C0C: 57DF6D3E  rlwinm r31, r30, 0xd, 0x14, 0x1f
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x0007FFFFu64;
	// 82B97C10: 7CA73C36  srd r7, r5, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[5].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82B97C14: 7FE94838  and r9, r31, r9
	ctx.r[9].u64 = ctx.r[31].u64 & ctx.r[9].u64;
	// 82B97C18: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B97C1C: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82B97C20: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 82B97C24: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82B97C28: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82B97C2C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B97C30: 7CEB5B78  or r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 82B97C34: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B97C38: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B97C3C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82B97C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97C48 size=16
    let mut pc: u32 = 0x82B97C48;
    'dispatch: loop {
        match pc {
            0x82B97C48 => {
    //   block [0x82B97C48..0x82B97C58)
	// 82B97C48: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97C4C: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82B97C50: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82B97C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97C58 size=52
    let mut pc: u32 = 0x82B97C58;
    'dispatch: loop {
        match pc {
            0x82B97C58 => {
    //   block [0x82B97C58..0x82B97C8C)
	// 82B97C58: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B97C5C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B97C60: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B97C64: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B97C68: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B97C6C: 554BB7FE  rlwinm r11, r10, 0x16, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000003FFu64;
	// 82B97C70: 552A6CFE  srwi r10, r9, 0x13
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(19);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B97C74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82B97C78: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82B97C7C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B97C80: 516A077A  rlwimi r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x0000000000000004) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82B97C84: 5543077E  clrlwi r3, r10, 0x1d
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82B97C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97C90 size=140
    let mut pc: u32 = 0x82B97C90;
    'dispatch: loop {
        match pc {
            0x82B97C90 => {
    //   block [0x82B97C90..0x82B97D1C)
	// 82B97C90: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82B97C94: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82B97C98: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97C9C: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82B97CA0: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82B97CA4: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82B97CA8: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82B97CAC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82B97CB0: 5529003C  rlwinm r9, r9, 0, 0, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B97CB4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82B97CB8: 7D262B78  or r6, r9, r5
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82B97CBC: 791FFFE6  rldicr r31, r8, 0x3f, 0x3f
	ctx.r[31].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B97CC0: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B97CC4: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B97CC8: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B97CCC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82B97CD0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82B97CD4: 53C5FB7E  rlwimi r5, r30, 0x1f, 0xd, 0x1f
	ctx.r[5].u64 = (((ctx.r[30].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFF80000);
	// 82B97CD8: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 82B97CDC: 53C5F856  rlwimi r5, r30, 0x1f, 1, 0xb
	ctx.r[5].u64 = (((ctx.r[30].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[5].u64 & 0xFFFFFFFF800FFFFF);
	// 82B97CE0: 78FE0020  clrldi r30, r7, 0x20
	ctx.r[30].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82B97CE4: 54A76D3E  rlwinm r7, r5, 0xd, 0x14, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x0007FFFFu64;
	// 82B97CE8: 7FE5F436  srd r5, r31, r30
	if (ctx.r[30].u8 & 0x40) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = (ctx.r[31].u64) >> ((ctx.r[30].u8 & 0x3F) as u32);
	}
	// 82B97CEC: 7CE74838  and r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82B97CF0: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82B97CF4: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82B97CF8: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 82B97CFC: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82B97D00: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82B97D04: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B97D08: 7CAB5B78  or r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82B97D0C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B97D10: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B97D14: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82B97D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97D20 size=16
    let mut pc: u32 = 0x82B97D20;
    'dispatch: loop {
        match pc {
            0x82B97D20 => {
    //   block [0x82B97D20..0x82B97D30)
	// 82B97D20: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97D24: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82B97D28: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82B97D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97D30 size=20
    let mut pc: u32 = 0x82B97D30;
    'dispatch: loop {
        match pc {
            0x82B97D30 => {
    //   block [0x82B97D30..0x82B97D44)
	// 82B97D30: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B97D34: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B97D38: 816B048C  lwz r11, 0x48c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1164 as u32) ) } as u64;
	// 82B97D3C: 55634FBE  rlwinm r3, r11, 9, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 82B97D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97D48 size=148
    let mut pc: u32 = 0x82B97D48;
    'dispatch: loop {
        match pc {
            0x82B97D48 => {
    //   block [0x82B97D48..0x82B97DDC)
	// 82B97D48: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82B97D4C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82B97D50: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97D54: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82B97D58: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B97D5C: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82B97D60: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82B97D64: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82B97D68: 552907B8  rlwinm r9, r9, 0, 0x1e, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B97D6C: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82B97D70: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82B97D74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B97D78: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B97D7C: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B97D80: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B97D84: 78A5FFE6  rldicr r5, r5, 0x3f, 0x3f
	ctx.r[5].u64 = (ctx.r[5].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B97D88: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82B97D8C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82B97D90: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82B97D94: 53FEFB7E  rlwimi r30, r31, 0x1f, 0xd, 0x1f
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[30].u64 & 0xFFFFFFFFFFF80000);
	// 82B97D98: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 82B97D9C: 53FEF856  rlwimi r30, r31, 0x1f, 1, 0xb
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[30].u64 & 0xFFFFFFFF800FFFFF);
	// 82B97DA0: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82B97DA4: 57DF6D3E  rlwinm r31, r30, 0xd, 0x14, 0x1f
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x0007FFFFu64;
	// 82B97DA8: 7CA73C36  srd r7, r5, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[5].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82B97DAC: 7FE94838  and r9, r31, r9
	ctx.r[9].u64 = ctx.r[31].u64 & ctx.r[9].u64;
	// 82B97DB0: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B97DB4: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82B97DB8: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 82B97DBC: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82B97DC0: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82B97DC4: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B97DC8: 7CEB5B78  or r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 82B97DCC: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B97DD0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B97DD4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82B97DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97DE0 size=16
    let mut pc: u32 = 0x82B97DE0;
    'dispatch: loop {
        match pc {
            0x82B97DE0 => {
    //   block [0x82B97DE0..0x82B97DF0)
	// 82B97DE0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97DE4: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82B97DE8: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B97DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97DF0 size=12
    let mut pc: u32 = 0x82B97DF0;
    'dispatch: loop {
        match pc {
            0x82B97DF0 => {
    //   block [0x82B97DF0..0x82B97DFC)
	// 82B97DF0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97DF4: 886B2E94  lbz r3, 0x2e94(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11924 as u32) ) } as u64;
	// 82B97DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97E00 size=92
    let mut pc: u32 = 0x82B97E00;
    'dispatch: loop {
        match pc {
            0x82B97E00 => {
    //   block [0x82B97E00..0x82B97E5C)
	// 82B97E00: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82B97E04: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 82B97E08: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B97E0C: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82B97E10: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B97E14: C00A1100  lfs f0, 0x1100(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97E18: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B97E1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B97E20: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82B97E24: 794AFFE6  rldicr r10, r10, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B97E28: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82B97E2C: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82B97E30: C1A10024  lfs f13, 0x24(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82B97E34: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82B97E38: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82B97E3C: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82B97E40: 8121FFF4  lwz r9, -0xc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82B97E44: 51282DF4  rlwimi r8, r9, 5, 0x17, 0x1a
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(5) as u64) & 0x00000000000001E0) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFE1F);
	// 82B97E48: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82B97E4C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B97E50: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B97E54: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B97E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97E60 size=68
    let mut pc: u32 = 0x82B97E60;
    'dispatch: loop {
        match pc {
            0x82B97E60 => {
    //   block [0x82B97E60..0x82B97EA4)
	// 82B97E60: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B97E64: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B97E68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82B97E6C: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82B97E70: C00A0C40  lfs f0, 0xc40(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97E74: 556BB810  slwi r11, r11, 0x17
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(23);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B97E78: 7D6BE670  srawi r11, r11, 0x1c
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 28) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 28) as i64;
	// 82B97E7C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82B97E80: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82B97E84: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B97E88: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82B97E8C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82B97E90: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82B97E94: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82B97E98: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82B97E9C: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B97EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97EA8 size=64
    let mut pc: u32 = 0x82B97EA8;
    'dispatch: loop {
        match pc {
            0x82B97EA8 => {
    //   block [0x82B97EA8..0x82B97EE8)
	// 82B97EA8: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B97EAC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B97EB0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82B97EB4: 816B0490  lwz r11, 0x490(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1168 as u32) ) } as u64;
	// 82B97EB8: C00A0C50  lfs f0, 0xc50(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82B97EBC: 556B502A  slwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B97EC0: 7D6BB670  srawi r11, r11, 0x16
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 22) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 22) as i64;
	// 82B97EC4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82B97EC8: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82B97ECC: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B97ED0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82B97ED4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82B97ED8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82B97EDC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82B97EE0: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B97EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97EE8 size=108
    let mut pc: u32 = 0x82B97EE8;
    'dispatch: loop {
        match pc {
            0x82B97EE8 => {
    //   block [0x82B97EE8..0x82B97F54)
	// 82B97EE8: 39640C40  addi r11, r4, 0xc40
	ctx.r[11].s64 = ctx.r[4].s64 + 3136;
	// 82B97EEC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B97EF0: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B97EF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B97EF8: 419A0050  beq cr6, 0x82b97f48
	if ctx.cr[6].eq {
	pc = 0x82B97F48; continue 'dispatch;
	}
	// 82B97EFC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82B97F00: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82B97F04: 556BF73E  rlwinm r11, r11, 0x1e, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B97F08: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82B97F0C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82B97F10: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82B97F14: 41990008  bgt cr6, 0x82b97f1c
	if ctx.cr[6].gt {
	pc = 0x82B97F1C; continue 'dispatch;
	}
	// 82B97F18: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82B97F1C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B97F20: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
	// 82B97F24: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82B97F28: 516916BA  rlwimi r9, r11, 2, 0x1a, 0x1d
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(2) as u64) & 0x000000000000003C) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFC3);
	// 82B97F2C: 78EBFFE6  rldicr r11, r7, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[7].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B97F30: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82B97F34: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82B97F38: 7D6B5436  srd r11, r11, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82B97F3C: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B97F40: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B97F44: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B97F48: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97F4C: 98AB2EAE  stb r5, 0x2eae(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11950 as u32), ctx.r[5].u8 ) };
	// 82B97F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97F58 size=12
    let mut pc: u32 = 0x82B97F58;
    'dispatch: loop {
        match pc {
            0x82B97F58 => {
    //   block [0x82B97F58..0x82B97F64)
	// 82B97F58: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97F5C: 886B2EAE  lbz r3, 0x2eae(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11950 as u32) ) } as u64;
	// 82B97F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97F68 size=108
    let mut pc: u32 = 0x82B97F68;
    'dispatch: loop {
        match pc {
            0x82B97F68 => {
    //   block [0x82B97F68..0x82B97FD4)
	// 82B97F68: 39640C40  addi r11, r4, 0xc40
	ctx.r[11].s64 = ctx.r[4].s64 + 3136;
	// 82B97F6C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B97F70: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B97F74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B97F78: 419A0050  beq cr6, 0x82b97fc8
	if ctx.cr[6].eq {
	pc = 0x82B97FC8; continue 'dispatch;
	}
	// 82B97F7C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82B97F80: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82B97F84: 556BD73E  rlwinm r11, r11, 0x1a, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B97F88: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82B97F8C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82B97F90: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82B97F94: 41980008  blt cr6, 0x82b97f9c
	if ctx.cr[6].lt {
	pc = 0x82B97F9C; continue 'dispatch;
	}
	// 82B97F98: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82B97F9C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B97FA0: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
	// 82B97FA4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82B97FA8: 516935B2  rlwimi r9, r11, 6, 0x16, 0x19
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(6) as u64) & 0x00000000000003C0) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFC3F);
	// 82B97FAC: 78EBFFE6  rldicr r11, r7, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[7].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B97FB0: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82B97FB4: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82B97FB8: 7D6B5436  srd r11, r11, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82B97FBC: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B97FC0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B97FC4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B97FC8: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97FCC: 98AB2EC8  stb r5, 0x2ec8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11976 as u32), ctx.r[5].u8 ) };
	// 82B97FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B97FD8 size=12
    let mut pc: u32 = 0x82B97FD8;
    'dispatch: loop {
        match pc {
            0x82B97FD8 => {
    //   block [0x82B97FD8..0x82B97FE4)
	// 82B97FD8: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82B97FDC: 886B2EC8  lbz r3, 0x2ec8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11976 as u32) ) } as u64;
	// 82B97FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B97FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B97FE8 size=28
    let mut pc: u32 = 0x82B97FE8;
    'dispatch: loop {
        match pc {
            0x82B97FE8 => {
    //   block [0x82B97FE8..0x82B98004)
	// 82B97FE8: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B97FEC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B97FF0: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82B97FF4: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B97FF8: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82B97FFC: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B98000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98008 size=20
    let mut pc: u32 = 0x82B98008;
    'dispatch: loop {
        match pc {
            0x82B98008 => {
    //   block [0x82B98008..0x82B9801C)
	// 82B98008: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B9800C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B98010: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B98014: 5563B77E  rlwinm r3, r11, 0x16, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82B98018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98020 size=20
    let mut pc: u32 = 0x82B98020;
    'dispatch: loop {
        match pc {
            0x82B98020 => {
    //   block [0x82B98020..0x82B98034)
	// 82B98020: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B98024: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B98028: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9802C: 55639F7E  rlwinm r3, r11, 0x13, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 82B98030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98038 size=20
    let mut pc: u32 = 0x82B98038;
    'dispatch: loop {
        match pc {
            0x82B98038 => {
    //   block [0x82B98038..0x82B9804C)
	// 82B98038: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B9803C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B98040: 7D6B1A2E  lhzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B98044: 5563077E  clrlwi r3, r11, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B98048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98050 size=20
    let mut pc: u32 = 0x82B98050;
    'dispatch: loop {
        match pc {
            0x82B98050 => {
    //   block [0x82B98050..0x82B98064)
	// 82B98050: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B98054: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B98058: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82B9805C: 5563EFBE  rlwinm r3, r11, 0x1d, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B98060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98068 size=60
    let mut pc: u32 = 0x82B98068;
    'dispatch: loop {
        match pc {
            0x82B98068 => {
    //   block [0x82B98068..0x82B980A4)
	// 82B98068: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B9806C: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82B98070: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B98074: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B98078: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B9807C: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82B98080: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B98084: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B98088: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82B9808C: 50A8B152  rlwimi r8, r5, 0x16, 5, 9
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(22) as u64) & 0x0000000007C00000) | (ctx.r[8].u64 & 0xFFFFFFFFF83FFFFF);
	// 82B98090: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82B98094: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B98098: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9809C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B980A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B980A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B980A8 size=24
    let mut pc: u32 = 0x82B980A8;
    'dispatch: loop {
        match pc {
            0x82B980A8 => {
    //   block [0x82B980A8..0x82B980C0)
	// 82B980A8: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B980AC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B980B0: 816B0490  lwz r11, 0x490(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1168 as u32) ) } as u64;
	// 82B980B4: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B980B8: 7D63DE70  srawi r3, r11, 0x1b
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 27) as i64;
	// 82B980BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B980C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B980C0 size=60
    let mut pc: u32 = 0x82B980C0;
    'dispatch: loop {
        match pc {
            0x82B980C0 => {
    //   block [0x82B980C0..0x82B980FC)
	// 82B980C0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B980C4: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82B980C8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B980CC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B980D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B980D4: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82B980D8: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B980DC: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B980E0: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82B980E4: 50A8D808  rlwimi r8, r5, 0x1b, 0, 4
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(27) as u64) & 0x00000000F8000000) | (ctx.r[8].u64 & 0xFFFFFFFF07FFFFFF);
	// 82B980E8: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82B980EC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B980F0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B980F4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B980F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98100 size=20
    let mut pc: u32 = 0x82B98100;
    'dispatch: loop {
        match pc {
            0x82B98100 => {
    //   block [0x82B98100..0x82B98114)
	// 82B98100: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B98104: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B98108: 816B0490  lwz r11, 0x490(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1168 as u32) ) } as u64;
	// 82B9810C: 7D63DE70  srawi r3, r11, 0x1b
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 27) as i64;
	// 82B98110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98118 size=60
    let mut pc: u32 = 0x82B98118;
    'dispatch: loop {
        match pc {
            0x82B98118 => {
    //   block [0x82B98118..0x82B98154)
	// 82B98118: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B9811C: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82B98120: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B98124: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B98128: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B9812C: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82B98130: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B98134: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82B98138: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82B9813C: 50A8177A  rlwimi r8, r5, 2, 0x1d, 0x1d
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(2) as u64) & 0x0000000000000004) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82B98140: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82B98144: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B98148: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9814C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B98150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98158 size=20
    let mut pc: u32 = 0x82B98158;
    'dispatch: loop {
        match pc {
            0x82B98158 => {
    //   block [0x82B98158..0x82B9816C)
	// 82B98158: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B9815C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B98160: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82B98164: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B98168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98170 size=68
    let mut pc: u32 = 0x82B98170;
    'dispatch: loop {
        match pc {
            0x82B98170 => {
    //   block [0x82B98170..0x82B981B4)
	// 82B98170: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82B98174: 7CAA0034  cntlzw r10, r5
	ctx.r[10].u64 = if ctx.r[5].u32 == 0 { 32 } else { ctx.r[5].u32.leading_zeros() as u64 };
	// 82B98178: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82B9817C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B98180: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B98184: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82B98188: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82B9818C: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82B98190: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B98194: 7908FFE6  rldicr r8, r8, 0x3f, 0x3f
	ctx.r[8].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82B98198: 51475D28  rlwimi r7, r10, 0xb, 0x14, 0x14
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[7].u64 & 0xFFFFFFFFFFFFF7FF);
	// 82B9819C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82B981A0: 7D0B4C36  srd r11, r8, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[8].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82B981A4: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B981A8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B981AC: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B981B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B981B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B981B8 size=24
    let mut pc: u32 = 0x82B981B8;
    'dispatch: loop {
        match pc {
            0x82B981B8 => {
    //   block [0x82B981B8..0x82B981D0)
	// 82B981B8: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82B981BC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B981C0: 816B0484  lwz r11, 0x484(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1156 as u32) ) } as u64;
	// 82B981C4: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B981C8: 5563AFFE  rlwinm r3, r11, 0x15, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 82B981CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B981D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B981D0 size=60
    let mut pc: u32 = 0x82B981D0;
    'dispatch: loop {
        match pc {
            0x82B981D0 => {
    //   block [0x82B981D0..0x82B9820C)
	// 82B981D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B981D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B981D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B981DC: 39640C26  addi r11, r4, 0xc26
	ctx.r[11].s64 = ctx.r[4].s64 + 3110;
	// 82B981E0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B981E4: 7D0B182E  lwzx r8, r11, r3
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B981E8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82B981EC: 419A000C  beq cr6, 0x82b981f8
	if ctx.cr[6].eq {
	pc = 0x82B981F8; continue 'dispatch;
	}
	// 82B981F0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B981F4: 4B663F85  bl 0x821fc178
	ctx.lr = 0x82B981F8;
	sub_821FC178(ctx, base);
	// 82B981F8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B981FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B98200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B98204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B98208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98210 size=52
    let mut pc: u32 = 0x82B98210;
    'dispatch: loop {
        match pc {
            0x82B98210 => {
    //   block [0x82B98210..0x82B98244)
	// 82B98210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9821C: 810330A8  lwz r8, 0x30a8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82B98220: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82B98224: 419A000C  beq cr6, 0x82b98230
	if ctx.cr[6].eq {
	pc = 0x82B98230; continue 'dispatch;
	}
	// 82B98228: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B9822C: 4B663F4D  bl 0x821fc178
	ctx.lr = 0x82B98230;
	sub_821FC178(ctx, base);
	// 82B98230: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B98234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B98238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9823C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B98240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98248 size=208
    let mut pc: u32 = 0x82B98248;
    'dispatch: loop {
        match pc {
            0x82B98248 => {
    //   block [0x82B98248..0x82B98318)
	// 82B98248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9824C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B98254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B98258: 9421F990  stwu r1, -0x670(r1)
	ea = ctx.r[1].u32.wrapping_add(-1648 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9825C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B98260: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82B98264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B98268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82B9826C: 4800C375  bl 0x82ba45e0
	ctx.lr = 0x82B98270;
	sub_82BA45E0(ctx, base);
	// 82B98270: 3BDF3A9C  addi r30, r31, 0x3a9c
	ctx.r[30].s64 = ctx.r[31].s64 + 15004;
	// 82B98274: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82B98278: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82B9827C: 390B0600  addi r8, r11, 0x600
	ctx.r[8].s64 = ctx.r[11].s64 + 1536;
	// 82B98280: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B98284: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B98288: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9828C: 40820014  bne 0x82b982a0
	if !ctx.cr[0].eq {
	pc = 0x82B982A0; continue 'dispatch;
	}
	// 82B98290: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B98294: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B98298: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82B9829C: 409AFFE4  bne cr6, 0x82b98280
	if !ctx.cr[6].eq {
	pc = 0x82B98280; continue 'dispatch;
	}
	// 82B982A0: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B982A4: 4182005C  beq 0x82b98300
	if ctx.cr[0].eq {
	pc = 0x82B98300; continue 'dispatch;
	}
	// 82B982A8: 4872169D  bl 0x832b9944
	ctx.lr = 0x82B982AC;
	// extern call 0x832B9944 → crate::xboxkrnl::KeGetCurrentProcessType
	crate::xboxkrnl::KeGetCurrentProcessType(ctx, base);
	// 82B982AC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82B982B0: 419A0034  beq cr6, 0x82b982e4
	if ctx.cr[6].eq {
	pc = 0x82B982E4; continue 'dispatch;
	}
	// 82B982B4: 807F5D90  lwz r3, 0x5d90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23952 as u32) ) } as u64;
	// 82B982B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B982BC: 419A001C  beq cr6, 0x82b982d8
	if ctx.cr[6].eq {
	pc = 0x82B982D8; continue 'dispatch;
	}
	// 82B982C0: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82B982C4: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B982C8: 40820010  bne 0x82b982d8
	if !ctx.cr[0].eq {
	pc = 0x82B982D8; continue 'dispatch;
	}
	// 82B982CC: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82B982D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B982D4: 4B6701C5  bl 0x82208498
	ctx.lr = 0x82B982D8;
	sub_82208498(ctx, base);
	// 82B982D8: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82B982DC: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82B982E0: 917F5D94  stw r11, 0x5d94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(23956 as u32), ctx.r[11].u32 ) };
	// 82B982E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B982E8: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82B982EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B982F0: 48111191  bl 0x82ca9480
	ctx.lr = 0x82B982F4;
	sub_82CA9480(ctx, base);
	// 82B982F4: 817F4144  lwz r11, 0x4144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16708 as u32) ) } as u64;
	// 82B982F8: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82B982FC: 917F4144  stw r11, 0x4144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16708 as u32), ctx.r[11].u32 ) };
	// 82B98300: 38210670  addi r1, r1, 0x670
	ctx.r[1].s64 = ctx.r[1].s64 + 1648;
	// 82B98304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B98308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9830C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B98310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B98314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98318 size=208
    let mut pc: u32 = 0x82B98318;
    'dispatch: loop {
        match pc {
            0x82B98318 => {
    //   block [0x82B98318..0x82B983E8)
	// 82B98318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B98324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B98328: 9421F990  stwu r1, -0x670(r1)
	ea = ctx.r[1].u32.wrapping_add(-1648 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9832C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B98330: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82B98334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B98338: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82B9833C: 4800C33D  bl 0x82ba4678
	ctx.lr = 0x82B98340;
	sub_82BA4678(ctx, base);
	// 82B98340: 3BDF3A9C  addi r30, r31, 0x3a9c
	ctx.r[30].s64 = ctx.r[31].s64 + 15004;
	// 82B98344: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82B98348: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82B9834C: 390B0600  addi r8, r11, 0x600
	ctx.r[8].s64 = ctx.r[11].s64 + 1536;
	// 82B98350: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B98354: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B98358: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9835C: 40820014  bne 0x82b98370
	if !ctx.cr[0].eq {
	pc = 0x82B98370; continue 'dispatch;
	}
	// 82B98360: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B98364: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B98368: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82B9836C: 409AFFE4  bne cr6, 0x82b98350
	if !ctx.cr[6].eq {
	pc = 0x82B98350; continue 'dispatch;
	}
	// 82B98370: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B98374: 4182005C  beq 0x82b983d0
	if ctx.cr[0].eq {
	pc = 0x82B983D0; continue 'dispatch;
	}
	// 82B98378: 487215CD  bl 0x832b9944
	ctx.lr = 0x82B9837C;
	// extern call 0x832B9944 → crate::xboxkrnl::KeGetCurrentProcessType
	crate::xboxkrnl::KeGetCurrentProcessType(ctx, base);
	// 82B9837C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82B98380: 419A0034  beq cr6, 0x82b983b4
	if ctx.cr[6].eq {
	pc = 0x82B983B4; continue 'dispatch;
	}
	// 82B98384: 807F5D90  lwz r3, 0x5d90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23952 as u32) ) } as u64;
	// 82B98388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9838C: 419A001C  beq cr6, 0x82b983a8
	if ctx.cr[6].eq {
	pc = 0x82B983A8; continue 'dispatch;
	}
	// 82B98390: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82B98394: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B98398: 40820010  bne 0x82b983a8
	if !ctx.cr[0].eq {
	pc = 0x82B983A8; continue 'dispatch;
	}
	// 82B9839C: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82B983A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B983A4: 4B6700F5  bl 0x82208498
	ctx.lr = 0x82B983A8;
	sub_82208498(ctx, base);
	// 82B983A8: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82B983AC: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82B983B0: 917F5D94  stw r11, 0x5d94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(23956 as u32), ctx.r[11].u32 ) };
	// 82B983B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B983B8: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82B983BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B983C0: 481110C1  bl 0x82ca9480
	ctx.lr = 0x82B983C4;
	sub_82CA9480(ctx, base);
	// 82B983C4: 817F4144  lwz r11, 0x4144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16708 as u32) ) } as u64;
	// 82B983C8: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82B983CC: 917F4144  stw r11, 0x4144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16708 as u32), ctx.r[11].u32 ) };
	// 82B983D0: 38210670  addi r1, r1, 0x670
	ctx.r[1].s64 = ctx.r[1].s64 + 1648;
	// 82B983D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B983D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B983DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B983E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B983E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B983E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B983E8 size=44
    let mut pc: u32 = 0x82B983E8;
    'dispatch: loop {
        match pc {
            0x82B983E8 => {
    //   block [0x82B983E8..0x82B98414)
	// 82B983E8: 54C9083C  slwi r9, r6, 1
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B983EC: 7D092850  subf r8, r9, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 82B983F0: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82B983F4: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82B983F8: 41990008  bgt cr6, 0x82b98400
	if ctx.cr[6].gt {
	pc = 0x82B98400; continue 'dispatch;
	}
	// 82B983FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B98400: 7CE92050  subf r7, r9, r4
	ctx.r[7].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82B98404: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B98408: 4099000C  ble cr6, 0x82b98414
	if !ctx.cr[6].gt {
		sub_82B98414(ctx, base);
		return;
	}
	// 82B9840C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82B98410: 48000014  b 0x82b98424
	sub_82B98414(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98414 size=60
    let mut pc: u32 = 0x82B98414;
    'dispatch: loop {
        match pc {
            0x82B98414 => {
    //   block [0x82B98414..0x82B98450)
	// 82B98414: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82B98418: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82B9841C: 41990008  bgt cr6, 0x82b98424
	if ctx.cr[6].gt {
	pc = 0x82B98424; continue 'dispatch;
	}
	// 82B98420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B98424: 7D691850  subf r11, r9, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[9].s64;
	// 82B98428: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9842C: 41990034  bgt cr6, 0x82b98460
	if ctx.cr[6].gt {
		sub_82B98450(ctx, base);
		return;
	}
	// 82B98430: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82B98434: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82B98438: 41990008  bgt cr6, 0x82b98440
	if ctx.cr[6].gt {
	pc = 0x82B98440; continue 'dispatch;
	}
	// 82B9843C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B98440: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B98444: 4099000C  ble cr6, 0x82b98450
	if !ctx.cr[6].gt {
		sub_82B98450(ctx, base);
		return;
	}
	// 82B98448: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82B9844C: 48000014  b 0x82b98460
	sub_82B98450(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98450 size=28
    let mut pc: u32 = 0x82B98450;
    'dispatch: loop {
        match pc {
            0x82B98450 => {
    //   block [0x82B98450..0x82B9846C)
	// 82B98450: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82B98454: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82B98458: 41990008  bgt cr6, 0x82b98460
	if ctx.cr[6].gt {
	pc = 0x82B98460; continue 'dispatch;
	}
	// 82B9845C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B98460: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B98464: 206B0020  subfic r3, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[3].s64 = (32 as i64) - ctx.r[11].s64;
	// 82B98468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82B98470 size=344
    let mut pc: u32 = 0x82B98470;
    'dispatch: loop {
        match pc {
            0x82B98470 => {
    //   block [0x82B98470..0x82B985C8)
	// 82B98470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98474: 48110F85  bl 0x82ca93f8
	ctx.lr = 0x82B98478;
	sub_82CA93D0(ctx, base);
	// 82B98478: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9847C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82B98480: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82B98484: 394B6F08  addi r10, r11, 0x6f08
	ctx.r[10].s64 = ctx.r[11].s64 + 28424;
	// 82B98488: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82B9848C: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 82B98490: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82B98494: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B98498: 5527083C  slwi r7, r9, 1
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9849C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B984A0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82B984A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B984A8: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B984AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B984B0: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B984B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82B984B8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82B984BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B984C0: 7F0750AE  lbzx r24, r7, r10
	ctx.r[24].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82B984C4: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82B984C8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82B984CC: 4B662C5D  bl 0x821fb128
	ctx.lr = 0x82B984D0;
	sub_821FB128(ctx, base);
	// 82B984D0: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82B984D4: 395DFFFF  addi r10, r29, -1
	ctx.r[10].s64 = ctx.r[29].s64 + -1;
	// 82B984D8: 393CFFFF  addi r9, r28, -1
	ctx.r[9].s64 = ctx.r[28].s64 + -1;
	// 82B984DC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B984E0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82B984E4: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82B984E8: 216B0020  subfic r11, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[11].s64;
	// 82B984EC: 214A0020  subfic r10, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[10].s64;
	// 82B984F0: 21090020  subfic r8, r9, 0x20
	ctx.xer.ca = ctx.r[9].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[9].s64;
	// 82B984F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B984F8: 7CEB5010  subfc r7, r11, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[11].u32;
	ctx.r[7].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82B984FC: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B98500: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82B98504: 7D2B5030  slw r11, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B98508: 7CE73910  subfe r7, r7, r7
	let x = (!ctx.r[7].u32);
	let y = ctx.r[7].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[7].u32 = res;
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B9850C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82B98510: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 82B98514: 54EB07FE  clrlwi r11, r7, 0x1f
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 82B98518: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9851C: 40980070  bge cr6, 0x82b9858c
	if !ctx.cr[6].lt {
	pc = 0x82B9858C; continue 'dispatch;
	}
	// 82B98520: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82B98524: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82B98528: 7D4AFE30  sraw r10, r10, r31
	tmp.u32 = ctx.r[31].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> tmp.u32) as i64;
	// 82B9852C: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82B98530: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82B98534: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B98538: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9853C: 7CEAC1D6  mullw r7, r10, r24
	ctx.r[7].s64 = (ctx.r[10].s32 as i64) * (ctx.r[24].s32 as i64);
	// 82B98540: 81010064  lwz r8, 0x64(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B98544: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B98548: 80810114  lwz r4, 0x114(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 82B9854C: 8061011C  lwz r3, 0x11c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 82B98550: 7CE749D6  mullw r7, r7, r9
	ctx.r[7].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82B98554: 0CCA0000  twi 6, r10, 0
	// 82B98558: 7D495396  divwu r10, r9, r10
	ctx.r[10].u32 = ctx.r[9].u32 / ctx.r[10].u32;
	// 82B9855C: 54E6E8FE  srwi r6, r7, 3
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B98560: 7CE8D9D6  mullw r7, r8, r27
	ctx.r[7].s64 = (ctx.r[8].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82B98564: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B98568: 7D082B96  divwu r8, r8, r5
	ctx.r[8].u32 = ctx.r[8].u32 / ctx.r[5].u32;
	// 82B9856C: 7D263A14  add r9, r6, r7
	ctx.r[9].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 82B98570: 7D4BD1D6  mullw r10, r11, r26
	ctx.r[10].s64 = (ctx.r[11].s32 as i64) * (ctx.r[26].s32 as i64);
	// 82B98574: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B98578: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9857C: 0CC50000  twi 6, r5, 0
	// 82B98580: 7C695214  add r3, r9, r10
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B98584: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82B98588: 48110EC0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82B9858C: 696B0004  xori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 ^ 4;
	// 82B98590: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 82B98594: 38FFFFFE  addi r7, r31, -2
	ctx.r[7].s64 = ctx.r[31].s64 + -2;
	// 82B98598: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82B9859C: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82B985A0: 7D4A3C30  srw r10, r10, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82B985A4: 7D4B312E  stwx r10, r11, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32), ctx.r[10].u32) };
	// 82B985A8: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82B985AC: 4098FF84  bge cr6, 0x82b98530
	if !ctx.cr[6].lt {
	pc = 0x82B98530; continue 'dispatch;
	}
	// 82B985B0: 7D7F4050  subf r11, r31, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[31].s64;
	// 82B985B4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82B985B8: 41990008  bgt cr6, 0x82b985c0
	if ctx.cr[6].gt {
	pc = 0x82B985C0; continue 'dispatch;
	}
	// 82B985BC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82B985C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B985C4: 4BFFFF70  b 0x82b98534
	pc = 0x82B98534; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B985C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B985C8 size=256
    let mut pc: u32 = 0x82B985C8;
    'dispatch: loop {
        match pc {
            0x82B985C8 => {
    //   block [0x82B985C8..0x82B986C8)
	// 82B985C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B985CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B985D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B985D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B985D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B985DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82B985E0: 3921005C  addi r9, r1, 0x5c
	ctx.r[9].s64 = ctx.r[1].s64 + 92;
	// 82B985E4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82B985E8: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82B985EC: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82B985F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82B985F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B985F8: 4B662769  bl 0x821fad60
	ctx.lr = 0x82B985FC;
	sub_821FAD60(ctx, base);
	// 82B985FC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82B98600: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B98604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B98608: 5567CF7E  rlwinm r7, r11, 0x19, 0x1d, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82B9860C: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82B98610: 5569CEB8  rlwinm r9, r11, 0x19, 0x1a, 0x1c
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82B98614: 5566E77E  rlwinm r6, r11, 0x1c, 0x1d, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82B98618: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9861C: 5567FF7E  rlwinm r7, r11, 0x1f, 0x1d, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82B98620: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B98624: 5545D7BE  rlwinm r5, r10, 0x1a, 0x1e, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82B98628: 7D293378  or r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[6].u64;
	// 82B9862C: 5546C7BE  rlwinm r6, r10, 0x18, 0x1e, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B98630: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B98634: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B98638: 5547E7BE  rlwinm r7, r10, 0x1c, 0x1e, 0x1f
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82B9863C: 512B083C  rlwimi r11, r9, 1, 0, 0x1e
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(1) as u64) & 0x00000000FFFFFFFE) | (ctx.r[11].u64 & 0xFFFFFFFF00000001);
	// 82B98640: 5549F7BE  rlwinm r9, r10, 0x1e, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82B98644: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98648: 554A0FFE  srwi r10, r10, 0x1f
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9864C: 7D6B3378  or r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 82B98650: 5506D7BE  rlwinm r6, r8, 0x1a, 0x1e, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x0000003Fu64;
	// 82B98654: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98658: 7D6B2B78  or r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[5].u64;
	// 82B9865C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98660: 7D6B3B78  or r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 82B98664: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98668: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82B9866C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98670: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B98674: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98678: 7D6B3378  or r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 82B9867C: 51683032  rlwimi r8, r11, 6, 0, 0x19
	ctx.r[8].u64 = (((ctx.r[11].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[8].u64 & 0xFFFFFFFF0000003F);
	// 82B98680: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B98684: 4B66264D  bl 0x821facd0
	ctx.lr = 0x82B98688;
	sub_821FACD0(ctx, base);
	// 82B98688: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9868C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B98690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B98694: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B98698: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82B9869C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B986A0: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B986A4: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82B986A8: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82B986AC: 911E0018  stw r8, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82B986B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B986B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B986B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B986BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B986C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B986C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B986C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B986C8 size=172
    let mut pc: u32 = 0x82B986C8;
    'dispatch: loop {
        match pc {
            0x82B986C8 => {
    //   block [0x82B986C8..0x82B98774)
	// 82B986C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B986CC: 48110D41  bl 0x82ca940c
	ctx.lr = 0x82B986D0;
	sub_82CA93D0(ctx, base);
	// 82B986D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B986D4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82B986D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82B986DC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82B986E0: 3961006C  addi r11, r1, 0x6c
	ctx.r[11].s64 = ctx.r[1].s64 + 108;
	// 82B986E4: 39410064  addi r10, r1, 0x64
	ctx.r[10].s64 = ctx.r[1].s64 + 100;
	// 82B986E8: 39210068  addi r9, r1, 0x68
	ctx.r[9].s64 = ctx.r[1].s64 + 104;
	// 82B986EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B986F0: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82B986F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B986F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B986FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B98700: 4B6DFA39  bl 0x82278138
	ctx.lr = 0x82B98704;
	sub_82278138(ctx, base);
	// 82B98704: 81010064  lwz r8, 0x64(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B98708: 80E10068  lwz r7, 0x68(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82B9870C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82B98710: 419A004C  beq cr6, 0x82b9875c
	if ctx.cr[6].eq {
	pc = 0x82B9875C; continue 'dispatch;
	}
	// 82B98714: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82B98718: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82B9871C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B98720: 396B6F08  addi r11, r11, 0x6f08
	ctx.r[11].s64 = ctx.r[11].s64 + 28424;
	// 82B98724: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B98728: 554A0E7C  rlwinm r10, r10, 1, 0x19, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82B9872C: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B98730: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82B98734: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B98738: 7D6639D6  mullw r11, r6, r7
	ctx.r[11].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82B9873C: 7D4A20AE  lbzx r10, r10, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82B98740: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82B98744: 5549E8FE  srwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B98748: 7D4541D6  mullw r10, r5, r8
	ctx.r[10].s64 = (ctx.r[5].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82B9874C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82B98750: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B98754: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B98758: 48000008  b 0x82b98760
	pc = 0x82B98760; continue 'dispatch;
	// 82B9875C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B98760: 90FD0000  stw r7, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82B98764: 911D0004  stw r8, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82B98768: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9876C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B98770: 48110CEC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98778 size=220
    let mut pc: u32 = 0x82B98778;
    'dispatch: loop {
        match pc {
            0x82B98778 => {
    //   block [0x82B98778..0x82B98854)
	// 82B98778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9877C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B98784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B98788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9878C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B98790: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82B98794: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82B98798: 38C1006C  addi r6, r1, 0x6c
	ctx.r[6].s64 = ctx.r[1].s64 + 108;
	// 82B9879C: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82B987A0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82B987A4: 4B662395  bl 0x821fab38
	ctx.lr = 0x82B987A8;
	sub_821FAB38(ctx, base);
	// 82B987A8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82B987AC: 55690FFE  srwi r9, r11, 0x1f
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B987B0: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B987B4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B987B8: 81010064  lwz r8, 0x64(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B987BC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82B987C0: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82B987C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82B987C8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82B987CC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B987D0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82B987D4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82B987D8: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82B987DC: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82B987E0: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82B987E4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82B987E8: 41980008  blt cr6, 0x82b987f0
	if ctx.cr[6].lt {
	pc = 0x82B987F0; continue 'dispatch;
	}
	// 82B987EC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B987F0: 34CBFFFC  addic. r6, r11, -4
	ctx.xer.ca = (ctx.r[11].u32 > (!(-4 as u32)));
	ctx.r[6].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82B987F4: 41810008  bgt 0x82b987fc
	if ctx.cr[0].gt {
	pc = 0x82B987FC; continue 'dispatch;
	}
	// 82B987F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B987FC: 39610064  addi r11, r1, 0x64
	ctx.r[11].s64 = ctx.r[1].s64 + 100;
	// 82B98800: 39410068  addi r10, r1, 0x68
	ctx.r[10].s64 = ctx.r[1].s64 + 104;
	// 82B98804: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82B98808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9880C: 3901006C  addi r8, r1, 0x6c
	ctx.r[8].s64 = ctx.r[1].s64 + 108;
	// 82B98810: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82B98814: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B98818: 4B6DF921  bl 0x82278138
	ctx.lr = 0x82B9881C;
	sub_82278138(ctx, base);
	// 82B9881C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B98820: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B98824: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82B98828: 8101006C  lwz r8, 0x6c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9882C: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82B98830: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B98834: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82B98838: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9883C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B98840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B98844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B98848: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9884C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B98850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98858 size=4
    let mut pc: u32 = 0x82B98858;
    'dispatch: loop {
        match pc {
            0x82B98858 => {
    //   block [0x82B98858..0x82B9885C)
	// 82B98858: 4B662378  b 0x821fabd0
	sub_821FABD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98860 size=208
    let mut pc: u32 = 0x82B98860;
    'dispatch: loop {
        match pc {
            0x82B98860 => {
    //   block [0x82B98860..0x82B98930)
	// 82B98860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B9886C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B98870: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B98874: 39210074  addi r9, r1, 0x74
	ctx.r[9].s64 = ctx.r[1].s64 + 116;
	// 82B98878: 39010078  addi r8, r1, 0x78
	ctx.r[8].s64 = ctx.r[1].s64 + 120;
	// 82B9887C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B98880: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82B98884: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82B98888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82B9888C: 3941007C  addi r10, r1, 0x7c
	ctx.r[10].s64 = ctx.r[1].s64 + 124;
	// 82B98890: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82B98894: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82B98898: 39010084  addi r8, r1, 0x84
	ctx.r[8].s64 = ctx.r[1].s64 + 132;
	// 82B9889C: 38E10088  addi r7, r1, 0x88
	ctx.r[7].s64 = ctx.r[1].s64 + 136;
	// 82B988A0: 38C1008C  addi r6, r1, 0x8c
	ctx.r[6].s64 = ctx.r[1].s64 + 140;
	// 82B988A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82B988A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B988AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B988B0: 4B661E11  bl 0x821fa6c0
	ctx.lr = 0x82B988B4;
	sub_821FA6C0(ctx, base);
	// 82B988B4: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B988B8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82B988BC: 4B694115  bl 0x8222c9d0
	ctx.lr = 0x82B988C0;
	sub_8222C9D0(ctx, base);
	// 82B988C0: 7C681B79  or. r8, r3, r3
	ctx.r[8].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B988C4: 4082000C  bne 0x82b988d0
	if !ctx.cr[0].eq {
	pc = 0x82B988D0; continue 'dispatch;
	}
	// 82B988C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B988CC: 4800004C  b 0x82b98918
	pc = 0x82B98918; continue 'dispatch;
	// 82B988D0: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B988D4: 38E04401  li r7, 0x4401
	ctx.r[7].s64 = 17409;
	// 82B988D8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B988DC: 57CAE006  slwi r10, r30, 0x1c
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(28);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B988E0: 556B02BE  clrlwi r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x003FFFFFu64;
	// 82B988E4: 93E80018  stw r31, 0x18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82B988E8: 50E9A2D2  rlwimi r9, r7, 0x14, 0xb, 9
	ctx.r[9].u64 = (((ctx.r[7].u32).rotate_left(20) as u64) & 0xFFFFFFFFFFDFFFFF) | (ctx.r[9].u64 & 0x0000000000200000);
	// 82B988EC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B988F0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B988F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B988F8: 3D20FFFF  lis r9, -1
	ctx.r[9].s64 = -65536;
	// 82B988FC: 556B0286  rlwinm r11, r11, 0, 0xa, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B98900: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82B98904: 91280014  stw r9, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82B98908: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9890C: 9168001C  stw r11, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82B98910: 4B663869  bl 0x821fc178
	ctx.lr = 0x82B98914;
	sub_821FC178(ctx, base);
	// 82B98914: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B98918: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82B9891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B98920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B98924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B98928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9892C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98930 size=28
    let mut pc: u32 = 0x82B98930;
    'dispatch: loop {
        match pc {
            0x82B98930 => {
    //   block [0x82B98930..0x82B9894C)
	// 82B98930: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82B98934: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82B98938: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82B9893C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82B98940: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B98944: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B98948: 4B612B98  b 0x821ab4e0
	sub_821AB4E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98950 size=20
    let mut pc: u32 = 0x82B98950;
    'dispatch: loop {
        match pc {
            0x82B98950 => {
    //   block [0x82B98950..0x82B98964)
	// 82B98950: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82B98954: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82B98958: 55650026  rlwinm r5, r11, 0, 0, 0x13
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9895C: 55440026  rlwinm r4, r10, 0, 0, 0x13
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B98960: 4B729CD0  b 0x822c2630
	sub_822C2630(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98968 size=16
    let mut pc: u32 = 0x82B98968;
    'dispatch: loop {
        match pc {
            0x82B98968 => {
    //   block [0x82B98968..0x82B98978)
	// 82B98968: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82B9896C: 556B36BE  srwi r11, r11, 0x1a
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(26);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98970: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82B98974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98978 size=180
    let mut pc: u32 = 0x82B98978;
    'dispatch: loop {
        match pc {
            0x82B98978 => {
    //   block [0x82B98978..0x82B98A2C)
	// 82B98978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9897C: 48110A91  bl 0x82ca940c
	ctx.lr = 0x82B98980;
	sub_82CA93D0(ctx, base);
	// 82B98980: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B98984: 39210074  addi r9, r1, 0x74
	ctx.r[9].s64 = ctx.r[1].s64 + 116;
	// 82B98988: 39010078  addi r8, r1, 0x78
	ctx.r[8].s64 = ctx.r[1].s64 + 120;
	// 82B9898C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82B98990: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82B98994: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82B98998: 3941007C  addi r10, r1, 0x7c
	ctx.r[10].s64 = ctx.r[1].s64 + 124;
	// 82B9899C: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82B989A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82B989A4: 39010084  addi r8, r1, 0x84
	ctx.r[8].s64 = ctx.r[1].s64 + 132;
	// 82B989A8: 38E10088  addi r7, r1, 0x88
	ctx.r[7].s64 = ctx.r[1].s64 + 136;
	// 82B989AC: 38C1008C  addi r6, r1, 0x8c
	ctx.r[6].s64 = ctx.r[1].s64 + 140;
	// 82B989B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B989B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B989B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82B989BC: 4B661D05  bl 0x821fa6c0
	ctx.lr = 0x82B989C0;
	sub_821FA6C0(ctx, base);
	// 82B989C0: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B989C4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82B989C8: 4B694009  bl 0x8222c9d0
	ctx.lr = 0x82B989CC;
	sub_8222C9D0(ctx, base);
	// 82B989CC: 7C681B79  or. r8, r3, r3
	ctx.r[8].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B989D0: 4082000C  bne 0x82b989dc
	if !ctx.cr[0].eq {
	pc = 0x82B989DC; continue 'dispatch;
	}
	// 82B989D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B989D8: 4800004C  b 0x82b98a24
	pc = 0x82B98A24; continue 'dispatch;
	// 82B989DC: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B989E0: 38E04401  li r7, 0x4401
	ctx.r[7].s64 = 17409;
	// 82B989E4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B989E8: 57AAE006  slwi r10, r29, 0x1c
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(28);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B989EC: 556B02BE  clrlwi r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x003FFFFFu64;
	// 82B989F0: 93E80018  stw r31, 0x18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82B989F4: 50E9A2D2  rlwimi r9, r7, 0x14, 0xb, 9
	ctx.r[9].u64 = (((ctx.r[7].u32).rotate_left(20) as u64) & 0xFFFFFFFFFFDFFFFF) | (ctx.r[9].u64 & 0x0000000000200000);
	// 82B989F8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B989FC: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B98A00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B98A04: 3D20FFFF  lis r9, -1
	ctx.r[9].s64 = -65536;
	// 82B98A08: 53CBB112  rlwimi r11, r30, 0x16, 4, 9
	ctx.r[11].u64 = (((ctx.r[30].u32).rotate_left(22) as u64) & 0x000000000FC00000) | (ctx.r[11].u64 & 0xFFFFFFFFF03FFFFF);
	// 82B98A0C: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82B98A10: 91280014  stw r9, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82B98A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B98A18: 9168001C  stw r11, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82B98A1C: 4B66375D  bl 0x821fc178
	ctx.lr = 0x82B98A20;
	sub_821FC178(ctx, base);
	// 82B98A20: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B98A24: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82B98A28: 48110A34  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98A30 size=28
    let mut pc: u32 = 0x82B98A30;
    'dispatch: loop {
        match pc {
            0x82B98A30 => {
    //   block [0x82B98A30..0x82B98A4C)
	// 82B98A30: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82B98A34: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82B98A38: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82B98A3C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82B98A40: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82B98A44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B98A48: 4B612A98  b 0x821ab4e0
	sub_821AB4E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98A50 size=4
    let mut pc: u32 = 0x82B98A50;
    'dispatch: loop {
        match pc {
            0x82B98A50 => {
    //   block [0x82B98A50..0x82B98A54)
	// 82B98A50: 4BFFFB78  b 0x82b985c8
	sub_82B985C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98A58 size=196
    let mut pc: u32 = 0x82B98A58;
    'dispatch: loop {
        match pc {
            0x82B98A58 => {
    //   block [0x82B98A58..0x82B98B1C)
	// 82B98A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98A60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B98A64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B98A68: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B98A6C: 39210074  addi r9, r1, 0x74
	ctx.r[9].s64 = ctx.r[1].s64 + 116;
	// 82B98A70: 39010078  addi r8, r1, 0x78
	ctx.r[8].s64 = ctx.r[1].s64 + 120;
	// 82B98A74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B98A78: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82B98A7C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82B98A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82B98A84: 3941007C  addi r10, r1, 0x7c
	ctx.r[10].s64 = ctx.r[1].s64 + 124;
	// 82B98A88: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82B98A8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82B98A90: 39010084  addi r8, r1, 0x84
	ctx.r[8].s64 = ctx.r[1].s64 + 132;
	// 82B98A94: 38E10088  addi r7, r1, 0x88
	ctx.r[7].s64 = ctx.r[1].s64 + 136;
	// 82B98A98: 38C1008C  addi r6, r1, 0x8c
	ctx.r[6].s64 = ctx.r[1].s64 + 140;
	// 82B98A9C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82B98AA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B98AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B98AA8: 4B661C19  bl 0x821fa6c0
	ctx.lr = 0x82B98AAC;
	sub_821FA6C0(ctx, base);
	// 82B98AAC: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B98AB0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82B98AB4: 4B693F1D  bl 0x8222c9d0
	ctx.lr = 0x82B98AB8;
	sub_8222C9D0(ctx, base);
	// 82B98AB8: 7C681B79  or. r8, r3, r3
	ctx.r[8].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B98ABC: 4082000C  bne 0x82b98ac8
	if !ctx.cr[0].eq {
	pc = 0x82B98AC8; continue 'dispatch;
	}
	// 82B98AC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B98AC4: 48000040  b 0x82b98b04
	pc = 0x82B98B04; continue 'dispatch;
	// 82B98AC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B98ACC: 39404401  li r10, 0x4401
	ctx.r[10].s64 = 17409;
	// 82B98AD0: 8128001C  lwz r9, 0x1c(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B98AD4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82B98AD8: 514BA2D2  rlwimi r11, r10, 0x14, 0xb, 9
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(20) as u64) & 0xFFFFFFFFFFDFFFFF) | (ctx.r[11].u64 & 0x0000000000200000);
	// 82B98ADC: 93E80018  stw r31, 0x18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82B98AE0: 3D40FFFF  lis r10, -1
	ctx.r[10].s64 = -65536;
	// 82B98AE4: 90E80004  stw r7, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82B98AE8: 53C9E006  rlwimi r9, r30, 0x1c, 0, 3
	ctx.r[9].u64 = (((ctx.r[30].u32).rotate_left(28) as u64) & 0x00000000F0000000) | (ctx.r[9].u64 & 0xFFFFFFFF0FFFFFFF);
	// 82B98AEC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B98AF0: 91480014  stw r10, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82B98AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B98AF8: 9128001C  stw r9, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82B98AFC: 4B66367D  bl 0x821fc178
	ctx.lr = 0x82B98B00;
	sub_821FC178(ctx, base);
	// 82B98B00: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B98B04: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82B98B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B98B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B98B10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B98B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B98B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98B20 size=4
    let mut pc: u32 = 0x82B98B20;
    'dispatch: loop {
        match pc {
            0x82B98B20 => {
    //   block [0x82B98B20..0x82B98B24)
	// 82B98B20: 4BFFFBA8  b 0x82b986c8
	sub_82B986C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98B28 size=288
    let mut pc: u32 = 0x82B98B28;
    'dispatch: loop {
        match pc {
            0x82B98B28 => {
    //   block [0x82B98B28..0x82B98C48)
	// 82B98B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98B2C: 481108C5  bl 0x82ca93f0
	ctx.lr = 0x82B98B30;
	sub_82CA93D0(ctx, base);
	// 82B98B30: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B98B34: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82B98B38: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82B98B3C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B98B40: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B98B44: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82B98B48: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82B98B4C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82B98B50: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82B98B54: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82B98B58: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 82B98B5C: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
	// 82B98B60: 4B693E71  bl 0x8222c9d0
	ctx.lr = 0x82B98B64;
	sub_8222C9D0(ctx, base);
	// 82B98B64: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82B98B68: 4082000C  bne 0x82b98b74
	if !ctx.cr[0].eq {
	pc = 0x82B98B74; continue 'dispatch;
	}
	// 82B98B6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B98B70: 480000D0  b 0x82b98c40
	pc = 0x82B98C40; continue 'dispatch;
	// 82B98B74: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82B98B78: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82B98B7C: 39610084  addi r11, r1, 0x84
	ctx.r[11].s64 = ctx.r[1].s64 + 132;
	// 82B98B80: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82B98B84: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82B98B88: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82B98B8C: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82B98B90: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82B98B94: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82B98B98: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82B98B9C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82B98BA0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82B98BA4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82B98BA8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82B98BAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82B98BB0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82B98BB4: 4B662A9D  bl 0x821fb650
	ctx.lr = 0x82B98BB8;
	sub_821FB650(ctx, base);
	// 82B98BB8: 7FABE8F8  nor r11, r29, r29
	ctx.r[11].u64 = !(ctx.r[29].u64 | ctx.r[29].u64);
	// 82B98BBC: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82B98BC0: 556BF7FE  rlwinm r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B98BC4: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82B98BC8: 556BE006  slwi r11, r11, 0x1c
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(28);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B98BCC: 657E8C80  oris r30, r11, 0x8c80
	ctx.r[30].u64 = ctx.r[11].u64 | 2357198848;
	// 82B98BD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B98BD4: 4B693DFD  bl 0x8222c9d0
	ctx.lr = 0x82B98BD8;
	sub_8222C9D0(ctx, base);
	// 82B98BD8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B98BDC: 40820014  bne 0x82b98bf0
	if !ctx.cr[0].eq {
	pc = 0x82B98BF0; continue 'dispatch;
	}
	// 82B98BE0: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B98BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B98BE8: 4B6696D1  bl 0x822022b8
	ctx.lr = 0x82B98BEC;
	sub_822022B8(ctx, base);
	// 82B98BEC: 4BFFFF80  b 0x82b98b6c
	pc = 0x82B98B6C; continue 'dispatch;
	// 82B98BF0: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82B98BF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B98BF8: 419A002C  beq cr6, 0x82b98c24
	if ctx.cr[6].eq {
	pc = 0x82B98C24; continue 'dispatch;
	}
	// 82B98BFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B98C00: 4B693DD1  bl 0x8222c9d0
	ctx.lr = 0x82B98C04;
	sub_8222C9D0(ctx, base);
	// 82B98C04: 7C761B79  or. r22, r3, r3
	ctx.r[22].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82B98C08: 4082001C  bne 0x82b98c24
	if !ctx.cr[0].eq {
	pc = 0x82B98C24; continue 'dispatch;
	}
	// 82B98C0C: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B98C10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B98C14: 4B6696A5  bl 0x822022b8
	ctx.lr = 0x82B98C18;
	sub_822022B8(ctx, base);
	// 82B98C18: 3C80B180  lis r4, -0x4e80
	ctx.r[4].s64 = -1317011456;
	// 82B98C1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B98C20: 4BFFFFC8  b 0x82b98be8
	pc = 0x82B98BE8; continue 'dispatch;
	// 82B98C24: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82B98C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B98C2C: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82B98C30: 53AB0026  rlwimi r11, r29, 0, 0, 0x13
	ctx.r[11].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[11].u64 & 0xFFFFFFFF00000FFF);
	// 82B98C34: 5156053E  rlwimi r22, r10, 0, 0x14, 0x1f
	ctx.r[22].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x0000000000000FFF) | (ctx.r[22].u64 & 0xFFFFFFFFFFFFF000);
	// 82B98C38: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82B98C3C: 92DF0030  stw r22, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[22].u32 ) };
	// 82B98C40: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82B98C44: 481107FC  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98C48 size=36
    let mut pc: u32 = 0x82B98C48;
    'dispatch: loop {
        match pc {
            0x82B98C48 => {
    //   block [0x82B98C48..0x82B98C6C)
	// 82B98C48: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B98C4C: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82B98C50: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82B98C54: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B98C58: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 82B98C5C: 5566273E  srwi r6, r11, 0x1c
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B98C60: 556556BE  rlwinm r5, r11, 0xa, 0x1a, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x003FFFFFu64;
	// 82B98C64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B98C68: 4B612878  b 0x821ab4e0
	sub_821AB4E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98C70 size=24
    let mut pc: u32 = 0x82B98C70;
    'dispatch: loop {
        match pc {
            0x82B98C70 => {
    //   block [0x82B98C70..0x82B98C88)
	// 82B98C70: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B98C74: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82B98C78: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82B98C7C: 55650026  rlwinm r5, r11, 0, 0, 0x13
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B98C80: 55440026  rlwinm r4, r10, 0, 0, 0x13
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B98C84: 4B7299AC  b 0x822c2630
	sub_822C2630(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98C88 size=52
    let mut pc: u32 = 0x82B98C88;
    'dispatch: loop {
        match pc {
            0x82B98C88 => {
    //   block [0x82B98C88..0x82B98CBC)
	// 82B98C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98C90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B98C94: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B98C98: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82B98C9C: 419A000C  beq cr6, 0x82b98ca8
	if ctx.cr[6].eq {
	pc = 0x82B98CA8; continue 'dispatch;
	}
	// 82B98CA0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B98CA4: 4B6634D5  bl 0x821fc178
	ctx.lr = 0x82B98CA8;
	sub_821FC178(ctx, base);
	// 82B98CA8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82B98CAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B98CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B98CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B98CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98CC0 size=68
    let mut pc: u32 = 0x82B98CC0;
    'dispatch: loop {
        match pc {
            0x82B98CC0 => {
    //   block [0x82B98CC0..0x82B98D04)
	// 82B98CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B98CC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B98CCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B98CD0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B98CD4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82B98CD8: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B98CDC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82B98CE0: 5564273E  srwi r4, r11, 0x1c
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82B98CE4: 4BFFF8E5  bl 0x82b985c8
	ctx.lr = 0x82B98CE8;
	sub_82B985C8(ctx, base);
	// 82B98CE8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82B98CEC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B98CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B98CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B98CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B98CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B98D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98D08 size=28
    let mut pc: u32 = 0x82B98D08;
    'dispatch: loop {
        match pc {
            0x82B98D08 => {
    //   block [0x82B98D08..0x82B98D24)
	// 82B98D08: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B98D0C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82B98D10: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82B98D14: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B98D18: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82B98D1C: 5564273E  srwi r4, r11, 0x1c
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82B98D20: 4BFFF9A8  b 0x82b986c8
	sub_82B986C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B98D28 size=472
    let mut pc: u32 = 0x82B98D28;
    'dispatch: loop {
        match pc {
            0x82B98D28 => {
    //   block [0x82B98D28..0x82B98F00)
	// 82B98D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B98D2C: 481106B1  bl 0x82ca93dc
	ctx.lr = 0x82B98D30;
	sub_82CA93D0(ctx, base);
	// 82B98D30: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B98D34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82B98D38: 3D40C000  lis r10, -0x4000
	ctx.r[10].s64 = -1073741824;
	// 82B98D3C: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 82B98D40: 614A3B00  ori r10, r10, 0x3b00
	ctx.r[10].u64 = ctx.r[10].u64 | 15104;
	// 82B98D44: 39200300  li r9, 0x300
	ctx.r[9].s64 = 768;
	// 82B98D48: 3D00C019  lis r8, -0x3fe7
	ctx.r[8].s64 = -1072103424;
	// 82B98D4C: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 82B98D50: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B98D54: 61082B00  ori r8, r8, 0x2b00
	ctx.r[8].u64 = ctx.r[8].u64 | 11008;
	// 82B98D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B98D5C: 3CC08209  lis r6, -0x7df7
	ctx.r[6].s64 = -2113339392;
	// 82B98D60: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82B98D64: 3BC65EE0  addi r30, r6, 0x5ee0
	ctx.r[30].s64 = ctx.r[6].s64 + 24288;
	// 82B98D68: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82B98D6C: 389EFFA0  addi r4, r30, -0x60
	ctx.r[4].s64 = ctx.r[30].s64 + -96;
	// 82B98D70: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82B98D74: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B98D78: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82B98D7C: 94FD0004  stwu r7, 4(r29)
	ea = ctx.r[29].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[29].u32 = ea;
	// 82B98D80: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82B98D84: 481106FD  bl 0x82ca9480
	ctx.lr = 0x82B98D88;
	sub_82CA9480(ctx, base);
	// 82B98D88: 397D0060  addi r11, r29, 0x60
	ctx.r[11].s64 = ctx.r[29].s64 + 96;
	// 82B98D8C: 3D40C00A  lis r10, -0x3ff6
	ctx.r[10].s64 = -1073086464;
	// 82B98D90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B98D94: 614A2B00  ori r10, r10, 0x2b00
	ctx.r[10].u64 = ctx.r[10].u64 | 11008;
	// 82B98D98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B98D9C: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B98DA0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82B98DA4: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 82B98DA8: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82B98DAC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82B98DB0: 951E0004  stwu r8, 4(r30)
	ea = ctx.r[30].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[30].u32 = ea;
	// 82B98DB4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82B98DB8: 481106C9  bl 0x82ca9480
	ctx.lr = 0x82B98DBC;
	sub_82CA9480(ctx, base);
	// 82B98DBC: 397E0024  addi r11, r30, 0x24
	ctx.r[11].s64 = ctx.r[30].s64 + 36;
	// 82B98DC0: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82B98DC4: 3D201000  lis r9, 0x1000
	ctx.r[9].s64 = 268435456;
	// 82B98DC8: 614A2180  ori r10, r10, 0x2180
	ctx.r[10].u64 = ctx.r[10].u64 | 8576;
	// 82B98DCC: 6129000E  ori r9, r9, 0xe
	ctx.r[9].u64 = ctx.r[9].u64 | 14;
	// 82B98DD0: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B98DD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B98DD8: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82B98DDC: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 82B98DE0: 61082100  ori r8, r8, 0x2100
	ctx.r[8].u64 = ctx.r[8].u64 | 8448;
	// 82B98DE4: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 82B98DE8: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82B98DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B98DF0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82B98DF4: 38A02293  li r5, 0x2293
	ctx.r[5].s64 = 8851;
	// 82B98DF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B98DFC: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 82B98E00: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E04: 3FC00000  lis r30, 0
	ctx.r[30].s64 = 0;
	// 82B98E08: 60632204  ori r3, r3, 0x2204
	ctx.r[3].u64 = ctx.r[3].u64 | 8708;
	// 82B98E0C: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82B98E10: 3EC00008  lis r22, 8
	ctx.r[22].s64 = 524288;
	// 82B98E14: 3FA00001  lis r29, 1
	ctx.r[29].s64 = 65536;
	// 82B98E18: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E1C: 3B800300  li r28, 0x300
	ctx.r[28].s64 = 768;
	// 82B98E20: 3B602312  li r27, 0x2312
	ctx.r[27].s64 = 8978;
	// 82B98E24: 63DEFFFF  ori r30, r30, 0xffff
	ctx.r[30].u64 = ctx.r[30].u64 | 65535;
	// 82B98E28: 3B40200D  li r26, 0x200d
	ctx.r[26].s64 = 8205;
	// 82B98E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82B98E30: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E34: 3B202200  li r25, 0x2200
	ctx.r[25].s64 = 8704;
	// 82B98E38: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82B98E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82B98E40: 3AE02280  li r23, 0x2280
	ctx.r[23].s64 = 8832;
	// 82B98E44: 3AA02302  li r21, 0x2302
	ctx.r[21].s64 = 8962;
	// 82B98E48: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E4C: 38C02208  li r6, 0x2208
	ctx.r[6].s64 = 8712;
	// 82B98E50: 62D60008  ori r22, r22, 8
	ctx.r[22].u64 = ctx.r[22].u64 | 8;
	// 82B98E54: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E58: 39202203  li r9, 0x2203
	ctx.r[9].s64 = 8707;
	// 82B98E5C: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E60: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B98E64: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E68: 38802104  li r4, 0x2104
	ctx.r[4].s64 = 8452;
	// 82B98E6C: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B98E74: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E78: 97AB0004  stwu r29, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E7C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82B98E80: 3E800002  lis r20, 2
	ctx.r[20].s64 = 131072;
	// 82B98E84: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82B98E88: 62942080  ori r20, r20, 0x2080
	ctx.r[20].u64 = ctx.r[20].u64 | 8320;
	// 82B98E8C: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82B98E90: 978B0004  stwu r28, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[28].u32) };
	ctx.r[11].u32 = ea;
	// 82B98E94: 3E200010  lis r17, 0x10
	ctx.r[17].s64 = 1048576;
	// 82B98E98: 623D0010  ori r29, r17, 0x10
	ctx.r[29].u64 = ctx.r[17].u64 | 16;
	// 82B98E9C: 976B0004  stwu r27, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[27].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EA0: 97CB0004  stwu r30, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EA4: 974B0004  stwu r26, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[26].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EA8: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EAC: 972B0004  stwu r25, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[25].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EB0: 970B0004  stwu r24, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[24].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EB4: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EB8: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EBC: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EC0: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EC4: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EC8: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 82B98ECC: 96EB0004  stwu r23, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[23].u32) };
	ctx.r[11].u32 = ea;
	// 82B98ED0: 96CB0004  stwu r22, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[22].u32) };
	ctx.r[11].u32 = ea;
	// 82B98ED4: 96AB0004  stwu r21, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[21].u32) };
	ctx.r[11].u32 = ea;
	// 82B98ED8: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EDC: 968B0004  stwu r20, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[20].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EE0: 966B0004  stwu r19, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[19].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EE4: 964B0004  stwu r18, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[18].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EE8: 97AB0004  stwu r29, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[11].u32 = ea;
	// 82B98EEC: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82B98EF0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B98EF4: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82B98EF8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82B98EFC: 48110530  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B98F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B98F00 size=280
    let mut pc: u32 = 0x82B98F00;
    'dispatch: loop {
        match pc {
            0x82B98F00 => {
    //   block [0x82B98F00..0x82B99018)
	// 82B98F00: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B98F04: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B98F08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B98F0C: 798C4FE6  rldicr r12, r12, 0x29, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(41) & 0xFFFFFFFFFFFFFFFF;
	// 82B98F10: 7D696378  or r9, r11, r12
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82B98F14: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B98F18: 794B1FE6  rldicr r11, r10, 0x23, 0x3f
	ctx.r[11].u64 = (ctx.r[10].u64).rotate_left(35) & 0xFFFFFFFFFFFFFFFF;
	// 82B98F1C: F9230010  std r9, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u64 ) };
	// 82B98F20: 798C47E6  rldicr r12, r12, 0x28, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(40) & 0xFFFFFFFFFFFFFFFF;
	// 82B98F24: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82B98F28: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B98F2C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B98F30: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98F34: 798C3FE6  rldicr r12, r12, 0x27, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(39) & 0xFFFFFFFFFFFFFFFF;
	// 82B98F38: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B98F3C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B98F40: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98F44: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B98F48: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B98F4C: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82B98F50: 798C67E6  rldicr r12, r12, 0x2c, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(44) & 0xFFFFFFFFFFFFFFFF;
	// 82B98F54: E9430010  ld r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B98F58: 614A0080  ori r10, r10, 0x80
	ctx.r[10].u64 = ctx.r[10].u64 | 128;
	// 82B98F5C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98F60: 614A0040  ori r10, r10, 0x40
	ctx.r[10].u64 = ctx.r[10].u64 | 64;
	// 82B98F64: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98F68: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 82B98F6C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98F70: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B98F74: 654A0008  oris r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 524288;
	// 82B98F78: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82B98F7C: E9430010  ld r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B98F80: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B98F84: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98F88: 654A0008  oris r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 524288;
	// 82B98F8C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B98F90: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98F94: 654A0010  oris r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 1048576;
	// 82B98F98: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 82B98F9C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98FA0: 614A0800  ori r10, r10, 0x800
	ctx.r[10].u64 = ctx.r[10].u64 | 2048;
	// 82B98FA4: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98FA8: 614A0100  ori r10, r10, 0x100
	ctx.r[10].u64 = ctx.r[10].u64 | 256;
	// 82B98FAC: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98FB0: 614A0008  ori r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 8;
	// 82B98FB4: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98FB8: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B98FBC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B98FC0: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B98FC4: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 82B98FC8: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B98FCC: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B98FD0: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82B98FD4: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B98FD8: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B98FDC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B98FE0: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82B98FE4: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82B98FE8: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82B98FEC: 816328C4  lwz r11, 0x28c4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10436 as u32) ) } as u64;
	// 82B98FF0: 814328C8  lwz r10, 0x28c8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10440 as u32) ) } as u64;
	// 82B98FF4: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B98FF8: 554A881C  slwi r10, r10, 0x11
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(17);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B98FFC: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B99000: 7D278E70  srawi r7, r9, 0x11
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 17) as i64;
	// 82B99004: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99008: 7D468E70  srawi r6, r10, 0x11
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[10].s32 >> 17) as i64;
	// 82B9900C: 7D058E70  srawi r5, r8, 0x11
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[8].s32 >> 17) as i64;
	// 82B99010: 7D648E70  srawi r4, r11, 0x11
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 17) as i64;
	// 82B99014: 4B660904  b 0x821f9918
	sub_821F9918(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B99018 size=408
    let mut pc: u32 = 0x82B99018;
    'dispatch: loop {
        match pc {
            0x82B99018 => {
    //   block [0x82B99018..0x82B991B0)
	// 82B99018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9901C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B99020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B99024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B99028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9902C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B99030: 3C80B580  lis r4, -0x4a80
	ctx.r[4].s64 = -1249902592;
	// 82B99034: 38602000  li r3, 0x2000
	ctx.r[3].s64 = 8192;
	// 82B99038: 4B693999  bl 0x8222c9d0
	ctx.lr = 0x82B9903C;
	sub_8222C9D0(ctx, base);
	// 82B9903C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82B99040: 4082000C  bne 0x82b9904c
	if !ctx.cr[0].eq {
	pc = 0x82B9904C; continue 'dispatch;
	}
	// 82B99044: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B99048: 48000150  b 0x82b99198
	pc = 0x82B99198; continue 'dispatch;
	// 82B9904C: 93FE35D4  stw r31, 0x35d4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13780 as u32), ctx.r[31].u32 ) };
	// 82B99050: 38BF2000  addi r5, r31, 0x2000
	ctx.r[5].s64 = ctx.r[31].s64 + 8192;
	// 82B99054: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82B99058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9905C: 4BFFFCCD  bl 0x82b98d28
	ctx.lr = 0x82B99060;
	sub_82B98D28(ctx, base);
	// 82B99060: 57EB653E  srwi r11, r31, 0x14
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99064: 813E35D8  lwz r9, 0x35d8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13784 as u32) ) } as u64;
	// 82B99068: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82B9906C: 394B0200  addi r10, r11, 0x200
	ctx.r[10].s64 = ctx.r[11].s64 + 512;
	// 82B99070: 57EB00FE  clrlwi r11, r31, 3
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x1FFFFFFFu64;
	// 82B99074: 554A04E6  rlwinm r10, r10, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B99078: 38E30007  addi r7, r3, 7
	ctx.r[7].s64 = ctx.r[3].s64 + 7;
	// 82B9907C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B99080: 5128000E  rlwimi r8, r9, 0, 0, 7
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FF000000) | (ctx.r[8].u64 & 0xFFFFFFFF00FFFFFF);
	// 82B99084: 54EB1034  rlwinm r11, r7, 2, 0, 0x1a
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82B99088: 915E35DC  stw r10, 0x35dc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13788 as u32), ctx.r[10].u32 ) };
	// 82B9908C: 911E35D8  stw r8, 0x35d8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13784 as u32), ctx.r[8].u32 ) };
	// 82B99090: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82B99094: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82B99098: 395E3660  addi r10, r30, 0x3660
	ctx.r[10].s64 = ctx.r[30].s64 + 13920;
	// 82B9909C: 390BFFFC  addi r8, r11, -4
	ctx.r[8].s64 = ctx.r[11].s64 + -4;
	// 82B990A0: 38E005C8  li r7, 0x5c8
	ctx.r[7].s64 = 1480;
	// 82B990A4: 7CC900D0  neg r6, r9
	ctx.r[6].s64 = -ctx.r[9].s64;
	// 82B990A8: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 82B990AC: 5126446E  rlwimi r6, r9, 8, 0x11, 0x17
	ctx.r[6].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x0000000000007F00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF80FF);
	// 82B990B0: 94E80004  stwu r7, 4(r8)
	ea = ctx.r[8].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[8].u32 = ea;
	// 82B990B4: 3CA00007  lis r5, 7
	ctx.r[5].s64 = 458752;
	// 82B990B8: 54C72376  rlwinm r7, r6, 4, 0xd, 0x1b
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x0FFFFFFFu64;
	// 82B990BC: 60A38D00  ori r3, r5, 0x8d00
	ctx.r[3].u64 = ctx.r[5].u64 | 36096;
	// 82B990C0: 54E70566  rlwinm r7, r7, 0, 0x15, 0x13
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82B990C4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82B990C8: 94880004  stwu r4, 4(r8)
	ea = ctx.r[8].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[8].u32 = ea;
	// 82B990CC: 60E50001  ori r5, r7, 1
	ctx.r[5].u64 = ctx.r[7].u64 | 1;
	// 82B990D0: 94680004  stwu r3, 4(r8)
	ea = ctx.r[8].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[8].u32 = ea;
	// 82B990D4: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82B990D8: 94A80004  stwu r5, 4(r8)
	ea = ctx.r[8].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[8].u32 = ea;
	// 82B990DC: 4082FFF8  bne 0x82b990d4
	if !ctx.cr[0].eq {
	pc = 0x82B990D4; continue 'dispatch;
	}
	// 82B990E0: 38A00D00  li r5, 0xd00
	ctx.r[5].s64 = 3328;
	// 82B990E4: 5566653E  srwi r6, r11, 0x14
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B990E8: 94A80004  stwu r5, 4(r8)
	ea = ctx.r[8].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[8].u32 = ea;
	// 82B990EC: 556500FE  clrlwi r5, r11, 3
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82B990F0: 38C60200  addi r6, r6, 0x200
	ctx.r[6].s64 = ctx.r[6].s64 + 512;
	// 82B990F4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82B990F8: 54C604E6  rlwinm r6, r6, 0, 0x13, 0x13
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82B990FC: 2B090070  cmplwi cr6, r9, 0x70
	ctx.cr[6].compare_u32(ctx.r[9].u32, 112 as u32, &mut ctx.xer);
	// 82B99100: 94E80004  stwu r7, 4(r8)
	ea = ctx.r[8].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[8].u32 = ea;
	// 82B99104: 7CC62A14  add r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82B99108: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9910C: 7D0B4050  subf r8, r11, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82B99110: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82B99114: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82B99118: 7D081670  srawi r8, r8, 2
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 2) as i64;
	// 82B9911C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82B99120: 39080007  addi r8, r8, 7
	ctx.r[8].s64 = ctx.r[8].s64 + 7;
	// 82B99124: 50E6000E  rlwimi r6, r7, 0, 0, 7
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(0) as u64) & 0x00000000FF000000) | (ctx.r[6].u64 & 0xFFFFFFFF00FFFFFF);
	// 82B99128: 55081034  rlwinm r8, r8, 2, 0, 0x1a
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	// 82B9912C: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82B99130: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82B99134: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82B99138: 4099FF64  ble cr6, 0x82b9909c
	if !ctx.cr[6].gt {
	pc = 0x82B9909C; continue 'dispatch;
	}
	// 82B9913C: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 82B99140: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82B99144: 3D00C000  lis r8, -0x4000
	ctx.r[8].s64 = -1073741824;
	// 82B99148: 3CE00001  lis r7, 1
	ctx.r[7].s64 = 65536;
	// 82B9914C: 61083600  ori r8, r8, 0x3600
	ctx.r[8].u64 = ctx.r[8].u64 | 13824;
	// 82B99150: 60E70081  ori r7, r7, 0x81
	ctx.r[7].u64 = ctx.r[7].u64 | 129;
	// 82B99154: 950A0004  stwu r8, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[10].u32 = ea;
	// 82B99158: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9915C: 94EA0004  stwu r7, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[10].u32 = ea;
	// 82B99160: 4082FFE4  bne 0x82b99144
	if !ctx.cr[0].eq {
	pc = 0x82B99144; continue 'dispatch;
	}
	// 82B99164: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82B99168: 811E39E0  lwz r8, 0x39e0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14816 as u32) ) } as u64;
	// 82B9916C: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B99170: 38EA0004  addi r7, r10, 4
	ctx.r[7].s64 = ctx.r[10].s64 + 4;
	// 82B99174: 39490200  addi r10, r9, 0x200
	ctx.r[10].s64 = ctx.r[9].s64 + 512;
	// 82B99178: 556B00FE  clrlwi r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82B9917C: 554A04E6  rlwinm r10, r10, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B99180: 7CE91670  srawi r9, r7, 2
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[7].s32 >> 2) as i64;
	// 82B99184: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B99188: 5109000E  rlwimi r9, r8, 0, 0, 7
	ctx.r[9].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0x00000000FF000000) | (ctx.r[9].u64 & 0xFFFFFFFF00FFFFFF);
	// 82B9918C: 917E39E4  stw r11, 0x39e4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(14820 as u32), ctx.r[11].u32 ) };
	// 82B99190: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B99194: 913E39E0  stw r9, 0x39e0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(14816 as u32), ctx.r[9].u32 ) };
	// 82B99198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9919C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B991A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B991A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B991A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B991AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B991B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B991B0 size=84
    let mut pc: u32 = 0x82B991B0;
    'dispatch: loop {
        match pc {
            0x82B991B0 => {
    //   block [0x82B991B0..0x82B99204)
	// 82B991B0: 396409E8  addi r11, r4, 0x9e8
	ctx.r[11].s64 = ctx.r[4].s64 + 2536;
	// 82B991B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B991B8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B991BC: 8945000B  lbz r10, 0xb(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(11 as u32) ) } as u64;
	// 82B991C0: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82B991C4: 89250007  lbz r9, 7(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(7 as u32) ) } as u64;
	// 82B991C8: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82B991CC: 89050003  lbz r8, 3(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(3 as u32) ) } as u64;
	// 82B991D0: 38A50010  addi r5, r5, 0x10
	ctx.r[5].s64 = ctx.r[5].s64 + 16;
	// 82B991D4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82B991D8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B991DC: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82B991E0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B991E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B991E8: 4082FFD4  bne 0x82b991bc
	if !ctx.cr[0].eq {
	pc = 0x82B991BC; continue 'dispatch;
	}
	// 82B991EC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B991F0: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B991F4: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82B991F8: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82B991FC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B99200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B99208 size=84
    let mut pc: u32 = 0x82B99208;
    'dispatch: loop {
        match pc {
            0x82B99208 => {
    //   block [0x82B99208..0x82B9925C)
	// 82B99208: 396409F8  addi r11, r4, 0x9f8
	ctx.r[11].s64 = ctx.r[4].s64 + 2552;
	// 82B9920C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99210: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B99214: 8945000B  lbz r10, 0xb(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(11 as u32) ) } as u64;
	// 82B99218: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82B9921C: 89250007  lbz r9, 7(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(7 as u32) ) } as u64;
	// 82B99220: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82B99224: 89050003  lbz r8, 3(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(3 as u32) ) } as u64;
	// 82B99228: 38A50010  addi r5, r5, 0x10
	ctx.r[5].s64 = ctx.r[5].s64 + 16;
	// 82B9922C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82B99230: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B99234: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82B99238: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B9923C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B99240: 4082FFD4  bne 0x82b99214
	if !ctx.cr[0].eq {
	pc = 0x82B99214; continue 'dispatch;
	}
	// 82B99244: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B99248: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82B9924C: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82B99250: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82B99254: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B99258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82B99260 size=80
    let mut pc: u32 = 0x82B99260;
    'dispatch: loop {
        match pc {
            0x82B99260 => {
    //   block [0x82B99260..0x82B992B0)
	// 82B99260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B99264: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B99268: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9926C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B99270: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B99274: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99278: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9927C: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B99280: 90640000  stw r3, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82B99284: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82B99288: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82B9928C: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 82B99290: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B99294: 714A0340  andi. r10, r10, 0x340
	ctx.r[10].u64 = ctx.r[10].u64 & 832;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B99298: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 82B9929C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B992A0: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B992A4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B992A8: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B992AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B992B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82B992B0 size=272
    let mut pc: u32 = 0x82B992B0;
    'dispatch: loop {
        match pc {
            0x82B992B0 => {
    //   block [0x82B992B0..0x82B993C0)
	// 82B992B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B992B4: 48110151  bl 0x82ca9404
	ctx.lr = 0x82B992B8;
	sub_82CA93D0(ctx, base);
	// 82B992B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B992BC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 82B992C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82B992C4: 814B2150  lwz r10, 0x2150(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8528 as u32) ) } as u64;
	// 82B992C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B992CC: 419A0020  beq cr6, 0x82b992ec
	if ctx.cr[6].eq {
	pc = 0x82B992EC; continue 'dispatch;
	}
	// 82B992D0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B992D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B992D8: 4E800421  bctrl
	ctx.lr = 0x82B992DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B992DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B992E0: 4082000C  bne 0x82b992ec
	if !ctx.cr[0].eq {
	pc = 0x82B992EC; continue 'dispatch;
	}
	// 82B992E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B992E8: 480000D0  b 0x82b993b8
	pc = 0x82B993B8; continue 'dispatch;
	// 82B992EC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B992F0: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B992F4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B992F8: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82B992FC: 839D0008  lwz r28, 8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B99300: 7F6BEA14  add r27, r11, r29
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82B99304: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 82B99308: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B9930C: 714A0340  andi. r10, r10, 0x340
	ctx.r[10].u64 = ctx.r[10].u64 & 832;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B99310: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 82B99314: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B99318: 4B6936B9  bl 0x8222c9d0
	ctx.lr = 0x82B9931C;
	sub_8222C9D0(ctx, base);
	// 82B9931C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82B99320: 4182FFC4  beq 0x82b992e4
	if ctx.cr[0].eq {
	pc = 0x82B992E4; continue 'dispatch;
	}
	// 82B99324: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B99328: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9932C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82B99330: FBDF0000  std r30, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82B99334: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82B99338: FBDF0008  std r30, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u64 ) };
	// 82B9933C: FBDF0010  std r30, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 82B99340: FBDF0018  std r30, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u64 ) };
	// 82B99344: FBDF0020  std r30, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u64 ) };
	// 82B99348: 4B66F151  bl 0x82208498
	ctx.lr = 0x82B9934C;
	sub_82208498(ctx, base);
	// 82B9934C: 3C80B580  lis r4, -0x4a80
	ctx.r[4].s64 = -1249902592;
	// 82B99350: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82B99354: 4B69367D  bl 0x8222c9d0
	ctx.lr = 0x82B99358;
	sub_8222C9D0(ctx, base);
	// 82B99358: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9935C: 40820014  bne 0x82b99370
	if !ctx.cr[0].eq {
	pc = 0x82B99370; continue 'dispatch;
	}
	// 82B99360: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B99364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99368: 4B668F51  bl 0x822022b8
	ctx.lr = 0x82B9936C;
	sub_822022B8(ctx, base);
	// 82B9936C: 4BFFFF78  b 0x82b992e4
	pc = 0x82B992E4; continue 'dispatch;
	// 82B99370: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82B99374: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82B99378: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9937C: 48110105  bl 0x82ca9480
	ctx.lr = 0x82B99380;
	sub_82CA9480(ctx, base);
	// 82B99380: FBDF0000  std r30, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82B99384: 3D600010  lis r11, 0x10
	ctx.r[11].s64 = 1048576;
	// 82B99388: FBDF0008  std r30, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u64 ) };
	// 82B9938C: FBDF0010  std r30, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 82B99390: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B99394: FBDF0018  std r30, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u64 ) };
	// 82B99398: 3D20FFFF  lis r9, -1
	ctx.r[9].s64 = -65536;
	// 82B9939C: 616B0007  ori r11, r11, 7
	ctx.r[11].u64 = ctx.r[11].u64 | 7;
	// 82B993A0: FBDF0020  std r30, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u64 ) };
	// 82B993A4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82B993A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B993AC: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82B993B0: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82B993B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B993B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B993BC: 48110098  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B993C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B993C0 size=248
    let mut pc: u32 = 0x82B993C0;
    'dispatch: loop {
        match pc {
            0x82B993C0 => {
    //   block [0x82B993C0..0x82B994B8)
	// 82B993C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B993C4: 48110035  bl 0x82ca93f8
	ctx.lr = 0x82B993C8;
	sub_82CA93D0(ctx, base);
	// 82B993C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B993CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B993D0: 38A00368  li r5, 0x368
	ctx.r[5].s64 = 872;
	// 82B993D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B993D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B993DC: 481105D5  bl 0x82ca99b0
	ctx.lr = 0x82B993E0;
	sub_82CA99B0(ctx, base);
	// 82B993E0: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82B993E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B993E8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82B993EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B993F0: 3D20FFFF  lis r9, -1
	ctx.r[9].s64 = -65536;
	// 82B993F4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82B993F8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82B993FC: 817F0368  lwz r11, 0x368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(872 as u32) ) } as u64;
	// 82B99400: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B99404: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B99408: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9940C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82B99410: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B99414: 4182009C  beq 0x82b994b0
	if ctx.cr[0].eq {
	pc = 0x82B994B0; continue 'dispatch;
	}
	// 82B99418: 3B3F0028  addi r25, r31, 0x28
	ctx.r[25].s64 = ctx.r[31].s64 + 40;
	// 82B9941C: 3B7F0380  addi r27, r31, 0x380
	ctx.r[27].s64 = ctx.r[31].s64 + 896;
	// 82B99420: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 82B99424: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99428: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82B9942C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82B99430: 394B0368  addi r10, r11, 0x368
	ctx.r[10].s64 = ctx.r[11].s64 + 872;
	// 82B99434: 80EB0380  lwz r7, 0x380(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(896 as u32) ) } as u64;
	// 82B99438: 810B0368  lwz r8, 0x368(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(872 as u32) ) } as u64;
	// 82B9943C: 38E70009  addi r7, r7, 9
	ctx.r[7].s64 = ctx.r[7].s64 + 9;
	// 82B99440: 816B0384  lwz r11, 0x384(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(900 as u32) ) } as u64;
	// 82B99444: 7F494214  add r26, r9, r8
	ctx.r[26].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B99448: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9944C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B99450: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B99454: 419A0044  beq cr6, 0x82b99498
	if ctx.cr[6].eq {
	pc = 0x82B99498; continue 'dispatch;
	}
	// 82B99458: 3BD9001C  addi r30, r25, 0x1c
	ctx.r[30].s64 = ctx.r[25].s64 + 28;
	// 82B9945C: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82B99460: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82B99464: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99468: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9946C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B99470: 7C8BD214  add r4, r11, r26
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82B99474: 7F1E2040  cmplw cr6, r30, r4
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82B99478: 419A0010  beq cr6, 0x82b99488
	if ctx.cr[6].eq {
	pc = 0x82B99488; continue 'dispatch;
	}
	// 82B9947C: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82B99480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B99484: 4810FFFD  bl 0x82ca9480
	ctx.lr = 0x82B99488;
	sub_82CA9480(ctx, base);
	// 82B99488: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82B9948C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82B99490: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82B99494: 4082FFD0  bne 0x82b99464
	if !ctx.cr[0].eq {
	pc = 0x82B99464; continue 'dispatch;
	}
	// 82B99498: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82B9949C: 3718FFFF  addic. r24, r24, -1
	ctx.xer.ca = (ctx.r[24].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82B994A0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B994A4: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82B994A8: 3B3901A0  addi r25, r25, 0x1a0
	ctx.r[25].s64 = ctx.r[25].s64 + 416;
	// 82B994AC: 4082FF78  bne 0x82b99424
	if !ctx.cr[0].eq {
	pc = 0x82B99424; continue 'dispatch;
	}
	// 82B994B0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82B994B4: 4810FF94  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B994B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82B994B8 size=240
    let mut pc: u32 = 0x82B994B8;
    'dispatch: loop {
        match pc {
            0x82B994B8 => {
    //   block [0x82B994B8..0x82B995A8)
	// 82B994B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B994BC: 4810FF49  bl 0x82ca9404
	ctx.lr = 0x82B994C0;
	sub_82CA93D0(ctx, base);
	// 82B994C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B994C4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 82B994C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B994CC: 814B214C  lwz r10, 0x214c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8524 as u32) ) } as u64;
	// 82B994D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B994D4: 419A0020  beq cr6, 0x82b994f4
	if ctx.cr[6].eq {
	pc = 0x82B994F4; continue 'dispatch;
	}
	// 82B994D8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B994DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B994E0: 4E800421  bctrl
	ctx.lr = 0x82B994E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B994E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B994E8: 4082000C  bne 0x82b994f4
	if !ctx.cr[0].eq {
	pc = 0x82B994F4; continue 'dispatch;
	}
	// 82B994EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B994F0: 480000B0  b 0x82b995a0
	pc = 0x82B995A0; continue 'dispatch;
	// 82B994F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B994F8: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B994FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B99500: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82B99504: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B99508: 7F8BFA14  add r28, r11, r31
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82B9950C: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 82B99510: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B99514: 714A0340  andi. r10, r10, 0x340
	ctx.r[10].u64 = ctx.r[10].u64 & 832;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B99518: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 82B9951C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B99520: 4B6934B1  bl 0x8222c9d0
	ctx.lr = 0x82B99524;
	sub_8222C9D0(ctx, base);
	// 82B99524: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82B99528: 4182FFC4  beq 0x82b994ec
	if ctx.cr[0].eq {
	pc = 0x82B994EC; continue 'dispatch;
	}
	// 82B9952C: 38A00368  li r5, 0x368
	ctx.r[5].s64 = 872;
	// 82B99530: 837F0004  lwz r27, 4(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B99534: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B99538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9953C: 48110475  bl 0x82ca99b0
	ctx.lr = 0x82B99540;
	sub_82CA99B0(ctx, base);
	// 82B99540: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82B99544: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82B99548: 387E0368  addi r3, r30, 0x368
	ctx.r[3].s64 = ctx.r[30].s64 + 872;
	// 82B9954C: 4B66EF4D  bl 0x82208498
	ctx.lr = 0x82B99550;
	sub_82208498(ctx, base);
	// 82B99550: 3C80B580  lis r4, -0x4a80
	ctx.r[4].s64 = -1249902592;
	// 82B99554: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B99558: 4B693479  bl 0x8222c9d0
	ctx.lr = 0x82B9955C;
	sub_8222C9D0(ctx, base);
	// 82B9955C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82B99560: 40820014  bne 0x82b99574
	if !ctx.cr[0].eq {
	pc = 0x82B99574; continue 'dispatch;
	}
	// 82B99564: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B99568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9956C: 4B668D4D  bl 0x822022b8
	ctx.lr = 0x82B99570;
	sub_822022B8(ctx, base);
	// 82B99570: 4BFFFF7C  b 0x82b994ec
	pc = 0x82B994EC; continue 'dispatch;
	// 82B99574: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82B99578: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82B9957C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99580: 4810FF01  bl 0x82ca9480
	ctx.lr = 0x82B99584;
	sub_82CA9480(ctx, base);
	// 82B99584: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82B99588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9958C: 4BFFFE35  bl 0x82b993c0
	ctx.lr = 0x82B99590;
	sub_82B993C0(ctx, base);
	// 82B99590: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B99598: 656B0010  oris r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 1048576;
	// 82B9959C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B995A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B995A4: 4810FEB0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B995A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B995A8 size=136
    let mut pc: u32 = 0x82B995A8;
    'dispatch: loop {
        match pc {
            0x82B995A8 => {
    //   block [0x82B995A8..0x82B99630)
	// 82B995A8: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 82B995AC: 3D20002A  lis r9, 0x2a
	ctx.r[9].s64 = 2752512;
	// 82B995B0: B141FFF0  sth r10, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u16 ) };
	// 82B995B4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82B995B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B995BC: 612623B9  ori r6, r9, 0x23b9
	ctx.r[6].u64 = ctx.r[9].u64 | 9145;
	// 82B995C0: 91433090  stw r10, 0x3090(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12432 as u32), ctx.r[10].u32 ) };
	// 82B995C4: 9961FFF8  stb r11, -8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[11].u8 ) };
	// 82B995C8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82B995CC: 90C32FD8  stw r6, 0x2fd8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12248 as u32), ctx.r[6].u32 ) };
	// 82B995D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B995D4: 9961FFF9  stb r11, -7(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-7 as u32), ctx.r[11].u8 ) };
	// 82B995D8: 39232FA0  addi r9, r3, 0x2fa0
	ctx.r[9].s64 = ctx.r[3].s64 + 12192;
	// 82B995DC: 9961FFFA  stb r11, -6(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-6 as u32), ctx.r[11].u8 ) };
	// 82B995E0: 80C1FFF8  lwz r6, -8(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B995E4: 39490034  addi r10, r9, 0x34
	ctx.r[10].s64 = ctx.r[9].s64 + 52;
	// 82B995E8: B161FFF2  sth r11, -0xe(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-14 as u32), ctx.r[11].u16 ) };
	// 82B995EC: B1632FD4  sth r11, 0x2fd4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12244 as u32), ctx.r[11].u16 ) };
	// 82B995F0: B1632FD6  sth r11, 0x2fd6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12246 as u32), ctx.r[11].u16 ) };
	// 82B995F4: 99632FDC  stb r11, 0x2fdc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12252 as u32), ctx.r[11].u8 ) };
	// 82B995F8: 99632FDD  stb r11, 0x2fdd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12253 as u32), ctx.r[11].u8 ) };
	// 82B995FC: 99632FDE  stb r11, 0x2fde(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12254 as u32), ctx.r[11].u8 ) };
	// 82B99600: 80E1FFF0  lwz r7, -0x10(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82B99604: 91032FE4  stw r8, 0x2fe4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12260 as u32), ctx.r[8].u32 ) };
	// 82B99608: 90E32FE0  stw r7, 0x2fe0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12256 as u32), ctx.r[7].u32 ) };
	// 82B9960C: 90C32FE8  stw r6, 0x2fe8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12264 as u32), ctx.r[6].u32 ) };
	// 82B99610: 91632FBC  stw r11, 0x2fbc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12220 as u32), ctx.r[11].u32 ) };
	// 82B99614: 91632FD0  stw r11, 0x2fd0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12240 as u32), ctx.r[11].u32 ) };
	// 82B99618: 90A32FB8  stw r5, 0x2fb8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12216 as u32), ctx.r[5].u32 ) };
	// 82B9961C: 91232E2C  stw r9, 0x2e2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11820 as u32), ctx.r[9].u32 ) };
	// 82B99620: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82B99624: 656B0008  oris r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 524288;
	// 82B99628: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82B9962C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B99630 size=236
    let mut pc: u32 = 0x82B99630;
    'dispatch: loop {
        match pc {
            0x82B99630 => {
    //   block [0x82B99630..0x82B9971C)
	// 82B99630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B99634: 4810FDD1  bl 0x82ca9404
	ctx.lr = 0x82B99638;
	sub_82CA93D0(ctx, base);
	// 82B99638: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9963C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82B99640: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82B99644: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82B99648: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82B9964C: FB610050  std r27, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u64 ) };
	// 82B99650: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82B99654: FB610058  std r27, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u64 ) };
	// 82B99658: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9965C: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82B99660: 48000028  b 0x82b99688
	pc = 0x82B99688; continue 'dispatch;
	// 82B99664: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B99668: 41990008  bgt cr6, 0x82b99670
	if ctx.cr[6].gt {
	pc = 0x82B99670; continue 'dispatch;
	}
	// 82B9966C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82B99670: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82B99674: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82B99678: 390000FF  li r8, 0xff
	ctx.r[8].s64 = 255;
	// 82B9967C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B99680: 7D0B49AE  stbx r8, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82B99684: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99688: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82B9968C: 409AFFD8  bne cr6, 0x82b99664
	if !ctx.cr[6].eq {
	pc = 0x82B99664; continue 'dispatch;
	}
	// 82B99690: 1D7E000C  mulli r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 * 12;
	// 82B99694: 38AB0038  addi r5, r11, 0x38
	ctx.r[5].s64 = ctx.r[11].s64 + 56;
	// 82B99698: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9969C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B996A0: 48110311  bl 0x82ca99b0
	ctx.lr = 0x82B996A4;
	sub_82CA99B0(ctx, base);
	// 82B996A4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82B996A8: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82B996AC: 3D200010  lis r9, 0x10
	ctx.r[9].s64 = 1048576;
	// 82B996B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82B996B4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82B996B8: 61290005  ori r9, r9, 5
	ctx.r[9].u64 = ctx.r[9].u64 | 5;
	// 82B996BC: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82B996C0: 3CE0FFFF  lis r7, -1
	ctx.r[7].s64 = -65536;
	// 82B996C4: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82B996C8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B996CC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82B996D0: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82B996D4: 937F0030  stw r27, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[27].u32 ) };
	// 82B996D8: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82B996DC: F95F0028  std r10, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u64 ) };
	// 82B996E0: 419A0034  beq cr6, 0x82b99714
	if ctx.cr[6].eq {
	pc = 0x82B99714; continue 'dispatch;
	}
	// 82B996E4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82B996E8: 395F0034  addi r10, r31, 0x34
	ctx.r[10].s64 = ctx.r[31].s64 + 52;
	// 82B996EC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B996F0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82B996F4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B996F8: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B996FC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82B99700: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B99704: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82B99708: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82B9970C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82B99710: 4082FFDC  bne 0x82b996ec
	if !ctx.cr[0].eq {
	pc = 0x82B996EC; continue 'dispatch;
	}
	// 82B99714: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B99718: 4810FD3C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B99720 size=132
    let mut pc: u32 = 0x82B99720;
    'dispatch: loop {
        match pc {
            0x82B99720 => {
    //   block [0x82B99720..0x82B997A4)
	// 82B99720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B99724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B99728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B9972C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B99730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B99734: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B99738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9973C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82B99740: A13E0000  lhz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99744: 48000010  b 0x82b99754
	pc = 0x82B99754; continue 'dispatch;
	// 82B99748: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82B9974C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B99750: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99754: 2B0900FF  cmplwi cr6, r9, 0xff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 255 as u32, &mut ctx.xer);
	// 82B99758: 409AFFF0  bne cr6, 0x82b99748
	if !ctx.cr[6].eq {
	pc = 0x82B99748; continue 'dispatch;
	}
	// 82B9975C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B99760: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B99764: 386B0038  addi r3, r11, 0x38
	ctx.r[3].s64 = ctx.r[11].s64 + 56;
	// 82B99768: 4B693269  bl 0x8222c9d0
	ctx.lr = 0x82B9976C;
	sub_8222C9D0(ctx, base);
	// 82B9976C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82B99770: 4082000C  bne 0x82b9977c
	if !ctx.cr[0].eq {
	pc = 0x82B9977C; continue 'dispatch;
	}
	// 82B99774: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B99778: 48000014  b 0x82b9978c
	pc = 0x82B9978C; continue 'dispatch;
	// 82B9977C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82B99780: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B99784: 4BFFFEAD  bl 0x82b99630
	ctx.lr = 0x82B99788;
	sub_82B99630(ctx, base);
	// 82B99788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9978C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B99790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B99794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B99798: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9979C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B997A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B997A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B997A8 size=4
    let mut pc: u32 = 0x82B997A8;
    'dispatch: loop {
        match pc {
            0x82B997A8 => {
    //   block [0x82B997A8..0x82B997AC)
	// 82B997A8: 4B66ECF0  b 0x82208498
	sub_82208498(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B997B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B997B0 size=16
    let mut pc: u32 = 0x82B997B0;
    'dispatch: loop {
        match pc {
            0x82B997B0 => {
    //   block [0x82B997B0..0x82B997C0)
	// 82B997B0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82B997B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82B997B8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82B997BC: 4B66ECDC  b 0x82208498
	sub_82208498(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B997C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B997C0 size=240
    let mut pc: u32 = 0x82B997C0;
    'dispatch: loop {
        match pc {
            0x82B997C0 => {
    //   block [0x82B997C0..0x82B998B0)
	// 82B997C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B997C4: 4810FC45  bl 0x82ca9408
	ctx.lr = 0x82B997C8;
	sub_82CA93D0(ctx, base);
	// 82B997C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B997CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B997D0: 549D063F  clrlwi. r29, r4, 0x18
	ctx.r[29].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B997D4: 4082001C  bne 0x82b997f0
	if !ctx.cr[0].eq {
	pc = 0x82B997F0; continue 'dispatch;
	}
	// 82B997D8: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B997DC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B997E0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B997E4: 38CB98B0  addi r6, r11, -0x6750
	ctx.r[6].s64 = ctx.r[11].s64 + -26448;
	// 82B997E8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82B997EC: 481123CD  bl 0x82cabbb8
	ctx.lr = 0x82B997F0;
	sub_82CABBB8(ctx, base);
	// 82B997F0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82B997F4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B997F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82B997FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B99800: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B99804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B99808: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82B9980C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B99810: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82B99814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B99818: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82B9981C: 40990078  ble cr6, 0x82b99894
	if !ctx.cr[6].gt {
	pc = 0x82B99894; continue 'dispatch;
	}
	// 82B99820: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	// 82B99824: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99828: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82B9982C: 552B673E  rlwinm r11, r9, 0xc, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000FFFFFu64;
	// 82B99830: 7D4B40AE  lbzx r10, r11, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B99834: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 82B99838: 409A0040  bne cr6, 0x82b99878
	if !ctx.cr[6].eq {
	pc = 0x82B99878; continue 'dispatch;
	}
	// 82B9983C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B99840: 419A000C  beq cr6, 0x82b9984c
	if ctx.cr[6].eq {
	pc = 0x82B9984C; continue 'dispatch;
	}
	// 82B99844: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82B99848: 4800000C  b 0x82b99854
	pc = 0x82B99854; continue 'dispatch;
	// 82B9984C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82B99850: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B99854: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82B99858: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82B9985C: 4098000C  bge cr6, 0x82b99868
	if !ctx.cr[6].lt {
	pc = 0x82B99868; continue 'dispatch;
	}
	// 82B99860: 7C6B5830  slw r11, r3, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[3].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B99864: 7D642378  or r4, r11, r4
	ctx.r[4].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82B99868: 2B0A0010  cmplwi cr6, r10, 0x10
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	// 82B9986C: 4098000C  bge cr6, 0x82b99878
	if !ctx.cr[6].lt {
	pc = 0x82B99878; continue 'dispatch;
	}
	// 82B99870: 7C6B5030  slw r11, r3, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[3].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B99874: 7D652B78  or r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 | ctx.r[5].u64;
	// 82B99878: 5149831E  rlwimi r9, r10, 0x10, 0xc, 0xf
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000000F0000) | (ctx.r[9].u64 & 0xFFFFFFFFFFF0FFFF);
	// 82B9987C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82B99880: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B99884: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82B99888: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9988C: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B99890: 4198FF94  blt cr6, 0x82b99824
	if ctx.cr[6].lt {
	pc = 0x82B99824; continue 'dispatch;
	}
	// 82B99894: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B99898: 409A0010  bne cr6, 0x82b998a8
	if !ctx.cr[6].eq {
	pc = 0x82B998A8; continue 'dispatch;
	}
	// 82B9989C: 7F042840  cmplw cr6, r4, r5
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82B998A0: 419A0008  beq cr6, 0x82b998a8
	if ctx.cr[6].eq {
	pc = 0x82B998A8; continue 'dispatch;
	}
	// 82B998A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B998A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B998AC: 4810FBAC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B998B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B998B0 size=16
    let mut pc: u32 = 0x82B998B0;
    'dispatch: loop {
        match pc {
            0x82B998B0 => {
    //   block [0x82B998B0..0x82B998C0)
	// 82B998B0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B998B4: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B998B8: 7C6A5850  subf r3, r10, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82B998BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B998C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B998C0 size=132
    let mut pc: u32 = 0x82B998C0;
    'dispatch: loop {
        match pc {
            0x82B998C0 => {
    //   block [0x82B998C0..0x82B99944)
	// 82B998C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B998C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B998C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B998CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B998D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B998D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B998D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B998DC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B998E0: 7FCB2A14  add r30, r11, r5
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82B998E4: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B998E8: 4199001C  bgt cr6, 0x82b99904
	if ctx.cr[6].gt {
	pc = 0x82B99904; continue 'dispatch;
	}
	// 82B998EC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B998F0: 419A0028  beq cr6, 0x82b99918
	if ctx.cr[6].eq {
	pc = 0x82B99918; continue 'dispatch;
	}
	// 82B998F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B998F8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B998FC: 4810FB85  bl 0x82ca9480
	ctx.lr = 0x82B99900;
	sub_82CA9480(ctx, base);
	// 82B99900: 48000018  b 0x82b99918
	pc = 0x82B99918; continue 'dispatch;
	// 82B99904: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B99908: 419A0010  beq cr6, 0x82b99918
	if ctx.cr[6].eq {
	pc = 0x82B99918; continue 'dispatch;
	}
	// 82B9990C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82B99910: 616B4005  ori r11, r11, 0x4005
	ctx.r[11].u64 = ctx.r[11].u64 | 16389;
	// 82B99914: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82B99918: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9991C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82B99920: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B99924: 40990008  ble cr6, 0x82b9992c
	if !ctx.cr[6].gt {
	pc = 0x82B9992C; continue 'dispatch;
	}
	// 82B99928: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82B9992C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B99930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B99934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B99938: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9993C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B99940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B99948 size=128
    let mut pc: u32 = 0x82B99948;
    'dispatch: loop {
        match pc {
            0x82B99948 => {
    //   block [0x82B99948..0x82B999C8)
	// 82B99948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9994C: 4810FABD  bl 0x82ca9408
	ctx.lr = 0x82B99950;
	sub_82CA93D0(ctx, base);
	// 82B99950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B99954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B99958: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B9995C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82B99960: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82B99964: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82B99968: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82B9996C: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82B99970: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82B99974: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82B99978: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82B9997C: 419A0014  beq cr6, 0x82b99990
	if ctx.cr[6].eq {
	pc = 0x82B99990; continue 'dispatch;
	}
	// 82B99980: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82B99984: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B99988: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82B9998C: 48110025  bl 0x82ca99b0
	ctx.lr = 0x82B99990;
	sub_82CA99B0(ctx, base);
	// 82B99990: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82B99994: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B99998: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82B9999C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82B999A0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82B999A4: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82B999A8: 419A0014  beq cr6, 0x82b999bc
	if ctx.cr[6].eq {
	pc = 0x82B999BC; continue 'dispatch;
	}
	// 82B999AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82B999B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B999B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B999B8: 4810FFF9  bl 0x82ca99b0
	ctx.lr = 0x82B999BC;
	sub_82CA99B0(ctx, base);
	// 82B999BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B999C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B999C4: 4810FA94  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B999C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B999C8 size=1120
    let mut pc: u32 = 0x82B999C8;
    'dispatch: loop {
        match pc {
            0x82B999C8 => {
    //   block [0x82B999C8..0x82B99E28)
	// 82B999C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B999CC: 4810FA0D  bl 0x82ca93d8
	ctx.lr = 0x82B999D0;
	sub_82CA93D0(ctx, base);
	// 82B999D0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B999D4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82B999D8: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 82B999DC: 807A2560  lwz r3, 0x2560(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9568 as u32) ) } as u64;
	// 82B999E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82B999E4: 4198043C  blt cr6, 0x82b99e20
	if ctx.cr[6].lt {
	pc = 0x82B99E20; continue 'dispatch;
	}
	// 82B999E8: 817A255C  lwz r11, 0x255c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9564 as u32) ) } as u64;
	// 82B999EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B999F0: 409A0420  bne cr6, 0x82b99e10
	if !ctx.cr[6].eq {
	pc = 0x82B99E10; continue 'dispatch;
	}
	// 82B999F4: 3BF40014  addi r31, r20, 0x14
	ctx.r[31].s64 = ctx.r[20].s64 + 20;
	// 82B999F8: 82340018  lwz r17, 0x18(r20)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B999FC: 3BDA23A0  addi r30, r26, 0x23a0
	ctx.r[30].s64 = ctx.r[26].s64 + 9120;
	// 82B99A00: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82B99A04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B99A08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99A0C: 4BFFFEB5  bl 0x82b998c0
	ctx.lr = 0x82B99A10;
	sub_82B998C0(ctx, base);
	// 82B99A10: 817A2564  lwz r11, 0x2564(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9572 as u32) ) } as u64;
	// 82B99A14: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 82B99A18: 814B4DB4  lwz r10, 0x4db4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B99A1C: 554AD7FF  rlwinm. r10, r10, 0x1a, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B99A20: 82140018  lwz r16, 0x18(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B99A24: 4182015C  beq 0x82b99b80
	if ctx.cr[0].eq {
	pc = 0x82B99B80; continue 'dispatch;
	}
	// 82B99A28: 815A2558  lwz r10, 0x2558(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9560 as u32) ) } as u64;
	// 82B99A2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B99A30: 419A0150  beq cr6, 0x82b99b80
	if ctx.cr[6].eq {
	pc = 0x82B99B80; continue 'dispatch;
	}
	// 82B99A34: 814B4DDC  lwz r10, 0x4ddc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19932 as u32) ) } as u64;
	// 82B99A38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B99A3C: 419A0144  beq cr6, 0x82b99b80
	if ctx.cr[6].eq {
	pc = 0x82B99B80; continue 'dispatch;
	}
	// 82B99A40: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82B99A44: 7E759B78  mr r21, r19
	ctx.r[21].u64 = ctx.r[19].u64;
	// 82B99A48: 924B4DDC  stw r18, 0x4ddc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(19932 as u32), ctx.r[18].u32 ) };
	// 82B99A4C: 817A2564  lwz r11, 0x2564(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9572 as u32) ) } as u64;
	// 82B99A50: 7E599378  mr r25, r18
	ctx.r[25].u64 = ctx.r[18].u64;
	// 82B99A54: EADE0000  ld r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82B99A58: 814B4DB4  lwz r10, 0x4db4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B99A5C: 82EB4DE0  lwz r23, 0x4de0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19936 as u32) ) } as u64;
	// 82B99A60: 555835EE  rlwinm r24, r10, 6, 0x17, 0x17
	ctx.r[24].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 82B99A64: 572B00BE  clrlwi r11, r25, 2
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x3FFFFFFFu64;
	// 82B99A68: 572A103A  slwi r10, r25, 2
	ctx.r[10].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B99A6C: 216B003F  subfic r11, r11, 0x3f
	ctx.xer.ca = ctx.r[11].u32 <= 63 as u32;
	ctx.r[11].s64 = (63 as i64) - ctx.r[11].s64;
	// 82B99A70: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82B99A74: 7E6B5836  sld r11, r19, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[19].u64) << ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82B99A78: 7D6BB038  and r11, r11, r22
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[22].u64;
	// 82B99A7C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82B99A80: 419A017C  beq cr6, 0x82b99bfc
	if ctx.cr[6].eq {
	pc = 0x82B99BFC; continue 'dispatch;
	}
	// 82B99A84: 39790001  addi r11, r25, 1
	ctx.r[11].s64 = ctx.r[25].s64 + 1;
	// 82B99A88: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82B99A8C: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 82B99A90: 4098004C  bge cr6, 0x82b99adc
	if !ctx.cr[6].lt {
	pc = 0x82B99ADC; continue 'dispatch;
	}
	// 82B99A94: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B99A98: 5549F0BE  srwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B99A9C: 2129003F  subfic r9, r9, 0x3f
	ctx.xer.ca = ctx.r[9].u32 <= 63 as u32;
	ctx.r[9].s64 = (63 as i64) - ctx.r[9].s64;
	// 82B99AA0: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82B99AA4: 7E694836  sld r9, r19, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = (ctx.r[19].u64) << ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82B99AA8: 7D29B038  and r9, r9, r22
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[22].u64;
	// 82B99AAC: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 82B99AB0: 419A002C  beq cr6, 0x82b99adc
	if ctx.cr[6].eq {
	pc = 0x82B99ADC; continue 'dispatch;
	}
	// 82B99AB4: 813A2558  lwz r9, 0x2558(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9560 as u32) ) } as u64;
	// 82B99AB8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B99ABC: 409A000C  bne cr6, 0x82b99ac8
	if !ctx.cr[6].eq {
	pc = 0x82B99AC8; continue 'dispatch;
	}
	// 82B99AC0: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82B99AC4: 419A0018  beq cr6, 0x82b99adc
	if ctx.cr[6].eq {
	pc = 0x82B99ADC; continue 'dispatch;
	}
	// 82B99AC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B99ACC: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82B99AD0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B99AD4: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 82B99AD8: 4198FFC0  blt cr6, 0x82b99a98
	if ctx.cr[6].lt {
	pc = 0x82B99A98; continue 'dispatch;
	}
	// 82B99ADC: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99AE0: 83740004  lwz r27, 4(r20)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B99AE4: 7D5CC850  subf r10, r28, r25
	ctx.r[10].s64 = ctx.r[25].s64 - ctx.r[28].s64;
	// 82B99AE8: 7FCBC214  add r30, r11, r24
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82B99AEC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82B99AF0: B3C10050  sth r30, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u16 ) };
	// 82B99AF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99AFC: 3BAA0001  addi r29, r10, 1
	ctx.r[29].s64 = ctx.r[10].s64 + 1;
	// 82B99B00: 4BFFFDC1  bl 0x82b998c0
	ctx.lr = 0x82B99B04;
	sub_82B998C0(ctx, base);
	// 82B99B04: 57AB2436  rlwinm r11, r29, 4, 0x10, 0x1b
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0FFFFFFFu64;
	// 82B99B08: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82B99B0C: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82B99B10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99B14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99B18: 4BFFFDA9  bl 0x82b998c0
	ctx.lr = 0x82B99B1C;
	sub_82B998C0(ctx, base);
	// 82B99B1C: 397E0030  addi r11, r30, 0x30
	ctx.r[11].s64 = ctx.r[30].s64 + 48;
	// 82B99B20: 57A53032  slwi r5, r29, 6
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B99B24: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99B28: 7C8BD214  add r4, r11, r26
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82B99B2C: 817A2558  lwz r11, 0x2558(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9560 as u32) ) } as u64;
	// 82B99B30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B99B34: 419A0084  beq cr6, 0x82b99bb8
	if ctx.cr[6].eq {
	pc = 0x82B99BB8; continue 'dispatch;
	}
	// 82B99B38: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82B99B3C: 4BFFFD85  bl 0x82b998c0
	ctx.lr = 0x82B99B40;
	sub_82B998C0(ctx, base);
	// 82B99B40: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82B99B44: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99B48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99B4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99B50: 4BFFFD71  bl 0x82b998c0
	ctx.lr = 0x82B99B54;
	sub_82B998C0(ctx, base);
	// 82B99B54: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82B99B58: 419A00A4  beq cr6, 0x82b99bfc
	if ctx.cr[6].eq {
	pc = 0x82B99BFC; continue 'dispatch;
	}
	// 82B99B5C: 7F1CB840  cmplw cr6, r28, r23
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82B99B60: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82B99B64: 41990008  bgt cr6, 0x82b99b6c
	if ctx.cr[6].gt {
	pc = 0x82B99B6C; continue 'dispatch;
	}
	// 82B99B68: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82B99B6C: 7D5C5850  subf r10, r28, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82B99B70: 7D3DE214  add r9, r29, r28
	ctx.r[9].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82B99B74: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B99B78: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82B99B7C: 48000030  b 0x82b99bac
	pc = 0x82B99BAC; continue 'dispatch;
	// 82B99B80: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82B99B84: 7E559378  mr r21, r18
	ctx.r[21].u64 = ctx.r[18].u64;
	// 82B99B88: 4BFFFEC4  b 0x82b99a4c
	pc = 0x82B99A4C; continue 'dispatch;
	// 82B99B8C: 811A2564  lwz r8, 0x2564(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9572 as u32) ) } as u64;
	// 82B99B90: 80E84DE0  lwz r7, 0x4de0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19936 as u32) ) } as u64;
	// 82B99B94: 81084DE4  lwz r8, 0x4de4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19940 as u32) ) } as u64;
	// 82B99B98: 7CE75850  subf r7, r7, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82B99B9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B99BA0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B99BA4: 7D47412E  stwx r10, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 82B99BA8: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 82B99BAC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B99BB0: 4198FFDC  blt cr6, 0x82b99b8c
	if ctx.cr[6].lt {
	pc = 0x82B99B8C; continue 'dispatch;
	}
	// 82B99BB4: 48000048  b 0x82b99bfc
	pc = 0x82B99BFC; continue 'dispatch;
	// 82B99BB8: 7F1CB840  cmplw cr6, r28, r23
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82B99BBC: 40980014  bge cr6, 0x82b99bd0
	if !ctx.cr[6].lt {
	pc = 0x82B99BD0; continue 'dispatch;
	}
	// 82B99BC0: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82B99BC4: 4BFFFCFD  bl 0x82b998c0
	ctx.lr = 0x82B99BC8;
	sub_82B998C0(ctx, base);
	// 82B99BC8: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82B99BCC: 48000020  b 0x82b99bec
	pc = 0x82B99BEC; continue 'dispatch;
	// 82B99BD0: 817A2564  lwz r11, 0x2564(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(9572 as u32) ) } as u64;
	// 82B99BD4: 814B4DE0  lwz r10, 0x4de0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19936 as u32) ) } as u64;
	// 82B99BD8: 816B4DE4  lwz r11, 0x4de4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19940 as u32) ) } as u64;
	// 82B99BDC: 7D4AE050  subf r10, r10, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[10].s64;
	// 82B99BE0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B99BE4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B99BE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B99BEC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99BF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99BF8: 4BFFFCC9  bl 0x82b998c0
	ctx.lr = 0x82B99BFC;
	sub_82B998C0(ctx, base);
	// 82B99BFC: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82B99C00: 2B190040  cmplwi cr6, r25, 0x40
	ctx.cr[6].compare_u32(ctx.r[25].u32, 64 as u32, &mut ctx.xer);
	// 82B99C04: 4198FE60  blt cr6, 0x82b99a64
	if ctx.cr[6].lt {
	pc = 0x82B99A64; continue 'dispatch;
	}
	// 82B99C08: 92410050  stw r18, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[18].u32 ) };
	// 82B99C0C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99C10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99C18: 4BFFFCA9  bl 0x82b998c0
	ctx.lr = 0x82B99C1C;
	sub_82B998C0(ctx, base);
	// 82B99C1C: 7E5E9378  mr r30, r18
	ctx.r[30].u64 = ctx.r[18].u64;
	// 82B99C20: 57CBD97E  srwi r11, r30, 5
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99C24: 57CA06FE  clrlwi r10, r30, 0x1b
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x0000001Fu64;
	// 82B99C28: 396B08FE  addi r11, r11, 0x8fe
	ctx.r[11].s64 = ctx.r[11].s64 + 2302;
	// 82B99C2C: 7E6A5030  slw r10, r19, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[19].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B99C30: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99C34: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82B99C38: 7D4B5839  and. r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B99C3C: 41820088  beq 0x82b99cc4
	if ctx.cr[0].eq {
	pc = 0x82B99CC4; continue 'dispatch;
	}
	// 82B99C40: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82B99C44: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82B99C48: 4800002C  b 0x82b99c74
	pc = 0x82B99C74; continue 'dispatch;
	// 82B99C4C: 5569D97E  srwi r9, r11, 5
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B99C50: 556806FE  clrlwi r8, r11, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B99C54: 392908FE  addi r9, r9, 0x8fe
	ctx.r[9].s64 = ctx.r[9].s64 + 2302;
	// 82B99C58: 7E684030  slw r8, r19, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[19].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B99C5C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B99C60: 7D29D02E  lwzx r9, r9, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82B99C64: 7D094839  and. r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 & ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B99C68: 41820014  beq 0x82b99c7c
	if ctx.cr[0].eq {
	pc = 0x82B99C7C; continue 'dispatch;
	}
	// 82B99C6C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B99C70: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B99C74: 2B0B08E8  cmplwi cr6, r11, 0x8e8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2280 as u32, &mut ctx.xer);
	// 82B99C78: 4198FFD4  blt cr6, 0x82b99c4c
	if ctx.cr[6].lt {
	pc = 0x82B99C4C; continue 'dispatch;
	}
	// 82B99C7C: 5549143A  rlwinm r9, r10, 2, 0x10, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82B99C80: 7D6AF050  subf r11, r10, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 82B99C84: B1210050  sth r9, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u16 ) };
	// 82B99C88: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82B99C8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99C94: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82B99C98: 555C103A  slwi r28, r10, 2
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82B99C9C: 4BFFFC25  bl 0x82b998c0
	ctx.lr = 0x82B99CA0;
	sub_82B998C0(ctx, base);
	// 82B99CA0: B3A10050  sth r29, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u16 ) };
	// 82B99CA4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82B99CA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99CB0: 4BFFFC11  bl 0x82b998c0
	ctx.lr = 0x82B99CB4;
	sub_82B998C0(ctx, base);
	// 82B99CB4: 57A5103A  slwi r5, r29, 2
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B99CB8: 7C9CD214  add r4, r28, r26
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[26].u64;
	// 82B99CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99CC0: 4BFFFC01  bl 0x82b998c0
	ctx.lr = 0x82B99CC4;
	sub_82B998C0(ctx, base);
	// 82B99CC4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B99CC8: 2B1E08E8  cmplwi cr6, r30, 0x8e8
	ctx.cr[6].compare_u32(ctx.r[30].u32, 2280 as u32, &mut ctx.xer);
	// 82B99CCC: 4198FF54  blt cr6, 0x82b99c20
	if ctx.cr[6].lt {
	pc = 0x82B99C20; continue 'dispatch;
	}
	// 82B99CD0: 92410050  stw r18, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[18].u32 ) };
	// 82B99CD4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99CD8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99CE0: 4BFFFBE1  bl 0x82b998c0
	ctx.lr = 0x82B99CE4;
	sub_82B998C0(ctx, base);
	// 82B99CE4: 7E5C9378  mr r28, r18
	ctx.r[28].u64 = ctx.r[18].u64;
	// 82B99CE8: 397C094E  addi r11, r28, 0x94e
	ctx.r[11].s64 = ctx.r[28].s64 + 2382;
	// 82B99CEC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99CF0: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82B99CF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B99CF8: 419A00D0  beq cr6, 0x82b99dc8
	if ctx.cr[6].eq {
	pc = 0x82B99DC8; continue 'dispatch;
	}
	// 82B99CFC: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 82B99D00: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82B99D04: 4800002C  b 0x82b99d30
	pc = 0x82B99D30; continue 'dispatch;
	// 82B99D08: 556AD97E  srwi r10, r11, 5
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B99D0C: 556906FE  clrlwi r9, r11, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B99D10: 394A094E  addi r10, r10, 0x94e
	ctx.r[10].s64 = ctx.r[10].s64 + 2382;
	// 82B99D14: 7E694830  slw r9, r19, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[19].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B99D18: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B99D1C: 7D4AD02E  lwzx r10, r10, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82B99D20: 7D2A5039  and. r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B99D24: 41820014  beq 0x82b99d38
	if ctx.cr[0].eq {
	pc = 0x82B99D38; continue 'dispatch;
	}
	// 82B99D28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B99D2C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82B99D30: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82B99D34: 4198FFD4  blt cr6, 0x82b99d08
	if ctx.cr[6].lt {
	pc = 0x82B99D08; continue 'dispatch;
	}
	// 82B99D38: 395D08C0  addi r10, r29, 0x8c0
	ctx.r[10].s64 = ctx.r[29].s64 + 2240;
	// 82B99D3C: 7D7DE050  subf r11, r29, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 82B99D40: 554A143A  rlwinm r10, r10, 2, 0x10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82B99D44: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82B99D48: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82B99D4C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99D54: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82B99D58: 4BFFFB69  bl 0x82b998c0
	ctx.lr = 0x82B99D5C;
	sub_82B998C0(ctx, base);
	// 82B99D5C: 57CB0C3C  rlwinm r11, r30, 1, 0x10, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x7FFFFFFFu64;
	// 82B99D60: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82B99D64: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82B99D68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99D70: 4BFFFB51  bl 0x82b998c0
	ctx.lr = 0x82B99D74;
	sub_82B998C0(ctx, base);
	// 82B99D74: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82B99D78: 419A0050  beq cr6, 0x82b99dc8
	if ctx.cr[6].eq {
	pc = 0x82B99DC8; continue 'dispatch;
	}
	// 82B99D7C: 397D0946  addi r11, r29, 0x946
	ctx.r[11].s64 = ctx.r[29].s64 + 2374;
	// 82B99D80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B99D84: 7FABD214  add r29, r11, r26
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82B99D88: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82B99D8C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99D90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99D94: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B99D98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99D9C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B99DA0: 4BFFFB21  bl 0x82b998c0
	ctx.lr = 0x82B99DA4;
	sub_82B998C0(ctx, base);
	// 82B99DA4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99DA8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99DAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99DB4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B99DB8: 4BFFFB09  bl 0x82b998c0
	ctx.lr = 0x82B99DBC;
	sub_82B998C0(ctx, base);
	// 82B99DBC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82B99DC0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82B99DC4: 4082FFC4  bne 0x82b99d88
	if !ctx.cr[0].eq {
	pc = 0x82B99D88; continue 'dispatch;
	}
	// 82B99DC8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82B99DCC: 2B1C0008  cmplwi cr6, r28, 8
	ctx.cr[6].compare_u32(ctx.r[28].u32, 8 as u32, &mut ctx.xer);
	// 82B99DD0: 4198FF18  blt cr6, 0x82b99ce8
	if ctx.cr[6].lt {
	pc = 0x82B99CE8; continue 'dispatch;
	}
	// 82B99DD4: 92410050  stw r18, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[18].u32 ) };
	// 82B99DD8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99DDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99DE4: 4BFFFADD  bl 0x82b998c0
	ctx.lr = 0x82B99DE8;
	sub_82B998C0(ctx, base);
	// 82B99DE8: 39710010  addi r11, r17, 0x10
	ctx.r[11].s64 = ctx.r[17].s64 + 16;
	// 82B99DEC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B99DF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B99DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B99DF8: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B99DFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B99E00: 7D70F050  subf r11, r16, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[16].s64;
	// 82B99E04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B99E08: 4BFFFAB9  bl 0x82b998c0
	ctx.lr = 0x82B99E0C;
	sub_82B998C0(ctx, base);
	// 82B99E0C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82B99E10: 80740010  lwz r3, 0x10(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B99E14: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82B99E18: 41980008  blt cr6, 0x82b99e20
	if ctx.cr[6].lt {
	pc = 0x82B99E20; continue 'dispatch;
	}
	// 82B99E1C: 80740024  lwz r3, 0x24(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(36 as u32) ) } as u64;
	// 82B99E20: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82B99E24: 4810F604  b 0x82ca9428
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B99E28 size=192
    let mut pc: u32 = 0x82B99E28;
    'dispatch: loop {
        match pc {
            0x82B99E28 => {
    //   block [0x82B99E28..0x82B99EE8)
	// 82B99E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B99E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B99E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B99E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B99E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B99E3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B99E40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B99E44: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B99E48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B99E4C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82B99E50: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82B99E54: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82B99E58: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82B99E5C: 9BDF007C  stb r30, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u8 ) };
	// 82B99E60: 4810FB51  bl 0x82ca99b0
	ctx.lr = 0x82B99E64;
	sub_82CA99B0(ctx, base);
	// 82B99E64: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B99E68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B99E6C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82B99E70: 4810FB41  bl 0x82ca99b0
	ctx.lr = 0x82B99E74;
	sub_82CA99B0(ctx, base);
	// 82B99E74: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B99E78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B99E7C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82B99E80: 4810FB31  bl 0x82ca99b0
	ctx.lr = 0x82B99E84;
	sub_82CA99B0(ctx, base);
	// 82B99E84: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B99E88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B99E8C: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82B99E90: 4810FB21  bl 0x82ca99b0
	ctx.lr = 0x82B99E94;
	sub_82CA99B0(ctx, base);
	// 82B99E94: 93DF0100  stw r30, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[30].u32 ) };
	// 82B99E98: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 82B99E9C: 93DF0108  stw r30, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[30].u32 ) };
	// 82B99EA0: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82B99EA4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82B99EA8: 93DF013C  stw r30, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[30].u32 ) };
	// 82B99EAC: 93DF0150  stw r30, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[30].u32 ) };
	// 82B99EB0: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 82B99EB4: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 82B99EB8: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 82B99EBC: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 82B99EC0: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82B99EC4: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 82B99EC8: 93DF0158  stw r30, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[30].u32 ) };
	// 82B99ECC: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 82B99ED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B99ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B99ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B99EDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B99EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B99EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B99EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B99EE8 size=640
    let mut pc: u32 = 0x82B99EE8;
    'dispatch: loop {
        match pc {
            0x82B99EE8 => {
    //   block [0x82B99EE8..0x82B9A168)
	// 82B99EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B99EEC: 4810F515  bl 0x82ca9400
	ctx.lr = 0x82B99EF0;
	sub_82CA93D0(ctx, base);
	// 82B99EF0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B99EF4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82B99EF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82B99EFC: 4BFFFF2D  bl 0x82b99e28
	ctx.lr = 0x82B99F00;
	sub_82B99E28(ctx, base);
	// 82B99F00: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82B99F04: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82B99F08: 3D408007  lis r10, -0x7ff9
	ctx.r[10].s64 = -2147024896;
	// 82B99F0C: 817C0068  lwz r11, 0x68(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(104 as u32) ) } as u64;
	// 82B99F10: 3BFC0060  addi r31, r28, 0x60
	ctx.r[31].s64 = ctx.r[28].s64 + 96;
	// 82B99F14: 937C0064  stw r27, 0x64(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82B99F18: 615A000E  ori r26, r10, 0xe
	ctx.r[26].u64 = ctx.r[10].u64 | 14;
	// 82B99F1C: 2B0B0064  cmplwi cr6, r11, 0x64
	ctx.cr[6].compare_u32(ctx.r[11].u32, 100 as u32, &mut ctx.xer);
	// 82B99F20: 40980060  bge cr6, 0x82b99f80
	if !ctx.cr[6].lt {
	pc = 0x82B99F80; continue 'dispatch;
	}
	// 82B99F24: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B99F28: 2B1E0064  cmplwi cr6, r30, 0x64
	ctx.cr[6].compare_u32(ctx.r[30].u32, 100 as u32, &mut ctx.xer);
	// 82B99F2C: 40980008  bge cr6, 0x82b99f34
	if !ctx.cr[6].lt {
	pc = 0x82B99F34; continue 'dispatch;
	}
	// 82B99F30: 3BC00064  li r30, 0x64
	ctx.r[30].s64 = 100;
	// 82B99F34: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B99F38: 1C7E000C  mulli r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 * 12;
	// 82B99F3C: 4B692A95  bl 0x8222c9d0
	ctx.lr = 0x82B99F40;
	sub_8222C9D0(ctx, base);
	// 82B99F40: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B99F44: 4082000C  bne 0x82b99f50
	if !ctx.cr[0].eq {
	pc = 0x82B99F50; continue 'dispatch;
	}
	// 82B99F48: 935C0050  stw r26, 0x50(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82B99F4C: 48000034  b 0x82b99f80
	pc = 0x82B99F80; continue 'dispatch;
	// 82B99F50: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99F54: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B99F58: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B99F5C: 419A0020  beq cr6, 0x82b99f7c
	if ctx.cr[6].eq {
	pc = 0x82B99F7C; continue 'dispatch;
	}
	// 82B99F60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B99F64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B99F68: 1CAB000C  mulli r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 * 12;
	// 82B99F6C: 4810F515  bl 0x82ca9480
	ctx.lr = 0x82B99F70;
	sub_82CA9480(ctx, base);
	// 82B99F70: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99F74: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B99F78: 4B668341  bl 0x822022b8
	ctx.lr = 0x82B99F7C;
	sub_822022B8(ctx, base);
	// 82B99F7C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B99F80: 817C0074  lwz r11, 0x74(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(116 as u32) ) } as u64;
	// 82B99F84: 3BFC006C  addi r31, r28, 0x6c
	ctx.r[31].s64 = ctx.r[28].s64 + 108;
	// 82B99F88: 937C0070  stw r27, 0x70(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 82B99F8C: 2B0B0032  cmplwi cr6, r11, 0x32
	ctx.cr[6].compare_u32(ctx.r[11].u32, 50 as u32, &mut ctx.xer);
	// 82B99F90: 40980060  bge cr6, 0x82b99ff0
	if !ctx.cr[6].lt {
	pc = 0x82B99FF0; continue 'dispatch;
	}
	// 82B99F94: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B99F98: 2B1E0032  cmplwi cr6, r30, 0x32
	ctx.cr[6].compare_u32(ctx.r[30].u32, 50 as u32, &mut ctx.xer);
	// 82B99F9C: 40980008  bge cr6, 0x82b99fa4
	if !ctx.cr[6].lt {
	pc = 0x82B99FA4; continue 'dispatch;
	}
	// 82B99FA0: 3BC00032  li r30, 0x32
	ctx.r[30].s64 = 50;
	// 82B99FA4: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B99FA8: 57C31838  slwi r3, r30, 3
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B99FAC: 4B692A25  bl 0x8222c9d0
	ctx.lr = 0x82B99FB0;
	sub_8222C9D0(ctx, base);
	// 82B99FB0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B99FB4: 4082000C  bne 0x82b99fc0
	if !ctx.cr[0].eq {
	pc = 0x82B99FC0; continue 'dispatch;
	}
	// 82B99FB8: 935C0050  stw r26, 0x50(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82B99FBC: 48000034  b 0x82b99ff0
	pc = 0x82B99FF0; continue 'dispatch;
	// 82B99FC0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99FC4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B99FC8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B99FCC: 419A0020  beq cr6, 0x82b99fec
	if ctx.cr[6].eq {
	pc = 0x82B99FEC; continue 'dispatch;
	}
	// 82B99FD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B99FD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B99FD8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B99FDC: 4810F4A5  bl 0x82ca9480
	ctx.lr = 0x82B99FE0;
	sub_82CA9480(ctx, base);
	// 82B99FE0: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B99FE4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99FE8: 4B6682D1  bl 0x822022b8
	ctx.lr = 0x82B99FEC;
	sub_822022B8(ctx, base);
	// 82B99FEC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B99FF0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B99FF4: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B99FF8: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B99FFC: 41820014  beq 0x82b9a010
	if ctx.cr[0].eq {
	pc = 0x82B9A010; continue 'dispatch;
	}
	// 82B9A000: 817C0114  lwz r11, 0x114(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(276 as u32) ) } as u64;
	// 82B9A004: 3BFC010C  addi r31, r28, 0x10c
	ctx.r[31].s64 = ctx.r[28].s64 + 268;
	// 82B9A008: 937C0110  stw r27, 0x110(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(272 as u32), ctx.r[27].u32 ) };
	// 82B9A00C: 480000F0  b 0x82b9a0fc
	pc = 0x82B9A0FC; continue 'dispatch;
	// 82B9A010: 817C0120  lwz r11, 0x120(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(288 as u32) ) } as u64;
	// 82B9A014: 3BFC0118  addi r31, r28, 0x118
	ctx.r[31].s64 = ctx.r[28].s64 + 280;
	// 82B9A018: 937C011C  stw r27, 0x11c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(284 as u32), ctx.r[27].u32 ) };
	// 82B9A01C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82B9A020: 40980060  bge cr6, 0x82b9a080
	if !ctx.cr[6].lt {
	pc = 0x82B9A080; continue 'dispatch;
	}
	// 82B9A024: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B9A028: 2B1E0002  cmplwi cr6, r30, 2
	ctx.cr[6].compare_u32(ctx.r[30].u32, 2 as u32, &mut ctx.xer);
	// 82B9A02C: 40980008  bge cr6, 0x82b9a034
	if !ctx.cr[6].lt {
	pc = 0x82B9A034; continue 'dispatch;
	}
	// 82B9A030: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82B9A034: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9A038: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B9A03C: 4B692995  bl 0x8222c9d0
	ctx.lr = 0x82B9A040;
	sub_8222C9D0(ctx, base);
	// 82B9A040: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9A044: 4082000C  bne 0x82b9a050
	if !ctx.cr[0].eq {
	pc = 0x82B9A050; continue 'dispatch;
	}
	// 82B9A048: 935C0050  stw r26, 0x50(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82B9A04C: 48000034  b 0x82b9a080
	pc = 0x82B9A080; continue 'dispatch;
	// 82B9A050: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A054: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B9A058: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9A05C: 419A0020  beq cr6, 0x82b9a07c
	if ctx.cr[6].eq {
	pc = 0x82B9A07C; continue 'dispatch;
	}
	// 82B9A060: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A064: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9A068: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9A06C: 4810F415  bl 0x82ca9480
	ctx.lr = 0x82B9A070;
	sub_82CA9480(ctx, base);
	// 82B9A070: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9A074: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A078: 4B668241  bl 0x822022b8
	ctx.lr = 0x82B9A07C;
	sub_822022B8(ctx, base);
	// 82B9A07C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9A080: 817C012C  lwz r11, 0x12c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(300 as u32) ) } as u64;
	// 82B9A084: 3BFC0124  addi r31, r28, 0x124
	ctx.r[31].s64 = ctx.r[28].s64 + 292;
	// 82B9A088: 937C0128  stw r27, 0x128(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(296 as u32), ctx.r[27].u32 ) };
	// 82B9A08C: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82B9A090: 40980060  bge cr6, 0x82b9a0f0
	if !ctx.cr[6].lt {
	pc = 0x82B9A0F0; continue 'dispatch;
	}
	// 82B9A094: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B9A098: 2B1E0010  cmplwi cr6, r30, 0x10
	ctx.cr[6].compare_u32(ctx.r[30].u32, 16 as u32, &mut ctx.xer);
	// 82B9A09C: 40980008  bge cr6, 0x82b9a0a4
	if !ctx.cr[6].lt {
	pc = 0x82B9A0A4; continue 'dispatch;
	}
	// 82B9A0A0: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82B9A0A4: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9A0A8: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B9A0AC: 4B692925  bl 0x8222c9d0
	ctx.lr = 0x82B9A0B0;
	sub_8222C9D0(ctx, base);
	// 82B9A0B0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9A0B4: 4082000C  bne 0x82b9a0c0
	if !ctx.cr[0].eq {
	pc = 0x82B9A0C0; continue 'dispatch;
	}
	// 82B9A0B8: 935C0050  stw r26, 0x50(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82B9A0BC: 48000034  b 0x82b9a0f0
	pc = 0x82B9A0F0; continue 'dispatch;
	// 82B9A0C0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A0C4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B9A0C8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9A0CC: 419A0020  beq cr6, 0x82b9a0ec
	if ctx.cr[6].eq {
	pc = 0x82B9A0EC; continue 'dispatch;
	}
	// 82B9A0D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A0D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9A0D8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9A0DC: 4810F3A5  bl 0x82ca9480
	ctx.lr = 0x82B9A0E0;
	sub_82CA9480(ctx, base);
	// 82B9A0E0: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9A0E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A0E8: 4B6681D1  bl 0x822022b8
	ctx.lr = 0x82B9A0EC;
	sub_822022B8(ctx, base);
	// 82B9A0EC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9A0F0: 817C0138  lwz r11, 0x138(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(312 as u32) ) } as u64;
	// 82B9A0F4: 3BFC0130  addi r31, r28, 0x130
	ctx.r[31].s64 = ctx.r[28].s64 + 304;
	// 82B9A0F8: 937C0134  stw r27, 0x134(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(308 as u32), ctx.r[27].u32 ) };
	// 82B9A0FC: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82B9A100: 40980060  bge cr6, 0x82b9a160
	if !ctx.cr[6].lt {
	pc = 0x82B9A160; continue 'dispatch;
	}
	// 82B9A104: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B9A108: 2B1E0010  cmplwi cr6, r30, 0x10
	ctx.cr[6].compare_u32(ctx.r[30].u32, 16 as u32, &mut ctx.xer);
	// 82B9A10C: 40980008  bge cr6, 0x82b9a114
	if !ctx.cr[6].lt {
	pc = 0x82B9A114; continue 'dispatch;
	}
	// 82B9A110: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82B9A114: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9A118: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B9A11C: 4B6928B5  bl 0x8222c9d0
	ctx.lr = 0x82B9A120;
	sub_8222C9D0(ctx, base);
	// 82B9A120: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9A124: 4082000C  bne 0x82b9a130
	if !ctx.cr[0].eq {
	pc = 0x82B9A130; continue 'dispatch;
	}
	// 82B9A128: 935C0050  stw r26, 0x50(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82B9A12C: 48000034  b 0x82b9a160
	pc = 0x82B9A160; continue 'dispatch;
	// 82B9A130: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A134: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B9A138: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9A13C: 419A0020  beq cr6, 0x82b9a15c
	if ctx.cr[6].eq {
	pc = 0x82B9A15C; continue 'dispatch;
	}
	// 82B9A140: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A144: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9A148: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9A14C: 4810F335  bl 0x82ca9480
	ctx.lr = 0x82B9A150;
	sub_82CA9480(ctx, base);
	// 82B9A150: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9A154: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A158: 4B668161  bl 0x822022b8
	ctx.lr = 0x82B9A15C;
	sub_822022B8(ctx, base);
	// 82B9A15C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9A160: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B9A164: 4810F2EC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9A168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9A168 size=1652
    let mut pc: u32 = 0x82B9A168;
    'dispatch: loop {
        match pc {
            0x82B9A168 => {
    //   block [0x82B9A168..0x82B9A7DC)
	// 82B9A168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9A16C: 4810F275  bl 0x82ca93e0
	ctx.lr = 0x82B9A170;
	sub_82CA93D0(ctx, base);
	// 82B9A170: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9A174: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82B9A178: 3A400001  li r18, 1
	ctx.r[18].s64 = 1;
	// 82B9A17C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A180: 925A0054  stw r18, 0x54(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(84 as u32), ctx.r[18].u32 ) };
	// 82B9A184: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9A188: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A18C: 40820648  bne 0x82b9a7d4
	if !ctx.cr[0].eq {
	pc = 0x82B9A7D4; continue 'dispatch;
	}
	// 82B9A190: 817A0130  lwz r11, 0x130(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(304 as u32) ) } as u64;
	// 82B9A194: 3A7A0130  addi r19, r26, 0x130
	ctx.r[19].s64 = ctx.r[26].s64 + 304;
	// 82B9A198: 815A0134  lwz r10, 0x134(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9A19C: 7D7F07B4  extsw r31, r11
	ctx.r[31].s64 = ctx.r[11].s32 as i64;
	// 82B9A1A0: 4800009C  b 0x82b9a23c
	pc = 0x82B9A23C; continue 'dispatch;
	// 82B9A1A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A1A8: 556AC73E  rlwinm r10, r11, 0x18, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B9A1AC: 5569A33E  srwi r9, r11, 0xc
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(12);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9A1B0: 7D4AD214  add r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82B9A1B4: 5568A73E  rlwinm r8, r11, 0x14, 0x1c, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9A1B8: 894A0140  lbz r10, 0x140(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(320 as u32) ) } as u64;
	// 82B9A1BC: 7D4A4838  and r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[9].u64;
	// 82B9A1C0: 554A073E  clrlwi r10, r10, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82B9A1C4: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9A1C8: 419A0068  beq cr6, 0x82b9a230
	if ctx.cr[6].eq {
	pc = 0x82B9A230; continue 'dispatch;
	}
	// 82B9A1CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B9A1D0: 419A0010  beq cr6, 0x82b9a1e0
	if ctx.cr[6].eq {
	pc = 0x82B9A1E0; continue 'dispatch;
	}
	// 82B9A1D4: 514B6426  rlwimi r11, r10, 0xc, 0x10, 0x13
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0FFF);
	// 82B9A1D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9A1DC: 48000054  b 0x82b9a230
	pc = 0x82B9A230; continue 'dispatch;
	// 82B9A1E0: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A1E4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82B9A1E8: 41990044  bgt cr6, 0x82b9a22c
	if ctx.cr[6].gt {
	pc = 0x82B9A22C; continue 'dispatch;
	}
	// 82B9A1EC: 81530004  lwz r10, 4(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A1F0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9A1F4: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9A1F8: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9A1FC: 40980030  bge cr6, 0x82b9a22c
	if !ctx.cr[6].lt {
	pc = 0x82B9A22C; continue 'dispatch;
	}
	// 82B9A200: 7D5F5050  subf r10, r31, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 82B9A204: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82B9A208: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9A20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9A210: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82B9A214: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82B9A218: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9A21C: 48116915  bl 0x82cb0b30
	ctx.lr = 0x82B9A220;
	sub_82CB0B30(ctx, base);
	// 82B9A220: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A224: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82B9A228: 91730004  stw r11, 4(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9A22C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82B9A230: 81530004  lwz r10, 4(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A234: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82B9A238: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A23C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9A240: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9A244: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9A248: 4198FF5C  blt cr6, 0x82b9a1a4
	if ctx.cr[6].lt {
	pc = 0x82B9A1A4; continue 'dispatch;
	}
	// 82B9A24C: 815A0128  lwz r10, 0x128(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(296 as u32) ) } as u64;
	// 82B9A250: 3B3A0124  addi r25, r26, 0x124
	ctx.r[25].s64 = ctx.r[26].s64 + 292;
	// 82B9A254: 817A0124  lwz r11, 0x124(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(292 as u32) ) } as u64;
	// 82B9A258: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82B9A25C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9A260: 7D7B07B4  extsw r27, r11
	ctx.r[27].s64 = ctx.r[11].s32 as i64;
	// 82B9A264: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82B9A268: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9A26C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82B9A270: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82B9A274: 480003B0  b 0x82b9a624
	pc = 0x82B9A624; continue 'dispatch;
	// 82B9A278: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A27C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82B9A280: 82DA0060  lwz r22, 0x60(r26)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9A284: 57FE053E  clrlwi r30, r31, 0x14
	ctx.r[30].u64 = ctx.r[31].u32 as u64 & 0x00000FFFu64;
	// 82B9A288: 1D7E000C  mulli r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 * 12;
	// 82B9A28C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B9A290: 7EABB214  add r21, r11, r22
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 82B9A294: 4800116D  bl 0x82b9b400
	ctx.lr = 0x82B9A298;
	sub_82B9B400(ctx, base);
	// 82B9A298: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82B9A29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82B9A2A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B9A2A4: 419A02C0  beq cr6, 0x82b9a564
	if ctx.cr[6].eq {
	pc = 0x82B9A564; continue 'dispatch;
	}
	// 82B9A2A8: 817BFFFC  lwz r11, -4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9A2AC: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9A2B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9A2B4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9A2B8: 409A001C  bne cr6, 0x82b9a2d4
	if !ctx.cr[6].eq {
	pc = 0x82B9A2D4; continue 'dispatch;
	}
	// 82B9A2BC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A2C0: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A2C4: 40820010  bne 0x82b9a2d4
	if !ctx.cr[0].eq {
	pc = 0x82B9A2D4; continue 'dispatch;
	}
	// 82B9A2C8: 57EB0253  rlwinm. r11, r31, 0, 9, 9
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A2CC: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82B9A2D0: 41820008  beq 0x82b9a2d8
	if ctx.cr[0].eq {
	pc = 0x82B9A2D8; continue 'dispatch;
	}
	// 82B9A2D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9A2D8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A2DC: 41820034  beq 0x82b9a310
	if ctx.cr[0].eq {
	pc = 0x82B9A310; continue 'dispatch;
	}
	// 82B9A2E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A2E4: 81550000  lwz r10, 0(r21)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A2E8: 7D495A78  xor r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82B9A2EC: 55290575  rlwinm. r9, r9, 0, 0x15, 0x1a
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9A2F0: 40820020  bne 0x82b9a310
	if !ctx.cr[0].eq {
	pc = 0x82B9A310; continue 'dispatch;
	}
	// 82B9A2F4: 7D495A78  xor r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82B9A2F8: 55290529  rlwinm. r9, r9, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9A2FC: 40820014  bne 0x82b9a310
	if !ctx.cr[0].eq {
	pc = 0x82B9A310; continue 'dispatch;
	}
	// 82B9A300: 7D4B5A78  xor r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82B9A304: 556B0003  rlwinm. r11, r11, 0, 0, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A308: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82B9A30C: 41820008  beq 0x82b9a314
	if ctx.cr[0].eq {
	pc = 0x82B9A314; continue 'dispatch;
	}
	// 82B9A310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9A314: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A318: 41820030  beq 0x82b9a348
	if ctx.cr[0].eq {
	pc = 0x82B9A348; continue 'dispatch;
	}
	// 82B9A31C: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A320: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82B9A324: 5569053E  clrlwi r9, r11, 0x14
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9A328: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9A32C: 7D29F050  subf r9, r9, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	// 82B9A330: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9A334: 7D4A4830  slw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9A338: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82B9A33C: 556B053F  clrlwi. r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A340: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82B9A344: 41820008  beq 0x82b9a34c
	if ctx.cr[0].eq {
	pc = 0x82B9A34C; continue 'dispatch;
	}
	// 82B9A348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9A34C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A350: 5567063E  clrlwi r7, r11, 0x18
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B9A354: 554B0FFF  rlwinm. r11, r10, 1, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A358: 40820010  bne 0x82b9a368
	if !ctx.cr[0].eq {
	pc = 0x82B9A368; continue 'dispatch;
	}
	// 82B9A35C: 81550004  lwz r10, 4(r21)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A360: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9A364: 41820040  beq 0x82b9a3a4
	if ctx.cr[0].eq {
	pc = 0x82B9A3A4; continue 'dispatch;
	}
	// 82B9A368: 54EA063F  clrlwi. r10, r7, 0x18
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9A36C: 41820030  beq 0x82b9a39c
	if ctx.cr[0].eq {
	pc = 0x82B9A39C; continue 'dispatch;
	}
	// 82B9A370: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9A374: 419A0028  beq cr6, 0x82b9a39c
	if ctx.cr[6].eq {
	pc = 0x82B9A39C; continue 'dispatch;
	}
	// 82B9A378: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A37C: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A380: 4182001C  beq 0x82b9a39c
	if ctx.cr[0].eq {
	pc = 0x82B9A39C; continue 'dispatch;
	}
	// 82B9A384: 81750008  lwz r11, 8(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9A388: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9A38C: 7D6B5278  xor r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[10].u64;
	// 82B9A390: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A394: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82B9A398: 41820008  beq 0x82b9a3a0
	if ctx.cr[0].eq {
	pc = 0x82B9A3A0; continue 'dispatch;
	}
	// 82B9A39C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9A3A0: 5567063E  clrlwi r7, r11, 0x18
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B9A3A4: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A3A8: 418201BC  beq 0x82b9a564
	if ctx.cr[0].eq {
	pc = 0x82B9A564; continue 'dispatch;
	}
	// 82B9A3AC: 7F17C040  cmplw cr6, r23, r24
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82B9A3B0: 419A01B4  beq cr6, 0x82b9a564
	if ctx.cr[6].eq {
	pc = 0x82B9A564; continue 'dispatch;
	}
	// 82B9A3B4: 39770008  addi r11, r23, 8
	ctx.r[11].s64 = ctx.r[23].s64 + 8;
	// 82B9A3B8: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82B9A3BC: 409A0028  bne cr6, 0x82b9a3e4
	if !ctx.cr[6].eq {
	pc = 0x82B9A3E4; continue 'dispatch;
	}
	// 82B9A3C0: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A3C4: 556B0426  rlwinm r11, r11, 0, 0x10, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9A3C8: 2B0B1000  cmplwi cr6, r11, 0x1000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4096 as u32, &mut ctx.xer);
	// 82B9A3CC: 409A0010  bne cr6, 0x82b9a3dc
	if !ctx.cr[6].eq {
	pc = 0x82B9A3DC; continue 'dispatch;
	}
	// 82B9A3D0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A3D4: 556B0421  rlwinm. r11, r11, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A3D8: 4082000C  bne 0x82b9a3e4
	if !ctx.cr[0].eq {
	pc = 0x82B9A3E4; continue 'dispatch;
	}
	// 82B9A3DC: 7E4A9378  mr r10, r18
	ctx.r[10].u64 = ctx.r[18].u64;
	// 82B9A3E0: 48000008  b 0x82b9a3e8
	pc = 0x82B9A3E8; continue 'dispatch;
	// 82B9A3E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9A3E8: 80D70004  lwz r6, 4(r23)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A3EC: 5547063E  clrlwi r7, r10, 0x18
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9A3F0: 80B80004  lwz r5, 4(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A3F4: 54CBA73E  rlwinm r11, r6, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x00000FFFu64;
	// 82B9A3F8: 54AAA73E  rlwinm r10, r5, 0x14, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00000FFFu64;
	// 82B9A3FC: 7E495830  slw r9, r18, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[18].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9A400: 7E485030  slw r8, r18, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[18].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9A404: 712B6018  andi. r11, r9, 0x6018
	ctx.r[11].u64 = ctx.r[9].u64 & 24600;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A408: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9A40C: 710A6018  andi. r10, r8, 0x6018
	ctx.r[10].u64 = ctx.r[8].u64 & 24600;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9A410: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9A414: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82B9A418: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82B9A41C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9A420: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A424: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82B9A428: 4082000C  bne 0x82b9a434
	if !ctx.cr[0].eq {
	pc = 0x82B9A434; continue 'dispatch;
	}
	// 82B9A42C: 5544063F  clrlwi. r4, r10, 0x18
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82B9A430: 41820040  beq 0x82b9a470
	if ctx.cr[0].eq {
	pc = 0x82B9A470; continue 'dispatch;
	}
	// 82B9A434: 54E7063F  clrlwi. r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82B9A438: 41820030  beq 0x82b9a468
	if ctx.cr[0].eq {
	pc = 0x82B9A468; continue 'dispatch;
	}
	// 82B9A43C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9A440: 419A0028  beq cr6, 0x82b9a468
	if ctx.cr[6].eq {
	pc = 0x82B9A468; continue 'dispatch;
	}
	// 82B9A444: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A448: 41820020  beq 0x82b9a468
	if ctx.cr[0].eq {
	pc = 0x82B9A468; continue 'dispatch;
	}
	// 82B9A44C: 7CCB2A78  xor r11, r6, r5
	ctx.r[11].u64 = ctx.r[6].u64 ^ ctx.r[5].u64;
	// 82B9A450: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A454: 40820014  bne 0x82b9a468
	if !ctx.cr[0].eq {
	pc = 0x82B9A468; continue 'dispatch;
	}
	// 82B9A458: 7CCB2A78  xor r11, r6, r5
	ctx.r[11].u64 = ctx.r[6].u64 ^ ctx.r[5].u64;
	// 82B9A45C: 556B05BB  rlwinm. r11, r11, 0, 0x16, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A460: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82B9A464: 41820008  beq 0x82b9a46c
	if ctx.cr[0].eq {
	pc = 0x82B9A46C; continue 'dispatch;
	}
	// 82B9A468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9A46C: 5567063E  clrlwi r7, r11, 0x18
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B9A470: 552B0674  rlwinm r11, r9, 0, 0x19, 0x1a
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9A474: 550A0674  rlwinm r10, r8, 0, 0x19, 0x1a
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9A478: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9A47C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82B9A480: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9A484: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9A488: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82B9A48C: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82B9A490: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A494: 4082000C  bne 0x82b9a4a0
	if !ctx.cr[0].eq {
	pc = 0x82B9A4A0; continue 'dispatch;
	}
	// 82B9A498: 5549063F  clrlwi. r9, r10, 0x18
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9A49C: 41820048  beq 0x82b9a4e4
	if ctx.cr[0].eq {
	pc = 0x82B9A4E4; continue 'dispatch;
	}
	// 82B9A4A0: 54E9063F  clrlwi. r9, r7, 0x18
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9A4A4: 41820038  beq 0x82b9a4dc
	if ctx.cr[0].eq {
	pc = 0x82B9A4DC; continue 'dispatch;
	}
	// 82B9A4A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9A4AC: 419A0030  beq cr6, 0x82b9a4dc
	if ctx.cr[6].eq {
	pc = 0x82B9A4DC; continue 'dispatch;
	}
	// 82B9A4B0: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A4B4: 41820028  beq 0x82b9a4dc
	if ctx.cr[0].eq {
	pc = 0x82B9A4DC; continue 'dispatch;
	}
	// 82B9A4B8: 7CCB2A78  xor r11, r6, r5
	ctx.r[11].u64 = ctx.r[6].u64 ^ ctx.r[5].u64;
	// 82B9A4BC: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A4C0: 4082001C  bne 0x82b9a4dc
	if !ctx.cr[0].eq {
	pc = 0x82B9A4DC; continue 'dispatch;
	}
	// 82B9A4C4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82B9A4C8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82B9A4CC: 4800142D  bl 0x82b9b8f8
	ctx.lr = 0x82B9A4D0;
	sub_82B9B8F8(ctx, base);
	// 82B9A4D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A4D4: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82B9A4D8: 40820008  bne 0x82b9a4e0
	if !ctx.cr[0].eq {
	pc = 0x82B9A4E0; continue 'dispatch;
	}
	// 82B9A4DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9A4E0: 5567063E  clrlwi r7, r11, 0x18
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B9A4E4: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A4E8: 4182007C  beq 0x82b9a564
	if ctx.cr[0].eq {
	pc = 0x82B9A564; continue 'dispatch;
	}
	// 82B9A4EC: 815A006C  lwz r10, 0x6c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9A4F0: 397A006C  addi r11, r26, 0x6c
	ctx.r[11].s64 = ctx.r[26].s64 + 108;
	// 82B9A4F4: 813A0070  lwz r9, 0x70(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9A4F8: 5548003E  slwi r8, r10, 0
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9A4FC: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9A500: 7CCAC050  subf r6, r10, r24
	ctx.r[6].s64 = ctx.r[24].s64 - ctx.r[10].s64;
	// 82B9A504: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B9A508: 7CC81E70  srawi r8, r6, 3
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[6].s32 >> 3) as i64;
	// 82B9A50C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9A510: 40980054  bge cr6, 0x82b9a564
	if !ctx.cr[6].lt {
	pc = 0x82B9A564; continue 'dispatch;
	}
	// 82B9A514: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A518: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A51C: 54CB1838  slwi r11, r6, 3
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9A520: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9A524: 54E9063F  clrlwi. r9, r7, 0x18
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9A528: 4182003C  beq 0x82b9a564
	if ctx.cr[0].eq {
	pc = 0x82B9A564; continue 'dispatch;
	}
	// 82B9A52C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A530: 5529A73E  rlwinm r9, r9, 0x14, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82B9A534: 7E494830  slw r9, r18, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[18].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9A538: 71290B80  andi. r9, r9, 0xb80
	ctx.r[9].u64 = ctx.r[9].u64 & 2944;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9A53C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9A540: 41820018  beq 0x82b9a558
	if ctx.cr[0].eq {
	pc = 0x82B9A558; continue 'dispatch;
	}
	// 82B9A544: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A548: 552904FE  clrlwi r9, r9, 0x13
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00001FFFu64;
	// 82B9A54C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9A550: 409A0008  bne cr6, 0x82b9a558
	if !ctx.cr[6].eq {
	pc = 0x82B9A558; continue 'dispatch;
	}
	// 82B9A554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82B9A558: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82B9A55C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9A560: 4198FFC4  blt cr6, 0x82b9a524
	if ctx.cr[6].lt {
	pc = 0x82B9A524; continue 'dispatch;
	}
	// 82B9A564: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A568: 40820024  bne 0x82b9a58c
	if !ctx.cr[0].eq {
	pc = 0x82B9A58C; continue 'dispatch;
	}
	// 82B9A56C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82B9A570: 419A0094  beq cr6, 0x82b9a604
	if ctx.cr[6].eq {
	pc = 0x82B9A604; continue 'dispatch;
	}
	// 82B9A574: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A578: 4182008C  beq 0x82b9a604
	if ctx.cr[0].eq {
	pc = 0x82B9A604; continue 'dispatch;
	}
	// 82B9A57C: 817BFFFC  lwz r11, -4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9A580: 7E549378  mr r20, r18
	ctx.r[20].u64 = ctx.r[18].u64;
	// 82B9A584: 524BA296  rlwimi r11, r18, 0x14, 0xa, 0xb
	ctx.r[11].u64 = (((ctx.r[18].u32).rotate_left(20) as u64) & 0x0000000000300000) | (ctx.r[11].u64 & 0xFFFFFFFFFFCFFFFF);
	// 82B9A588: 48000078  b 0x82b9a600
	pc = 0x82B9A600; continue 'dispatch;
	// 82B9A58C: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A590: 40820074  bne 0x82b9a604
	if !ctx.cr[0].eq {
	pc = 0x82B9A604; continue 'dispatch;
	}
	// 82B9A594: 817BFFFC  lwz r11, -4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9A598: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82B9A59C: 556A053E  clrlwi r10, r11, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9A5A0: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 * 12;
	// 82B9A5A4: 7D2AB214  add r9, r10, r22
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[22].u64;
	// 82B9A5A8: 7D4AB02E  lwzx r10, r10, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82B9A5AC: 5547C9FE  srwi r7, r10, 7
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9A5B0: 7CE75278  xor r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 ^ ctx.r[10].u64;
	// 82B9A5B4: 54E70575  rlwinm. r7, r7, 0, 0x15, 0x1a
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82B9A5B8: 40820040  bne 0x82b9a5f8
	if !ctx.cr[0].eq {
	pc = 0x82B9A5F8; continue 'dispatch;
	}
	// 82B9A5BC: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A5C0: 554A273A  rlwinm r10, r10, 4, 0x1c, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0FFFFFFFu64;
	// 82B9A5C4: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82B9A5C8: 5526077E  clrlwi r6, r9, 0x1d
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9A5CC: 5525EF7E  rlwinm r5, r9, 0x1d, 0x1d, 0x1f
	ctx.r[5].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9A5D0: 5524D77E  rlwinm r4, r9, 0x1a, 0x1d, 0x1f
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 82B9A5D4: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82B9A5D8: 5529BF7E  rlwinm r9, r9, 0x17, 0x1d, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000001FFu64;
	// 82B9A5DC: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82B9A5E0: 90810068  stw r4, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82B9A5E4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82B9A5E8: 7D4A382E  lwzx r10, r10, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82B9A5EC: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 82B9A5F0: 419A0008  beq cr6, 0x82b9a5f8
	if ctx.cr[6].eq {
	pc = 0x82B9A5F8; continue 'dispatch;
	}
	// 82B9A5F4: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82B9A5F8: 510BA296  rlwimi r11, r8, 0x14, 0xa, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(20) as u64) & 0x0000000000300000) | (ctx.r[11].u64 & 0xFFFFFFFFFFCFFFFF);
	// 82B9A5FC: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82B9A600: 917BFFFC  stw r11, -4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82B9A604: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A608: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82B9A60C: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A610: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82B9A614: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9A618: 7F17C378  mr r23, r24
	ctx.r[23].u64 = ctx.r[24].u64;
	// 82B9A61C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9A620: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 82B9A624: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9A628: 4198FC50  blt cr6, 0x82b9a278
	if ctx.cr[6].lt {
	pc = 0x82B9A278; continue 'dispatch;
	}
	// 82B9A62C: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A630: 41820084  beq 0x82b9a6b4
	if ctx.cr[0].eq {
	pc = 0x82B9A6B4; continue 'dispatch;
	}
	// 82B9A634: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A638: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82B9A63C: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A640: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9A644: 813A0060  lwz r9, 0x60(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9A648: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9A64C: 8148FFFC  lwz r10, -4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9A650: 554B053E  clrlwi r11, r10, 0x14
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9A654: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B9A658: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9A65C: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A660: 5566C9FE  srwi r6, r11, 7
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9A664: 7CC65A78  xor r6, r6, r11
	ctx.r[6].u64 = ctx.r[6].u64 ^ ctx.r[11].u64;
	// 82B9A668: 54C60575  rlwinm. r6, r6, 0, 0x15, 0x1a
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82B9A66C: 40820040  bne 0x82b9a6ac
	if !ctx.cr[0].eq {
	pc = 0x82B9A6AC; continue 'dispatch;
	}
	// 82B9A670: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A674: 556B273A  rlwinm r11, r11, 4, 0x1c, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 82B9A678: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82B9A67C: 5525077E  clrlwi r5, r9, 0x1d
	ctx.r[5].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9A680: 5524EF7E  rlwinm r4, r9, 0x1d, 0x1d, 0x1f
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9A684: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82B9A688: 5525D77E  rlwinm r5, r9, 0x1a, 0x1d, 0x1f
	ctx.r[5].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 82B9A68C: 90810064  stw r4, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[4].u32 ) };
	// 82B9A690: 5529BF7E  rlwinm r9, r9, 0x17, 0x1d, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000001FFu64;
	// 82B9A694: 90A10068  stw r5, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[5].u32 ) };
	// 82B9A698: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82B9A69C: 7D6B302E  lwzx r11, r11, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82B9A6A0: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82B9A6A4: 419A0008  beq cr6, 0x82b9a6ac
	if ctx.cr[6].eq {
	pc = 0x82B9A6AC; continue 'dispatch;
	}
	// 82B9A6A8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82B9A6AC: 50EAA296  rlwimi r10, r7, 0x14, 0xa, 0xb
	ctx.r[10].u64 = (((ctx.r[7].u32).rotate_left(20) as u64) & 0x0000000000300000) | (ctx.r[10].u64 & 0xFFFFFFFFFFCFFFFF);
	// 82B9A6B0: 9148FFFC  stw r10, -4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 82B9A6B4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A6B8: 816B4DB8  lwz r11, 0x4db8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9A6BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A6C0: 419A0114  beq cr6, 0x82b9a7d4
	if ctx.cr[6].eq {
	pc = 0x82B9A7D4; continue 'dispatch;
	}
	// 82B9A6C4: 83F30000  lwz r31, 0(r19)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A6C8: 48000030  b 0x82b9a6f8
	pc = 0x82B9A6F8; continue 'dispatch;
	// 82B9A6CC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A6D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A6D4: 5546A73E  rlwinm r6, r10, 0x14, 0x1c, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9A6D8: 5545063E  clrlwi r5, r10, 0x18
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9A6DC: 806B4DB8  lwz r3, 0x4db8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9A6E0: 5544C73E  rlwinm r4, r10, 0x18, 0x1c, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9A6E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A6E8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B9A6EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9A6F0: 4E800421  bctrl
	ctx.lr = 0x82B9A6F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9A6F4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82B9A6F8: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A6FC: 81530000  lwz r10, 0(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A700: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9A704: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9A708: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9A70C: 4198FFC0  blt cr6, 0x82b9a6cc
	if ctx.cr[6].lt {
	pc = 0x82B9A6CC; continue 'dispatch;
	}
	// 82B9A710: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A714: 480000A8  b 0x82b9a7bc
	pc = 0x82B9A7BC; continue 'dispatch;
	// 82B9A718: 817A0104  lwz r11, 0x104(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(260 as u32) ) } as u64;
	// 82B9A71C: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82B9A720: 813A0100  lwz r9, 0x100(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(256 as u32) ) } as u64;
	// 82B9A724: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82B9A728: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A72C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82B9A730: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82B9A734: 550B053E  clrlwi r11, r8, 0x14
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x00000FFFu64;
	// 82B9A738: 7FC953D6  divw r30, r9, r10
	ctx.r[30].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	// 82B9A73C: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82B9A740: 48000E39  bl 0x82b9b578
	ctx.lr = 0x82B9A744;
	sub_82B9B578(ctx, base);
	// 82B9A744: 80C30004  lwz r6, 4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A748: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A74C: 54C805F2  rlwinm r8, r6, 0, 0x17, 0x19
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9A750: 54C9052C  rlwinm r9, r6, 0, 0x14, 0x16
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9A754: 54C706B8  rlwinm r7, r6, 0, 0x1a, 0x1c
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9A758: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B9A75C: 556A67BE  rlwinm r10, r11, 0xc, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82B9A760: 54C8077E  clrlwi r8, r6, 0x1d
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 82B9A764: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82B9A768: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82B9A76C: 7CC94214  add r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B9A770: 419A0010  beq cr6, 0x82b9a780
	if ctx.cr[6].eq {
	pc = 0x82B9A780; continue 'dispatch;
	}
	// 82B9A774: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 82B9A778: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82B9A77C: 409A0008  bne cr6, 0x82b9a784
	if !ctx.cr[6].eq {
	pc = 0x82B9A784; continue 'dispatch;
	}
	// 82B9A780: 7E489378  mr r8, r18
	ctx.r[8].u64 = ctx.r[18].u64;
	// 82B9A784: 80BA0000  lwz r5, 0(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A788: 556AC636  rlwinm r10, r11, 0x18, 0x18, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B9A78C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A790: 5569873E  rlwinm r9, r11, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9A794: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9A798: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82B9A79C: 80654DB8  lwz r3, 0x4db8(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9A7A0: 5485A6BE  rlwinm r5, r4, 0x14, 0x1a, 0x1f
	ctx.r[5].u64 = ctx.r[4].u32 as u64 & 0x00000FFFu64;
	// 82B9A7A4: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82B9A7A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A7AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82B9A7B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9A7B4: 4E800421  bctrl
	ctx.lr = 0x82B9A7B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9A7B8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82B9A7BC: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A7C0: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A7C4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9A7C8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9A7CC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9A7D0: 4198FF48  blt cr6, 0x82b9a718
	if ctx.cr[6].lt {
	pc = 0x82B9A718; continue 'dispatch;
	}
	// 82B9A7D4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82B9A7D8: 4810EC58  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9A7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9A7E0 size=2528
    let mut pc: u32 = 0x82B9A7E0;
    'dispatch: loop {
        match pc {
            0x82B9A7E0 => {
    //   block [0x82B9A7E0..0x82B9B1C0)
	// 82B9A7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9A7E4: 4810EC19  bl 0x82ca93fc
	ctx.lr = 0x82B9A7E8;
	sub_82CA93D0(ctx, base);
	// 82B9A7E8: 9421FBE0  stwu r1, -0x420(r1)
	ea = ctx.r[1].u32.wrapping_add(-1056 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9A7EC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82B9A7F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B9A7F4: 3BF90014  addi r31, r25, 0x14
	ctx.r[31].s64 = ctx.r[25].s64 + 20;
	// 82B9A7F8: 8119001C  lwz r8, 0x1c(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28 as u32) ) } as u64;
	// 82B9A7FC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82B9A800: 409A0060  bne cr6, 0x82b9a860
	if !ctx.cr[6].eq {
	pc = 0x82B9A860; continue 'dispatch;
	}
	// 82B9A804: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82B9A808: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9A80C: 48001705  bl 0x82b9bf10
	ctx.lr = 0x82B9A810;
	sub_82B9BF10(ctx, base);
	// 82B9A810: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A814: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9A818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9A81C: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9A820: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A824: 41820010  beq 0x82b9a834
	if ctx.cr[0].eq {
	pc = 0x82B9A834; continue 'dispatch;
	}
	// 82B9A828: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82B9A82C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82B9A830: 48000024  b 0x82b9a854
	pc = 0x82B9A854; continue 'dispatch;
	// 82B9A834: 815E013C  lwz r10, 0x13c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(316 as u32) ) } as u64;
	// 82B9A838: 817E0134  lwz r11, 0x134(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9A83C: 813E0128  lwz r9, 0x128(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(296 as u32) ) } as u64;
	// 82B9A840: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9A844: 815E011C  lwz r10, 0x11c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) } as u64;
	// 82B9A848: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9A84C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9A850: 396B0009  addi r11, r11, 9
	ctx.r[11].s64 = ctx.r[11].s64 + 9;
	// 82B9A854: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9A858: 4BFFF069  bl 0x82b998c0
	ctx.lr = 0x82B9A85C;
	sub_82B998C0(ctx, base);
	// 82B9A85C: 48000940  b 0x82b9b19c
	pc = 0x82B9B19C; continue 'dispatch;
	// 82B9A860: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A864: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9A868: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9A86C: 418203F8  beq 0x82b9ac64
	if ctx.cr[0].eq {
	pc = 0x82B9AC64; continue 'dispatch;
	}
	// 82B9A870: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82B9A874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82B9A878: 811E010C  lwz r8, 0x10c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(268 as u32) ) } as u64;
	// 82B9A87C: 3B9E010C  addi r28, r30, 0x10c
	ctx.r[28].s64 = ctx.r[30].s64 + 268;
	// 82B9A880: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9A884: 912100B0  stw r9, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[9].u32 ) };
	// 82B9A888: 7D0B07B4  extsw r11, r8
	ctx.r[11].s64 = ctx.r[8].s32 as i64;
	// 82B9A88C: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82B9A890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9A894: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9A898: 4098006C  bge cr6, 0x82b9a904
	if !ctx.cr[6].lt {
	pc = 0x82B9A904; continue 'dispatch;
	}
	// 82B9A89C: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A8A0: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A8A4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9A8A8: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82B9A8AC: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A8B0: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9A8B4: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A8B8: 388100B4  addi r4, r1, 0xb4
	ctx.r[4].s64 = ctx.r[1].s64 + 180;
	// 82B9A8BC: 54E98FFE  rlwinm r9, r7, 0x11, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x00007FFFu64;
	// 82B9A8C0: 54A5683E  rotlwi r5, r5, 0xd
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(13)) as u64;
	// 82B9A8C4: 54E3963E  rlwinm r3, r7, 0x12, 0x18, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x00003FFFu64;
	// 82B9A8C8: 7CA54B78  or r5, r5, r9
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	// 82B9A8CC: 54FDE426  rlwinm r29, r7, 0x1c, 0x10, 0x13
	ctx.r[29].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	// 82B9A8D0: 50A3083C  rlwimi r3, r5, 1, 0, 0x1e
	ctx.r[3].u64 = (((ctx.r[5].u32).rotate_left(1) as u64) & 0x00000000FFFFFFFE) | (ctx.r[3].u64 & 0xFFFFFFFF00000001);
	// 82B9A8D4: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9A8D8: 5465502A  slwi r5, r3, 0xa
	ctx.r[5].u32 = ctx.r[3].u32.wrapping_shl(10);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9A8DC: 54E70216  rlwinm r7, r7, 0, 8, 0xb
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9A8E0: 7CA9EB78  or r9, r5, r29
	ctx.r[9].u64 = ctx.r[5].u64 | ctx.r[29].u64;
	// 82B9A8E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9A8E8: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9A8EC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9A8F0: 7D26212E  stwx r9, r6, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u32) };
	// 82B9A8F4: 812100B0  lwz r9, 0xb0(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82B9A8F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82B9A8FC: 912100B0  stw r9, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[9].u32 ) };
	// 82B9A900: 4198FFAC  blt cr6, 0x82b9a8ac
	if ctx.cr[6].lt {
	pc = 0x82B9A8AC; continue 'dispatch;
	}
	// 82B9A904: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82B9A908: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82B9A90C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9A910: 409A000C  bne cr6, 0x82b9a91c
	if !ctx.cr[6].eq {
	pc = 0x82B9A91C; continue 'dispatch;
	}
	// 82B9A914: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9A918: 48000018  b 0x82b9a930
	pc = 0x82B9A930; continue 'dispatch;
	// 82B9A91C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B9A920: 409A000C  bne cr6, 0x82b9a92c
	if !ctx.cr[6].eq {
	pc = 0x82B9A92C; continue 'dispatch;
	}
	// 82B9A924: 935E0150  stw r26, 0x150(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[26].u32 ) };
	// 82B9A928: 4800000C  b 0x82b9a934
	pc = 0x82B9A934; continue 'dispatch;
	// 82B9A92C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82B9A930: 917E0150  stw r11, 0x150(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82B9A934: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A938: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A93C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9A940: 396A0020  addi r11, r10, 0x20
	ctx.r[11].s64 = ctx.r[10].s64 + 32;
	// 82B9A944: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9A948: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9A94C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9A950: 40990008  ble cr6, 0x82b9a958
	if !ctx.cr[6].gt {
	pc = 0x82B9A958; continue 'dispatch;
	}
	// 82B9A954: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9A958: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9A95C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82B9A960: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9A964: 41990008  bgt cr6, 0x82b9a96c
	if ctx.cr[6].gt {
	pc = 0x82B9A96C; continue 'dispatch;
	}
	// 82B9A968: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82B9A96C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82B9A970: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82B9A974: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9A978: 48001599  bl 0x82b9bf10
	ctx.lr = 0x82B9A97C;
	sub_82B9BF10(ctx, base);
	// 82B9A97C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A980: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82B9A984: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9A988: 5564EFFE  rlwinm r4, r11, 0x1d, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9A98C: 4BFFEE35  bl 0x82b997c0
	ctx.lr = 0x82B9A990;
	sub_82B997C0(ctx, base);
	// 82B9A990: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9A994: 40820014  bne 0x82b9a9a8
	if !ctx.cr[0].eq {
	pc = 0x82B9A9A8; continue 'dispatch;
	}
	// 82B9A998: 3D608876  lis r11, -0x778a
	ctx.r[11].s64 = -2005532672;
	// 82B9A99C: 616B0B81  ori r11, r11, 0xb81
	ctx.r[11].u64 = ctx.r[11].u64 | 2945;
	// 82B9A9A0: 917E0050  stw r11, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9A9A4: 480007F8  b 0x82b9b19c
	pc = 0x82B9B19C; continue 'dispatch;
	// 82B9A9A8: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82B9A9AC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A9B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9A9B4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9A9B8: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9A9BC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9A9C0: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9A9C4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9A9C8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9A9CC: 40990008  ble cr6, 0x82b9a9d4
	if !ctx.cr[6].gt {
	pc = 0x82B9A9D4; continue 'dispatch;
	}
	// 82B9A9D0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9A9D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9A9D8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9A9DC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9A9E0: 40990008  ble cr6, 0x82b9a9e8
	if !ctx.cr[6].gt {
	pc = 0x82B9A9E8; continue 'dispatch;
	}
	// 82B9A9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82B9A9E8: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82B9A9EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9A9F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9A9F4: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82B9A9F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9A9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9AA00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82B9AA04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9AA08: 40990080  ble cr6, 0x82b9aa88
	if !ctx.cr[6].gt {
	pc = 0x82B9AA88; continue 'dispatch;
	}
	// 82B9AA0C: 38E100B4  addi r7, r1, 0xb4
	ctx.r[7].s64 = ctx.r[1].s64 + 180;
	// 82B9AA10: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AA14: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9AA18: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82B9AA1C: 514B843E  rlwimi r11, r10, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82B9AA20: 5163C53E  rlwimi r3, r11, 0x18, 0x14, 0x1f
	ctx.r[3].u64 = (((ctx.r[11].u32).rotate_left(24) as u64) & 0x0000000000000FFF) | (ctx.r[3].u64 & 0xFFFFFFFFFFFFF000);
	// 82B9AA24: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82B9AA28: 546BC73E  rlwinm r11, r3, 0x18, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82B9AA2C: 90690000  stw r3, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82B9AA30: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82B9AA34: 40990008  ble cr6, 0x82b9aa3c
	if !ctx.cr[6].gt {
	pc = 0x82B9AA3C; continue 'dispatch;
	}
	// 82B9AA38: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82B9AA3C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B9AA40: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82B9AA44: 4099000C  ble cr6, 0x82b9aa50
	if !ctx.cr[6].gt {
	pc = 0x82B9AA50; continue 'dispatch;
	}
	// 82B9AA48: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82B9AA4C: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82B9AA50: 7D5F50F8  nor r31, r10, r10
	ctx.r[31].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82B9AA54: 837E0110  lwz r27, 0x110(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82B9AA58: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82B9AA5C: 57FFAFFE  rlwinm r31, r31, 0x15, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0x000007FFu64;
	// 82B9AA60: 554AB7FE  rlwinm r10, r10, 0x16, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000003FFu64;
	// 82B9AA64: 7FFF5830  slw r31, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9AA68: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9AA6C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82B9AA70: 7FE52B78  or r5, r31, r5
	ctx.r[5].u64 = ctx.r[31].u64 | ctx.r[5].u64;
	// 82B9AA74: 7D642378  or r4, r11, r4
	ctx.r[4].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82B9AA78: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82B9AA7C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9AA80: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82B9AA84: 4198FF8C  blt cr6, 0x82b9aa10
	if ctx.cr[6].lt {
	pc = 0x82B9AA10; continue 'dispatch;
	}
	// 82B9AA88: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9AA8C: 556A6FFE  rlwinm r10, r11, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0007FFFFu64;
	// 82B9AA90: 556B77FE  rlwinm r11, r11, 0xe, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0003FFFFu64;
	// 82B9AA94: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9AA98: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82B9AA9C: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 82B9AAA0: 40990008  ble cr6, 0x82b9aaa8
	if !ctx.cr[6].gt {
	pc = 0x82B9AAA8; continue 'dispatch;
	}
	// 82B9AAA4: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82B9AAA8: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82B9AAAC: 40990008  ble cr6, 0x82b9aab4
	if !ctx.cr[6].gt {
	pc = 0x82B9AAB4; continue 'dispatch;
	}
	// 82B9AAB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9AAB4: B0BD0018  sth r5, 0x18(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[5].u16 ) };
	// 82B9AAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9AABC: B09D001A  sth r4, 0x1a(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(26 as u32), ctx.r[4].u16 ) };
	// 82B9AAC0: 394101E0  addi r10, r1, 0x1e0
	ctx.r[10].s64 = ctx.r[1].s64 + 480;
	// 82B9AAC4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9AAC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9AACC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9AAD0: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 82B9AAD4: 4198FFF0  blt cr6, 0x82b9aac4
	if ctx.cr[6].lt {
	pc = 0x82B9AAC4; continue 'dispatch;
	}
	// 82B9AAD8: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82B9AADC: 394100B4  addi r10, r1, 0xb4
	ctx.r[10].s64 = ctx.r[1].s64 + 180;
	// 82B9AAE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9AAE4: 419A0024  beq cr6, 0x82b9ab08
	if ctx.cr[6].eq {
	pc = 0x82B9AB08; continue 'dispatch;
	}
	// 82B9AAE8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AAEC: 390101E0  addi r8, r1, 0x1e0
	ctx.r[8].s64 = ctx.r[1].s64 + 480;
	// 82B9AAF0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9AAF4: 552776BA  rlwinm r7, r9, 0xe, 0x1a, 0x1d
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0003FFFFu64;
	// 82B9AAF8: 5529873E  rlwinm r9, r9, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82B9AAFC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9AB00: 7D27412E  stwx r9, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82B9AB04: 4082FFE4  bne 0x82b9aae8
	if !ctx.cr[0].eq {
	pc = 0x82B9AAE8; continue 'dispatch;
	}
	// 82B9AB08: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9AB0C: 38E101E0  addi r7, r1, 0x1e0
	ctx.r[7].s64 = ctx.r[1].s64 + 480;
	// 82B9AB10: 38CBB658  addi r6, r11, -0x49a8
	ctx.r[6].s64 = ctx.r[11].s64 + -18856;
	// 82B9AB14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9AB18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9AB1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9AB20: 480017D1  bl 0x82b9c2f0
	ctx.lr = 0x82B9AB24;
	sub_82B9C2F0(ctx, base);
	// 82B9AB24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AB28: 806B4DB8  lwz r3, 0x4db8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9AB2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82B9AB30: 419A0018  beq cr6, 0x82b9ab48
	if ctx.cr[6].eq {
	pc = 0x82B9AB48; continue 'dispatch;
	}
	// 82B9AB34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AB38: 388101E0  addi r4, r1, 0x1e0
	ctx.r[4].s64 = ctx.r[1].s64 + 480;
	// 82B9AB3C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B9AB40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9AB44: 4E800421  bctrl
	ctx.lr = 0x82B9AB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9AB48: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9AB4C: 556BC6BF  rlwinm. r11, r11, 0x18, 0x1a, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9AB50: 4182000C  beq 0x82b9ab5c
	if ctx.cr[0].eq {
	pc = 0x82B9AB5C; continue 'dispatch;
	}
	// 82B9AB54: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9AB58: 48000020  b 0x82b9ab78
	pc = 0x82B9AB78; continue 'dispatch;
	// 82B9AB5C: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9AB60: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82B9AB64: 38CBB7E0  addi r6, r11, -0x4820
	ctx.r[6].s64 = ctx.r[11].s64 + -18464;
	// 82B9AB68: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9AB6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9AB70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9AB74: 4800177D  bl 0x82b9c2f0
	ctx.lr = 0x82B9AB78;
	sub_82B9C2F0(ctx, base);
	// 82B9AB78: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9AB7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9AB80: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B9AB84: 3D0082BA  lis r8, -0x7d46
	ctx.r[8].s64 = -2101739520;
	// 82B9AB88: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82B9AB8C: 512B44AE  rlwimi r11, r9, 8, 0x12, 0x17
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x0000000000003F00) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFC0FF);
	// 82B9AB90: 38C8BC08  addi r6, r8, -0x43f8
	ctx.r[6].s64 = ctx.r[8].s64 + -17400;
	// 82B9AB94: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9AB98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9AB9C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82B9ABA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9ABA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9ABA8: 48001749  bl 0x82b9c2f0
	ctx.lr = 0x82B9ABAC;
	sub_82B9C2F0(ctx, base);
	// 82B9ABAC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9ABB0: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82B9ABB4: 915D001C  stw r10, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82B9ABB8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9ABBC: 55490739  rlwinm. r9, r10, 0, 0x1c, 0x1c
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9ABC0: 4182000C  beq 0x82b9abcc
	if ctx.cr[0].eq {
	pc = 0x82B9ABCC; continue 'dispatch;
	}
	// 82B9ABC4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82B9ABC8: 48000020  b 0x82b9abe8
	pc = 0x82B9ABE8; continue 'dispatch;
	// 82B9ABCC: 5549077B  rlwinm. r9, r10, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9ABD0: 4182000C  beq 0x82b9abdc
	if ctx.cr[0].eq {
	pc = 0x82B9ABDC; continue 'dispatch;
	}
	// 82B9ABD4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82B9ABD8: 48000010  b 0x82b9abe8
	pc = 0x82B9ABE8; continue 'dispatch;
	// 82B9ABDC: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9ABE0: 41820008  beq 0x82b9abe8
	if ctx.cr[0].eq {
	pc = 0x82B9ABE8; continue 'dispatch;
	}
	// 82B9ABE4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82B9ABE8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9ABEC: 516AE046  rlwimi r10, r11, 0x1c, 1, 3
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(28) as u64) & 0x0000000070000000) | (ctx.r[10].u64 & 0xFFFFFFFF8FFFFFFF);
	// 82B9ABF0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9ABF4: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82B9ABF8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9ABFC: 514BB908  rlwimi r11, r10, 0x17, 4, 4
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(23) as u64) & 0x0000000008000000) | (ctx.r[11].u64 & 0xFFFFFFFFF7FFFFFF);
	// 82B9AC00: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9AC04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC08: 816B4DB8  lwz r11, 0x4db8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9AC0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9AC10: 419A0570  beq cr6, 0x82b9b180
	if ctx.cr[6].eq {
	pc = 0x82B9B180; continue 'dispatch;
	}
	// 82B9AC14: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC18: 48000030  b 0x82b9ac48
	pc = 0x82B9AC48; continue 'dispatch;
	// 82B9AC1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC20: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC24: 88BF0000  lbz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC28: 5546873E  rlwinm r6, r10, 0x10, 0x1c, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9AC2C: 5544673E  rlwinm r4, r10, 0xc, 0x1c, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 82B9AC30: 806B4DB8  lwz r3, 0x4db8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9AC34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC38: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B9AC3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9AC40: 4E800421  bctrl
	ctx.lr = 0x82B9AC44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9AC44: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82B9AC48: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AC4C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC50: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9AC54: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9AC58: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9AC5C: 4198FFC0  blt cr6, 0x82b9ac1c
	if ctx.cr[6].lt {
	pc = 0x82B9AC1C; continue 'dispatch;
	}
	// 82B9AC60: 48000520  b 0x82b9b180
	pc = 0x82B9B180; continue 'dispatch;
	// 82B9AC64: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AC68: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AC6C: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9AC70: 396A0024  addi r11, r10, 0x24
	ctx.r[11].s64 = ctx.r[10].s64 + 36;
	// 82B9AC74: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9AC78: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9AC7C: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82B9AC80: 40990008  ble cr6, 0x82b9ac88
	if !ctx.cr[6].gt {
	pc = 0x82B9AC88; continue 'dispatch;
	}
	// 82B9AC84: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9AC88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AC8C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82B9AC90: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9AC94: 41990008  bgt cr6, 0x82b9ac9c
	if ctx.cr[6].gt {
	pc = 0x82B9AC9C; continue 'dispatch;
	}
	// 82B9AC98: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 82B9AC9C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82B9ACA0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82B9ACA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9ACA8: 48001269  bl 0x82b9bf10
	ctx.lr = 0x82B9ACAC;
	sub_82B9BF10(ctx, base);
	// 82B9ACAC: 817E011C  lwz r11, 0x11c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) } as u64;
	// 82B9ACB0: 917B0018  stw r11, 0x18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82B9ACB4: 817E0128  lwz r11, 0x128(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(296 as u32) ) } as u64;
	// 82B9ACB8: 917B001C  stw r11, 0x1c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82B9ACBC: 817E013C  lwz r11, 0x13c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(316 as u32) ) } as u64;
	// 82B9ACC0: 917B0020  stw r11, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82B9ACC4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ACC8: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9ACCC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ACD0: 817E011C  lwz r11, 0x11c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) } as u64;
	// 82B9ACD4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9ACD8: 7D6A2A14  add r11, r10, r5
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82B9ACDC: 7C695214  add r3, r9, r10
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9ACE0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9ACE4: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9ACE8: 40990008  ble cr6, 0x82b9acf0
	if !ctx.cr[6].gt {
	pc = 0x82B9ACF0; continue 'dispatch;
	}
	// 82B9ACEC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9ACF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ACF4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9ACF8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9ACFC: 40990008  ble cr6, 0x82b9ad04
	if !ctx.cr[6].gt {
	pc = 0x82B9AD04; continue 'dispatch;
	}
	// 82B9AD00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9AD04: 809E0118  lwz r4, 0x118(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(280 as u32) ) } as u64;
	// 82B9AD08: 4810E779  bl 0x82ca9480
	ctx.lr = 0x82B9AD0C;
	sub_82CA9480(ctx, base);
	// 82B9AD0C: 817E0128  lwz r11, 0x128(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(296 as u32) ) } as u64;
	// 82B9AD10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AD14: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9AD18: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AD1C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9AD20: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9AD24: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9AD28: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9AD2C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9AD30: 40990008  ble cr6, 0x82b9ad38
	if !ctx.cr[6].gt {
	pc = 0x82B9AD38; continue 'dispatch;
	}
	// 82B9AD34: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9AD38: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AD3C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9AD40: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9AD44: 40990008  ble cr6, 0x82b9ad4c
	if !ctx.cr[6].gt {
	pc = 0x82B9AD4C; continue 'dispatch;
	}
	// 82B9AD48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82B9AD4C: 80BE0104  lwz r5, 0x104(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(260 as u32) ) } as u64;
	// 82B9AD50: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82B9AD54: 809E0100  lwz r4, 0x100(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) } as u64;
	// 82B9AD58: 397E0124  addi r11, r30, 0x124
	ctx.r[11].s64 = ctx.r[30].s64 + 292;
	// 82B9AD5C: 815E0128  lwz r10, 0x128(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(296 as u32) ) } as u64;
	// 82B9AD60: 80FE0124  lwz r7, 0x124(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(292 as u32) ) } as u64;
	// 82B9AD64: 7CA42850  subf r5, r4, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 82B9AD68: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9AD6C: 7CEA07B4  extsw r10, r7
	ctx.r[10].s64 = ctx.r[7].s32 as i64;
	// 82B9AD70: 7CC533D6  divw r6, r5, r6
	ctx.r[6].s32 = ctx.r[5].s32 / ctx.r[6].s32;
	// 82B9AD74: 48000028  b 0x82b9ad9c
	pc = 0x82B9AD9C; continue 'dispatch;
	// 82B9AD78: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AD7C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9AD80: 7CE83214  add r7, r8, r6
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82B9AD84: 51070026  rlwimi r7, r8, 0, 0, 0x13
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[7].u64 & 0xFFFFFFFF00000FFF);
	// 82B9AD88: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82B9AD8C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9AD90: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AD94: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9AD98: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AD9C: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82B9ADA0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9ADA4: 4198FFD4  blt cr6, 0x82b9ad78
	if ctx.cr[6].lt {
	pc = 0x82B9AD78; continue 'dispatch;
	}
	// 82B9ADA8: 815E0134  lwz r10, 0x134(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9ADAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82B9ADB0: 811E0130  lwz r8, 0x130(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) } as u64;
	// 82B9ADB4: 397E0130  addi r11, r30, 0x130
	ctx.r[11].s64 = ctx.r[30].s64 + 304;
	// 82B9ADB8: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9ADBC: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82B9ADC0: 7D0A07B4  extsw r10, r8
	ctx.r[10].s64 = ctx.r[8].s32 as i64;
	// 82B9ADC4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B9ADC8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9ADCC: 40980050  bge cr6, 0x82b9ae1c
	if !ctx.cr[6].lt {
	pc = 0x82B9AE1C; continue 'dispatch;
	}
	// 82B9ADD0: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ADD4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ADD8: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9ADDC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9ADE0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ADE4: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9ADE8: 38E10064  addi r7, r1, 0x64
	ctx.r[7].s64 = ctx.r[1].s64 + 100;
	// 82B9ADEC: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 82B9ADF0: 55250426  rlwinm r5, r9, 0, 0x10, 0x13
	ctx.r[5].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9ADF4: 51266026  rlwimi r6, r9, 0xc, 0, 0x13
	ctx.r[6].u64 = (((ctx.r[9].u32).rotate_left(12) as u64) & 0x00000000FFFFF000) | (ctx.r[6].u64 & 0xFFFFFFFF00000FFF);
	// 82B9ADF8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9ADFC: 54C96016  rlwinm r9, r6, 0xc, 0, 0xb
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x000FFFFFu64;
	// 82B9AE00: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9AE04: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82B9AE08: 7D28392E  stwx r9, r8, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), ctx.r[9].u32) };
	// 82B9AE0C: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9AE10: 38E90001  addi r7, r9, 1
	ctx.r[7].s64 = ctx.r[9].s64 + 1;
	// 82B9AE14: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82B9AE18: 4198FFC8  blt cr6, 0x82b9ade0
	if ctx.cr[6].lt {
	pc = 0x82B9ADE0; continue 'dispatch;
	}
	// 82B9AE1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AE20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82B9AE24: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9AE28: 5564EFFE  rlwinm r4, r11, 0x1d, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9AE2C: 4BFFE995  bl 0x82b997c0
	ctx.lr = 0x82B9AE30;
	sub_82B997C0(ctx, base);
	// 82B9AE30: 817E0134  lwz r11, 0x134(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9AE34: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AE38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9AE3C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AE40: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9AE44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9AE48: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9AE4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9AE50: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9AE54: 40990008  ble cr6, 0x82b9ae5c
	if !ctx.cr[6].gt {
	pc = 0x82B9AE5C; continue 'dispatch;
	}
	// 82B9AE58: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9AE5C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9AE60: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82B9AE64: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9AE68: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9AE6C: 41990008  bgt cr6, 0x82b9ae74
	if ctx.cr[6].gt {
	pc = 0x82B9AE74; continue 'dispatch;
	}
	// 82B9AE70: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82B9AE74: 813E0134  lwz r9, 0x134(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9AE78: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82B9AE7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9AE80: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82B9AE84: 4099003C  ble cr6, 0x82b9aec0
	if !ctx.cr[6].gt {
	pc = 0x82B9AEC0; continue 'dispatch;
	}
	// 82B9AE88: 39210064  addi r9, r1, 0x64
	ctx.r[9].s64 = ctx.r[1].s64 + 100;
	// 82B9AE8C: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AE90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9AE94: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9AE98: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82B9AE9C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82B9AEA0: 5107843E  rlwimi r7, r8, 0x10, 0x10, 0x1f
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF0000);
	// 82B9AEA4: 50E6C53E  rlwimi r6, r7, 0x18, 0x14, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(24) as u64) & 0x0000000000000FFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFFF000);
	// 82B9AEA8: 54C8043E  clrlwi r8, r6, 0x10
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 82B9AEAC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9AEB0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9AEB4: 811E0134  lwz r8, 0x134(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9AEB8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9AEBC: 4198FFD0  blt cr6, 0x82b9ae8c
	if ctx.cr[6].lt {
	pc = 0x82B9AE8C; continue 'dispatch;
	}
	// 82B9AEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9AEC4: 394102E0  addi r10, r1, 0x2e0
	ctx.r[10].s64 = ctx.r[1].s64 + 736;
	// 82B9AEC8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9AECC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9AED0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9AED4: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 82B9AED8: 4198FFF0  blt cr6, 0x82b9aec8
	if ctx.cr[6].lt {
	pc = 0x82B9AEC8; continue 'dispatch;
	}
	// 82B9AEDC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9AEE0: 39410064  addi r10, r1, 0x64
	ctx.r[10].s64 = ctx.r[1].s64 + 100;
	// 82B9AEE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9AEE8: 419A0024  beq cr6, 0x82b9af0c
	if ctx.cr[6].eq {
	pc = 0x82B9AF0C; continue 'dispatch;
	}
	// 82B9AEEC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AEF0: 390102E0  addi r8, r1, 0x2e0
	ctx.r[8].s64 = ctx.r[1].s64 + 736;
	// 82B9AEF4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9AEF8: 552776BA  rlwinm r7, r9, 0xe, 0x1a, 0x1d
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0003FFFFu64;
	// 82B9AEFC: 5529873E  rlwinm r9, r9, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82B9AF00: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9AF04: 7D27412E  stwx r9, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82B9AF08: 4082FFE4  bne 0x82b9aeec
	if !ctx.cr[0].eq {
	pc = 0x82B9AEEC; continue 'dispatch;
	}
	// 82B9AF0C: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9AF10: 38E102E0  addi r7, r1, 0x2e0
	ctx.r[7].s64 = ctx.r[1].s64 + 736;
	// 82B9AF14: 38CBB7B0  addi r6, r11, -0x4850
	ctx.r[6].s64 = ctx.r[11].s64 + -18512;
	// 82B9AF18: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9AF1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9AF20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9AF24: 480013CD  bl 0x82b9c2f0
	ctx.lr = 0x82B9AF28;
	sub_82B9C2F0(ctx, base);
	// 82B9AF28: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 82B9AF2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9AF30: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 82B9AF34: 83BE0134  lwz r29, 0x134(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9AF38: 4810EA79  bl 0x82ca99b0
	ctx.lr = 0x82B9AF3C;
	sub_82CA99B0(ctx, base);
	// 82B9AF3C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82B9AF40: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82B9AF44: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 82B9AF48: 4810EA69  bl 0x82ca99b0
	ctx.lr = 0x82B9AF4C;
	sub_82CA99B0(ctx, base);
	// 82B9AF4C: 93A10174  stw r29, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[29].u32 ) };
	// 82B9AF50: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82B9AF54: 93810170  stw r28, 0x170(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[28].u32 ) };
	// 82B9AF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9AF5C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B9AF60: 419A0050  beq cr6, 0x82b9afb0
	if ctx.cr[6].eq {
	pc = 0x82B9AFB0; continue 'dispatch;
	}
	// 82B9AF64: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82B9AF68: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AF6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9AF70: 5527C73E  rlwinm r7, r9, 0x18, 0x1c, 0x1f
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82B9AF74: 5526A73E  rlwinm r6, r9, 0x14, 0x1c, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82B9AF78: 7F495830  slw r9, r26, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[26].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9AF7C: 7D293039  and. r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9AF80: 41820014  beq 0x82b9af94
	if ctx.cr[0].eq {
	pc = 0x82B9AF94; continue 'dispatch;
	}
	// 82B9AF84: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9AF88: 38A10178  addi r5, r1, 0x178
	ctx.r[5].s64 = ctx.r[1].s64 + 376;
	// 82B9AF8C: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82B9AF90: 7D4929AE  stbx r10, r9, r5
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32), ctx.r[10].u8) };
	// 82B9AF94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9AF98: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82B9AF9C: 4198FFDC  blt cr6, 0x82b9af78
	if ctx.cr[6].lt {
	pc = 0x82B9AF78; continue 'dispatch;
	}
	// 82B9AFA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9AFA4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82B9AFA8: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82B9AFAC: 4198FFBC  blt cr6, 0x82b9af68
	if ctx.cr[6].lt {
	pc = 0x82B9AF68; continue 'dispatch;
	}
	// 82B9AFB0: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9AFB4: 38E10170  addi r7, r1, 0x170
	ctx.r[7].s64 = ctx.r[1].s64 + 368;
	// 82B9AFB8: 38CBBB40  addi r6, r11, -0x44c0
	ctx.r[6].s64 = ctx.r[11].s64 + -17600;
	// 82B9AFBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9AFC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9AFC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9AFC8: 48001329  bl 0x82b9c2f0
	ctx.lr = 0x82B9AFCC;
	sub_82B9C2F0(ctx, base);
	// 82B9AFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82B9AFD0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B9AFD4: 419A0034  beq cr6, 0x82b9b008
	if ctx.cr[6].eq {
	pc = 0x82B9B008; continue 'dispatch;
	}
	// 82B9AFD8: 392101B8  addi r9, r1, 0x1b8
	ctx.r[9].s64 = ctx.r[1].s64 + 440;
	// 82B9AFDC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82B9AFE0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82B9AFE4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AFE8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9AFEC: 5107811E  rlwimi r7, r8, 0x10, 4, 0xf
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x000000000FFF0000) | (ctx.r[7].u64 & 0xFFFFFFFFF000FFFF);
	// 82B9AFF0: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82B9AFF4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9AFF8: A0E90000  lhz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9AFFC: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82B9B000: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82B9B004: 4082FFE0  bne 0x82b9afe4
	if !ctx.cr[0].eq {
	pc = 0x82B9AFE4; continue 'dispatch;
	}
	// 82B9B008: 817E013C  lwz r11, 0x13c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(316 as u32) ) } as u64;
	// 82B9B00C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B010: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9B014: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B018: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9B01C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9B020: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9B024: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9B028: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9B02C: 40990008  ble cr6, 0x82b9b034
	if !ctx.cr[6].gt {
	pc = 0x82B9B034; continue 'dispatch;
	}
	// 82B9B030: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9B034: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B038: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82B9B03C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B040: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9B044: 41990008  bgt cr6, 0x82b9b04c
	if ctx.cr[6].gt {
	pc = 0x82B9B04C; continue 'dispatch;
	}
	// 82B9B048: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82B9B04C: 38A00070  li r5, 0x70
	ctx.r[5].s64 = 112;
	// 82B9B050: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B054: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82B9B058: 4810E959  bl 0x82ca99b0
	ctx.lr = 0x82B9B05C;
	sub_82CA99B0(ctx, base);
	// 82B9B05C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82B9B060: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82B9B064: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82B9B068: 4810E949  bl 0x82ca99b0
	ctx.lr = 0x82B9B06C;
	sub_82CA99B0(ctx, base);
	// 82B9B06C: 83FE0134  lwz r31, 0x134(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9B070: 817E013C  lwz r11, 0x13c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(316 as u32) ) } as u64;
	// 82B9B074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9B078: 93810100  stw r28, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[28].u32 ) };
	// 82B9B07C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82B9B080: 93A10108  stw r29, 0x108(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[29].u32 ) };
	// 82B9B084: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82B9B088: 9161010C  stw r11, 0x10c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), ctx.r[11].u32 ) };
	// 82B9B08C: 419A0050  beq cr6, 0x82b9b0dc
	if ctx.cr[6].eq {
	pc = 0x82B9B0DC; continue 'dispatch;
	}
	// 82B9B090: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82B9B094: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9B09C: 5527C73E  rlwinm r7, r9, 0x18, 0x1c, 0x1f
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82B9B0A0: 5526A73E  rlwinm r6, r9, 0x14, 0x1c, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82B9B0A4: 7F495830  slw r9, r26, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[26].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9B0A8: 7D293039  and. r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9B0AC: 41820014  beq 0x82b9b0c0
	if ctx.cr[0].eq {
	pc = 0x82B9B0C0; continue 'dispatch;
	}
	// 82B9B0B0: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9B0B4: 38A10110  addi r5, r1, 0x110
	ctx.r[5].s64 = ctx.r[1].s64 + 272;
	// 82B9B0B8: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82B9B0BC: 7D4929AE  stbx r10, r9, r5
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32), ctx.r[10].u8) };
	// 82B9B0C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9B0C4: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82B9B0C8: 4198FFDC  blt cr6, 0x82b9b0a4
	if ctx.cr[6].lt {
	pc = 0x82B9B0A4; continue 'dispatch;
	}
	// 82B9B0CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9B0D0: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82B9B0D4: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82B9B0D8: 4198FFBC  blt cr6, 0x82b9b094
	if ctx.cr[6].lt {
	pc = 0x82B9B094; continue 'dispatch;
	}
	// 82B9B0DC: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9B0E0: 38E10100  addi r7, r1, 0x100
	ctx.r[7].s64 = ctx.r[1].s64 + 256;
	// 82B9B0E4: 38CBBA70  addi r6, r11, -0x4590
	ctx.r[6].s64 = ctx.r[11].s64 + -17808;
	// 82B9B0E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9B0EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B0F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9B0F4: 480011FD  bl 0x82b9c2f0
	ctx.lr = 0x82B9B0F8;
	sub_82B9C2F0(ctx, base);
	// 82B9B0F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82B9B0FC: 419A0048  beq cr6, 0x82b9b144
	if ctx.cr[6].eq {
	pc = 0x82B9B144; continue 'dispatch;
	}
	// 82B9B100: 39010150  addi r8, r1, 0x150
	ctx.r[8].s64 = ctx.r[1].s64 + 336;
	// 82B9B104: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82B9B108: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82B9B10C: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B110: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B114: A0E80000  lhz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B118: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9B11C: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B120: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82B9B124: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82B9B128: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9B12C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82B9B130: 38EBFFFC  addi r7, r11, -4
	ctx.r[7].s64 = ctx.r[11].s64 + -4;
	// 82B9B134: 80EBFFFC  lwz r7, -4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9B138: 60E71000  ori r7, r7, 0x1000
	ctx.r[7].u64 = ctx.r[7].u64 | 4096;
	// 82B9B13C: 90EBFFFC  stw r7, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 82B9B140: 4082FFCC  bne 0x82b9b10c
	if !ctx.cr[0].eq {
	pc = 0x82B9B10C; continue 'dispatch;
	}
	// 82B9B144: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9B148: 556B06BF  clrlwi. r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B14C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9B150: 40820024  bne 0x82b9b174
	if !ctx.cr[0].eq {
	pc = 0x82B9B174; continue 'dispatch;
	}
	// 82B9B154: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9B158: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82B9B15C: 38CBB7E0  addi r6, r11, -0x4820
	ctx.r[6].s64 = ctx.r[11].s64 + -18464;
	// 82B9B160: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9B164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9B16C: 48001185  bl 0x82b9c2f0
	ctx.lr = 0x82B9B170;
	sub_82B9C2F0(ctx, base);
	// 82B9B170: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9B174: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B178: 516A06BE  rlwimi r10, r11, 0, 0x1a, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x000000000000003F) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFC0);
	// 82B9B17C: 915B0008  stw r10, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82B9B180: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9B184: 80FE0100  lwz r7, 0x100(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) } as u64;
	// 82B9B188: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9B18C: 38CBB9A0  addi r6, r11, -0x4660
	ctx.r[6].s64 = ctx.r[11].s64 + -18016;
	// 82B9B190: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82B9B194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9B198: 48001159  bl 0x82b9c2f0
	ctx.lr = 0x82B9B19C;
	sub_82B9C2F0(ctx, base);
	// 82B9B19C: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9B1A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82B9B1A4: 41980014  blt cr6, 0x82b9b1b8
	if ctx.cr[6].lt {
	pc = 0x82B9B1B8; continue 'dispatch;
	}
	// 82B9B1A8: 80790010  lwz r3, 0x10(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B9B1AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82B9B1B0: 41980008  blt cr6, 0x82b9b1b8
	if ctx.cr[6].lt {
	pc = 0x82B9B1B8; continue 'dispatch;
	}
	// 82B9B1B4: 80790024  lwz r3, 0x24(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 82B9B1B8: 38210420  addi r1, r1, 0x420
	ctx.r[1].s64 = ctx.r[1].s64 + 1056;
	// 82B9B1BC: 4810E290  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9B1C0 size=280
    let mut pc: u32 = 0x82B9B1C0;
    'dispatch: loop {
        match pc {
            0x82B9B1C0 => {
    //   block [0x82B9B1C0..0x82B9B2D8)
	// 82B9B1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9B1C4: 4810E239  bl 0x82ca93fc
	ctx.lr = 0x82B9B1C8;
	sub_82CA93D0(ctx, base);
	// 82B9B1C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9B1CC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82B9B1D0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82B9B1D4: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82B9B1D8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82B9B1DC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82B9B1E0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82B9B1E4: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9B1E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82B9B1EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B9B1F0: 48001669  bl 0x82b9c858
	ctx.lr = 0x82B9B1F4;
	sub_82B9C858(ctx, base);
	// 82B9B1F4: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82B9B1F8: 817E0128  lwz r11, 0x128(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(296 as u32) ) } as u64;
	// 82B9B1FC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82B9B200: 53A93672  rlwimi r9, r29, 6, 0x19, 0x19
	ctx.r[9].u64 = (((ctx.r[29].u32).rotate_left(6) as u64) & 0x0000000000000040) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFBF);
	// 82B9B204: 815E012C  lwz r10, 0x12c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(300 as u32) ) } as u64;
	// 82B9B208: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9B20C: 7129004F  andi. r9, r9, 0x4f
	ctx.r[9].u64 = ctx.r[9].u64 & 79;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9B210: 839E0064  lwz r28, 0x64(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9B214: 5128402E  rlwimi r8, r9, 8, 0, 0x17
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[8].u64 & 0xFFFFFFFF000000FF);
	// 82B9B218: 3BFE0124  addi r31, r30, 0x124
	ctx.r[31].s64 = ctx.r[30].s64 + 292;
	// 82B9B21C: 3B3E0050  addi r25, r30, 0x50
	ctx.r[25].s64 = ctx.r[30].s64 + 80;
	// 82B9B220: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9B224: 511C4026  rlwimi r28, r8, 8, 0, 0x13
	ctx.r[28].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x00000000FFFFF000) | (ctx.r[28].u64 & 0xFFFFFFFF00000FFF);
	// 82B9B228: 40990068  ble cr6, 0x82b9b290
	if !ctx.cr[6].gt {
	pc = 0x82B9B290; continue 'dispatch;
	}
	// 82B9B22C: 555D083C  slwi r29, r10, 1
	ctx.r[29].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82B9B230: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9B234: 40980008  bge cr6, 0x82b9b23c
	if !ctx.cr[6].lt {
	pc = 0x82B9B23C; continue 'dispatch;
	}
	// 82B9B238: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82B9B23C: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9B240: 57A3103A  slwi r3, r29, 2
	ctx.r[3].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B9B244: 4B69178D  bl 0x8222c9d0
	ctx.lr = 0x82B9B248;
	sub_8222C9D0(ctx, base);
	// 82B9B248: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82B9B24C: 40820014  bne 0x82b9b260
	if !ctx.cr[0].eq {
	pc = 0x82B9B260; continue 'dispatch;
	}
	// 82B9B250: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 82B9B254: 616B000E  ori r11, r11, 0xe
	ctx.r[11].u64 = ctx.r[11].u64 | 14;
	// 82B9B258: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B25C: 4800005C  b 0x82b9b2b8
	pc = 0x82B9B2B8; continue 'dispatch;
	// 82B9B260: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B264: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82B9B268: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9B26C: 419A0020  beq cr6, 0x82b9b28c
	if ctx.cr[6].eq {
	pc = 0x82B9B28C; continue 'dispatch;
	}
	// 82B9B270: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B274: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82B9B278: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9B27C: 4810E205  bl 0x82ca9480
	ctx.lr = 0x82B9B280;
	sub_82CA9480(ctx, base);
	// 82B9B280: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9B284: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B288: 4B667031  bl 0x822022b8
	ctx.lr = 0x82B9B28C;
	sub_822022B8(ctx, base);
	// 82B9B28C: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82B9B290: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B294: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B298: 41980020  blt cr6, 0x82b9b2b8
	if ctx.cr[6].lt {
	pc = 0x82B9B2B8; continue 'dispatch;
	}
	// 82B9B29C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B2A0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B2A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9B2A8: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82B9B2AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B2B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9B2B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9B2B8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82B9B2BC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82B9B2C0: 387E0060  addi r3, r30, 0x60
	ctx.r[3].s64 = ctx.r[30].s64 + 96;
	// 82B9B2C4: 4800388D  bl 0x82b9eb50
	ctx.lr = 0x82B9B2C8;
	sub_82B9EB50(ctx, base);
	// 82B9B2C8: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9B2CC: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82B9B2D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B9B2D4: 4810E178  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B2D8 size=100
    let mut pc: u32 = 0x82B9B2D8;
    'dispatch: loop {
        match pc {
            0x82B9B2D8 => {
    //   block [0x82B9B2D8..0x82B9B33C)
	// 82B9B2D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B2DC: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9B2E0: 556A8FFF  rlwinm. r10, r11, 0x11, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B2E4: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82B9B2E8: 5529873E  rlwinm r9, r9, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82B9B2EC: 41820010  beq 0x82b9b2fc
	if ctx.cr[0].eq {
	pc = 0x82B9B2FC; continue 'dispatch;
	}
	// 82B9B2F0: 55680463  rlwinm. r8, r11, 0, 0x11, 0x11
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B9B2F4: 41820008  beq 0x82b9b2fc
	if ctx.cr[0].eq {
	pc = 0x82B9B2FC; continue 'dispatch;
	}
	// 82B9B2F8: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 82B9B2FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B9B300: 419A0014  beq cr6, 0x82b9b314
	if ctx.cr[6].eq {
	pc = 0x82B9B314; continue 'dispatch;
	}
	// 82B9B304: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B9B308: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82B9B30C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9B310: 41980008  blt cr6, 0x82b9b318
	if ctx.cr[6].lt {
	pc = 0x82B9B318; continue 'dispatch;
	}
	// 82B9B314: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9B318: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B31C: 41820010  beq 0x82b9b32c
	if ctx.cr[0].eq {
	pc = 0x82B9B32C; continue 'dispatch;
	}
	// 82B9B320: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82B9B324: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9B328: 409A0008  bne cr6, 0x82b9b330
	if !ctx.cr[6].eq {
	pc = 0x82B9B330; continue 'dispatch;
	}
	// 82B9B32C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9B330: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B334: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82B9B338: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B33C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B33C size=8
    let mut pc: u32 = 0x82B9B33C;
    'dispatch: loop {
        match pc {
            0x82B9B33C => {
    //   block [0x82B9B33C..0x82B9B344)
	// 82B9B33C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9B340: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B344 size=12
    let mut pc: u32 = 0x82B9B344;
    'dispatch: loop {
        match pc {
            0x82B9B344 => {
    //   block [0x82B9B344..0x82B9B350)
	// 82B9B344: 552B063E  clrlwi r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82B9B348: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9B350 size=176
    let mut pc: u32 = 0x82B9B350;
    'dispatch: loop {
        match pc {
            0x82B9B350 => {
    //   block [0x82B9B350..0x82B9B400)
	// 82B9B350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9B354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9B358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B9B35C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9B360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9B364: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B9B368: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9B36C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82B9B370: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9B374: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B378: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9B37C: 480014DD  bl 0x82b9c858
	ctx.lr = 0x82B9B380;
	sub_82B9C858(ctx, base);
	// 82B9B380: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82B9B384: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B9B388: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82B9B38C: 480037C5  bl 0x82b9eb50
	ctx.lr = 0x82B9B390;
	sub_82B9EB50(ctx, base);
	// 82B9B390: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B394: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9B398: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B39C: 40820044  bne 0x82b9b3e0
	if !ctx.cr[0].eq {
	pc = 0x82B9B3E0; continue 'dispatch;
	}
	// 82B9B3A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9B3A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9B3A8: 4BFFFF31  bl 0x82b9b2d8
	ctx.lr = 0x82B9B3AC;
	sub_82B9B2D8(ctx, base);
	// 82B9B3AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9B3B0: 41820030  beq 0x82b9b3e0
	if ctx.cr[0].eq {
	pc = 0x82B9B3E0; continue 'dispatch;
	}
	// 82B9B3B4: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82B9B3B8: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9B3BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9B3C0: 917F013C  stw r11, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 82B9B3C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B3C8: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B9B3CC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82B9B3D0: 394B0140  addi r10, r11, 0x140
	ctx.r[10].s64 = ctx.r[11].s64 + 320;
	// 82B9B3D4: 894B0140  lbz r10, 0x140(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(320 as u32) ) } as u64;
	// 82B9B3D8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82B9B3DC: 994B0140  stb r10, 0x140(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u8 ) };
	// 82B9B3E0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9B3E4: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82B9B3E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9B3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9B3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9B3F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9B3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9B3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B400 size=116
    let mut pc: u32 = 0x82B9B400;
    'dispatch: loop {
        match pc {
            0x82B9B400 => {
    //   block [0x82B9B400..0x82B9B474)
	// 82B9B400: 81430070  lwz r10, 0x70(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9B404: 3963006C  addi r11, r3, 0x6c
	ctx.r[11].s64 = ctx.r[3].s64 + 108;
	// 82B9B408: 8123006C  lwz r9, 0x6c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9B40C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9B410: 7D2307B4  extsw r3, r9
	ctx.r[3].s64 = ctx.r[9].s32 as i64;
	// 82B9B414: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82B9B418: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9B41C: 40980064  bge cr6, 0x82b9b480
	if !ctx.cr[6].lt {
		sub_82B9B474(ctx, base);
		return;
	}
	// 82B9B420: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B424: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B428: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9B42C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9B430: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B434: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B9B438: 5568A73E  rlwinm r8, r11, 0x14, 0x1c, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B43C: 7D4A4030  slw r10, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9B440: 714A607E  andi. r10, r10, 0x607e
	ctx.r[10].u64 = ctx.r[10].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B444: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9B448: 4182002C  beq 0x82b9b474
	if ctx.cr[0].eq {
		sub_82B9B474(ctx, base);
		return;
	}
	// 82B9B44C: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B450: 40820024  bne 0x82b9b474
	if !ctx.cr[0].eq {
		sub_82B9B474(ctx, base);
		return;
	}
	// 82B9B454: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B458: 556A053E  clrlwi r10, r11, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B45C: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82B9B460: 41990014  bgt cr6, 0x82b9b474
	if ctx.cr[6].gt {
		sub_82B9B474(ctx, base);
		return;
	}
	// 82B9B464: 556BA77E  rlwinm r11, r11, 0x14, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B468: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9B46C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9B470: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B474 size=20
    let mut pc: u32 = 0x82B9B474;
    'dispatch: loop {
        match pc {
            0x82B9B474 => {
    //   block [0x82B9B474..0x82B9B488)
	// 82B9B474: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82B9B478: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9B47C: 4198FFB4  blt cr6, 0x82b9b430
	if ctx.cr[6].lt {
		sub_82B9B400(ctx, base);
		return;
	}
	// 82B9B480: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9B484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9B488 size=240
    let mut pc: u32 = 0x82B9B488;
    'dispatch: loop {
        match pc {
            0x82B9B488 => {
    //   block [0x82B9B488..0x82B9B578)
	// 82B9B488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9B48C: 4810DF81  bl 0x82ca940c
	ctx.lr = 0x82B9B490;
	sub_82CA93D0(ctx, base);
	// 82B9B490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9B494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9B498: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B9B49C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82B9B4A0: 4BFFFF61  bl 0x82b9b400
	ctx.lr = 0x82B9B4A4;
	sub_82B9B400(ctx, base);
	// 82B9B4A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9B4A8: 418200C8  beq 0x82b9b570
	if ctx.cr[0].eq {
	pc = 0x82B9B570; continue 'dispatch;
	}
	// 82B9B4AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B4B0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82B9B4B4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9B4B8: 5569053E  clrlwi r9, r11, 0x14
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B4BC: 7D29F050  subf r9, r9, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	// 82B9B4C0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9B4C4: 7D4A4830  slw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9B4C8: 419A0018  beq cr6, 0x82b9b4e0
	if ctx.cr[6].eq {
	pc = 0x82B9B4E0; continue 'dispatch;
	}
	// 82B9B4CC: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9B4D0: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9B4D4: 516A0406  rlwimi r10, r11, 0, 0x10, 3
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0xFFFFFFFFF000FFFF) | (ctx.r[10].u64 & 0x000000000FFF0000);
	// 82B9B4D8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B9B4DC: 4800001C  b 0x82b9b4f8
	pc = 0x82B9B4F8; continue 'dispatch;
	// 82B9B4E0: 3D20F000  lis r9, -0x1000
	ctx.r[9].s64 = -268435456;
	// 82B9B4E4: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82B9B4E8: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82B9B4EC: 5149811E  rlwimi r9, r10, 0x10, 4, 0xf
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000FFF0000) | (ctx.r[9].u64 & 0xFFFFFFFFF000FFFF);
	// 82B9B4F0: 7D2B5838  and r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82B9B4F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B4F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B4FC: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9B500: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9B504: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B508: 41820068  beq 0x82b9b570
	if ctx.cr[0].eq {
	pc = 0x82B9B570; continue 'dispatch;
	}
	// 82B9B50C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9B510: 419A0060  beq cr6, 0x82b9b570
	if ctx.cr[6].eq {
	pc = 0x82B9B570; continue 'dispatch;
	}
	// 82B9B514: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9B518: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82B9B51C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9B520: 409A0050  bne cr6, 0x82b9b570
	if !ctx.cr[6].eq {
	pc = 0x82B9B570; continue 'dispatch;
	}
	// 82B9B524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9B528: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9B52C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B530: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9B534: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82B9B538: 997F007C  stb r11, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u8 ) };
	// 82B9B53C: 4810E475  bl 0x82ca99b0
	ctx.lr = 0x82B9B540;
	sub_82CA99B0(ctx, base);
	// 82B9B540: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9B544: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B548: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82B9B54C: 4810E465  bl 0x82ca99b0
	ctx.lr = 0x82B9B550;
	sub_82CA99B0(ctx, base);
	// 82B9B550: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9B554: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B558: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82B9B55C: 4810E455  bl 0x82ca99b0
	ctx.lr = 0x82B9B560;
	sub_82CA99B0(ctx, base);
	// 82B9B560: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9B564: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9B568: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82B9B56C: 4810E445  bl 0x82ca99b0
	ctx.lr = 0x82B9B570;
	sub_82CA99B0(ctx, base);
	// 82B9B570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9B574: 4810DEE8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B578 size=20
    let mut pc: u32 = 0x82B9B578;
    'dispatch: loop {
        match pc {
            0x82B9B578 => {
    //   block [0x82B9B578..0x82B9B58C)
	// 82B9B578: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9B57C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B580: 4098000C  bge cr6, 0x82b9b58c
	if !ctx.cr[6].lt {
		sub_82B9B58C(ctx, base);
		return;
	}
	// 82B9B584: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9B588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B58C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B58C size=140
    let mut pc: u32 = 0x82B9B58C;
    'dispatch: loop {
        match pc {
            0x82B9B58C => {
    //   block [0x82B9B58C..0x82B9B618)
	// 82B9B58C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82B9B590: 419A00B8  beq cr6, 0x82b9b648
	if ctx.cr[6].eq {
		sub_82B9B618(ctx, base);
		return;
	}
	// 82B9B594: 81430070  lwz r10, 0x70(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9B598: 3963006C  addi r11, r3, 0x6c
	ctx.r[11].s64 = ctx.r[3].s64 + 108;
	// 82B9B59C: 8103006C  lwz r8, 0x6c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9B5A0: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9B5A4: 7D0A07B4  extsw r10, r8
	ctx.r[10].s64 = ctx.r[8].s32 as i64;
	// 82B9B5A8: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82B9B5AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9B5B0: 40980098  bge cr6, 0x82b9b648
	if !ctx.cr[6].lt {
		sub_82B9B618(ctx, base);
		return;
	}
	// 82B9B5B4: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B5B8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B5BC: 550B1838  slwi r11, r8, 3
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9B5C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82B9B5C4: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9B5C8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B5CC: 5569A73E  rlwinm r9, r11, 0x14, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B5D0: 7D094830  slw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9B5D4: 7129607E  andi. r9, r9, 0x607e
	ctx.r[9].u64 = ctx.r[9].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9B5D8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9B5DC: 4182002C  beq 0x82b9b608
	if ctx.cr[0].eq {
	pc = 0x82B9B608; continue 'dispatch;
	}
	// 82B9B5E0: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B5E4: 40820024  bne 0x82b9b608
	if !ctx.cr[0].eq {
	pc = 0x82B9B608; continue 'dispatch;
	}
	// 82B9B5E8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B5EC: 5569053E  clrlwi r9, r11, 0x14
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B5F0: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82B9B5F4: 41990014  bgt cr6, 0x82b9b608
	if ctx.cr[6].gt {
	pc = 0x82B9B608; continue 'dispatch;
	}
	// 82B9B5F8: 556BA77E  rlwinm r11, r11, 0x14, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B5FC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9B600: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9B604: 41980014  blt cr6, 0x82b9b618
	if ctx.cr[6].lt {
		sub_82B9B618(ctx, base);
		return;
	}
	// 82B9B608: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82B9B60C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82B9B610: 4198FFB8  blt cr6, 0x82b9b5c8
	if ctx.cr[6].lt {
	pc = 0x82B9B5C8; continue 'dispatch;
	}
	// 82B9B614: 48000034  b 0x82b9b648
	sub_82B9B618(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B618 size=64
    let mut pc: u32 = 0x82B9B618;
    'dispatch: loop {
        match pc {
            0x82B9B618 => {
    //   block [0x82B9B618..0x82B9B658)
	// 82B9B618: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B61C: 556A053E  clrlwi r10, r11, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B620: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9B624: 7D4A2050  subf r10, r10, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 82B9B628: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9B62C: 7D0A5030  slw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9B630: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82B9B634: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9B638: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9B63C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9B640: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82B9B644: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B648: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9B64C: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 82B9B650: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9B654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B658 size=8
    let mut pc: u32 = 0x82B9B658;
    'dispatch: loop {
        match pc {
            0x82B9B658 => {
    //   block [0x82B9B658..0x82B9B660)
	// 82B9B658: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B65C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B660 size=48
    let mut pc: u32 = 0x82B9B660;
    'dispatch: loop {
        match pc {
            0x82B9B660 => {
    //   block [0x82B9B660..0x82B9B690)
	// 82B9B660: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B664: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B668: 41820028  beq 0x82b9b690
	if ctx.cr[0].eq {
		sub_82B9B690(ctx, base);
		return;
	}
	// 82B9B66C: 556AB63A  rlwinm r10, r11, 0x16, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82B9B670: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B674: 514B63A6  rlwimi r11, r10, 0xc, 0xe, 0x13
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(12) as u64) & 0x000000000003F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFC0FFF);
	// 82B9B678: 556AEE3A  rlwinm r10, r11, 0x1d, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9B67C: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B680: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B684: 514B2D74  rlwimi r11, r10, 5, 0x15, 0x1a
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(5) as u64) & 0x00000000000007E0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF81F);
	// 82B9B688: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B690 size=216
    let mut pc: u32 = 0x82B9B690;
    'dispatch: loop {
        match pc {
            0x82B9B690 => {
    //   block [0x82B9B690..0x82B9B768)
	// 82B9B690: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B694: 40820024  bne 0x82b9b6b8
	if !ctx.cr[0].eq {
	pc = 0x82B9B6B8; continue 'dispatch;
	}
	// 82B9B698: 556AD63A  rlwinm r10, r11, 0x1a, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B9B69C: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B6A0: 514B44AE  rlwimi r11, r10, 8, 0x12, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x0000000000003F00) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFC0FF);
	// 82B9B6A4: 556A163A  rlwinm r10, r11, 2, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82B9B6A8: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B6AC: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B6B0: 516A0032  rlwimi r10, r11, 0, 0, 0x19
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFC0) | (ctx.r[10].u64 & 0xFFFFFFFF0000003F);
	// 82B9B6B4: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B9B6B8: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82B9B6BC: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B6C0: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B6C4: 396B5E3C  addi r11, r11, 0x5e3c
	ctx.r[11].s64 = ctx.r[11].s64 + 24124;
	// 82B9B6C8: 552636BE  srwi r6, r9, 0x1a
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shr(26);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9B6CC: 38ABFFE0  addi r5, r11, -0x20
	ctx.r[5].s64 = ctx.r[11].s64 + -32;
	// 82B9B6D0: 554446FE  rlwinm r4, r10, 8, 0x1b, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82B9B6D4: 7CC658AE  lbzx r6, r6, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B9B6D8: 7D6428AE  lbzx r11, r4, r5
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82B9B6DC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82B9B6E0: 41980024  blt cr6, 0x82b9b704
	if ctx.cr[6].lt {
	pc = 0x82B9B704; continue 'dispatch;
	}
	// 82B9B6E4: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B6E8: 4182001C  beq 0x82b9b704
	if ctx.cr[0].eq {
	pc = 0x82B9B704; continue 'dispatch;
	}
	// 82B9B6EC: 89470009  lbz r10, 9(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(9 as u32) ) } as u64;
	// 82B9B6F0: 5545163A  rlwinm r5, r10, 2, 0x18, 0x1d
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82B9B6F4: 554A0032  rlwinm r10, r10, 0, 0, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9B6F8: 7CA5402E  lwzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B6FC: 7CAA5378  or r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 82B9B700: 99470009  stb r10, 9(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(9 as u32), ctx.r[10].u8 ) };
	// 82B9B704: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82B9B708: 41980028  blt cr6, 0x82b9b730
	if ctx.cr[6].lt {
	pc = 0x82B9B730; continue 'dispatch;
	}
	// 82B9B70C: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B710: 554A0043  rlwinm. r10, r10, 0, 1, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B714: 4182001C  beq 0x82b9b730
	if ctx.cr[0].eq {
	pc = 0x82B9B730; continue 'dispatch;
	}
	// 82B9B718: 8947000A  lbz r10, 0xa(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(10 as u32) ) } as u64;
	// 82B9B71C: 5545163A  rlwinm r5, r10, 2, 0x18, 0x1d
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82B9B720: 554A0032  rlwinm r10, r10, 0, 0, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9B724: 7CA5402E  lwzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B728: 7CAA5378  or r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 82B9B72C: 9947000A  stb r10, 0xa(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(10 as u32), ctx.r[10].u8 ) };
	// 82B9B730: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82B9B734: 4098000C  bge cr6, 0x82b9b740
	if !ctx.cr[6].lt {
	pc = 0x82B9B740; continue 'dispatch;
	}
	// 82B9B738: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82B9B73C: 409A0024  bne cr6, 0x82b9b760
	if !ctx.cr[6].eq {
	pc = 0x82B9B760; continue 'dispatch;
	}
	// 82B9B740: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B744: 556A0085  rlwinm. r10, r11, 0, 2, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B748: 41820018  beq 0x82b9b760
	if ctx.cr[0].eq {
	pc = 0x82B9B760; continue 'dispatch;
	}
	// 82B9B74C: 556A163A  rlwinm r10, r11, 2, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82B9B750: 556B0632  rlwinm r11, r11, 0, 0x18, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9B754: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B758: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9B75C: 9967000B  stb r11, 0xb(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(11 as u32), ctx.r[11].u8 ) };
	// 82B9B760: 2B060002  cmplwi cr6, r6, 2
	ctx.cr[6].compare_u32(ctx.r[6].u32, 2 as u32, &mut ctx.xer);
	// 82B9B764: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B768 size=72
    let mut pc: u32 = 0x82B9B768;
    'dispatch: loop {
        match pc {
            0x82B9B768 => {
    //   block [0x82B9B768..0x82B9B7B0)
	// 82B9B768: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B76C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82B9B770: 80C70004  lwz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B774: 516AF108  rlwimi r10, r11, 0x1e, 4, 4
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(30) as u64) & 0x0000000008000000) | (ctx.r[10].u64 & 0xFFFFFFFFF7FFFFFF);
	// 82B9B778: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82B9B77C: 514537BE  rlwimi r5, r10, 6, 0x1e, 0x1f
	ctx.r[5].u64 = (((ctx.r[10].u32).rotate_left(6) as u64) & 0x0000000000000003) | (ctx.r[5].u64 & 0xFFFFFFFFFFFFFFFC);
	// 82B9B780: 54AA06BE  clrlwi r10, r5, 0x1a
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 82B9B784: 5545163A  rlwinm r5, r10, 2, 0x18, 0x1d
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82B9B788: 554A0032  rlwinm r10, r10, 0, 0, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9B78C: 7D05402E  lwzx r8, r5, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B790: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 82B9B794: 514606BA  rlwimi r6, r10, 0, 0x1a, 0x1d
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x000000000000003C) | (ctx.r[6].u64 & 0xFFFFFFFFFFFFFFC3);
	// 82B9B798: 514BE084  rlwimi r11, r10, 0x1c, 2, 2
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(28) as u64) & 0x0000000020000000) | (ctx.r[11].u64 & 0xFFFFFFFFDFFFFFFF);
	// 82B9B79C: 5149D14A  rlwimi r9, r10, 0x1a, 5, 5
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(26) as u64) & 0x0000000004000000) | (ctx.r[9].u64 & 0xFFFFFFFFFBFFFFFF);
	// 82B9B7A0: 98C70007  stb r6, 7(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(7 as u32), ctx.r[6].u8 ) };
	// 82B9B7A4: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9B7A8: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B9B7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B7B0 size=8
    let mut pc: u32 = 0x82B9B7B0;
    'dispatch: loop {
        match pc {
            0x82B9B7B0 => {
    //   block [0x82B9B7B0..0x82B9B7B8)
	// 82B9B7B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B7B4: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B7B8 size=8
    let mut pc: u32 = 0x82B9B7B8;
    'dispatch: loop {
        match pc {
            0x82B9B7B8 => {
    //   block [0x82B9B7B8..0x82B9B7C0)
	// 82B9B7B8: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B7BC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B7C0 size=12
    let mut pc: u32 = 0x82B9B7C0;
    'dispatch: loop {
        match pc {
            0x82B9B7C0 => {
    //   block [0x82B9B7C0..0x82B9B7CC)
	// 82B9B7C0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B7C4: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B7C8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B7CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B7CC size=20
    let mut pc: u32 = 0x82B9B7CC;
    'dispatch: loop {
        match pc {
            0x82B9B7CC => {
    //   block [0x82B9B7CC..0x82B9B7E0)
	// 82B9B7CC: 556A163A  rlwinm r10, r11, 2, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82B9B7D0: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9B7D4: 516A0032  rlwimi r10, r11, 0, 0, 0x19
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFC0) | (ctx.r[10].u64 & 0xFFFFFFFF0000003F);
	// 82B9B7D8: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B9B7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B7E0 size=280
    let mut pc: u32 = 0x82B9B7E0;
    'dispatch: loop {
        match pc {
            0x82B9B7E0 => {
    //   block [0x82B9B7E0..0x82B9B8F8)
	// 82B9B7E0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82B9B7E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B7E8: 40820108  bne 0x82b9b8f0
	if !ctx.cr[0].eq {
	pc = 0x82B9B8F0; continue 'dispatch;
	}
	// 82B9B7EC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B7F0: 548A063F  clrlwi. r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B7F4: 41820020  beq 0x82b9b814
	if ctx.cr[0].eq {
	pc = 0x82B9B814; continue 'dispatch;
	}
	// 82B9B7F8: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B7FC: 552AA6BE  rlwinm r10, r9, 0x14, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82B9B800: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9B804: 41990008  bgt cr6, 0x82b9b80c
	if ctx.cr[6].gt {
	pc = 0x82B9B80C; continue 'dispatch;
	}
	// 82B9B808: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9B80C: 552ADEBE  rlwinm r10, r9, 0x1b, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9B810: 480000D0  b 0x82b9b8e0
	pc = 0x82B9B8E0; continue 'dispatch;
	// 82B9B814: 80C70000  lwz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B818: 54CA0421  rlwinm. r10, r6, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9B81C: 40820024  bne 0x82b9b840
	if !ctx.cr[0].eq {
	pc = 0x82B9B840; continue 'dispatch;
	}
	// 82B9B820: 54CAC6BE  rlwinm r10, r6, 0x18, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82B9B824: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9B828: 41990008  bgt cr6, 0x82b9b830
	if ctx.cr[6].gt {
	pc = 0x82B9B830; continue 'dispatch;
	}
	// 82B9B82C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9B830: 54CA06BE  clrlwi r10, r6, 0x1a
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x0000003Fu64;
	// 82B9B834: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9B838: 41990008  bgt cr6, 0x82b9b840
	if ctx.cr[6].gt {
	pc = 0x82B9B840; continue 'dispatch;
	}
	// 82B9B83C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9B840: 3D208331  lis r9, -0x7ccf
	ctx.r[9].s64 = -2093940736;
	// 82B9B844: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B848: 54C536BE  srwi r5, r6, 0x1a
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shr(26);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9B84C: 39295E3C  addi r9, r9, 0x5e3c
	ctx.r[9].s64 = ctx.r[9].s64 + 24124;
	// 82B9B850: 554346FE  rlwinm r3, r10, 8, 0x1b, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82B9B854: 3BE9FFE0  addi r31, r9, -0x20
	ctx.r[31].s64 = ctx.r[9].s64 + -32;
	// 82B9B858: 7C8548AE  lbzx r4, r5, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9B85C: 7CA3F8AE  lbzx r5, r3, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82B9B860: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82B9B864: 4198001C  blt cr6, 0x82b9b880
	if ctx.cr[6].lt {
	pc = 0x82B9B880; continue 'dispatch;
	}
	// 82B9B868: 55490001  rlwinm. r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9B86C: 41820014  beq 0x82b9b880
	if ctx.cr[0].eq {
	pc = 0x82B9B880; continue 'dispatch;
	}
	// 82B9B870: 554986BE  rlwinm r9, r10, 0x10, 0x1a, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9B874: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9B878: 41990008  bgt cr6, 0x82b9b880
	if ctx.cr[6].gt {
	pc = 0x82B9B880; continue 'dispatch;
	}
	// 82B9B87C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82B9B880: 2B050002  cmplwi cr6, r5, 2
	ctx.cr[6].compare_u32(ctx.r[5].u32, 2 as u32, &mut ctx.xer);
	// 82B9B884: 4198001C  blt cr6, 0x82b9b8a0
	if ctx.cr[6].lt {
	pc = 0x82B9B8A0; continue 'dispatch;
	}
	// 82B9B888: 55490043  rlwinm. r9, r10, 0, 1, 1
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9B88C: 41820014  beq 0x82b9b8a0
	if ctx.cr[0].eq {
	pc = 0x82B9B8A0; continue 'dispatch;
	}
	// 82B9B890: 5549C6BE  rlwinm r9, r10, 0x18, 0x1a, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9B894: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9B898: 41990008  bgt cr6, 0x82b9b8a0
	if ctx.cr[6].gt {
	pc = 0x82B9B8A0; continue 'dispatch;
	}
	// 82B9B89C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82B9B8A0: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 82B9B8A4: 4098000C  bge cr6, 0x82b9b8b0
	if !ctx.cr[6].lt {
	pc = 0x82B9B8B0; continue 'dispatch;
	}
	// 82B9B8A8: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82B9B8AC: 409A001C  bne cr6, 0x82b9b8c8
	if !ctx.cr[6].eq {
	pc = 0x82B9B8C8; continue 'dispatch;
	}
	// 82B9B8B0: 55490085  rlwinm. r9, r10, 0, 2, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9B8B4: 41820014  beq 0x82b9b8c8
	if ctx.cr[0].eq {
	pc = 0x82B9B8C8; continue 'dispatch;
	}
	// 82B9B8B8: 554906BE  clrlwi r9, r10, 0x1a
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82B9B8BC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9B8C0: 41990008  bgt cr6, 0x82b9b8c8
	if ctx.cr[6].gt {
	pc = 0x82B9B8C8; continue 'dispatch;
	}
	// 82B9B8C4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82B9B8C8: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 82B9B8CC: 409A0020  bne cr6, 0x82b9b8ec
	if !ctx.cr[6].eq {
	pc = 0x82B9B8EC; continue 'dispatch;
	}
	// 82B9B8D0: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B8D4: 5146F108  rlwimi r6, r10, 0x1e, 4, 4
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(30) as u64) & 0x0000000008000000) | (ctx.r[6].u64 & 0xFFFFFFFFF7FFFFFF);
	// 82B9B8D8: 50C937BE  rlwimi r9, r6, 6, 0x1e, 0x1f
	ctx.r[9].u64 = (((ctx.r[6].u32).rotate_left(6) as u64) & 0x0000000000000003) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFFC);
	// 82B9B8DC: 552A06BE  clrlwi r10, r9, 0x1a
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 82B9B8E0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9B8E4: 41990008  bgt cr6, 0x82b9b8ec
	if ctx.cr[6].gt {
	pc = 0x82B9B8EC; continue 'dispatch;
	}
	// 82B9B8E8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9B8EC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9B8F0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82B9B8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B8F8 size=36
    let mut pc: u32 = 0x82B9B8F8;
    'dispatch: loop {
        match pc {
            0x82B9B8F8 => {
    //   block [0x82B9B8F8..0x82B9B91C)
	// 82B9B8F8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B8FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B9B900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9B904: 552B053E  clrlwi r11, r9, 0x14
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82B9B908: 5527A77F  rlwinm. r7, r9, 0x14, 0x1d, 0x1f
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82B9B90C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B9B910: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82B9B914: 5524853E  rlwinm r4, r9, 0x10, 0x14, 0x1f
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82B9B918: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B91C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B91C size=124
    let mut pc: u32 = 0x82B9B91C;
    'dispatch: loop {
        match pc {
            0x82B9B91C => {
    //   block [0x82B9B91C..0x82B9B998)
	// 82B9B91C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9B920: 7C6B2830  slw r11, r3, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[3].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9B924: 7D6B2039  and. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B928: 40820058  bne 0x82b9b980
	if !ctx.cr[0].eq {
	pc = 0x82B9B980; continue 'dispatch;
	}
	// 82B9B92C: 896A0008  lbz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9B930: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9B934: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9B938: 552936BE  srwi r9, r9, 0x1a
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(26);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9B93C: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82B9B940: 41980010  blt cr6, 0x82b9b950
	if ctx.cr[6].lt {
	pc = 0x82B9B950; continue 'dispatch;
	}
	// 82B9B944: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82B9B948: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82B9B94C: 40990008  ble cr6, 0x82b9b954
	if !ctx.cr[6].gt {
	pc = 0x82B9B954; continue 'dispatch;
	}
	// 82B9B950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9B954: 5568063E  clrlwi r8, r11, 0x18
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82B9B958: 2B09001B  cmplwi cr6, r9, 0x1b
	ctx.cr[6].compare_u32(ctx.r[9].u32, 27 as u32, &mut ctx.xer);
	// 82B9B95C: 41980010  blt cr6, 0x82b9b96c
	if ctx.cr[6].lt {
	pc = 0x82B9B96C; continue 'dispatch;
	}
	// 82B9B960: 2B090022  cmplwi cr6, r9, 0x22
	ctx.cr[6].compare_u32(ctx.r[9].u32, 34 as u32, &mut ctx.xer);
	// 82B9B964: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82B9B968: 40990008  ble cr6, 0x82b9b970
	if !ctx.cr[6].gt {
	pc = 0x82B9B970; continue 'dispatch;
	}
	// 82B9B96C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9B970: 5509063F  clrlwi. r9, r8, 0x18
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9B974: 40820024  bne 0x82b9b998
	if !ctx.cr[0].eq {
		sub_82B9B998(ctx, base);
		return;
	}
	// 82B9B978: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B97C: 4082001C  bne 0x82b9b998
	if !ctx.cr[0].eq {
		sub_82B9B998(ctx, base);
		return;
	}
	// 82B9B980: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82B9B984: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 82B9B988: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82B9B98C: 7F063840  cmplw cr6, r6, r7
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82B9B990: 4198FF90  blt cr6, 0x82b9b920
	if ctx.cr[6].lt {
	pc = 0x82B9B920; continue 'dispatch;
	}
	// 82B9B994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9B998 size=8
    let mut pc: u32 = 0x82B9B998;
    'dispatch: loop {
        match pc {
            0x82B9B998 => {
    //   block [0x82B9B998..0x82B9B9A0)
	// 82B9B998: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9B99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82B9B9A0 size=208
    let mut pc: u32 = 0x82B9B9A0;
    'dispatch: loop {
        match pc {
            0x82B9B9A0 => {
    //   block [0x82B9B9A0..0x82B9BA70)
	// 82B9B9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9B9A4: 4810DA69  bl 0x82ca940c
	ctx.lr = 0x82B9B9A8;
	sub_82CA93D0(ctx, base);
	// 82B9B9A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9B9AC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82B9B9B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B9B4: 418200B4  beq 0x82b9ba68
	if ctx.cr[0].eq {
	pc = 0x82B9BA68; continue 'dispatch;
	}
	// 82B9B9B8: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9B9BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9B9C0: 57FEA73E  rlwinm r30, r31, 0x14, 0x1c, 0x1f
	ctx.r[30].u64 = ctx.r[31].u32 as u64 & 0x00000FFFu64;
	// 82B9B9C4: 7D6BF030  slw r11, r11, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9B9C8: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9B9CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9B9D0: 41820098  beq 0x82b9ba68
	if ctx.cr[0].eq {
	pc = 0x82B9BA68; continue 'dispatch;
	}
	// 82B9B9D4: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82B9B9D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9B9DC: 4BFFFF1D  bl 0x82b9b8f8
	ctx.lr = 0x82B9B9E0;
	sub_82B9B8F8(ctx, base);
	// 82B9B9E0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82B9B9E4: 2B1E0006  cmplwi cr6, r30, 6
	ctx.cr[6].compare_u32(ctx.r[30].u32, 6 as u32, &mut ctx.xer);
	// 82B9B9E8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9B9EC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9B9F0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82B9B9F4: 41990038  bgt cr6, 0x82b9ba2c
	if ctx.cr[6].gt {
	pc = 0x82B9BA2C; continue 'dispatch;
	}
	// 82B9B9F8: 2B1E0005  cmplwi cr6, r30, 5
	ctx.cr[6].compare_u32(ctx.r[30].u32, 5 as u32, &mut ctx.xer);
	// 82B9B9FC: 40980028  bge cr6, 0x82b9ba24
	if !ctx.cr[6].lt {
	pc = 0x82B9BA24; continue 'dispatch;
	}
	// 82B9BA00: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82B9BA04: 419A0064  beq cr6, 0x82b9ba68
	if ctx.cr[6].eq {
	pc = 0x82B9BA68; continue 'dispatch;
	}
	// 82B9BA08: 2B1E0002  cmplwi cr6, r30, 2
	ctx.cr[6].compare_u32(ctx.r[30].u32, 2 as u32, &mut ctx.xer);
	// 82B9BA0C: 40990018  ble cr6, 0x82b9ba24
	if !ctx.cr[6].gt {
	pc = 0x82B9BA24; continue 'dispatch;
	}
	// 82B9BA10: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 82B9BA14: 419A003C  beq cr6, 0x82b9ba50
	if ctx.cr[6].eq {
	pc = 0x82B9BA50; continue 'dispatch;
	}
	// 82B9BA18: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 82B9BA1C: 419A0020  beq cr6, 0x82b9ba3c
	if ctx.cr[6].eq {
	pc = 0x82B9BA3C; continue 'dispatch;
	}
	// 82B9BA20: 48000048  b 0x82b9ba68
	pc = 0x82B9BA68; continue 'dispatch;
	// 82B9BA24: 517F4DAC  rlwimi r31, r11, 9, 0x16, 0x16
	ctx.r[31].u64 = (((ctx.r[11].u32).rotate_left(9) as u64) & 0x0000000000000200) | (ctx.r[31].u64 & 0xFFFFFFFFFFFFFDFF);
	// 82B9BA28: 4800003C  b 0x82b9ba64
	pc = 0x82B9BA64; continue 'dispatch;
	// 82B9BA2C: 2B1E000D  cmplwi cr6, r30, 0xd
	ctx.cr[6].compare_u32(ctx.r[30].u32, 13 as u32, &mut ctx.xer);
	// 82B9BA30: 419A0020  beq cr6, 0x82b9ba50
	if ctx.cr[6].eq {
	pc = 0x82B9BA50; continue 'dispatch;
	}
	// 82B9BA34: 2B1E000E  cmplwi cr6, r30, 0xe
	ctx.cr[6].compare_u32(ctx.r[30].u32, 14 as u32, &mut ctx.xer);
	// 82B9BA38: 409A0030  bne cr6, 0x82b9ba68
	if !ctx.cr[6].eq {
	pc = 0x82B9BA68; continue 'dispatch;
	}
	// 82B9BA3C: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82B9BA40: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B9BA44: 716B000A  andi. r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u64 & 10;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BA48: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9BA4C: 48000014  b 0x82b9ba60
	pc = 0x82B9BA60; continue 'dispatch;
	// 82B9BA50: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82B9BA54: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B9BA58: 716B000A  andi. r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u64 & 10;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BA5C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82B9BA60: 517F6426  rlwimi r31, r11, 0xc, 0x10, 0x13
	ctx.r[31].u64 = (((ctx.r[11].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[31].u64 & 0xFFFFFFFFFFFF0FFF);
	// 82B9BA64: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82B9BA68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9BA6C: 4810D9F0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9BA70 size=208
    let mut pc: u32 = 0x82B9BA70;
    'dispatch: loop {
        match pc {
            0x82B9BA70 => {
    //   block [0x82B9BA70..0x82B9BB40)
	// 82B9BA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9BA74: 4810D999  bl 0x82ca940c
	ctx.lr = 0x82B9BA78;
	sub_82CA93D0(ctx, base);
	// 82B9BA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9BA7C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82B9BA80: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82B9BA84: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82B9BA88: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BA8C: 408200AC  bne 0x82b9bb38
	if !ctx.cr[0].eq {
	pc = 0x82B9BB38; continue 'dispatch;
	}
	// 82B9BA90: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BA94: 408200A4  bne 0x82b9bb38
	if !ctx.cr[0].eq {
	pc = 0x82B9BB38; continue 'dispatch;
	}
	// 82B9BA98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9BA9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9BAA0: 4BFFF839  bl 0x82b9b2d8
	ctx.lr = 0x82B9BAA4;
	sub_82B9B2D8(ctx, base);
	// 82B9BAA4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9BAA8: 41820090  beq 0x82b9bb38
	if ctx.cr[0].eq {
	pc = 0x82B9BB38; continue 'dispatch;
	}
	// 82B9BAAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BAB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9BAB4: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9BAB8: 556406BE  clrlwi r4, r11, 0x1a
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B9BABC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9BAC0: 7D6B2830  slw r11, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9BAC4: 7D6B3039  and. r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BAC8: 41820064  beq 0x82b9bb2c
	if ctx.cr[0].eq {
	pc = 0x82B9BB2C; continue 'dispatch;
	}
	// 82B9BACC: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82B9BAD0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9BAD4: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82B9BAD8: 7D6BF8AE  lbzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82B9BADC: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82B9BAE0: 419A004C  beq cr6, 0x82b9bb2c
	if ctx.cr[6].eq {
	pc = 0x82B9BB2C; continue 'dispatch;
	}
	// 82B9BAE4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9BAE8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BAEC: 396B0028  addi r11, r11, 0x28
	ctx.r[11].s64 = ctx.r[11].s64 + 40;
	// 82B9BAF0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9BAF4: 57BE053E  clrlwi r30, r29, 0x14
	ctx.r[30].u64 = ctx.r[29].u32 as u64 & 0x00000FFFu64;
	// 82B9BAF8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9BAFC: 7D0A4A2E  lhzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9BB00: 5508053E  clrlwi r8, r8, 0x14
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000FFFu64;
	// 82B9BB04: 7CEBFA2E  lhzx r7, r11, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82B9BB08: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82B9BB0C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9BB10: 7FC8192E  stwx r30, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[30].u32) };
	// 82B9BB14: 7D0BFA2E  lhzx r8, r11, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82B9BB18: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82B9BB1C: 7D0BFB2E  sthx r8, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u16) };
	// 82B9BB20: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9BB24: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9BB28: 7CC65878  andc r6, r6, r11
	ctx.r[6].u64 = ctx.r[6].u64 & !ctx.r[11].u64;
	// 82B9BB2C: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82B9BB30: 2B050004  cmplwi cr6, r5, 4
	ctx.cr[6].compare_u32(ctx.r[5].u32, 4 as u32, &mut ctx.xer);
	// 82B9BB34: 4198FF88  blt cr6, 0x82b9babc
	if ctx.cr[6].lt {
	pc = 0x82B9BABC; continue 'dispatch;
	}
	// 82B9BB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9BB3C: 4810D920  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9BB40 size=196
    let mut pc: u32 = 0x82B9BB40;
    'dispatch: loop {
        match pc {
            0x82B9BB40 => {
    //   block [0x82B9BB40..0x82B9BC04)
	// 82B9BB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9BB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9BB48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B9BB4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9BB50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9BB54: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82B9BB58: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82B9BB5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BB60: 4082008C  bne 0x82b9bbec
	if !ctx.cr[0].eq {
	pc = 0x82B9BBEC; continue 'dispatch;
	}
	// 82B9BB64: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BB68: 40820084  bne 0x82b9bbec
	if !ctx.cr[0].eq {
	pc = 0x82B9BBEC; continue 'dispatch;
	}
	// 82B9BB6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9BB70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9BB74: 4BFFF765  bl 0x82b9b2d8
	ctx.lr = 0x82B9BB78;
	sub_82B9B2D8(ctx, base);
	// 82B9BB78: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9BB7C: 41820070  beq 0x82b9bbec
	if ctx.cr[0].eq {
	pc = 0x82B9BBEC; continue 'dispatch;
	}
	// 82B9BB80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BB84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9BB88: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9BB8C: 556506BE  clrlwi r5, r11, 0x1a
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B9BB90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9BB94: 7D6B3030  slw r11, r11, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9BB98: 7D6B3839  and. r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BB9C: 41820044  beq 0x82b9bbe0
	if ctx.cr[0].eq {
	pc = 0x82B9BBE0; continue 'dispatch;
	}
	// 82B9BBA0: 39650002  addi r11, r5, 2
	ctx.r[11].s64 = ctx.r[5].s64 + 2;
	// 82B9BBA4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9BBA8: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82B9BBAC: 7D6BF8AE  lbzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82B9BBB0: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82B9BBB4: 419A002C  beq cr6, 0x82b9bbe0
	if ctx.cr[6].eq {
	pc = 0x82B9BBE0; continue 'dispatch;
	}
	// 82B9BBB8: 394B0024  addi r10, r11, 0x24
	ctx.r[10].s64 = ctx.r[11].s64 + 36;
	// 82B9BBBC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BBC0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9BBC4: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9BBC8: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82B9BBCC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9BBD0: 7D4BFB2E  sthx r10, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u16) };
	// 82B9BBD4: 7D69402E  lwzx r11, r9, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9BBD8: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9BBDC: 7CE75878  andc r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 & !ctx.r[11].u64;
	// 82B9BBE0: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82B9BBE4: 2B060004  cmplwi cr6, r6, 4
	ctx.cr[6].compare_u32(ctx.r[6].u32, 4 as u32, &mut ctx.xer);
	// 82B9BBE8: 4198FFA8  blt cr6, 0x82b9bb90
	if ctx.cr[6].lt {
	pc = 0x82B9BB90; continue 'dispatch;
	}
	// 82B9BBEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9BBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9BBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9BBF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9BBFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9BC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9BC08 size=8
    let mut pc: u32 = 0x82B9BC08;
    'dispatch: loop {
        match pc {
            0x82B9BC08 => {
    //   block [0x82B9BC08..0x82B9BC10)
	// 82B9BC08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BC0C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9BC10 size=8
    let mut pc: u32 = 0x82B9BC10;
    'dispatch: loop {
        match pc {
            0x82B9BC10 => {
    //   block [0x82B9BC10..0x82B9BC18)
	// 82B9BC10: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BC14: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9BC18 size=48
    let mut pc: u32 = 0x82B9BC18;
    'dispatch: loop {
        match pc {
            0x82B9BC18 => {
    //   block [0x82B9BC18..0x82B9BC48)
	// 82B9BC18: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BC1C: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9BC20: 4182003C  beq 0x82b9bc5c
	if ctx.cr[0].eq {
		sub_82B9BC48(ctx, base);
		return;
	}
	// 82B9BC24: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B9BC28: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82B9BC2C: 4199001C  bgt cr6, 0x82b9bc48
	if ctx.cr[6].gt {
		sub_82B9BC48(ctx, base);
		return;
	}
	// 82B9BC30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B9BC34: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BC38: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9BC3C: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82B9BC40: 512B0036  rlwimi r11, r9, 0, 0, 0x1b
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF0) | (ctx.r[11].u64 & 0xFFFFFFFF0000000F);
	// 82B9BC44: 48000014  b 0x82b9bc58
	sub_82B9BC48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9BC48 size=60
    let mut pc: u32 = 0x82B9BC48;
    'dispatch: loop {
        match pc {
            0x82B9BC48 => {
    //   block [0x82B9BC48..0x82B9BC84)
	// 82B9BC48: 2B0B003D  cmplwi cr6, r11, 0x3d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 61 as u32, &mut ctx.xer);
	// 82B9BC4C: 409A0010  bne cr6, 0x82b9bc5c
	if !ctx.cr[6].eq {
	pc = 0x82B9BC5C; continue 'dispatch;
	}
	// 82B9BC50: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BC54: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82B9BC58: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9BC5C: 89670008  lbz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9BC60: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9BC64: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82B9BC68: 4198000C  blt cr6, 0x82b9bc74
	if ctx.cr[6].lt {
	pc = 0x82B9BC74; continue 'dispatch;
	}
	// 82B9BC6C: 2B0B001B  cmplwi cr6, r11, 0x1b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 27 as u32, &mut ctx.xer);
	// 82B9BC70: 4099001C  ble cr6, 0x82b9bc8c
	if !ctx.cr[6].gt {
		sub_82B9BC8C(ctx, base);
		return;
	}
	// 82B9BC74: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BC78: 556B36BE  srwi r11, r11, 0x1a
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(26);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9BC7C: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82B9BC80: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BC84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9BC84 size=8
    let mut pc: u32 = 0x82B9BC84;
    'dispatch: loop {
        match pc {
            0x82B9BC84 => {
    //   block [0x82B9BC84..0x82B9BC8C)
	// 82B9BC84: 2B0B0027  cmplwi cr6, r11, 0x27
	ctx.cr[6].compare_u32(ctx.r[11].u32, 39 as u32, &mut ctx.xer);
	// 82B9BC88: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BC8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9BC8C size=16
    let mut pc: u32 = 0x82B9BC8C;
    'dispatch: loop {
        match pc {
            0x82B9BC8C => {
    //   block [0x82B9BC8C..0x82B9BC9C)
	// 82B9BC8C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BC90: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82B9BC94: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9BC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9BCA0 size=624
    //   switch @ 0x82B9BCDC: r9 with 12 label(s)
    //       case  0 → 0x82B9BCEC
    //       case  1 → 0x82B9BCEC
    //       case  2 → 0x82B9BD5C
    //       case  3 → 0x82B9BD5C
    //       case  4 → 0x82B9BD10
    //       case  5 → 0x82B9BD10
    //       case  6 → 0x82B9BD30
    //       case  7 → 0x82B9BD5C
    //       case  8 → 0x82B9BD30
    //       case  9 → 0x82B9BD48
    //       case 10 → 0x82B9BCEC
    //       case 11 → 0x82B9BCEC
    let mut pc: u32 = 0x82B9BCA0;
    'dispatch: loop {
        match pc {
            0x82B9BCA0 => {
    //   block [0x82B9BCA0..0x82B9BCEC)
	// 82B9BCA0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82B9BCA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BCA8: 418200BC  beq 0x82b9bd64
	if ctx.cr[0].eq {
	pc = 0x82B9BD64; continue 'dispatch;
	}
	// 82B9BCAC: 81470004  lwz r10, 4(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9BCB0: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BCB4: 5549A73E  rlwinm r9, r10, 0x14, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9BCB8: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82B9BCBC: 2B09000B  cmplwi cr6, r9, 0xb
	ctx.cr[6].compare_u32(ctx.r[9].u32, 11 as u32, &mut ctx.xer);
	// 82B9BCC0: 4199009C  bgt cr6, 0x82b9bd5c
	if ctx.cr[6].gt {
	pc = 0x82B9BD5C; continue 'dispatch;
	}
	// 82B9BCC4: 3D80820A  lis r12, -0x7df6
	ctx.r[12].s64 = -2113273856;
	// 82B9BCC8: 398C0310  addi r12, r12, 0x310
	ctx.r[12].s64 = ctx.r[12].s64 + 784;
	// 82B9BCCC: 7C0C48AE  lbzx r0, r12, r9
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9BCD0: 3D8082BA  lis r12, -0x7d46
	ctx.r[12].s64 = -2101739520;
	// 82B9BCD4: 398CBCEC  addi r12, r12, -0x4314
	ctx.r[12].s64 = ctx.r[12].s64 + -17172;
	// 82B9BCD8: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82B9BCDC: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82B9BCE0: 60000000  nop
	// 82B9BCE4: 60000000  nop
	// 82B9BCE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x82B9BCEC => {
    //   block [0x82B9BCEC..0x82B9BD10)
	// 82B9BCEC: 554AC7BE  rlwinm r10, r10, 0x18, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9BCF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B9BCF4: 214A0003  subfic r10, r10, 3
	ctx.xer.ca = ctx.r[10].u32 <= 3 as u32;
	ctx.r[10].s64 = (3 as i64) - ctx.r[10].s64;
	// 82B9BCF8: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9BCFC: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9BD00: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9BD04: 516A0416  rlwimi r10, r11, 0, 0x10, 0xb
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[10].u64 & 0x00000000000F0000);
	// 82B9BD08: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9BD0C: 48000050  b 0x82b9bd5c
	pc = 0x82B9BD5C; continue 'dispatch;
            }
            0x82B9BD10 => {
    //   block [0x82B9BD10..0x82B9BD30)
	// 82B9BD10: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BD14: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B9BD18: 554A7F3E  rlwinm r10, r10, 0xf, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0001FFFFu64;
	// 82B9BD1C: 214A000F  subfic r10, r10, 0xf
	ctx.xer.ca = ctx.r[10].u32 <= 15 as u32;
	ctx.r[10].s64 = (15 as i64) - ctx.r[10].s64;
	// 82B9BD20: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9BD24: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9BD28: 516A001E  rlwimi r10, r11, 0, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82B9BD2C: 4BFFFFDC  b 0x82b9bd08
	pc = 0x82B9BD08; continue 'dispatch;
            }
            0x82B9BD30 => {
    //   block [0x82B9BD30..0x82B9BD48)
	// 82B9BD30: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BD34: 552704A5  rlwinm. r7, r9, 0, 0x12, 0x12
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82B9BD38: 40820024  bne 0x82b9bd5c
	if !ctx.cr[0].eq {
	pc = 0x82B9BD5C; continue 'dispatch;
	}
	// 82B9BD3C: 55290463  rlwinm. r9, r9, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9BD40: 4082001C  bne 0x82b9bd5c
	if !ctx.cr[0].eq {
	pc = 0x82B9BD5C; continue 'dispatch;
	}
	// 82B9BD44: 4BFFFFA8  b 0x82b9bcec
	pc = 0x82B9BCEC; continue 'dispatch;
            }
            0x82B9BD48 => {
    //   block [0x82B9BD48..0x82B9BD5C)
	// 82B9BD48: 554A056C  rlwinm r10, r10, 0, 0x15, 0x16
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9BD4C: 2B0A0600  cmplwi cr6, r10, 0x600
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1536 as u32, &mut ctx.xer);
	// 82B9BD50: 409A000C  bne cr6, 0x82b9bd5c
	if !ctx.cr[6].eq {
	pc = 0x82B9BD5C; continue 'dispatch;
	}
	// 82B9BD54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B9BD58: 91480018  stw r10, 0x18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	pc = 0x82B9BD5C; continue 'dispatch;
            }
            0x82B9BD5C => {
    //   block [0x82B9BD5C..0x82B9BF10)
	// 82B9BD5C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9BD60: 480001A8  b 0x82b9bf08
	pc = 0x82B9BF08; continue 'dispatch;
	// 82B9BD64: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9BD68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B9BD6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BD70: 409A0034  bne cr6, 0x82b9bda4
	if !ctx.cr[6].eq {
	pc = 0x82B9BDA4; continue 'dispatch;
	}
	// 82B9BD74: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BD78: 9068000C  stw r3, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82B9BD7C: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9BD80: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9BD84: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82B9BD88: 91680010  stw r11, 0x10(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82B9BD8C: 41820010  beq 0x82b9bd9c
	if ctx.cr[0].eq {
	pc = 0x82B9BD9C; continue 'dispatch;
	}
	// 82B9BD90: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BD94: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82B9BD98: 41820008  beq 0x82b9bda0
	if ctx.cr[0].eq {
	pc = 0x82B9BDA0; continue 'dispatch;
	}
	// 82B9BD9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9BDA0: 91680014  stw r11, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82B9BDA4: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9BDA8: 548A063F  clrlwi. r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9BDAC: 4182002C  beq 0x82b9bdd8
	if ctx.cr[0].eq {
	pc = 0x82B9BDD8; continue 'dispatch;
	}
	// 82B9BDB0: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BDB4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82B9BDB8: 554906FF  clrlwi. r9, r10, 0x1b
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9BDBC: 5549A6BE  rlwinm r9, r10, 0x14, 0x1a, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9BDC0: 419A000C  beq cr6, 0x82b9bdcc
	if ctx.cr[6].eq {
	pc = 0x82B9BDCC; continue 'dispatch;
	}
	// 82B9BDC4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9BDC8: 40980008  bge cr6, 0x82b9bdd0
	if !ctx.cr[6].lt {
	pc = 0x82B9BDD0; continue 'dispatch;
	}
	// 82B9BDCC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82B9BDD0: 554ADEBE  rlwinm r10, r10, 0x1b, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9BDD4: 4800011C  b 0x82b9bef0
	pc = 0x82B9BEF0; continue 'dispatch;
	// 82B9BDD8: 3D408331  lis r10, -0x7ccf
	ctx.r[10].s64 = -2093940736;
	// 82B9BDDC: 81270008  lwz r9, 8(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9BDE0: 80A70000  lwz r5, 0(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BDE4: 394A5E3C  addi r10, r10, 0x5e3c
	ctx.r[10].s64 = ctx.r[10].s64 + 24124;
	// 82B9BDE8: 552646FE  rlwinm r6, r9, 8, 0x1b, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82B9BDEC: 388AFFE0  addi r4, r10, -0x20
	ctx.r[4].s64 = ctx.r[10].s64 + -32;
	// 82B9BDF0: 54BF36BE  srwi r31, r5, 0x1a
	ctx.r[31].u32 = ctx.r[5].u32.wrapping_shr(26);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82B9BDF4: 7CC620AE  lbzx r6, r6, r4
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82B9BDF8: 7C9F50AE  lbzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82B9BDFC: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82B9BE00: 41980028  blt cr6, 0x82b9be28
	if ctx.cr[6].lt {
	pc = 0x82B9BE28; continue 'dispatch;
	}
	// 82B9BE04: 552A0001  rlwinm. r10, r9, 0, 0, 0
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9BE08: 41820020  beq 0x82b9be28
	if ctx.cr[0].eq {
	pc = 0x82B9BE28; continue 'dispatch;
	}
	// 82B9BE0C: 89470009  lbz r10, 9(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(9 as u32) ) } as u64;
	// 82B9BE10: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82B9BE14: 554A06BE  clrlwi r10, r10, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82B9BE18: 419A000C  beq cr6, 0x82b9be24
	if ctx.cr[6].eq {
	pc = 0x82B9BE24; continue 'dispatch;
	}
	// 82B9BE1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9BE20: 40980008  bge cr6, 0x82b9be28
	if !ctx.cr[6].lt {
	pc = 0x82B9BE28; continue 'dispatch;
	}
	// 82B9BE24: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9BE28: 2B060002  cmplwi cr6, r6, 2
	ctx.cr[6].compare_u32(ctx.r[6].u32, 2 as u32, &mut ctx.xer);
	// 82B9BE2C: 41980028  blt cr6, 0x82b9be54
	if ctx.cr[6].lt {
	pc = 0x82B9BE54; continue 'dispatch;
	}
	// 82B9BE30: 552A0043  rlwinm. r10, r9, 0, 1, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9BE34: 41820020  beq 0x82b9be54
	if ctx.cr[0].eq {
	pc = 0x82B9BE54; continue 'dispatch;
	}
	// 82B9BE38: 8947000A  lbz r10, 0xa(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(10 as u32) ) } as u64;
	// 82B9BE3C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82B9BE40: 554A06BE  clrlwi r10, r10, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82B9BE44: 419A000C  beq cr6, 0x82b9be50
	if ctx.cr[6].eq {
	pc = 0x82B9BE50; continue 'dispatch;
	}
	// 82B9BE48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9BE4C: 40980008  bge cr6, 0x82b9be54
	if !ctx.cr[6].lt {
	pc = 0x82B9BE54; continue 'dispatch;
	}
	// 82B9BE50: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9BE54: 2B060003  cmplwi cr6, r6, 3
	ctx.cr[6].compare_u32(ctx.r[6].u32, 3 as u32, &mut ctx.xer);
	// 82B9BE58: 4098000C  bge cr6, 0x82b9be64
	if !ctx.cr[6].lt {
	pc = 0x82B9BE64; continue 'dispatch;
	}
	// 82B9BE5C: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82B9BE60: 409A0024  bne cr6, 0x82b9be84
	if !ctx.cr[6].eq {
	pc = 0x82B9BE84; continue 'dispatch;
	}
	// 82B9BE64: 552A0085  rlwinm. r10, r9, 0, 2, 2
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9BE68: 4182001C  beq 0x82b9be84
	if ctx.cr[0].eq {
	pc = 0x82B9BE84; continue 'dispatch;
	}
	// 82B9BE6C: 552A06BE  clrlwi r10, r9, 0x1a
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 82B9BE70: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82B9BE74: 419A000C  beq cr6, 0x82b9be80
	if ctx.cr[6].eq {
	pc = 0x82B9BE80; continue 'dispatch;
	}
	// 82B9BE78: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9BE7C: 40980008  bge cr6, 0x82b9be84
	if !ctx.cr[6].lt {
	pc = 0x82B9BE84; continue 'dispatch;
	}
	// 82B9BE80: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9BE84: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 82B9BE88: 409A002C  bne cr6, 0x82b9beb4
	if !ctx.cr[6].eq {
	pc = 0x82B9BEB4; continue 'dispatch;
	}
	// 82B9BE8C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82B9BE90: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9BE94: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82B9BE98: 512AF108  rlwimi r10, r9, 0x1e, 4, 4
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(30) as u64) & 0x0000000008000000) | (ctx.r[10].u64 & 0xFFFFFFFFF7FFFFFF);
	// 82B9BE9C: 514737BE  rlwimi r7, r10, 6, 0x1e, 0x1f
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(6) as u64) & 0x0000000000000003) | (ctx.r[7].u64 & 0xFFFFFFFFFFFFFFFC);
	// 82B9BEA0: 54EA06BE  clrlwi r10, r7, 0x1a
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x0000003Fu64;
	// 82B9BEA4: 419A000C  beq cr6, 0x82b9beb0
	if ctx.cr[6].eq {
	pc = 0x82B9BEB0; continue 'dispatch;
	}
	// 82B9BEA8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9BEAC: 40980008  bge cr6, 0x82b9beb4
	if !ctx.cr[6].lt {
	pc = 0x82B9BEB4; continue 'dispatch;
	}
	// 82B9BEB0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9BEB4: 54AA0421  rlwinm. r10, r5, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9BEB8: 54AA06BE  clrlwi r10, r5, 0x1a
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 82B9BEBC: 4182001C  beq 0x82b9bed8
	if ctx.cr[0].eq {
	pc = 0x82B9BED8; continue 'dispatch;
	}
	// 82B9BEC0: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 82B9BEC4: 41980040  blt cr6, 0x82b9bf04
	if ctx.cr[6].lt {
	pc = 0x82B9BF04; continue 'dispatch;
	}
	// 82B9BEC8: 2B0A0025  cmplwi cr6, r10, 0x25
	ctx.cr[6].compare_u32(ctx.r[10].u32, 37 as u32, &mut ctx.xer);
	// 82B9BECC: 41990038  bgt cr6, 0x82b9bf04
	if ctx.cr[6].gt {
	pc = 0x82B9BF04; continue 'dispatch;
	}
	// 82B9BED0: 90680018  stw r3, 0x18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82B9BED4: 48000030  b 0x82b9bf04
	pc = 0x82B9BF04; continue 'dispatch;
	// 82B9BED8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82B9BEDC: 419A000C  beq cr6, 0x82b9bee8
	if ctx.cr[6].eq {
	pc = 0x82B9BEE8; continue 'dispatch;
	}
	// 82B9BEE0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9BEE4: 40980008  bge cr6, 0x82b9beec
	if !ctx.cr[6].lt {
	pc = 0x82B9BEEC; continue 'dispatch;
	}
	// 82B9BEE8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9BEEC: 54AAC6BE  rlwinm r10, r5, 0x18, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82B9BEF0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82B9BEF4: 419A000C  beq cr6, 0x82b9bf00
	if ctx.cr[6].eq {
	pc = 0x82B9BF00; continue 'dispatch;
	}
	// 82B9BEF8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9BEFC: 40980008  bge cr6, 0x82b9bf04
	if !ctx.cr[6].lt {
	pc = 0x82B9BF04; continue 'dispatch;
	}
	// 82B9BF00: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82B9BF04: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9BF08: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82B9BF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9BF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9BF10 size=992
    let mut pc: u32 = 0x82B9BF10;
    'dispatch: loop {
        match pc {
            0x82B9BF10 => {
    //   block [0x82B9BF10..0x82B9C2F0)
	// 82B9BF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9BF14: 4810D4E5  bl 0x82ca93f8
	ctx.lr = 0x82B9BF18;
	sub_82CA93D0(ctx, base);
	// 82B9BF18: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9BF1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9BF20: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82B9BF24: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82B9BF28: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9BF2C: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9BF30: 39240001  addi r9, r4, 1
	ctx.r[9].s64 = ctx.r[4].s64 + 1;
	// 82B9BF34: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9BF38: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9BF3C: 553AF87E  srwi r26, r9, 1
	ctx.r[26].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82B9BF40: 396B001F  addi r11, r11, 0x1f
	ctx.r[11].s64 = ctx.r[11].s64 + 31;
	// 82B9BF44: 7D4AD214  add r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82B9BF48: 556B0034  rlwinm r11, r11, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9BF4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9BF50: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9BF54: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82B9BF58: 1F6A000C  mulli r27, r10, 0xc
	ctx.r[27].s64 = ctx.r[10].s64 * 12;
	// 82B9BF5C: 409A0014  bne cr6, 0x82b9bf70
	if !ctx.cr[6].eq {
	pc = 0x82B9BF70; continue 'dispatch;
	}
	// 82B9BF60: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82B9BF64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9BF68: 4BFFD959  bl 0x82b998c0
	ctx.lr = 0x82B9BF6C;
	sub_82B998C0(ctx, base);
	// 82B9BF6C: 4800037C  b 0x82b9c2e8
	pc = 0x82B9C2E8; continue 'dispatch;
	// 82B9BF70: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9BF74: 93780004  stw r27, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82B9BF78: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9BF7C: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9BF80: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9BF84: 7D6ADA14  add r11, r10, r27
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82B9BF88: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82B9BF8C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9BF90: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9BF94: 40990008  ble cr6, 0x82b9bf9c
	if !ctx.cr[6].gt {
	pc = 0x82B9BF9C; continue 'dispatch;
	}
	// 82B9BF98: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9BF9C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9BFA0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82B9BFA4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9BFA8: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82B9BFAC: 41990008  bgt cr6, 0x82b9bfb4
	if ctx.cr[6].gt {
	pc = 0x82B9BFB4; continue 'dispatch;
	}
	// 82B9BFB0: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82B9BFB4: 80DF006C  lwz r6, 0x6c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9BFB8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82B9BFBC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82B9BFC0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82B9BFC4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9BFC8: 419A00B8  beq cr6, 0x82b9c080
	if ctx.cr[6].eq {
	pc = 0x82B9C080; continue 'dispatch;
	}
	// 82B9BFCC: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82B9BFD0: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 82B9BFD4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82B9BFD8: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B9BFDC: 550BA73E  rlwinm r11, r8, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x00000FFFu64;
	// 82B9BFE0: 7F2B5830  slw r11, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[25].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9BFE4: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9BFE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9BFEC: 41820014  beq 0x82b9c000
	if ctx.cr[0].eq {
	pc = 0x82B9C000; continue 'dispatch;
	}
	// 82B9BFF0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9BFF4: 7CABD214  add r5, r11, r26
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82B9BFF8: 51650026  rlwimi r5, r11, 0, 0, 0x13
	ctx.r[5].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[5].u64 & 0xFFFFFFFF00000FFF);
	// 82B9BFFC: 48000008  b 0x82b9c004
	pc = 0x82B9C004; continue 'dispatch;
	// 82B9C000: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C004: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82B9C008: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82B9C00C: 40980038  bge cr6, 0x82b9c044
	if !ctx.cr[6].lt {
	pc = 0x82B9C044; continue 'dispatch;
	}
	// 82B9C010: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82B9C014: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 82B9C018: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82B9C01C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B9C020: 552BA73E  rlwinm r11, r9, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82B9C024: 7F2B5830  slw r11, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[25].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C028: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C02C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9C030: 41820020  beq 0x82b9c050
	if ctx.cr[0].eq {
	pc = 0x82B9C050; continue 'dispatch;
	}
	// 82B9C034: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C038: 7D67D214  add r11, r7, r26
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[26].u64;
	// 82B9C03C: 50EB0026  rlwimi r11, r7, 0, 0, 0x13
	ctx.r[11].u64 = (((ctx.r[7].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[11].u64 & 0xFFFFFFFF00000FFF);
	// 82B9C040: 48000014  b 0x82b9c054
	pc = 0x82B9C054; continue 'dispatch;
	// 82B9C044: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82B9C048: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82B9C04C: 48000008  b 0x82b9c054
	pc = 0x82B9C054; continue 'dispatch;
	// 82B9C050: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C054: 5567843E  srwi r7, r11, 0x10
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9C058: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82B9C05C: 5529801E  slwi r9, r9, 0x10
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9C060: 5168801E  rlwimi r8, r11, 0x10, 0, 0xf
	ctx.r[8].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[8].u64 & 0xFFFFFFFF0000FFFF);
	// 82B9C064: 7CEB4B78  or r11, r7, r9
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 82B9C068: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82B9C06C: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82B9C070: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9C074: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82B9C078: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82B9C07C: 4198FF50  blt cr6, 0x82b9bfcc
	if ctx.cr[6].lt {
	pc = 0x82B9BFCC; continue 'dispatch;
	}
	// 82B9C080: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9C084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9C088: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9C08C: 1CAB000C  mulli r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 * 12;
	// 82B9C090: 4810D3F1  bl 0x82ca9480
	ctx.lr = 0x82B9C094;
	sub_82CA9480(ctx, base);
	// 82B9C094: 813F0154  lwz r9, 0x154(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82B9C098: 939F0100  stw r28, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[28].u32 ) };
	// 82B9C09C: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 82B9C0A0: 3D6082BA  lis r11, -0x7d46
	ctx.r[11].s64 = -2101739520;
	// 82B9C0A4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82B9C0A8: 38CBBCA0  addi r6, r11, -0x4360
	ctx.r[6].s64 = ctx.r[11].s64 + -17248;
	// 82B9C0AC: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82B9C0B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9C0B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82B9C0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9C0BC: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9C0C0: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B9C0C4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82B9C0C8: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 82B9C0CC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B9C0D0: 813F0158  lwz r9, 0x158(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 82B9C0D4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82B9C0D8: 813F015C  lwz r9, 0x15c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 82B9C0DC: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82B9C0E0: 93780004  stw r27, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82B9C0E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C0E8: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9C0EC: 556BF7FE  rlwinm r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B9C0F0: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82B9C0F4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82B9C0F8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82B9C0FC: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82B9C100: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82B9C104: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82B9C108: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82B9C10C: 480001E5  bl 0x82b9c2f0
	ctx.lr = 0x82B9C110;
	sub_82B9C2F0(ctx, base);
	// 82B9C110: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82B9C114: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C118: 419A000C  beq cr6, 0x82b9c124
	if ctx.cr[6].eq {
	pc = 0x82B9C124; continue 'dispatch;
	}
	// 82B9C11C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C120: 932B4DF0  stw r25, 0x4df0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(19952 as u32), ctx.r[25].u32 ) };
	// 82B9C124: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9C128: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82B9C12C: 91580010  stw r10, 0x10(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82B9C130: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C134: 814A4DB4  lwz r10, 0x4db4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9C138: 5549EFFF  rlwinm. r9, r10, 0x1d, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9C13C: 41820014  beq 0x82b9c150
	if ctx.cr[0].eq {
	pc = 0x82B9C150; continue 'dispatch;
	}
	// 82B9C140: 554BF7FF  rlwinm. r11, r10, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C144: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82B9C148: 40820008  bne 0x82b9c150
	if !ctx.cr[0].eq {
	pc = 0x82B9C150; continue 'dispatch;
	}
	// 82B9C14C: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 82B9C150: 554AF7FF  rlwinm. r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C154: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82B9C158: 41820080  beq 0x82b9c1d8
	if ctx.cr[0].eq {
	pc = 0x82B9C1D8; continue 'dispatch;
	}
	// 82B9C15C: 815F0110  lwz r10, 0x110(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 82B9C160: 813F010C  lwz r9, 0x10c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 82B9C164: 514B2DB4  rlwimi r11, r10, 5, 0x16, 0x1a
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(5) as u64) & 0x00000000000003E0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFC1F);
	// 82B9C168: 556A05B5  rlwinm. r10, r11, 0, 0x16, 0x1a
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C16C: 418200E4  beq 0x82b9c250
	if ctx.cr[0].eq {
	pc = 0x82B9C250; continue 'dispatch;
	}
	// 82B9C170: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82B9C174: 38CA1738  addi r6, r10, 0x1738
	ctx.r[6].s64 = ctx.r[10].s64 + 5944;
	// 82B9C178: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C17C: 54E5043E  clrlwi r5, r7, 0x10
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82B9C180: 5548873E  rlwinm r8, r10, 0x10, 0x1c, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9C184: 554A673E  rlwinm r10, r10, 0xc, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 82B9C188: 7F2A5030  slw r10, r25, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[25].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C18C: 7D0830AE  lbzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82B9C190: 7D4A2B78  or r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 82B9C194: 5508503E  rotlwi r8, r8, 0xa
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(10)) as u64;
	// 82B9C198: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9C19C: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82B9C1A0: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82B9C1A4: 5168059C  rlwimi r8, r11, 0, 0x16, 0xe
	ctx.r[8].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFE03FF) | (ctx.r[8].u64 & 0x000000000001FC00);
	// 82B9C1A8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82B9C1AC: 419A0014  beq cr6, 0x82b9c1c0
	if ctx.cr[6].eq {
	pc = 0x82B9C1C0; continue 'dispatch;
	}
	// 82B9C1B0: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82B9C1B4: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82B9C1B8: 51680034  rlwimi r8, r11, 0, 0, 0x1a
	ctx.r[8].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFE0) | (ctx.r[8].u64 & 0xFFFFFFFF0000001F);
	// 82B9C1BC: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82B9C1C0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82B9C1C4: 556ADEFE  rlwinm r10, r11, 0x1b, 0x1b, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9C1C8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9C1CC: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9C1D0: 4198FFA8  blt cr6, 0x82b9c178
	if ctx.cr[6].lt {
	pc = 0x82B9C178; continue 'dispatch;
	}
	// 82B9C1D4: 4800007C  b 0x82b9c250
	pc = 0x82B9C250; continue 'dispatch;
	// 82B9C1D8: 815F0134  lwz r10, 0x134(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82B9C1DC: 813F0130  lwz r9, 0x130(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82B9C1E0: 514B2DB4  rlwimi r11, r10, 5, 0x16, 0x1a
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(5) as u64) & 0x00000000000003E0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFC1F);
	// 82B9C1E4: 556A05B5  rlwinm. r10, r11, 0, 0x16, 0x1a
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C1E8: 41820068  beq 0x82b9c250
	if ctx.cr[0].eq {
	pc = 0x82B9C250; continue 'dispatch;
	}
	// 82B9C1EC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82B9C1F0: 38CA1738  addi r6, r10, 0x1738
	ctx.r[6].s64 = ctx.r[10].s64 + 5944;
	// 82B9C1F4: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C1F8: 54E5043E  clrlwi r5, r7, 0x10
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82B9C1FC: 5548A73E  rlwinm r8, r10, 0x14, 0x1c, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9C200: 554AC73E  rlwinm r10, r10, 0x18, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9C204: 7F2A5030  slw r10, r25, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[25].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C208: 7D0830AE  lbzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82B9C20C: 7D4A2B78  or r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 82B9C210: 5508503E  rotlwi r8, r8, 0xa
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(10)) as u64;
	// 82B9C214: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9C218: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82B9C21C: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82B9C220: 5168059C  rlwimi r8, r11, 0, 0x16, 0xe
	ctx.r[8].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFE03FF) | (ctx.r[8].u64 & 0x000000000001FC00);
	// 82B9C224: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82B9C228: 419A0014  beq cr6, 0x82b9c23c
	if ctx.cr[6].eq {
	pc = 0x82B9C23C; continue 'dispatch;
	}
	// 82B9C22C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82B9C230: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82B9C234: 51680034  rlwimi r8, r11, 0, 0, 0x1a
	ctx.r[8].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFE0) | (ctx.r[8].u64 & 0xFFFFFFFF0000001F);
	// 82B9C238: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82B9C23C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82B9C240: 556ADEFE  rlwinm r10, r11, 0x1b, 0x1b, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9C244: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9C248: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9C24C: 4198FFA8  blt cr6, 0x82b9c1f4
	if ctx.cr[6].lt {
	pc = 0x82B9C1F4; continue 'dispatch;
	}
	// 82B9C250: 91780014  stw r11, 0x14(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82B9C254: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C258: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9C25C: 556AF7FF  rlwinm. r10, r11, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C260: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9C264: 4182003C  beq 0x82b9c2a0
	if ctx.cr[0].eq {
	pc = 0x82B9C2A0; continue 'dispatch;
	}
	// 82B9C268: 556BCFFF  rlwinm. r11, r11, 0x19, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C26C: 4182000C  beq 0x82b9c278
	if ctx.cr[0].eq {
	pc = 0x82B9C278; continue 'dispatch;
	}
	// 82B9C270: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9C274: 4800000C  b 0x82b9c280
	pc = 0x82B9C280; continue 'dispatch;
	// 82B9C278: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9C27C: 556B7FFE  rlwinm r11, r11, 0xf, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 82B9C280: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9C284: 813F0150  lwz r9, 0x150(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82B9C288: 811F005C  lwz r8, 0x5c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82B9C28C: 516A8B9C  rlwimi r10, r11, 0x11, 0xe, 0xe
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(17) as u64) & 0x0000000000020000) | (ctx.r[10].u64 & 0xFFFFFFFFFFFDFFFF);
	// 82B9C290: 5128173A  rlwimi r8, r9, 2, 0x1c, 0x1d
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(2) as u64) & 0x000000000000000C) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFF3);
	// 82B9C294: 754A780E  andis. r10, r10, 0x780e
	ctx.r[10].u64 = ctx.r[10].u64 & 2014183424;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C298: 710BFF0E  andi. r11, r8, 0xff0e
	ctx.r[11].u64 = ctx.r[8].u64 & 65294;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C29C: 48000044  b 0x82b9c2e0
	pc = 0x82B9C2E0; continue 'dispatch;
	// 82B9C2A0: 556ACFFF  rlwinm. r10, r11, 0x19, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C2A4: 4182000C  beq 0x82b9c2b0
	if ctx.cr[0].eq {
	pc = 0x82B9C2B0; continue 'dispatch;
	}
	// 82B9C2A8: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9C2AC: 4800000C  b 0x82b9c2b8
	pc = 0x82B9C2B8; continue 'dispatch;
	// 82B9C2B0: A17F0058  lhz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9C2B4: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82B9C2B8: 813F0058  lwz r9, 0x58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9C2BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B9C2C0: 516983DE  rlwimi r9, r11, 0x10, 0xf, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x0000000000010000) | (ctx.r[9].u64 & 0xFFFFFFFFFFFEFFFF);
	// 82B9C2C4: 752A87F1  andis. r10, r9, 0x87f1
	ctx.r[10].u64 = ctx.r[9].u64 & 2280718336;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C2C8: 419A000C  beq cr6, 0x82b9c2d4
	if ctx.cr[6].eq {
	pc = 0x82B9C2D4; continue 'dispatch;
	}
	// 82B9C2CC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82B9C2D0: 4800000C  b 0x82b9c2dc
	pc = 0x82B9C2DC; continue 'dispatch;
	// 82B9C2D4: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82B9C2D8: 556B77FE  rlwinm r11, r11, 0xe, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0003FFFFu64;
	// 82B9C2DC: 556B935A  rlwinm r11, r11, 0x12, 0xd, 0xd
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82B9C2E0: 91580008  stw r10, 8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82B9C2E4: 9178000C  stw r11, 0xc(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82B9C2E8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82B9C2EC: 4810D15C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9C2F0 size=588
    let mut pc: u32 = 0x82B9C2F0;
    'dispatch: loop {
        match pc {
            0x82B9C2F0 => {
    //   block [0x82B9C2F0..0x82B9C53C)
	// 82B9C2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9C2F4: 4810D0DD  bl 0x82ca93d0
	ctx.lr = 0x82B9C2F8;
	sub_82CA93D0(ctx, base);
	// 82B9C2F8: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9C2FC: 83030100  lwz r24, 0x100(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82B9C300: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82B9C304: 83830104  lwz r28, 0x104(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(260 as u32) ) } as u64;
	// 82B9C308: 7CD13378  mr r17, r6
	ctx.r[17].u64 = ctx.r[6].u64;
	// 82B9C30C: 7CF03B78  mr r16, r7
	ctx.r[16].u64 = ctx.r[7].u64;
	// 82B9C310: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C314: 3B60000C  li r27, 0xc
	ctx.r[27].s64 = 12;
	// 82B9C318: 418200C4  beq 0x82b9c3dc
	if ctx.cr[0].eq {
	pc = 0x82B9C3DC; continue 'dispatch;
	}
	// 82B9C31C: 7D78E050  subf r11, r24, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[24].s64;
	// 82B9C320: 7FABDBD7  divw. r29, r11, r27
	ctx.r[29].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9C324: 418200B8  beq 0x82b9c3dc
	if ctx.cr[0].eq {
	pc = 0x82B9C3DC; continue 'dispatch;
	}
	// 82B9C328: 3BF80008  addi r31, r24, 8
	ctx.r[31].s64 = ctx.r[24].s64 + 8;
	// 82B9C32C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B9C330: 817FFFF8  lwz r11, -8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9C334: 7E088378  mr r8, r16
	ctx.r[8].u64 = ctx.r[16].u64;
	// 82B9C338: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82B9C33C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82B9C340: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9C344: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9C348: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82B9C34C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B9C350: A17FFFFE  lhz r11, -2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82B9C354: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82B9C358: A17FFFFC  lhz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9C35C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C360: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C364: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9C368: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9C36C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C370: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9C374: 7E2903A6  mtctr r17
	ctx.ctr.u64 = ctx.r[17].u64;
	// 82B9C378: 4E800421  bctrl
	ctx.lr = 0x82B9C37C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9C37C: 7E088378  mr r8, r16
	ctx.r[8].u64 = ctx.r[16].u64;
	// 82B9C380: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82B9C384: 38DE0001  addi r6, r30, 1
	ctx.r[6].s64 = ctx.r[30].s64 + 1;
	// 82B9C388: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9C38C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9C390: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B9C394: 7E2903A6  mtctr r17
	ctx.ctr.u64 = ctx.r[17].u64;
	// 82B9C398: 4E800421  bctrl
	ctx.lr = 0x82B9C39C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9C39C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9C3A0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9C3A4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82B9C3A8: 917FFFF8  stw r11, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82B9C3AC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C3B0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82B9C3B4: 514B801E  rlwimi r11, r10, 0x10, 0, 0xf
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF0000FFFF);
	// 82B9C3B8: 917FFFFC  stw r11, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82B9C3BC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C3C0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B9C3C4: 556B801E  slwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9C3C8: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C3CC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9C3D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9C3D4: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82B9C3D8: 4082FF58  bne 0x82b9c330
	if !ctx.cr[0].eq {
	pc = 0x82B9C330; continue 'dispatch;
	}
	// 82B9C3DC: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C3E0: 41820154  beq 0x82b9c534
	if ctx.cr[0].eq {
	pc = 0x82B9C534; continue 'dispatch;
	}
	// 82B9C3E4: 7D78E050  subf r11, r24, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[24].s64;
	// 82B9C3E8: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82B9C3EC: 7E4BDBD6  divw r18, r11, r27
	ctx.r[18].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	// 82B9C3F0: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 82B9C3F4: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82B9C3F8: 7D2E4B78  mr r14, r9
	ctx.r[14].u64 = ctx.r[9].u64;
	// 82B9C3FC: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 82B9C400: 419A0134  beq cr6, 0x82b9c534
	if ctx.cr[6].eq {
	pc = 0x82B9C534; continue 'dispatch;
	}
	// 82B9C404: 3AB80008  addi r21, r24, 8
	ctx.r[21].s64 = ctx.r[24].s64 + 8;
	// 82B9C408: 7E539378  mr r19, r18
	ctx.r[19].u64 = ctx.r[18].u64;
	// 82B9C40C: 8175FFFC  lwz r11, -4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9C410: 3AE10060  addi r23, r1, 0x60
	ctx.r[23].s64 = ctx.r[1].s64 + 96;
	// 82B9C414: 81550000  lwz r10, 0(r21)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C418: 3A800002  li r20, 2
	ctx.r[20].s64 = 2;
	// 82B9C41C: 8115FFF8  lwz r8, -8(r21)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9C420: 5567843E  srwi r7, r11, 0x10
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9C424: 5546801E  slwi r6, r10, 0x10
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9C428: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9C42C: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82B9C430: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C434: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82B9C438: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 82B9C43C: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82B9C440: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82B9C444: 81570004  lwz r10, 4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C448: 554BA73E  rlwinm r11, r10, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9C44C: 7ECB5830  slw r11, r22, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[22].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C450: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C454: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9C458: 418200A8  beq 0x82b9c500
	if ctx.cr[0].eq {
	pc = 0x82B9C500; continue 'dispatch;
	}
	// 82B9C45C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C460: 5579A77F  rlwinm. r25, r11, 0x14, 0x1d, 0x1f
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82B9C464: 4182009C  beq 0x82b9c500
	if ctx.cr[0].eq {
	pc = 0x82B9C500; continue 'dispatch;
	}
	// 82B9C468: 557D053E  clrlwi r29, r11, 0x14
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9C46C: 554826B6  rlwinm r8, r10, 4, 0x1a, 0x1b
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0FFFFFFFu64;
	// 82B9C470: 1D5D000C  mulli r10, r29, 0xc
	ctx.r[10].s64 = ctx.r[29].s64 * 12;
	// 82B9C474: 5567273E  srwi r7, r11, 0x1c
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9C478: 7FEAC214  add r31, r10, r24
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82B9C47C: 557B853E  rlwinm r27, r11, 0x10, 0x14, 0x1f
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9C480: 7D1A3B78  or r26, r8, r7
	ctx.r[26].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82B9C484: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B9C488: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82B9C48C: 419A0074  beq cr6, 0x82b9c500
	if ctx.cr[6].eq {
	pc = 0x82B9C500; continue 'dispatch;
	}
	// 82B9C490: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82B9C494: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9C498: 41990064  bgt cr6, 0x82b9c4fc
	if ctx.cr[6].gt {
	pc = 0x82B9C4FC; continue 'dispatch;
	}
	// 82B9C49C: 409A0048  bne cr6, 0x82b9c4e4
	if !ctx.cr[6].eq {
	pc = 0x82B9C4E4; continue 'dispatch;
	}
	// 82B9C4A0: 7ECBF030  slw r11, r22, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[22].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C4A4: 7ECAE030  slw r10, r22, r28
	if (ctx.r[28].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[22].u32) << ((ctx.r[28].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C4A8: 7D6BD038  and r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[26].u64;
	// 82B9C4AC: 7D4AD838  and r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[27].u64;
	// 82B9C4B0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9C4B4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82B9C4B8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9C4BC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9C4C0: 7E088378  mr r8, r16
	ctx.r[8].u64 = ctx.r[16].u64;
	// 82B9C4C4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82B9C4C8: 7CDDF214  add r6, r29, r30
	ctx.r[6].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82B9C4CC: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 82B9C4D0: 69440001  xori r4, r10, 1
	ctx.r[4].u64 = ctx.r[10].u64 ^ 1;
	// 82B9C4D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9C4D8: 7E2903A6  mtctr r17
	ctx.ctr.u64 = ctx.r[17].u64;
	// 82B9C4DC: 4E800421  bctrl
	ctx.lr = 0x82B9C4E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9C4E0: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 82B9C4E4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B9C4E8: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 82B9C4EC: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82B9C4F0: 7F1EC840  cmplw cr6, r30, r25
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82B9C4F4: 4198FFA0  blt cr6, 0x82b9c494
	if ctx.cr[6].lt {
	pc = 0x82B9C494; continue 'dispatch;
	}
	// 82B9C4F8: 48000008  b 0x82b9c500
	pc = 0x82B9C500; continue 'dispatch;
	// 82B9C4FC: 7ECFB378  mr r15, r22
	ctx.r[15].u64 = ctx.r[22].u64;
	// 82B9C500: 3694FFFF  addic. r20, r20, -1
	ctx.xer.ca = (ctx.r[20].u32 > (!(-1 as u32)));
	ctx.r[20].s64 = ctx.r[20].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 82B9C504: 3AF70008  addi r23, r23, 8
	ctx.r[23].s64 = ctx.r[23].s64 + 8;
	// 82B9C508: 4082FF3C  bne 0x82b9c444
	if !ctx.cr[0].eq {
	pc = 0x82B9C444; continue 'dispatch;
	}
	// 82B9C50C: 3673FFFF  addic. r19, r19, -1
	ctx.xer.ca = (ctx.r[19].u32 > (!(-1 as u32)));
	ctx.r[19].s64 = ctx.r[19].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 82B9C510: 3AB5000C  addi r21, r21, 0xc
	ctx.r[21].s64 = ctx.r[21].s64 + 12;
	// 82B9C514: 4082FEF8  bne 0x82b9c40c
	if !ctx.cr[0].eq {
	pc = 0x82B9C40C; continue 'dispatch;
	}
	// 82B9C518: 7F0E4840  cmplw cr6, r14, r9
	ctx.cr[6].compare_u32(ctx.r[14].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9C51C: 409A0010  bne cr6, 0x82b9c52c
	if !ctx.cr[6].eq {
	pc = 0x82B9C52C; continue 'dispatch;
	}
	// 82B9C520: 2F0F0000  cmpwi cr6, r15, 0
	ctx.cr[6].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 82B9C524: 419A0010  beq cr6, 0x82b9c534
	if ctx.cr[6].eq {
	pc = 0x82B9C534; continue 'dispatch;
	}
	// 82B9C528: 3929000C  addi r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 + 12;
	// 82B9C52C: 2F0F0000  cmpwi cr6, r15, 0
	ctx.cr[6].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 82B9C530: 409AFEC4  bne cr6, 0x82b9c3f4
	if !ctx.cr[6].eq {
	pc = 0x82B9C3F4; continue 'dispatch;
	}
	// 82B9C534: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82B9C538: 4810CEE8  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9C540 size=144
    let mut pc: u32 = 0x82B9C540;
    'dispatch: loop {
        match pc {
            0x82B9C540 => {
    //   block [0x82B9C540..0x82B9C5D0)
	// 82B9C540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9C544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9C548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9C54C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9C550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9C554: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C558: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9C55C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9C560: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C564: 41820058  beq 0x82b9c5bc
	if ctx.cr[0].eq {
	pc = 0x82B9C5BC; continue 'dispatch;
	}
	// 82B9C568: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9C56C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9C570: 4182004C  beq 0x82b9c5bc
	if ctx.cr[0].eq {
	pc = 0x82B9C5BC; continue 'dispatch;
	}
	// 82B9C574: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82B9C578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9C57C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B9C580: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9C584: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82B9C588: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9C58C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82B9C590: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C594: 554A01BE  clrlwi r10, r10, 6
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 82B9C598: 512BC8CE  rlwimi r11, r9, 0x19, 3, 7
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(25) as u64) & 0x000000001F000000) | (ctx.r[11].u64 & 0xFFFFFFFFE0FFFFFF);
	// 82B9C59C: 654AC800  oris r10, r10, 0xc800
	ctx.r[10].u64 = ctx.r[10].u64 | 3355443200;
	// 82B9C5A0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82B9C5A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82B9C5A8: 4BFFEDA9  bl 0x82b9b350
	ctx.lr = 0x82B9C5AC;
	sub_82B9B350(ctx, base);
	// 82B9C5AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82B9C5B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9C5B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9C5B8: 4BFFEED1  bl 0x82b9b488
	ctx.lr = 0x82B9C5BC;
	sub_82B9B488(ctx, base);
	// 82B9C5BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9C5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9C5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9C5C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9C5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C5D0 size=16
    let mut pc: u32 = 0x82B9C5D0;
    'dispatch: loop {
        match pc {
            0x82B9C5D0 => {
    //   block [0x82B9C5D0..0x82B9C5E0)
	// 82B9C5D0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C5D4: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9C5D8: 2B0B0013  cmplwi cr6, r11, 0x13
	ctx.cr[6].compare_u32(ctx.r[11].u32, 19 as u32, &mut ctx.xer);
	// 82B9C5DC: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C5E0 size=152
    let mut pc: u32 = 0x82B9C5E0;
    'dispatch: loop {
        match pc {
            0x82B9C5E0 => {
    //   block [0x82B9C5E0..0x82B9C678)
	// 82B9C5E0: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C5E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B9C5E8: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9C5EC: 5529077E  clrlwi r9, r9, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9C5F0: 2B090007  cmplwi cr6, r9, 7
	ctx.cr[6].compare_u32(ctx.r[9].u32, 7 as u32, &mut ctx.xer);
	// 82B9C5F4: 419A001C  beq cr6, 0x82b9c610
	if ctx.cr[6].eq {
	pc = 0x82B9C610; continue 'dispatch;
	}
	// 82B9C5F8: 5569E8FA  rlwinm r9, r11, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9C5FC: 556806FE  clrlwi r8, r11, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9C600: 7D484030  slw r8, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[10].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C604: 7CE9182E  lwzx r7, r9, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C608: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82B9C60C: 7D09192E  stwx r8, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 82B9C610: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C614: 552906B8  rlwinm r9, r9, 0, 0x1a, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C618: 2B090038  cmplwi cr6, r9, 0x38
	ctx.cr[6].compare_u32(ctx.r[9].u32, 56 as u32, &mut ctx.xer);
	// 82B9C61C: 419A0020  beq cr6, 0x82b9c63c
	if ctx.cr[6].eq {
	pc = 0x82B9C63C; continue 'dispatch;
	}
	// 82B9C620: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82B9C624: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9C628: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9C62C: 7D494830  slw r9, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C630: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C634: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9C638: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82B9C63C: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C640: 552905F2  rlwinm r9, r9, 0, 0x17, 0x19
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C644: 2B0901C0  cmplwi cr6, r9, 0x1c0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 448 as u32, &mut ctx.xer);
	// 82B9C648: 419A0020  beq cr6, 0x82b9c668
	if ctx.cr[6].eq {
	pc = 0x82B9C668; continue 'dispatch;
	}
	// 82B9C64C: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82B9C650: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9C654: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9C658: 7D494830  slw r9, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C65C: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C660: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9C664: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82B9C668: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C66C: 5529052C  rlwinm r9, r9, 0, 0x14, 0x16
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C670: 2B090E00  cmplwi cr6, r9, 0xe00
	ctx.cr[6].compare_u32(ctx.r[9].u32, 3584 as u32, &mut ctx.xer);
	// 82B9C674: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C678 size=32
    let mut pc: u32 = 0x82B9C678;
    'dispatch: loop {
        match pc {
            0x82B9C678 => {
    //   block [0x82B9C678..0x82B9C698)
	// 82B9C678: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82B9C67C: 5569E8FA  rlwinm r9, r11, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9C680: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9C684: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C688: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C68C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9C690: 7D69192E  stwx r11, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82B9C694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C698 size=44
    let mut pc: u32 = 0x82B9C698;
    'dispatch: loop {
        match pc {
            0x82B9C698 => {
    //   block [0x82B9C698..0x82B9C6C4)
	// 82B9C698: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C69C: 552B06FE  clrlwi r11, r9, 0x1b
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9C6A0: 2B0B0012  cmplwi cr6, r11, 0x12
	ctx.cr[6].compare_u32(ctx.r[11].u32, 18 as u32, &mut ctx.xer);
	// 82B9C6A4: 41990134  bgt cr6, 0x82b9c7d8
	if ctx.cr[6].gt {
		sub_82B9C7D8(ctx, base);
		return;
	}
	// 82B9C6A8: 419A00F8  beq cr6, 0x82b9c7a0
	if ctx.cr[6].eq {
		sub_82B9C7A0(ctx, base);
		return;
	}
	// 82B9C6AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9C6B0: 419A00B0  beq cr6, 0x82b9c760
	if ctx.cr[6].eq {
		sub_82B9C760(ctx, base);
		return;
	}
	// 82B9C6B4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82B9C6B8: 419A0014  beq cr6, 0x82b9c6cc
	if ctx.cr[6].eq {
		sub_82B9C6CC(ctx, base);
		return;
	}
	// 82B9C6BC: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 82B9C6C0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C6C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C6C4 size=8
    let mut pc: u32 = 0x82B9C6C4;
    'dispatch: loop {
        match pc {
            0x82B9C6C4 => {
    //   block [0x82B9C6C4..0x82B9C6CC)
	// 82B9C6C4: 2B0B0011  cmplwi cr6, r11, 0x11
	ctx.cr[6].compare_u32(ctx.r[11].u32, 17 as u32, &mut ctx.xer);
	// 82B9C6C8: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C6CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C6CC size=108
    let mut pc: u32 = 0x82B9C6CC;
    'dispatch: loop {
        match pc {
            0x82B9C6CC => {
    //   block [0x82B9C6CC..0x82B9C738)
	// 82B9C6CC: 552937BE  rlwinm r9, r9, 6, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x03FFFFFFu64;
	// 82B9C6D0: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C6D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9C6D8: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9C6DC: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9C6E0: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9C6E4: 7D694830  slw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C6E8: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C6EC: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9C6F0: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82B9C6F4: 81260008  lwz r9, 8(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9C6F8: 55290422  rlwinm r9, r9, 0, 0x10, 0x11
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C6FC: 2B094000  cmplwi cr6, r9, 0x4000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16384 as u32, &mut ctx.xer);
	// 82B9C700: 41980028  blt cr6, 0x82b9c728
	if ctx.cr[6].lt {
	pc = 0x82B9C728; continue 'dispatch;
	}
	// 82B9C704: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C708: 552927BE  rlwinm r9, r9, 4, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0FFFFFFFu64;
	// 82B9C70C: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9C710: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9C714: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9C718: 7D694830  slw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C71C: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C720: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9C724: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82B9C728: 81260008  lwz r9, 8(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9C72C: 55290422  rlwinm r9, r9, 0, 0x10, 0x11
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C730: 2B098000  cmplwi cr6, r9, 0x8000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32768 as u32, &mut ctx.xer);
	// 82B9C734: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C738 size=40
    let mut pc: u32 = 0x82B9C738;
    'dispatch: loop {
        match pc {
            0x82B9C738 => {
    //   block [0x82B9C738..0x82B9C760)
	// 82B9C738: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C73C: 552917BE  srwi r9, r9, 0x1e
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(30);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9C740: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9C744: 5549E8FA  rlwinm r9, r10, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82B9C748: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9C74C: 7D6B5030  slw r11, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C750: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C754: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9C758: 7D69192E  stwx r11, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82B9C75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C760 size=20
    let mut pc: u32 = 0x82B9C760;
    'dispatch: loop {
        match pc {
            0x82B9C760 => {
    //   block [0x82B9C760..0x82B9C774)
	// 82B9C760: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82B9C764: 419A0010  beq cr6, 0x82b9c774
	if ctx.cr[6].eq {
		sub_82B9C774(ctx, base);
		return;
	}
	// 82B9C768: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C76C: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C770: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C774 size=44
    let mut pc: u32 = 0x82B9C774;
    'dispatch: loop {
        match pc {
            0x82B9C774 => {
    //   block [0x82B9C774..0x82B9C7A0)
	// 82B9C774: 552A17BE  srwi r10, r9, 0x1e
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(30);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C778: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9C77C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9C780: 556AE8FA  rlwinm r10, r11, 0x1d, 3, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9C784: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82B9C788: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9C78C: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C790: 7D2A182E  lwzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C794: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82B9C798: 7D6A192E  stwx r11, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82B9C79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C7A0 size=56
    let mut pc: u32 = 0x82B9C7A0;
    'dispatch: loop {
        match pc {
            0x82B9C7A0 => {
    //   block [0x82B9C7A0..0x82B9C7D8)
	// 82B9C7A0: 552A37BE  rlwinm r10, r9, 6, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x03FFFFFFu64;
	// 82B9C7A4: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9C7A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9C7AC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82B9C7B0: 5549E8FA  rlwinm r9, r10, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82B9C7B4: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9C7B8: 7D6A5030  slw r10, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C7BC: 7CE9182E  lwzx r7, r9, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C7C0: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82B9C7C4: 7D49192E  stwx r10, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 82B9C7C8: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C7CC: 554A27BE  rlwinm r10, r10, 4, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0FFFFFFFu64;
	// 82B9C7D0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82B9C7D4: 4BFFFF70  b 0x82b9c744
	sub_82B9C738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C7D8 size=20
    let mut pc: u32 = 0x82B9C7D8;
    'dispatch: loop {
        match pc {
            0x82B9C7D8 => {
    //   block [0x82B9C7D8..0x82B9C7EC)
	// 82B9C7D8: 2B0B0013  cmplwi cr6, r11, 0x13
	ctx.cr[6].compare_u32(ctx.r[11].u32, 19 as u32, &mut ctx.xer);
	// 82B9C7DC: 419AFEF0  beq cr6, 0x82b9c6cc
	if ctx.cr[6].eq {
		sub_82B9C6CC(ctx, base);
		return;
	}
	// 82B9C7E0: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82B9C7E4: 419A0060  beq cr6, 0x82b9c844
	if ctx.cr[6].eq {
		sub_82B9C844(ctx, base);
		return;
	}
	// 82B9C7E8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C7EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C7EC size=8
    let mut pc: u32 = 0x82B9C7EC;
    'dispatch: loop {
        match pc {
            0x82B9C7EC => {
    //   block [0x82B9C7EC..0x82B9C7F4)
	// 82B9C7EC: 2B0B001A  cmplwi cr6, r11, 0x1a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 26 as u32, &mut ctx.xer);
	// 82B9C7F0: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C7F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C7F4 size=80
    let mut pc: u32 = 0x82B9C7F4;
    'dispatch: loop {
        match pc {
            0x82B9C7F4 => {
    //   block [0x82B9C7F4..0x82B9C844)
	// 82B9C7F4: 552937BE  rlwinm r9, r9, 6, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x03FFFFFFu64;
	// 82B9C7F8: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C7FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9C800: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9C804: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9C808: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9C80C: 7D694830  slw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C810: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C814: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9C818: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82B9C81C: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C820: 552927BE  rlwinm r9, r9, 4, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0FFFFFFFu64;
	// 82B9C824: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9C828: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82B9C82C: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9C830: 7D694830  slw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C834: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82B9C838: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9C83C: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82B9C840: 4BFFFEF8  b 0x82b9c738
	sub_82B9C738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C844(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9C844 size=16
    let mut pc: u32 = 0x82B9C844;
    'dispatch: loop {
        match pc {
            0x82B9C844 => {
    //   block [0x82B9C844..0x82B9C854)
	// 82B9C844: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C848: 552B37BE  rlwinm r11, r9, 6, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x03FFFFFFu64;
	// 82B9C84C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9C850: 4BFFFF30  b 0x82b9c780
	sub_82B9C774(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9C858 size=2064
    let mut pc: u32 = 0x82B9C858;
    'dispatch: loop {
        match pc {
            0x82B9C858 => {
    //   block [0x82B9C858..0x82B9D068)
	// 82B9C858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9C85C: 4810CB81  bl 0x82ca93dc
	ctx.lr = 0x82B9C860;
	sub_82CA93D0(ctx, base);
	// 82B9C860: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9C864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9C868: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B9C86C: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 82B9C870: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82B9C874: 7CF33B78  mr r19, r7
	ctx.r[19].u64 = ctx.r[7].u64;
	// 82B9C878: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C87C: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9C880: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9C884: 556BE7FF  rlwinm. r11, r11, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C888: 418207D8  beq 0x82b9d060
	if ctx.cr[0].eq {
	pc = 0x82B9D060; continue 'dispatch;
	}
	// 82B9C88C: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9C890: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82B9C894: 3A400001  li r18, 1
	ctx.r[18].s64 = 1;
	// 82B9C898: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9C89C: 41820034  beq 0x82b9c8d0
	if ctx.cr[0].eq {
	pc = 0x82B9C8D0; continue 'dispatch;
	}
	// 82B9C8A0: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9C8A4: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9C8A8: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9C8AC: 354AFFF8  addic. r10, r10, -8
	ctx.xer.ca = (ctx.r[10].u32 > (!(-8 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C8B0: 41820020  beq 0x82b9c8d0
	if ctx.cr[0].eq {
	pc = 0x82B9C8D0; continue 'dispatch;
	}
	// 82B9C8B4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C8B8: 554AA73E  rlwinm r10, r10, 0x14, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9C8BC: 7E4A5030  slw r10, r18, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[18].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9C8C0: 714A607E  andi. r10, r10, 0x607e
	ctx.r[10].u64 = ctx.r[10].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C8C4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9C8C8: 7E4A9378  mr r10, r18
	ctx.r[10].u64 = ctx.r[18].u64;
	// 82B9C8CC: 40820008  bne 0x82b9c8d4
	if !ctx.cr[0].eq {
	pc = 0x82B9C8D4; continue 'dispatch;
	}
	// 82B9C8D0: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 82B9C8D4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C8D8: 3B3F006C  addi r25, r31, 0x6c
	ctx.r[25].s64 = ctx.r[31].s64 + 108;
	// 82B9C8DC: 40820020  bne 0x82b9c8fc
	if !ctx.cr[0].eq {
	pc = 0x82B9C8FC; continue 'dispatch;
	}
	// 82B9C8E0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9C8E4: 39401000  li r10, 0x1000
	ctx.r[10].s64 = 4096;
	// 82B9C8E8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82B9C8EC: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9C8F0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82B9C8F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9C8F8: 480000BC  b 0x82b9c9b4
	pc = 0x82B9C9B4; continue 'dispatch;
	// 82B9C8FC: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C900: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9C904: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9C908: 394BFFF8  addi r10, r11, -8
	ctx.r[10].s64 = ctx.r[11].s64 + -8;
	// 82B9C90C: 816BFFF8  lwz r11, -8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9C910: 556B0466  rlwinm r11, r11, 0, 0x11, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C914: 2B0B6000  cmplwi cr6, r11, 0x6000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24576 as u32, &mut ctx.xer);
	// 82B9C918: 40980010  bge cr6, 0x82b9c928
	if !ctx.cr[6].lt {
	pc = 0x82B9C928; continue 'dispatch;
	}
	// 82B9C91C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82B9C920: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C924: 419A009C  beq cr6, 0x82b9c9c0
	if ctx.cr[6].eq {
	pc = 0x82B9C9C0; continue 'dispatch;
	}
	// 82B9C928: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C92C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82B9C930: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C934: 4198072C  blt cr6, 0x82b9d060
	if ctx.cr[6].lt {
	pc = 0x82B9D060; continue 'dispatch;
	}
	// 82B9C938: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C93C: 811F0078  lwz r8, 0x78(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82B9C940: 5569A73E  rlwinm r9, r11, 0x14, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9C944: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B9C948: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82B9C94C: 409A0018  bne cr6, 0x82b9c964
	if !ctx.cr[6].eq {
	pc = 0x82B9C964; continue 'dispatch;
	}
	// 82B9C950: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 82B9C954: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82B9C958: 38E7B530  addi r7, r7, -0x4ad0
	ctx.r[7].s64 = ctx.r[7].s64 + -19152;
	// 82B9C95C: 7D2938AE  lbzx r9, r9, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82B9C960: 48000014  b 0x82b9c974
	pc = 0x82B9C974; continue 'dispatch;
	// 82B9C964: 3D008210  lis r8, -0x7df0
	ctx.r[8].s64 = -2112880640;
	// 82B9C968: 5527063E  clrlwi r7, r9, 0x18
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82B9C96C: 3908B530  addi r8, r8, -0x4ad0
	ctx.r[8].s64 = ctx.r[8].s64 + -19152;
	// 82B9C970: 7D0740AE  lbzx r8, r7, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9C974: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82B9C978: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82B9C97C: 512B6426  rlwimi r11, r9, 0xc, 0x10, 0x13
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0FFF);
	// 82B9C980: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9C984: E96A0000  ld r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82B9C988: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82B9C98C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C990: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9C994: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82B9C998: 510B6426  rlwimi r11, r8, 0xc, 0x10, 0x13
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0FFF);
	// 82B9C99C: 512A0026  rlwimi r10, r9, 0, 0, 0x13
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[10].u64 & 0xFFFFFFFF00000FFF);
	// 82B9C9A0: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C9A4: 714A8FFF  andi. r10, r10, 0x8fff
	ctx.r[10].u64 = ctx.r[10].u64 & 36863;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9C9A8: 556B0566  rlwinm r11, r11, 0, 0x15, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C9AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82B9C9B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9C9B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9C9B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82B9C9BC: 4800226D  bl 0x82b9ec28
	ctx.lr = 0x82B9C9C0;
	sub_82B9EC28(ctx, base);
	// 82B9C9C0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82B9C9C4: 92BF0078  stw r21, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[21].u32 ) };
	// 82B9C9C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9C9CC: 41980694  blt cr6, 0x82b9d060
	if ctx.cr[6].lt {
	pc = 0x82B9D060; continue 'dispatch;
	}
	// 82B9C9D0: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9C9D4: 57D6063F  clrlwi. r22, r30, 0x18
	ctx.r[22].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82B9C9D8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9C9DC: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9C9E0: 813F0070  lwz r9, 0x70(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9C9E4: 7EEA5A14  add r23, r10, r11
	ctx.r[23].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9C9E8: 3B69FFFF  addi r27, r9, -1
	ctx.r[27].s64 = ctx.r[9].s64 + -1;
	// 82B9C9EC: 8157FFF8  lwz r10, -8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9C9F0: 554B0026  rlwinm r11, r10, 0, 0, 0x13
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9C9F4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82B9C9F8: 396B1000  addi r11, r11, 0x1000
	ctx.r[11].s64 = ctx.r[11].s64 + 4096;
	// 82B9C9FC: 5554A77E  rlwinm r20, r10, 0x14, 0x1d, 0x1f
	ctx.r[20].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9CA00: 512B0520  rlwimi r11, r9, 0, 0x14, 0x10
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF8FFF) | (ctx.r[11].u64 & 0x0000000000007000);
	// 82B9CA04: 9177FFF8  stw r11, -8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82B9CA08: 4182001C  beq 0x82b9ca24
	if ctx.cr[0].eq {
	pc = 0x82B9CA24; continue 'dispatch;
	}
	// 82B9CA0C: 568A083C  slwi r10, r20, 1
	ctx.r[10].u32 = ctx.r[20].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9CA10: 7E4A5030  slw r10, r18, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[18].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9CA14: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9CA18: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9CA1C: 516A0406  rlwimi r10, r11, 0, 0x10, 3
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0xFFFFFFFFF000FFFF) | (ctx.r[10].u64 & 0x000000000FFF0000);
	// 82B9CA20: 9157FFF8  stw r10, -8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 82B9CA24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CA28: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9CA2C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9CA30: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CA34: 418205E8  beq 0x82b9d01c
	if ctx.cr[0].eq {
	pc = 0x82B9D01C; continue 'dispatch;
	}
	// 82B9CA38: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82B9CA3C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9CA40: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82B9CA44: 7EB8AB78  mr r24, r21
	ctx.r[24].u64 = ctx.r[21].u64;
	// 82B9CA48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CA4C: FAAB0000  std r21, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82B9CA50: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82B9CA54: FAAB0008  std r21, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[21].u64 ) };
	// 82B9CA58: FAA90008  std r21, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[21].u64 ) };
	// 82B9CA5C: FAAB0010  std r21, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[21].u64 ) };
	// 82B9CA60: FAA90010  std r21, 0x10(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[21].u64 ) };
	// 82B9CA64: FAAB0018  std r21, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[21].u64 ) };
	// 82B9CA68: FAA90018  std r21, 0x18(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[21].u64 ) };
	// 82B9CA6C: 419A0080  beq cr6, 0x82b9caec
	if ctx.cr[6].eq {
	pc = 0x82B9CAEC; continue 'dispatch;
	}
	// 82B9CA70: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82B9CA74: 419A001C  beq cr6, 0x82b9ca90
	if ctx.cr[6].eq {
	pc = 0x82B9CA90; continue 'dispatch;
	}
	// 82B9CA78: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CA7C: 556B06FF  clrlwi. r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CA80: 40820010  bne 0x82b9ca90
	if !ctx.cr[0].eq {
	pc = 0x82B9CA90; continue 'dispatch;
	}
	// 82B9CA84: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9CA88: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CA8C: 40820060  bne 0x82b9caec
	if !ctx.cr[0].eq {
	pc = 0x82B9CAEC; continue 'dispatch;
	}
	// 82B9CA90: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 82B9CA94: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 82B9CA98: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CA9C: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CAA0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CAA4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CAA8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9CAAC: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82B9CAB0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CAB4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CAB8: 4082FFE4  bne 0x82b9ca9c
	if !ctx.cr[0].eq {
	pc = 0x82B9CA9C; continue 'dispatch;
	}
	// 82B9CABC: 397F00C0  addi r11, r31, 0xc0
	ctx.r[11].s64 = ctx.r[31].s64 + 192;
	// 82B9CAC0: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82B9CAC4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CAC8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CACC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CAD0: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CAD4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9CAD8: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82B9CADC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CAE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CAE4: 4082FFE4  bne 0x82b9cac8
	if !ctx.cr[0].eq {
	pc = 0x82B9CAC8; continue 'dispatch;
	}
	// 82B9CAE8: 92BF0004  stw r21, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 82B9CAEC: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82B9CAF0: 419A0374  beq cr6, 0x82b9ce64
	if ctx.cr[6].eq {
	pc = 0x82B9CE64; continue 'dispatch;
	}
	// 82B9CAF4: 83B30000  lwz r29, 0(r19)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CAF8: 57AB035B  rlwinm. r11, r29, 0, 0xd, 0xd
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CAFC: 57BC06FE  clrlwi r28, r29, 0x1b
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x0000001Fu64;
	// 82B9CB00: 41820028  beq 0x82b9cb28
	if ctx.cr[0].eq {
	pc = 0x82B9CB28; continue 'dispatch;
	}
	// 82B9CB04: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 82B9CB08: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 82B9CB0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B9CB10: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82B9CB14: 4BFFFABD  bl 0x82b9c5d0
	ctx.lr = 0x82B9CB18;
	sub_82B9C5D0(ctx, base);
	// 82B9CB18: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B9CB1C: 2B1E0040  cmplwi cr6, r30, 0x40
	ctx.cr[6].compare_u32(ctx.r[30].u32, 64 as u32, &mut ctx.xer);
	// 82B9CB20: 4198FFE8  blt cr6, 0x82b9cb08
	if ctx.cr[6].lt {
	pc = 0x82B9CB08; continue 'dispatch;
	}
	// 82B9CB24: 48000014  b 0x82b9cb38
	pc = 0x82B9CB38; continue 'dispatch;
	// 82B9CB28: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 82B9CB2C: 57A4A6BE  rlwinm r4, r29, 0x14, 0x1a, 0x1f
	ctx.r[4].u64 = ctx.r[29].u32 as u64 & 0x00000FFFu64;
	// 82B9CB30: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82B9CB34: 4BFFFA9D  bl 0x82b9c5d0
	ctx.lr = 0x82B9CB38;
	sub_82B9C5D0(ctx, base);
	// 82B9CB38: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82B9CB3C: 409A0010  bne cr6, 0x82b9cb4c
	if !ctx.cr[6].eq {
	pc = 0x82B9CB4C; continue 'dispatch;
	}
	// 82B9CB40: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9CB44: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CB48: 4082004C  bne 0x82b9cb94
	if !ctx.cr[0].eq {
	pc = 0x82B9CB94; continue 'dispatch;
	}
	// 82B9CB4C: 57AB0529  rlwinm. r11, r29, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CB50: 41820030  beq 0x82b9cb80
	if ctx.cr[0].eq {
	pc = 0x82B9CB80; continue 'dispatch;
	}
	// 82B9CB54: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 82B9CB58: 563D063E  clrlwi r29, r17, 0x18
	ctx.r[29].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	// 82B9CB5C: 7E669B78  mr r6, r19
	ctx.r[6].u64 = ctx.r[19].u64;
	// 82B9CB60: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82B9CB64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B9CB68: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82B9CB6C: 4BFFFB2D  bl 0x82b9c698
	ctx.lr = 0x82B9CB70;
	sub_82B9C698(ctx, base);
	// 82B9CB70: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B9CB74: 2B1E0040  cmplwi cr6, r30, 0x40
	ctx.cr[6].compare_u32(ctx.r[30].u32, 64 as u32, &mut ctx.xer);
	// 82B9CB78: 4198FFE4  blt cr6, 0x82b9cb5c
	if ctx.cr[6].lt {
	pc = 0x82B9CB5C; continue 'dispatch;
	}
	// 82B9CB7C: 48000018  b 0x82b9cb94
	pc = 0x82B9CB94; continue 'dispatch;
	// 82B9CB80: 7E669B78  mr r6, r19
	ctx.r[6].u64 = ctx.r[19].u64;
	// 82B9CB84: 5625063E  clrlwi r5, r17, 0x18
	ctx.r[5].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	// 82B9CB88: 57A4DEBE  rlwinm r4, r29, 0x1b, 0x1a, 0x1f
	ctx.r[4].u64 = ctx.r[29].u32 as u64 & 0x0000001Fu64;
	// 82B9CB8C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82B9CB90: 4BFFFB09  bl 0x82b9c698
	ctx.lr = 0x82B9CB94;
	sub_82B9C698(ctx, base);
	// 82B9CB94: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9CB98: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9CB9C: 418200EC  beq 0x82b9cc88
	if ctx.cr[0].eq {
	pc = 0x82B9CC88; continue 'dispatch;
	}
	// 82B9CBA0: 2B1C0018  cmplwi cr6, r28, 0x18
	ctx.cr[6].compare_u32(ctx.r[28].u32, 24 as u32, &mut ctx.xer);
	// 82B9CBA4: 41980014  blt cr6, 0x82b9cbb8
	if ctx.cr[6].lt {
	pc = 0x82B9CBB8; continue 'dispatch;
	}
	// 82B9CBA8: 2B1C001A  cmplwi cr6, r28, 0x1a
	ctx.cr[6].compare_u32(ctx.r[28].u32, 26 as u32, &mut ctx.xer);
	// 82B9CBAC: 4199000C  bgt cr6, 0x82b9cbb8
	if ctx.cr[6].gt {
	pc = 0x82B9CBB8; continue 'dispatch;
	}
	// 82B9CBB0: 7E589378  mr r24, r18
	ctx.r[24].u64 = ctx.r[18].u64;
	// 82B9CBB4: 480000D4  b 0x82b9cc88
	pc = 0x82B9CC88; continue 'dispatch;
	// 82B9CBB8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82B9CBBC: 409A00CC  bne cr6, 0x82b9cc88
	if !ctx.cr[6].eq {
	pc = 0x82B9CC88; continue 'dispatch;
	}
	// 82B9CBC0: 562B063F  clrlwi. r11, r17, 0x18
	ctx.r[11].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CBC4: 418200C4  beq 0x82b9cc88
	if ctx.cr[0].eq {
	pc = 0x82B9CC88; continue 'dispatch;
	}
	// 82B9CBC8: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CBCC: 408200BC  bne 0x82b9cc88
	if !ctx.cr[0].eq {
	pc = 0x82B9CC88; continue 'dispatch;
	}
	// 82B9CBD0: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9CBD4: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CBD8: 40820028  bne 0x82b9cc00
	if !ctx.cr[0].eq {
	pc = 0x82B9CC00; continue 'dispatch;
	}
	// 82B9CBDC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82B9CBE0: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82B9CBE4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82B9CBE8: 929F000C  stw r20, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 82B9CBEC: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CBF0: 925F0004  stw r18, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[18].u32 ) };
	// 82B9CBF4: 4810C88D  bl 0x82ca9480
	ctx.lr = 0x82B9CBF8;
	sub_82CA9480(ctx, base);
	// 82B9CBF8: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82B9CBFC: 48000080  b 0x82b9cc7c
	pc = 0x82B9CC7C; continue 'dispatch;
	// 82B9CC00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9CC04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CC08: 419A0080  beq cr6, 0x82b9cc88
	if ctx.cr[6].eq {
	pc = 0x82B9CC88; continue 'dispatch;
	}
	// 82B9CC0C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82B9CC10: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82B9CC14: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82B9CC18: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CC1C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CC20: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CC24: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CC28: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9CC2C: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9CC30: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CC34: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CC38: 4082FFE4  bne 0x82b9cc1c
	if !ctx.cr[0].eq {
	pc = 0x82B9CC1C; continue 'dispatch;
	}
	// 82B9CC3C: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82B9CC40: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82B9CC44: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82B9CC48: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CC4C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CC50: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CC54: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CC58: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9CC5C: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82B9CC60: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CC64: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CC68: 4082FFE4  bne 0x82b9cc4c
	if !ctx.cr[0].eq {
	pc = 0x82B9CC4C; continue 'dispatch;
	}
	// 82B9CC6C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82B9CC70: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CC74: 4810C80D  bl 0x82ca9480
	ctx.lr = 0x82B9CC78;
	sub_82CA9480(ctx, base);
	// 82B9CC78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9CC7C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CC80: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82B9CC84: 4810C7FD  bl 0x82ca9480
	ctx.lr = 0x82B9CC88;
	sub_82CA9480(ctx, base);
	// 82B9CC88: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 82B9CC8C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82B9CC90: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CC94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B9CC98: 4810C7E9  bl 0x82ca9480
	ctx.lr = 0x82B9CC9C;
	sub_82CA9480(ctx, base);
	// 82B9CC9C: 3BBF00E0  addi r29, r31, 0xe0
	ctx.r[29].s64 = ctx.r[31].s64 + 224;
	// 82B9CCA0: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82B9CCA4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82B9CCA8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CCAC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CCB0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CCB4: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CCB8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9CCBC: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9CCC0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CCC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CCC8: 4082FFE4  bne 0x82b9ccac
	if !ctx.cr[0].eq {
	pc = 0x82B9CCAC; continue 'dispatch;
	}
	// 82B9CCCC: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82B9CCD0: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82B9CCD4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82B9CCD8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9CCDC: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9CCE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CCE4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CCE8: 2B0B0020  cmplwi cr6, r11, 0x20
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32 as u32, &mut ctx.xer);
	// 82B9CCEC: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 82B9CCF0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B9CCF4: 4198FFDC  blt cr6, 0x82b9ccd0
	if ctx.cr[6].lt {
	pc = 0x82B9CCD0; continue 'dispatch;
	}
	// 82B9CCF8: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82B9CCFC: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82B9CD00: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CD04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82B9CD08: 409A0014  bne cr6, 0x82b9cd1c
	if !ctx.cr[6].eq {
	pc = 0x82B9CD1C; continue 'dispatch;
	}
	// 82B9CD0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9CD10: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9CD14: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82B9CD18: 4198FFE8  blt cr6, 0x82b9cd00
	if ctx.cr[6].lt {
	pc = 0x82B9CD00; continue 'dispatch;
	}
	// 82B9CD1C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9CD20: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82B9CD24: 40980034  bge cr6, 0x82b9cd58
	if !ctx.cr[6].lt {
	pc = 0x82B9CD58; continue 'dispatch;
	}
	// 82B9CD28: 556AE8FA  rlwinm r10, r11, 0x1d, 3, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9CD2C: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82B9CD30: 556806FE  clrlwi r8, r11, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9CD34: 7E484030  slw r8, r18, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[18].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9CD38: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9CD3C: 7D0A5039  and. r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CD40: 40820010  bne 0x82b9cd50
	if !ctx.cr[0].eq {
	pc = 0x82B9CD50; continue 'dispatch;
	}
	// 82B9CD44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9CD48: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82B9CD4C: 4198FFDC  blt cr6, 0x82b9cd28
	if ctx.cr[6].lt {
	pc = 0x82B9CD28; continue 'dispatch;
	}
	// 82B9CD50: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82B9CD54: 4198011C  blt cr6, 0x82b9ce70
	if ctx.cr[6].lt {
	pc = 0x82B9CE70; continue 'dispatch;
	}
	// 82B9CD58: 562B063F  clrlwi. r11, r17, 0x18
	ctx.r[11].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CD5C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82B9CD60: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CD64: 4182003C  beq 0x82b9cda0
	if ctx.cr[0].eq {
	pc = 0x82B9CDA0; continue 'dispatch;
	}
	// 82B9CD68: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 82B9CD6C: 4810C715  bl 0x82ca9480
	ctx.lr = 0x82B9CD70;
	sub_82CA9480(ctx, base);
	// 82B9CD70: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82B9CD74: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82B9CD78: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CD7C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CD80: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CD84: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CD88: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9CD8C: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9CD90: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CD94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CD98: 4082FFE4  bne 0x82b9cd7c
	if !ctx.cr[0].eq {
	pc = 0x82B9CD7C; continue 'dispatch;
	}
	// 82B9CD9C: 48000038  b 0x82b9cdd4
	pc = 0x82B9CDD4; continue 'dispatch;
	// 82B9CDA0: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 82B9CDA4: 4810C6DD  bl 0x82ca9480
	ctx.lr = 0x82B9CDA8;
	sub_82CA9480(ctx, base);
	// 82B9CDA8: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82B9CDAC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82B9CDB0: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CDB4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CDB8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CDBC: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CDC0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82B9CDC4: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82B9CDC8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CDCC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CDD0: 4082FFE4  bne 0x82b9cdb4
	if !ctx.cr[0].eq {
	pc = 0x82B9CDB4; continue 'dispatch;
	}
	// 82B9CDD4: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82B9CDD8: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82B9CDDC: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82B9CDE0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9CDE4: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9CDE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CDEC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CDF0: 2B0B0020  cmplwi cr6, r11, 0x20
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32 as u32, &mut ctx.xer);
	// 82B9CDF4: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 82B9CDF8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B9CDFC: 4198FFDC  blt cr6, 0x82b9cdd8
	if ctx.cr[6].lt {
	pc = 0x82B9CDD8; continue 'dispatch;
	}
	// 82B9CE00: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82B9CE04: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82B9CE08: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CE0C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82B9CE10: 409A0014  bne cr6, 0x82b9ce24
	if !ctx.cr[6].eq {
	pc = 0x82B9CE24; continue 'dispatch;
	}
	// 82B9CE14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9CE18: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82B9CE1C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82B9CE20: 4198FFE8  blt cr6, 0x82b9ce08
	if ctx.cr[6].lt {
	pc = 0x82B9CE08; continue 'dispatch;
	}
	// 82B9CE24: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9CE28: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82B9CE2C: 40980048  bge cr6, 0x82b9ce74
	if !ctx.cr[6].lt {
	pc = 0x82B9CE74; continue 'dispatch;
	}
	// 82B9CE30: 556AE8FA  rlwinm r10, r11, 0x1d, 3, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82B9CE34: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82B9CE38: 556806FE  clrlwi r8, r11, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9CE3C: 7E484030  slw r8, r18, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[18].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9CE40: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82B9CE44: 7D0A5039  and. r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CE48: 40820010  bne 0x82b9ce58
	if !ctx.cr[0].eq {
	pc = 0x82B9CE58; continue 'dispatch;
	}
	// 82B9CE4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9CE50: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82B9CE54: 4198FFDC  blt cr6, 0x82b9ce30
	if ctx.cr[6].lt {
	pc = 0x82B9CE30; continue 'dispatch;
	}
	// 82B9CE58: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82B9CE5C: 40980018  bge cr6, 0x82b9ce74
	if !ctx.cr[6].lt {
	pc = 0x82B9CE74; continue 'dispatch;
	}
	// 82B9CE60: 48000010  b 0x82b9ce70
	pc = 0x82B9CE70; continue 'dispatch;
	// 82B9CE64: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9CE68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9CE6C: 41820008  beq 0x82b9ce74
	if ctx.cr[0].eq {
	pc = 0x82B9CE74; continue 'dispatch;
	}
	// 82B9CE70: 7E589378  mr r24, r18
	ctx.r[24].u64 = ctx.r[18].u64;
	// 82B9CE74: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CE78: 418200C0  beq 0x82b9cf38
	if ctx.cr[0].eq {
	pc = 0x82B9CF38; continue 'dispatch;
	}
	// 82B9CE7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9CE80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82B9CE84: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82B9CE88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CE8C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CE90: 419A0050  beq cr6, 0x82b9cee0
	if ctx.cr[6].eq {
	pc = 0x82B9CEE0; continue 'dispatch;
	}
	// 82B9CE94: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9CE98: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82B9CE9C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9CEA0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CEA4: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9CEA8: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9CEAC: 7D294030  slw r9, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9CEB0: 7D0A582E  lwzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B9CEB4: 5529801E  slwi r9, r9, 0x10
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9CEB8: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82B9CEBC: 51090406  rlwimi r9, r8, 0, 0x10, 3
	ctx.r[9].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFF000FFFF) | (ctx.r[9].u64 & 0x000000000FFF0000);
	// 82B9CEC0: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82B9CEC4: 92BF0004  stw r21, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 82B9CEC8: 4810C5B9  bl 0x82ca9480
	ctx.lr = 0x82B9CECC;
	sub_82CA9480(ctx, base);
	// 82B9CECC: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82B9CED0: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82B9CED4: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CED8: 4810C5A9  bl 0x82ca9480
	ctx.lr = 0x82B9CEDC;
	sub_82CA9480(ctx, base);
	// 82B9CEDC: 48000038  b 0x82b9cf14
	pc = 0x82B9CF14; continue 'dispatch;
	// 82B9CEE0: 568B083C  slwi r11, r20, 1
	ctx.r[11].u32 = ctx.r[20].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9CEE4: 8157FFF8  lwz r10, -8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9CEE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9CEEC: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9CEF0: 556B801E  slwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9CEF4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9CEF8: 514B0406  rlwimi r11, r10, 0, 0x10, 3
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0xFFFFFFFFF000FFFF) | (ctx.r[11].u64 & 0x000000000FFF0000);
	// 82B9CEFC: 9177FFF8  stw r11, -8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82B9CF00: 4810CAB1  bl 0x82ca99b0
	ctx.lr = 0x82B9CF04;
	sub_82CA99B0(ctx, base);
	// 82B9CF04: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CF08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9CF0C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82B9CF10: 4810CAA1  bl 0x82ca99b0
	ctx.lr = 0x82B9CF14;
	sub_82CA99B0(ctx, base);
	// 82B9CF14: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CF18: 9ABF007C  stb r21, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[21].u8 ) };
	// 82B9CF1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9CF20: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82B9CF24: 4810CA8D  bl 0x82ca99b0
	ctx.lr = 0x82B9CF28;
	sub_82CA99B0(ctx, base);
	// 82B9CF28: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9CF2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9CF30: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82B9CF34: 4810CA7D  bl 0x82ca99b0
	ctx.lr = 0x82B9CF38;
	sub_82CA99B0(ctx, base);
	// 82B9CF38: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82B9CF3C: 419A00E0  beq cr6, 0x82b9d01c
	if ctx.cr[6].eq {
	pc = 0x82B9D01C; continue 'dispatch;
	}
	// 82B9CF40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9CF44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CF48: 409A00D4  bne cr6, 0x82b9d01c
	if !ctx.cr[6].eq {
	pc = 0x82B9D01C; continue 'dispatch;
	}
	// 82B9CF4C: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CF50: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9CF54: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82B9CF58: 4198000C  blt cr6, 0x82b9cf64
	if ctx.cr[6].lt {
	pc = 0x82B9CF64; continue 'dispatch;
	}
	// 82B9CF5C: 2B0B001A  cmplwi cr6, r11, 0x1a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 26 as u32, &mut ctx.xer);
	// 82B9CF60: 40990008  ble cr6, 0x82b9cf68
	if !ctx.cr[6].gt {
	pc = 0x82B9CF68; continue 'dispatch;
	}
	// 82B9CF64: 9A5F007C  stb r18, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[18].u8 ) };
	// 82B9CF68: 562B063F  clrlwi. r11, r17, 0x18
	ctx.r[11].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9CF6C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82B9CF70: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CF74: 41820058  beq 0x82b9cfcc
	if ctx.cr[0].eq {
	pc = 0x82B9CFCC; continue 'dispatch;
	}
	// 82B9CF78: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 82B9CF7C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82B9CF80: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CF84: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CF88: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B9CF8C: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9CF90: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CF94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CF98: 4082FFE8  bne 0x82b9cf80
	if !ctx.cr[0].eq {
	pc = 0x82B9CF80; continue 'dispatch;
	}
	// 82B9CF9C: 397F00C0  addi r11, r31, 0xc0
	ctx.r[11].s64 = ctx.r[31].s64 + 192;
	// 82B9CFA0: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82B9CFA4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CFA8: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82B9CFAC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CFB0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CFB4: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B9CFB8: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9CFBC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CFC0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CFC4: 4082FFE8  bne 0x82b9cfac
	if !ctx.cr[0].eq {
	pc = 0x82B9CFAC; continue 'dispatch;
	}
	// 82B9CFC8: 48000054  b 0x82b9d01c
	pc = 0x82B9D01C; continue 'dispatch;
	// 82B9CFCC: 397F00A0  addi r11, r31, 0xa0
	ctx.r[11].s64 = ctx.r[31].s64 + 160;
	// 82B9CFD0: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82B9CFD4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9CFD8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9CFDC: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B9CFE0: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9CFE4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9CFE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9CFEC: 4082FFE8  bne 0x82b9cfd4
	if !ctx.cr[0].eq {
	pc = 0x82B9CFD4; continue 'dispatch;
	}
	// 82B9CFF0: 397F00E0  addi r11, r31, 0xe0
	ctx.r[11].s64 = ctx.r[31].s64 + 224;
	// 82B9CFF4: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82B9CFF8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82B9CFFC: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82B9D000: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D004: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9D008: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B9D00C: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9D010: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9D014: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9D018: 4082FFE8  bne 0x82b9d000
	if !ctx.cr[0].eq {
	pc = 0x82B9D000; continue 'dispatch;
	}
	// 82B9D01C: 562B063F  clrlwi. r11, r17, 0x18
	ctx.r[11].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D020: 41820040  beq 0x82b9d060
	if ctx.cr[0].eq {
	pc = 0x82B9D060; continue 'dispatch;
	}
	// 82B9D024: 2B140004  cmplwi cr6, r20, 4
	ctx.cr[6].compare_u32(ctx.r[20].u32, 4 as u32, &mut ctx.xer);
	// 82B9D028: 40980020  bge cr6, 0x82b9d048
	if !ctx.cr[6].lt {
	pc = 0x82B9D048; continue 'dispatch;
	}
	// 82B9D02C: 7E4BA030  slw r11, r18, r20
	if (ctx.r[20].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[18].u32) << ((ctx.r[20].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D030: 8157FFF8  lwz r10, -8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9D034: 556BE006  slwi r11, r11, 0x1c
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(28);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9D038: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9D03C: 514B013E  rlwimi r11, r10, 0, 4, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x000000000FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFFF0000000);
	// 82B9D040: 9177FFF8  stw r11, -8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82B9D044: 4800001C  b 0x82b9d060
	pc = 0x82B9D060; continue 'dispatch;
	// 82B9D048: 3974FFFC  addi r11, r20, -4
	ctx.r[11].s64 = ctx.r[20].s64 + -4;
	// 82B9D04C: 8157FFFC  lwz r10, -4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82B9D050: 7E4B5830  slw r11, r18, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[18].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D054: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9D058: 514B003A  rlwimi r11, r10, 0, 0, 0x1d
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[11].u64 & 0xFFFFFFFF00000003);
	// 82B9D05C: 9177FFFC  stw r11, -4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82B9D060: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82B9D064: 4810C3C8  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D068 size=284
    let mut pc: u32 = 0x82B9D068;
    'dispatch: loop {
        match pc {
            0x82B9D068 => {
    //   block [0x82B9D068..0x82B9D184)
	// 82B9D068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D06C: 4810C395  bl 0x82ca9400
	ctx.lr = 0x82B9D070;
	sub_82CA93D0(ctx, base);
	// 82B9D070: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D074: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D078: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82B9D07C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B9D080: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D084: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D088: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82B9D08C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B9D090: 936B0088  stw r27, 0x88(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 82B9D094: 816A4DB4  lwz r11, 0x4db4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9D098: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9D09C: 556BE7FF  rlwinm. r11, r11, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D0A0: 418200C4  beq 0x82b9d164
	if ctx.cr[0].eq {
	pc = 0x82B9D164; continue 'dispatch;
	}
	// 82B9D0A4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D0A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B9D0AC: 557CA73E  rlwinm r28, r11, 0x14, 0x1c, 0x1f
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9D0B0: 7D5DE030  slw r29, r10, r28
	if (ctx.r[28].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[10].u32) << ((ctx.r[28].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D0B4: 73AB6FF8  andi. r11, r29, 0x6ff8
	ctx.r[11].u64 = ctx.r[29].u64 & 28664;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D0B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9D0BC: 41820038  beq 0x82b9d0f4
	if ctx.cr[0].eq {
	pc = 0x82B9D0F4; continue 'dispatch;
	}
	// 82B9D0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9D0C4: 835F0064  lwz r26, 0x64(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9D0C8: 4BFFF479  bl 0x82b9c540
	ctx.lr = 0x82B9D0CC;
	sub_82B9C540(ctx, base);
	// 82B9D0CC: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82B9D0D0: 7D5A5851  subf. r10, r26, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9D0D4: 41820020  beq 0x82b9d0f4
	if ctx.cr[0].eq {
	pc = 0x82B9D0F4; continue 'dispatch;
	}
	// 82B9D0D8: 73AB607E  andi. r11, r29, 0x607e
	ctx.r[11].u64 = ctx.r[29].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D0DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9D0E0: 41820014  beq 0x82b9d0f4
	if ctx.cr[0].eq {
	pc = 0x82B9D0F4; continue 'dispatch;
	}
	// 82B9D0E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D0E8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9D0EC: 516A0026  rlwimi r10, r11, 0, 0, 0x13
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[10].u64 & 0xFFFFFFFF00000FFF);
	// 82B9D0F0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B9D0F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D0F8: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9D0FC: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9D100: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D104: 41820060  beq 0x82b9d164
	if ctx.cr[0].eq {
	pc = 0x82B9D164; continue 'dispatch;
	}
	// 82B9D108: 2B1C000C  cmplwi cr6, r28, 0xc
	ctx.cr[6].compare_u32(ctx.r[28].u32, 12 as u32, &mut ctx.xer);
	// 82B9D10C: 409A0058  bne cr6, 0x82b9d164
	if !ctx.cr[6].eq {
	pc = 0x82B9D164; continue 'dispatch;
	}
	// 82B9D110: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D114: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D118: 4082004C  bne 0x82b9d164
	if !ctx.cr[0].eq {
	pc = 0x82B9D164; continue 'dispatch;
	}
	// 82B9D11C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9D120: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82B9D124: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9D128: 9B7F007C  stb r27, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[27].u8 ) };
	// 82B9D12C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82B9D130: 4810C881  bl 0x82ca99b0
	ctx.lr = 0x82B9D134;
	sub_82CA99B0(ctx, base);
	// 82B9D134: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9D138: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9D13C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82B9D140: 4810C871  bl 0x82ca99b0
	ctx.lr = 0x82B9D144;
	sub_82CA99B0(ctx, base);
	// 82B9D144: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9D148: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9D14C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82B9D150: 4810C861  bl 0x82ca99b0
	ctx.lr = 0x82B9D154;
	sub_82CA99B0(ctx, base);
	// 82B9D154: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9D158: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9D15C: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82B9D160: 4810C851  bl 0x82ca99b0
	ctx.lr = 0x82B9D164;
	sub_82CA99B0(ctx, base);
	// 82B9D164: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82B9D168: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B9D16C: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 82B9D170: 48001AB9  bl 0x82b9ec28
	ctx.lr = 0x82B9D174;
	sub_82B9EC28(ctx, base);
	// 82B9D174: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9D178: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82B9D17C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B9D180: 4810C2D0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D188 size=472
    let mut pc: u32 = 0x82B9D188;
    'dispatch: loop {
        match pc {
            0x82B9D188 => {
    //   block [0x82B9D188..0x82B9D360)
	// 82B9D188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D18C: 4810C26D  bl 0x82ca93f8
	ctx.lr = 0x82B9D190;
	sub_82CA93D0(ctx, base);
	// 82B9D190: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D194: 3BA30170  addi r29, r3, 0x170
	ctx.r[29].s64 = ctx.r[3].s64 + 368;
	// 82B9D198: 3B632840  addi r27, r3, 0x2840
	ctx.r[27].s64 = ctx.r[3].s64 + 10304;
	// 82B9D19C: 3B200040  li r25, 0x40
	ctx.r[25].s64 = 64;
	// 82B9D1A0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82B9D1A4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82B9D1A8: 239A003F  subfic r28, r26, 0x3f
	ctx.xer.ca = ctx.r[26].u32 <= 63 as u32;
	ctx.r[28].s64 = (63 as i64) - ctx.r[26].s64;
	// 82B9D1AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B9D1B0: 5785103A  slwi r5, r28, 2
	ctx.r[5].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9D1B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9D1B8: 39250030  addi r9, r5, 0x30
	ctx.r[9].s64 = ctx.r[5].s64 + 48;
	// 82B9D1BC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82B9D1C0: 55272036  slwi r7, r9, 4
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9D1C4: 7D27EA14  add r9, r7, r29
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 82B9D1C8: 7D0A2A14  add r8, r10, r5
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82B9D1CC: 5506D97E  srwi r6, r8, 5
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shr(5);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9D1D0: 550806FE  clrlwi r8, r8, 0x1b
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82B9D1D4: 38C608EE  addi r6, r6, 0x8ee
	ctx.r[6].s64 = ctx.r[6].s64 + 2286;
	// 82B9D1D8: 7FE84030  slw r8, r31, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[31].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D1DC: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9D1E0: 7CC6E82E  lwzx r6, r6, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82B9D1E4: 7D083038  and r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[6].u64;
	// 82B9D1E8: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82B9D1EC: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82B9D1F0: 69080001  xori r8, r8, 1
	ctx.r[8].u64 = ctx.r[8].u64 ^ 1;
	// 82B9D1F4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B9D1F8: 419A002C  beq cr6, 0x82b9d224
	if ctx.cr[6].eq {
	pc = 0x82B9D224; continue 'dispatch;
	}
	// 82B9D1FC: 7FE65030  slw r6, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D200: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D204: 7CDEF378  or r30, r6, r30
	ctx.r[30].u64 = ctx.r[6].u64 | ctx.r[30].u64;
	// 82B9D208: 80890004  lwz r4, 4(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D20C: 80C90008  lwz r6, 8(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9D210: 8309000C  lwz r24, 0xc(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9D214: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9D218: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82B9D21C: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82B9D220: 930B000C  stw r24, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[24].u32 ) };
	// 82B9D224: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9D228: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82B9D22C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82B9D230: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82B9D234: 4198FF94  blt cr6, 0x82b9d1c8
	if ctx.cr[6].lt {
	pc = 0x82B9D1C8; continue 'dispatch;
	}
	// 82B9D238: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9D23C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9D240: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82B9D244: 7D27DA14  add r9, r7, r27
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[27].u64;
	// 82B9D248: 7D0A2A14  add r8, r10, r5
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82B9D24C: 5507D97E  srwi r7, r8, 5
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shr(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9D250: 550806FE  clrlwi r8, r8, 0x1b
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82B9D254: 38E708EE  addi r7, r7, 0x8ee
	ctx.r[7].s64 = ctx.r[7].s64 + 2286;
	// 82B9D258: 7FE84030  slw r8, r31, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[31].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D25C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9D260: 7CE7D82E  lwzx r7, r7, r27
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82B9D264: 7D083838  and r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[7].u64;
	// 82B9D268: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82B9D26C: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82B9D270: 69080001  xori r8, r8, 1
	ctx.r[8].u64 = ctx.r[8].u64 ^ 1;
	// 82B9D274: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B9D278: 419A002C  beq cr6, 0x82b9d2a4
	if ctx.cr[6].eq {
	pc = 0x82B9D2A4; continue 'dispatch;
	}
	// 82B9D27C: 7FE75030  slw r7, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D280: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D284: 7CE42378  or r4, r7, r4
	ctx.r[4].u64 = ctx.r[7].u64 | ctx.r[4].u64;
	// 82B9D288: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D28C: 80E90008  lwz r7, 8(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9D290: 8309000C  lwz r24, 0xc(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9D294: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9D298: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82B9D29C: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82B9D2A0: 930B000C  stw r24, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[24].u32 ) };
	// 82B9D2A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9D2A8: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82B9D2AC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82B9D2B0: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82B9D2B4: 4198FF94  blt cr6, 0x82b9d248
	if ctx.cr[6].lt {
	pc = 0x82B9D248; continue 'dispatch;
	}
	// 82B9D2B8: 7C8BF379  or. r11, r4, r30
	ctx.r[11].u64 = ctx.r[4].u64 | ctx.r[30].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D2BC: 41820078  beq 0x82b9d334
	if ctx.cr[0].eq {
	pc = 0x82B9D334; continue 'dispatch;
	}
	// 82B9D2C0: 7C85F039  and. r5, r4, r30
	ctx.r[5].u64 = ctx.r[4].u64 & ctx.r[30].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82B9D2C4: 41820064  beq 0x82b9d328
	if ctx.cr[0].eq {
	pc = 0x82B9D328; continue 'dispatch;
	}
	// 82B9D2C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9D2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82B9D2D0: 7FEB3030  slw r11, r31, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D2D4: 7D6B2839  and. r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[5].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D2D8: 41820040  beq 0x82b9d318
	if ctx.cr[0].eq {
	pc = 0x82B9D318; continue 'dispatch;
	}
	// 82B9D2DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82B9D2E0: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82B9D2E4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82B9D2E8: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9D2EC: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82B9D2F0: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D2F4: 8B0A0000  lbz r24, 0(r10)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D2F8: 7CF83851  subf. r7, r24, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[24].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82B9D2FC: 40820014  bne 0x82b9d310
	if !ctx.cr[0].eq {
	pc = 0x82B9D310; continue 'dispatch;
	}
	// 82B9D300: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9D304: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9D308: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82B9D30C: 409AFFE4  bne cr6, 0x82b9d2f0
	if !ctx.cr[6].eq {
	pc = 0x82B9D2F0; continue 'dispatch;
	}
	// 82B9D310: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9D314: 40820014  bne 0x82b9d328
	if !ctx.cr[0].eq {
	pc = 0x82B9D328; continue 'dispatch;
	}
	// 82B9D318: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82B9D31C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82B9D320: 2B090040  cmplwi cr6, r9, 0x40
	ctx.cr[6].compare_u32(ctx.r[9].u32, 64 as u32, &mut ctx.xer);
	// 82B9D324: 4198FFAC  blt cr6, 0x82b9d2d0
	if ctx.cr[6].lt {
	pc = 0x82B9D2D0; continue 'dispatch;
	}
	// 82B9D328: 7C8BF079  andc. r11, r4, r30
	ctx.r[11].u64 = ctx.r[4].u64 & !ctx.r[30].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D32C: 40820014  bne 0x82b9d340
	if !ctx.cr[0].eq {
	pc = 0x82B9D340; continue 'dispatch;
	}
	// 82B9D330: 7F99E378  mr r25, r28
	ctx.r[25].u64 = ctx.r[28].u64;
	// 82B9D334: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82B9D338: 2B1A0040  cmplwi cr6, r26, 0x40
	ctx.cr[6].compare_u32(ctx.r[26].u32, 64 as u32, &mut ctx.xer);
	// 82B9D33C: 4198FE6C  blt cr6, 0x82b9d1a8
	if ctx.cr[6].lt {
	pc = 0x82B9D1A8; continue 'dispatch;
	}
	// 82B9D340: 93234DE0  stw r25, 0x4de0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(19936 as u32), ctx.r[25].u32 ) };
	// 82B9D344: 38A34DBC  addi r5, r3, 0x4dbc
	ctx.r[5].s64 = ctx.r[3].s64 + 19900;
	// 82B9D348: 93E34DDC  stw r31, 0x4ddc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(19932 as u32), ctx.r[31].u32 ) };
	// 82B9D34C: 20990040  subfic r4, r25, 0x40
	ctx.xer.ca = ctx.r[25].u32 <= 64 as u32;
	ctx.r[4].s64 = (64 as i64) - ctx.r[25].s64;
	// 82B9D350: 38634DE4  addi r3, r3, 0x4de4
	ctx.r[3].s64 = ctx.r[3].s64 + 19940;
	// 82B9D354: 48001A6D  bl 0x82b9edc0
	ctx.lr = 0x82B9D358;
	sub_82B9EDC0(ctx, base);
	// 82B9D358: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82B9D35C: 4810C0EC  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D360 size=160
    let mut pc: u32 = 0x82B9D360;
    'dispatch: loop {
        match pc {
            0x82B9D360 => {
    //   block [0x82B9D360..0x82B9D400)
	// 82B9D360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9D368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9D36C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9D374: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82B9D378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D37C: 419A000C  beq cr6, 0x82b9d388
	if ctx.cr[6].eq {
	pc = 0x82B9D388; continue 'dispatch;
	}
	// 82B9D380: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D384: 4B664F35  bl 0x822022b8
	ctx.lr = 0x82B9D388;
	sub_822022B8(ctx, base);
	// 82B9D388: 807F0124  lwz r3, 0x124(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 82B9D38C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D390: 419A000C  beq cr6, 0x82b9d39c
	if ctx.cr[6].eq {
	pc = 0x82B9D39C; continue 'dispatch;
	}
	// 82B9D394: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D398: 4B664F21  bl 0x822022b8
	ctx.lr = 0x82B9D39C;
	sub_822022B8(ctx, base);
	// 82B9D39C: 807F0118  lwz r3, 0x118(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 82B9D3A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D3A4: 419A000C  beq cr6, 0x82b9d3b0
	if ctx.cr[6].eq {
	pc = 0x82B9D3B0; continue 'dispatch;
	}
	// 82B9D3A8: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D3AC: 4B664F0D  bl 0x822022b8
	ctx.lr = 0x82B9D3B0;
	sub_822022B8(ctx, base);
	// 82B9D3B0: 807F010C  lwz r3, 0x10c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 82B9D3B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D3B8: 419A000C  beq cr6, 0x82b9d3c4
	if ctx.cr[6].eq {
	pc = 0x82B9D3C4; continue 'dispatch;
	}
	// 82B9D3BC: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D3C0: 4B664EF9  bl 0x822022b8
	ctx.lr = 0x82B9D3C4;
	sub_822022B8(ctx, base);
	// 82B9D3C4: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9D3C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D3CC: 419A000C  beq cr6, 0x82b9d3d8
	if ctx.cr[6].eq {
	pc = 0x82B9D3D8; continue 'dispatch;
	}
	// 82B9D3D0: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D3D4: 4B664EE5  bl 0x822022b8
	ctx.lr = 0x82B9D3D8;
	sub_822022B8(ctx, base);
	// 82B9D3D8: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9D3DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D3E0: 419A000C  beq cr6, 0x82b9d3ec
	if ctx.cr[6].eq {
	pc = 0x82B9D3EC; continue 'dispatch;
	}
	// 82B9D3E4: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D3E8: 4B664ED1  bl 0x822022b8
	ctx.lr = 0x82B9D3EC;
	sub_822022B8(ctx, base);
	// 82B9D3EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B9D3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9D3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9D3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9D3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D400 size=180
    let mut pc: u32 = 0x82B9D400;
    'dispatch: loop {
        match pc {
            0x82B9D400 => {
    //   block [0x82B9D400..0x82B9D4B4)
	// 82B9D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D404: 4810C005  bl 0x82ca9408
	ctx.lr = 0x82B9D408;
	sub_82CA93D0(ctx, base);
	// 82B9D408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D40C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9D410: 817F4DB0  lwz r11, 0x4db0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19888 as u32) ) } as u64;
	// 82B9D414: 378BFFFF  addic. r28, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82B9D418: 939F4DB0  stw r28, 0x4db0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19888 as u32), ctx.r[28].u32 ) };
	// 82B9D41C: 4082008C  bne 0x82b9d4a8
	if !ctx.cr[0].eq {
	pc = 0x82B9D4A8; continue 'dispatch;
	}
	// 82B9D420: 807F4DB8  lwz r3, 0x4db8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9D424: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D428: 419A001C  beq cr6, 0x82b9d444
	if ctx.cr[6].eq {
	pc = 0x82B9D444; continue 'dispatch;
	}
	// 82B9D42C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D430: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9D438: 4E800421  bctrl
	ctx.lr = 0x82B9D43C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9D43C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9D440: 917F4DB8  stw r11, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[11].u32 ) };
	// 82B9D444: 807F4DE4  lwz r3, 0x4de4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19940 as u32) ) } as u64;
	// 82B9D448: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D44C: 419A000C  beq cr6, 0x82b9d458
	if ctx.cr[6].eq {
	pc = 0x82B9D458; continue 'dispatch;
	}
	// 82B9D450: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D454: 4B664E65  bl 0x822022b8
	ctx.lr = 0x82B9D458;
	sub_822022B8(ctx, base);
	// 82B9D458: 807F4DD0  lwz r3, 0x4dd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19920 as u32) ) } as u64;
	// 82B9D45C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D460: 419A000C  beq cr6, 0x82b9d46c
	if ctx.cr[6].eq {
	pc = 0x82B9D46C; continue 'dispatch;
	}
	// 82B9D464: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D468: 4B664E51  bl 0x822022b8
	ctx.lr = 0x82B9D46C;
	sub_822022B8(ctx, base);
	// 82B9D46C: 807F4DC4  lwz r3, 0x4dc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19908 as u32) ) } as u64;
	// 82B9D470: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D474: 419A000C  beq cr6, 0x82b9d480
	if ctx.cr[6].eq {
	pc = 0x82B9D480; continue 'dispatch;
	}
	// 82B9D478: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D47C: 4B664E3D  bl 0x822022b8
	ctx.lr = 0x82B9D480;
	sub_822022B8(ctx, base);
	// 82B9D480: 3BDF4DB0  addi r30, r31, 0x4db0
	ctx.r[30].s64 = ctx.r[31].s64 + 19888;
	// 82B9D484: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82B9D488: 3BDED930  addi r30, r30, -0x26d0
	ctx.r[30].s64 = ctx.r[30].s64 + -9936;
	// 82B9D48C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9D490: 4BFFFED1  bl 0x82b9d360
	ctx.lr = 0x82B9D494;
	sub_82B9D360(ctx, base);
	// 82B9D494: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9D498: 4080FFF0  bge 0x82b9d488
	if !ctx.cr[0].lt {
	pc = 0x82B9D488; continue 'dispatch;
	}
	// 82B9D49C: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9D4A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9D4A4: 4B664E15  bl 0x822022b8
	ctx.lr = 0x82B9D4A8;
	sub_822022B8(ctx, base);
	// 82B9D4A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82B9D4AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9D4B0: 4810BFA8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D4B8 size=4
    let mut pc: u32 = 0x82B9D4B8;
    'dispatch: loop {
        match pc {
            0x82B9D4B8 => {
    //   block [0x82B9D4B8..0x82B9D4BC)
	// 82B9D4B8: 48000A20  b 0x82b9ded8
	sub_82B9DED8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D4C0 size=32
    let mut pc: u32 = 0x82B9D4C0;
    'dispatch: loop {
        match pc {
            0x82B9D4C0 => {
    //   block [0x82B9D4C0..0x82B9D4E0)
	// 82B9D4C0: 81634DB4  lwz r11, 0x4db4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9D4C4: 708A00F9  andi. r10, r4, 0xf9
	ctx.r[10].u64 = ctx.r[4].u64 & 249;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9D4C8: 556B076E  rlwinm r11, r11, 0, 0x1d, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9D4CC: 548907FE  clrlwi r9, r4, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82B9D4D0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9D4D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B9D4D8: 91634DB4  stw r11, 0x4db4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(19892 as u32), ctx.r[11].u32 ) };
	// 82B9D4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D4E0 size=20
    let mut pc: u32 = 0x82B9D4E0;
    'dispatch: loop {
        match pc {
            0x82B9D4E0 => {
    //   block [0x82B9D4E0..0x82B9D4F4)
	// 82B9D4E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D4E4: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D4E8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D4EC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82B9D4F0: 4BFFCC78  b 0x82b9a168
	sub_82B9A168(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D4F8 size=28
    let mut pc: u32 = 0x82B9D4F8;
    'dispatch: loop {
        match pc {
            0x82B9D4F8 => {
    //   block [0x82B9D4F8..0x82B9D514)
	// 82B9D4F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D4FC: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D500: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D504: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82B9D508: 908B0068  stw r4, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82B9D50C: 90AB006C  stw r5, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[5].u32 ) };
	// 82B9D510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D518 size=40
    let mut pc: u32 = 0x82B9D518;
    'dispatch: loop {
        match pc {
            0x82B9D518 => {
    //   block [0x82B9D518..0x82B9D540)
	// 82B9D518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D51C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D520: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D524: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D528: 914B0164  stw r10, 0x164(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 82B9D52C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D530: 914B0168  stw r10, 0x168(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(360 as u32), ctx.r[10].u32 ) };
	// 82B9D534: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9D538: 914B016C  stw r10, 0x16c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(364 as u32), ctx.r[10].u32 ) };
	// 82B9D53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D540 size=120
    let mut pc: u32 = 0x82B9D540;
    'dispatch: loop {
        match pc {
            0x82B9D540 => {
    //   block [0x82B9D540..0x82B9D5B8)
	// 82B9D540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9D548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B9D54C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9D550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9D558: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B9D55C: 807F4DB8  lwz r3, 0x4db8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9D560: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9D564: 419A001C  beq cr6, 0x82b9d580
	if ctx.cr[6].eq {
	pc = 0x82B9D580; continue 'dispatch;
	}
	// 82B9D568: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D56C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9D574: 4E800421  bctrl
	ctx.lr = 0x82B9D578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9D578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9D57C: 917F4DB8  stw r11, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[11].u32 ) };
	// 82B9D580: 93DF4DB8  stw r30, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[30].u32 ) };
	// 82B9D584: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82B9D588: 419A0018  beq cr6, 0x82b9d5a0
	if ctx.cr[6].eq {
	pc = 0x82B9D5A0; continue 'dispatch;
	}
	// 82B9D58C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9D594: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9D59C: 4E800421  bctrl
	ctx.lr = 0x82B9D5A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9D5A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9D5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9D5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9D5AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9D5B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9D5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D5B8 size=124
    let mut pc: u32 = 0x82B9D5B8;
    'dispatch: loop {
        match pc {
            0x82B9D5B8 => {
    //   block [0x82B9D5B8..0x82B9D634)
	// 82B9D5B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D5BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9D5C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82B9D5C4: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D5C8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D5CC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82B9D5D0: 396B0170  addi r11, r11, 0x170
	ctx.r[11].s64 = ctx.r[11].s64 + 368;
	// 82B9D5D4: 914B255C  stw r10, 0x255c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(9564 as u32), ctx.r[10].u32 ) };
	// 82B9D5D8: 419A0020  beq cr6, 0x82b9d5f8
	if ctx.cr[6].eq {
	pc = 0x82B9D5F8; continue 'dispatch;
	}
	// 82B9D5DC: 394B2518  addi r10, r11, 0x2518
	ctx.r[10].s64 = ctx.r[11].s64 + 9496;
	// 82B9D5E0: 5489E8FA  rlwinm r9, r4, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 82B9D5E4: 548706FE  clrlwi r7, r4, 0x1b
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82B9D5E8: 7D073830  slw r7, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[8].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D5EC: 7CC9502E  lwzx r6, r9, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82B9D5F0: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82B9D5F4: 7CE9512E  stwx r7, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u32) };
	// 82B9D5F8: 394B2538  addi r10, r11, 0x2538
	ctx.r[10].s64 = ctx.r[11].s64 + 9528;
	// 82B9D5FC: 5489E8FA  rlwinm r9, r4, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 82B9D600: 548706FE  clrlwi r7, r4, 0x1b
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82B9D604: 5486D97E  srwi r6, r4, 5
	ctx.r[6].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9D608: 7D073830  slw r7, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[8].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D60C: 7CA9502E  lwzx r5, r9, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82B9D610: 20C6001F  subfic r6, r6, 0x1f
	ctx.xer.ca = ctx.r[6].u32 <= 31 as u32;
	ctx.r[6].s64 = (31 as i64) - ctx.r[6].s64;
	// 82B9D614: 7CE72B78  or r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 82B9D618: 78C60020  clrldi r6, r6, 0x20
	ctx.r[6].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	// 82B9D61C: 7CE9512E  stwx r7, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u32) };
	// 82B9D620: 7D0A3036  sld r10, r8, r6
	if (ctx.r[6].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[8].u64) << ((ctx.r[6].u8 & 0x3F) as u32);
	}
	// 82B9D624: E92B23A8  ld r9, 0x23a8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(9128 as u32) ) };
	// 82B9D628: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82B9D62C: F94B23A8  std r10, 0x23a8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(9128 as u32), ctx.r[10].u64 ) };
	// 82B9D630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D638 size=124
    let mut pc: u32 = 0x82B9D638;
    'dispatch: loop {
        match pc {
            0x82B9D638 => {
    //   block [0x82B9D638..0x82B9D6B4)
	// 82B9D638: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D63C: 2144003F  subfic r10, r4, 0x3f
	ctx.xer.ca = ctx.r[4].u32 <= 63 as u32;
	ctx.r[10].s64 = (63 as i64) - ctx.r[4].s64;
	// 82B9D640: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82B9D644: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D648: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D64C: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82B9D650: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82B9D654: 7D075036  sld r7, r8, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[8].u64) << ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82B9D658: E8CB2518  ld r6, 0x2518(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(9496 as u32) ) };
	// 82B9D65C: 388408C8  addi r4, r4, 0x8c8
	ctx.r[4].s64 = ctx.r[4].s64 + 2248;
	// 82B9D660: 912B26CC  stw r9, 0x26cc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(9932 as u32), ctx.r[9].u32 ) };
	// 82B9D664: 394B0170  addi r10, r11, 0x170
	ctx.r[10].s64 = ctx.r[11].s64 + 368;
	// 82B9D668: 7CE93378  or r9, r7, r6
	ctx.r[9].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82B9D66C: 5487103A  slwi r7, r4, 2
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9D670: F92B2518  std r9, 0x2518(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(9496 as u32), ctx.r[9].u64 ) };
	// 82B9D674: 396A23F8  addi r11, r10, 0x23f8
	ctx.r[11].s64 = ctx.r[10].s64 + 9208;
	// 82B9D678: 54E9D97A  rlwinm r9, r7, 0x1b, 5, 0x1d
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82B9D67C: 54E6F6FE  rlwinm r6, r7, 0x1e, 0x1b, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 82B9D680: 7D083030  slw r8, r8, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D684: 7CC9582E  lwzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82B9D688: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 82B9D68C: 7D09592E  stwx r8, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82B9D690: 89650007  lbz r11, 7(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(7 as u32) ) } as u64;
	// 82B9D694: 89250003  lbz r9, 3(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(3 as u32) ) } as u64;
	// 82B9D698: 8905000B  lbz r8, 0xb(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(11 as u32) ) } as u64;
	// 82B9D69C: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 82B9D6A0: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82B9D6A4: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9D6A8: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82B9D6AC: 7D67512E  stwx r11, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82B9D6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D6B8 size=148
    let mut pc: u32 = 0x82B9D6B8;
    'dispatch: loop {
        match pc {
            0x82B9D6B8 => {
    //   block [0x82B9D6B8..0x82B9D74C)
	// 82B9D6B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D6BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82B9D6C0: 5489F0BE  srwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9D6C4: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D6C8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D6CC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82B9D6D0: 914B26CC  stw r10, 0x26cc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(9932 as u32), ctx.r[10].u32 ) };
	// 82B9D6D4: 2149003F  subfic r10, r9, 0x3f
	ctx.xer.ca = ctx.r[9].u32 <= 63 as u32;
	ctx.r[10].s64 = (63 as i64) - ctx.r[9].s64;
	// 82B9D6D8: E90B2510  ld r8, 0x2510(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(9488 as u32) ) };
	// 82B9D6DC: 392B0170  addi r9, r11, 0x170
	ctx.r[9].s64 = ctx.r[11].s64 + 368;
	// 82B9D6E0: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82B9D6E4: 80EB26D4  lwz r7, 0x26d4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9940 as u32) ) } as u64;
	// 82B9D6E8: 7CCA5036  sld r10, r6, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[6].u64) << ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82B9D6EC: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82B9D6F0: 390923B8  addi r8, r9, 0x23b8
	ctx.r[8].s64 = ctx.r[9].s64 + 9144;
	// 82B9D6F4: F94B2510  std r10, 0x2510(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(9488 as u32), ctx.r[10].u64 ) };
	// 82B9D6F8: 81674DB4  lwz r11, 0x4db4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9D6FC: 556B35EE  rlwinm r11, r11, 6, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82B9D700: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82B9D704: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D708: 396A0030  addi r11, r10, 0x30
	ctx.r[11].s64 = ctx.r[10].s64 + 48;
	// 82B9D70C: 5547E8FA  rlwinm r7, r10, 0x1d, 3, 0x1d
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82B9D710: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9D714: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9D718: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9D71C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82B9D720: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9D724: 7CCA5030  slw r10, r6, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[6].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9D728: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82B9D72C: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9D730: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82B9D734: 8125000C  lwz r9, 0xc(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9D738: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82B9D73C: 7D67402E  lwzx r11, r7, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82B9D740: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82B9D744: 7D67412E  stwx r11, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	// 82B9D748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D750 size=84
    let mut pc: u32 = 0x82B9D750;
    'dispatch: loop {
        match pc {
            0x82B9D750 => {
    //   block [0x82B9D750..0x82B9D7A4)
	// 82B9D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D754: 4810BCB5  bl 0x82ca9408
	ctx.lr = 0x82B9D758;
	sub_82CA93D0(ctx, base);
	// 82B9D758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D75C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9D760: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82B9D764: 3BBF4DC4  addi r29, r31, 0x4dc4
	ctx.r[29].s64 = ctx.r[31].s64 + 19908;
	// 82B9D768: 397E0003  addi r11, r30, 3
	ctx.r[11].s64 = ctx.r[30].s64 + 3;
	// 82B9D76C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82B9D770: 38BF4DBC  addi r5, r31, 0x4dbc
	ctx.r[5].s64 = ctx.r[31].s64 + 19900;
	// 82B9D774: 5564F0BE  srwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82B9D778: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9D77C: 48001645  bl 0x82b9edc0
	ctx.lr = 0x82B9D780;
	sub_82B9EDC0(ctx, base);
	// 82B9D780: 817F4DBC  lwz r11, 0x4dbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19900 as u32) ) } as u64;
	// 82B9D784: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9D788: 41980014  blt cr6, 0x82b9d79c
	if ctx.cr[6].lt {
	pc = 0x82B9D79C; continue 'dispatch;
	}
	// 82B9D78C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82B9D790: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D794: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82B9D798: 4810BCE9  bl 0x82ca9480
	ctx.lr = 0x82B9D79C;
	sub_82CA9480(ctx, base);
	// 82B9D79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9D7A0: 4810BCB8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D7A8 size=152
    let mut pc: u32 = 0x82B9D7A8;
    'dispatch: loop {
        match pc {
            0x82B9D7A8 => {
    //   block [0x82B9D7A8..0x82B9D840)
	// 82B9D7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9D7B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D7B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D7B8: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D7BC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D7C0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82B9D7C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D7C8: 814A4DB4  lwz r10, 0x4db4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9D7CC: 554AF7FF  rlwinm. r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9D7D0: 41820040  beq 0x82b9d810
	if ctx.cr[0].eq {
	pc = 0x82B9D810; continue 'dispatch;
	}
	// 82B9D7D4: 54C9881C  slwi r9, r6, 0x11
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9D7D8: 54CA05EE  rlwinm r10, r6, 0, 0x17, 0x17
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9D7DC: 50852636  rlwimi r5, r4, 4, 0x18, 0x1b
	ctx.r[5].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x00000000000000F0) | (ctx.r[5].u64 & 0xFFFFFFFFFFFFFF0F);
	// 82B9D7E0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82B9D7E4: 54C905AC  rlwinm r9, r6, 0, 0x16, 0x16
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9D7E8: 50AA4BEC  rlwimi r10, r5, 9, 0xf, 0x16
	ctx.r[10].u64 = (((ctx.r[5].u32).rotate_left(9) as u64) & 0x000000000001FE00) | (ctx.r[10].u64 & 0xFFFFFFFFFFFE01FF);
	// 82B9D7EC: 38AB0050  addi r5, r11, 0x50
	ctx.r[5].s64 = ctx.r[11].s64 + 80;
	// 82B9D7F0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9D7F4: 386B010C  addi r3, r11, 0x10c
	ctx.r[3].s64 = ctx.r[11].s64 + 268;
	// 82B9D7F8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82B9D7FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9D800: 554B2834  slwi r11, r10, 5
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9D804: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9D808: 480014F1  bl 0x82b9ecf8
	ctx.lr = 0x82B9D80C;
	sub_82B9ECF8(ctx, base);
	// 82B9D80C: 48000024  b 0x82b9d830
	pc = 0x82B9D830; continue 'dispatch;
	// 82B9D810: 50A42636  rlwimi r4, r5, 4, 0x18, 0x1b
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(4) as u64) & 0x00000000000000F0) | (ctx.r[4].u64 & 0xFFFFFFFFFFFFFF0F);
	// 82B9D814: 38AB0050  addi r5, r11, 0x50
	ctx.r[5].s64 = ctx.r[11].s64 + 80;
	// 82B9D818: 548A063E  clrlwi r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82B9D81C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9D820: 5146402E  rlwimi r6, r10, 8, 0, 0x17
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[6].u64 & 0xFFFFFFFF000000FF);
	// 82B9D824: 386B0130  addi r3, r11, 0x130
	ctx.r[3].s64 = ctx.r[11].s64 + 304;
	// 82B9D828: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82B9D82C: 480014CD  bl 0x82b9ecf8
	ctx.lr = 0x82B9D830;
	sub_82B9ECF8(ctx, base);
	// 82B9D830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B9D834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9D838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9D83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D840 size=92
    let mut pc: u32 = 0x82B9D840;
    'dispatch: loop {
        match pc {
            0x82B9D840 => {
    //   block [0x82B9D840..0x82B9D89C)
	// 82B9D840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9D848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D84C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D850: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D854: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D858: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82B9D85C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D860: 814A4DB4  lwz r10, 0x4db4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9D864: 554AF7FF  rlwinm. r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9D868: 40820024  bne 0x82b9d88c
	if !ctx.cr[0].eq {
	pc = 0x82B9D88C; continue 'dispatch;
	}
	// 82B9D86C: 50A40EFC  rlwimi r4, r5, 1, 0x1b, 0x1e
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(1) as u64) & 0x000000000000001E) | (ctx.r[4].u64 & 0xFFFFFFFFFFFFFFE1);
	// 82B9D870: 38AB0050  addi r5, r11, 0x50
	ctx.r[5].s64 = ctx.r[11].s64 + 80;
	// 82B9D874: 548A06FE  clrlwi r10, r4, 0x1b
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82B9D878: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9D87C: 5146402E  rlwimi r6, r10, 8, 0, 0x17
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[6].u64 & 0xFFFFFFFF000000FF);
	// 82B9D880: 386B0118  addi r3, r11, 0x118
	ctx.r[3].s64 = ctx.r[11].s64 + 280;
	// 82B9D884: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82B9D888: 48001471  bl 0x82b9ecf8
	ctx.lr = 0x82B9D88C;
	sub_82B9ECF8(ctx, base);
	// 82B9D88C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B9D890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9D894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9D898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D8A0 size=88
    let mut pc: u32 = 0x82B9D8A0;
    'dispatch: loop {
        match pc {
            0x82B9D8A0 => {
    //   block [0x82B9D8A0..0x82B9D8F8)
	// 82B9D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D8A4: 4810BB69  bl 0x82ca940c
	ctx.lr = 0x82B9D8A8;
	sub_82CA93D0(ctx, base);
	// 82B9D8A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D8AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D8B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82B9D8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9D8B8: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D8BC: 7FCB1A14  add r30, r11, r3
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D8C0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82B9D8C4: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 82B9D8C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9D8CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82B9D8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9D8D4: 4BFFEF85  bl 0x82b9c858
	ctx.lr = 0x82B9D8D8;
	sub_82B9C858(ctx, base);
	// 82B9D8D8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82B9D8DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82B9D8E0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82B9D8E4: 4800126D  bl 0x82b9eb50
	ctx.lr = 0x82B9D8E8;
	sub_82B9EB50(ctx, base);
	// 82B9D8E8: 817E0074  lwz r11, 0x74(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82B9D8EC: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82B9D8F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9D8F4: 4810BB68  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D8F8 size=92
    let mut pc: u32 = 0x82B9D8F8;
    'dispatch: loop {
        match pc {
            0x82B9D8F8 => {
    //   block [0x82B9D8F8..0x82B9D954)
	// 82B9D8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D8FC: 4810BB11  bl 0x82ca940c
	ctx.lr = 0x82B9D900;
	sub_82CA93D0(ctx, base);
	// 82B9D900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D904: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D908: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82B9D90C: 7CAA0034  cntlzw r10, r5
	ctx.r[10].u64 = if ctx.r[5].u32 == 0 { 32 } else { ctx.r[5].u32.leading_zeros() as u64 };
	// 82B9D910: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D914: 7FCB1A14  add r30, r11, r3
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D918: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82B9D91C: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 82B9D920: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9D924: 5545DFFE  rlwinm r5, r10, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9D928: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82B9D92C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9D930: 4BFFEF29  bl 0x82b9c858
	ctx.lr = 0x82B9D934;
	sub_82B9C858(ctx, base);
	// 82B9D934: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82B9D938: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82B9D93C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82B9D940: 48001211  bl 0x82b9eb50
	ctx.lr = 0x82B9D944;
	sub_82B9EB50(ctx, base);
	// 82B9D944: 817E0074  lwz r11, 0x74(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82B9D948: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82B9D94C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9D950: 4810BB0C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D958 size=20
    let mut pc: u32 = 0x82B9D958;
    'dispatch: loop {
        match pc {
            0x82B9D958 => {
    //   block [0x82B9D958..0x82B9D96C)
	// 82B9D958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D95C: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D960: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D964: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82B9D968: 4BFFD858  b 0x82b9b1c0
	sub_82B9B1C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D970 size=20
    let mut pc: u32 = 0x82B9D970;
    'dispatch: loop {
        match pc {
            0x82B9D970 => {
    //   block [0x82B9D970..0x82B9D984)
	// 82B9D970: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D974: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9D978: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9D97C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82B9D980: 4BFFD9D0  b 0x82b9b350
	sub_82B9B350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9D988 size=4
    let mut pc: u32 = 0x82B9D988;
    'dispatch: loop {
        match pc {
            0x82B9D988 => {
    //   block [0x82B9D988..0x82B9D98C)
	// 82B9D988: 4BFFF6E0  b 0x82b9d068
	sub_82B9D068(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9D990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9D990 size=200
    let mut pc: u32 = 0x82B9D990;
    'dispatch: loop {
        match pc {
            0x82B9D990 => {
    //   block [0x82B9D990..0x82B9DA58)
	// 82B9D990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9D994: 4810BA6D  bl 0x82ca9400
	ctx.lr = 0x82B9D998;
	sub_82CA93D0(ctx, base);
	// 82B9D998: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9D99C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82B9D9A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82B9D9A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B9D9A8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82B9D9AC: 388B76FC  addi r4, r11, 0x76fc
	ctx.r[4].s64 = ctx.r[11].s64 + 30460;
	// 82B9D9B0: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 82B9D9B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82B9D9B8: 4810C739  bl 0x82caa0f0
	ctx.lr = 0x82B9D9BC;
	sub_82CAA0F0(ctx, base);
	// 82B9D9BC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9D9C0: 40820008  bne 0x82b9d9c8
	if !ctx.cr[0].eq {
	pc = 0x82B9D9C8; continue 'dispatch;
	}
	// 82B9D9C4: 3B7B000A  addi r27, r27, 0xa
	ctx.r[27].s64 = ctx.r[27].s64 + 10;
	// 82B9D9C8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82B9D9CC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9D9D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9D9D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B9D9D8: 409AFFF4  bne cr6, 0x82b9d9cc
	if !ctx.cr[6].eq {
	pc = 0x82B9D9CC; continue 'dispatch;
	}
	// 82B9D9DC: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82B9D9E0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82B9D9E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82B9D9E8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82B9D9EC: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82B9D9F0: 3BBE4DD0  addi r29, r30, 0x4dd0
	ctx.r[29].s64 = ctx.r[30].s64 + 19920;
	// 82B9D9F4: 397F0003  addi r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 3;
	// 82B9D9F8: FB4A0000  std r26, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82B9D9FC: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82B9DA00: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9DA04: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82B9DA08: 38BE4DBC  addi r5, r30, 0x4dbc
	ctx.r[5].s64 = ctx.r[30].s64 + 19900;
	// 82B9DA0C: 3B8B0002  addi r28, r11, 2
	ctx.r[28].s64 = ctx.r[11].s64 + 2;
	// 82B9DA10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9DA14: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82B9DA18: 480013A9  bl 0x82b9edc0
	ctx.lr = 0x82B9DA1C;
	sub_82B9EDC0(ctx, base);
	// 82B9DA1C: 817E4DBC  lwz r11, 0x4dbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19900 as u32) ) } as u64;
	// 82B9DA20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9DA24: 4198002C  blt cr6, 0x82b9da50
	if ctx.cr[6].lt {
	pc = 0x82B9DA50; continue 'dispatch;
	}
	// 82B9DA28: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DA2C: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9DA30: E9210050  ld r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82B9DA34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82B9DA38: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9DA3C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82B9DA40: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82B9DA44: 934AFFFC  stw r26, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[26].u32 ) };
	// 82B9DA48: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82B9DA4C: 4810BA35  bl 0x82ca9480
	ctx.lr = 0x82B9DA50;
	sub_82CA9480(ctx, base);
	// 82B9DA50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B9DA54: 4810B9FC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DA58 size=20
    let mut pc: u32 = 0x82B9DA58;
    'dispatch: loop {
        match pc {
            0x82B9DA58 => {
    //   block [0x82B9DA58..0x82B9DA6C)
	// 82B9DA58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DA5C: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DA60: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DA64: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82B9DA68: 4BFFDA20  b 0x82b9b488
	sub_82B9B488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9DA70 size=148
    let mut pc: u32 = 0x82B9DA70;
    'dispatch: loop {
        match pc {
            0x82B9DA70 => {
    //   block [0x82B9DA70..0x82B9DB04)
	// 82B9DA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9DA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9DA78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9DA7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9DA80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DA84: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DA88: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DA8C: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82B9DA90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9DA94: 4BFFEAAD  bl 0x82b9c540
	ctx.lr = 0x82B9DA98;
	sub_82B9C540(ctx, base);
	// 82B9DA98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DA9C: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9DAA0: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9DAA4: 556BE7FF  rlwinm. r11, r11, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9DAA8: 41820044  beq 0x82b9daec
	if ctx.cr[0].eq {
	pc = 0x82B9DAEC; continue 'dispatch;
	}
	// 82B9DAAC: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9DAB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B9DAB4: 419A0038  beq cr6, 0x82b9daec
	if ctx.cr[6].eq {
	pc = 0x82B9DAEC; continue 'dispatch;
	}
	// 82B9DAB8: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82B9DABC: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9DAC0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9DAC4: 356BFFF8  addic. r11, r11, -8
	ctx.xer.ca = (ctx.r[11].u32 > (!(-8 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9DAC8: 41820024  beq 0x82b9daec
	if ctx.cr[0].eq {
	pc = 0x82B9DAEC; continue 'dispatch;
	}
	// 82B9DACC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9DAD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9DAD4: 554AA73E  rlwinm r10, r10, 0x14, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9DAD8: 7D6A5030  slw r10, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9DADC: 714A607E  andi. r10, r10, 0x607e
	ctx.r[10].u64 = ctx.r[10].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9DAE0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9DAE4: 41820008  beq 0x82b9daec
	if ctx.cr[0].eq {
	pc = 0x82B9DAEC; continue 'dispatch;
	}
	// 82B9DAE8: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82B9DAEC: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9DAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B9DAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9DAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9DAFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9DB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DB08 size=104
    let mut pc: u32 = 0x82B9DB08;
    'dispatch: loop {
        match pc {
            0x82B9DB08 => {
    //   block [0x82B9DB08..0x82B9DB70)
	// 82B9DB08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DB0C: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DB10: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DB14: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B9DB18: 814A4DB4  lwz r10, 0x4db4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9DB1C: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82B9DB20: 554AE7FF  rlwinm. r10, r10, 0x1c, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9DB24: 41820044  beq 0x82b9db68
	if ctx.cr[0].eq {
	pc = 0x82B9DB68; continue 'dispatch;
	}
	// 82B9DB28: 812B0080  lwz r9, 0x80(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82B9DB2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82B9DB30: 419A0038  beq cr6, 0x82b9db68
	if ctx.cr[6].eq {
	pc = 0x82B9DB68; continue 'dispatch;
	}
	// 82B9DB34: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9DB38: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9DB3C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82B9DB40: 354AFFF8  addic. r10, r10, -8
	ctx.xer.ca = (ctx.r[10].u32 > (!(-8 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9DB44: 41820024  beq 0x82b9db68
	if ctx.cr[0].eq {
	pc = 0x82B9DB68; continue 'dispatch;
	}
	// 82B9DB48: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9DB4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B9DB50: 5529A73E  rlwinm r9, r9, 0x14, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82B9DB54: 7D494830  slw r9, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9DB58: 7129607E  andi. r9, r9, 0x607e
	ctx.r[9].u64 = ctx.r[9].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9DB5C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9DB60: 41820008  beq 0x82b9db68
	if ctx.cr[0].eq {
	pc = 0x82B9DB68; continue 'dispatch;
	}
	// 82B9DB64: 914B0088  stw r10, 0x88(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82B9DB68: 806B0080  lwz r3, 0x80(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82B9DB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DB70 size=20
    let mut pc: u32 = 0x82B9DB70;
    'dispatch: loop {
        match pc {
            0x82B9DB70 => {
    //   block [0x82B9DB70..0x82B9DB84)
	// 82B9DB70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DB74: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DB78: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DB7C: 806B0074  lwz r3, 0x74(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82B9DB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DB88 size=24
    let mut pc: u32 = 0x82B9DB88;
    'dispatch: loop {
        match pc {
            0x82B9DB88 => {
    //   block [0x82B9DB88..0x82B9DBA0)
	// 82B9DB88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DB8C: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DB90: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DB94: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82B9DB98: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82B9DB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DBA0 size=32
    let mut pc: u32 = 0x82B9DBA0;
    'dispatch: loop {
        match pc {
            0x82B9DBA0 => {
    //   block [0x82B9DBA0..0x82B9DBC0)
	// 82B9DBA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DBA4: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DBA8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DBAC: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9DBB0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9DBB4: 4098000C  bge cr6, 0x82b9dbc0
	if !ctx.cr[6].lt {
		sub_82B9DBC0(ctx, base);
		return;
	}
	// 82B9DBB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9DBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DBC0 size=16
    let mut pc: u32 = 0x82B9DBC0;
    'dispatch: loop {
        match pc {
            0x82B9DBC0 => {
    //   block [0x82B9DBC0..0x82B9DBD0)
	// 82B9DBC0: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9DBC4: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9DBC8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9DBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DBD0 size=20
    let mut pc: u32 = 0x82B9DBD0;
    'dispatch: loop {
        match pc {
            0x82B9DBD0 => {
    //   block [0x82B9DBD0..0x82B9DBE4)
	// 82B9DBD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DBD4: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DBD8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DBDC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82B9DBE0: 4BFFD998  b 0x82b9b578
	sub_82B9B578(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DBE8 size=44
    let mut pc: u32 = 0x82B9DBE8;
    'dispatch: loop {
        match pc {
            0x82B9DBE8 => {
    //   block [0x82B9DBE8..0x82B9DC14)
	// 82B9DBE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DBEC: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DBF0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82B9DBF4: 814B0080  lwz r10, 0x80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82B9DBF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82B9DBFC: 419A0058  beq cr6, 0x82b9dc54
	if ctx.cr[6].eq {
		sub_82B9DC54(ctx, base);
		return;
	}
	// 82B9DC00: 812B0060  lwz r9, 0x60(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9DC04: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9DC08: 4098000C  bge cr6, 0x82b9dc14
	if !ctx.cr[6].lt {
		sub_82B9DC14(ctx, base);
		return;
	}
	// 82B9DC0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9DC10: 48000014  b 0x82b9dc24
	sub_82B9DC14(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DC14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DC14 size=64
    let mut pc: u32 = 0x82B9DC14;
    'dispatch: loop {
        match pc {
            0x82B9DC14 => {
    //   block [0x82B9DC14..0x82B9DC54)
	// 82B9DC14: 812B007C  lwz r9, 0x7c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9DC18: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9DC1C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82B9DC20: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82B9DC24: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9DC28: 554AA73F  rlwinm. r10, r10, 0x14, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9DC2C: 41820028  beq 0x82b9dc54
	if ctx.cr[0].eq {
		sub_82B9DC54(ctx, base);
		return;
	}
	// 82B9DC30: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 82B9DC34: 40990014  ble cr6, 0x82b9dc48
	if !ctx.cr[6].gt {
	pc = 0x82B9DC48; continue 'dispatch;
	}
	// 82B9DC38: 2B0A000C  cmplwi cr6, r10, 0xc
	ctx.cr[6].compare_u32(ctx.r[10].u32, 12 as u32, &mut ctx.xer);
	// 82B9DC3C: 40990018  ble cr6, 0x82b9dc54
	if !ctx.cr[6].gt {
		sub_82B9DC54(ctx, base);
		return;
	}
	// 82B9DC40: 2B0A000E  cmplwi cr6, r10, 0xe
	ctx.cr[6].compare_u32(ctx.r[10].u32, 14 as u32, &mut ctx.xer);
	// 82B9DC44: 41990010  bgt cr6, 0x82b9dc54
	if ctx.cr[6].gt {
		sub_82B9DC54(ctx, base);
		return;
	}
	// 82B9DC48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DC4C: 5563A77E  rlwinm r3, r11, 0x14, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9DC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DC54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9DC54 size=8
    let mut pc: u32 = 0x82B9DC54;
    'dispatch: loop {
        match pc {
            0x82B9DC54 => {
    //   block [0x82B9DC54..0x82B9DC5C)
	// 82B9DC54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9DC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9DC60 size=88
    let mut pc: u32 = 0x82B9DC60;
    'dispatch: loop {
        match pc {
            0x82B9DC60 => {
    //   block [0x82B9DC60..0x82B9DCB8)
	// 82B9DC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9DC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9DC68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9DC6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9DC70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9DC74: 817F4DB4  lwz r11, 0x4db4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9DC78: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9DC7C: 4182000C  beq 0x82b9dc88
	if ctx.cr[0].eq {
	pc = 0x82B9DC88; continue 'dispatch;
	}
	// 82B9DC80: 4BFFF509  bl 0x82b9d188
	ctx.lr = 0x82B9DC84;
	sub_82B9D188(ctx, base);
	// 82B9DC84: 48000018  b 0x82b9dc9c
	pc = 0x82B9DC9C; continue 'dispatch;
	// 82B9DC88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DC8C: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82B9DC90: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82B9DC94: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82B9DC98: 4BFFC4D1  bl 0x82b9a168
	ctx.lr = 0x82B9DC9C;
	sub_82B9A168(ctx, base);
	// 82B9DC9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9DCA0: 917F4DC0  stw r11, 0x4dc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19904 as u32), ctx.r[11].u32 ) };
	// 82B9DCA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B9DCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9DCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9DCB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9DCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9DCB8 size=144
    let mut pc: u32 = 0x82B9DCB8;
    'dispatch: loop {
        match pc {
            0x82B9DCB8 => {
    //   block [0x82B9DCB8..0x82B9DD48)
	// 82B9DCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9DCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9DCC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B9DCC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9DCC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9DCCC: 81634DBC  lwz r11, 0x4dbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(19900 as u32) ) } as u64;
	// 82B9DCD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9DCD4: 4098000C  bge cr6, 0x82b9dce0
	if !ctx.cr[6].lt {
	pc = 0x82B9DCE0; continue 'dispatch;
	}
	// 82B9DCD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9DCDC: 48000054  b 0x82b9dd30
	pc = 0x82B9DD30; continue 'dispatch;
	// 82B9DCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9DCE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9DCE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9DCEC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82B9DCF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9DCF4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82B9DCF8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82B9DCFC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82B9DD00: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82B9DD04: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82B9DD08: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82B9DD0C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82B9DD10: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82B9DD14: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82B9DD18: 480002F1  bl 0x82b9e008
	ctx.lr = 0x82B9DD1C;
	sub_82B9E008(ctx, base);
	// 82B9DD1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9DD20: 4180000C  blt 0x82b9dd2c
	if ctx.cr[0].lt {
	pc = 0x82B9DD2C; continue 'dispatch;
	}
	// 82B9DD24: 83E10070  lwz r31, 0x70(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9DD28: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82B9DD2C: 7C7EFA14  add r3, r30, r31
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82B9DD30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82B9DD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9DD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9DD3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9DD40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9DD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9DD48 size=212
    let mut pc: u32 = 0x82B9DD48;
    'dispatch: loop {
        match pc {
            0x82B9DD48 => {
    //   block [0x82B9DD48..0x82B9DE1C)
	// 82B9DD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9DD4C: 4810B6B9  bl 0x82ca9404
	ctx.lr = 0x82B9DD50;
	sub_82CA93D0(ctx, base);
	// 82B9DD50: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9DD54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9DD58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82B9DD5C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82B9DD60: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82B9DD64: 817F4DBC  lwz r11, 0x4dbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19900 as u32) ) } as u64;
	// 82B9DD68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9DD6C: 4098000C  bge cr6, 0x82b9dd78
	if !ctx.cr[6].lt {
	pc = 0x82B9DD78; continue 'dispatch;
	}
	// 82B9DD70: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82B9DD74: 480000A0  b 0x82b9de14
	pc = 0x82B9DE14; continue 'dispatch;
	// 82B9DD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82B9DD7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9DD80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9DD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9DD88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9DD8C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82B9DD90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82B9DD94: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82B9DD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82B9DD9C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82B9DDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82B9DDA4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82B9DDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82B9DDAC: 4800025D  bl 0x82b9e008
	ctx.lr = 0x82B9DDB0;
	sub_82B9E008(ctx, base);
	// 82B9DDB0: 907F4DBC  stw r3, 0x4dbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19900 as u32), ctx.r[3].u32 ) };
	// 82B9DDB4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9DDB8: 4180005C  blt 0x82b9de14
	if ctx.cr[0].lt {
	pc = 0x82B9DE14; continue 'dispatch;
	}
	// 82B9DDBC: 80A10070  lwz r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9DDC0: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82B9DDC4: 7F853A14  add r28, r5, r7
	ctx.r[28].u64 = ctx.r[5].u64 + ctx.r[7].u64;
	// 82B9DDC8: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82B9DDCC: 40990010  ble cr6, 0x82b9dddc
	if !ctx.cr[6].gt {
	pc = 0x82B9DDDC; continue 'dispatch;
	}
	// 82B9DDD0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 82B9DDD4: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 82B9DDD8: 4800003C  b 0x82b9de14
	pc = 0x82B9DE14; continue 'dispatch;
	// 82B9DDDC: 7CC5F214  add r6, r5, r30
	ctx.r[6].u64 = ctx.r[5].u64 + ctx.r[30].u64;
	// 82B9DDE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82B9DDE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82B9DDE8: 4BFFBB61  bl 0x82b99948
	ctx.lr = 0x82B9DDEC;
	sub_82B99948(ctx, base);
	// 82B9DDEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9DDF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9DDF4: 48000215  bl 0x82b9e008
	ctx.lr = 0x82B9DDF8;
	sub_82B9E008(ctx, base);
	// 82B9DDF8: 907F4DBC  stw r3, 0x4dbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19900 as u32), ctx.r[3].u32 ) };
	// 82B9DDFC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9DE00: 41800010  blt 0x82b9de10
	if ctx.cr[0].lt {
	pc = 0x82B9DE10; continue 'dispatch;
	}
	// 82B9DE04: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82B9DE08: 419A0008  beq cr6, 0x82b9de10
	if ctx.cr[6].eq {
	pc = 0x82B9DE10; continue 'dispatch;
	}
	// 82B9DE0C: 939B0000  stw r28, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82B9DE10: 807F4DBC  lwz r3, 0x4dbc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19900 as u32) ) } as u64;
	// 82B9DE14: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82B9DE18: 4810B63C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9DE20 size=180
    let mut pc: u32 = 0x82B9DE20;
    'dispatch: loop {
        match pc {
            0x82B9DE20 => {
    //   block [0x82B9DE20..0x82B9DED4)
	// 82B9DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9DE24: 4810B5E1  bl 0x82ca9404
	ctx.lr = 0x82B9DE28;
	sub_82CA93D0(ctx, base);
	// 82B9DE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9DE2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9DE30: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82B9DE34: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82B9DE38: 3BDF26CC  addi r30, r31, 0x26cc
	ctx.r[30].s64 = ctx.r[31].s64 + 9932;
	// 82B9DE3C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82B9DE40: 937F4DB0  stw r27, 0x4db0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19888 as u32), ctx.r[27].u32 ) };
	// 82B9DE44: 387ED944  addi r3, r30, -0x26bc
	ctx.r[3].s64 = ctx.r[30].s64 + -9916;
	// 82B9DE48: 4BFFBFE1  bl 0x82b99e28
	ctx.lr = 0x82B9DE4C;
	sub_82B99E28(ctx, base);
	// 82B9DE4C: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82B9DE50: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82B9DE54: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82B9DE58: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9DE5C: 2B1D0002  cmplwi cr6, r29, 2
	ctx.cr[6].compare_u32(ctx.r[29].u32, 2 as u32, &mut ctx.xer);
	// 82B9DE60: 917EFFFC  stw r11, -4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82B9DE64: 3BDE26D0  addi r30, r30, 0x26d0
	ctx.r[30].s64 = ctx.r[30].s64 + 9936;
	// 82B9DE68: 4198FFDC  blt cr6, 0x82b9de44
	if ctx.cr[6].lt {
	pc = 0x82B9DE44; continue 'dispatch;
	}
	// 82B9DE6C: 939F4DBC  stw r28, 0x4dbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19900 as u32), ctx.r[28].u32 ) };
	// 82B9DE70: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82B9DE74: 939F4DB4  stw r28, 0x4db4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19892 as u32), ctx.r[28].u32 ) };
	// 82B9DE78: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 82B9DE7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9DE80: 4BFFBFA9  bl 0x82b99e28
	ctx.lr = 0x82B9DE84;
	sub_82B99E28(ctx, base);
	// 82B9DE84: 937E26BC  stw r27, 0x26bc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(9916 as u32), ctx.r[27].u32 ) };
	// 82B9DE88: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9DE8C: 3BDE26D0  addi r30, r30, 0x26d0
	ctx.r[30].s64 = ctx.r[30].s64 + 9936;
	// 82B9DE90: 4082FFEC  bne 0x82b9de7c
	if !ctx.cr[0].eq {
	pc = 0x82B9DE7C; continue 'dispatch;
	}
	// 82B9DE94: 807F4DB8  lwz r3, 0x4db8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9DE98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9DE9C: 419A0018  beq cr6, 0x82b9deb4
	if ctx.cr[6].eq {
	pc = 0x82B9DEB4; continue 'dispatch;
	}
	// 82B9DEA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DEA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9DEA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9DEAC: 4E800421  bctrl
	ctx.lr = 0x82B9DEB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9DEB0: 939F4DB8  stw r28, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[28].u32 ) };
	// 82B9DEB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82B9DEB8: 939F4DC0  stw r28, 0x4dc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19904 as u32), ctx.r[28].u32 ) };
	// 82B9DEBC: 939F4DDC  stw r28, 0x4ddc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19932 as u32), ctx.r[28].u32 ) };
	// 82B9DEC0: 917F4DE0  stw r11, 0x4de0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19936 as u32), ctx.r[11].u32 ) };
	// 82B9DEC4: 939F4DE8  stw r28, 0x4de8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19944 as u32), ctx.r[28].u32 ) };
	// 82B9DEC8: 939F4DF0  stw r28, 0x4df0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19952 as u32), ctx.r[28].u32 ) };
	// 82B9DECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9DED0: 4810B584  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9DED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82B9DED8 size=300
    let mut pc: u32 = 0x82B9DED8;
    'dispatch: loop {
        match pc {
            0x82B9DED8 => {
    //   block [0x82B9DED8..0x82B9E004)
	// 82B9DED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9DEDC: 4810B529  bl 0x82ca9404
	ctx.lr = 0x82B9DEE0;
	sub_82CA93D0(ctx, base);
	// 82B9DEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9DEE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9DEE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82B9DEEC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82B9DEF0: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82B9DEF4: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 82B9DEF8: 93BF4DBC  stw r29, 0x4dbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19900 as u32), ctx.r[29].u32 ) };
	// 82B9DEFC: 93BF4DB4  stw r29, 0x4db4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19892 as u32), ctx.r[29].u32 ) };
	// 82B9DF00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9DF04: 4BFFBF25  bl 0x82b99e28
	ctx.lr = 0x82B9DF08;
	sub_82B99E28(ctx, base);
	// 82B9DF08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9DF0C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82B9DF10: 917E26BC  stw r11, 0x26bc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(9916 as u32), ctx.r[11].u32 ) };
	// 82B9DF14: 3BDE26D0  addi r30, r30, 0x26d0
	ctx.r[30].s64 = ctx.r[30].s64 + 9936;
	// 82B9DF18: 4082FFE8  bne 0x82b9df00
	if !ctx.cr[0].eq {
	pc = 0x82B9DF00; continue 'dispatch;
	}
	// 82B9DF1C: 807F4DB8  lwz r3, 0x4db8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82B9DF20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82B9DF24: 419A0018  beq cr6, 0x82b9df3c
	if ctx.cr[6].eq {
	pc = 0x82B9DF3C; continue 'dispatch;
	}
	// 82B9DF28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9DF2C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9DF30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82B9DF34: 4E800421  bctrl
	ctx.lr = 0x82B9DF38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9DF38: 93BF4DB8  stw r29, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[29].u32 ) };
	// 82B9DF3C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82B9DF40: 937F4DB4  stw r27, 0x4db4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19892 as u32), ctx.r[27].u32 ) };
	// 82B9DF44: 93BF4DC0  stw r29, 0x4dc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19904 as u32), ctx.r[29].u32 ) };
	// 82B9DF48: 3BDF26D0  addi r30, r31, 0x26d0
	ctx.r[30].s64 = ctx.r[31].s64 + 9936;
	// 82B9DF4C: 917F4DE0  stw r11, 0x4de0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19936 as u32), ctx.r[11].u32 ) };
	// 82B9DF50: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 82B9DF54: 93BF4DDC  stw r29, 0x4ddc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19932 as u32), ctx.r[29].u32 ) };
	// 82B9DF58: 93BF4DE8  stw r29, 0x4de8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19944 as u32), ctx.r[29].u32 ) };
	// 82B9DF5C: 93BF4DF0  stw r29, 0x4df0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19952 as u32), ctx.r[29].u32 ) };
	// 82B9DF60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82B9DF64: 387ED940  addi r3, r30, -0x26c0
	ctx.r[3].s64 = ctx.r[30].s64 + -9920;
	// 82B9DF68: 4BFFBF81  bl 0x82b99ee8
	ctx.lr = 0x82B9DF6C;
	sub_82B99EE8(ctx, base);
	// 82B9DF6C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82B9DF70: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9DF74: 38A023A0  li r5, 0x23a0
	ctx.r[5].s64 = 9120;
	// 82B9DF78: FBBEFE40  std r29, -0x1c0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(-448 as u32), ctx.r[29].u64 ) };
	// 82B9DF7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9DF80: FBBEFE48  std r29, -0x1b8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(-440 as u32), ctx.r[29].u64 ) };
	// 82B9DF84: 387EDAA0  addi r3, r30, -0x2560
	ctx.r[3].s64 = ctx.r[30].s64 + -9568;
	// 82B9DF88: FBBEFE50  std r29, -0x1b0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(-432 as u32), ctx.r[29].u64 ) };
	// 82B9DF8C: 4810BA25  bl 0x82ca99b0
	ctx.lr = 0x82B9DF90;
	sub_82CA99B0(ctx, base);
	// 82B9DF90: 38A00120  li r5, 0x120
	ctx.r[5].s64 = 288;
	// 82B9DF94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9DF98: 387EFE98  addi r3, r30, -0x168
	ctx.r[3].s64 = ctx.r[30].s64 + -360;
	// 82B9DF9C: 4810BA15  bl 0x82ca99b0
	ctx.lr = 0x82B9DFA0;
	sub_82CA99B0(ctx, base);
	// 82B9DFA0: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9DFA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9DFA8: 387EFFB8  addi r3, r30, -0x48
	ctx.r[3].s64 = ctx.r[30].s64 + -72;
	// 82B9DFAC: 4810BA05  bl 0x82ca99b0
	ctx.lr = 0x82B9DFB0;
	sub_82CA99B0(ctx, base);
	// 82B9DFB0: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82B9DFB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9DFB8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82B9DFBC: 4810B9F5  bl 0x82ca99b0
	ctx.lr = 0x82B9DFC0;
	sub_82CA99B0(ctx, base);
	// 82B9DFC0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82B9DFC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82B9DFC8: 387EFE58  addi r3, r30, -0x1a8
	ctx.r[3].s64 = ctx.r[30].s64 + -424;
	// 82B9DFCC: 4810B9E5  bl 0x82ca99b0
	ctx.lr = 0x82B9DFD0;
	sub_82CA99B0(ctx, base);
	// 82B9DFD0: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82B9DFD4: 3BDE26D0  addi r30, r30, 0x26d0
	ctx.r[30].s64 = ctx.r[30].s64 + 9936;
	// 82B9DFD8: 4082FF88  bne 0x82b9df60
	if !ctx.cr[0].eq {
	pc = 0x82B9DF60; continue 'dispatch;
	}
	// 82B9DFDC: 93BF4DC8  stw r29, 0x4dc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19912 as u32), ctx.r[29].u32 ) };
	// 82B9DFE0: 817F4DB4  lwz r11, 0x4db4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9DFE4: 556BD7FE  rlwinm r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82B9DFE8: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82B9DFEC: 93BF4DD4  stw r29, 0x4dd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19924 as u32), ctx.r[29].u32 ) };
	// 82B9DFF0: 93BF4DB8  stw r29, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[29].u32 ) };
	// 82B9DFF4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B9DFF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9DFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9E000: 4810B454  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9E008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9E008 size=484
    let mut pc: u32 = 0x82B9E008;
    'dispatch: loop {
        match pc {
            0x82B9E008 => {
    //   block [0x82B9E008..0x82B9E1EC)
	// 82B9E008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9E00C: 4810B3F1  bl 0x82ca93fc
	ctx.lr = 0x82B9E010;
	sub_82CA93D0(ctx, base);
	// 82B9E010: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9E014: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B9E018: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82B9E01C: 807E4DBC  lwz r3, 0x4dbc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19900 as u32) ) } as u64;
	// 82B9E020: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82B9E024: 419801C0  blt cr6, 0x82b9e1e4
	if ctx.cr[6].lt {
	pc = 0x82B9E1E4; continue 'dispatch;
	}
	// 82B9E028: 817E4DC0  lwz r11, 0x4dc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19904 as u32) ) } as u64;
	// 82B9E02C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E030: 409A0010  bne cr6, 0x82b9e040
	if !ctx.cr[6].eq {
	pc = 0x82B9E040; continue 'dispatch;
	}
	// 82B9E034: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82B9E038: 60634005  ori r3, r3, 0x4005
	ctx.r[3].u64 = ctx.r[3].u64 | 16389;
	// 82B9E03C: 480001A8  b 0x82b9e1e4
	pc = 0x82B9E1E4; continue 'dispatch;
	// 82B9E040: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82B9E044: 815E4DB4  lwz r10, 0x4db4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82B9E048: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82B9E04C: 833A0018  lwz r25, 0x18(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B9E050: 5549F7FE  rlwinm r9, r10, 0x1e, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82B9E054: 3D00102A  lis r8, 0x102a
	ctx.r[8].s64 = 271187968;
	// 82B9E058: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82B9E05C: FBAB0000  std r29, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 82B9E060: 61081100  ori r8, r8, 0x1100
	ctx.r[8].u64 = ctx.r[8].u64 | 4352;
	// 82B9E064: FBAB0008  std r29, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 82B9E068: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9E06C: FBAB0010  std r29, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u64 ) };
	// 82B9E070: 5148FEB4  rlwimi r8, r10, 0x1f, 0x1a, 0x1a
	ctx.r[8].u64 = (((ctx.r[10].u32).rotate_left(31) as u64) & 0x0000000000000020) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFDF);
	// 82B9E074: FBAB0018  std r29, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u64 ) };
	// 82B9E078: 93AB0020  stw r29, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82B9E07C: 7D0B4B78  or r11, r8, r9
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 82B9E080: 3BFA0014  addi r31, r26, 0x14
	ctx.r[31].s64 = ctx.r[26].s64 + 20;
	// 82B9E084: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 82B9E088: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82B9E08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E090: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9E094: 714A00DE  andi. r10, r10, 0xde
	ctx.r[10].u64 = ctx.r[10].u64 & 222;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9E098: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9E09C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82B9E0A0: 4BFFB821  bl 0x82b998c0
	ctx.lr = 0x82B9E0A4;
	sub_82B998C0(ctx, base);
	// 82B9E0A4: 817E4DD4  lwz r11, 0x4dd4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19924 as u32) ) } as u64;
	// 82B9E0A8: 5565103B  rlwinm. r5, r11, 2, 0, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82B9E0AC: 4182001C  beq 0x82b9e0c8
	if ctx.cr[0].eq {
	pc = 0x82B9E0C8; continue 'dispatch;
	}
	// 82B9E0B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E0B8: 809E4DD0  lwz r4, 0x4dd0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19920 as u32) ) } as u64;
	// 82B9E0BC: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 82B9E0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82B9E0C4: 4BFFB7FD  bl 0x82b998c0
	ctx.lr = 0x82B9E0C8;
	sub_82B998C0(ctx, base);
	// 82B9E0C8: 817E4DC8  lwz r11, 0x4dc8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19912 as u32) ) } as u64;
	// 82B9E0CC: 557C103B  rlwinm. r28, r11, 2, 0, 0x1d
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82B9E0D0: 41820034  beq 0x82b9e104
	if ctx.cr[0].eq {
	pc = 0x82B9E104; continue 'dispatch;
	}
	// 82B9E0D4: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82B9E0D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82B9E0DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E0E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82B9E0E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E0E8: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 82B9E0EC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82B9E0F0: 4BFFB7D1  bl 0x82b998c0
	ctx.lr = 0x82B9E0F4;
	sub_82B998C0(ctx, base);
	// 82B9E0F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82B9E0F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E0FC: 809E4DC4  lwz r4, 0x4dc4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19908 as u32) ) } as u64;
	// 82B9E100: 4BFFB7C1  bl 0x82b998c0
	ctx.lr = 0x82B9E104;
	sub_82B998C0(ctx, base);
	// 82B9E104: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82B9E108: 3B810074  addi r28, r1, 0x74
	ctx.r[28].s64 = ctx.r[1].s64 + 116;
	// 82B9E10C: 3BBE0170  addi r29, r30, 0x170
	ctx.r[29].s64 = ctx.r[30].s64 + 368;
	// 82B9E110: 817D255C  lwz r11, 0x255c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(9564 as u32) ) } as u64;
	// 82B9E114: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E118: 409A0024  bne cr6, 0x82b9e13c
	if !ctx.cr[6].eq {
	pc = 0x82B9E13C; continue 'dispatch;
	}
	// 82B9E11C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E120: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82B9E124: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9E128: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9E12C: 4BFFB89D  bl 0x82b999c8
	ctx.lr = 0x82B9E130;
	sub_82B999C8(ctx, base);
	// 82B9E130: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E134: 40800008  bge 0x82b9e13c
	if !ctx.cr[0].lt {
	pc = 0x82B9E13C; continue 'dispatch;
	}
	// 82B9E138: 907E4DBC  stw r3, 0x4dbc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(19900 as u32), ctx.r[3].u32 ) };
	// 82B9E13C: 817DFF10  lwz r11, -0xf0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-240 as u32) ) } as u64;
	// 82B9E140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9E144: 419A0030  beq cr6, 0x82b9e174
	if ctx.cr[6].eq {
	pc = 0x82B9E174; continue 'dispatch;
	}
	// 82B9E148: 7F6B0034  cntlzw r11, r27
	ctx.r[11].u64 = if ctx.r[27].u32 == 0 { 32 } else { ctx.r[27].u32.leading_zeros() as u64 };
	// 82B9E14C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E150: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82B9E154: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9E158: 387DFEA0  addi r3, r29, -0x160
	ctx.r[3].s64 = ctx.r[29].s64 + -352;
	// 82B9E15C: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 82B9E160: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82B9E164: 4BFFC67D  bl 0x82b9a7e0
	ctx.lr = 0x82B9E168;
	sub_82B9A7E0(ctx, base);
	// 82B9E168: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E16C: 40800008  bge 0x82b9e174
	if !ctx.cr[0].lt {
	pc = 0x82B9E174; continue 'dispatch;
	}
	// 82B9E170: 907E4DBC  stw r3, 0x4dbc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(19900 as u32), ctx.r[3].u32 ) };
	// 82B9E174: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82B9E178: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 82B9E17C: 3BBD26D0  addi r29, r29, 0x26d0
	ctx.r[29].s64 = ctx.r[29].s64 + 9936;
	// 82B9E180: 2B1B0002  cmplwi cr6, r27, 2
	ctx.cr[6].compare_u32(ctx.r[27].u32, 2 as u32, &mut ctx.xer);
	// 82B9E184: 4198FF8C  blt cr6, 0x82b9e110
	if ctx.cr[6].lt {
	pc = 0x82B9E110; continue 'dispatch;
	}
	// 82B9E188: 817E4DF0  lwz r11, 0x4df0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19952 as u32) ) } as u64;
	// 82B9E18C: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 82B9E190: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9E194: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82B9E198: 813A000C  lwz r9, 0xc(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9E19C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E1A0: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9E1A4: 516A26F6  rlwimi r10, r11, 4, 0x1b, 0x1b
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(4) as u64) & 0x0000000000000010) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFEF);
	// 82B9E1A8: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E1AC: 933F0004  stw r25, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82B9E1B0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82B9E1B4: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82B9E1B8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82B9E1BC: 4BFFB705  bl 0x82b998c0
	ctx.lr = 0x82B9E1C0;
	sub_82B998C0(ctx, base);
	// 82B9E1C0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82B9E1C4: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B9E1C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E1CC: 41980010  blt cr6, 0x82b9e1dc
	if ctx.cr[6].lt {
	pc = 0x82B9E1DC; continue 'dispatch;
	}
	// 82B9E1D0: 817A0024  lwz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82B9E1D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E1D8: 40980008  bge cr6, 0x82b9e1e0
	if !ctx.cr[6].lt {
	pc = 0x82B9E1E0; continue 'dispatch;
	}
	// 82B9E1DC: 917E4DBC  stw r11, 0x4dbc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(19900 as u32), ctx.r[11].u32 ) };
	// 82B9E1E0: 807E4DBC  lwz r3, 0x4dbc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19900 as u32) ) } as u64;
	// 82B9E1E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82B9E1E8: 4810B264  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9E1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9E1F0 size=76
    let mut pc: u32 = 0x82B9E1F0;
    'dispatch: loop {
        match pc {
            0x82B9E1F0 => {
    //   block [0x82B9E1F0..0x82B9E23C)
	// 82B9E1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9E1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9E1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9E1FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9E200: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9E204: 38604E00  li r3, 0x4e00
	ctx.r[3].s64 = 19968;
	// 82B9E208: 4B68E7C9  bl 0x8222c9d0
	ctx.lr = 0x82B9E20C;
	sub_8222C9D0(ctx, base);
	// 82B9E20C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82B9E210: 4082000C  bne 0x82b9e21c
	if !ctx.cr[0].eq {
	pc = 0x82B9E21C; continue 'dispatch;
	}
	// 82B9E214: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9E218: 48000010  b 0x82b9e228
	pc = 0x82B9E228; continue 'dispatch;
	// 82B9E21C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E220: 4BFFFC01  bl 0x82b9de20
	ctx.lr = 0x82B9E224;
	sub_82B9DE20(ctx, base);
	// 82B9E224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B9E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9E234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9E238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9E240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82B9E240 size=408
    let mut pc: u32 = 0x82B9E240;
    'dispatch: loop {
        match pc {
            0x82B9E240 => {
    //   block [0x82B9E240..0x82B9E3D8)
	// 82B9E240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9E244: 4810B1AD  bl 0x82ca93f0
	ctx.lr = 0x82B9E248;
	sub_82CA93D0(ctx, base);
	// 82B9E248: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9E24C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E250: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82B9E254: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 82B9E258: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 82B9E25C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9E260: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82B9E264: 409A0020  bne cr6, 0x82b9e284
	if !ctx.cr[6].eq {
	pc = 0x82B9E284; continue 'dispatch;
	}
	// 82B9E268: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E26C: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E270: 40820014  bne 0x82b9e284
	if !ctx.cr[0].eq {
	pc = 0x82B9E284; continue 'dispatch;
	}
	// 82B9E274: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E278: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9E27C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B9E280: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9E284: 56CB077D  rlwinm. r11, r22, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E288: 41820148  beq 0x82b9e3d0
	if ctx.cr[0].eq {
	pc = 0x82B9E3D0; continue 'dispatch;
	}
	// 82B9E28C: 83F80004  lwz r31, 4(r24)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E290: 57EB0529  rlwinm. r11, r31, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E294: 41820010  beq 0x82b9e2a4
	if ctx.cr[0].eq {
	pc = 0x82B9E2A4; continue 'dispatch;
	}
	// 82B9E298: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82B9E29C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82B9E2A0: 4800000C  b 0x82b9e2ac
	pc = 0x82B9E2AC; continue 'dispatch;
	// 82B9E2A4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E2A8: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82B9E2AC: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E2B0: 5579A77F  rlwinm. r25, r11, 0x14, 0x1d, 0x1f
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82B9E2B4: 4182011C  beq 0x82b9e3d0
	if ctx.cr[0].eq {
	pc = 0x82B9E3D0; continue 'dispatch;
	}
	// 82B9E2B8: 556A053E  clrlwi r10, r11, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9E2BC: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 * 12;
	// 82B9E2C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82B9E2C4: 41980104  blt cr6, 0x82b9e3c8
	if ctx.cr[6].lt {
	pc = 0x82B9E3C8; continue 'dispatch;
	}
	// 82B9E2C8: 1D39000C  mulli r9, r25, 0xc
	ctx.r[9].s64 = ctx.r[25].s64 * 12;
	// 82B9E2CC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82B9E2D0: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82B9E2D4: 419900F4  bgt cr6, 0x82b9e3c8
	if ctx.cr[6].gt {
	pc = 0x82B9E3C8; continue 'dispatch;
	}
	// 82B9E2D8: 57E926B6  rlwinm r9, r31, 4, 0x1a, 0x1b
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x0FFFFFFFu64;
	// 82B9E2DC: 5568273E  srwi r8, r11, 0x1c
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9E2E0: 7FAA3A14  add r29, r10, r7
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82B9E2E4: 557C853E  rlwinm r28, r11, 0x10, 0x14, 0x1f
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9E2E8: 7D3A4378  or r26, r9, r8
	ctx.r[26].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82B9E2EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82B9E2F0: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82B9E2F4: 419A00DC  beq cr6, 0x82b9e3d0
	if ctx.cr[6].eq {
	pc = 0x82B9E3D0; continue 'dispatch;
	}
	// 82B9E2F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B9E2FC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82B9E300: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82B9E304: 7F6AF030  slw r10, r27, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[27].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E308: 7D6BF030  slw r11, r11, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E30C: 7D6BE038  and r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[28].u64;
	// 82B9E310: 7F69F830  slw r9, r27, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[27].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E314: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9E318: 7D4AE038  and r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[28].u64;
	// 82B9E31C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9E320: 7D29D038  and r9, r9, r26
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[26].u64;
	// 82B9E324: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 82B9E328: 69680001  xori r8, r11, 1
	ctx.r[8].u64 = ctx.r[11].u64 ^ 1;
	// 82B9E32C: 7D2B0034  cntlzw r11, r9
	ctx.r[11].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82B9E330: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82B9E334: 5569DFFE  rlwinm r9, r11, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9E338: 554B003C  rlwinm r11, r10, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9E33C: 5508063F  clrlwi. r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82B9E340: 692A0001  xori r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u64 ^ 1;
	// 82B9E344: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82B9E348: 41820008  beq 0x82b9e350
	if ctx.cr[0].eq {
	pc = 0x82B9E350; continue 'dispatch;
	}
	// 82B9E34C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82B9E350: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9E354: 41820008  beq 0x82b9e35c
	if ctx.cr[0].eq {
	pc = 0x82B9E35C; continue 'dispatch;
	}
	// 82B9E358: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82B9E35C: 81580004  lwz r10, 4(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E360: 55490529  rlwinm. r9, r10, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9E364: 41820008  beq 0x82b9e36c
	if ctx.cr[0].eq {
	pc = 0x82B9E36C; continue 'dispatch;
	}
	// 82B9E368: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82B9E36C: 554A0426  rlwinm r10, r10, 0, 0x10, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9E370: 2B0A5000  cmplwi cr6, r10, 0x5000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 20480 as u32, &mut ctx.xer);
	// 82B9E374: 409A0008  bne cr6, 0x82b9e37c
	if !ctx.cr[6].eq {
	pc = 0x82B9E37C; continue 'dispatch;
	}
	// 82B9E378: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82B9E37C: 7D6AB038  and r10, r11, r22
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[22].u64;
	// 82B9E380: 554A077D  rlwinm. r10, r10, 0, 0x1d, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9E384: 4182002C  beq 0x82b9e3b0
	if ctx.cr[0].eq {
	pc = 0x82B9E3B0; continue 'dispatch;
	}
	// 82B9E388: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E38C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82B9E390: 80C10104  lwz r6, 0x104(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82B9E394: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82B9E398: 554A053E  clrlwi r10, r10, 0x14
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82B9E39C: 7C8AFA14  add r4, r10, r31
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82B9E3A0: 7EE903A6  mtctr r23
	ctx.ctr.u64 = ctx.r[23].u64;
	// 82B9E3A4: 4E800421  bctrl
	ctx.lr = 0x82B9E3A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9E3A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E3AC: 41800024  blt 0x82b9e3d0
	if ctx.cr[0].lt {
	pc = 0x82B9E3D0; continue 'dispatch;
	}
	// 82B9E3B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82B9E3B4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82B9E3B8: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82B9E3BC: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82B9E3C0: 4198FF40  blt cr6, 0x82b9e300
	if ctx.cr[6].lt {
	pc = 0x82B9E300; continue 'dispatch;
	}
	// 82B9E3C4: 4800000C  b 0x82b9e3d0
	pc = 0x82B9E3D0; continue 'dispatch;
	// 82B9E3C8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82B9E3CC: 60634005  ori r3, r3, 0x4005
	ctx.r[3].u64 = ctx.r[3].u64 | 16389;
	// 82B9E3D0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82B9E3D4: 4810B06C  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9E3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9E3D8 size=888
    let mut pc: u32 = 0x82B9E3D8;
    'dispatch: loop {
        match pc {
            0x82B9E3D8 => {
    //   block [0x82B9E3D8..0x82B9E750)
	// 82B9E3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9E3DC: 4810AFF5  bl 0x82ca93d0
	ctx.lr = 0x82B9E3E0;
	sub_82CA93D0(ctx, base);
	// 82B9E3E0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9E3E4: 7D545378  mr r20, r10
	ctx.r[20].u64 = ctx.r[10].u64;
	// 82B9E3E8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82B9E3EC: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82B9E3F0: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82B9E3F4: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82B9E3F8: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82B9E3FC: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82B9E400: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 82B9E404: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82B9E408: 7D0E4378  mr r14, r8
	ctx.r[14].u64 = ctx.r[8].u64;
	// 82B9E40C: 7D354B78  mr r21, r9
	ctx.r[21].u64 = ctx.r[9].u64;
	// 82B9E410: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9E414: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82B9E418: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82B9E41C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82B9E420: 3A20FFFF  li r17, -1
	ctx.r[17].s64 = -1;
	// 82B9E424: 39E0FFFF  li r15, -1
	ctx.r[15].s64 = -1;
	// 82B9E428: 3ACBFFFC  addi r22, r11, -4
	ctx.r[22].s64 = ctx.r[11].s64 + -4;
	// 82B9E42C: 3B2AFFFC  addi r25, r10, -4
	ctx.r[25].s64 = ctx.r[10].s64 + -4;
	// 82B9E430: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82B9E434: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82B9E438: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82B9E43C: 7D7F5B96  divwu r11, r31, r11
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[11].u32;
	// 82B9E440: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B9E444: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B9E448: 7D4BF850  subf r10, r11, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82B9E44C: 7D4A4B96  divwu r10, r10, r9
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[9].u32;
	// 82B9E450: 419A0014  beq cr6, 0x82b9e464
	if ctx.cr[6].eq {
	pc = 0x82B9E464; continue 'dispatch;
	}
	// 82B9E454: 7F1F8040  cmplw cr6, r31, r16
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[16].u32, &mut ctx.xer);
	// 82B9E458: 409802E8  bge cr6, 0x82b9e740
	if !ctx.cr[6].lt {
	pc = 0x82B9E740; continue 'dispatch;
	}
	// 82B9E45C: 7F8B9214  add r28, r11, r18
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82B9E460: 48000014  b 0x82b9e474
	pc = 0x82B9E474; continue 'dispatch;
	// 82B9E464: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9E468: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82B9E46C: 409802D4  bge cr6, 0x82b9e740
	if !ctx.cr[6].lt {
	pc = 0x82B9E740; continue 'dispatch;
	}
	// 82B9E470: 7F8B9A14  add r28, r11, r19
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82B9E474: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E478: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9E47C: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82B9E480: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82B9E484: 56A707FF  clrlwi. r7, r21, 0x1f
	ctx.r[7].u64 = ctx.r[21].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82B9E488: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82B9E48C: A17C0006  lhz r11, 6(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(6 as u32) ) } as u64;
	// 82B9E490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82B9E494: A17C0004  lhz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E498: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9E49C: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9E4A0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9E4A4: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82B9E4A8: A17C0008  lhz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9E4AC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82B9E4B0: 41820024  beq 0x82b9e4d4
	if ctx.cr[0].eq {
	pc = 0x82B9E4D4; continue 'dispatch;
	}
	// 82B9E4B4: 80C10194  lwz r6, 0x194(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(404 as u32) ) } as u64;
	// 82B9E4B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82B9E4BC: 7C9F4B96  divwu r4, r31, r9
	ctx.r[4].u32 = ctx.r[31].u32 / ctx.r[9].u32;
	// 82B9E4C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B9E4C4: 7E8903A6  mtctr r20
	ctx.ctr.u64 = ctx.r[20].u64;
	// 82B9E4C8: 4E800421  bctrl
	ctx.lr = 0x82B9E4CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9E4CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E4D0: 41800278  blt 0x82b9e748
	if ctx.cr[0].lt {
	pc = 0x82B9E748; continue 'dispatch;
	}
	// 82B9E4D4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E4D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82B9E4DC: 556AA73E  rlwinm r10, r11, 0x14, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9E4E0: 2B0A000F  cmplwi cr6, r10, 0xf
	ctx.cr[6].compare_u32(ctx.r[10].u32, 15 as u32, &mut ctx.xer);
	// 82B9E4E4: 419901E4  bgt cr6, 0x82b9e6c8
	if ctx.cr[6].gt {
	pc = 0x82B9E6C8; continue 'dispatch;
	}
	// 82B9E4E8: 3D80820A  lis r12, -0x7df6
	ctx.r[12].s64 = -2113273856;
	// 82B9E4EC: 398C0300  addi r12, r12, 0x300
	ctx.r[12].s64 = ctx.r[12].s64 + 768;
	// 82B9E4F0: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82B9E4F4: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82B9E4F8: 3D8082BA  lis r12, -0x7d46
	ctx.r[12].s64 = -2101739520;
	// 82B9E4FC: 398CE510  addi r12, r12, -0x1af0
	ctx.r[12].s64 = ctx.r[12].s64 + -6896;
	// 82B9E500: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82B9E504: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82B9E508: 60000000  nop
	// 82B9E50C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82B9E510: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82B9E514: 480001B0  b 0x82b9e6c4
	pc = 0x82B9E6C4; continue 'dispatch;
	// 82B9E518: 556ADEFA  rlwinm r10, r11, 0x1b, 0x1b, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9E51C: 5569F6FE  rlwinm r9, r11, 0x1e, 0x1b, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B9E520: 556BB7FE  rlwinm r11, r11, 0x16, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82B9E524: 7F494830  slw r9, r26, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[26].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E528: 7D4AB82E  lwzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82B9E52C: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82B9E530: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82B9E534: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9E538: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82B9E53C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9E540: 409A0184  bne cr6, 0x82b9e6c4
	if !ctx.cr[6].eq {
	pc = 0x82B9E6C4; continue 'dispatch;
	}
	// 82B9E544: 4BFFFFCC  b 0x82b9e510
	pc = 0x82B9E510; continue 'dispatch;
	// 82B9E548: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E54C: 5549967A  rlwinm r9, r10, 0x12, 0x19, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 82B9E550: 7D29702E  lwzx r9, r9, r14
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82B9E554: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9E558: 418200C4  beq 0x82b9e61c
	if ctx.cr[0].eq {
	pc = 0x82B9E61C; continue 'dispatch;
	}
	// 82B9E55C: 39EF0001  addi r15, r15, 1
	ctx.r[15].s64 = ctx.r[15].s64 + 1;
	// 82B9E560: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 82B9E564: 2F0F0004  cmpwi cr6, r15, 4
	ctx.cr[6].compare_i32(ctx.r[15].s32, 4, &mut ctx.xer);
	// 82B9E568: 4198000C  blt cr6, 0x82b9e574
	if ctx.cr[6].lt {
	pc = 0x82B9E574; continue 'dispatch;
	}
	// 82B9E56C: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82B9E570: 48000158  b 0x82b9e6c8
	pc = 0x82B9E6C8; continue 'dispatch;
	// 82B9E574: 91190000  stw r8, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82B9E578: 4800014C  b 0x82b9e6c4
	pc = 0x82B9E6C4; continue 'dispatch;
	// 82B9E57C: 2F0F0000  cmpwi cr6, r15, 0
	ctx.cr[6].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 82B9E580: 4198FFEC  blt cr6, 0x82b9e56c
	if ctx.cr[6].lt {
	pc = 0x82B9E56C; continue 'dispatch;
	}
	// 82B9E584: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E588: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E58C: 5527967A  rlwinm r7, r9, 0x12, 0x19, 0x1d
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00003FFFu64;
	// 82B9E590: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9E594: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B9E598: 7CE7702E  lwzx r7, r7, r14
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82B9E59C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82B9E5A0: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82B9E5A4: 4098000C  bge cr6, 0x82b9e5b0
	if !ctx.cr[6].lt {
	pc = 0x82B9E5B0; continue 'dispatch;
	}
	// 82B9E5A8: 552A04FE  clrlwi r10, r9, 0x13
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00001FFFu64;
	// 82B9E5AC: 48000074  b 0x82b9e620
	pc = 0x82B9E620; continue 'dispatch;
	// 82B9E5B0: 39EFFFFF  addi r15, r15, -1
	ctx.r[15].s64 = ctx.r[15].s64 + -1;
	// 82B9E5B4: 3B39FFFC  addi r25, r25, -4
	ctx.r[25].s64 = ctx.r[25].s64 + -4;
	// 82B9E5B8: 4800010C  b 0x82b9e6c4
	pc = 0x82B9E6C4; continue 'dispatch;
	// 82B9E5BC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E5C0: 554904A5  rlwinm. r9, r10, 0, 0x12, 0x12
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9E5C4: 40820038  bne 0x82b9e5fc
	if !ctx.cr[0].eq {
	pc = 0x82B9E5FC; continue 'dispatch;
	}
	// 82B9E5C8: 55490463  rlwinm. r9, r10, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9E5CC: 408200F8  bne 0x82b9e6c4
	if !ctx.cr[0].eq {
	pc = 0x82B9E6C4; continue 'dispatch;
	}
	// 82B9E5D0: 5569DEFA  rlwinm r9, r11, 0x1b, 0x1b, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9E5D4: 5567F6FE  rlwinm r7, r11, 0x1e, 0x1b, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B9E5D8: 5566B7FE  rlwinm r6, r11, 0x16, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82B9E5DC: 7F473830  slw r7, r26, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[26].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E5E0: 7D29B82E  lwzx r9, r9, r23
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82B9E5E4: 7CE94838  and r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82B9E5E8: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82B9E5EC: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9E5F0: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 82B9E5F4: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82B9E5F8: 409A00CC  bne cr6, 0x82b9e6c4
	if !ctx.cr[6].eq {
	pc = 0x82B9E6C4; continue 'dispatch;
	}
	// 82B9E5FC: 3A310001  addi r17, r17, 1
	ctx.r[17].s64 = ctx.r[17].s64 + 1;
	// 82B9E600: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 82B9E604: 2F110004  cmpwi cr6, r17, 4
	ctx.cr[6].compare_i32(ctx.r[17].s32, 4, &mut ctx.xer);
	// 82B9E608: 4098FF64  bge cr6, 0x82b9e56c
	if !ctx.cr[6].lt {
	pc = 0x82B9E56C; continue 'dispatch;
	}
	// 82B9E60C: 57A9801E  slwi r9, r29, 0x10
	ctx.r[9].u32 = ctx.r[29].u32.wrapping_shl(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82B9E610: 38FF0006  addi r7, r31, 6
	ctx.r[7].s64 = ctx.r[31].s64 + 6;
	// 82B9E614: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82B9E618: 91360000  stw r9, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B9E61C: 554A04FE  clrlwi r10, r10, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00001FFFu64;
	// 82B9E620: 557DAFFE  rlwinm r29, r11, 0x15, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 82B9E624: 1FEA0006  mulli r31, r10, 6
	ctx.r[31].s64 = ctx.r[10].s64 * 6;
	// 82B9E628: 480000A0  b 0x82b9e6c8
	pc = 0x82B9E6C8; continue 'dispatch;
	// 82B9E62C: 2F110000  cmpwi cr6, r17, 0
	ctx.cr[6].compare_i32(ctx.r[17].s32, 0, &mut ctx.xer);
	// 82B9E630: 4198FF3C  blt cr6, 0x82b9e56c
	if ctx.cr[6].lt {
	pc = 0x82B9E56C; continue 'dispatch;
	}
	// 82B9E634: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E638: 3A31FFFF  addi r17, r17, -1
	ctx.r[17].s64 = ctx.r[17].s64 + -1;
	// 82B9E63C: 3AD6FFFC  addi r22, r22, -4
	ctx.r[22].s64 = ctx.r[22].s64 + -4;
	// 82B9E640: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9E644: 557D843E  srwi r29, r11, 0x10
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82B9E648: 48000080  b 0x82b9e6c8
	pc = 0x82B9E6C8; continue 'dispatch;
	// 82B9E64C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E650: 554904A5  rlwinm. r9, r10, 0, 0x12, 0x12
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9E654: 4082FFC8  bne 0x82b9e61c
	if !ctx.cr[0].eq {
	pc = 0x82B9E61C; continue 'dispatch;
	}
	// 82B9E658: 55490463  rlwinm. r9, r10, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9E65C: 40820068  bne 0x82b9e6c4
	if !ctx.cr[0].eq {
	pc = 0x82B9E6C4; continue 'dispatch;
	}
	// 82B9E660: 5569DEFA  rlwinm r9, r11, 0x1b, 0x1b, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9E664: 5567F6FE  rlwinm r7, r11, 0x1e, 0x1b, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B9E668: 5566B7FE  rlwinm r6, r11, 0x16, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82B9E66C: 7F473830  slw r7, r26, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[26].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E670: 7D29B82E  lwzx r9, r9, r23
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82B9E674: 7CE94838  and r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82B9E678: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82B9E67C: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82B9E680: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 82B9E684: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82B9E688: 409A003C  bne cr6, 0x82b9e6c4
	if !ctx.cr[6].eq {
	pc = 0x82B9E6C4; continue 'dispatch;
	}
	// 82B9E68C: 4BFFFF90  b 0x82b9e61c
	pc = 0x82B9E61C; continue 'dispatch;
	// 82B9E690: 556ADEFA  rlwinm r10, r11, 0x1b, 0x1b, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9E694: 5569F6FE  rlwinm r9, r11, 0x1e, 0x1b, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82B9E698: 556BB7FE  rlwinm r11, r11, 0x16, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82B9E69C: 7F494830  slw r9, r26, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[26].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E6A0: 7D4AB82E  lwzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82B9E6A4: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82B9E6A8: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82B9E6AC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82B9E6B0: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82B9E6B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9E6B8: 409A000C  bne cr6, 0x82b9e6c4
	if !ctx.cr[6].eq {
	pc = 0x82B9E6C4; continue 'dispatch;
	}
	// 82B9E6BC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82B9E6C0: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82B9E6C4: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82B9E6C8: 550B063F  clrlwi. r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E6CC: 41820038  beq 0x82b9e704
	if ctx.cr[0].eq {
	pc = 0x82B9E704; continue 'dispatch;
	}
	// 82B9E6D0: 81610194  lwz r11, 0x194(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(404 as u32) ) } as u64;
	// 82B9E6D4: 7E8AA378  mr r10, r20
	ctx.r[10].u64 = ctx.r[20].u64;
	// 82B9E6D8: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 82B9E6DC: 7E088378  mr r8, r16
	ctx.r[8].u64 = ctx.r[16].u64;
	// 82B9E6E0: 7E479378  mr r7, r18
	ctx.r[7].u64 = ctx.r[18].u64;
	// 82B9E6E4: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82B9E6E8: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 82B9E6EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9E6F0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82B9E6F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82B9E6F8: 4BFFFB49  bl 0x82b9e240
	ctx.lr = 0x82B9E6FC;
	sub_82B9E240(ctx, base);
	// 82B9E6FC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E700: 41800048  blt 0x82b9e748
	if ctx.cr[0].lt {
	pc = 0x82B9E748; continue 'dispatch;
	}
	// 82B9E704: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9E708: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9E70C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9E710: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82B9E714: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82B9E718: 514B801E  rlwimi r11, r10, 0x10, 0, 0xf
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF0000FFFF);
	// 82B9E71C: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9E720: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82B9E724: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9E728: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9E72C: 556B801E  slwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9E730: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82B9E734: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82B9E738: 4182FCFC  beq 0x82b9e434
	if ctx.cr[0].eq {
	pc = 0x82B9E434; continue 'dispatch;
	}
	// 82B9E73C: 4800000C  b 0x82b9e748
	pc = 0x82B9E748; continue 'dispatch;
	// 82B9E740: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82B9E744: 60634005  ori r3, r3, 0x4005
	ctx.r[3].u64 = ctx.r[3].u64 | 16389;
	// 82B9E748: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82B9E74C: 4810ACD4  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9E750 size=372
    let mut pc: u32 = 0x82B9E750;
    'dispatch: loop {
        match pc {
            0x82B9E750 => {
    //   block [0x82B9E750..0x82B9E8C4)
	// 82B9E750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9E754: 4810AC8D  bl 0x82ca93e0
	ctx.lr = 0x82B9E758;
	sub_82CA93D0(ctx, base);
	// 82B9E758: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9E75C: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 82B9E760: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82B9E764: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82B9E768: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82B9E76C: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82B9E770: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 82B9E774: 578B0739  rlwinm. r11, r28, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9E77C: 41820018  beq 0x82b9e794
	if ctx.cr[0].eq {
	pc = 0x82B9E794; continue 'dispatch;
	}
	// 82B9E780: 81610154  lwz r11, 0x154(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) } as u64;
	// 82B9E784: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82B9E788: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82B9E78C: 4BFFFC4D  bl 0x82b9e3d8
	ctx.lr = 0x82B9E790;
	sub_82B9E3D8(ctx, base);
	// 82B9E790: 4800012C  b 0x82b9e8bc
	pc = 0x82B9E8BC; continue 'dispatch;
	// 82B9E794: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82B9E798: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82B9E79C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82B9E7A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82B9E7A4: 419A0118  beq cr6, 0x82b9e8bc
	if ctx.cr[6].eq {
	pc = 0x82B9E8BC; continue 'dispatch;
	}
	// 82B9E7A8: 82E10154  lwz r23, 0x154(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) } as u64;
	// 82B9E7AC: 579207FE  clrlwi r18, r28, 0x1f
	ctx.r[18].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	// 82B9E7B0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E7B4: 3A9E0004  addi r20, r30, 4
	ctx.r[20].s64 = ctx.r[30].s64 + 4;
	// 82B9E7B8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9E7BC: 3A7E0008  addi r19, r30, 8
	ctx.r[19].s64 = ctx.r[30].s64 + 8;
	// 82B9E7C0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E7C4: 5568843E  srwi r8, r11, 0x10
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9E7C8: 5547801E  slwi r7, r10, 0x10
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82B9E7CC: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9E7D0: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82B9E7D4: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9E7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82B9E7DC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82B9E7E0: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82B9E7E4: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 82B9E7E8: 91010078  stw r8, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u32 ) };
	// 82B9E7EC: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82B9E7F0: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 82B9E7F4: 419A002C  beq cr6, 0x82b9e820
	if ctx.cr[6].eq {
	pc = 0x82B9E820; continue 'dispatch;
	}
	// 82B9E7F8: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82B9E7FC: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82B9E800: 7D785B96  divwu r11, r24, r11
	ctx.r[11].u32 = ctx.r[24].u32 / ctx.r[11].u32;
	// 82B9E804: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82B9E808: 7C8BD214  add r4, r11, r26
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82B9E80C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82B9E810: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 82B9E814: 4E800421  bctrl
	ctx.lr = 0x82B9E818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9E818: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E81C: 418000A0  blt 0x82b9e8bc
	if ctx.cr[0].lt {
	pc = 0x82B9E8BC; continue 'dispatch;
	}
	// 82B9E820: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E824: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82B9E828: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82B9E82C: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9E830: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E834: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9E838: 41820034  beq 0x82b9e86c
	if ctx.cr[0].eq {
	pc = 0x82B9E86C; continue 'dispatch;
	}
	// 82B9E83C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82B9E840: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82B9E844: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82B9E848: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82B9E84C: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	// 82B9E850: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82B9E854: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82B9E858: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82B9E85C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9E860: 4BFFF9E1  bl 0x82b9e240
	ctx.lr = 0x82B9E864;
	sub_82B9E240(ctx, base);
	// 82B9E864: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E868: 41800054  blt 0x82b9e8bc
	if ctx.cr[0].lt {
	pc = 0x82B9E8BC; continue 'dispatch;
	}
	// 82B9E86C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82B9E870: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82B9E874: 2B1A0002  cmplwi cr6, r26, 2
	ctx.cr[6].compare_u32(ctx.r[26].u32, 2 as u32, &mut ctx.xer);
	// 82B9E878: 4198FF78  blt cr6, 0x82b9e7f0
	if ctx.cr[6].lt {
	pc = 0x82B9E7F0; continue 'dispatch;
	}
	// 82B9E87C: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82B9E880: 3B18000C  addi r24, r24, 0xc
	ctx.r[24].s64 = ctx.r[24].s64 + 12;
	// 82B9E884: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82B9E888: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82B9E88C: 5568843E  srwi r8, r11, 0x10
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82B9E890: 80E10074  lwz r7, 0x74(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82B9E894: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82B9E898: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82B9E89C: 5167801E  rlwimi r7, r11, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82B9E8A0: 7D4B4378  or r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82B9E8A4: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82B9E8A8: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82B9E8AC: 90F40000  stw r7, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82B9E8B0: 7F183040  cmplw cr6, r24, r6
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82B9E8B4: 91730000  stw r11, 0(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9E8B8: 4198FEF8  blt cr6, 0x82b9e7b0
	if ctx.cr[6].lt {
	pc = 0x82B9E7B0; continue 'dispatch;
	}
	// 82B9E8BC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82B9E8C0: 4810AB70  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9E8C8 size=640
    let mut pc: u32 = 0x82B9E8C8;
    'dispatch: loop {
        match pc {
            0x82B9E8C8 => {
    //   block [0x82B9E8C8..0x82B9EB48)
	// 82B9E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9E8CC: 4810AB0D  bl 0x82ca93d8
	ctx.lr = 0x82B9E8D0;
	sub_82CA93D0(ctx, base);
	// 82B9E8D0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9E8D4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82B9E8D8: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82B9E8DC: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82B9E8E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82B9E8E4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82B9E8E8: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E8EC: 3A370014  addi r17, r23, 0x14
	ctx.r[17].s64 = ctx.r[23].s64 + 20;
	// 82B9E8F0: 82570004  lwz r18, 4(r23)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E8F4: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82B9E8F8: 557007FE  clrlwi r16, r11, 0x1f
	ctx.r[16].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82B9E8FC: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E900: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9E904: 419A0220  beq cr6, 0x82b9eb24
	if ctx.cr[6].eq {
	pc = 0x82B9EB24; continue 'dispatch;
	}
	// 82B9E908: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82B9E90C: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82B9E910: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 82B9E914: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82B9E918: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9E91C: 7E8BFA14  add r20, r11, r31
	ctx.r[20].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82B9E920: 480001FC  b 0x82b9eb1c
	pc = 0x82B9EB1C; continue 'dispatch;
	// 82B9E924: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82B9E928: A15F0000  lhz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E92C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82B9E930: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82B9E934: 40820014  bne 0x82b9e948
	if !ctx.cr[0].eq {
	pc = 0x82B9E948; continue 'dispatch;
	}
	// 82B9E938: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 82B9E93C: 2B130003  cmplwi cr6, r19, 3
	ctx.cr[6].compare_u32(ctx.r[19].u32, 3 as u32, &mut ctx.xer);
	// 82B9E940: 409801E4  bge cr6, 0x82b9eb24
	if !ctx.cr[6].lt {
	pc = 0x82B9EB24; continue 'dispatch;
	}
	// 82B9E944: 480001D8  b 0x82b9eb1c
	pc = 0x82B9EB1C; continue 'dispatch;
	// 82B9E948: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 82B9E94C: 409A00A0  bne cr6, 0x82b9e9ec
	if !ctx.cr[6].eq {
	pc = 0x82B9E9EC; continue 'dispatch;
	}
	// 82B9E950: 5569073F  clrlwi. r9, r11, 0x1c
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82B9E954: 408201E4  bne 0x82b9eb38
	if !ctx.cr[0].eq {
	pc = 0x82B9EB38; continue 'dispatch;
	}
	// 82B9E958: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E95C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82B9E960: 7D299214  add r9, r9, r18
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[18].u64;
	// 82B9E964: 7FC9BA14  add r30, r9, r23
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[23].u64;
	// 82B9E968: 555C043E  clrlwi r28, r10, 0x10
	ctx.r[28].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9E96C: 2B1C0200  cmplwi cr6, r28, 0x200
	ctx.cr[6].compare_u32(ctx.r[28].u32, 512 as u32, &mut ctx.xer);
	// 82B9E970: 409801C8  bge cr6, 0x82b9eb38
	if !ctx.cr[6].lt {
	pc = 0x82B9EB38; continue 'dispatch;
	}
	// 82B9E974: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82B9E978: 2F100000  cmpwi cr6, r16, 0
	ctx.cr[6].compare_i32(ctx.r[16].s32, 0, &mut ctx.xer);
	// 82B9E97C: 419A0008  beq cr6, 0x82b9e984
	if ctx.cr[6].eq {
	pc = 0x82B9E984; continue 'dispatch;
	}
	// 82B9E980: 38C6FF00  addi r6, r6, -0x100
	ctx.r[6].s64 = ctx.r[6].s64 + -256;
	// 82B9E984: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9E988: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82B9E98C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82B9E990: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 82B9E994: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9E998: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82B9E99C: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9E9A0: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82B9E9A4: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82B9E9A8: 90890008  stw r4, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82B9E9AC: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9E9B0: 9069000C  stw r3, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82B9E9B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82B9E9B8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82B9E9BC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82B9E9C0: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82B9E9C4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82B9E9C8: 7EA903A6  mtctr r21
	ctx.ctr.u64 = ctx.r[21].u64;
	// 82B9E9CC: 4E800421  bctrl
	ctx.lr = 0x82B9E9D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9E9D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9E9D4: 4180016C  blt 0x82b9eb40
	if ctx.cr[0].lt {
	pc = 0x82B9EB40; continue 'dispatch;
	}
	// 82B9E9D8: 395C0001  addi r10, r28, 1
	ctx.r[10].s64 = ctx.r[28].s64 + 1;
	// 82B9E9DC: 57AB043F  clrlwi. r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9E9E0: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9E9E4: 4082FF84  bne 0x82b9e968
	if !ctx.cr[0].eq {
	pc = 0x82B9E968; continue 'dispatch;
	}
	// 82B9E9E8: 48000134  b 0x82b9eb1c
	pc = 0x82B9EB1C; continue 'dispatch;
	// 82B9E9EC: 2B130001  cmplwi cr6, r19, 1
	ctx.cr[6].compare_u32(ctx.r[19].u32, 1 as u32, &mut ctx.xer);
	// 82B9E9F0: 409A008C  bne cr6, 0x82b9ea7c
	if !ctx.cr[6].eq {
	pc = 0x82B9EA7C; continue 'dispatch;
	}
	// 82B9E9F4: 555E043E  clrlwi r30, r10, 0x10
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9E9F8: 2B1E2320  cmplwi cr6, r30, 0x2320
	ctx.cr[6].compare_u32(ctx.r[30].u32, 8992 as u32, &mut ctx.xer);
	// 82B9E9FC: 4198013C  blt cr6, 0x82b9eb38
	if ctx.cr[6].lt {
	pc = 0x82B9EB38; continue 'dispatch;
	}
	// 82B9EA00: 2B1E23A0  cmplwi cr6, r30, 0x23a0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 9120 as u32, &mut ctx.xer);
	// 82B9EA04: 40980134  bge cr6, 0x82b9eb38
	if !ctx.cr[6].lt {
	pc = 0x82B9EB38; continue 'dispatch;
	}
	// 82B9EA08: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EA0C: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 82B9EA10: 393EDCE0  addi r9, r30, -0x2320
	ctx.r[9].s64 = ctx.r[30].s64 + -8992;
	// 82B9EA14: 5547063E  clrlwi r7, r10, 0x18
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9EA18: 5548863E  rlwinm r8, r10, 0x10, 0x18, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9EA1C: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82B9EA20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82B9EA24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82B9EA28: 554AC63E  rlwinm r10, r10, 0x18, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82B9EA2C: 90E1006C  stw r7, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[7].u32 ) };
	// 82B9EA30: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82B9EA34: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82B9EA38: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82B9EA3C: 91010068  stw r8, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82B9EA40: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9EA44: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82B9EA48: 5526F0BE  srwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82B9EA4C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82B9EA50: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82B9EA54: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82B9EA58: 7EA903A6  mtctr r21
	ctx.ctr.u64 = ctx.r[21].u64;
	// 82B9EA5C: 4E800421  bctrl
	ctx.lr = 0x82B9EA60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9EA60: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9EA64: 418000DC  blt 0x82b9eb40
	if ctx.cr[0].lt {
	pc = 0x82B9EB40; continue 'dispatch;
	}
	// 82B9EA68: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 82B9EA6C: 57AB043F  clrlwi. r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9EA70: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9EA74: 4082FF80  bne 0x82b9e9f4
	if !ctx.cr[0].eq {
	pc = 0x82B9E9F4; continue 'dispatch;
	}
	// 82B9EA78: 480000A4  b 0x82b9eb1c
	pc = 0x82B9EB1C; continue 'dispatch;
	// 82B9EA7C: 555B043E  clrlwi r27, r10, 0x10
	ctx.r[27].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9EA80: 2B1B2300  cmplwi cr6, r27, 0x2300
	ctx.cr[6].compare_u32(ctx.r[27].u32, 8960 as u32, &mut ctx.xer);
	// 82B9EA84: 419800B4  blt cr6, 0x82b9eb38
	if ctx.cr[6].lt {
	pc = 0x82B9EB38; continue 'dispatch;
	}
	// 82B9EA88: 2B1B2320  cmplwi cr6, r27, 0x2320
	ctx.cr[6].compare_u32(ctx.r[27].u32, 8992 as u32, &mut ctx.xer);
	// 82B9EA8C: 409800AC  bge cr6, 0x82b9eb38
	if !ctx.cr[6].lt {
	pc = 0x82B9EB38; continue 'dispatch;
	}
	// 82B9EA90: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 82B9EA94: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EA98: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82B9EA9C: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EAA0: 393BDD00  addi r9, r27, -0x2300
	ctx.r[9].s64 = ctx.r[27].s64 + -8960;
	// 82B9EAA4: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82B9EAA8: 553D1834  rlwinm r29, r9, 3, 0, 0x1a
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x1FFFFFFFu64;
	// 82B9EAAC: 5579043E  clrlwi r25, r11, 0x10
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82B9EAB0: 3BEA0004  addi r31, r10, 4
	ctx.r[31].s64 = ctx.r[10].s64 + 4;
	// 82B9EAB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B9EAB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82B9EABC: 7D6BF030  slw r11, r11, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82B9EAC0: 7D6AD039  and. r10, r11, r26
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[26].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82B9EAC4: 4082003C  bne 0x82b9eb00
	if !ctx.cr[0].eq {
	pc = 0x82B9EB00; continue 'dispatch;
	}
	// 82B9EAC8: 7D6BE038  and r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[28].u64;
	// 82B9EACC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82B9EAD0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82B9EAD4: 7CDEEA14  add r6, r30, r29
	ctx.r[6].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82B9EAD8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82B9EADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9EAE0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82B9EAE4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82B9EAE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82B9EAEC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82B9EAF0: 7EA903A6  mtctr r21
	ctx.ctr.u64 = ctx.r[21].u64;
	// 82B9EAF4: 4E800421  bctrl
	ctx.lr = 0x82B9EAF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82B9EAF8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82B9EAFC: 41800044  blt 0x82b9eb40
	if ctx.cr[0].lt {
	pc = 0x82B9EB40; continue 'dispatch;
	}
	// 82B9EB00: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82B9EB04: 2B1E0020  cmplwi cr6, r30, 0x20
	ctx.cr[6].compare_u32(ctx.r[30].u32, 32 as u32, &mut ctx.xer);
	// 82B9EB08: 4198FFB0  blt cr6, 0x82b9eab8
	if ctx.cr[6].lt {
	pc = 0x82B9EAB8; continue 'dispatch;
	}
	// 82B9EB0C: 395B0004  addi r10, r27, 4
	ctx.r[10].s64 = ctx.r[27].s64 + 4;
	// 82B9EB10: 572B043F  clrlwi. r11, r25, 0x10
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9EB14: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82B9EB18: 4082FF64  bne 0x82b9ea7c
	if !ctx.cr[0].eq {
	pc = 0x82B9EA7C; continue 'dispatch;
	}
	// 82B9EB1C: 7F1FA040  cmplw cr6, r31, r20
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[20].u32, &mut ctx.xer);
	// 82B9EB20: 4198FE04  blt cr6, 0x82b9e924
	if ctx.cr[6].lt {
	pc = 0x82B9E924; continue 'dispatch;
	}
	// 82B9EB24: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82B9EB28: 3A310008  addi r17, r17, 8
	ctx.r[17].s64 = ctx.r[17].s64 + 8;
	// 82B9EB2C: 2B180002  cmplwi cr6, r24, 2
	ctx.cr[6].compare_u32(ctx.r[24].u32, 2 as u32, &mut ctx.xer);
	// 82B9EB30: 4198FDCC  blt cr6, 0x82b9e8fc
	if ctx.cr[6].lt {
	pc = 0x82B9E8FC; continue 'dispatch;
	}
	// 82B9EB34: 4800000C  b 0x82b9eb40
	pc = 0x82B9EB40; continue 'dispatch;
	// 82B9EB38: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82B9EB3C: 60634005  ori r3, r3, 0x4005
	ctx.r[3].u64 = ctx.r[3].u64 | 16389;
	// 82B9EB40: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82B9EB44: 4810A8E4  b 0x82ca9428
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9EB48 size=8
    let mut pc: u32 = 0x82B9EB48;
    'dispatch: loop {
        match pc {
            0x82B9EB48 => {
    //   block [0x82B9EB48..0x82B9EB50)
	// 82B9EB48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82B9EB4C: 4BFFFD7C  b 0x82b9e8c8
	sub_82B9E8C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9EB50 size=216
    let mut pc: u32 = 0x82B9EB50;
    'dispatch: loop {
        match pc {
            0x82B9EB50 => {
    //   block [0x82B9EB50..0x82B9EC28)
	// 82B9EB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9EB54: 4810A8B1  bl 0x82ca9404
	ctx.lr = 0x82B9EB58;
	sub_82CA93D0(ctx, base);
	// 82B9EB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9EB5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9EB60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82B9EB64: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82B9EB68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EB6C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9EB70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9EB74: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9EB78: 40990068  ble cr6, 0x82b9ebe0
	if !ctx.cr[6].gt {
	pc = 0x82B9EBE0; continue 'dispatch;
	}
	// 82B9EB7C: 555E083C  slwi r30, r10, 1
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B9EB80: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9EB84: 40980008  bge cr6, 0x82b9eb8c
	if !ctx.cr[6].lt {
	pc = 0x82B9EB8C; continue 'dispatch;
	}
	// 82B9EB88: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82B9EB8C: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9EB90: 1C7E000C  mulli r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 * 12;
	// 82B9EB94: 4B68DE3D  bl 0x8222c9d0
	ctx.lr = 0x82B9EB98;
	sub_8222C9D0(ctx, base);
	// 82B9EB98: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9EB9C: 40820014  bne 0x82b9ebb0
	if !ctx.cr[0].eq {
	pc = 0x82B9EBB0; continue 'dispatch;
	}
	// 82B9EBA0: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 82B9EBA4: 616B000E  ori r11, r11, 0xe
	ctx.r[11].u64 = ctx.r[11].u64 | 14;
	// 82B9EBA8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9EBAC: 48000074  b 0x82b9ec20
	pc = 0x82B9EC20; continue 'dispatch;
	// 82B9EBB0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EBB4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B9EBB8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9EBBC: 419A0020  beq cr6, 0x82b9ebdc
	if ctx.cr[6].eq {
	pc = 0x82B9EBDC; continue 'dispatch;
	}
	// 82B9EBC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EBC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9EBC8: 1CAB000C  mulli r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 * 12;
	// 82B9EBCC: 4810A8B5  bl 0x82ca9480
	ctx.lr = 0x82B9EBD0;
	sub_82CA9480(ctx, base);
	// 82B9EBD0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EBD4: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9EBD8: 4B6636E1  bl 0x822022b8
	ctx.lr = 0x82B9EBDC;
	sub_822022B8(ctx, base);
	// 82B9EBDC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9EBE0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EBE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9EBE8: 41980038  blt cr6, 0x82b9ec20
	if ctx.cr[6].lt {
	pc = 0x82B9EC20; continue 'dispatch;
	}
	// 82B9EBEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EBF0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EBF4: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EBF8: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82B9EBFC: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82B9EC00: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EC04: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9EC08: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82B9EC0C: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9EC10: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82B9EC14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EC18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9EC1C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9EC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9EC24: 4810A830  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9EC28 size=208
    let mut pc: u32 = 0x82B9EC28;
    'dispatch: loop {
        match pc {
            0x82B9EC28 => {
    //   block [0x82B9EC28..0x82B9ECF8)
	// 82B9EC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9EC2C: 4810A7D9  bl 0x82ca9404
	ctx.lr = 0x82B9EC30;
	sub_82CA93D0(ctx, base);
	// 82B9EC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9EC34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9EC38: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82B9EC3C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82B9EC40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EC44: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9EC48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9EC4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9EC50: 40990068  ble cr6, 0x82b9ecb8
	if !ctx.cr[6].gt {
	pc = 0x82B9ECB8; continue 'dispatch;
	}
	// 82B9EC54: 555E083C  slwi r30, r10, 1
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B9EC58: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9EC5C: 40980008  bge cr6, 0x82b9ec64
	if !ctx.cr[6].lt {
	pc = 0x82B9EC64; continue 'dispatch;
	}
	// 82B9EC60: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82B9EC64: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9EC68: 57C31838  slwi r3, r30, 3
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B9EC6C: 4B68DD65  bl 0x8222c9d0
	ctx.lr = 0x82B9EC70;
	sub_8222C9D0(ctx, base);
	// 82B9EC70: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9EC74: 40820014  bne 0x82b9ec88
	if !ctx.cr[0].eq {
	pc = 0x82B9EC88; continue 'dispatch;
	}
	// 82B9EC78: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 82B9EC7C: 616B000E  ori r11, r11, 0xe
	ctx.r[11].u64 = ctx.r[11].u64 | 14;
	// 82B9EC80: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9EC84: 4800006C  b 0x82b9ecf0
	pc = 0x82B9ECF0; continue 'dispatch;
	// 82B9EC88: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EC8C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B9EC90: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9EC94: 419A0020  beq cr6, 0x82b9ecb4
	if ctx.cr[6].eq {
	pc = 0x82B9ECB4; continue 'dispatch;
	}
	// 82B9EC98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EC9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9ECA0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9ECA4: 4810A7DD  bl 0x82ca9480
	ctx.lr = 0x82B9ECA8;
	sub_82CA9480(ctx, base);
	// 82B9ECA8: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9ECAC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ECB0: 4B663609  bl 0x822022b8
	ctx.lr = 0x82B9ECB4;
	sub_822022B8(ctx, base);
	// 82B9ECB4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9ECB8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ECBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9ECC0: 41980030  blt cr6, 0x82b9ecf0
	if ctx.cr[6].lt {
	pc = 0x82B9ECF0; continue 'dispatch;
	}
	// 82B9ECC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ECC8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ECCC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9ECD0: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ECD4: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82B9ECD8: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82B9ECDC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ECE0: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9ECE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ECE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9ECEC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9ECF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9ECF4: 4810A760  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9ECF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9ECF8 size=196
    let mut pc: u32 = 0x82B9ECF8;
    'dispatch: loop {
        match pc {
            0x82B9ECF8 => {
    //   block [0x82B9ECF8..0x82B9EDBC)
	// 82B9ECF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9ECFC: 4810A709  bl 0x82ca9404
	ctx.lr = 0x82B9ED00;
	sub_82CA93D0(ctx, base);
	// 82B9ED00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9ED04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9ED08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82B9ED0C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82B9ED10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ED14: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9ED18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9ED1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82B9ED20: 40990068  ble cr6, 0x82b9ed88
	if !ctx.cr[6].gt {
	pc = 0x82B9ED88; continue 'dispatch;
	}
	// 82B9ED24: 555E083C  slwi r30, r10, 1
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B9ED28: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9ED2C: 40980008  bge cr6, 0x82b9ed34
	if !ctx.cr[6].lt {
	pc = 0x82B9ED34; continue 'dispatch;
	}
	// 82B9ED30: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82B9ED34: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9ED38: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B9ED3C: 4B68DC95  bl 0x8222c9d0
	ctx.lr = 0x82B9ED40;
	sub_8222C9D0(ctx, base);
	// 82B9ED40: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9ED44: 40820014  bne 0x82b9ed58
	if !ctx.cr[0].eq {
	pc = 0x82B9ED58; continue 'dispatch;
	}
	// 82B9ED48: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 82B9ED4C: 616B000E  ori r11, r11, 0xe
	ctx.r[11].u64 = ctx.r[11].u64 | 14;
	// 82B9ED50: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9ED54: 48000060  b 0x82b9edb4
	pc = 0x82B9EDB4; continue 'dispatch;
	// 82B9ED58: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ED5C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B9ED60: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9ED64: 419A0020  beq cr6, 0x82b9ed84
	if ctx.cr[6].eq {
	pc = 0x82B9ED84; continue 'dispatch;
	}
	// 82B9ED68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ED6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9ED70: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9ED74: 4810A70D  bl 0x82ca9480
	ctx.lr = 0x82B9ED78;
	sub_82CA9480(ctx, base);
	// 82B9ED78: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9ED7C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ED80: 4B663539  bl 0x822022b8
	ctx.lr = 0x82B9ED84;
	sub_822022B8(ctx, base);
	// 82B9ED84: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9ED88: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ED8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9ED90: 41980024  blt cr6, 0x82b9edb4
	if ctx.cr[6].lt {
	pc = 0x82B9EDB4; continue 'dispatch;
	}
	// 82B9ED94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9ED98: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9ED9C: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EDA0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82B9EDA4: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82B9EDA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EDAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82B9EDB0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82B9EDB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9EDB8: 4810A69C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9EDC0 size=148
    let mut pc: u32 = 0x82B9EDC0;
    'dispatch: loop {
        match pc {
            0x82B9EDC0 => {
    //   block [0x82B9EDC0..0x82B9EE54)
	// 82B9EDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9EDC4: 4810A641  bl 0x82ca9404
	ctx.lr = 0x82B9EDC8;
	sub_82CA93D0(ctx, base);
	// 82B9EDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9EDCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9EDD0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82B9EDD4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82B9EDD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82B9EDDC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9EDE0: 40990068  ble cr6, 0x82b9ee48
	if !ctx.cr[6].gt {
	pc = 0x82B9EE48; continue 'dispatch;
	}
	// 82B9EDE4: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82B9EDE8: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82B9EDEC: 40980008  bge cr6, 0x82b9edf4
	if !ctx.cr[6].lt {
	pc = 0x82B9EDF4; continue 'dispatch;
	}
	// 82B9EDF0: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82B9EDF4: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 82B9EDF8: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82B9EDFC: 4B68DBD5  bl 0x8222c9d0
	ctx.lr = 0x82B9EE00;
	sub_8222C9D0(ctx, base);
	// 82B9EE00: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82B9EE04: 40820014  bne 0x82b9ee18
	if !ctx.cr[0].eq {
	pc = 0x82B9EE18; continue 'dispatch;
	}
	// 82B9EE08: 3D608007  lis r11, -0x7ff9
	ctx.r[11].s64 = -2147024896;
	// 82B9EE0C: 616B000E  ori r11, r11, 0xe
	ctx.r[11].u64 = ctx.r[11].u64 | 14;
	// 82B9EE10: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9EE14: 48000034  b 0x82b9ee48
	pc = 0x82B9EE48; continue 'dispatch;
	// 82B9EE18: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EE1C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82B9EE20: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82B9EE24: 419A0020  beq cr6, 0x82b9ee44
	if ctx.cr[6].eq {
	pc = 0x82B9EE44; continue 'dispatch;
	}
	// 82B9EE28: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82B9EE2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82B9EE30: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82B9EE34: 4810A64D  bl 0x82ca9480
	ctx.lr = 0x82B9EE38;
	sub_82CA9480(ctx, base);
	// 82B9EE38: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82B9EE3C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EE40: 4B663479  bl 0x822022b8
	ctx.lr = 0x82B9EE44;
	sub_822022B8(ctx, base);
	// 82B9EE44: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82B9EE48: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82B9EE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82B9EE50: 4810A604  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9EE58 size=100
    let mut pc: u32 = 0x82B9EE58;
    'dispatch: loop {
        match pc {
            0x82B9EE58 => {
    //   block [0x82B9EE58..0x82B9EEBC)
	// 82B9EE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9EE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9EE60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9EE64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9EE68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82B9EE6C: 4871AAD9  bl 0x832b9944
	ctx.lr = 0x82B9EE70;
	// extern call 0x832B9944 → crate::xboxkrnl::KeGetCurrentProcessType
	crate::xboxkrnl::KeGetCurrentProcessType(ctx, base);
	// 82B9EE70: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82B9EE74: 409A0010  bne cr6, 0x82b9ee84
	if !ctx.cr[6].eq {
	pc = 0x82B9EE84; continue 'dispatch;
	}
	// 82B9EE78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82B9EE7C: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82B9EE80: 4800000C  b 0x82b9ee8c
	pc = 0x82B9EE8C; continue 'dispatch;
	// 82B9EE84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82B9EE88: 816B09B0  lwz r11, 0x9b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2480 as u32) ) } as u64;
	// 82B9EE8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82B9EE90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82B9EE94: 419A0010  beq cr6, 0x82b9eea4
	if ctx.cr[6].eq {
	pc = 0x82B9EEA4; continue 'dispatch;
	}
	// 82B9EE98: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82B9EE9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82B9EEA0: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82B9EEA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82B9EEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82B9EEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9EEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9EEB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9EEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9EEC0 size=16
    let mut pc: u32 = 0x82B9EEC0;
    'dispatch: loop {
        match pc {
            0x82B9EEC0 => {
    //   block [0x82B9EEC0..0x82B9EED0)
	// 82B9EEC0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B9EEC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9EEC8: 5564003A  rlwinm r4, r11, 0, 0, 0x1d
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82B9EECC: 4B723764  b 0x822c2630
	sub_822C2630(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82B9EED0 size=12
    let mut pc: u32 = 0x82B9EED0;
    'dispatch: loop {
        match pc {
            0x82B9EED0 => {
    //   block [0x82B9EED0..0x82B9EEDC)
	// 82B9EED0: 80830018  lwz r4, 0x18(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82B9EED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82B9EED8: 4B723758  b 0x822c2630
	sub_822C2630(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82B9EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82B9EEE0 size=340
    let mut pc: u32 = 0x82B9EEE0;
    'dispatch: loop {
        match pc {
            0x82B9EEE0 => {
    //   block [0x82B9EEE0..0x82B9F034)
	// 82B9EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82B9EEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82B9EEE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82B9EEEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82B9EEF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82B9EEF4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 82B9EEF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82B9EEFC: 816B713C  lwz r11, 0x713c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28988 as u32) ) } as u64;
	// 82B9EF00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82B9EF04: 409A0118  bne cr6, 0x82b9f01c
	if !ctx.cr[6].eq {
	pc = 0x82B9F01C; continue 'dispatch;
	}
	// 82B9EF08: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82B9EF0C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82B9EF10: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82B9EF14: 4099000C  ble cr6, 0x82b9ef20
	if !ctx.cr[6].gt {
	pc = 0x82B9EF20; continue 'dispatch;
	}
	// 82B9EF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82B9EF1C: 4B649FA5  bl 0x821e8ec0
	ctx.lr = 0x82B9EF20;
	sub_821E8EC0(ctx, base);
	// 82B9EF20: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82B9EF24: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 82B9EF28: 616B2100  ori r11, r11, 0x2100
	ctx.r[11].u64 = ctx.r[11].u64 | 8448;
	// 82B9EF2C: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82B9EF30: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF34: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82B9EF38: 3D00C000  lis r8, -0x4000
	ctx.r[8].s64 = -1073741824;
	// 82B9EF3C: 38E00100  li r7, 0x100
	ctx.r[7].s64 = 256;
	// 82B9EF40: 61083B00  ori r8, r8, 0x3b00
	ctx.r[8].u64 = ctx.r[8].u64 | 15104;
	// 82B9EF44: 3CC0C010  lis r6, -0x3ff0
	ctx.r[6].s64 = -1072693248;
	// 82B9EF48: 95430004  stwu r10, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82B9EF50: 60CA2B00  ori r10, r6, 0x2b00
	ctx.r[10].u64 = ctx.r[6].u64 | 11008;
	// 82B9EF54: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 82B9EF58: 3C808209  lis r4, -0x7df7
	ctx.r[4].s64 = -2113339392;
	// 82B9EF5C: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 82B9EF60: 95230004  stwu r9, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF64: 38845DE4  addi r4, r4, 0x5de4
	ctx.r[4].s64 = ctx.r[4].s64 + 24036;
	// 82B9EF68: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF6C: 95030004  stwu r8, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF70: 94E30004  stwu r7, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF74: 95430004  stwu r10, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF78: 97C30004  stwu r30, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[3].u32 = ea;
	// 82B9EF7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82B9EF80: 94DE0004  stwu r6, 4(r30)
	ea = ctx.r[30].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[30].u32 = ea;
	// 82B9EF84: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82B9EF88: 4810A4F9  bl 0x82ca9480
	ctx.lr = 0x82B9EF8C;
	sub_82CA9480(ctx, base);
	// 82B9EF8C: 397E003C  addi r11, r30, 0x3c
	ctx.r[11].s64 = ctx.r[30].s64 + 60;
	// 82B9EF90: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82B9EF94: 3D200700  lis r9, 0x700
	ctx.r[9].s64 = 117440512;
	// 82B9EF98: 614A2180  ori r10, r10, 0x2180
	ctx.r[10].u64 = ctx.r[10].u64 | 8576;
	// 82B9EF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82B9EFA0: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B9EFA4: 39402208  li r10, 0x2208
	ctx.r[10].s64 = 8712;
	// 82B9EFA8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82B9EFAC: 3CC0C000  lis r6, -0x4000
	ctx.r[6].s64 = -1073741824;
	// 82B9EFB0: 3CA00001  lis r5, 1
	ctx.r[5].s64 = 65536;
	// 82B9EFB4: 60C63600  ori r6, r6, 0x3600
	ctx.r[6].u64 = ctx.r[6].u64 | 13824;
	// 82B9EFB8: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82B9EFBC: 60A50081  ori r5, r5, 0x81
	ctx.r[5].u64 = ctx.r[5].u64 | 129;
	// 82B9EFC0: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B9EFC4: 798C4FE6  rldicr r12, r12, 0x29, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(41) & 0xFFFFFFFFFFFFFFFF;
	// 82B9EFC8: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82B9EFCC: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82B9EFD0: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82B9EFD4: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 82B9EFD8: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 82B9EFDC: E95F0010  ld r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82B9EFE0: 654A0008  oris r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 524288;
	// 82B9EFE4: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B9EFE8: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B9EFEC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B9EFF0: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B9EFF4: 798C47E6  rldicr r12, r12, 0x28, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(40) & 0xFFFFFFFFFFFFFFFF;
	// 82B9EFF8: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B9EFFC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82B9F000: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B9F004: 798C3FE6  rldicr r12, r12, 0x27, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(39) & 0xFFFFFFFFFFFFFFFF;
	// 82B9F008: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82B9F00C: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B9F010: 614A0008  ori r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 8;
	// 82B9F014: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82B9F018: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82B9F01C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82B9F020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82B9F024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82B9F028: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82B9F02C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82B9F030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


