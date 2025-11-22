pub fn sub_8323DCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DCE0 size=12
    let mut pc: u32 = 0x8323DCE0;
    'dispatch: loop {
        match pc {
            0x8323DCE0 => {
    //   block [0x8323DCE0..0x8323DCEC)
	// 8323DCE0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DCE4: 386B2038  addi r3, r11, 0x2038
	ctx.r[3].s64 = ctx.r[11].s64 + 8248;
	// 8323DCE8: 4BF6A7F0  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DCF0 size=12
    let mut pc: u32 = 0x8323DCF0;
    'dispatch: loop {
        match pc {
            0x8323DCF0 => {
    //   block [0x8323DCF0..0x8323DCFC)
	// 8323DCF0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DCF4: 386B2060  addi r3, r11, 0x2060
	ctx.r[3].s64 = ctx.r[11].s64 + 8288;
	// 8323DCF8: 4BF6A7E0  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD00 size=12
    let mut pc: u32 = 0x8323DD00;
    'dispatch: loop {
        match pc {
            0x8323DD00 => {
    //   block [0x8323DD00..0x8323DD0C)
	// 8323DD00: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DD04: 386B2088  addi r3, r11, 0x2088
	ctx.r[3].s64 = ctx.r[11].s64 + 8328;
	// 8323DD08: 4BF6A7D0  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD10 size=12
    let mut pc: u32 = 0x8323DD10;
    'dispatch: loop {
        match pc {
            0x8323DD10 => {
    //   block [0x8323DD10..0x8323DD1C)
	// 8323DD10: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DD14: 386B20B0  addi r3, r11, 0x20b0
	ctx.r[3].s64 = ctx.r[11].s64 + 8368;
	// 8323DD18: 4BF6A7C0  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD20 size=12
    let mut pc: u32 = 0x8323DD20;
    'dispatch: loop {
        match pc {
            0x8323DD20 => {
    //   block [0x8323DD20..0x8323DD2C)
	// 8323DD20: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD24: 386BB8C4  addi r3, r11, -0x473c
	ctx.r[3].s64 = ctx.r[11].s64 + -18236;
	// 8323DD28: 4BDB9F20  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD30 size=12
    let mut pc: u32 = 0x8323DD30;
    'dispatch: loop {
        match pc {
            0x8323DD30 => {
    //   block [0x8323DD30..0x8323DD3C)
	// 8323DD30: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD34: 386BB8D0  addi r3, r11, -0x4730
	ctx.r[3].s64 = ctx.r[11].s64 + -18224;
	// 8323DD38: 4BDB9F10  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD40 size=12
    let mut pc: u32 = 0x8323DD40;
    'dispatch: loop {
        match pc {
            0x8323DD40 => {
    //   block [0x8323DD40..0x8323DD4C)
	// 8323DD40: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD44: 386BB8B8  addi r3, r11, -0x4748
	ctx.r[3].s64 = ctx.r[11].s64 + -18248;
	// 8323DD48: 4BDB9F00  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD50 size=12
    let mut pc: u32 = 0x8323DD50;
    'dispatch: loop {
        match pc {
            0x8323DD50 => {
    //   block [0x8323DD50..0x8323DD5C)
	// 8323DD50: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD54: 386BB8E0  addi r3, r11, -0x4720
	ctx.r[3].s64 = ctx.r[11].s64 + -18208;
	// 8323DD58: 4BDB9EF0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD60 size=12
    let mut pc: u32 = 0x8323DD60;
    'dispatch: loop {
        match pc {
            0x8323DD60 => {
    //   block [0x8323DD60..0x8323DD6C)
	// 8323DD60: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD64: 386BB90C  addi r3, r11, -0x46f4
	ctx.r[3].s64 = ctx.r[11].s64 + -18164;
	// 8323DD68: 4BDB9EE0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD70 size=12
    let mut pc: u32 = 0x8323DD70;
    'dispatch: loop {
        match pc {
            0x8323DD70 => {
    //   block [0x8323DD70..0x8323DD7C)
	// 8323DD70: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD74: 386BB900  addi r3, r11, -0x4700
	ctx.r[3].s64 = ctx.r[11].s64 + -18176;
	// 8323DD78: 4BDB9ED0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD80 size=12
    let mut pc: u32 = 0x8323DD80;
    'dispatch: loop {
        match pc {
            0x8323DD80 => {
    //   block [0x8323DD80..0x8323DD8C)
	// 8323DD80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD84: 386BB924  addi r3, r11, -0x46dc
	ctx.r[3].s64 = ctx.r[11].s64 + -18140;
	// 8323DD88: 4BDB9EC0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DD90 size=12
    let mut pc: u32 = 0x8323DD90;
    'dispatch: loop {
        match pc {
            0x8323DD90 => {
    //   block [0x8323DD90..0x8323DD9C)
	// 8323DD90: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DD94: 386BB930  addi r3, r11, -0x46d0
	ctx.r[3].s64 = ctx.r[11].s64 + -18128;
	// 8323DD98: 4BDB9EB0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DDA0 size=12
    let mut pc: u32 = 0x8323DDA0;
    'dispatch: loop {
        match pc {
            0x8323DDA0 => {
    //   block [0x8323DDA0..0x8323DDAC)
	// 8323DDA0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DDA4: 386BB94C  addi r3, r11, -0x46b4
	ctx.r[3].s64 = ctx.r[11].s64 + -18100;
	// 8323DDA8: 4BDB9EA0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DDB0 size=12
    let mut pc: u32 = 0x8323DDB0;
    'dispatch: loop {
        match pc {
            0x8323DDB0 => {
    //   block [0x8323DDB0..0x8323DDBC)
	// 8323DDB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DDB4: 386BB96C  addi r3, r11, -0x4694
	ctx.r[3].s64 = ctx.r[11].s64 + -18068;
	// 8323DDB8: 4BDB9E90  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DDC0 size=12
    let mut pc: u32 = 0x8323DDC0;
    'dispatch: loop {
        match pc {
            0x8323DDC0 => {
    //   block [0x8323DDC0..0x8323DDCC)
	// 8323DDC0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DDC4: 386BB960  addi r3, r11, -0x46a0
	ctx.r[3].s64 = ctx.r[11].s64 + -18080;
	// 8323DDC8: 4BDB9E80  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DDD0 size=48
    let mut pc: u32 = 0x8323DDD0;
    'dispatch: loop {
        match pc {
            0x8323DDD0 => {
    //   block [0x8323DDD0..0x8323DE00)
	// 8323DDD0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8323DDD4: 394B7E04  addi r10, r11, 0x7e04
	ctx.r[10].s64 = ctx.r[11].s64 + 32260;
	// 8323DDD8: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 8323DDDC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8323DDE0: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8323DDE4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8323DDE8: 4082FFF4  bne 0x8323dddc
	if !ctx.cr[0].eq {
	pc = 0x8323DDDC; continue 'dispatch;
	}
	// 8323DDEC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8323DDF0: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8323DDF4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8323DDF8: 916AB980  stw r11, -0x4680(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18048 as u32), ctx.r[11].u32 ) };
	// 8323DDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE00 size=12
    let mut pc: u32 = 0x8323DE00;
    'dispatch: loop {
        match pc {
            0x8323DE00 => {
    //   block [0x8323DE00..0x8323DE0C)
	// 8323DE00: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE04: 386BB990  addi r3, r11, -0x4670
	ctx.r[3].s64 = ctx.r[11].s64 + -18032;
	// 8323DE08: 4BDB9E40  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE10 size=12
    let mut pc: u32 = 0x8323DE10;
    'dispatch: loop {
        match pc {
            0x8323DE10 => {
    //   block [0x8323DE10..0x8323DE1C)
	// 8323DE10: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE14: 386BB99C  addi r3, r11, -0x4664
	ctx.r[3].s64 = ctx.r[11].s64 + -18020;
	// 8323DE18: 4BDB9E30  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE20 size=12
    let mut pc: u32 = 0x8323DE20;
    'dispatch: loop {
        match pc {
            0x8323DE20 => {
    //   block [0x8323DE20..0x8323DE2C)
	// 8323DE20: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE24: 386BB9B4  addi r3, r11, -0x464c
	ctx.r[3].s64 = ctx.r[11].s64 + -17996;
	// 8323DE28: 4BDB9E20  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE30 size=12
    let mut pc: u32 = 0x8323DE30;
    'dispatch: loop {
        match pc {
            0x8323DE30 => {
    //   block [0x8323DE30..0x8323DE3C)
	// 8323DE30: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE34: 386BB9CC  addi r3, r11, -0x4634
	ctx.r[3].s64 = ctx.r[11].s64 + -17972;
	// 8323DE38: 4BDB9E10  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE40 size=12
    let mut pc: u32 = 0x8323DE40;
    'dispatch: loop {
        match pc {
            0x8323DE40 => {
    //   block [0x8323DE40..0x8323DE4C)
	// 8323DE40: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE44: 386BBBEC  addi r3, r11, -0x4414
	ctx.r[3].s64 = ctx.r[11].s64 + -17428;
	// 8323DE48: 4BDB9E00  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE50 size=12
    let mut pc: u32 = 0x8323DE50;
    'dispatch: loop {
        match pc {
            0x8323DE50 => {
    //   block [0x8323DE50..0x8323DE5C)
	// 8323DE50: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE54: 386BBC04  addi r3, r11, -0x43fc
	ctx.r[3].s64 = ctx.r[11].s64 + -17404;
	// 8323DE58: 4BDB9DF0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE60 size=12
    let mut pc: u32 = 0x8323DE60;
    'dispatch: loop {
        match pc {
            0x8323DE60 => {
    //   block [0x8323DE60..0x8323DE6C)
	// 8323DE60: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE64: 386BBBF8  addi r3, r11, -0x4408
	ctx.r[3].s64 = ctx.r[11].s64 + -17416;
	// 8323DE68: 4BDB9DE0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE70 size=12
    let mut pc: u32 = 0x8323DE70;
    'dispatch: loop {
        match pc {
            0x8323DE70 => {
    //   block [0x8323DE70..0x8323DE7C)
	// 8323DE70: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE74: 386BBC2C  addi r3, r11, -0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + -17364;
	// 8323DE78: 4BDB9DD0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE80 size=12
    let mut pc: u32 = 0x8323DE80;
    'dispatch: loop {
        match pc {
            0x8323DE80 => {
    //   block [0x8323DE80..0x8323DE8C)
	// 8323DE80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE84: 386BBC40  addi r3, r11, -0x43c0
	ctx.r[3].s64 = ctx.r[11].s64 + -17344;
	// 8323DE88: 4BDB9DC0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DE90 size=12
    let mut pc: u32 = 0x8323DE90;
    'dispatch: loop {
        match pc {
            0x8323DE90 => {
    //   block [0x8323DE90..0x8323DE9C)
	// 8323DE90: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DE94: 386BBC64  addi r3, r11, -0x439c
	ctx.r[3].s64 = ctx.r[11].s64 + -17308;
	// 8323DE98: 4BDB9DB0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DEA0 size=12
    let mut pc: u32 = 0x8323DEA0;
    'dispatch: loop {
        match pc {
            0x8323DEA0 => {
    //   block [0x8323DEA0..0x8323DEAC)
	// 8323DEA0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DEA4: 386BBC58  addi r3, r11, -0x43a8
	ctx.r[3].s64 = ctx.r[11].s64 + -17320;
	// 8323DEA8: 4BDB9DA0  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DEB0 size=12
    let mut pc: u32 = 0x8323DEB0;
    'dispatch: loop {
        match pc {
            0x8323DEB0 => {
    //   block [0x8323DEB0..0x8323DEBC)
	// 8323DEB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DEB4: 386BBC70  addi r3, r11, -0x4390
	ctx.r[3].s64 = ctx.r[11].s64 + -17296;
	// 8323DEB8: 4BDB9D90  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DEC0 size=12
    let mut pc: u32 = 0x8323DEC0;
    'dispatch: loop {
        match pc {
            0x8323DEC0 => {
    //   block [0x8323DEC0..0x8323DECC)
	// 8323DEC0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DEC4: 386BBC90  addi r3, r11, -0x4370
	ctx.r[3].s64 = ctx.r[11].s64 + -17264;
	// 8323DEC8: 4BDB9D80  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DED0 size=12
    let mut pc: u32 = 0x8323DED0;
    'dispatch: loop {
        match pc {
            0x8323DED0 => {
    //   block [0x8323DED0..0x8323DEDC)
	// 8323DED0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DED4: 386BBC84  addi r3, r11, -0x437c
	ctx.r[3].s64 = ctx.r[11].s64 + -17276;
	// 8323DED8: 4BDB9D70  b 0x82ff7c48
	sub_82FF7C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DEE0 size=36
    let mut pc: u32 = 0x8323DEE0;
    'dispatch: loop {
        match pc {
            0x8323DEE0 => {
    //   block [0x8323DEE0..0x8323DF04)
	// 8323DEE0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323DEE8: 396BBD30  addi r11, r11, -0x42d0
	ctx.r[11].s64 = ctx.r[11].s64 + -17104;
	// 8323DEEC: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 8323DEF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8323DEF4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8323DEF8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8323DEFC: 4200FFF8  bdnz 0x8323def4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8323DEF4; continue 'dispatch;
	}
	// 8323DF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DF08 size=84
    let mut pc: u32 = 0x8323DF08;
    'dispatch: loop {
        match pc {
            0x8323DF08 => {
    //   block [0x8323DF08..0x8323DF5C)
	// 8323DF08: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DF0C: 38E0001F  li r7, 0x1f
	ctx.r[7].s64 = 31;
	// 8323DF10: 396BBE08  addi r11, r11, -0x41f8
	ctx.r[11].s64 = ctx.r[11].s64 + -16888;
	// 8323DF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323DF18: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 8323DF1C: 912AFFFC  stw r9, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 8323DF20: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8323DF24: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8323DF28: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8323DF2C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8323DF30: 4080FFEC  bge 0x8323df1c
	if !ctx.cr[0].lt {
	pc = 0x8323DF1C; continue 'dispatch;
	}
	// 8323DF34: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8323DF38: 912B020C  stw r9, 0x20c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(524 as u32), ctx.r[9].u32 ) };
	// 8323DF3C: 3CE08324  lis r7, -0x7cdc
	ctx.r[7].s64 = -2094792704;
	// 8323DF40: 912B0218  stw r9, 0x218(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(536 as u32), ctx.r[9].u32 ) };
	// 8323DF44: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 8323DF48: 914B0208  stw r10, 0x208(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(520 as u32), ctx.r[10].u32 ) };
	// 8323DF4C: 914B0214  stw r10, 0x214(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(532 as u32), ctx.r[10].u32 ) };
	// 8323DF50: 38672110  addi r3, r7, 0x2110
	ctx.r[3].s64 = ctx.r[7].s64 + 8464;
	// 8323DF54: 910B0210  stw r8, 0x210(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(528 as u32), ctx.r[8].u32 ) };
	// 8323DF58: 4BF6A580  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DF60 size=12
    let mut pc: u32 = 0x8323DF60;
    'dispatch: loop {
        match pc {
            0x8323DF60 => {
    //   block [0x8323DF60..0x8323DF6C)
	// 8323DF60: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DF64: 386B2120  addi r3, r11, 0x2120
	ctx.r[3].s64 = ctx.r[11].s64 + 8480;
	// 8323DF68: 4BF6A570  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DF70 size=12
    let mut pc: u32 = 0x8323DF70;
    'dispatch: loop {
        match pc {
            0x8323DF70 => {
    //   block [0x8323DF70..0x8323DF7C)
	// 8323DF70: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DF74: 386B2130  addi r3, r11, 0x2130
	ctx.r[3].s64 = ctx.r[11].s64 + 8496;
	// 8323DF78: 4BF6A560  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DF80 size=12
    let mut pc: u32 = 0x8323DF80;
    'dispatch: loop {
        match pc {
            0x8323DF80 => {
    //   block [0x8323DF80..0x8323DF8C)
	// 8323DF80: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DF84: 386B2140  addi r3, r11, 0x2140
	ctx.r[3].s64 = ctx.r[11].s64 + 8512;
	// 8323DF88: 4BF6A550  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323DF90 size=12
    let mut pc: u32 = 0x8323DF90;
    'dispatch: loop {
        match pc {
            0x8323DF90 => {
    //   block [0x8323DF90..0x8323DF9C)
	// 8323DF90: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DF94: 386B23A8  addi r3, r11, 0x23a8
	ctx.r[3].s64 = ctx.r[11].s64 + 9128;
	// 8323DF98: 4BF6A540  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323DFA0 size=80
    let mut pc: u32 = 0x8323DFA0;
    'dispatch: loop {
        match pc {
            0x8323DFA0 => {
    //   block [0x8323DFA0..0x8323DFF0)
	// 8323DFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323DFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323DFA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323DFAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323DFB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323DFB4: 3BEBC1B0  addi r31, r11, -0x3e50
	ctx.r[31].s64 = ctx.r[11].s64 + -15952;
	// 8323DFB8: 387F0200  addi r3, r31, 0x200
	ctx.r[3].s64 = ctx.r[31].s64 + 512;
	// 8323DFBC: 48004A21  bl 0x832429dc
	ctx.lr = 0x8323DFC0;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8323DFC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323DFC4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8323DFC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323DFCC: 4BF6A215  bl 0x831a81e0
	ctx.lr = 0x8323DFD0;
	sub_831A81E0(ctx, base);
	// 8323DFD0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323DFD4: 386B2148  addi r3, r11, 0x2148
	ctx.r[3].s64 = ctx.r[11].s64 + 8520;
	// 8323DFD8: 4BF6A501  bl 0x831a84d8
	ctx.lr = 0x8323DFDC;
	sub_831A84D8(ctx, base);
	// 8323DFDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323DFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323DFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323DFE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323DFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323DFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323DFF0 size=60
    let mut pc: u32 = 0x8323DFF0;
    'dispatch: loop {
        match pc {
            0x8323DFF0 => {
    //   block [0x8323DFF0..0x8323E02C)
	// 8323DFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323DFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323DFF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323DFFC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E000: 38A0006C  li r5, 0x6c
	ctx.r[5].s64 = 108;
	// 8323E004: 386BC3E0  addi r3, r11, -0x3c20
	ctx.r[3].s64 = ctx.r[11].s64 + -15392;
	// 8323E008: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323E00C: 4BF6A1D5  bl 0x831a81e0
	ctx.lr = 0x8323E010;
	sub_831A81E0(ctx, base);
	// 8323E010: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E014: 386B23F0  addi r3, r11, 0x23f0
	ctx.r[3].s64 = ctx.r[11].s64 + 9200;
	// 8323E018: 4BF6A4C1  bl 0x831a84d8
	ctx.lr = 0x8323E01C;
	sub_831A84D8(ctx, base);
	// 8323E01C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E030 size=52
    let mut pc: u32 = 0x8323E030;
    'dispatch: loop {
        match pc {
            0x8323E030 => {
    //   block [0x8323E030..0x8323E064)
	// 8323E030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E03C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E040: 386BC3DC  addi r3, r11, -0x3c24
	ctx.r[3].s64 = ctx.r[11].s64 + -15396;
	// 8323E044: 4BE900C5  bl 0x830ce108
	ctx.lr = 0x8323E048;
	sub_830CE108(ctx, base);
	// 8323E048: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E04C: 386B2150  addi r3, r11, 0x2150
	ctx.r[3].s64 = ctx.r[11].s64 + 8528;
	// 8323E050: 4BF6A489  bl 0x831a84d8
	ctx.lr = 0x8323E054;
	sub_831A84D8(ctx, base);
	// 8323E054: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E068 size=12
    let mut pc: u32 = 0x8323E068;
    'dispatch: loop {
        match pc {
            0x8323E068 => {
    //   block [0x8323E068..0x8323E074)
	// 8323E068: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E06C: 386B2470  addi r3, r11, 0x2470
	ctx.r[3].s64 = ctx.r[11].s64 + 9328;
	// 8323E070: 4BF6A468  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E078 size=12
    let mut pc: u32 = 0x8323E078;
    'dispatch: loop {
        match pc {
            0x8323E078 => {
    //   block [0x8323E078..0x8323E084)
	// 8323E078: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E07C: 386B2480  addi r3, r11, 0x2480
	ctx.r[3].s64 = ctx.r[11].s64 + 9344;
	// 8323E080: 4BF6A458  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E088 size=12
    let mut pc: u32 = 0x8323E088;
    'dispatch: loop {
        match pc {
            0x8323E088 => {
    //   block [0x8323E088..0x8323E094)
	// 8323E088: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E08C: 386B2460  addi r3, r11, 0x2460
	ctx.r[3].s64 = ctx.r[11].s64 + 9312;
	// 8323E090: 4BF6A448  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E098 size=12
    let mut pc: u32 = 0x8323E098;
    'dispatch: loop {
        match pc {
            0x8323E098 => {
    //   block [0x8323E098..0x8323E0A4)
	// 8323E098: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E09C: 386B24C0  addi r3, r11, 0x24c0
	ctx.r[3].s64 = ctx.r[11].s64 + 9408;
	// 8323E0A0: 4BF6A438  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E0A8 size=12
    let mut pc: u32 = 0x8323E0A8;
    'dispatch: loop {
        match pc {
            0x8323E0A8 => {
    //   block [0x8323E0A8..0x8323E0B4)
	// 8323E0A8: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E0AC: 386B24D0  addi r3, r11, 0x24d0
	ctx.r[3].s64 = ctx.r[11].s64 + 9424;
	// 8323E0B0: 4BF6A428  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E0B8 size=56
    let mut pc: u32 = 0x8323E0B8;
    'dispatch: loop {
        match pc {
            0x8323E0B8 => {
    //   block [0x8323E0B8..0x8323E0F0)
	// 8323E0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E0C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E0C4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E0C8: 396BD998  addi r11, r11, -0x2668
	ctx.r[11].s64 = ctx.r[11].s64 + -9832;
	// 8323E0CC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8323E0D0: 4800490D  bl 0x832429dc
	ctx.lr = 0x8323E0D4;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8323E0D4: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E0D8: 386B24E0  addi r3, r11, 0x24e0
	ctx.r[3].s64 = ctx.r[11].s64 + 9440;
	// 8323E0DC: 4BF6A3FD  bl 0x831a84d8
	ctx.lr = 0x8323E0E0;
	sub_831A84D8(ctx, base);
	// 8323E0E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E0F0 size=56
    let mut pc: u32 = 0x8323E0F0;
    'dispatch: loop {
        match pc {
            0x8323E0F0 => {
    //   block [0x8323E0F0..0x8323E128)
	// 8323E0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E0F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E0FC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E100: 396BD9C4  addi r11, r11, -0x263c
	ctx.r[11].s64 = ctx.r[11].s64 + -9788;
	// 8323E104: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8323E108: 480048D5  bl 0x832429dc
	ctx.lr = 0x8323E10C;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8323E10C: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E110: 386B24F0  addi r3, r11, 0x24f0
	ctx.r[3].s64 = ctx.r[11].s64 + 9456;
	// 8323E114: 4BF6A3C5  bl 0x831a84d8
	ctx.lr = 0x8323E118;
	sub_831A84D8(ctx, base);
	// 8323E118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E128 size=56
    let mut pc: u32 = 0x8323E128;
    'dispatch: loop {
        match pc {
            0x8323E128 => {
    //   block [0x8323E128..0x8323E160)
	// 8323E128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E134: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E138: 396BD9F0  addi r11, r11, -0x2610
	ctx.r[11].s64 = ctx.r[11].s64 + -9744;
	// 8323E13C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8323E140: 4800489D  bl 0x832429dc
	ctx.lr = 0x8323E144;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8323E144: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E148: 386B2500  addi r3, r11, 0x2500
	ctx.r[3].s64 = ctx.r[11].s64 + 9472;
	// 8323E14C: 4BF6A38D  bl 0x831a84d8
	ctx.lr = 0x8323E150;
	sub_831A84D8(ctx, base);
	// 8323E150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E160 size=52
    let mut pc: u32 = 0x8323E160;
    'dispatch: loop {
        match pc {
            0x8323E160 => {
    //   block [0x8323E160..0x8323E194)
	// 8323E160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E16C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E170: 386BDB6C  addi r3, r11, -0x2494
	ctx.r[3].s64 = ctx.r[11].s64 + -9364;
	// 8323E174: 4BEDAB45  bl 0x83118cb8
	ctx.lr = 0x8323E178;
	sub_83118CB8(ctx, base);
	// 8323E178: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E17C: 386B2510  addi r3, r11, 0x2510
	ctx.r[3].s64 = ctx.r[11].s64 + 9488;
	// 8323E180: 4BF6A359  bl 0x831a84d8
	ctx.lr = 0x8323E184;
	sub_831A84D8(ctx, base);
	// 8323E184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E198 size=84
    let mut pc: u32 = 0x8323E198;
    'dispatch: loop {
        match pc {
            0x8323E198 => {
    //   block [0x8323E198..0x8323E1EC)
	// 8323E198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E1A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323E1A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E1A8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E1AC: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 8323E1B0: 3BEBDB90  addi r31, r11, -0x2470
	ctx.r[31].s64 = ctx.r[11].s64 + -9328;
	// 8323E1B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323E1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323E1BC: 4BF6A025  bl 0x831a81e0
	ctx.lr = 0x8323E1C0;
	sub_831A81E0(ctx, base);
	// 8323E1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E1C4: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 8323E1C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8323E1CC: 917F0324  stw r11, 0x324(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(804 as u32), ctx.r[11].u32 ) };
	// 8323E1D0: 387F0194  addi r3, r31, 0x194
	ctx.r[3].s64 = ctx.r[31].s64 + 404;
	// 8323E1D4: 4BF6A00D  bl 0x831a81e0
	ctx.lr = 0x8323E1D8;
	sub_831A81E0(ctx, base);
	// 8323E1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E1E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323E1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E1F0 size=52
    let mut pc: u32 = 0x8323E1F0;
    'dispatch: loop {
        match pc {
            0x8323E1F0 => {
    //   block [0x8323E1F0..0x8323E224)
	// 8323E1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E1F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E1FC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E200: 386BDB30  addi r3, r11, -0x24d0
	ctx.r[3].s64 = ctx.r[11].s64 + -9424;
	// 8323E204: 480047D9  bl 0x832429dc
	ctx.lr = 0x8323E208;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8323E208: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E20C: 386B2520  addi r3, r11, 0x2520
	ctx.r[3].s64 = ctx.r[11].s64 + 9504;
	// 8323E210: 4BF6A2C9  bl 0x831a84d8
	ctx.lr = 0x8323E214;
	sub_831A84D8(ctx, base);
	// 8323E214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E228 size=12
    let mut pc: u32 = 0x8323E228;
    'dispatch: loop {
        match pc {
            0x8323E228 => {
    //   block [0x8323E228..0x8323E234)
	// 8323E228: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E22C: 386B2528  addi r3, r11, 0x2528
	ctx.r[3].s64 = ctx.r[11].s64 + 9512;
	// 8323E230: 4BF6A2A8  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8323E238 size=104
    let mut pc: u32 = 0x8323E238;
    'dispatch: loop {
        match pc {
            0x8323E238 => {
    //   block [0x8323E238..0x8323E2A0)
	// 8323E238: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8323E23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E240: 396B7F20  addi r11, r11, 0x7f20
	ctx.r[11].s64 = ctx.r[11].s64 + 32544;
	// 8323E244: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8323E248: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 8323E24C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8323E250: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8323E254: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8323E258: 4200FFF8  bdnz 0x8323e250
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8323E250; continue 'dispatch;
	}
	// 8323E25C: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8323E260: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 8323E264: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8323E268: 3CA08208  lis r5, -0x7df8
	ctx.r[5].s64 = -2113404928;
	// 8323E26C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8323E270: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8323E274: C008F524  lfs f0, -0xadc(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2780 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8323E278: C1A7093C  lfs f13, 0x93c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2364 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8323E27C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8323E280: C1866B3C  lfs f12, 0x6b3c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(27452 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8323E284: 912B0024  stw r9, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 8323E288: C1656154  lfs f11, 0x6154(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24916 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8323E28C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8323E290: D1AB0010  stfs f13, 0x10(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8323E294: D18B0008  stfs f12, 8(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8323E298: D16B0014  stfs f11, 0x14(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8323E29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E2A0 size=12
    let mut pc: u32 = 0x8323E2A0;
    'dispatch: loop {
        match pc {
            0x8323E2A0 => {
    //   block [0x8323E2A0..0x8323E2AC)
	// 8323E2A0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8323E2A4: 386B2548  addi r3, r11, 0x2548
	ctx.r[3].s64 = ctx.r[11].s64 + 9544;
	// 8323E2A8: 4BF6A230  b 0x831a84d8
	sub_831A84D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E2B0 size=56
    let mut pc: u32 = 0x8323E2B0;
    'dispatch: loop {
        match pc {
            0x8323E2B0 => {
    //   block [0x8323E2B0..0x8323E2E8)
	// 8323E2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E2BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323E2C0: 396B0C24  addi r11, r11, 0xc24
	ctx.r[11].s64 = ctx.r[11].s64 + 3108;
	// 8323E2C4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8323E2C8: 48004715  bl 0x832429dc
	ctx.lr = 0x8323E2CC;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8323E2CC: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 8323E2D0: 386A2560  addi r3, r10, 0x2560
	ctx.r[3].s64 = ctx.r[10].s64 + 9568;
	// 8323E2D4: 4BF6A205  bl 0x831a84d8
	ctx.lr = 0x8323E2D8;
	sub_831A84D8(ctx, base);
	// 8323E2D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E2E8 size=56
    let mut pc: u32 = 0x8323E2E8;
    'dispatch: loop {
        match pc {
            0x8323E2E8 => {
    //   block [0x8323E2E8..0x8323E320)
	// 8323E2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E2F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E2F4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323E2F8: 396B0C44  addi r11, r11, 0xc44
	ctx.r[11].s64 = ctx.r[11].s64 + 3140;
	// 8323E2FC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8323E300: 480046DD  bl 0x832429dc
	ctx.lr = 0x8323E304;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8323E304: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 8323E308: 386A2578  addi r3, r10, 0x2578
	ctx.r[3].s64 = ctx.r[10].s64 + 9592;
	// 8323E30C: 4BF6A1CD  bl 0x831a84d8
	ctx.lr = 0x8323E310;
	sub_831A84D8(ctx, base);
	// 8323E310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323E314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E320 size=32
    let mut pc: u32 = 0x8323E320;
    'dispatch: loop {
        match pc {
            0x8323E320 => {
    //   block [0x8323E320..0x8323E340)
	// 8323E320: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323E324: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 8323E328: 38E80D28  addi r7, r8, 0xd28
	ctx.r[7].s64 = ctx.r[8].s64 + 3368;
	// 8323E32C: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 8323E330: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8323E334: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 8323E338: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8323E33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E340 size=32
    let mut pc: u32 = 0x8323E340;
    'dispatch: loop {
        match pc {
            0x8323E340 => {
    //   block [0x8323E340..0x8323E360)
	// 8323E340: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323E344: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 8323E348: 38E80D8C  addi r7, r8, 0xd8c
	ctx.r[7].s64 = ctx.r[8].s64 + 3468;
	// 8323E34C: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 8323E350: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8323E354: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 8323E358: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8323E35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E360 size=108
    let mut pc: u32 = 0x8323E360;
    'dispatch: loop {
        match pc {
            0x8323E360 => {
    //   block [0x8323E360..0x8323E3CC)
	// 8323E360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E36C: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E370: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E374: 38EA2B48  addi r7, r10, 0x2b48
	ctx.r[7].s64 = ctx.r[10].s64 + 11080;
	// 8323E378: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E37C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8323E380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E384: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E388: 3889FC14  addi r4, r9, -0x3ec
	ctx.r[4].s64 = ctx.r[9].s64 + -1004;
	// 8323E38C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323E390: 3868D5E8  addi r3, r8, -0x2a18
	ctx.r[3].s64 = ctx.r[8].s64 + -10776;
	// 8323E394: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E39C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E3A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323E3A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E3A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E3B0: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323E3B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E3B8: 4BC69E11  bl 0x82ea81c8
	ctx.lr = 0x8323E3BC;
	sub_82EA81C8(ctx, base);
	// 8323E3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E3D0 size=108
    let mut pc: u32 = 0x8323E3D0;
    'dispatch: loop {
        match pc {
            0x8323E3D0 => {
    //   block [0x8323E3D0..0x8323E43C)
	// 8323E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E3DC: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E3E0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E3E4: 38EA2BF0  addi r7, r10, 0x2bf0
	ctx.r[7].s64 = ctx.r[10].s64 + 11248;
	// 8323E3E8: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E3EC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8323E3F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E3F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E3F8: 3889FC14  addi r4, r9, -0x3ec
	ctx.r[4].s64 = ctx.r[9].s64 + -1004;
	// 8323E3FC: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323E400: 3868D618  addi r3, r8, -0x29e8
	ctx.r[3].s64 = ctx.r[8].s64 + -10728;
	// 8323E404: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E40C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E410: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323E414: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E418: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E420: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8323E424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E428: 4BC69DA1  bl 0x82ea81c8
	ctx.lr = 0x8323E42C;
	sub_82EA81C8(ctx, base);
	// 8323E42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E440 size=296
    let mut pc: u32 = 0x8323E440;
    'dispatch: loop {
        match pc {
            0x8323E440 => {
    //   block [0x8323E440..0x8323E568)
	// 8323E440: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 8323E444: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 8323E448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E44C: 39680DA8  addi r11, r8, 0xda8
	ctx.r[11].s64 = ctx.r[8].s64 + 3496;
	// 8323E450: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323E454: 81290D98  lwz r9, 0xd98(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3480 as u32) ) } as u64;
	// 8323E458: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 8323E45C: 3908FAC8  addi r8, r8, -0x538
	ctx.r[8].s64 = ctx.r[8].s64 + -1336;
	// 8323E460: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323E464: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8323E468: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8323E46C: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8323E470: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323E474: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 8323E478: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E47C: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 8323E480: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8323E484: 98EB006C  stb r7, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[7].u8 ) };
	// 8323E488: 3929FABC  addi r9, r9, -0x544
	ctx.r[9].s64 = ctx.r[9].s64 + -1348;
	// 8323E48C: B10B0070  sth r8, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[8].u16 ) };
	// 8323E490: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8323E494: 98CB006D  stb r6, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[6].u8 ) };
	// 8323E498: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8323E49C: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8323E4A0: B0AB006E  sth r5, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[5].u16 ) };
	// 8323E4A4: B08B0072  sth r4, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[4].u16 ) };
	// 8323E4A8: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8323E4AC: 912B0078  stw r9, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 8323E4B0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8323E4B4: 910B0080  stw r8, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 8323E4B8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8323E4BC: 98EB0084  stb r7, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[7].u8 ) };
	// 8323E4C0: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 8323E4C4: B12B0086  sth r9, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[9].u16 ) };
	// 8323E4C8: B10B008A  sth r8, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[8].u16 ) };
	// 8323E4CC: 3927F49C  addi r9, r7, -0xb64
	ctx.r[9].s64 = ctx.r[7].s64 + -2916;
	// 8323E4D0: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8323E4D4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8323E4D8: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 8323E4DC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8323E4E0: 98CB0085  stb r6, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[6].u8 ) };
	// 8323E4E4: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 8323E4E8: 912B0090  stw r9, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 8323E4EC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8323E4F0: 98EB009C  stb r7, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[7].u8 ) };
	// 8323E4F4: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8323E4F8: 992B009D  stb r9, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[9].u8 ) };
	// 8323E4FC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E500: 910B0098  stw r8, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[8].u32 ) };
	// 8323E504: B10B00A0  sth r8, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[8].u16 ) };
	// 8323E508: 3909FAB4  addi r8, r9, -0x54c
	ctx.r[8].s64 = ctx.r[9].s64 + -1356;
	// 8323E50C: B0EB00A2  sth r7, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[7].u16 ) };
	// 8323E510: 392B005C  addi r9, r11, 0x5c
	ctx.r[9].s64 = ctx.r[11].s64 + 92;
	// 8323E514: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 8323E518: 392B0074  addi r9, r11, 0x74
	ctx.r[9].s64 = ctx.r[11].s64 + 116;
	// 8323E51C: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 8323E520: 392B008C  addi r9, r11, 0x8c
	ctx.r[9].s64 = ctx.r[11].s64 + 140;
	// 8323E524: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 8323E528: 910B00A8  stw r8, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[8].u32 ) };
	// 8323E52C: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8323E530: 392B00A4  addi r9, r11, 0xa4
	ctx.r[9].s64 = ctx.r[11].s64 + 164;
	// 8323E534: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 8323E538: 990B00B4  stb r8, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[8].u8 ) };
	// 8323E53C: 392B00BC  addi r9, r11, 0xbc
	ctx.r[9].s64 = ctx.r[11].s64 + 188;
	// 8323E540: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8323E544: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 8323E548: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8323E54C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8323E550: 912B00B0  stw r9, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[9].u32 ) };
	// 8323E554: B0EB00BA  sth r7, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[7].u16 ) };
	// 8323E558: B12B00B6  sth r9, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[9].u16 ) };
	// 8323E55C: B10B00B8  sth r8, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[8].u16 ) };
	// 8323E560: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 8323E564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E568 size=112
    let mut pc: u32 = 0x8323E568;
    'dispatch: loop {
        match pc {
            0x8323E568 => {
    //   block [0x8323E568..0x8323E5D8)
	// 8323E568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E574: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8323E578: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323E57C: 38CA0DA8  addi r6, r10, 0xda8
	ctx.r[6].s64 = ctx.r[10].s64 + 3496;
	// 8323E580: 3CE08343  lis r7, -0x7cbd
	ctx.r[7].s64 = -2092761088;
	// 8323E584: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8323E588: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323E58C: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8323E590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E594: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 8323E598: 3888FAA4  addi r4, r8, -0x55c
	ctx.r[4].s64 = ctx.r[8].s64 + -1372;
	// 8323E59C: 3867D648  addi r3, r7, -0x29b8
	ctx.r[3].s64 = ctx.r[7].s64 + -10680;
	// 8323E5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E5A4: 39292E08  addi r9, r9, 0x2e08
	ctx.r[9].s64 = ctx.r[9].s64 + 11784;
	// 8323E5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E5AC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8323E5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E5B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E5BC: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323E5C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E5C4: 4BC69C05  bl 0x82ea81c8
	ctx.lr = 0x8323E5C8;
	sub_82EA81C8(ctx, base);
	// 8323E5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E5D8 size=108
    let mut pc: u32 = 0x8323E5D8;
    'dispatch: loop {
        match pc {
            0x8323E5D8 => {
    //   block [0x8323E5D8..0x8323E644)
	// 8323E5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E5E4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E5E8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E5EC: 38EA2E44  addi r7, r10, 0x2e44
	ctx.r[7].s64 = ctx.r[10].s64 + 11844;
	// 8323E5F0: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E5F4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323E5F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E5FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E600: 3889FB24  addi r4, r9, -0x4dc
	ctx.r[4].s64 = ctx.r[9].s64 + -1244;
	// 8323E604: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323E608: 3868D678  addi r3, r8, -0x2988
	ctx.r[3].s64 = ctx.r[8].s64 + -10632;
	// 8323E60C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E614: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E618: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323E61C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E628: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323E62C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E630: 4BC69B99  bl 0x82ea81c8
	ctx.lr = 0x8323E634;
	sub_82EA81C8(ctx, base);
	// 8323E634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E63C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E648 size=108
    let mut pc: u32 = 0x8323E648;
    'dispatch: loop {
        match pc {
            0x8323E648 => {
    //   block [0x8323E648..0x8323E6B4)
	// 8323E648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E654: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E658: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E65C: 38EA2E74  addi r7, r10, 0x2e74
	ctx.r[7].s64 = ctx.r[10].s64 + 11892;
	// 8323E660: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E664: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323E668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E66C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E670: 3889FB34  addi r4, r9, -0x4cc
	ctx.r[4].s64 = ctx.r[9].s64 + -1228;
	// 8323E674: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323E678: 3868D6A8  addi r3, r8, -0x2958
	ctx.r[3].s64 = ctx.r[8].s64 + -10584;
	// 8323E67C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E684: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E688: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323E68C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E690: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E698: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8323E69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E6A0: 4BC69B29  bl 0x82ea81c8
	ctx.lr = 0x8323E6A4;
	sub_82EA81C8(ctx, base);
	// 8323E6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E6B8 size=108
    let mut pc: u32 = 0x8323E6B8;
    'dispatch: loop {
        match pc {
            0x8323E6B8 => {
    //   block [0x8323E6B8..0x8323E724)
	// 8323E6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E6C4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E6C8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E6CC: 38EA2EB8  addi r7, r10, 0x2eb8
	ctx.r[7].s64 = ctx.r[10].s64 + 11960;
	// 8323E6D0: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E6D4: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8323E6D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E6DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E6E0: 3889FC14  addi r4, r9, -0x3ec
	ctx.r[4].s64 = ctx.r[9].s64 + -1004;
	// 8323E6E4: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323E6E8: 3868D6D8  addi r3, r8, -0x2928
	ctx.r[3].s64 = ctx.r[8].s64 + -10536;
	// 8323E6EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E6F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E6F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323E6FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E700: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E708: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323E70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E710: 4BC69AB9  bl 0x82ea81c8
	ctx.lr = 0x8323E714;
	sub_82EA81C8(ctx, base);
	// 8323E714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E728 size=108
    let mut pc: u32 = 0x8323E728;
    'dispatch: loop {
        match pc {
            0x8323E728 => {
    //   block [0x8323E728..0x8323E794)
	// 8323E728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E734: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E738: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E73C: 38EA2FA8  addi r7, r10, 0x2fa8
	ctx.r[7].s64 = ctx.r[10].s64 + 12200;
	// 8323E740: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E744: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8323E748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E74C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E750: 3889FC14  addi r4, r9, -0x3ec
	ctx.r[4].s64 = ctx.r[9].s64 + -1004;
	// 8323E754: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323E758: 3868D708  addi r3, r8, -0x28f8
	ctx.r[3].s64 = ctx.r[8].s64 + -10488;
	// 8323E75C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E764: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E768: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323E76C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E770: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E778: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8323E77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E780: 4BC69A49  bl 0x82ea81c8
	ctx.lr = 0x8323E784;
	sub_82EA81C8(ctx, base);
	// 8323E784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323E798 size=352
    let mut pc: u32 = 0x8323E798;
    'dispatch: loop {
        match pc {
            0x8323E798 => {
    //   block [0x8323E798..0x8323E8F8)
	// 8323E798: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 8323E79C: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 8323E7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E7A4: 39680E78  addi r11, r8, 0xe78
	ctx.r[11].s64 = ctx.r[8].s64 + 3704;
	// 8323E7A8: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323E7AC: 81290E68  lwz r9, 0xe68(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3688 as u32) ) } as u64;
	// 8323E7B0: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 8323E7B4: 3908FAC8  addi r8, r8, -0x538
	ctx.r[8].s64 = ctx.r[8].s64 + -1336;
	// 8323E7B8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8323E7BC: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8323E7C0: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8323E7C4: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8323E7C8: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8323E7CC: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 8323E7D0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E7D4: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 8323E7D8: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8323E7DC: 3929FABC  addi r9, r9, -0x544
	ctx.r[9].s64 = ctx.r[9].s64 + -1348;
	// 8323E7E0: 98EB006C  stb r7, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[7].u8 ) };
	// 8323E7E4: B10B0070  sth r8, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[8].u16 ) };
	// 8323E7E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8323E7EC: 98CB006D  stb r6, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[6].u8 ) };
	// 8323E7F0: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8323E7F4: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8323E7F8: B0AB006E  sth r5, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[5].u16 ) };
	// 8323E7FC: B08B0072  sth r4, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[4].u16 ) };
	// 8323E800: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8323E804: 912B0078  stw r9, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 8323E808: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8323E80C: 910B0080  stw r8, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 8323E810: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8323E814: 98EB0084  stb r7, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[7].u8 ) };
	// 8323E818: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 8323E81C: B12B0086  sth r9, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[9].u16 ) };
	// 8323E820: B10B008A  sth r8, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[8].u16 ) };
	// 8323E824: 3927F49C  addi r9, r7, -0xb64
	ctx.r[9].s64 = ctx.r[7].s64 + -2916;
	// 8323E828: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8323E82C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8323E830: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 8323E834: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8323E838: 98CB0085  stb r6, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[6].u8 ) };
	// 8323E83C: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 8323E840: 912B0090  stw r9, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 8323E844: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8323E848: 98EB009C  stb r7, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[7].u8 ) };
	// 8323E84C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8323E850: 992B009D  stb r9, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[9].u8 ) };
	// 8323E854: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E858: 910B0098  stw r8, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[8].u32 ) };
	// 8323E85C: B10B00A0  sth r8, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[8].u16 ) };
	// 8323E860: 3909FAB4  addi r8, r9, -0x54c
	ctx.r[8].s64 = ctx.r[9].s64 + -1356;
	// 8323E864: B0EB00A2  sth r7, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[7].u16 ) };
	// 8323E868: 392B005C  addi r9, r11, 0x5c
	ctx.r[9].s64 = ctx.r[11].s64 + 92;
	// 8323E86C: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 8323E870: 392B0074  addi r9, r11, 0x74
	ctx.r[9].s64 = ctx.r[11].s64 + 116;
	// 8323E874: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 8323E878: 392B008C  addi r9, r11, 0x8c
	ctx.r[9].s64 = ctx.r[11].s64 + 140;
	// 8323E87C: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 8323E880: 910B00A8  stw r8, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[8].u32 ) };
	// 8323E884: 392B00A4  addi r9, r11, 0xa4
	ctx.r[9].s64 = ctx.r[11].s64 + 164;
	// 8323E888: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8323E88C: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 8323E890: 392B00BC  addi r9, r11, 0xbc
	ctx.r[9].s64 = ctx.r[11].s64 + 188;
	// 8323E894: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 8323E898: 392B00D4  addi r9, r11, 0xd4
	ctx.r[9].s64 = ctx.r[11].s64 + 212;
	// 8323E89C: 990B00B4  stb r8, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[8].u8 ) };
	// 8323E8A0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8323E8A4: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8323E8A8: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8323E8AC: 912B00B0  stw r9, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[9].u32 ) };
	// 8323E8B0: B12B00B6  sth r9, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[9].u16 ) };
	// 8323E8B4: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8323E8B8: B10B00B8  sth r8, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[8].u16 ) };
	// 8323E8BC: B0EB00BA  sth r7, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[7].u16 ) };
	// 8323E8C0: 39292EA4  addi r9, r9, 0x2ea4
	ctx.r[9].s64 = ctx.r[9].s64 + 11940;
	// 8323E8C4: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 8323E8C8: 912B00C0  stw r9, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[9].u32 ) };
	// 8323E8CC: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 8323E8D0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8323E8D4: 914B00C4  stw r10, 0xc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8323E8D8: 910B00C8  stw r8, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[8].u32 ) };
	// 8323E8DC: 98EB00CC  stb r7, 0xcc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(204 as u32), ctx.r[7].u8 ) };
	// 8323E8E0: 994B00CD  stb r10, 0xcd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(205 as u32), ctx.r[10].u8 ) };
	// 8323E8E4: B0EB00D2  sth r7, 0xd2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(210 as u32), ctx.r[7].u16 ) };
	// 8323E8E8: B12B00CE  sth r9, 0xce(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(206 as u32), ctx.r[9].u16 ) };
	// 8323E8EC: B10B00D0  sth r8, 0xd0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[8].u16 ) };
	// 8323E8F0: 914B00D4  stw r10, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 8323E8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E8F8 size=112
    let mut pc: u32 = 0x8323E8F8;
    'dispatch: loop {
        match pc {
            0x8323E8F8 => {
    //   block [0x8323E8F8..0x8323E968)
	// 8323E8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E904: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8323E908: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 8323E90C: 38CA0E78  addi r6, r10, 0xe78
	ctx.r[6].s64 = ctx.r[10].s64 + 3704;
	// 8323E910: 3CE08343  lis r7, -0x7cbd
	ctx.r[7].s64 = -2092761088;
	// 8323E914: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 8323E918: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8323E91C: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8323E920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E924: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 8323E928: 3888FAA4  addi r4, r8, -0x55c
	ctx.r[4].s64 = ctx.r[8].s64 + -1372;
	// 8323E92C: 3867D738  addi r3, r7, -0x28c8
	ctx.r[3].s64 = ctx.r[7].s64 + -10440;
	// 8323E930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E934: 39293208  addi r9, r9, 0x3208
	ctx.r[9].s64 = ctx.r[9].s64 + 12808;
	// 8323E938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E93C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8323E940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E948: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E94C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8323E950: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E954: 4BC69875  bl 0x82ea81c8
	ctx.lr = 0x8323E958;
	sub_82EA81C8(ctx, base);
	// 8323E958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E95C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E968 size=108
    let mut pc: u32 = 0x8323E968;
    'dispatch: loop {
        match pc {
            0x8323E968 => {
    //   block [0x8323E968..0x8323E9D4)
	// 8323E968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E974: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E978: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E97C: 38EA3244  addi r7, r10, 0x3244
	ctx.r[7].s64 = ctx.r[10].s64 + 12868;
	// 8323E980: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E984: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8323E988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E98C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323E990: 3889FB24  addi r4, r9, -0x4dc
	ctx.r[4].s64 = ctx.r[9].s64 + -1244;
	// 8323E994: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323E998: 3868D768  addi r3, r8, -0x2898
	ctx.r[3].s64 = ctx.r[8].s64 + -10392;
	// 8323E99C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323E9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323E9A4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323E9A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323E9AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323E9B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323E9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323E9B8: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8323E9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323E9C0: 4BC69809  bl 0x82ea81c8
	ctx.lr = 0x8323E9C4;
	sub_82EA81C8(ctx, base);
	// 8323E9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323E9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323E9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323E9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323E9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323E9D8 size=108
    let mut pc: u32 = 0x8323E9D8;
    'dispatch: loop {
        match pc {
            0x8323E9D8 => {
    //   block [0x8323E9D8..0x8323EA44)
	// 8323E9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323E9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323E9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323E9E4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8323E9E8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 8323E9EC: 38EA3278  addi r7, r10, 0x3278
	ctx.r[7].s64 = ctx.r[10].s64 + 12920;
	// 8323E9F0: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 8323E9F4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8323E9F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8323E9FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323EA00: 3889FB34  addi r4, r9, -0x4cc
	ctx.r[4].s64 = ctx.r[9].s64 + -1228;
	// 8323EA04: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8323EA08: 3868D798  addi r3, r8, -0x2868
	ctx.r[3].s64 = ctx.r[8].s64 + -10344;
	// 8323EA0C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8323EA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8323EA14: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8323EA18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8323EA1C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8323EA20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8323EA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8323EA28: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8323EA2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323EA30: 4BC69799  bl 0x82ea81c8
	ctx.lr = 0x8323EA34;
	sub_82EA81C8(ctx, base);
	// 8323EA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8323EA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EA48 size=32
    let mut pc: u32 = 0x8323EA48;
    'dispatch: loop {
        match pc {
            0x8323EA48 => {
    //   block [0x8323EA48..0x8323EA68)
	// 8323EA48: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8323EA4C: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 8323EA50: 38E8100C  addi r7, r8, 0x100c
	ctx.r[7].s64 = ctx.r[8].s64 + 4108;
	// 8323EA54: 816970E8  lwz r11, 0x70e8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28904 as u32) ) } as u64;
	// 8323EA58: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8323EA5C: 914970E8  stw r10, 0x70e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28904 as u32), ctx.r[10].u32 ) };
	// 8323EA60: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8323EA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EA68 size=12
    let mut pc: u32 = 0x8323EA68;
    'dispatch: loop {
        match pc {
            0x8323EA68 => {
    //   block [0x8323EA68..0x8323EA74)
	// 8323EA68: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EA6C: 386B1110  addi r3, r11, 0x1110
	ctx.r[3].s64 = ctx.r[11].s64 + 4368;
	// 8323EA70: 4BBB3240  b 0x82df1cb0
	sub_82DF1CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EA78 size=12
    let mut pc: u32 = 0x8323EA78;
    'dispatch: loop {
        match pc {
            0x8323EA78 => {
    //   block [0x8323EA78..0x8323EA84)
	// 8323EA78: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EA7C: 386B14B4  addi r3, r11, 0x14b4
	ctx.r[3].s64 = ctx.r[11].s64 + 5300;
	// 8323EA80: 4B082C78  b 0x822c16f8
	sub_822C16F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EA88 size=12
    let mut pc: u32 = 0x8323EA88;
    'dispatch: loop {
        match pc {
            0x8323EA88 => {
    //   block [0x8323EA88..0x8323EA94)
	// 8323EA88: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EA8C: 386B155C  addi r3, r11, 0x155c
	ctx.r[3].s64 = ctx.r[11].s64 + 5468;
	// 8323EA90: 4BBB3220  b 0x82df1cb0
	sub_82DF1CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EA98 size=12
    let mut pc: u32 = 0x8323EA98;
    'dispatch: loop {
        match pc {
            0x8323EA98 => {
    //   block [0x8323EA98..0x8323EAA4)
	// 8323EA98: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EA9C: 386B12B0  addi r3, r11, 0x12b0
	ctx.r[3].s64 = ctx.r[11].s64 + 4784;
	// 8323EAA0: 4BC61030  b 0x82e9fad0
	sub_82E9FAD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EAA8 size=12
    let mut pc: u32 = 0x8323EAA8;
    'dispatch: loop {
        match pc {
            0x8323EAA8 => {
    //   block [0x8323EAA8..0x8323EAB4)
	// 8323EAA8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EAAC: 386B1270  addi r3, r11, 0x1270
	ctx.r[3].s64 = ctx.r[11].s64 + 4720;
	// 8323EAB0: 4BC60C28  b 0x82e9f6d8
	sub_82E9F6D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EAB8 size=12
    let mut pc: u32 = 0x8323EAB8;
    'dispatch: loop {
        match pc {
            0x8323EAB8 => {
    //   block [0x8323EAB8..0x8323EAC4)
	// 8323EAB8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EABC: 386B1208  addi r3, r11, 0x1208
	ctx.r[3].s64 = ctx.r[11].s64 + 4616;
	// 8323EAC0: 4BC60980  b 0x82e9f440
	sub_82E9F440(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EAC8 size=12
    let mut pc: u32 = 0x8323EAC8;
    'dispatch: loop {
        match pc {
            0x8323EAC8 => {
    //   block [0x8323EAC8..0x8323EAD4)
	// 8323EAC8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EACC: 386B139C  addi r3, r11, 0x139c
	ctx.r[3].s64 = ctx.r[11].s64 + 5020;
	// 8323EAD0: 4BBB31E0  b 0x82df1cb0
	sub_82DF1CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EAD8 size=20
    let mut pc: u32 = 0x8323EAD8;
    'dispatch: loop {
        match pc {
            0x8323EAD8 => {
    //   block [0x8323EAD8..0x8323EAEC)
	// 8323EAD8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EADC: 396B14A8  addi r11, r11, 0x14a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5288;
	// 8323EAE0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EAE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EAE8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EAEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EAEC size=8
    let mut pc: u32 = 0x8323EAEC;
    'dispatch: loop {
        match pc {
            0x8323EAEC => {
    //   block [0x8323EAEC..0x8323EAF4)
	// 8323EAEC: 4B081DA4  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EAF8 size=68
    let mut pc: u32 = 0x8323EAF8;
    'dispatch: loop {
        match pc {
            0x8323EAF8 => {
    //   block [0x8323EAF8..0x8323EB3C)
	// 8323EAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EB00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EB04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EB08: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EB0C: 3BEB1694  addi r31, r11, 0x1694
	ctx.r[31].s64 = ctx.r[11].s64 + 5780;
	// 8323EB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EB14: 4B08D4E5  bl 0x822cbff8
	ctx.lr = 0x8323EB18;
	sub_822CBFF8(ctx, base);
	// 8323EB18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EB1C: 4B08174D  bl 0x822c0268
	ctx.lr = 0x8323EB20;
	sub_822C0268(ctx, base);
	// 8323EB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8323EB24: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8323EB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323EB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EB34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EB40 size=20
    let mut pc: u32 = 0x8323EB40;
    'dispatch: loop {
        match pc {
            0x8323EB40 => {
    //   block [0x8323EB40..0x8323EB54)
	// 8323EB40: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EB44: 396B16C4  addi r11, r11, 0x16c4
	ctx.r[11].s64 = ctx.r[11].s64 + 5828;
	// 8323EB48: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EB4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EB50: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EB54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EB54 size=8
    let mut pc: u32 = 0x8323EB54;
    'dispatch: loop {
        match pc {
            0x8323EB54 => {
    //   block [0x8323EB54..0x8323EB5C)
	// 8323EB54: 4B081D3C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EB60 size=20
    let mut pc: u32 = 0x8323EB60;
    'dispatch: loop {
        match pc {
            0x8323EB60 => {
    //   block [0x8323EB60..0x8323EB74)
	// 8323EB60: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EB64: 396B16CC  addi r11, r11, 0x16cc
	ctx.r[11].s64 = ctx.r[11].s64 + 5836;
	// 8323EB68: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EB6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EB70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EB74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EB74 size=8
    let mut pc: u32 = 0x8323EB74;
    'dispatch: loop {
        match pc {
            0x8323EB74 => {
    //   block [0x8323EB74..0x8323EB7C)
	// 8323EB74: 4B081D1C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EB80 size=76
    let mut pc: u32 = 0x8323EB80;
    'dispatch: loop {
        match pc {
            0x8323EB80 => {
    //   block [0x8323EB80..0x8323EBCC)
	// 8323EB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EB88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EB8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EB90: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EB94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323EB98: 3BEB16F8  addi r31, r11, 0x16f8
	ctx.r[31].s64 = ctx.r[11].s64 + 5880;
	// 8323EB9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8323EBA0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8323EBA4: 4B08613D  bl 0x822c4ce0
	ctx.lr = 0x8323EBA8;
	sub_822C4CE0(ctx, base);
	// 8323EBA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EBAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323EBB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8323EBB4: 4B08612D  bl 0x822c4ce0
	ctx.lr = 0x8323EBB8;
	sub_822C4CE0(ctx, base);
	// 8323EBB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323EBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EBC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EBD0 size=12
    let mut pc: u32 = 0x8323EBD0;
    'dispatch: loop {
        match pc {
            0x8323EBD0 => {
    //   block [0x8323EBD0..0x8323EBDC)
	// 8323EBD0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EBD4: 386B2610  addi r3, r11, 0x2610
	ctx.r[3].s64 = ctx.r[11].s64 + 9744;
	// 8323EBD8: 4BBB4850  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EBE0 size=12
    let mut pc: u32 = 0x8323EBE0;
    'dispatch: loop {
        match pc {
            0x8323EBE0 => {
    //   block [0x8323EBE0..0x8323EBEC)
	// 8323EBE0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EBE4: 386B2618  addi r3, r11, 0x2618
	ctx.r[3].s64 = ctx.r[11].s64 + 9752;
	// 8323EBE8: 4BBB4840  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EBF0 size=12
    let mut pc: u32 = 0x8323EBF0;
    'dispatch: loop {
        match pc {
            0x8323EBF0 => {
    //   block [0x8323EBF0..0x8323EBFC)
	// 8323EBF0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EBF4: 386B2638  addi r3, r11, 0x2638
	ctx.r[3].s64 = ctx.r[11].s64 + 9784;
	// 8323EBF8: 4BBB4830  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EC00 size=20
    let mut pc: u32 = 0x8323EC00;
    'dispatch: loop {
        match pc {
            0x8323EC00 => {
    //   block [0x8323EC00..0x8323EC14)
	// 8323EC00: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EC04: 396B2670  addi r11, r11, 0x2670
	ctx.r[11].s64 = ctx.r[11].s64 + 9840;
	// 8323EC08: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EC0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EC10: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EC14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EC14 size=8
    let mut pc: u32 = 0x8323EC14;
    'dispatch: loop {
        match pc {
            0x8323EC14 => {
    //   block [0x8323EC14..0x8323EC1C)
	// 8323EC14: 4B081C7C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EC20 size=60
    let mut pc: u32 = 0x8323EC20;
    'dispatch: loop {
        match pc {
            0x8323EC20 => {
    //   block [0x8323EC20..0x8323EC5C)
	// 8323EC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EC28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EC2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EC30: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EC34: 3BEB2650  addi r31, r11, 0x2650
	ctx.r[31].s64 = ctx.r[11].s64 + 9808;
	// 8323EC38: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8323EC3C: 4B22B3C5  bl 0x8246a000
	ctx.lr = 0x8323EC40;
	sub_8246A000(ctx, base);
	// 8323EC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EC44: 4B89E8AD  bl 0x82add4f0
	ctx.lr = 0x8323EC48;
	sub_82ADD4F0(ctx, base);
	// 8323EC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323EC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EC54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EC60 size=20
    let mut pc: u32 = 0x8323EC60;
    'dispatch: loop {
        match pc {
            0x8323EC60 => {
    //   block [0x8323EC60..0x8323EC74)
	// 8323EC60: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EC64: 396B26C4  addi r11, r11, 0x26c4
	ctx.r[11].s64 = ctx.r[11].s64 + 9924;
	// 8323EC68: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EC6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EC70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EC74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EC74 size=8
    let mut pc: u32 = 0x8323EC74;
    'dispatch: loop {
        match pc {
            0x8323EC74 => {
    //   block [0x8323EC74..0x8323EC7C)
	// 8323EC74: 4B081C1C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EC80 size=20
    let mut pc: u32 = 0x8323EC80;
    'dispatch: loop {
        match pc {
            0x8323EC80 => {
    //   block [0x8323EC80..0x8323EC94)
	// 8323EC80: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EC84: 396B26CC  addi r11, r11, 0x26cc
	ctx.r[11].s64 = ctx.r[11].s64 + 9932;
	// 8323EC88: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EC8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EC90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EC94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EC94 size=8
    let mut pc: u32 = 0x8323EC94;
    'dispatch: loop {
        match pc {
            0x8323EC94 => {
    //   block [0x8323EC94..0x8323EC9C)
	// 8323EC94: 4B081BFC  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ECA0 size=20
    let mut pc: u32 = 0x8323ECA0;
    'dispatch: loop {
        match pc {
            0x8323ECA0 => {
    //   block [0x8323ECA0..0x8323ECB4)
	// 8323ECA0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323ECA4: 396B26D4  addi r11, r11, 0x26d4
	ctx.r[11].s64 = ctx.r[11].s64 + 9940;
	// 8323ECA8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323ECAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323ECB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ECB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ECB4 size=8
    let mut pc: u32 = 0x8323ECB4;
    'dispatch: loop {
        match pc {
            0x8323ECB4 => {
    //   block [0x8323ECB4..0x8323ECBC)
	// 8323ECB4: 4B081BDC  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323ECB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ECC0 size=20
    let mut pc: u32 = 0x8323ECC0;
    'dispatch: loop {
        match pc {
            0x8323ECC0 => {
    //   block [0x8323ECC0..0x8323ECD4)
	// 8323ECC0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323ECC4: 396B26DC  addi r11, r11, 0x26dc
	ctx.r[11].s64 = ctx.r[11].s64 + 9948;
	// 8323ECC8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323ECCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323ECD0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ECD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ECD4 size=8
    let mut pc: u32 = 0x8323ECD4;
    'dispatch: loop {
        match pc {
            0x8323ECD4 => {
    //   block [0x8323ECD4..0x8323ECDC)
	// 8323ECD4: 4B081BBC  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323ECD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ECE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ECE0 size=20
    let mut pc: u32 = 0x8323ECE0;
    'dispatch: loop {
        match pc {
            0x8323ECE0 => {
    //   block [0x8323ECE0..0x8323ECF4)
	// 8323ECE0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323ECE4: 396B26E4  addi r11, r11, 0x26e4
	ctx.r[11].s64 = ctx.r[11].s64 + 9956;
	// 8323ECE8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323ECEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323ECF0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ECF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ECF4 size=8
    let mut pc: u32 = 0x8323ECF4;
    'dispatch: loop {
        match pc {
            0x8323ECF4 => {
    //   block [0x8323ECF4..0x8323ECFC)
	// 8323ECF4: 4B081B9C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323ECF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ED00 size=20
    let mut pc: u32 = 0x8323ED00;
    'dispatch: loop {
        match pc {
            0x8323ED00 => {
    //   block [0x8323ED00..0x8323ED14)
	// 8323ED00: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323ED04: 396B26EC  addi r11, r11, 0x26ec
	ctx.r[11].s64 = ctx.r[11].s64 + 9964;
	// 8323ED08: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323ED0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323ED10: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ED14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ED14 size=8
    let mut pc: u32 = 0x8323ED14;
    'dispatch: loop {
        match pc {
            0x8323ED14 => {
    //   block [0x8323ED14..0x8323ED1C)
	// 8323ED14: 4B081B7C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323ED18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ED20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ED20 size=20
    let mut pc: u32 = 0x8323ED20;
    'dispatch: loop {
        match pc {
            0x8323ED20 => {
    //   block [0x8323ED20..0x8323ED34)
	// 8323ED20: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323ED24: 396B26F4  addi r11, r11, 0x26f4
	ctx.r[11].s64 = ctx.r[11].s64 + 9972;
	// 8323ED28: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323ED2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323ED30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ED34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ED34 size=8
    let mut pc: u32 = 0x8323ED34;
    'dispatch: loop {
        match pc {
            0x8323ED34 => {
    //   block [0x8323ED34..0x8323ED3C)
	// 8323ED34: 4B081B5C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323ED38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ED40 size=20
    let mut pc: u32 = 0x8323ED40;
    'dispatch: loop {
        match pc {
            0x8323ED40 => {
    //   block [0x8323ED40..0x8323ED54)
	// 8323ED40: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323ED44: 396B26BC  addi r11, r11, 0x26bc
	ctx.r[11].s64 = ctx.r[11].s64 + 9916;
	// 8323ED48: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323ED4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323ED50: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ED54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323ED54 size=8
    let mut pc: u32 = 0x8323ED54;
    'dispatch: loop {
        match pc {
            0x8323ED54 => {
    //   block [0x8323ED54..0x8323ED5C)
	// 8323ED54: 4B081B3C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323ED58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323ED60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323ED60 size=80
    let mut pc: u32 = 0x8323ED60;
    'dispatch: loop {
        match pc {
            0x8323ED60 => {
    //   block [0x8323ED60..0x8323EDB0)
	// 8323ED60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323ED64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323ED68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323ED6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323ED70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323ED74: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323ED78: 3BC00011  li r30, 0x11
	ctx.r[30].s64 = 17;
	// 8323ED7C: 396B3578  addi r11, r11, 0x3578
	ctx.r[11].s64 = ctx.r[11].s64 + 13688;
	// 8323ED80: 3BEB02D8  addi r31, r11, 0x2d8
	ctx.r[31].s64 = ctx.r[11].s64 + 728;
	// 8323ED84: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323ED88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323ED8C: 4B089F2D  bl 0x822c8cb8
	ctx.lr = 0x8323ED90;
	sub_822C8CB8(ctx, base);
	// 8323ED90: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323ED94: 4080FFF0  bge 0x8323ed84
	if !ctx.cr[0].lt {
	pc = 0x8323ED84; continue 'dispatch;
	}
	// 8323ED98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323ED9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EDA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323EDA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EDB0 size=80
    let mut pc: u32 = 0x8323EDB0;
    'dispatch: loop {
        match pc {
            0x8323EDB0 => {
    //   block [0x8323EDB0..0x8323EE00)
	// 8323EDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EDB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323EDBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EDC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EDC4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EDC8: 3BC0000D  li r30, 0xd
	ctx.r[30].s64 = 13;
	// 8323EDCC: 396B3848  addi r11, r11, 0x3848
	ctx.r[11].s64 = ctx.r[11].s64 + 14408;
	// 8323EDD0: 3BEB0238  addi r31, r11, 0x238
	ctx.r[31].s64 = ctx.r[11].s64 + 568;
	// 8323EDD4: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323EDD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EDDC: 4B089EDD  bl 0x822c8cb8
	ctx.lr = 0x8323EDE0;
	sub_822C8CB8(ctx, base);
	// 8323EDE0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323EDE4: 4080FFF0  bge 0x8323edd4
	if !ctx.cr[0].lt {
	pc = 0x8323EDD4; continue 'dispatch;
	}
	// 8323EDE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323EDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EDF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323EDF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EE00 size=80
    let mut pc: u32 = 0x8323EE00;
    'dispatch: loop {
        match pc {
            0x8323EE00 => {
    //   block [0x8323EE00..0x8323EE50)
	// 8323EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EE08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323EE0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EE10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EE14: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EE18: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8323EE1C: 396B3A78  addi r11, r11, 0x3a78
	ctx.r[11].s64 = ctx.r[11].s64 + 14968;
	// 8323EE20: 3BEB00D0  addi r31, r11, 0xd0
	ctx.r[31].s64 = ctx.r[11].s64 + 208;
	// 8323EE24: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323EE28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EE2C: 4B089E8D  bl 0x822c8cb8
	ctx.lr = 0x8323EE30;
	sub_822C8CB8(ctx, base);
	// 8323EE30: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323EE34: 4080FFF0  bge 0x8323ee24
	if !ctx.cr[0].lt {
	pc = 0x8323EE24; continue 'dispatch;
	}
	// 8323EE38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323EE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EE44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323EE48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EE50 size=80
    let mut pc: u32 = 0x8323EE50;
    'dispatch: loop {
        match pc {
            0x8323EE50 => {
    //   block [0x8323EE50..0x8323EEA0)
	// 8323EE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EE58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323EE5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EE60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EE64: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EE68: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8323EE6C: 396B3B40  addi r11, r11, 0x3b40
	ctx.r[11].s64 = ctx.r[11].s64 + 15168;
	// 8323EE70: 3BEB00D0  addi r31, r11, 0xd0
	ctx.r[31].s64 = ctx.r[11].s64 + 208;
	// 8323EE74: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323EE78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EE7C: 4B089E3D  bl 0x822c8cb8
	ctx.lr = 0x8323EE80;
	sub_822C8CB8(ctx, base);
	// 8323EE80: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323EE84: 4080FFF0  bge 0x8323ee74
	if !ctx.cr[0].lt {
	pc = 0x8323EE74; continue 'dispatch;
	}
	// 8323EE88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323EE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EE94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323EE98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EEA0 size=80
    let mut pc: u32 = 0x8323EEA0;
    'dispatch: loop {
        match pc {
            0x8323EEA0 => {
    //   block [0x8323EEA0..0x8323EEF0)
	// 8323EEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EEA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323EEAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EEB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EEB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EEB8: 3BC0000D  li r30, 0xd
	ctx.r[30].s64 = 13;
	// 8323EEBC: 396B3C08  addi r11, r11, 0x3c08
	ctx.r[11].s64 = ctx.r[11].s64 + 15368;
	// 8323EEC0: 3BEB0238  addi r31, r11, 0x238
	ctx.r[31].s64 = ctx.r[11].s64 + 568;
	// 8323EEC4: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323EEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EECC: 4B089DED  bl 0x822c8cb8
	ctx.lr = 0x8323EED0;
	sub_822C8CB8(ctx, base);
	// 8323EED0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323EED4: 4080FFF0  bge 0x8323eec4
	if !ctx.cr[0].lt {
	pc = 0x8323EEC4; continue 'dispatch;
	}
	// 8323EED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323EEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EEE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323EEE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EEF0 size=80
    let mut pc: u32 = 0x8323EEF0;
    'dispatch: loop {
        match pc {
            0x8323EEF0 => {
    //   block [0x8323EEF0..0x8323EF40)
	// 8323EEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EEF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323EEFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EF00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EF04: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EF08: 3BC0004F  li r30, 0x4f
	ctx.r[30].s64 = 79;
	// 8323EF0C: 396B2768  addi r11, r11, 0x2768
	ctx.r[11].s64 = ctx.r[11].s64 + 10088;
	// 8323EF10: 3BEB0C88  addi r31, r11, 0xc88
	ctx.r[31].s64 = ctx.r[11].s64 + 3208;
	// 8323EF14: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323EF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EF1C: 4B089D9D  bl 0x822c8cb8
	ctx.lr = 0x8323EF20;
	sub_822C8CB8(ctx, base);
	// 8323EF20: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323EF24: 4080FFF0  bge 0x8323ef14
	if !ctx.cr[0].lt {
	pc = 0x8323EF14; continue 'dispatch;
	}
	// 8323EF28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323EF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EF34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323EF38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EF40 size=80
    let mut pc: u32 = 0x8323EF40;
    'dispatch: loop {
        match pc {
            0x8323EF40 => {
    //   block [0x8323EF40..0x8323EF90)
	// 8323EF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EF48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323EF4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EF50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EF54: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EF58: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 8323EF5C: 396B33E8  addi r11, r11, 0x33e8
	ctx.r[11].s64 = ctx.r[11].s64 + 13288;
	// 8323EF60: 3BEB0198  addi r31, r11, 0x198
	ctx.r[31].s64 = ctx.r[11].s64 + 408;
	// 8323EF64: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323EF68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323EF6C: 4B089D4D  bl 0x822c8cb8
	ctx.lr = 0x8323EF70;
	sub_822C8CB8(ctx, base);
	// 8323EF70: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323EF74: 4080FFF0  bge 0x8323ef64
	if !ctx.cr[0].lt {
	pc = 0x8323EF64; continue 'dispatch;
	}
	// 8323EF78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323EF84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323EF88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323EF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EF90 size=20
    let mut pc: u32 = 0x8323EF90;
    'dispatch: loop {
        match pc {
            0x8323EF90 => {
    //   block [0x8323EF90..0x8323EFA4)
	// 8323EF90: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EF94: 396B3E70  addi r11, r11, 0x3e70
	ctx.r[11].s64 = ctx.r[11].s64 + 15984;
	// 8323EF98: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EF9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EFA0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EFA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EFA4 size=8
    let mut pc: u32 = 0x8323EFA4;
    'dispatch: loop {
        match pc {
            0x8323EFA4 => {
    //   block [0x8323EFA4..0x8323EFAC)
	// 8323EFA4: 4B0818EC  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EFB0 size=20
    let mut pc: u32 = 0x8323EFB0;
    'dispatch: loop {
        match pc {
            0x8323EFB0 => {
    //   block [0x8323EFB0..0x8323EFC4)
	// 8323EFB0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EFB4: 396B3E78  addi r11, r11, 0x3e78
	ctx.r[11].s64 = ctx.r[11].s64 + 15992;
	// 8323EFB8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323EFBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323EFC0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EFC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323EFC4 size=8
    let mut pc: u32 = 0x8323EFC4;
    'dispatch: loop {
        match pc {
            0x8323EFC4 => {
    //   block [0x8323EFC4..0x8323EFCC)
	// 8323EFC4: 4B0818CC  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323EFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323EFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323EFD0 size=88
    let mut pc: u32 = 0x8323EFD0;
    'dispatch: loop {
        match pc {
            0x8323EFD0 => {
    //   block [0x8323EFD0..0x8323F028)
	// 8323EFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323EFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323EFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323EFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323EFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323EFE4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323EFE8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 8323EFEC: 396B3E80  addi r11, r11, 0x3e80
	ctx.r[11].s64 = ctx.r[11].s64 + 16000;
	// 8323EFF0: 3BEB0024  addi r31, r11, 0x24
	ctx.r[31].s64 = ctx.r[11].s64 + 36;
	// 8323EFF4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 8323EFF8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8323EFFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F000: 419A0008  beq cr6, 0x8323f008
	if ctx.cr[6].eq {
	pc = 0x8323F008; continue 'dispatch;
	}
	// 8323F004: 4B08188D  bl 0x822c0890
	ctx.lr = 0x8323F008;
	sub_822C0890(ctx, base);
	// 8323F008: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F00C: 4080FFE8  bge 0x8323eff4
	if !ctx.cr[0].lt {
	pc = 0x8323EFF4; continue 'dispatch;
	}
	// 8323F010: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F01C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F028 size=20
    let mut pc: u32 = 0x8323F028;
    'dispatch: loop {
        match pc {
            0x8323F028 => {
    //   block [0x8323F028..0x8323F03C)
	// 8323F028: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F02C: 396B3E48  addi r11, r11, 0x3e48
	ctx.r[11].s64 = ctx.r[11].s64 + 15944;
	// 8323F030: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F034: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F038: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F03C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F03C size=8
    let mut pc: u32 = 0x8323F03C;
    'dispatch: loop {
        match pc {
            0x8323F03C => {
    //   block [0x8323F03C..0x8323F044)
	// 8323F03C: 4B081854  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F048 size=20
    let mut pc: u32 = 0x8323F048;
    'dispatch: loop {
        match pc {
            0x8323F048 => {
    //   block [0x8323F048..0x8323F05C)
	// 8323F048: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F04C: 396B3F40  addi r11, r11, 0x3f40
	ctx.r[11].s64 = ctx.r[11].s64 + 16192;
	// 8323F050: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F054: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F058: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F05C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F05C size=8
    let mut pc: u32 = 0x8323F05C;
    'dispatch: loop {
        match pc {
            0x8323F05C => {
    //   block [0x8323F05C..0x8323F064)
	// 8323F05C: 4B081834  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F068 size=20
    let mut pc: u32 = 0x8323F068;
    'dispatch: loop {
        match pc {
            0x8323F068 => {
    //   block [0x8323F068..0x8323F07C)
	// 8323F068: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F06C: 396B3F48  addi r11, r11, 0x3f48
	ctx.r[11].s64 = ctx.r[11].s64 + 16200;
	// 8323F070: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F074: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F078: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F07C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F07C size=8
    let mut pc: u32 = 0x8323F07C;
    'dispatch: loop {
        match pc {
            0x8323F07C => {
    //   block [0x8323F07C..0x8323F084)
	// 8323F07C: 4B081814  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F088 size=20
    let mut pc: u32 = 0x8323F088;
    'dispatch: loop {
        match pc {
            0x8323F088 => {
    //   block [0x8323F088..0x8323F09C)
	// 8323F088: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F08C: 396B3F50  addi r11, r11, 0x3f50
	ctx.r[11].s64 = ctx.r[11].s64 + 16208;
	// 8323F090: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F094: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F098: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F09C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F09C size=8
    let mut pc: u32 = 0x8323F09C;
    'dispatch: loop {
        match pc {
            0x8323F09C => {
    //   block [0x8323F09C..0x8323F0A4)
	// 8323F09C: 4B0817F4  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F0A8 size=80
    let mut pc: u32 = 0x8323F0A8;
    'dispatch: loop {
        match pc {
            0x8323F0A8 => {
    //   block [0x8323F0A8..0x8323F0F8)
	// 8323F0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F0B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F0B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F0B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F0BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F0C0: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8323F0C4: 396B4B18  addi r11, r11, 0x4b18
	ctx.r[11].s64 = ctx.r[11].s64 + 19224;
	// 8323F0C8: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 8323F0CC: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F0D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F0D4: 4B089BE5  bl 0x822c8cb8
	ctx.lr = 0x8323F0D8;
	sub_822C8CB8(ctx, base);
	// 8323F0D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F0DC: 4080FFF0  bge 0x8323f0cc
	if !ctx.cr[0].lt {
	pc = 0x8323F0CC; continue 'dispatch;
	}
	// 8323F0E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F0EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F0F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F0F8 size=80
    let mut pc: u32 = 0x8323F0F8;
    'dispatch: loop {
        match pc {
            0x8323F0F8 => {
    //   block [0x8323F0F8..0x8323F148)
	// 8323F0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F100: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F104: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F10C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F110: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8323F114: 396B4B90  addi r11, r11, 0x4b90
	ctx.r[11].s64 = ctx.r[11].s64 + 19344;
	// 8323F118: 3BEB0058  addi r31, r11, 0x58
	ctx.r[31].s64 = ctx.r[11].s64 + 88;
	// 8323F11C: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F124: 4B089B95  bl 0x822c8cb8
	ctx.lr = 0x8323F128;
	sub_822C8CB8(ctx, base);
	// 8323F128: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F12C: 4080FFF0  bge 0x8323f11c
	if !ctx.cr[0].lt {
	pc = 0x8323F11C; continue 'dispatch;
	}
	// 8323F130: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F13C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F140: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F148 size=80
    let mut pc: u32 = 0x8323F148;
    'dispatch: loop {
        match pc {
            0x8323F148 => {
    //   block [0x8323F148..0x8323F198)
	// 8323F148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F150: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F154: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F15C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F160: 3BC00049  li r30, 0x49
	ctx.r[30].s64 = 73;
	// 8323F164: 396B3F88  addi r11, r11, 0x3f88
	ctx.r[11].s64 = ctx.r[11].s64 + 16264;
	// 8323F168: 3BEB0B98  addi r31, r11, 0xb98
	ctx.r[31].s64 = ctx.r[11].s64 + 2968;
	// 8323F16C: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F174: 4B089B45  bl 0x822c8cb8
	ctx.lr = 0x8323F178;
	sub_822C8CB8(ctx, base);
	// 8323F178: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F17C: 4080FFF0  bge 0x8323f16c
	if !ctx.cr[0].lt {
	pc = 0x8323F16C; continue 'dispatch;
	}
	// 8323F180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F18C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F198 size=12
    let mut pc: u32 = 0x8323F198;
    'dispatch: loop {
        match pc {
            0x8323F198 => {
    //   block [0x8323F198..0x8323F1A4)
	// 8323F198: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F19C: 386B4C34  addi r3, r11, 0x4c34
	ctx.r[3].s64 = ctx.r[11].s64 + 19508;
	// 8323F1A0: 4B193248  b 0x823d23e8
	sub_823D23E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F1A8 size=12
    let mut pc: u32 = 0x8323F1A8;
    'dispatch: loop {
        match pc {
            0x8323F1A8 => {
    //   block [0x8323F1A8..0x8323F1B4)
	// 8323F1A8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F1AC: 386B4CF8  addi r3, r11, 0x4cf8
	ctx.r[3].s64 = ctx.r[11].s64 + 19704;
	// 8323F1B0: 4B1E1C50  b 0x82420e00
	sub_82420E00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F1B8 size=80
    let mut pc: u32 = 0x8323F1B8;
    'dispatch: loop {
        match pc {
            0x8323F1B8 => {
    //   block [0x8323F1B8..0x8323F208)
	// 8323F1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F1C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F1C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F1C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F1CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F1D0: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 8323F1D4: 396B6038  addi r11, r11, 0x6038
	ctx.r[11].s64 = ctx.r[11].s64 + 24632;
	// 8323F1D8: 3BEB00A8  addi r31, r11, 0xa8
	ctx.r[31].s64 = ctx.r[11].s64 + 168;
	// 8323F1DC: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F1E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F1E4: 4B089AD5  bl 0x822c8cb8
	ctx.lr = 0x8323F1E8;
	sub_822C8CB8(ctx, base);
	// 8323F1E8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F1EC: 4080FFF0  bge 0x8323f1dc
	if !ctx.cr[0].lt {
	pc = 0x8323F1DC; continue 'dispatch;
	}
	// 8323F1F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F1FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F208 size=80
    let mut pc: u32 = 0x8323F208;
    'dispatch: loop {
        match pc {
            0x8323F208 => {
    //   block [0x8323F208..0x8323F258)
	// 8323F208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F218: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F21C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F220: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8323F224: 396B60D8  addi r11, r11, 0x60d8
	ctx.r[11].s64 = ctx.r[11].s64 + 24792;
	// 8323F228: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 8323F22C: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F234: 4B089A85  bl 0x822c8cb8
	ctx.lr = 0x8323F238;
	sub_822C8CB8(ctx, base);
	// 8323F238: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F23C: 4080FFF0  bge 0x8323f22c
	if !ctx.cr[0].lt {
	pc = 0x8323F22C; continue 'dispatch;
	}
	// 8323F240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F24C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F258 size=80
    let mut pc: u32 = 0x8323F258;
    'dispatch: loop {
        match pc {
            0x8323F258 => {
    //   block [0x8323F258..0x8323F2A8)
	// 8323F258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F26C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F270: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8323F274: 396B61A0  addi r11, r11, 0x61a0
	ctx.r[11].s64 = ctx.r[11].s64 + 24992;
	// 8323F278: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 8323F27C: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F284: 4B089A35  bl 0x822c8cb8
	ctx.lr = 0x8323F288;
	sub_822C8CB8(ctx, base);
	// 8323F288: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F28C: 4080FFF0  bge 0x8323f27c
	if !ctx.cr[0].lt {
	pc = 0x8323F27C; continue 'dispatch;
	}
	// 8323F290: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F29C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F2A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F2A8 size=80
    let mut pc: u32 = 0x8323F2A8;
    'dispatch: loop {
        match pc {
            0x8323F2A8 => {
    //   block [0x8323F2A8..0x8323F2F8)
	// 8323F2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F2B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F2B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F2B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F2BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F2C0: 3BC0000D  li r30, 0xd
	ctx.r[30].s64 = 13;
	// 8323F2C4: 396B5E08  addi r11, r11, 0x5e08
	ctx.r[11].s64 = ctx.r[11].s64 + 24072;
	// 8323F2C8: 3BEB0238  addi r31, r11, 0x238
	ctx.r[31].s64 = ctx.r[11].s64 + 568;
	// 8323F2CC: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F2D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F2D4: 4B0899E5  bl 0x822c8cb8
	ctx.lr = 0x8323F2D8;
	sub_822C8CB8(ctx, base);
	// 8323F2D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F2DC: 4080FFF0  bge 0x8323f2cc
	if !ctx.cr[0].lt {
	pc = 0x8323F2CC; continue 'dispatch;
	}
	// 8323F2E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F2EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F2F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F2F8 size=80
    let mut pc: u32 = 0x8323F2F8;
    'dispatch: loop {
        match pc {
            0x8323F2F8 => {
    //   block [0x8323F2F8..0x8323F348)
	// 8323F2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F30C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F310: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8323F314: 396B6150  addi r11, r11, 0x6150
	ctx.r[11].s64 = ctx.r[11].s64 + 24912;
	// 8323F318: 3BEB0058  addi r31, r11, 0x58
	ctx.r[31].s64 = ctx.r[11].s64 + 88;
	// 8323F31C: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F324: 4B089995  bl 0x822c8cb8
	ctx.lr = 0x8323F328;
	sub_822C8CB8(ctx, base);
	// 8323F328: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F32C: 4080FFF0  bge 0x8323f31c
	if !ctx.cr[0].lt {
	pc = 0x8323F31C; continue 'dispatch;
	}
	// 8323F330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F33C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F348 size=20
    let mut pc: u32 = 0x8323F348;
    'dispatch: loop {
        match pc {
            0x8323F348 => {
    //   block [0x8323F348..0x8323F35C)
	// 8323F348: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F34C: 396B6220  addi r11, r11, 0x6220
	ctx.r[11].s64 = ctx.r[11].s64 + 25120;
	// 8323F350: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F354: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F358: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F35C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F35C size=8
    let mut pc: u32 = 0x8323F35C;
    'dispatch: loop {
        match pc {
            0x8323F35C => {
    //   block [0x8323F35C..0x8323F364)
	// 8323F35C: 4B081534  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F368 size=20
    let mut pc: u32 = 0x8323F368;
    'dispatch: loop {
        match pc {
            0x8323F368 => {
    //   block [0x8323F368..0x8323F37C)
	// 8323F368: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F36C: 396B6228  addi r11, r11, 0x6228
	ctx.r[11].s64 = ctx.r[11].s64 + 25128;
	// 8323F370: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F378: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F37C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F37C size=8
    let mut pc: u32 = 0x8323F37C;
    'dispatch: loop {
        match pc {
            0x8323F37C => {
    //   block [0x8323F37C..0x8323F384)
	// 8323F37C: 4B081514  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F388 size=20
    let mut pc: u32 = 0x8323F388;
    'dispatch: loop {
        match pc {
            0x8323F388 => {
    //   block [0x8323F388..0x8323F39C)
	// 8323F388: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F38C: 396B6218  addi r11, r11, 0x6218
	ctx.r[11].s64 = ctx.r[11].s64 + 25112;
	// 8323F390: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F394: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F398: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F39C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F39C size=8
    let mut pc: u32 = 0x8323F39C;
    'dispatch: loop {
        match pc {
            0x8323F39C => {
    //   block [0x8323F39C..0x8323F3A4)
	// 8323F39C: 4B0814F4  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F3A8 size=12
    let mut pc: u32 = 0x8323F3A8;
    'dispatch: loop {
        match pc {
            0x8323F3A8 => {
    //   block [0x8323F3A8..0x8323F3B4)
	// 8323F3A8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F3AC: 386B6234  addi r3, r11, 0x6234
	ctx.r[3].s64 = ctx.r[11].s64 + 25140;
	// 8323F3B0: 4BBB4078  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F3B8 size=12
    let mut pc: u32 = 0x8323F3B8;
    'dispatch: loop {
        match pc {
            0x8323F3B8 => {
    //   block [0x8323F3B8..0x8323F3C4)
	// 8323F3B8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F3BC: 386B623C  addi r3, r11, 0x623c
	ctx.r[3].s64 = ctx.r[11].s64 + 25148;
	// 8323F3C0: 4BBB4068  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F3C8 size=80
    let mut pc: u32 = 0x8323F3C8;
    'dispatch: loop {
        match pc {
            0x8323F3C8 => {
    //   block [0x8323F3C8..0x8323F418)
	// 8323F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F3D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F3D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F3D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F3DC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F3E0: 3BC0000C  li r30, 0xc
	ctx.r[30].s64 = 12;
	// 8323F3E4: 396B6280  addi r11, r11, 0x6280
	ctx.r[11].s64 = ctx.r[11].s64 + 25216;
	// 8323F3E8: 3BEB0210  addi r31, r11, 0x210
	ctx.r[31].s64 = ctx.r[11].s64 + 528;
	// 8323F3EC: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323F3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F3F4: 4B0898C5  bl 0x822c8cb8
	ctx.lr = 0x8323F3F8;
	sub_822C8CB8(ctx, base);
	// 8323F3F8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F3FC: 4080FFF0  bge 0x8323f3ec
	if !ctx.cr[0].lt {
	pc = 0x8323F3EC; continue 'dispatch;
	}
	// 8323F400: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F40C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F410: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F418 size=12
    let mut pc: u32 = 0x8323F418;
    'dispatch: loop {
        match pc {
            0x8323F418 => {
    //   block [0x8323F418..0x8323F424)
	// 8323F418: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F41C: 386B6488  addi r3, r11, 0x6488
	ctx.r[3].s64 = ctx.r[11].s64 + 25736;
	// 8323F420: 4BBB4008  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F428 size=12
    let mut pc: u32 = 0x8323F428;
    'dispatch: loop {
        match pc {
            0x8323F428 => {
    //   block [0x8323F428..0x8323F434)
	// 8323F428: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F42C: 386B64A4  addi r3, r11, 0x64a4
	ctx.r[3].s64 = ctx.r[11].s64 + 25764;
	// 8323F430: 4BBB3FF8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F438 size=12
    let mut pc: u32 = 0x8323F438;
    'dispatch: loop {
        match pc {
            0x8323F438 => {
    //   block [0x8323F438..0x8323F444)
	// 8323F438: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F43C: 386B6550  addi r3, r11, 0x6550
	ctx.r[3].s64 = ctx.r[11].s64 + 25936;
	// 8323F440: 4BBB3FE8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F448 size=12
    let mut pc: u32 = 0x8323F448;
    'dispatch: loop {
        match pc {
            0x8323F448 => {
    //   block [0x8323F448..0x8323F454)
	// 8323F448: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F44C: 386B65B8  addi r3, r11, 0x65b8
	ctx.r[3].s64 = ctx.r[11].s64 + 26040;
	// 8323F450: 4BBB3FD8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F458 size=12
    let mut pc: u32 = 0x8323F458;
    'dispatch: loop {
        match pc {
            0x8323F458 => {
    //   block [0x8323F458..0x8323F464)
	// 8323F458: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F45C: 386B65C0  addi r3, r11, 0x65c0
	ctx.r[3].s64 = ctx.r[11].s64 + 26048;
	// 8323F460: 4BBB3FC8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F468 size=12
    let mut pc: u32 = 0x8323F468;
    'dispatch: loop {
        match pc {
            0x8323F468 => {
    //   block [0x8323F468..0x8323F474)
	// 8323F468: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F46C: 386B6604  addi r3, r11, 0x6604
	ctx.r[3].s64 = ctx.r[11].s64 + 26116;
	// 8323F470: 4BBB3FB8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F478 size=68
    let mut pc: u32 = 0x8323F478;
    'dispatch: loop {
        match pc {
            0x8323F478 => {
    //   block [0x8323F478..0x8323F4BC)
	// 8323F478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F488: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F48C: 83EB6674  lwz r31, 0x6674(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8323F490: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8323F494: 419A0014  beq cr6, 0x8323f4a8
	if ctx.cr[6].eq {
	pc = 0x8323F4A8; continue 'dispatch;
	}
	// 8323F498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F49C: 4B22AB65  bl 0x8246a000
	ctx.lr = 0x8323F4A0;
	sub_8246A000(ctx, base);
	// 8323F4A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F4A4: 4B080DC5  bl 0x822c0268
	ctx.lr = 0x8323F4A8;
	sub_822C0268(ctx, base);
	// 8323F4A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323F4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F4B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F4C0 size=60
    let mut pc: u32 = 0x8323F4C0;
    'dispatch: loop {
        match pc {
            0x8323F4C0 => {
    //   block [0x8323F4C0..0x8323F4FC)
	// 8323F4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F4C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F4CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F4D0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F4D4: 3BEB6678  addi r31, r11, 0x6678
	ctx.r[31].s64 = ctx.r[11].s64 + 26232;
	// 8323F4D8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8323F4DC: 4B38EDFD  bl 0x825ce2d8
	ctx.lr = 0x8323F4E0;
	sub_825CE2D8(ctx, base);
	// 8323F4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F4E4: 4B24CAE5  bl 0x8248bfc8
	ctx.lr = 0x8323F4E8;
	sub_8248BFC8(ctx, base);
	// 8323F4E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323F4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F4F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F500 size=68
    let mut pc: u32 = 0x8323F500;
    'dispatch: loop {
        match pc {
            0x8323F500 => {
    //   block [0x8323F500..0x8323F544)
	// 8323F500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F50C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F510: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F514: 83EB6670  lwz r31, 0x6670(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26224 as u32) ) } as u64;
	// 8323F518: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8323F51C: 419A0014  beq cr6, 0x8323f530
	if ctx.cr[6].eq {
	pc = 0x8323F530; continue 'dispatch;
	}
	// 8323F520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F524: 4B24C38D  bl 0x8248b8b0
	ctx.lr = 0x8323F528;
	sub_8248B8B0(ctx, base);
	// 8323F528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F52C: 4B080D3D  bl 0x822c0268
	ctx.lr = 0x8323F530;
	sub_822C0268(ctx, base);
	// 8323F530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323F534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F53C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F548 size=12
    let mut pc: u32 = 0x8323F548;
    'dispatch: loop {
        match pc {
            0x8323F548 => {
    //   block [0x8323F548..0x8323F554)
	// 8323F548: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F54C: 386B6938  addi r3, r11, 0x6938
	ctx.r[3].s64 = ctx.r[11].s64 + 26936;
	// 8323F550: 4B89D568  b 0x82adcab8
	sub_82ADCAB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F558 size=12
    let mut pc: u32 = 0x8323F558;
    'dispatch: loop {
        match pc {
            0x8323F558 => {
    //   block [0x8323F558..0x8323F564)
	// 8323F558: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F55C: 386B690C  addi r3, r11, 0x690c
	ctx.r[3].s64 = ctx.r[11].s64 + 26892;
	// 8323F560: 4B2E7118  b 0x82526678
	sub_82526678(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F568 size=28
    let mut pc: u32 = 0x8323F568;
    'dispatch: loop {
        match pc {
            0x8323F568 => {
    //   block [0x8323F568..0x8323F584)
	// 8323F568: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 8323F56C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8323F570: 39496958  addi r10, r9, 0x6958
	ctx.r[10].s64 = ctx.r[9].s64 + 26968;
	// 8323F574: 396B03BC  addi r11, r11, 0x3bc
	ctx.r[11].s64 = ctx.r[11].s64 + 956;
	// 8323F578: 386A0008  addi r3, r10, 8
	ctx.r[3].s64 = ctx.r[10].s64 + 8;
	// 8323F57C: 91696958  stw r11, 0x6958(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(26968 as u32), ctx.r[11].u32 ) };
	// 8323F580: 4BBB2FD0  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F588 size=12
    let mut pc: u32 = 0x8323F588;
    'dispatch: loop {
        match pc {
            0x8323F588 => {
    //   block [0x8323F588..0x8323F594)
	// 8323F588: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F58C: 386B6E80  addi r3, r11, 0x6e80
	ctx.r[3].s64 = ctx.r[11].s64 + 28288;
	// 8323F590: 4BBB2FC0  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F598 size=12
    let mut pc: u32 = 0x8323F598;
    'dispatch: loop {
        match pc {
            0x8323F598 => {
    //   block [0x8323F598..0x8323F5A4)
	// 8323F598: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F59C: 386B6CB0  addi r3, r11, 0x6cb0
	ctx.r[3].s64 = ctx.r[11].s64 + 27824;
	// 8323F5A0: 4BBB2FB0  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F5A8 size=68
    let mut pc: u32 = 0x8323F5A8;
    'dispatch: loop {
        match pc {
            0x8323F5A8 => {
    //   block [0x8323F5A8..0x8323F5EC)
	// 8323F5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F5B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F5B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F5B8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F5BC: 3BEB6F70  addi r31, r11, 0x6f70
	ctx.r[31].s64 = ctx.r[11].s64 + 28528;
	// 8323F5C0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8323F5C4: 4B96E735  bl 0x82badcf8
	ctx.lr = 0x8323F5C8;
	sub_82BADCF8(ctx, base);
	// 8323F5C8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8323F5CC: 4B96E72D  bl 0x82badcf8
	ctx.lr = 0x8323F5D0;
	sub_82BADCF8(ctx, base);
	// 8323F5D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F5D4: 4BBB2F7D  bl 0x82df2550
	ctx.lr = 0x8323F5D8;
	sub_82DF2550(ctx, base);
	// 8323F5D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323F5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F5E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F5F0 size=12
    let mut pc: u32 = 0x8323F5F0;
    'dispatch: loop {
        match pc {
            0x8323F5F0 => {
    //   block [0x8323F5F0..0x8323F5FC)
	// 8323F5F0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F5F4: 386B6F5C  addi r3, r11, 0x6f5c
	ctx.r[3].s64 = ctx.r[11].s64 + 28508;
	// 8323F5F8: 4B2AC350  b 0x824eb948
	sub_824EB948(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F600 size=20
    let mut pc: u32 = 0x8323F600;
    'dispatch: loop {
        match pc {
            0x8323F600 => {
    //   block [0x8323F600..0x8323F614)
	// 8323F600: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F604: 396B6FF8  addi r11, r11, 0x6ff8
	ctx.r[11].s64 = ctx.r[11].s64 + 28664;
	// 8323F608: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F60C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F610: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F614(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F614 size=8
    let mut pc: u32 = 0x8323F614;
    'dispatch: loop {
        match pc {
            0x8323F614 => {
    //   block [0x8323F614..0x8323F61C)
	// 8323F614: 4B08127C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F620 size=20
    let mut pc: u32 = 0x8323F620;
    'dispatch: loop {
        match pc {
            0x8323F620 => {
    //   block [0x8323F620..0x8323F634)
	// 8323F620: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F624: 396B7008  addi r11, r11, 0x7008
	ctx.r[11].s64 = ctx.r[11].s64 + 28680;
	// 8323F628: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F62C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F630: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F634(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F634 size=8
    let mut pc: u32 = 0x8323F634;
    'dispatch: loop {
        match pc {
            0x8323F634 => {
    //   block [0x8323F634..0x8323F63C)
	// 8323F634: 4B08125C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F640 size=20
    let mut pc: u32 = 0x8323F640;
    'dispatch: loop {
        match pc {
            0x8323F640 => {
    //   block [0x8323F640..0x8323F654)
	// 8323F640: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F644: 396B7010  addi r11, r11, 0x7010
	ctx.r[11].s64 = ctx.r[11].s64 + 28688;
	// 8323F648: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F64C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F650: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F654 size=8
    let mut pc: u32 = 0x8323F654;
    'dispatch: loop {
        match pc {
            0x8323F654 => {
    //   block [0x8323F654..0x8323F65C)
	// 8323F654: 4B08123C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F660 size=12
    let mut pc: u32 = 0x8323F660;
    'dispatch: loop {
        match pc {
            0x8323F660 => {
    //   block [0x8323F660..0x8323F66C)
	// 8323F660: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F664: 386B7148  addi r3, r11, 0x7148
	ctx.r[3].s64 = ctx.r[11].s64 + 29000;
	// 8323F668: 4B22A998  b 0x8246a000
	sub_8246A000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F670 size=12
    let mut pc: u32 = 0x8323F670;
    'dispatch: loop {
        match pc {
            0x8323F670 => {
    //   block [0x8323F670..0x8323F67C)
	// 8323F670: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F674: 386B715C  addi r3, r11, 0x715c
	ctx.r[3].s64 = ctx.r[11].s64 + 29020;
	// 8323F678: 4BBB3DB0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F680 size=12
    let mut pc: u32 = 0x8323F680;
    'dispatch: loop {
        match pc {
            0x8323F680 => {
    //   block [0x8323F680..0x8323F68C)
	// 8323F680: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F684: 386B7160  addi r3, r11, 0x7160
	ctx.r[3].s64 = ctx.r[11].s64 + 29024;
	// 8323F688: 4BBB3DA0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F690 size=12
    let mut pc: u32 = 0x8323F690;
    'dispatch: loop {
        match pc {
            0x8323F690 => {
    //   block [0x8323F690..0x8323F69C)
	// 8323F690: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F694: 386B7168  addi r3, r11, 0x7168
	ctx.r[3].s64 = ctx.r[11].s64 + 29032;
	// 8323F698: 4BBB3D90  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F6A0 size=96
    let mut pc: u32 = 0x8323F6A0;
    'dispatch: loop {
        match pc {
            0x8323F6A0 => {
    //   block [0x8323F6A0..0x8323F700)
	// 8323F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F6A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F6AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F6B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F6B4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F6B8: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8323F6BC: 396B7188  addi r11, r11, 0x7188
	ctx.r[11].s64 = ctx.r[11].s64 + 29064;
	// 8323F6C0: 3BEB004C  addi r31, r11, 0x4c
	ctx.r[31].s64 = ctx.r[11].s64 + 76;
	// 8323F6C4: 3BFFFFE8  addi r31, r31, -0x18
	ctx.r[31].s64 = ctx.r[31].s64 + -24;
	// 8323F6C8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8323F6CC: 4BBB3D5D  bl 0x82df3428
	ctx.lr = 0x8323F6D0;
	sub_82DF3428(ctx, base);
	// 8323F6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F6D4: 4BBB3D55  bl 0x82df3428
	ctx.lr = 0x8323F6D8;
	sub_82DF3428(ctx, base);
	// 8323F6D8: 387FFFFC  addi r3, r31, -4
	ctx.r[3].s64 = ctx.r[31].s64 + -4;
	// 8323F6DC: 4BBB3D4D  bl 0x82df3428
	ctx.lr = 0x8323F6E0;
	sub_82DF3428(ctx, base);
	// 8323F6E0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F6E4: 4080FFE0  bge 0x8323f6c4
	if !ctx.cr[0].lt {
	pc = 0x8323F6C4; continue 'dispatch;
	}
	// 8323F6E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F6F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F6F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F700 size=12
    let mut pc: u32 = 0x8323F700;
    'dispatch: loop {
        match pc {
            0x8323F700 => {
    //   block [0x8323F700..0x8323F70C)
	// 8323F700: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F704: 386B71FC  addi r3, r11, 0x71fc
	ctx.r[3].s64 = ctx.r[11].s64 + 29180;
	// 8323F708: 4BBB3D20  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F710 size=12
    let mut pc: u32 = 0x8323F710;
    'dispatch: loop {
        match pc {
            0x8323F710 => {
    //   block [0x8323F710..0x8323F71C)
	// 8323F710: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F714: 386B7200  addi r3, r11, 0x7200
	ctx.r[3].s64 = ctx.r[11].s64 + 29184;
	// 8323F718: 4BBB3D10  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F720 size=12
    let mut pc: u32 = 0x8323F720;
    'dispatch: loop {
        match pc {
            0x8323F720 => {
    //   block [0x8323F720..0x8323F72C)
	// 8323F720: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F724: 386B72B4  addi r3, r11, 0x72b4
	ctx.r[3].s64 = ctx.r[11].s64 + 29364;
	// 8323F728: 4BBB3D00  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F730 size=12
    let mut pc: u32 = 0x8323F730;
    'dispatch: loop {
        match pc {
            0x8323F730 => {
    //   block [0x8323F730..0x8323F73C)
	// 8323F730: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F734: 386B72B8  addi r3, r11, 0x72b8
	ctx.r[3].s64 = ctx.r[11].s64 + 29368;
	// 8323F738: 4BBB3CF0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F740 size=12
    let mut pc: u32 = 0x8323F740;
    'dispatch: loop {
        match pc {
            0x8323F740 => {
    //   block [0x8323F740..0x8323F74C)
	// 8323F740: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F744: 386B7B64  addi r3, r11, 0x7b64
	ctx.r[3].s64 = ctx.r[11].s64 + 31588;
	// 8323F748: 4BBB3CE0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F750 size=12
    let mut pc: u32 = 0x8323F750;
    'dispatch: loop {
        match pc {
            0x8323F750 => {
    //   block [0x8323F750..0x8323F75C)
	// 8323F750: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F754: 386B7BC0  addi r3, r11, 0x7bc0
	ctx.r[3].s64 = ctx.r[11].s64 + 31680;
	// 8323F758: 4BBB3CD0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F760 size=12
    let mut pc: u32 = 0x8323F760;
    'dispatch: loop {
        match pc {
            0x8323F760 => {
    //   block [0x8323F760..0x8323F76C)
	// 8323F760: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F764: 386B7C34  addi r3, r11, 0x7c34
	ctx.r[3].s64 = ctx.r[11].s64 + 31796;
	// 8323F768: 4B192C80  b 0x823d23e8
	sub_823D23E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F770 size=12
    let mut pc: u32 = 0x8323F770;
    'dispatch: loop {
        match pc {
            0x8323F770 => {
    //   block [0x8323F770..0x8323F77C)
	// 8323F770: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F774: 386B7D38  addi r3, r11, 0x7d38
	ctx.r[3].s64 = ctx.r[11].s64 + 32056;
	// 8323F778: 4BBB3CB0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F780 size=12
    let mut pc: u32 = 0x8323F780;
    'dispatch: loop {
        match pc {
            0x8323F780 => {
    //   block [0x8323F780..0x8323F78C)
	// 8323F780: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F784: 386B7D40  addi r3, r11, 0x7d40
	ctx.r[3].s64 = ctx.r[11].s64 + 32064;
	// 8323F788: 4B38EB50  b 0x825ce2d8
	sub_825CE2D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F790 size=4
    let mut pc: u32 = 0x8323F790;
    'dispatch: loop {
        match pc {
            0x8323F790 => {
    //   block [0x8323F790..0x8323F794)
	// 8323F790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F798 size=4
    let mut pc: u32 = 0x8323F798;
    'dispatch: loop {
        match pc {
            0x8323F798 => {
    //   block [0x8323F798..0x8323F79C)
	// 8323F798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F7A0 size=12
    let mut pc: u32 = 0x8323F7A0;
    'dispatch: loop {
        match pc {
            0x8323F7A0 => {
    //   block [0x8323F7A0..0x8323F7AC)
	// 8323F7A0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F7A4: 386B7DFC  addi r3, r11, 0x7dfc
	ctx.r[3].s64 = ctx.r[11].s64 + 32252;
	// 8323F7A8: 4B39F8B0  b 0x825df058
	sub_825DF058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F7B0 size=20
    let mut pc: u32 = 0x8323F7B0;
    'dispatch: loop {
        match pc {
            0x8323F7B0 => {
    //   block [0x8323F7B0..0x8323F7C4)
	// 8323F7B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8323F7B4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8323F7B8: 396BED84  addi r11, r11, -0x127c
	ctx.r[11].s64 = ctx.r[11].s64 + -4732;
	// 8323F7BC: 916A7E30  stw r11, 0x7e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32304 as u32), ctx.r[11].u32 ) };
	// 8323F7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F7C8 size=12
    let mut pc: u32 = 0x8323F7C8;
    'dispatch: loop {
        match pc {
            0x8323F7C8 => {
    //   block [0x8323F7C8..0x8323F7D4)
	// 8323F7C8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F7CC: 386B7E34  addi r3, r11, 0x7e34
	ctx.r[3].s64 = ctx.r[11].s64 + 32308;
	// 8323F7D0: 4B39E5A0  b 0x825ddd70
	sub_825DDD70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F7D8 size=36
    let mut pc: u32 = 0x8323F7D8;
    'dispatch: loop {
        match pc {
            0x8323F7D8 => {
    //   block [0x8323F7D8..0x8323F7FC)
	// 8323F7D8: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 8323F7DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8323F7E0: 39497E10  addi r10, r9, 0x7e10
	ctx.r[10].s64 = ctx.r[9].s64 + 32272;
	// 8323F7E4: 396BEF28  addi r11, r11, -0x10d8
	ctx.r[11].s64 = ctx.r[11].s64 + -4312;
	// 8323F7E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8323F7EC: 91697E10  stw r11, 0x7e10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32272 as u32), ctx.r[11].u32 ) };
	// 8323F7F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8323F7F4: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 8323F7F8: 4B0854E8  b 0x822c4ce0
	sub_822C4CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F800 size=20
    let mut pc: u32 = 0x8323F800;
    'dispatch: loop {
        match pc {
            0x8323F800 => {
    //   block [0x8323F800..0x8323F814)
	// 8323F800: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8323F804: 396B7E78  addi r11, r11, 0x7e78
	ctx.r[11].s64 = ctx.r[11].s64 + 32376;
	// 8323F808: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F80C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F810: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F814 size=8
    let mut pc: u32 = 0x8323F814;
    'dispatch: loop {
        match pc {
            0x8323F814 => {
    //   block [0x8323F814..0x8323F81C)
	// 8323F814: 4B08107C  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F820 size=4
    let mut pc: u32 = 0x8323F820;
    'dispatch: loop {
        match pc {
            0x8323F820 => {
    //   block [0x8323F820..0x8323F824)
	// 8323F820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F828 size=60
    let mut pc: u32 = 0x8323F828;
    'dispatch: loop {
        match pc {
            0x8323F828 => {
    //   block [0x8323F828..0x8323F864)
	// 8323F828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F838: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F83C: 3BEB8584  addi r31, r11, -0x7a7c
	ctx.r[31].s64 = ctx.r[11].s64 + -31356;
	// 8323F840: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8323F844: 4B38EA95  bl 0x825ce2d8
	ctx.lr = 0x8323F848;
	sub_825CE2D8(ctx, base);
	// 8323F848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F84C: 4B24C77D  bl 0x8248bfc8
	ctx.lr = 0x8323F850;
	sub_8248BFC8(ctx, base);
	// 8323F850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323F854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F85C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F868 size=60
    let mut pc: u32 = 0x8323F868;
    'dispatch: loop {
        match pc {
            0x8323F868 => {
    //   block [0x8323F868..0x8323F8A4)
	// 8323F868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F878: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F87C: 3BEB86F0  addi r31, r11, -0x7910
	ctx.r[31].s64 = ctx.r[11].s64 + -30992;
	// 8323F880: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8323F884: 4B38EA55  bl 0x825ce2d8
	ctx.lr = 0x8323F888;
	sub_825CE2D8(ctx, base);
	// 8323F888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F88C: 4B24C73D  bl 0x8248bfc8
	ctx.lr = 0x8323F890;
	sub_8248BFC8(ctx, base);
	// 8323F890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323F894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F89C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F8A8 size=80
    let mut pc: u32 = 0x8323F8A8;
    'dispatch: loop {
        match pc {
            0x8323F8A8 => {
    //   block [0x8323F8A8..0x8323F8F8)
	// 8323F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F8B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F8B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F8B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F8BC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F8C0: 3BC00057  li r30, 0x57
	ctx.r[30].s64 = 87;
	// 8323F8C4: 396BA118  addi r11, r11, -0x5ee8
	ctx.r[11].s64 = ctx.r[11].s64 + -24296;
	// 8323F8C8: 3BEB0B00  addi r31, r11, 0xb00
	ctx.r[31].s64 = ctx.r[11].s64 + 2816;
	// 8323F8CC: 3BFFFFE0  addi r31, r31, -0x20
	ctx.r[31].s64 = ctx.r[31].s64 + -32;
	// 8323F8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F8D4: 4B0893E5  bl 0x822c8cb8
	ctx.lr = 0x8323F8D8;
	sub_822C8CB8(ctx, base);
	// 8323F8D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F8DC: 4080FFF0  bge 0x8323f8cc
	if !ctx.cr[0].lt {
	pc = 0x8323F8CC; continue 'dispatch;
	}
	// 8323F8E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323F8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323F8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323F8EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323F8F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323F8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F8F8 size=20
    let mut pc: u32 = 0x8323F8F8;
    'dispatch: loop {
        match pc {
            0x8323F8F8 => {
    //   block [0x8323F8F8..0x8323F90C)
	// 8323F8F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F8FC: 396BAC64  addi r11, r11, -0x539c
	ctx.r[11].s64 = ctx.r[11].s64 + -21404;
	// 8323F900: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F904: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F908: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F90C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F90C size=40
    let mut pc: u32 = 0x8323F90C;
    'dispatch: loop {
        match pc {
            0x8323F90C => {
    //   block [0x8323F90C..0x8323F934)
	// 8323F90C: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 8323F910: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8323F914: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8323F918: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8323F91C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8323F920: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8323F924: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8323F928: 4082FFE8  bne 0x8323f910
	if !ctx.cr[0].eq {
	pc = 0x8323F910; continue 'dispatch;
	}
	// 8323F92C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8323F930: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F934 size=16
    let mut pc: u32 = 0x8323F934;
    'dispatch: loop {
        match pc {
            0x8323F934 => {
    //   block [0x8323F934..0x8323F944)
	// 8323F934: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8323F938: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8323F93C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8323F940: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F944 size=4
    let mut pc: u32 = 0x8323F944;
    'dispatch: loop {
        match pc {
            0x8323F944 => {
    //   block [0x8323F944..0x8323F948)
	// 8323F944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F948 size=12
    let mut pc: u32 = 0x8323F948;
    'dispatch: loop {
        match pc {
            0x8323F948 => {
    //   block [0x8323F948..0x8323F954)
	// 8323F948: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F94C: 386BAC78  addi r3, r11, -0x5388
	ctx.r[3].s64 = ctx.r[11].s64 + -21384;
	// 8323F950: 4BBB3AD8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F958 size=20
    let mut pc: u32 = 0x8323F958;
    'dispatch: loop {
        match pc {
            0x8323F958 => {
    //   block [0x8323F958..0x8323F96C)
	// 8323F958: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F95C: 396BAD20  addi r11, r11, -0x52e0
	ctx.r[11].s64 = ctx.r[11].s64 + -21216;
	// 8323F960: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F964: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F968: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F96C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F96C size=40
    let mut pc: u32 = 0x8323F96C;
    'dispatch: loop {
        match pc {
            0x8323F96C => {
    //   block [0x8323F96C..0x8323F994)
	// 8323F96C: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 8323F970: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8323F974: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8323F978: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8323F97C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8323F980: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8323F984: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8323F988: 4082FFE8  bne 0x8323f970
	if !ctx.cr[0].eq {
	pc = 0x8323F970; continue 'dispatch;
	}
	// 8323F98C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8323F990: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F994 size=16
    let mut pc: u32 = 0x8323F994;
    'dispatch: loop {
        match pc {
            0x8323F994 => {
    //   block [0x8323F994..0x8323F9A4)
	// 8323F994: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8323F998: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8323F99C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8323F9A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F9A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F9A4 size=4
    let mut pc: u32 = 0x8323F9A4;
    'dispatch: loop {
        match pc {
            0x8323F9A4 => {
    //   block [0x8323F9A4..0x8323F9A8)
	// 8323F9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F9A8 size=20
    let mut pc: u32 = 0x8323F9A8;
    'dispatch: loop {
        match pc {
            0x8323F9A8 => {
    //   block [0x8323F9A8..0x8323F9BC)
	// 8323F9A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F9AC: 396BAD44  addi r11, r11, -0x52bc
	ctx.r[11].s64 = ctx.r[11].s64 + -21180;
	// 8323F9B0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323F9B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323F9B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F9BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323F9BC size=8
    let mut pc: u32 = 0x8323F9BC;
    'dispatch: loop {
        match pc {
            0x8323F9BC => {
    //   block [0x8323F9BC..0x8323F9C4)
	// 8323F9BC: 4B080ED4  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8323F9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323F9C8 size=80
    let mut pc: u32 = 0x8323F9C8;
    'dispatch: loop {
        match pc {
            0x8323F9C8 => {
    //   block [0x8323F9C8..0x8323FA18)
	// 8323F9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323F9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323F9D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323F9D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323F9D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323F9DC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323F9E0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8323F9E4: 396BAE24  addi r11, r11, -0x51dc
	ctx.r[11].s64 = ctx.r[11].s64 + -20956;
	// 8323F9E8: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8323F9EC: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 8323F9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323F9F4: 4BBB3A35  bl 0x82df3428
	ctx.lr = 0x8323F9F8;
	sub_82DF3428(ctx, base);
	// 8323F9F8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323F9FC: 4080FFF0  bge 0x8323f9ec
	if !ctx.cr[0].lt {
	pc = 0x8323F9EC; continue 'dispatch;
	}
	// 8323FA00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FA0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FA10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA18 size=12
    let mut pc: u32 = 0x8323FA18;
    'dispatch: loop {
        match pc {
            0x8323FA18 => {
    //   block [0x8323FA18..0x8323FA24)
	// 8323FA18: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA1C: 386BAE5C  addi r3, r11, -0x51a4
	ctx.r[3].s64 = ctx.r[11].s64 + -20900;
	// 8323FA20: 4BBB3A08  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA28 size=12
    let mut pc: u32 = 0x8323FA28;
    'dispatch: loop {
        match pc {
            0x8323FA28 => {
    //   block [0x8323FA28..0x8323FA34)
	// 8323FA28: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA2C: 386BAE64  addi r3, r11, -0x519c
	ctx.r[3].s64 = ctx.r[11].s64 + -20892;
	// 8323FA30: 4BBB39F8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA38 size=12
    let mut pc: u32 = 0x8323FA38;
    'dispatch: loop {
        match pc {
            0x8323FA38 => {
    //   block [0x8323FA38..0x8323FA44)
	// 8323FA38: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA3C: 386BAE6C  addi r3, r11, -0x5194
	ctx.r[3].s64 = ctx.r[11].s64 + -20884;
	// 8323FA40: 4BBB39E8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA48 size=12
    let mut pc: u32 = 0x8323FA48;
    'dispatch: loop {
        match pc {
            0x8323FA48 => {
    //   block [0x8323FA48..0x8323FA54)
	// 8323FA48: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA4C: 386BAE74  addi r3, r11, -0x518c
	ctx.r[3].s64 = ctx.r[11].s64 + -20876;
	// 8323FA50: 4BBB39D8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA58 size=12
    let mut pc: u32 = 0x8323FA58;
    'dispatch: loop {
        match pc {
            0x8323FA58 => {
    //   block [0x8323FA58..0x8323FA64)
	// 8323FA58: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA5C: 386BAE7C  addi r3, r11, -0x5184
	ctx.r[3].s64 = ctx.r[11].s64 + -20868;
	// 8323FA60: 4BBB39C8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA68 size=12
    let mut pc: u32 = 0x8323FA68;
    'dispatch: loop {
        match pc {
            0x8323FA68 => {
    //   block [0x8323FA68..0x8323FA74)
	// 8323FA68: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA6C: 386BAE84  addi r3, r11, -0x517c
	ctx.r[3].s64 = ctx.r[11].s64 + -20860;
	// 8323FA70: 4BBB39B8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA78 size=12
    let mut pc: u32 = 0x8323FA78;
    'dispatch: loop {
        match pc {
            0x8323FA78 => {
    //   block [0x8323FA78..0x8323FA84)
	// 8323FA78: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA7C: 386BAE8C  addi r3, r11, -0x5174
	ctx.r[3].s64 = ctx.r[11].s64 + -20852;
	// 8323FA80: 4BBB39A8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA88 size=12
    let mut pc: u32 = 0x8323FA88;
    'dispatch: loop {
        match pc {
            0x8323FA88 => {
    //   block [0x8323FA88..0x8323FA94)
	// 8323FA88: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA8C: 386BAE94  addi r3, r11, -0x516c
	ctx.r[3].s64 = ctx.r[11].s64 + -20844;
	// 8323FA90: 4BBB3998  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FA98 size=12
    let mut pc: u32 = 0x8323FA98;
    'dispatch: loop {
        match pc {
            0x8323FA98 => {
    //   block [0x8323FA98..0x8323FAA4)
	// 8323FA98: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FA9C: 386BAE9C  addi r3, r11, -0x5164
	ctx.r[3].s64 = ctx.r[11].s64 + -20836;
	// 8323FAA0: 4BBB3988  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FAA8 size=12
    let mut pc: u32 = 0x8323FAA8;
    'dispatch: loop {
        match pc {
            0x8323FAA8 => {
    //   block [0x8323FAA8..0x8323FAB4)
	// 8323FAA8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FAAC: 386BAEA4  addi r3, r11, -0x515c
	ctx.r[3].s64 = ctx.r[11].s64 + -20828;
	// 8323FAB0: 4BBB3978  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FAB8 size=12
    let mut pc: u32 = 0x8323FAB8;
    'dispatch: loop {
        match pc {
            0x8323FAB8 => {
    //   block [0x8323FAB8..0x8323FAC4)
	// 8323FAB8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FABC: 386BAEAC  addi r3, r11, -0x5154
	ctx.r[3].s64 = ctx.r[11].s64 + -20820;
	// 8323FAC0: 4BBB3968  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FAC8 size=12
    let mut pc: u32 = 0x8323FAC8;
    'dispatch: loop {
        match pc {
            0x8323FAC8 => {
    //   block [0x8323FAC8..0x8323FAD4)
	// 8323FAC8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FACC: 386BAEB4  addi r3, r11, -0x514c
	ctx.r[3].s64 = ctx.r[11].s64 + -20812;
	// 8323FAD0: 4BBB3958  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FAD8 size=12
    let mut pc: u32 = 0x8323FAD8;
    'dispatch: loop {
        match pc {
            0x8323FAD8 => {
    //   block [0x8323FAD8..0x8323FAE4)
	// 8323FAD8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FADC: 386BAEBC  addi r3, r11, -0x5144
	ctx.r[3].s64 = ctx.r[11].s64 + -20804;
	// 8323FAE0: 4BBB3948  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FAE8 size=12
    let mut pc: u32 = 0x8323FAE8;
    'dispatch: loop {
        match pc {
            0x8323FAE8 => {
    //   block [0x8323FAE8..0x8323FAF4)
	// 8323FAE8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FAEC: 386BAEC4  addi r3, r11, -0x513c
	ctx.r[3].s64 = ctx.r[11].s64 + -20796;
	// 8323FAF0: 4BBB3938  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FAF8 size=12
    let mut pc: u32 = 0x8323FAF8;
    'dispatch: loop {
        match pc {
            0x8323FAF8 => {
    //   block [0x8323FAF8..0x8323FB04)
	// 8323FAF8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FAFC: 386BAECC  addi r3, r11, -0x5134
	ctx.r[3].s64 = ctx.r[11].s64 + -20788;
	// 8323FB00: 4BBB3928  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FB08 size=12
    let mut pc: u32 = 0x8323FB08;
    'dispatch: loop {
        match pc {
            0x8323FB08 => {
    //   block [0x8323FB08..0x8323FB14)
	// 8323FB08: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB0C: 386BAED4  addi r3, r11, -0x512c
	ctx.r[3].s64 = ctx.r[11].s64 + -20780;
	// 8323FB10: 4BBB3918  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FB18 size=12
    let mut pc: u32 = 0x8323FB18;
    'dispatch: loop {
        match pc {
            0x8323FB18 => {
    //   block [0x8323FB18..0x8323FB24)
	// 8323FB18: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB1C: 386BAEDC  addi r3, r11, -0x5124
	ctx.r[3].s64 = ctx.r[11].s64 + -20772;
	// 8323FB20: 4BBB3908  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FB28 size=12
    let mut pc: u32 = 0x8323FB28;
    'dispatch: loop {
        match pc {
            0x8323FB28 => {
    //   block [0x8323FB28..0x8323FB34)
	// 8323FB28: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB2C: 386BAEE4  addi r3, r11, -0x511c
	ctx.r[3].s64 = ctx.r[11].s64 + -20764;
	// 8323FB30: 4BBB38F8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FB38 size=12
    let mut pc: u32 = 0x8323FB38;
    'dispatch: loop {
        match pc {
            0x8323FB38 => {
    //   block [0x8323FB38..0x8323FB44)
	// 8323FB38: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB3C: 386BAEEC  addi r3, r11, -0x5114
	ctx.r[3].s64 = ctx.r[11].s64 + -20756;
	// 8323FB40: 4BBB38E8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FB48 size=12
    let mut pc: u32 = 0x8323FB48;
    'dispatch: loop {
        match pc {
            0x8323FB48 => {
    //   block [0x8323FB48..0x8323FB54)
	// 8323FB48: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB4C: 386BAEF4  addi r3, r11, -0x510c
	ctx.r[3].s64 = ctx.r[11].s64 + -20748;
	// 8323FB50: 4BBB38D8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FB58 size=12
    let mut pc: u32 = 0x8323FB58;
    'dispatch: loop {
        match pc {
            0x8323FB58 => {
    //   block [0x8323FB58..0x8323FB64)
	// 8323FB58: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB5C: 386BAEFC  addi r3, r11, -0x5104
	ctx.r[3].s64 = ctx.r[11].s64 + -20740;
	// 8323FB60: 4BBB38C8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FB68 size=12
    let mut pc: u32 = 0x8323FB68;
    'dispatch: loop {
        match pc {
            0x8323FB68 => {
    //   block [0x8323FB68..0x8323FB74)
	// 8323FB68: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB6C: 386BAE54  addi r3, r11, -0x51ac
	ctx.r[3].s64 = ctx.r[11].s64 + -20908;
	// 8323FB70: 4BBB38B8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FB78 size=60
    let mut pc: u32 = 0x8323FB78;
    'dispatch: loop {
        match pc {
            0x8323FB78 => {
    //   block [0x8323FB78..0x8323FBB4)
	// 8323FB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FB80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FB84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FB88: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FB8C: 3BEBB2AC  addi r31, r11, -0x4d54
	ctx.r[31].s64 = ctx.r[11].s64 + -19796;
	// 8323FB90: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8323FB94: 4BBB3895  bl 0x82df3428
	ctx.lr = 0x8323FB98;
	sub_82DF3428(ctx, base);
	// 8323FB98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FB9C: 4BBB388D  bl 0x82df3428
	ctx.lr = 0x8323FBA0;
	sub_82DF3428(ctx, base);
	// 8323FBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323FBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FBAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FBB8 size=60
    let mut pc: u32 = 0x8323FBB8;
    'dispatch: loop {
        match pc {
            0x8323FBB8 => {
    //   block [0x8323FBB8..0x8323FBF4)
	// 8323FBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FBC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FBC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FBC8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FBCC: 3BEBB3D0  addi r31, r11, -0x4c30
	ctx.r[31].s64 = ctx.r[11].s64 + -19504;
	// 8323FBD0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8323FBD4: 4BBB3855  bl 0x82df3428
	ctx.lr = 0x8323FBD8;
	sub_82DF3428(ctx, base);
	// 8323FBD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FBDC: 4BBB384D  bl 0x82df3428
	ctx.lr = 0x8323FBE0;
	sub_82DF3428(ctx, base);
	// 8323FBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323FBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FBEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FBF8 size=20
    let mut pc: u32 = 0x8323FBF8;
    'dispatch: loop {
        match pc {
            0x8323FBF8 => {
    //   block [0x8323FBF8..0x8323FC0C)
	// 8323FBF8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FBFC: 396BB4EC  addi r11, r11, -0x4b14
	ctx.r[11].s64 = ctx.r[11].s64 + -19220;
	// 8323FC00: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8323FC04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8323FC08: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FC0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FC0C size=40
    let mut pc: u32 = 0x8323FC0C;
    'dispatch: loop {
        match pc {
            0x8323FC0C => {
    //   block [0x8323FC0C..0x8323FC34)
	// 8323FC0C: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 8323FC10: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8323FC14: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8323FC18: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8323FC1C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8323FC20: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8323FC24: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8323FC28: 4082FFE8  bne 0x8323fc10
	if !ctx.cr[0].eq {
	pc = 0x8323FC10; continue 'dispatch;
	}
	// 8323FC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8323FC30: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FC34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FC34 size=16
    let mut pc: u32 = 0x8323FC34;
    'dispatch: loop {
        match pc {
            0x8323FC34 => {
    //   block [0x8323FC34..0x8323FC44)
	// 8323FC34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8323FC38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8323FC3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8323FC40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FC44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FC44 size=4
    let mut pc: u32 = 0x8323FC44;
    'dispatch: loop {
        match pc {
            0x8323FC44 => {
    //   block [0x8323FC44..0x8323FC48)
	// 8323FC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FC48 size=88
    let mut pc: u32 = 0x8323FC48;
    'dispatch: loop {
        match pc {
            0x8323FC48 => {
    //   block [0x8323FC48..0x8323FCA0)
	// 8323FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FC50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323FC54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FC5C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FC60: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8323FC64: 396BB558  addi r11, r11, -0x4aa8
	ctx.r[11].s64 = ctx.r[11].s64 + -19112;
	// 8323FC68: 3BEB0060  addi r31, r11, 0x60
	ctx.r[31].s64 = ctx.r[11].s64 + 96;
	// 8323FC6C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 8323FC70: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8323FC74: 4BBB37B5  bl 0x82df3428
	ctx.lr = 0x8323FC78;
	sub_82DF3428(ctx, base);
	// 8323FC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FC7C: 4BBB37AD  bl 0x82df3428
	ctx.lr = 0x8323FC80;
	sub_82DF3428(ctx, base);
	// 8323FC80: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323FC84: 4080FFE8  bge 0x8323fc6c
	if !ctx.cr[0].lt {
	pc = 0x8323FC6C; continue 'dispatch;
	}
	// 8323FC88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FC94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FC98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FCA0 size=80
    let mut pc: u32 = 0x8323FCA0;
    'dispatch: loop {
        match pc {
            0x8323FCA0 => {
    //   block [0x8323FCA0..0x8323FCF0)
	// 8323FCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FCA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323FCAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FCB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FCB4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FCB8: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8323FCBC: 396BB68C  addi r11, r11, -0x4974
	ctx.r[11].s64 = ctx.r[11].s64 + -18804;
	// 8323FCC0: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 8323FCC4: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 8323FCC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FCCC: 4BBB375D  bl 0x82df3428
	ctx.lr = 0x8323FCD0;
	sub_82DF3428(ctx, base);
	// 8323FCD0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323FCD4: 4080FFF0  bge 0x8323fcc4
	if !ctx.cr[0].lt {
	pc = 0x8323FCC4; continue 'dispatch;
	}
	// 8323FCD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FCE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FCE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FCF0 size=80
    let mut pc: u32 = 0x8323FCF0;
    'dispatch: loop {
        match pc {
            0x8323FCF0 => {
    //   block [0x8323FCF0..0x8323FD40)
	// 8323FCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FCF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323FCFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FD00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FD04: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FD08: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8323FD0C: 396BB698  addi r11, r11, -0x4968
	ctx.r[11].s64 = ctx.r[11].s64 + -18792;
	// 8323FD10: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 8323FD14: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 8323FD18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FD1C: 4BBB370D  bl 0x82df3428
	ctx.lr = 0x8323FD20;
	sub_82DF3428(ctx, base);
	// 8323FD20: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323FD24: 4080FFF0  bge 0x8323fd14
	if !ctx.cr[0].lt {
	pc = 0x8323FD14; continue 'dispatch;
	}
	// 8323FD28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FD2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FD30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FD34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FD38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FD40 size=60
    let mut pc: u32 = 0x8323FD40;
    'dispatch: loop {
        match pc {
            0x8323FD40 => {
    //   block [0x8323FD40..0x8323FD7C)
	// 8323FD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FD48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FD4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FD50: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FD54: 3BEBB738  addi r31, r11, -0x48c8
	ctx.r[31].s64 = ctx.r[11].s64 + -18632;
	// 8323FD58: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8323FD5C: 4BBB36CD  bl 0x82df3428
	ctx.lr = 0x8323FD60;
	sub_82DF3428(ctx, base);
	// 8323FD60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FD64: 4BBB36C5  bl 0x82df3428
	ctx.lr = 0x8323FD68;
	sub_82DF3428(ctx, base);
	// 8323FD68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323FD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8323FD80 size=12
    let mut pc: u32 = 0x8323FD80;
    'dispatch: loop {
        match pc {
            0x8323FD80 => {
    //   block [0x8323FD80..0x8323FD8C)
	// 8323FD80: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FD84: 386BB7E8  addi r3, r11, -0x4818
	ctx.r[3].s64 = ctx.r[11].s64 + -18456;
	// 8323FD88: 4BBB36A0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FD90 size=96
    let mut pc: u32 = 0x8323FD90;
    'dispatch: loop {
        match pc {
            0x8323FD90 => {
    //   block [0x8323FD90..0x8323FDF0)
	// 8323FD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FD98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323FD9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FDA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FDA4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FDA8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 8323FDAC: 396BB854  addi r11, r11, -0x47ac
	ctx.r[11].s64 = ctx.r[11].s64 + -18348;
	// 8323FDB0: 3BEB0034  addi r31, r11, 0x34
	ctx.r[31].s64 = ctx.r[11].s64 + 52;
	// 8323FDB4: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 8323FDB8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8323FDBC: 4BBB366D  bl 0x82df3428
	ctx.lr = 0x8323FDC0;
	sub_82DF3428(ctx, base);
	// 8323FDC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FDC4: 4BBB3665  bl 0x82df3428
	ctx.lr = 0x8323FDC8;
	sub_82DF3428(ctx, base);
	// 8323FDC8: 387FFFFC  addi r3, r31, -4
	ctx.r[3].s64 = ctx.r[31].s64 + -4;
	// 8323FDCC: 4BBB365D  bl 0x82df3428
	ctx.lr = 0x8323FDD0;
	sub_82DF3428(ctx, base);
	// 8323FDD0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323FDD4: 4080FFE0  bge 0x8323fdb4
	if !ctx.cr[0].lt {
	pc = 0x8323FDB4; continue 'dispatch;
	}
	// 8323FDD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FDE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FDE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FDF0 size=80
    let mut pc: u32 = 0x8323FDF0;
    'dispatch: loop {
        match pc {
            0x8323FDF0 => {
    //   block [0x8323FDF0..0x8323FE40)
	// 8323FDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FDF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323FDFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FE00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FE04: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FE08: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 8323FE0C: 396BB8D8  addi r11, r11, -0x4728
	ctx.r[11].s64 = ctx.r[11].s64 + -18216;
	// 8323FE10: 3BEB0148  addi r31, r11, 0x148
	ctx.r[31].s64 = ctx.r[11].s64 + 328;
	// 8323FE14: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8323FE18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FE1C: 4B088E9D  bl 0x822c8cb8
	ctx.lr = 0x8323FE20;
	sub_822C8CB8(ctx, base);
	// 8323FE20: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323FE24: 4080FFF0  bge 0x8323fe14
	if !ctx.cr[0].lt {
	pc = 0x8323FE14; continue 'dispatch;
	}
	// 8323FE28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FE34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FE38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FE40 size=80
    let mut pc: u32 = 0x8323FE40;
    'dispatch: loop {
        match pc {
            0x8323FE40 => {
    //   block [0x8323FE40..0x8323FE90)
	// 8323FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FE48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323FE4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FE50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FE54: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FE58: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8323FE5C: 396BCAF0  addi r11, r11, -0x3510
	ctx.r[11].s64 = ctx.r[11].s64 + -13584;
	// 8323FE60: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 8323FE64: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 8323FE68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FE6C: 4BBB35BD  bl 0x82df3428
	ctx.lr = 0x8323FE70;
	sub_82DF3428(ctx, base);
	// 8323FE70: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323FE74: 4080FFF0  bge 0x8323fe64
	if !ctx.cr[0].lt {
	pc = 0x8323FE64; continue 'dispatch;
	}
	// 8323FE78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FE84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FE88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FE90 size=60
    let mut pc: u32 = 0x8323FE90;
    'dispatch: loop {
        match pc {
            0x8323FE90 => {
    //   block [0x8323FE90..0x8323FECC)
	// 8323FE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FE98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FE9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FEA0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FEA4: 3BEBE3D8  addi r31, r11, -0x1c28
	ctx.r[31].s64 = ctx.r[11].s64 + -7208;
	// 8323FEA8: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8323FEAC: 4BBB357D  bl 0x82df3428
	ctx.lr = 0x8323FEB0;
	sub_82DF3428(ctx, base);
	// 8323FEB0: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8323FEB4: 4BBB3575  bl 0x82df3428
	ctx.lr = 0x8323FEB8;
	sub_82DF3428(ctx, base);
	// 8323FEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323FEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FEC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FED0 size=60
    let mut pc: u32 = 0x8323FED0;
    'dispatch: loop {
        match pc {
            0x8323FED0 => {
    //   block [0x8323FED0..0x8323FF0C)
	// 8323FED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FEDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FEE0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FEE4: 3BEBCB68  addi r31, r11, -0x3498
	ctx.r[31].s64 = ctx.r[11].s64 + -13464;
	// 8323FEE8: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8323FEEC: 4BBB353D  bl 0x82df3428
	ctx.lr = 0x8323FEF0;
	sub_82DF3428(ctx, base);
	// 8323FEF0: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8323FEF4: 4BBB3535  bl 0x82df3428
	ctx.lr = 0x8323FEF8;
	sub_82DF3428(ctx, base);
	// 8323FEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FF04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FF10 size=80
    let mut pc: u32 = 0x8323FF10;
    'dispatch: loop {
        match pc {
            0x8323FF10 => {
    //   block [0x8323FF10..0x8323FF60)
	// 8323FF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FF18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8323FF1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FF20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FF24: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FF28: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8323FF2C: 396BFC48  addi r11, r11, -0x3b8
	ctx.r[11].s64 = ctx.r[11].s64 + -952;
	// 8323FF30: 3BEB002C  addi r31, r11, 0x2c
	ctx.r[31].s64 = ctx.r[11].s64 + 44;
	// 8323FF34: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 8323FF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8323FF3C: 4BBB34ED  bl 0x82df3428
	ctx.lr = 0x8323FF40;
	sub_82DF3428(ctx, base);
	// 8323FF40: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8323FF44: 4080FFF0  bge 0x8323ff34
	if !ctx.cr[0].lt {
	pc = 0x8323FF34; continue 'dispatch;
	}
	// 8323FF48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8323FF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FF54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8323FF58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FF60 size=60
    let mut pc: u32 = 0x8323FF60;
    'dispatch: loop {
        match pc {
            0x8323FF60 => {
    //   block [0x8323FF60..0x8323FF9C)
	// 8323FF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FF68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FF6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FF70: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FF74: 3BEB1370  addi r31, r11, 0x1370
	ctx.r[31].s64 = ctx.r[11].s64 + 4976;
	// 8323FF78: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8323FF7C: 4BBB34AD  bl 0x82df3428
	ctx.lr = 0x8323FF80;
	sub_82DF3428(ctx, base);
	// 8323FF80: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8323FF84: 4BBB34A5  bl 0x82df3428
	ctx.lr = 0x8323FF88;
	sub_82DF3428(ctx, base);
	// 8323FF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323FF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FF94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FFA0 size=60
    let mut pc: u32 = 0x8323FFA0;
    'dispatch: loop {
        match pc {
            0x8323FFA0 => {
    //   block [0x8323FFA0..0x8323FFDC)
	// 8323FFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FFA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FFAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FFB0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FFB4: 3BEB2A08  addi r31, r11, 0x2a08
	ctx.r[31].s64 = ctx.r[11].s64 + 10760;
	// 8323FFB8: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8323FFBC: 4BBB346D  bl 0x82df3428
	ctx.lr = 0x8323FFC0;
	sub_82DF3428(ctx, base);
	// 8323FFC0: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8323FFC4: 4BBB3465  bl 0x82df3428
	ctx.lr = 0x8323FFC8;
	sub_82DF3428(ctx, base);
	// 8323FFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8323FFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8323FFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8323FFD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8323FFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8323FFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8323FFE0 size=60
    let mut pc: u32 = 0x8323FFE0;
    'dispatch: loop {
        match pc {
            0x8323FFE0 => {
    //   block [0x8323FFE0..0x8324001C)
	// 8323FFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8323FFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8323FFE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8323FFEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8323FFF0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8323FFF4: 3BEB40A0  addi r31, r11, 0x40a0
	ctx.r[31].s64 = ctx.r[11].s64 + 16544;
	// 8323FFF8: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8323FFFC: 4BBB342D  bl 0x82df3428
	ctx.lr = 0x83240000;
	sub_82DF3428(ctx, base);
	// 83240000: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83240004: 4BBB3425  bl 0x82df3428
	ctx.lr = 0x83240008;
	sub_82DF3428(ctx, base);
	// 83240008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324000C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240020 size=60
    let mut pc: u32 = 0x83240020;
    'dispatch: loop {
        match pc {
            0x83240020 => {
    //   block [0x83240020..0x8324005C)
	// 83240020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324002C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240030: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83240034: 3BEB5738  addi r31, r11, 0x5738
	ctx.r[31].s64 = ctx.r[11].s64 + 22328;
	// 83240038: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8324003C: 4BBB33ED  bl 0x82df3428
	ctx.lr = 0x83240040;
	sub_82DF3428(ctx, base);
	// 83240040: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83240044: 4BBB33E5  bl 0x82df3428
	ctx.lr = 0x83240048;
	sub_82DF3428(ctx, base);
	// 83240048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324004C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240054: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240060 size=60
    let mut pc: u32 = 0x83240060;
    'dispatch: loop {
        match pc {
            0x83240060 => {
    //   block [0x83240060..0x8324009C)
	// 83240060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324006C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240070: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83240074: 3BEB6DD0  addi r31, r11, 0x6dd0
	ctx.r[31].s64 = ctx.r[11].s64 + 28112;
	// 83240078: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8324007C: 4BBB33AD  bl 0x82df3428
	ctx.lr = 0x83240080;
	sub_82DF3428(ctx, base);
	// 83240080: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83240084: 4BBB33A5  bl 0x82df3428
	ctx.lr = 0x83240088;
	sub_82DF3428(ctx, base);
	// 83240088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324008C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


