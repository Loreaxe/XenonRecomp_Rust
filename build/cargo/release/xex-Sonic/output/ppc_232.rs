pub fn sub_82FEC838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC838 size=8
    let mut pc: u32 = 0x82FEC838;
    'dispatch: loop {
        match pc {
            0x82FEC838 => {
    //   block [0x82FEC838..0x82FEC840)
	// 82FEC838: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEC83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC840 size=8
    let mut pc: u32 = 0x82FEC840;
    'dispatch: loop {
        match pc {
            0x82FEC840 => {
    //   block [0x82FEC840..0x82FEC848)
	// 82FEC840: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEC844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC848 size=8
    let mut pc: u32 = 0x82FEC848;
    'dispatch: loop {
        match pc {
            0x82FEC848 => {
    //   block [0x82FEC848..0x82FEC850)
	// 82FEC848: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEC84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC850 size=8
    let mut pc: u32 = 0x82FEC850;
    'dispatch: loop {
        match pc {
            0x82FEC850 => {
    //   block [0x82FEC850..0x82FEC858)
	// 82FEC850: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEC854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC858 size=8
    let mut pc: u32 = 0x82FEC858;
    'dispatch: loop {
        match pc {
            0x82FEC858 => {
    //   block [0x82FEC858..0x82FEC860)
	// 82FEC858: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEC85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC860 size=8
    let mut pc: u32 = 0x82FEC860;
    'dispatch: loop {
        match pc {
            0x82FEC860 => {
    //   block [0x82FEC860..0x82FEC868)
	// 82FEC860: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC868 size=60
    let mut pc: u32 = 0x82FEC868;
    'dispatch: loop {
        match pc {
            0x82FEC868 => {
    //   block [0x82FEC868..0x82FEC8A4)
	// 82FEC868: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82FEC86C: 4198008C  blt cr6, 0x82fec8f8
	if ctx.cr[6].lt {
		sub_82FEC8F8(ctx, base);
		return;
	}
	// 82FEC870: 419A007C  beq cr6, 0x82fec8ec
	if ctx.cr[6].eq {
		sub_82FEC8EC(ctx, base);
		return;
	}
	// 82FEC874: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82FEC878: 4198005C  blt cr6, 0x82fec8d4
	if ctx.cr[6].lt {
		sub_82FEC8D4(ctx, base);
		return;
	}
	// 82FEC87C: 2B040005  cmplwi cr6, r4, 5
	ctx.cr[6].compare_u32(ctx.r[4].u32, 5 as u32, &mut ctx.xer);
	// 82FEC880: 419A0048  beq cr6, 0x82fec8c8
	if ctx.cr[6].eq {
		sub_82FEC8C8(ctx, base);
		return;
	}
	// 82FEC884: 2B040006  cmplwi cr6, r4, 6
	ctx.cr[6].compare_u32(ctx.r[4].u32, 6 as u32, &mut ctx.xer);
	// 82FEC888: 419A0034  beq cr6, 0x82fec8bc
	if ctx.cr[6].eq {
		sub_82FEC8BC(ctx, base);
		return;
	}
	// 82FEC88C: 2B040009  cmplwi cr6, r4, 9
	ctx.cr[6].compare_u32(ctx.r[4].u32, 9 as u32, &mut ctx.xer);
	// 82FEC890: 419A0020  beq cr6, 0x82fec8b0
	if ctx.cr[6].eq {
		sub_82FEC8B0(ctx, base);
		return;
	}
	// 82FEC894: 2B04000C  cmplwi cr6, r4, 0xc
	ctx.cr[6].compare_u32(ctx.r[4].u32, 12 as u32, &mut ctx.xer);
	// 82FEC898: 419A000C  beq cr6, 0x82fec8a4
	if ctx.cr[6].eq {
		sub_82FEC8A4(ctx, base);
		return;
	}
	// 82FEC89C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC8A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC8A4 size=12
    let mut pc: u32 = 0x82FEC8A4;
    'dispatch: loop {
        match pc {
            0x82FEC8A4 => {
    //   block [0x82FEC8A4..0x82FEC8B0)
	// 82FEC8A4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEC8A8: 5563BFFE  rlwinm r3, r11, 0x17, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 82FEC8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC8B0 size=12
    let mut pc: u32 = 0x82FEC8B0;
    'dispatch: loop {
        match pc {
            0x82FEC8B0 => {
    //   block [0x82FEC8B0..0x82FEC8BC)
	// 82FEC8B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEC8B4: 5563C7FE  rlwinm r3, r11, 0x18, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FEC8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC8BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC8BC size=12
    let mut pc: u32 = 0x82FEC8BC;
    'dispatch: loop {
        match pc {
            0x82FEC8BC => {
    //   block [0x82FEC8BC..0x82FEC8C8)
	// 82FEC8BC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEC8C0: 5563CFFE  rlwinm r3, r11, 0x19, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82FEC8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC8C8 size=12
    let mut pc: u32 = 0x82FEC8C8;
    'dispatch: loop {
        match pc {
            0x82FEC8C8 => {
    //   block [0x82FEC8C8..0x82FEC8D4)
	// 82FEC8C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEC8CC: 5563D7FE  rlwinm r3, r11, 0x1a, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82FEC8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC8D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC8D4 size=24
    let mut pc: u32 = 0x82FEC8D4;
    'dispatch: loop {
        match pc {
            0x82FEC8D4 => {
    //   block [0x82FEC8D4..0x82FEC8EC)
	// 82FEC8D4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEC8D8: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FEC8DC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FEC8E0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FEC8E4: 386B000F  addi r3, r11, 0xf
	ctx.r[3].s64 = ctx.r[11].s64 + 15;
	// 82FEC8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC8EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC8EC size=12
    let mut pc: u32 = 0x82FEC8EC;
    'dispatch: loop {
        match pc {
            0x82FEC8EC => {
    //   block [0x82FEC8EC..0x82FEC8F8)
	// 82FEC8EC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEC8F0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82FEC8F4: 48000008  b 0x82fec8fc
	sub_82FEC8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC8F8 size=12
    let mut pc: u32 = 0x82FEC8F8;
    'dispatch: loop {
        match pc {
            0x82FEC8F8 => {
    //   block [0x82FEC8F8..0x82FEC904)
	// 82FEC8F8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEC8FC: 556307BE  clrlwi r3, r11, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82FEC900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC908 size=48
    let mut pc: u32 = 0x82FEC908;
    'dispatch: loop {
        match pc {
            0x82FEC908 => {
    //   block [0x82FEC908..0x82FEC938)
	// 82FEC908: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 82FEC90C: 419A0054  beq cr6, 0x82fec960
	if ctx.cr[6].eq {
		sub_82FEC960(ctx, base);
		return;
	}
	// 82FEC910: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82FEC914: 419A0044  beq cr6, 0x82fec958
	if ctx.cr[6].eq {
		sub_82FEC958(ctx, base);
		return;
	}
	// 82FEC918: 2F040007  cmpwi cr6, r4, 7
	ctx.cr[6].compare_i32(ctx.r[4].s32, 7, &mut ctx.xer);
	// 82FEC91C: 419A0034  beq cr6, 0x82fec950
	if ctx.cr[6].eq {
		sub_82FEC950(ctx, base);
		return;
	}
	// 82FEC920: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 82FEC924: 419A0024  beq cr6, 0x82fec948
	if ctx.cr[6].eq {
		sub_82FEC948(ctx, base);
		return;
	}
	// 82FEC928: 2F04000A  cmpwi cr6, r4, 0xa
	ctx.cr[6].compare_i32(ctx.r[4].s32, 10, &mut ctx.xer);
	// 82FEC92C: 419A0014  beq cr6, 0x82fec940
	if ctx.cr[6].eq {
		sub_82FEC940(ctx, base);
		return;
	}
	// 82FEC930: 2F04000B  cmpwi cr6, r4, 0xb
	ctx.cr[6].compare_i32(ctx.r[4].s32, 11, &mut ctx.xer);
	// 82FEC934: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC938 size=8
    let mut pc: u32 = 0x82FEC938;
    'dispatch: loop {
        match pc {
            0x82FEC938 => {
    //   block [0x82FEC938..0x82FEC940)
	// 82FEC938: 90A30020  stw r5, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82FEC93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC940 size=8
    let mut pc: u32 = 0x82FEC940;
    'dispatch: loop {
        match pc {
            0x82FEC940 => {
    //   block [0x82FEC940..0x82FEC948)
	// 82FEC940: 90A3001C  stw r5, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82FEC944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC948 size=8
    let mut pc: u32 = 0x82FEC948;
    'dispatch: loop {
        match pc {
            0x82FEC948 => {
    //   block [0x82FEC948..0x82FEC950)
	// 82FEC948: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82FEC94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC950 size=8
    let mut pc: u32 = 0x82FEC950;
    'dispatch: loop {
        match pc {
            0x82FEC950 => {
    //   block [0x82FEC950..0x82FEC958)
	// 82FEC950: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82FEC954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC958 size=8
    let mut pc: u32 = 0x82FEC958;
    'dispatch: loop {
        match pc {
            0x82FEC958 => {
    //   block [0x82FEC958..0x82FEC960)
	// 82FEC958: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82FEC95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC960 size=8
    let mut pc: u32 = 0x82FEC960;
    'dispatch: loop {
        match pc {
            0x82FEC960 => {
    //   block [0x82FEC960..0x82FEC968)
	// 82FEC960: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82FEC964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC968 size=52
    let mut pc: u32 = 0x82FEC968;
    'dispatch: loop {
        match pc {
            0x82FEC968 => {
    //   block [0x82FEC968..0x82FEC99C)
	// 82FEC968: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82FEC96C: 41980088  blt cr6, 0x82fec9f4
	if ctx.cr[6].lt {
		sub_82FEC9F4(ctx, base);
		return;
	}
	// 82FEC970: 419A007C  beq cr6, 0x82fec9ec
	if ctx.cr[6].eq {
		sub_82FEC9EC(ctx, base);
		return;
	}
	// 82FEC974: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82FEC978: 41980064  blt cr6, 0x82fec9dc
	if ctx.cr[6].lt {
		sub_82FEC9DC(ctx, base);
		return;
	}
	// 82FEC97C: 2B040005  cmplwi cr6, r4, 5
	ctx.cr[6].compare_u32(ctx.r[4].u32, 5 as u32, &mut ctx.xer);
	// 82FEC980: 419A004C  beq cr6, 0x82fec9cc
	if ctx.cr[6].eq {
		sub_82FEC9CC(ctx, base);
		return;
	}
	// 82FEC984: 2B040006  cmplwi cr6, r4, 6
	ctx.cr[6].compare_u32(ctx.r[4].u32, 6 as u32, &mut ctx.xer);
	// 82FEC988: 419A0034  beq cr6, 0x82fec9bc
	if ctx.cr[6].eq {
		sub_82FEC9BC(ctx, base);
		return;
	}
	// 82FEC98C: 2B040009  cmplwi cr6, r4, 9
	ctx.cr[6].compare_u32(ctx.r[4].u32, 9 as u32, &mut ctx.xer);
	// 82FEC990: 419A001C  beq cr6, 0x82fec9ac
	if ctx.cr[6].eq {
		sub_82FEC9AC(ctx, base);
		return;
	}
	// 82FEC994: 2B04000C  cmplwi cr6, r4, 0xc
	ctx.cr[6].compare_u32(ctx.r[4].u32, 12 as u32, &mut ctx.xer);
	// 82FEC998: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC99C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FEC99C size=16
    let mut pc: u32 = 0x82FEC99C;
    'dispatch: loop {
        match pc {
            0x82FEC99C => {
    //   block [0x82FEC99C..0x82FEC9AC)
	// 82FEC99C: 21650000  subfic r11, r5, 0
	ctx.xer.ca = ctx.r[5].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[5].s64;
	// 82FEC9A0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FEC9A4: 556B05AC  rlwinm r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FEC9A8: 48000050  b 0x82fec9f8
	sub_82FEC9F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC9AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FEC9AC size=16
    let mut pc: u32 = 0x82FEC9AC;
    'dispatch: loop {
        match pc {
            0x82FEC9AC => {
    //   block [0x82FEC9AC..0x82FEC9BC)
	// 82FEC9AC: 21650000  subfic r11, r5, 0
	ctx.xer.ca = ctx.r[5].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[5].s64;
	// 82FEC9B0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FEC9B4: 556B05EE  rlwinm r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FEC9B8: 48000040  b 0x82fec9f8
	sub_82FEC9F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC9BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FEC9BC size=16
    let mut pc: u32 = 0x82FEC9BC;
    'dispatch: loop {
        match pc {
            0x82FEC9BC => {
    //   block [0x82FEC9BC..0x82FEC9CC)
	// 82FEC9BC: 21650000  subfic r11, r5, 0
	ctx.xer.ca = ctx.r[5].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[5].s64;
	// 82FEC9C0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FEC9C4: 556B0630  rlwinm r11, r11, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FEC9C8: 48000030  b 0x82fec9f8
	sub_82FEC9F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC9CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FEC9CC size=16
    let mut pc: u32 = 0x82FEC9CC;
    'dispatch: loop {
        match pc {
            0x82FEC9CC => {
    //   block [0x82FEC9CC..0x82FEC9DC)
	// 82FEC9CC: 21650000  subfic r11, r5, 0
	ctx.xer.ca = ctx.r[5].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[5].s64;
	// 82FEC9D0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FEC9D4: 556B0672  rlwinm r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FEC9D8: 48000020  b 0x82fec9f8
	sub_82FEC9F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC9DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC9DC size=16
    let mut pc: u32 = 0x82FEC9DC;
    'dispatch: loop {
        match pc {
            0x82FEC9DC => {
    //   block [0x82FEC9DC..0x82FEC9EC)
	// 82FEC9DC: 3965FFF1  addi r11, r5, -0xf
	ctx.r[11].s64 = ctx.r[5].s64 + -15;
	// 82FEC9E0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FEC9E4: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FEC9E8: 48000010  b 0x82fec9f8
	sub_82FEC9F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC9EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC9EC size=8
    let mut pc: u32 = 0x82FEC9EC;
    'dispatch: loop {
        match pc {
            0x82FEC9EC => {
    //   block [0x82FEC9EC..0x82FEC9F4)
	// 82FEC9EC: 54AB173A  rlwinm r11, r5, 2, 0x1c, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 82FEC9F0: 48000008  b 0x82fec9f8
	sub_82FEC9F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC9F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC9F4 size=20
    let mut pc: u32 = 0x82FEC9F4;
    'dispatch: loop {
        match pc {
            0x82FEC9F4 => {
    //   block [0x82FEC9F4..0x82FECA08)
	// 82FEC9F4: 54AB07BE  clrlwi r11, r5, 0x1e
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 82FEC9F8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC9FC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82FECA00: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FECA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECA08 size=8
    let mut pc: u32 = 0x82FECA08;
    'dispatch: loop {
        match pc {
            0x82FECA08 => {
    //   block [0x82FECA08..0x82FECA10)
	// 82FECA08: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FECA0C: 4BFFFC74  b 0x82fec680
	sub_82FEC680(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECA10 size=20
    let mut pc: u32 = 0x82FECA10;
    'dispatch: loop {
        match pc {
            0x82FECA10 => {
    //   block [0x82FECA10..0x82FECA24)
	// 82FECA10: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FECA14: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FECA18: 4198000C  blt cr6, 0x82feca24
	if ctx.cr[6].lt {
		sub_82FECA24(ctx, base);
		return;
	}
	// 82FECA1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FECA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECA24 size=8
    let mut pc: u32 = 0x82FECA24;
    'dispatch: loop {
        match pc {
            0x82FECA24 => {
    //   block [0x82FECA24..0x82FECA2C)
	// 82FECA24: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FECA28: 4803FE48  b 0x8302c870
	sub_8302C870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECA30 size=20
    let mut pc: u32 = 0x82FECA30;
    'dispatch: loop {
        match pc {
            0x82FECA30 => {
    //   block [0x82FECA30..0x82FECA44)
	// 82FECA30: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FECA34: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FECA38: 4198000C  blt cr6, 0x82feca44
	if ctx.cr[6].lt {
		sub_82FECA44(ctx, base);
		return;
	}
	// 82FECA3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FECA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECA44 size=8
    let mut pc: u32 = 0x82FECA44;
    'dispatch: loop {
        match pc {
            0x82FECA44 => {
    //   block [0x82FECA44..0x82FECA4C)
	// 82FECA44: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FECA48: 4803FE28  b 0x8302c870
	sub_8302C870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECA50 size=20
    let mut pc: u32 = 0x82FECA50;
    'dispatch: loop {
        match pc {
            0x82FECA50 => {
    //   block [0x82FECA50..0x82FECA64)
	// 82FECA50: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FECA54: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FECA58: 4198000C  blt cr6, 0x82feca64
	if ctx.cr[6].lt {
		sub_82FECA64(ctx, base);
		return;
	}
	// 82FECA5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FECA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECA64 size=8
    let mut pc: u32 = 0x82FECA64;
    'dispatch: loop {
        match pc {
            0x82FECA64 => {
    //   block [0x82FECA64..0x82FECA6C)
	// 82FECA64: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FECA68: 4803FE08  b 0x8302c870
	sub_8302C870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECA70 size=148
    let mut pc: u32 = 0x82FECA70;
    'dispatch: loop {
        match pc {
            0x82FECA70 => {
    //   block [0x82FECA70..0x82FECB04)
	// 82FECA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECA74: 481BB6F5  bl 0x831a8168
	ctx.lr = 0x82FECA78;
	sub_831A8130(ctx, base);
	// 82FECA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECA7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FECA80: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FECA84: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FECA88: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FECA8C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FECA90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FECA94: 40990054  ble cr6, 0x82fecae8
	if !ctx.cr[6].gt {
	pc = 0x82FECAE8; continue 'dispatch;
	}
	// 82FECA98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FECA9C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FECAA0: 4803FDD1  bl 0x8302c870
	ctx.lr = 0x82FECAA4;
	sub_8302C870(ctx, base);
	// 82FECAA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FECAA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FECAAC: 4BFE7195  bl 0x82fd3c40
	ctx.lr = 0x82FECAB0;
	sub_82FD3C40(ctx, base);
	// 82FECAB0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FECAB4: 41820024  beq 0x82fecad8
	if ctx.cr[0].eq {
	pc = 0x82FECAD8; continue 'dispatch;
	}
	// 82FECAB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FECABC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FECAC0: 4803FDB1  bl 0x8302c870
	ctx.lr = 0x82FECAC4;
	sub_8302C870(ctx, base);
	// 82FECAC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FECAC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FECACC: 4BFE7175  bl 0x82fd3c40
	ctx.lr = 0x82FECAD0;
	sub_82FD3C40(ctx, base);
	// 82FECAD0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FECAD4: 40820020  bne 0x82fecaf4
	if !ctx.cr[0].eq {
	pc = 0x82FECAF4; continue 'dispatch;
	}
	// 82FECAD8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FECADC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FECAE0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FECAE4: 4198FFB4  blt cr6, 0x82feca98
	if ctx.cr[6].lt {
	pc = 0x82FECA98; continue 'dispatch;
	}
	// 82FECAE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FECAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FECAF0: 481BB6C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FECAF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FECAF8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FECAFC: 4803FD75  bl 0x8302c870
	ctx.lr = 0x82FECB00;
	sub_8302C870(ctx, base);
	// 82FECB00: 4BFFFFEC  b 0x82fecaec
	pc = 0x82FECAEC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECB08 size=8
    let mut pc: u32 = 0x82FECB08;
    'dispatch: loop {
        match pc {
            0x82FECB08 => {
    //   block [0x82FECB08..0x82FECB10)
	// 82FECB08: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FECB0C: 8213D978  lwz r16, -0x2688(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-9864 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECB10 size=252
    let mut pc: u32 = 0x82FECB10;
    'dispatch: loop {
        match pc {
            0x82FECB10 => {
    //   block [0x82FECB10..0x82FECC0C)
	// 82FECB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECB14: 481BB655  bl 0x831a8168
	ctx.lr = 0x82FECB18;
	sub_831A8130(ctx, base);
	// 82FECB18: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FECB1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECB20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FECB24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FECB28: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FECB2C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FECB30: 909E0000  stw r4, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FECB34: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FECB38: 4BFEB761  bl 0x82fd8298
	ctx.lr = 0x82FECB3C;
	sub_82FD8298(ctx, base);
	// 82FECB3C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FECB40: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FECB44: 4182002C  beq 0x82fecb70
	if ctx.cr[0].eq {
	pc = 0x82FECB70; continue 'dispatch;
	}
	// 82FECB48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FECB4C: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECB50: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FECB54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FECB58: 4805FC99  bl 0x8304c7f0
	ctx.lr = 0x82FECB5C;
	sub_8304C7F0(ctx, base);
	// 82FECB5C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FECB60: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 82FECB64: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FECB68: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FECB6C: 48000008  b 0x82fecb74
	pc = 0x82FECB74; continue 'dispatch;
	// 82FECB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FECB74: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FECB78: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECB7C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FECB80: 4BFEB719  bl 0x82fd8298
	ctx.lr = 0x82FECB84;
	sub_82FD8298(ctx, base);
	// 82FECB84: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FECB88: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FECB8C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FECB90: 3B8B6C68  addi r28, r11, 0x6c68
	ctx.r[28].s64 = ctx.r[11].s64 + 27752;
	// 82FECB94: 41820024  beq 0x82fecbb8
	if ctx.cr[0].eq {
	pc = 0x82FECBB8; continue 'dispatch;
	}
	// 82FECB98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FECB9C: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECBA0: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FECBA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FECBA8: 4BFE5E99  bl 0x82fd2a40
	ctx.lr = 0x82FECBAC;
	sub_82FD2A40(ctx, base);
	// 82FECBAC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FECBB0: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FECBB4: 48000008  b 0x82fecbbc
	pc = 0x82FECBBC; continue 'dispatch;
	// 82FECBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FECBBC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FECBC0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECBC4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FECBC8: 4BFEB6D1  bl 0x82fd8298
	ctx.lr = 0x82FECBCC;
	sub_82FD8298(ctx, base);
	// 82FECBCC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FECBD0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FECBD4: 41820024  beq 0x82fecbf8
	if ctx.cr[0].eq {
	pc = 0x82FECBF8; continue 'dispatch;
	}
	// 82FECBD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FECBDC: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECBE0: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FECBE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FECBE8: 4BFE5E59  bl 0x82fd2a40
	ctx.lr = 0x82FECBEC;
	sub_82FD2A40(ctx, base);
	// 82FECBEC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FECBF0: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FECBF4: 48000008  b 0x82fecbfc
	pc = 0x82FECBFC; continue 'dispatch;
	// 82FECBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FECBFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FECC00: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FECC04: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FECC08: 481BB5B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECC0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECC0C size=48
    let mut pc: u32 = 0x82FECC0C;
    'dispatch: loop {
        match pc {
            0x82FECC0C => {
    //   block [0x82FECC0C..0x82FECC3C)
	// 82FECC0C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FECC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECC18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECC1C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FECC20: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECC24: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FECC28: 4BFEB6B9  bl 0x82fd82e0
	ctx.lr = 0x82FECC2C;
	sub_82FD82E0(ctx, base);
	// 82FECC2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FECC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECC3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECC3C size=48
    let mut pc: u32 = 0x82FECC3C;
    'dispatch: loop {
        match pc {
            0x82FECC3C => {
    //   block [0x82FECC3C..0x82FECC6C)
	// 82FECC3C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FECC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECC4C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FECC50: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECC54: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FECC58: 4BFEB689  bl 0x82fd82e0
	ctx.lr = 0x82FECC5C;
	sub_82FD82E0(ctx, base);
	// 82FECC5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FECC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECC6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECC6C size=48
    let mut pc: u32 = 0x82FECC6C;
    'dispatch: loop {
        match pc {
            0x82FECC6C => {
    //   block [0x82FECC6C..0x82FECC9C)
	// 82FECC6C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FECC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECC7C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FECC80: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECC84: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FECC88: 4BFEB659  bl 0x82fd82e0
	ctx.lr = 0x82FECC8C;
	sub_82FD82E0(ctx, base);
	// 82FECC8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FECC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECCA0 size=8
    let mut pc: u32 = 0x82FECCA0;
    'dispatch: loop {
        match pc {
            0x82FECCA0 => {
    //   block [0x82FECCA0..0x82FECCA8)
	// 82FECCA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FECCA4: 8213DA00  lwz r16, -0x2600(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-9728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECCA8 size=104
    let mut pc: u32 = 0x82FECCA8;
    'dispatch: loop {
        match pc {
            0x82FECCA8 => {
    //   block [0x82FECCA8..0x82FECD10)
	// 82FECCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECCAC: 481BB4B1  bl 0x831a815c
	ctx.lr = 0x82FECCB0;
	sub_831A8130(ctx, base);
	// 82FECCB0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FECCB4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECCB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FECCBC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FECCC0: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FECCC4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FECCC8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82FECCCC: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82FECCD0: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FECCD4: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82FECCD8: 4BFEC259  bl 0x82fd8f30
	ctx.lr = 0x82FECCDC;
	sub_82FD8F30(ctx, base);
	// 82FECCDC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FECCE0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82FECCE4: 396BD9E8  addi r11, r11, -0x2618
	ctx.r[11].s64 = ctx.r[11].s64 + -9752;
	// 82FECCE8: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FECCEC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FECCF0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FECCF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FECCF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FECCFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FECD00: 4BFEC639  bl 0x82fd9338
	ctx.lr = 0x82FECD04;
	sub_82FD9338(ctx, base);
	// 82FECD04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FECD08: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FECD0C: 481BB4A0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECD10 size=40
    let mut pc: u32 = 0x82FECD10;
    'dispatch: loop {
        match pc {
            0x82FECD10 => {
    //   block [0x82FECD10..0x82FECD38)
	// 82FECD10: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FECD14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECD18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECD1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECD20: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FECD24: 4BFEC155  bl 0x82fd8e78
	ctx.lr = 0x82FECD28;
	sub_82FD8E78(ctx, base);
	// 82FECD28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FECD2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECD30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECD38 size=60
    let mut pc: u32 = 0x82FECD38;
    'dispatch: loop {
        match pc {
            0x82FECD38 => {
    //   block [0x82FECD38..0x82FECD74)
	// 82FECD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FECD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECD48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FECD4C: 4BFEC25D  bl 0x82fd8fa8
	ctx.lr = 0x82FECD50;
	sub_82FD8FA8(ctx, base);
	// 82FECD50: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FECD54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FECD58: 396BD9E8  addi r11, r11, -0x2618
	ctx.r[11].s64 = ctx.r[11].s64 + -9752;
	// 82FECD5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FECD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FECD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECD6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FECD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECD78 size=16
    let mut pc: u32 = 0x82FECD78;
    'dispatch: loop {
        match pc {
            0x82FECD78 => {
    //   block [0x82FECD78..0x82FECD88)
	// 82FECD78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FECD7C: 396BD9E8  addi r11, r11, -0x2618
	ctx.r[11].s64 = ctx.r[11].s64 + -9752;
	// 82FECD80: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FECD84: 4BFEC0F4  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECD88 size=8
    let mut pc: u32 = 0x82FECD88;
    'dispatch: loop {
        match pc {
            0x82FECD88 => {
    //   block [0x82FECD88..0x82FECD90)
	// 82FECD88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FECD8C: 8213DA38  lwz r16, -0x25c8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-9672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECD90 size=92
    let mut pc: u32 = 0x82FECD90;
    'dispatch: loop {
        match pc {
            0x82FECD90 => {
    //   block [0x82FECD90..0x82FECDEC)
	// 82FECD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECD94: 481BB3D9  bl 0x831a816c
	ctx.lr = 0x82FECD98;
	sub_831A8130(ctx, base);
	// 82FECD98: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FECD9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECDA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FECDA4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FECDA8: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FECDAC: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FECDB0: 4BFEB4E9  bl 0x82fd8298
	ctx.lr = 0x82FECDB4;
	sub_82FD8298(ctx, base);
	// 82FECDB4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FECDB8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FECDBC: 41820024  beq 0x82fecde0
	if ctx.cr[0].eq {
	pc = 0x82FECDE0; continue 'dispatch;
	}
	// 82FECDC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FECDC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FECDC8: 4BFEC1E1  bl 0x82fd8fa8
	ctx.lr = 0x82FECDCC;
	sub_82FD8FA8(ctx, base);
	// 82FECDCC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FECDD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FECDD4: 396BD9E8  addi r11, r11, -0x2618
	ctx.r[11].s64 = ctx.r[11].s64 + -9752;
	// 82FECDD8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FECDDC: 48000008  b 0x82fecde4
	pc = 0x82FECDE4; continue 'dispatch;
	// 82FECDE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FECDE4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FECDE8: 481BB3D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECDEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECDEC size=48
    let mut pc: u32 = 0x82FECDEC;
    'dispatch: loop {
        match pc {
            0x82FECDEC => {
    //   block [0x82FECDEC..0x82FECE1C)
	// 82FECDEC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FECDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECDF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECDFC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FECE00: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FECE04: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FECE08: 4BFEB4D9  bl 0x82fd82e0
	ctx.lr = 0x82FECE0C;
	sub_82FD82E0(ctx, base);
	// 82FECE0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FECE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FECE20 size=12
    let mut pc: u32 = 0x82FECE20;
    'dispatch: loop {
        match pc {
            0x82FECE20 => {
    //   block [0x82FECE20..0x82FECE2C)
	// 82FECE20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FECE24: 386B83CC  addi r3, r11, -0x7c34
	ctx.r[3].s64 = ctx.r[11].s64 + -31796;
	// 82FECE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECE30 size=88
    let mut pc: u32 = 0x82FECE30;
    'dispatch: loop {
        match pc {
            0x82FECE30 => {
    //   block [0x82FECE30..0x82FECE88)
	// 82FECE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECE38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FECE3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FECE40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECE44: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FECE48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FECE4C: 396BD9E8  addi r11, r11, -0x2618
	ctx.r[11].s64 = ctx.r[11].s64 + -9752;
	// 82FECE50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FECE54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FECE58: 4BFEC021  bl 0x82fd8e78
	ctx.lr = 0x82FECE5C;
	sub_82FD8E78(ctx, base);
	// 82FECE5C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FECE60: 4182000C  beq 0x82fece6c
	if ctx.cr[0].eq {
	pc = 0x82FECE6C; continue 'dispatch;
	}
	// 82FECE64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FECE68: 4BFEB479  bl 0x82fd82e0
	ctx.lr = 0x82FECE6C;
	sub_82FD82E0(ctx, base);
	// 82FECE6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FECE70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FECE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECE7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FECE80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FECE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECE88 size=204
    let mut pc: u32 = 0x82FECE88;
    'dispatch: loop {
        match pc {
            0x82FECE88 => {
    //   block [0x82FECE88..0x82FECF54)
	// 82FECE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECE90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FECE94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FECE98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECE9C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FECEA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FECEA4: 616B8004  ori r11, r11, 0x8004
	ctx.r[11].u64 = ctx.r[11].u64 | 32772;
	// 82FECEA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FECEAC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECEB0: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FECEB4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FECEB8: 41980020  blt cr6, 0x82feced8
	if ctx.cr[6].lt {
	pc = 0x82FECED8; continue 'dispatch;
	}
	// 82FECEBC: 48005EAD  bl 0x82ff2d68
	ctx.lr = 0x82FECEC0;
	sub_82FF2D68(ctx, base);
	// 82FECEC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FECEC4: 40820014  bne 0x82feced8
	if !ctx.cr[0].eq {
	pc = 0x82FECED8; continue 'dispatch;
	}
	// 82FECEC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FECECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FECED0: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FECED4: 48000068  b 0x82fecf3c
	pc = 0x82FECF3C; continue 'dispatch;
	// 82FECED8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECEDC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FECEE0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FECEE4: 7D6BFA2E  lhzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FECEE8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82FECEEC: 2B0A000D  cmplwi cr6, r10, 0xd
	ctx.cr[6].compare_u32(ctx.r[10].u32, 13 as u32, &mut ctx.xer);
	// 82FECEF0: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FECEF4: 419A0028  beq cr6, 0x82fecf1c
	if ctx.cr[6].eq {
	pc = 0x82FECF1C; continue 'dispatch;
	}
	// 82FECEF8: 2B0A0085  cmplwi cr6, r10, 0x85
	ctx.cr[6].compare_u32(ctx.r[10].u32, 133 as u32, &mut ctx.xer);
	// 82FECEFC: 419A000C  beq cr6, 0x82fecf08
	if ctx.cr[6].eq {
	pc = 0x82FECF08; continue 'dispatch;
	}
	// 82FECF00: 2B0A2028  cmplwi cr6, r10, 0x2028
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8232 as u32, &mut ctx.xer);
	// 82FECF04: 409A0034  bne cr6, 0x82fecf38
	if !ctx.cr[6].eq {
	pc = 0x82FECF38; continue 'dispatch;
	}
	// 82FECF08: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FECF0C: 616B8058  ori r11, r11, 0x8058
	ctx.r[11].u64 = ctx.r[11].u64 | 32856;
	// 82FECF10: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FECF14: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FECF18: 41820020  beq 0x82fecf38
	if ctx.cr[0].eq {
	pc = 0x82FECF38; continue 'dispatch;
	}
	// 82FECF1C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FECF20: 616B8034  ori r11, r11, 0x8034
	ctx.r[11].u64 = ctx.r[11].u64 | 32820;
	// 82FECF24: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FECF28: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FECF2C: 409A000C  bne cr6, 0x82fecf38
	if !ctx.cr[6].eq {
	pc = 0x82FECF38; continue 'dispatch;
	}
	// 82FECF30: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82FECF34: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FECF38: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FECF3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FECF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FECF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FECF48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FECF4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FECF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FECF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FECF58 size=496
    let mut pc: u32 = 0x82FECF58;
    'dispatch: loop {
        match pc {
            0x82FECF58 => {
    //   block [0x82FECF58..0x82FED148)
	// 82FECF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FECF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FECF60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FECF64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FECF68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FECF6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FECF70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FECF74: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECF78: 2B0B000D  cmplwi cr6, r11, 0xd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 13 as u32, &mut ctx.xer);
	// 82FECF7C: 409A00A8  bne cr6, 0x82fed024
	if !ctx.cr[6].eq {
	pc = 0x82FED024; continue 'dispatch;
	}
	// 82FECF80: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FECF84: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 82FECF88: 616A8034  ori r10, r11, 0x8034
	ctx.r[10].u64 = ctx.r[11].u64 | 32820;
	// 82FECF8C: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 82FECF90: 6129C008  ori r9, r9, 0xc008
	ctx.r[9].u64 = ctx.r[9].u64 | 49160;
	// 82FECF94: 396BC00C  addi r11, r11, -0x3ff4
	ctx.r[11].s64 = ctx.r[11].s64 + -16372;
	// 82FECF98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FECF9C: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FECFA0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82FECFA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECFA8: 7D1F492E  stwx r8, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82FECFAC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FECFB0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FECFB4: 409A017C  bne cr6, 0x82fed130
	if !ctx.cr[6].eq {
	pc = 0x82FED130; continue 'dispatch;
	}
	// 82FECFB8: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FECFBC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECFC0: 616B8004  ori r11, r11, 0x8004
	ctx.r[11].u64 = ctx.r[11].u64 | 32772;
	// 82FECFC4: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FECFC8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FECFCC: 41980010  blt cr6, 0x82fecfdc
	if ctx.cr[6].lt {
	pc = 0x82FECFDC; continue 'dispatch;
	}
	// 82FECFD0: 48005D99  bl 0x82ff2d68
	ctx.lr = 0x82FECFD4;
	sub_82FF2D68(ctx, base);
	// 82FECFD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FECFD8: 41820040  beq 0x82fed018
	if ctx.cr[0].eq {
	pc = 0x82FED018; continue 'dispatch;
	}
	// 82FECFDC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FECFE0: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 82FECFE4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FECFE8: 7D6BFA2E  lhzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FECFEC: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82FECFF0: 419A0020  beq cr6, 0x82fed010
	if ctx.cr[6].eq {
	pc = 0x82FED010; continue 'dispatch;
	}
	// 82FECFF4: 2B0B0085  cmplwi cr6, r11, 0x85
	ctx.cr[6].compare_u32(ctx.r[11].u32, 133 as u32, &mut ctx.xer);
	// 82FECFF8: 409A0020  bne cr6, 0x82fed018
	if !ctx.cr[6].eq {
	pc = 0x82FED018; continue 'dispatch;
	}
	// 82FECFFC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED000: 616B8058  ori r11, r11, 0x8058
	ctx.r[11].u64 = ctx.r[11].u64 | 32856;
	// 82FED004: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FED008: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED00C: 4182000C  beq 0x82fed018
	if ctx.cr[0].eq {
	pc = 0x82FED018; continue 'dispatch;
	}
	// 82FED010: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82FED014: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FED018: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82FED01C: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FED020: 48000110  b 0x82fed130
	pc = 0x82FED130; continue 'dispatch;
	// 82FED024: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82FED028: 409A0020  bne cr6, 0x82fed048
	if !ctx.cr[6].eq {
	pc = 0x82FED048; continue 'dispatch;
	}
	// 82FED02C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FED030: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FED034: 616AC008  ori r10, r11, 0xc008
	ctx.r[10].u64 = ctx.r[11].u64 | 49160;
	// 82FED038: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 82FED03C: 396BC00C  addi r11, r11, -0x3ff4
	ctx.r[11].s64 = ctx.r[11].s64 + -16372;
	// 82FED040: 7D3F512E  stwx r9, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FED044: 4800001C  b 0x82fed060
	pc = 0x82FED060; continue 'dispatch;
	// 82FED048: 2B0B0085  cmplwi cr6, r11, 0x85
	ctx.cr[6].compare_u32(ctx.r[11].u32, 133 as u32, &mut ctx.xer);
	// 82FED04C: 419A0024  beq cr6, 0x82fed070
	if ctx.cr[6].eq {
	pc = 0x82FED070; continue 'dispatch;
	}
	// 82FED050: 2B0B2028  cmplwi cr6, r11, 0x2028
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8232 as u32, &mut ctx.xer);
	// 82FED054: 419A001C  beq cr6, 0x82fed070
	if ctx.cr[6].eq {
	pc = 0x82FED070; continue 'dispatch;
	}
	// 82FED058: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 82FED05C: 396BC008  addi r11, r11, -0x3ff8
	ctx.r[11].s64 = ctx.r[11].s64 + -16376;
	// 82FED060: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED064: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FED068: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FED06C: 480000C4  b 0x82fed130
	pc = 0x82FED130; continue 'dispatch;
	// 82FED070: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FED074: 41820068  beq 0x82fed0dc
	if ctx.cr[0].eq {
	pc = 0x82FED0DC; continue 'dispatch;
	}
	// 82FED078: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED07C: 616B805C  ori r11, r11, 0x805c
	ctx.r[11].u64 = ctx.r[11].u64 | 32860;
	// 82FED080: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FED084: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FED088: 409A0054  bne cr6, 0x82fed0dc
	if !ctx.cr[6].eq {
	pc = 0x82FED0DC; continue 'dispatch;
	}
	// 82FED08C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FED094: 61688060  ori r8, r11, 0x8060
	ctx.r[8].u64 = ctx.r[11].u64 | 32864;
	// 82FED098: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FED0A0: 61678040  ori r7, r11, 0x8040
	ctx.r[7].u64 = ctx.r[11].u64 | 32832;
	// 82FED0A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FED0A8: 38C00048  li r6, 0x48
	ctx.r[6].s64 = 72;
	// 82FED0AC: 388BDA68  addi r4, r11, -0x2598
	ctx.r[4].s64 = ctx.r[11].s64 + -9624;
	// 82FED0B0: 7D7F402E  lwzx r11, r31, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FED0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FED0B8: 38A003A5  li r5, 0x3a5
	ctx.r[5].s64 = 933;
	// 82FED0BC: 7CFF382E  lwzx r7, r31, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FED0C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FED0C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FED0C8: 4BFFFBE1  bl 0x82fecca8
	ctx.lr = 0x82FED0CC;
	sub_82FECCA8(ctx, base);
	// 82FED0CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FED0D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FED0D4: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FED0D8: 481C3B51  bl 0x831b0c28
	ctx.lr = 0x82FED0DC;
	sub_831B0C28(ctx, base);
	// 82FED0DC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED0E0: 616B8058  ori r11, r11, 0x8058
	ctx.r[11].u64 = ctx.r[11].u64 | 32856;
	// 82FED0E4: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FED0E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED0EC: 41820044  beq 0x82fed130
	if ctx.cr[0].eq {
	pc = 0x82FED130; continue 'dispatch;
	}
	// 82FED0F0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED0F4: 616B8034  ori r11, r11, 0x8034
	ctx.r[11].u64 = ctx.r[11].u64 | 32820;
	// 82FED0F8: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FED0FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FED100: 409A0030  bne cr6, 0x82fed130
	if !ctx.cr[6].eq {
	pc = 0x82FED130; continue 'dispatch;
	}
	// 82FED104: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FED108: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FED10C: 616AC008  ori r10, r11, 0xc008
	ctx.r[10].u64 = ctx.r[11].u64 | 49160;
	// 82FED110: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 82FED114: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82FED118: 396BC00C  addi r11, r11, -0x3ff4
	ctx.r[11].s64 = ctx.r[11].s64 + -16372;
	// 82FED11C: 7D3F512E  stwx r9, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FED120: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED124: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FED128: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FED12C: B11E0000  sth r8, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82FED130: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FED134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FED138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FED13C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FED140: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FED144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED148 size=68
    let mut pc: u32 = 0x82FED148;
    'dispatch: loop {
        match pc {
            0x82FED148 => {
    //   block [0x82FED148..0x82FED18C)
	// 82FED148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FED154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED158: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FED15C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FED160: 396BDA9C  addi r11, r11, -0x2564
	ctx.r[11].s64 = ctx.r[11].s64 + -9572;
	// 82FED164: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FED168: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FED16C: 41820008  beq 0x82fed174
	if ctx.cr[0].eq {
	pc = 0x82FED174; continue 'dispatch;
	}
	// 82FED170: 4B2D30F9  bl 0x822c0268
	ctx.lr = 0x82FED174;
	sub_822C0268(ctx, base);
	// 82FED174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FED178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FED17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FED180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FED184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FED188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FED190 size=64
    let mut pc: u32 = 0x82FED190;
    'dispatch: loop {
        match pc {
            0x82FED190 => {
    //   block [0x82FED190..0x82FED1D0)
	// 82FED190: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FED194: 90830028  stw r4, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82FED198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FED19C: 394BDAB0  addi r10, r11, -0x2550
	ctx.r[10].s64 = ctx.r[11].s64 + -9552;
	// 82FED1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FED1A4: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FED1A8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FED1AC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FED1B0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FED1B4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FED1B8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FED1BC: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FED1C0: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FED1C4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FED1C8: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82FED1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FED1D0 size=8
    let mut pc: u32 = 0x82FED1D0;
    'dispatch: loop {
        match pc {
            0x82FED1D0 => {
    //   block [0x82FED1D0..0x82FED1D8)
	// 82FED1D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FED1D4: 8213DB1C  lwz r16, -0x24e4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-9444 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED1D8 size=576
    let mut pc: u32 = 0x82FED1D8;
    'dispatch: loop {
        match pc {
            0x82FED1D8 => {
    //   block [0x82FED1D8..0x82FED418)
	// 82FED1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED1DC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FED1E0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FED1E4: 481BAF69  bl 0x831a814c
	ctx.lr = 0x82FED1E8;
	sub_831A8130(ctx, base);
	// 82FED1E8: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 82FED1EC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED1F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FED1F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FED1F8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82FED1FC: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82FED200: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 82FED204: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED208: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 82FED20C: 93BF00F4  stw r29, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 82FED210: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FED214: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED218: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED21C: 4E800421  bctrl
	ctx.lr = 0x82FED220;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED220: 7C751B79  or. r21, r3, r3
	ctx.r[21].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82FED224: 92BF0070  stw r21, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[21].u32 ) };
	// 82FED228: 4082000C  bne 0x82fed234
	if !ctx.cr[0].eq {
	pc = 0x82FED234; continue 'dispatch;
	}
	// 82FED22C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED230: 480001E0  b 0x82fed410
	pc = 0x82FED410; continue 'dispatch;
	// 82FED234: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FED23C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED240: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED244: 4E800421  bctrl
	ctx.lr = 0x82FED248;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED248: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED24C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FED254: 419A00D8  beq cr6, 0x82fed32c
	if ctx.cr[6].eq {
	pc = 0x82FED32C; continue 'dispatch;
	}
	// 82FED258: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 82FED25C: 809D0028  lwz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED260: 60638064  ori r3, r3, 0x8064
	ctx.r[3].u64 = ctx.r[3].u64 | 32868;
	// 82FED264: 4BFEB035  bl 0x82fd8298
	ctx.lr = 0x82FED268;
	sub_82FD8298(ctx, base);
	// 82FED268: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED26C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED270: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82FED274: 935F0074  stw r26, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[26].u32 ) };
	// 82FED278: 418200AC  beq 0x82fed324
	if ctx.cr[0].eq {
	pc = 0x82FED324; continue 'dispatch;
	}
	// 82FED27C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FED284: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED28C: 4E800421  bctrl
	ctx.lr = 0x82FED290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED290: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FED294: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED298: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED29C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED2A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FED2A4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED2A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED2AC: 4E800421  bctrl
	ctx.lr = 0x82FED2B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED2B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FED2B4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED2B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED2BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED2C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FED2C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FED2C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED2CC: 4E800421  bctrl
	ctx.lr = 0x82FED2D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED2D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FED2D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED2D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED2DC: 80BD0020  lwz r5, 0x20(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FED2E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED2E4: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED2E8: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82FED2EC: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82FED2F0: 9AC1005F  stb r22, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[22].u8 ) };
	// 82FED2F4: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82FED2F8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82FED2FC: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82FED300: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82FED304: 98610057  stb r3, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[3].u8 ) };
	// 82FED308: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FED30C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FED310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FED314: 480054CD  bl 0x82ff27e0
	ctx.lr = 0x82FED318;
	sub_82FF27E0(ctx, base);
	// 82FED318: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED31C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED320: 48000008  b 0x82fed328
	pc = 0x82FED328; continue 'dispatch;
	// 82FED324: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED328: 480000AC  b 0x82fed3d4
	pc = 0x82FED3D4; continue 'dispatch;
	// 82FED32C: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 82FED330: 809D0028  lwz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED334: 60638064  ori r3, r3, 0x8064
	ctx.r[3].u64 = ctx.r[3].u64 | 32868;
	// 82FED338: 4BFEAF61  bl 0x82fd8298
	ctx.lr = 0x82FED33C;
	sub_82FD8298(ctx, base);
	// 82FED33C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED340: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED344: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FED348: 937F0074  stw r27, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
	// 82FED34C: 41820084  beq 0x82fed3d0
	if ctx.cr[0].eq {
	pc = 0x82FED3D0; continue 'dispatch;
	}
	// 82FED350: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FED358: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED35C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED360: 4E800421  bctrl
	ctx.lr = 0x82FED364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED364: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FED368: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED36C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED370: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED374: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FED378: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FED37C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED380: 4E800421  bctrl
	ctx.lr = 0x82FED384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED384: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FED388: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED38C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED390: 807D0020  lwz r3, 0x20(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FED394: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FED398: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED39C: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82FED3A0: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82FED3A4: 9AC10057  stb r22, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[22].u8 ) };
	// 82FED3A8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82FED3AC: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82FED3B0: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82FED3B4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FED3B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FED3BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FED3C0: 48006E29  bl 0x82ff41e8
	ctx.lr = 0x82FED3C4;
	sub_82FF41E8(ctx, base);
	// 82FED3C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED3C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FED3CC: 48000008  b 0x82fed3d4
	pc = 0x82FED3D4; continue 'dispatch;
	// 82FED3D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED3D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FED3D8: 409A0020  bne cr6, 0x82fed3f8
	if !ctx.cr[6].eq {
	pc = 0x82FED3F8; continue 'dispatch;
	}
	// 82FED3DC: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED3E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FED3E4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82FED3E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED3F0: 4E800421  bctrl
	ctx.lr = 0x82FED3F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED3F4: 4BFFFE38  b 0x82fed22c
	pc = 0x82FED22C; continue 'dispatch;
	// 82FED3F8: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED3FC: 616A8028  ori r10, r11, 0x8028
	ctx.r[10].u64 = ctx.r[11].u64 | 32808;
	// 82FED400: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FED404: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FED408: 913D0014  stw r9, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FED40C: 7D63512E  stwx r11, r3, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82FED410: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 82FED414: 481BAD88  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FED418 size=8
    let mut pc: u32 = 0x82FED418;
    'dispatch: loop {
        match pc {
            0x82FED418 => {
    //   block [0x82FED418..0x82FED420)
	// 82FED418: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FED41C: 8213DB1C  lwz r16, -0x24e4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-9444 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED420 size=24
    let mut pc: u32 = 0x82FED420;
    'dispatch: loop {
        match pc {
            0x82FED420 => {
    //   block [0x82FED420..0x82FED438)
	// 82FED420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED42C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FED430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED434: 481C37F5  bl 0x831b0c28
	ctx.lr = 0x82FED438;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED440 size=52
    let mut pc: u32 = 0x82FED440;
    'dispatch: loop {
        match pc {
            0x82FED440 => {
    //   block [0x82FED440..0x82FED474)
	// 82FED440: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 82FED444: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED448: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED44C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED450: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FED454: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FED458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED45C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED460: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED464: 4E800421  bctrl
	ctx.lr = 0x82FED468;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED468: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FED46C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED470: 481C37B9  bl 0x831b0c28
	ctx.lr = 0x82FED474;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED474 size=48
    let mut pc: u32 = 0x82FED474;
    'dispatch: loop {
        match pc {
            0x82FED474 => {
    //   block [0x82FED474..0x82FED4A4)
	// 82FED474: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 82FED478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED484: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82FED488: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED48C: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FED490: 4BFEAE51  bl 0x82fd82e0
	ctx.lr = 0x82FED494;
	sub_82FD82E0(ctx, base);
	// 82FED494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FED498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FED49C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FED4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED4A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED4A4 size=48
    let mut pc: u32 = 0x82FED4A4;
    'dispatch: loop {
        match pc {
            0x82FED4A4 => {
    //   block [0x82FED4A4..0x82FED4D4)
	// 82FED4A4: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 82FED4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED4B4: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82FED4B8: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED4BC: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FED4C0: 4BFEAE21  bl 0x82fd82e0
	ctx.lr = 0x82FED4C4;
	sub_82FD82E0(ctx, base);
	// 82FED4C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FED4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FED4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FED4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FED4D8 size=8
    let mut pc: u32 = 0x82FED4D8;
    'dispatch: loop {
        match pc {
            0x82FED4D8 => {
    //   block [0x82FED4D8..0x82FED4E0)
	// 82FED4D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FED4DC: 8213DC00  lwz r16, -0x2400(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-9216 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED4E0 size=304
    let mut pc: u32 = 0x82FED4E0;
    'dispatch: loop {
        match pc {
            0x82FED4E0 => {
    //   block [0x82FED4E0..0x82FED610)
	// 82FED4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED4E4: 481BAC71  bl 0x831a8154
	ctx.lr = 0x82FED4E8;
	sub_831A8130(ctx, base);
	// 82FED4E8: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 82FED4EC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED4F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FED4F4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FED4F8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FED4FC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82FED500: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82FED504: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED508: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82FED50C: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 82FED510: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82FED514: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 82FED518: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 82FED51C: 4BFEAD7D  bl 0x82fd8298
	ctx.lr = 0x82FED520;
	sub_82FD8298(ctx, base);
	// 82FED520: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82FED524: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED528: 4182002C  beq 0x82fed554
	if ctx.cr[0].eq {
	pc = 0x82FED554; continue 'dispatch;
	}
	// 82FED52C: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82FED530: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED534: 5785083C  slwi r5, r28, 1
	ctx.r[5].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FED538: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FED53C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FED540: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FED544: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82FED548: 4BFF0A31  bl 0x82fddf78
	ctx.lr = 0x82FED54C;
	sub_82FDDF78(ctx, base);
	// 82FED54C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FED550: 48000008  b 0x82fed558
	pc = 0x82FED558; continue 'dispatch;
	// 82FED554: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FED558: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FED55C: 409A000C  bne cr6, 0x82fed568
	if !ctx.cr[6].eq {
	pc = 0x82FED568; continue 'dispatch;
	}
	// 82FED560: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED564: 480000A4  b 0x82fed608
	pc = 0x82FED608; continue 'dispatch;
	// 82FED568: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 82FED56C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED570: 60638064  ori r3, r3, 0x8064
	ctx.r[3].u64 = ctx.r[3].u64 | 32868;
	// 82FED574: 4BFEAD25  bl 0x82fd8298
	ctx.lr = 0x82FED578;
	sub_82FD8298(ctx, base);
	// 82FED578: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82FED57C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED580: 41820044  beq 0x82fed5c4
	if ctx.cr[0].eq {
	pc = 0x82FED5C4; continue 'dispatch;
	}
	// 82FED584: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FED58C: 80BE0020  lwz r5, 0x20(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FED590: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82FED594: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82FED598: 9AE1005F  stb r23, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[23].u8 ) };
	// 82FED59C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82FED5A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FED5A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FED5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FED5AC: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82FED5B0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FED5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FED5B8: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 82FED5BC: 480054F5  bl 0x82ff2ab0
	ctx.lr = 0x82FED5C0;
	sub_82FF2AB0(ctx, base);
	// 82FED5C0: 48000008  b 0x82fed5c8
	pc = 0x82FED5C8; continue 'dispatch;
	// 82FED5C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED5C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FED5CC: 409A0024  bne cr6, 0x82fed5f0
	if !ctx.cr[6].eq {
	pc = 0x82FED5F0; continue 'dispatch;
	}
	// 82FED5D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED5D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FED5D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FED5DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED5E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED5E4: 4E800421  bctrl
	ctx.lr = 0x82FED5E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED5E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED5EC: 4800001C  b 0x82fed608
	pc = 0x82FED608; continue 'dispatch;
	// 82FED5F0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FED5F4: 616A8028  ori r10, r11, 0x8028
	ctx.r[10].u64 = ctx.r[11].u64 | 32808;
	// 82FED5F8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FED5FC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FED600: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FED604: 7D63512E  stwx r11, r3, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82FED608: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 82FED60C: 481BAB98  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED610 size=48
    let mut pc: u32 = 0x82FED610;
    'dispatch: loop {
        match pc {
            0x82FED610 => {
    //   block [0x82FED610..0x82FED640)
	// 82FED610: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FED614: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED618: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED61C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED620: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FED624: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED628: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FED62C: 4BFEACB5  bl 0x82fd82e0
	ctx.lr = 0x82FED630;
	sub_82FD82E0(ctx, base);
	// 82FED630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FED634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FED638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FED63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED640 size=48
    let mut pc: u32 = 0x82FED640;
    'dispatch: loop {
        match pc {
            0x82FED640 => {
    //   block [0x82FED640..0x82FED670)
	// 82FED640: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FED644: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED648: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED64C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED650: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FED654: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FED658: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FED65C: 4BFEAC85  bl 0x82fd82e0
	ctx.lr = 0x82FED660;
	sub_82FD82E0(ctx, base);
	// 82FED660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FED664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FED668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FED66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FED670 size=60
    let mut pc: u32 = 0x82FED670;
    'dispatch: loop {
        match pc {
            0x82FED670 => {
    //   block [0x82FED670..0x82FED6AC)
	// 82FED670: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED674: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FED678: 419A0034  beq cr6, 0x82fed6ac
	if ctx.cr[6].eq {
		sub_82FED6AC(ctx, base);
		return;
	}
	// 82FED67C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FED680: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED684: 614A8050  ori r10, r10, 0x8050
	ctx.r[10].u64 = ctx.r[10].u64 | 32848;
	// 82FED688: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FED68C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FED690: 409A001C  bne cr6, 0x82fed6ac
	if !ctx.cr[6].eq {
		sub_82FED6AC(ctx, base);
		return;
	}
	// 82FED694: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FED698: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FED69C: 614A802C  ori r10, r10, 0x802c
	ctx.r[10].u64 = ctx.r[10].u64 | 32812;
	// 82FED6A0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FED6A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FED6A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED6AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FED6AC size=8
    let mut pc: u32 = 0x82FED6AC;
    'dispatch: loop {
        match pc {
            0x82FED6AC => {
    //   block [0x82FED6AC..0x82FED6B4)
	// 82FED6AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FED6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FED6B8 size=16
    let mut pc: u32 = 0x82FED6B8;
    'dispatch: loop {
        match pc {
            0x82FED6B8 => {
    //   block [0x82FED6B8..0x82FED6C8)
	// 82FED6B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FED6BC: 396BDC50  addi r11, r11, -0x23b0
	ctx.r[11].s64 = ctx.r[11].s64 + -9136;
	// 82FED6C0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FED6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED6C8 size=148
    let mut pc: u32 = 0x82FED6C8;
    'dispatch: loop {
        match pc {
            0x82FED6C8 => {
    //   block [0x82FED6C8..0x82FED75C)
	// 82FED6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED6CC: 481BAA9D  bl 0x831a8168
	ctx.lr = 0x82FED6D0;
	sub_831A8130(ctx, base);
	// 82FED6D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED6D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FED6D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FED6DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FED6E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED6E4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FED6E8: 41980030  blt cr6, 0x82fed718
	if ctx.cr[6].lt {
	pc = 0x82FED718; continue 'dispatch;
	}
	// 82FED6EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FED6F0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FED6F4: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FED6F8: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FED6FC: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 82FED700: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FED704: 4BFE3255  bl 0x82fd0958
	ctx.lr = 0x82FED708;
	sub_82FD0958(ctx, base);
	// 82FED708: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FED70C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FED710: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FED714: 481C3515  bl 0x831b0c28
	ctx.lr = 0x82FED718;
	sub_831B0C28(ctx, base);
	// 82FED718: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED71C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED720: 41820028  beq 0x82fed748
	if ctx.cr[0].eq {
	pc = 0x82FED748; continue 'dispatch;
	}
	// 82FED724: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED728: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FED72C: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FED730: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED734: 41820014  beq 0x82fed748
	if ctx.cr[0].eq {
	pc = 0x82FED748; continue 'dispatch;
	}
	// 82FED738: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FED73C: 48004A4D  bl 0x82ff2188
	ctx.lr = 0x82FED740;
	sub_82FF2188(ctx, base);
	// 82FED740: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FED744: 4BFEAB9D  bl 0x82fd82e0
	ctx.lr = 0x82FED748;
	sub_82FD82E0(ctx, base);
	// 82FED748: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED74C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FED750: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82FED754: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FED758: 481BAA60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED760 size=124
    let mut pc: u32 = 0x82FED760;
    'dispatch: loop {
        match pc {
            0x82FED760 => {
    //   block [0x82FED760..0x82FED7DC)
	// 82FED760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED764: 481BAA01  bl 0x831a8164
	ctx.lr = 0x82FED768;
	sub_831A8130(ctx, base);
	// 82FED768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED76C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FED770: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FED774: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82FED778: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED77C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FED780: 40990050  ble cr6, 0x82fed7d0
	if !ctx.cr[6].gt {
	pc = 0x82FED7D0; continue 'dispatch;
	}
	// 82FED784: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82FED788: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED78C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED790: 41820024  beq 0x82fed7b4
	if ctx.cr[0].eq {
	pc = 0x82FED7B4; continue 'dispatch;
	}
	// 82FED794: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED798: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FED79C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED7A0: 41820014  beq 0x82fed7b4
	if ctx.cr[0].eq {
	pc = 0x82FED7B4; continue 'dispatch;
	}
	// 82FED7A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FED7A8: 480049E1  bl 0x82ff2188
	ctx.lr = 0x82FED7AC;
	sub_82FF2188(ctx, base);
	// 82FED7AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FED7B0: 4BFEAB31  bl 0x82fd82e0
	ctx.lr = 0x82FED7B4;
	sub_82FD82E0(ctx, base);
	// 82FED7B4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED7B8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FED7BC: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 82FED7C0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FED7C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED7C8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FED7CC: 4198FFBC  blt cr6, 0x82fed788
	if ctx.cr[6].lt {
	pc = 0x82FED788; continue 'dispatch;
	}
	// 82FED7D0: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82FED7D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FED7D8: 481BA9DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED7E0 size=260
    let mut pc: u32 = 0x82FED7E0;
    'dispatch: loop {
        match pc {
            0x82FED7E0 => {
    //   block [0x82FED7E0..0x82FED8E4)
	// 82FED7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED7E4: 481BA989  bl 0x831a816c
	ctx.lr = 0x82FED7E8;
	sub_831A8130(ctx, base);
	// 82FED7E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FED7F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FED7F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED7F8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FED7FC: 41980030  blt cr6, 0x82fed82c
	if ctx.cr[6].lt {
	pc = 0x82FED82C; continue 'dispatch;
	}
	// 82FED800: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FED804: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FED808: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FED80C: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FED810: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 82FED814: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FED818: 4BFE3141  bl 0x82fd0958
	ctx.lr = 0x82FED81C;
	sub_82FD0958(ctx, base);
	// 82FED81C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FED820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FED824: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FED828: 481C3401  bl 0x831b0c28
	ctx.lr = 0x82FED82C;
	sub_831B0C28(ctx, base);
	// 82FED82C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED830: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED834: 41820028  beq 0x82fed85c
	if ctx.cr[0].eq {
	pc = 0x82FED85C; continue 'dispatch;
	}
	// 82FED838: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED83C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FED840: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FED844: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED848: 41820014  beq 0x82fed85c
	if ctx.cr[0].eq {
	pc = 0x82FED85C; continue 'dispatch;
	}
	// 82FED84C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FED850: 48004939  bl 0x82ff2188
	ctx.lr = 0x82FED854;
	sub_82FF2188(ctx, base);
	// 82FED854: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FED858: 4BFEAA89  bl 0x82fd82e0
	ctx.lr = 0x82FED85C;
	sub_82FD82E0(ctx, base);
	// 82FED85C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED860: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FED864: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FED868: 409A0018  bne cr6, 0x82fed880
	if !ctx.cr[6].eq {
	pc = 0x82FED880; continue 'dispatch;
	}
	// 82FED86C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED870: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FED874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FED878: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82FED87C: 48000054  b 0x82fed8d0
	pc = 0x82FED8D0; continue 'dispatch;
	// 82FED880: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FED884: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FED888: 40980030  bge cr6, 0x82fed8b8
	if !ctx.cr[6].lt {
	pc = 0x82FED8B8; continue 'dispatch;
	}
	// 82FED88C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FED890: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED894: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FED898: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FED89C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FED8A0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED8A4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FED8A8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED8AC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82FED8B0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FED8B4: 4198FFDC  blt cr6, 0x82fed890
	if ctx.cr[6].lt {
	pc = 0x82FED890; continue 'dispatch;
	}
	// 82FED8B8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FED8C0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED8C4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FED8C8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FED8CC: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 82FED8D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED8D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FED8D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FED8DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FED8E0: 481BA8DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED8E8 size=104
    let mut pc: u32 = 0x82FED8E8;
    'dispatch: loop {
        match pc {
            0x82FED8E8 => {
    //   block [0x82FED8E8..0x82FED950)
	// 82FED8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED8F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FED8F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED8F8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED8FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED900: 4182003C  beq 0x82fed93c
	if ctx.cr[0].eq {
	pc = 0x82FED93C; continue 'dispatch;
	}
	// 82FED904: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FED908: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED90C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED910: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FED914: 41820028  beq 0x82fed93c
	if ctx.cr[0].eq {
	pc = 0x82FED93C; continue 'dispatch;
	}
	// 82FED918: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED91C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FED920: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FED924: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED928: 41820014  beq 0x82fed93c
	if ctx.cr[0].eq {
	pc = 0x82FED93C; continue 'dispatch;
	}
	// 82FED92C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FED930: 48004859  bl 0x82ff2188
	ctx.lr = 0x82FED934;
	sub_82FF2188(ctx, base);
	// 82FED934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FED938: 4BFEA9A9  bl 0x82fd82e0
	ctx.lr = 0x82FED93C;
	sub_82FD82E0(ctx, base);
	// 82FED93C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FED940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FED944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FED948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FED94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED950 size=132
    let mut pc: u32 = 0x82FED950;
    'dispatch: loop {
        match pc {
            0x82FED950 => {
    //   block [0x82FED950..0x82FED9D4)
	// 82FED950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED954: 481BA815  bl 0x831a8168
	ctx.lr = 0x82FED958;
	sub_831A8130(ctx, base);
	// 82FED958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FED960: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FED964: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED968: 4182004C  beq 0x82fed9b4
	if ctx.cr[0].eq {
	pc = 0x82FED9B4; continue 'dispatch;
	}
	// 82FED96C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED970: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FED974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FED978: 4099003C  ble cr6, 0x82fed9b4
	if !ctx.cr[6].gt {
	pc = 0x82FED9B4; continue 'dispatch;
	}
	// 82FED97C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FED980: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED984: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FED988: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FED98C: 41820014  beq 0x82fed9a0
	if ctx.cr[0].eq {
	pc = 0x82FED9A0; continue 'dispatch;
	}
	// 82FED990: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FED994: 480047F5  bl 0x82ff2188
	ctx.lr = 0x82FED998;
	sub_82FF2188(ctx, base);
	// 82FED998: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FED99C: 4BFEA945  bl 0x82fd82e0
	ctx.lr = 0x82FED9A0;
	sub_82FD82E0(ctx, base);
	// 82FED9A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED9A4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FED9A8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FED9AC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FED9B0: 4198FFD0  blt cr6, 0x82fed980
	if ctx.cr[6].lt {
	pc = 0x82FED980; continue 'dispatch;
	}
	// 82FED9B4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FED9B8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FED9BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FED9C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED9C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FED9C8: 4E800421  bctrl
	ctx.lr = 0x82FED9CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FED9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FED9D0: 481BA7E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FED9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FED9D8 size=96
    let mut pc: u32 = 0x82FED9D8;
    'dispatch: loop {
        match pc {
            0x82FED9D8 => {
    //   block [0x82FED9D8..0x82FEDA38)
	// 82FED9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FED9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FED9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FED9E4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FED9E8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FED9EC: 41980030  blt cr6, 0x82feda1c
	if ctx.cr[6].lt {
	pc = 0x82FEDA1C; continue 'dispatch;
	}
	// 82FED9F0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FED9F4: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FED9F8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FED9FC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FEDA00: 38A000F1  li r5, 0xf1
	ctx.r[5].s64 = 241;
	// 82FEDA04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDA08: 4BFE2F51  bl 0x82fd0958
	ctx.lr = 0x82FEDA0C;
	sub_82FD0958(ctx, base);
	// 82FEDA0C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEDA10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDA14: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FEDA18: 481C3211  bl 0x831b0c28
	ctx.lr = 0x82FEDA1C;
	sub_831B0C28(ctx, base);
	// 82FEDA1C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDA20: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEDA24: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEDA28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEDA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEDA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEDA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDA38 size=128
    let mut pc: u32 = 0x82FEDA38;
    'dispatch: loop {
        match pc {
            0x82FEDA38 => {
    //   block [0x82FEDA38..0x82FEDAB8)
	// 82FEDA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDA3C: 481BA731  bl 0x831a816c
	ctx.lr = 0x82FEDA40;
	sub_831A8130(ctx, base);
	// 82FEDA40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDA44: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEDA48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEDA4C: 396BDC50  addi r11, r11, -0x23b0
	ctx.r[11].s64 = ctx.r[11].s64 + -9136;
	// 82FEDA50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEDA54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEDA58: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FEDA5C: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FEDA60: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82FEDA64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEDA68: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82FEDA6C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FEDA70: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FEDA74: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FEDA78: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEDA7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEDA80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEDA84: 4E800421  bctrl
	ctx.lr = 0x82FEDA88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEDA88: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FEDA8C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FEDA90: 419A001C  beq cr6, 0x82fedaac
	if ctx.cr[6].eq {
	pc = 0x82FEDAAC; continue 'dispatch;
	}
	// 82FEDA94: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FEDA98: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDA9C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FEDAA0: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FEDAA4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FEDAA8: 4082FFF0  bne 0x82feda98
	if !ctx.cr[0].eq {
	pc = 0x82FEDA98; continue 'dispatch;
	}
	// 82FEDAAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEDAB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEDAB4: 481BA708  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDAB8 size=172
    let mut pc: u32 = 0x82FEDAB8;
    'dispatch: loop {
        match pc {
            0x82FEDAB8 => {
    //   block [0x82FEDAB8..0x82FEDB64)
	// 82FEDAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEDAC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEDAC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEDAC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDACC: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FEDAD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEDAD4: 616B8004  ori r11, r11, 0x8004
	ctx.r[11].u64 = ctx.r[11].u64 | 32772;
	// 82FEDAD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEDADC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEDAE0: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEDAE4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEDAE8: 41980030  blt cr6, 0x82fedb18
	if ctx.cr[6].lt {
	pc = 0x82FEDB18; continue 'dispatch;
	}
	// 82FEDAEC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FEDAF0: 616BC019  ori r11, r11, 0xc019
	ctx.r[11].u64 = ctx.r[11].u64 | 49177;
	// 82FEDAF4: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEDAF8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDAFC: 4182000C  beq 0x82fedb08
	if ctx.cr[0].eq {
	pc = 0x82FEDB08; continue 'dispatch;
	}
	// 82FEDB00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEDB04: 48000048  b 0x82fedb4c
	pc = 0x82FEDB4C; continue 'dispatch;
	// 82FEDB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEDB0C: 4800525D  bl 0x82ff2d68
	ctx.lr = 0x82FEDB10;
	sub_82FF2D68(ctx, base);
	// 82FEDB10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEDB14: 4182FFEC  beq 0x82fedb00
	if ctx.cr[0].eq {
	pc = 0x82FEDB00; continue 'dispatch;
	}
	// 82FEDB18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEDB1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEDB20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEDB24: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FEDB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEDB2C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEDB30: 7D6BFA2E  lhzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FEDB34: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FEDB38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEDB3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FEDB40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEDB44: 4BFFF415  bl 0x82fecf58
	ctx.lr = 0x82FEDB48;
	sub_82FECF58(ctx, base);
	// 82FEDB48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEDB4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEDB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEDB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEDB58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEDB5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEDB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDB68 size=124
    let mut pc: u32 = 0x82FEDB68;
    'dispatch: loop {
        match pc {
            0x82FEDB68 => {
    //   block [0x82FEDB68..0x82FEDBE4)
	// 82FEDB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDB6C: 481BA601  bl 0x831a816c
	ctx.lr = 0x82FEDB70;
	sub_831A8130(ctx, base);
	// 82FEDB70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDB74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEDB78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEDB7C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDB80: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 82FEDB84: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDB88: 41820014  beq 0x82fedb9c
	if ctx.cr[0].eq {
	pc = 0x82FEDB9C; continue 'dispatch;
	}
	// 82FEDB8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEDB90: 480045F9  bl 0x82ff2188
	ctx.lr = 0x82FEDB94;
	sub_82FF2188(ctx, base);
	// 82FEDB94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEDB98: 4BFEA749  bl 0x82fd82e0
	ctx.lr = 0x82FEDB9C;
	sub_82FD82E0(ctx, base);
	// 82FEDB9C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDBA0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FEDBA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDBA8: 41820014  beq 0x82fedbbc
	if ctx.cr[0].eq {
	pc = 0x82FEDBBC; continue 'dispatch;
	}
	// 82FEDBAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEDBB0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDBB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEDBB8: 4E800421  bctrl
	ctx.lr = 0x82FEDBBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEDBBC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDBC0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FEDBC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDBC8: 41820014  beq 0x82fedbdc
	if ctx.cr[0].eq {
	pc = 0x82FEDBDC; continue 'dispatch;
	}
	// 82FEDBCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEDBD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDBD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEDBD8: 4E800421  bctrl
	ctx.lr = 0x82FEDBDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEDBDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEDBE0: 481BA5DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDBE8 size=92
    let mut pc: u32 = 0x82FEDBE8;
    'dispatch: loop {
        match pc {
            0x82FEDBE8 => {
    //   block [0x82FEDBE8..0x82FEDC44)
	// 82FEDBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEDBF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDBF4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDBF8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEDBFC: 40820030  bne 0x82fedc2c
	if !ctx.cr[0].eq {
	pc = 0x82FEDC2C; continue 'dispatch;
	}
	// 82FEDC00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEDC04: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEDC08: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82FEDC0C: 388BAF10  addi r4, r11, -0x50f0
	ctx.r[4].s64 = ctx.r[11].s64 + -20720;
	// 82FEDC10: 38A0006A  li r5, 0x6a
	ctx.r[5].s64 = 106;
	// 82FEDC14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDC18: 4BFE6159  bl 0x82fd3d70
	ctx.lr = 0x82FEDC1C;
	sub_82FD3D70(ctx, base);
	// 82FEDC1C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEDC20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDC24: 388BC550  addi r4, r11, -0x3ab0
	ctx.r[4].s64 = ctx.r[11].s64 + -15024;
	// 82FEDC28: 481C3001  bl 0x831b0c28
	ctx.lr = 0x82FEDC2C;
	sub_831B0C28(ctx, base);
	// 82FEDC2C: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FEDC30: 480ACE29  bl 0x8309aa58
	ctx.lr = 0x82FEDC34;
	sub_8309AA58(ctx, base);
	// 82FEDC34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEDC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEDC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEDC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDC48 size=88
    let mut pc: u32 = 0x82FEDC48;
    'dispatch: loop {
        match pc {
            0x82FEDC48 => {
    //   block [0x82FEDC48..0x82FEDCA0)
	// 82FEDC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEDC50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDC54: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDC58: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEDC5C: 40990030  ble cr6, 0x82fedc8c
	if !ctx.cr[6].gt {
	pc = 0x82FEDC8C; continue 'dispatch;
	}
	// 82FEDC60: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEDC64: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEDC68: 38C0004F  li r6, 0x4f
	ctx.r[6].s64 = 79;
	// 82FEDC6C: 388BAF10  addi r4, r11, -0x50f0
	ctx.r[4].s64 = ctx.r[11].s64 + -20720;
	// 82FEDC70: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 82FEDC74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDC78: 4BFE2CE1  bl 0x82fd0958
	ctx.lr = 0x82FEDC7C;
	sub_82FD0958(ctx, base);
	// 82FEDC7C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEDC80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDC84: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FEDC88: 481C2FA1  bl 0x831b0c28
	ctx.lr = 0x82FEDC8C;
	sub_831B0C28(ctx, base);
	// 82FEDC8C: 4BFFFD4D  bl 0x82fed9d8
	ctx.lr = 0x82FEDC90;
	sub_82FED9D8(ctx, base);
	// 82FEDC90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEDC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEDC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEDC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEDCA0 size=8
    let mut pc: u32 = 0x82FEDCA0;
    'dispatch: loop {
        match pc {
            0x82FEDCA0 => {
    //   block [0x82FEDCA0..0x82FEDCA8)
	// 82FEDCA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEDCA4: 8213DC88  lwz r16, -0x2378(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-9080 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDCA8 size=164
    let mut pc: u32 = 0x82FEDCA8;
    'dispatch: loop {
        match pc {
            0x82FEDCA8 => {
    //   block [0x82FEDCA8..0x82FEDD4C)
	// 82FEDCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDCAC: 481BA4B9  bl 0x831a8164
	ctx.lr = 0x82FEDCB0;
	sub_831A8130(ctx, base);
	// 82FEDCB0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FEDCB4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDCB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEDCBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEDCC0: 396BDC68  addi r11, r11, -0x2398
	ctx.r[11].s64 = ctx.r[11].s64 + -9112;
	// 82FEDCC4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FEDCC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEDCCC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEDCD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDCD4: 4182004C  beq 0x82fedd20
	if ctx.cr[0].eq {
	pc = 0x82FEDD20; continue 'dispatch;
	}
	// 82FEDCD8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDCDC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FEDCE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEDCE4: 4099003C  ble cr6, 0x82fedd20
	if !ctx.cr[6].gt {
	pc = 0x82FEDD20; continue 'dispatch;
	}
	// 82FEDCE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEDCEC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDCF0: 7F8BE82E  lwzx r28, r11, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FEDCF4: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDCF8: 41820014  beq 0x82fedd0c
	if ctx.cr[0].eq {
	pc = 0x82FEDD0C; continue 'dispatch;
	}
	// 82FEDCFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEDD00: 48004489  bl 0x82ff2188
	ctx.lr = 0x82FEDD04;
	sub_82FF2188(ctx, base);
	// 82FEDD04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEDD08: 4BFEA5D9  bl 0x82fd82e0
	ctx.lr = 0x82FEDD0C;
	sub_82FD82E0(ctx, base);
	// 82FEDD0C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDD10: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FEDD14: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FEDD18: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEDD1C: 4198FFD0  blt cr6, 0x82fedcec
	if ctx.cr[6].lt {
	pc = 0x82FEDCEC; continue 'dispatch;
	}
	// 82FEDD20: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEDD24: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDD28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEDD2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDD30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEDD34: 4E800421  bctrl
	ctx.lr = 0x82FEDD38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEDD38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEDD3C: 396BDC50  addi r11, r11, -0x23b0
	ctx.r[11].s64 = ctx.r[11].s64 + -9136;
	// 82FEDD40: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEDD44: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FEDD48: 481BA46C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDD4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDD4C size=40
    let mut pc: u32 = 0x82FEDD4C;
    'dispatch: loop {
        match pc {
            0x82FEDD4C => {
    //   block [0x82FEDD4C..0x82FEDD74)
	// 82FEDD4C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEDD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEDD58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDD5C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEDD60: 4BFFF959  bl 0x82fed6b8
	ctx.lr = 0x82FEDD64;
	sub_82FED6B8(ctx, base);
	// 82FEDD64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEDD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEDD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEDD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDD78 size=76
    let mut pc: u32 = 0x82FEDD78;
    'dispatch: loop {
        match pc {
            0x82FEDD78 => {
    //   block [0x82FEDD78..0x82FEDDC4)
	// 82FEDD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEDD80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEDD84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEDD88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDD8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEDD90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEDD94: 4BFFFF15  bl 0x82fedca8
	ctx.lr = 0x82FEDD98;
	sub_82FEDCA8(ctx, base);
	// 82FEDD98: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEDD9C: 4182000C  beq 0x82fedda8
	if ctx.cr[0].eq {
	pc = 0x82FEDDA8; continue 'dispatch;
	}
	// 82FEDDA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEDDA4: 4BFEA53D  bl 0x82fd82e0
	ctx.lr = 0x82FEDDA8;
	sub_82FD82E0(ctx, base);
	// 82FEDDA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEDDAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEDDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEDDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEDDB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEDDBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEDDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEDDC8 size=60
    let mut pc: u32 = 0x82FEDDC8;
    'dispatch: loop {
        match pc {
            0x82FEDDC8 => {
    //   block [0x82FEDDC8..0x82FEDE04)
	// 82FEDDC8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDDCC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDDD0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FEDDD4: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEDDD8: 41820020  beq 0x82feddf8
	if ctx.cr[0].eq {
	pc = 0x82FEDDF8; continue 'dispatch;
	}
	// 82FEDDDC: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FEDDE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDDE4: 614AC019  ori r10, r10, 0xc019
	ctx.r[10].u64 = ctx.r[10].u64 | 49177;
	// 82FEDDE8: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEDDEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDDF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FEDDF4: 40820008  bne 0x82feddfc
	if !ctx.cr[0].eq {
	pc = 0x82FEDDFC; continue 'dispatch;
	}
	// 82FEDDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEDDFC: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FEDE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDE08 size=176
    let mut pc: u32 = 0x82FEDE08;
    'dispatch: loop {
        match pc {
            0x82FEDE08 => {
    //   block [0x82FEDE08..0x82FEDEB8)
	// 82FEDE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDE0C: 481BA35D  bl 0x831a8168
	ctx.lr = 0x82FEDE10;
	sub_831A8130(ctx, base);
	// 82FEDE10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDE14: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FEDE18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEDE1C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FEDE20: 617E8028  ori r30, r11, 0x8028
	ctx.r[30].u64 = ctx.r[11].u64 | 32808;
	// 82FEDE24: 48000050  b 0x82fede74
	pc = 0x82FEDE74; continue 'dispatch;
	// 82FEDE28: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDE2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDE30: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FEDE34: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEDE38: 40820054  bne 0x82fede8c
	if !ctx.cr[0].eq {
	pc = 0x82FEDE8C; continue 'dispatch;
	}
	// 82FEDE3C: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDE40: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDE44: 41820014  beq 0x82fede58
	if ctx.cr[0].eq {
	pc = 0x82FEDE58; continue 'dispatch;
	}
	// 82FEDE48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEDE4C: 4800433D  bl 0x82ff2188
	ctx.lr = 0x82FEDE50;
	sub_82FF2188(ctx, base);
	// 82FEDE50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEDE54: 4BFEA48D  bl 0x82fd82e0
	ctx.lr = 0x82FEDE58;
	sub_82FD82E0(ctx, base);
	// 82FEDE58: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDE5C: 4BFFFD8D  bl 0x82fedbe8
	ctx.lr = 0x82FEDE60;
	sub_82FEDBE8(ctx, base);
	// 82FEDE60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FEDE64: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDE68: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FEDE6C: 4BFFFD7D  bl 0x82fedbe8
	ctx.lr = 0x82FEDE70;
	sub_82FEDBE8(ctx, base);
	// 82FEDE70: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FEDE74: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDE78: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FEDE7C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FEDE80: 409AFFA8  bne cr6, 0x82fede28
	if !ctx.cr[6].eq {
	pc = 0x82FEDE28; continue 'dispatch;
	}
	// 82FEDE84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FEDE88: 481BA330  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FEDE8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEDE90: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEDE94: 38C00045  li r6, 0x45
	ctx.r[6].s64 = 69;
	// 82FEDE98: 388BDCB8  addi r4, r11, -0x2348
	ctx.r[4].s64 = ctx.r[11].s64 + -9032;
	// 82FEDE9C: 38A0016E  li r5, 0x16e
	ctx.r[5].s64 = 366;
	// 82FEDEA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDEA4: 4BFE31B5  bl 0x82fd1058
	ctx.lr = 0x82FEDEA8;
	sub_82FD1058(ctx, base);
	// 82FEDEA8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEDEAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEDEB0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FEDEB4: 481C2D75  bl 0x831b0c28
	ctx.lr = 0x82FEDEB8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEDEB8 size=20
    let mut pc: u32 = 0x82FEDEB8;
    'dispatch: loop {
        match pc {
            0x82FEDEB8 => {
    //   block [0x82FEDEB8..0x82FEDECC)
	// 82FEDEB8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDEBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDEC0: 4082000C  bne 0x82fedecc
	if !ctx.cr[0].eq {
		sub_82FEDECC(ctx, base);
		return;
	}
	// 82FEDEC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEDEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDECC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEDECC size=16
    let mut pc: u32 = 0x82FEDECC;
    'dispatch: loop {
        match pc {
            0x82FEDECC => {
    //   block [0x82FEDECC..0x82FEDEDC)
	// 82FEDECC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDED0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDED4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FEDED8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDEDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEDEDC size=8
    let mut pc: u32 = 0x82FEDEDC;
    'dispatch: loop {
        match pc {
            0x82FEDEDC => {
    //   block [0x82FEDEDC..0x82FEDEE4)
	// 82FEDEDC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FEDEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDEE8 size=204
    let mut pc: u32 = 0x82FEDEE8;
    'dispatch: loop {
        match pc {
            0x82FEDEE8 => {
    //   block [0x82FEDEE8..0x82FEDFB4)
	// 82FEDEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDEEC: 481BA279  bl 0x831a8164
	ctx.lr = 0x82FEDEF0;
	sub_831A8130(ctx, base);
	// 82FEDEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDEF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEDEF8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FEDEFC: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEDF00: 839D0008  lwz r28, 8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDF04: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDF08: 4182009C  beq 0x82fedfa4
	if ctx.cr[0].eq {
	pc = 0x82FEDFA4; continue 'dispatch;
	}
	// 82FEDF0C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDF10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEDF14: 409A0014  bne cr6, 0x82fedf28
	if !ctx.cr[6].eq {
	pc = 0x82FEDF28; continue 'dispatch;
	}
	// 82FEDF18: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEDF1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEDF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEDF24: 419A0008  beq cr6, 0x82fedf2c
	if ctx.cr[6].eq {
	pc = 0x82FEDF2C; continue 'dispatch;
	}
	// 82FEDF28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FEDF2C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEDF30: 40820074  bne 0x82fedfa4
	if !ctx.cr[0].eq {
	pc = 0x82FEDFA4; continue 'dispatch;
	}
	// 82FEDF34: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDF38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDF3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDF40: 41820064  beq 0x82fedfa4
	if ctx.cr[0].eq {
	pc = 0x82FEDFA4; continue 'dispatch;
	}
	// 82FEDF44: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82FEDF48: 48000038  b 0x82fedf80
	pc = 0x82FEDF80; continue 'dispatch;
	// 82FEDF4C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDF50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEDF54: 409A0014  bne cr6, 0x82fedf68
	if !ctx.cr[6].eq {
	pc = 0x82FEDF68; continue 'dispatch;
	}
	// 82FEDF58: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEDF5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEDF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEDF64: 419A0008  beq cr6, 0x82fedf6c
	if ctx.cr[6].eq {
	pc = 0x82FEDF6C; continue 'dispatch;
	}
	// 82FEDF68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FEDF6C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEDF70: 40820024  bne 0x82fedf94
	if !ctx.cr[0].eq {
	pc = 0x82FEDF94; continue 'dispatch;
	}
	// 82FEDF74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FEDF78: 419A002C  beq cr6, 0x82fedfa4
	if ctx.cr[6].eq {
	pc = 0x82FEDFA4; continue 'dispatch;
	}
	// 82FEDF7C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82FEDF80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEDF84: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEDF88: 4BFFFCC1  bl 0x82fedc48
	ctx.lr = 0x82FEDF8C;
	sub_82FEDC48(ctx, base);
	// 82FEDF8C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FEDF90: 4082FFBC  bne 0x82fedf4c
	if !ctx.cr[0].eq {
	pc = 0x82FEDF4C; continue 'dispatch;
	}
	// 82FEDF94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEDF98: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDF9C: 4BFFFCAD  bl 0x82fedc48
	ctx.lr = 0x82FEDFA0;
	sub_82FEDC48(ctx, base);
	// 82FEDFA0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FEDFA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEDFA8: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FEDFAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FEDFB0: 481BA204  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEDFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEDFB8 size=320
    let mut pc: u32 = 0x82FEDFB8;
    'dispatch: loop {
        match pc {
            0x82FEDFB8 => {
    //   block [0x82FEDFB8..0x82FEE0F8)
	// 82FEDFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEDFBC: 481BA1A9  bl 0x831a8164
	ctx.lr = 0x82FEDFC0;
	sub_831A8130(ctx, base);
	// 82FEDFC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEDFC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEDFC8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEDFCC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDFD0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FEDFD4: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEDFD8: 40820114  bne 0x82fee0ec
	if !ctx.cr[0].eq {
	pc = 0x82FEE0EC; continue 'dispatch;
	}
	// 82FEDFDC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FEDFE0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEDFE4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FEDFE8: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEDFEC: 616B8049  ori r11, r11, 0x8049
	ctx.r[11].u64 = ctx.r[11].u64 | 32841;
	// 82FEDFF0: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 82FEDFF4: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEDFF8: 7F7E58AE  lbzx r27, r30, r11
	ctx.r[27].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEDFFC: 7F9E502E  lwzx r28, r30, r10
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEE000: 41820014  beq 0x82fee014
	if ctx.cr[0].eq {
	pc = 0x82FEE014; continue 'dispatch;
	}
	// 82FEE004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE008: 48004181  bl 0x82ff2188
	ctx.lr = 0x82FEE00C;
	sub_82FF2188(ctx, base);
	// 82FEE00C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE010: 4BFEA2D1  bl 0x82fd82e0
	ctx.lr = 0x82FEE014;
	sub_82FD82E0(ctx, base);
	// 82FEE014: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE018: 4BFFFBD1  bl 0x82fedbe8
	ctx.lr = 0x82FEE01C;
	sub_82FEDBE8(ctx, base);
	// 82FEE01C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FEE020: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE024: 4BFFFBC5  bl 0x82fedbe8
	ctx.lr = 0x82FEE028;
	sub_82FEDBE8(ctx, base);
	// 82FEE028: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FEE02C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FEE030: 419A0010  beq cr6, 0x82fee040
	if ctx.cr[6].eq {
	pc = 0x82FEE040; continue 'dispatch;
	}
	// 82FEE034: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEE038: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE03C: 4082000C  bne 0x82fee048
	if !ctx.cr[0].eq {
	pc = 0x82FEE048; continue 'dispatch;
	}
	// 82FEE040: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE044: 4182001C  beq 0x82fee060
	if ctx.cr[0].eq {
	pc = 0x82FEE060; continue 'dispatch;
	}
	// 82FEE048: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEE04C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEE050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEE054: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82FEE058: 388BC778  addi r4, r11, -0x3888
	ctx.r[4].s64 = ctx.r[11].s64 + -14472;
	// 82FEE05C: 481C2BCD  bl 0x831b0c28
	ctx.lr = 0x82FEE060;
	sub_831B0C28(ctx, base);
	// 82FEE060: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82FEE064: 615E8004  ori r30, r10, 0x8004
	ctx.r[30].u64 = ctx.r[10].u64 | 32772;
	// 82FEE068: 48000068  b 0x82fee0d0
	pc = 0x82FEE0D0; continue 'dispatch;
	// 82FEE06C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE070: 48004CF9  bl 0x82ff2d68
	ctx.lr = 0x82FEE074;
	sub_82FF2D68(ctx, base);
	// 82FEE074: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE078: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FEE07C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEE080: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE084: 40820060  bne 0x82fee0e4
	if !ctx.cr[0].eq {
	pc = 0x82FEE0E4; continue 'dispatch;
	}
	// 82FEE088: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE08C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE090: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FEE094: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE098: 40820054  bne 0x82fee0ec
	if !ctx.cr[0].eq {
	pc = 0x82FEE0EC; continue 'dispatch;
	}
	// 82FEE09C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE0A0: 41820014  beq 0x82fee0b4
	if ctx.cr[0].eq {
	pc = 0x82FEE0B4; continue 'dispatch;
	}
	// 82FEE0A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEE0A8: 480040E1  bl 0x82ff2188
	ctx.lr = 0x82FEE0AC;
	sub_82FF2188(ctx, base);
	// 82FEE0AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEE0B0: 4BFEA231  bl 0x82fd82e0
	ctx.lr = 0x82FEE0B4;
	sub_82FD82E0(ctx, base);
	// 82FEE0B4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE0B8: 4BFFFB31  bl 0x82fedbe8
	ctx.lr = 0x82FEE0BC;
	sub_82FEDBE8(ctx, base);
	// 82FEE0BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FEE0C0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE0C4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FEE0C8: 4BFFFB21  bl 0x82fedbe8
	ctx.lr = 0x82FEE0CC;
	sub_82FEDBE8(ctx, base);
	// 82FEE0CC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FEE0D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE0D4: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FEE0D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEE0DC: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE0E0: 4182FF8C  beq 0x82fee06c
	if ctx.cr[0].eq {
	pc = 0x82FEE06C; continue 'dispatch;
	}
	// 82FEE0E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEE0E8: 48000008  b 0x82fee0f0
	pc = 0x82FEE0F0; continue 'dispatch;
	// 82FEE0EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE0F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FEE0F4: 481BA0C0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEE0F8 size=20
    let mut pc: u32 = 0x82FEE0F8;
    'dispatch: loop {
        match pc {
            0x82FEE0F8 => {
    //   block [0x82FEE0F8..0x82FEE10C)
	// 82FEE0F8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEE0FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEE100: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEE104: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FEE108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE110 size=100
    let mut pc: u32 = 0x82FEE110;
    'dispatch: loop {
        match pc {
            0x82FEE110 => {
    //   block [0x82FEE110..0x82FEE174)
	// 82FEE110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE11C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE124: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE128: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE12C: 4BFFF98D  bl 0x82fedab8
	ctx.lr = 0x82FEE130;
	sub_82FEDAB8(ctx, base);
	// 82FEE130: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE134: 40820028  bne 0x82fee15c
	if !ctx.cr[0].eq {
	pc = 0x82FEE15C; continue 'dispatch;
	}
	// 82FEE138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE13C: 4BFFFE7D  bl 0x82fedfb8
	ctx.lr = 0x82FEE140;
	sub_82FEDFB8(ctx, base);
	// 82FEE140: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE144: 4082000C  bne 0x82fee150
	if !ctx.cr[0].eq {
	pc = 0x82FEE150; continue 'dispatch;
	}
	// 82FEE148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE14C: 48000014  b 0x82fee160
	pc = 0x82FEE160; continue 'dispatch;
	// 82FEE150: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE154: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE158: 4BFFF961  bl 0x82fedab8
	ctx.lr = 0x82FEE15C;
	sub_82FEDAB8(ctx, base);
	// 82FEE15C: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEE160: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE16C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE178 size=108
    let mut pc: u32 = 0x82FEE178;
    'dispatch: loop {
        match pc {
            0x82FEE178 => {
    //   block [0x82FEE178..0x82FEE1E4)
	// 82FEE178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEE184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE18C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FEE190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEE194: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEE198: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE19C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FEE1A0: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FEE1A4: 48000014  b 0x82fee1b8
	pc = 0x82FEE1B8; continue 'dispatch;
	// 82FEE1A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE1AC: 4BFFFE0D  bl 0x82fedfb8
	ctx.lr = 0x82FEE1B0;
	sub_82FEDFB8(ctx, base);
	// 82FEE1B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE1B4: 41820018  beq 0x82fee1cc
	if ctx.cr[0].eq {
	pc = 0x82FEE1CC; continue 'dispatch;
	}
	// 82FEE1B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEE1BC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE1C0: 480054C9  bl 0x82ff3688
	ctx.lr = 0x82FEE1C4;
	sub_82FF3688(ctx, base);
	// 82FEE1C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE1C8: 4182FFE0  beq 0x82fee1a8
	if ctx.cr[0].eq {
	pc = 0x82FEE1A8; continue 'dispatch;
	}
	// 82FEE1CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE1D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEE1DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE1E8 size=92
    let mut pc: u32 = 0x82FEE1E8;
    'dispatch: loop {
        match pc {
            0x82FEE1E8 => {
    //   block [0x82FEE1E8..0x82FEE244)
	// 82FEE1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE1EC: 481B9F81  bl 0x831a816c
	ctx.lr = 0x82FEE1F0;
	sub_831A8130(ctx, base);
	// 82FEE1F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE1F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FEE1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEE1FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEE200: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FEE204: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE208: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FEE20C: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FEE210: 48000014  b 0x82fee224
	pc = 0x82FEE224; continue 'dispatch;
	// 82FEE214: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE218: 4BFFFDA1  bl 0x82fedfb8
	ctx.lr = 0x82FEE21C;
	sub_82FEDFB8(ctx, base);
	// 82FEE21C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE220: 4182001C  beq 0x82fee23c
	if ctx.cr[0].eq {
	pc = 0x82FEE23C; continue 'dispatch;
	}
	// 82FEE224: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FEE228: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE22C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEE230: 48005509  bl 0x82ff3738
	ctx.lr = 0x82FEE234;
	sub_82FF3738(ctx, base);
	// 82FEE234: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE238: 4182FFDC  beq 0x82fee214
	if ctx.cr[0].eq {
	pc = 0x82FEE214; continue 'dispatch;
	}
	// 82FEE23C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE240: 481B9F7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE248 size=100
    let mut pc: u32 = 0x82FEE248;
    'dispatch: loop {
        match pc {
            0x82FEE248 => {
    //   block [0x82FEE248..0x82FEE2AC)
	// 82FEE248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE254: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE25C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE260: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE264: 4BFFEC25  bl 0x82fece88
	ctx.lr = 0x82FEE268;
	sub_82FECE88(ctx, base);
	// 82FEE268: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE26C: 40820028  bne 0x82fee294
	if !ctx.cr[0].eq {
	pc = 0x82FEE294; continue 'dispatch;
	}
	// 82FEE270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE274: 4BFFFD45  bl 0x82fedfb8
	ctx.lr = 0x82FEE278;
	sub_82FEDFB8(ctx, base);
	// 82FEE278: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE27C: 4082000C  bne 0x82fee288
	if !ctx.cr[0].eq {
	pc = 0x82FEE288; continue 'dispatch;
	}
	// 82FEE280: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE284: 48000014  b 0x82fee298
	pc = 0x82FEE298; continue 'dispatch;
	// 82FEE288: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE28C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE290: 4BFFEBF9  bl 0x82fece88
	ctx.lr = 0x82FEE294;
	sub_82FECE88(ctx, base);
	// 82FEE294: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEE298: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE2A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE2A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE2B0 size=128
    let mut pc: u32 = 0x82FEE2B0;
    'dispatch: loop {
        match pc {
            0x82FEE2B0 => {
    //   block [0x82FEE2B0..0x82FEE330)
	// 82FEE2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE2B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEE2BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE2C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE2C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEE2CC: 4800002C  b 0x82fee2f8
	pc = 0x82FEE2F8; continue 'dispatch;
	// 82FEE2D0: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FEE2D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE2D8: 614AC019  ori r10, r10, 0xc019
	ctx.r[10].u64 = ctx.r[10].u64 | 49177;
	// 82FEE2DC: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEE2E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE2E4: 41820030  beq 0x82fee314
	if ctx.cr[0].eq {
	pc = 0x82FEE314; continue 'dispatch;
	}
	// 82FEE2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE2EC: 4BFFFCCD  bl 0x82fedfb8
	ctx.lr = 0x82FEE2F0;
	sub_82FEDFB8(ctx, base);
	// 82FEE2F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE2F4: 41820020  beq 0x82fee314
	if ctx.cr[0].eq {
	pc = 0x82FEE314; continue 'dispatch;
	}
	// 82FEE2F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEE2FC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE300: 48005131  bl 0x82ff3430
	ctx.lr = 0x82FEE304;
	sub_82FF3430(ctx, base);
	// 82FEE304: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE308: 4182FFC8  beq 0x82fee2d0
	if ctx.cr[0].eq {
	pc = 0x82FEE2D0; continue 'dispatch;
	}
	// 82FEE30C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEE310: 48000008  b 0x82fee318
	pc = 0x82FEE318; continue 'dispatch;
	// 82FEE314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE318: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE324: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEE328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE330 size=112
    let mut pc: u32 = 0x82FEE330;
    'dispatch: loop {
        match pc {
            0x82FEE330 => {
    //   block [0x82FEE330..0x82FEE3A0)
	// 82FEE330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE33C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE340: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE344: 4800002C  b 0x82fee370
	pc = 0x82FEE370; continue 'dispatch;
	// 82FEE348: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FEE34C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE350: 614AC019  ori r10, r10, 0xc019
	ctx.r[10].u64 = ctx.r[10].u64 | 49177;
	// 82FEE354: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEE358: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE35C: 4182002C  beq 0x82fee388
	if ctx.cr[0].eq {
	pc = 0x82FEE388; continue 'dispatch;
	}
	// 82FEE360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE364: 4BFFFC55  bl 0x82fedfb8
	ctx.lr = 0x82FEE368;
	sub_82FEDFB8(ctx, base);
	// 82FEE368: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE36C: 4182001C  beq 0x82fee388
	if ctx.cr[0].eq {
	pc = 0x82FEE388; continue 'dispatch;
	}
	// 82FEE370: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE374: 4800558D  bl 0x82ff3900
	ctx.lr = 0x82FEE378;
	sub_82FF3900(ctx, base);
	// 82FEE378: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE37C: 4182FFCC  beq 0x82fee348
	if ctx.cr[0].eq {
	pc = 0x82FEE348; continue 'dispatch;
	}
	// 82FEE380: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEE384: 48000008  b 0x82fee38c
	pc = 0x82FEE38C; continue 'dispatch;
	// 82FEE388: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE38C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE3A0 size=128
    let mut pc: u32 = 0x82FEE3A0;
    'dispatch: loop {
        match pc {
            0x82FEE3A0 => {
    //   block [0x82FEE3A0..0x82FEE420)
	// 82FEE3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE3A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEE3AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE3B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE3B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE3B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEE3BC: 4800002C  b 0x82fee3e8
	pc = 0x82FEE3E8; continue 'dispatch;
	// 82FEE3C0: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FEE3C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE3C8: 614AC019  ori r10, r10, 0xc019
	ctx.r[10].u64 = ctx.r[10].u64 | 49177;
	// 82FEE3CC: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEE3D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE3D4: 41820030  beq 0x82fee404
	if ctx.cr[0].eq {
	pc = 0x82FEE404; continue 'dispatch;
	}
	// 82FEE3D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE3DC: 4BFFFBDD  bl 0x82fedfb8
	ctx.lr = 0x82FEE3E0;
	sub_82FEDFB8(ctx, base);
	// 82FEE3E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE3E4: 41820020  beq 0x82fee404
	if ctx.cr[0].eq {
	pc = 0x82FEE404; continue 'dispatch;
	}
	// 82FEE3E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEE3EC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE3F0: 48004F91  bl 0x82ff3380
	ctx.lr = 0x82FEE3F4;
	sub_82FF3380(ctx, base);
	// 82FEE3F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE3F8: 4182FFC8  beq 0x82fee3c0
	if ctx.cr[0].eq {
	pc = 0x82FEE3C0; continue 'dispatch;
	}
	// 82FEE3FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEE400: 48000008  b 0x82fee408
	pc = 0x82FEE408; continue 'dispatch;
	// 82FEE404: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE408: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE414: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEE418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE420 size=124
    let mut pc: u32 = 0x82FEE420;
    'dispatch: loop {
        match pc {
            0x82FEE420 => {
    //   block [0x82FEE420..0x82FEE49C)
	// 82FEE420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE424: 481B9D49  bl 0x831a816c
	ctx.lr = 0x82FEE428;
	sub_831A8130(ctx, base);
	// 82FEE428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE42C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE430: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEE434: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEE438: 48000024  b 0x82fee45c
	pc = 0x82FEE45C; continue 'dispatch;
	// 82FEE43C: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEE440: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE444: 41820008  beq 0x82fee44c
	if ctx.cr[0].eq {
	pc = 0x82FEE44C; continue 'dispatch;
	}
	// 82FEE448: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82FEE44C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE450: 4BFFFB69  bl 0x82fedfb8
	ctx.lr = 0x82FEE454;
	sub_82FEDFB8(ctx, base);
	// 82FEE454: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE458: 4182001C  beq 0x82fee474
	if ctx.cr[0].eq {
	pc = 0x82FEE474; continue 'dispatch;
	}
	// 82FEE45C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FEE460: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE464: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE468: 48005391  bl 0x82ff37f8
	ctx.lr = 0x82FEE46C;
	sub_82FF37F8(ctx, base);
	// 82FEE46C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE470: 4182FFCC  beq 0x82fee43c
	if ctx.cr[0].eq {
	pc = 0x82FEE43C; continue 'dispatch;
	}
	// 82FEE474: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEE478: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE47C: 40820010  bne 0x82fee48c
	if !ctx.cr[0].eq {
	pc = 0x82FEE48C; continue 'dispatch;
	}
	// 82FEE480: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEE488: 41820008  beq 0x82fee490
	if ctx.cr[0].eq {
	pc = 0x82FEE490; continue 'dispatch;
	}
	// 82FEE48C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FEE490: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FEE494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FEE498: 481B9D24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE4A0 size=92
    let mut pc: u32 = 0x82FEE4A0;
    'dispatch: loop {
        match pc {
            0x82FEE4A0 => {
    //   block [0x82FEE4A0..0x82FEE4FC)
	// 82FEE4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE4A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEE4AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE4B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE4B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEE4BC: 4BFFFC55  bl 0x82fee110
	ctx.lr = 0x82FEE4C0;
	sub_82FEE110(ctx, base);
	// 82FEE4C0: 546B043F  clrlwi. r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE4C4: 41820020  beq 0x82fee4e4
	if ctx.cr[0].eq {
	pc = 0x82FEE4E4; continue 'dispatch;
	}
	// 82FEE4C8: 57DE043E  clrlwi r30, r30, 0x10
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 82FEE4CC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FEE4D0: 419A0014  beq cr6, 0x82fee4e4
	if ctx.cr[6].eq {
	pc = 0x82FEE4E4; continue 'dispatch;
	}
	// 82FEE4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE4D8: 4BFFFC39  bl 0x82fee110
	ctx.lr = 0x82FEE4DC;
	sub_82FEE110(ctx, base);
	// 82FEE4DC: 546B043F  clrlwi. r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE4E0: 4082FFEC  bne 0x82fee4cc
	if !ctx.cr[0].eq {
	pc = 0x82FEE4CC; continue 'dispatch;
	}
	// 82FEE4E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE4F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEE4F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE500 size=84
    let mut pc: u32 = 0x82FEE500;
    'dispatch: loop {
        match pc {
            0x82FEE500 => {
    //   block [0x82FEE500..0x82FEE554)
	// 82FEE500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE504: 481B9C69  bl 0x831a816c
	ctx.lr = 0x82FEE508;
	sub_831A8130(ctx, base);
	// 82FEE508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE50C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEE510: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FEE514: 48000020  b 0x82fee534
	pc = 0x82FEE534; continue 'dispatch;
	// 82FEE518: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEE51C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEE520: 4BFE3891  bl 0x82fd1db0
	ctx.lr = 0x82FEE524;
	sub_82FD1DB0(ctx, base);
	// 82FEE524: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FEE528: 409A0020  bne cr6, 0x82fee548
	if !ctx.cr[6].eq {
	pc = 0x82FEE548; continue 'dispatch;
	}
	// 82FEE52C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE530: 4BFFFBE1  bl 0x82fee110
	ctx.lr = 0x82FEE534;
	sub_82FEE110(ctx, base);
	// 82FEE534: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE538: 4BFFFD11  bl 0x82fee248
	ctx.lr = 0x82FEE53C;
	sub_82FEE248(ctx, base);
	// 82FEE53C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE540: 57EB043F  clrlwi. r11, r31, 0x10
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE544: 4082FFD4  bne 0x82fee518
	if !ctx.cr[0].eq {
	pc = 0x82FEE518; continue 'dispatch;
	}
	// 82FEE548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE54C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE550: 481B9C6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE558 size=112
    let mut pc: u32 = 0x82FEE558;
    'dispatch: loop {
        match pc {
            0x82FEE558 => {
    //   block [0x82FEE558..0x82FEE5C8)
	// 82FEE558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE55C: 481B9C11  bl 0x831a816c
	ctx.lr = 0x82FEE560;
	sub_831A8130(ctx, base);
	// 82FEE560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE564: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEE568: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FEE56C: 4800003C  b 0x82fee5a8
	pc = 0x82FEE5A8; continue 'dispatch;
	// 82FEE570: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FEE574: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE578: 61298054  ori r9, r9, 0x8054
	ctx.r[9].u64 = ctx.r[9].u64 | 32852;
	// 82FEE57C: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FEE580: 7D6A58AE  lbzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEE584: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE588: 40820034  bne 0x82fee5bc
	if !ctx.cr[0].eq {
	pc = 0x82FEE5BC; continue 'dispatch;
	}
	// 82FEE58C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEE590: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEE594: 4BFE381D  bl 0x82fd1db0
	ctx.lr = 0x82FEE598;
	sub_82FD1DB0(ctx, base);
	// 82FEE598: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FEE59C: 409A0020  bne cr6, 0x82fee5bc
	if !ctx.cr[6].eq {
	pc = 0x82FEE5BC; continue 'dispatch;
	}
	// 82FEE5A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE5A4: 4BFFFB6D  bl 0x82fee110
	ctx.lr = 0x82FEE5A8;
	sub_82FEE110(ctx, base);
	// 82FEE5A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEE5AC: 4BFFFC9D  bl 0x82fee248
	ctx.lr = 0x82FEE5B0;
	sub_82FEE248(ctx, base);
	// 82FEE5B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEE5B4: 57EB043F  clrlwi. r11, r31, 0x10
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE5B8: 4082FFB8  bne 0x82fee570
	if !ctx.cr[0].eq {
	pc = 0x82FEE570; continue 'dispatch;
	}
	// 82FEE5BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEE5C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE5C4: 481B9BF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE5C8 size=48
    let mut pc: u32 = 0x82FEE5C8;
    'dispatch: loop {
        match pc {
            0x82FEE5C8 => {
    //   block [0x82FEE5C8..0x82FEE5F8)
	// 82FEE5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE5D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE5D8: 4BFFF911  bl 0x82fedee8
	ctx.lr = 0x82FEE5DC;
	sub_82FEDEE8(ctx, base);
	// 82FEE5DC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FEE5E0: 616BC014  ori r11, r11, 0xc014
	ctx.r[11].u64 = ctx.r[11].u64 | 49172;
	// 82FEE5E4: 7C63582E  lwzx r3, r3, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEE5E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE5F8 size=168
    let mut pc: u32 = 0x82FEE5F8;
    'dispatch: loop {
        match pc {
            0x82FEE5F8 => {
    //   block [0x82FEE5F8..0x82FEE6A0)
	// 82FEE5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEE604: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE608: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE60C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FEE610: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE614: 419A005C  beq cr6, 0x82fee670
	if ctx.cr[6].eq {
	pc = 0x82FEE670; continue 'dispatch;
	}
	// 82FEE618: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE61C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE620: 419A0050  beq cr6, 0x82fee670
	if ctx.cr[6].eq {
	pc = 0x82FEE670; continue 'dispatch;
	}
	// 82FEE624: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE628: 4BFFF8C1  bl 0x82fedee8
	ctx.lr = 0x82FEE62C;
	sub_82FEDEE8(ctx, base);
	// 82FEE62C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FEE630: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FEE634: 616B8040  ori r11, r11, 0x8040
	ctx.r[11].u64 = ctx.r[11].u64 | 32832;
	// 82FEE638: 614AC01C  ori r10, r10, 0xc01c
	ctx.r[10].u64 = ctx.r[10].u64 | 49180;
	// 82FEE63C: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 82FEE640: 3D000001  lis r8, 1
	ctx.r[8].s64 = 65536;
	// 82FEE644: 6129C00C  ori r9, r9, 0xc00c
	ctx.r[9].u64 = ctx.r[9].u64 | 49164;
	// 82FEE648: 7D63582E  lwzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEE64C: 6108C008  ori r8, r8, 0xc008
	ctx.r[8].u64 = ctx.r[8].u64 | 49160;
	// 82FEE650: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEE654: 7D63502E  lwzx r11, r3, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEE658: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FEE65C: 7D63482E  lwzx r11, r3, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FEE660: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FEE664: 7D63402E  lwzx r11, r3, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FEE668: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FEE66C: 48000020  b 0x82fee68c
	pc = 0x82FEE68C; continue 'dispatch;
	// 82FEE670: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEE674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEE678: 396B8158  addi r11, r11, -0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -32424;
	// 82FEE67C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FEE680: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEE684: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FEE688: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FEE68C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEE690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEE69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEE6A0 size=8
    let mut pc: u32 = 0x82FEE6A0;
    'dispatch: loop {
        match pc {
            0x82FEE6A0 => {
    //   block [0x82FEE6A0..0x82FEE6A8)
	// 82FEE6A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEE6A4: 8213DD00  lwz r16, -0x2300(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-8960 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE6A8 size=376
    let mut pc: u32 = 0x82FEE6A8;
    'dispatch: loop {
        match pc {
            0x82FEE6A8 => {
    //   block [0x82FEE6A8..0x82FEE820)
	// 82FEE6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE6AC: 481B9AB1  bl 0x831a815c
	ctx.lr = 0x82FEE6B0;
	sub_831A8130(ctx, base);
	// 82FEE6B0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FEE6B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE6B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEE6BC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FEE6C0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82FEE6C4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FEE6C8: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FEE6CC: 419A0058  beq cr6, 0x82fee724
	if ctx.cr[6].eq {
	pc = 0x82FEE724; continue 'dispatch;
	}
	// 82FEE6D0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE6D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE6D8: 4182004C  beq 0x82fee724
	if ctx.cr[0].eq {
	pc = 0x82FEE724; continue 'dispatch;
	}
	// 82FEE6DC: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE6E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEE6E4: 837A0010  lwz r27, 0x10(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE6E8: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE6EC: 41820038  beq 0x82fee724
	if ctx.cr[0].eq {
	pc = 0x82FEE724; continue 'dispatch;
	}
	// 82FEE6F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEE6F4: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE6F8: 4BFFF551  bl 0x82fedc48
	ctx.lr = 0x82FEE6FC;
	sub_82FEDC48(ctx, base);
	// 82FEE6FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE700: 41820018  beq 0x82fee718
	if ctx.cr[0].eq {
	pc = 0x82FEE718; continue 'dispatch;
	}
	// 82FEE704: 80830010  lwz r4, 0x10(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE708: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FEE70C: 4BFE5535  bl 0x82fd3c40
	ctx.lr = 0x82FEE710;
	sub_82FD3C40(ctx, base);
	// 82FEE710: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEE714: 4082005C  bne 0x82fee770
	if !ctx.cr[0].eq {
	pc = 0x82FEE770; continue 'dispatch;
	}
	// 82FEE718: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FEE71C: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FEE720: 4198FFD0  blt cr6, 0x82fee6f0
	if ctx.cr[6].lt {
	pc = 0x82FEE6F0; continue 'dispatch;
	}
	// 82FEE724: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE72C: 409A006C  bne cr6, 0x82fee798
	if !ctx.cr[6].eq {
	pc = 0x82FEE798; continue 'dispatch;
	}
	// 82FEE730: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEE734: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEE738: 4BFE9B61  bl 0x82fd8298
	ctx.lr = 0x82FEE73C;
	sub_82FD8298(ctx, base);
	// 82FEE73C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FEE740: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEE744: 4182004C  beq 0x82fee790
	if ctx.cr[0].eq {
	pc = 0x82FEE790; continue 'dispatch;
	}
	// 82FEE748: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FEE74C: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEE750: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82FEE754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEE758: 4BFFF2E1  bl 0x82feda38
	ctx.lr = 0x82FEE75C;
	sub_82FEDA38(ctx, base);
	// 82FEE75C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEE760: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82FEE764: 396BDC68  addi r11, r11, -0x2398
	ctx.r[11].s64 = ctx.r[11].s64 + -9112;
	// 82FEE768: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEE76C: 48000028  b 0x82fee794
	pc = 0x82FEE794; continue 'dispatch;
	// 82FEE770: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82FEE774: 419A0014  beq cr6, 0x82fee788
	if ctx.cr[6].eq {
	pc = 0x82FEE788; continue 'dispatch;
	}
	// 82FEE778: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FEE77C: 48003A0D  bl 0x82ff2188
	ctx.lr = 0x82FEE780;
	sub_82FF2188(ctx, base);
	// 82FEE780: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FEE784: 4BFE9B5D  bl 0x82fd82e0
	ctx.lr = 0x82FEE788;
	sub_82FD82E0(ctx, base);
	// 82FEE788: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE78C: 4800008C  b 0x82fee818
	pc = 0x82FEE818; continue 'dispatch;
	// 82FEE790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEE794: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FEE798: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE79C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE7A0: 409A004C  bne cr6, 0x82fee7ec
	if !ctx.cr[6].eq {
	pc = 0x82FEE7EC; continue 'dispatch;
	}
	// 82FEE7A4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEE7A8: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEE7AC: 4BFE9AED  bl 0x82fd8298
	ctx.lr = 0x82FEE7B0;
	sub_82FD8298(ctx, base);
	// 82FEE7B0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FEE7B4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEE7B8: 4182002C  beq 0x82fee7e4
	if ctx.cr[0].eq {
	pc = 0x82FEE7E4; continue 'dispatch;
	}
	// 82FEE7BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEE7C0: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEE7C4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82FEE7C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEE7CC: 4805E025  bl 0x8304c7f0
	ctx.lr = 0x82FEE7D0;
	sub_8304C7F0(ctx, base);
	// 82FEE7D0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FEE7D4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82FEE7D8: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 82FEE7DC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEE7E0: 48000008  b 0x82fee7e8
	pc = 0x82FEE7E8; continue 'dispatch;
	// 82FEE7E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEE7E8: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FEE7EC: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE7F0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEE7F4: 41820018  beq 0x82fee80c
	if ctx.cr[0].eq {
	pc = 0x82FEE80C; continue 'dispatch;
	}
	// 82FEE7F8: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE7FC: 4804C955  bl 0x8303b150
	ctx.lr = 0x82FEE800;
	sub_8303B150(ctx, base);
	// 82FEE800: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEE804: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEE808: 4804C949  bl 0x8303b150
	ctx.lr = 0x82FEE80C;
	sub_8303B150(ctx, base);
	// 82FEE80C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEE810: 933E0008  stw r25, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82FEE814: 935E0004  stw r26, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82FEE818: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FEE81C: 481B9990  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE820 size=48
    let mut pc: u32 = 0x82FEE820;
    'dispatch: loop {
        match pc {
            0x82FEE820 => {
    //   block [0x82FEE820..0x82FEE850)
	// 82FEE820: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEE824: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE828: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE82C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE830: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEE834: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEE838: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEE83C: 4BFE9AA5  bl 0x82fd82e0
	ctx.lr = 0x82FEE840;
	sub_82FD82E0(ctx, base);
	// 82FEE840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE850 size=48
    let mut pc: u32 = 0x82FEE850;
    'dispatch: loop {
        match pc {
            0x82FEE850 => {
    //   block [0x82FEE850..0x82FEE880)
	// 82FEE850: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEE854: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE858: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE85C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE860: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEE864: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEE868: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEE86C: 4BFE9A75  bl 0x82fd82e0
	ctx.lr = 0x82FEE870;
	sub_82FD82E0(ctx, base);
	// 82FEE870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE880 size=84
    let mut pc: u32 = 0x82FEE880;
    'dispatch: loop {
        match pc {
            0x82FEE880 => {
    //   block [0x82FEE880..0x82FEE8D4)
	// 82FEE880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE88C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE890: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE894: 409A001C  bne cr6, 0x82fee8b0
	if !ctx.cr[6].eq {
	pc = 0x82FEE8B0; continue 'dispatch;
	}
	// 82FEE898: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE89C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE8A0: 409A0010  bne cr6, 0x82fee8b0
	if !ctx.cr[6].eq {
	pc = 0x82FEE8B0; continue 'dispatch;
	}
	// 82FEE8A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEE8A8: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 82FEE8AC: 48000018  b 0x82fee8c4
	pc = 0x82FEE8C4; continue 'dispatch;
	// 82FEE8B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE8B4: 4BFFF635  bl 0x82fedee8
	ctx.lr = 0x82FEE8B8;
	sub_82FEDEE8(ctx, base);
	// 82FEE8B8: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FEE8BC: 616BC01C  ori r11, r11, 0xc01c
	ctx.r[11].u64 = ctx.r[11].u64 | 49180;
	// 82FEE8C0: 7C63582E  lwzx r3, r3, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEE8C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE8D8 size=84
    let mut pc: u32 = 0x82FEE8D8;
    'dispatch: loop {
        match pc {
            0x82FEE8D8 => {
    //   block [0x82FEE8D8..0x82FEE92C)
	// 82FEE8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE8E4: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE8E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE8EC: 409A001C  bne cr6, 0x82fee908
	if !ctx.cr[6].eq {
	pc = 0x82FEE908; continue 'dispatch;
	}
	// 82FEE8F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE8F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE8F8: 409A0010  bne cr6, 0x82fee908
	if !ctx.cr[6].eq {
	pc = 0x82FEE908; continue 'dispatch;
	}
	// 82FEE8FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEE900: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 82FEE904: 48000018  b 0x82fee91c
	pc = 0x82FEE91C; continue 'dispatch;
	// 82FEE908: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE90C: 4BFFF5DD  bl 0x82fedee8
	ctx.lr = 0x82FEE910;
	sub_82FEDEE8(ctx, base);
	// 82FEE910: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FEE914: 616B8040  ori r11, r11, 0x8040
	ctx.r[11].u64 = ctx.r[11].u64 | 32832;
	// 82FEE918: 7C63582E  lwzx r3, r3, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEE91C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE930 size=80
    let mut pc: u32 = 0x82FEE930;
    'dispatch: loop {
        match pc {
            0x82FEE930 => {
    //   block [0x82FEE930..0x82FEE980)
	// 82FEE930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE93C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE940: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE944: 409A0018  bne cr6, 0x82fee95c
	if !ctx.cr[6].eq {
	pc = 0x82FEE95C; continue 'dispatch;
	}
	// 82FEE948: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE94C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE950: 409A000C  bne cr6, 0x82fee95c
	if !ctx.cr[6].eq {
	pc = 0x82FEE95C; continue 'dispatch;
	}
	// 82FEE954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE958: 48000018  b 0x82fee970
	pc = 0x82FEE970; continue 'dispatch;
	// 82FEE95C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE960: 4BFFF589  bl 0x82fedee8
	ctx.lr = 0x82FEE964;
	sub_82FEDEE8(ctx, base);
	// 82FEE964: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FEE968: 616BC008  ori r11, r11, 0xc008
	ctx.r[11].u64 = ctx.r[11].u64 | 49160;
	// 82FEE96C: 7C63582E  lwzx r3, r3, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEE970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE980 size=80
    let mut pc: u32 = 0x82FEE980;
    'dispatch: loop {
        match pc {
            0x82FEE980 => {
    //   block [0x82FEE980..0x82FEE9D0)
	// 82FEE980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEE988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE98C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEE990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE994: 409A0018  bne cr6, 0x82fee9ac
	if !ctx.cr[6].eq {
	pc = 0x82FEE9AC; continue 'dispatch;
	}
	// 82FEE998: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEE99C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEE9A0: 409A000C  bne cr6, 0x82fee9ac
	if !ctx.cr[6].eq {
	pc = 0x82FEE9AC; continue 'dispatch;
	}
	// 82FEE9A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEE9A8: 48000018  b 0x82fee9c0
	pc = 0x82FEE9C0; continue 'dispatch;
	// 82FEE9AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FEE9B0: 4BFFF539  bl 0x82fedee8
	ctx.lr = 0x82FEE9B4;
	sub_82FEDEE8(ctx, base);
	// 82FEE9B4: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FEE9B8: 616BC00C  ori r11, r11, 0xc00c
	ctx.r[11].u64 = ctx.r[11].u64 | 49164;
	// 82FEE9BC: 7C63582E  lwzx r3, r3, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEE9C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEE9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEE9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEE9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEE9D0 size=8
    let mut pc: u32 = 0x82FEE9D0;
    'dispatch: loop {
        match pc {
            0x82FEE9D0 => {
    //   block [0x82FEE9D0..0x82FEE9D8)
	// 82FEE9D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEE9D4: 8213DD68  lwz r16, -0x2298(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-8856 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEE9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEE9D8 size=140
    let mut pc: u32 = 0x82FEE9D8;
    'dispatch: loop {
        match pc {
            0x82FEE9D8 => {
    //   block [0x82FEE9D8..0x82FEEA64)
	// 82FEE9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEE9DC: 481B9791  bl 0x831a816c
	ctx.lr = 0x82FEE9E0;
	sub_831A8130(ctx, base);
	// 82FEE9E0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEE9E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEE9E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEE9EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEE9F0: 396BDAB0  addi r11, r11, -0x2550
	ctx.r[11].s64 = ctx.r[11].s64 + -9552;
	// 82FEE9F4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FEE9F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEE9FC: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEEA00: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEEA04: 41820014  beq 0x82feea18
	if ctx.cr[0].eq {
	pc = 0x82FEEA18; continue 'dispatch;
	}
	// 82FEEA08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEA0C: 4800377D  bl 0x82ff2188
	ctx.lr = 0x82FEEA10;
	sub_82FF2188(ctx, base);
	// 82FEEA10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEA14: 4BFE98CD  bl 0x82fd82e0
	ctx.lr = 0x82FEEA18;
	sub_82FD82E0(ctx, base);
	// 82FEEA18: 83BE0018  lwz r29, 0x18(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEEA1C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEEA20: 41820014  beq 0x82feea34
	if ctx.cr[0].eq {
	pc = 0x82FEEA34; continue 'dispatch;
	}
	// 82FEEA24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEA28: 4BFFF281  bl 0x82fedca8
	ctx.lr = 0x82FEEA2C;
	sub_82FEDCA8(ctx, base);
	// 82FEEA2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEA30: 4BFE98B1  bl 0x82fd82e0
	ctx.lr = 0x82FEEA34;
	sub_82FD82E0(ctx, base);
	// 82FEEA34: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEEA38: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEEA3C: 41820014  beq 0x82feea50
	if ctx.cr[0].eq {
	pc = 0x82FEEA50; continue 'dispatch;
	}
	// 82FEEA40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEA44: 4805E3AD  bl 0x8304cdf0
	ctx.lr = 0x82FEEA48;
	sub_8304CDF0(ctx, base);
	// 82FEEA48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEA4C: 4BFE9895  bl 0x82fd82e0
	ctx.lr = 0x82FEEA50;
	sub_82FD82E0(ctx, base);
	// 82FEEA50: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEEA54: 396BDA9C  addi r11, r11, -0x2564
	ctx.r[11].s64 = ctx.r[11].s64 + -9572;
	// 82FEEA58: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEEA5C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEEA60: 481B975C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEA64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEA64 size=40
    let mut pc: u32 = 0x82FEEA64;
    'dispatch: loop {
        match pc {
            0x82FEEA64 => {
    //   block [0x82FEEA64..0x82FEEA8C)
	// 82FEEA64: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEEA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEA70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEA74: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEEA78: 480778D1  bl 0x83066348
	ctx.lr = 0x82FEEA7C;
	sub_83066348(ctx, base);
	// 82FEEA7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEEA90 size=8
    let mut pc: u32 = 0x82FEEA90;
    'dispatch: loop {
        match pc {
            0x82FEEA90 => {
    //   block [0x82FEEA90..0x82FEEA98)
	// 82FEEA90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEEA94: 8213DDD0  lwz r16, -0x2230(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-8752 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEA98 size=940
    let mut pc: u32 = 0x82FEEA98;
    'dispatch: loop {
        match pc {
            0x82FEEA98 => {
    //   block [0x82FEEA98..0x82FEEE44)
	// 82FEEA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEA9C: 481B96B5  bl 0x831a8150
	ctx.lr = 0x82FEEAA0;
	sub_831A8130(ctx, base);
	// 82FEEAA0: 3BE1FE70  addi r31, r1, -0x190
	ctx.r[31].s64 = ctx.r[1].s64 + -400;
	// 82FEEAA4: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEAA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEEAAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FEEAB0: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEEAB4: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FEEAB8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FEEABC: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEAC0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82FEEAC4: 93DF01A4  stw r30, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[30].u32 ) };
	// 82FEEAC8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82FEEACC: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 82FEEAD0: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 82FEEAD4: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 82FEEAD8: 4BFF0381  bl 0x82fdee58
	ctx.lr = 0x82FEEADC;
	sub_82FDEE58(ctx, base);
	// 82FEEADC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FEEAE0: 38BF00A0  addi r5, r31, 0xa0
	ctx.r[5].s64 = ctx.r[31].s64 + 160;
	// 82FEEAE4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FEEAE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEAEC: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82FEEAF0: 4BFE39B9  bl 0x82fd24a8
	ctx.lr = 0x82FEEAF4;
	sub_82FD24A8(ctx, base);
	// 82FEEAF4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FEEAF8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEEAFC: 815F00B8  lwz r10, 0xb8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FEEB00: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEEB04: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEEB08: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEEB0C: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 82FEEB10: 839F00B8  lwz r28, 0xb8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FEEB14: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEB18: 4BFF0341  bl 0x82fdee58
	ctx.lr = 0x82FEEB1C;
	sub_82FDEE58(ctx, base);
	// 82FEEB1C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEEB20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEEB24: 41820024  beq 0x82feeb48
	if ctx.cr[0].eq {
	pc = 0x82FEEB48; continue 'dispatch;
	}
	// 82FEEB28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEEB2C: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 82FEEB30: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEEB34: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEEB38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEEB3C: 4E800421  bctrl
	ctx.lr = 0x82FEEB40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEEB40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEEB44: 40820018  bne 0x82feeb5c
	if !ctx.cr[0].eq {
	pc = 0x82FEEB5C; continue 'dispatch;
	}
	// 82FEEB48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEEB4C: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82FEEB50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEEB54: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEEB58: 4BFEAA19  bl 0x82fd9570
	ctx.lr = 0x82FEEB5C;
	sub_82FD9570(ctx, base);
	// 82FEEB5C: 93BA0000  stw r29, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82FEEB60: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEEB64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEEB68: 419A006C  beq cr6, 0x82feebd4
	if ctx.cr[6].eq {
	pc = 0x82FEEBD4; continue 'dispatch;
	}
	// 82FEEB6C: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 82FEEB70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEEB74: 4BFFFA85  bl 0x82fee5f8
	ctx.lr = 0x82FEEB78;
	sub_82FEE5F8(ctx, base);
	// 82FEEB78: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEEB7C: 813F0078  lwz r9, 0x78(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEEB80: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 82FEEB84: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEEB88: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEEB8C: 396B8158  addi r11, r11, -0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -32424;
	// 82FEEB90: 7FAA4B2E  sthx r29, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u16) };
	// 82FEEB94: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82FEEB98: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEEB9C: 937F00C4  stw r27, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[27].u32 ) };
	// 82FEEBA0: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82FEEBA4: 915F00C0  stw r10, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 82FEEBA8: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEEBAC: 915F00C8  stw r10, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 82FEEBB0: 815F0080  lwz r10, 0x80(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82FEEBB4: 915F00CC  stw r10, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[10].u32 ) };
	// 82FEEBB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEEBBC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEEBC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEEBC4: 4E800421  bctrl
	ctx.lr = 0x82FEEBC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEEBC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEEBCC: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FEEBD0: 40820174  bne 0x82feed44
	if !ctx.cr[0].eq {
	pc = 0x82FEED44; continue 'dispatch;
	}
	// 82FEEBD4: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 82FEEBD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEEBDC: 4BFFFA1D  bl 0x82fee5f8
	ctx.lr = 0x82FEEBE0;
	sub_82FEE5F8(ctx, base);
	// 82FEEBE0: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEEBE4: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEBE8: 48025831  bl 0x83014418
	ctx.lr = 0x82FEEBEC;
	sub_83014418(ctx, base);
	// 82FEEBEC: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEEBF0: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEEBF4: 38DF0100  addi r6, r31, 0x100
	ctx.r[6].s64 = ctx.r[31].s64 + 256;
	// 82FEEBF8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEEBFC: 839F0080  lwz r28, 0x80(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82FEEC00: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEEC04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEEC08: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 82FEEC0C: 80BF0078  lwz r5, 0x78(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEEC10: 48026C91  bl 0x830158a0
	ctx.lr = 0x82FEEC14;
	sub_830158A0(ctx, base);
	// 82FEEC14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEEC18: 41820090  beq 0x82feeca8
	if ctx.cr[0].eq {
	pc = 0x82FEECA8; continue 'dispatch;
	}
	// 82FEEC1C: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEEC20: 480258A9  bl 0x830144c8
	ctx.lr = 0x82FEEC24;
	sub_830144C8(ctx, base);
	// 82FEEC24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEEC28: 40820080  bne 0x82feeca8
	if !ctx.cr[0].eq {
	pc = 0x82FEECA8; continue 'dispatch;
	}
	// 82FEEC2C: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FEEC30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEEC34: 41820040  beq 0x82feec74
	if ctx.cr[0].eq {
	pc = 0x82FEEC74; continue 'dispatch;
	}
	// 82FEEC38: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEEC3C: 480258C5  bl 0x83014500
	ctx.lr = 0x82FEEC40;
	sub_83014500(ctx, base);
	// 82FEEC40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEEC44: 41820030  beq 0x82feec74
	if ctx.cr[0].eq {
	pc = 0x82FEEC74; continue 'dispatch;
	}
	// 82FEEC48: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEEC4C: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEC50: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEEC54: 388BDCB8  addi r4, r11, -0x2348
	ctx.r[4].s64 = ctx.r[11].s64 + -9032;
	// 82FEEC58: 38A0023B  li r5, 0x23b
	ctx.r[5].s64 = 571;
	// 82FEEC5C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEEC60: 4BFEAEB9  bl 0x82fd9b18
	ctx.lr = 0x82FEEC64;
	sub_82FD9B18(ctx, base);
	// 82FEEC64: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEEC68: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEEC6C: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FEEC70: 481C1FB9  bl 0x831b0c28
	ctx.lr = 0x82FEEC74;
	sub_831B0C28(ctx, base);
	// 82FEEC74: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82FEEC78: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEC7C: 4BFE961D  bl 0x82fd8298
	ctx.lr = 0x82FEEC80;
	sub_82FD8298(ctx, base);
	// 82FEEC80: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEEC84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEEC88: 41820014  beq 0x82feec9c
	if ctx.cr[0].eq {
	pc = 0x82FEEC9C; continue 'dispatch;
	}
	// 82FEEC8C: 389F0100  addi r4, r31, 0x100
	ctx.r[4].s64 = ctx.r[31].s64 + 256;
	// 82FEEC90: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEC94: 4802557D  bl 0x83014210
	ctx.lr = 0x82FEEC98;
	sub_83014210(ctx, base);
	// 82FEEC98: 48000008  b 0x82feeca0
	pc = 0x82FEECA0; continue 'dispatch;
	// 82FEEC9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEECA0: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FEECA4: 48000098  b 0x82feed3c
	pc = 0x82FEED3C; continue 'dispatch;
	// 82FEECA8: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FEECAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEECB0: 40820108  bne 0x82feedb8
	if !ctx.cr[0].eq {
	pc = 0x82FEEDB8; continue 'dispatch;
	}
	// 82FEECB4: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEECB8: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEECBC: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82FEECC0: 4BFF0199  bl 0x82fdee58
	ctx.lr = 0x82FEECC4;
	sub_82FDEE58(ctx, base);
	// 82FEECC4: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEECC8: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEECCC: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 82FEECD0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEECD4: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 82FEECD8: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEECDC: 4BFEC8E5  bl 0x82fdb5c0
	ctx.lr = 0x82FEECE0;
	sub_82FDB5C0(ctx, base);
	// 82FEECE0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEECE4: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEECE8: 4BFE95B1  bl 0x82fd8298
	ctx.lr = 0x82FEECEC;
	sub_82FD8298(ctx, base);
	// 82FEECEC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEECF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEECF4: 41820028  beq 0x82feed1c
	if ctx.cr[0].eq {
	pc = 0x82FEED1C; continue 'dispatch;
	}
	// 82FEECF8: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FEECFC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEED00: 815F00F8  lwz r10, 0xf8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 82FEED04: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEED08: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 82FEED0C: 80BF00F8  lwz r5, 0xf8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 82FEED10: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEED14: 4BFE9D9D  bl 0x82fd8ab0
	ctx.lr = 0x82FEED18;
	sub_82FD8AB0(ctx, base);
	// 82FEED18: 48000008  b 0x82feed20
	pc = 0x82FEED20; continue 'dispatch;
	// 82FEED1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEED20: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FEED24: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82FEED28: 809F00F8  lwz r4, 0xf8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 82FEED2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEED30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEED34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEED38: 4E800421  bctrl
	ctx.lr = 0x82FEED3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEED3C: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEED40: 48026271  bl 0x83014fb0
	ctx.lr = 0x82FEED44;
	sub_83014FB0(ctx, base);
	// 82FEED44: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEED48: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FEED4C: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 82FEED50: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82FEED54: 893F01E7  lbz r9, 0x1e7(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(487 as u32) ) } as u64;
	// 82FEED58: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82FEED5C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82FEED60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEED64: 4BFFE475  bl 0x82fed1d8
	ctx.lr = 0x82FEED68;
	sub_82FED1D8(ctx, base);
	// 82FEED68: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEED6C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FEED70: 40820074  bne 0x82feede4
	if !ctx.cr[0].eq {
	pc = 0x82FEEDE4; continue 'dispatch;
	}
	// 82FEED74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEED78: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEED7C: 48079F5D  bl 0x83068cd8
	ctx.lr = 0x82FEED80;
	sub_83068CD8(ctx, base);
	// 82FEED80: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FEED84: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEED88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEED8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEED90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEED94: 4E800421  bctrl
	ctx.lr = 0x82FEED98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEED98: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FEED9C: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FEEDA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEEDA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEEDA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEEDAC: 4E800421  bctrl
	ctx.lr = 0x82FEEDB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEEDB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEEDB4: 48000088  b 0x82feee3c
	pc = 0x82FEEE3C; continue 'dispatch;
	// 82FEEDB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEEDBC: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEDC0: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEEDC4: 388BDCB8  addi r4, r11, -0x2348
	ctx.r[4].s64 = ctx.r[11].s64 + -9032;
	// 82FEEDC8: 38A00236  li r5, 0x236
	ctx.r[5].s64 = 566;
	// 82FEEDCC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEEDD0: 4BFEAD49  bl 0x82fd9b18
	ctx.lr = 0x82FEEDD4;
	sub_82FD9B18(ctx, base);
	// 82FEEDD4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEEDD8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEEDDC: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FEEDE0: 481C1E49  bl 0x831b0c28
	ctx.lr = 0x82FEEDE4;
	sub_831B0C28(ctx, base);
	// 82FEEDE4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FEEDE8: 616A8028  ori r10, r11, 0x8028
	ctx.r[10].u64 = ctx.r[11].u64 | 32808;
	// 82FEEDEC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEEDF0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FEEDF4: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FEEDF8: 7D7C512E  stwx r11, r28, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82FEEDFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEEE00: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEEE04: 48079ED5  bl 0x83068cd8
	ctx.lr = 0x82FEEE08;
	sub_83068CD8(ctx, base);
	// 82FEEE08: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FEEE0C: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEEE10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEEE14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEEE18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEEE1C: 4E800421  bctrl
	ctx.lr = 0x82FEEE20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEEE20: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FEEE24: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FEEE28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEEE2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEEE30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEEE34: 4E800421  bctrl
	ctx.lr = 0x82FEEE38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEEE38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEEE3C: 383F0190  addi r1, r31, 0x190
	ctx.r[1].s64 = ctx.r[31].s64 + 400;
	// 82FEEE40: 481B9360  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEE44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEE44 size=40
    let mut pc: u32 = 0x82FEEE44;
    'dispatch: loop {
        match pc {
            0x82FEEE44 => {
    //   block [0x82FEEE44..0x82FEEE6C)
	// 82FEEE44: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEEE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEE50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEE54: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FEEE58: 4BFF0081  bl 0x82fdeed8
	ctx.lr = 0x82FEEE5C;
	sub_82FDEED8(ctx, base);
	// 82FEEE5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEE60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEE64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEE6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEE6C size=40
    let mut pc: u32 = 0x82FEEE6C;
    'dispatch: loop {
        match pc {
            0x82FEEE6C => {
    //   block [0x82FEEE6C..0x82FEEE94)
	// 82FEEE6C: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEEE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEE7C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEEE80: 4BFF0059  bl 0x82fdeed8
	ctx.lr = 0x82FEEE84;
	sub_82FDEED8(ctx, base);
	// 82FEEE84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEE94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEE94 size=40
    let mut pc: u32 = 0x82FEEE94;
    'dispatch: loop {
        match pc {
            0x82FEEE94 => {
    //   block [0x82FEEE94..0x82FEEEBC)
	// 82FEEE94: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEEE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEEA4: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEEEA8: 48026109  bl 0x83014fb0
	ctx.lr = 0x82FEEEAC;
	sub_83014FB0(ctx, base);
	// 82FEEEAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEEBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEEBC size=48
    let mut pc: u32 = 0x82FEEEBC;
    'dispatch: loop {
        match pc {
            0x82FEEEBC => {
    //   block [0x82FEEEBC..0x82FEEEEC)
	// 82FEEEBC: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEEEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEEC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEECC: 817F01A4  lwz r11, 0x1a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 82FEEED0: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEED4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEEED8: 4BFE9409  bl 0x82fd82e0
	ctx.lr = 0x82FEEEDC;
	sub_82FD82E0(ctx, base);
	// 82FEEEDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEEEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEEEC size=40
    let mut pc: u32 = 0x82FEEEEC;
    'dispatch: loop {
        match pc {
            0x82FEEEEC => {
    //   block [0x82FEEEEC..0x82FEEF14)
	// 82FEEEEC: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEEEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEEF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEEFC: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82FEEF00: 4BFEFFD9  bl 0x82fdeed8
	ctx.lr = 0x82FEEF04;
	sub_82FDEED8(ctx, base);
	// 82FEEF04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEF14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEF14 size=48
    let mut pc: u32 = 0x82FEEF14;
    'dispatch: loop {
        match pc {
            0x82FEEF14 => {
    //   block [0x82FEEF14..0x82FEEF44)
	// 82FEEF14: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEEF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEF24: 817F01A4  lwz r11, 0x1a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 82FEEF28: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEF2C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEEF30: 4BFE93B1  bl 0x82fd82e0
	ctx.lr = 0x82FEEF34;
	sub_82FD82E0(ctx, base);
	// 82FEEF34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEF44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEF44 size=40
    let mut pc: u32 = 0x82FEEF44;
    'dispatch: loop {
        match pc {
            0x82FEEF44 => {
    //   block [0x82FEEF44..0x82FEEF6C)
	// 82FEEF44: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEEF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEEF50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEF54: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEEF58: 48029831  bl 0x83018788
	ctx.lr = 0x82FEEF5C;
	sub_83018788(ctx, base);
	// 82FEEF5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEEF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEEF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEEF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEEF70 size=8
    let mut pc: u32 = 0x82FEEF70;
    'dispatch: loop {
        match pc {
            0x82FEEF70 => {
    //   block [0x82FEEF70..0x82FEEF78)
	// 82FEEF70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEEF74: 8213DED8  lwz r16, -0x2128(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-8488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEEF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEEF78 size=952
    let mut pc: u32 = 0x82FEEF78;
    'dispatch: loop {
        match pc {
            0x82FEEF78 => {
    //   block [0x82FEEF78..0x82FEF330)
	// 82FEEF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEEF7C: 481B91D1  bl 0x831a814c
	ctx.lr = 0x82FEEF80;
	sub_831A8130(ctx, base);
	// 82FEEF80: 3BE1FE70  addi r31, r1, -0x190
	ctx.r[31].s64 = ctx.r[1].s64 + -400;
	// 82FEEF84: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEEF88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEEF8C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FEEF90: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEEF94: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEEF98: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FEEF9C: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEFA0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FEEFA4: 93DF01A4  stw r30, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[30].u32 ) };
	// 82FEEFA8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82FEEFAC: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 82FEEFB0: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 82FEEFB4: 7D555378  mr r21, r10
	ctx.r[21].u64 = ctx.r[10].u64;
	// 82FEEFB8: 4BFEFEA1  bl 0x82fdee58
	ctx.lr = 0x82FEEFBC;
	sub_82FDEE58(ctx, base);
	// 82FEEFBC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FEEFC0: 38BF0080  addi r5, r31, 0x80
	ctx.r[5].s64 = ctx.r[31].s64 + 128;
	// 82FEEFC4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FEEFC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEEFCC: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82FEEFD0: 4BFE34D9  bl 0x82fd24a8
	ctx.lr = 0x82FEEFD4;
	sub_82FD24A8(ctx, base);
	// 82FEEFD4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEEFD8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FEEFDC: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FEEFE0: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEEFE4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEEFE8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEEFEC: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 82FEEFF0: 83BF0098  lwz r29, 0x98(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FEEFF4: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEEFF8: 4BFEFE61  bl 0x82fdee58
	ctx.lr = 0x82FEEFFC;
	sub_82FDEE58(ctx, base);
	// 82FEEFFC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEF000: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF004: 41820024  beq 0x82fef028
	if ctx.cr[0].eq {
	pc = 0x82FEF028; continue 'dispatch;
	}
	// 82FEF008: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF00C: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 82FEF010: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEF014: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEF018: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF01C: 4E800421  bctrl
	ctx.lr = 0x82FEF020;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF020: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF024: 40820018  bne 0x82fef03c
	if !ctx.cr[0].eq {
	pc = 0x82FEF03C; continue 'dispatch;
	}
	// 82FEF028: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEF02C: 939F0064  stw r28, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82FEF030: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEF034: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEF038: 4BFEA539  bl 0x82fd9570
	ctx.lr = 0x82FEF03C;
	sub_82FD9570(ctx, base);
	// 82FEF03C: 833F01E4  lwz r25, 0x1e4(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(484 as u32) ) } as u64;
	// 82FEF040: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FEF044: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEF048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEF04C: 419A005C  beq cr6, 0x82fef0a8
	if ctx.cr[6].eq {
	pc = 0x82FEF0A8; continue 'dispatch;
	}
	// 82FEF050: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEF054: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 82FEF058: 813F0078  lwz r9, 0x78(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF05C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEF060: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEF064: 396B8158  addi r11, r11, -0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -32424;
	// 82FEF068: 7F8A4B2E  sthx r28, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[28].u16) };
	// 82FEF06C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82FEF070: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEF074: 935F00A4  stw r26, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[26].u32 ) };
	// 82FEF078: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82FEF07C: 937F00AC  stw r27, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[27].u32 ) };
	// 82FEF080: 915F00A0  stw r10, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82FEF084: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF088: 915F00A8  stw r10, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 82FEF08C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF090: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEF094: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF098: 4E800421  bctrl
	ctx.lr = 0x82FEF09C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF09C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF0A0: 90790000  stw r3, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FEF0A4: 4082018C  bne 0x82fef230
	if !ctx.cr[0].eq {
	pc = 0x82FEF230; continue 'dispatch;
	}
	// 82FEF0A8: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82FEF0AC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FEF0B0: 419A0010  beq cr6, 0x82fef0c0
	if ctx.cr[6].eq {
	pc = 0x82FEF0C0; continue 'dispatch;
	}
	// 82FEF0B4: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF0B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF0BC: 40820014  bne 0x82fef0d0
	if !ctx.cr[0].eq {
	pc = 0x82FEF0D0; continue 'dispatch;
	}
	// 82FEF0C0: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 82FEF0C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEF0C8: 4BFFF531  bl 0x82fee5f8
	ctx.lr = 0x82FEF0CC;
	sub_82FEE5F8(ctx, base);
	// 82FEF0CC: 83BF00E0  lwz r29, 0xe0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82FEF0D0: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEF0D4: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF0D8: 48025341  bl 0x83014418
	ctx.lr = 0x82FEF0DC;
	sub_83014418(ctx, base);
	// 82FEF0DC: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEF0E0: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF0E4: 38DF0100  addi r6, r31, 0x100
	ctx.r[6].s64 = ctx.r[31].s64 + 256;
	// 82FEF0E8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEF0EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEF0F0: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEF0F4: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 82FEF0F8: 80BF0078  lwz r5, 0x78(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF0FC: 480267A5  bl 0x830158a0
	ctx.lr = 0x82FEF100;
	sub_830158A0(ctx, base);
	// 82FEF100: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF104: 41820090  beq 0x82fef194
	if ctx.cr[0].eq {
	pc = 0x82FEF194; continue 'dispatch;
	}
	// 82FEF108: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEF10C: 480253BD  bl 0x830144c8
	ctx.lr = 0x82FEF110;
	sub_830144C8(ctx, base);
	// 82FEF110: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF114: 40820080  bne 0x82fef194
	if !ctx.cr[0].eq {
	pc = 0x82FEF194; continue 'dispatch;
	}
	// 82FEF118: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FEF11C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF120: 41820040  beq 0x82fef160
	if ctx.cr[0].eq {
	pc = 0x82FEF160; continue 'dispatch;
	}
	// 82FEF124: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEF128: 480253D9  bl 0x83014500
	ctx.lr = 0x82FEF12C;
	sub_83014500(ctx, base);
	// 82FEF12C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF130: 41820030  beq 0x82fef160
	if ctx.cr[0].eq {
	pc = 0x82FEF160; continue 'dispatch;
	}
	// 82FEF134: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEF138: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF13C: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEF140: 388BDCB8  addi r4, r11, -0x2348
	ctx.r[4].s64 = ctx.r[11].s64 + -9032;
	// 82FEF144: 38A002AD  li r5, 0x2ad
	ctx.r[5].s64 = 685;
	// 82FEF148: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82FEF14C: 4BFEA9CD  bl 0x82fd9b18
	ctx.lr = 0x82FEF150;
	sub_82FD9B18(ctx, base);
	// 82FEF150: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEF154: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82FEF158: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FEF15C: 481C1ACD  bl 0x831b0c28
	ctx.lr = 0x82FEF160;
	sub_831B0C28(ctx, base);
	// 82FEF160: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82FEF164: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF168: 4BFE9131  bl 0x82fd8298
	ctx.lr = 0x82FEF16C;
	sub_82FD8298(ctx, base);
	// 82FEF16C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEF170: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF174: 41820014  beq 0x82fef188
	if ctx.cr[0].eq {
	pc = 0x82FEF188; continue 'dispatch;
	}
	// 82FEF178: 389F0100  addi r4, r31, 0x100
	ctx.r[4].s64 = ctx.r[31].s64 + 256;
	// 82FEF17C: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF180: 48025091  bl 0x83014210
	ctx.lr = 0x82FEF184;
	sub_83014210(ctx, base);
	// 82FEF184: 48000008  b 0x82fef18c
	pc = 0x82FEF18C; continue 'dispatch;
	// 82FEF188: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEF18C: 90790000  stw r3, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FEF190: 48000098  b 0x82fef228
	pc = 0x82FEF228; continue 'dispatch;
	// 82FEF194: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FEF198: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF19C: 40820108  bne 0x82fef2a4
	if !ctx.cr[0].eq {
	pc = 0x82FEF2A4; continue 'dispatch;
	}
	// 82FEF1A0: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEF1A4: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF1A8: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEF1AC: 4BFEFCAD  bl 0x82fdee58
	ctx.lr = 0x82FEF1B0;
	sub_82FDEE58(ctx, base);
	// 82FEF1B0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEF1B4: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF1B8: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 82FEF1BC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEF1C0: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 82FEF1C4: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF1C8: 4BFEC3F9  bl 0x82fdb5c0
	ctx.lr = 0x82FEF1CC;
	sub_82FDB5C0(ctx, base);
	// 82FEF1CC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEF1D0: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF1D4: 4BFE90C5  bl 0x82fd8298
	ctx.lr = 0x82FEF1D8;
	sub_82FD8298(ctx, base);
	// 82FEF1D8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEF1DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF1E0: 41820028  beq 0x82fef208
	if ctx.cr[0].eq {
	pc = 0x82FEF208; continue 'dispatch;
	}
	// 82FEF1E4: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FEF1E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEF1EC: 815F00D8  lwz r10, 0xd8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEF1F0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEF1F4: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 82FEF1F8: 80BF00D8  lwz r5, 0xd8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEF1FC: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF200: 4BFE98B1  bl 0x82fd8ab0
	ctx.lr = 0x82FEF204;
	sub_82FD8AB0(ctx, base);
	// 82FEF204: 48000008  b 0x82fef20c
	pc = 0x82FEF20C; continue 'dispatch;
	// 82FEF208: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEF20C: 90790000  stw r3, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FEF210: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FEF214: 809F00D8  lwz r4, 0xd8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEF218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF21C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEF220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF224: 4E800421  bctrl
	ctx.lr = 0x82FEF228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF228: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEF22C: 48025D85  bl 0x83014fb0
	ctx.lr = 0x82FEF230;
	sub_83014FB0(ctx, base);
	// 82FEF230: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF234: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FEF238: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82FEF23C: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	// 82FEF240: 893F01EF  lbz r9, 0x1ef(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(495 as u32) ) } as u64;
	// 82FEF244: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82FEF248: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82FEF24C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEF250: 4BFFDF89  bl 0x82fed1d8
	ctx.lr = 0x82FEF254;
	sub_82FED1D8(ctx, base);
	// 82FEF254: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FEF258: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FEF25C: 40820074  bne 0x82fef2d0
	if !ctx.cr[0].eq {
	pc = 0x82FEF2D0; continue 'dispatch;
	}
	// 82FEF260: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEF264: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEF268: 48079A71  bl 0x83068cd8
	ctx.lr = 0x82FEF26C;
	sub_83068CD8(ctx, base);
	// 82FEF26C: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FEF270: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF274: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF278: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEF27C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF280: 4E800421  bctrl
	ctx.lr = 0x82FEF284;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF284: 807F008C  lwz r3, 0x8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FEF288: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FEF28C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF290: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEF294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF298: 4E800421  bctrl
	ctx.lr = 0x82FEF29C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF29C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEF2A0: 48000088  b 0x82fef328
	pc = 0x82FEF328; continue 'dispatch;
	// 82FEF2A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEF2A8: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF2AC: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEF2B0: 388BDCB8  addi r4, r11, -0x2348
	ctx.r[4].s64 = ctx.r[11].s64 + -9032;
	// 82FEF2B4: 38A002A8  li r5, 0x2a8
	ctx.r[5].s64 = 680;
	// 82FEF2B8: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82FEF2BC: 4BFEA85D  bl 0x82fd9b18
	ctx.lr = 0x82FEF2C0;
	sub_82FD9B18(ctx, base);
	// 82FEF2C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEF2C4: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82FEF2C8: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FEF2CC: 481C195D  bl 0x831b0c28
	ctx.lr = 0x82FEF2D0;
	sub_831B0C28(ctx, base);
	// 82FEF2D0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FEF2D4: 616A8028  ori r10, r11, 0x8028
	ctx.r[10].u64 = ctx.r[11].u64 | 32808;
	// 82FEF2D8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEF2DC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FEF2E0: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FEF2E4: 7D7D512E  stwx r11, r29, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82FEF2E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEF2EC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEF2F0: 480799E9  bl 0x83068cd8
	ctx.lr = 0x82FEF2F4;
	sub_83068CD8(ctx, base);
	// 82FEF2F4: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FEF2F8: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FEF2FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF300: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEF304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF308: 4E800421  bctrl
	ctx.lr = 0x82FEF30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF30C: 807F008C  lwz r3, 0x8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FEF310: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FEF314: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF318: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEF31C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF320: 4E800421  bctrl
	ctx.lr = 0x82FEF324;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF324: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEF328: 383F0190  addi r1, r31, 0x190
	ctx.r[1].s64 = ctx.r[31].s64 + 400;
	// 82FEF32C: 481B8E70  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF330 size=40
    let mut pc: u32 = 0x82FEF330;
    'dispatch: loop {
        match pc {
            0x82FEF330 => {
    //   block [0x82FEF330..0x82FEF358)
	// 82FEF330: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEF334: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF338: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF33C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF340: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEF344: 4BFEFB95  bl 0x82fdeed8
	ctx.lr = 0x82FEF348;
	sub_82FDEED8(ctx, base);
	// 82FEF348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF358 size=40
    let mut pc: u32 = 0x82FEF358;
    'dispatch: loop {
        match pc {
            0x82FEF358 => {
    //   block [0x82FEF358..0x82FEF380)
	// 82FEF358: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEF35C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF360: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF364: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF368: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEF36C: 4BFEFB6D  bl 0x82fdeed8
	ctx.lr = 0x82FEF370;
	sub_82FDEED8(ctx, base);
	// 82FEF370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF380 size=40
    let mut pc: u32 = 0x82FEF380;
    'dispatch: loop {
        match pc {
            0x82FEF380 => {
    //   block [0x82FEF380..0x82FEF3A8)
	// 82FEF380: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEF384: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF388: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF38C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF390: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 82FEF394: 48025C1D  bl 0x83014fb0
	ctx.lr = 0x82FEF398;
	sub_83014FB0(ctx, base);
	// 82FEF398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF3A8 size=48
    let mut pc: u32 = 0x82FEF3A8;
    'dispatch: loop {
        match pc {
            0x82FEF3A8 => {
    //   block [0x82FEF3A8..0x82FEF3D8)
	// 82FEF3A8: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEF3AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF3B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF3B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF3B8: 817F01A4  lwz r11, 0x1a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 82FEF3BC: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF3C0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEF3C4: 4BFE8F1D  bl 0x82fd82e0
	ctx.lr = 0x82FEF3C8;
	sub_82FD82E0(ctx, base);
	// 82FEF3C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF3D8 size=40
    let mut pc: u32 = 0x82FEF3D8;
    'dispatch: loop {
        match pc {
            0x82FEF3D8 => {
    //   block [0x82FEF3D8..0x82FEF400)
	// 82FEF3D8: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEF3DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF3E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF3E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF3E8: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEF3EC: 4BFEFAED  bl 0x82fdeed8
	ctx.lr = 0x82FEF3F0;
	sub_82FDEED8(ctx, base);
	// 82FEF3F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF400 size=48
    let mut pc: u32 = 0x82FEF400;
    'dispatch: loop {
        match pc {
            0x82FEF400 => {
    //   block [0x82FEF400..0x82FEF430)
	// 82FEF400: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEF404: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF408: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF40C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF410: 817F01A4  lwz r11, 0x1a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 82FEF414: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEF418: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEF41C: 4BFE8EC5  bl 0x82fd82e0
	ctx.lr = 0x82FEF420;
	sub_82FD82E0(ctx, base);
	// 82FEF420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF430 size=40
    let mut pc: u32 = 0x82FEF430;
    'dispatch: loop {
        match pc {
            0x82FEF430 => {
    //   block [0x82FEF430..0x82FEF458)
	// 82FEF430: 3BECFE70  addi r31, r12, -0x190
	ctx.r[31].s64 = ctx.r[12].s64 + -400;
	// 82FEF434: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF438: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF43C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF440: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEF444: 48029345  bl 0x83018788
	ctx.lr = 0x82FEF448;
	sub_83018788(ctx, base);
	// 82FEF448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF458 size=76
    let mut pc: u32 = 0x82FEF458;
    'dispatch: loop {
        match pc {
            0x82FEF458 => {
    //   block [0x82FEF458..0x82FEF4A4)
	// 82FEF458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEF464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEF468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF46C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEF470: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEF474: 4BFFF565  bl 0x82fee9d8
	ctx.lr = 0x82FEF478;
	sub_82FEE9D8(ctx, base);
	// 82FEF478: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF47C: 4182000C  beq 0x82fef488
	if ctx.cr[0].eq {
	pc = 0x82FEF488; continue 'dispatch;
	}
	// 82FEF480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEF484: 4BFE8E5D  bl 0x82fd82e0
	ctx.lr = 0x82FEF488;
	sub_82FD82E0(ctx, base);
	// 82FEF488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEF48C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEF490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF498: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEF49C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEF4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF4A8 size=92
    let mut pc: u32 = 0x82FEF4A8;
    'dispatch: loop {
        match pc {
            0x82FEF4A8 => {
    //   block [0x82FEF4A8..0x82FEF504)
	// 82FEF4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF4B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEF4B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF4B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEF4BC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FEF4C0: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FEF4C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEF4C8: 396BDFF4  addi r11, r11, -0x200c
	ctx.r[11].s64 = ctx.r[11].s64 + -8204;
	// 82FEF4CC: 394A7840  addi r10, r10, 0x7840
	ctx.r[10].s64 = ctx.r[10].s64 + 30784;
	// 82FEF4D0: 3929A93C  addi r9, r9, -0x56c4
	ctx.r[9].s64 = ctx.r[9].s64 + -22212;
	// 82FEF4D4: 548807FF  clrlwi. r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FEF4D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FEF4DC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FEF4E0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FEF4E4: 41820008  beq 0x82fef4ec
	if ctx.cr[0].eq {
	pc = 0x82FEF4EC; continue 'dispatch;
	}
	// 82FEF4E8: 4BFE8DF9  bl 0x82fd82e0
	ctx.lr = 0x82FEF4EC;
	sub_82FD82E0(ctx, base);
	// 82FEF4EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEF4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEF500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF508 size=80
    let mut pc: u32 = 0x82FEF508;
    'dispatch: loop {
        match pc {
            0x82FEF508 => {
    //   block [0x82FEF508..0x82FEF558)
	// 82FEF508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEF514: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF518: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FEF51C: 807FB8AC  lwz r3, -0x4754(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18260 as u32) ) } as u64;
	// 82FEF520: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FEF524: 419A0018  beq cr6, 0x82fef53c
	if ctx.cr[6].eq {
	pc = 0x82FEF53C; continue 'dispatch;
	}
	// 82FEF528: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF52C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEF530: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF534: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF538: 4E800421  bctrl
	ctx.lr = 0x82FEF53C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF53C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEF540: 917FB8AC  stw r11, -0x4754(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18260 as u32), ctx.r[11].u32 ) };
	// 82FEF544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEF554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF558 size=80
    let mut pc: u32 = 0x82FEF558;
    'dispatch: loop {
        match pc {
            0x82FEF558 => {
    //   block [0x82FEF558..0x82FEF5A8)
	// 82FEF558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEF564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF568: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FEF56C: 807FB8B4  lwz r3, -0x474c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18252 as u32) ) } as u64;
	// 82FEF570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FEF574: 419A0018  beq cr6, 0x82fef58c
	if ctx.cr[6].eq {
	pc = 0x82FEF58C; continue 'dispatch;
	}
	// 82FEF578: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF57C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEF580: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF584: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEF588: 4E800421  bctrl
	ctx.lr = 0x82FEF58C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEF58C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEF590: 917FB8B4  stw r11, -0x474c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18252 as u32), ctx.r[11].u32 ) };
	// 82FEF594: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF5A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEF5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF5A8 size=344
    let mut pc: u32 = 0x82FEF5A8;
    'dispatch: loop {
        match pc {
            0x82FEF5A8 => {
    //   block [0x82FEF5A8..0x82FEF700)
	// 82FEF5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF5AC: 481B8BB5  bl 0x831a8160
	ctx.lr = 0x82FEF5B0;
	sub_831A8130(ctx, base);
	// 82FEF5B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF5B4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FEF5B8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FEF5BC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FEF5C0: 419A0134  beq cr6, 0x82fef6f4
	if ctx.cr[6].eq {
	pc = 0x82FEF6F4; continue 'dispatch;
	}
	// 82FEF5C4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FEF5C8: 419A0014  beq cr6, 0x82fef5dc
	if ctx.cr[6].eq {
	pc = 0x82FEF5DC; continue 'dispatch;
	}
	// 82FEF5CC: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF5D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEF5D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF5D8: 40820008  bne 0x82fef5e0
	if !ctx.cr[0].eq {
	pc = 0x82FEF5E0; continue 'dispatch;
	}
	// 82FEF5DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEF5E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEF5E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEF5E8: 3BEBDFA8  addi r31, r11, -0x2058
	ctx.r[31].s64 = ctx.r[11].s64 + -8280;
	// 82FEF5EC: 555A063E  clrlwi r26, r10, 0x18
	ctx.r[26].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82FEF5F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEF5F4: 4BFE464D  bl 0x82fd3c40
	ctx.lr = 0x82FEF5F8;
	sub_82FD3C40(ctx, base);
	// 82FEF5F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEF5FC: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82FEF600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEF604: 4BFE463D  bl 0x82fd3c40
	ctx.lr = 0x82FEF608;
	sub_82FD3C40(ctx, base);
	// 82FEF608: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FEF60C: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82FEF610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEF614: 4BFE462D  bl 0x82fd3c40
	ctx.lr = 0x82FEF618;
	sub_82FD3C40(ctx, base);
	// 82FEF618: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEF61C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEF620: 388B7F24  addi r4, r11, 0x7f24
	ctx.r[4].s64 = ctx.r[11].s64 + 32548;
	// 82FEF624: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEF628: 4BFE22F1  bl 0x82fd1918
	ctx.lr = 0x82FEF62C;
	sub_82FD1918(ctx, base);
	// 82FEF62C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEF630: 40820024  bne 0x82fef654
	if !ctx.cr[0].eq {
	pc = 0x82FEF654; continue 'dispatch;
	}
	// 82FEF634: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF638: 40820014  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF63C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF640: 4082000C  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF644: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF648: 4182000C  beq 0x82fef654
	if ctx.cr[0].eq {
	pc = 0x82FEF654; continue 'dispatch;
	}
	// 82FEF64C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEF650: 480000A8  b 0x82fef6f8
	pc = 0x82FEF6F8; continue 'dispatch;
	// 82FEF654: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 82FEF658: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEF65C: 4BFE22BD  bl 0x82fd1918
	ctx.lr = 0x82FEF660;
	sub_82FD1918(ctx, base);
	// 82FEF660: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEF664: 40820024  bne 0x82fef688
	if !ctx.cr[0].eq {
	pc = 0x82FEF688; continue 'dispatch;
	}
	// 82FEF668: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF66C: 4082FFE0  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF670: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF674: 4082FFD8  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF678: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF67C: 4082FFD0  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF680: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF684: 4082FFC8  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF688: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 82FEF68C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEF690: 4BFE2289  bl 0x82fd1918
	ctx.lr = 0x82FEF694;
	sub_82FD1918(ctx, base);
	// 82FEF694: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEF698: 40820014  bne 0x82fef6ac
	if !ctx.cr[0].eq {
	pc = 0x82FEF6AC; continue 'dispatch;
	}
	// 82FEF69C: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF6A0: 4082FFAC  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF6A4: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF6A8: 4082FFA4  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF6AC: 389F0038  addi r4, r31, 0x38
	ctx.r[4].s64 = ctx.r[31].s64 + 56;
	// 82FEF6B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEF6B4: 4BFE2265  bl 0x82fd1918
	ctx.lr = 0x82FEF6B8;
	sub_82FD1918(ctx, base);
	// 82FEF6B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEF6BC: 40820014  bne 0x82fef6d0
	if !ctx.cr[0].eq {
	pc = 0x82FEF6D0; continue 'dispatch;
	}
	// 82FEF6C0: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF6C4: 4082FF88  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF6C8: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF6CC: 4082FF80  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF6D0: 389F0044  addi r4, r31, 0x44
	ctx.r[4].s64 = ctx.r[31].s64 + 68;
	// 82FEF6D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEF6D8: 4BFE2241  bl 0x82fd1918
	ctx.lr = 0x82FEF6DC;
	sub_82FD1918(ctx, base);
	// 82FEF6DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEF6E0: 40820014  bne 0x82fef6f4
	if !ctx.cr[0].eq {
	pc = 0x82FEF6F4; continue 'dispatch;
	}
	// 82FEF6E4: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF6E8: 4082FF64  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF6EC: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF6F0: 4082FF5C  bne 0x82fef64c
	if !ctx.cr[0].eq {
	pc = 0x82FEF64C; continue 'dispatch;
	}
	// 82FEF6F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEF6F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FEF6FC: 481B8AB4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEF700 size=8
    let mut pc: u32 = 0x82FEF700;
    'dispatch: loop {
        match pc {
            0x82FEF700 => {
    //   block [0x82FEF700..0x82FEF708)
	// 82FEF700: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEF704: 8213E028  lwz r16, -0x1fd8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-8152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF708 size=204
    let mut pc: u32 = 0x82FEF708;
    'dispatch: loop {
        match pc {
            0x82FEF708 => {
    //   block [0x82FEF708..0x82FEF7D4)
	// 82FEF708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF70C: 481B8A5D  bl 0x831a8168
	ctx.lr = 0x82FEF710;
	sub_831A8130(ctx, base);
	// 82FEF710: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FEF714: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF718: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEF71C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FEF720: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FEF724: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FEF728: 419A0034  beq cr6, 0x82fef75c
	if ctx.cr[6].eq {
	pc = 0x82FEF75C; continue 'dispatch;
	}
	// 82FEF72C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF730: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF734: 41820028  beq 0x82fef75c
	if ctx.cr[0].eq {
	pc = 0x82FEF75C; continue 'dispatch;
	}
	// 82FEF738: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FEF73C: 48000008  b 0x82fef744
	pc = 0x82FEF744; continue 'dispatch;
	// 82FEF740: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FEF744: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEF748: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF74C: 4082FFF4  bne 0x82fef740
	if !ctx.cr[0].eq {
	pc = 0x82FEF740; continue 'dispatch;
	}
	// 82FEF750: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FEF754: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FEF758: 48000008  b 0x82fef760
	pc = 0x82FEF760; continue 'dispatch;
	// 82FEF75C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEF760: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEF764: 4BFE9F95  bl 0x82fd96f8
	ctx.lr = 0x82FEF768;
	sub_82FD96F8(ctx, base);
	// 82FEF768: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEF76C: 4082002C  bne 0x82fef798
	if !ctx.cr[0].eq {
	pc = 0x82FEF798; continue 'dispatch;
	}
	// 82FEF770: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FEF774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEF778: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FEF77C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEF780: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FEF784: 4800A74D  bl 0x82ff9ed0
	ctx.lr = 0x82FEF788;
	sub_82FF9ED0(ctx, base);
	// 82FEF788: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEF78C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEF790: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FEF794: 481C1495  bl 0x831b0c28
	ctx.lr = 0x82FEF798;
	sub_831B0C28(ctx, base);
	// 82FEF798: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82FEF79C: 4B2D119D  bl 0x822c0938
	ctx.lr = 0x82FEF7A0;
	sub_822C0938(ctx, base);
	// 82FEF7A0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEF7A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF7A8: 41820020  beq 0x82fef7c8
	if ctx.cr[0].eq {
	pc = 0x82FEF7C8; continue 'dispatch;
	}
	// 82FEF7AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FEF7B0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FEF7B4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FEF7B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FEF7BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEF7C0: 480018F9  bl 0x82ff10b8
	ctx.lr = 0x82FEF7C4;
	sub_82FF10B8(ctx, base);
	// 82FEF7C4: 48000008  b 0x82fef7cc
	pc = 0x82FEF7CC; continue 'dispatch;
	// 82FEF7C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEF7CC: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FEF7D0: 481B89E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF7D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF7D4 size=40
    let mut pc: u32 = 0x82FEF7D4;
    'dispatch: loop {
        match pc {
            0x82FEF7D4 => {
    //   block [0x82FEF7D4..0x82FEF7FC)
	// 82FEF7D4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEF7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF7E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF7E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEF7E8: 4B2D0A81  bl 0x822c0268
	ctx.lr = 0x82FEF7EC;
	sub_822C0268(ctx, base);
	// 82FEF7EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEF800 size=8
    let mut pc: u32 = 0x82FEF800;
    'dispatch: loop {
        match pc {
            0x82FEF800 => {
    //   block [0x82FEF800..0x82FEF808)
	// 82FEF800: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEF804: 8213E070  lwz r16, -0x1f90(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-8080 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF808 size=96
    let mut pc: u32 = 0x82FEF808;
    'dispatch: loop {
        match pc {
            0x82FEF808 => {
    //   block [0x82FEF808..0x82FEF868)
	// 82FEF808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF80C: 481B8959  bl 0x831a8164
	ctx.lr = 0x82FEF810;
	sub_831A8130(ctx, base);
	// 82FEF810: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FEF814: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF818: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82FEF81C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FEF820: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEF824: 3860009C  li r3, 0x9c
	ctx.r[3].s64 = 156;
	// 82FEF828: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FEF82C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FEF830: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82FEF834: 4BFE8A65  bl 0x82fd8298
	ctx.lr = 0x82FEF838;
	sub_82FD8298(ctx, base);
	// 82FEF838: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEF83C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF840: 4182001C  beq 0x82fef85c
	if ctx.cr[0].eq {
	pc = 0x82FEF85C; continue 'dispatch;
	}
	// 82FEF844: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82FEF848: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FEF84C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEF850: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEF854: 4BFF637D  bl 0x82fe5bd0
	ctx.lr = 0x82FEF858;
	sub_82FE5BD0(ctx, base);
	// 82FEF858: 48000008  b 0x82fef860
	pc = 0x82FEF860; continue 'dispatch;
	// 82FEF85C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEF860: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FEF864: 481B8950  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF868 size=44
    let mut pc: u32 = 0x82FEF868;
    'dispatch: loop {
        match pc {
            0x82FEF868 => {
    //   block [0x82FEF868..0x82FEF894)
	// 82FEF868: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEF86C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF870: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF878: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FEF87C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEF880: 4BFE8A61  bl 0x82fd82e0
	ctx.lr = 0x82FEF884;
	sub_82FD82E0(ctx, base);
	// 82FEF884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF898 size=52
    let mut pc: u32 = 0x82FEF898;
    'dispatch: loop {
        match pc {
            0x82FEF898 => {
    //   block [0x82FEF898..0x82FEF8CC)
	// 82FEF898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF8A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF8A4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FEF8A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEF8AC: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FEF8B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEF8B4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FEF8B8: 4800A619  bl 0x82ff9ed0
	ctx.lr = 0x82FEF8BC;
	sub_82FF9ED0(ctx, base);
	// 82FEF8BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEF8C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEF8C4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FEF8C8: 481C1361  bl 0x831b0c28
	ctx.lr = 0x82FEF8CC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEF8D0 size=8
    let mut pc: u32 = 0x82FEF8D0;
    'dispatch: loop {
        match pc {
            0x82FEF8D0 => {
    //   block [0x82FEF8D0..0x82FEF8D8)
	// 82FEF8D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEF8D4: 8213E0A8  lwz r16, -0x1f58(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-8024 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF8D8 size=92
    let mut pc: u32 = 0x82FEF8D8;
    'dispatch: loop {
        match pc {
            0x82FEF8D8 => {
    //   block [0x82FEF8D8..0x82FEF934)
	// 82FEF8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEF8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEF8E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEF8EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF8F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEF8F4: 3860009C  li r3, 0x9c
	ctx.r[3].s64 = 156;
	// 82FEF8F8: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82FEF8FC: 4BFE899D  bl 0x82fd8298
	ctx.lr = 0x82FEF900;
	sub_82FD8298(ctx, base);
	// 82FEF900: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEF904: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF908: 41820010  beq 0x82fef918
	if ctx.cr[0].eq {
	pc = 0x82FEF918; continue 'dispatch;
	}
	// 82FEF90C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEF910: 4BFF2C89  bl 0x82fe2598
	ctx.lr = 0x82FEF914;
	sub_82FE2598(ctx, base);
	// 82FEF914: 48000008  b 0x82fef91c
	pc = 0x82FEF91C; continue 'dispatch;
	// 82FEF918: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEF91C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEF920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF928: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEF92C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEF930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF934 size=44
    let mut pc: u32 = 0x82FEF934;
    'dispatch: loop {
        match pc {
            0x82FEF934 => {
    //   block [0x82FEF934..0x82FEF960)
	// 82FEF934: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEF938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEF940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF944: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FEF948: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEF94C: 4BFE8995  bl 0x82fd82e0
	ctx.lr = 0x82FEF950;
	sub_82FD82E0(ctx, base);
	// 82FEF950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEF954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEF958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEF95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEF960 size=8
    let mut pc: u32 = 0x82FEF960;
    'dispatch: loop {
        match pc {
            0x82FEF960 => {
    //   block [0x82FEF960..0x82FEF968)
	// 82FEF960: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEF964: 8213E0E0  lwz r16, -0x1f20(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF968 size=144
    let mut pc: u32 = 0x82FEF968;
    'dispatch: loop {
        match pc {
            0x82FEF968 => {
    //   block [0x82FEF968..0x82FEF9F8)
	// 82FEF968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEF96C: 481B8801  bl 0x831a816c
	ctx.lr = 0x82FEF970;
	sub_831A8130(ctx, base);
	// 82FEF970: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FEF974: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEF978: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FEF97C: 7C8B0734  extsh r11, r4
	ctx.r[11].s64 = ctx.r[4].s16 as i64;
	// 82FEF980: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82FEF984: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FEF988: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 82FEF98C: 409A0024  bne cr6, 0x82fef9b0
	if !ctx.cr[6].eq {
	pc = 0x82FEF9B0; continue 'dispatch;
	}
	// 82FEF990: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEF994: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FEF998: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEF99C: 4800A535  bl 0x82ff9ed0
	ctx.lr = 0x82FEF9A0;
	sub_82FF9ED0(ctx, base);
	// 82FEF9A0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEF9A4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEF9A8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FEF9AC: 481C127D  bl 0x831b0c28
	ctx.lr = 0x82FEF9B0;
	sub_831B0C28(ctx, base);
	// 82FEF9B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEF9B4: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 82FEF9B8: 4BFE88E1  bl 0x82fd8298
	ctx.lr = 0x82FEF9BC;
	sub_82FD8298(ctx, base);
	// 82FEF9BC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEF9C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEF9C4: 41820018  beq 0x82fef9dc
	if ctx.cr[0].eq {
	pc = 0x82FEF9DC; continue 'dispatch;
	}
	// 82FEF9C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FEF9CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FEF9D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEF9D4: 4803FC45  bl 0x8302f618
	ctx.lr = 0x82FEF9D8;
	sub_8302F618(ctx, base);
	// 82FEF9D8: 48000008  b 0x82fef9e0
	pc = 0x82FEF9E0; continue 'dispatch;
	// 82FEF9DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEF9E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FEF9E4: 38630068  addi r3, r3, 0x68
	ctx.r[3].s64 = ctx.r[3].s64 + 104;
	// 82FEF9E8: 409A0008  bne cr6, 0x82fef9f0
	if !ctx.cr[6].eq {
	pc = 0x82FEF9F0; continue 'dispatch;
	}
	// 82FEF9EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEF9F0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FEF9F4: 481B87C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEF9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEF9F8 size=44
    let mut pc: u32 = 0x82FEF9F8;
    'dispatch: loop {
        match pc {
            0x82FEF9F8 => {
    //   block [0x82FEF9F8..0x82FEFA24)
	// 82FEF9F8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEF9FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFA00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFA04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFA08: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FEFA0C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEFA10: 4BFE88D1  bl 0x82fd82e0
	ctx.lr = 0x82FEFA14;
	sub_82FD82E0(ctx, base);
	// 82FEFA14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEFA28 size=8
    let mut pc: u32 = 0x82FEFA28;
    'dispatch: loop {
        match pc {
            0x82FEFA28 => {
    //   block [0x82FEFA28..0x82FEFA30)
	// 82FEFA28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEFA2C: 8213E118  lwz r16, -0x1ee8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7912 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFA30 size=92
    let mut pc: u32 = 0x82FEFA30;
    'dispatch: loop {
        match pc {
            0x82FEFA30 => {
    //   block [0x82FEFA30..0x82FEFA8C)
	// 82FEFA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFA38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEFA3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEFA40: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEFA44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFA48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEFA4C: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82FEFA50: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82FEFA54: 4BFE8845  bl 0x82fd8298
	ctx.lr = 0x82FEFA58;
	sub_82FD8298(ctx, base);
	// 82FEFA58: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEFA5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEFA60: 41820010  beq 0x82fefa70
	if ctx.cr[0].eq {
	pc = 0x82FEFA70; continue 'dispatch;
	}
	// 82FEFA64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEFA68: 480426C1  bl 0x83032128
	ctx.lr = 0x82FEFA6C;
	sub_83032128(ctx, base);
	// 82FEFA6C: 48000008  b 0x82fefa74
	pc = 0x82FEFA74; continue 'dispatch;
	// 82FEFA70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEFA74: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEFA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFA80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEFA84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEFA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFA8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFA8C size=44
    let mut pc: u32 = 0x82FEFA8C;
    'dispatch: loop {
        match pc {
            0x82FEFA8C => {
    //   block [0x82FEFA8C..0x82FEFAB8)
	// 82FEFA8C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEFA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFA98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFA9C: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FEFAA0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEFAA4: 4BFE883D  bl 0x82fd82e0
	ctx.lr = 0x82FEFAA8;
	sub_82FD82E0(ctx, base);
	// 82FEFAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEFAB8 size=8
    let mut pc: u32 = 0x82FEFAB8;
    'dispatch: loop {
        match pc {
            0x82FEFAB8 => {
    //   block [0x82FEFAB8..0x82FEFAC0)
	// 82FEFAB8: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FEFABC: 4BFFF9EC  b 0x82fef4a8
	sub_82FEF4A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFAC0 size=84
    let mut pc: u32 = 0x82FEFAC0;
    'dispatch: loop {
        match pc {
            0x82FEFAC0 => {
    //   block [0x82FEFAC0..0x82FEFB14)
	// 82FEFAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFAC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEFACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEFAD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFAD4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FEFAD8: 83FEB8B0  lwz r31, -0x4750(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18256 as u32) ) } as u64;
	// 82FEFADC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FEFAE0: 419A0014  beq cr6, 0x82fefaf4
	if ctx.cr[6].eq {
	pc = 0x82FEFAF4; continue 'dispatch;
	}
	// 82FEFAE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEFAE8: 48005CA1  bl 0x82ff5788
	ctx.lr = 0x82FEFAEC;
	sub_82FF5788(ctx, base);
	// 82FEFAEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEFAF0: 4BFE87F1  bl 0x82fd82e0
	ctx.lr = 0x82FEFAF4;
	sub_82FD82E0(ctx, base);
	// 82FEFAF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEFAF8: 917EB8B0  stw r11, -0x4750(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18256 as u32), ctx.r[11].u32 ) };
	// 82FEFAFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEFB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFB08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEFB0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEFB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEFB18 size=8
    let mut pc: u32 = 0x82FEFB18;
    'dispatch: loop {
        match pc {
            0x82FEFB18 => {
    //   block [0x82FEFB18..0x82FEFB20)
	// 82FEFB18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEFB1C: 8213E158  lwz r16, -0x1ea8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7848 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFB20 size=164
    let mut pc: u32 = 0x82FEFB20;
    'dispatch: loop {
        match pc {
            0x82FEFB20 => {
    //   block [0x82FEFB20..0x82FEFBC4)
	// 82FEFB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFB28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEFB2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEFB30: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEFB34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFB38: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FEFB3C: 807EB8B0  lwz r3, -0x4750(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18256 as u32) ) } as u64;
	// 82FEFB40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FEFB44: 409A0068  bne cr6, 0x82fefbac
	if !ctx.cr[6].eq {
	pc = 0x82FEFBAC; continue 'dispatch;
	}
	// 82FEFB48: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FEFB4C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFB50: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 82FEFB54: 48005C85  bl 0x82ff57d8
	ctx.lr = 0x82FEFB58;
	sub_82FF57D8(ctx, base);
	// 82FEFB58: 817EB8B0  lwz r11, -0x4750(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18256 as u32) ) } as u64;
	// 82FEFB5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEFB60: 409A0040  bne cr6, 0x82fefba0
	if !ctx.cr[6].eq {
	pc = 0x82FEFBA0; continue 'dispatch;
	}
	// 82FEFB64: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FEFB68: 4BFE86E1  bl 0x82fd8248
	ctx.lr = 0x82FEFB6C;
	sub_82FD8248(ctx, base);
	// 82FEFB6C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FEFB70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEFB74: 41820010  beq 0x82fefb84
	if ctx.cr[0].eq {
	pc = 0x82FEFB84; continue 'dispatch;
	}
	// 82FEFB78: 48005BD1  bl 0x82ff5748
	ctx.lr = 0x82FEFB7C;
	sub_82FF5748(ctx, base);
	// 82FEFB7C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FEFB80: 48000008  b 0x82fefb88
	pc = 0x82FEFB88; continue 'dispatch;
	// 82FEFB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FEFB88: 3D6082FF  lis r11, -0x7d01
	ctx.r[11].s64 = -2097217536;
	// 82FEFB8C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FEFB90: 913EB8B0  stw r9, -0x4750(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18256 as u32), ctx.r[9].u32 ) };
	// 82FEFB94: 388BFAC0  addi r4, r11, -0x540
	ctx.r[4].s64 = ctx.r[11].s64 + -1344;
	// 82FEFB98: 386AB8C4  addi r3, r10, -0x473c
	ctx.r[3].s64 = ctx.r[10].s64 + -18236;
	// 82FEFB9C: 48007F9D  bl 0x82ff7b38
	ctx.lr = 0x82FEFBA0;
	sub_82FF7B38(ctx, base);
	// 82FEFBA0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFBA4: 48005C6D  bl 0x82ff5810
	ctx.lr = 0x82FEFBA8;
	sub_82FF5810(ctx, base);
	// 82FEFBA8: 807EB8B0  lwz r3, -0x4750(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18256 as u32) ) } as u64;
	// 82FEFBAC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEFBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFBB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEFBBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEFBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFBC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFBC4 size=40
    let mut pc: u32 = 0x82FEFBC4;
    'dispatch: loop {
        match pc {
            0x82FEFBC4 => {
    //   block [0x82FEFBC4..0x82FEFBEC)
	// 82FEFBC4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEFBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFBD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFBD4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFBD8: 48005C39  bl 0x82ff5810
	ctx.lr = 0x82FEFBDC;
	sub_82FF5810(ctx, base);
	// 82FEFBDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFBEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFBEC size=40
    let mut pc: u32 = 0x82FEFBEC;
    'dispatch: loop {
        match pc {
            0x82FEFBEC => {
    //   block [0x82FEFBEC..0x82FEFC14)
	// 82FEFBEC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEFBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFBFC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FEFC00: 4BFE86E1  bl 0x82fd82e0
	ctx.lr = 0x82FEFC04;
	sub_82FD82E0(ctx, base);
	// 82FEFC04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEFC18 size=8
    let mut pc: u32 = 0x82FEFC18;
    'dispatch: loop {
        match pc {
            0x82FEFC18 => {
    //   block [0x82FEFC18..0x82FEFC20)
	// 82FEFC18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEFC1C: 8213E1A8  lwz r16, -0x1e58(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7768 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFC20 size=160
    let mut pc: u32 = 0x82FEFC20;
    'dispatch: loop {
        match pc {
            0x82FEFC20 => {
    //   block [0x82FEFC20..0x82FEFCC0)
	// 82FEFC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFC28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEFC2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEFC30: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEFC34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFC38: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FEFC3C: 807EB8AC  lwz r3, -0x4754(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18260 as u32) ) } as u64;
	// 82FEFC40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FEFC44: 409A0064  bne cr6, 0x82fefca8
	if !ctx.cr[6].eq {
	pc = 0x82FEFCA8; continue 'dispatch;
	}
	// 82FEFC48: 4BFFFED9  bl 0x82fefb20
	ctx.lr = 0x82FEFC4C;
	sub_82FEFB20(ctx, base);
	// 82FEFC4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FEFC50: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFC54: 48005B85  bl 0x82ff57d8
	ctx.lr = 0x82FEFC58;
	sub_82FF57D8(ctx, base);
	// 82FEFC58: 817EB8AC  lwz r11, -0x4754(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18260 as u32) ) } as u64;
	// 82FEFC5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEFC60: 409A003C  bne cr6, 0x82fefc9c
	if !ctx.cr[6].eq {
	pc = 0x82FEFC9C; continue 'dispatch;
	}
	// 82FEFC64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEFC68: 386B9490  addi r3, r11, -0x6b70
	ctx.r[3].s64 = ctx.r[11].s64 + -27504;
	// 82FEFC6C: 4BFE8425  bl 0x82fd8090
	ctx.lr = 0x82FEFC70;
	sub_82FD8090(ctx, base);
	// 82FEFC70: 907EB8AC  stw r3, -0x4754(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18260 as u32), ctx.r[3].u32 ) };
	// 82FEFC74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEFC78: 40820010  bne 0x82fefc88
	if !ctx.cr[0].eq {
	pc = 0x82FEFC88; continue 'dispatch;
	}
	// 82FEFC7C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FEFC80: 48004DC9  bl 0x82ff4a48
	ctx.lr = 0x82FEFC84;
	sub_82FF4A48(ctx, base);
	// 82FEFC84: 48000018  b 0x82fefc9c
	pc = 0x82FEFC9C; continue 'dispatch;
	// 82FEFC88: 3D6082FF  lis r11, -0x7d01
	ctx.r[11].s64 = -2097217536;
	// 82FEFC8C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FEFC90: 388BF508  addi r4, r11, -0xaf8
	ctx.r[4].s64 = ctx.r[11].s64 + -2808;
	// 82FEFC94: 386AB8D0  addi r3, r10, -0x4730
	ctx.r[3].s64 = ctx.r[10].s64 + -18224;
	// 82FEFC98: 48007EA1  bl 0x82ff7b38
	ctx.lr = 0x82FEFC9C;
	sub_82FF7B38(ctx, base);
	// 82FEFC9C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFCA0: 48005B71  bl 0x82ff5810
	ctx.lr = 0x82FEFCA4;
	sub_82FF5810(ctx, base);
	// 82FEFCA4: 807EB8AC  lwz r3, -0x4754(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18260 as u32) ) } as u64;
	// 82FEFCA8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEFCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFCB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEFCB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEFCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFCC0 size=40
    let mut pc: u32 = 0x82FEFCC0;
    'dispatch: loop {
        match pc {
            0x82FEFCC0 => {
    //   block [0x82FEFCC0..0x82FEFCE8)
	// 82FEFCC0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEFCC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFCC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFCCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFCD0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFCD4: 48005B3D  bl 0x82ff5810
	ctx.lr = 0x82FEFCD8;
	sub_82FF5810(ctx, base);
	// 82FEFCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEFCE8 size=8
    let mut pc: u32 = 0x82FEFCE8;
    'dispatch: loop {
        match pc {
            0x82FEFCE8 => {
    //   block [0x82FEFCE8..0x82FEFCF0)
	// 82FEFCE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEFCEC: 8213E1E8  lwz r16, -0x1e18(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7704 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFCF0 size=192
    let mut pc: u32 = 0x82FEFCF0;
    'dispatch: loop {
        match pc {
            0x82FEFCF0 => {
    //   block [0x82FEFCF0..0x82FEFDB0)
	// 82FEFCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFCF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEFCFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEFD00: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEFD04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFD08: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FEFD0C: 807EB8B4  lwz r3, -0x474c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18252 as u32) ) } as u64;
	// 82FEFD10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FEFD14: 409A0084  bne cr6, 0x82fefd98
	if !ctx.cr[6].eq {
	pc = 0x82FEFD98; continue 'dispatch;
	}
	// 82FEFD18: 4BFFFE09  bl 0x82fefb20
	ctx.lr = 0x82FEFD1C;
	sub_82FEFB20(ctx, base);
	// 82FEFD1C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FEFD20: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFD24: 48005AB5  bl 0x82ff57d8
	ctx.lr = 0x82FEFD28;
	sub_82FF57D8(ctx, base);
	// 82FEFD28: 817EB8B4  lwz r11, -0x474c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18252 as u32) ) } as u64;
	// 82FEFD2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEFD30: 409A005C  bne cr6, 0x82fefd8c
	if !ctx.cr[6].eq {
	pc = 0x82FEFD8C; continue 'dispatch;
	}
	// 82FEFD34: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FEFD38: 4BFE8511  bl 0x82fd8248
	ctx.lr = 0x82FEFD3C;
	sub_82FD8248(ctx, base);
	// 82FEFD3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEFD40: 41820030  beq 0x82fefd70
	if ctx.cr[0].eq {
	pc = 0x82FEFD70; continue 'dispatch;
	}
	// 82FEFD44: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEFD48: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FEFD4C: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FEFD50: 396B7840  addi r11, r11, 0x7840
	ctx.r[11].s64 = ctx.r[11].s64 + 30784;
	// 82FEFD54: 394ADFFC  addi r10, r10, -0x2004
	ctx.r[10].s64 = ctx.r[10].s64 + -8196;
	// 82FEFD58: 3929DFF4  addi r9, r9, -0x200c
	ctx.r[9].s64 = ctx.r[9].s64 + -8204;
	// 82FEFD5C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82FEFD60: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FEFD64: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FEFD68: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FEFD6C: 48000008  b 0x82fefd74
	pc = 0x82FEFD74; continue 'dispatch;
	// 82FEFD70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEFD74: 3D6082FF  lis r11, -0x7d01
	ctx.r[11].s64 = -2097217536;
	// 82FEFD78: 911EB8B4  stw r8, -0x474c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18252 as u32), ctx.r[8].u32 ) };
	// 82FEFD7C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FEFD80: 388BF558  addi r4, r11, -0xaa8
	ctx.r[4].s64 = ctx.r[11].s64 + -2728;
	// 82FEFD84: 386AB8B8  addi r3, r10, -0x4748
	ctx.r[3].s64 = ctx.r[10].s64 + -18248;
	// 82FEFD88: 48007DB1  bl 0x82ff7b38
	ctx.lr = 0x82FEFD8C;
	sub_82FF7B38(ctx, base);
	// 82FEFD8C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFD90: 48005A81  bl 0x82ff5810
	ctx.lr = 0x82FEFD94;
	sub_82FF5810(ctx, base);
	// 82FEFD94: 807EB8B4  lwz r3, -0x474c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18252 as u32) ) } as u64;
	// 82FEFD98: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEFD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFDA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEFDA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEFDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFDB0 size=40
    let mut pc: u32 = 0x82FEFDB0;
    'dispatch: loop {
        match pc {
            0x82FEFDB0 => {
    //   block [0x82FEFDB0..0x82FEFDD8)
	// 82FEFDB0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEFDB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFDB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFDBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFDC0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFDC4: 48005A4D  bl 0x82ff5810
	ctx.lr = 0x82FEFDC8;
	sub_82FF5810(ctx, base);
	// 82FEFDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEFDD8 size=4
    let mut pc: u32 = 0x82FEFDD8;
    'dispatch: loop {
        match pc {
            0x82FEFDD8 => {
    //   block [0x82FEFDD8..0x82FEFDDC)
	// 82FEFDD8: 4BFFFF18  b 0x82fefcf0
	sub_82FEFCF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFDE0 size=64
    let mut pc: u32 = 0x82FEFDE0;
    'dispatch: loop {
        match pc {
            0x82FEFDE0 => {
    //   block [0x82FEFDE0..0x82FEFE20)
	// 82FEFDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFDE4: 481B8389  bl 0x831a816c
	ctx.lr = 0x82FEFDE8;
	sub_831A8130(ctx, base);
	// 82FEFDE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFDEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEFDF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEFDF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FEFDF8: 4BFFFE29  bl 0x82fefc20
	ctx.lr = 0x82FEFDFC;
	sub_82FEFC20(ctx, base);
	// 82FEFDFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFE00: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FEFE04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FEFE08: 389F0002  addi r4, r31, 2
	ctx.r[4].s64 = ctx.r[31].s64 + 2;
	// 82FEFE0C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEFE10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEFE14: 4E800421  bctrl
	ctx.lr = 0x82FEFE18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEFE18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEFE1C: 481B83A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEFE20 size=8
    let mut pc: u32 = 0x82FEFE20;
    'dispatch: loop {
        match pc {
            0x82FEFE20 => {
    //   block [0x82FEFE20..0x82FEFE28)
	// 82FEFE20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEFE24: 8213E228  lwz r16, -0x1dd8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7640 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFE28 size=216
    let mut pc: u32 = 0x82FEFE28;
    'dispatch: loop {
        match pc {
            0x82FEFE28 => {
    //   block [0x82FEFE28..0x82FEFF00)
	// 82FEFE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFE2C: 481B8339  bl 0x831a8164
	ctx.lr = 0x82FEFE30;
	sub_831A8130(ctx, base);
	// 82FEFE30: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FEFE34: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFE38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEFE3C: 4BFFFEB5  bl 0x82fefcf0
	ctx.lr = 0x82FEFE40;
	sub_82FEFCF0(ctx, base);
	// 82FEFE40: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FEFE44: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FEFE48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEFE4C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFE50: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FEFE54: 4804466D  bl 0x830344c0
	ctx.lr = 0x82FEFE58;
	sub_830344C0(ctx, base);
	// 82FEFE58: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FEFE5C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FEFE60: 409A0020  bne cr6, 0x82fefe80
	if !ctx.cr[6].eq {
	pc = 0x82FEFE80; continue 'dispatch;
	}
	// 82FEFE64: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFE68: 48044621  bl 0x83034488
	ctx.lr = 0x82FEFE6C;
	sub_83034488(ctx, base);
	// 82FEFE6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEFE70: 4182006C  beq 0x82fefedc
	if ctx.cr[0].eq {
	pc = 0x82FEFEDC; continue 'dispatch;
	}
	// 82FEFE74: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFE78: 48044991  bl 0x83034808
	ctx.lr = 0x82FEFE7C;
	sub_83034808(ctx, base);
	// 82FEFE7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FEFE80: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFE84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FEFE88: 48044981  bl 0x83034808
	ctx.lr = 0x82FEFE8C;
	sub_83034808(ctx, base);
	// 82FEFE8C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FEFE90: 41820018  beq 0x82fefea8
	if ctx.cr[0].eq {
	pc = 0x82FEFEA8; continue 'dispatch;
	}
	// 82FEFE94: A07D0000  lhz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFE98: 4BFE16F1  bl 0x82fd1588
	ctx.lr = 0x82FEFE9C;
	sub_82FD1588(ctx, base);
	// 82FEFE9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEFEA0: 41820008  beq 0x82fefea8
	if ctx.cr[0].eq {
	pc = 0x82FEFEA8; continue 'dispatch;
	}
	// 82FEFEA4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82FEFEA8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFEAC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FEFEB0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEFEB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FEFEB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEFEBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEFEC0: 4E800421  bctrl
	ctx.lr = 0x82FEFEC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEFEC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEFEC8: 41820024  beq 0x82fefeec
	if ctx.cr[0].eq {
	pc = 0x82FEFEEC; continue 'dispatch;
	}
	// 82FEFECC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FEFED0: 409AFF8C  bne cr6, 0x82fefe5c
	if !ctx.cr[6].eq {
	pc = 0x82FEFE5C; continue 'dispatch;
	}
	// 82FEFED4: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82FEFED8: 4BFFFF84  b 0x82fefe5c
	pc = 0x82FEFE5C; continue 'dispatch;
	// 82FEFEDC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFEE0: 480445A1  bl 0x83034480
	ctx.lr = 0x82FEFEE4;
	sub_83034480(ctx, base);
	// 82FEFEE4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FEFEE8: 48000010  b 0x82fefef8
	pc = 0x82FEFEF8; continue 'dispatch;
	// 82FEFEEC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFEF0: 48044591  bl 0x83034480
	ctx.lr = 0x82FEFEF4;
	sub_83034480(ctx, base);
	// 82FEFEF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEFEF8: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FEFEFC: 481B82B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFF00 size=40
    let mut pc: u32 = 0x82FEFF00;
    'dispatch: loop {
        match pc {
            0x82FEFF00 => {
    //   block [0x82FEFF00..0x82FEFF28)
	// 82FEFF00: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEFF04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFF08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFF0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFF10: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEFF14: 4804456D  bl 0x83034480
	ctx.lr = 0x82FEFF18;
	sub_83034480(ctx, base);
	// 82FEFF18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFF28 size=80
    let mut pc: u32 = 0x82FEFF28;
    'dispatch: loop {
        match pc {
            0x82FEFF28 => {
    //   block [0x82FEFF28..0x82FEFF78)
	// 82FEFF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFF30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEFF34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFF38: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FEFF3C: 817FB8DC  lwz r11, -0x4724(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18212 as u32) ) } as u64;
	// 82FEFF40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEFF44: 419A0020  beq cr6, 0x82feff64
	if ctx.cr[6].eq {
	pc = 0x82FEFF64; continue 'dispatch;
	}
	// 82FEFF48: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FEFF4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFF50: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FEFF54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEFF58: 4E800421  bctrl
	ctx.lr = 0x82FEFF5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEFF5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEFF60: 917FB8DC  stw r11, -0x4724(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18212 as u32), ctx.r[11].u32 ) };
	// 82FEFF64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEFF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEFF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEFF70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEFF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEFF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEFF78 size=188
    let mut pc: u32 = 0x82FEFF78;
    'dispatch: loop {
        match pc {
            0x82FEFF78 => {
    //   block [0x82FEFF78..0x82FF0034)
	// 82FEFF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEFF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEFF80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEFF84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEFF88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEFF8C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FEFF90: 3BCBB8DC  addi r30, r11, -0x4724
	ctx.r[30].s64 = ctx.r[11].s64 + -18212;
	// 82FEFF94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFF98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEFF9C: 409A007C  bne cr6, 0x82ff0018
	if !ctx.cr[6].eq {
	pc = 0x82FF0018; continue 'dispatch;
	}
	// 82FEFFA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEFFA4: 386BE268  addi r3, r11, -0x1d98
	ctx.r[3].s64 = ctx.r[11].s64 + -7576;
	// 82FEFFA8: 4BFE8A21  bl 0x82fd89c8
	ctx.lr = 0x82FEFFAC;
	sub_82FD89C8(ctx, base);
	// 82FEFFAC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FEFFB0: 808BB7E8  lwz r4, -0x4818(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FEFFB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFFB8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEFFBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEFFC0: 4E800421  bctrl
	ctx.lr = 0x82FEFFC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEFFC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEFFC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEFFCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEFFD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FEFFD4: 48004F6D  bl 0x82ff4f40
	ctx.lr = 0x82FEFFD8;
	sub_82FF4F40(ctx, base);
	// 82FEFFD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEFFDC: 41820028  beq 0x82ff0004
	if ctx.cr[0].eq {
	pc = 0x82FF0004; continue 'dispatch;
	}
	// 82FEFFE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FEFFE4: 419A0034  beq cr6, 0x82ff0018
	if ctx.cr[6].eq {
	pc = 0x82FF0018; continue 'dispatch;
	}
	// 82FEFFE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFFEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEFFF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEFFF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEFFF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEFFFC: 4E800421  bctrl
	ctx.lr = 0x82FF0000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0000: 48000018  b 0x82ff0018
	pc = 0x82FF0018; continue 'dispatch;
	// 82FF0004: 3D6082FF  lis r11, -0x7d01
	ctx.r[11].s64 = -2097217536;
	// 82FF0008: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FF000C: 388BFF28  addi r4, r11, -0xd8
	ctx.r[4].s64 = ctx.r[11].s64 + -216;
	// 82FF0010: 386AB8E0  addi r3, r10, -0x4720
	ctx.r[3].s64 = ctx.r[10].s64 + -18208;
	// 82FF0014: 48007B25  bl 0x82ff7b38
	ctx.lr = 0x82FF0018;
	sub_82FF7B38(ctx, base);
	// 82FF0018: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF001C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF0020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0028: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF002C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF0030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF0038 size=8
    let mut pc: u32 = 0x82FF0038;
    'dispatch: loop {
        match pc {
            0x82FF0038 => {
    //   block [0x82FF0038..0x82FF0040)
	// 82FF0038: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF003C: 8213E368  lwz r16, -0x1c98(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7320 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0040 size=300
    let mut pc: u32 = 0x82FF0040;
    'dispatch: loop {
        match pc {
            0x82FF0040 => {
    //   block [0x82FF0040..0x82FF016C)
	// 82FF0040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0044: 481B8119  bl 0x831a815c
	ctx.lr = 0x82FF0048;
	sub_831A8130(ctx, base);
	// 82FF0048: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF004C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0050: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF0054: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF0058: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FF005C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82FF0060: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF0064: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF0068: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 82FF006C: 396BE278  addi r11, r11, -0x1d88
	ctx.r[11].s64 = ctx.r[11].s64 + -7560;
	// 82FF0070: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 82FF0074: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF0078: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF007C: 4BFF057D  bl 0x82fe05f8
	ctx.lr = 0x82FF0080;
	sub_82FE05F8(ctx, base);
	// 82FF0080: 3B7E000C  addi r27, r30, 0xc
	ctx.r[27].s64 = ctx.r[30].s64 + 12;
	// 82FF0084: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82FF0088: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF008C: 4800C455  bl 0x82ffc4e0
	ctx.lr = 0x82FF0090;
	sub_82FFC4E0(ctx, base);
	// 82FF0090: 389D001C  addi r4, r29, 0x1c
	ctx.r[4].s64 = ctx.r[29].s64 + 28;
	// 82FF0094: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 82FF0098: 4819DB19  bl 0x8318dbb0
	ctx.lr = 0x82FF009C;
	sub_8318DBB0(ctx, base);
	// 82FF009C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF00A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF00A4: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF00A8: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82FF00AC: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FF00B0: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82FF00B4: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FF00B8: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FF00BC: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82FF00C0: 897D0040  lbz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FF00C4: 9B5E0041  stb r26, 0x41(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(65 as u32), ctx.r[26].u8 ) };
	// 82FF00C8: 997E0040  stb r11, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82FF00CC: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF00D0: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF00D4: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FF00D8: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82FF00DC: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FF00E0: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FF00E4: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FF00E8: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82FF00EC: 4BFEF13D  bl 0x82fdf228
	ctx.lr = 0x82FF00F0;
	sub_82FDF228(ctx, base);
	// 82FF00F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF00F4: 41820018  beq 0x82ff010c
	if ctx.cr[0].eq {
	pc = 0x82FF010C; continue 'dispatch;
	}
	// 82FF00F8: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF00FC: 41820010  beq 0x82ff010c
	if ctx.cr[0].eq {
	pc = 0x82FF010C; continue 'dispatch;
	}
	// 82FF0100: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF0104: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF0108: 4800D291  bl 0x82ffd398
	ctx.lr = 0x82FF010C;
	sub_82FFD398(ctx, base);
	// 82FF010C: 807D0028  lwz r3, 0x28(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF0110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0114: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0118: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF011C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0120: 4E800421  bctrl
	ctx.lr = 0x82FF0124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0124: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82FF0128: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF012C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0134: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF0138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF013C: 4E800421  bctrl
	ctx.lr = 0x82FF0140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0140: 907E002C  stw r3, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82FF0144: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FF0148: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF014C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0150: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF0154: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0158: 4E800421  bctrl
	ctx.lr = 0x82FF015C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF015C: 907E0030  stw r3, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FF0160: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0164: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF0168: 481B8044  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF016C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF016C size=40
    let mut pc: u32 = 0x82FF016C;
    'dispatch: loop {
        match pc {
            0x82FF016C => {
    //   block [0x82FF016C..0x82FF0194)
	// 82FF016C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF017C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0180: 48015A01  bl 0x83005b80
	ctx.lr = 0x82FF0184;
	sub_83005B80(ctx, base);
	// 82FF0184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF018C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0194 size=44
    let mut pc: u32 = 0x82FF0194;
    'dispatch: loop {
        match pc {
            0x82FF0194 => {
    //   block [0x82FF0194..0x82FF01C0)
	// 82FF0194: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF019C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF01A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF01A4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF01A8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82FF01AC: 480D7935  bl 0x830c7ae0
	ctx.lr = 0x82FF01B0;
	sub_830C7AE0(ctx, base);
	// 82FF01B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF01B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF01B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF01BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF01C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF01C0 size=44
    let mut pc: u32 = 0x82FF01C0;
    'dispatch: loop {
        match pc {
            0x82FF01C0 => {
    //   block [0x82FF01C0..0x82FF01EC)
	// 82FF01C0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF01C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF01C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF01CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF01D0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF01D4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FF01D8: 4BFF1051  bl 0x82fe1228
	ctx.lr = 0x82FF01DC;
	sub_82FE1228(ctx, base);
	// 82FF01DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF01E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF01E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF01E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF01EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF01EC size=44
    let mut pc: u32 = 0x82FF01EC;
    'dispatch: loop {
        match pc {
            0x82FF01EC => {
    //   block [0x82FF01EC..0x82FF0218)
	// 82FF01EC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF01F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF01F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF01F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF01FC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0200: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 82FF0204: 480D78DD  bl 0x830c7ae0
	ctx.lr = 0x82FF0208;
	sub_830C7AE0(ctx, base);
	// 82FF0208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF0218 size=8
    let mut pc: u32 = 0x82FF0218;
    'dispatch: loop {
        match pc {
            0x82FF0218 => {
    //   block [0x82FF0218..0x82FF0220)
	// 82FF0218: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF021C: 8213E3D0  lwz r16, -0x1c30(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7216 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0220 size=108
    let mut pc: u32 = 0x82FF0220;
    'dispatch: loop {
        match pc {
            0x82FF0220 => {
    //   block [0x82FF0220..0x82FF028C)
	// 82FF0220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF022C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF0230: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF0234: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0238: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF023C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF0240: 396BE278  addi r11, r11, -0x1d88
	ctx.r[11].s64 = ctx.r[11].s64 + -7560;
	// 82FF0244: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FF0248: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF024C: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 82FF0250: 480D7891  bl 0x830c7ae0
	ctx.lr = 0x82FF0254;
	sub_830C7AE0(ctx, base);
	// 82FF0254: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 82FF0258: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82FF025C: 48009DAD  bl 0x82ffa008
	ctx.lr = 0x82FF0260;
	sub_82FFA008(ctx, base);
	// 82FF0260: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82FF0264: 480D787D  bl 0x830c7ae0
	ctx.lr = 0x82FF0268;
	sub_830C7AE0(ctx, base);
	// 82FF0268: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF026C: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 82FF0270: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF0274: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF0278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF027C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF0284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF0288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF028C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF028C size=40
    let mut pc: u32 = 0x82FF028C;
    'dispatch: loop {
        match pc {
            0x82FF028C => {
    //   block [0x82FF028C..0x82FF02B4)
	// 82FF028C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF0290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF029C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF02A0: 480158E1  bl 0x83005b80
	ctx.lr = 0x82FF02A4;
	sub_83005B80(ctx, base);
	// 82FF02A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF02A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF02AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF02B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF02B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF02B4 size=44
    let mut pc: u32 = 0x82FF02B4;
    'dispatch: loop {
        match pc {
            0x82FF02B4 => {
    //   block [0x82FF02B4..0x82FF02E0)
	// 82FF02B4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF02B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF02BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF02C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF02C4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF02C8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82FF02CC: 480D7815  bl 0x830c7ae0
	ctx.lr = 0x82FF02D0;
	sub_830C7AE0(ctx, base);
	// 82FF02D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF02D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF02D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF02DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF02E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF02E0 size=44
    let mut pc: u32 = 0x82FF02E0;
    'dispatch: loop {
        match pc {
            0x82FF02E0 => {
    //   block [0x82FF02E0..0x82FF030C)
	// 82FF02E0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF02E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF02E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF02EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF02F0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF02F4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FF02F8: 4BFF0F31  bl 0x82fe1228
	ctx.lr = 0x82FF02FC;
	sub_82FE1228(ctx, base);
	// 82FF02FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF0310 size=8
    let mut pc: u32 = 0x82FF0310;
    'dispatch: loop {
        match pc {
            0x82FF0310 => {
    //   block [0x82FF0310..0x82FF0318)
	// 82FF0310: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF0314: 8213E420  lwz r16, -0x1be0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-7136 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0318 size=204
    let mut pc: u32 = 0x82FF0318;
    'dispatch: loop {
        match pc {
            0x82FF0318 => {
    //   block [0x82FF0318..0x82FF03E4)
	// 82FF0318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF031C: 481B7E4D  bl 0x831a8168
	ctx.lr = 0x82FF0320;
	sub_831A8130(ctx, base);
	// 82FF0320: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF0324: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0328: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF032C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF0330: 3B9D0004  addi r28, r29, 4
	ctx.r[28].s64 = ctx.r[29].s64 + 4;
	// 82FF0334: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF0338: 4BFEEEF1  bl 0x82fdf228
	ctx.lr = 0x82FF033C;
	sub_82FDF228(ctx, base);
	// 82FF033C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0340: 41820048  beq 0x82ff0388
	if ctx.cr[0].eq {
	pc = 0x82FF0388; continue 'dispatch;
	}
	// 82FF0344: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF0348: 4BFEEEE1  bl 0x82fdf228
	ctx.lr = 0x82FF034C;
	sub_82FDF228(ctx, base);
	// 82FF034C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FF0350: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82FF0354: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF0358: 4BFF4561  bl 0x82fe48b8
	ctx.lr = 0x82FF035C;
	sub_82FE48B8(ctx, base);
	// 82FF035C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF0360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0364: 41820018  beq 0x82ff037c
	if ctx.cr[0].eq {
	pc = 0x82FF037C; continue 'dispatch;
	}
	// 82FF0368: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF036C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF0370: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF0374: 4BFFFCCD  bl 0x82ff0040
	ctx.lr = 0x82FF0378;
	sub_82FF0040(ctx, base);
	// 82FF0378: 48000008  b 0x82ff0380
	pc = 0x82FF0380; continue 'dispatch;
	// 82FF037C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF0380: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF0384: 48000040  b 0x82ff03c4
	pc = 0x82FF03C4; continue 'dispatch;
	// 82FF0388: 4BFFFBF1  bl 0x82feff78
	ctx.lr = 0x82FF038C;
	sub_82FEFF78(ctx, base);
	// 82FF038C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FF0390: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82FF0394: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF0398: 4BFF4521  bl 0x82fe48b8
	ctx.lr = 0x82FF039C;
	sub_82FE48B8(ctx, base);
	// 82FF039C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF03A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF03A4: 41820018  beq 0x82ff03bc
	if ctx.cr[0].eq {
	pc = 0x82FF03BC; continue 'dispatch;
	}
	// 82FF03A8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF03AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF03B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF03B4: 4BFFFC8D  bl 0x82ff0040
	ctx.lr = 0x82FF03B8;
	sub_82FF0040(ctx, base);
	// 82FF03B8: 48000008  b 0x82ff03c0
	pc = 0x82FF03C0; continue 'dispatch;
	// 82FF03BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF03C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF03C4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF03C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FF03CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF03D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF03D4: 4BFEF3A5  bl 0x82fdf778
	ctx.lr = 0x82FF03D8;
	sub_82FDF778(ctx, base);
	// 82FF03D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF03DC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF03E0: 481B7DD8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF03E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF03E4 size=48
    let mut pc: u32 = 0x82FF03E4;
    'dispatch: loop {
        match pc {
            0x82FF03E4 => {
    //   block [0x82FF03E4..0x82FF0414)
	// 82FF03E4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF03E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF03EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF03F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF03F4: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FF03F8: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF03FC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF0400: 480D76E1  bl 0x830c7ae0
	ctx.lr = 0x82FF0404;
	sub_830C7AE0(ctx, base);
	// 82FF0404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF040C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0414 size=48
    let mut pc: u32 = 0x82FF0414;
    'dispatch: loop {
        match pc {
            0x82FF0414 => {
    //   block [0x82FF0414..0x82FF0444)
	// 82FF0414: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF0418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0424: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FF0428: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF042C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF0430: 480D76B1  bl 0x830c7ae0
	ctx.lr = 0x82FF0434;
	sub_830C7AE0(ctx, base);
	// 82FF0434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF043C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0448 size=280
    let mut pc: u32 = 0x82FF0448;
    'dispatch: loop {
        match pc {
            0x82FF0448 => {
    //   block [0x82FF0448..0x82FF0560)
	// 82FF0448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF044C: 481B7D21  bl 0x831a816c
	ctx.lr = 0x82FF0450;
	sub_831A8130(ctx, base);
	// 82FF0450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0458: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF045C: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 82FF0460: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0464: 4BFEEDC5  bl 0x82fdf228
	ctx.lr = 0x82FF0468;
	sub_82FDF228(ctx, base);
	// 82FF0468: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF046C: 41820020  beq 0x82ff048c
	if ctx.cr[0].eq {
	pc = 0x82FF048C; continue 'dispatch;
	}
	// 82FF0470: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0474: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0478: 4BFEEE31  bl 0x82fdf2a8
	ctx.lr = 0x82FF047C;
	sub_82FDF2A8(ctx, base);
	// 82FF047C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0480: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82FF0484: 4BE4FA85  bl 0x82e3ff08
	ctx.lr = 0x82FF0488;
	sub_82E3FF08(ctx, base);
	// 82FF0488: 480000D0  b 0x82ff0558
	pc = 0x82FF0558; continue 'dispatch;
	// 82FF048C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF0490: 419A00C8  beq cr6, 0x82ff0558
	if ctx.cr[6].eq {
	pc = 0x82FF0558; continue 'dispatch;
	}
	// 82FF0494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0498: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FF049C: 4BFF2765  bl 0x82fe2c00
	ctx.lr = 0x82FF04A0;
	sub_82FE2C00(ctx, base);
	// 82FF04A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF04A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF04A8: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FF04AC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82FF04B0: 4BFF2751  bl 0x82fe2c00
	ctx.lr = 0x82FF04B4;
	sub_82FE2C00(ctx, base);
	// 82FF04B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF04B8: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FF04BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF04C0: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FF04C4: 4BFF273D  bl 0x82fe2c00
	ctx.lr = 0x82FF04C8;
	sub_82FE2C00(ctx, base);
	// 82FF04C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF04CC: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF04D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF04D4: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82FF04D8: 4BFF1501  bl 0x82fe19d8
	ctx.lr = 0x82FF04DC;
	sub_82FE19D8(ctx, base);
	// 82FF04DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF04E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF04E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF04E8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF04EC: 4BFEEDBD  bl 0x82fdf2a8
	ctx.lr = 0x82FF04F0;
	sub_82FDF2A8(ctx, base);
	// 82FF04F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF04F4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82FF04F8: 4BE4FA11  bl 0x82e3ff08
	ctx.lr = 0x82FF04FC;
	sub_82E3FF08(ctx, base);
	// 82FF04FC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF0500: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF0504: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0508: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF050C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0510: 4E800421  bctrl
	ctx.lr = 0x82FF0514;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0514: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF0518: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF051C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF0520: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0524: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF0528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF052C: 4E800421  bctrl
	ctx.lr = 0x82FF0530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0530: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF0534: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FF0538: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF053C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0540: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF0544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0548: 4E800421  bctrl
	ctx.lr = 0x82FF054C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF054C: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82FF0550: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82FF0554: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FF0558: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF055C: 481B7C60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF0560 size=8
    let mut pc: u32 = 0x82FF0560;
    'dispatch: loop {
        match pc {
            0x82FF0560 => {
    //   block [0x82FF0560..0x82FF0568)
	// 82FF0560: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82FF0564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0568 size=140
    let mut pc: u32 = 0x82FF0568;
    'dispatch: loop {
        match pc {
            0x82FF0568 => {
    //   block [0x82FF0568..0x82FF05F4)
	// 82FF0568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0570: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF0574: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF0578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF057C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0580: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF0584: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82FF0588: 4BFEEE09  bl 0x82fdf390
	ctx.lr = 0x82FF058C;
	sub_82FDF390(ctx, base);
	// 82FF058C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF0590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF0594: 419A0020  beq cr6, 0x82ff05b4
	if ctx.cr[6].eq {
	pc = 0x82FF05B4; continue 'dispatch;
	}
	// 82FF0598: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FF059C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF05A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF05A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF05A8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF05AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF05B0: 4E800421  bctrl
	ctx.lr = 0x82FF05B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF05B4: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF05B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF05BC: 419A0020  beq cr6, 0x82ff05dc
	if ctx.cr[6].eq {
	pc = 0x82FF05DC; continue 'dispatch;
	}
	// 82FF05C0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FF05C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF05C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF05CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF05D0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF05D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF05D8: 4E800421  bctrl
	ctx.lr = 0x82FF05DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF05DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF05E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF05E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF05E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF05EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF05F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF05F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF05F8 size=84
    let mut pc: u32 = 0x82FF05F8;
    'dispatch: loop {
        match pc {
            0x82FF05F8 => {
    //   block [0x82FF05F8..0x82FF064C)
	// 82FF05F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF05FC: 481B7B71  bl 0x831a816c
	ctx.lr = 0x82FF0600;
	sub_831A8130(ctx, base);
	// 82FF0600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0604: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF0608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF060C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF0610: 419A0034  beq cr6, 0x82ff0644
	if ctx.cr[6].eq {
	pc = 0x82FF0644; continue 'dispatch;
	}
	// 82FF0614: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82FF0618: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF061C: 4BFEEC0D  bl 0x82fdf228
	ctx.lr = 0x82FF0620;
	sub_82FDF228(ctx, base);
	// 82FF0620: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0624: 41820010  beq 0x82ff0634
	if ctx.cr[0].eq {
	pc = 0x82FF0634; continue 'dispatch;
	}
	// 82FF0628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF062C: 4BFEEBFD  bl 0x82fdf228
	ctx.lr = 0x82FF0630;
	sub_82FDF228(ctx, base);
	// 82FF0630: 48000008  b 0x82ff0638
	pc = 0x82FF0638; continue 'dispatch;
	// 82FF0634: 4BFFF945  bl 0x82feff78
	ctx.lr = 0x82FF0638;
	sub_82FEFF78(ctx, base);
	// 82FF0638: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF063C: 4BFF25C5  bl 0x82fe2c00
	ctx.lr = 0x82FF0640;
	sub_82FE2C00(ctx, base);
	// 82FF0640: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82FF0644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF0648: 481B7B74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0650 size=76
    let mut pc: u32 = 0x82FF0650;
    'dispatch: loop {
        match pc {
            0x82FF0650 => {
    //   block [0x82FF0650..0x82FF069C)
	// 82FF0650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0654: 481B7B19  bl 0x831a816c
	ctx.lr = 0x82FF0658;
	sub_831A8130(ctx, base);
	// 82FF0658: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF065C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0660: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF0664: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82FF0668: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF066C: 4BFEEBBD  bl 0x82fdf228
	ctx.lr = 0x82FF0670;
	sub_82FDF228(ctx, base);
	// 82FF0670: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0674: 41820010  beq 0x82ff0684
	if ctx.cr[0].eq {
	pc = 0x82FF0684; continue 'dispatch;
	}
	// 82FF0678: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF067C: 4BFEEBAD  bl 0x82fdf228
	ctx.lr = 0x82FF0680;
	sub_82FDF228(ctx, base);
	// 82FF0680: 48000008  b 0x82ff0688
	pc = 0x82FF0688; continue 'dispatch;
	// 82FF0684: 4BFFF8F5  bl 0x82feff78
	ctx.lr = 0x82FF0688;
	sub_82FEFF78(ctx, base);
	// 82FF0688: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF068C: 4BFF2575  bl 0x82fe2c00
	ctx.lr = 0x82FF0690;
	sub_82FE2C00(ctx, base);
	// 82FF0690: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82FF0694: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF0698: 481B7B24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF06A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF06A0 size=76
    let mut pc: u32 = 0x82FF06A0;
    'dispatch: loop {
        match pc {
            0x82FF06A0 => {
    //   block [0x82FF06A0..0x82FF06EC)
	// 82FF06A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF06A4: 481B7AC9  bl 0x831a816c
	ctx.lr = 0x82FF06A8;
	sub_831A8130(ctx, base);
	// 82FF06A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF06AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF06B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF06B4: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82FF06B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF06BC: 4BFEEB6D  bl 0x82fdf228
	ctx.lr = 0x82FF06C0;
	sub_82FDF228(ctx, base);
	// 82FF06C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF06C4: 41820010  beq 0x82ff06d4
	if ctx.cr[0].eq {
	pc = 0x82FF06D4; continue 'dispatch;
	}
	// 82FF06C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF06CC: 4BFEEB5D  bl 0x82fdf228
	ctx.lr = 0x82FF06D0;
	sub_82FDF228(ctx, base);
	// 82FF06D0: 48000008  b 0x82ff06d8
	pc = 0x82FF06D8; continue 'dispatch;
	// 82FF06D4: 4BFFF8A5  bl 0x82feff78
	ctx.lr = 0x82FF06D8;
	sub_82FEFF78(ctx, base);
	// 82FF06D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF06DC: 4BFF2525  bl 0x82fe2c00
	ctx.lr = 0x82FF06E0;
	sub_82FE2C00(ctx, base);
	// 82FF06E0: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82FF06E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF06E8: 481B7AD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF06F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF06F0 size=448
    let mut pc: u32 = 0x82FF06F0;
    'dispatch: loop {
        match pc {
            0x82FF06F0 => {
    //   block [0x82FF06F0..0x82FF08B0)
	// 82FF06F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF06F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF06F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF06FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF0700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0708: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FF070C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF0710: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 82FF0714: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FF0718: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FF071C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FF0720: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FF0724: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0728: 418200A8  beq 0x82ff07d0
	if ctx.cr[0].eq {
	pc = 0x82FF07D0; continue 'dispatch;
	}
	// 82FF072C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FF0730: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 82FF0734: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FF0738: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF073C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF0740: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FF0744: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0748: 41820028  beq 0x82ff0770
	if ctx.cr[0].eq {
	pc = 0x82FF0770; continue 'dispatch;
	}
	// 82FF074C: 897F0041  lbz r11, 0x41(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 82FF0750: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0754: 4082009C  bne 0x82ff07f0
	if !ctx.cr[0].eq {
	pc = 0x82FF07F0; continue 'dispatch;
	}
	// 82FF0758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF075C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0764: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF0768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF076C: 4E800020  blr
	return;
	// 82FF0770: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF0778: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF077C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0780: 4E800421  bctrl
	ctx.lr = 0x82FF0784;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0784: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0788: 41820020  beq 0x82ff07a8
	if ctx.cr[0].eq {
	pc = 0x82FF07A8; continue 'dispatch;
	}
	// 82FF078C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF0794: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF0798: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF079C: 4E800421  bctrl
	ctx.lr = 0x82FF07A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF07A0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF07A4: 4800000C  b 0x82ff07b0
	pc = 0x82FF07B0; continue 'dispatch;
	// 82FF07A8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF07AC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF07B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF07B4: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82FF07B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF07BC: 48009715  bl 0x82ff9ed0
	ctx.lr = 0x82FF07C0;
	sub_82FF9ED0(ctx, base);
	// 82FF07C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF07C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF07C8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FF07CC: 481C045D  bl 0x831b0c28
	ctx.lr = 0x82FF07D0;
	sub_831B0C28(ctx, base);
	// 82FF07D0: 897F0041  lbz r11, 0x41(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 82FF07D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF07D8: 41820034  beq 0x82ff080c
	if ctx.cr[0].eq {
	pc = 0x82FF080C; continue 'dispatch;
	}
	// 82FF07DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FF07E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF07E4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FF07E8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82FF07EC: 4BFEEF8D  bl 0x82fdf778
	ctx.lr = 0x82FF07F0;
	sub_82FDF778(ctx, base);
	// 82FF07F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF07F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF07F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF07FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0804: 4E800421  bctrl
	ctx.lr = 0x82FF0808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0808: 4BFFFF50  b 0x82ff0758
	pc = 0x82FF0758; continue 'dispatch;
	// 82FF080C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF0814: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF0818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF081C: 4E800421  bctrl
	ctx.lr = 0x82FF0820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0820: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF0824: 4182002C  beq 0x82ff0850
	if ctx.cr[0].eq {
	pc = 0x82FF0850; continue 'dispatch;
	}
	// 82FF0828: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FF082C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF0830: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FF0834: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82FF0838: 4BFEEF41  bl 0x82fdf778
	ctx.lr = 0x82FF083C;
	sub_82FDF778(ctx, base);
	// 82FF083C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FF0840: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF0844: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0848: 4BFF4E99  bl 0x82fe56e0
	ctx.lr = 0x82FF084C;
	sub_82FE56E0(ctx, base);
	// 82FF084C: 4BFFFF0C  b 0x82ff0758
	pc = 0x82FF0758; continue 'dispatch;
	// 82FF0850: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF0858: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF085C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0860: 4E800421  bctrl
	ctx.lr = 0x82FF0864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0864: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0868: 41820020  beq 0x82ff0888
	if ctx.cr[0].eq {
	pc = 0x82FF0888; continue 'dispatch;
	}
	// 82FF086C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF0874: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF0878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF087C: 4E800421  bctrl
	ctx.lr = 0x82FF0880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0880: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF0884: 4800000C  b 0x82ff0890
	pc = 0x82FF0890; continue 'dispatch;
	// 82FF0888: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF088C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF0890: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF0894: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82FF0898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF089C: 48009635  bl 0x82ff9ed0
	ctx.lr = 0x82FF08A0;
	sub_82FF9ED0(ctx, base);
	// 82FF08A0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF08A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF08A8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FF08AC: 481C037D  bl 0x831b0c28
	ctx.lr = 0x82FF08B0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF08B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF08B0 size=8
    let mut pc: u32 = 0x82FF08B0;
    'dispatch: loop {
        match pc {
            0x82FF08B0 => {
    //   block [0x82FF08B0..0x82FF08B8)
	// 82FF08B0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82FF08B4: 4BFEEF0C  b 0x82fdf7c0
	sub_82FDF7C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF08B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF08B8 size=8
    let mut pc: u32 = 0x82FF08B8;
    'dispatch: loop {
        match pc {
            0x82FF08B8 => {
    //   block [0x82FF08B8..0x82FF08C0)
	// 82FF08B8: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82FF08BC: 4BFEF3F4  b 0x82fdfcb0
	sub_82FDFCB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF08C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF08C0 size=1012
    let mut pc: u32 = 0x82FF08C0;
    'dispatch: loop {
        match pc {
            0x82FF08C0 => {
    //   block [0x82FF08C0..0x82FF0CB4)
	// 82FF08C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF08C4: 481B7895  bl 0x831a8158
	ctx.lr = 0x82FF08C8;
	sub_831A8130(ctx, base);
	// 82FF08C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF08CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF08D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF08D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF08D8: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FF08DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF08E0: 4E800421  bctrl
	ctx.lr = 0x82FF08E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF08E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF08E8: 4182000C  beq 0x82ff08f4
	if ctx.cr[0].eq {
	pc = 0x82FF08F4; continue 'dispatch;
	}
	// 82FF08EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF08F0: 480003BC  b 0x82ff0cac
	pc = 0x82FF0CAC; continue 'dispatch;
	// 82FF08F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF08F8: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82FF08FC: 4BFEEEDD  bl 0x82fdf7d8
	ctx.lr = 0x82FF0900;
	sub_82FDF7D8(ctx, base);
	// 82FF0900: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF0904: 4082000C  bne 0x82ff0910
	if !ctx.cr[0].eq {
	pc = 0x82FF0910; continue 'dispatch;
	}
	// 82FF0908: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF090C: 480003A0  b 0x82ff0cac
	pc = 0x82FF0CAC; continue 'dispatch;
	// 82FF0910: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0914: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0918: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FF091C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0920: 4E800421  bctrl
	ctx.lr = 0x82FF0924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0924: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0928: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF092C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0930: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FF0934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0938: 40820014  bne 0x82ff094c
	if !ctx.cr[0].eq {
	pc = 0x82FF094C; continue 'dispatch;
	}
	// 82FF093C: 4E800421  bctrl
	ctx.lr = 0x82FF0940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0940: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0944: 41820034  beq 0x82ff0978
	if ctx.cr[0].eq {
	pc = 0x82FF0978; continue 'dispatch;
	}
	// 82FF0948: 4BFFFFC0  b 0x82ff0908
	pc = 0x82FF0908; continue 'dispatch;
	// 82FF094C: 4E800421  bctrl
	ctx.lr = 0x82FF0950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0950: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0958: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF095C: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FF0960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0964: 4E800421  bctrl
	ctx.lr = 0x82FF0968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0968: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF096C: 4BFE32D5  bl 0x82fd3c40
	ctx.lr = 0x82FF0970;
	sub_82FD3C40(ctx, base);
	// 82FF0970: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF0974: 4182FF94  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0978: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF097C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0980: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FF0984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0988: 4E800421  bctrl
	ctx.lr = 0x82FF098C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF098C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0990: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0998: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FF099C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF09A0: 40820014  bne 0x82ff09b4
	if !ctx.cr[0].eq {
	pc = 0x82FF09B4; continue 'dispatch;
	}
	// 82FF09A4: 4E800421  bctrl
	ctx.lr = 0x82FF09A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF09A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF09AC: 41820034  beq 0x82ff09e0
	if ctx.cr[0].eq {
	pc = 0x82FF09E0; continue 'dispatch;
	}
	// 82FF09B0: 4BFFFF58  b 0x82ff0908
	pc = 0x82FF0908; continue 'dispatch;
	// 82FF09B4: 4E800421  bctrl
	ctx.lr = 0x82FF09B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF09B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF09BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF09C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF09C4: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FF09C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF09CC: 4E800421  bctrl
	ctx.lr = 0x82FF09D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF09D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF09D4: 4BFE326D  bl 0x82fd3c40
	ctx.lr = 0x82FF09D8;
	sub_82FD3C40(ctx, base);
	// 82FF09D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF09DC: 4182FF2C  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF09E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF09E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF09E8: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FF09EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF09F0: 4E800421  bctrl
	ctx.lr = 0x82FF09F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF09F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF09F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF09FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0A00: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FF0A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0A08: 40820014  bne 0x82ff0a1c
	if !ctx.cr[0].eq {
	pc = 0x82FF0A1C; continue 'dispatch;
	}
	// 82FF0A0C: 4E800421  bctrl
	ctx.lr = 0x82FF0A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0A10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0A14: 41820034  beq 0x82ff0a48
	if ctx.cr[0].eq {
	pc = 0x82FF0A48; continue 'dispatch;
	}
	// 82FF0A18: 4BFFFEF0  b 0x82ff0908
	pc = 0x82FF0908; continue 'dispatch;
	// 82FF0A1C: 4E800421  bctrl
	ctx.lr = 0x82FF0A20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0A20: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0A24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0A28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0A2C: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FF0A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0A34: 4E800421  bctrl
	ctx.lr = 0x82FF0A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0A38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF0A3C: 4BFE3205  bl 0x82fd3c40
	ctx.lr = 0x82FF0A40;
	sub_82FD3C40(ctx, base);
	// 82FF0A40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF0A44: 4182FEC4  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0A48: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0A4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0A50: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0A54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0A58: 4E800421  bctrl
	ctx.lr = 0x82FF0A5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0A5C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0A60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0A64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0A68: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0A6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0A70: 418200F8  beq 0x82ff0b68
	if ctx.cr[0].eq {
	pc = 0x82FF0B68; continue 'dispatch;
	}
	// 82FF0A74: 4E800421  bctrl
	ctx.lr = 0x82FF0A78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0A78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0A7C: 4182FE8C  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0A80: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0A84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0A88: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0A90: 4E800421  bctrl
	ctx.lr = 0x82FF0A94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0A94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0A98: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FF0A9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0AA0: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0AA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0AA8: 4E800421  bctrl
	ctx.lr = 0x82FF0AAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0AAC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0AB0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FF0AB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF0AB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF0ABC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0AC0: 4E800421  bctrl
	ctx.lr = 0x82FF0AC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0AC4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0AC8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FF0ACC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF0AD0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF0AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0AD8: 4E800421  bctrl
	ctx.lr = 0x82FF0ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0ADC: 7F181840  cmplw cr6, r24, r3
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FF0AE0: 409AFE28  bne cr6, 0x82ff0908
	if !ctx.cr[6].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0AE4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FF0AE8: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82FF0AEC: 419A0088  beq cr6, 0x82ff0b74
	if ctx.cr[6].eq {
	pc = 0x82FF0B74; continue 'dispatch;
	}
	// 82FF0AF0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0AF4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FF0AF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF0AFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF0B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0B04: 4E800421  bctrl
	ctx.lr = 0x82FF0B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0B08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0B0C: 833B0000  lwz r25, 0(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0B10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0B14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF0B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0B1C: 4E800421  bctrl
	ctx.lr = 0x82FF0B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0B20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF0B24: 8179000C  lwz r11, 0xc(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF0B28: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF0B2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0B30: 4E800421  bctrl
	ctx.lr = 0x82FF0B34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0B34: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FF0B38: 4182FDD0  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0B3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF0B44: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FF0B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0B4C: 4E800421  bctrl
	ctx.lr = 0x82FF0B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0B50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF0B54: 4182FDB4  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0B58: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FF0B5C: 7F1AC040  cmplw cr6, r26, r24
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82FF0B60: 4198FF90  blt cr6, 0x82ff0af0
	if ctx.cr[6].lt {
	pc = 0x82FF0AF0; continue 'dispatch;
	}
	// 82FF0B64: 48000010  b 0x82ff0b74
	pc = 0x82FF0B74; continue 'dispatch;
	// 82FF0B68: 4E800421  bctrl
	ctx.lr = 0x82FF0B6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0B6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0B70: 4082FD98  bne 0x82ff0908
	if !ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0B74: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0B78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0B7C: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FF0B80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0B84: 4E800421  bctrl
	ctx.lr = 0x82FF0B88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0B88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0B8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0B90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0B94: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FF0B98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0B9C: 418200F8  beq 0x82ff0c94
	if ctx.cr[0].eq {
	pc = 0x82FF0C94; continue 'dispatch;
	}
	// 82FF0BA0: 4E800421  bctrl
	ctx.lr = 0x82FF0BA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0BA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0BA8: 4182FD60  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0BAC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0BB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0BB4: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FF0BB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0BBC: 4E800421  bctrl
	ctx.lr = 0x82FF0BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0BC0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0BC4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FF0BC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0BCC: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FF0BD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0BD4: 4E800421  bctrl
	ctx.lr = 0x82FF0BD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0BD8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0BDC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FF0BE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF0BE4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF0BE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0BEC: 4E800421  bctrl
	ctx.lr = 0x82FF0BF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0BF0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0BF4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FF0BF8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF0BFC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF0C00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0C04: 4E800421  bctrl
	ctx.lr = 0x82FF0C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0C08: 7F181840  cmplw cr6, r24, r3
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FF0C0C: 409AFCFC  bne cr6, 0x82ff0908
	if !ctx.cr[6].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0C10: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FF0C14: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82FF0C18: 419A0088  beq cr6, 0x82ff0ca0
	if ctx.cr[6].eq {
	pc = 0x82FF0CA0; continue 'dispatch;
	}
	// 82FF0C1C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0C20: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FF0C24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF0C28: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF0C2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0C30: 4E800421  bctrl
	ctx.lr = 0x82FF0C34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF0C38: 833B0000  lwz r25, 0(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0C3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0C40: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF0C44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0C48: 4E800421  bctrl
	ctx.lr = 0x82FF0C4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0C4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF0C50: 8179000C  lwz r11, 0xc(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF0C54: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF0C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0C5C: 4E800421  bctrl
	ctx.lr = 0x82FF0C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0C60: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FF0C64: 4182FCA4  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0C68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF0C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF0C70: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FF0C74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF0C78: 4E800421  bctrl
	ctx.lr = 0x82FF0C7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0C7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF0C80: 4182FC88  beq 0x82ff0908
	if ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0C84: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FF0C88: 7F1AC040  cmplw cr6, r26, r24
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82FF0C8C: 4198FF90  blt cr6, 0x82ff0c1c
	if ctx.cr[6].lt {
	pc = 0x82FF0C1C; continue 'dispatch;
	}
	// 82FF0C90: 48000010  b 0x82ff0ca0
	pc = 0x82FF0CA0; continue 'dispatch;
	// 82FF0C94: 4E800421  bctrl
	ctx.lr = 0x82FF0C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF0C98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0C9C: 4082FC6C  bne 0x82ff0908
	if !ctx.cr[0].eq {
	pc = 0x82FF0908; continue 'dispatch;
	}
	// 82FF0CA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0CA4: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82FF0CA8: 4800B901  bl 0x82ffc5a8
	ctx.lr = 0x82FF0CAC;
	sub_82FFC5A8(ctx, base);
	// 82FF0CAC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FF0CB0: 481B74F8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF0CB8 size=8
    let mut pc: u32 = 0x82FF0CB8;
    'dispatch: loop {
        match pc {
            0x82FF0CB8 => {
    //   block [0x82FF0CB8..0x82FF0CC0)
	// 82FF0CB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF0CBC: 8213E4C0  lwz r16, -0x1b40(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6976 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0CC0 size=488
    let mut pc: u32 = 0x82FF0CC0;
    'dispatch: loop {
        match pc {
            0x82FF0CC0 => {
    //   block [0x82FF0CC0..0x82FF0EA8)
	// 82FF0CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0CC4: 481B749D  bl 0x831a8160
	ctx.lr = 0x82FF0CC8;
	sub_831A8130(ctx, base);
	// 82FF0CC8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF0CCC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0CD0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF0CD4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF0CD8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FF0CDC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FF0CE0: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF0CE4: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 82FF0CE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF0CEC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF0CF0: 396BE278  addi r11, r11, -0x1d88
	ctx.r[11].s64 = ctx.r[11].s64 + -7560;
	// 82FF0CF4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FF0CF8: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82FF0CFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF0D00: 409A0008  bne cr6, 0x82ff0d08
	if !ctx.cr[6].eq {
	pc = 0x82FF0D08; continue 'dispatch;
	}
	// 82FF0D04: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FF0D08: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82FF0D0C: 4BFEE50D  bl 0x82fdf218
	ctx.lr = 0x82FF0D10;
	sub_82FDF218(ctx, base);
	// 82FF0D10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF0D14: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82FF0D18: 4800B781  bl 0x82ffc498
	ctx.lr = 0x82FF0D1C;
	sub_82FFC498(ctx, base);
	// 82FF0D1C: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 82FF0D20: 4819CE91  bl 0x8318dbb0
	ctx.lr = 0x82FF0D24;
	sub_8318DBB0(ctx, base);
	// 82FF0D24: 935E0034  stw r26, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[26].u32 ) };
	// 82FF0D28: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF0D2C: 935E0038  stw r26, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 82FF0D30: 935E003C  stw r26, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[26].u32 ) };
	// 82FF0D34: 9B5E0040  stb r26, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[26].u8 ) };
	// 82FF0D38: 9B9E0041  stb r28, 0x41(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(65 as u32), ctx.r[28].u8 ) };
	// 82FF0D3C: 935E0024  stw r26, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 82FF0D40: 935E0028  stw r26, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[26].u32 ) };
	// 82FF0D44: 935E002C  stw r26, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82FF0D48: 935E0030  stw r26, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[26].u32 ) };
	// 82FF0D4C: 419A00A8  beq cr6, 0x82ff0df4
	if ctx.cr[6].eq {
	pc = 0x82FF0DF4; continue 'dispatch;
	}
	// 82FF0D50: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF0D54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0D58: 4BFF0C81  bl 0x82fe19d8
	ctx.lr = 0x82FF0D5C;
	sub_82FE19D8(ctx, base);
	// 82FF0D5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF0D60: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF0D64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0D68: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF0D6C: 4BFF0C85  bl 0x82fe19f0
	ctx.lr = 0x82FF0D70;
	sub_82FE19F0(ctx, base);
	// 82FF0D70: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF0D74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0D78: 41820014  beq 0x82ff0d8c
	if ctx.cr[0].eq {
	pc = 0x82FF0D8C; continue 'dispatch;
	}
	// 82FF0D7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0D80: 48043C21  bl 0x830349a0
	ctx.lr = 0x82FF0D84;
	sub_830349A0(ctx, base);
	// 82FF0D84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF0D88: 48000008  b 0x82ff0d90
	pc = 0x82FF0D90; continue 'dispatch;
	// 82FF0D8C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82FF0D90: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF0D94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0D98: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FF0D9C: 4BFF0C55  bl 0x82fe19f0
	ctx.lr = 0x82FF0DA0;
	sub_82FE19F0(ctx, base);
	// 82FF0DA0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF0DA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0DA8: 41820014  beq 0x82ff0dbc
	if ctx.cr[0].eq {
	pc = 0x82FF0DBC; continue 'dispatch;
	}
	// 82FF0DAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0DB0: 48043BF1  bl 0x830349a0
	ctx.lr = 0x82FF0DB4;
	sub_830349A0(ctx, base);
	// 82FF0DB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF0DB8: 48000008  b 0x82ff0dc0
	pc = 0x82FF0DC0; continue 'dispatch;
	// 82FF0DBC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82FF0DC0: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF0DC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0DC8: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FF0DCC: 4BFF0C25  bl 0x82fe19f0
	ctx.lr = 0x82FF0DD0;
	sub_82FE19F0(ctx, base);
	// 82FF0DD0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF0DD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0DD8: 41820010  beq 0x82ff0de8
	if ctx.cr[0].eq {
	pc = 0x82FF0DE8; continue 'dispatch;
	}
	// 82FF0DDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0DE0: 48043BC1  bl 0x830349a0
	ctx.lr = 0x82FF0DE4;
	sub_830349A0(ctx, base);
	// 82FF0DE4: 48000008  b 0x82ff0dec
	pc = 0x82FF0DEC; continue 'dispatch;
	// 82FF0DE8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FF0DEC: 907E0030  stw r3, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FF0DF0: 480000AC  b 0x82ff0e9c
	pc = 0x82FF0E9C; continue 'dispatch;
	// 82FF0DF4: 4BFFF185  bl 0x82feff78
	ctx.lr = 0x82FF0DF8;
	sub_82FEFF78(ctx, base);
	// 82FF0DF8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF0DFC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF0E00: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FF0E04: 4BFF0BD5  bl 0x82fe19d8
	ctx.lr = 0x82FF0E08;
	sub_82FE19D8(ctx, base);
	// 82FF0E08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF0E0C: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF0E10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0E14: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF0E18: 4BFF0BD9  bl 0x82fe19f0
	ctx.lr = 0x82FF0E1C;
	sub_82FE19F0(ctx, base);
	// 82FF0E1C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF0E20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0E24: 41820014  beq 0x82ff0e38
	if ctx.cr[0].eq {
	pc = 0x82FF0E38; continue 'dispatch;
	}
	// 82FF0E28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0E2C: 48043B75  bl 0x830349a0
	ctx.lr = 0x82FF0E30;
	sub_830349A0(ctx, base);
	// 82FF0E30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF0E34: 48000008  b 0x82ff0e3c
	pc = 0x82FF0E3C; continue 'dispatch;
	// 82FF0E38: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82FF0E3C: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF0E40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0E44: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FF0E48: 4BFF0BA9  bl 0x82fe19f0
	ctx.lr = 0x82FF0E4C;
	sub_82FE19F0(ctx, base);
	// 82FF0E4C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF0E50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0E54: 41820014  beq 0x82ff0e68
	if ctx.cr[0].eq {
	pc = 0x82FF0E68; continue 'dispatch;
	}
	// 82FF0E58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0E5C: 48043B45  bl 0x830349a0
	ctx.lr = 0x82FF0E60;
	sub_830349A0(ctx, base);
	// 82FF0E60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF0E64: 48000008  b 0x82ff0e6c
	pc = 0x82FF0E6C; continue 'dispatch;
	// 82FF0E68: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82FF0E6C: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF0E70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF0E74: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FF0E78: 4BFF0B79  bl 0x82fe19f0
	ctx.lr = 0x82FF0E7C;
	sub_82FE19F0(ctx, base);
	// 82FF0E7C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF0E80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF0E84: 41820010  beq 0x82ff0e94
	if ctx.cr[0].eq {
	pc = 0x82FF0E94; continue 'dispatch;
	}
	// 82FF0E88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF0E8C: 48043B15  bl 0x830349a0
	ctx.lr = 0x82FF0E90;
	sub_830349A0(ctx, base);
	// 82FF0E90: 48000008  b 0x82ff0e98
	pc = 0x82FF0E98; continue 'dispatch;
	// 82FF0E94: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FF0E98: 907E0030  stw r3, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FF0E9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF0EA0: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF0EA4: 481B730C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0EA8 size=40
    let mut pc: u32 = 0x82FF0EA8;
    'dispatch: loop {
        match pc {
            0x82FF0EA8 => {
    //   block [0x82FF0EA8..0x82FF0ED0)
	// 82FF0EA8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0EAC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0EB0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0EB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0EB8: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0EBC: 48014CC5  bl 0x83005b80
	ctx.lr = 0x82FF0EC0;
	sub_83005B80(ctx, base);
	// 82FF0EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0ED0 size=44
    let mut pc: u32 = 0x82FF0ED0;
    'dispatch: loop {
        match pc {
            0x82FF0ED0 => {
    //   block [0x82FF0ED0..0x82FF0EFC)
	// 82FF0ED0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0ED4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0ED8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0EDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0EE0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0EE4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82FF0EE8: 480D6BF9  bl 0x830c7ae0
	ctx.lr = 0x82FF0EEC;
	sub_830C7AE0(ctx, base);
	// 82FF0EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0EFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0EFC size=44
    let mut pc: u32 = 0x82FF0EFC;
    'dispatch: loop {
        match pc {
            0x82FF0EFC => {
    //   block [0x82FF0EFC..0x82FF0F28)
	// 82FF0EFC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0F0C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0F10: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FF0F14: 4BFF0315  bl 0x82fe1228
	ctx.lr = 0x82FF0F18;
	sub_82FE1228(ctx, base);
	// 82FF0F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0F28 size=44
    let mut pc: u32 = 0x82FF0F28;
    'dispatch: loop {
        match pc {
            0x82FF0F28 => {
    //   block [0x82FF0F28..0x82FF0F54)
	// 82FF0F28: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0F2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0F30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0F34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0F38: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF0F3C: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 82FF0F40: 480D6BA1  bl 0x830c7ae0
	ctx.lr = 0x82FF0F44;
	sub_830C7AE0(ctx, base);
	// 82FF0F44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0F54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0F54 size=44
    let mut pc: u32 = 0x82FF0F54;
    'dispatch: loop {
        match pc {
            0x82FF0F54 => {
    //   block [0x82FF0F54..0x82FF0F80)
	// 82FF0F54: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0F64: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FF0F68: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF0F6C: 480D6B75  bl 0x830c7ae0
	ctx.lr = 0x82FF0F70;
	sub_830C7AE0(ctx, base);
	// 82FF0F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0F80 size=44
    let mut pc: u32 = 0x82FF0F80;
    'dispatch: loop {
        match pc {
            0x82FF0F80 => {
    //   block [0x82FF0F80..0x82FF0FAC)
	// 82FF0F80: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0F84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0F88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0F8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0F90: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FF0F94: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF0F98: 480D6B49  bl 0x830c7ae0
	ctx.lr = 0x82FF0F9C;
	sub_830C7AE0(ctx, base);
	// 82FF0F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0FAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0FAC size=44
    let mut pc: u32 = 0x82FF0FAC;
    'dispatch: loop {
        match pc {
            0x82FF0FAC => {
    //   block [0x82FF0FAC..0x82FF0FD8)
	// 82FF0FAC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0FBC: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FF0FC0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF0FC4: 480D6B1D  bl 0x830c7ae0
	ctx.lr = 0x82FF0FC8;
	sub_830C7AE0(ctx, base);
	// 82FF0FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF0FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF0FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF0FD8 size=44
    let mut pc: u32 = 0x82FF0FD8;
    'dispatch: loop {
        match pc {
            0x82FF0FD8 => {
    //   block [0x82FF0FD8..0x82FF1004)
	// 82FF0FD8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF0FDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF0FE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF0FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF0FE8: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF0FEC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF0FF0: 480D6AF1  bl 0x830c7ae0
	ctx.lr = 0x82FF0FF4;
	sub_830C7AE0(ctx, base);
	// 82FF0FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF0FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF0FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1004 size=44
    let mut pc: u32 = 0x82FF1004;
    'dispatch: loop {
        match pc {
            0x82FF1004 => {
    //   block [0x82FF1004..0x82FF1030)
	// 82FF1004: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF1008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF100C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1014: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF1018: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF101C: 480D6AC5  bl 0x830c7ae0
	ctx.lr = 0x82FF1020;
	sub_830C7AE0(ctx, base);
	// 82FF1020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF102C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1030 size=44
    let mut pc: u32 = 0x82FF1030;
    'dispatch: loop {
        match pc {
            0x82FF1030 => {
    //   block [0x82FF1030..0x82FF105C)
	// 82FF1030: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF1034: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1038: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF103C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1040: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF1044: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF1048: 480D6A99  bl 0x830c7ae0
	ctx.lr = 0x82FF104C;
	sub_830C7AE0(ctx, base);
	// 82FF104C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1060 size=76
    let mut pc: u32 = 0x82FF1060;
    'dispatch: loop {
        match pc {
            0x82FF1060 => {
    //   block [0x82FF1060..0x82FF10AC)
	// 82FF1060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1068: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF106C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF1070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF1078: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF107C: 4BFFF1A5  bl 0x82ff0220
	ctx.lr = 0x82FF1080;
	sub_82FF0220(ctx, base);
	// 82FF1080: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF1084: 4182000C  beq 0x82ff1090
	if ctx.cr[0].eq {
	pc = 0x82FF1090; continue 'dispatch;
	}
	// 82FF1088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF108C: 4B2CF1DD  bl 0x822c0268
	ctx.lr = 0x82FF1090;
	sub_822C0268(ctx, base);
	// 82FF1090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF1094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF1098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF109C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF10A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF10A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF10A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF10B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF10B0 size=8
    let mut pc: u32 = 0x82FF10B0;
    'dispatch: loop {
        match pc {
            0x82FF10B0 => {
    //   block [0x82FF10B0..0x82FF10B8)
	// 82FF10B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF10B4: 8213E5B8  lwz r16, -0x1a48(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF10B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF10B8 size=1112
    let mut pc: u32 = 0x82FF10B8;
    'dispatch: loop {
        match pc {
            0x82FF10B8 => {
    //   block [0x82FF10B8..0x82FF1510)
	// 82FF10B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF10BC: 481B708D  bl 0x831a8148
	ctx.lr = 0x82FF10C0;
	sub_831A8130(ctx, base);
	// 82FF10C0: 3BE1DFD0  addi r31, r1, -0x2030
	ctx.r[31].s64 = ctx.r[1].s64 + -8240;
	// 82FF10C4: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FF10C8: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82FF10CC: 9421DFD0  stwu r1, -0x2030(r1)
	ea = ctx.r[1].u32.wrapping_add(-8240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF10D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF10D4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF10D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF10DC: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82FF10E0: 7CF43B78  mr r20, r7
	ctx.r[20].u64 = ctx.r[7].u64;
	// 82FF10E4: 93DF2044  stw r30, 0x2044(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8260 as u32), ctx.r[30].u32 ) };
	// 82FF10E8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82FF10EC: 939F204C  stw r28, 0x204c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8268 as u32), ctx.r[28].u32 ) };
	// 82FF10F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF10F4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FF10F8: 396BE278  addi r11, r11, -0x1d88
	ctx.r[11].s64 = ctx.r[11].s64 + -7560;
	// 82FF10FC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FF1100: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 82FF1104: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF1108: 409A0008  bne cr6, 0x82ff1110
	if !ctx.cr[6].eq {
	pc = 0x82FF1110; continue 'dispatch;
	}
	// 82FF110C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FF1110: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82FF1114: 4BFEE105  bl 0x82fdf218
	ctx.lr = 0x82FF1118;
	sub_82FDF218(ctx, base);
	// 82FF1118: 3AFE000C  addi r23, r30, 0xc
	ctx.r[23].s64 = ctx.r[30].s64 + 12;
	// 82FF111C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF1120: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FF1124: 4800B375  bl 0x82ffc498
	ctx.lr = 0x82FF1128;
	sub_82FFC498(ctx, base);
	// 82FF1128: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 82FF112C: 4819CA85  bl 0x8318dbb0
	ctx.lr = 0x82FF1130;
	sub_8318DBB0(ctx, base);
	// 82FF1130: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF1134: 931E003C  stw r24, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[24].u32 ) };
	// 82FF1138: 9B1E0040  stb r24, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[24].u8 ) };
	// 82FF113C: 9B7E0041  stb r27, 0x41(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(65 as u32), ctx.r[27].u8 ) };
	// 82FF1140: 931E0024  stw r24, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[24].u32 ) };
	// 82FF1144: 931E0028  stw r24, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[24].u32 ) };
	// 82FF1148: 931E002C  stw r24, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[24].u32 ) };
	// 82FF114C: 931E0030  stw r24, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 82FF1150: 931E0034  stw r24, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[24].u32 ) };
	// 82FF1154: 931E0038  stw r24, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[24].u32 ) };
	// 82FF1158: 4BFF06B9  bl 0x82fe1810
	ctx.lr = 0x82FF115C;
	sub_82FE1810(ctx, base);
	// 82FF115C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FF1160: 40800040  bge 0x82ff11a0
	if !ctx.cr[0].lt {
	pc = 0x82FF11A0; continue 'dispatch;
	}
	// 82FF1164: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1168: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF116C: 4182000C  beq 0x82ff1178
	if ctx.cr[0].eq {
	pc = 0x82FF1178; continue 'dispatch;
	}
	// 82FF1170: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF1174: 4800000C  b 0x82ff1180
	pc = 0x82FF1180; continue 'dispatch;
	// 82FF1178: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF117C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF1180: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF1184: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FF1188: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF118C: 48008D45  bl 0x82ff9ed0
	ctx.lr = 0x82FF1190;
	sub_82FF9ED0(ctx, base);
	// 82FF1190: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF1194: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF1198: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FF119C: 481BFA8D  bl 0x831b0c28
	ctx.lr = 0x82FF11A0;
	sub_831B0C28(ctx, base);
	// 82FF11A0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FF11A4: 40990184  ble cr6, 0x82ff1328
	if !ctx.cr[6].gt {
	pc = 0x82FF1328; continue 'dispatch;
	}
	// 82FF11A8: 2F1B0F9F  cmpwi cr6, r27, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3999, &mut ctx.xer);
	// 82FF11AC: 3EC08339  lis r22, -0x7cc7
	ctx.r[22].s64 = -2093416448;
	// 82FF11B0: 41980064  blt cr6, 0x82ff1214
	if ctx.cr[6].lt {
	pc = 0x82FF1214; continue 'dispatch;
	}
	// 82FF11B4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF11B8: 419A0034  beq cr6, 0x82ff11ec
	if ctx.cr[6].eq {
	pc = 0x82FF11EC; continue 'dispatch;
	}
	// 82FF11BC: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF11C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF11C4: 41820028  beq 0x82ff11ec
	if ctx.cr[0].eq {
	pc = 0x82FF11EC; continue 'dispatch;
	}
	// 82FF11C8: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FF11CC: 48000008  b 0x82ff11d4
	pc = 0x82FF11D4; continue 'dispatch;
	// 82FF11D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF11D4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF11D8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF11DC: 4082FFF4  bne 0x82ff11d0
	if !ctx.cr[0].eq {
	pc = 0x82FF11D0; continue 'dispatch;
	}
	// 82FF11E0: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FF11E4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF11E8: 48000008  b 0x82ff11f0
	pc = 0x82FF11F0; continue 'dispatch;
	// 82FF11EC: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FF11F0: 8076B7E8  lwz r3, -0x4818(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF11F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF11F8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FF11FC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1200: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF1204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF1208: 4E800421  bctrl
	ctx.lr = 0x82FF120C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF120C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FF1210: 48000008  b 0x82ff1218
	pc = 0x82FF1218; continue 'dispatch;
	// 82FF1214: 3B3F0080  addi r25, r31, 0x80
	ctx.r[25].s64 = ctx.r[31].s64 + 128;
	// 82FF1218: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FF121C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF1220: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FF1224: 4BFE0985  bl 0x82fd1ba8
	ctx.lr = 0x82FF1228;
	sub_82FD1BA8(ctx, base);
	// 82FF1228: 577A083C  slwi r26, r27, 1
	ctx.r[26].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82FF122C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FF1230: 7F1ACB2E  sthx r24, r26, r25
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32), ctx.r[24].u16) };
	// 82FF1234: 419A0068  beq cr6, 0x82ff129c
	if ctx.cr[6].eq {
	pc = 0x82FF129C; continue 'dispatch;
	}
	// 82FF1238: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FF123C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1240: 4BFF0401  bl 0x82fe1640
	ctx.lr = 0x82FF1244;
	sub_82FE1640(ctx, base);
	// 82FF1244: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF1248: 4182001C  beq 0x82ff1264
	if ctx.cr[0].eq {
	pc = 0x82FF1264; continue 'dispatch;
	}
	// 82FF124C: 7D7AEA14  add r11, r26, r29
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[29].u64;
	// 82FF1250: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1254: 388B0002  addi r4, r11, 2
	ctx.r[4].s64 = ctx.r[11].s64 + 2;
	// 82FF1258: 4BFF03E9  bl 0x82fe1640
	ctx.lr = 0x82FF125C;
	sub_82FE1640(ctx, base);
	// 82FF125C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF1260: 408200A8  bne 0x82ff1308
	if !ctx.cr[0].eq {
	pc = 0x82FF1308; continue 'dispatch;
	}
	// 82FF1264: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1268: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF126C: 4182000C  beq 0x82ff1278
	if ctx.cr[0].eq {
	pc = 0x82FF1278; continue 'dispatch;
	}
	// 82FF1270: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF1274: 48000008  b 0x82ff127c
	pc = 0x82FF127C; continue 'dispatch;
	// 82FF1278: 80D6B7E8  lwz r6, -0x4818(r22)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF127C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF1280: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FF1284: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF1288: 48008C49  bl 0x82ff9ed0
	ctx.lr = 0x82FF128C;
	sub_82FF9ED0(ctx, base);
	// 82FF128C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF1290: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF1294: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FF1298: 481BF991  bl 0x831b0c28
	ctx.lr = 0x82FF129C;
	sub_831B0C28(ctx, base);
	// 82FF129C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF12A0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FF12A4: 4BFE8455  bl 0x82fd96f8
	ctx.lr = 0x82FF12A8;
	sub_82FD96F8(ctx, base);
	// 82FF12A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF12AC: 418200E8  beq 0x82ff1394
	if ctx.cr[0].eq {
	pc = 0x82FF1394; continue 'dispatch;
	}
	// 82FF12B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF12B4: 419A0034  beq cr6, 0x82ff12e8
	if ctx.cr[6].eq {
	pc = 0x82FF12E8; continue 'dispatch;
	}
	// 82FF12B8: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF12BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF12C0: 41820028  beq 0x82ff12e8
	if ctx.cr[0].eq {
	pc = 0x82FF12E8; continue 'dispatch;
	}
	// 82FF12C4: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FF12C8: 48000008  b 0x82ff12d0
	pc = 0x82FF12D0; continue 'dispatch;
	// 82FF12CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF12D0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF12D4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF12D8: 4082FFF4  bne 0x82ff12cc
	if !ctx.cr[0].eq {
	pc = 0x82FF12CC; continue 'dispatch;
	}
	// 82FF12DC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FF12E0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF12E4: 48000008  b 0x82ff12ec
	pc = 0x82FF12EC; continue 'dispatch;
	// 82FF12E8: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FF12EC: 7D5B5850  subf r10, r27, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82FF12F0: 7D7AEA14  add r11, r26, r29
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[29].u64;
	// 82FF12F4: 388AFFFF  addi r4, r10, -1
	ctx.r[4].s64 = ctx.r[10].s64 + -1;
	// 82FF12F8: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 82FF12FC: 4BFE83FD  bl 0x82fd96f8
	ctx.lr = 0x82FF1300;
	sub_82FD96F8(ctx, base);
	// 82FF1300: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF1304: 41820090  beq 0x82ff1394
	if ctx.cr[0].eq {
	pc = 0x82FF1394; continue 'dispatch;
	}
	// 82FF1308: 2F1B0F9F  cmpwi cr6, r27, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3999, &mut ctx.xer);
	// 82FF130C: 4198001C  blt cr6, 0x82ff1328
	if ctx.cr[6].lt {
	pc = 0x82FF1328; continue 'dispatch;
	}
	// 82FF1310: 8076B7E8  lwz r3, -0x4818(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF1314: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FF1318: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF131C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF1320: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF1324: 4E800421  bctrl
	ctx.lr = 0x82FF1328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF1328: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FF132C: 419A0108  beq cr6, 0x82ff1434
	if ctx.cr[6].eq {
	pc = 0x82FF1434; continue 'dispatch;
	}
	// 82FF1330: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FF1334: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1338: 4BFF18C9  bl 0x82fe2c00
	ctx.lr = 0x82FF133C;
	sub_82FE2C00(ctx, base);
	// 82FF133C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF1340: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82FF1344: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1348: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82FF134C: 4BFF18B5  bl 0x82fe2c00
	ctx.lr = 0x82FF1350;
	sub_82FE2C00(ctx, base);
	// 82FF1350: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF1354: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF1358: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF135C: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FF1360: 4BFF0679  bl 0x82fe19d8
	ctx.lr = 0x82FF1364;
	sub_82FE19D8(ctx, base);
	// 82FF1364: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF1368: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF136C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1370: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF1374: 4BFF067D  bl 0x82fe19f0
	ctx.lr = 0x82FF1378;
	sub_82FE19F0(ctx, base);
	// 82FF1378: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF137C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1380: 4182004C  beq 0x82ff13cc
	if ctx.cr[0].eq {
	pc = 0x82FF13CC; continue 'dispatch;
	}
	// 82FF1384: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF1388: 48043619  bl 0x830349a0
	ctx.lr = 0x82FF138C;
	sub_830349A0(ctx, base);
	// 82FF138C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF1390: 48000040  b 0x82ff13d0
	pc = 0x82FF13D0; continue 'dispatch;
	// 82FF1394: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1398: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF139C: 4182000C  beq 0x82ff13a8
	if ctx.cr[0].eq {
	pc = 0x82FF13A8; continue 'dispatch;
	}
	// 82FF13A0: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF13A4: 48000008  b 0x82ff13ac
	pc = 0x82FF13AC; continue 'dispatch;
	// 82FF13A8: 80D6B7E8  lwz r6, -0x4818(r22)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF13AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF13B0: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FF13B4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF13B8: 48008B19  bl 0x82ff9ed0
	ctx.lr = 0x82FF13BC;
	sub_82FF9ED0(ctx, base);
	// 82FF13BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF13C0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF13C4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FF13C8: 481BF861  bl 0x831b0c28
	ctx.lr = 0x82FF13CC;
	sub_831B0C28(ctx, base);
	// 82FF13CC: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FF13D0: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF13D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF13D8: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FF13DC: 4BFF0615  bl 0x82fe19f0
	ctx.lr = 0x82FF13E0;
	sub_82FE19F0(ctx, base);
	// 82FF13E0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF13E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF13E8: 41820014  beq 0x82ff13fc
	if ctx.cr[0].eq {
	pc = 0x82FF13FC; continue 'dispatch;
	}
	// 82FF13EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF13F0: 480435B1  bl 0x830349a0
	ctx.lr = 0x82FF13F4;
	sub_830349A0(ctx, base);
	// 82FF13F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF13F8: 48000008  b 0x82ff1400
	pc = 0x82FF1400; continue 'dispatch;
	// 82FF13FC: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FF1400: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF1404: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1408: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FF140C: 4BFF05E5  bl 0x82fe19f0
	ctx.lr = 0x82FF1410;
	sub_82FE19F0(ctx, base);
	// 82FF1410: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF1414: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1418: 41820010  beq 0x82ff1428
	if ctx.cr[0].eq {
	pc = 0x82FF1428; continue 'dispatch;
	}
	// 82FF141C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF1420: 48043581  bl 0x830349a0
	ctx.lr = 0x82FF1424;
	sub_830349A0(ctx, base);
	// 82FF1424: 48000008  b 0x82ff142c
	pc = 0x82FF142C; continue 'dispatch;
	// 82FF1428: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FF142C: 907E0030  stw r3, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FF1430: 480000D4  b 0x82ff1504
	pc = 0x82FF1504; continue 'dispatch;
	// 82FF1434: 4BFFEB45  bl 0x82feff78
	ctx.lr = 0x82FF1438;
	sub_82FEFF78(ctx, base);
	// 82FF1438: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FF143C: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FF1440: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FF1444: 4BFF17BD  bl 0x82fe2c00
	ctx.lr = 0x82FF1448;
	sub_82FE2C00(ctx, base);
	// 82FF1448: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF144C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82FF1450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1454: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82FF1458: 4BFF17A9  bl 0x82fe2c00
	ctx.lr = 0x82FF145C;
	sub_82FE2C00(ctx, base);
	// 82FF145C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF1460: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF1464: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF1468: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FF146C: 4BFF056D  bl 0x82fe19d8
	ctx.lr = 0x82FF1470;
	sub_82FE19D8(ctx, base);
	// 82FF1470: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF1474: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF1478: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF147C: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF1480: 4BFF0571  bl 0x82fe19f0
	ctx.lr = 0x82FF1484;
	sub_82FE19F0(ctx, base);
	// 82FF1484: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF1488: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF148C: 41820014  beq 0x82ff14a0
	if ctx.cr[0].eq {
	pc = 0x82FF14A0; continue 'dispatch;
	}
	// 82FF1490: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF1494: 4804350D  bl 0x830349a0
	ctx.lr = 0x82FF1498;
	sub_830349A0(ctx, base);
	// 82FF1498: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF149C: 48000008  b 0x82ff14a4
	pc = 0x82FF14A4; continue 'dispatch;
	// 82FF14A0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FF14A4: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF14A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF14AC: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FF14B0: 4BFF0541  bl 0x82fe19f0
	ctx.lr = 0x82FF14B4;
	sub_82FE19F0(ctx, base);
	// 82FF14B4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF14B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF14BC: 41820014  beq 0x82ff14d0
	if ctx.cr[0].eq {
	pc = 0x82FF14D0; continue 'dispatch;
	}
	// 82FF14C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF14C4: 480434DD  bl 0x830349a0
	ctx.lr = 0x82FF14C8;
	sub_830349A0(ctx, base);
	// 82FF14C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF14CC: 48000008  b 0x82ff14d4
	pc = 0x82FF14D4; continue 'dispatch;
	// 82FF14D0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FF14D4: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 82FF14D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF14DC: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FF14E0: 4BFF0511  bl 0x82fe19f0
	ctx.lr = 0x82FF14E4;
	sub_82FE19F0(ctx, base);
	// 82FF14E4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF14E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF14EC: 41820010  beq 0x82ff14fc
	if ctx.cr[0].eq {
	pc = 0x82FF14FC; continue 'dispatch;
	}
	// 82FF14F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF14F4: 480434AD  bl 0x830349a0
	ctx.lr = 0x82FF14F8;
	sub_830349A0(ctx, base);
	// 82FF14F8: 48000008  b 0x82ff1500
	pc = 0x82FF1500; continue 'dispatch;
	// 82FF14FC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FF1500: 907E0030  stw r3, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FF1504: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1508: 383F2030  addi r1, r31, 0x2030
	ctx.r[1].s64 = ctx.r[31].s64 + 8240;
	// 82FF150C: 481B6C8C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1510 size=40
    let mut pc: u32 = 0x82FF1510;
    'dispatch: loop {
        match pc {
            0x82FF1510 => {
    //   block [0x82FF1510..0x82FF1538)
	// 82FF1510: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF1514: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1518: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF151C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1520: 807F2044  lwz r3, 0x2044(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8260 as u32) ) } as u64;
	// 82FF1524: 4801465D  bl 0x83005b80
	ctx.lr = 0x82FF1528;
	sub_83005B80(ctx, base);
	// 82FF1528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF152C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1538 size=44
    let mut pc: u32 = 0x82FF1538;
    'dispatch: loop {
        match pc {
            0x82FF1538 => {
    //   block [0x82FF1538..0x82FF1564)
	// 82FF1538: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF153C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1540: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1548: 817F2044  lwz r11, 0x2044(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8260 as u32) ) } as u64;
	// 82FF154C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82FF1550: 480D6591  bl 0x830c7ae0
	ctx.lr = 0x82FF1554;
	sub_830C7AE0(ctx, base);
	// 82FF1554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF155C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1564 size=44
    let mut pc: u32 = 0x82FF1564;
    'dispatch: loop {
        match pc {
            0x82FF1564 => {
    //   block [0x82FF1564..0x82FF1590)
	// 82FF1564: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF1568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF156C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1574: 817F2044  lwz r11, 0x2044(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8260 as u32) ) } as u64;
	// 82FF1578: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FF157C: 4BFEFCAD  bl 0x82fe1228
	ctx.lr = 0x82FF1580;
	sub_82FE1228(ctx, base);
	// 82FF1580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF158C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1590 size=44
    let mut pc: u32 = 0x82FF1590;
    'dispatch: loop {
        match pc {
            0x82FF1590 => {
    //   block [0x82FF1590..0x82FF15BC)
	// 82FF1590: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF1594: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1598: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF159C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF15A0: 817F2044  lwz r11, 0x2044(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8260 as u32) ) } as u64;
	// 82FF15A4: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 82FF15A8: 480D6539  bl 0x830c7ae0
	ctx.lr = 0x82FF15AC;
	sub_830C7AE0(ctx, base);
	// 82FF15AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF15B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF15B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF15B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF15BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF15BC size=44
    let mut pc: u32 = 0x82FF15BC;
    'dispatch: loop {
        match pc {
            0x82FF15BC => {
    //   block [0x82FF15BC..0x82FF15E8)
	// 82FF15BC: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF15C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF15C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF15C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF15CC: 809F204C  lwz r4, 0x204c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8268 as u32) ) } as u64;
	// 82FF15D0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF15D4: 480D650D  bl 0x830c7ae0
	ctx.lr = 0x82FF15D8;
	sub_830C7AE0(ctx, base);
	// 82FF15D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF15DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF15E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF15E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF15E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF15E8 size=44
    let mut pc: u32 = 0x82FF15E8;
    'dispatch: loop {
        match pc {
            0x82FF15E8 => {
    //   block [0x82FF15E8..0x82FF1614)
	// 82FF15E8: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF15EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF15F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF15F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF15F8: 809F204C  lwz r4, 0x204c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8268 as u32) ) } as u64;
	// 82FF15FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF1600: 480D64E1  bl 0x830c7ae0
	ctx.lr = 0x82FF1604;
	sub_830C7AE0(ctx, base);
	// 82FF1604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF160C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1614(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1614 size=44
    let mut pc: u32 = 0x82FF1614;
    'dispatch: loop {
        match pc {
            0x82FF1614 => {
    //   block [0x82FF1614..0x82FF1640)
	// 82FF1614: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF1618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF161C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1624: 809F204C  lwz r4, 0x204c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8268 as u32) ) } as u64;
	// 82FF1628: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF162C: 480D64B5  bl 0x830c7ae0
	ctx.lr = 0x82FF1630;
	sub_830C7AE0(ctx, base);
	// 82FF1630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF163C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1640 size=44
    let mut pc: u32 = 0x82FF1640;
    'dispatch: loop {
        match pc {
            0x82FF1640 => {
    //   block [0x82FF1640..0x82FF166C)
	// 82FF1640: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF1644: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1648: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF164C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1650: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF1654: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF1658: 480D6489  bl 0x830c7ae0
	ctx.lr = 0x82FF165C;
	sub_830C7AE0(ctx, base);
	// 82FF165C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF166C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF166C size=44
    let mut pc: u32 = 0x82FF166C;
    'dispatch: loop {
        match pc {
            0x82FF166C => {
    //   block [0x82FF166C..0x82FF1698)
	// 82FF166C: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF1670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF167C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF1680: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF1684: 480D645D  bl 0x830c7ae0
	ctx.lr = 0x82FF1688;
	sub_830C7AE0(ctx, base);
	// 82FF1688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF168C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1698 size=44
    let mut pc: u32 = 0x82FF1698;
    'dispatch: loop {
        match pc {
            0x82FF1698 => {
    //   block [0x82FF1698..0x82FF16C4)
	// 82FF1698: 3BECDFD0  addi r31, r12, -0x2030
	ctx.r[31].s64 = ctx.r[12].s64 + -8240;
	// 82FF169C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF16A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF16A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF16A8: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF16AC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF16B0: 480D6431  bl 0x830c7ae0
	ctx.lr = 0x82FF16B4;
	sub_830C7AE0(ctx, base);
	// 82FF16B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF16B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF16BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF16C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF16C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FF16C8 size=328
    let mut pc: u32 = 0x82FF16C8;
    'dispatch: loop {
        match pc {
            0x82FF16C8 => {
    //   block [0x82FF16C8..0x82FF1810)
	// 82FF16C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF16CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF16D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF16D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF16D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF16DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF16E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF16E4: 396BE6E0  addi r11, r11, -0x1920
	ctx.r[11].s64 = ctx.r[11].s64 + -6432;
	// 82FF16E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF16EC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82FF16F0: 2F0403E5  cmpwi cr6, r4, 0x3e5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 997, &mut ctx.xer);
	// 82FF16F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF16F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF16FC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82FF1700: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FF1704: 396BE690  addi r11, r11, -0x1970
	ctx.r[11].s64 = ctx.r[11].s64 + -6512;
	// 82FF1708: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82FF170C: 40990034  ble cr6, 0x82ff1740
	if !ctx.cr[6].gt {
	pc = 0x82FF1740; continue 'dispatch;
	}
	// 82FF1710: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF1714: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FF1718: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF171C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FF1720: 419A00B8  beq cr6, 0x82ff17d8
	if ctx.cr[6].eq {
	pc = 0x82FF17D8; continue 'dispatch;
	}
	// 82FF1724: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FF1728: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FF172C: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF1730: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF1734: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF1738: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82FF173C: 4198FFD4  blt cr6, 0x82ff1710
	if ctx.cr[6].lt {
	pc = 0x82FF1710; continue 'dispatch;
	}
	// 82FF1740: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF1744: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82FF1748: 391F0014  addi r8, r31, 0x14
	ctx.r[8].s64 = ctx.r[31].s64 + 20;
	// 82FF174C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF1750: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF1754: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82FF1758: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FF175C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF1760: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82FF1764: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FF1768: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FF176C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82FF1770: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82FF1774: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FF1778: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FF177C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82FF1780: 7C0047AE  stfiwx f0, 0, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32, tmp.u32) };
	// 82FF1784: 4BFF026D  bl 0x82fe19f0
	ctx.lr = 0x82FF1788;
	sub_82FE19F0(ctx, base);
	// 82FF1788: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF178C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FF1790: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FF1794: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FF1798: 40990024  ble cr6, 0x82ff17bc
	if !ctx.cr[6].gt {
	pc = 0x82FF17BC; continue 'dispatch;
	}
	// 82FF179C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FF17A0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF17A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF17A8: 7FCA492E  stwx r30, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 82FF17AC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FF17B0: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF17B4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF17B8: 4198FFE8  blt cr6, 0x82ff17a0
	if ctx.cr[6].lt {
	pc = 0x82FF17A0; continue 'dispatch;
	}
	// 82FF17BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF17C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FF17C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF17C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF17CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF17D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF17D4: 4E800020  blr
	return;
	// 82FF17D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF17DC: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FF17E0: 38C00178  li r6, 0x178
	ctx.r[6].s64 = 376;
	// 82FF17E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FF17E8: 3889E6A8  addi r4, r9, -0x1958
	ctx.r[4].s64 = ctx.r[9].s64 + -6488;
	// 82FF17EC: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82FF17F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF17F4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF17F8: 80EA0090  lwz r7, 0x90(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF17FC: 4BFDF85D  bl 0x82fd1058
	ctx.lr = 0x82FF1800;
	sub_82FD1058(ctx, base);
	// 82FF1800: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF1804: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF1808: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FF180C: 481BF41D  bl 0x831b0c28
	ctx.lr = 0x82FF1810;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1810 size=216
    let mut pc: u32 = 0x82FF1810;
    'dispatch: loop {
        match pc {
            0x82FF1810 => {
    //   block [0x82FF1810..0x82FF18E8)
	// 82FF1810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF181C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF1820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF1828: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF182C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1830: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF1834: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF1838: 41980008  blt cr6, 0x82ff1840
	if ctx.cr[6].lt {
	pc = 0x82FF1840; continue 'dispatch;
	}
	// 82FF183C: 48000215  bl 0x82ff1a50
	ctx.lr = 0x82FF1840;
	sub_82FF1A50(ctx, base);
	// 82FF1840: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF1844: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1848: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF184C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FF1850: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1854: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF1858: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF185C: 4E800421  bctrl
	ctx.lr = 0x82FF1860;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF1860: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF1864: 80AB0090  lwz r5, 0x90(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF1868: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF186C: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FF1870: 4BFE0391  bl 0x82fd1c00
	ctx.lr = 0x82FF1874;
	sub_82FD1C00(ctx, base);
	// 82FF1874: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82FF1878: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF187C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF1880: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82FF1884: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FF1888: 48000038  b 0x82ff18c0
	pc = 0x82FF18C0; continue 'dispatch;
	// 82FF188C: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82FF1890: 419A0038  beq cr6, 0x82ff18c8
	if ctx.cr[6].eq {
	pc = 0x82FF18C8; continue 'dispatch;
	}
	// 82FF1894: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1898: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FF189C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF18A0: 41980014  blt cr6, 0x82ff18b4
	if ctx.cr[6].lt {
	pc = 0x82FF18B4; continue 'dispatch;
	}
	// 82FF18A4: 7CEB4B96  divwu r7, r11, r9
	ctx.r[7].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 82FF18A8: 0CC90000  twi 6, r9, 0
	// 82FF18AC: 7D2749D6  mullw r9, r7, r9
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82FF18B0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82FF18B4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF18B8: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FF18BC: 7D27482E  lwzx r9, r7, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF18C0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF18C4: 4082FFC8  bne 0x82ff188c
	if !ctx.cr[0].eq {
	pc = 0x82FF188C; continue 'dispatch;
	}
	// 82FF18C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF18CC: 7FCB412E  stwx r30, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[30].u32) };
	// 82FF18D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF18D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF18D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF18DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF18E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF18E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF18E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF18E8 size=184
    let mut pc: u32 = 0x82FF18E8;
    'dispatch: loop {
        match pc {
            0x82FF18E8 => {
    //   block [0x82FF18E8..0x82FF19A0)
	// 82FF18E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF18EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF18F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF18F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF18F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF18FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF1900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF1904: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1908: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF190C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF1910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF1914: 4E800421  bctrl
	ctx.lr = 0x82FF1918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF1918: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF191C: 80AB0090  lwz r5, 0x90(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF1920: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1924: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FF1928: 4BFE02D9  bl 0x82fd1c00
	ctx.lr = 0x82FF192C;
	sub_82FD1C00(ctx, base);
	// 82FF192C: 39230001  addi r9, r3, 1
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	// 82FF1930: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF1934: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF1938: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82FF193C: 48000030  b 0x82ff196c
	pc = 0x82FF196C; continue 'dispatch;
	// 82FF1940: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FF1944: 419A0038  beq cr6, 0x82ff197c
	if ctx.cr[6].eq {
	pc = 0x82FF197C; continue 'dispatch;
	}
	// 82FF1948: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF194C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FF1950: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF1954: 41980014  blt cr6, 0x82ff1968
	if ctx.cr[6].lt {
	pc = 0x82FF1968; continue 'dispatch;
	}
	// 82FF1958: 7CEB5396  divwu r7, r11, r10
	ctx.r[7].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82FF195C: 0CCA0000  twi 6, r10, 0
	// 82FF1960: 7D4751D6  mullw r10, r7, r10
	ctx.r[10].s64 = (ctx.r[7].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82FF1964: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FF1968: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF196C: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FF1970: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1974: 4082FFCC  bne 0x82ff1940
	if !ctx.cr[0].eq {
	pc = 0x82FF1940; continue 'dispatch;
	}
	// 82FF1978: 48000010  b 0x82ff1988
	pc = 0x82FF1988; continue 'dispatch;
	// 82FF197C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF1980: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82FF1984: 7D4B412E  stwx r10, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 82FF1988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF198C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1994: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF1998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF199C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF19A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF19A0 size=176
    let mut pc: u32 = 0x82FF19A0;
    'dispatch: loop {
        match pc {
            0x82FF19A0 => {
    //   block [0x82FF19A0..0x82FF1A50)
	// 82FF19A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF19A4: 481B67C1  bl 0x831a8164
	ctx.lr = 0x82FF19A8;
	sub_831A8130(ctx, base);
	// 82FF19A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF19AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF19B0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FF19B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF19B8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF19BC: 80AB0090  lwz r5, 0x90(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF19C0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF19C4: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FF19C8: 4BFE0239  bl 0x82fd1c00
	ctx.lr = 0x82FF19CC;
	sub_82FD1C00(ctx, base);
	// 82FF19CC: 3B830001  addi r28, r3, 1
	ctx.r[28].s64 = ctx.r[3].s64 + 1;
	// 82FF19D0: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF19D4: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82FF19D8: 48000054  b 0x82ff1a2c
	pc = 0x82FF1A2C; continue 'dispatch;
	// 82FF19DC: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82FF19E0: 419A0028  beq cr6, 0x82ff1a08
	if ctx.cr[6].eq {
	pc = 0x82FF1A08; continue 'dispatch;
	}
	// 82FF19E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF19E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF19EC: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF19F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF19F4: 4E800421  bctrl
	ctx.lr = 0x82FF19F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF19F8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF19FC: 4BFE2245  bl 0x82fd3c40
	ctx.lr = 0x82FF1A00;
	sub_82FD3C40(ctx, base);
	// 82FF1A00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF1A04: 40820040  bne 0x82ff1a44
	if !ctx.cr[0].eq {
	pc = 0x82FF1A44; continue 'dispatch;
	}
	// 82FF1A08: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1A0C: 7FBDE214  add r29, r29, r28
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82FF1A10: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF1A14: 41980014  blt cr6, 0x82ff1a28
	if ctx.cr[6].lt {
	pc = 0x82FF1A28; continue 'dispatch;
	}
	// 82FF1A18: 7D5D5B96  divwu r10, r29, r11
	ctx.r[10].u32 = ctx.r[29].u32 / ctx.r[11].u32;
	// 82FF1A1C: 0CCB0000  twi 6, r11, 0
	// 82FF1A20: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82FF1A24: 7FABE850  subf r29, r11, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82FF1A28: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF1A2C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF1A30: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF1A34: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1A38: 4082FFA4  bne 0x82ff19dc
	if !ctx.cr[0].eq {
	pc = 0x82FF19DC; continue 'dispatch;
	}
	// 82FF1A3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF1A40: 48000008  b 0x82ff1a48
	pc = 0x82FF1A48; continue 'dispatch;
	// 82FF1A44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF1A4C: 481B6768  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FF1A50 size=288
    let mut pc: u32 = 0x82FF1A50;
    'dispatch: loop {
        match pc {
            0x82FF1A50 => {
    //   block [0x82FF1A50..0x82FF1B70)
	// 82FF1A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1A54: 481B6715  bl 0x831a8168
	ctx.lr = 0x82FF1A58;
	sub_831A8130(ctx, base);
	// 82FF1A58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF1A60: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF1A64: 3BCBE690  addi r30, r11, -0x1970
	ctx.r[30].s64 = ctx.r[11].s64 + -6512;
	// 82FF1A68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF1A6C: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1A70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF1A74: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF1A78: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF1A7C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF1A80: 7D4AF02E  lwzx r10, r10, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FF1A84: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1A88: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FF1A8C: 4082003C  bne 0x82ff1ac8
	if !ctx.cr[0].eq {
	pc = 0x82FF1AC8; continue 'dispatch;
	}
	// 82FF1A90: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82FF1A94: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF1A98: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF1A9C: 38C00178  li r6, 0x178
	ctx.r[6].s64 = 376;
	// 82FF1AA0: 388BE6A8  addi r4, r11, -0x1958
	ctx.r[4].s64 = ctx.r[11].s64 + -6488;
	// 82FF1AA4: 38A000D7  li r5, 0xd7
	ctx.r[5].s64 = 215;
	// 82FF1AA8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF1AAC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FF1AB0: 80E90090  lwz r7, 0x90(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FF1AB4: 4BFDF5A5  bl 0x82fd1058
	ctx.lr = 0x82FF1AB8;
	sub_82FD1058(ctx, base);
	// 82FF1AB8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF1ABC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF1AC0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FF1AC4: 481BF165  bl 0x831b0c28
	ctx.lr = 0x82FF1AC8;
	sub_831B0C28(ctx, base);
	// 82FF1AC8: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FF1ACC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF1AD0: 4BFEFF21  bl 0x82fe19f0
	ctx.lr = 0x82FF1AD4;
	sub_82FE19F0(ctx, base);
	// 82FF1AD4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF1ADC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FF1AE0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FF1AE4: 40990028  ble cr6, 0x82ff1b0c
	if !ctx.cr[6].gt {
	pc = 0x82FF1B0C; continue 'dispatch;
	}
	// 82FF1AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF1AEC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF1AF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF1AF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF1AF8: 7D0A492E  stwx r8, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82FF1AFC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FF1B00: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1B04: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF1B08: 4198FFE4  blt cr6, 0x82ff1aec
	if ctx.cr[6].lt {
	pc = 0x82FF1AEC; continue 'dispatch;
	}
	// 82FF1B0C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1B10: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 82FF1B14: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF1B18: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82FF1B1C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FF1B20: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82FF1B24: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82FF1B28: C01E0014  lfs f0, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FF1B2C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FF1B30: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82FF1B34: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82FF1B38: 419A0030  beq cr6, 0x82ff1b68
	if ctx.cr[6].eq {
	pc = 0x82FF1B68; continue 'dispatch;
	}
	// 82FF1B3C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FF1B40: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1B44: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1B48: 41820014  beq 0x82ff1b5c
	if ctx.cr[0].eq {
	pc = 0x82FF1B5C; continue 'dispatch;
	}
	// 82FF1B4C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82FF1B50: 419A000C  beq cr6, 0x82ff1b5c
	if ctx.cr[6].eq {
	pc = 0x82FF1B5C; continue 'dispatch;
	}
	// 82FF1B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF1B58: 4BFFFCB9  bl 0x82ff1810
	ctx.lr = 0x82FF1B5C;
	sub_82FF1810(ctx, base);
	// 82FF1B5C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FF1B60: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FF1B64: 4082FFDC  bne 0x82ff1b40
	if !ctx.cr[0].eq {
	pc = 0x82FF1B40; continue 'dispatch;
	}
	// 82FF1B68: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FF1B6C: 481B664C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1B70 size=76
    let mut pc: u32 = 0x82FF1B70;
    'dispatch: loop {
        match pc {
            0x82FF1B70 => {
    //   block [0x82FF1B70..0x82FF1BBC)
	// 82FF1B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1B78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF1B7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1B80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF1B84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF1B88: 396BE6E0  addi r11, r11, -0x1920
	ctx.r[11].s64 = ctx.r[11].s64 + -6432;
	// 82FF1B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF1B90: 548907FF  clrlwi. r9, r4, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FF1B94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF1B98: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FF1B9C: 41820008  beq 0x82ff1ba4
	if ctx.cr[0].eq {
	pc = 0x82FF1BA4; continue 'dispatch;
	}
	// 82FF1BA0: 4B2CE6C9  bl 0x822c0268
	ctx.lr = 0x82FF1BA4;
	sub_822C0268(ctx, base);
	// 82FF1BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF1BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF1BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF1BC0 size=8
    let mut pc: u32 = 0x82FF1BC0;
    'dispatch: loop {
        match pc {
            0x82FF1BC0 => {
    //   block [0x82FF1BC0..0x82FF1BC8)
	// 82FF1BC0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF1BC4: 8213E708  lwz r16, -0x18f8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1BC8 size=108
    let mut pc: u32 = 0x82FF1BC8;
    'dispatch: loop {
        match pc {
            0x82FF1BC8 => {
    //   block [0x82FF1BC8..0x82FF1C34)
	// 82FF1BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1BCC: 481B65A1  bl 0x831a816c
	ctx.lr = 0x82FF1BD0;
	sub_831A8130(ctx, base);
	// 82FF1BD0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF1BD4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1BD8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF1BDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FF1BE0: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82FF1BE4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FF1BE8: 9BBE0000  stb r29, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82FF1BEC: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FF1BF0: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FF1BF4: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FF1BF8: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82FF1BFC: 9BBE001C  stb r29, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 82FF1C00: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FF1C04: 909E0014  stw r4, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82FF1C08: 4BFE6691  bl 0x82fd8298
	ctx.lr = 0x82FF1C0C;
	sub_82FD8298(ctx, base);
	// 82FF1C0C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF1C10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1C14: 41820010  beq 0x82ff1c24
	if ctx.cr[0].eq {
	pc = 0x82FF1C24; continue 'dispatch;
	}
	// 82FF1C18: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1C1C: 4BFEC595  bl 0x82fde1b0
	ctx.lr = 0x82FF1C20;
	sub_82FDE1B0(ctx, base);
	// 82FF1C20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF1C24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1C28: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FF1C2C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF1C30: 481B658C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1C34 size=48
    let mut pc: u32 = 0x82FF1C34;
    'dispatch: loop {
        match pc {
            0x82FF1C34 => {
    //   block [0x82FF1C34..0x82FF1C64)
	// 82FF1C34: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF1C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1C44: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FF1C48: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1C4C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF1C50: 4BFE6691  bl 0x82fd82e0
	ctx.lr = 0x82FF1C54;
	sub_82FD82E0(ctx, base);
	// 82FF1C54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF1C68 size=8
    let mut pc: u32 = 0x82FF1C68;
    'dispatch: loop {
        match pc {
            0x82FF1C68 => {
    //   block [0x82FF1C68..0x82FF1C70)
	// 82FF1C68: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF1C6C: 4BFEC6FC  b 0x82fde368
	sub_82FDE368(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF1C70 size=12
    let mut pc: u32 = 0x82FF1C70;
    'dispatch: loop {
        match pc {
            0x82FF1C70 => {
    //   block [0x82FF1C70..0x82FF1C7C)
	// 82FF1C70: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF1C74: 908B0020  stw r4, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82FF1C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1C80 size=192
    let mut pc: u32 = 0x82FF1C80;
    'dispatch: loop {
        match pc {
            0x82FF1C80 => {
    //   block [0x82FF1C80..0x82FF1D40)
	// 82FF1C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1C84: 481B64E9  bl 0x831a816c
	ctx.lr = 0x82FF1C88;
	sub_831A8130(ctx, base);
	// 82FF1C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1C8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF1C90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF1C94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF1C98: 419A0034  beq cr6, 0x82ff1ccc
	if ctx.cr[6].eq {
	pc = 0x82FF1CCC; continue 'dispatch;
	}
	// 82FF1C9C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1CA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1CA4: 41820028  beq 0x82ff1ccc
	if ctx.cr[0].eq {
	pc = 0x82FF1CCC; continue 'dispatch;
	}
	// 82FF1CA8: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FF1CAC: 48000008  b 0x82ff1cb4
	pc = 0x82FF1CB4; continue 'dispatch;
	// 82FF1CB0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF1CB4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1CB8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1CBC: 4082FFF4  bne 0x82ff1cb0
	if !ctx.cr[0].eq {
	pc = 0x82FF1CB0; continue 'dispatch;
	}
	// 82FF1CC0: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FF1CC4: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF1CC8: 48000008  b 0x82ff1cd0
	pc = 0x82FF1CD0; continue 'dispatch;
	// 82FF1CCC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF1CD0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF1CD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1CD8: 4182000C  beq 0x82ff1ce4
	if ctx.cr[0].eq {
	pc = 0x82FF1CE4; continue 'dispatch;
	}
	// 82FF1CDC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF1CE0: 40990044  ble cr6, 0x82ff1d24
	if !ctx.cr[6].gt {
	pc = 0x82FF1D24; continue 'dispatch;
	}
	// 82FF1CE4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1CE8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1CEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1CF0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF1CF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF1CF8: 4E800421  bctrl
	ctx.lr = 0x82FF1CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF1CFC: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82FF1D00: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1D04: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FF1D08: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FF1D0C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF1D10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1D14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF1D18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF1D1C: 4E800421  bctrl
	ctx.lr = 0x82FF1D20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF1D20: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FF1D24: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FF1D28: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1D2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF1D30: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FF1D34: 481B67DD  bl 0x831a8510
	ctx.lr = 0x82FF1D38;
	sub_831A8510(ctx, base);
	// 82FF1D38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF1D3C: 481B6480  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1D40 size=96
    let mut pc: u32 = 0x82FF1D40;
    'dispatch: loop {
        match pc {
            0x82FF1D40 => {
    //   block [0x82FF1D40..0x82FF1DA0)
	// 82FF1D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1D48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF1D4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF1D54: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF1D58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1D5C: 41820018  beq 0x82ff1d74
	if ctx.cr[0].eq {
	pc = 0x82FF1D74; continue 'dispatch;
	}
	// 82FF1D60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1D64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF1D68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF1D70: 4E800421  bctrl
	ctx.lr = 0x82FF1D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF1D74: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1D78: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF1D7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF1D80: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF1D84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF1D88: 4E800421  bctrl
	ctx.lr = 0x82FF1D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF1D8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1D98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF1D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF1DA0 size=8
    let mut pc: u32 = 0x82FF1DA0;
    'dispatch: loop {
        match pc {
            0x82FF1DA0 => {
    //   block [0x82FF1DA0..0x82FF1DA8)
	// 82FF1DA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF1DA4: 8213E784  lwz r16, -0x187c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6268 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1DA8 size=200
    let mut pc: u32 = 0x82FF1DA8;
    'dispatch: loop {
        match pc {
            0x82FF1DA8 => {
    //   block [0x82FF1DA8..0x82FF1E70)
	// 82FF1DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1DAC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FF1DB0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FF1DB4: 481B63A9  bl 0x831a815c
	ctx.lr = 0x82FF1DB8;
	sub_831A8130(ctx, base);
	// 82FF1DB8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FF1DBC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1DC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF1DC4: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82FF1DC8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FF1DCC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF1DD0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FF1DD4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FF1DD8: 993E0000  stb r9, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82FF1DDC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82FF1DE0: 893F00FF  lbz r9, 0xff(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(255 as u32) ) } as u64;
	// 82FF1DE4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FF1DE8: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FF1DEC: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FF1DF0: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FF1DF4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FF1DF8: 993E001C  stb r9, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[9].u8 ) };
	// 82FF1DFC: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FF1E00: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82FF1E04: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82FF1E08: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82FF1E0C: 4BFE648D  bl 0x82fd8298
	ctx.lr = 0x82FF1E10;
	sub_82FD8298(ctx, base);
	// 82FF1E10: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1E14: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1E18: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF1E1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1E20: 41820024  beq 0x82ff1e44
	if ctx.cr[0].eq {
	pc = 0x82FF1E44; continue 'dispatch;
	}
	// 82FF1E24: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FF1E28: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1E2C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FF1E30: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FF1E34: 4BFECDF5  bl 0x82fdec28
	ctx.lr = 0x82FF1E38;
	sub_82FDEC28(ctx, base);
	// 82FF1E38: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1E3C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1E40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF1E44: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FF1E48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1E4C: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FF1E50: 4BFFFE31  bl 0x82ff1c80
	ctx.lr = 0x82FF1E54;
	sub_82FF1C80(ctx, base);
	// 82FF1E54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1E58: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1E5C: 48000008  b 0x82ff1e64
	pc = 0x82FF1E64; continue 'dispatch;
	// 82FF1E60: 83DF00B4  lwz r30, 0xb4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FF1E64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1E68: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FF1E6C: 481B6340  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF1E70 size=8
    let mut pc: u32 = 0x82FF1E70;
    'dispatch: loop {
        match pc {
            0x82FF1E70 => {
    //   block [0x82FF1E70..0x82FF1E78)
	// 82FF1E70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF1E74: 8213E784  lwz r16, -0x187c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6268 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1E78 size=24
    let mut pc: u32 = 0x82FF1E78;
    'dispatch: loop {
        match pc {
            0x82FF1E78 => {
    //   block [0x82FF1E78..0x82FF1E90)
	// 82FF1E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1E84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF1E88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF1E8C: 481BED9D  bl 0x831b0c28
	ctx.lr = 0x82FF1E90;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1E98 size=48
    let mut pc: u32 = 0x82FF1E98;
    'dispatch: loop {
        match pc {
            0x82FF1E98 => {
    //   block [0x82FF1E98..0x82FF1EC8)
	// 82FF1E98: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FF1E9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1EA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1EA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1EA8: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FF1EAC: 4BFFFE95  bl 0x82ff1d40
	ctx.lr = 0x82FF1EB0;
	sub_82FF1D40(ctx, base);
	// 82FF1EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1EB4: 3C6082FF  lis r3, -0x7d01
	ctx.r[3].s64 = -2097217536;
	// 82FF1EB8: 38631E60  addi r3, r3, 0x1e60
	ctx.r[3].s64 = ctx.r[3].s64 + 7776;
	// 82FF1EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1EC4: 481B62E8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1EC8 size=48
    let mut pc: u32 = 0x82FF1EC8;
    'dispatch: loop {
        match pc {
            0x82FF1EC8 => {
    //   block [0x82FF1EC8..0x82FF1EF8)
	// 82FF1EC8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FF1ECC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1ED0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1ED4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1ED8: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FF1EDC: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1EE0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF1EE4: 4BFE63FD  bl 0x82fd82e0
	ctx.lr = 0x82FF1EE8;
	sub_82FD82E0(ctx, base);
	// 82FF1EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF1EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF1EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF1EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF1EF8 size=8
    let mut pc: u32 = 0x82FF1EF8;
    'dispatch: loop {
        match pc {
            0x82FF1EF8 => {
    //   block [0x82FF1EF8..0x82FF1F00)
	// 82FF1EF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF1EFC: 8213E82C  lwz r16, -0x17d4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6100 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1F00 size=188
    let mut pc: u32 = 0x82FF1F00;
    'dispatch: loop {
        match pc {
            0x82FF1F00 => {
    //   block [0x82FF1F00..0x82FF1FBC)
	// 82FF1F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1F04: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FF1F08: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FF1F0C: 481B6255  bl 0x831a8160
	ctx.lr = 0x82FF1F10;
	sub_831A8130(ctx, base);
	// 82FF1F10: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF1F14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1F18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF1F1C: 897F00E7  lbz r11, 0xe7(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(231 as u32) ) } as u64;
	// 82FF1F20: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FF1F24: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF1F28: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FF1F2C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FF1F30: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF1F34: 991E0000  stb r8, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82FF1F38: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FF1F3C: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FF1F40: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FF1F44: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FF1F48: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FF1F4C: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FF1F50: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FF1F54: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82FF1F58: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82FF1F5C: 4BFE633D  bl 0x82fd8298
	ctx.lr = 0x82FF1F60;
	sub_82FD8298(ctx, base);
	// 82FF1F60: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1F64: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1F68: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF1F6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF1F70: 41820020  beq 0x82ff1f90
	if ctx.cr[0].eq {
	pc = 0x82FF1F90; continue 'dispatch;
	}
	// 82FF1F74: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF1F78: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF1F7C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF1F80: 4BFECDD1  bl 0x82fded50
	ctx.lr = 0x82FF1F84;
	sub_82FDED50(ctx, base);
	// 82FF1F84: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1F88: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1F8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF1F90: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FF1F94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1F98: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FF1F9C: 4BFFFCE5  bl 0x82ff1c80
	ctx.lr = 0x82FF1FA0;
	sub_82FF1C80(ctx, base);
	// 82FF1FA0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1FA4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF1FA8: 48000008  b 0x82ff1fb0
	pc = 0x82FF1FB0; continue 'dispatch;
	// 82FF1FAC: 83DF00A4  lwz r30, 0xa4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF1FB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF1FB4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF1FB8: 481B61F8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1FBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF1FBC size=8
    let mut pc: u32 = 0x82FF1FBC;
    'dispatch: loop {
        match pc {
            0x82FF1FBC => {
    //   block [0x82FF1FBC..0x82FF1FC4)
	// 82FF1FBC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF1FC0: 8213E82C  lwz r16, -0x17d4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6100 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1FC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1FC4 size=24
    let mut pc: u32 = 0x82FF1FC4;
    'dispatch: loop {
        match pc {
            0x82FF1FC4 => {
    //   block [0x82FF1FC4..0x82FF1FDC)
	// 82FF1FC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1FC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1FCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1FD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF1FD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF1FD8: 481BEC51  bl 0x831b0c28
	ctx.lr = 0x82FF1FDC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF1FE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF1FE4 size=48
    let mut pc: u32 = 0x82FF1FE4;
    'dispatch: loop {
        match pc {
            0x82FF1FE4 => {
    //   block [0x82FF1FE4..0x82FF2014)
	// 82FF1FE4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF1FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF1FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF1FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF1FF4: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF1FF8: 4BFFFD49  bl 0x82ff1d40
	ctx.lr = 0x82FF1FFC;
	sub_82FF1D40(ctx, base);
	// 82FF1FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF2000: 3C6082FF  lis r3, -0x7d01
	ctx.r[3].s64 = -2097217536;
	// 82FF2004: 38631FAC  addi r3, r3, 0x1fac
	ctx.r[3].s64 = ctx.r[3].s64 + 8108;
	// 82FF2008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF200C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF2010: 481B61A0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2014 size=48
    let mut pc: u32 = 0x82FF2014;
    'dispatch: loop {
        match pc {
            0x82FF2014 => {
    //   block [0x82FF2014..0x82FF2044)
	// 82FF2014: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF2018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF201C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF2020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF2024: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF2028: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF202C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF2030: 4BFE62B1  bl 0x82fd82e0
	ctx.lr = 0x82FF2034;
	sub_82FD82E0(ctx, base);
	// 82FF2034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF2038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF203C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF2040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


