pub fn sub_82D5B350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5B350 size=60
    let mut pc: u32 = 0x82D5B350;
    'dispatch: loop {
        match pc {
            0x82D5B350 => {
    //   block [0x82D5B350..0x82D5B38C)
	// 82D5B350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5B354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5B358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5B35C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5B360: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5B364: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5B368: 4BFFFE41  bl 0x82d5b1a8
	ctx.lr = 0x82D5B36C;
	sub_82D5B1A8(ctx, base);
	// 82D5B36C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5B370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5B374: 4BFFBDBD  bl 0x82d57130
	ctx.lr = 0x82D5B378;
	sub_82D57130(ctx, base);
	// 82D5B378: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5B37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5B380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5B384: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5B388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5B390 size=136
    let mut pc: u32 = 0x82D5B390;
    'dispatch: loop {
        match pc {
            0x82D5B390 => {
    //   block [0x82D5B390..0x82D5B418)
	// 82D5B390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5B394: 4BF4E06D  bl 0x82ca9400
	ctx.lr = 0x82D5B398;
	sub_82CA93D0(ctx, base);
	// 82D5B398: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5B39C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5B3A0: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D5B3A4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5B3A8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82D5B3AC: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D5B3B0: 7C9BF1D6  mullw r4, r27, r30
	ctx.r[4].s64 = (ctx.r[27].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D5B3B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5B3B8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5B3BC: 4BFF9E8D  bl 0x82d55248
	ctx.lr = 0x82D5B3C0;
	sub_82D55248(ctx, base);
	// 82D5B3C0: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5B3C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D5B3C8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5B3CC: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D5B3D0: 4BF4E0B1  bl 0x82ca9480
	ctx.lr = 0x82D5B3D4;
	sub_82CA9480(ctx, base);
	// 82D5B3D4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5B3D8: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5B3DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5B3E0: 409A001C  bne cr6, 0x82d5b3fc
	if !ctx.cr[6].eq {
	pc = 0x82D5B3FC; continue 'dispatch;
	}
	// 82D5B3E4: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82D5B3E8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5B3EC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5B3F0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5B3F4: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D5B3F8: 4BFF9ED1  bl 0x82d552c8
	ctx.lr = 0x82D5B3FC;
	sub_82D552C8(ctx, base);
	// 82D5B3FC: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5B400: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D5B404: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5B408: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 82D5B40C: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5B410: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5B414: 4BF4E03C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5B418 size=152
    let mut pc: u32 = 0x82D5B418;
    'dispatch: loop {
        match pc {
            0x82D5B418 => {
    //   block [0x82D5B418..0x82D5B4B0)
	// 82D5B418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5B41C: 4BF4DFE5  bl 0x82ca9400
	ctx.lr = 0x82D5B420;
	sub_82CA93D0(ctx, base);
	// 82D5B420: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5B424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5B428: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5B42C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5B430: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5B434: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82D5B438: 409A0008  bne cr6, 0x82d5b440
	if !ctx.cr[6].eq {
	pc = 0x82D5B440; continue 'dispatch;
	}
	// 82D5B43C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82D5B440: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5B444: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D5B448: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D5B44C: 7C9AF1D6  mullw r4, r26, r30
	ctx.r[4].s64 = (ctx.r[26].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D5B450: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5B454: 4BFF9DF5  bl 0x82d55248
	ctx.lr = 0x82D5B458;
	sub_82D55248(ctx, base);
	// 82D5B458: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5B45C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5B460: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D5B464: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D5B468: 4BF4E019  bl 0x82ca9480
	ctx.lr = 0x82D5B46C;
	sub_82CA9480(ctx, base);
	// 82D5B46C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5B470: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5B474: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5B478: 409A001C  bne cr6, 0x82d5b494
	if !ctx.cr[6].eq {
	pc = 0x82D5B494; continue 'dispatch;
	}
	// 82D5B47C: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82D5B480: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5B484: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5B488: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5B48C: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D5B490: 4BFF9E39  bl 0x82d552c8
	ctx.lr = 0x82D5B494;
	sub_82D552C8(ctx, base);
	// 82D5B494: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5B498: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82D5B49C: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5B4A0: 7D6BD378  or r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[26].u64;
	// 82D5B4A4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5B4A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5B4AC: 4BF4DFA4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5B4B0 size=64
    let mut pc: u32 = 0x82D5B4B0;
    'dispatch: loop {
        match pc {
            0x82D5B4B0 => {
    //   block [0x82D5B4B0..0x82D5B4F0)
	// 82D5B4B0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D5B4B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5B4B8: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82D5B4BC: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B4C0: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5B4C4: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D5B4C8: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5B4CC: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D5B4D0: 419A0020  beq cr6, 0x82d5b4f0
	if ctx.cr[6].eq {
		sub_82D5B4F0(ctx, base);
		return;
	}
	// 82D5B4D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D5B4D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D5B4DC: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82D5B4E0: 4198FFDC  blt cr6, 0x82d5b4bc
	if ctx.cr[6].lt {
	pc = 0x82D5B4BC; continue 'dispatch;
	}
	// 82D5B4E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5B4E8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5B4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5B4F0 size=12
    let mut pc: u32 = 0x82D5B4F0;
    'dispatch: loop {
        match pc {
            0x82D5B4F0 => {
    //   block [0x82D5B4F0..0x82D5B4FC)
	// 82D5B4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5B4F4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5B4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5B500 size=100
    let mut pc: u32 = 0x82D5B500;
    'dispatch: loop {
        match pc {
            0x82D5B500 => {
    //   block [0x82D5B500..0x82D5B564)
	// 82D5B500: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B504: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5B508: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5B50C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5B510: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B514: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5B518: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5B51C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5B520: C0030030  lfs f0, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B524: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5B528: D1A30030  stfs f13, 0x30(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D5B52C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5B530: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B534: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5B538: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5B53C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5B540: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B544: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5B548: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82D5B54C: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5B550: C0030038  lfs f0, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B554: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5B558: D1A30038  stfs f13, 0x38(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82D5B55C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5B560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5B568 size=132
    let mut pc: u32 = 0x82D5B568;
    'dispatch: loop {
        match pc {
            0x82D5B568 => {
    //   block [0x82D5B568..0x82D5B5EC)
	// 82D5B568: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B56C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5B570: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B574: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82D5B578: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B57C: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82D5B580: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B584: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82D5B588: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B58C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5B590: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B594: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5B598: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B59C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5B5A0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5A4: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5B5A8: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5AC: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5B5B0: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5B4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D5B5B8: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5BC: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5B5C0: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5C4: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5B5C8: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5CC: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5B5D0: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5D4: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82D5B5D8: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5DC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5B5E0: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B5E4: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82D5B5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5B5F0 size=80
    let mut pc: u32 = 0x82D5B5F0;
    'dispatch: loop {
        match pc {
            0x82D5B5F0 => {
    //   block [0x82D5B5F0..0x82D5B640)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5B640 size=300
    let mut pc: u32 = 0x82D5B640;
    'dispatch: loop {
        match pc {
            0x82D5B640 => {
    //   block [0x82D5B640..0x82D5B76C)
	// 82D5B640: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5B770 size=704
    let mut pc: u32 = 0x82D5B770;
    'dispatch: loop {
        match pc {
            0x82D5B770 => {
    //   block [0x82D5B770..0x82D5BA30)
	// 82D5B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5B778: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 82D5B77C: 4BF52525  bl 0x82cadca0
	ctx.lr = 0x82D5B780;
	sub_82CADCA0(ctx, base);
	// 82D5B780: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5B784: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D5B788: 38C1FF00  addi r6, r1, -0x100
	ctx.r[6].s64 = ctx.r[1].s64 + -256;
	// 82D5B78C: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82D5B790: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82D5B794: C16B0004  lfs f11, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D5B798: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 82D5B79C: C12B0020  lfs f9, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D5B7A0: C10B001C  lfs f8, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82D5B7A4: EF6902F2  fmuls f27, f9, f11
	ctx.f[27].f64 = (((ctx.f[9].f64 * ctx.f[11].f64) as f32) as f64);
	// 82D5B7A8: C06B0038  lfs f3, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82D5B7AC: EF430232  fmuls f26, f3, f8
	ctx.f[26].f64 = (((ctx.f[3].f64 * ctx.f[8].f64) as f32) as f64);
	// 82D5B7B0: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B7B4: C3EB000C  lfs f31, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82D5B7B8: EF090032  fmuls f24, f9, f0
	ctx.f[24].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5B7BC: C3AB0030  lfs f29, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82D5B7C0: EEE307F2  fmuls f23, f3, f31
	ctx.f[23].f64 = (((ctx.f[3].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D5B7C4: C0AB0028  lfs f5, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82D5B7C8: EEDD02F2  fmuls f22, f29, f11
	ctx.f[22].f64 = (((ctx.f[29].f64 * ctx.f[11].f64) as f32) as f64);
	// 82D5B7CC: EEA50232  fmuls f21, f5, f8
	ctx.f[21].f64 = (((ctx.f[5].f64 * ctx.f[8].f64) as f32) as f64);
	// 82D5B7D0: C14B0024  lfs f10, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D5B7D4: EE9D0032  fmuls f20, f29, f0
	ctx.f[20].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5B7D8: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5B7DC: C18B0010  lfs f12, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D5B7E0: EE6507F2  fmuls f19, f5, f31
	ctx.f[19].f64 = (((ctx.f[5].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D5B7E4: C0EB0018  lfs f7, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82D5B7E8: EE1D02B2  fmuls f16, f29, f10
	ctx.f[16].f64 = (((ctx.f[29].f64 * ctx.f[10].f64) as f32) as f64);
	// 82D5B7EC: C0CB002C  lfs f6, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82D5B7F0: EF8B0332  fmuls f28, f11, f12
	ctx.f[28].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 82D5B7F4: C08B003C  lfs f4, 0x3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82D5B7F8: EF2301B2  fmuls f25, f3, f6
	ctx.f[25].f64 = (((ctx.f[3].f64 * ctx.f[6].f64) as f32) as f64);
	// 82D5B7FC: EF6ADB78  fmsubs f27, f10, f13, f27
	ctx.f[27].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 - ctx.f[27].f64) as f32) as f64);
	// 82D5B800: C04B0008  lfs f2, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82D5B804: EF44D1F8  fmsubs f26, f4, f7, f26
	ctx.f[26].f64 = (((ctx.f[4].f64 * ctx.f[7].f64 - ctx.f[26].f64) as f32) as f64);
	// 82D5B808: C3CB0034  lfs f30, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82D5B80C: EE4AC338  fmsubs f18, f10, f12, f24
	ctx.f[18].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 - ctx.f[24].f64) as f32) as f64);
	// 82D5B810: D241FF18  stfs f18, -0xe8(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-232 as u32), tmp.u32 ) };
	// 82D5B814: EF04B8B8  fmsubs f24, f4, f2, f23
	ctx.f[24].f64 = (((ctx.f[4].f64 * ctx.f[2].f64 - ctx.f[23].f64) as f32) as f64);
	// 82D5B818: EEFEB378  fmsubs f23, f30, f13, f22
	ctx.f[23].f64 = (((ctx.f[30].f64 * ctx.f[13].f64 - ctx.f[22].f64) as f32) as f64);
	// 82D5B81C: D2E1FF10  stfs f23, -0xf0(r1)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-240 as u32), tmp.u32 ) };
	// 82D5B820: EEC6A9F8  fmsubs f22, f6, f7, f21
	ctx.f[22].f64 = (((ctx.f[6].f64 * ctx.f[7].f64 - ctx.f[21].f64) as f32) as f64);
	// 82D5B824: D2C1FF08  stfs f22, -0xf8(r1)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-248 as u32), tmp.u32 ) };
	// 82D5B828: EEBEA338  fmsubs f21, f30, f12, f20
	ctx.f[21].f64 = (((ctx.f[30].f64 * ctx.f[12].f64 - ctx.f[20].f64) as f32) as f64);
	// 82D5B82C: D2A1FF14  stfs f21, -0xec(r1)
	tmp.f32 = (ctx.f[21].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-236 as u32), tmp.u32 ) };
	// 82D5B830: EE8698B8  fmsubs f20, f6, f2, f19
	ctx.f[20].f64 = (((ctx.f[6].f64 * ctx.f[2].f64 - ctx.f[19].f64) as f32) as f64);
	// 82D5B834: D281FF04  stfs f20, -0xfc(r1)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-252 as u32), tmp.u32 ) };
	// 82D5B838: EE7E8278  fmsubs f19, f30, f9, f16
	ctx.f[19].f64 = (((ctx.f[30].f64 * ctx.f[9].f64 - ctx.f[16].f64) as f32) as f64);
	// 82D5B83C: D261FF0C  stfs f19, -0xf4(r1)
	tmp.f32 = (ctx.f[19].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-244 as u32), tmp.u32 ) };
	// 82D5B840: EF8DE038  fmsubs f28, f13, f0, f28
	ctx.f[28].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[28].f64) as f32) as f64);
	// 82D5B844: EF24C978  fmsubs f25, f4, f5, f25
	ctx.f[25].f64 = (((ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[25].f64) as f32) as f64);
	// 82D5B848: EE1A06F2  fmuls f16, f26, f27
	ctx.f[16].f64 = (((ctx.f[26].f64 * ctx.f[27].f64) as f32) as f64);
	// 82D5B84C: EE3F01F2  fmuls f17, f31, f7
	ctx.f[17].f64 = (((ctx.f[31].f64 * ctx.f[7].f64) as f32) as f64);
	// 82D5B850: EE198738  fmsubs f16, f25, f28, f16
	ctx.f[16].f64 = (((ctx.f[25].f64 * ctx.f[28].f64 - ctx.f[16].f64) as f32) as f64);
	// 82D5B854: EE228A38  fmsubs f17, f2, f8, f17
	ctx.f[17].f64 = (((ctx.f[2].f64 * ctx.f[8].f64 - ctx.f[17].f64) as f32) as f64);
	// 82D5B858: D221FF1C  stfs f17, -0xe4(r1)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-228 as u32), tmp.u32 ) };
	// 82D5B85C: EE5884BA  fmadds f18, f24, f18, f16
	ctx.f[18].f64 = (((ctx.f[24].f64 * ctx.f[18].f64 + ctx.f[16].f64) as f32) as f64);
	// 82D5B860: EE5695FA  fmadds f18, f22, f23, f18
	ctx.f[18].f64 = (((ctx.f[22].f64 * ctx.f[23].f64 + ctx.f[18].f64) as f32) as f64);
	// 82D5B864: EE54957C  fnmsubs f18, f20, f21, f18
	ctx.f[18].f64 = -(((ctx.f[20].f64 * ctx.f[21].f64 - ctx.f[18].f64) as f32) as f64);
	// 82D5B868: EE5194FA  fmadds f18, f17, f19, f18
	ctx.f[18].f64 = (((ctx.f[17].f64 * ctx.f[19].f64 + ctx.f[18].f64) as f32) as f64);
	// 82D5B86C: C22A0C14  lfs f17, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[17].f64 = (tmp.f32 as f64);
	// 82D5B870: EE319024  fdivs f17, f17, f18
	ctx.f[17].f64 = ((ctx.f[17].f64 / ctx.f[18].f64) as f32) as f64;
	// 82D5B874: D221FF00  stfs f17, -0x100(r1)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-256 as u32), tmp.u32 ) };
	// 82D5B878: FE409210  fabs f18, f18
	ctx.f[18].u64 = ctx.f[18].u64 & !0x8000_0000_0000_0000u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5BA30 size=92
    let mut pc: u32 = 0x82D5BA30;
    'dispatch: loop {
        match pc {
            0x82D5BA30 => {
    //   block [0x82D5BA30..0x82D5BA8C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5BA90 size=92
    let mut pc: u32 = 0x82D5BA90;
    'dispatch: loop {
        match pc {
            0x82D5BA90 => {
    //   block [0x82D5BA90..0x82D5BAEC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5BAF0 size=80
    let mut pc: u32 = 0x82D5BAF0;
    'dispatch: loop {
        match pc {
            0x82D5BAF0 => {
    //   block [0x82D5BAF0..0x82D5BB40)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5BB40 size=160
    let mut pc: u32 = 0x82D5BB40;
    'dispatch: loop {
        match pc {
            0x82D5BB40 => {
    //   block [0x82D5BB40..0x82D5BBE0)
	// 82D5BB40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5BB44: C003000C  lfs f0, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BB48: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82D5BB4C: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 82D5BB50: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5BB54: 119F038C  vspltisw v12, -1
	for i in 0..4 {
		ctx.v[12].u32[i] = 4294967295;
	}
	// 82D5BB58: D1A1FFE4  stfs f13, -0x1c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82D5BB5C: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5BB60: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BB64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5BB68: D1A1FFE8  stfs f13, -0x18(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82D5BB6C: C1A3003C  lfs f13, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5BB70: D1A1FFEC  stfs f13, -0x14(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82D5BB74: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82D5BB78: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5BB7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5BB80: D001FFD4  stfs f0, -0x2c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82D5BB84: 396B14A0  addi r11, r11, 0x14a0
	ctx.r[11].s64 = ctx.r[11].s64 + 5280;
	// 82D5BB88: D001FFD8  stfs f0, -0x28(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82D5BB8C: D1A1FFDC  stfs f13, -0x24(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82D5BB90: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5BBE0 size=36
    let mut pc: u32 = 0x82D5BBE0;
    'dispatch: loop {
        match pc {
            0x82D5BBE0 => {
    //   block [0x82D5BBE0..0x82D5BC04)
	// 82D5BBE0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5BBE4: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BBE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5BBEC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5BBF0: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5BBF4: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5BBF8: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BBFC: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82D5BC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5BC08 size=164
    let mut pc: u32 = 0x82D5BC08;
    'dispatch: loop {
        match pc {
            0x82D5BC08 => {
    //   block [0x82D5BC08..0x82D5BCAC)
	// 82D5BC08: C0040008  lfs f0, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BC0C: C1430020  lfs f10, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D5BC10: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5BC14: C1A4000C  lfs f13, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5BC18: C1230030  lfs f9, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D5BC1C: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D5BC20: C1030010  lfs f8, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82D5BC24: C1640000  lfs f11, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D5BC28: C0E30000  lfs f7, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82D5BC2C: ED49537A  fmadds f10, f9, f13, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC30: ED48533A  fmadds f10, f8, f12, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC34: ED4752FA  fmadds f10, f7, f11, f10
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC38: D1450000  stfs f10, 0(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5BC3C: C1430024  lfs f10, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D5BC40: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5BC44: C1230034  lfs f9, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D5BC48: C1030014  lfs f8, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82D5BC4C: C0E30004  lfs f7, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82D5BC50: ED49537A  fmadds f10, f9, f13, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC54: ED48533A  fmadds f10, f8, f12, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC58: ED4752FA  fmadds f10, f7, f11, f10
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC5C: D1450004  stfs f10, 4(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5BC60: C1430028  lfs f10, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D5BC64: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5BC68: C1230038  lfs f9, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D5BC6C: C1030018  lfs f8, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82D5BC70: C0E30008  lfs f7, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82D5BC74: ED49537A  fmadds f10, f9, f13, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC78: ED48533A  fmadds f10, f8, f12, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC7C: ED4752FA  fmadds f10, f7, f11, f10
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D5BC80: D1450008  stfs f10, 8(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5BC84: C143002C  lfs f10, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D5BC88: EC0A0032  fmuls f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5BC8C: C143003C  lfs f10, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D5BC90: C123001C  lfs f9, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D5BC94: C103000C  lfs f8, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82D5BC98: EC0A037A  fmadds f0, f10, f13, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D5BC9C: EC09033A  fmadds f0, f9, f12, f0
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D5BCA0: EC0802FA  fmadds f0, f8, f11, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D5BCA4: D005000C  stfs f0, 0xc(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5BCA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5BCB0 size=132
    let mut pc: u32 = 0x82D5BCB0;
    'dispatch: loop {
        match pc {
            0x82D5BCB0 => {
    //   block [0x82D5BCB0..0x82D5BD34)
	// 82D5BCB0: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCB4: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5BCB8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCBC: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5BCC0: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCC4: D0040020  stfs f0, 0x20(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5BCC8: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCCC: D0040030  stfs f0, 0x30(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D5BCD0: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCD4: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5BCD8: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCDC: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82D5BCE0: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCE4: D0040024  stfs f0, 0x24(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5BCE8: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCEC: D0040034  stfs f0, 0x34(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82D5BCF0: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCF4: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5BCF8: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BCFC: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5BD00: C0030028  lfs f0, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD04: D0040028  stfs f0, 0x28(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82D5BD08: C003002C  lfs f0, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD0C: D0040038  stfs f0, 0x38(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82D5BD10: C0030030  lfs f0, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD14: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5BD18: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD1C: D004001C  stfs f0, 0x1c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5BD20: C0030038  lfs f0, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD24: D004002C  stfs f0, 0x2c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5BD28: C003003C  lfs f0, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD2C: D004003C  stfs f0, 0x3c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82D5BD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5BD38 size=132
    let mut pc: u32 = 0x82D5BD38;
    'dispatch: loop {
        match pc {
            0x82D5BD38 => {
    //   block [0x82D5BD38..0x82D5BDBC)
	// 82D5BD38: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD3C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5BD40: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD44: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5BD48: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD4C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5BD50: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD54: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5BD58: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD5C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5BD60: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD64: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82D5BD68: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD6C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5BD70: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD74: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5BD78: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD7C: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5BD80: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD84: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5BD88: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD8C: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82D5BD90: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD94: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5BD98: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BD9C: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D5BDA0: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDA4: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82D5BDA8: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDAC: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82D5BDB0: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDB4: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82D5BDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5BDC0 size=132
    let mut pc: u32 = 0x82D5BDC0;
    'dispatch: loop {
        match pc {
            0x82D5BDC0 => {
    //   block [0x82D5BDC0..0x82D5BE44)
	// 82D5BDC0: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDC4: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5BDC8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDCC: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5BDD0: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDD4: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5BDD8: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDDC: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5BDE0: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDE4: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5BDE8: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDEC: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82D5BDF0: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDF4: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5BDF8: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BDFC: D004001C  stfs f0, 0x1c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5BE00: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE04: D0040020  stfs f0, 0x20(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5BE08: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE0C: D0040024  stfs f0, 0x24(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5BE10: C0030028  lfs f0, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE14: D0040028  stfs f0, 0x28(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82D5BE18: C003002C  lfs f0, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE1C: D004002C  stfs f0, 0x2c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5BE20: C0030030  lfs f0, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE24: D0040030  stfs f0, 0x30(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D5BE28: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE2C: D0040034  stfs f0, 0x34(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82D5BE30: C0030038  lfs f0, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE34: D0040038  stfs f0, 0x38(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82D5BE38: C003003C  lfs f0, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE3C: D004003C  stfs f0, 0x3c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82D5BE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5BE48 size=132
    let mut pc: u32 = 0x82D5BE48;
    'dispatch: loop {
        match pc {
            0x82D5BE48 => {
    //   block [0x82D5BE48..0x82D5BECC)
	// 82D5BE48: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE4C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5BE50: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE54: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5BE58: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE5C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5BE60: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE64: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5BE68: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE6C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5BE70: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE74: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82D5BE78: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE7C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5BE80: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE84: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5BE88: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE8C: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5BE90: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE94: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5BE98: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BE9C: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82D5BEA0: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BEA4: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5BEA8: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BEAC: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D5BEB0: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BEB4: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82D5BEB8: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BEBC: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82D5BEC0: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5BEC4: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82D5BEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5BED0 size=80
    let mut pc: u32 = 0x82D5BED0;
    'dispatch: loop {
        match pc {
            0x82D5BED0 => {
    //   block [0x82D5BED0..0x82D5BF20)
	// 82D5BED0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5BF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5BF20 size=244
    let mut pc: u32 = 0x82D5BF20;
    'dispatch: loop {
        match pc {
            0x82D5BF20 => {
    //   block [0x82D5BF20..0x82D5C014)
	// 82D5BF20: 3901FFC0  addi r8, r1, -0x40
	ctx.r[8].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C018 size=128
    let mut pc: u32 = 0x82D5C018;
    'dispatch: loop {
        match pc {
            0x82D5C018 => {
    //   block [0x82D5C018..0x82D5C098)
	// 82D5C018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5C020: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5C024: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C028: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C098 size=116
    let mut pc: u32 = 0x82D5C098;
    'dispatch: loop {
        match pc {
            0x82D5C098 => {
    //   block [0x82D5C098..0x82D5C10C)
	// 82D5C098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5C0A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5C0A4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C0A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5C0AC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D5C0B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C0B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5C0B8: 4BFFFE69  bl 0x82d5bf20
	ctx.lr = 0x82D5C0BC;
	sub_82D5BF20(ctx, base);
	// 82D5C0BC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82D5C0C0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82D5C0C4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82D5C0C8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5C110 size=172
    let mut pc: u32 = 0x82D5C110;
    'dispatch: loop {
        match pc {
            0x82D5C110 => {
    //   block [0x82D5C110..0x82D5C1BC)
	// 82D5C110: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5C1C0 size=180
    let mut pc: u32 = 0x82D5C1C0;
    'dispatch: loop {
        match pc {
            0x82D5C1C0 => {
    //   block [0x82D5C1C0..0x82D5C274)
	// 82D5C1C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C278 size=128
    let mut pc: u32 = 0x82D5C278;
    'dispatch: loop {
        match pc {
            0x82D5C278 => {
    //   block [0x82D5C278..0x82D5C2F8)
	// 82D5C278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C27C: 4BF4D189  bl 0x82ca9404
	ctx.lr = 0x82D5C280;
	sub_82CA93D0(ctx, base);
	// 82D5C280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C284: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5C288: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D5C28C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D5C290: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D5C294: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5C298: 419A0058  beq cr6, 0x82d5c2f0
	if ctx.cr[6].eq {
	pc = 0x82D5C2F0; continue 'dispatch;
	}
	// 82D5C29C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5C2A0: 4BFFC989  bl 0x82d58c28
	ctx.lr = 0x82D5C2A4;
	sub_82D58C28(ctx, base);
	// 82D5C2A4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5C2A8: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5C2AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C2B0: 816B7414  lwz r11, 0x7414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29716 as u32) ) } as u64;
	// 82D5C2B4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D5C2B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C2BC: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D5C2C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C2C4: 4E800421  bctrl
	ctx.lr = 0x82D5C2C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C2C8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C2CC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82D5C2D0: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C2D4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82D5C2D8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D5C2DC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82D5C2E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5C2E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C2EC: 4E800421  bctrl
	ctx.lr = 0x82D5C2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5C2F4: 4BF4D160  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C2F8 size=112
    let mut pc: u32 = 0x82D5C2F8;
    'dispatch: loop {
        match pc {
            0x82D5C2F8 => {
    //   block [0x82D5C2F8..0x82D5C368)
	// 82D5C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C2FC: 4BF4D105  bl 0x82ca9400
	ctx.lr = 0x82D5C300;
	sub_82CA93D0(ctx, base);
	// 82D5C300: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C304: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5C308: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5C30C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82D5C310: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5C314: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D5C318: 806B7414  lwz r3, 0x7414(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29716 as u32) ) } as u64;
	// 82D5C31C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82D5C320: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82D5C324: 7C86F9D6  mullw r4, r6, r31
	ctx.r[4].s64 = (ctx.r[6].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82D5C328: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C32C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D5C330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C334: 4E800421  bctrl
	ctx.lr = 0x82D5C338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C338: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C33C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82D5C340: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C344: 7CFBF9D6  mullw r7, r27, r31
	ctx.r[7].s64 = (ctx.r[27].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82D5C348: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5C34C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D5C350: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D5C354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5C358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C35C: 4E800421  bctrl
	ctx.lr = 0x82D5C360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C360: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5C364: 4BF4D0EC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C368 size=212
    let mut pc: u32 = 0x82D5C368;
    'dispatch: loop {
        match pc {
            0x82D5C368 => {
    //   block [0x82D5C368..0x82D5C43C)
	// 82D5C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C36C: 4BF4D091  bl 0x82ca93fc
	ctx.lr = 0x82D5C370;
	sub_82CA93D0(ctx, base);
	// 82D5C370: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C374: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5C378: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D5C37C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D5C380: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82D5C384: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5C388: 419A00AC  beq cr6, 0x82d5c434
	if ctx.cr[6].eq {
	pc = 0x82D5C434; continue 'dispatch;
	}
	// 82D5C38C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C390: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C394: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5C398: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5C39C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C3A0: 4E800421  bctrl
	ctx.lr = 0x82D5C3A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C3A4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D5C3A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C3AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5C3B0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82D5C3B4: 4BFF973D  bl 0x82d55af0
	ctx.lr = 0x82D5C3B8;
	sub_82D55AF0(ctx, base);
	// 82D5C3B8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C3BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5C3C0: 419A0030  beq cr6, 0x82d5c3f0
	if ctx.cr[6].eq {
	pc = 0x82D5C3F0; continue 'dispatch;
	}
	// 82D5C3C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C3C8: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C3CC: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 82D5C3D0: 4BFF9381  bl 0x82d55750
	ctx.lr = 0x82D5C3D4;
	sub_82D55750(ctx, base);
	// 82D5C3D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5C3D8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5C3DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C3E0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D5C3E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D5C3E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C3EC: 4E800421  bctrl
	ctx.lr = 0x82D5C3F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C3F0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D5C3F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D5C3F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C3FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5C400: 48000379  bl 0x82d5c778
	ctx.lr = 0x82D5C404;
	sub_82D5C778(ctx, base);
	// 82D5C404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C408: 4BFF9351  bl 0x82d55758
	ctx.lr = 0x82D5C40C;
	sub_82D55758(ctx, base);
	// 82D5C40C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5C410: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D5C414: 409AFFDC  bne cr6, 0x82d5c3f0
	if !ctx.cr[6].eq {
	pc = 0x82D5C3F0; continue 'dispatch;
	}
	// 82D5C418: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82D5C41C: 419A0018  beq cr6, 0x82d5c434
	if ctx.cr[6].eq {
	pc = 0x82D5C434; continue 'dispatch;
	}
	// 82D5C420: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C424: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C428: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D5C42C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C430: 4E800421  bctrl
	ctx.lr = 0x82D5C434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C434: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5C438: 4BF4D014  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5C440 size=8
    let mut pc: u32 = 0x82D5C440;
    'dispatch: loop {
        match pc {
            0x82D5C440 => {
    //   block [0x82D5C440..0x82D5C448)
	// 82D5C440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5C444: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5C448 size=12
    let mut pc: u32 = 0x82D5C448;
    'dispatch: loop {
        match pc {
            0x82D5C448 => {
    //   block [0x82D5C448..0x82D5C454)
	// 82D5C448: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5C44C: 806B776C  lwz r3, 0x776c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30572 as u32) ) } as u64;
	// 82D5C450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5C458 size=32
    let mut pc: u32 = 0x82D5C458;
    'dispatch: loop {
        match pc {
            0x82D5C458 => {
    //   block [0x82D5C458..0x82D5C478)
	// 82D5C458: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5C45C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5C460: 816B7768  lwz r11, 0x7768(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30568 as u32) ) } as u64;
	// 82D5C464: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D5C468: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C46C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5C470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C474: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C478 size=276
    let mut pc: u32 = 0x82D5C478;
    'dispatch: loop {
        match pc {
            0x82D5C478 => {
    //   block [0x82D5C478..0x82D5C58C)
	// 82D5C478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C47C: 4BF4CF7D  bl 0x82ca93f8
	ctx.lr = 0x82D5C480;
	sub_82CA93D0(ctx, base);
	// 82D5C480: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C484: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5C488: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5C48C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D5C490: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D5C494: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82D5C498: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82D5C49C: 419A00E8  beq cr6, 0x82d5c584
	if ctx.cr[6].eq {
	pc = 0x82D5C584; continue 'dispatch;
	}
	// 82D5C4A0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C4A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C4A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D5C4AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5C4B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C4B4: 4E800421  bctrl
	ctx.lr = 0x82D5C4B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C4B8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82D5C4BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C4C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5C4C4: 4BFF962D  bl 0x82d55af0
	ctx.lr = 0x82D5C4C8;
	sub_82D55AF0(ctx, base);
	// 82D5C4C8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C4CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5C4D0: 419A002C  beq cr6, 0x82d5c4fc
	if ctx.cr[6].eq {
	pc = 0x82D5C4FC; continue 'dispatch;
	}
	// 82D5C4D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C4D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D5C4DC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82D5C4E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5C4E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5C4E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5C4EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C4F0: 4E800421  bctrl
	ctx.lr = 0x82D5C4F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C4F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5C4F8: 4BF4CF50  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82D5C4FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C500: 4BFF95E1  bl 0x82d55ae0
	ctx.lr = 0x82D5C504;
	sub_82D55AE0(ctx, base);
	// 82D5C504: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5C508: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D5C50C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D5C510: 816B7414  lwz r11, 0x7414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29716 as u32) ) } as u64;
	// 82D5C514: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D5C518: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C51C: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D5C520: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C524: 4E800421  bctrl
	ctx.lr = 0x82D5C528;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C528: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D5C52C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D5C530: 419A000C  beq cr6, 0x82d5c53c
	if ctx.cr[6].eq {
	pc = 0x82D5C53C; continue 'dispatch;
	}
	// 82D5C534: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5C538: 48000010  b 0x82d5c548
	pc = 0x82D5C548; continue 'dispatch;
	// 82D5C53C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C540: 4BFF9211  bl 0x82d55750
	ctx.lr = 0x82D5C544;
	sub_82D55750(ctx, base);
	// 82D5C544: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5C548: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C54C: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82D5C550: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D5C554: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D5C558: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82D5C55C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5C560: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5C564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C568: 4E800421  bctrl
	ctx.lr = 0x82D5C56C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C56C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82D5C570: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D5C574: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5C578: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5C57C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C580: 4BFFFDE9  bl 0x82d5c368
	ctx.lr = 0x82D5C584;
	sub_82D5C368(ctx, base);
	// 82D5C584: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5C588: 4BF4CEC0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C590 size=116
    let mut pc: u32 = 0x82D5C590;
    'dispatch: loop {
        match pc {
            0x82D5C590 => {
    //   block [0x82D5C590..0x82D5C604)
	// 82D5C590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5C598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5C59C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C5A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5C5A4: 887F000D  lbz r3, 0xd(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5C5A8: 48000791  bl 0x82d5cd38
	ctx.lr = 0x82D5C5AC;
	sub_82D5CD38(ctx, base);
	// 82D5C5AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5C5B0: A14B0008  lhz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5C5B4: 7D430734  extsh r3, r10
	ctx.r[3].s64 = ctx.r[10].s16 as i64;
	// 82D5C5B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5C5BC: 41990034  bgt cr6, 0x82d5c5f0
	if ctx.cr[6].gt {
	pc = 0x82D5C5F0; continue 'dispatch;
	}
	// 82D5C5C0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C5C4: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82D5C5C8: 409A0024  bne cr6, 0x82d5c5ec
	if !ctx.cr[6].eq {
	pc = 0x82D5C5EC; continue 'dispatch;
	}
	// 82D5C5CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C5D0: 48000789  bl 0x82d5cd58
	ctx.lr = 0x82D5C5D4;
	sub_82D5CD58(ctx, base);
	// 82D5C5D4: 4BFF950D  bl 0x82d55ae0
	ctx.lr = 0x82D5C5D8;
	sub_82D55AE0(ctx, base);
	// 82D5C5D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5C5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5C5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5C5E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5C5E8: 4E800020  blr
	return;
	// 82D5C5EC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D5C5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5C5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5C5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5C5FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5C600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C608 size=148
    let mut pc: u32 = 0x82D5C608;
    'dispatch: loop {
        match pc {
            0x82D5C608 => {
    //   block [0x82D5C608..0x82D5C69C)
	// 82D5C608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C60C: 4BF4CDF1  bl 0x82ca93fc
	ctx.lr = 0x82D5C610;
	sub_82CA93D0(ctx, base);
	// 82D5C610: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C614: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D5C618: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D5C61C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5C620: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5C624: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D5C628: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82D5C62C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82D5C630: 4BFF94B1  bl 0x82d55ae0
	ctx.lr = 0x82D5C634;
	sub_82D55AE0(ctx, base);
	// 82D5C634: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C638: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D5C63C: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C644: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5C648: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C64C: 4E800421  bctrl
	ctx.lr = 0x82D5C650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C650: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D5C654: 4099002C  ble cr6, 0x82d5c680
	if !ctx.cr[6].gt {
	pc = 0x82D5C680; continue 'dispatch;
	}
	// 82D5C658: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D5C65C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5C660: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D5C664: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5C668: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5C66C: 4BFFFE0D  bl 0x82d5c478
	ctx.lr = 0x82D5C670;
	sub_82D5C478(ctx, base);
	// 82D5C670: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D5C674: 7FFFD214  add r31, r31, r26
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 82D5C678: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5C67C: 409AFFDC  bne cr6, 0x82d5c658
	if !ctx.cr[6].eq {
	pc = 0x82D5C658; continue 'dispatch;
	}
	// 82D5C680: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C684: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C688: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5C68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C690: 4E800421  bctrl
	ctx.lr = 0x82D5C694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C694: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5C698: 4BF4CDB4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C6A0 size=212
    let mut pc: u32 = 0x82D5C6A0;
    'dispatch: loop {
        match pc {
            0x82D5C6A0 => {
    //   block [0x82D5C6A0..0x82D5C774)
	// 82D5C6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C6A4: 4BF4CD61  bl 0x82ca9404
	ctx.lr = 0x82D5C6A8;
	sub_82CA93D0(ctx, base);
	// 82D5C6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C6AC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D5C6B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5C6B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5C6B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D5C6BC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82D5C6C0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D5C6C4: 409900A8  ble cr6, 0x82d5c76c
	if !ctx.cr[6].gt {
	pc = 0x82D5C76C; continue 'dispatch;
	}
	// 82D5C6C8: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5C6CC: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82D5C6D0: 419A0034  beq cr6, 0x82d5c704
	if ctx.cr[6].eq {
	pc = 0x82D5C704; continue 'dispatch;
	}
	// 82D5C6D4: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 82D5C6D8: 409A0094  bne cr6, 0x82d5c76c
	if !ctx.cr[6].eq {
	pc = 0x82D5C76C; continue 'dispatch;
	}
	// 82D5C6DC: 4800067D  bl 0x82d5cd58
	ctx.lr = 0x82D5C6E0;
	sub_82D5CD58(ctx, base);
	// 82D5C6E0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5C6E4: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82D5C6E8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82D5C6EC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82D5C6F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5C6F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C6F8: 4BFFFF11  bl 0x82d5c608
	ctx.lr = 0x82D5C6FC;
	sub_82D5C608(ctx, base);
	// 82D5C6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5C700: 4BF4CD54  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5C704: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C708: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5C70C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C710: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5C714: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C718: 4E800421  bctrl
	ctx.lr = 0x82D5C71C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C71C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5C720: 48000639  bl 0x82d5cd58
	ctx.lr = 0x82D5C724;
	sub_82D5CD58(ctx, base);
	// 82D5C724: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5C728: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D5C72C: 4099002C  ble cr6, 0x82d5c758
	if !ctx.cr[6].gt {
	pc = 0x82D5C758; continue 'dispatch;
	}
	// 82D5C730: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D5C734: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C738: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D5C73C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D5C740: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5C744: 4BFFFD35  bl 0x82d5c478
	ctx.lr = 0x82D5C748;
	sub_82D5C478(ctx, base);
	// 82D5C748: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82D5C74C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82D5C750: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D5C754: 409AFFDC  bne cr6, 0x82d5c730
	if !ctx.cr[6].eq {
	pc = 0x82D5C730; continue 'dispatch;
	}
	// 82D5C758: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C75C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5C760: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5C764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C768: 4E800421  bctrl
	ctx.lr = 0x82D5C76C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5C770: 4BF4CCE4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5C778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5C778 size=828
    let mut pc: u32 = 0x82D5C778;
    'dispatch: loop {
        match pc {
            0x82D5C778 => {
    //   block [0x82D5C778..0x82D5C854)
	// 82D5C778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5C77C: 4BF4CC71  bl 0x82ca93ec
	ctx.lr = 0x82D5C780;
	sub_82CA93D0(ctx, base);
	// 82D5C780: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5C784: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82D5C788: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82D5C78C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D5C790: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82D5C794: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82D5C798: 419A0314  beq cr6, 0x82d5caac
	if ctx.cr[6].eq {
	pc = 0x82D5CAAC; continue 'dispatch;
	}
	// 82D5C79C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82D5C7A0: 409A000C  bne cr6, 0x82d5c7ac
	if !ctx.cr[6].eq {
	pc = 0x82D5C7AC; continue 'dispatch;
	}
	// 82D5C7A4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5C7A8: 832B776C  lwz r25, 0x776c(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30572 as u32) ) } as u64;
	// 82D5C7AC: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C7B0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82D5C7B4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82D5C7B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5C7BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5C7C0: 4E800421  bctrl
	ctx.lr = 0x82D5C7C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5C7C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D5C7C8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82D5C7CC: 4BFF922D  bl 0x82d559f8
	ctx.lr = 0x82D5C7D0;
	sub_82D559F8(ctx, base);
	// 82D5C7D0: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82D5C7D4: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82D5C7D8: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82D5C7DC: 409902D0  ble cr6, 0x82d5caac
	if !ctx.cr[6].gt {
	pc = 0x82D5CAAC; continue 'dispatch;
	}
	// 82D5C7E0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D5C7E4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82D5C7E8: 4BFF9219  bl 0x82d55a00
	ctx.lr = 0x82D5C7EC;
	sub_82D55A00(ctx, base);
	// 82D5C7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5C7F0: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5C7F4: 556BBA7E  srwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5C7F8: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D5C7FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5C800: 409A02A0  bne cr6, 0x82d5caa0
	if !ctx.cr[6].eq {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5C804: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5C808: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 82D5C80C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82D5C810: 41990290  bgt cr6, 0x82d5caa0
	if ctx.cr[6].gt {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5C814: 3D8082D6  lis r12, -0x7d2a
	ctx.r[12].s64 = -2099904512;
	// 82D5C818: 398CC82C  addi r12, r12, -0x37d4
	ctx.r[12].s64 = ctx.r[12].s64 + -14292;
	// 82D5C81C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D5C820: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D5C824: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D5C828: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82D5CA40; continue 'dispatch;
		},
		1 => {
	pc = 0x82D5CAA0; continue 'dispatch;
		},
		2 => {
	pc = 0x82D5C8E0; continue 'dispatch;
		},
		3 => {
	pc = 0x82D5CAA0; continue 'dispatch;
		},
		4 => {
	pc = 0x82D5CAA0; continue 'dispatch;
		},
		5 => {
	pc = 0x82D5CAA0; continue 'dispatch;
		},
		6 => {
	pc = 0x82D5C944; continue 'dispatch;
		},
		7 => {
	pc = 0x82D5C9B8; continue 'dispatch;
		},
		8 => {
	pc = 0x82D5CA70; continue 'dispatch;
		},
		9 => {
	pc = 0x82D5C854; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D5C82C: 82D5CA40  lwz r22, -0x35c0(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-13760 as u32) ) } as u64;
	// 82D5C830: 82D5CAA0  lwz r22, -0x3560(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-13664 as u32) ) } as u64;
	// 82D5C834: 82D5C8E0  lwz r22, -0x3720(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-14112 as u32) ) } as u64;
	// 82D5C838: 82D5CAA0  lwz r22, -0x3560(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-13664 as u32) ) } as u64;
	// 82D5C83C: 82D5CAA0  lwz r22, -0x3560(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-13664 as u32) ) } as u64;
	// 82D5C840: 82D5CAA0  lwz r22, -0x3560(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-13664 as u32) ) } as u64;
	// 82D5C844: 82D5C944  lwz r22, -0x36bc(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-14012 as u32) ) } as u64;
	// 82D5C848: 82D5C9B8  lwz r22, -0x3648(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-13896 as u32) ) } as u64;
	// 82D5C84C: 82D5CA70  lwz r22, -0x3590(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-13712 as u32) ) } as u64;
	// 82D5C850: 82D5C854  lwz r22, -0x37ac(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-14252 as u32) ) } as u64;
            }
            0x82D5C854 => {
    //   block [0x82D5C854..0x82D5C8E0)
	// 82D5C854: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5C858: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82D5C85C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5C860: 48005B71  bl 0x82d623d0
	ctx.lr = 0x82D5C864;
	sub_82D623D0(ctx, base);
	// 82D5C864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C868: 48000509  bl 0x82d5cd70
	ctx.lr = 0x82D5C86C;
	sub_82D5CD70(ctx, base);
	// 82D5C86C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D5C870: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82D5C874: 4199002C  bgt cr6, 0x82d5c8a0
	if ctx.cr[6].gt {
	pc = 0x82D5C8A0; continue 'dispatch;
	}
	// 82D5C878: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D5C87C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5C880: 48005C71  bl 0x82d624f0
	ctx.lr = 0x82D5C884;
	sub_82D624F0(ctx, base);
	// 82D5C884: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5C888: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5C88C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D5C890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C894: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C898: 4BFFF9E1  bl 0x82d5c278
	ctx.lr = 0x82D5C89C;
	sub_82D5C278(ctx, base);
	// 82D5C89C: 48000204  b 0x82d5caa0
	pc = 0x82D5CAA0; continue 'dispatch;
	// 82D5C8A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D5C8A4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82D5C8A8: 409901F8  ble cr6, 0x82d5caa0
	if !ctx.cr[6].gt {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5C8AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5C8B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5C8B4: 48005C3D  bl 0x82d624f0
	ctx.lr = 0x82D5C8B8;
	sub_82D624F0(ctx, base);
	// 82D5C8B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5C8BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5C8C0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D5C8C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C8C8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C8CC: 4BFFF9AD  bl 0x82d5c278
	ctx.lr = 0x82D5C8D0;
	sub_82D5C278(ctx, base);
	// 82D5C8D0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82D5C8D4: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82D5C8D8: 4198FFD4  blt cr6, 0x82d5c8ac
	if ctx.cr[6].lt {
	pc = 0x82D5C8AC; continue 'dispatch;
	}
	// 82D5C8DC: 480001C4  b 0x82d5caa0
	pc = 0x82D5CAA0; continue 'dispatch;
            }
            0x82D5C8E0 => {
    //   block [0x82D5C8E0..0x82D5C944)
	// 82D5C8E0: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D5C8E4: 7FCBC214  add r30, r11, r24
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82D5C8E8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5C8EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5C8F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5C8F4: 409A01AC  bne cr6, 0x82d5caa0
	if !ctx.cr[6].eq {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5C8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C8FC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5C900: 557B00BE  clrlwi r27, r11, 2
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5C904: 4BFFFC8D  bl 0x82d5c590
	ctx.lr = 0x82D5C908;
	sub_82D5C590(ctx, base);
	// 82D5C908: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82D5C90C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82D5C910: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C914: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82D5C918: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82D5C91C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D5C920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C924: 4BFFF9D5  bl 0x82d5c2f8
	ctx.lr = 0x82D5C928;
	sub_82D5C2F8(ctx, base);
	// 82D5C928: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82D5C92C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5C930: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C934: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D5C938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C93C: 4BFFFD65  bl 0x82d5c6a0
	ctx.lr = 0x82D5C940;
	sub_82D5C6A0(ctx, base);
	// 82D5C940: 48000160  b 0x82d5caa0
	pc = 0x82D5CAA0; continue 'dispatch;
            }
            0x82D5C944 => {
    //   block [0x82D5C944..0x82D5C9B8)
	// 82D5C944: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5C948: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82D5C94C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5C950: 48005A81  bl 0x82d623d0
	ctx.lr = 0x82D5C954;
	sub_82D623D0(ctx, base);
	// 82D5C954: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D5C958: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5C95C: 48005C85  bl 0x82d625e0
	ctx.lr = 0x82D5C960;
	sub_82D625E0(ctx, base);
	// 82D5C960: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5C964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C968: E96B0000  ld r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82D5C96C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82D5C970: 4BFFFC21  bl 0x82d5c590
	ctx.lr = 0x82D5C974;
	sub_82D5C590(ctx, base);
	// 82D5C974: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D5C978: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82D5C97C: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5C980: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82D5C984: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82D5C988: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D5C98C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5C990: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5C994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C998: 4BFFF961  bl 0x82d5c2f8
	ctx.lr = 0x82D5C99C;
	sub_82D5C2F8(ctx, base);
	// 82D5C99C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82D5C9A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5C9A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5C9A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5C9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5C9B0: 4BFFFCF1  bl 0x82d5c6a0
	ctx.lr = 0x82D5C9B4;
	sub_82D5C6A0(ctx, base);
	// 82D5C9B4: 480000EC  b 0x82d5caa0
	pc = 0x82D5CAA0; continue 'dispatch;
            }
            0x82D5C9B8 => {
    //   block [0x82D5C9B8..0x82D5CA40)
	// 82D5C9B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5C9BC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82D5C9C0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82D5C9C4: 48005A0D  bl 0x82d623d0
	ctx.lr = 0x82D5C9C8;
	sub_82D623D0(ctx, base);
	// 82D5C9C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D5C9CC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82D5C9D0: 48005C21  bl 0x82d625f0
	ctx.lr = 0x82D5C9D4;
	sub_82D625F0(ctx, base);
	// 82D5C9D4: 83C30008  lwz r30, 8(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5C9D8: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5C9DC: 83630004  lwz r27, 4(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5C9E0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D5C9E4: 409900BC  ble cr6, 0x82d5caa0
	if !ctx.cr[6].gt {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5C9E8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82D5C9EC: 419A00B4  beq cr6, 0x82d5caa0
	if ctx.cr[6].eq {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5C9F0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82D5C9F4: 419A00AC  beq cr6, 0x82d5caa0
	if ctx.cr[6].eq {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5C9F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5C9FC: 4BFF90E5  bl 0x82d55ae0
	ctx.lr = 0x82D5CA00;
	sub_82D55AE0(ctx, base);
	// 82D5CA00: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82D5CA04: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D5CA08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5CA0C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D5CA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CA14: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82D5CA18: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82D5CA1C: 4BFFF8DD  bl 0x82d5c2f8
	ctx.lr = 0x82D5CA20;
	sub_82D5C2F8(ctx, base);
	// 82D5CA20: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82D5CA24: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82D5CA28: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D5CA2C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D5CA30: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D5CA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CA38: 4BFFFBD1  bl 0x82d5c608
	ctx.lr = 0x82D5CA3C;
	sub_82D5C608(ctx, base);
	// 82D5CA3C: 48000064  b 0x82d5caa0
	pc = 0x82D5CAA0; continue 'dispatch;
            }
            0x82D5CA40 => {
    //   block [0x82D5CA40..0x82D5CA70)
	// 82D5CA40: 897F000D  lbz r11, 0xd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5CA44: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82D5CA48: 409A0058  bne cr6, 0x82d5caa0
	if !ctx.cr[6].eq {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5CA4C: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D5CA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CA54: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CA58: 7FCBC02E  lwzx r30, r11, r24
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D5CA5C: 480002FD  bl 0x82d5cd58
	ctx.lr = 0x82D5CA60;
	sub_82D5CD58(ctx, base);
	// 82D5CA60: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5CA64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5CA68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CA6C: 48000028  b 0x82d5ca94
	pc = 0x82D5CA94; continue 'dispatch;
            }
            0x82D5CA70 => {
    //   block [0x82D5CA70..0x82D5CAA0)
	// 82D5CA70: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D5CA74: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82D5CA78: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CA7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D5CA80: 419A0020  beq cr6, 0x82d5caa0
	if ctx.cr[6].eq {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5CA84: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CA88: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D5CA8C: 419A0014  beq cr6, 0x82d5caa0
	if ctx.cr[6].eq {
	pc = 0x82D5CAA0; continue 'dispatch;
	}
	// 82D5CA90: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CA94: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82D5CA98: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5CA9C: 4BFFF9DD  bl 0x82d5c478
	ctx.lr = 0x82D5CAA0;
	sub_82D5C478(ctx, base);
	pc = 0x82D5CAA0; continue 'dispatch;
            }
            0x82D5CAA0 => {
    //   block [0x82D5CAA0..0x82D5CAB4)
	// 82D5CAA0: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82D5CAA4: 7F17A800  cmpw cr6, r23, r21
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[21].s32, &mut ctx.xer);
	// 82D5CAA8: 4198FD38  blt cr6, 0x82d5c7e0
	if ctx.cr[6].lt {
	pc = 0x82D5C7E0; continue 'dispatch;
	}
	// 82D5CAAC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82D5CAB0: 4BF4C98C  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5CAB8 size=100
    let mut pc: u32 = 0x82D5CAB8;
    'dispatch: loop {
        match pc {
            0x82D5CAB8 => {
    //   block [0x82D5CAB8..0x82D5CB1C)
	// 82D5CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5CABC: 4BF4C94D  bl 0x82ca9408
	ctx.lr = 0x82D5CAC0;
	sub_82CA93D0(ctx, base);
	// 82D5CAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5CAC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5CAC8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D5CACC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D5CAD0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D5CAD4: 419A0040  beq cr6, 0x82d5cb14
	if ctx.cr[6].eq {
	pc = 0x82D5CB14; continue 'dispatch;
	}
	// 82D5CAD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5CADC: 409A000C  bne cr6, 0x82d5cae8
	if !ctx.cr[6].eq {
	pc = 0x82D5CAE8; continue 'dispatch;
	}
	// 82D5CAE0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CAE4: 83CB776C  lwz r30, 0x776c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30572 as u32) ) } as u64;
	// 82D5CAE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5CAEC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D5CAF0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D5CAF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CAF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5CAFC: 4BFFFC7D  bl 0x82d5c778
	ctx.lr = 0x82D5CB00;
	sub_82D5C778(ctx, base);
	// 82D5CB00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CB04: 4BFF8C55  bl 0x82d55758
	ctx.lr = 0x82D5CB08;
	sub_82D55758(ctx, base);
	// 82D5CB08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5CB0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D5CB10: 409AFFDC  bne cr6, 0x82d5caec
	if !ctx.cr[6].eq {
	pc = 0x82D5CAEC; continue 'dispatch;
	}
	// 82D5CB14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5CB18: 4BF4C940  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CB20 size=40
    let mut pc: u32 = 0x82D5CB20;
    'dispatch: loop {
        match pc {
            0x82D5CB20 => {
    //   block [0x82D5CB20..0x82D5CB48)
	// 82D5CB20: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82D5CB24: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D5CB28: 409A000C  bne cr6, 0x82d5cb34
	if !ctx.cr[6].eq {
	pc = 0x82D5CB34; continue 'dispatch;
	}
	// 82D5CB2C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CB30: 80EB776C  lwz r7, 0x776c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30572 as u32) ) } as u64;
	// 82D5CB34: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5CB38: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D5CB3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5CB40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5CB44: 4BFFF824  b 0x82d5c368
	sub_82D5C368(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5CB48 size=108
    let mut pc: u32 = 0x82D5CB48;
    'dispatch: loop {
        match pc {
            0x82D5CB48 => {
    //   block [0x82D5CB48..0x82D5CBB4)
	// 82D5CB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5CB4C: 4BF4C8C1  bl 0x82ca940c
	ctx.lr = 0x82D5CB50;
	sub_82CA93D0(ctx, base);
	// 82D5CB50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5CB54: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5CB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5CB5C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D5CB60: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5CB64: 409A000C  bne cr6, 0x82d5cb70
	if !ctx.cr[6].eq {
	pc = 0x82D5CB70; continue 'dispatch;
	}
	// 82D5CB68: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CB6C: 83CB776C  lwz r30, 0x776c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30572 as u32) ) } as u64;
	// 82D5CB70: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CB74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CB78: 806B7768  lwz r3, 0x7768(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30568 as u32) ) } as u64;
	// 82D5CB7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CB80: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5CB84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5CB88: 4E800421  bctrl
	ctx.lr = 0x82D5CB8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5CB8C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5CB90: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D5CB94: 419A0018  beq cr6, 0x82d5cbac
	if ctx.cr[6].eq {
	pc = 0x82D5CBAC; continue 'dispatch;
	}
	// 82D5CB98: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82D5CB9C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5CBA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CBA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5CBA8: 4BFFF7C1  bl 0x82d5c368
	ctx.lr = 0x82D5CBAC;
	sub_82D5C368(ctx, base);
	// 82D5CBAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5CBB0: 4BF4C8AC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5CBB8 size=204
    let mut pc: u32 = 0x82D5CBB8;
    'dispatch: loop {
        match pc {
            0x82D5CBB8 => {
    //   block [0x82D5CBB8..0x82D5CC84)
	// 82D5CBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5CBBC: 4BF4C849  bl 0x82ca9404
	ctx.lr = 0x82D5CBC0;
	sub_82CA93D0(ctx, base);
	// 82D5CBC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5CBC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5CBC8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5CBCC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5CBD0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D5CBD4: 419A00A4  beq cr6, 0x82d5cc78
	if ctx.cr[6].eq {
	pc = 0x82D5CC78; continue 'dispatch;
	}
	// 82D5CBD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5CBDC: 409A000C  bne cr6, 0x82d5cbe8
	if !ctx.cr[6].eq {
	pc = 0x82D5CBE8; continue 'dispatch;
	}
	// 82D5CBE0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CBE4: 83CB776C  lwz r30, 0x776c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30572 as u32) ) } as u64;
	// 82D5CBE8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CBEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5CBF0: 806B7768  lwz r3, 0x7768(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30568 as u32) ) } as u64;
	// 82D5CBF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CBF8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5CBFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5CC00: 4E800421  bctrl
	ctx.lr = 0x82D5CC04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5CC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5CC08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D5CC0C: 419A006C  beq cr6, 0x82d5cc78
	if ctx.cr[6].eq {
	pc = 0x82D5CC78; continue 'dispatch;
	}
	// 82D5CC10: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CC14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CC18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5CC1C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5CC20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5CC24: 4E800421  bctrl
	ctx.lr = 0x82D5CC28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5CC28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5CC2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CC30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5CC34: 4BFF8EBD  bl 0x82d55af0
	ctx.lr = 0x82D5CC38;
	sub_82D55AF0(ctx, base);
	// 82D5CC38: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CC3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5CC40: 419A0038  beq cr6, 0x82d5cc78
	if ctx.cr[6].eq {
	pc = 0x82D5CC78; continue 'dispatch;
	}
	// 82D5CC44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CC48: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CC4C: 4BFF8B05  bl 0x82d55750
	ctx.lr = 0x82D5CC50;
	sub_82D55750(ctx, base);
	// 82D5CC50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5CC54: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CC58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5CC5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5CC60: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5CC64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5CC68: 4E800421  bctrl
	ctx.lr = 0x82D5CC6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5CC6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CC70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5CC74: 4BF4C7E0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5CC78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5CC7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5CC80: 4BF4C7D4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5CC88 size=88
    let mut pc: u32 = 0x82D5CC88;
    'dispatch: loop {
        match pc {
            0x82D5CC88 => {
    //   block [0x82D5CC88..0x82D5CCE0)
	// 82D5CC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5CC8C: 4BF4C781  bl 0x82ca940c
	ctx.lr = 0x82D5CC90;
	sub_82CA93D0(ctx, base);
	// 82D5CC90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5CC94: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CC98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5CC9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5CCA0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D5CCA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CCA8: 806B7768  lwz r3, 0x7768(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30568 as u32) ) } as u64;
	// 82D5CCAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CCB0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5CCB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5CCB8: 4E800421  bctrl
	ctx.lr = 0x82D5CCBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5CCBC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5CCC0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D5CCC4: 419A0014  beq cr6, 0x82d5ccd8
	if ctx.cr[6].eq {
	pc = 0x82D5CCD8; continue 'dispatch;
	}
	// 82D5CCC8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5CCCC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5CCD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CCD4: 4BFFFAA5  bl 0x82d5c778
	ctx.lr = 0x82D5CCD8;
	sub_82D5C778(ctx, base);
	// 82D5CCD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5CCDC: 4BF4C780  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5CCE0 size=88
    let mut pc: u32 = 0x82D5CCE0;
    'dispatch: loop {
        match pc {
            0x82D5CCE0 => {
    //   block [0x82D5CCE0..0x82D5CD38)
	// 82D5CCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5CCE4: 4BF4C729  bl 0x82ca940c
	ctx.lr = 0x82D5CCE8;
	sub_82CA93D0(ctx, base);
	// 82D5CCE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5CCEC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5CCF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5CCF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5CCF8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D5CCFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CD00: 806B7768  lwz r3, 0x7768(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30568 as u32) ) } as u64;
	// 82D5CD04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5CD08: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5CD0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5CD10: 4E800421  bctrl
	ctx.lr = 0x82D5CD14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5CD14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5CD18: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D5CD1C: 419A0014  beq cr6, 0x82d5cd30
	if ctx.cr[6].eq {
	pc = 0x82D5CD30; continue 'dispatch;
	}
	// 82D5CD20: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D5CD24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5CD28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5CD2C: 4BFFFD8D  bl 0x82d5cab8
	ctx.lr = 0x82D5CD30;
	sub_82D5CAB8(ctx, base);
	// 82D5CD30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5CD34: 4BF4C728  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD38 size=28
    let mut pc: u32 = 0x82D5CD38;
    'dispatch: loop {
        match pc {
            0x82D5CD38 => {
    //   block [0x82D5CD38..0x82D5CD54)
	// 82D5CD38: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5CD3C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D5CD40: 7D435214  add r10, r3, r10
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 82D5CD44: 396B73F0  addi r11, r11, 0x73f0
	ctx.r[11].s64 = ctx.r[11].s64 + 29680;
	// 82D5CD48: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5CD4C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D5CD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD58 size=8
    let mut pc: u32 = 0x82D5CD58;
    'dispatch: loop {
        match pc {
            0x82D5CD58 => {
    //   block [0x82D5CD58..0x82D5CD60)
	// 82D5CD58: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD60 size=8
    let mut pc: u32 = 0x82D5CD60;
    'dispatch: loop {
        match pc {
            0x82D5CD60 => {
    //   block [0x82D5CD60..0x82D5CD68)
	// 82D5CD60: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD68 size=8
    let mut pc: u32 = 0x82D5CD68;
    'dispatch: loop {
        match pc {
            0x82D5CD68 => {
    //   block [0x82D5CD68..0x82D5CD70)
	// 82D5CD68: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5CD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD70 size=12
    let mut pc: u32 = 0x82D5CD70;
    'dispatch: loop {
        match pc {
            0x82D5CD70 => {
    //   block [0x82D5CD70..0x82D5CD7C)
	// 82D5CD70: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82D5CD74: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82D5CD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD80 size=8
    let mut pc: u32 = 0x82D5CD80;
    'dispatch: loop {
        match pc {
            0x82D5CD80 => {
    //   block [0x82D5CD80..0x82D5CD88)
	// 82D5CD80: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5CD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD88 size=16
    let mut pc: u32 = 0x82D5CD88;
    'dispatch: loop {
        match pc {
            0x82D5CD88 => {
    //   block [0x82D5CD88..0x82D5CD98)
	// 82D5CD88: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5CD8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5CD90: 419A0008  beq cr6, 0x82d5cd98
	if ctx.cr[6].eq {
		sub_82D5CD98(ctx, base);
		return;
	}
	// 82D5CD94: 4BFF893C  b 0x82d556d0
	sub_82D556D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CD98 size=8
    let mut pc: u32 = 0x82D5CD98;
    'dispatch: loop {
        match pc {
            0x82D5CD98 => {
    //   block [0x82D5CD98..0x82D5CDA0)
	// 82D5CD98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5CD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5CDA0 size=364
    let mut pc: u32 = 0x82D5CDA0;
    'dispatch: loop {
        match pc {
            0x82D5CDA0 => {
    //   block [0x82D5CDA0..0x82D5CE58)
	// 82D5CDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5CDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5CDA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5CDAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5CDB0: 8943000C  lbz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5CDB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82D5CDB8: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 82D5CDBC: 2B09001E  cmplwi cr6, r9, 0x1e
	ctx.cr[6].compare_u32(ctx.r[9].u32, 30 as u32, &mut ctx.xer);
	// 82D5CDC0: 41990134  bgt cr6, 0x82d5cef4
	if ctx.cr[6].gt {
	pc = 0x82D5CEF4; continue 'dispatch;
	}
	// 82D5CDC4: 3D8082D6  lis r12, -0x7d2a
	ctx.r[12].s64 = -2099904512;
	// 82D5CDC8: 398CCDDC  addi r12, r12, -0x3224
	ctx.r[12].s64 = ctx.r[12].s64 + -12836;
	// 82D5CDCC: 5520103A  slwi r0, r9, 2
	ctx.r[0].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D5CDD0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D5CDD4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D5CDD8: 4E800420  bctr
	match ctx.r[9].u64 {
		0 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		1 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		2 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		3 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		4 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		5 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		6 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		7 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		8 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		9 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		10 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		11 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		12 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		13 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		14 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		15 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		16 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		17 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		18 => {
	pc = 0x82D5CEF4; continue 'dispatch;
		},
		19 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		20 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		21 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		22 => {
	pc = 0x82D5CEF4; continue 'dispatch;
		},
		23 => {
	pc = 0x82D5CE94; continue 'dispatch;
		},
		24 => {
	pc = 0x82D5CED4; continue 'dispatch;
		},
		25 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		26 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		27 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		28 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		29 => {
	pc = 0x82D5CE58; continue 'dispatch;
		},
		30 => {
	pc = 0x82D5CE94; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D5CDDC: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDE0: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDE4: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDE8: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDEC: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDF0: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDF4: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDF8: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CDFC: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE00: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE04: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE08: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE0C: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE10: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE14: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE18: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE1C: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE20: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE24: 82D5CEF4  lwz r22, -0x310c(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12556 as u32) ) } as u64;
	// 82D5CE28: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE2C: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE30: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE34: 82D5CEF4  lwz r22, -0x310c(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12556 as u32) ) } as u64;
	// 82D5CE38: 82D5CE94  lwz r22, -0x316c(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12652 as u32) ) } as u64;
	// 82D5CE3C: 82D5CED4  lwz r22, -0x312c(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12588 as u32) ) } as u64;
	// 82D5CE40: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE44: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE48: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE4C: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE50: 82D5CE58  lwz r22, -0x31a8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12712 as u32) ) } as u64;
	// 82D5CE54: 82D5CE94  lwz r22, -0x316c(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12652 as u32) ) } as u64;
            }
            0x82D5CE58 => {
    //   block [0x82D5CE58..0x82D5CE94)
	// 82D5CE58: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82D5CE5C: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 82D5CE60: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D5CE64: 409A0008  bne cr6, 0x82d5ce6c
	if !ctx.cr[6].eq {
	pc = 0x82D5CE6C; continue 'dispatch;
	}
	// 82D5CE68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D5CE6C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D5CE70: 396B73F0  addi r11, r11, 0x73f0
	ctx.r[11].s64 = ctx.r[11].s64 + 29680;
	// 82D5CE74: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 82D5CE78: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5CE7C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D5CE80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5CE84: 7D6B422E  lhzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D5CE88: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D5CE8C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82D5CE90: 48000064  b 0x82d5cef4
	pc = 0x82D5CEF4; continue 'dispatch;
            }
            0x82D5CE94 => {
    //   block [0x82D5CE94..0x82D5CED4)
	// 82D5CE94: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82D5CE98: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 82D5CE9C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D5CEA0: 409A0008  bne cr6, 0x82d5cea8
	if !ctx.cr[6].eq {
	pc = 0x82D5CEA8; continue 'dispatch;
	}
	// 82D5CEA4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D5CEA8: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D5CEAC: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5CEB0: 394A73F0  addi r10, r10, 0x73f0
	ctx.r[10].s64 = ctx.r[10].s64 + 29680;
	// 82D5CEB4: 390A0008  addi r8, r10, 8
	ctx.r[8].s64 = ctx.r[10].s64 + 8;
	// 82D5CEB8: 556A083E  rotlwi r10, r11, 1
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82D5CEBC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5CEC0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5CEC4: 7D6B422E  lhzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D5CEC8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D5CECC: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82D5CED0: 48000024  b 0x82d5cef4
	pc = 0x82D5CEF4; continue 'dispatch;
            }
            0x82D5CED4 => {
    //   block [0x82D5CED4..0x82D5CEF4)
	// 82D5CED4: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82D5CED8: 7D7F0734  extsh r31, r11
	ctx.r[31].s64 = ctx.r[11].s16 as i64;
	// 82D5CEDC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D5CEE0: 409A0008  bne cr6, 0x82d5cee8
	if !ctx.cr[6].eq {
	pc = 0x82D5CEE8; continue 'dispatch;
	}
	// 82D5CEE4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82D5CEE8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CEEC: 4BFF8BF5  bl 0x82d55ae0
	ctx.lr = 0x82D5CEF0;
	sub_82D55AE0(ctx, base);
	// 82D5CEF0: 7D63F9D6  mullw r11, r3, r31
	ctx.r[11].s64 = (ctx.r[3].s32 as i64) * (ctx.r[31].s32 as i64);
	pc = 0x82D5CEF4; continue 'dispatch;
            }
            0x82D5CEF4 => {
    //   block [0x82D5CEF4..0x82D5CF0C)
	// 82D5CEF4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D5CEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5CEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5CF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5CF04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5CF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5CF10 size=20
    let mut pc: u32 = 0x82D5CF10;
    'dispatch: loop {
        match pc {
            0x82D5CF10 => {
    //   block [0x82D5CF10..0x82D5CF24)
	// 82D5CF10: A1640010  lhz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5CF14: 556BBA7E  srwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5CF18: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D5CF1C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5CF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5CF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D5CF28 size=236
    let mut pc: u32 = 0x82D5CF28;
    'dispatch: loop {
        match pc {
            0x82D5CF28 => {
    //   block [0x82D5CF28..0x82D5D014)
	// 82D5CF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5CF2C: 4BF4C4E1  bl 0x82ca940c
	ctx.lr = 0x82D5CF30;
	sub_82CA93D0(ctx, base);
	// 82D5CF30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5CF34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5CF38: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5CF3C: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 82D5CF40: 419A000C  beq cr6, 0x82d5cf4c
	if ctx.cr[6].eq {
	pc = 0x82D5CF4C; continue 'dispatch;
	}
	// 82D5CF44: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 82D5CF48: 409A0008  bne cr6, 0x82d5cf50
	if !ctx.cr[6].eq {
	pc = 0x82D5CF50; continue 'dispatch;
	}
	// 82D5CF4C: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5CF50: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 82D5CF54: 409A0064  bne cr6, 0x82d5cfb8
	if !ctx.cr[6].eq {
	pc = 0x82D5CFB8; continue 'dispatch;
	}
	// 82D5CF58: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CF5C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82D5CF60: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D5CF64: 4BFF89FD  bl 0x82d55960
	ctx.lr = 0x82D5CF68;
	sub_82D55960(ctx, base);
	// 82D5CF68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5CF6C: 40990044  ble cr6, 0x82d5cfb0
	if !ctx.cr[6].gt {
	pc = 0x82D5CFB0; continue 'dispatch;
	}
	// 82D5CF70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CF74: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CF78: 4BFF8A11  bl 0x82d55988
	ctx.lr = 0x82D5CF7C;
	sub_82D55988(ctx, base);
	// 82D5CF7C: 4BFFFFAD  bl 0x82d5cf28
	ctx.lr = 0x82D5CF80;
	sub_82D5CF28(ctx, base);
	// 82D5CF80: 7F03E800  cmpw cr6, r3, r29
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D5CF84: 40990018  ble cr6, 0x82d5cf9c
	if !ctx.cr[6].gt {
	pc = 0x82D5CF9C; continue 'dispatch;
	}
	// 82D5CF88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5CF8C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CF90: 4BFF89F9  bl 0x82d55988
	ctx.lr = 0x82D5CF94;
	sub_82D55988(ctx, base);
	// 82D5CF94: 4BFFFF95  bl 0x82d5cf28
	ctx.lr = 0x82D5CF98;
	sub_82D5CF28(ctx, base);
	// 82D5CF98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5CF9C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5CFA0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82D5CFA4: 4BFF89BD  bl 0x82d55960
	ctx.lr = 0x82D5CFA8;
	sub_82D55960(ctx, base);
	// 82D5CFA8: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82D5CFAC: 4198FFC4  blt cr6, 0x82d5cf70
	if ctx.cr[6].lt {
	pc = 0x82D5CF70; continue 'dispatch;
	}
	// 82D5CFB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5CFB4: 48000024  b 0x82d5cfd8
	pc = 0x82D5CFD8; continue 'dispatch;
	// 82D5CFB8: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D5CFBC: 394A73F0  addi r10, r10, 0x73f0
	ctx.r[10].s64 = ctx.r[10].s64 + 29680;
	// 82D5CFC0: 392A000A  addi r9, r10, 0xa
	ctx.r[9].s64 = ctx.r[10].s64 + 10;
	// 82D5CFC4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5CFC8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5CFCC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5CFD0: 7D6B4A2E  lhzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5CFD4: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82D5CFD8: A17E0010  lhz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5CFDC: 556A05F0  rlwinm r10, r11, 0, 0x17, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5CFE0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5CFE4: 419A0028  beq cr6, 0x82d5d00c
	if ctx.cr[6].eq {
	pc = 0x82D5D00C; continue 'dispatch;
	}
	// 82D5CFE8: 556BC63E  rlwinm r11, r11, 0x18, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82D5CFEC: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D5CFF0: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82D5CFF4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82D5CFF8: 556B0738  rlwinm r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5CFFC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82D5D000: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82D5D004: 40990008  ble cr6, 0x82d5d00c
	if !ctx.cr[6].gt {
	pc = 0x82D5D00C; continue 'dispatch;
	}
	// 82D5D008: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D5D00C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5D010: 4BF4C44C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5D018 size=8
    let mut pc: u32 = 0x82D5D018;
    'dispatch: loop {
        match pc {
            0x82D5D018 => {
    //   block [0x82D5D018..0x82D5D020)
	// 82D5D018: 8863000D  lbz r3, 0xd(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5D01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5D020 size=64
    let mut pc: u32 = 0x82D5D020;
    'dispatch: loop {
        match pc {
            0x82D5D020 => {
    //   block [0x82D5D020..0x82D5D060)
	// 82D5D020: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5D024: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82D5D028: 419A0040  beq cr6, 0x82d5d068
	if ctx.cr[6].eq {
		sub_82D5D068(ctx, base);
		return;
	}
	// 82D5D02C: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 82D5D030: 419A0038  beq cr6, 0x82d5d068
	if ctx.cr[6].eq {
		sub_82D5D068(ctx, base);
		return;
	}
	// 82D5D034: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82D5D038: 419A0028  beq cr6, 0x82d5d060
	if ctx.cr[6].eq {
		sub_82D5D060(ctx, base);
		return;
	}
	// 82D5D03C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5D040: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D5D044: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D5D048: 394A73F0  addi r10, r10, 0x73f0
	ctx.r[10].s64 = ctx.r[10].s64 + 29680;
	// 82D5D04C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D050: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82D5D054: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5D058: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82D5D05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5D060 size=8
    let mut pc: u32 = 0x82D5D060;
    'dispatch: loop {
        match pc {
            0x82D5D060 => {
    //   block [0x82D5D060..0x82D5D068)
	// 82D5D060: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5D064: 4BFF8A7C  b 0x82d55ae0
	sub_82D55AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5D068 size=8
    let mut pc: u32 = 0x82D5D068;
    'dispatch: loop {
        match pc {
            0x82D5D068 => {
    //   block [0x82D5D068..0x82D5D070)
	// 82D5D068: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D5D06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5D070 size=116
    let mut pc: u32 = 0x82D5D070;
    'dispatch: loop {
        match pc {
            0x82D5D070 => {
    //   block [0x82D5D070..0x82D5D0E4)
	// 82D5D070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5D074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5D078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5D07C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5D080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5D084: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5D088: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D5D08C: 4BFFFD15  bl 0x82d5cda0
	ctx.lr = 0x82D5D090;
	sub_82D5CDA0(ctx, base);
	// 82D5D090: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82D5D094: 419A0028  beq cr6, 0x82d5d0bc
	if ctx.cr[6].eq {
	pc = 0x82D5D0BC; continue 'dispatch;
	}
	// 82D5D098: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82D5D09C: 419A0014  beq cr6, 0x82d5d0b0
	if ctx.cr[6].eq {
	pc = 0x82D5D0B0; continue 'dispatch;
	}
	// 82D5D0A0: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82D5D0A4: 409A0024  bne cr6, 0x82d5d0c8
	if !ctx.cr[6].eq {
	pc = 0x82D5D0C8; continue 'dispatch;
	}
	// 82D5D0A8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D0AC: 48000020  b 0x82d5d0cc
	pc = 0x82D5D0CC; continue 'dispatch;
	// 82D5D0B0: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D0B4: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82D5D0B8: 48000014  b 0x82d5d0cc
	pc = 0x82D5D0CC; continue 'dispatch;
	// 82D5D0BC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D0C0: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82D5D0C4: 48000008  b 0x82d5d0cc
	pc = 0x82D5D0CC; continue 'dispatch;
	// 82D5D0C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5D0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5D0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5D0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5D0D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5D0DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5D0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5D0E8 size=100
    let mut pc: u32 = 0x82D5D0E8;
    'dispatch: loop {
        match pc {
            0x82D5D0E8 => {
    //   block [0x82D5D0E8..0x82D5D14C)
	// 82D5D0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5D0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5D0F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5D0F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5D0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5D0FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5D100: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5D104: 4BFFFC9D  bl 0x82d5cda0
	ctx.lr = 0x82D5D108;
	sub_82D5CDA0(ctx, base);
	// 82D5D108: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82D5D10C: 419A0024  beq cr6, 0x82d5d130
	if ctx.cr[6].eq {
	pc = 0x82D5D130; continue 'dispatch;
	}
	// 82D5D110: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82D5D114: 419A0014  beq cr6, 0x82d5d128
	if ctx.cr[6].eq {
	pc = 0x82D5D128; continue 'dispatch;
	}
	// 82D5D118: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82D5D11C: 409A0018  bne cr6, 0x82d5d134
	if !ctx.cr[6].eq {
	pc = 0x82D5D134; continue 'dispatch;
	}
	// 82D5D120: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82D5D124: 48000010  b 0x82d5d134
	pc = 0x82D5D134; continue 'dispatch;
	// 82D5D128: B3DF0000  sth r30, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82D5D12C: 48000008  b 0x82d5d134
	pc = 0x82D5D134; continue 'dispatch;
	// 82D5D130: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82D5D134: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5D138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5D13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5D140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5D144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5D148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5D150 size=284
    let mut pc: u32 = 0x82D5D150;
    'dispatch: loop {
        match pc {
            0x82D5D150 => {
    //   block [0x82D5D150..0x82D5D26C)
	// 82D5D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5D154: 4BF4C2B9  bl 0x82ca940c
	ctx.lr = 0x82D5D158;
	sub_82CA93D0(ctx, base);
	// 82D5D158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5D15C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82D5D160: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D5D164: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5D168: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5D16C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82D5D170: 394A73F0  addi r10, r10, 0x73f0
	ctx.r[10].s64 = ctx.r[10].s64 + 29680;
	// 82D5D174: 409A00CC  bne cr6, 0x82d5d240
	if !ctx.cr[6].eq {
	pc = 0x82D5D240; continue 'dispatch;
	}
	// 82D5D178: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 82D5D17C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5D180: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5D184: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D188: 7FAB482E  lwzx r29, r11, r9
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5D18C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D5D190: 419A0068  beq cr6, 0x82d5d1f8
	if ctx.cr[6].eq {
	pc = 0x82D5D1F8; continue 'dispatch;
	}
	// 82D5D194: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D198: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5D19C: 419A005C  beq cr6, 0x82d5d1f8
	if ctx.cr[6].eq {
	pc = 0x82D5D1F8; continue 'dispatch;
	}
	// 82D5D1A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5D1A4: 4BFFBA85  bl 0x82d58c28
	ctx.lr = 0x82D5D1A8;
	sub_82D58C28(ctx, base);
	// 82D5D1A8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5D1AC: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5D1B0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D1B4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5D1B8: 40980024  bge cr6, 0x82d5d1dc
	if !ctx.cr[6].lt {
	pc = 0x82D5D1DC; continue 'dispatch;
	}
	// 82D5D1BC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D1C0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5D1C4: 41980008  blt cr6, 0x82d5d1cc
	if ctx.cr[6].lt {
	pc = 0x82D5D1CC; continue 'dispatch;
	}
	// 82D5D1C8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5D1CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D1D0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5D1D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5D1D8: 4BFF9D39  bl 0x82d56f10
	ctx.lr = 0x82D5D1DC;
	sub_82D56F10(ctx, base);
	// 82D5D1DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5D1E0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D1E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5D1E8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D5D1EC: 4BFFBB45  bl 0x82d58d30
	ctx.lr = 0x82D5D1F0;
	sub_82D58D30(ctx, base);
	// 82D5D1F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5D1F4: 4BF4C268  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D5D1F8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5D1FC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D200: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82D5D204: 40980020  bge cr6, 0x82d5d224
	if !ctx.cr[6].lt {
	pc = 0x82D5D224; continue 'dispatch;
	}
	// 82D5D208: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D5D20C: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82D5D210: 41990008  bgt cr6, 0x82d5d218
	if ctx.cr[6].gt {
	pc = 0x82D5D218; continue 'dispatch;
	}
	// 82D5D214: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5D218: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D21C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5D220: 4BFF9CF1  bl 0x82d56f10
	ctx.lr = 0x82D5D224;
	sub_82D56F10(ctx, base);
	// 82D5D224: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D5D228: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D22C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5D230: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D5D234: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82D5D238: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5D23C: 4BF4C220  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D5D240: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 82D5D244: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5D248: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D5D24C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5D250: 38894CE0  addi r4, r9, 0x4ce0
	ctx.r[4].s64 = ctx.r[9].s64 + 19680;
	// 82D5D254: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5D25C: 7CAB402E  lwzx r5, r11, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D5D260: 4BFFBC31  bl 0x82d58e90
	ctx.lr = 0x82D5D264;
	sub_82D58E90(ctx, base);
	// 82D5D264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5D268: 4BF4C1F4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5D270 size=924
    let mut pc: u32 = 0x82D5D270;
    'dispatch: loop {
        match pc {
            0x82D5D270 => {
    //   block [0x82D5D270..0x82D5D60C)
	// 82D5D270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5D274: 4BF4C17D  bl 0x82ca93f0
	ctx.lr = 0x82D5D278;
	sub_82CA93D0(ctx, base);
	// 82D5D278: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5D27C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D5D280: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82D5D284: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82D5D288: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82D5D28C: 3BCB16F8  addi r30, r11, 0x16f8
	ctx.r[30].s64 = ctx.r[11].s64 + 5880;
	// 82D5D290: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5D294: 8BFD000C  lbz r31, 0xc(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5D298: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5D29C: 419A0010  beq cr6, 0x82d5d2ac
	if ctx.cr[6].eq {
	pc = 0x82D5D2AC; continue 'dispatch;
	}
	// 82D5D2A0: 4BFF84B1  bl 0x82d55750
	ctx.lr = 0x82D5D2A4;
	sub_82D55750(ctx, base);
	// 82D5D2A4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D5D2A8: 48000008  b 0x82d5d2b0
	pc = 0x82D5D2B0; continue 'dispatch;
	// 82D5D2AC: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 82D5D2B0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5D2B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5D2B8: 419A0008  beq cr6, 0x82d5d2c0
	if ctx.cr[6].eq {
	pc = 0x82D5D2C0; continue 'dispatch;
	}
	// 82D5D2BC: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D2C0: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D2C4: 3B200004  li r25, 4
	ctx.r[25].s64 = 4;
	// 82D5D2C8: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D5D2CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5D2D0: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D5D2D4: 4BFF7F75  bl 0x82d55248
	ctx.lr = 0x82D5D2D8;
	sub_82D55248(ctx, base);
	// 82D5D2D8: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82D5D2DC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82D5D2E0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D5D2E4: 2F1F0014  cmpwi cr6, r31, 0x14
	ctx.cr[6].compare_i32(ctx.r[31].s32, 20, &mut ctx.xer);
	// 82D5D2E8: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82D5D2EC: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82D5D2F0: 9B630000  stb r27, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82D5D2F4: 419802BC  blt cr6, 0x82d5d5b0
	if ctx.cr[6].lt {
	pc = 0x82D5D5B0; continue 'dispatch;
	}
	// 82D5D2F8: 2F1F001B  cmpwi cr6, r31, 0x1b
	ctx.cr[6].compare_i32(ctx.r[31].s32, 27, &mut ctx.xer);
	// 82D5D2FC: 419A02B4  beq cr6, 0x82d5d5b0
	if ctx.cr[6].eq {
	pc = 0x82D5D5B0; continue 'dispatch;
	}
	// 82D5D300: 2F1F001D  cmpwi cr6, r31, 0x1d
	ctx.cr[6].compare_i32(ctx.r[31].s32, 29, &mut ctx.xer);
	// 82D5D304: 419A02AC  beq cr6, 0x82d5d5b0
	if ctx.cr[6].eq {
	pc = 0x82D5D5B0; continue 'dispatch;
	}
	// 82D5D308: 2F1F0014  cmpwi cr6, r31, 0x14
	ctx.cr[6].compare_i32(ctx.r[31].s32, 20, &mut ctx.xer);
	// 82D5D30C: 409A013C  bne cr6, 0x82d5d448
	if !ctx.cr[6].eq {
	pc = 0x82D5D448; continue 'dispatch;
	}
	// 82D5D310: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5D314: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5D318: 419A0020  beq cr6, 0x82d5d338
	if ctx.cr[6].eq {
	pc = 0x82D5D338; continue 'dispatch;
	}
	// 82D5D31C: 4BFF8435  bl 0x82d55750
	ctx.lr = 0x82D5D320;
	sub_82D55750(ctx, base);
	// 82D5D320: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D324: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5D328: 388B4DA0  addi r4, r11, 0x4da0
	ctx.r[4].s64 = ctx.r[11].s64 + 19872;
	// 82D5D32C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D330: 4BFFBB61  bl 0x82d58e90
	ctx.lr = 0x82D5D334;
	sub_82D58E90(ctx, base);
	// 82D5D334: 48000290  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D338: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5D33C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82D5D340: 409A00AC  bne cr6, 0x82d5d3ec
	if !ctx.cr[6].eq {
	pc = 0x82D5D3EC; continue 'dispatch;
	}
	// 82D5D344: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D348: 3BCB4D98  addi r30, r11, 0x4d98
	ctx.r[30].s64 = ctx.r[11].s64 + 19864;
	// 82D5D34C: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D350: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5D354: 419A0058  beq cr6, 0x82d5d3ac
	if ctx.cr[6].eq {
	pc = 0x82D5D3AC; continue 'dispatch;
	}
	// 82D5D358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5D35C: 4BFFB8CD  bl 0x82d58c28
	ctx.lr = 0x82D5D360;
	sub_82D58C28(ctx, base);
	// 82D5D360: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D364: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5D368: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D36C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5D370: 40980024  bge cr6, 0x82d5d394
	if !ctx.cr[6].lt {
	pc = 0x82D5D394; continue 'dispatch;
	}
	// 82D5D374: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D378: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5D37C: 41980008  blt cr6, 0x82d5d384
	if ctx.cr[6].lt {
	pc = 0x82D5D384; continue 'dispatch;
	}
	// 82D5D380: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5D384: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D388: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5D38C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D390: 4BFF9B81  bl 0x82d56f10
	ctx.lr = 0x82D5D394;
	sub_82D56F10(ctx, base);
	// 82D5D394: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5D398: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D39C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5D3A0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D5D3A4: 4BFFB98D  bl 0x82d58d30
	ctx.lr = 0x82D5D3A8;
	sub_82D58D30(ctx, base);
	// 82D5D3A8: 4800021C  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D3AC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D3B0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D3B4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82D5D3B8: 40980024  bge cr6, 0x82d5d3dc
	if !ctx.cr[6].lt {
	pc = 0x82D5D3DC; continue 'dispatch;
	}
	// 82D5D3BC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D3C0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82D5D3C4: 41990008  bgt cr6, 0x82d5d3cc
	if ctx.cr[6].gt {
	pc = 0x82D5D3CC; continue 'dispatch;
	}
	// 82D5D3C8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82D5D3CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D3D0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5D3D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D3D8: 4BFF9B39  bl 0x82d56f10
	ctx.lr = 0x82D5D3DC;
	sub_82D56F10(ctx, base);
	// 82D5D3DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D3E0: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82D5D3E4: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82D5D3E8: 480001DC  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D3EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D3F0: 3BCB4D90  addi r30, r11, 0x4d90
	ctx.r[30].s64 = ctx.r[11].s64 + 19856;
	// 82D5D3F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5D3F8: 4BFFB831  bl 0x82d58c28
	ctx.lr = 0x82D5D3FC;
	sub_82D58C28(ctx, base);
	// 82D5D3FC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D400: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5D404: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D408: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5D40C: 40980024  bge cr6, 0x82d5d430
	if !ctx.cr[6].lt {
	pc = 0x82D5D430; continue 'dispatch;
	}
	// 82D5D410: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D414: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5D418: 41980008  blt cr6, 0x82d5d420
	if ctx.cr[6].lt {
	pc = 0x82D5D420; continue 'dispatch;
	}
	// 82D5D41C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5D420: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D424: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5D428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D42C: 4BFF9AE5  bl 0x82d56f10
	ctx.lr = 0x82D5D430;
	sub_82D56F10(ctx, base);
	// 82D5D430: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5D434: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D438: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5D43C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D5D440: 4BFFB8F1  bl 0x82d58d30
	ctx.lr = 0x82D5D444;
	sub_82D58D30(ctx, base);
	// 82D5D444: 48000180  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D448: 2F1F0016  cmpwi cr6, r31, 0x16
	ctx.cr[6].compare_i32(ctx.r[31].s32, 22, &mut ctx.xer);
	// 82D5D44C: 419A008C  beq cr6, 0x82d5d4d8
	if ctx.cr[6].eq {
	pc = 0x82D5D4D8; continue 'dispatch;
	}
	// 82D5D450: 2F1F001A  cmpwi cr6, r31, 0x1a
	ctx.cr[6].compare_i32(ctx.r[31].s32, 26, &mut ctx.xer);
	// 82D5D454: 419A0084  beq cr6, 0x82d5d4d8
	if ctx.cr[6].eq {
	pc = 0x82D5D4D8; continue 'dispatch;
	}
	// 82D5D458: 2F1F0018  cmpwi cr6, r31, 0x18
	ctx.cr[6].compare_i32(ctx.r[31].s32, 24, &mut ctx.xer);
	// 82D5D45C: 409A001C  bne cr6, 0x82d5d478
	if !ctx.cr[6].eq {
	pc = 0x82D5D478; continue 'dispatch;
	}
	// 82D5D460: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D464: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5D468: 388B4D88  addi r4, r11, 0x4d88
	ctx.r[4].s64 = ctx.r[11].s64 + 19848;
	// 82D5D46C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D470: 4BFFBA21  bl 0x82d58e90
	ctx.lr = 0x82D5D474;
	sub_82D58E90(ctx, base);
	// 82D5D474: 48000150  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D478: 2F1F001F  cmpwi cr6, r31, 0x1f
	ctx.cr[6].compare_i32(ctx.r[31].s32, 31, &mut ctx.xer);
	// 82D5D47C: 409A001C  bne cr6, 0x82d5d498
	if !ctx.cr[6].eq {
	pc = 0x82D5D498; continue 'dispatch;
	}
	// 82D5D480: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D484: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5D488: 388B4D7C  addi r4, r11, 0x4d7c
	ctx.r[4].s64 = ctx.r[11].s64 + 19836;
	// 82D5D48C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D490: 4BFFBA01  bl 0x82d58e90
	ctx.lr = 0x82D5D494;
	sub_82D58E90(ctx, base);
	// 82D5D494: 48000130  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D498: 2F1F0019  cmpwi cr6, r31, 0x19
	ctx.cr[6].compare_i32(ctx.r[31].s32, 25, &mut ctx.xer);
	// 82D5D49C: 409A0128  bne cr6, 0x82d5d5c4
	if !ctx.cr[6].eq {
	pc = 0x82D5D5C4; continue 'dispatch;
	}
	// 82D5D4A0: A17D000E  lhz r11, 0xe(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(14 as u32) ) } as u64;
	// 82D5D4A4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D5D4A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D4AC: 7D660734  extsh r6, r11
	ctx.r[6].s64 = ctx.r[11].s16 as i64;
	// 82D5D4B0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82D5D4B4: 409A0014  bne cr6, 0x82d5d4c8
	if !ctx.cr[6].eq {
	pc = 0x82D5D4C8; continue 'dispatch;
	}
	// 82D5D4B8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D4BC: 388B4D70  addi r4, r11, 0x4d70
	ctx.r[4].s64 = ctx.r[11].s64 + 19824;
	// 82D5D4C0: 4BFFB9D1  bl 0x82d58e90
	ctx.lr = 0x82D5D4C4;
	sub_82D58E90(ctx, base);
	// 82D5D4C4: 48000100  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D4C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D4CC: 388B4D60  addi r4, r11, 0x4d60
	ctx.r[4].s64 = ctx.r[11].s64 + 19808;
	// 82D5D4D0: 4BFFB9C1  bl 0x82d58e90
	ctx.lr = 0x82D5D4D4;
	sub_82D58E90(ctx, base);
	// 82D5D4D4: 480000F0  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D4D8: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D5D4DC: 2F1F0016  cmpwi cr6, r31, 0x16
	ctx.cr[6].compare_i32(ctx.r[31].s32, 22, &mut ctx.xer);
	// 82D5D4E0: 409A0010  bne cr6, 0x82d5d4f0
	if !ctx.cr[6].eq {
	pc = 0x82D5D4F0; continue 'dispatch;
	}
	// 82D5D4E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D5D4E8: 38AA4D58  addi r5, r10, 0x4d58
	ctx.r[5].s64 = ctx.r[10].s64 + 19800;
	// 82D5D4EC: 4800000C  b 0x82d5d4f8
	pc = 0x82D5D4F8; continue 'dispatch;
	// 82D5D4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D5D4F4: 38AA4D48  addi r5, r10, 0x4d48
	ctx.r[5].s64 = ctx.r[10].s64 + 19784;
	// 82D5D4F8: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82D5D4FC: 41980084  blt cr6, 0x82d5d580
	if ctx.cr[6].lt {
	pc = 0x82D5D580; continue 'dispatch;
	}
	// 82D5D500: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82D5D504: 419A007C  beq cr6, 0x82d5d580
	if ctx.cr[6].eq {
	pc = 0x82D5D580; continue 'dispatch;
	}
	// 82D5D508: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82D5D50C: 409A0038  bne cr6, 0x82d5d544
	if !ctx.cr[6].eq {
	pc = 0x82D5D544; continue 'dispatch;
	}
	// 82D5D510: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5D514: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D518: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5D51C: 419A0018  beq cr6, 0x82d5d534
	if ctx.cr[6].eq {
	pc = 0x82D5D534; continue 'dispatch;
	}
	// 82D5D520: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D524: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82D5D528: 388B4D38  addi r4, r11, 0x4d38
	ctx.r[4].s64 = ctx.r[11].s64 + 19768;
	// 82D5D52C: 4BFFB965  bl 0x82d58e90
	ctx.lr = 0x82D5D530;
	sub_82D58E90(ctx, base);
	// 82D5D530: 48000094  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D534: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D538: 388B4D28  addi r4, r11, 0x4d28
	ctx.r[4].s64 = ctx.r[11].s64 + 19752;
	// 82D5D53C: 4BFFB955  bl 0x82d58e90
	ctx.lr = 0x82D5D540;
	sub_82D58E90(ctx, base);
	// 82D5D540: 48000084  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D544: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 82D5D548: 409A001C  bne cr6, 0x82d5d564
	if !ctx.cr[6].eq {
	pc = 0x82D5D564; continue 'dispatch;
	}
	// 82D5D54C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D550: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82D5D554: 388B4D14  addi r4, r11, 0x4d14
	ctx.r[4].s64 = ctx.r[11].s64 + 19732;
	// 82D5D558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D55C: 4BFFB935  bl 0x82d58e90
	ctx.lr = 0x82D5D560;
	sub_82D58E90(ctx, base);
	// 82D5D560: 48000064  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D564: 2F0B001C  cmpwi cr6, r11, 0x1c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 28, &mut ctx.xer);
	// 82D5D568: 409A005C  bne cr6, 0x82d5d5c4
	if !ctx.cr[6].eq {
	pc = 0x82D5D5C4; continue 'dispatch;
	}
	// 82D5D56C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D574: 388B4CF8  addi r4, r11, 0x4cf8
	ctx.r[4].s64 = ctx.r[11].s64 + 19704;
	// 82D5D578: 4BFFB919  bl 0x82d58e90
	ctx.lr = 0x82D5D57C;
	sub_82D58E90(ctx, base);
	// 82D5D57C: 48000048  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D580: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D5D584: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D5D588: 394A73F0  addi r10, r10, 0x73f0
	ctx.r[10].s64 = ctx.r[10].s64 + 29680;
	// 82D5D58C: 38894CE8  addi r4, r9, 0x4ce8
	ctx.r[4].s64 = ctx.r[9].s64 + 19688;
	// 82D5D590: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 82D5D594: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5D598: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D59C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5D5A0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D5A4: 7CCB482E  lwzx r6, r11, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5D5A8: 4BFFB8E9  bl 0x82d58e90
	ctx.lr = 0x82D5D5AC;
	sub_82D58E90(ctx, base);
	// 82D5D5AC: 48000018  b 0x82d5d5c4
	pc = 0x82D5D5C4; continue 'dispatch;
	// 82D5D5B0: A17D000E  lhz r11, 0xe(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(14 as u32) ) } as u64;
	// 82D5D5B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D5D5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D5BC: 7D640734  extsh r4, r11
	ctx.r[4].s64 = ctx.r[11].s16 as i64;
	// 82D5D5C0: 4BFFFB91  bl 0x82d5d150
	ctx.lr = 0x82D5D5C4;
	sub_82D5D150(ctx, base);
	// 82D5D5C4: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82D5D5C8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D5CC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82D5D5D0: 4BFFB649  bl 0x82d58c18
	ctx.lr = 0x82D5D5D4;
	sub_82D58C18(ctx, base);
	// 82D5D5D4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D5D8: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D5D5DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5D5E0: 3BE9FFFF  addi r31, r9, -1
	ctx.r[31].s64 = ctx.r[9].s64 + -1;
	// 82D5D5E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5D5E8: 409A0018  bne cr6, 0x82d5d600
	if !ctx.cr[6].eq {
	pc = 0x82D5D600; continue 'dispatch;
	}
	// 82D5D5EC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5D5F0: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D5D5F4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D5F8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D5FC: 4BFF7CCD  bl 0x82d552c8
	ctx.lr = 0x82D5D600;
	sub_82D552C8(ctx, base);
	// 82D5D600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D604: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82D5D608: 4BF4BE38  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5D610 size=812
    let mut pc: u32 = 0x82D5D610;
    'dispatch: loop {
        match pc {
            0x82D5D610 => {
    //   block [0x82D5D610..0x82D5D93C)
	// 82D5D610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5D614: 4BF4BDF1  bl 0x82ca9404
	ctx.lr = 0x82D5D618;
	sub_82CA93D0(ctx, base);
	// 82D5D618: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5D61C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82D5D620: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82D5D624: 388B2A50  addi r4, r11, 0x2a50
	ctx.r[4].s64 = ctx.r[11].s64 + 10832;
	// 82D5D628: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5D62C: 4BFFB3DD  bl 0x82d58a08
	ctx.lr = 0x82D5D630;
	sub_82D58A08(ctx, base);
	// 82D5D630: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5D634: 409A0010  bne cr6, 0x82d5d644
	if !ctx.cr[6].eq {
	pc = 0x82D5D644; continue 'dispatch;
	}
	// 82D5D638: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82D5D63C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D640: 4BF4BE14  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5D644: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D648: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82D5D64C: 388B4DC8  addi r4, r11, 0x4dc8
	ctx.r[4].s64 = ctx.r[11].s64 + 19912;
	// 82D5D650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D654: 4BFFB3B5  bl 0x82d58a08
	ctx.lr = 0x82D5D658;
	sub_82D58A08(ctx, base);
	// 82D5D658: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5D65C: 409A0010  bne cr6, 0x82d5d66c
	if !ctx.cr[6].eq {
	pc = 0x82D5D66C; continue 'dispatch;
	}
	// 82D5D660: 3860001F  li r3, 0x1f
	ctx.r[3].s64 = 31;
	// 82D5D664: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D668: 4BF4BDEC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5D66C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D670: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5D674: 388B4DBC  addi r4, r11, 0x4dbc
	ctx.r[4].s64 = ctx.r[11].s64 + 19900;
	// 82D5D678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D67C: 4BFFB38D  bl 0x82d58a08
	ctx.lr = 0x82D5D680;
	sub_82D58A08(ctx, base);
	// 82D5D680: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5D684: 409A0010  bne cr6, 0x82d5d694
	if !ctx.cr[6].eq {
	pc = 0x82D5D694; continue 'dispatch;
	}
	// 82D5D688: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 82D5D68C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D690: 4BF4BDC4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5D694: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D698: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 82D5D69C: 388B4DAC  addi r4, r11, 0x4dac
	ctx.r[4].s64 = ctx.r[11].s64 + 19884;
	// 82D5D6A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D6A4: 4BFFB365  bl 0x82d58a08
	ctx.lr = 0x82D5D6A8;
	sub_82D58A08(ctx, base);
	// 82D5D6A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5D6AC: 409A0010  bne cr6, 0x82d5d6bc
	if !ctx.cr[6].eq {
	pc = 0x82D5D6BC; continue 'dispatch;
	}
	// 82D5D6B0: 3860001A  li r3, 0x1a
	ctx.r[3].s64 = 26;
	// 82D5D6B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D6B8: 4BF4BD9C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5D6BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D6C0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82D5D6C4: 388B4D98  addi r4, r11, 0x4d98
	ctx.r[4].s64 = ctx.r[11].s64 + 19864;
	// 82D5D6C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D6CC: 4BFFB33D  bl 0x82d58a08
	ctx.lr = 0x82D5D6D0;
	sub_82D58A08(ctx, base);
	// 82D5D6D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5D6D4: 409A0010  bne cr6, 0x82d5d6e4
	if !ctx.cr[6].eq {
	pc = 0x82D5D6E4; continue 'dispatch;
	}
	// 82D5D6D8: 3860001D  li r3, 0x1d
	ctx.r[3].s64 = 29;
	// 82D5D6DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D6E0: 4BF4BD74  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5D6E4: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 82D5D6E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D6EC: 4BFFB5AD  bl 0x82d58c98
	ctx.lr = 0x82D5D6F0;
	sub_82D58C98(ctx, base);
	// 82D5D6F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5D6F4: 419A001C  beq cr6, 0x82d5d710
	if ctx.cr[6].eq {
	pc = 0x82D5D710; continue 'dispatch;
	}
	// 82D5D6F8: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 82D5D6FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5D700: 409A0010  bne cr6, 0x82d5d710
	if !ctx.cr[6].eq {
	pc = 0x82D5D710; continue 'dispatch;
	}
	// 82D5D704: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82D5D708: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D70C: 4BF4BD48  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5D710: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D5D714: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 82D5D718: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D5D71C: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82D5D720: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82D5D724: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82D5D728: 419A0058  beq cr6, 0x82d5d780
	if ctx.cr[6].eq {
	pc = 0x82D5D780; continue 'dispatch;
	}
	// 82D5D72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D730: 4BFFB4F9  bl 0x82d58c28
	ctx.lr = 0x82D5D734;
	sub_82D58C28(ctx, base);
	// 82D5D734: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D738: 3BC30001  addi r30, r3, 1
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	// 82D5D73C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D740: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D5D744: 40980024  bge cr6, 0x82d5d768
	if !ctx.cr[6].lt {
	pc = 0x82D5D768; continue 'dispatch;
	}
	// 82D5D748: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5D74C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5D750: 41980008  blt cr6, 0x82d5d758
	if ctx.cr[6].lt {
	pc = 0x82D5D758; continue 'dispatch;
	}
	// 82D5D754: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D5D758: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D75C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5D760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D764: 4BFF97AD  bl 0x82d56f10
	ctx.lr = 0x82D5D768;
	sub_82D56F10(ctx, base);
	// 82D5D768: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5D76C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D770: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5D774: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82D5D778: 4BFFB5B9  bl 0x82d58d30
	ctx.lr = 0x82D5D77C;
	sub_82D58D30(ctx, base);
	// 82D5D77C: 48000024  b 0x82d5d7a0
	pc = 0x82D5D7A0; continue 'dispatch;
	// 82D5D780: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D784: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5D788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D78C: 4BFF9785  bl 0x82d56f10
	ctx.lr = 0x82D5D790;
	sub_82D56F10(ctx, base);
	// 82D5D790: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5D794: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D5D798: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D79C: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82D5D7A0: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 82D5D7A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5D7A8: 4BFFB4E9  bl 0x82d58c90
	ctx.lr = 0x82D5D7AC;
	sub_82D58C90(ctx, base);
	// 82D5D7AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5D7B0: 419A00E4  beq cr6, 0x82d5d894
	if ctx.cr[6].eq {
	pc = 0x82D5D894; continue 'dispatch;
	}
	// 82D5D7B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D5D7B8: 7FFF1850  subf r31, r31, r3
	ctx.r[31].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82D5D7BC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5D7C0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5D7C4: 40990008  ble cr6, 0x82d5d7cc
	if !ctx.cr[6].gt {
	pc = 0x82D5D7CC; continue 'dispatch;
	}
	// 82D5D7C8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D5D7CC: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 82D5D7D0: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D7D4: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82D5D7D8: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82D5D7DC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D5D7E0: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 82D5D7E4: 4099001C  ble cr6, 0x82d5d800
	if !ctx.cr[6].gt {
	pc = 0x82D5D800; continue 'dispatch;
	}
	// 82D5D7E8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D5D7EC: 41980008  blt cr6, 0x82d5d7f4
	if ctx.cr[6].lt {
	pc = 0x82D5D7F4; continue 'dispatch;
	}
	// 82D5D7F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5D7F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D7F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5D7FC: 4BFF9715  bl 0x82d56f10
	ctx.lr = 0x82D5D800;
	sub_82D56F10(ctx, base);
	// 82D5D800: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5D804: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5D808: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5D80C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82D5D810: 4BFFB521  bl 0x82d58d30
	ctx.lr = 0x82D5D814;
	sub_82D58D30(ctx, base);
	// 82D5D814: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5D818: 7F6BF9AE  stbx r27, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[27].u8) };
	// 82D5D81C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D820: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D824: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D5D828: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D5D82C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5D830: 40980024  bge cr6, 0x82d5d854
	if !ctx.cr[6].lt {
	pc = 0x82D5D854; continue 'dispatch;
	}
	// 82D5D834: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5D838: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D5D83C: 40980008  bge cr6, 0x82d5d844
	if !ctx.cr[6].lt {
	pc = 0x82D5D844; continue 'dispatch;
	}
	// 82D5D840: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82D5D844: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D848: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5D84C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D850: 4BFF96C1  bl 0x82d56f10
	ctx.lr = 0x82D5D854;
	sub_82D56F10(ctx, base);
	// 82D5D854: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5D858: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5D85C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D860: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D5D864: 4BFFB4CD  bl 0x82d58d30
	ctx.lr = 0x82D5D868;
	sub_82D58D30(ctx, base);
	// 82D5D868: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D5D86C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5D870: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5D874: 409A0020  bne cr6, 0x82d5d894
	if !ctx.cr[6].eq {
	pc = 0x82D5D894; continue 'dispatch;
	}
	// 82D5D878: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D87C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5D880: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5D884: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5D888: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D88C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5D890: 4BFF7A39  bl 0x82d552c8
	ctx.lr = 0x82D5D894;
	sub_82D552C8(ctx, base);
	// 82D5D894: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D5D898: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82D5D89C: 3BAB73F0  addi r29, r11, 0x73f0
	ctx.r[29].s64 = ctx.r[11].s64 + 29680;
	// 82D5D8A0: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 82D5D8A4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D8A8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D8AC: 4BFFB12D  bl 0x82d589d8
	ctx.lr = 0x82D5D8B0;
	sub_82D589D8(ctx, base);
	// 82D5D8B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5D8B4: 419A0050  beq cr6, 0x82d5d904
	if ctx.cr[6].eq {
	pc = 0x82D5D904; continue 'dispatch;
	}
	// 82D5D8B8: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82D5D8BC: 397D0184  addi r11, r29, 0x184
	ctx.r[11].s64 = ctx.r[29].s64 + 388;
	// 82D5D8C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82D5D8C4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5D8C8: 4198FFDC  blt cr6, 0x82d5d8a4
	if ctx.cr[6].lt {
	pc = 0x82D5D8A4; continue 'dispatch;
	}
	// 82D5D8CC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D8D0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5D8D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5D8D8: 409A0020  bne cr6, 0x82d5d8f8
	if !ctx.cr[6].eq {
	pc = 0x82D5D8F8; continue 'dispatch;
	}
	// 82D5D8DC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D8E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5D8E4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5D8E8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D8EC: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D8F0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5D8F4: 4BFF79D5  bl 0x82d552c8
	ctx.lr = 0x82D5D8F8;
	sub_82D552C8(ctx, base);
	// 82D5D8F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5D8FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D900: 4BF4BB54  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5D904: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D908: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5D90C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5D910: 409A0020  bne cr6, 0x82d5d930
	if !ctx.cr[6].eq {
	pc = 0x82D5D930; continue 'dispatch;
	}
	// 82D5D914: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D918: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5D91C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5D920: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D924: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D928: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5D92C: 4BFF799D  bl 0x82d552c8
	ctx.lr = 0x82D5D930;
	sub_82D552C8(ctx, base);
	// 82D5D930: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5D934: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5D938: 4BF4BB1C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5D940 size=612
    let mut pc: u32 = 0x82D5D940;
    'dispatch: loop {
        match pc {
            0x82D5D940 => {
    //   block [0x82D5D940..0x82D5DBA4)
	// 82D5D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5D944: 4BF4BAC5  bl 0x82ca9408
	ctx.lr = 0x82D5D948;
	sub_82CA93D0(ctx, base);
	// 82D5D948: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5D94C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5D950: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5D954: 388B4DBC  addi r4, r11, 0x4dbc
	ctx.r[4].s64 = ctx.r[11].s64 + 19900;
	// 82D5D958: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5D95C: 4BFFB0AD  bl 0x82d58a08
	ctx.lr = 0x82D5D960;
	sub_82D58A08(ctx, base);
	// 82D5D960: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5D964: 409A00AC  bne cr6, 0x82d5da10
	if !ctx.cr[6].eq {
	pc = 0x82D5DA10; continue 'dispatch;
	}
	// 82D5D968: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82D5D96C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5D970: 4BFFB2B9  bl 0x82d58c28
	ctx.lr = 0x82D5D974;
	sub_82D58C28(ctx, base);
	// 82D5D974: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D5D978: 3BC3FFFF  addi r30, r3, -1
	ctx.r[30].s64 = ctx.r[3].s64 + -1;
	// 82D5D97C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82D5D980: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 82D5D984: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82D5D988: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5D98C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D5D990: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82D5D994: 4099001C  ble cr6, 0x82d5d9b0
	if !ctx.cr[6].gt {
	pc = 0x82D5D9B0; continue 'dispatch;
	}
	// 82D5D998: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5D99C: 41980008  blt cr6, 0x82d5d9a4
	if ctx.cr[6].lt {
	pc = 0x82D5D9A4; continue 'dispatch;
	}
	// 82D5D9A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5D9A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5D9A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5D9AC: 4BFF9565  bl 0x82d56f10
	ctx.lr = 0x82D5D9B0;
	sub_82D56F10(ctx, base);
	// 82D5D9B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5D9B4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D9B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5D9BC: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82D5D9C0: 4BFFB371  bl 0x82d58d30
	ctx.lr = 0x82D5D9C4;
	sub_82D58D30(ctx, base);
	// 82D5D9C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D9C8: 7FEBF1AE  stbx r31, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u8) };
	// 82D5D9CC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D9D0: 4BFFFC41  bl 0x82d5d610
	ctx.lr = 0x82D5D9D4;
	sub_82D5D610(ctx, base);
	// 82D5D9D4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5D9D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5D9DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5D9E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5D9E4: 409A0020  bne cr6, 0x82d5da04
	if !ctx.cr[6].eq {
	pc = 0x82D5DA04; continue 'dispatch;
	}
	// 82D5D9E8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5D9EC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5D9F0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5D9F4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5D9F8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5D9FC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5DA00: 4BFF78C9  bl 0x82d552c8
	ctx.lr = 0x82D5DA04;
	sub_82D552C8(ctx, base);
	// 82D5DA04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DA08: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D5DA0C: 4BF4BA4C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D5DA10: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5DA14: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 82D5DA18: 388B4DAC  addi r4, r11, 0x4dac
	ctx.r[4].s64 = ctx.r[11].s64 + 19884;
	// 82D5DA1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DA20: 4BFFAFE9  bl 0x82d58a08
	ctx.lr = 0x82D5DA24;
	sub_82D58A08(ctx, base);
	// 82D5DA24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5DA28: 409A00AC  bne cr6, 0x82d5dad4
	if !ctx.cr[6].eq {
	pc = 0x82D5DAD4; continue 'dispatch;
	}
	// 82D5DA2C: 3B9F000E  addi r28, r31, 0xe
	ctx.r[28].s64 = ctx.r[31].s64 + 14;
	// 82D5DA30: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5DA34: 4BFFB1F5  bl 0x82d58c28
	ctx.lr = 0x82D5DA38;
	sub_82D58C28(ctx, base);
	// 82D5DA38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D5DA3C: 3BC3FFFF  addi r30, r3, -1
	ctx.r[30].s64 = ctx.r[3].s64 + -1;
	// 82D5DA40: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82D5DA44: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 82D5DA48: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82D5DA4C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5DA50: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82D5DA54: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82D5DA58: 4099001C  ble cr6, 0x82d5da74
	if !ctx.cr[6].gt {
	pc = 0x82D5DA74; continue 'dispatch;
	}
	// 82D5DA5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5DA60: 41980008  blt cr6, 0x82d5da68
	if ctx.cr[6].lt {
	pc = 0x82D5DA68; continue 'dispatch;
	}
	// 82D5DA64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5DA68: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5DA6C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5DA70: 4BFF94A1  bl 0x82d56f10
	ctx.lr = 0x82D5DA74;
	sub_82D56F10(ctx, base);
	// 82D5DA74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5DA78: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5DA7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5DA80: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82D5DA84: 4BFFB2AD  bl 0x82d58d30
	ctx.lr = 0x82D5DA88;
	sub_82D58D30(ctx, base);
	// 82D5DA88: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5DA8C: 7FEBF1AE  stbx r31, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u8) };
	// 82D5DA90: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5DA94: 4BFFFB7D  bl 0x82d5d610
	ctx.lr = 0x82D5DA98;
	sub_82D5D610(ctx, base);
	// 82D5DA98: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D5DA9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5DAA0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5DAA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5DAA8: 409A0020  bne cr6, 0x82d5dac8
	if !ctx.cr[6].eq {
	pc = 0x82D5DAC8; continue 'dispatch;
	}
	// 82D5DAAC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DAB0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5DAB4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5DAB8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5DABC: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5DAC0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5DAC4: 4BFF7805  bl 0x82d552c8
	ctx.lr = 0x82D5DAC8;
	sub_82D552C8(ctx, base);
	// 82D5DAC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DACC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D5DAD0: 4BF4B988  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D5DAD4: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 82D5DAD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DADC: 4BFFB1B5  bl 0x82d58c90
	ctx.lr = 0x82D5DAE0;
	sub_82D58C90(ctx, base);
	// 82D5DAE0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D5DAE4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82D5DAE8: 419A00B0  beq cr6, 0x82d5db98
	if ctx.cr[6].eq {
	pc = 0x82D5DB98; continue 'dispatch;
	}
	// 82D5DAEC: 3880005D  li r4, 0x5d
	ctx.r[4].s64 = 93;
	// 82D5DAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DAF4: 4BFFB19D  bl 0x82d58c90
	ctx.lr = 0x82D5DAF8;
	sub_82D58C90(ctx, base);
	// 82D5DAF8: 7D7C1850  subf r11, r28, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[28].s64;
	// 82D5DAFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D5DB00: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82D5DB04: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82D5DB08: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 82D5DB0C: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82D5DB10: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5DB14: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82D5DB18: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82D5DB1C: 4099001C  ble cr6, 0x82d5db38
	if !ctx.cr[6].gt {
	pc = 0x82D5DB38; continue 'dispatch;
	}
	// 82D5DB20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5DB24: 41980008  blt cr6, 0x82d5db2c
	if ctx.cr[6].lt {
	pc = 0x82D5DB2C; continue 'dispatch;
	}
	// 82D5DB28: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5DB2C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5DB30: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D5DB34: 4BFF93DD  bl 0x82d56f10
	ctx.lr = 0x82D5DB38;
	sub_82D56F10(ctx, base);
	// 82D5DB38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5DB3C: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D5DB40: 389C0001  addi r4, r28, 1
	ctx.r[4].s64 = ctx.r[28].s64 + 1;
	// 82D5DB44: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82D5DB48: 4BFFB1E9  bl 0x82d58d30
	ctx.lr = 0x82D5DB4C;
	sub_82D58D30(ctx, base);
	// 82D5DB4C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D5DB50: 7FEBF1AE  stbx r31, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u8) };
	// 82D5DB54: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D5DB58: 4BFFB0F9  bl 0x82d58c50
	ctx.lr = 0x82D5DB5C;
	sub_82D58C50(ctx, base);
	// 82D5DB5C: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D5DB60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5DB64: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5DB68: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5DB6C: 409A0020  bne cr6, 0x82d5db8c
	if !ctx.cr[6].eq {
	pc = 0x82D5DB8C; continue 'dispatch;
	}
	// 82D5DB70: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DB74: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5DB78: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5DB7C: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D5DB80: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5DB84: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5DB88: 4BFF7741  bl 0x82d552c8
	ctx.lr = 0x82D5DB8C;
	sub_82D552C8(ctx, base);
	// 82D5DB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DB90: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D5DB94: 4BF4B8C4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D5DB98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5DB9C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D5DBA0: 4BF4B8B8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5DBA8 size=60
    let mut pc: u32 = 0x82D5DBA8;
    'dispatch: loop {
        match pc {
            0x82D5DBA8 => {
    //   block [0x82D5DBA8..0x82D5DBE4)
	// 82D5DBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5DBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5DBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5DBB4: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DBB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5DBBC: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82D5DBC0: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 82D5DBC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DBC8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5DBCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DBD0: 4E800421  bctrl
	ctx.lr = 0x82D5DBD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5DBD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5DBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5DBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5DBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5DBE8 size=60
    let mut pc: u32 = 0x82D5DBE8;
    'dispatch: loop {
        match pc {
            0x82D5DBE8 => {
    //   block [0x82D5DBE8..0x82D5DC24)
	// 82D5DBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5DBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5DBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5DBF4: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DBF8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5DBFC: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82D5DC00: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 82D5DC04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DC08: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5DC0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DC10: 4E800421  bctrl
	ctx.lr = 0x82D5DC14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5DC14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5DC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5DC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5DC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5DC28 size=20
    let mut pc: u32 = 0x82D5DC28;
    'dispatch: loop {
        match pc {
            0x82D5DC28 => {
    //   block [0x82D5DC28..0x82D5DC3C)
	// 82D5DC28: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DC2C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DC30: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82D5DC34: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5DC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D5DC40 size=456
    let mut pc: u32 = 0x82D5DC40;
    'dispatch: loop {
        match pc {
            0x82D5DC40 => {
    //   block [0x82D5DC40..0x82D5DE08)
	// 82D5DC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5DC44: 4BF4B7B5  bl 0x82ca93f8
	ctx.lr = 0x82D5DC48;
	sub_82CA93D0(ctx, base);
	// 82D5DC48: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5DC4C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82D5DC50: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D5DC54: 8978000C  lbz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5DC58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5DC5C: 409A0024  bne cr6, 0x82d5dc80
	if !ctx.cr[6].eq {
	pc = 0x82D5DC80; continue 'dispatch;
	}
	// 82D5DC60: 80780008  lwz r3, 8(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DC64: 7CBC31D6  mullw r5, r28, r6
	ctx.r[5].s64 = (ctx.r[28].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82D5DC68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DC6C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5DC70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DC74: 4E800421  bctrl
	ctx.lr = 0x82D5DC78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5DC78: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82D5DC7C: 4BF4B7CC  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82D5DC80: 7FFC31D6  mullw r31, r28, r6
	ctx.r[31].s64 = (ctx.r[28].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82D5DC84: 7FEB4E70  srawi r11, r31, 9
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 9) as i64;
	// 82D5DC88: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 82D5DC8C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82D5DC90: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82D5DC94: 7FBEE3D6  divw r29, r30, r28
	ctx.r[29].s32 = ctx.r[30].s32 / ctx.r[28].s32;
	// 82D5DC98: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5DC9C: 0CDC0000  twi 6, r28, 0
	// 82D5DCA0: 7F4BF850  subf r26, r11, r31
	ctx.r[26].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82D5DCA4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D5DCA8: 574B083E  rotlwi r11, r26, 1
	ctx.r[11].u64 = ((ctx.r[26].u32).rotate_left(1)) as u64;
	// 82D5DCAC: 7F3AE3D6  divw r25, r26, r28
	ctx.r[25].s32 = ctx.r[26].s32 / ctx.r[28].s32;
	// 82D5DCB0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5DCB4: 0CDC0000  twi 6, r28, 0
	// 82D5DCB8: 7F8B5878  andc r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 & !ctx.r[11].u64;
	// 82D5DCBC: 0CABFFFF  twi 5, r11, -1
	// 82D5DCC0: 40990140  ble cr6, 0x82d5de00
	if !ctx.cr[6].gt {
	pc = 0x82D5DE00; continue 'dispatch;
	}
	// 82D5DCC4: 2F1F0200  cmpwi cr6, r31, 0x200
	ctx.cr[6].compare_i32(ctx.r[31].s32, 512, &mut ctx.xer);
	// 82D5DCC8: 4098000C  bge cr6, 0x82d5dcd4
	if !ctx.cr[6].lt {
	pc = 0x82D5DCD4; continue 'dispatch;
	}
	// 82D5DCCC: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82D5DCD0: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82D5DCD4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5DCD8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D5DCDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5DCE0: 4BFFB051  bl 0x82d58d30
	ctx.lr = 0x82D5DCE4;
	sub_82D58D30(ctx, base);
	// 82D5DCE4: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 82D5DCE8: 419A00BC  beq cr6, 0x82d5dda4
	if ctx.cr[6].eq {
	pc = 0x82D5DDA4; continue 'dispatch;
	}
	// 82D5DCEC: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82D5DCF0: 419A0070  beq cr6, 0x82d5dd60
	if ctx.cr[6].eq {
	pc = 0x82D5DD60; continue 'dispatch;
	}
	// 82D5DCF4: 2F1C0008  cmpwi cr6, r28, 8
	ctx.cr[6].compare_i32(ctx.r[28].s32, 8, &mut ctx.xer);
	// 82D5DCF8: 409A00DC  bne cr6, 0x82d5ddd4
	if !ctx.cr[6].eq {
	pc = 0x82D5DDD4; continue 'dispatch;
	}
	// 82D5DCFC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5DD00: 409900D4  ble cr6, 0x82d5ddd4
	if !ctx.cr[6].gt {
	pc = 0x82D5DDD4; continue 'dispatch;
	}
	// 82D5DD04: 39610056  addi r11, r1, 0x56
	ctx.r[11].s64 = ctx.r[1].s64 + 86;
	// 82D5DD08: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82D5DD0C: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82D5DD10: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D5DD14: 892BFFFA  lbz r9, -6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-6 as u32) ) } as u64;
	// 82D5DD18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5DD1C: 990BFFFA  stb r8, -6(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-6 as u32), ctx.r[8].u8 ) };
	// 82D5DD20: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 82D5DD24: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DD28: 892BFFFB  lbz r9, -5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-5 as u32) ) } as u64;
	// 82D5DD2C: 990BFFFB  stb r8, -5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-5 as u32), ctx.r[8].u8 ) };
	// 82D5DD30: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82D5DD34: 890BFFFF  lbz r8, -1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82D5DD38: 892BFFFC  lbz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82D5DD3C: 990BFFFC  stb r8, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u8 ) };
	// 82D5DD40: 992BFFFF  stb r9, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 82D5DD44: 890BFFFE  lbz r8, -2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82D5DD48: 892BFFFD  lbz r9, -3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-3 as u32) ) } as u64;
	// 82D5DD4C: 990BFFFD  stb r8, -3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-3 as u32), ctx.r[8].u8 ) };
	// 82D5DD50: 992BFFFE  stb r9, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[9].u8 ) };
	// 82D5DD54: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82D5DD58: 409AFFB4  bne cr6, 0x82d5dd0c
	if !ctx.cr[6].eq {
	pc = 0x82D5DD0C; continue 'dispatch;
	}
	// 82D5DD5C: 48000078  b 0x82d5ddd4
	pc = 0x82D5DDD4; continue 'dispatch;
	// 82D5DD60: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5DD64: 40990070  ble cr6, 0x82d5ddd4
	if !ctx.cr[6].gt {
	pc = 0x82D5DDD4; continue 'dispatch;
	}
	// 82D5DD68: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 82D5DD6C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82D5DD70: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82D5DD74: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D5DD78: 892BFFFE  lbz r9, -2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82D5DD7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5DD80: 990BFFFE  stb r8, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[8].u8 ) };
	// 82D5DD84: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 82D5DD88: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DD8C: 892BFFFF  lbz r9, -1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82D5DD90: 990BFFFF  stb r8, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[8].u8 ) };
	// 82D5DD94: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82D5DD98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D5DD9C: 409AFFD4  bne cr6, 0x82d5dd70
	if !ctx.cr[6].eq {
	pc = 0x82D5DD70; continue 'dispatch;
	}
	// 82D5DDA0: 48000034  b 0x82d5ddd4
	pc = 0x82D5DDD4; continue 'dispatch;
	// 82D5DDA4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D5DDA8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5DDAC: 40990028  ble cr6, 0x82d5ddd4
	if !ctx.cr[6].gt {
	pc = 0x82D5DDD4; continue 'dispatch;
	}
	// 82D5DDB0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82D5DDB4: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82D5DDB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D5DDBC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DDC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5DDC4: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82D5DDC8: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 82D5DDCC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82D5DDD0: 409AFFE4  bne cr6, 0x82d5ddb4
	if !ctx.cr[6].eq {
	pc = 0x82D5DDB4; continue 'dispatch;
	}
	// 82D5DDD4: 80780008  lwz r3, 8(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DDD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5DDDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5DDE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DDE4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5DDE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DDEC: 4E800421  bctrl
	ctx.lr = 0x82D5DDF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5DDF0: 7FFEF850  subf r31, r30, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 82D5DDF4: 7F7EDA14  add r27, r30, r27
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 82D5DDF8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D5DDFC: 4199FEC8  bgt cr6, 0x82d5dcc4
	if ctx.cr[6].gt {
	pc = 0x82D5DCC4; continue 'dispatch;
	}
	// 82D5DE00: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82D5DE04: 4BF4B644  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5DE08 size=20
    let mut pc: u32 = 0x82D5DE08;
    'dispatch: loop {
        match pc {
            0x82D5DE08 => {
    //   block [0x82D5DE08..0x82D5DE1C)
	// 82D5DE08: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DE0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DE10: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5DE14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DE18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5DE20 size=8
    let mut pc: u32 = 0x82D5DE20;
    'dispatch: loop {
        match pc {
            0x82D5DE20 => {
    //   block [0x82D5DE20..0x82D5DE28)
	// 82D5DE20: 9883000C  stb r4, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u8 ) };
	// 82D5DE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5DE28 size=12
    let mut pc: u32 = 0x82D5DE28;
    'dispatch: loop {
        match pc {
            0x82D5DE28 => {
    //   block [0x82D5DE28..0x82D5DE34)
	// 82D5DE28: 8964000C  lbz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5DE2C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5DE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5DE38 size=64
    let mut pc: u32 = 0x82D5DE38;
    'dispatch: loop {
        match pc {
            0x82D5DE38 => {
    //   block [0x82D5DE38..0x82D5DE78)
	// 82D5DE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5DE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5DE40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5DE44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5DE48: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DE4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5DE50: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DE54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5DE58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DE5C: 4E800421  bctrl
	ctx.lr = 0x82D5DE60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5DE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DE64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5DE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5DE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5DE70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5DE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5DE78 size=8
    let mut pc: u32 = 0x82D5DE78;
    'dispatch: loop {
        match pc {
            0x82D5DE78 => {
    //   block [0x82D5DE78..0x82D5DE80)
	// 82D5DE78: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5DE80 size=140
    let mut pc: u32 = 0x82D5DE80;
    'dispatch: loop {
        match pc {
            0x82D5DE80 => {
    //   block [0x82D5DE80..0x82D5DF0C)
	// 82D5DE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5DE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5DE88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5DE8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5DE90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5DE94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5DE98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5DE9C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5DEA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5DEA4: 419A0010  beq cr6, 0x82d5deb4
	if ctx.cr[6].eq {
	pc = 0x82D5DEB4; continue 'dispatch;
	}
	// 82D5DEA8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5DEAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5DEB0: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5DEB4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5DEB8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5DEBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5DEC0: 419A0030  beq cr6, 0x82d5def0
	if ctx.cr[6].eq {
	pc = 0x82D5DEF0; continue 'dispatch;
	}
	// 82D5DEC4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5DEC8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5DECC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D5DED0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5DED4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5DED8: 409A0018  bne cr6, 0x82d5def0
	if !ctx.cr[6].eq {
	pc = 0x82D5DEF0; continue 'dispatch;
	}
	// 82D5DEDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DEE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5DEE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DEE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DEEC: 4E800421  bctrl
	ctx.lr = 0x82D5DEF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5DEF0: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82D5DEF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5DEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5DEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5DF00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5DF04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5DF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5DF10 size=40
    let mut pc: u32 = 0x82D5DF10;
    'dispatch: loop {
        match pc {
            0x82D5DF10 => {
    //   block [0x82D5DF10..0x82D5DF38)
	// 82D5DF10: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5DF14: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82D5DF18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5DF1C: 98A3000C  stb r5, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u8 ) };
	// 82D5DF20: 396B4F1C  addi r11, r11, 0x4f1c
	ctx.r[11].s64 = ctx.r[11].s64 + 20252;
	// 82D5DF24: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D5DF28: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5DF2C: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5DF30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5DF34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5DF38 size=16
    let mut pc: u32 = 0x82D5DF38;
    'dispatch: loop {
        match pc {
            0x82D5DF38 => {
    //   block [0x82D5DF38..0x82D5DF48)
	// 82D5DF38: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5DF3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5DF40: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5DF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5DF48 size=100
    let mut pc: u32 = 0x82D5DF48;
    'dispatch: loop {
        match pc {
            0x82D5DF48 => {
    //   block [0x82D5DF48..0x82D5DFAC)
	// 82D5DF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5DF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5DF50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5DF54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5DF58: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5DF5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5DF60: 396B4F1C  addi r11, r11, 0x4f1c
	ctx.r[11].s64 = ctx.r[11].s64 + 20252;
	// 82D5DF64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5DF68: 98BF000C  stb r5, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u8 ) };
	// 82D5DF6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5DF70: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D5DF74: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D5DF78: 806B7700  lwz r3, 0x7700(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30464 as u32) ) } as u64;
	// 82D5DF7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DF80: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5DF84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5DF88: 4E800421  bctrl
	ctx.lr = 0x82D5DF8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5DF8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5DF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5DF94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D5DF98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5DF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5DFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5DFA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5DFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5DFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5DFB0 size=128
    let mut pc: u32 = 0x82D5DFB0;
    'dispatch: loop {
        match pc {
            0x82D5DFB0 => {
    //   block [0x82D5DFB0..0x82D5E030)
	// 82D5DFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5DFB4: 4BF4B459  bl 0x82ca940c
	ctx.lr = 0x82D5DFB8;
	sub_82CA93D0(ctx, base);
	// 82D5DFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5DFBC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5DFC0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5DFC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5DFC8: 396B4F1C  addi r11, r11, 0x4f1c
	ctx.r[11].s64 = ctx.r[11].s64 + 20252;
	// 82D5DFCC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D5DFD0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82D5DFD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5DFD8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D5DFDC: 98DF000C  stb r6, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u8 ) };
	// 82D5DFE0: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D5DFE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5DFE8: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82D5DFEC: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D5DFF0: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5DFF4: 4BFF7255  bl 0x82d55248
	ctx.lr = 0x82D5DFF8;
	sub_82D55248(ctx, base);
	// 82D5DFF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D5DFFC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D5E000: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 82D5E004: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D5E008: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5E00C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82D5E010: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E014: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D5E018: 48001CC9  bl 0x82d5fce0
	ctx.lr = 0x82D5E01C;
	sub_82D5FCE0(ctx, base);
	// 82D5E01C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5E020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5E024: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D5E028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5E02C: 4BF4B430  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E030 size=136
    let mut pc: u32 = 0x82D5E030;
    'dispatch: loop {
        match pc {
            0x82D5E030 => {
    //   block [0x82D5E030..0x82D5E0B8)
	// 82D5E030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E034: 4BF4B3D5  bl 0x82ca9408
	ctx.lr = 0x82D5E038;
	sub_82CA93D0(ctx, base);
	// 82D5E038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E03C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D5E040: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E044: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5E048: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D5E04C: 394A4F1C  addi r10, r10, 0x4f1c
	ctx.r[10].s64 = ctx.r[10].s64 + 20252;
	// 82D5E050: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82D5E054: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82D5E058: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D5E05C: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D5E060: 997E000C  stb r11, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82D5E064: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82D5E068: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D5E06C: B39E0006  sth r28, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82D5E070: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5E074: 4BFF71D5  bl 0x82d55248
	ctx.lr = 0x82D5E078;
	sub_82D55248(ctx, base);
	// 82D5E078: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5E07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5E080: 396B3BA8  addi r11, r11, 0x3ba8
	ctx.r[11].s64 = ctx.r[11].s64 + 15272;
	// 82D5E084: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82D5E088: B39F0006  sth r28, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82D5E08C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5E090: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D5E094: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82D5E098: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E09C: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82D5E0A0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D5E0A4: 4BFFA4F5  bl 0x82d58598
	ctx.lr = 0x82D5E0A8;
	sub_82D58598(ctx, base);
	// 82D5E0A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5E0AC: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82D5E0B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5E0B4: 4BF4B3A4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E0B8 size=124
    let mut pc: u32 = 0x82D5E0B8;
    'dispatch: loop {
        match pc {
            0x82D5E0B8 => {
    //   block [0x82D5E0B8..0x82D5E134)
	// 82D5E0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E0C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5E0C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E0C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5E0CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5E0D0: 396B4F1C  addi r11, r11, 0x4f1c
	ctx.r[11].s64 = ctx.r[11].s64 + 20252;
	// 82D5E0D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E0D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5E0DC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E0E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5E0E4: 419A0030  beq cr6, 0x82d5e114
	if ctx.cr[6].eq {
	pc = 0x82D5E114; continue 'dispatch;
	}
	// 82D5E0E8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5E0EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5E0F0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D5E0F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5E0F8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5E0FC: 409A0018  bne cr6, 0x82d5e114
	if !ctx.cr[6].eq {
	pc = 0x82D5E114; continue 'dispatch;
	}
	// 82D5E100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E104: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5E108: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E10C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5E110: 4E800421  bctrl
	ctx.lr = 0x82D5E114;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5E114: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5E118: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D5E11C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5E120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E12C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5E130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E138 size=48
    let mut pc: u32 = 0x82D5E138;
    'dispatch: loop {
        match pc {
            0x82D5E138 => {
    //   block [0x82D5E138..0x82D5E168)
	// 82D5E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E144: B081007E  sth r4, 0x7e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[4].u16 ) };
	// 82D5E148: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E14C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82D5E150: 3881007E  addi r4, r1, 0x7e
	ctx.r[4].s64 = ctx.r[1].s64 + 126;
	// 82D5E154: 4BFFFAED  bl 0x82d5dc40
	ctx.lr = 0x82D5E158;
	sub_82D5DC40(ctx, base);
	// 82D5E158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E168 size=48
    let mut pc: u32 = 0x82D5E168;
    'dispatch: loop {
        match pc {
            0x82D5E168 => {
    //   block [0x82D5E168..0x82D5E198)
	// 82D5E168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E174: B081007E  sth r4, 0x7e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[4].u16 ) };
	// 82D5E178: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E17C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82D5E180: 3881007E  addi r4, r1, 0x7e
	ctx.r[4].s64 = ctx.r[1].s64 + 126;
	// 82D5E184: 4BFFFABD  bl 0x82d5dc40
	ctx.lr = 0x82D5E188;
	sub_82D5DC40(ctx, base);
	// 82D5E188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E198 size=48
    let mut pc: u32 = 0x82D5E198;
    'dispatch: loop {
        match pc {
            0x82D5E198 => {
    //   block [0x82D5E198..0x82D5E1C8)
	// 82D5E198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E1A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E1A4: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82D5E1A8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E1AC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D5E1B0: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82D5E1B4: 4BFFFA8D  bl 0x82d5dc40
	ctx.lr = 0x82D5E1B8;
	sub_82D5DC40(ctx, base);
	// 82D5E1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E1C8 size=48
    let mut pc: u32 = 0x82D5E1C8;
    'dispatch: loop {
        match pc {
            0x82D5E1C8 => {
    //   block [0x82D5E1C8..0x82D5E1F8)
	// 82D5E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E1D4: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82D5E1D8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E1DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D5E1E0: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82D5E1E4: 4BFFFA5D  bl 0x82d5dc40
	ctx.lr = 0x82D5E1E8;
	sub_82D5DC40(ctx, base);
	// 82D5E1E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E1F8 size=48
    let mut pc: u32 = 0x82D5E1F8;
    'dispatch: loop {
        match pc {
            0x82D5E1F8 => {
    //   block [0x82D5E1F8..0x82D5E228)
	// 82D5E1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E204: F8810078  std r4, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u64 ) };
	// 82D5E208: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E20C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5E210: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 82D5E214: 4BFFFA2D  bl 0x82d5dc40
	ctx.lr = 0x82D5E218;
	sub_82D5DC40(ctx, base);
	// 82D5E218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E228 size=48
    let mut pc: u32 = 0x82D5E228;
    'dispatch: loop {
        match pc {
            0x82D5E228 => {
    //   block [0x82D5E228..0x82D5E258)
	// 82D5E228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E234: F8810078  std r4, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u64 ) };
	// 82D5E238: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E23C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5E240: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 82D5E244: 4BFFF9FD  bl 0x82d5dc40
	ctx.lr = 0x82D5E248;
	sub_82D5DC40(ctx, base);
	// 82D5E248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D5E258 size=48
    let mut pc: u32 = 0x82D5E258;
    'dispatch: loop {
        match pc {
            0x82D5E258 => {
    //   block [0x82D5E258..0x82D5E288)
	// 82D5E258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E264: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E268: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82D5E26C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D5E270: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82D5E274: 4BFFF9CD  bl 0x82d5dc40
	ctx.lr = 0x82D5E278;
	sub_82D5DC40(ctx, base);
	// 82D5E278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E288 size=48
    let mut pc: u32 = 0x82D5E288;
    'dispatch: loop {
        match pc {
            0x82D5E288 => {
    //   block [0x82D5E288..0x82D5E2B8)
	// 82D5E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E294: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D5E298: D8210078  stfd f1, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.f[1].u64 ) };
	// 82D5E29C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5E2A0: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 82D5E2A4: 4BFFF99D  bl 0x82d5dc40
	ctx.lr = 0x82D5E2A8;
	sub_82D5DC40(ctx, base);
	// 82D5E2A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5E2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D5E2B8 size=172
    let mut pc: u32 = 0x82D5E2B8;
    'dispatch: loop {
        match pc {
            0x82D5E2B8 => {
    //   block [0x82D5E2B8..0x82D5E364)
	// 82D5E2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E2BC: 4BF4B149  bl 0x82ca9404
	ctx.lr = 0x82D5E2C0;
	sub_82CA93D0(ctx, base);
	// 82D5E2C0: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E2C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D5E2C8: 897C000C  lbz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5E2CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5E2D0: 409A0020  bne cr6, 0x82d5e2f0
	if !ctx.cr[6].eq {
	pc = 0x82D5E2F0; continue 'dispatch;
	}
	// 82D5E2D4: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E2D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E2DC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5E2E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5E2E4: 4E800421  bctrl
	ctx.lr = 0x82D5E2E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5E2E8: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82D5E2EC: 4BF4B168  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5E2F0: 7CAB4E70  srawi r11, r5, 9
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 9) as i64;
	// 82D5E2F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D5E2F8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82D5E2FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5E300: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E304: 3BE00200  li r31, 0x200
	ctx.r[31].s64 = 512;
	// 82D5E308: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D5E30C: 7F6B2850  subf r27, r11, r5
	ctx.r[27].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82D5E310: 4099004C  ble cr6, 0x82d5e35c
	if !ctx.cr[6].gt {
	pc = 0x82D5E35C; continue 'dispatch;
	}
	// 82D5E314: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82D5E318: 40980008  bge cr6, 0x82d5e320
	if !ctx.cr[6].lt {
	pc = 0x82D5E320; continue 'dispatch;
	}
	// 82D5E31C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82D5E320: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5E324: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5E328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5E32C: 4BFFAA05  bl 0x82d58d30
	ctx.lr = 0x82D5E330;
	sub_82D58D30(ctx, base);
	// 82D5E330: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E334: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5E338: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5E33C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E340: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5E344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5E348: 4E800421  bctrl
	ctx.lr = 0x82D5E34C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5E34C: 7FDFF050  subf r30, r31, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82D5E350: 7FBFEA14  add r29, r31, r29
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82D5E354: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D5E358: 4199FFBC  bgt cr6, 0x82d5e314
	if ctx.cr[6].gt {
	pc = 0x82D5E314; continue 'dispatch;
	}
	// 82D5E35C: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82D5E360: 4BF4B0F4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D5E368 size=172
    let mut pc: u32 = 0x82D5E368;
    'dispatch: loop {
        match pc {
            0x82D5E368 => {
    //   block [0x82D5E368..0x82D5E414)
	// 82D5E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E36C: 4BF4B099  bl 0x82ca9404
	ctx.lr = 0x82D5E370;
	sub_82CA93D0(ctx, base);
	// 82D5E370: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E374: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D5E378: 897C000C  lbz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5E37C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5E380: 409A0020  bne cr6, 0x82d5e3a0
	if !ctx.cr[6].eq {
	pc = 0x82D5E3A0; continue 'dispatch;
	}
	// 82D5E384: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E388: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E38C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5E390: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5E394: 4E800421  bctrl
	ctx.lr = 0x82D5E398;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5E398: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82D5E39C: 4BF4B0B8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5E3A0: 7CAB4E70  srawi r11, r5, 9
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 9) as i64;
	// 82D5E3A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D5E3A8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82D5E3AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5E3B0: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E3B4: 3BE00200  li r31, 0x200
	ctx.r[31].s64 = 512;
	// 82D5E3B8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D5E3BC: 7F6B2850  subf r27, r11, r5
	ctx.r[27].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82D5E3C0: 4099004C  ble cr6, 0x82d5e40c
	if !ctx.cr[6].gt {
	pc = 0x82D5E40C; continue 'dispatch;
	}
	// 82D5E3C4: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82D5E3C8: 40980008  bge cr6, 0x82d5e3d0
	if !ctx.cr[6].lt {
	pc = 0x82D5E3D0; continue 'dispatch;
	}
	// 82D5E3CC: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82D5E3D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5E3D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5E3D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5E3DC: 4BFFA955  bl 0x82d58d30
	ctx.lr = 0x82D5E3E0;
	sub_82D58D30(ctx, base);
	// 82D5E3E0: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E3E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5E3E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5E3EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E3F0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5E3F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5E3F8: 4E800421  bctrl
	ctx.lr = 0x82D5E3FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5E3FC: 7FDFF050  subf r30, r31, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82D5E400: 7FBFEA14  add r29, r31, r29
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82D5E404: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D5E408: 4199FFBC  bgt cr6, 0x82d5e3c4
	if ctx.cr[6].gt {
	pc = 0x82D5E3C4; continue 'dispatch;
	}
	// 82D5E40C: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82D5E410: 4BF4B044  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E418 size=12
    let mut pc: u32 = 0x82D5E418;
    'dispatch: loop {
        match pc {
            0x82D5E418 => {
    //   block [0x82D5E418..0x82D5E424)
	// 82D5E418: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E41C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82D5E420: 4BFFF820  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E428 size=12
    let mut pc: u32 = 0x82D5E428;
    'dispatch: loop {
        match pc {
            0x82D5E428 => {
    //   block [0x82D5E428..0x82D5E434)
	// 82D5E428: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E42C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82D5E430: 4BFFF810  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E438 size=12
    let mut pc: u32 = 0x82D5E438;
    'dispatch: loop {
        match pc {
            0x82D5E438 => {
    //   block [0x82D5E438..0x82D5E444)
	// 82D5E438: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E43C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D5E440: 4BFFF800  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E448 size=12
    let mut pc: u32 = 0x82D5E448;
    'dispatch: loop {
        match pc {
            0x82D5E448 => {
    //   block [0x82D5E448..0x82D5E454)
	// 82D5E448: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E44C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D5E450: 4BFFF7F0  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E458 size=12
    let mut pc: u32 = 0x82D5E458;
    'dispatch: loop {
        match pc {
            0x82D5E458 => {
    //   block [0x82D5E458..0x82D5E464)
	// 82D5E458: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E45C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5E460: 4BFFF7E0  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E468 size=12
    let mut pc: u32 = 0x82D5E468;
    'dispatch: loop {
        match pc {
            0x82D5E468 => {
    //   block [0x82D5E468..0x82D5E474)
	// 82D5E468: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E46C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5E470: 4BFFF7D0  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E478 size=12
    let mut pc: u32 = 0x82D5E478;
    'dispatch: loop {
        match pc {
            0x82D5E478 => {
    //   block [0x82D5E478..0x82D5E484)
	// 82D5E478: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E47C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D5E480: 4BFFF7C0  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E488 size=12
    let mut pc: u32 = 0x82D5E488;
    'dispatch: loop {
        match pc {
            0x82D5E488 => {
    //   block [0x82D5E488..0x82D5E494)
	// 82D5E488: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82D5E48C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82D5E490: 4BFFF7B0  b 0x82d5dc40
	sub_82D5DC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E498 size=100
    let mut pc: u32 = 0x82D5E498;
    'dispatch: loop {
        match pc {
            0x82D5E498 => {
    //   block [0x82D5E498..0x82D5E4FC)
	// 82D5E498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E4A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5E4A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5E4A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E4AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5E4B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5E4B4: 4BFFFC05  bl 0x82d5e0b8
	ctx.lr = 0x82D5E4B8;
	sub_82D5E0B8(ctx, base);
	// 82D5E4B8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D5E4BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5E4C0: 419A0020  beq cr6, 0x82d5e4e0
	if ctx.cr[6].eq {
	pc = 0x82D5E4E0; continue 'dispatch;
	}
	// 82D5E4C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E4C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5E4CC: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D5E4D0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E4D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5E4D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5E4DC: 4BFF6DED  bl 0x82d552c8
	ctx.lr = 0x82D5E4E0;
	sub_82D552C8(ctx, base);
	// 82D5E4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5E4E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5E4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E4F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5E4F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5E4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E500 size=60
    let mut pc: u32 = 0x82D5E500;
    'dispatch: loop {
        match pc {
            0x82D5E500 => {
    //   block [0x82D5E500..0x82D5E53C)
	// 82D5E500: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E504: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5E508: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D5E50C: 40990028  ble cr6, 0x82d5e534
	if !ctx.cr[6].gt {
	pc = 0x82D5E534; continue 'dispatch;
	}
	// 82D5E510: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E514: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82D5E518: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E51C: 7F072000  cmpw cr6, r7, r4
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82D5E520: 419A001C  beq cr6, 0x82d5e53c
	if ctx.cr[6].eq {
		sub_82D5E53C(ctx, base);
		return;
	}
	// 82D5E524: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5E528: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82D5E52C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D5E530: 4198FFE8  blt cr6, 0x82d5e518
	if ctx.cr[6].lt {
	pc = 0x82D5E518; continue 'dispatch;
	}
	// 82D5E534: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5E538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E53C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E53C size=24
    let mut pc: u32 = 0x82D5E53C;
    'dispatch: loop {
        match pc {
            0x82D5E53C => {
    //   block [0x82D5E53C..0x82D5E554)
	// 82D5E53C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E540: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5E544: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D5E548: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E54C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5E550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E558 size=132
    let mut pc: u32 = 0x82D5E558;
    'dispatch: loop {
        match pc {
            0x82D5E558 => {
    //   block [0x82D5E558..0x82D5E5DC)
	// 82D5E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E55C: 4BF4AEA9  bl 0x82ca9404
	ctx.lr = 0x82D5E560;
	sub_82CA93D0(ctx, base);
	// 82D5E560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5E568: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5E56C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D5E570: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D5E574: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E578: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5E57C: 40990038  ble cr6, 0x82d5e5b4
	if !ctx.cr[6].gt {
	pc = 0x82D5E5B4; continue 'dispatch;
	}
	// 82D5E580: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D5E584: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E588: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5E58C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D5E590: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E594: 4BFFA47D  bl 0x82d58a10
	ctx.lr = 0x82D5E598;
	sub_82D58A10(ctx, base);
	// 82D5E598: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5E59C: 419A0024  beq cr6, 0x82d5e5c0
	if ctx.cr[6].eq {
	pc = 0x82D5E5C0; continue 'dispatch;
	}
	// 82D5E5A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E5A4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D5E5A8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82D5E5AC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5E5B0: 4198FFD4  blt cr6, 0x82d5e584
	if ctx.cr[6].lt {
	pc = 0x82D5E584; continue 'dispatch;
	}
	// 82D5E5B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5E5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5E5BC: 4BF4AE98  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D5E5C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E5C4: 57AA1838  slwi r10, r29, 3
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E5C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5E5CC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5E5D0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5E5D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5E5D8: 4BF4AE7C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E5E0 size=180
    let mut pc: u32 = 0x82D5E5E0;
    'dispatch: loop {
        match pc {
            0x82D5E5E0 => {
    //   block [0x82D5E5E0..0x82D5E694)
	// 82D5E5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E5E4: 4BF4AE1D  bl 0x82ca9400
	ctx.lr = 0x82D5E5E8;
	sub_82CA93D0(ctx, base);
	// 82D5E5E8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E5EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5E5F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D5E5F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D5E5F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5E5FC: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82D5E600: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E604: 4BFFF90D  bl 0x82d5df10
	ctx.lr = 0x82D5E608;
	sub_82D5DF10(ctx, base);
	// 82D5E608: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E60C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5E610: 4BFFA619  bl 0x82d58c28
	ctx.lr = 0x82D5E614;
	sub_82D58C28(ctx, base);
	// 82D5E614: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5E618: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5E61C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5E620: 4BFFF7E9  bl 0x82d5de08
	ctx.lr = 0x82D5E624;
	sub_82D5DE08(ctx, base);
	// 82D5E624: 835B0008  lwz r26, 8(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E628: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82D5E62C: 4099004C  ble cr6, 0x82d5e678
	if !ctx.cr[6].gt {
	pc = 0x82D5E678; continue 'dispatch;
	}
	// 82D5E630: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D5E634: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82D5E638: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E63C: 7FBE5A14  add r29, r30, r11
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D5E640: 839D0004  lwz r28, 4(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E644: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5E648: 4BFFA5E1  bl 0x82d58c28
	ctx.lr = 0x82D5E64C;
	sub_82D58C28(ctx, base);
	// 82D5E64C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5E650: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5E654: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5E658: 4BFFF7B1  bl 0x82d5de08
	ctx.lr = 0x82D5E65C;
	sub_82D5DE08(ctx, base);
	// 82D5E65C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5E660: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E664: 4BFFFB35  bl 0x82d5e198
	ctx.lr = 0x82D5E668;
	sub_82D5E198(ctx, base);
	// 82D5E668: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82D5E66C: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82D5E670: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D5E674: 409AFFC4  bne cr6, 0x82d5e638
	if !ctx.cr[6].eq {
	pc = 0x82D5E638; continue 'dispatch;
	}
	// 82D5E678: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82D5E67C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5E680: 4BFFFB19  bl 0x82d5e198
	ctx.lr = 0x82D5E684;
	sub_82D5E198(ctx, base);
	// 82D5E684: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5E688: 4BFFFA31  bl 0x82d5e0b8
	ctx.lr = 0x82D5E68C;
	sub_82D5E0B8(ctx, base);
	// 82D5E68C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D5E690: 4BF4ADC0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E698 size=16
    let mut pc: u32 = 0x82D5E698;
    'dispatch: loop {
        match pc {
            0x82D5E698 => {
    //   block [0x82D5E698..0x82D5E6A8)
	// 82D5E698: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5E69C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5E6A0: 419A0008  beq cr6, 0x82d5e6a8
	if ctx.cr[6].eq {
		sub_82D5E6A8(ctx, base);
		return;
	}
	// 82D5E6A4: 4BFF702C  b 0x82d556d0
	sub_82D556D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E6A8 size=8
    let mut pc: u32 = 0x82D5E6A8;
    'dispatch: loop {
        match pc {
            0x82D5E6A8 => {
    //   block [0x82D5E6A8..0x82D5E6B0)
	// 82D5E6A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5E6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E6B0 size=196
    let mut pc: u32 = 0x82D5E6B0;
    'dispatch: loop {
        match pc {
            0x82D5E6B0 => {
    //   block [0x82D5E6B0..0x82D5E774)
	// 82D5E6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E6B4: 4BF4AD45  bl 0x82ca93f8
	ctx.lr = 0x82D5E6B8;
	sub_82CA93D0(ctx, base);
	// 82D5E6B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E6BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5E6C0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D5E6C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D5E6C8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D5E6CC: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82D5E6D0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5E6D4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E6D8: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 82D5E6DC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82D5E6E0: 4198007C  blt cr6, 0x82d5e75c
	if ctx.cr[6].lt {
	pc = 0x82D5E75C; continue 'dispatch;
	}
	// 82D5E6E4: 573B1838  slwi r27, r25, 3
	ctx.r[27].u32 = ctx.r[25].u32.wrapping_shl(3);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82D5E6E8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5E6EC: 419A0070  beq cr6, 0x82d5e75c
	if ctx.cr[6].eq {
	pc = 0x82D5E75C; continue 'dispatch;
	}
	// 82D5E6F0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E6F4: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D5E6F8: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E6FC: 7FCAE838  and r10, r30, r29
	ctx.r[10].u64 = ctx.r[30].u64 & ctx.r[29].u64;
	// 82D5E700: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D5E704: 409A0048  bne cr6, 0x82d5e74c
	if !ctx.cr[6].eq {
	pc = 0x82D5E74C; continue 'dispatch;
	}
	// 82D5E708: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E70C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E710: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5E714: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E718: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D5E71C: 409A0010  bne cr6, 0x82d5e72c
	if !ctx.cr[6].eq {
	pc = 0x82D5E72C; continue 'dispatch;
	}
	// 82D5E720: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D5E724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5E728: 4BFF8871  bl 0x82d56f98
	ctx.lr = 0x82D5E72C;
	sub_82D56F98(ctx, base);
	// 82D5E72C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E730: 7FBDF078  andc r29, r29, r30
	ctx.r[29].u64 = ctx.r[29].u64 & !ctx.r[30].u64;
	// 82D5E734: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E738: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E73C: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82D5E740: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5E744: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5E748: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5E74C: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 82D5E750: 3B7BFFF8  addi r27, r27, -8
	ctx.r[27].s64 = ctx.r[27].s64 + -8;
	// 82D5E754: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82D5E758: 4098FF90  bge cr6, 0x82d5e6e8
	if !ctx.cr[6].lt {
	pc = 0x82D5E6E8; continue 'dispatch;
	}
	// 82D5E75C: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82D5E760: 93B80000  stw r29, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82D5E764: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D5E768: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82D5E76C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5E770: 4BF4ACD8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E778 size=72
    let mut pc: u32 = 0x82D5E778;
    'dispatch: loop {
        match pc {
            0x82D5E778 => {
    //   block [0x82D5E778..0x82D5E7C0)
	// 82D5E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5E780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5E784: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5E78C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D5E790: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5E794: 48000155  bl 0x82d5e8e8
	ctx.lr = 0x82D5E798;
	sub_82D5E8E8(ctx, base);
	// 82D5E798: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5E79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5E7A0: 4BFFFE41  bl 0x82d5e5e0
	ctx.lr = 0x82D5E7A4;
	sub_82D5E5E0(ctx, base);
	// 82D5E7A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5E7A8: 48000131  bl 0x82d5e8d8
	ctx.lr = 0x82D5E7AC;
	sub_82D5E8D8(ctx, base);
	// 82D5E7AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5E7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5E7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5E7B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E7C0 size=172
    let mut pc: u32 = 0x82D5E7C0;
    'dispatch: loop {
        match pc {
            0x82D5E7C0 => {
    //   block [0x82D5E7C0..0x82D5E86C)
	// 82D5E7C0: 546A07FE  clrlwi r10, r3, 0x1f
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 82D5E7C4: 3D60EDB8  lis r11, -0x1248
	ctx.r[11].s64 = -306708480;
	// 82D5E7C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5E7CC: 616B8320  ori r11, r11, 0x8320
	ctx.r[11].u64 = ctx.r[11].u64 | 33568;
	// 82D5E7D0: 546AF87E  srwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E7D4: 419A0008  beq cr6, 0x82d5e7dc
	if ctx.cr[6].eq {
	pc = 0x82D5E7DC; continue 'dispatch;
	}
	// 82D5E7D8: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E7DC: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D5E7E0: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E7E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D5E7E8: 419A0008  beq cr6, 0x82d5e7f0
	if ctx.cr[6].eq {
	pc = 0x82D5E7F0; continue 'dispatch;
	}
	// 82D5E7EC: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E7F0: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D5E7F4: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E7F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D5E7FC: 419A0008  beq cr6, 0x82d5e804
	if ctx.cr[6].eq {
	pc = 0x82D5E804; continue 'dispatch;
	}
	// 82D5E800: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E804: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D5E808: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E80C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D5E810: 419A0008  beq cr6, 0x82d5e818
	if ctx.cr[6].eq {
	pc = 0x82D5E818; continue 'dispatch;
	}
	// 82D5E814: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E818: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D5E81C: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E820: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D5E824: 419A0008  beq cr6, 0x82d5e82c
	if ctx.cr[6].eq {
	pc = 0x82D5E82C; continue 'dispatch;
	}
	// 82D5E828: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E82C: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D5E830: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E834: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D5E838: 419A0008  beq cr6, 0x82d5e840
	if ctx.cr[6].eq {
	pc = 0x82D5E840; continue 'dispatch;
	}
	// 82D5E83C: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E840: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D5E844: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E848: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D5E84C: 419A0008  beq cr6, 0x82d5e854
	if ctx.cr[6].eq {
	pc = 0x82D5E854; continue 'dispatch;
	}
	// 82D5E850: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E854: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D5E858: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D5E85C: 419A0010  beq cr6, 0x82d5e86c
	if ctx.cr[6].eq {
		sub_82D5E86C(ctx, base);
		return;
	}
	// 82D5E860: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5E864: 7D435A78  xor r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D5E868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E86C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E86C size=8
    let mut pc: u32 = 0x82D5E86C;
    'dispatch: loop {
        match pc {
            0x82D5E86C => {
    //   block [0x82D5E86C..0x82D5E874)
	// 82D5E86C: 5543F87E  srwi r3, r10, 1
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D5E870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5E878 size=92
    let mut pc: u32 = 0x82D5E878;
    'dispatch: loop {
        match pc {
            0x82D5E878 => {
    //   block [0x82D5E878..0x82D5E8D4)
	// 82D5E878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5E87C: 4BF4AB89  bl 0x82ca9404
	ctx.lr = 0x82D5E880;
	sub_82CA93D0(ctx, base);
	// 82D5E880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5E884: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D5E888: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D5E88C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5E890: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D5E894: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5E898: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E89C: 40990028  ble cr6, 0x82d5e8c4
	if !ctx.cr[6].gt {
	pc = 0x82D5E8C4; continue 'dispatch;
	}
	// 82D5E8A0: 7D7FE0AE  lbzx r11, r31, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5E8A4: 7D6BF278  xor r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[30].u64;
	// 82D5E8A8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82D5E8AC: 4BFFFF15  bl 0x82d5e7c0
	ctx.lr = 0x82D5E8B0;
	sub_82D5E7C0(ctx, base);
	// 82D5E8B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82D5E8B4: 57CBC23E  srwi r11, r30, 8
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E8B8: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D5E8BC: 7C7E5A78  xor r30, r3, r11
	ctx.r[30].u64 = ctx.r[3].u64 ^ ctx.r[11].u64;
	// 82D5E8C0: 4198FFE0  blt cr6, 0x82d5e8a0
	if ctx.cr[6].lt {
	pc = 0x82D5E8A0; continue 'dispatch;
	}
	// 82D5E8C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D5E8C8: 93DB0008  stw r30, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82D5E8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5E8D0: 4BF4AB84  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E8D8 size=12
    let mut pc: u32 = 0x82D5E8D8;
    'dispatch: loop {
        match pc {
            0x82D5E8D8 => {
    //   block [0x82D5E8D8..0x82D5E8E4)
	// 82D5E8D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E8DC: 7D6358F8  nor r3, r11, r11
	ctx.r[3].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82D5E8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E8E8 size=32
    let mut pc: u32 = 0x82D5E8E8;
    'dispatch: loop {
        match pc {
            0x82D5E8E8 => {
    //   block [0x82D5E8E8..0x82D5E908)
	// 82D5E8E8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5E8EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5E8F0: 396B4F2C  addi r11, r11, 0x4f2c
	ctx.r[11].s64 = ctx.r[11].s64 + 20268;
	// 82D5E8F4: 7C8920F8  nor r9, r4, r4
	ctx.r[9].u64 = !(ctx.r[4].u64 | ctx.r[4].u64);
	// 82D5E8F8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D5E8FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5E900: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D5E904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E908 size=12
    let mut pc: u32 = 0x82D5E908;
    'dispatch: loop {
        match pc {
            0x82D5E908 => {
    //   block [0x82D5E908..0x82D5E914)
	// 82D5E908: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5E90C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5E910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E918 size=68
    let mut pc: u32 = 0x82D5E918;
    'dispatch: loop {
        match pc {
            0x82D5E918 => {
    //   block [0x82D5E918..0x82D5E95C)
	// 82D5E918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5E91C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5E920: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E924: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 82D5E928: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D5E92C: 419A0028  beq cr6, 0x82d5e954
	if ctx.cr[6].eq {
	pc = 0x82D5E954; continue 'dispatch;
	}
	// 82D5E930: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5E934: 55472834  slwi r7, r10, 5
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D5E938: 7D4A3850  subf r10, r10, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82D5E93C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E940: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D5E944: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82D5E948: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82D5E94C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D5E950: 409AFFE0  bne cr6, 0x82d5e930
	if !ctx.cr[6].eq {
	pc = 0x82D5E930; continue 'dispatch;
	}
	// 82D5E954: 5543007E  clrlwi r3, r10, 1
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82D5E958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E960 size=20
    let mut pc: u32 = 0x82D5E960;
    'dispatch: loop {
        match pc {
            0x82D5E960 => {
    //   block [0x82D5E960..0x82D5E974)
	// 82D5E960: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5E964: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5E968: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E96C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5E970: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E974(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E974 size=16
    let mut pc: u32 = 0x82D5E974;
    'dispatch: loop {
        match pc {
            0x82D5E974 => {
    //   block [0x82D5E974..0x82D5E984)
	// 82D5E974: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E978: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E97C: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D5E980: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E984 size=20
    let mut pc: u32 = 0x82D5E984;
    'dispatch: loop {
        match pc {
            0x82D5E984 => {
    //   block [0x82D5E984..0x82D5E998)
	// 82D5E984: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82D5E988: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D5E98C: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D5E990: 4099FFE8  ble cr6, 0x82d5e978
	if !ctx.cr[6].gt {
		sub_82D5E974(ctx, base);
		return;
	}
	// 82D5E994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E998 size=28
    let mut pc: u32 = 0x82D5E998;
    'dispatch: loop {
        match pc {
            0x82D5E998 => {
    //   block [0x82D5E998..0x82D5E9B4)
	// 82D5E998: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E99C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E9A0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D5E9A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5E9A8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E9AC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5E9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E9B8 size=32
    let mut pc: u32 = 0x82D5E9B8;
    'dispatch: loop {
        match pc {
            0x82D5E9B8 => {
    //   block [0x82D5E9B8..0x82D5E9D8)
	// 82D5E9B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E9BC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E9C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5E9C4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E9C8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D5E9CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E9D0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5E9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E9D8 size=32
    let mut pc: u32 = 0x82D5E9D8;
    'dispatch: loop {
        match pc {
            0x82D5E9D8 => {
    //   block [0x82D5E9D8..0x82D5E9F8)
	// 82D5E9D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5E9DC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5E9E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5E9E4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E9E8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D5E9EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5E9F0: 7CAB512E  stwx r5, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 82D5E9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5E9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5E9F8 size=20
    let mut pc: u32 = 0x82D5E9F8;
    'dispatch: loop {
        match pc {
            0x82D5E9F8 => {
    //   block [0x82D5E9F8..0x82D5EA0C)
	// 82D5E9F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5E9FC: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 82D5EA00: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EA04: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D5EA08: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EA0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5EA0C size=24
    let mut pc: u32 = 0x82D5EA0C;
    'dispatch: loop {
        match pc {
            0x82D5EA0C => {
    //   block [0x82D5EA0C..0x82D5EA24)
	// 82D5EA0C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EA10: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EA14: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5EA18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EA1C: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82D5EA20: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EA24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5EA24 size=20
    let mut pc: u32 = 0x82D5EA24;
    'dispatch: loop {
        match pc {
            0x82D5EA24 => {
    //   block [0x82D5EA24..0x82D5EA38)
	// 82D5EA24: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82D5EA28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D5EA2C: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D5EA30: 4099FFE8  ble cr6, 0x82d5ea18
	if !ctx.cr[6].gt {
		sub_82D5EA0C(ctx, base);
		return;
	}
	// 82D5EA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5EA38 size=28
    let mut pc: u32 = 0x82D5EA38;
    'dispatch: loop {
        match pc {
            0x82D5EA38 => {
    //   block [0x82D5EA38..0x82D5EA54)
	// 82D5EA38: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EA3C: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5EA40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5EA44: 40990008  ble cr6, 0x82d5ea4c
	if !ctx.cr[6].gt {
	pc = 0x82D5EA4C; continue 'dispatch;
	}
	// 82D5EA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5EA4C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5EA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5EA58 size=316
    let mut pc: u32 = 0x82D5EA58;
    'dispatch: loop {
        match pc {
            0x82D5EA58 => {
    //   block [0x82D5EA58..0x82D5EB94)
	// 82D5EA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5EA5C: 4BF4A9A9  bl 0x82ca9404
	ctx.lr = 0x82D5EA60;
	sub_82CA93D0(ctx, base);
	// 82D5EA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5EA64: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5EA68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5EA6C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D5EA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5EA74: 895C0000  lbz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EA78: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 82D5EA7C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D5EA80: 419A002C  beq cr6, 0x82d5eaac
	if ctx.cr[6].eq {
	pc = 0x82D5EAAC; continue 'dispatch;
	}
	// 82D5EA84: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82D5EA88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D5EA8C: 55672834  slwi r7, r11, 5
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D5EA90: 7D6B3850  subf r11, r11, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 82D5EA94: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EA98: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D5EA9C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82D5EAA0: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82D5EAA4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D5EAA8: 409AFFE0  bne cr6, 0x82d5ea88
	if !ctx.cr[6].eq {
	pc = 0x82D5EA88; continue 'dispatch;
	}
	// 82D5EAAC: 557D007E  clrlwi r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82D5EAB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5EAB4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EAB8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EABC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D5EAC0: 40990014  ble cr6, 0x82d5ead4
	if !ctx.cr[6].gt {
	pc = 0x82D5EAD4; continue 'dispatch;
	}
	// 82D5EAC4: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82D5EAC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5EACC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D5EAD0: 48000689  bl 0x82d5f158
	ctx.lr = 0x82D5EAD4;
	sub_82D5F158(ctx, base);
	// 82D5EAD4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EAD8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EADC: 7D5EE838  and r30, r10, r29
	ctx.r[30].u64 = ctx.r[10].u64 & ctx.r[29].u64;
	// 82D5EAE0: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EAE4: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5EAE8: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D5EAEC: 419A0054  beq cr6, 0x82d5eb40
	if ctx.cr[6].eq {
	pc = 0x82D5EB40; continue 'dispatch;
	}
	// 82D5EAF0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EAF4: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5EAF8: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82D5EAFC: 409A0024  bne cr6, 0x82d5eb20
	if !ctx.cr[6].eq {
	pc = 0x82D5EB20; continue 'dispatch;
	}
	// 82D5EB00: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82D5EB04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5EB08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EB0C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EB10: 7C8B482E  lwzx r4, r11, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5EB14: 4BFF9EC5  bl 0x82d589d8
	ctx.lr = 0x82D5EB18;
	sub_82D589D8(ctx, base);
	// 82D5EB18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5EB1C: 419A0030  beq cr6, 0x82d5eb4c
	if ctx.cr[6].eq {
	pc = 0x82D5EB4C; continue 'dispatch;
	}
	// 82D5EB20: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EB24: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82D5EB28: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EB2C: 7D7E5038  and r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82D5EB30: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EB34: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5EB38: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D5EB3C: 409AFFB4  bne cr6, 0x82d5eaf0
	if !ctx.cr[6].eq {
	pc = 0x82D5EAF0; continue 'dispatch;
	}
	// 82D5EB40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5EB44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EB48: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5EB4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EB50: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5EB54: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82D5EB58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EB5C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EB60: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D5EB64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EB68: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EB6C: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82D5EB70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EB74: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EB78: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EB7C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EB80: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D5EB84: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EB88: 7F6B512E  stwx r27, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u32) };
	// 82D5EB8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5EB90: 4BF4A8C4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5EB98 size=212
    let mut pc: u32 = 0x82D5EB98;
    'dispatch: loop {
        match pc {
            0x82D5EB98 => {
    //   block [0x82D5EB98..0x82D5EC6C)
	// 82D5EB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5EB9C: 4BF4A86D  bl 0x82ca9408
	ctx.lr = 0x82D5EBA0;
	sub_82CA93D0(ctx, base);
	// 82D5EBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5EBA4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5EBA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5EBAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5EBB0: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EBB4: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 82D5EBB8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D5EBBC: 419A002C  beq cr6, 0x82d5ebe8
	if ctx.cr[6].eq {
	pc = 0x82D5EBE8; continue 'dispatch;
	}
	// 82D5EBC0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82D5EBC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EBC8: 55472834  slwi r7, r10, 5
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D5EBCC: 7D4A3850  subf r10, r10, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82D5EBD0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EBD4: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D5EBD8: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82D5EBDC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82D5EBE0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D5EBE4: 409AFFE0  bne cr6, 0x82d5ebc4
	if !ctx.cr[6].eq {
	pc = 0x82D5EBC4; continue 'dispatch;
	}
	// 82D5EBE8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EBEC: 555D007E  clrlwi r29, r10, 1
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82D5EBF0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EBF4: 7D7FE838  and r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 & ctx.r[29].u64;
	// 82D5EBF8: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5EBFC: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5EC00: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D5EC04: 419A004C  beq cr6, 0x82d5ec50
	if ctx.cr[6].eq {
	pc = 0x82D5EC50; continue 'dispatch;
	}
	// 82D5EC08: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82D5EC0C: 409A0024  bne cr6, 0x82d5ec30
	if !ctx.cr[6].eq {
	pc = 0x82D5EC30; continue 'dispatch;
	}
	// 82D5EC10: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82D5EC14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5EC18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EC1C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EC20: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5EC24: 4BFF9DB5  bl 0x82d589d8
	ctx.lr = 0x82D5EC28;
	sub_82D589D8(ctx, base);
	// 82D5EC28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5EC2C: 419A0034  beq cr6, 0x82d5ec60
	if ctx.cr[6].eq {
	pc = 0x82D5EC60; continue 'dispatch;
	}
	// 82D5EC30: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EC34: 393F0001  addi r9, r31, 1
	ctx.r[9].s64 = ctx.r[31].s64 + 1;
	// 82D5EC38: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EC3C: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82D5EC40: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5EC44: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5EC48: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D5EC4C: 409AFFBC  bne cr6, 0x82d5ec08
	if !ctx.cr[6].eq {
	pc = 0x82D5EC08; continue 'dispatch;
	}
	// 82D5EC50: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EC54: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82D5EC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5EC5C: 4BF4A7FC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D5EC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5EC64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5EC68: 4BF4A7F0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5EC70 size=136
    let mut pc: u32 = 0x82D5EC70;
    'dispatch: loop {
        match pc {
            0x82D5EC70 => {
    //   block [0x82D5EC70..0x82D5ECF8)
	// 82D5EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5EC78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5EC7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5EC80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5EC84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5EC88: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5EC8C: 4BFFFF0D  bl 0x82d5eb98
	ctx.lr = 0x82D5EC90;
	sub_82D5EB98(ctx, base);
	// 82D5EC90: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EC94: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5EC98: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D5EC9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5ECA0: 40990008  ble cr6, 0x82d5eca8
	if !ctx.cr[6].gt {
	pc = 0x82D5ECA8; continue 'dispatch;
	}
	// 82D5ECA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5ECA8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D5ECAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5ECB0: 419A002C  beq cr6, 0x82d5ecdc
	if ctx.cr[6].eq {
	pc = 0x82D5ECDC; continue 'dispatch;
	}
	// 82D5ECB4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5ECB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5ECBC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5ECC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D5ECC4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5ECC8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D5ECCC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5ECD0: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5ECD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5ECD8: 48000008  b 0x82d5ece0
	pc = 0x82D5ECE0; continue 'dispatch;
	// 82D5ECDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5ECE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5ECE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5ECE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5ECEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5ECF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5ECF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5ECF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5ECF8 size=48
    let mut pc: u32 = 0x82D5ECF8;
    'dispatch: loop {
        match pc {
            0x82D5ECF8 => {
    //   block [0x82D5ECF8..0x82D5ED28)
	// 82D5ECF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5ECFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5ED00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5ED04: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D5ED08: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D5ED0C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82D5ED10: 4BFFFF61  bl 0x82d5ec70
	ctx.lr = 0x82D5ED14;
	sub_82D5EC70(ctx, base);
	// 82D5ED14: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5ED18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5ED1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5ED20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5ED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5ED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5ED28 size=124
    let mut pc: u32 = 0x82D5ED28;
    'dispatch: loop {
        match pc {
            0x82D5ED28 => {
    //   block [0x82D5ED28..0x82D5EDA4)
	// 82D5ED28: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5ED2C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D5ED30: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5ED34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82D5ED38: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5ED3C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5ED40: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5ED44: 7CA9412E  stwx r5, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[5].u32) };
	// 82D5ED48: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5ED4C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5ED50: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82D5ED54: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82D5ED58: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D5ED5C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D5ED60: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 82D5ED64: 419A0020  beq cr6, 0x82d5ed84
	if ctx.cr[6].eq {
	pc = 0x82D5ED84; continue 'dispatch;
	}
	// 82D5ED68: 5507003E  slwi r7, r8, 0
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D5ED6C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D5ED70: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82D5ED74: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D5ED78: 7CC6382E  lwzx r6, r6, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D5ED7C: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 82D5ED80: 409AFFEC  bne cr6, 0x82d5ed6c
	if !ctx.cr[6].eq {
	pc = 0x82D5ED6C; continue 'dispatch;
	}
	// 82D5ED84: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 82D5ED88: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82D5ED8C: 7CEB4838  and r11, r7, r9
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82D5ED90: 7CC44838  and r4, r6, r9
	ctx.r[4].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 82D5ED94: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D5ED98: 7D08302E  lwzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82D5ED9C: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82D5EDA0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EDA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5EDA4 size=208
    let mut pc: u32 = 0x82D5EDA4;
    'dispatch: loop {
        match pc {
            0x82D5EDA4 => {
    //   block [0x82D5EDA4..0x82D5EE74)
	// 82D5EDA4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EDA8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D5EDAC: 7CE8302E  lwzx r7, r8, r6
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82D5EDB0: 7CE94838  and r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82D5EDB4: 4198000C  blt cr6, 0x82d5edc0
	if ctx.cr[6].lt {
	pc = 0x82D5EDC0; continue 'dispatch;
	}
	// 82D5EDB8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82D5EDBC: 41990094  bgt cr6, 0x82d5ee50
	if ctx.cr[6].gt {
	pc = 0x82D5EE50; continue 'dispatch;
	}
	// 82D5EDC0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82D5EDC4: 40980014  bge cr6, 0x82d5edd8
	if !ctx.cr[6].lt {
	pc = 0x82D5EDD8; continue 'dispatch;
	}
	// 82D5EDC8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82D5EDCC: 41990084  bgt cr6, 0x82d5ee50
	if ctx.cr[6].gt {
	pc = 0x82D5EE50; continue 'dispatch;
	}
	// 82D5EDD0: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D5EDD4: 4099007C  ble cr6, 0x82d5ee50
	if !ctx.cr[6].gt {
	pc = 0x82D5EE50; continue 'dispatch;
	}
	// 82D5EDD8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82D5EDDC: 4099000C  ble cr6, 0x82d5ede8
	if !ctx.cr[6].gt {
	pc = 0x82D5EDE8; continue 'dispatch;
	}
	// 82D5EDE0: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D5EDE4: 4198006C  blt cr6, 0x82d5ee50
	if ctx.cr[6].lt {
	pc = 0x82D5EE50; continue 'dispatch;
	}
	// 82D5EDE8: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5EDEC: 7CE9412E  stwx r7, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u32) };
	// 82D5EDF0: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EDF4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EDF8: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D5EDFC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82D5EE00: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82D5EE04: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D5EE08: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D5EE0C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5EE10: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D5EE14: 7CE9412E  stwx r7, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u32) };
	// 82D5EE18: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EE1C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EE20: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D5EE24: 7CE85A14  add r7, r8, r11
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D5EE28: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82D5EE2C: 39070002  addi r8, r7, 2
	ctx.r[8].s64 = ctx.r[7].s64 + 2;
	// 82D5EE30: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D5EE34: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D5EE38: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D5EE3C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D5EE40: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5EE44: 7D07492E  stwx r8, r7, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82D5EE48: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EE4C: 7CA6492E  stwx r5, r6, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32), ctx.r[5].u32) };
	// 82D5EE50: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EE54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EE58: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EE5C: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82D5EE60: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D5EE64: 7D06402E  lwzx r8, r6, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D5EE68: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82D5EE6C: 409AFF38  bne cr6, 0x82d5eda4
	if !ctx.cr[6].eq {
	pc = 0x82D5EDA4; continue 'dispatch;
	}
	// 82D5EE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5EE78 size=116
    let mut pc: u32 = 0x82D5EE78;
    'dispatch: loop {
        match pc {
            0x82D5EE78 => {
    //   block [0x82D5EE78..0x82D5EEEC)
	// 82D5EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5EE80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5EE84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5EE88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5EE8C: 4BFFFD0D  bl 0x82d5eb98
	ctx.lr = 0x82D5EE90;
	sub_82D5EB98(ctx, base);
	// 82D5EE90: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EE94: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5EE98: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5EE9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5EEA0: 40990008  ble cr6, 0x82d5eea8
	if !ctx.cr[6].gt {
	pc = 0x82D5EEA8; continue 'dispatch;
	}
	// 82D5EEA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5EEA8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D5EEAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5EEB0: 419A0024  beq cr6, 0x82d5eed4
	if ctx.cr[6].eq {
	pc = 0x82D5EED4; continue 'dispatch;
	}
	// 82D5EEB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5EEB8: 4BFFFE71  bl 0x82d5ed28
	ctx.lr = 0x82D5EEBC;
	sub_82D5ED28(ctx, base);
	// 82D5EEBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5EEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5EEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5EEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5EECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5EED0: 4E800020  blr
	return;
	// 82D5EED4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5EED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5EEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5EEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5EEE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5EEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5EEF0 size=184
    let mut pc: u32 = 0x82D5EEF0;
    'dispatch: loop {
        match pc {
            0x82D5EEF0 => {
    //   block [0x82D5EEF0..0x82D5EFA8)
	// 82D5EEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5EEF4: 4BF4A509  bl 0x82ca93fc
	ctx.lr = 0x82D5EEF8;
	sub_82CA93D0(ctx, base);
	// 82D5EEF8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5EEFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5EF00: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D5EF04: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D5EF08: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EF0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5EF10: 41980084  blt cr6, 0x82d5ef94
	if ctx.cr[6].lt {
	pc = 0x82D5EF94; continue 'dispatch;
	}
	// 82D5EF14: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82D5EF18: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EF1C: 7FBA502E  lwzx r29, r26, r10
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5EF20: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82D5EF24: 419A005C  beq cr6, 0x82d5ef80
	if ctx.cr[6].eq {
	pc = 0x82D5EF80; continue 'dispatch;
	}
	// 82D5EF28: 7D2BDA14  add r9, r11, r27
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D5EF2C: 7D7FE838  and r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 & ctx.r[29].u64;
	// 82D5EF30: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D5EF34: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5EF38: 7F89502E  lwzx r28, r9, r10
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5EF3C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EF40: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D5EF44: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5EF48: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82D5EF4C: 409A0024  bne cr6, 0x82d5ef70
	if !ctx.cr[6].eq {
	pc = 0x82D5EF70; continue 'dispatch;
	}
	// 82D5EF50: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82D5EF54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5EF58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5EF5C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5EF60: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5EF64: 4BFF9A75  bl 0x82d589d8
	ctx.lr = 0x82D5EF68;
	sub_82D589D8(ctx, base);
	// 82D5EF68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5EF6C: 419A0014  beq cr6, 0x82d5ef80
	if ctx.cr[6].eq {
	pc = 0x82D5EF80; continue 'dispatch;
	}
	// 82D5EF70: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EF74: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 82D5EF78: 7D5F5838  and r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82D5EF7C: 4BFFFFC0  b 0x82d5ef3c
	pc = 0x82D5EF3C; continue 'dispatch;
	// 82D5EF80: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EF84: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82D5EF88: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82D5EF8C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5EF90: 4099FF88  ble cr6, 0x82d5ef18
	if !ctx.cr[6].gt {
	pc = 0x82D5EF18; continue 'dispatch;
	}
	// 82D5EF94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5EF98: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82D5EF9C: 99790000  stb r11, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5EFA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5EFA4: 4BF4A4A8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5EFA8 size=68
    let mut pc: u32 = 0x82D5EFA8;
    'dispatch: loop {
        match pc {
            0x82D5EFA8 => {
    //   block [0x82D5EFA8..0x82D5EFEC)
	// 82D5EFA8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D5EFB0: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82D5EFB4: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5EFB8: 4081002C  ble 0x82d5efe4
	if !ctx.cr[0].gt {
	pc = 0x82D5EFE4; continue 'dispatch;
	}
	// 82D5EFBC: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82D5EFC0: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82D5EFC4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EFC8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D5EFCC: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82D5EFD0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D5EFD4: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5EFD8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D5EFDC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D5EFE0: 4198FFE4  blt cr6, 0x82d5efc4
	if ctx.cr[6].lt {
	pc = 0x82D5EFC4; continue 'dispatch;
	}
	// 82D5EFE4: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82D5EFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5EFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5EFF0 size=52
    let mut pc: u32 = 0x82D5EFF0;
    'dispatch: loop {
        match pc {
            0x82D5EFF0 => {
    //   block [0x82D5EFF0..0x82D5F024)
	// 82D5EFF0: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EFF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5EFF8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5EFFC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5F000: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D5F004: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5F008: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D5F00C: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5F010: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82D5F014: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5F018: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D5F01C: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D5F020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5F028 size=100
    let mut pc: u32 = 0x82D5F028;
    'dispatch: loop {
        match pc {
            0x82D5F028 => {
    //   block [0x82D5F028..0x82D5F08C)
	// 82D5F028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5F02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5F030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5F034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5F038: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F03C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5F040: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D5F044: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82D5F048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5F04C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5F050: 4BFF61F9  bl 0x82d55248
	ctx.lr = 0x82D5F054;
	sub_82D55248(ctx, base);
	// 82D5F054: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82D5F058: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D5F05C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82D5F060: 4BFF9CE1  bl 0x82d58d40
	ctx.lr = 0x82D5F064;
	sub_82D58D40(ctx, base);
	// 82D5F064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5F068: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 82D5F06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5F070: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5F074: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D5F078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5F07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5F080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5F084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5F088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F090 size=44
    let mut pc: u32 = 0x82D5F090;
    'dispatch: loop {
        match pc {
            0x82D5F090 => {
    //   block [0x82D5F090..0x82D5F0BC)
	// 82D5F090: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F094: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5F098: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5F09C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5F0A0: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F0A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5F0A8: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D5F0AC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5F0B0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5F0B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D5F0B8: 4BFF6210  b 0x82d552c8
	sub_82D552C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5F0C0 size=152
    let mut pc: u32 = 0x82D5F0C0;
    'dispatch: loop {
        match pc {
            0x82D5F0C0 => {
    //   block [0x82D5F0C0..0x82D5F158)
	// 82D5F0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5F0C4: 4BF4A345  bl 0x82ca9408
	ctx.lr = 0x82D5F0C8;
	sub_82CA93D0(ctx, base);
	// 82D5F0C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5F0CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5F0D0: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F0D4: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D5F0D8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5F0DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5F0E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5F0E4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F0E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5F0EC: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5F0F0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5F0F4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5F0F8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D5F0FC: 4BFF61CD  bl 0x82d552c8
	ctx.lr = 0x82D5F100;
	sub_82D552C8(ctx, base);
	// 82D5F100: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5F104: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D5F108: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D5F10C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5F110: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5F114: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5F118: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D5F11C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5F120: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5F124: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5F128: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D5F12C: 4BFF611D  bl 0x82d55248
	ctx.lr = 0x82D5F130;
	sub_82D55248(ctx, base);
	// 82D5F130: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5F134: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D5F138: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5F13C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F140: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D5F144: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5F148: 4BFF9BE9  bl 0x82d58d30
	ctx.lr = 0x82D5F14C;
	sub_82D58D30(ctx, base);
	// 82D5F14C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5F150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5F154: 4BF4A304  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5F158 size=216
    let mut pc: u32 = 0x82D5F158;
    'dispatch: loop {
        match pc {
            0x82D5F158 => {
    //   block [0x82D5F158..0x82D5F230)
	// 82D5F158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5F15C: 4BF4A29D  bl 0x82ca93f8
	ctx.lr = 0x82D5F160;
	sub_82CA93D0(ctx, base);
	// 82D5F160: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5F164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5F168: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F16C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5F170: 3B200004  li r25, 4
	ctx.r[25].s64 = 4;
	// 82D5F174: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D5F178: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5F17C: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F180: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82D5F184: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D5F188: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5F18C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D5F190: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D5F194: 4BFF60B5  bl 0x82d55248
	ctx.lr = 0x82D5F198;
	sub_82D55248(ctx, base);
	// 82D5F198: 57C5103A  slwi r5, r30, 2
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D5F19C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82D5F1A0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D5F1A4: 4BFF9B9D  bl 0x82d58d40
	ctx.lr = 0x82D5F1A8;
	sub_82D58D40(ctx, base);
	// 82D5F1A8: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82D5F1AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5F1B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D5F1B4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5F1B8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D5F1BC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D5F1C0: 4099004C  ble cr6, 0x82d5f20c
	if !ctx.cr[6].gt {
	pc = 0x82D5F20C; continue 'dispatch;
	}
	// 82D5F1C4: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5F1C8: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 82D5F1CC: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D5F1D0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F1D4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82D5F1D8: 419A0020  beq cr6, 0x82d5f1f8
	if ctx.cr[6].eq {
	pc = 0x82D5F1F8; continue 'dispatch;
	}
	// 82D5F1DC: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5F1E0: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5F1E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5F1E8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D5F1EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5F1F0: 7CABD82E  lwzx r5, r11, r27
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82D5F1F4: 4BFFF865  bl 0x82d5ea58
	ctx.lr = 0x82D5F1F8;
	sub_82D5EA58(ctx, base);
	// 82D5F1F8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82D5F1FC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82D5F200: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82D5F204: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D5F208: 4198FFC8  blt cr6, 0x82d5f1d0
	if ctx.cr[6].lt {
	pc = 0x82D5F1D0; continue 'dispatch;
	}
	// 82D5F20C: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5F210: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D5F214: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5F218: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82D5F21C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D5F220: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D5F224: 4BFF60A5  bl 0x82d552c8
	ctx.lr = 0x82D5F228;
	sub_82D552C8(ctx, base);
	// 82D5F228: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D5F22C: 4BF4A21C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5F230 size=64
    let mut pc: u32 = 0x82D5F230;
    'dispatch: loop {
        match pc {
            0x82D5F230 => {
    //   block [0x82D5F230..0x82D5F270)
	// 82D5F230: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D5F234: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5F238: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82D5F23C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F240: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5F244: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D5F248: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5F24C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D5F250: 419A0020  beq cr6, 0x82d5f270
	if ctx.cr[6].eq {
		sub_82D5F270(ctx, base);
		return;
	}
	// 82D5F254: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D5F258: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D5F25C: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 82D5F260: 4198FFDC  blt cr6, 0x82d5f23c
	if ctx.cr[6].lt {
	pc = 0x82D5F23C; continue 'dispatch;
	}
	// 82D5F264: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5F268: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5F26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F270 size=12
    let mut pc: u32 = 0x82D5F270;
    'dispatch: loop {
        match pc {
            0x82D5F270 => {
    //   block [0x82D5F270..0x82D5F27C)
	// 82D5F270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5F274: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5F278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F280 size=12
    let mut pc: u32 = 0x82D5F280;
    'dispatch: loop {
        match pc {
            0x82D5F280 => {
    //   block [0x82D5F280..0x82D5F28C)
	// 82D5F280: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D5F284: 386B6E60  addi r3, r11, 0x6e60
	ctx.r[3].s64 = ctx.r[11].s64 + 28256;
	// 82D5F288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5F290 size=52
    let mut pc: u32 = 0x82D5F290;
    'dispatch: loop {
        match pc {
            0x82D5F290 => {
    //   block [0x82D5F290..0x82D5F2C4)
	// 82D5F290: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F294: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F298: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5F29C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5F2A0: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F2A4: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F2A8: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5F2AC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5F2B0: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F2B4: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F2B8: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5F2BC: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5F2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5F2C8 size=96
    let mut pc: u32 = 0x82D5F2C8;
    'dispatch: loop {
        match pc {
            0x82D5F2C8 => {
    //   block [0x82D5F2C8..0x82D5F328)
	// 82D5F2C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5F2CC: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F2D0: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5F2D4: C1A40014  lfs f13, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F2D8: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82D5F2DC: C1A40028  lfs f13, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F2E0: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F2E4: D1A30028  stfs f13, 0x28(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82D5F2E8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D5F2EC: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D5F2F0: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82D5F2F4: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F2F8: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D5F2FC: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F300: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D5F304: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F308: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D5F30C: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F310: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82D5F314: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F318: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D5F31C: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F320: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D5F324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F328 size=60
    let mut pc: u32 = 0x82D5F328;
    'dispatch: loop {
        match pc {
            0x82D5F328 => {
    //   block [0x82D5F328..0x82D5F364)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F368 size=80
    let mut pc: u32 = 0x82D5F368;
    'dispatch: loop {
        match pc {
            0x82D5F368 => {
    //   block [0x82D5F368..0x82D5F3B8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F3B8 size=236
    let mut pc: u32 = 0x82D5F3B8;
    'dispatch: loop {
        match pc {
            0x82D5F3B8 => {
    //   block [0x82D5F3B8..0x82D5F4A4)
	// 82D5F3B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5F4A8 size=140
    let mut pc: u32 = 0x82D5F4A8;
    'dispatch: loop {
        match pc {
            0x82D5F4A8 => {
    //   block [0x82D5F4A8..0x82D5F534)
	// 82D5F4A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5F4AC: C1A40004  lfs f13, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F4B0: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D5F4B4: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82D5F4B8: D1A1FFD8  stfs f13, -0x28(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82D5F4BC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82D5F4C0: D181FFD4  stfs f12, -0x2c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82D5F4C4: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F4C8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82D5F4CC: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82D5F4D0: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82D5F4D4: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 82D5F4D8: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82D5F4DC: D001FFEC  stfs f0, -0x14(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82D5F4E0: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D5F4E4: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F538 size=284
    let mut pc: u32 = 0x82D5F538;
    'dispatch: loop {
        match pc {
            0x82D5F538 => {
    //   block [0x82D5F538..0x82D5F654)
	// 82D5F538: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5F53C: EC010072  fmuls f0, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82D5F540: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 82D5F544: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 82D5F548: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F654 size=8
    let mut pc: u32 = 0x82D5F654;
    'dispatch: loop {
        match pc {
            0x82D5F654 => {
    //   block [0x82D5F654..0x82D5F65C)
	// 82D5F654: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5F658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F660 size=200
    let mut pc: u32 = 0x82D5F660;
    'dispatch: loop {
        match pc {
            0x82D5F660 => {
    //   block [0x82D5F660..0x82D5F728)
	// 82D5F660: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F728 size=68
    let mut pc: u32 = 0x82D5F728;
    'dispatch: loop {
        match pc {
            0x82D5F728 => {
    //   block [0x82D5F728..0x82D5F76C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F770 size=68
    let mut pc: u32 = 0x82D5F770;
    'dispatch: loop {
        match pc {
            0x82D5F770 => {
    //   block [0x82D5F770..0x82D5F7B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F7B8 size=60
    let mut pc: u32 = 0x82D5F7B8;
    'dispatch: loop {
        match pc {
            0x82D5F7B8 => {
    //   block [0x82D5F7B8..0x82D5F7F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F7F8 size=84
    let mut pc: u32 = 0x82D5F7F8;
    'dispatch: loop {
        match pc {
            0x82D5F7F8 => {
    //   block [0x82D5F7F8..0x82D5F84C)
	// 82D5F7F8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F850 size=4
    let mut pc: u32 = 0x82D5F850;
    'dispatch: loop {
        match pc {
            0x82D5F850 => {
    //   block [0x82D5F850..0x82D5F854)
	// 82D5F850: 4BFFFE10  b 0x82d5f660
	sub_82D5F660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5F858 size=84
    let mut pc: u32 = 0x82D5F858;
    'dispatch: loop {
        match pc {
            0x82D5F858 => {
    //   block [0x82D5F858..0x82D5F8AC)
	// 82D5F858: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5F8B0 size=180
    let mut pc: u32 = 0x82D5F8B0;
    'dispatch: loop {
        match pc {
            0x82D5F8B0 => {
    //   block [0x82D5F8B0..0x82D5F964)
	// 82D5F8B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5F8B4: C0050000  lfs f0, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F8B8: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82D5F8BC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82D5F8C0: C0050014  lfs f0, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F8C4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82D5F8C8: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82D5F8CC: C1A50028  lfs f13, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5F8D0: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F8D4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82D5F8D8: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82D5F8DC: D001FFEC  stfs f0, -0x14(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82D5F8E0: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D5F8E4: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F8E8: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82D5F8EC: C0050020  lfs f0, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F8F0: D001FFD8  stfs f0, -0x28(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82D5F8F4: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F8F8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5F8FC: C0050024  lfs f0, 0x24(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5F900: D1A1FFF8  stfs f13, -8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D5F904: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82D5F908: C1A50010  lfs f13, 0x10(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5F968 size=92
    let mut pc: u32 = 0x82D5F968;
    'dispatch: loop {
        match pc {
            0x82D5F968 => {
    //   block [0x82D5F968..0x82D5F9C4)
	// 82D5F968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5F96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5F970: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5F974: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82D5F978: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D5F97C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82D5F980: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5F984: 4BFFFED5  bl 0x82d5f858
	ctx.lr = 0x82D5F988;
	sub_82D5F858(ctx, base);
	// 82D5F988: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82D5F98C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82D5F990: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5F9C8 size=68
    let mut pc: u32 = 0x82D5F9C8;
    'dispatch: loop {
        match pc {
            0x82D5F9C8 => {
    //   block [0x82D5F9C8..0x82D5FA0C)
	// 82D5F9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5F9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5F9D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5F9D4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82D5F9D8: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 82D5F9DC: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82D5F9E0: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82D5F9E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5F9E8: 4BFFFEC9  bl 0x82d5f8b0
	ctx.lr = 0x82D5F9EC;
	sub_82D5F8B0(ctx, base);
	// 82D5F9EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D5F9F0: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82D5F9F4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82D5F9F8: 4BFFFE61  bl 0x82d5f858
	ctx.lr = 0x82D5F9FC;
	sub_82D5F858(ctx, base);
	// 82D5F9FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5FA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FA10 size=140
    let mut pc: u32 = 0x82D5FA10;
    'dispatch: loop {
        match pc {
            0x82D5FA10 => {
    //   block [0x82D5FA10..0x82D5FA9C)
	// 82D5FA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FA14: 4BF499F9  bl 0x82ca940c
	ctx.lr = 0x82D5FA18;
	sub_82CA93D0(ctx, base);
	// 82D5FA18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FA1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5FA20: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FA28: 419A0068  beq cr6, 0x82d5fa90
	if ctx.cr[6].eq {
	pc = 0x82D5FA90; continue 'dispatch;
	}
	// 82D5FA2C: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FA30: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D5FA34: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D5FA38: 40990038  ble cr6, 0x82d5fa70
	if !ctx.cr[6].gt {
	pc = 0x82D5FA70; continue 'dispatch;
	}
	// 82D5FA3C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FA40: 7CBFE850  subf r5, r31, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82D5FA44: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5FA48: 7C9F5A14  add r4, r31, r11
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D5FA4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FA50: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FA54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FA58: 4E800421  bctrl
	ctx.lr = 0x82D5FA5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FA5C: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82D5FA60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5FA64: 419A0020  beq cr6, 0x82d5fa84
	if ctx.cr[6].eq {
	pc = 0x82D5FA84; continue 'dispatch;
	}
	// 82D5FA68: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D5FA6C: 4198FFD0  blt cr6, 0x82d5fa3c
	if ctx.cr[6].lt {
	pc = 0x82D5FA3C; continue 'dispatch;
	}
	// 82D5FA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5FA74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5FA78: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D5FA7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FA80: 4BF499DC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D5FA84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5FA88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FA8C: 4BF499D0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D5FA90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5FA94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FA98: 4BF499C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FAA0 size=184
    let mut pc: u32 = 0x82D5FAA0;
    'dispatch: loop {
        match pc {
            0x82D5FAA0 => {
    //   block [0x82D5FAA0..0x82D5FB58)
	// 82D5FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FAA4: 4BF4995D  bl 0x82ca9400
	ctx.lr = 0x82D5FAA8;
	sub_82CA93D0(ctx, base);
	// 82D5FAA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FAAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FAB0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82D5FAB4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D5FAB8: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82D5FABC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FAC0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5FAC4: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D5FAC8: 7F1AF000  cmpw cr6, r26, r30
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D5FACC: 40990050  ble cr6, 0x82d5fb1c
	if !ctx.cr[6].gt {
	pc = 0x82D5FB1C; continue 'dispatch;
	}
	// 82D5FAD0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5FAD4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5FAD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5FADC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5FAE0: 4BFF9251  bl 0x82d58d30
	ctx.lr = 0x82D5FAE4;
	sub_82D58D30(ctx, base);
	// 82D5FAE4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FAE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5FAEC: 7F6BF214  add r27, r11, r30
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D5FAF0: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82D5FAF4: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82D5FAF8: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 82D5FAFC: 4BFFFF15  bl 0x82d5fa10
	ctx.lr = 0x82D5FB00;
	sub_82D5FA10(ctx, base);
	// 82D5FB00: 7F03D800  cmpw cr6, r3, r27
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82D5FB04: 409A0048  bne cr6, 0x82d5fb4c
	if !ctx.cr[6].eq {
	pc = 0x82D5FB4C; continue 'dispatch;
	}
	// 82D5FB08: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FB0C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5FB10: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D5FB14: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D5FB18: 4199FFB8  bgt cr6, 0x82d5fad0
	if ctx.cr[6].gt {
	pc = 0x82D5FAD0; continue 'dispatch;
	}
	// 82D5FB1C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5FB20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D5FB24: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FB28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5FB2C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5FB30: 4BFF9201  bl 0x82d58d30
	ctx.lr = 0x82D5FB34;
	sub_82D58D30(ctx, base);
	// 82D5FB34: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FB38: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D5FB3C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D5FB40: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D5FB44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5FB48: 4BF49908  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82D5FB4C: 7C7DD050  subf r3, r29, r26
	ctx.r[3].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	// 82D5FB50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5FB54: 4BF498FC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FB58 size=76
    let mut pc: u32 = 0x82D5FB58;
    'dispatch: loop {
        match pc {
            0x82D5FB58 => {
    //   block [0x82D5FB58..0x82D5FBA4)
	// 82D5FB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FB60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FB64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FB68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FB6C: 4BFFFEA5  bl 0x82d5fa10
	ctx.lr = 0x82D5FB70;
	sub_82D5FA10(ctx, base);
	// 82D5FB70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FB74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FB78: 419A0018  beq cr6, 0x82d5fb90
	if ctx.cr[6].eq {
	pc = 0x82D5FB90; continue 'dispatch;
	}
	// 82D5FB7C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D5FB80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FB84: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5FB88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FB8C: 4E800421  bctrl
	ctx.lr = 0x82D5FB90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FB90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5FB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FB9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FBA8 size=192
    let mut pc: u32 = 0x82D5FBA8;
    'dispatch: loop {
        match pc {
            0x82D5FBA8 => {
    //   block [0x82D5FBA8..0x82D5FC68)
	// 82D5FBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FBAC: 4BF49861  bl 0x82ca940c
	ctx.lr = 0x82D5FBB0;
	sub_82CA93D0(ctx, base);
	// 82D5FBB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FBB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FBB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5FBBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D5FBC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FBC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FBC8: 419A002C  beq cr6, 0x82d5fbf4
	if ctx.cr[6].eq {
	pc = 0x82D5FBF4; continue 'dispatch;
	}
	// 82D5FBCC: 4BFFFE45  bl 0x82d5fa10
	ctx.lr = 0x82D5FBD0;
	sub_82D5FA10(ctx, base);
	// 82D5FBD0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FBD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D5FBD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5FBDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FBE0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D5FBE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FBE8: 4E800421  bctrl
	ctx.lr = 0x82D5FBEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FBEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FBF0: 4BF4986C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D5FBF4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82D5FBF8: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 82D5FBFC: 41980028  blt cr6, 0x82d5fc24
	if ctx.cr[6].lt {
	pc = 0x82D5FC24; continue 'dispatch;
	}
	// 82D5FC00: 419A0018  beq cr6, 0x82d5fc18
	if ctx.cr[6].eq {
	pc = 0x82D5FC18; continue 'dispatch;
	}
	// 82D5FC04: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 82D5FC08: 40980020  bge cr6, 0x82d5fc28
	if !ctx.cr[6].lt {
	pc = 0x82D5FC28; continue 'dispatch;
	}
	// 82D5FC0C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FC10: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82D5FC14: 48000014  b 0x82d5fc28
	pc = 0x82D5FC28; continue 'dispatch;
	// 82D5FC18: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FC1C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D5FC20: 48000008  b 0x82d5fc28
	pc = 0x82D5FC28; continue 'dispatch;
	// 82D5FC24: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D5FC28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5FC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5FC30: 40980018  bge cr6, 0x82d5fc48
	if !ctx.cr[6].lt {
	pc = 0x82D5FC48; continue 'dispatch;
	}
	// 82D5FC34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5FC38: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5FC3C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D5FC40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FC44: 4BF49818  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D5FC48: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5FC4C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D5FC50: 4099000C  ble cr6, 0x82d5fc5c
	if !ctx.cr[6].gt {
	pc = 0x82D5FC5C; continue 'dispatch;
	}
	// 82D5FC54: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82D5FC58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5FC5C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D5FC60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FC64: 4BF497F8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FC68 size=120
    let mut pc: u32 = 0x82D5FC68;
    'dispatch: loop {
        match pc {
            0x82D5FC68 => {
    //   block [0x82D5FC68..0x82D5FCE0)
	// 82D5FC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FC70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FC74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FC78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FC7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FC80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FC84: 419A003C  beq cr6, 0x82d5fcc0
	if ctx.cr[6].eq {
	pc = 0x82D5FCC0; continue 'dispatch;
	}
	// 82D5FC88: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D5FC8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FC90: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D5FC94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FC98: 4E800421  bctrl
	ctx.lr = 0x82D5FC9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FC9C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5FCA0: 41980028  blt cr6, 0x82d5fcc8
	if ctx.cr[6].lt {
	pc = 0x82D5FCC8; continue 'dispatch;
	}
	// 82D5FCA4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FCA8: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82D5FCAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5FCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FCB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FCBC: 4E800020  blr
	return;
	// 82D5FCC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5FCC4: 4BFFFFE0  b 0x82d5fca4
	pc = 0x82D5FCA4; continue 'dispatch;
	// 82D5FCC8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D5FCCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5FCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FCD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FCE0 size=128
    let mut pc: u32 = 0x82D5FCE0;
    'dispatch: loop {
        match pc {
            0x82D5FCE0 => {
    //   block [0x82D5FCE0..0x82D5FD60)
	// 82D5FCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FCE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FCEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FCF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D5FCF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FCF8: 394A4F68  addi r10, r10, 0x4f68
	ctx.r[10].s64 = ctx.r[10].s64 + 20328;
	// 82D5FCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D5FD00: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82D5FD04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82D5FD08: 7CCB0774  extsb r11, r6
	ctx.r[11].s64 = ctx.r[6].s8 as i64;
	// 82D5FD0C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D5FD10: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 82D5FD14: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82D5FD18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5FD1C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82D5FD20: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82D5FD24: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82D5FD28: 409A0008  bne cr6, 0x82d5fd30
	if !ctx.cr[6].eq {
	pc = 0x82D5FD30; continue 'dispatch;
	}
	// 82D5FD2C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82D5FD30: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82D5FD34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5FD38: 993F0018  stb r9, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 82D5FD3C: 419A000C  beq cr6, 0x82d5fd48
	if ctx.cr[6].eq {
	pc = 0x82D5FD48; continue 'dispatch;
	}
	// 82D5FD40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D5FD44: 4BFF8FFD  bl 0x82d58d40
	ctx.lr = 0x82D5FD48;
	sub_82D58D40(ctx, base);
	// 82D5FD48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5FD4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5FD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FD58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FD60 size=120
    let mut pc: u32 = 0x82D5FD60;
    'dispatch: loop {
        match pc {
            0x82D5FD60 => {
    //   block [0x82D5FD60..0x82D5FDD8)
	// 82D5FD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FD68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FD6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FD70: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FD74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FD78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5FD7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FD80: 419A001C  beq cr6, 0x82d5fd9c
	if ctx.cr[6].eq {
	pc = 0x82D5FD9C; continue 'dispatch;
	}
	// 82D5FD84: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D5FD88: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FD8C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5FD90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FD94: 4E800421  bctrl
	ctx.lr = 0x82D5FD98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FD98: 48000020  b 0x82d5fdb8
	pc = 0x82D5FDB8; continue 'dispatch;
	// 82D5FD9C: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5FDA0: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5FDA4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D5FDA8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D5FDAC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D5FDB0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D5FDB4: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D5FDB8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FDBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5FDC0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5FDC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FDD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FDD8 size=100
    let mut pc: u32 = 0x82D5FDD8;
    'dispatch: loop {
        match pc {
            0x82D5FDD8 => {
    //   block [0x82D5FDD8..0x82D5FE3C)
	// 82D5FDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FDE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FDE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FDE8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FDEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FDF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5FDF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FDF8: 419A001C  beq cr6, 0x82d5fe14
	if ctx.cr[6].eq {
	pc = 0x82D5FE14; continue 'dispatch;
	}
	// 82D5FDFC: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D5FE00: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FE04: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D5FE08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FE0C: 4E800421  bctrl
	ctx.lr = 0x82D5FE10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FE10: 4800000C  b 0x82d5fe1c
	pc = 0x82D5FE1C; continue 'dispatch;
	// 82D5FE14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5FE18: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D5FE1C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FE20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5FE24: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5FE28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FE34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FE40 size=180
    let mut pc: u32 = 0x82D5FE40;
    'dispatch: loop {
        match pc {
            0x82D5FE40 => {
    //   block [0x82D5FE40..0x82D5FEF4)
	// 82D5FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FE48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5FE4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FE50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FE54: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5FE58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FE5C: 396B4F68  addi r11, r11, 0x4f68
	ctx.r[11].s64 = ctx.r[11].s64 + 20328;
	// 82D5FE60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5FE64: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5FE68: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82D5FE6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5FE70: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D5FE74: 995F0018  stb r10, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82D5FE78: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FE7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FE80: 419A001C  beq cr6, 0x82d5fe9c
	if ctx.cr[6].eq {
	pc = 0x82D5FE9C; continue 'dispatch;
	}
	// 82D5FE84: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5FE88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5FE8C: 419A0010  beq cr6, 0x82d5fe9c
	if ctx.cr[6].eq {
	pc = 0x82D5FE9C; continue 'dispatch;
	}
	// 82D5FE90: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5FE94: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D5FE98: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D5FE9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FEA0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5FEA4: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D5FEA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5FEAC: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82D5FEB0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5FEB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FEB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FEBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FEC0: 4E800421  bctrl
	ctx.lr = 0x82D5FEC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FEC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5FEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5FECC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82D5FED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5FED4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D5FED8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82D5FEDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5FEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FEE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5FEEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FEF8 size=208
    let mut pc: u32 = 0x82D5FEF8;
    'dispatch: loop {
        match pc {
            0x82D5FEF8 => {
    //   block [0x82D5FEF8..0x82D5FFC8)
	// 82D5FEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FF00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FF04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FF08: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5FF0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FF10: 396B4F68  addi r11, r11, 0x4f68
	ctx.r[11].s64 = ctx.r[11].s64 + 20328;
	// 82D5FF14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5FF18: 4BFFFAF9  bl 0x82d5fa10
	ctx.lr = 0x82D5FF1C;
	sub_82D5FA10(ctx, base);
	// 82D5FF1C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FF20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5FF24: 419A0014  beq cr6, 0x82d5ff38
	if ctx.cr[6].eq {
	pc = 0x82D5FF38; continue 'dispatch;
	}
	// 82D5FF28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FF2C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5FF30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FF34: 4E800421  bctrl
	ctx.lr = 0x82D5FF38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FF38: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5FF3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5FF40: 419A003C  beq cr6, 0x82d5ff7c
	if ctx.cr[6].eq {
	pc = 0x82D5FF7C; continue 'dispatch;
	}
	// 82D5FF44: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5FF48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FF4C: 419A0030  beq cr6, 0x82d5ff7c
	if ctx.cr[6].eq {
	pc = 0x82D5FF7C; continue 'dispatch;
	}
	// 82D5FF50: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5FF54: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5FF58: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D5FF5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5FF60: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5FF64: 409A0018  bne cr6, 0x82d5ff7c
	if !ctx.cr[6].eq {
	pc = 0x82D5FF7C; continue 'dispatch;
	}
	// 82D5FF68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FF6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5FF70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FF74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FF78: 4E800421  bctrl
	ctx.lr = 0x82D5FF7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FF7C: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D5FF80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FF84: 419A0024  beq cr6, 0x82d5ffa8
	if ctx.cr[6].eq {
	pc = 0x82D5FFA8; continue 'dispatch;
	}
	// 82D5FF88: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FF8C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5FF90: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5FF94: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5FF98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FF9C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5FFA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5FFA4: 4E800421  bctrl
	ctx.lr = 0x82D5FFA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5FFA8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5FFAC: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D5FFB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5FFB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5FFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5FFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5FFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5FFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5FFC8 size=100
    let mut pc: u32 = 0x82D5FFC8;
    'dispatch: loop {
        match pc {
            0x82D5FFC8 => {
    //   block [0x82D5FFC8..0x82D6002C)
	// 82D5FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5FFD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5FFD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5FFD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5FFDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5FFE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5FFE4: 4BFFFF15  bl 0x82d5fef8
	ctx.lr = 0x82D5FFE8;
	sub_82D5FEF8(ctx, base);
	// 82D5FFE8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D5FFEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5FFF0: 419A0020  beq cr6, 0x82d60010
	if ctx.cr[6].eq {
	pc = 0x82D60010; continue 'dispatch;
	}
	// 82D5FFF4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5FFF8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5FFFC: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D60000: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D60004: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D60008: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D6000C: 4BFF52BD  bl 0x82d552c8
	ctx.lr = 0x82D60010;
	sub_82D552C8(ctx, base);
	// 82D60010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D60014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D60018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6001C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60020: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D60024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D60028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60030 size=4
    let mut pc: u32 = 0x82D60030;
    'dispatch: loop {
        match pc {
            0x82D60030 => {
    //   block [0x82D60030..0x82D60034)
	// 82D60030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60038 size=4
    let mut pc: u32 = 0x82D60038;
    'dispatch: loop {
        match pc {
            0x82D60038 => {
    //   block [0x82D60038..0x82D6003C)
	// 82D60038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D60040 size=28
    let mut pc: u32 = 0x82D60040;
    'dispatch: loop {
        match pc {
            0x82D60040 => {
    //   block [0x82D60040..0x82D6005C)
	// 82D60040: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60044: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60048: 7D6A5810  subfc r11, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D6004C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82D60050: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D60054: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D60058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60060 size=108
    let mut pc: u32 = 0x82D60060;
    'dispatch: loop {
        match pc {
            0x82D60060 => {
    //   block [0x82D60060..0x82D600CC)
	// 82D60060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D60068: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6006C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D60070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60078: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6007C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D60080: 419A0024  beq cr6, 0x82d600a4
	if ctx.cr[6].eq {
	pc = 0x82D600A4; continue 'dispatch;
	}
	// 82D60084: 3FC0832F  lis r30, -0x7cd1
	ctx.r[30].s64 = -2094071808;
	// 82D60088: 817E6D68  lwz r11, 0x6d68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28008 as u32) ) } as u64;
	// 82D6008C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60090: 4E800421  bctrl
	ctx.lr = 0x82D60094;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60094: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D60098: 817E6D68  lwz r11, 0x6d68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28008 as u32) ) } as u64;
	// 82D6009C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D600A0: 4E800421  bctrl
	ctx.lr = 0x82D600A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D600A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D600A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D600AC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D600B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D600B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D600B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D600BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D600C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D600C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D600C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D600D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D600D0 size=372
    let mut pc: u32 = 0x82D600D0;
    'dispatch: loop {
        match pc {
            0x82D600D0 => {
    //   block [0x82D600D0..0x82D60244)
	// 82D600D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D600D4: 4BF49325  bl 0x82ca93f8
	ctx.lr = 0x82D600D8;
	sub_82CA93D0(ctx, base);
	// 82D600D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D600DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D600E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D600E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82D600E8: 40990154  ble cr6, 0x82d6023c
	if !ctx.cr[6].gt {
	pc = 0x82D6023C; continue 'dispatch;
	}
	// 82D600EC: 3FC0832F  lis r30, -0x7cd1
	ctx.r[30].s64 = -2094071808;
	// 82D600F0: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D600F4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82D600F8: 817E6D64  lwz r11, 0x6d64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28004 as u32) ) } as u64;
	// 82D600FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60100: 4E800421  bctrl
	ctx.lr = 0x82D60104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60104: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60108: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D6010C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D60110: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D60114: 40990028  ble cr6, 0x82d6013c
	if !ctx.cr[6].gt {
	pc = 0x82D6013C; continue 'dispatch;
	}
	// 82D60118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6011C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60120: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D60124: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D60128: 7D2BD12E  stwx r9, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[9].u32) };
	// 82D6012C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D60130: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60134: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D60138: 4198FFE4  blt cr6, 0x82d6011c
	if ctx.cr[6].lt {
	pc = 0x82D6011C; continue 'dispatch;
	}
	// 82D6013C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60140: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82D60144: 4099001C  ble cr6, 0x82d60160
	if !ctx.cr[6].gt {
	pc = 0x82D60160; continue 'dispatch;
	}
	// 82D60148: 3D4082D6  lis r10, -0x7d2a
	ctx.r[10].s64 = -2099904512;
	// 82D6014C: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82D60150: 38CA0040  addi r6, r10, 0x40
	ctx.r[6].s64 = ctx.r[10].s64 + 64;
	// 82D60154: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D60158: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D6015C: 48031335  bl 0x82d91490
	ctx.lr = 0x82D60160;
	sub_82D91490(ctx, base);
	// 82D60160: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60164: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82D60168: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D6016C: 817E6D64  lwz r11, 0x6d64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28004 as u32) ) } as u64;
	// 82D60170: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60174: 4E800421  bctrl
	ctx.lr = 0x82D60178;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60178: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6017C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D60180: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82D60184: 1C6B0054  mulli r3, r11, 0x54
	ctx.r[3].s64 = ctx.r[11].s64 * 84;
	// 82D60188: 817E6D64  lwz r11, 0x6d64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28004 as u32) ) } as u64;
	// 82D6018C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60190: 4E800421  bctrl
	ctx.lr = 0x82D60194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60194: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60198: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82D6019C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82D601A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D601A4: 4099005C  ble cr6, 0x82d60200
	if !ctx.cr[6].gt {
	pc = 0x82D60200; continue 'dispatch;
	}
	// 82D601A8: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82D601AC: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82D601B0: 7F7AC850  subf r27, r26, r25
	ctx.r[27].s64 = ctx.r[25].s64 - ctx.r[26].s64;
	// 82D601B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D601B8: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 82D601BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D601C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D601C4: 7D7BF12E  stwx r11, r27, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 82D601C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D601CC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D601D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D601D4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82D601D8: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82D601DC: 1D6B0054  mulli r11, r11, 0x54
	ctx.r[11].s64 = ctx.r[11].s64 * 84;
	// 82D601E0: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D601E4: 4BF4929D  bl 0x82ca9480
	ctx.lr = 0x82D601E8;
	sub_82CA9480(ctx, base);
	// 82D601E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D601EC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82D601F0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82D601F4: 3BBD0054  addi r29, r29, 0x54
	ctx.r[29].s64 = ctx.r[29].s64 + 84;
	// 82D601F8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D601FC: 4198FFB8  blt cr6, 0x82d601b4
	if ctx.cr[6].lt {
	pc = 0x82D601B4; continue 'dispatch;
	}
	// 82D60200: 3FC0832F  lis r30, -0x7cd1
	ctx.r[30].s64 = -2094071808;
	// 82D60204: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D60208: 817E6D68  lwz r11, 0x6d68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28008 as u32) ) } as u64;
	// 82D6020C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60210: 4E800421  bctrl
	ctx.lr = 0x82D60214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60214: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60218: 817E6D68  lwz r11, 0x6d68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28008 as u32) ) } as u64;
	// 82D6021C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60220: 4E800421  bctrl
	ctx.lr = 0x82D60224;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60224: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D60228: 817E6D68  lwz r11, 0x6d68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28008 as u32) ) } as u64;
	// 82D6022C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60230: 4E800421  bctrl
	ctx.lr = 0x82D60234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60234: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82D60238: 931F0004  stw r24, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82D6023C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D60240: 4BF49208  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60248 size=164
    let mut pc: u32 = 0x82D60248;
    'dispatch: loop {
        match pc {
            0x82D60248 => {
    //   block [0x82D60248..0x82D602EC)
	// 82D60248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6024C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D60250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D60254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D60258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6025C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60260: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D60264: 4BFFFE6D  bl 0x82d600d0
	ctx.lr = 0x82D60268;
	sub_82D600D0(ctx, base);
	// 82D60268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D6026C: 4BFFFE65  bl 0x82d600d0
	ctx.lr = 0x82D60270;
	sub_82D600D0(ctx, base);
	// 82D60270: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60274: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60278: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6027C: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60280: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60284: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D60288: 7CE85A14  add r7, r8, r11
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D6028C: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82D60290: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82D60294: 40980034  bge cr6, 0x82d602c8
	if !ctx.cr[6].lt {
	pc = 0x82D602C8; continue 'dispatch;
	}
	// 82D60298: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D6029C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82D602A0: 40980028  bge cr6, 0x82d602c8
	if !ctx.cr[6].lt {
	pc = 0x82D602C8; continue 'dispatch;
	}
	// 82D602A4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D602A8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D602AC: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D602B0: 409A0030  bne cr6, 0x82d602e0
	if !ctx.cr[6].eq {
	pc = 0x82D602E0; continue 'dispatch;
	}
	// 82D602B4: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82D602B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D602BC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D602C0: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82D602C4: 4198FFD8  blt cr6, 0x82d6029c
	if ctx.cr[6].lt {
	pc = 0x82D6029C; continue 'dispatch;
	}
	// 82D602C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D602CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D602D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D602D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D602D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D602DC: 4E800020  blr
	return;
	// 82D602E0: 4098FFDC  bge cr6, 0x82d602bc
	if !ctx.cr[6].lt {
	pc = 0x82D602BC; continue 'dispatch;
	}
	// 82D602E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D602E8: 4BFFFFD8  b 0x82d602c0
	pc = 0x82D602C0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D602F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D602F0 size=12
    let mut pc: u32 = 0x82D602F0;
    'dispatch: loop {
        match pc {
            0x82D602F0 => {
    //   block [0x82D602F0..0x82D602FC)
	// 82D602F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D602F4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D602F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60300 size=160
    let mut pc: u32 = 0x82D60300;
    'dispatch: loop {
        match pc {
            0x82D60300 => {
    //   block [0x82D60300..0x82D603A0)
	// 82D60300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D60308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6030C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60310: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D60314: 419A0020  beq cr6, 0x82d60334
	if ctx.cr[6].eq {
	pc = 0x82D60334; continue 'dispatch;
	}
	// 82D60318: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6031C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D60320: 419A0014  beq cr6, 0x82d60334
	if ctx.cr[6].eq {
	pc = 0x82D60334; continue 'dispatch;
	}
	// 82D60324: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60328: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6032C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D60330: 4BFF51A1  bl 0x82d554d0
	ctx.lr = 0x82D60334;
	sub_82D554D0(ctx, base);
	// 82D60334: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60338: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82D6033C: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D60340: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D60344: 419A004C  beq cr6, 0x82d60390
	if ctx.cr[6].eq {
	pc = 0x82D60390; continue 'dispatch;
	}
	// 82D60348: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6034C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D60350: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D60354: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D60358: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D6035C: 4198001C  blt cr6, 0x82d60378
	if ctx.cr[6].lt {
	pc = 0x82D60378; continue 'dispatch;
	}
	// 82D60360: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D60364: 4BFF4DC5  bl 0x82d55128
	ctx.lr = 0x82D60368;
	sub_82D55128(ctx, base);
	// 82D60368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D6036C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60374: 4E800020  blr
	return;
	// 82D60378: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6037C: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D60380: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60384: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D60388: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D6038C: 90A30050  stw r5, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82D60390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D60394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6039C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D603A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D603A0 size=184
    let mut pc: u32 = 0x82D603A0;
    'dispatch: loop {
        match pc {
            0x82D603A0 => {
    //   block [0x82D603A0..0x82D60458)
	// 82D603A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D603A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D603A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D603AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D603B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D603B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D603B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D603BC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D603C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D603C4: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82D603C8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D603CC: 419A0074  beq cr6, 0x82d60440
	if ctx.cr[6].eq {
	pc = 0x82D60440; continue 'dispatch;
	}
	// 82D603D0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D603D4: 40990064  ble cr6, 0x82d60438
	if !ctx.cr[6].gt {
	pc = 0x82D60438; continue 'dispatch;
	}
	// 82D603D8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D603DC: 419A0020  beq cr6, 0x82d603fc
	if ctx.cr[6].eq {
	pc = 0x82D603FC; continue 'dispatch;
	}
	// 82D603E0: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D603E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D603E8: 419A0014  beq cr6, 0x82d603fc
	if ctx.cr[6].eq {
	pc = 0x82D603FC; continue 'dispatch;
	}
	// 82D603EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D603F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D603F4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D603F8: 4BFF50D9  bl 0x82d554d0
	ctx.lr = 0x82D603FC;
	sub_82D554D0(ctx, base);
	// 82D603FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D60400: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60404: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D60408: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82D6040C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D60410: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82D60414: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D60418: 4BFF5061  bl 0x82d55478
	ctx.lr = 0x82D6041C;
	sub_82D55478(ctx, base);
	// 82D6041C: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82D60420: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D60424: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82D60428: 394BFFE0  addi r10, r11, -0x20
	ctx.r[10].s64 = ctx.r[11].s64 + -32;
	// 82D6042C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D60430: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D60434: 4800000C  b 0x82d60440
	pc = 0x82D60440; continue 'dispatch;
	// 82D60438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6043C: 4BFFFEC5  bl 0x82d60300
	ctx.lr = 0x82D60440;
	sub_82D60300(ctx, base);
	// 82D60440: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D60444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6044C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D60450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D60454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60458 size=96
    let mut pc: u32 = 0x82D60458;
    'dispatch: loop {
        match pc {
            0x82D60458 => {
    //   block [0x82D60458..0x82D604B8)
	// 82D60458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6045C: 4BF48FB1  bl 0x82ca940c
	ctx.lr = 0x82D60460;
	sub_82CA93D0(ctx, base);
	// 82D60460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60468: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D6046C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D60470: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D60474: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D60478: 419A0018  beq cr6, 0x82d60490
	if ctx.cr[6].eq {
	pc = 0x82D60490; continue 'dispatch;
	}
	// 82D6047C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60480: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60484: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D60488: 41820008  beq 0x82d60490
	if ctx.cr[0].eq {
	pc = 0x82D60490; continue 'dispatch;
	}
	// 82D6048C: 4BFFFE75  bl 0x82d60300
	ctx.lr = 0x82D60490;
	sub_82D60300(ctx, base);
	// 82D60490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D60494: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82D60498: 995F0010  stb r10, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82D6049C: 394BFFE0  addi r10, r11, -0x20
	ctx.r[10].s64 = ctx.r[11].s64 + -32;
	// 82D604A0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82D604A4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82D604A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D604AC: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D604B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D604B4: 4BF48FA8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D604B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D604B8 size=128
    let mut pc: u32 = 0x82D604B8;
    'dispatch: loop {
        match pc {
            0x82D604B8 => {
    //   block [0x82D604B8..0x82D60538)
	// 82D604B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D604BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D604C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D604C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D604C8: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D604CC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82D604D0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D604D4: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D604D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D604DC: 419A001C  beq cr6, 0x82d604f8
	if ctx.cr[6].eq {
	pc = 0x82D604F8; continue 'dispatch;
	}
	// 82D604E0: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D604E4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D604E8: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82D604EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D604F0: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82D604F4: 48000010  b 0x82d60504
	pc = 0x82D60504; continue 'dispatch;
	// 82D604F8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D604FC: 4BFF4B55  bl 0x82d55050
	ctx.lr = 0x82D60500;
	sub_82D55050(ctx, base);
	// 82D60500: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D60504: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82D60508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D6050C: 7D7F492E  stwx r11, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82D60510: 994B0010  stb r10, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82D60514: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D60518: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D6051C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D60520: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D60524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D60528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6052C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D60534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60538 size=32
    let mut pc: u32 = 0x82D60538;
    'dispatch: loop {
        match pc {
            0x82D60538 => {
    //   block [0x82D60538..0x82D60558)
	// 82D60538: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D6053C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82D60540: 54ABE8FE  srwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60544: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D60548: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D6054C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82D60550: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D60554: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60558 size=36
    let mut pc: u32 = 0x82D60558;
    'dispatch: loop {
        match pc {
            0x82D60558 => {
    //   block [0x82D60558..0x82D6057C)
	// 82D60558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D6055C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82D60560: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60564: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D60568: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D6056C: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82D60570: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D60574: 409AFFEC  bne cr6, 0x82d60560
	if !ctx.cr[6].eq {
	pc = 0x82D60560; continue 'dispatch;
	}
	// 82D60578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60580 size=68
    let mut pc: u32 = 0x82D60580;
    'dispatch: loop {
        match pc {
            0x82D60580 => {
    //   block [0x82D60580..0x82D605C4)
	// 82D60580: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60584: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60588: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6058C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D60590: 40990024  ble cr6, 0x82d605b4
	if !ctx.cr[6].gt {
	pc = 0x82D605B4; continue 'dispatch;
	}
	// 82D60594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D60598: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82D6059C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D605A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D605A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D605A8: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82D605AC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D605B0: 409AFFEC  bne cr6, 0x82d6059c
	if !ctx.cr[6].eq {
	pc = 0x82D6059C; continue 'dispatch;
	}
	// 82D605B4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D605B8: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D605BC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D605C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D605C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D605C8 size=32
    let mut pc: u32 = 0x82D605C8;
    'dispatch: loop {
        match pc {
            0x82D605C8 => {
    //   block [0x82D605C8..0x82D605E8)
	// 82D605C8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D605CC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82D605D0: 54ABE13E  srwi r11, r5, 4
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D605D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D605D8: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D605DC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82D605E0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D605E4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D605E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D605E8 size=36
    let mut pc: u32 = 0x82D605E8;
    'dispatch: loop {
        match pc {
            0x82D605E8 => {
    //   block [0x82D605E8..0x82D6060C)
	// 82D605E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D605EC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82D605F0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D605F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D605F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D605FC: 7D2A412A  stdx r9, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 82D60600: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82D60604: 409AFFEC  bne cr6, 0x82d605f0
	if !ctx.cr[6].eq {
	pc = 0x82D605F0; continue 'dispatch;
	}
	// 82D60608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60610 size=32
    let mut pc: u32 = 0x82D60610;
    'dispatch: loop {
        match pc {
            0x82D60610 => {
    //   block [0x82D60610..0x82D60630)
	// 82D60610: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82D60614: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82D60618: 40990010  ble cr6, 0x82d60628
	if !ctx.cr[6].gt {
	pc = 0x82D60628; continue 'dispatch;
	}
	// 82D6061C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60620: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82D60624: 4198FFF8  blt cr6, 0x82d6061c
	if ctx.cr[6].lt {
	pc = 0x82D6061C; continue 'dispatch;
	}
	// 82D60628: 55632834  slwi r3, r11, 5
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D6062C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60630 size=52
    let mut pc: u32 = 0x82D60630;
    'dispatch: loop {
        match pc {
            0x82D60630 => {
    //   block [0x82D60630..0x82D60664)
	// 82D60630: 3D209E37  lis r9, -0x61c9
	ctx.r[9].s64 = -1640562688;
	// 82D60634: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60638: 548BE13E  srwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6063C: 612879B1  ori r8, r9, 0x79b1
	ctx.r[8].u64 = ctx.r[9].u64 | 31153;
	// 82D60640: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60644: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82D60648: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82D6064C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60650: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D60654: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82D60658: 419A0024  beq cr6, 0x82d6067c
	if ctx.cr[6].eq {
		sub_82D60664(ctx, base);
		return;
	}
	// 82D6065C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D60660: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60664 size=32
    let mut pc: u32 = 0x82D60664;
    'dispatch: loop {
        match pc {
            0x82D60664 => {
    //   block [0x82D60664..0x82D60684)
	// 82D60664: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82D60668: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82D6066C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60670: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D60674: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82D60678: 409AFFE4  bne cr6, 0x82d6065c
	if !ctx.cr[6].eq {
		sub_82D60630(ctx, base);
		return;
	}
	// 82D6067C: 386A0001  addi r3, r10, 1
	ctx.r[3].s64 = ctx.r[10].s64 + 1;
	// 82D60680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60688 size=88
    let mut pc: u32 = 0x82D60688;
    'dispatch: loop {
        match pc {
            0x82D60688 => {
    //   block [0x82D60688..0x82D606E0)
	// 82D60688: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D6068C: 3D009E37  lis r8, -0x61c9
	ctx.r[8].s64 = -1640562688;
	// 82D60690: 548AE13E  srwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60694: 610779B1  ori r7, r8, 0x79b1
	ctx.r[7].u64 = ctx.r[8].u64 | 31153;
	// 82D60698: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6069C: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D606A0: 7D6A39D6  mullw r11, r10, r7
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82D606A4: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82D606A8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D606AC: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D606B0: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82D606B4: 419A0024  beq cr6, 0x82d606d8
	if ctx.cr[6].eq {
	pc = 0x82D606D8; continue 'dispatch;
	}
	// 82D606B8: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D606BC: 419A0024  beq cr6, 0x82d606e0
	if ctx.cr[6].eq {
		sub_82D606E0(ctx, base);
		return;
	}
	// 82D606C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D606C4: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82D606C8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D606CC: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D606D0: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82D606D4: 409AFFE4  bne cr6, 0x82d606b8
	if !ctx.cr[6].eq {
	pc = 0x82D606B8; continue 'dispatch;
	}
	// 82D606D8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D606DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D606E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D606E0 size=20
    let mut pc: u32 = 0x82D606E0;
    'dispatch: loop {
        match pc {
            0x82D606E0 => {
    //   block [0x82D606E0..0x82D606F4)
	// 82D606E0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D606E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D606E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D606EC: 7C6B402E  lwzx r3, r11, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D606F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D606F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D606F8 size=124
    let mut pc: u32 = 0x82D606F8;
    'dispatch: loop {
        match pc {
            0x82D606F8 => {
    //   block [0x82D606F8..0x82D60774)
	// 82D606F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D606FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D60700: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D60704: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D60708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6070C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60710: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D60714: 4BFFFF1D  bl 0x82d60630
	ctx.lr = 0x82D60718;
	sub_82D60630(ctx, base);
	// 82D60718: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6071C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D60720: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D60724: 40990008  ble cr6, 0x82d6072c
	if !ctx.cr[6].gt {
	pc = 0x82D6072C; continue 'dispatch;
	}
	// 82D60728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D6072C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D60730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D60734: 419A0024  beq cr6, 0x82d60758
	if ctx.cr[6].eq {
	pc = 0x82D60758; continue 'dispatch;
	}
	// 82D60738: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82D6073C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60740: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D60744: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60748: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6074C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D60750: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D60754: 48000008  b 0x82d6075c
	pc = 0x82D6075C; continue 'dispatch;
	// 82D60758: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D6075C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D60760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60768: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6076C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D60770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60778 size=320
    let mut pc: u32 = 0x82D60778;
    'dispatch: loop {
        match pc {
            0x82D60778 => {
    //   block [0x82D60778..0x82D608B8)
	// 82D60778: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82D6077C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D60780: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D60784: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 82D60788: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6078C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82D60790: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D60794: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60798: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D6079C: 7CAA492E  stwx r5, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[5].u32) };
	// 82D607A0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D607A4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D607A8: 7D6A4214  add r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D607AC: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82D607B0: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D607B4: 7CE7482E  lwzx r7, r7, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D607B8: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 82D607BC: 419A0020  beq cr6, 0x82d607dc
	if ctx.cr[6].eq {
	pc = 0x82D607DC; continue 'dispatch;
	}
	// 82D607C0: 5527003E  slwi r7, r9, 0
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D607C4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D607C8: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82D607CC: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D607D0: 7CC6382E  lwzx r6, r6, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D607D4: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 82D607D8: 409AFFEC  bne cr6, 0x82d607c4
	if !ctx.cr[6].eq {
	pc = 0x82D607C4; continue 'dispatch;
	}
	// 82D607DC: 38E80001  addi r7, r8, 1
	ctx.r[7].s64 = ctx.r[8].s64 + 1;
	// 82D607E0: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82D607E4: 7CEB5038  and r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 82D607E8: 7CDF5038  and r31, r6, r10
	ctx.r[31].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82D607EC: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D607F0: 7D29382E  lwzx r9, r9, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D607F4: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D607F8: 419A00B4  beq cr6, 0x82d608ac
	if ctx.cr[6].eq {
	pc = 0x82D608AC; continue 'dispatch;
	}
	// 82D607FC: 3D209E37  lis r9, -0x61c9
	ctx.r[9].s64 = -1640562688;
	// 82D60800: 612479B1  ori r4, r9, 0x79b1
	ctx.r[4].u64 = ctx.r[9].u64 | 31153;
	// 82D60804: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60808: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82D6080C: 7CC9382E  lwzx r6, r9, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D60810: 54DEE13E  srwi r30, r6, 4
	ctx.r[30].u32 = ctx.r[6].u32.wrapping_shr(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D60814: 7FDE21D6  mullw r30, r30, r4
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82D60818: 7FCA5038  and r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 & ctx.r[10].u64;
	// 82D6081C: 4198000C  blt cr6, 0x82d60828
	if ctx.cr[6].lt {
	pc = 0x82D60828; continue 'dispatch;
	}
	// 82D60820: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D60824: 41990068  bgt cr6, 0x82d6088c
	if ctx.cr[6].gt {
	pc = 0x82D6088C; continue 'dispatch;
	}
	// 82D60828: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D6082C: 40980014  bge cr6, 0x82d60840
	if !ctx.cr[6].lt {
	pc = 0x82D60840; continue 'dispatch;
	}
	// 82D60830: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D60834: 41990058  bgt cr6, 0x82d6088c
	if ctx.cr[6].gt {
	pc = 0x82D6088C; continue 'dispatch;
	}
	// 82D60838: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D6083C: 40990050  ble cr6, 0x82d6088c
	if !ctx.cr[6].gt {
	pc = 0x82D6088C; continue 'dispatch;
	}
	// 82D60840: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D60844: 4099000C  ble cr6, 0x82d60850
	if !ctx.cr[6].gt {
	pc = 0x82D60850; continue 'dispatch;
	}
	// 82D60848: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82D6084C: 41980040  blt cr6, 0x82d6088c
	if ctx.cr[6].lt {
	pc = 0x82D6088C; continue 'dispatch;
	}
	// 82D60850: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60854: 7CCA492E  stwx r6, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u32) };
	// 82D60858: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6085C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60860: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D60864: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D60868: 39060001  addi r8, r6, 1
	ctx.r[8].s64 = ctx.r[6].s64 + 1;
	// 82D6086C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D60870: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D60874: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60878: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82D6087C: 7CC6482E  lwzx r6, r6, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D60880: 7CCA492E  stwx r6, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u32) };
	// 82D60884: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60888: 7CA7512E  stwx r5, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 82D6088C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60890: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60894: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60898: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82D6089C: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D608A0: 7D27482E  lwzx r9, r7, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D608A4: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D608A8: 409AFF5C  bne cr6, 0x82d60804
	if !ctx.cr[6].eq {
	pc = 0x82D60804; continue 'dispatch;
	}
	// 82D608AC: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D608B0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82D608B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D608B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D608B8 size=16
    let mut pc: u32 = 0x82D608B8;
    'dispatch: loop {
        match pc {
            0x82D608B8 => {
    //   block [0x82D608B8..0x82D608C8)
	// 82D608B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D608BC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D608C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D608C4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D608C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D608C8 size=36
    let mut pc: u32 = 0x82D608C8;
    'dispatch: loop {
        match pc {
            0x82D608C8 => {
    //   block [0x82D608C8..0x82D608EC)
	// 82D608C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D608CC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D608D0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D608D4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D608D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D608DC: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D608E0: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D608E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D608E8: 4BFF49E0  b 0x82d552c8
	sub_82D552C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D608EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D608EC size=4
    let mut pc: u32 = 0x82D608EC;
    'dispatch: loop {
        match pc {
            0x82D608EC => {
    //   block [0x82D608EC..0x82D608F0)
	// 82D608EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D608F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D608F0 size=220
    let mut pc: u32 = 0x82D608F0;
    'dispatch: loop {
        match pc {
            0x82D608F0 => {
    //   block [0x82D608F0..0x82D609CC)
	// 82D608F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D608F4: 4BF48B19  bl 0x82ca940c
	ctx.lr = 0x82D608F8;
	sub_82CA93D0(ctx, base);
	// 82D608F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D608FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60900: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D60904: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D60908: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6090C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60910: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60914: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D60918: 40990010  ble cr6, 0x82d60928
	if !ctx.cr[6].gt {
	pc = 0x82D60928; continue 'dispatch;
	}
	// 82D6091C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60920: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D60924: 4800025D  bl 0x82d60b80
	ctx.lr = 0x82D60928;
	sub_82D60B80(ctx, base);
	// 82D60928: 392079B1  li r9, 0x79b1
	ctx.r[9].s64 = 31153;
	// 82D6092C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60930: 7BCAE102  rldicl r10, r30, 0x3c, 4
	ctx.r[10].u64 = ctx.r[30].u64 & 0x000000000000000Fu64;
	// 82D60934: 65279E37  oris r7, r9, 0x9e37
	ctx.r[7].u64 = ctx.r[9].u64 | 2654404608;
	// 82D60938: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6093C: 7D6807B4  extsw r8, r11
	ctx.r[8].s64 = ctx.r[11].s32 as i64;
	// 82D60940: 7D6A39D2  mulld r11, r10, r7
	ctx.r[11].s64 = ctx.r[10].s64 * ctx.r[7].s64;
	// 82D60944: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82D60948: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6094C: 7CE9502A  ldx r7, r9, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	// 82D60950: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 82D60954: 419A002C  beq cr6, 0x82d60980
	if ctx.cr[6].eq {
	pc = 0x82D60980; continue 'dispatch;
	}
	// 82D60958: 7D49502A  ldx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	// 82D6095C: 7F2AF040  cmpld cr6, r10, r30
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[30].u64, &mut ctx.xer);
	// 82D60960: 419A0020  beq cr6, 0x82d60980
	if ctx.cr[6].eq {
	pc = 0x82D60980; continue 'dispatch;
	}
	// 82D60964: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60968: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6096C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82D60970: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60974: 7CEA382A  ldx r7, r10, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) };
	// 82D60978: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 82D6097C: 409AFFDC  bne cr6, 0x82d60958
	if !ctx.cr[6].eq {
	pc = 0x82D60958; continue 'dispatch;
	}
	// 82D60980: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60984: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82D60988: 55481838  slwi r8, r10, 3
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6098C: 7D68482A  ldx r11, r8, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	// 82D60990: 7F2BF040  cmpld cr6, r11, r30
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[30].u64, &mut ctx.xer);
	// 82D60994: 409A0008  bne cr6, 0x82d6099c
	if !ctx.cr[6].eq {
	pc = 0x82D6099C; continue 'dispatch;
	}
	// 82D60998: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D6099C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D609A0: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82D609A4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D609A8: 7FC8492A  stdx r30, r8, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u64) };
	// 82D609AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D609B0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D609B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D609B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D609BC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D609C0: 7FAB492A  stdx r29, r11, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u64) };
	// 82D609C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D609C8: 4BF48A94  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D609D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D609D0 size=88
    let mut pc: u32 = 0x82D609D0;
    'dispatch: loop {
        match pc {
            0x82D609D0 => {
    //   block [0x82D609D0..0x82D60A28)
	// 82D609D0: 394079B1  li r10, 0x79b1
	ctx.r[10].s64 = 31153;
	// 82D609D4: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D609D8: 788BE102  rldicl r11, r4, 0x3c, 4
	ctx.r[11].u64 = ctx.r[4].u64 & 0x000000000000000Fu64;
	// 82D609DC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D609E0: 654A9E37  oris r10, r10, 0x9e37
	ctx.r[10].u64 = ctx.r[10].u64 | 2654404608;
	// 82D609E4: 7CE807B4  extsw r8, r7
	ctx.r[8].s64 = ctx.r[7].s32 as i64;
	// 82D609E8: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 82D609EC: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82D609F0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D609F4: 7D4A482A  ldx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82D609F8: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 82D609FC: 419A0024  beq cr6, 0x82d60a20
	if ctx.cr[6].eq {
	pc = 0x82D60A20; continue 'dispatch;
	}
	// 82D60A00: 7F2A2040  cmpld cr6, r10, r4
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[4].u64, &mut ctx.xer);
	// 82D60A04: 419A0024  beq cr6, 0x82d60a28
	if ctx.cr[6].eq {
		sub_82D60A28(ctx, base);
		return;
	}
	// 82D60A08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60A0C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82D60A10: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60A14: 7D4A482A  ldx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82D60A18: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 82D60A1C: 409AFFE4  bne cr6, 0x82d60a00
	if !ctx.cr[6].eq {
	pc = 0x82D60A00; continue 'dispatch;
	}
	// 82D60A20: 38670001  addi r3, r7, 1
	ctx.r[3].s64 = ctx.r[7].s64 + 1;
	// 82D60A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60A28 size=8
    let mut pc: u32 = 0x82D60A28;
    'dispatch: loop {
        match pc {
            0x82D60A28 => {
    //   block [0x82D60A28..0x82D60A30)
	// 82D60A28: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D60A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60A30 size=332
    let mut pc: u32 = 0x82D60A30;
    'dispatch: loop {
        match pc {
            0x82D60A30 => {
    //   block [0x82D60A30..0x82D60B7C)
	// 82D60A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60A34: 4BF489D9  bl 0x82ca940c
	ctx.lr = 0x82D60A38;
	sub_82CA93D0(ctx, base);
	// 82D60A38: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D60A3C: 7C8507B4  extsw r5, r4
	ctx.r[5].s64 = ctx.r[4].s32 as i64;
	// 82D60A40: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60A44: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82D60A48: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D60A4C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D60A50: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60A54: 7FCB512A  stdx r30, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u64) };
	// 82D60A58: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60A5C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60A60: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82D60A64: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D60A68: 7D495838  and r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82D60A6C: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60A70: 7D4A402A  ldx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) };
	// 82D60A74: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 82D60A78: 419A0020  beq cr6, 0x82d60a98
	if ctx.cr[6].eq {
	pc = 0x82D60A98; continue 'dispatch;
	}
	// 82D60A7C: 550A003E  slwi r10, r8, 0
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D60A80: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D60A84: 7D295838  and r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82D60A88: 55271838  slwi r7, r9, 3
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D60A8C: 7CE7502A  ldx r7, r7, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	// 82D60A90: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 82D60A94: 409AFFEC  bne cr6, 0x82d60a80
	if !ctx.cr[6].eq {
	pc = 0x82D60A80; continue 'dispatch;
	}
	// 82D60A98: 39450001  addi r10, r5, 1
	ctx.r[10].s64 = ctx.r[5].s64 + 1;
	// 82D60A9C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D60AA0: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82D60AA4: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82D60AA8: 5546003E  slwi r6, r10, 0
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D60AAC: 54C71838  slwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D60AB0: 7D28382A  ldx r9, r8, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	// 82D60AB4: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 82D60AB8: 419A00C0  beq cr6, 0x82d60b78
	if ctx.cr[6].eq {
	pc = 0x82D60B78; continue 'dispatch;
	}
	// 82D60ABC: 392079B1  li r9, 0x79b1
	ctx.r[9].s64 = 31153;
	// 82D60AC0: 65249E37  oris r4, r9, 0x9e37
	ctx.r[4].u64 = ctx.r[9].u64 | 2654404608;
	// 82D60AC4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60AC8: 7F2AF840  cmpld cr6, r10, r31
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[31].u64, &mut ctx.xer);
	// 82D60ACC: 7D09382A  ldx r8, r9, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) };
	// 82D60AD0: 791DE102  rldicl r29, r8, 0x3c, 4
	ctx.r[29].u64 = ctx.r[8].u64 & 0x000000000000000Fu64;
	// 82D60AD4: 7FBD21D2  mulld r29, r29, r4
	ctx.r[29].s64 = ctx.r[29].s64 * ctx.r[4].s64;
	// 82D60AD8: 7FAB5838  and r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 & ctx.r[11].u64;
	// 82D60ADC: 4198000C  blt cr6, 0x82d60ae8
	if ctx.cr[6].lt {
	pc = 0x82D60AE8; continue 'dispatch;
	}
	// 82D60AE0: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 82D60AE4: 4199006C  bgt cr6, 0x82d60b50
	if ctx.cr[6].gt {
	pc = 0x82D60B50; continue 'dispatch;
	}
	// 82D60AE8: 7F2A2840  cmpld cr6, r10, r5
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[5].u64, &mut ctx.xer);
	// 82D60AEC: 40980014  bge cr6, 0x82d60b00
	if !ctx.cr[6].lt {
	pc = 0x82D60B00; continue 'dispatch;
	}
	// 82D60AF0: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 82D60AF4: 4199005C  bgt cr6, 0x82d60b50
	if ctx.cr[6].gt {
	pc = 0x82D60B50; continue 'dispatch;
	}
	// 82D60AF8: 7F2B5040  cmpld cr6, r11, r10
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[10].u64, &mut ctx.xer);
	// 82D60AFC: 40990054  ble cr6, 0x82d60b50
	if !ctx.cr[6].gt {
	pc = 0x82D60B50; continue 'dispatch;
	}
	// 82D60B00: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 82D60B04: 4099000C  ble cr6, 0x82d60b10
	if !ctx.cr[6].gt {
	pc = 0x82D60B10; continue 'dispatch;
	}
	// 82D60B08: 7F2BF840  cmpld cr6, r11, r31
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[31].u64, &mut ctx.xer);
	// 82D60B0C: 41980044  blt cr6, 0x82d60b50
	if ctx.cr[6].lt {
	pc = 0x82D60B50; continue 'dispatch;
	}
	// 82D60B10: 54AB003E  slwi r11, r5, 0
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60B14: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82D60B18: 557D1838  slwi r29, r11, 3
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D60B1C: 7D1D492A  stdx r8, r29, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u64) };
	// 82D60B20: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60B24: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60B28: 7CC93214  add r6, r9, r6
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 82D60B2C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D60B30: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 82D60B34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D60B38: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D60B3C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60B40: 7D29402A  ldx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	// 82D60B44: 7D2B412A  stdx r9, r11, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 82D60B48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60B4C: 7FC7592A  stdx r30, r7, r11
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u64) };
	// 82D60B50: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60B54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D60B58: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60B5C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82D60B60: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82D60B64: 5546003E  slwi r6, r10, 0
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D60B68: 54C71838  slwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D60B6C: 7D27482A  ldx r9, r7, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) };
	// 82D60B70: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 82D60B74: 409AFF50  bne cr6, 0x82d60ac4
	if !ctx.cr[6].eq {
	pc = 0x82D60AC4; continue 'dispatch;
	}
	// 82D60B78: 4BF488E4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60B80 size=232
    let mut pc: u32 = 0x82D60B80;
    'dispatch: loop {
        match pc {
            0x82D60B80 => {
    //   block [0x82D60B80..0x82D60C68)
	// 82D60B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60B84: 4BF48871  bl 0x82ca93f4
	ctx.lr = 0x82D60B88;
	sub_82CA93D0(ctx, base);
	// 82D60B88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60B8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60B90: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60B94: 3B200004  li r25, 4
	ctx.r[25].s64 = 4;
	// 82D60B98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D60B9C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D60BA0: 57C42036  slwi r4, r30, 4
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D60BA4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D60BA8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60BAC: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60BB0: 55570000  rlwinm r23, r10, 0, 0, 0
	ctx.r[23].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D60BB4: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D60BB8: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 82D60BBC: 4BFF468D  bl 0x82d55248
	ctx.lr = 0x82D60BC0;
	sub_82D55248(ctx, base);
	// 82D60BC0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D60BC4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D60BC8: 40990028  ble cr6, 0x82d60bf0
	if !ctx.cr[6].gt {
	pc = 0x82D60BF0; continue 'dispatch;
	}
	// 82D60BCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D60BD0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D60BD4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82D60BD8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60BDC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D60BE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D60BE4: 7D2A412A  stdx r9, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 82D60BE8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82D60BEC: 409AFFEC  bne cr6, 0x82d60bd8
	if !ctx.cr[6].eq {
	pc = 0x82D60BD8; continue 'dispatch;
	}
	// 82D60BF0: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82D60BF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D60BF8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82D60BFC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D60C00: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D60C04: 40990040  ble cr6, 0x82d60c44
	if !ctx.cr[6].gt {
	pc = 0x82D60C44; continue 'dispatch;
	}
	// 82D60C08: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60C0C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82D60C10: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82D60C14: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82D60C18: E89D0000  ld r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 82D60C1C: 2F24FFFF  cmpdi cr6, r4, -1
	ctx.cr[6].compare_i64(ctx.r[4].s64, -1, &mut ctx.xer);
	// 82D60C20: 419A0010  beq cr6, 0x82d60c30
	if ctx.cr[6].eq {
	pc = 0x82D60C30; continue 'dispatch;
	}
	// 82D60C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D60C28: E8BE0000  ld r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82D60C2C: 4BFFFCC5  bl 0x82d608f0
	ctx.lr = 0x82D60C30;
	sub_82D608F0(ctx, base);
	// 82D60C30: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82D60C34: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82D60C38: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82D60C3C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82D60C40: 409AFFD8  bne cr6, 0x82d60c18
	if !ctx.cr[6].eq {
	pc = 0x82D60C18; continue 'dispatch;
	}
	// 82D60C44: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82D60C48: 409A0018  bne cr6, 0x82d60c60
	if !ctx.cr[6].eq {
	pc = 0x82D60C60; continue 'dispatch;
	}
	// 82D60C4C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D60C50: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D60C54: 57652036  slwi r5, r27, 4
	ctx.r[5].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D60C58: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82D60C5C: 4BFF466D  bl 0x82d552c8
	ctx.lr = 0x82D60C60;
	sub_82D552C8(ctx, base);
	// 82D60C60: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D60C64: 4BF487E0  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60C68 size=116
    let mut pc: u32 = 0x82D60C68;
    'dispatch: loop {
        match pc {
            0x82D60C68 => {
    //   block [0x82D60C68..0x82D60CDC)
	// 82D60C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D60C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D60C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60C7C: 4BFFF9B5  bl 0x82d60630
	ctx.lr = 0x82D60C80;
	sub_82D60630(ctx, base);
	// 82D60C80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60C84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D60C88: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D60C8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D60C90: 40990008  ble cr6, 0x82d60c98
	if !ctx.cr[6].gt {
	pc = 0x82D60C98; continue 'dispatch;
	}
	// 82D60C94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D60C98: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D60C9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D60CA0: 419A0024  beq cr6, 0x82d60cc4
	if ctx.cr[6].eq {
	pc = 0x82D60CC4; continue 'dispatch;
	}
	// 82D60CA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D60CA8: 4BFFFAD1  bl 0x82d60778
	ctx.lr = 0x82D60CAC;
	sub_82D60778(ctx, base);
	// 82D60CAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D60CB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D60CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60CBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D60CC0: 4E800020  blr
	return;
	// 82D60CC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D60CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D60CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60CD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D60CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60CE0 size=104
    let mut pc: u32 = 0x82D60CE0;
    'dispatch: loop {
        match pc {
            0x82D60CE0 => {
    //   block [0x82D60CE0..0x82D60D48)
	// 82D60CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60CE4: 4BF48729  bl 0x82ca940c
	ctx.lr = 0x82D60CE8;
	sub_82CA93D0(ctx, base);
	// 82D60CE8: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60CEC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D60CF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D60CF4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82D60CF8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D60CFC: 419A0040  beq cr6, 0x82d60d3c
	if ctx.cr[6].eq {
	pc = 0x82D60D3C; continue 'dispatch;
	}
	// 82D60D00: 2F1F0200  cmpwi cr6, r31, 0x200
	ctx.cr[6].compare_i32(ctx.r[31].s32, 512, &mut ctx.xer);
	// 82D60D04: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D60D08: 41990008  bgt cr6, 0x82d60d10
	if ctx.cr[6].gt {
	pc = 0x82D60D10; continue 'dispatch;
	}
	// 82D60D0C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D60D10: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60D14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D60D18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D60D1C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D60D20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60D24: 4E800421  bctrl
	ctx.lr = 0x82D60D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60D28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D60D2C: 419A0010  beq cr6, 0x82d60d3c
	if ctx.cr[6].eq {
	pc = 0x82D60D3C; continue 'dispatch;
	}
	// 82D60D30: 7FE3F850  subf r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82D60D34: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D60D38: 409AFFC8  bne cr6, 0x82d60d00
	if !ctx.cr[6].eq {
	pc = 0x82D60D00; continue 'dispatch;
	}
	// 82D60D3C: 7C7FE850  subf r3, r31, r29
	ctx.r[3].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82D60D40: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82D60D44: 4BF48718  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60D48 size=88
    let mut pc: u32 = 0x82D60D48;
    'dispatch: loop {
        match pc {
            0x82D60D48 => {
    //   block [0x82D60D48..0x82D60DA0)
	// 82D60D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D60D50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60D54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60D58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D60D5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D60D60: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D60D64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60D68: 4E800421  bctrl
	ctx.lr = 0x82D60D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60D6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D60D70: 419A001C  beq cr6, 0x82d60d8c
	if ctx.cr[6].eq {
	pc = 0x82D60D8C; continue 'dispatch;
	}
	// 82D60D74: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D60D78: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82D60D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D60D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60D88: 4E800020  blr
	return;
	// 82D60D8C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D60D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D60D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D60D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D60D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DA0 size=8
    let mut pc: u32 = 0x82D60DA0;
    'dispatch: loop {
        match pc {
            0x82D60DA0 => {
    //   block [0x82D60DA0..0x82D60DA8)
	// 82D60DA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D60DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DA8 size=8
    let mut pc: u32 = 0x82D60DA8;
    'dispatch: loop {
        match pc {
            0x82D60DA8 => {
    //   block [0x82D60DA8..0x82D60DB0)
	// 82D60DA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D60DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DB0 size=8
    let mut pc: u32 = 0x82D60DB0;
    'dispatch: loop {
        match pc {
            0x82D60DB0 => {
    //   block [0x82D60DB0..0x82D60DB8)
	// 82D60DB0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D60DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DB8 size=8
    let mut pc: u32 = 0x82D60DB8;
    'dispatch: loop {
        match pc {
            0x82D60DB8 => {
    //   block [0x82D60DB8..0x82D60DC0)
	// 82D60DB8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D60DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DC0 size=12
    let mut pc: u32 = 0x82D60DC0;
    'dispatch: loop {
        match pc {
            0x82D60DC0 => {
    //   block [0x82D60DC0..0x82D60DCC)
	// 82D60DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D60DC4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D60DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DD0 size=12
    let mut pc: u32 = 0x82D60DD0;
    'dispatch: loop {
        match pc {
            0x82D60DD0 => {
    //   block [0x82D60DD0..0x82D60DDC)
	// 82D60DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D60DD4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D60DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DE0 size=20
    let mut pc: u32 = 0x82D60DE0;
    'dispatch: loop {
        match pc {
            0x82D60DE0 => {
    //   block [0x82D60DE0..0x82D60DF4)
	// 82D60DE0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D60DE4: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82D60DE8: 386B5004  addi r3, r11, 0x5004
	ctx.r[3].s64 = ctx.r[11].s64 + 20484;
	// 82D60DEC: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82D60DF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60DF8 size=8
    let mut pc: u32 = 0x82D60DF8;
    'dispatch: loop {
        match pc {
            0x82D60DF8 => {
    //   block [0x82D60DF8..0x82D60E00)
	// 82D60DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D60DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60E00 size=24
    let mut pc: u32 = 0x82D60E00;
    'dispatch: loop {
        match pc {
            0x82D60E00 => {
    //   block [0x82D60E00..0x82D60E18)
	// 82D60E00: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D60E04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D60E08: 396B5024  addi r11, r11, 0x5024
	ctx.r[11].s64 = ctx.r[11].s64 + 20516;
	// 82D60E0C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D60E10: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D60E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D60E18 size=16
    let mut pc: u32 = 0x82D60E18;
    'dispatch: loop {
        match pc {
            0x82D60E18 => {
    //   block [0x82D60E18..0x82D60E28)
	// 82D60E18: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D60E1C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D60E20: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D60E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D60E28 size=208
    let mut pc: u32 = 0x82D60E28;
    'dispatch: loop {
        match pc {
            0x82D60E28 => {
    //   block [0x82D60E28..0x82D60EF8)
	// 82D60E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60E2C: 4BF485DD  bl 0x82ca9408
	ctx.lr = 0x82D60E30;
	sub_82CA93D0(ctx, base);
	// 82D60E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60E34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60E38: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D60E3C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D60E40: 40980018  bge cr6, 0x82d60e58
	if !ctx.cr[6].lt {
	pc = 0x82D60E58; continue 'dispatch;
	}
	// 82D60E44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D60E48: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D60E4C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D60E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D60E54: 4BF48604  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D60E58: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D60E5C: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D60E60: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D60E64: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D60E68: 40990024  ble cr6, 0x82d60e8c
	if !ctx.cr[6].gt {
	pc = 0x82D60E8C; continue 'dispatch;
	}
	// 82D60E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D60E70: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82D60E74: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D60E78: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82D60E7C: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D60E80: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D60E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D60E88: 4BF485D0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D60E8C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D60E90: 40990060  ble cr6, 0x82d60ef0
	if !ctx.cr[6].gt {
	pc = 0x82D60EF0; continue 'dispatch;
	}
	// 82D60E94: 7FCB4E70  srawi r11, r30, 9
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 9) as i64;
	// 82D60E98: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82D60E9C: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60EA0: 7F8BF050  subf r28, r11, r30
	ctx.r[28].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82D60EA4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82D60EA8: 23BC0200  subfic r29, r28, 0x200
	ctx.xer.ca = ctx.r[28].u32 <= 512 as u32;
	ctx.r[29].s64 = (512 as i64) - ctx.r[28].s64;
	// 82D60EAC: 409A0008  bne cr6, 0x82d60eb4
	if !ctx.cr[6].eq {
	pc = 0x82D60EB4; continue 'dispatch;
	}
	// 82D60EB0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D60EB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D60EB8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D60EBC: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D60EC0: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D60EC4: 4BFF7E75  bl 0x82d58d38
	ctx.lr = 0x82D60EC8;
	sub_82D58D38(ctx, base);
	// 82D60EC8: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 82D60ECC: 7FCA4E70  srawi r10, r30, 9
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 9) as i64;
	// 82D60ED0: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82D60ED4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D60ED8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82D60EDC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D60EE0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D60EE4: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D60EE8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D60EEC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D60EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D60EF4: 4BF48564  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60EF8 size=192
    let mut pc: u32 = 0x82D60EF8;
    'dispatch: loop {
        match pc {
            0x82D60EF8 => {
    //   block [0x82D60EF8..0x82D60FB8)
	// 82D60EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60EFC: 4BF48511  bl 0x82ca940c
	ctx.lr = 0x82D60F00;
	sub_82CA93D0(ctx, base);
	// 82D60F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60F08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D60F0C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60F10: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60F14: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D60F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60F1C: 4E800421  bctrl
	ctx.lr = 0x82D60F20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60F20: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60F24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D60F28: 409A0010  bne cr6, 0x82d60f38
	if !ctx.cr[6].eq {
	pc = 0x82D60F38; continue 'dispatch;
	}
	// 82D60F2C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D60F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D60F34: 4BF48528  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D60F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D60F3C: 4BFFFEED  bl 0x82d60e28
	ctx.lr = 0x82D60F40;
	sub_82D60E28(ctx, base);
	// 82D60F40: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D60F44: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D60F48: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D60F4C: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D60F50: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D60F54: 40990048  ble cr6, 0x82d60f9c
	if !ctx.cr[6].gt {
	pc = 0x82D60F9C; continue 'dispatch;
	}
	// 82D60F58: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D60F5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D60F60: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D60F64: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D60F68: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D60F6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D60F70: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D60F74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D60F78: 4E800421  bctrl
	ctx.lr = 0x82D60F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D60F7C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D60F80: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82D60F84: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82D60F88: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D60F8C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D60F90: 409A0018  bne cr6, 0x82d60fa8
	if !ctx.cr[6].eq {
	pc = 0x82D60FA8; continue 'dispatch;
	}
	// 82D60F94: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D60F98: 4198FFC0  blt cr6, 0x82d60f58
	if ctx.cr[6].lt {
	pc = 0x82D60F58; continue 'dispatch;
	}
	// 82D60F9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D60FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D60FA4: 4BF484B8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D60FA8: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82D60FAC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D60FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D60FB4: 4BF484A8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D60FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D60FB8 size=196
    let mut pc: u32 = 0x82D60FB8;
    'dispatch: loop {
        match pc {
            0x82D60FB8 => {
    //   block [0x82D60FB8..0x82D6107C)
	// 82D60FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D60FBC: 4BF48449  bl 0x82ca9404
	ctx.lr = 0x82D60FC0;
	sub_82CA93D0(ctx, base);
	// 82D60FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D60FC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D60FC8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D60FCC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D60FD0: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82D60FD4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D60FD8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D60FDC: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D60FE0: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D60FE4: 4099005C  ble cr6, 0x82d61040
	if !ctx.cr[6].gt {
	pc = 0x82D61040; continue 'dispatch;
	}
	// 82D60FE8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D60FEC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D60FF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D60FF4: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D60FF8: 4BFF7D39  bl 0x82d58d30
	ctx.lr = 0x82D60FFC;
	sub_82D58D30(ctx, base);
	// 82D60FFC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61000: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61004: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82D61008: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D6100C: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82D61010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61014: 814A0030  lwz r10, 0x30(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D61018: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D6101C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82D61020: 4E800421  bctrl
	ctx.lr = 0x82D61024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61024: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D61028: 409A0048  bne cr6, 0x82d61070
	if !ctx.cr[6].eq {
	pc = 0x82D61070; continue 'dispatch;
	}
	// 82D6102C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61030: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D61034: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D61038: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D6103C: 4199FFAC  bgt cr6, 0x82d60fe8
	if ctx.cr[6].gt {
	pc = 0x82D60FE8; continue 'dispatch;
	}
	// 82D61040: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D61044: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D61048: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6104C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D61050: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D61054: 4BFF7CDD  bl 0x82d58d30
	ctx.lr = 0x82D61058;
	sub_82D58D30(ctx, base);
	// 82D61058: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6105C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82D61060: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D61064: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D61068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D6106C: 4BF483E8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D61070: 7C7DD850  subf r3, r29, r27
	ctx.r[3].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 82D61074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D61078: 4BF483DC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61080 size=132
    let mut pc: u32 = 0x82D61080;
    'dispatch: loop {
        match pc {
            0x82D61080 => {
    //   block [0x82D61080..0x82D61104)
	// 82D61080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61084: 4BF48389  bl 0x82ca940c
	ctx.lr = 0x82D61088;
	sub_82CA93D0(ctx, base);
	// 82D61088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6108C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61090: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D61094: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82D61098: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6109C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D610A0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D610A4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D610A8: 40990038  ble cr6, 0x82d610e0
	if !ctx.cr[6].gt {
	pc = 0x82D610E0; continue 'dispatch;
	}
	// 82D610AC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D610B0: 7FCBF050  subf r30, r11, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82D610B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D610B8: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D610BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D610C0: 4E800421  bctrl
	ctx.lr = 0x82D610C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D610C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D610C8: 409A0030  bne cr6, 0x82d610f8
	if !ctx.cr[6].eq {
	pc = 0x82D610F8; continue 'dispatch;
	}
	// 82D610CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D610D0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D610D4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D610D8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D610DC: 4199FFD0  bgt cr6, 0x82d610ac
	if ctx.cr[6].gt {
	pc = 0x82D610AC; continue 'dispatch;
	}
	// 82D610E0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D610E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D610E8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D610EC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D610F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D610F4: 4BF48368  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D610F8: 7C7EE850  subf r3, r30, r29
	ctx.r[3].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82D610FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D61100: 4BF4835C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61108 size=108
    let mut pc: u32 = 0x82D61108;
    'dispatch: loop {
        match pc {
            0x82D61108 => {
    //   block [0x82D61108..0x82D61174)
	// 82D61108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61114: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61118: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6111C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61120: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D61124: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D61128: 409A002C  bne cr6, 0x82d61154
	if !ctx.cr[6].eq {
	pc = 0x82D61154; continue 'dispatch;
	}
	// 82D6112C: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D61134: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61138: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6113C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61140: 4E800421  bctrl
	ctx.lr = 0x82D61144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61144: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61148: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D6114C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D61150: 419A0008  beq cr6, 0x82d61158
	if ctx.cr[6].eq {
	pc = 0x82D61158; continue 'dispatch;
	}
	// 82D61154: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D61158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6115C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D61160: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D61164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6116C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D61178 size=24
    let mut pc: u32 = 0x82D61178;
    'dispatch: loop {
        match pc {
            0x82D61178 => {
    //   block [0x82D61178..0x82D61190)
	// 82D61178: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6117C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D61180: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D61184: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D61188: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D6118C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D61190 size=28
    let mut pc: u32 = 0x82D61190;
    'dispatch: loop {
        match pc {
            0x82D61190 => {
    //   block [0x82D61190..0x82D611AC)
	// 82D61190: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61194: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D61198: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82D6119C: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D611A0: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82D611A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D611A8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D611AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D611AC size=8
    let mut pc: u32 = 0x82D611AC;
    'dispatch: loop {
        match pc {
            0x82D611AC => {
    //   block [0x82D611AC..0x82D611B4)
	// 82D611AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D611B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D611B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D611B8 size=28
    let mut pc: u32 = 0x82D611B8;
    'dispatch: loop {
        match pc {
            0x82D611B8 => {
    //   block [0x82D611B8..0x82D611D4)
	// 82D611B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D611BC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D611C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D611C4: 41980010  blt cr6, 0x82d611d4
	if ctx.cr[6].lt {
		sub_82D611D4(ctx, base);
		return;
	}
	// 82D611C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D611CC: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82D611D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D611D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D611D4 size=8
    let mut pc: u32 = 0x82D611D4;
    'dispatch: loop {
        match pc {
            0x82D611D4 => {
    //   block [0x82D611D4..0x82D611DC)
	// 82D611D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D611D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D611E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D611E0 size=64
    let mut pc: u32 = 0x82D611E0;
    'dispatch: loop {
        match pc {
            0x82D611E0 => {
    //   block [0x82D611E0..0x82D61220)
	// 82D611E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D611E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D611E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D611EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D611F0: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D611F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D611F8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D611FC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D61200: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61204: 4E800421  bctrl
	ctx.lr = 0x82D61208;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6120C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61218: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D61220 size=48
    let mut pc: u32 = 0x82D61220;
    'dispatch: loop {
        match pc {
            0x82D61220 => {
    //   block [0x82D61220..0x82D61250)
	// 82D61220: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D61224: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82D61228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D6122C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61230: 912B001C  stw r9, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82D61234: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82D61238: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82D6123C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82D61240: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61244: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D61248: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6124C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61250 size=108
    let mut pc: u32 = 0x82D61250;
    'dispatch: loop {
        match pc {
            0x82D61250 => {
    //   block [0x82D61250..0x82D612BC)
	// 82D61250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6125C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61264: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61268: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6126C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D61270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61274: 4E800421  bctrl
	ctx.lr = 0x82D61278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61278: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D6127C: 41980028  blt cr6, 0x82d612a4
	if ctx.cr[6].lt {
	pc = 0x82D612A4; continue 'dispatch;
	}
	// 82D61280: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61284: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D61288: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D6128C: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82D61290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6129C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D612A0: 4E800020  blr
	return;
	// 82D612A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D612A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D612AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D612B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D612B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D612B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D612C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D612C0 size=128
    let mut pc: u32 = 0x82D612C0;
    'dispatch: loop {
        match pc {
            0x82D612C0 => {
    //   block [0x82D612C0..0x82D61340)
	// 82D612C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D612C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D612C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D612CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D612D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D612D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D612D8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D612DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D612E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D612E4: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D612E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D612EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D612F0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82D612F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D612F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D612FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61300: 4E800421  bctrl
	ctx.lr = 0x82D61304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61304: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D61308: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82D6130C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D61310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61314: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82D61318: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D6131C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D61320: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D61324: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D61328: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D6132C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61334: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D61338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D61340 size=32
    let mut pc: u32 = 0x82D61340;
    'dispatch: loop {
        match pc {
            0x82D61340 => {
    //   block [0x82D61340..0x82D61360)
	// 82D61340: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61344: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D61348: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6134C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D61350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61354: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D61358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6135C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61360 size=112
    let mut pc: u32 = 0x82D61360;
    'dispatch: loop {
        match pc {
            0x82D61360 => {
    //   block [0x82D61360..0x82D613D0)
	// 82D61360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6136C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D61374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61378: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D6137C: 394A5034  addi r10, r10, 0x5034
	ctx.r[10].s64 = ctx.r[10].s64 + 20532;
	// 82D61380: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D61384: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D61388: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82D6138C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D61390: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D61394: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D61398: 4BFFFF29  bl 0x82d612c0
	ctx.lr = 0x82D6139C;
	sub_82D612C0(ctx, base);
	// 82D6139C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D613A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D613A4: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D613A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D613AC: 419A0010  beq cr6, 0x82d613bc
	if ctx.cr[6].eq {
	pc = 0x82D613BC; continue 'dispatch;
	}
	// 82D613B0: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D613B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D613B8: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D613BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D613C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D613C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D613C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D613CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D613D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D613D0 size=156
    let mut pc: u32 = 0x82D613D0;
    'dispatch: loop {
        match pc {
            0x82D613D0 => {
    //   block [0x82D613D0..0x82D6146C)
	// 82D613D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D613D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D613D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D613DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D613E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D613E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D613E8: 396B5034  addi r11, r11, 0x5034
	ctx.r[11].s64 = ctx.r[11].s64 + 20532;
	// 82D613EC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D613F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D613F4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D613F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D613FC: 419A0030  beq cr6, 0x82d6142c
	if ctx.cr[6].eq {
	pc = 0x82D6142C; continue 'dispatch;
	}
	// 82D61400: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D61404: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D61408: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D6140C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D61410: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D61414: 409A0018  bne cr6, 0x82d6142c
	if !ctx.cr[6].eq {
	pc = 0x82D6142C; continue 'dispatch;
	}
	// 82D61418: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6141C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D61420: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61424: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61428: 4E800421  bctrl
	ctx.lr = 0x82D6142C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6142C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61430: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D61434: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D61438: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D6143C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61440: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D61444: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61448: 4E800421  bctrl
	ctx.lr = 0x82D6144C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6144C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D61450: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D61454: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D61458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D6145C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61470 size=100
    let mut pc: u32 = 0x82D61470;
    'dispatch: loop {
        match pc {
            0x82D61470 => {
    //   block [0x82D61470..0x82D614D4)
	// 82D61470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61478: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6147C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61488: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D6148C: 4BFFFF45  bl 0x82d613d0
	ctx.lr = 0x82D61490;
	sub_82D613D0(ctx, base);
	// 82D61490: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D61494: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D61498: 419A0020  beq cr6, 0x82d614b8
	if ctx.cr[6].eq {
	pc = 0x82D614B8; continue 'dispatch;
	}
	// 82D6149C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D614A0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D614A4: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D614A8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D614AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D614B0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D614B4: 4BFF3E15  bl 0x82d552c8
	ctx.lr = 0x82D614B8;
	sub_82D552C8(ctx, base);
	// 82D614B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D614BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D614C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D614C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D614C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D614CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D614D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D614D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D614D8 size=108
    let mut pc: u32 = 0x82D614D8;
    'dispatch: loop {
        match pc {
            0x82D614D8 => {
    //   block [0x82D614D8..0x82D61544)
	// 82D614D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D614DC: 4BF47F2D  bl 0x82ca9408
	ctx.lr = 0x82D614E0;
	sub_82CA93D0(ctx, base);
	// 82D614E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D614E4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D614E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D614EC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D614F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D614F4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D614F8: 40990034  ble cr6, 0x82d6152c
	if !ctx.cr[6].gt {
	pc = 0x82D6152C; continue 'dispatch;
	}
	// 82D614FC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61500: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82D61504: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 82D61508: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6150C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D61510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61514: 4E800421  bctrl
	ctx.lr = 0x82D61518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61518: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82D6151C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D61520: 419A0018  beq cr6, 0x82d61538
	if ctx.cr[6].eq {
	pc = 0x82D61538; continue 'dispatch;
	}
	// 82D61524: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D61528: 4198FFD4  blt cr6, 0x82d614fc
	if ctx.cr[6].lt {
	pc = 0x82D614FC; continue 'dispatch;
	}
	// 82D6152C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D61530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D61534: 4BF47F24  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D61538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6153C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D61540: 4BF47F18  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61548 size=64
    let mut pc: u32 = 0x82D61548;
    'dispatch: loop {
        match pc {
            0x82D61548 => {
    //   block [0x82D61548..0x82D61588)
	// 82D61548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6154C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61558: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6155C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61560: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61564: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D61568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6156C: 4E800421  bctrl
	ctx.lr = 0x82D61570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6157C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61588 size=108
    let mut pc: u32 = 0x82D61588;
    'dispatch: loop {
        match pc {
            0x82D61588 => {
    //   block [0x82D61588..0x82D615F4)
	// 82D61588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6158C: 4BF47E7D  bl 0x82ca9408
	ctx.lr = 0x82D61590;
	sub_82CA93D0(ctx, base);
	// 82D61590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61594: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D61598: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D6159C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D615A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D615A4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D615A8: 40990034  ble cr6, 0x82d615dc
	if !ctx.cr[6].gt {
	pc = 0x82D615DC; continue 'dispatch;
	}
	// 82D615AC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D615B0: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82D615B4: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 82D615B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D615BC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D615C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D615C4: 4E800421  bctrl
	ctx.lr = 0x82D615C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D615C8: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82D615CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D615D0: 419A0018  beq cr6, 0x82d615e8
	if ctx.cr[6].eq {
	pc = 0x82D615E8; continue 'dispatch;
	}
	// 82D615D4: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D615D8: 4198FFD4  blt cr6, 0x82d615ac
	if ctx.cr[6].lt {
	pc = 0x82D615AC; continue 'dispatch;
	}
	// 82D615DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D615E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D615E4: 4BF47E74  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D615E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D615EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D615F0: 4BF47E68  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D615F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D615F8 size=64
    let mut pc: u32 = 0x82D615F8;
    'dispatch: loop {
        match pc {
            0x82D615F8 => {
    //   block [0x82D615F8..0x82D61638)
	// 82D615F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D615FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61608: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6160C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61610: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61614: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D61618: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6161C: 4E800421  bctrl
	ctx.lr = 0x82D61620;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61624: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6162C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61630: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61638 size=68
    let mut pc: u32 = 0x82D61638;
    'dispatch: loop {
        match pc {
            0x82D61638 => {
    //   block [0x82D61638..0x82D6167C)
	// 82D61638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61644: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61648: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6164C: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82D61650: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82D61654: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D61658: 4BFF3BF1  bl 0x82d55248
	ctx.lr = 0x82D6165C;
	sub_82D55248(ctx, base);
	// 82D6165C: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82D61660: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82D61664: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D61668: 480014C1  bl 0x82d62b28
	ctx.lr = 0x82D6166C;
	sub_82D62B28(ctx, base);
	// 82D6166C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61680 size=148
    let mut pc: u32 = 0x82D61680;
    'dispatch: loop {
        match pc {
            0x82D61680 => {
    //   block [0x82D61680..0x82D61714)
	// 82D61680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61684: 4BF47D89  bl 0x82ca940c
	ctx.lr = 0x82D61688;
	sub_82CA93D0(ctx, base);
	// 82D61688: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6168C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D61690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D61694: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D61698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6169C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82D616A0: 396B50C8  addi r11, r11, 0x50c8
	ctx.r[11].s64 = ctx.r[11].s64 + 20680;
	// 82D616A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D616A8: 394A506C  addi r10, r10, 0x506c
	ctx.r[10].s64 = ctx.r[10].s64 + 20588;
	// 82D616AC: 392950A0  addi r9, r9, 0x50a0
	ctx.r[9].s64 = ctx.r[9].s64 + 20640;
	// 82D616B0: 3FA08333  lis r29, -0x7ccd
	ctx.r[29].s64 = -2093809664;
	// 82D616B4: B3DF0006  sth r30, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[30].u16 ) };
	// 82D616B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D616BC: B3DF000E  sth r30, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 82D616C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D616C4: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82D616C8: B3DF001A  sth r30, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[30].u16 ) };
	// 82D616CC: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82D616D0: 911F001C  stw r8, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82D616D4: 93FF0010  stw r31, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82D616D8: 93FF001C  stw r31, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82D616DC: 897D7764  lbz r11, 0x7764(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(30564 as u32) ) } as u64;
	// 82D616E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D616E4: 409A0024  bne cr6, 0x82d61708
	if !ctx.cr[6].eq {
	pc = 0x82D61708; continue 'dispatch;
	}
	// 82D616E8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D616EC: 814B7648  lwz r10, 0x7648(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30280 as u32) ) } as u64;
	// 82D616F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D616F4: 419A0014  beq cr6, 0x82d61708
	if ctx.cr[6].eq {
	pc = 0x82D61708; continue 'dispatch;
	}
	// 82D616F8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D616FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61700: 4E800421  bctrl
	ctx.lr = 0x82D61704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61704: 9BDD7764  stb r30, 0x7764(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(30564 as u32), ctx.r[30].u8 ) };
	// 82D61708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6170C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D61710: 4BF47D4C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61718 size=96
    let mut pc: u32 = 0x82D61718;
    'dispatch: loop {
        match pc {
            0x82D61718 => {
    //   block [0x82D61718..0x82D61778)
	// 82D61718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61728: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6172C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61730: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D61734: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82D61738: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6173C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D61740: 419A0020  beq cr6, 0x82d61760
	if ctx.cr[6].eq {
	pc = 0x82D61760; continue 'dispatch;
	}
	// 82D61744: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61748: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6174C: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D61750: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D61754: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D61758: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D6175C: 4BFF3B6D  bl 0x82d552c8
	ctx.lr = 0x82D61760;
	sub_82D552C8(ctx, base);
	// 82D61760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6176C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61778 size=104
    let mut pc: u32 = 0x82D61778;
    'dispatch: loop {
        match pc {
            0x82D61778 => {
    //   block [0x82D61778..0x82D617E0)
	// 82D61778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6177C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61788: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6178C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61790: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D61794: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82D61798: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6179C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D617A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D617A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D617A8: 419A0020  beq cr6, 0x82d617c8
	if ctx.cr[6].eq {
	pc = 0x82D617C8; continue 'dispatch;
	}
	// 82D617AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D617B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D617B4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82D617B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D617BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D617C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D617C4: 4BFF3B05  bl 0x82d552c8
	ctx.lr = 0x82D617C8;
	sub_82D552C8(ctx, base);
	// 82D617C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D617CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D617D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D617D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D617D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D617DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D617E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D617E0 size=1156
    let mut pc: u32 = 0x82D617E0;
    'dispatch: loop {
        match pc {
            0x82D617E0 => {
    //   block [0x82D617E0..0x82D61C64)
	// 82D617E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D617E4: 4BF47C29  bl 0x82ca940c
	ctx.lr = 0x82D617E8;
	sub_82CA93D0(ctx, base);
	// 82D617E8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82D617EC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82D617F0: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D617F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D617F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D617FC: 386101B0  addi r3, r1, 0x1b0
	ctx.r[3].s64 = ctx.r[1].s64 + 432;
	// 82D61800: 4BFF5931  bl 0x82d57130
	ctx.lr = 0x82D61804;
	sub_82D57130(ctx, base);
	// 82D61804: 396101E0  addi r11, r1, 0x1e0
	ctx.r[11].s64 = ctx.r[1].s64 + 480;
	// 82D61808: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D61C68 size=4
    let mut pc: u32 = 0x82D61C68;
    'dispatch: loop {
        match pc {
            0x82D61C68 => {
    //   block [0x82D61C68..0x82D61C6C)
	// 82D61C68: 4BFFFB78  b 0x82d617e0
	sub_82D617E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61C70 size=104
    let mut pc: u32 = 0x82D61C70;
    'dispatch: loop {
        match pc {
            0x82D61C70 => {
    //   block [0x82D61C70..0x82D61CD8)
	// 82D61C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61C78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61C7C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D61C80: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82D61C84: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82D61C88: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82D61C8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61CD8 size=108
    let mut pc: u32 = 0x82D61CD8;
    'dispatch: loop {
        match pc {
            0x82D61CD8 => {
    //   block [0x82D61CD8..0x82D61D44)
	// 82D61CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61CDC: 4BF47731  bl 0x82ca940c
	ctx.lr = 0x82D61CE0;
	sub_82CA93D0(ctx, base);
	// 82D61CE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61CE4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61CE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D61CEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D61CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D61CF4: 419A0048  beq cr6, 0x82d61d3c
	if ctx.cr[6].eq {
	pc = 0x82D61D3C; continue 'dispatch;
	}
	// 82D61CF8: 7FA42850  subf r29, r4, r5
	ctx.r[29].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 82D61CFC: 7CBDF82E  lwzx r5, r29, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82D61D00: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D61D04: 419A0038  beq cr6, 0x82d61d3c
	if ctx.cr[6].eq {
	pc = 0x82D61D3C; continue 'dispatch;
	}
	// 82D61D08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61D0C: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D61D10: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D61D14: 419A0018  beq cr6, 0x82d61d2c
	if ctx.cr[6].eq {
	pc = 0x82D61D2C; continue 'dispatch;
	}
	// 82D61D18: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D61D20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D61D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D61D28: 4E800421  bctrl
	ctx.lr = 0x82D61D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D61D2C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D61D30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61D34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D61D38: 409AFFC4  bne cr6, 0x82d61cfc
	if !ctx.cr[6].eq {
	pc = 0x82D61CFC; continue 'dispatch;
	}
	// 82D61D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D61D40: 4BF4771C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61D48 size=216
    let mut pc: u32 = 0x82D61D48;
    'dispatch: loop {
        match pc {
            0x82D61D48 => {
    //   block [0x82D61D48..0x82D61E20)
	// 82D61D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61D4C: 4BF476C1  bl 0x82ca940c
	ctx.lr = 0x82D61D50;
	sub_82CA93D0(ctx, base);
	// 82D61D50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61D54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D61D58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D61D5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D61D60: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61D64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D61D68: 41980024  blt cr6, 0x82d61d8c
	if ctx.cr[6].lt {
	pc = 0x82D61D8C; continue 'dispatch;
	}
	// 82D61D6C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61D70: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61D74: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D61D78: 409A0014  bne cr6, 0x82d61d8c
	if !ctx.cr[6].eq {
	pc = 0x82D61D8C; continue 'dispatch;
	}
	// 82D61D7C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82D61D80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D61D84: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D61D88: 4099FFE8  ble cr6, 0x82d61d70
	if !ctx.cr[6].gt {
	pc = 0x82D61D70; continue 'dispatch;
	}
	// 82D61D8C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61D90: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D61D94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D61D98: 40990008  ble cr6, 0x82d61da0
	if !ctx.cr[6].gt {
	pc = 0x82D61DA0; continue 'dispatch;
	}
	// 82D61D9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D61DA0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D61DA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D61DA8: 419A0070  beq cr6, 0x82d61e18
	if ctx.cr[6].eq {
	pc = 0x82D61E18; continue 'dispatch;
	}
	// 82D61DAC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61DB0: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D61DB4: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 82D61DB8: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D61DBC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61DC0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61DC4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82D61DC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D61DCC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D61DD0: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D61DD4: 4B503E2D  bl 0x82265c00
	ctx.lr = 0x82D61DD8;
	sub_82265C00(ctx, base);
	// 82D61DD8: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D61DDC: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82D61DE0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D61DE4: 4199002C  bgt cr6, 0x82d61e10
	if ctx.cr[6].gt {
	pc = 0x82D61E10; continue 'dispatch;
	}
	// 82D61DE8: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D61DEC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D61DF0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D61DF4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61DF8: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82D61DFC: 409A0014  bne cr6, 0x82d61e10
	if !ctx.cr[6].eq {
	pc = 0x82D61E10; continue 'dispatch;
	}
	// 82D61E00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D61E04: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D61E08: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D61E0C: 4099FFE8  ble cr6, 0x82d61df4
	if !ctx.cr[6].gt {
	pc = 0x82D61DF4; continue 'dispatch;
	}
	// 82D61E10: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D61E14: 4BFFFF78  b 0x82d61d8c
	pc = 0x82D61D8C; continue 'dispatch;
	// 82D61E18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D61E1C: 4BF47640  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61E20 size=104
    let mut pc: u32 = 0x82D61E20;
    'dispatch: loop {
        match pc {
            0x82D61E20 => {
    //   block [0x82D61E20..0x82D61E88)
	// 82D61E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61E30: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61E34: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D61E38: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82D61E3C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82D61E40: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D61E44: 4BFF3405  bl 0x82d55248
	ctx.lr = 0x82D61E48;
	sub_82D55248(ctx, base);
	// 82D61E48: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D61E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61E50: 396B50F8  addi r11, r11, 0x50f8
	ctx.r[11].s64 = ctx.r[11].s64 + 20728;
	// 82D61E54: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82D61E58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D61E5C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D61E60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D61E64: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D61E68: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D61E6C: 4B5050D5  bl 0x82266f40
	ctx.lr = 0x82D61E70;
	sub_82266F40(ctx, base);
	// 82D61E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D61E88 size=8
    let mut pc: u32 = 0x82D61E88;
    'dispatch: loop {
        match pc {
            0x82D61E88 => {
    //   block [0x82D61E88..0x82D61E90)
	// 82D61E88: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82D61E8C: 4B503D74  b 0x82265c00
	sub_82265C00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D61E90 size=16
    let mut pc: u32 = 0x82D61E90;
    'dispatch: loop {
        match pc {
            0x82D61E90 => {
    //   block [0x82D61E90..0x82D61EA0)
	// 82D61E90: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D61E98: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82D61E9C: 4BFFE7EC  b 0x82d60688
	sub_82D60688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61EA0 size=116
    let mut pc: u32 = 0x82D61EA0;
    'dispatch: loop {
        match pc {
            0x82D61EA0 => {
    //   block [0x82D61EA0..0x82D61F14)
	// 82D61EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61EA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D61EAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61EB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D61EBC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D61EC0: 4B505049  bl 0x82266f08
	ctx.lr = 0x82D61EC4;
	sub_82266F08(ctx, base);
	// 82D61EC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D61EC8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D61ECC: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D61ED0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D61ED4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D61ED8: 419A0020  beq cr6, 0x82d61ef8
	if ctx.cr[6].eq {
	pc = 0x82D61EF8; continue 'dispatch;
	}
	// 82D61EDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61EE0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D61EE4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82D61EE8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D61EEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D61EF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D61EF4: 4BFF33D5  bl 0x82d552c8
	ctx.lr = 0x82D61EF8;
	sub_82D552C8(ctx, base);
	// 82D61EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61EFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D61F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61F08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D61F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61F18 size=144
    let mut pc: u32 = 0x82D61F18;
    'dispatch: loop {
        match pc {
            0x82D61F18 => {
    //   block [0x82D61F18..0x82D61FA8)
	// 82D61F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61F1C: 4BF474E5  bl 0x82ca9400
	ctx.lr = 0x82D61F20;
	sub_82CA93D0(ctx, base);
	// 82D61F20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61F24: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D61F28: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D61F2C: 3BCB7724  addi r30, r11, 0x7724
	ctx.r[30].s64 = ctx.r[11].s64 + 30500;
	// 82D61F30: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D61F34: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82D61F38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D61F3C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82D61F40: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82D61F44: 4BFF380D  bl 0x82d55750
	ctx.lr = 0x82D61F48;
	sub_82D55750(ctx, base);
	// 82D61F48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D61F4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D61F50: 4BFF6A89  bl 0x82d589d8
	ctx.lr = 0x82D61F54;
	sub_82D589D8(ctx, base);
	// 82D61F54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D61F58: 419A002C  beq cr6, 0x82d61f84
	if ctx.cr[6].eq {
	pc = 0x82D61F84; continue 'dispatch;
	}
	// 82D61F5C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82D61F60: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82D61F64: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D61F68: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D61F6C: 4198FFD4  blt cr6, 0x82d61f40
	if ctx.cr[6].lt {
	pc = 0x82D61F40; continue 'dispatch;
	}
	// 82D61F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D61F74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D61F78: 997C0000  stb r11, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D61F7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D61F80: 4BF474D0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82D61F84: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D61F88: 393E0004  addi r9, r30, 4
	ctx.r[9].s64 = ctx.r[30].s64 + 4;
	// 82D61F8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D61F90: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D61F94: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D61F98: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82D61F9C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D61FA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D61FA4: 4BF474AC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


