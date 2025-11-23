pub fn sub_82C01330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01330 size=224
    let mut pc: u32 = 0x82C01330;
    'dispatch: loop {
        match pc {
            0x82C01330 => {
    //   block [0x82C01330..0x82C0136C)
	// 82C01330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01334: 480A80D9  bl 0x82ca940c
	ctx.lr = 0x82C01338;
	sub_82CA93D0(ctx, base);
	// 82C01338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0133C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C01340: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C01344: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C01348: F88100A0  std r4, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[4].u64 ) };
	// 82C0134C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01350: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01354: 419A0018  beq cr6, 0x82c0136c
	if ctx.cr[6].eq {
	pc = 0x82C0136C; continue 'dispatch;
	}
	// 82C01358: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0135C: 39200068  li r9, 0x68
	ctx.r[9].s64 = 104;
	// 82C01360: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C01364: 7CE84BD7  divw. r7, r8, r9
	ctx.r[7].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82C01368: 4082000C  bne 0x82c01374
	if !ctx.cr[0].eq {
	pc = 0x82C01374; continue 'dispatch;
	}
	pc = 0x82C0136C; continue 'dispatch;
            }
            0x82C0136C => {
    //   block [0x82C0136C..0x82C01374)
	// 82C0136C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C01370: 48000034  b 0x82c013a4
	pc = 0x82C013A4; continue 'dispatch;
            }
            0x82C01374 => {
    //   block [0x82C01374..0x82C01380)
	// 82C01374: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C01378: 40990008  ble cr6, 0x82c01380
	if !ctx.cr[6].gt {
	pc = 0x82C01380; continue 'dispatch;
	}
	// 82C0137C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C01380; continue 'dispatch;
            }
            0x82C01380 => {
    //   block [0x82C01380..0x82C01394)
	// 82C01380: 814100A0  lwz r10, 0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82C01384: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C01388: 419A000C  beq cr6, 0x82c01394
	if ctx.cr[6].eq {
	pc = 0x82C01394; continue 'dispatch;
	}
	// 82C0138C: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C01390: 419A0008  beq cr6, 0x82c01398
	if ctx.cr[6].eq {
	pc = 0x82C01398; continue 'dispatch;
	}
	pc = 0x82C01394; continue 'dispatch;
            }
            0x82C01394 => {
    //   block [0x82C01394..0x82C01398)
	// 82C01394: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C01398; continue 'dispatch;
            }
            0x82C01398 => {
    //   block [0x82C01398..0x82C013A4)
	// 82C01398: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82C0139C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C013A0: 7FC84BD6  divw r30, r8, r9
	ctx.r[30].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	pc = 0x82C013A4; continue 'dispatch;
            }
            0x82C013A4 => {
    //   block [0x82C013A4..0x82C013C4)
	// 82C013A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C013A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C013AC: 4BFFFC05  bl 0x82c00fb0
	ctx.lr = 0x82C013B0;
	sub_82C00FB0(ctx, base);
	// 82C013B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C013B4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C013B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C013BC: 40990008  ble cr6, 0x82c013c4
	if !ctx.cr[6].gt {
	pc = 0x82C013C4; continue 'dispatch;
	}
	// 82C013C0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C013C4; continue 'dispatch;
            }
            0x82C013C4 => {
    //   block [0x82C013C4..0x82C013F4)
	// 82C013C4: 1D5E0068  mulli r10, r30, 0x68
	ctx.r[10].s32 = ((ctx.r[30].s32 as i64 * 104 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82C013C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C013CC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C013D0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82C013D4: E9010050  ld r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C013D8: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82C013DC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C013E0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C013E4: 41990010  bgt cr6, 0x82c013f4
	if ctx.cr[6].gt {
	pc = 0x82C013F4; continue 'dispatch;
	}
	// 82C013E8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C013EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C013F0: 40980008  bge cr6, 0x82c013f8
	if !ctx.cr[6].lt {
	pc = 0x82C013F8; continue 'dispatch;
	}
	pc = 0x82C013F4; continue 'dispatch;
            }
            0x82C013F4 => {
    //   block [0x82C013F4..0x82C013F8)
	// 82C013F4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C013F8; continue 'dispatch;
            }
            0x82C013F8 => {
    //   block [0x82C013F8..0x82C01410)
	// 82C013F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C013FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C01400: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C01404: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C01408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0140C: 480A8050  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01410 size=168
    let mut pc: u32 = 0x82C01410;
    'dispatch: loop {
        match pc {
            0x82C01410 => {
    //   block [0x82C01410..0x82C01434)
	// 82C01410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0141C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01420: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82C01424: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01428: 409A000C  bne cr6, 0x82c01434
	if !ctx.cr[6].eq {
	pc = 0x82C01434; continue 'dispatch;
	}
	// 82C0142C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C01430: 48000010  b 0x82c01440
	pc = 0x82C01440; continue 'dispatch;
            }
            0x82C01434 => {
    //   block [0x82C01434..0x82C01440)
	// 82C01434: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01438: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C0143C: 7D291670  srawi r9, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 2) as i64;
	pc = 0x82C01440; continue 'dispatch;
            }
            0x82C01440 => {
    //   block [0x82C01440..0x82C01480)
	// 82C01440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01444: 419A003C  beq cr6, 0x82c01480
	if ctx.cr[6].eq {
	pc = 0x82C01480; continue 'dispatch;
	}
	// 82C01448: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0144C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C01450: 7D0A1670  srawi r10, r8, 2
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[8].s32 >> 2) as i64;
	// 82C01454: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C01458: 40980028  bge cr6, 0x82c01480
	if !ctx.cr[6].lt {
	pc = 0x82C01480; continue 'dispatch;
	}
	// 82C0145C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01460: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01464: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82C01468: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0146C: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C01470: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0147C: 4E800020  blr
	return;
            }
            0x82C01480 => {
    //   block [0x82C01480..0x82C01490)
	// 82C01480: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01484: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C01488: 40990008  ble cr6, 0x82c01490
	if !ctx.cr[6].gt {
	pc = 0x82C01490; continue 'dispatch;
	}
	// 82C0148C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C01490; continue 'dispatch;
            }
            0x82C01490 => {
    //   block [0x82C01490..0x82C014B8)
	// 82C01490: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82C01494: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C01498: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C0149C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C014A0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C014A4: 4BFFFDAD  bl 0x82c01250
	ctx.lr = 0x82C014A8;
	sub_82C01250(ctx, base);
	// 82C014A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C014AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C014B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C014B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C014B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C014B8 size=192
    let mut pc: u32 = 0x82C014B8;
    'dispatch: loop {
        match pc {
            0x82C014B8 => {
    //   block [0x82C014B8..0x82C014E8)
	// 82C014B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C014BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C014C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C014C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C014C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C014CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C014D0: 39200068  li r9, 0x68
	ctx.r[9].s64 = 104;
	// 82C014D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C014D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C014DC: 409A000C  bne cr6, 0x82c014e8
	if !ctx.cr[6].eq {
	pc = 0x82C014E8; continue 'dispatch;
	}
	// 82C014E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C014E4: 48000010  b 0x82c014f4
	pc = 0x82C014F4; continue 'dispatch;
            }
            0x82C014E8 => {
    //   block [0x82C014E8..0x82C014F4)
	// 82C014E8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C014EC: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C014F0: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	pc = 0x82C014F4; continue 'dispatch;
            }
            0x82C014F4 => {
    //   block [0x82C014F4..0x82C01528)
	// 82C014F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C014F8: 419A003C  beq cr6, 0x82c01534
	if ctx.cr[6].eq {
	pc = 0x82C01534; continue 'dispatch;
	}
	// 82C014FC: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01500: 7CEB4050  subf r7, r11, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82C01504: 7CC74BD6  divw r6, r7, r9
	ctx.r[6].s32 = ctx.r[7].s32 / ctx.r[9].s32;
	// 82C01508: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C0150C: 40980028  bge cr6, 0x82c01534
	if !ctx.cr[6].lt {
	pc = 0x82C01534; continue 'dispatch;
	}
	// 82C01510: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01514: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C01518: 419A0010  beq cr6, 0x82c01528
	if ctx.cr[6].eq {
	pc = 0x82C01528; continue 'dispatch;
	}
	// 82C0151C: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 82C01520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C01524: 480A7F5D  bl 0x82ca9480
	ctx.lr = 0x82C01528;
	sub_82CA9480(ctx, base);
	pc = 0x82C01528; continue 'dispatch;
            }
            0x82C01528 => {
    //   block [0x82C01528..0x82C01534)
	// 82C01528: 397E0068  addi r11, r30, 0x68
	ctx.r[11].s64 = ctx.r[30].s64 + 104;
	// 82C0152C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C01530: 48000030  b 0x82c01560
	pc = 0x82C01560; continue 'dispatch;
            }
            0x82C01534 => {
    //   block [0x82C01534..0x82C01544)
	// 82C01534: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01538: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0153C: 40990008  ble cr6, 0x82c01544
	if !ctx.cr[6].gt {
	pc = 0x82C01544; continue 'dispatch;
	}
	// 82C01540: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C01544; continue 'dispatch;
            }
            0x82C01544 => {
    //   block [0x82C01544..0x82C01560)
	// 82C01544: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82C01548: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C0154C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C01550: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82C01554: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C01558: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C0155C: 4BFFFDD5  bl 0x82c01330
	ctx.lr = 0x82C01560;
	sub_82C01330(ctx, base);
	pc = 0x82C01560; continue 'dispatch;
            }
            0x82C01560 => {
    //   block [0x82C01560..0x82C01578)
	// 82C01560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C01564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0156C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C01570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01578 size=116
    let mut pc: u32 = 0x82C01578;
    'dispatch: loop {
        match pc {
            0x82C01578 => {
    //   block [0x82C01578..0x82C015EC)
	// 82C01578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0157C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0158C: 48074FED  bl 0x82c76578
	ctx.lr = 0x82C01590;
	sub_82C76578(ctx, base);
	// 82C01590: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01594: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01598: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 82C0159C: 388B9C60  addi r4, r11, -0x63a0
	ctx.r[4].s64 = ctx.r[11].s64 + -25504;
	// 82C015A0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C015A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C015A8: 4E800421  bctrl
	ctx.lr = 0x82C015AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C015AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C015B0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C015B4: 80E8000C  lwz r7, 0xc(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C015B8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C015BC: 4E800421  bctrl
	ctx.lr = 0x82C015C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C015C0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C015C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C015C8: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C015CC: 80A60014  lwz r5, 0x14(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C015D0: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82C015D4: 4E800421  bctrl
	ctx.lr = 0x82C015D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C015D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C015DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C015E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C015E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C015E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C015F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C015F0 size=116
    let mut pc: u32 = 0x82C015F0;
    'dispatch: loop {
        match pc {
            0x82C015F0 => {
    //   block [0x82C015F0..0x82C01650)
	// 82C015F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C015F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C015F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C015FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01604: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01608: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0160C: 419A0044  beq cr6, 0x82c01650
	if ctx.cr[6].eq {
	pc = 0x82C01650; continue 'dispatch;
	}
	// 82C01610: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C01614: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C01618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0161C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C01620: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C01624: 4E800421  bctrl
	ctx.lr = 0x82C01628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C01628: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0162C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01630: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C01634: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C01638: 4E800421  bctrl
	ctx.lr = 0x82C0163C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0163C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01640: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01644: 80C70008  lwz r6, 8(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01648: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C0164C: 4E800421  bctrl
	ctx.lr = 0x82C01650;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C01650 => {
    //   block [0x82C01650..0x82C01664)
	// 82C01650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0165C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01668 size=60
    let mut pc: u32 = 0x82C01668;
    'dispatch: loop {
        match pc {
            0x82C01668 => {
    //   block [0x82C01668..0x82C016A4)
	// 82C01668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0166C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01678: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C0167C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C01680: 386B9C70  addi r3, r11, -0x6390
	ctx.r[3].s64 = ctx.r[11].s64 + -25488;
	// 82C01684: 4BF51AA5  bl 0x82b53128
	ctx.lr = 0x82C01688;
	sub_82B53128(ctx, base);
	// 82C01688: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C0168C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C01690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0169C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C016A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C016A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C016A8 size=160
    let mut pc: u32 = 0x82C016A8;
    'dispatch: loop {
        match pc {
            0x82C016A8 => {
    //   block [0x82C016A8..0x82C01728)
	// 82C016A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C016AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C016B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C016B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C016B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C016BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C016C0: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82C016C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C016C8: 4B5FF0A9  bl 0x82200770
	ctx.lr = 0x82C016CC;
	sub_82200770(ctx, base);
	// 82C016CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C016D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C016D4: 419A0054  beq cr6, 0x82c01728
	if ctx.cr[6].eq {
	pc = 0x82C01728; continue 'dispatch;
	}
	// 82C016D8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C016DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C016E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C016E4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C016E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C016EC: 4E800421  bctrl
	ctx.lr = 0x82C016F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C016F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C016F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C016F8: 41980034  blt cr6, 0x82c0172c
	if ctx.cr[6].lt {
	pc = 0x82C0172C; continue 'dispatch;
	}
	// 82C016FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C01700: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82C01704: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82C01708: 553FDFFE  rlwinm r31, r9, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82C0170C: 4B5C1EAD  bl 0x821c35b8
	ctx.lr = 0x82C01710;
	sub_821C35B8(ctx, base);
	// 82C01710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0171C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01724: 4E800020  blr
	return;
            }
            0x82C01728 => {
    //   block [0x82C01728..0x82C0172C)
	// 82C01728: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	pc = 0x82C0172C; continue 'dispatch;
            }
            0x82C0172C => {
    //   block [0x82C0172C..0x82C01748)
	// 82C0172C: 4B5C1E8D  bl 0x821c35b8
	ctx.lr = 0x82C01730;
	sub_821C35B8(ctx, base);
	// 82C01730: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C01734: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0173C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01748 size=104
    let mut pc: u32 = 0x82C01748;
    'dispatch: loop {
        match pc {
            0x82C01748 => {
    //   block [0x82C01748..0x82C01794)
	// 82C01748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0174C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01754: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0175C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C01760: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82C01764: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C01768: 4B5FF009  bl 0x82200770
	ctx.lr = 0x82C0176C;
	sub_82200770(ctx, base);
	// 82C0176C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01770: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01774: 419A0020  beq cr6, 0x82c01794
	if ctx.cr[6].eq {
	pc = 0x82C01794; continue 'dispatch;
	}
	// 82C01778: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0177C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01780: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01784: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C01788: 4E800421  bctrl
	ctx.lr = 0x82C0178C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0178C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C01790: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
            }
            0x82C01794 => {
    //   block [0x82C01794..0x82C017B0)
	// 82C01794: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C01798: 4B5C1E21  bl 0x821c35b8
	ctx.lr = 0x82C0179C;
	sub_821C35B8(ctx, base);
	// 82C0179C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C017A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C017A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C017A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C017AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C017B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C017B0 size=72
    let mut pc: u32 = 0x82C017B0;
    'dispatch: loop {
        match pc {
            0x82C017B0 => {
    //   block [0x82C017B0..0x82C017F8)
	// 82C017B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C017B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C017B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C017BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C017C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C017C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C017C8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82C017CC: 394B9CBC  addi r10, r11, -0x6344
	ctx.r[10].s64 = ctx.r[11].s64 + -25412;
	// 82C017D0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C017D4: 4BF956A5  bl 0x82b96e78
	ctx.lr = 0x82C017D8;
	sub_82B96E78(ctx, base);
	// 82C017D8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C017DC: 39099CA4  addi r8, r9, -0x635c
	ctx.r[8].s64 = ctx.r[9].s64 + -25436;
	// 82C017E0: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C017E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C017E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C017EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C017F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C017F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C017F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C017F8 size=180
    let mut pc: u32 = 0x82C017F8;
    'dispatch: loop {
        match pc {
            0x82C017F8 => {
    //   block [0x82C017F8..0x82C0182C)
	// 82C017F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C017FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01800: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C01804: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0180C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01810: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C01814: 4BFFFE95  bl 0x82c016a8
	ctx.lr = 0x82C01818;
	sub_82C016A8(ctx, base);
	// 82C01818: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C0181C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01820: 409A000C  bne cr6, 0x82c0182c
	if !ctx.cr[6].eq {
	pc = 0x82C0182C; continue 'dispatch;
	}
	// 82C01824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01828: 4BFFFF21  bl 0x82c01748
	ctx.lr = 0x82C0182C;
	sub_82C01748(ctx, base);
	pc = 0x82C0182C; continue 'dispatch;
            }
            0x82C0182C => {
    //   block [0x82C0182C..0x82C01840)
	// 82C0182C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01830: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01834: 419A000C  beq cr6, 0x82c01840
	if ctx.cr[6].eq {
	pc = 0x82C01840; continue 'dispatch;
	}
	// 82C01838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0183C: 4BFFFF0D  bl 0x82c01748
	ctx.lr = 0x82C01840;
	sub_82C01748(ctx, base);
	pc = 0x82C01840; continue 'dispatch;
            }
            0x82C01840 => {
    //   block [0x82C01840..0x82C01884)
	// 82C01840: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C01844: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82C01848: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0184C: 4B5FEF25  bl 0x82200770
	ctx.lr = 0x82C01850;
	sub_82200770(ctx, base);
	// 82C01850: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01858: 409A002C  bne cr6, 0x82c01884
	if !ctx.cr[6].eq {
	pc = 0x82C01884; continue 'dispatch;
	}
	// 82C0185C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C01860: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01868: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0186C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C01870: 4E800421  bctrl
	ctx.lr = 0x82C01874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C01874: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C01878: 4B5C1D41  bl 0x821c35b8
	ctx.lr = 0x82C0187C;
	sub_821C35B8(ctx, base);
	// 82C0187C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C01880: 48000014  b 0x82c01894
	pc = 0x82C01894; continue 'dispatch;
            }
            0x82C01884 => {
    //   block [0x82C01884..0x82C01894)
	// 82C01884: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C01888: 4B5C1D31  bl 0x821c35b8
	ctx.lr = 0x82C0188C;
	sub_821C35B8(ctx, base);
	// 82C0188C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82C01890: 60634005  ori r3, r3, 0x4005
	ctx.r[3].u64 = ctx.r[3].u64 | 16389;
	pc = 0x82C01894; continue 'dispatch;
            }
            0x82C01894 => {
    //   block [0x82C01894..0x82C018AC)
	// 82C01894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0189C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C018A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C018A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C018A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C018B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C018B0 size=140
    let mut pc: u32 = 0x82C018B0;
    'dispatch: loop {
        match pc {
            0x82C018B0 => {
    //   block [0x82C018B0..0x82C018F8)
	// 82C018B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C018B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C018B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C018BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C018C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C018C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C018C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C018CC: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82C018D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C018D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C018D8: 4B5FEE99  bl 0x82200770
	ctx.lr = 0x82C018DC;
	sub_82200770(ctx, base);
	// 82C018DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C018E0: 4BFFFDC9  bl 0x82c016a8
	ctx.lr = 0x82C018E4;
	sub_82C016A8(ctx, base);
	// 82C018E4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C018E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C018EC: 409A000C  bne cr6, 0x82c018f8
	if !ctx.cr[6].eq {
	pc = 0x82C018F8; continue 'dispatch;
	}
	// 82C018F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C018F4: 4BFFFE55  bl 0x82c01748
	ctx.lr = 0x82C018F8;
	sub_82C01748(ctx, base);
	pc = 0x82C018F8; continue 'dispatch;
            }
            0x82C018F8 => {
    //   block [0x82C018F8..0x82C0191C)
	// 82C018F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C018FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01900: 419A001C  beq cr6, 0x82c0191c
	if ctx.cr[6].eq {
	pc = 0x82C0191C; continue 'dispatch;
	}
	// 82C01904: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01908: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C0190C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01910: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C01914: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C01918: 4E800421  bctrl
	ctx.lr = 0x82C0191C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C0191C => {
    //   block [0x82C0191C..0x82C0193C)
	// 82C0191C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C01920: 4B5C1C99  bl 0x821c35b8
	ctx.lr = 0x82C01924;
	sub_821C35B8(ctx, base);
	// 82C01924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0192C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01930: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C01934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01940 size=180
    let mut pc: u32 = 0x82C01940;
    'dispatch: loop {
        match pc {
            0x82C01940 => {
    //   block [0x82C01940..0x82C01988)
	// 82C01940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C0194C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01958: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C0195C: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82C01960: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C01964: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C01968: 4B5FEE09  bl 0x82200770
	ctx.lr = 0x82C0196C;
	sub_82200770(ctx, base);
	// 82C0196C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01970: 4BFFFD39  bl 0x82c016a8
	ctx.lr = 0x82C01974;
	sub_82C016A8(ctx, base);
	// 82C01974: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C01978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0197C: 409A000C  bne cr6, 0x82c01988
	if !ctx.cr[6].eq {
	pc = 0x82C01988; continue 'dispatch;
	}
	// 82C01980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01984: 4BFFFDC5  bl 0x82c01748
	ctx.lr = 0x82C01988;
	sub_82C01748(ctx, base);
	pc = 0x82C01988; continue 'dispatch;
            }
            0x82C01988 => {
    //   block [0x82C01988..0x82C019D4)
	// 82C01988: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0198C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01990: 419A0044  beq cr6, 0x82c019d4
	if ctx.cr[6].eq {
	pc = 0x82C019D4; continue 'dispatch;
	}
	// 82C01994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C01998: 4808C599  bl 0x82c8df30
	ctx.lr = 0x82C0199C;
	sub_82C8DF30(ctx, base);
	// 82C0199C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C019A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C019A4: 4808BE05  bl 0x82c8d7a8
	ctx.lr = 0x82C019A8;
	sub_82C8D7A8(ctx, base);
	// 82C019A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C019AC: 4808C4F5  bl 0x82c8dea0
	ctx.lr = 0x82C019B0;
	sub_82C8DEA0(ctx, base);
	// 82C019B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C019B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C019B8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C019BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C019C0: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C019C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C019C8: 4E800421  bctrl
	ctx.lr = 0x82C019CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C019CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C019D0: 4808C741  bl 0x82c8e110
	ctx.lr = 0x82C019D4;
	sub_82C8E110(ctx, base);
            }
            0x82C019D4 => {
    //   block [0x82C019D4..0x82C019F4)
	// 82C019D4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C019D8: 4B5C1BE1  bl 0x821c35b8
	ctx.lr = 0x82C019DC;
	sub_821C35B8(ctx, base);
	// 82C019DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C019E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C019E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C019E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C019EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C019F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C019F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C019F8 size=168
    let mut pc: u32 = 0x82C019F8;
    'dispatch: loop {
        match pc {
            0x82C019F8 => {
    //   block [0x82C019F8..0x82C01A38)
	// 82C019F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C019FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01A04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01A0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C01A10: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82C01A14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C01A18: 4B5FED59  bl 0x82200770
	ctx.lr = 0x82C01A1C;
	sub_82200770(ctx, base);
	// 82C01A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01A20: 4BFFFC89  bl 0x82c016a8
	ctx.lr = 0x82C01A24;
	sub_82C016A8(ctx, base);
	// 82C01A24: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C01A28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01A2C: 409A000C  bne cr6, 0x82c01a38
	if !ctx.cr[6].eq {
	pc = 0x82C01A38; continue 'dispatch;
	}
	// 82C01A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01A34: 4BFFFD15  bl 0x82c01748
	ctx.lr = 0x82C01A38;
	sub_82C01748(ctx, base);
	pc = 0x82C01A38; continue 'dispatch;
            }
            0x82C01A38 => {
    //   block [0x82C01A38..0x82C01A80)
	// 82C01A38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01A3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01A40: 419A0040  beq cr6, 0x82c01a80
	if ctx.cr[6].eq {
	pc = 0x82C01A80; continue 'dispatch;
	}
	// 82C01A44: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01A48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C01A4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01A50: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01A54: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C01A58: 4E800421  bctrl
	ctx.lr = 0x82C01A5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C01A5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C01A60: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C01A64: 4B5C1B55  bl 0x821c35b8
	ctx.lr = 0x82C01A68;
	sub_821C35B8(ctx, base);
	// 82C01A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01A6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01A78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01A7C: 4E800020  blr
	return;
            }
            0x82C01A80 => {
    //   block [0x82C01A80..0x82C01AA0)
	// 82C01A80: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C01A84: 4B5C1B35  bl 0x821c35b8
	ctx.lr = 0x82C01A88;
	sub_821C35B8(ctx, base);
	// 82C01A88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C01A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01A98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01AA0 size=80
    let mut pc: u32 = 0x82C01AA0;
    'dispatch: loop {
        match pc {
            0x82C01AA0 => {
    //   block [0x82C01AA0..0x82C01AF0)
	// 82C01AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01AA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01AAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01AB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01AB4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C01AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C01ABC: 392A9CBC  addi r9, r10, -0x6344
	ctx.r[9].s64 = ctx.r[10].s64 + -25412;
	// 82C01AC0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82C01AC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C01AC8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C01ACC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C01AD0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C01AD4: 4BF3BDDD  bl 0x82b3d8b0
	ctx.lr = 0x82C01AD8;
	sub_82B3D8B0(ctx, base);
	// 82C01AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01ADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01AF0 size=176
    let mut pc: u32 = 0x82C01AF0;
    'dispatch: loop {
        match pc {
            0x82C01AF0 => {
    //   block [0x82C01AF0..0x82C01B80)
	// 82C01AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C01AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01B00: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01B04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01B08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C01B0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C01B10: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82C01B14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C01B18: 4B5FEC59  bl 0x82200770
	ctx.lr = 0x82C01B1C;
	sub_82200770(ctx, base);
	// 82C01B1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01B20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01B24: 419A005C  beq cr6, 0x82c01b80
	if ctx.cr[6].eq {
	pc = 0x82C01B80; continue 'dispatch;
	}
	// 82C01B28: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01B2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C01B30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C01B34: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C01B38: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01B3C: 812A0034  lwz r9, 0x34(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C01B40: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C01B44: 4E800421  bctrl
	ctx.lr = 0x82C01B48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C01B48: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C01B4C: 41980034  blt cr6, 0x82c01b80
	if ctx.cr[6].lt {
	pc = 0x82C01B80; continue 'dispatch;
	}
	// 82C01B50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C01B54: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C01B58: 4B6F03E9  bl 0x822f1f40
	ctx.lr = 0x82C01B5C;
	sub_822F1F40(ctx, base);
	// 82C01B5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C01B60: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82C01B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C01B68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C01B6C: 4B58CE95  bl 0x8218ea00
	ctx.lr = 0x82C01B70;
	sub_8218EA00(ctx, base);
	// 82C01B70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C01B74: 4B56FC9D  bl 0x82171810
	ctx.lr = 0x82C01B78;
	sub_82171810(ctx, base);
	// 82C01B78: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C01B7C: 4BC43C35  bl 0x828457b0
	ctx.lr = 0x82C01B80;
	sub_828457B0(ctx, base);
            }
            0x82C01B80 => {
    //   block [0x82C01B80..0x82C01BA0)
	// 82C01B80: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C01B84: 4B5C1A35  bl 0x821c35b8
	ctx.lr = 0x82C01B88;
	sub_821C35B8(ctx, base);
	// 82C01B88: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C01B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01B94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C01B98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01BA0 size=112
    let mut pc: u32 = 0x82C01BA0;
    'dispatch: loop {
        match pc {
            0x82C01BA0 => {
    //   block [0x82C01BA0..0x82C01BB4)
	// 82C01BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01BA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01BAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01BB0: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	pc = 0x82C01BB4; continue 'dispatch;
            }
            0x82C01BB4 => {
    //   block [0x82C01BB4..0x82C01BF8)
	// 82C01BB4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82C01BB8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82C01BBC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82C01BC0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C01BC4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82C01BC8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82C01BCC: 4082FFE8  bne 0x82c01bb4
	if !ctx.cr[0].eq {
	pc = 0x82C01BB4; continue 'dispatch;
	}
	// 82C01BD0: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82C01BD4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82C01BD8: 409A0020  bne cr6, 0x82c01bf8
	if !ctx.cr[6].eq {
	pc = 0x82C01BF8; continue 'dispatch;
	}
	// 82C01BDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C01BE0: 419A0018  beq cr6, 0x82c01bf8
	if ctx.cr[6].eq {
	pc = 0x82C01BF8; continue 'dispatch;
	}
	// 82C01BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01BE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C01BEC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C01BF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C01BF4: 4E800421  bctrl
	ctx.lr = 0x82C01BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C01BF8 => {
    //   block [0x82C01BF8..0x82C01C10)
	// 82C01BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01C08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01C10 size=72
    let mut pc: u32 = 0x82C01C10;
    'dispatch: loop {
        match pc {
            0x82C01C10 => {
    //   block [0x82C01C10..0x82C01C44)
	// 82C01C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01C18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01C1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01C20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01C24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01C28: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C01C2C: 392B9CA4  addi r9, r11, -0x635c
	ctx.r[9].s64 = ctx.r[11].s64 + -25436;
	// 82C01C30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C01C34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C01C38: 419A000C  beq cr6, 0x82c01c44
	if ctx.cr[6].eq {
	pc = 0x82C01C44; continue 'dispatch;
	}
	// 82C01C3C: 4BC43B75  bl 0x828457b0
	ctx.lr = 0x82C01C40;
	sub_828457B0(ctx, base);
	// 82C01C40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C01C44; continue 'dispatch;
            }
            0x82C01C44 => {
    //   block [0x82C01C44..0x82C01C58)
	// 82C01C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01C50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01C58 size=196
    let mut pc: u32 = 0x82C01C58;
    'dispatch: loop {
        match pc {
            0x82C01C58 => {
    //   block [0x82C01C58..0x82C01C74)
	// 82C01C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01C64: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82C01C68: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C01C6C: 394A8F88  addi r10, r10, -0x7078
	ctx.r[10].s64 = ctx.r[10].s64 + -28792;
	// 82C01C70: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	pc = 0x82C01C74; continue 'dispatch;
            }
            0x82C01C74 => {
    //   block [0x82C01C74..0x82C01C94)
	// 82C01C74: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01C78: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01C7C: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C01C80: 40820014  bne 0x82c01c94
	if !ctx.cr[0].eq {
	pc = 0x82C01C94; continue 'dispatch;
	}
	// 82C01C84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C01C88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C01C8C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82C01C90: 409AFFE4  bne cr6, 0x82c01c74
	if !ctx.cr[6].eq {
	pc = 0x82C01C74; continue 'dispatch;
	}
	pc = 0x82C01C94; continue 'dispatch;
            }
            0x82C01C94 => {
    //   block [0x82C01C94..0x82C01CAC)
	// 82C01C94: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C01C98: 419A003C  beq cr6, 0x82c01cd4
	if ctx.cr[6].eq {
	pc = 0x82C01CD4; continue 'dispatch;
	}
	// 82C01C9C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C01CA0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C01CA4: 394A9CD4  addi r10, r10, -0x632c
	ctx.r[10].s64 = ctx.r[10].s64 + -25388;
	// 82C01CA8: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	pc = 0x82C01CAC; continue 'dispatch;
            }
            0x82C01CAC => {
    //   block [0x82C01CAC..0x82C01CCC)
	// 82C01CAC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01CB0: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01CB4: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C01CB8: 40820014  bne 0x82c01ccc
	if !ctx.cr[0].eq {
	pc = 0x82C01CCC; continue 'dispatch;
	}
	// 82C01CBC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C01CC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C01CC4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82C01CC8: 409AFFE4  bne cr6, 0x82c01cac
	if !ctx.cr[6].eq {
	pc = 0x82C01CAC; continue 'dispatch;
	}
	pc = 0x82C01CCC; continue 'dispatch;
            }
            0x82C01CCC => {
    //   block [0x82C01CCC..0x82C01CD4)
	// 82C01CCC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C01CD0: 409A002C  bne cr6, 0x82c01cfc
	if !ctx.cr[6].eq {
	pc = 0x82C01CFC; continue 'dispatch;
	}
	pc = 0x82C01CD4; continue 'dispatch;
            }
            0x82C01CD4 => {
    //   block [0x82C01CD4..0x82C01CFC)
	// 82C01CD4: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C01CD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01CDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01CE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C01CE4: 4E800421  bctrl
	ctx.lr = 0x82C01CE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C01CE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C01CEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01CF8: 4E800020  blr
	return;
            }
            0x82C01CFC => {
    //   block [0x82C01CFC..0x82C01D1C)
	// 82C01CFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C01D00: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82C01D04: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C01D08: 60634002  ori r3, r3, 0x4002
	ctx.r[3].u64 = ctx.r[3].u64 | 16386;
	// 82C01D0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01D20 size=108
    let mut pc: u32 = 0x82C01D20;
    'dispatch: loop {
        match pc {
            0x82C01D20 => {
    //   block [0x82C01D20..0x82C01D74)
	// 82C01D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01D28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C01D2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01D30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01D38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01D3C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82C01D40: 394B9CBC  addi r10, r11, -0x6344
	ctx.r[10].s64 = ctx.r[11].s64 + -25412;
	// 82C01D44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C01D48: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C01D4C: 4BF9512D  bl 0x82b96e78
	ctx.lr = 0x82C01D50;
	sub_82B96E78(ctx, base);
	// 82C01D50: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C01D54: 57C807FE  clrlwi r8, r30, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C01D58: 38E99CA4  addi r7, r9, -0x635c
	ctx.r[7].s64 = ctx.r[9].s64 + -25436;
	// 82C01D5C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C01D60: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C01D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C01D68: 419A000C  beq cr6, 0x82c01d74
	if ctx.cr[6].eq {
	pc = 0x82C01D74; continue 'dispatch;
	}
	// 82C01D6C: 4BC43A45  bl 0x828457b0
	ctx.lr = 0x82C01D70;
	sub_828457B0(ctx, base);
	// 82C01D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C01D74; continue 'dispatch;
            }
            0x82C01D74 => {
    //   block [0x82C01D74..0x82C01D8C)
	// 82C01D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C01D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01D80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C01D84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01D90 size=32
    let mut pc: u32 = 0x82C01D90;
    'dispatch: loop {
        match pc {
            0x82C01D90 => {
    //   block [0x82C01D90..0x82C01DB0)
	// 82C01D90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01D94: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C01D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C01D9C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82C01DA0: 392B9CE4  addi r9, r11, -0x631c
	ctx.r[9].s64 = ctx.r[11].s64 + -25372;
	// 82C01DA4: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C01DA8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C01DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01DB0 size=16
    let mut pc: u32 = 0x82C01DB0;
    'dispatch: loop {
        match pc {
            0x82C01DB0 => {
    //   block [0x82C01DB0..0x82C01DC0)
	// 82C01DB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01DB4: 394B94F8  addi r10, r11, -0x6b08
	ctx.r[10].s64 = ctx.r[11].s64 + -27400;
	// 82C01DB8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C01DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01DC0 size=40
    let mut pc: u32 = 0x82C01DC0;
    'dispatch: loop {
        match pc {
            0x82C01DC0 => {
    //   block [0x82C01DC0..0x82C01DE8)
	// 82C01DC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C01DC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C01DC8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01DCC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01DD0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C01DD4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82C01DD8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01DDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C01DE0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C01DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01DE8 size=40
    let mut pc: u32 = 0x82C01DE8;
    'dispatch: loop {
        match pc {
            0x82C01DE8 => {
    //   block [0x82C01DE8..0x82C01E10)
	// 82C01DE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C01DEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C01DF0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01DF4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01DF8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C01DFC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82C01E00: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01E04: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C01E08: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C01E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01E10 size=20
    let mut pc: u32 = 0x82C01E10;
    'dispatch: loop {
        match pc {
            0x82C01E10 => {
    //   block [0x82C01E10..0x82C01E24)
	// 82C01E10: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01E14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C01E18: 409A000C  bne cr6, 0x82c01e24
	if !ctx.cr[6].eq {
		sub_82C01E24(ctx, base);
		return;
	}
	// 82C01E1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C01E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01E24 size=44
    let mut pc: u32 = 0x82C01E24;
    'dispatch: loop {
        match pc {
            0x82C01E24 => {
    //   block [0x82C01E24..0x82C01E3C)
	// 82C01E24: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01E28: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C01E2C: 409A0010  bne cr6, 0x82c01e3c
	if !ctx.cr[6].eq {
	pc = 0x82C01E3C; continue 'dispatch;
	}
	// 82C01E30: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01E34: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C01E38: 419AFFE4  beq cr6, 0x82c01e1c
	if ctx.cr[6].eq {
		sub_82C01E10(ctx, base);
		return;
	}
	pc = 0x82C01E3C; continue 'dispatch;
            }
            0x82C01E3C => {
    //   block [0x82C01E3C..0x82C01E50)
	// 82C01E3C: 7D4B0034  cntlzw r11, r10
	ctx.r[11].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82C01E40: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82C01E44: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82C01E48: 69230001  xori r3, r9, 1
	ctx.r[3].u64 = ctx.r[9].u64 ^ 1;
	// 82C01E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01E50 size=40
    let mut pc: u32 = 0x82C01E50;
    'dispatch: loop {
        match pc {
            0x82C01E50 => {
    //   block [0x82C01E50..0x82C01E78)
	// 82C01E50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C01E54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C01E58: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C01E5C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01E60: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C01E64: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82C01E68: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01E6C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C01E70: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C01E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01E78 size=12
    let mut pc: u32 = 0x82C01E78;
    'dispatch: loop {
        match pc {
            0x82C01E78 => {
    //   block [0x82C01E78..0x82C01E84)
	// 82C01E78: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82C01E7C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C01E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01E88 size=32
    let mut pc: u32 = 0x82C01E88;
    'dispatch: loop {
        match pc {
            0x82C01E88 => {
    //   block [0x82C01E88..0x82C01EA8)
	// 82C01E88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C01E8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C01E90: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01E94: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C01E98: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82C01E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C01EA0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C01EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01EA8 size=28
    let mut pc: u32 = 0x82C01EA8;
    'dispatch: loop {
        match pc {
            0x82C01EA8 => {
    //   block [0x82C01EA8..0x82C01EC4)
	// 82C01EA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01EAC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C01EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C01EB4: 392B9D04  addi r9, r11, -0x62fc
	ctx.r[9].s64 = ctx.r[11].s64 + -25340;
	// 82C01EB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C01EBC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C01EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01EC8 size=16
    let mut pc: u32 = 0x82C01EC8;
    'dispatch: loop {
        match pc {
            0x82C01EC8 => {
    //   block [0x82C01EC8..0x82C01ED8)
	// 82C01EC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01ECC: 394B94F8  addi r10, r11, -0x6b08
	ctx.r[10].s64 = ctx.r[11].s64 + -27400;
	// 82C01ED0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C01ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01ED8 size=24
    let mut pc: u32 = 0x82C01ED8;
    'dispatch: loop {
        match pc {
            0x82C01ED8 => {
    //   block [0x82C01ED8..0x82C01EF0)
	// 82C01ED8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C01EDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C01EE0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01EE4: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82C01EE8: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C01EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01EF0 size=24
    let mut pc: u32 = 0x82C01EF0;
    'dispatch: loop {
        match pc {
            0x82C01EF0 => {
    //   block [0x82C01EF0..0x82C01F08)
	// 82C01EF0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C01EF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C01EF8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01EFC: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82C01F00: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C01F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01F08 size=24
    let mut pc: u32 = 0x82C01F08;
    'dispatch: loop {
        match pc {
            0x82C01F08 => {
    //   block [0x82C01F08..0x82C01F20)
	// 82C01F08: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C01F0C: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82C01F10: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82C01F14: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82C01F18: 69030001  xori r3, r8, 1
	ctx.r[3].u64 = ctx.r[8].u64 ^ 1;
	// 82C01F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01F20 size=72
    let mut pc: u32 = 0x82C01F20;
    'dispatch: loop {
        match pc {
            0x82C01F20 => {
    //   block [0x82C01F20..0x82C01F54)
	// 82C01F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C01F28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C01F2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01F30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C01F34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C01F38: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C01F3C: 392B94F8  addi r9, r11, -0x6b08
	ctx.r[9].s64 = ctx.r[11].s64 + -27400;
	// 82C01F40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C01F44: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C01F48: 419A000C  beq cr6, 0x82c01f54
	if ctx.cr[6].eq {
	pc = 0x82C01F54; continue 'dispatch;
	}
	// 82C01F4C: 4BC43865  bl 0x828457b0
	ctx.lr = 0x82C01F50;
	sub_828457B0(ctx, base);
	// 82C01F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C01F54; continue 'dispatch;
            }
            0x82C01F54 => {
    //   block [0x82C01F54..0x82C01F68)
	// 82C01F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C01F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C01F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C01F60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C01F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01F68 size=20
    let mut pc: u32 = 0x82C01F68;
    'dispatch: loop {
        match pc {
            0x82C01F68 => {
    //   block [0x82C01F68..0x82C01F7C)
	// 82C01F68: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C01F6C: 816A4B90  lwz r11, 0x4b90(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19344 as u32) ) } as u64;
	// 82C01F70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C01F74: 916A4B90  stw r11, 0x4b90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(19344 as u32), ctx.r[11].u32 ) };
	// 82C01F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C01F80 size=20
    let mut pc: u32 = 0x82C01F80;
    'dispatch: loop {
        match pc {
            0x82C01F80 => {
    //   block [0x82C01F80..0x82C01F94)
	// 82C01F80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C01F84: 816A4B90  lwz r11, 0x4b90(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19344 as u32) ) } as u64;
	// 82C01F88: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C01F8C: 916A4B90  stw r11, 0x4b90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(19344 as u32), ctx.r[11].u32 ) };
	// 82C01F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C01F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C01F98 size=208
    let mut pc: u32 = 0x82C01F98;
    'dispatch: loop {
        match pc {
            0x82C01F98 => {
    //   block [0x82C01F98..0x82C01FF0)
	// 82C01F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C01F9C: 480A7469  bl 0x82ca9404
	ctx.lr = 0x82C01FA0;
	sub_82CA93D0(ctx, base);
	// 82C01FA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C01FA4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C01FA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C01FAC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C01FB0: 816B4B90  lwz r11, 0x4b90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19344 as u32) ) } as u64;
	// 82C01FB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C01FB8: 419900A8  bgt cr6, 0x82c02060
	if ctx.cr[6].gt {
	pc = 0x82C02060; continue 'dispatch;
	}
	// 82C01FBC: 3FA08333  lis r29, -0x7ccd
	ctx.r[29].s64 = -2093809664;
	// 82C01FC0: 897D4BA0  lbz r11, 0x4ba0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(19360 as u32) ) } as u64;
	// 82C01FC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01FC8: 409A0098  bne cr6, 0x82c02060
	if !ctx.cr[6].eq {
	pc = 0x82C02060; continue 'dispatch;
	}
	// 82C01FCC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C01FD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C01FD4: 3BEA4B94  addi r31, r10, 0x4b94
	ctx.r[31].s64 = ctx.r[10].s64 + 19348;
	// 82C01FD8: 997D4BA0  stb r11, 0x4ba0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(19360 as u32), ctx.r[11].u8 ) };
	// 82C01FDC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C01FE0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C01FE4: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C01FE8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C01FEC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	pc = 0x82C01FF0; continue 'dispatch;
            }
            0x82C01FF0 => {
    //   block [0x82C01FF0..0x82C02000)
	// 82C01FF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C01FF4: 419A000C  beq cr6, 0x82c02000
	if ctx.cr[6].eq {
	pc = 0x82C02000; continue 'dispatch;
	}
	// 82C01FF8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C01FFC: 419A0008  beq cr6, 0x82c02004
	if ctx.cr[6].eq {
	pc = 0x82C02004; continue 'dispatch;
	}
	pc = 0x82C02000; continue 'dispatch;
            }
            0x82C02000 => {
    //   block [0x82C02000..0x82C02004)
	// 82C02000: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C02004; continue 'dispatch;
            }
            0x82C02004 => {
    //   block [0x82C02004..0x82C02018)
	// 82C02004: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C02008: 419A0050  beq cr6, 0x82c02058
	if ctx.cr[6].eq {
	pc = 0x82C02058; continue 'dispatch;
	}
	// 82C0200C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02010: 409A0008  bne cr6, 0x82c02018
	if !ctx.cr[6].eq {
	pc = 0x82C02018; continue 'dispatch;
	}
	// 82C02014: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C02018; continue 'dispatch;
            }
            0x82C02018 => {
    //   block [0x82C02018..0x82C02028)
	// 82C02018: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0201C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C02020: 409A0008  bne cr6, 0x82c02028
	if !ctx.cr[6].eq {
	pc = 0x82C02028; continue 'dispatch;
	}
	// 82C02024: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C02028; continue 'dispatch;
            }
            0x82C02028 => {
    //   block [0x82C02028..0x82C02044)
	// 82C02028: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0202C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02030: 419A0014  beq cr6, 0x82c02044
	if ctx.cr[6].eq {
	pc = 0x82C02044; continue 'dispatch;
	}
	// 82C02034: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C02038: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C0203C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C02040: 4E800421  bctrl
	ctx.lr = 0x82C02044;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C02044 => {
    //   block [0x82C02044..0x82C02058)
	// 82C02044: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02048: 4B6FE0E1  bl 0x82300128
	ctx.lr = 0x82C0204C;
	sub_82300128(ctx, base);
	// 82C0204C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C02050: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C02054: 4BFFFF9C  b 0x82c01ff0
	pc = 0x82C01FF0; continue 'dispatch;
            }
            0x82C02058 => {
    //   block [0x82C02058..0x82C02060)
	// 82C02058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0205C: 997D4BA0  stb r11, 0x4ba0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(19360 as u32), ctx.r[11].u8 ) };
	pc = 0x82C02060; continue 'dispatch;
            }
            0x82C02060 => {
    //   block [0x82C02060..0x82C02068)
	// 82C02060: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C02064: 480A73F0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02068 size=136
    let mut pc: u32 = 0x82C02068;
    'dispatch: loop {
        match pc {
            0x82C02068 => {
    //   block [0x82C02068..0x82C020C8)
	// 82C02068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C02074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0207C: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82C02080: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C02084: 817F4B90  lwz r11, 0x4b90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19344 as u32) ) } as u64;
	// 82C02088: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C0208C: 917F4B90  stw r11, 0x4b90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19344 as u32), ctx.r[11].u32 ) };
	// 82C02090: 4B625CF9  bl 0x82227d88
	ctx.lr = 0x82C02094;
	sub_82227D88(ctx, base);
	// 82C02094: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82C02098: 41980030  blt cr6, 0x82c020c8
	if ctx.cr[6].lt {
	pc = 0x82C020C8; continue 'dispatch;
	}
	// 82C0209C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C020A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C020A4: 4B62FC8D  bl 0x82231d30
	ctx.lr = 0x82C020A8;
	sub_82231D30(ctx, base);
	// 82C020A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C020AC: 419A001C  beq cr6, 0x82c020c8
	if ctx.cr[6].eq {
	pc = 0x82C020C8; continue 'dispatch;
	}
	// 82C020B0: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82C020B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82C020B8: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82C020BC: 388B1F98  addi r4, r11, 0x1f98
	ctx.r[4].s64 = ctx.r[11].s64 + 8088;
	// 82C020C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C020C4: 4BE2ED8D  bl 0x82a30e50
	ctx.lr = 0x82C020C8;
	sub_82A30E50(ctx, base);
	pc = 0x82C020C8; continue 'dispatch;
            }
            0x82C020C8 => {
    //   block [0x82C020C8..0x82C020F0)
	// 82C020C8: 817F4B90  lwz r11, 0x4b90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19344 as u32) ) } as u64;
	// 82C020CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C020D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C020D4: 917F4B90  stw r11, 0x4b90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19344 as u32), ctx.r[11].u32 ) };
	// 82C020D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C020DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C020E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C020E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C020E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C020EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C020F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C020F0 size=196
    let mut pc: u32 = 0x82C020F0;
    'dispatch: loop {
        match pc {
            0x82C020F0 => {
    //   block [0x82C020F0..0x82C02178)
	// 82C020F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C020F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C020F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C020FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02104: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C02108: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82C0210C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C02110: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C02114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02118: 817E4B90  lwz r11, 0x4b90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19344 as u32) ) } as u64;
	// 82C0211C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C02120: 917E4B90  stw r11, 0x4b90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(19344 as u32), ctx.r[11].u32 ) };
	// 82C02124: 4BE2ED2D  bl 0x82a30e50
	ctx.lr = 0x82C02128;
	sub_82A30E50(ctx, base);
	// 82C02128: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C0212C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C02130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02134: 38ABA064  addi r5, r11, -0x5f9c
	ctx.r[5].s64 = ctx.r[11].s64 + -24476;
	// 82C02138: 388AA050  addi r4, r10, -0x5fb0
	ctx.r[4].s64 = ctx.r[10].s64 + -24496;
	// 82C0213C: 4BE230BD  bl 0x82a251f8
	ctx.lr = 0x82C02140;
	sub_82A251F8(ctx, base);
	// 82C02140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02144: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 82C02148: 4BFC9BC1  bl 0x82bcbd08
	ctx.lr = 0x82C0214C;
	sub_82BCBD08(ctx, base);
	// 82C0214C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C02150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02154: 38899D28  addi r4, r9, -0x62d8
	ctx.r[4].s64 = ctx.r[9].s64 + -25304;
	// 82C02158: 4BE23909  bl 0x82a25a60
	ctx.lr = 0x82C0215C;
	sub_82A25A60(ctx, base);
	// 82C0215C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C02160: 409A0018  bne cr6, 0x82c02178
	if !ctx.cr[6].eq {
	pc = 0x82C02178; continue 'dispatch;
	}
	// 82C02164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82C02168: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0216C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C02170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02174: 4B598795  bl 0x8219a908
	ctx.lr = 0x82C02178;
	sub_8219A908(ctx, base);
	pc = 0x82C02178; continue 'dispatch;
            }
            0x82C02178 => {
    //   block [0x82C02178..0x82C021B4)
	// 82C02178: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82C0217C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82C02180: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82C02184: 388B1F98  addi r4, r11, 0x1f98
	ctx.r[4].s64 = ctx.r[11].s64 + 8088;
	// 82C02188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0218C: 4BE2ECC5  bl 0x82a30e50
	ctx.lr = 0x82C02190;
	sub_82A30E50(ctx, base);
	// 82C02190: 817E4B90  lwz r11, 0x4b90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19344 as u32) ) } as u64;
	// 82C02194: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C02198: 917E4B90  stw r11, 0x4b90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(19344 as u32), ctx.r[11].u32 ) };
	// 82C0219C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C021A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C021A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C021A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C021AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C021B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C021B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C021B8 size=120
    let mut pc: u32 = 0x82C021B8;
    'dispatch: loop {
        match pc {
            0x82C021B8 => {
    //   block [0x82C021B8..0x82C021F8)
	// 82C021B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C021BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C021C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C021C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C021C8: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82C021CC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C021D0: 38A10084  addi r5, r1, 0x84
	ctx.r[5].s64 = ctx.r[1].s64 + 132;
	// 82C021D4: 3BEB4B94  addi r31, r11, 0x4b94
	ctx.r[31].s64 = ctx.r[11].s64 + 19348;
	// 82C021D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C021DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C021E0: 4B78CD39  bl 0x8238ef18
	ctx.lr = 0x82C021E4;
	sub_8238EF18(ctx, base);
	// 82C021E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C021E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C021EC: 419A000C  beq cr6, 0x82c021f8
	if ctx.cr[6].eq {
	pc = 0x82C021F8; continue 'dispatch;
	}
	// 82C021F0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C021F4: 419A0008  beq cr6, 0x82c021fc
	if ctx.cr[6].eq {
	pc = 0x82C021FC; continue 'dispatch;
	}
	pc = 0x82C021F8; continue 'dispatch;
            }
            0x82C021F8 => {
    //   block [0x82C021F8..0x82C021FC)
	// 82C021F8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C021FC; continue 'dispatch;
            }
            0x82C021FC => {
    //   block [0x82C021FC..0x82C0221C)
	// 82C021FC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02200: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02204: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C02208: 409A0014  bne cr6, 0x82c0221c
	if !ctx.cr[6].eq {
	pc = 0x82C0221C; continue 'dispatch;
	}
	// 82C0220C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C02210: 38A10084  addi r5, r1, 0x84
	ctx.r[5].s64 = ctx.r[1].s64 + 132;
	// 82C02214: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02218: 48000511  bl 0x82c02728
	ctx.lr = 0x82C0221C;
	sub_82C02728(ctx, base);
	pc = 0x82C0221C; continue 'dispatch;
            }
            0x82C0221C => {
    //   block [0x82C0221C..0x82C02230)
	// 82C0221C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C02220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0222C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02230 size=120
    let mut pc: u32 = 0x82C02230;
    'dispatch: loop {
        match pc {
            0x82C02230 => {
    //   block [0x82C02230..0x82C02270)
	// 82C02230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0223C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02240: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82C02244: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C02248: 38A10084  addi r5, r1, 0x84
	ctx.r[5].s64 = ctx.r[1].s64 + 132;
	// 82C0224C: 3BEB4B94  addi r31, r11, 0x4b94
	ctx.r[31].s64 = ctx.r[11].s64 + 19348;
	// 82C02250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02254: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C02258: 4B78CCC1  bl 0x8238ef18
	ctx.lr = 0x82C0225C;
	sub_8238EF18(ctx, base);
	// 82C0225C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C02260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02264: 419A000C  beq cr6, 0x82c02270
	if ctx.cr[6].eq {
	pc = 0x82C02270; continue 'dispatch;
	}
	// 82C02268: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C0226C: 419A0008  beq cr6, 0x82c02274
	if ctx.cr[6].eq {
	pc = 0x82C02274; continue 'dispatch;
	}
	pc = 0x82C02270; continue 'dispatch;
            }
            0x82C02270 => {
    //   block [0x82C02270..0x82C02274)
	// 82C02270: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C02274; continue 'dispatch;
            }
            0x82C02274 => {
    //   block [0x82C02274..0x82C02294)
	// 82C02274: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02278: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C0227C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C02280: 419A0014  beq cr6, 0x82c02294
	if ctx.cr[6].eq {
	pc = 0x82C02294; continue 'dispatch;
	}
	// 82C02284: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C02288: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C0228C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02290: 480822A1  bl 0x82c84530
	ctx.lr = 0x82C02294;
	sub_82C84530(ctx, base);
	pc = 0x82C02294; continue 'dispatch;
            }
            0x82C02294 => {
    //   block [0x82C02294..0x82C022A8)
	// 82C02294: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C02298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0229C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C022A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C022A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C022A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C022A8 size=100
    let mut pc: u32 = 0x82C022A8;
    'dispatch: loop {
        match pc {
            0x82C022A8 => {
    //   block [0x82C022A8..0x82C022D0)
	// 82C022A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C022AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C022B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C022B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C022B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C022BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C022C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C022C4: 897F0011  lbz r11, 0x11(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C022C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C022CC: 409A0028  bne cr6, 0x82c022f4
	if !ctx.cr[6].eq {
	pc = 0x82C022F4; continue 'dispatch;
	}
	pc = 0x82C022D0; continue 'dispatch;
            }
            0x82C022D0 => {
    //   block [0x82C022D0..0x82C022F4)
	// 82C022D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C022D4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C022D8: 4BFFFFD1  bl 0x82c022a8
	ctx.lr = 0x82C022DC;
	sub_82C022A8(ctx, base);
	// 82C022DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C022E0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C022E4: 4BC434CD  bl 0x828457b0
	ctx.lr = 0x82C022E8;
	sub_828457B0(ctx, base);
	// 82C022E8: 897F0011  lbz r11, 0x11(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C022EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C022F0: 419AFFE0  beq cr6, 0x82c022d0
	if ctx.cr[6].eq {
	pc = 0x82C022D0; continue 'dispatch;
	}
	pc = 0x82C022F4; continue 'dispatch;
            }
            0x82C022F4 => {
    //   block [0x82C022F4..0x82C0230C)
	// 82C022F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C022F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C022FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02300: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C02304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02310 size=84
    let mut pc: u32 = 0x82C02310;
    'dispatch: loop {
        match pc {
            0x82C02310 => {
    //   block [0x82C02310..0x82C02364)
	// 82C02310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0231C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02324: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02328: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0232C: 4BFFFF7D  bl 0x82c022a8
	ctx.lr = 0x82C02330;
	sub_82C022A8(ctx, base);
	// 82C02330: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02334: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C02338: 91290004  stw r9, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0233C: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02340: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C02344: 91080000  stw r8, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C02348: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0234C: 90E70008  stw r7, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C02350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0235C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02368 size=136
    let mut pc: u32 = 0x82C02368;
    'dispatch: loop {
        match pc {
            0x82C02368 => {
    //   block [0x82C02368..0x82C02394)
	// 82C02368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02370: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02374: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02378: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0237C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82C02380: 4B6E7E39  bl 0x822ea1b8
	ctx.lr = 0x82C02384;
	sub_822EA1B8(ctx, base);
	// 82C02384: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C02388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0238C: 419A0008  beq cr6, 0x82c02394
	if ctx.cr[6].eq {
	pc = 0x82C02394; continue 'dispatch;
	}
	// 82C02390: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82C02394; continue 'dispatch;
            }
            0x82C02394 => {
    //   block [0x82C02394..0x82C023A0)
	// 82C02394: 35430004  addic. r10, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C02398: 41820008  beq 0x82c023a0
	if ctx.cr[0].eq {
	pc = 0x82C023A0; continue 'dispatch;
	}
	// 82C0239C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82C023A0; continue 'dispatch;
            }
            0x82C023A0 => {
    //   block [0x82C023A0..0x82C023AC)
	// 82C023A0: 35430008  addic. r10, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C023A4: 41820008  beq 0x82c023ac
	if ctx.cr[0].eq {
	pc = 0x82C023AC; continue 'dispatch;
	}
	// 82C023A8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82C023AC; continue 'dispatch;
            }
            0x82C023AC => {
    //   block [0x82C023AC..0x82C023F0)
	// 82C023AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C023B0: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82C023B4: 99430010  stb r10, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82C023B8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C023BC: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 82C023C0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C023C4: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C023C8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C023CC: 91290000  stw r9, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C023D0: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C023D4: 91080008  stw r8, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C023D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C023DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C023E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C023E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C023E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C023EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C023F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C023F0 size=824
    let mut pc: u32 = 0x82C023F0;
    'dispatch: loop {
        match pc {
            0x82C023F0 => {
    //   block [0x82C023F0..0x82C02470)
	// 82C023F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C023F4: 480A7005  bl 0x82ca93f8
	ctx.lr = 0x82C023F8;
	sub_82CA93D0(ctx, base);
	// 82C023F8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C023FC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C02400: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82C02404: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82C02408: 6169FFFE  ori r9, r11, 0xfffe
	ctx.r[9].u64 = ctx.r[11].u64 | 65534;
	// 82C0240C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C02410: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02414: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82C02418: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82C0241C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82C02420: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C02424: 4198005C  blt cr6, 0x82c02480
	if ctx.cr[6].lt {
	pc = 0x82C02480; continue 'dispatch;
	}
	// 82C02428: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C0242C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02430: 388B1690  addi r4, r11, 0x1690
	ctx.r[4].s64 = ctx.r[11].s64 + 5776;
	// 82C02434: 4B6EFB0D  bl 0x822f1f40
	ctx.lr = 0x82C02438;
	sub_822F1F40(ctx, base);
	// 82C02438: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C0243C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C02440: 4B6EF931  bl 0x822f1d70
	ctx.lr = 0x82C02444;
	sub_822F1D70(ctx, base);
	// 82C02444: 4B6EF9DD  bl 0x822f1e20
	ctx.lr = 0x82C02448;
	sub_822F1E20(ctx, base);
	// 82C02448: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82C0244C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C02450: 392A1720  addi r9, r10, 0x1720
	ctx.r[9].s64 = ctx.r[10].s64 + 5920;
	// 82C02454: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82C02458: 4BA79389  bl 0x8267b7e0
	ctx.lr = 0x82C0245C;
	sub_8267B7E0(ctx, base);
	// 82C0245C: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C02460: 2B080010  cmplwi cr6, r8, 0x10
	ctx.cr[6].compare_u32(ctx.r[8].u32, 16 as u32, &mut ctx.xer);
	// 82C02464: 4198000C  blt cr6, 0x82c02470
	if ctx.cr[6].lt {
	pc = 0x82C02470; continue 'dispatch;
	}
	// 82C02468: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C0246C: 4BC43345  bl 0x828457b0
	ctx.lr = 0x82C02470;
	sub_828457B0(ctx, base);
	pc = 0x82C02470; continue 'dispatch;
            }
            0x82C02470 => {
    //   block [0x82C02470..0x82C02480)
	// 82C02470: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82C02474: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82C02478: 9B410054  stb r26, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u8 ) };
	// 82C0247C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	pc = 0x82C02480; continue 'dispatch;
            }
            0x82C02480 => {
    //   block [0x82C02480..0x82C024B4)
	// 82C02480: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82C02484: 83DB0004  lwz r30, 4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02488: 4B6E7D31  bl 0x822ea1b8
	ctx.lr = 0x82C0248C;
	sub_822EA1B8(ctx, base);
	// 82C0248C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82C02490: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82C02494: 419A0020  beq cr6, 0x82c024b4
	if ctx.cr[6].eq {
	pc = 0x82C024B4; continue 'dispatch;
	}
	// 82C02498: 93D90000  stw r30, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C0249C: 93F90004  stw r31, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C024A0: 93D90008  stw r30, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C024A4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C024A8: 9179000C  stw r11, 0xc(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C024AC: 9B590010  stb r26, 0x10(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(16 as u32), ctx.r[26].u8 ) };
	// 82C024B0: 9B590011  stb r26, 0x11(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(17 as u32), ctx.r[26].u8 ) };
	pc = 0x82C024B4; continue 'dispatch;
            }
            0x82C024B4 => {
    //   block [0x82C024B4..0x82C024E4)
	// 82C024B4: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C024B8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C024BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C024C0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C024C4: 915B0008  stw r10, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C024C8: 409A001C  bne cr6, 0x82c024e4
	if !ctx.cr[6].eq {
	pc = 0x82C024E4; continue 'dispatch;
	}
	// 82C024CC: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82C024D0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C024D4: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C024D8: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C024DC: 932A0008  stw r25, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82C024E0: 48000044  b 0x82c02524
	pc = 0x82C02524; continue 'dispatch;
            }
            0x82C024E4 => {
    //   block [0x82C024E4..0x82C0250C)
	// 82C024E4: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82C024E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C024EC: 419A0020  beq cr6, 0x82c0250c
	if ctx.cr[6].eq {
	pc = 0x82C0250C; continue 'dispatch;
	}
	// 82C024F0: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C024F4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C024F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C024FC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C02500: 409A0024  bne cr6, 0x82c02524
	if !ctx.cr[6].eq {
	pc = 0x82C02524; continue 'dispatch;
	}
	// 82C02504: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C02508: 4800001C  b 0x82c02524
	pc = 0x82C02524; continue 'dispatch;
            }
            0x82C0250C => {
    //   block [0x82C0250C..0x82C02524)
	// 82C0250C: 933F0008  stw r25, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82C02510: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02514: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02518: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0251C: 409A0008  bne cr6, 0x82c02524
	if !ctx.cr[6].eq {
	pc = 0x82C02524; continue 'dispatch;
	}
	// 82C02520: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	pc = 0x82C02524; continue 'dispatch;
            }
            0x82C02524 => {
    //   block [0x82C02524..0x82C02540)
	// 82C02524: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02528: 39790004  addi r11, r25, 4
	ctx.r[11].s64 = ctx.r[25].s64 + 4;
	// 82C0252C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82C02530: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82C02534: 892A0010  lbz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C02538: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0253C: 409A01CC  bne cr6, 0x82c02708
	if !ctx.cr[6].eq {
	pc = 0x82C02708; continue 'dispatch;
	}
	pc = 0x82C02540; continue 'dispatch;
            }
            0x82C02540 => {
    //   block [0x82C02540..0x82C02588)
	// 82C02540: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02544: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02548: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0254C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C02550: 409A00D8  bne cr6, 0x82c02628
	if !ctx.cr[6].eq {
	pc = 0x82C02628; continue 'dispatch;
	}
	// 82C02554: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02558: 892A0010  lbz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0255C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C02560: 409A0028  bne cr6, 0x82c02588
	if !ctx.cr[6].eq {
	pc = 0x82C02588; continue 'dispatch;
	}
	// 82C02564: 5489003E  slwi r9, r4, 0
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C02568: 9BC90010  stb r30, 0x10(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C0256C: 9BCA0010  stb r30, 0x10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C02570: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02574: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02578: 9B470010  stb r26, 0x10(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[26].u8 ) };
	// 82C0257C: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02580: 83E60004  lwz r31, 4(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02584: 48000170  b 0x82c026f4
	pc = 0x82C026F4; continue 'dispatch;
            }
            0x82C02588 => {
    //   block [0x82C02588..0x82C025A0)
	// 82C02588: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0258C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C02590: 409A0010  bne cr6, 0x82c025a0
	if !ctx.cr[6].eq {
	pc = 0x82C025A0; continue 'dispatch;
	}
	// 82C02594: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C02598: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C0259C: 4BE185CD  bl 0x82a1ab68
	ctx.lr = 0x82C025A0;
	sub_82A1AB68(ctx, base);
	pc = 0x82C025A0; continue 'dispatch;
            }
            0x82C025A0 => {
    //   block [0x82C025A0..0x82C025DC)
	// 82C025A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025A4: 9BCB0010  stb r30, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C025A8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025AC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025B0: 9B490010  stb r26, 0x10(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[26].u8 ) };
	// 82C025B4: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025B8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C025C0: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C025C4: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C025C8: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C025CC: 88C90011  lbz r6, 0x11(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C025D0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C025D4: 409A0008  bne cr6, 0x82c025dc
	if !ctx.cr[6].eq {
	pc = 0x82C025DC; continue 'dispatch;
	}
	// 82C025D8: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82C025DC; continue 'dispatch;
            }
            0x82C025DC => {
    //   block [0x82C025DC..0x82C02600)
	// 82C025DC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025E0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C025E4: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025E8: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C025EC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C025F0: 409A0010  bne cr6, 0x82c02600
	if !ctx.cr[6].eq {
	pc = 0x82C02600; continue 'dispatch;
	}
	// 82C025F4: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C025F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C025FC: 480000F4  b 0x82c026f0
	pc = 0x82C026F0; continue 'dispatch;
            }
            0x82C02600 => {
    //   block [0x82C02600..0x82C0261C)
	// 82C02600: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02604: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02608: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C0260C: 409A0010  bne cr6, 0x82c0261c
	if !ctx.cr[6].eq {
	pc = 0x82C0261C; continue 'dispatch;
	}
	// 82C02610: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C02614: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C02618: 480000D8  b 0x82c026f0
	pc = 0x82C026F0; continue 'dispatch;
            }
            0x82C0261C => {
    //   block [0x82C0261C..0x82C02628)
	// 82C0261C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C02620: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C02624: 480000CC  b 0x82c026f0
	pc = 0x82C026F0; continue 'dispatch;
            }
            0x82C02628 => {
    //   block [0x82C02628..0x82C0265C)
	// 82C02628: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0262C: 892A0010  lbz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C02630: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C02634: 409A0028  bne cr6, 0x82c0265c
	if !ctx.cr[6].eq {
	pc = 0x82C0265C; continue 'dispatch;
	}
	// 82C02638: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0263C: 9BC90010  stb r30, 0x10(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C02640: 9BCA0010  stb r30, 0x10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C02644: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02648: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0264C: 9B470010  stb r26, 0x10(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[26].u8 ) };
	// 82C02650: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02654: 83E60004  lwz r31, 4(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02658: 4800009C  b 0x82c026f4
	pc = 0x82C026F4; continue 'dispatch;
            }
            0x82C0265C => {
    //   block [0x82C0265C..0x82C02674)
	// 82C0265C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02660: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C02664: 409A0010  bne cr6, 0x82c02674
	if !ctx.cr[6].eq {
	pc = 0x82C02674; continue 'dispatch;
	}
	// 82C02668: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C0266C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C02670: 4B7113A9  bl 0x82313a18
	ctx.lr = 0x82C02674;
	sub_82313A18(ctx, base);
	pc = 0x82C02674; continue 'dispatch;
            }
            0x82C02674 => {
    //   block [0x82C02674..0x82C026B0)
	// 82C02674: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02678: 9BCB0010  stb r30, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C0267C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02680: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02684: 9B490010  stb r26, 0x10(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[26].u8 ) };
	// 82C02688: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0268C: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02690: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02694: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02698: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C0269C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C026A0: 88C90011  lbz r6, 0x11(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C026A4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C026A8: 409A0008  bne cr6, 0x82c026b0
	if !ctx.cr[6].eq {
	pc = 0x82C026B0; continue 'dispatch;
	}
	// 82C026AC: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82C026B0; continue 'dispatch;
            }
            0x82C026B0 => {
    //   block [0x82C026B0..0x82C026D0)
	// 82C026B0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C026B4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C026B8: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C026BC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C026C0: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C026C4: 409A000C  bne cr6, 0x82c026d0
	if !ctx.cr[6].eq {
	pc = 0x82C026D0; continue 'dispatch;
	}
	// 82C026C8: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C026CC: 48000020  b 0x82c026ec
	pc = 0x82C026EC; continue 'dispatch;
            }
            0x82C026D0 => {
    //   block [0x82C026D0..0x82C026E8)
	// 82C026D0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C026D4: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C026D8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C026DC: 409A000C  bne cr6, 0x82c026e8
	if !ctx.cr[6].eq {
	pc = 0x82C026E8; continue 'dispatch;
	}
	// 82C026E0: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C026E4: 48000008  b 0x82c026ec
	pc = 0x82C026EC; continue 'dispatch;
            }
            0x82C026E8 => {
    //   block [0x82C026E8..0x82C026EC)
	// 82C026E8: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x82C026EC; continue 'dispatch;
            }
            0x82C026EC => {
    //   block [0x82C026EC..0x82C026F0)
	// 82C026EC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82C026F0; continue 'dispatch;
            }
            0x82C026F0 => {
    //   block [0x82C026F0..0x82C026F4)
	// 82C026F0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x82C026F4; continue 'dispatch;
            }
            0x82C026F4 => {
    //   block [0x82C026F4..0x82C02708)
	// 82C026F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C026F8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82C026FC: 892A0010  lbz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C02700: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C02704: 419AFE3C  beq cr6, 0x82c02540
	if ctx.cr[6].eq {
	pc = 0x82C02540; continue 'dispatch;
	}
	pc = 0x82C02708; continue 'dispatch;
            }
            0x82C02708 => {
    //   block [0x82C02708..0x82C02728)
	// 82C02708: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0270C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82C02710: 93380004  stw r25, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82C02714: 93780000  stw r27, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C02718: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0271C: 9BCA0010  stb r30, 0x10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C02720: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C02724: 480A6D24  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C02728 size=316
    let mut pc: u32 = 0x82C02728;
    'dispatch: loop {
        match pc {
            0x82C02728 => {
    //   block [0x82C02728..0x82C02760)
	// 82C02728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0272C: 480A6CD5  bl 0x82ca9400
	ctx.lr = 0x82C02730;
	sub_82CA93D0(ctx, base);
	// 82C02730: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02734: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C02738: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82C0273C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C02740: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C02744: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82C02748: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0274C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02750: 894B0011  lbz r10, 0x11(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C02754: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C02758: 409A003C  bne cr6, 0x82c02794
	if !ctx.cr[6].eq {
	pc = 0x82C02794; continue 'dispatch;
	}
	// 82C0275C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C02760; continue 'dispatch;
            }
            0x82C02760 => {
    //   block [0x82C02760..0x82C02784)
	// 82C02760: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C02764: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C02768: 7D095010  subfc r8, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C0276C: 7CE84110  subfe r7, r8, r8
	let x = (!ctx.r[8].u32);
	let y = ctx.r[8].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[7].u32 = res;
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C02770: 54FD07FE  clrlwi r29, r7, 0x1f
	ctx.r[29].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 82C02774: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C02778: 419A000C  beq cr6, 0x82c02784
	if ctx.cr[6].eq {
	pc = 0x82C02784; continue 'dispatch;
	}
	// 82C0277C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02780: 48000008  b 0x82c02788
	pc = 0x82C02788; continue 'dispatch;
            }
            0x82C02784 => {
    //   block [0x82C02784..0x82C02788)
	// 82C02784: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82C02788; continue 'dispatch;
            }
            0x82C02788 => {
    //   block [0x82C02788..0x82C02794)
	// 82C02788: 892B0011  lbz r9, 0x11(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C0278C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C02790: 419AFFD0  beq cr6, 0x82c02760
	if ctx.cr[6].eq {
	pc = 0x82C02760; continue 'dispatch;
	}
	pc = 0x82C02794; continue 'dispatch;
            }
            0x82C02794 => {
    //   block [0x82C02794..0x82C027F4)
	// 82C02794: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82C02798: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82C0279C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82C027A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C027A4: 419A0054  beq cr6, 0x82c027f8
	if ctx.cr[6].eq {
	pc = 0x82C027F8; continue 'dispatch;
	}
	// 82C027A8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C027AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C027B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C027B4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C027B8: 409A003C  bne cr6, 0x82c027f4
	if !ctx.cr[6].eq {
	pc = 0x82C027F4; continue 'dispatch;
	}
	// 82C027BC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C027C0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82C027C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C027C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C027CC: 4BFFFC25  bl 0x82c023f0
	ctx.lr = 0x82C027D0;
	sub_82C023F0(ctx, base);
	// 82C027D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C027D4: 9B5E0008  stb r26, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[26].u8 ) };
	// 82C027D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C027DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C027E0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C027E4: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C027E8: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C027EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C027F0: 480A6C60  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C027F4 => {
    //   block [0x82C027F4..0x82C027F8)
	// 82C027F4: 4B853EF5  bl 0x824566e8
	ctx.lr = 0x82C027F8;
	sub_824566E8(ctx, base);
	pc = 0x82C027F8; continue 'dispatch;
            }
            0x82C027F8 => {
    //   block [0x82C027F8..0x82C02848)
	// 82C027F8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C027FC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02800: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C02804: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C02808: 40980040  bge cr6, 0x82c02848
	if !ctx.cr[6].lt {
	pc = 0x82C02848; continue 'dispatch;
	}
	// 82C0280C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C02810: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82C02814: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C02818: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C0281C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02820: 4BFFFBD1  bl 0x82c023f0
	ctx.lr = 0x82C02824;
	sub_82C023F0(ctx, base);
	// 82C02824: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C02828: 9B5E0008  stb r26, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[26].u8 ) };
	// 82C0282C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C02830: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02834: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02838: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0283C: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C02840: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C02844: 480A6C0C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C02848 => {
    //   block [0x82C02848..0x82C02864)
	// 82C02848: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C0284C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C02850: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C02854: 995E0008  stb r10, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82C02858: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C0285C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C02860: 480A6BF0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02868 size=100
    let mut pc: u32 = 0x82C02868;
    'dispatch: loop {
        match pc {
            0x82C02868 => {
    //   block [0x82C02868..0x82C028CC)
	// 82C02868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0286C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C02874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0287C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02880: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C02884: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C02888: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C0288C: 38890CA0  addi r4, r9, 0xca0
	ctx.r[4].s64 = ctx.r[9].s64 + 3232;
	// 82C02890: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02894: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C02898: 514B0F7E  rlwimi r11, r10, 1, 0x1d, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(1) as u64) & 0x0000000000000007) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFF8);
	// 82C0289C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C028A0: 4B672B99  bl 0x82275438
	ctx.lr = 0x82C028A4;
	sub_82275438(ctx, base);
	// 82C028A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C028A8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C028AC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C028B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C028B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C028B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C028BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C028C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C028C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C028C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C028D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C028D0 size=24
    let mut pc: u32 = 0x82C028D0;
    'dispatch: loop {
        match pc {
            0x82C028D0 => {
    //   block [0x82C028D0..0x82C028E8)
	// 82C028D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C028D4: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82C028D8: 616A0004  ori r10, r11, 4
	ctx.r[10].u64 = ctx.r[11].u64 | 4;
	// 82C028DC: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82C028E0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C028E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C028E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C028E8 size=20
    let mut pc: u32 = 0x82C028E8;
    'dispatch: loop {
        match pc {
            0x82C028E8 => {
    //   block [0x82C028E8..0x82C028FC)
	// 82C028E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C028EC: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82C028F0: 616A0008  ori r10, r11, 8
	ctx.r[10].u64 = ctx.r[11].u64 | 8;
	// 82C028F4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C028F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C02900 size=36
    let mut pc: u32 = 0x82C02900;
    'dispatch: loop {
        match pc {
            0x82C02900 => {
    //   block [0x82C02900..0x82C02924)
	// 82C02900: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C02904: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02908: 61490040  ori r9, r10, 0x40
	ctx.r[9].u64 = ctx.r[10].u64 | 64;
	// 82C0290C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C02910: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C02914: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C02918: 4098000C  bge cr6, 0x82c02924
	if !ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x82C02924);
		return;
	}
	// 82C0291C: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82C02920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C02940 size=20
    let mut pc: u32 = 0x82C02940;
    'dispatch: loop {
        match pc {
            0x82C02940 => {
    //   block [0x82C02940..0x82C02954)
	// 82C02940: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02944: D0230034  stfs f1, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82C02948: 616A0080  ori r10, r11, 0x80
	ctx.r[10].u64 = ctx.r[11].u64 | 128;
	// 82C0294C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C02950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02958 size=32
    let mut pc: u32 = 0x82C02958;
    'dispatch: loop {
        match pc {
            0x82C02958 => {
    //   block [0x82C02958..0x82C02970)
	// 82C02958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0295C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C02960: 90830038  stw r4, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 82C02964: 616A0100  ori r10, r11, 0x100
	ctx.r[10].u64 = ctx.r[11].u64 | 256;
	// 82C02968: 409A0008  bne cr6, 0x82c02970
	if !ctx.cr[6].eq {
	pc = 0x82C02970; continue 'dispatch;
	}
	// 82C0296C: 556A062C  rlwinm r10, r11, 0, 0x18, 0x16
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	pc = 0x82C02970; continue 'dispatch;
            }
            0x82C02970 => {
    //   block [0x82C02970..0x82C02978)
	// 82C02970: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C02974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02978 size=20
    let mut pc: u32 = 0x82C02978;
    'dispatch: loop {
        match pc {
            0x82C02978 => {
    //   block [0x82C02978..0x82C0298C)
	// 82C02978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0297C: 98830040  stb r4, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[4].u8 ) };
	// 82C02980: 616A0200  ori r10, r11, 0x200
	ctx.r[10].u64 = ctx.r[11].u64 | 512;
	// 82C02984: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C02988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02990 size=20
    let mut pc: u32 = 0x82C02990;
    'dispatch: loop {
        match pc {
            0x82C02990 => {
    //   block [0x82C02990..0x82C029A4)
	// 82C02990: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02994: 98830041  stb r4, 0x41(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(65 as u32), ctx.r[4].u8 ) };
	// 82C02998: 616A0400  ori r10, r11, 0x400
	ctx.r[10].u64 = ctx.r[11].u64 | 1024;
	// 82C0299C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C029A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C029A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C029A8 size=32
    let mut pc: u32 = 0x82C029A8;
    'dispatch: loop {
        match pc {
            0x82C029A8 => {
    //   block [0x82C029A8..0x82C029C0)
	// 82C029A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C029AC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C029B0: 9083003C  stw r4, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 82C029B4: 616A1000  ori r10, r11, 0x1000
	ctx.r[10].u64 = ctx.r[11].u64 | 4096;
	// 82C029B8: 409A0008  bne cr6, 0x82c029c0
	if !ctx.cr[6].eq {
	pc = 0x82C029C0; continue 'dispatch;
	}
	// 82C029BC: 556A0524  rlwinm r10, r11, 0, 0x14, 0x12
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	pc = 0x82C029C0; continue 'dispatch;
            }
            0x82C029C0 => {
    //   block [0x82C029C0..0x82C029C8)
	// 82C029C0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C029C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C029C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C029C8 size=16
    let mut pc: u32 = 0x82C029C8;
    'dispatch: loop {
        match pc {
            0x82C029C8 => {
    //   block [0x82C029C8..0x82C029D8)
	// 82C029C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C029CC: 616A2000  ori r10, r11, 0x2000
	ctx.r[10].u64 = ctx.r[11].u64 | 8192;
	// 82C029D0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C029D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C029D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C029D8 size=16
    let mut pc: u32 = 0x82C029D8;
    'dispatch: loop {
        match pc {
            0x82C029D8 => {
    //   block [0x82C029D8..0x82C029E8)
	// 82C029D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C029DC: 616A4000  ori r10, r11, 0x4000
	ctx.r[10].u64 = ctx.r[11].u64 | 16384;
	// 82C029E0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C029E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C029E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C029E8 size=64
    let mut pc: u32 = 0x82C029E8;
    'dispatch: loop {
        match pc {
            0x82C029E8 => {
    //   block [0x82C029E8..0x82C02A28)
	// 82C029E8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82C029EC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C029F0: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 82C029F4: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 82C029F8: 61470800  ori r7, r10, 0x800
	ctx.r[7].u64 = ctx.r[10].u64 | 2048;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02A28 size=108
    let mut pc: u32 = 0x82C02A28;
    'dispatch: loop {
        match pc {
            0x82C02A28 => {
    //   block [0x82C02A28..0x82C02A94)
	// 82C02A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02A30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C02A34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02A38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02A40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C02A44: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C02A48: 4B5F4DB9  bl 0x821f7800
	ctx.lr = 0x82C02A4C;
	sub_821F7800(ctx, base);
	// 82C02A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02A50: 4B672931  bl 0x82275380
	ctx.lr = 0x82C02A54;
	sub_82275380(ctx, base);
	// 82C02A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C02A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C02A5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02A60: 4B62A471  bl 0x8222ced0
	ctx.lr = 0x82C02A64;
	sub_8222CED0(ctx, base);
	// 82C02A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02A68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C02A6C: 4B6627B5  bl 0x82265220
	ctx.lr = 0x82C02A70;
	sub_82265220(ctx, base);
	// 82C02A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C02A74: 4B612365  bl 0x82214dd8
	ctx.lr = 0x82C02A78;
	sub_82214DD8(ctx, base);
	// 82C02A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02A7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C02A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02A88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C02A8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02A98 size=16
    let mut pc: u32 = 0x82C02A98;
    'dispatch: loop {
        match pc {
            0x82C02A98 => {
    //   block [0x82C02A98..0x82C02AA8)
	// 82C02A98: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C02A9C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02AA0: 388B4BAC  addi r4, r11, 0x4bac
	ctx.r[4].s64 = ctx.r[11].s64 + 19372;
	// 82C02AA4: 4B6626FC  b 0x822651a0
	sub_822651A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02AA8 size=60
    let mut pc: u32 = 0x82C02AA8;
    'dispatch: loop {
        match pc {
            0x82C02AA8 => {
    //   block [0x82C02AA8..0x82C02AE4)
	// 82C02AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02AB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02AB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02AB8: 3FE0832D  lis r31, -0x7cd3
	ctx.r[31].s64 = -2094202880;
	// 82C02ABC: 807F611C  lwz r3, 0x611c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24860 as u32) ) } as u64;
	// 82C02AC0: 480017C1  bl 0x82c04280
	ctx.lr = 0x82C02AC4;
	sub_82C04280(ctx, base);
	// 82C02AC4: 807F611C  lwz r3, 0x611c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24860 as u32) ) } as u64;
	// 82C02AC8: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82C02ACC: 917F611C  stw r11, 0x611c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24860 as u32), ctx.r[11].u32 ) };
	// 82C02AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02AE8 size=16
    let mut pc: u32 = 0x82C02AE8;
    'dispatch: loop {
        match pc {
            0x82C02AE8 => {
    //   block [0x82C02AE8..0x82C02AF8)
	// 82C02AE8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02AEC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C02AF0: 480018B0  b 0x82c043a0
	sub_82C043A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02AF8 size=60
    let mut pc: u32 = 0x82C02AF8;
    'dispatch: loop {
        match pc {
            0x82C02AF8 => {
    //   block [0x82C02AF8..0x82C02B34)
	// 82C02AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02B00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02B04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02B08: 3FE0832D  lis r31, -0x7cd3
	ctx.r[31].s64 = -2094202880;
	// 82C02B0C: 807F6120  lwz r3, 0x6120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24864 as u32) ) } as u64;
	// 82C02B10: 48001929  bl 0x82c04438
	ctx.lr = 0x82C02B14;
	sub_82C04438(ctx, base);
	// 82C02B14: 807F6120  lwz r3, 0x6120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24864 as u32) ) } as u64;
	// 82C02B18: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82C02B1C: 917F6120  stw r11, 0x6120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24864 as u32), ctx.r[11].u32 ) };
	// 82C02B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02B2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B38 size=8
    let mut pc: u32 = 0x82C02B38;
    'dispatch: loop {
        match pc {
            0x82C02B38 => {
    //   block [0x82C02B38..0x82C02B40)
	// 82C02B38: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02B3C: 480019C4  b 0x82c04500
	sub_82C04500(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B40 size=8
    let mut pc: u32 = 0x82C02B40;
    'dispatch: loop {
        match pc {
            0x82C02B40 => {
    //   block [0x82C02B40..0x82C02B48)
	// 82C02B40: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02B44: 480011BC  b 0x82c03d00
	sub_82C03D00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B48 size=16
    let mut pc: u32 = 0x82C02B48;
    'dispatch: loop {
        match pc {
            0x82C02B48 => {
    //   block [0x82C02B48..0x82C02B58)
	// 82C02B48: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02B4C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C02B50: 48001A18  b 0x82c04568
	sub_82C04568(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B58 size=16
    let mut pc: u32 = 0x82C02B58;
    'dispatch: loop {
        match pc {
            0x82C02B58 => {
    //   block [0x82C02B58..0x82C02B68)
	// 82C02B58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02B5C: 48001D54  b 0x82c048b0
	sub_82C048B0(ctx, base);
	return;
	// 82C02B60: 48002058  b 0x82c04bb8
	sub_82C04BB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B68 size=8
    let mut pc: u32 = 0x82C02B68;
    'dispatch: loop {
        match pc {
            0x82C02B68 => {
    //   block [0x82C02B68..0x82C02B70)
	// 82C02B68: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C02B6C: 48001A54  b 0x82c045c0
	sub_82C045C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B70 size=16
    let mut pc: u32 = 0x82C02B70;
    'dispatch: loop {
        match pc {
            0x82C02B70 => {
    //   block [0x82C02B70..0x82C02B80)
	// 82C02B70: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82C02B74: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02B78: 48001AC0  b 0x82c04638
	sub_82C04638(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B80 size=8
    let mut pc: u32 = 0x82C02B80;
    'dispatch: loop {
        match pc {
            0x82C02B80 => {
    //   block [0x82C02B80..0x82C02B88)
	// 82C02B80: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C02B84: 48001B1C  b 0x82c046a0
	sub_82C046A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02B88 size=16
    let mut pc: u32 = 0x82C02B88;
    'dispatch: loop {
        match pc {
            0x82C02B88 => {
    //   block [0x82C02B88..0x82C02B98)
	// 82C02B88: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C02B8C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C02B90: 48001B88  b 0x82c04718
	sub_82C04718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02B98 size=60
    let mut pc: u32 = 0x82C02B98;
    'dispatch: loop {
        match pc {
            0x82C02B98 => {
    //   block [0x82C02B98..0x82C02BD4)
	// 82C02B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02BA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02BA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02BA8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C02BAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C02BB0: 386B4BBC  addi r3, r11, 0x4bbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19388;
	// 82C02BB4: 4B672885  bl 0x82275438
	ctx.lr = 0x82C02BB8;
	sub_82275438(ctx, base);
	// 82C02BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02BBC: 48001BC5  bl 0x82c04780
	ctx.lr = 0x82C02BC0;
	sub_82C04780(ctx, base);
	// 82C02BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02BD8 size=84
    let mut pc: u32 = 0x82C02BD8;
    'dispatch: loop {
        match pc {
            0x82C02BD8 => {
    //   block [0x82C02BD8..0x82C02C20)
	// 82C02BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02BDC: 480A682D  bl 0x82ca9408
	ctx.lr = 0x82C02BE0;
	sub_82CA93D0(ctx, base);
	// 82C02BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02BE4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C02BE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02BEC: 3BAB4BC0  addi r29, r11, 0x4bc0
	ctx.r[29].s64 = ctx.r[11].s64 + 19392;
	// 82C02BF0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C02BF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C02BF8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C02BFC: 4B5ED645  bl 0x821f0240
	ctx.lr = 0x82C02C00;
	sub_821F0240(ctx, base);
	// 82C02C00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C02C04: 48001C3D  bl 0x82c04840
	ctx.lr = 0x82C02C08;
	sub_82C04840(ctx, base);
	// 82C02C08: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82C02C0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02C10: 419A0010  beq cr6, 0x82c02c20
	if ctx.cr[6].eq {
	pc = 0x82C02C20; continue 'dispatch;
	}
	// 82C02C14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C02C18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C02C1C: 4B67281D  bl 0x82275438
	ctx.lr = 0x82C02C20;
	sub_82275438(ctx, base);
	pc = 0x82C02C20; continue 'dispatch;
            }
            0x82C02C20 => {
    //   block [0x82C02C20..0x82C02C2C)
	// 82C02C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C02C28: 480A6830  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02C30 size=12
    let mut pc: u32 = 0x82C02C30;
    'dispatch: loop {
        match pc {
            0x82C02C30 => {
    //   block [0x82C02C30..0x82C02C3C)
	// 82C02C30: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82C02C34: 386B6124  addi r3, r11, 0x6124
	ctx.r[3].s64 = ctx.r[11].s64 + 24868;
	// 82C02C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02C40 size=108
    let mut pc: u32 = 0x82C02C40;
    'dispatch: loop {
        match pc {
            0x82C02C40 => {
    //   block [0x82C02C40..0x82C02CAC)
	// 82C02C40: 39000032  li r8, 0x32
	ctx.r[8].s64 = 50;
	// 82C02C44: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82C02C48: 3CC00000  lis r6, 0
	ctx.r[6].s64 = 0;
	// 82C02C4C: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C02C50: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C02C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82C02C58: 60C48000  ori r4, r6, 0x8000
	ctx.r[4].u64 = ctx.r[6].u64 | 32768;
	// 82C02C5C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C02C60: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82C02C64: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 82C02C68: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82C02C6C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C02C70: 38A06000  li r5, 0x6000
	ctx.r[5].s64 = 24576;
	// 82C02C74: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82C02C78: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82C02C7C: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82C02C80: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82C02C84: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82C02C88: 3CC00004  lis r6, 4
	ctx.r[6].s64 = 262144;
	// 82C02C8C: 91230024  stw r9, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 82C02C90: 9103002C  stw r8, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 82C02C94: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82C02C98: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82C02C9C: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82C02CA0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82C02CA4: 98E30038  stb r7, 0x38(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[7].u8 ) };
	// 82C02CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C02CB0 size=28
    let mut pc: u32 = 0x82C02CB0;
    'dispatch: loop {
        match pc {
            0x82C02CB0 => {
    //   block [0x82C02CB0..0x82C02CCC)
	// 82C02CB0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02CB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C02CB8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C02CBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02CC0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02CC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C02CC8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02CD0 size=144
    let mut pc: u32 = 0x82C02CD0;
    'dispatch: loop {
        match pc {
            0x82C02CD0 => {
    //   block [0x82C02CD0..0x82C02D10)
	// 82C02CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02CDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02CE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C02CE4: 409A002C  bne cr6, 0x82c02d10
	if !ctx.cr[6].eq {
	pc = 0x82C02D10; continue 'dispatch;
	}
	// 82C02CE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C02CEC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C02CF0: 388BA094  addi r4, r11, -0x5f6c
	ctx.r[4].s64 = ctx.r[11].s64 + -24428;
	// 82C02CF4: 386A4BAC  addi r3, r10, 0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + 19372;
	// 82C02CF8: 4B672741  bl 0x82275438
	ctx.lr = 0x82C02CFC;
	sub_82275438(ctx, base);
	// 82C02CFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C02D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02D0C: 4E800020  blr
	return;
            }
            0x82C02D10 => {
    //   block [0x82C02D10..0x82C02D24)
	// 82C02D10: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02D14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02D18: 409A000C  bne cr6, 0x82c02d24
	if !ctx.cr[6].eq {
	pc = 0x82C02D24; continue 'dispatch;
	}
	// 82C02D1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C02D20: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82C02D24; continue 'dispatch;
            }
            0x82C02D24 => {
    //   block [0x82C02D24..0x82C02D38)
	// 82C02D24: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02D28: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82C02D2C: 4198000C  blt cr6, 0x82c02d38
	if ctx.cr[6].lt {
	pc = 0x82C02D38; continue 'dispatch;
	}
	// 82C02D30: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82C02D34: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82C02D38; continue 'dispatch;
            }
            0x82C02D38 => {
    //   block [0x82C02D38..0x82C02D4C)
	// 82C02D38: 89630038  lbz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C02D3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02D40: 419A000C  beq cr6, 0x82c02d4c
	if ctx.cr[6].eq {
	pc = 0x82C02D4C; continue 'dispatch;
	}
	// 82C02D44: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82C02D48: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	pc = 0x82C02D4C; continue 'dispatch;
            }
            0x82C02D4C => {
    //   block [0x82C02D4C..0x82C02D60)
	// 82C02D4C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C02D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02D60 size=172
    let mut pc: u32 = 0x82C02D60;
    'dispatch: loop {
        match pc {
            0x82C02D60 => {
    //   block [0x82C02D60..0x82C02E0C)
	// 82C02D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C02D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02D78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C02D7C: 4B927C9D  bl 0x8252aa18
	ctx.lr = 0x82C02D80;
	sub_8252AA18(ctx, base);
	// 82C02D80: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02D84: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C02D88: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C02D8C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C02D90: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C02D94: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C02D98: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C02D9C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82C02DA0: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C02DA4: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82C02DA8: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C02DAC: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82C02DB0: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C02DB4: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82C02DB8: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C02DBC: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82C02DC0: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C02DC4: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82C02DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02DCC: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C02DD0: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C02DD4: 815E002C  lwz r10, 0x2c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C02DD8: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82C02DDC: 813E0030  lwz r9, 0x30(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C02DE0: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82C02DE4: 811E0034  lwz r8, 0x34(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C02DE8: 911F0034  stw r8, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 82C02DEC: 88FE0038  lbz r7, 0x38(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C02DF0: 98FF0038  stb r7, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[7].u8 ) };
	// 82C02DF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C02DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02E00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C02E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02E10 size=148
    let mut pc: u32 = 0x82C02E10;
    'dispatch: loop {
        match pc {
            0x82C02E10 => {
    //   block [0x82C02E10..0x82C02E84)
	// 82C02E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02E18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C02E1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02E28: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C02E2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02E30: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C02E34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C02E38: 4E800421  bctrl
	ctx.lr = 0x82C02E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C02E3C: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C02E40: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C02E44: 419A0040  beq cr6, 0x82c02e84
	if ctx.cr[6].eq {
	pc = 0x82C02E84; continue 'dispatch;
	}
	// 82C02E48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C02E4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C02E50: 48003259  bl 0x82c060a8
	ctx.lr = 0x82C02E54;
	sub_82C060A8(ctx, base);
	// 82C02E54: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C02E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02E5C: 419A0028  beq cr6, 0x82c02e84
	if ctx.cr[6].eq {
	pc = 0x82C02E84; continue 'dispatch;
	}
	// 82C02E60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C02E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02E68: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C02E6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C02E70: 4E800421  bctrl
	ctx.lr = 0x82C02E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C02E74: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C02E78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C02E7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C02E80: 409A0008  bne cr6, 0x82c02e88
	if !ctx.cr[6].eq {
	pc = 0x82C02E88; continue 'dispatch;
	}
            }
            0x82C02E84 => {
    //   block [0x82C02E84..0x82C02E88)
	// 82C02E84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82C02E88; continue 'dispatch;
            }
            0x82C02E88 => {
    //   block [0x82C02E88..0x82C02EA4)
	// 82C02E88: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C02E8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C02E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02E98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C02E9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02EA8 size=48
    let mut pc: u32 = 0x82C02EA8;
    'dispatch: loop {
        match pc {
            0x82C02EA8 => {
    //   block [0x82C02EA8..0x82C02ED8)
	// 82C02EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02EB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02EB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02EB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02EBC: 48003BAD  bl 0x82c06a68
	ctx.lr = 0x82C02EC0;
	sub_82C06A68(ctx, base);
	// 82C02EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02ED8 size=48
    let mut pc: u32 = 0x82C02ED8;
    'dispatch: loop {
        match pc {
            0x82C02ED8 => {
    //   block [0x82C02ED8..0x82C02F08)
	// 82C02ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02EE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C02EEC: 48003C1D  bl 0x82c06b08
	ctx.lr = 0x82C02EF0;
	sub_82C06B08(ctx, base);
	// 82C02EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C02EF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C02EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02F00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02F08 size=196
    let mut pc: u32 = 0x82C02F08;
    'dispatch: loop {
        match pc {
            0x82C02F08 => {
    //   block [0x82C02F08..0x82C02F34)
	// 82C02F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C02F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C02F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C02F18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02F1C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C02F20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C02F24: 3BEB4BA4  addi r31, r11, 0x4ba4
	ctx.r[31].s64 = ctx.r[11].s64 + 19364;
	// 82C02F28: 896B4BA4  lbz r11, 0x4ba4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19364 as u32) ) } as u64;
	// 82C02F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02F30: 419A000C  beq cr6, 0x82c02f3c
	if ctx.cr[6].eq {
	pc = 0x82C02F3C; continue 'dispatch;
	}
	pc = 0x82C02F34; continue 'dispatch;
            }
            0x82C02F34 => {
    //   block [0x82C02F34..0x82C02F3C)
	// 82C02F34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C02F38: 4800007C  b 0x82c02fb4
	pc = 0x82C02FB4; continue 'dispatch;
            }
            0x82C02F3C => {
    //   block [0x82C02F3C..0x82C02F8C)
	// 82C02F3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C02F40: 4BFFFD91  bl 0x82c02cd0
	ctx.lr = 0x82C02F44;
	sub_82C02CD0(ctx, base);
	// 82C02F44: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C02F48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C02F4C: 419AFFE8  beq cr6, 0x82c02f34
	if ctx.cr[6].eq {
	pc = 0x82C02F34; continue 'dispatch;
	}
	// 82C02F50: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82C02F54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C02F58: 386B6124  addi r3, r11, 0x6124
	ctx.r[3].s64 = ctx.r[11].s64 + 24868;
	// 82C02F5C: 4BFFFE05  bl 0x82c02d60
	ctx.lr = 0x82C02F60;
	sub_82C02D60(ctx, base);
	// 82C02F60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C02F64: 48000C85  bl 0x82c03be8
	ctx.lr = 0x82C02F68;
	sub_82C03BE8(ctx, base);
	// 82C02F68: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C02F6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C02F70: 419AFFC4  beq cr6, 0x82c02f34
	if ctx.cr[6].eq {
	pc = 0x82C02F34; continue 'dispatch;
	}
	// 82C02F74: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82C02F78: 4B61C2E1  bl 0x8221f258
	ctx.lr = 0x82C02F7C;
	sub_8221F258(ctx, base);
	// 82C02F7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C02F80: 419A000C  beq cr6, 0x82c02f8c
	if ctx.cr[6].eq {
	pc = 0x82C02F8C; continue 'dispatch;
	}
	// 82C02F84: 48003355  bl 0x82c062d8
	ctx.lr = 0x82C02F88;
	sub_82C062D8(ctx, base);
	// 82C02F88: 48000008  b 0x82c02f90
	pc = 0x82C02F90; continue 'dispatch;
            }
            0x82C02F8C => {
    //   block [0x82C02F8C..0x82C02F90)
	// 82C02F8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82C02F90; continue 'dispatch;
            }
            0x82C02F90 => {
    //   block [0x82C02F90..0x82C02FB4)
	// 82C02F90: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C02F94: 48003BF5  bl 0x82c06b88
	ctx.lr = 0x82C02F98;
	sub_82C06B88(ctx, base);
	// 82C02F98: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C02F9C: 48003D05  bl 0x82c06ca0
	ctx.lr = 0x82C02FA0;
	sub_82C06CA0(ctx, base);
	// 82C02FA0: 48003D69  bl 0x82c06d08
	ctx.lr = 0x82C02FA4;
	sub_82C06D08(ctx, base);
	// 82C02FA4: 48003D8D  bl 0x82c06d30
	ctx.lr = 0x82C02FA8;
	sub_82C06D30(ctx, base);
	// 82C02FA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C02FAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C02FB0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	pc = 0x82C02FB4; continue 'dispatch;
            }
            0x82C02FB4 => {
    //   block [0x82C02FB4..0x82C02FCC)
	// 82C02FB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C02FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C02FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C02FC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C02FC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C02FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C02FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C02FD0 size=540
    let mut pc: u32 = 0x82C02FD0;
    'dispatch: loop {
        match pc {
            0x82C02FD0 => {
    //   block [0x82C02FD0..0x82C03008)
	// 82C02FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C02FD4: 480A6435  bl 0x82ca9408
	ctx.lr = 0x82C02FD8;
	sub_82CA93D0(ctx, base);
	// 82C02FD8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C02FDC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C02FE0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C02FE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C02FE8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C02FEC: 419A001C  beq cr6, 0x82c03008
	if ctx.cr[6].eq {
	pc = 0x82C03008; continue 'dispatch;
	}
	// 82C02FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C02FF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C02FF8: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C02FFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C03000: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 82C03004: 48002785  bl 0x82c05788
	ctx.lr = 0x82C03008;
	sub_82C05788(ctx, base);
	pc = 0x82C03008; continue 'dispatch;
            }
            0x82C03008 => {
    //   block [0x82C03008..0x82C03068)
	// 82C03008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C0300C: 48005FDD  bl 0x82c08fe8
	ctx.lr = 0x82C03010;
	sub_82C08FE8(ctx, base);
	// 82C03010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C03014: 3CC00001  lis r6, 1
	ctx.r[6].s64 = 65536;
	// 82C03018: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0301C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C03020: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03024: 4BF4B3CD  bl 0x82b4e3f0
	ctx.lr = 0x82C03028;
	sub_82B4E3F0(ctx, base);
	// 82C03028: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C0302C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C03030: 419A00DC  beq cr6, 0x82c0310c
	if ctx.cr[6].eq {
	pc = 0x82C0310C; continue 'dispatch;
	}
	// 82C03034: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 82C03038: 41980160  blt cr6, 0x82c03198
	if ctx.cr[6].lt {
	pc = 0x82C03198; continue 'dispatch;
	}
	// 82C0303C: 419A012C  beq cr6, 0x82c03168
	if ctx.cr[6].eq {
	pc = 0x82C03168; continue 'dispatch;
	}
	// 82C03040: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 82C03044: 41980024  blt cr6, 0x82c03068
	if ctx.cr[6].lt {
	pc = 0x82C03068; continue 'dispatch;
	}
	// 82C03048: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0304C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C03050: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03054: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03058: 4E800421  bctrl
	ctx.lr = 0x82C0305C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0305C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03060: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C03064: 480A63F4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C03068 => {
    //   block [0x82C03068..0x82C030B8)
	// 82C03068: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0306C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C03070: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C03074: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03078: 4E800421  bctrl
	ctx.lr = 0x82C0307C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0307C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82C03080: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C03084: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C03088: 4BF47E99  bl 0x82b4af20
	ctx.lr = 0x82C0308C;
	sub_82B4AF20(ctx, base);
	// 82C0308C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C03090: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C03094: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03098: 48067769  bl 0x82c6a800
	ctx.lr = 0x82C0309C;
	sub_82C6A800(ctx, base);
	// 82C0309C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C030A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C030A4: 419A0014  beq cr6, 0x82c030b8
	if ctx.cr[6].eq {
	pc = 0x82C030B8; continue 'dispatch;
	}
	// 82C030A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C030AC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C030B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C030B4: 4E800421  bctrl
	ctx.lr = 0x82C030B8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C030B8 => {
    //   block [0x82C030B8..0x82C030F0)
	// 82C030B8: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C030BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C030C0: 419A0030  beq cr6, 0x82c030f0
	if ctx.cr[6].eq {
	pc = 0x82C030F0; continue 'dispatch;
	}
	// 82C030C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C030C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C030CC: 4800632D  bl 0x82c093f8
	ctx.lr = 0x82C030D0;
	sub_82C093F8(ctx, base);
	// 82C030D0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C030D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C030D8: 409A0040  bne cr6, 0x82c03118
	if !ctx.cr[6].eq {
	pc = 0x82C03118; continue 'dispatch;
	}
	// 82C030DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C030E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C030E4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C030E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C030EC: 4E800421  bctrl
	ctx.lr = 0x82C030F0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C030F0 => {
    //   block [0x82C030F0..0x82C0310C)
	// 82C030F0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C030F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C030F8: 419A0014  beq cr6, 0x82c0310c
	if ctx.cr[6].eq {
	pc = 0x82C0310C; continue 'dispatch;
	}
	// 82C030FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03100: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03104: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03108: 4E800421  bctrl
	ctx.lr = 0x82C0310C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C0310C => {
    //   block [0x82C0310C..0x82C03118)
	// 82C0310C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03110: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C03114: 480A6344  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C03118 => {
    //   block [0x82C03118..0x82C0315C)
	// 82C03118: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C0311C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C03120: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C03124: 4BFFFEAD  bl 0x82c02fd0
	ctx.lr = 0x82C03128;
	sub_82C02FD0(ctx, base);
	// 82C03128: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0312C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C03130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C03134: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03138: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0313C: 4E800421  bctrl
	ctx.lr = 0x82C03140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C03140: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C03144: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03148: 419A0014  beq cr6, 0x82c0315c
	if ctx.cr[6].eq {
	pc = 0x82C0315C; continue 'dispatch;
	}
	// 82C0314C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03150: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03154: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03158: 4E800421  bctrl
	ctx.lr = 0x82C0315C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C0315C => {
    //   block [0x82C0315C..0x82C03168)
	// 82C0315C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C03160: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C03164: 480A62F4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C03168 => {
    //   block [0x82C03168..0x82C03198)
	// 82C03168: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C0316C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C03170: 48005D39  bl 0x82c08ea8
	ctx.lr = 0x82C03174;
	sub_82C08EA8(ctx, base);
	// 82C03174: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C03178: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C0317C: 388BA0B8  addi r4, r11, -0x5f48
	ctx.r[4].s64 = ctx.r[11].s64 + -24392;
	// 82C03180: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C03184: 4BFFFC8D  bl 0x82c02e10
	ctx.lr = 0x82C03188;
	sub_82C02E10(ctx, base);
	// 82C03188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0318C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C03190: 48005DF9  bl 0x82c08f88
	ctx.lr = 0x82C03194;
	sub_82C08F88(ctx, base);
	// 82C03194: 48000030  b 0x82c031c4
	pc = 0x82C031C4; continue 'dispatch;
            }
            0x82C03198 => {
    //   block [0x82C03198..0x82C031C4)
	// 82C03198: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C0319C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C031A0: 480056F9  bl 0x82c08898
	ctx.lr = 0x82C031A4;
	sub_82C08898(ctx, base);
	// 82C031A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C031A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C031AC: 388BA0B8  addi r4, r11, -0x5f48
	ctx.r[4].s64 = ctx.r[11].s64 + -24392;
	// 82C031B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C031B4: 4BFFFC5D  bl 0x82c02e10
	ctx.lr = 0x82C031B8;
	sub_82C02E10(ctx, base);
	// 82C031B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C031BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C031C0: 48005981  bl 0x82c08b40
	ctx.lr = 0x82C031C4;
	sub_82C08B40(ctx, base);
	pc = 0x82C031C4; continue 'dispatch;
            }
            0x82C031C4 => {
    //   block [0x82C031C4..0x82C031E0)
	// 82C031C4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C031C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C031CC: 419A0014  beq cr6, 0x82c031e0
	if ctx.cr[6].eq {
	pc = 0x82C031E0; continue 'dispatch;
	}
	// 82C031D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C031D4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C031D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C031DC: 4E800421  bctrl
	ctx.lr = 0x82C031E0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C031E0 => {
    //   block [0x82C031E0..0x82C031EC)
	// 82C031E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C031E4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C031E8: 480A6270  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C031F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C031F0 size=168
    let mut pc: u32 = 0x82C031F0;
    'dispatch: loop {
        match pc {
            0x82C031F0 => {
    //   block [0x82C031F0..0x82C03208)
	// 82C031F0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C031F4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C031F8: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C031FC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C03200: 409A0030  bne cr6, 0x82c03230
	if !ctx.cr[6].eq {
	pc = 0x82C03230; continue 'dispatch;
	}
	// 82C03204: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C03208; continue 'dispatch;
            }
            0x82C03208 => {
    //   block [0x82C03208..0x82C03220)
	// 82C03208: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0320C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C03210: 40980010  bge cr6, 0x82c03220
	if !ctx.cr[6].lt {
	pc = 0x82C03220; continue 'dispatch;
	}
	// 82C03214: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C03218: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0321C: 48000008  b 0x82c03224
	pc = 0x82C03224; continue 'dispatch;
            }
            0x82C03220 => {
    //   block [0x82C03220..0x82C03224)
	// 82C03220: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82C03224; continue 'dispatch;
            }
            0x82C03224 => {
    //   block [0x82C03224..0x82C03230)
	// 82C03224: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C03228: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C0322C: 419AFFDC  beq cr6, 0x82c03208
	if ctx.cr[6].eq {
	pc = 0x82C03208; continue 'dispatch;
	}
	pc = 0x82C03230; continue 'dispatch;
            }
            0x82C03230 => {
    //   block [0x82C03230..0x82C03254)
	// 82C03230: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03234: 9141FFF4  stw r10, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[10].u32 ) };
	// 82C03238: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C0323C: 9081FFF0  stw r4, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[4].u32 ) };
	// 82C03240: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03244: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C03248: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0324C: 409A0030  bne cr6, 0x82c0327c
	if !ctx.cr[6].eq {
	pc = 0x82C0327C; continue 'dispatch;
	}
	// 82C03250: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C03254; continue 'dispatch;
            }
            0x82C03254 => {
    //   block [0x82C03254..0x82C03268)
	// 82C03254: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C03258: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C0325C: 4098000C  bge cr6, 0x82c03268
	if !ctx.cr[6].lt {
	pc = 0x82C03268; continue 'dispatch;
	}
	// 82C03260: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03264: 4800000C  b 0x82c03270
	pc = 0x82C03270; continue 'dispatch;
            }
            0x82C03268 => {
    //   block [0x82C03268..0x82C03270)
	// 82C03268: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C0326C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C03270; continue 'dispatch;
            }
            0x82C03270 => {
    //   block [0x82C03270..0x82C0327C)
	// 82C03270: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C03274: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C03278: 419AFFDC  beq cr6, 0x82c03254
	if ctx.cr[6].eq {
	pc = 0x82C03254; continue 'dispatch;
	}
	pc = 0x82C0327C; continue 'dispatch;
            }
            0x82C0327C => {
    //   block [0x82C0327C..0x82C03298)
	// 82C0327C: E961FFF0  ld r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03280: 9141FFF4  stw r10, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[10].u32 ) };
	// 82C03284: 9081FFF0  stw r4, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[4].u32 ) };
	// 82C03288: E941FFF0  ld r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0328C: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82C03290: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82C03294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03298 size=520
    let mut pc: u32 = 0x82C03298;
    'dispatch: loop {
        match pc {
            0x82C03298 => {
    //   block [0x82C03298..0x82C032E4)
	// 82C03298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0329C: 480A6159  bl 0x82ca93f4
	ctx.lr = 0x82C032A0;
	sub_82CA93D0(ctx, base);
	// 82C032A0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82C032A4: 9421EF00  stwu r1, -0x1100(r1)
	ea = ctx.r[1].u32.wrapping_add(-4352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C032A8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C032AC: 3EE08333  lis r23, -0x7ccd
	ctx.r[23].s64 = -2093809664;
	// 82C032B0: 3B2B4BB0  addi r25, r11, 0x4bb0
	ctx.r[25].s64 = ctx.r[11].s64 + 19376;
	// 82C032B4: 3B800007  li r28, 7
	ctx.r[28].s64 = 7;
	// 82C032B8: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82C032BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C032C0: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82C032C4: 3F60832D  lis r27, -0x7cd3
	ctx.r[27].s64 = -2094202880;
	// 82C032C8: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C032CC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C032D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82C032D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82C032D8: E9210058  ld r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C032DC: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 82C032E0: 3B0BFFDF  addi r24, r11, -0x21
	ctx.r[24].s64 = ctx.r[11].s64 + -33;
	pc = 0x82C032E4; continue 'dispatch;
            }
            0x82C032E4 => {
    //   block [0x82C032E4..0x82C032F8)
	// 82C032E4: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C032E8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C032EC: 419A000C  beq cr6, 0x82c032f8
	if ctx.cr[6].eq {
	pc = 0x82C032F8; continue 'dispatch;
	}
	// 82C032F0: 7F1DC840  cmplw cr6, r29, r25
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82C032F4: 419A0008  beq cr6, 0x82c032fc
	if ctx.cr[6].eq {
	pc = 0x82C032FC; continue 'dispatch;
	}
	pc = 0x82C032F8; continue 'dispatch;
            }
            0x82C032F8 => {
    //   block [0x82C032F8..0x82C032FC)
	// 82C032F8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C032FC; continue 'dispatch;
            }
            0x82C032FC => {
    //   block [0x82C032FC..0x82C03314)
	// 82C032FC: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C03300: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C03304: 419A0148  beq cr6, 0x82c0344c
	if ctx.cr[6].eq {
	pc = 0x82C0344C; continue 'dispatch;
	}
	// 82C03308: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C0330C: 409A0008  bne cr6, 0x82c03314
	if !ctx.cr[6].eq {
	pc = 0x82C03314; continue 'dispatch;
	}
	// 82C03310: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C03314; continue 'dispatch;
            }
            0x82C03314 => {
    //   block [0x82C03314..0x82C03324)
	// 82C03314: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03318: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0331C: 409A0008  bne cr6, 0x82c03324
	if !ctx.cr[6].eq {
	pc = 0x82C03324; continue 'dispatch;
	}
	// 82C03320: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C03324; continue 'dispatch;
            }
            0x82C03324 => {
    //   block [0x82C03324..0x82C03338)
	// 82C03324: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C03328: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82C0332C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03330: 419A0008  beq cr6, 0x82c03338
	if ctx.cr[6].eq {
	pc = 0x82C03338; continue 'dispatch;
	}
	// 82C03334: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C03338; continue 'dispatch;
            }
            0x82C03338 => {
    //   block [0x82C03338..0x82C033B4)
	// 82C03338: 93810088  stw r28, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[28].u32 ) };
	// 82C0333C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82C03340: 93E10084  stw r31, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 82C03344: 38A00801  li r5, 0x801
	ctx.r[5].s64 = 2049;
	// 82C03348: B3E10074  sth r31, 0x74(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u16 ) };
	// 82C0334C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82C03350: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82C03354: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03358: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82C0335C: 4B577B6D  bl 0x8217aec8
	ctx.lr = 0x82C03360;
	sub_8217AEC8(ctx, base);
	// 82C03360: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82C03364: 4B577D1D  bl 0x8217b080
	ctx.lr = 0x82C03368;
	sub_8217B080(ctx, base);
	// 82C03368: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82C0336C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82C03370: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C03374: 4B57789D  bl 0x8217ac10
	ctx.lr = 0x82C03378;
	sub_8217AC10(ctx, base);
	// 82C03378: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C0337C: 4B577BF5  bl 0x8217af70
	ctx.lr = 0x82C03380;
	sub_8217AF70(ctx, base);
	// 82C03380: 809B6124  lwz r4, 0x6124(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24868 as u32) ) } as u64;
	// 82C03384: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C03388: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82C0338C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03390: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03394: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C03398: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0339C: 4E800421  bctrl
	ctx.lr = 0x82C033A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C033A0: 81210088  lwz r9, 0x88(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C033A4: 2B090008  cmplwi cr6, r9, 8
	ctx.cr[6].compare_u32(ctx.r[9].u32, 8 as u32, &mut ctx.xer);
	// 82C033A8: 4198000C  blt cr6, 0x82c033b4
	if ctx.cr[6].lt {
	pc = 0x82C033B4; continue 'dispatch;
	}
	// 82C033AC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82C033B0: 4BC42401  bl 0x828457b0
	ctx.lr = 0x82C033B4;
	sub_828457B0(ctx, base);
            }
            0x82C033B4 => {
    //   block [0x82C033B4..0x82C0340C)
	// 82C033B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C033B8: 93810088  stw r28, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[28].u32 ) };
	// 82C033BC: 93E10084  stw r31, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 82C033C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C033C4: B3E10074  sth r31, 0x74(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u16 ) };
	// 82C033C8: 419A0074  beq cr6, 0x82c0343c
	if ctx.cr[6].eq {
	pc = 0x82C0343C; continue 'dispatch;
	}
	// 82C033CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C033D0: 48002EA1  bl 0x82c06270
	ctx.lr = 0x82C033D4;
	sub_82C06270(ctx, base);
	// 82C033D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C033D8: 80974BA8  lwz r4, 0x4ba8(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(19368 as u32) ) } as u64;
	// 82C033DC: 48002CBD  bl 0x82c06098
	ctx.lr = 0x82C033E0;
	sub_82C06098(ctx, base);
	// 82C033E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C033E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C033E8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C033EC: 4BFFFBE5  bl 0x82c02fd0
	ctx.lr = 0x82C033F0;
	sub_82C02FD0(ctx, base);
	// 82C033F0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C033F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C033F8: 419A0020  beq cr6, 0x82c03418
	if ctx.cr[6].eq {
	pc = 0x82C03418; continue 'dispatch;
	}
	// 82C033FC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03400: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C03404: 409A0008  bne cr6, 0x82c0340c
	if !ctx.cr[6].eq {
	pc = 0x82C0340C; continue 'dispatch;
	}
	// 82C03408: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82C0340C; continue 'dispatch;
            }
            0x82C0340C => {
    //   block [0x82C0340C..0x82C03418)
	// 82C0340C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C03410: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C03414: 4800206D  bl 0x82c05480
	ctx.lr = 0x82C03418;
	sub_82C05480(ctx, base);
	pc = 0x82C03418; continue 'dispatch;
            }
            0x82C03418 => {
    //   block [0x82C03418..0x82C0343C)
	// 82C03418: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C0341C: 48002FA5  bl 0x82c063c0
	ctx.lr = 0x82C03420;
	sub_82C063C0(ctx, base);
	// 82C03420: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C03424: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03428: 419A0014  beq cr6, 0x82c0343c
	if ctx.cr[6].eq {
	pc = 0x82C0343C; continue 'dispatch;
	}
	// 82C0342C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03430: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03434: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03438: 4E800421  bctrl
	ctx.lr = 0x82C0343C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C0343C => {
    //   block [0x82C0343C..0x82C0344C)
	// 82C0343C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C03440: 4B953419  bl 0x82556858
	ctx.lr = 0x82C03444;
	sub_82556858(ctx, base);
	// 82C03444: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03448: 4BFFFE9C  b 0x82c032e4
	pc = 0x82C032E4; continue 'dispatch;
            }
            0x82C0344C => {
    //   block [0x82C0344C..0x82C03474)
	// 82C0344C: 48000735  bl 0x82c03b80
	ctx.lr = 0x82C03450;
	sub_82C03B80(ctx, base);
	// 82C03450: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03454: 816B4BBC  lwz r11, 0x4bbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19388 as u32) ) } as u64;
	// 82C03458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0345C: 419A0018  beq cr6, 0x82c03474
	if ctx.cr[6].eq {
	pc = 0x82C03474; continue 'dispatch;
	}
	// 82C03460: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03464: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C03468: 4099000C  ble cr6, 0x82c03474
	if !ctx.cr[6].gt {
	pc = 0x82C03474; continue 'dispatch;
	}
	// 82C0346C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03470: 48001311  bl 0x82c04780
	ctx.lr = 0x82C03474;
	sub_82C04780(ctx, base);
	pc = 0x82C03474; continue 'dispatch;
            }
            0x82C03474 => {
    //   block [0x82C03474..0x82C03498)
	// 82C03474: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03478: 816B4BC0  lwz r11, 0x4bc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19392 as u32) ) } as u64;
	// 82C0347C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03480: 419A0018  beq cr6, 0x82c03498
	if ctx.cr[6].eq {
	pc = 0x82C03498; continue 'dispatch;
	}
	// 82C03484: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03488: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C0348C: 4099000C  ble cr6, 0x82c03498
	if !ctx.cr[6].gt {
	pc = 0x82C03498; continue 'dispatch;
	}
	// 82C03490: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03494: 480013AD  bl 0x82c04840
	ctx.lr = 0x82C03498;
	sub_82C04840(ctx, base);
	pc = 0x82C03498; continue 'dispatch;
            }
            0x82C03498 => {
    //   block [0x82C03498..0x82C034A0)
	// 82C03498: 38211100  addi r1, r1, 0x1100
	ctx.r[1].s64 = ctx.r[1].s64 + 4352;
	// 82C0349C: 480A5FA8  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C034A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C034A0 size=116
    let mut pc: u32 = 0x82C034A0;
    'dispatch: loop {
        match pc {
            0x82C034A0 => {
    //   block [0x82C034A0..0x82C03514)
	// 82C034A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C034A4: 480A5F65  bl 0x82ca9408
	ctx.lr = 0x82C034A8;
	sub_82CA93D0(ctx, base);
	// 82C034A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C034AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C034B0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C034B4: 93E100BC  stw r31, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[31].u32 ) };
	// 82C034B8: 38A100BC  addi r5, r1, 0xbc
	ctx.r[5].s64 = ctx.r[1].s64 + 188;
	// 82C034BC: 3B8B4BB0  addi r28, r11, 0x4bb0
	ctx.r[28].s64 = ctx.r[11].s64 + 19376;
	// 82C034C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C034C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C034C8: 4BFFFD29  bl 0x82c031f0
	ctx.lr = 0x82C034CC;
	sub_82C031F0(ctx, base);
	// 82C034CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C034D0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82C034D4: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C034D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82C034DC: EBC10068  ld r30, 0x68(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82C034E0: EBA10060  ld r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C034E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C034E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C034EC: 48045E2D  bl 0x82c49318
	ctx.lr = 0x82C034F0;
	sub_82C49318(ctx, base);
	// 82C034F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C034F4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C034F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C034FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C03500: 4B71E9C1  bl 0x82321ec0
	ctx.lr = 0x82C03504;
	sub_82321EC0(ctx, base);
	// 82C03504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C03508: 48000CD1  bl 0x82c041d8
	ctx.lr = 0x82C0350C;
	sub_82C041D8(ctx, base);
	// 82C0350C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C03510: 480A5F48  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03518 size=344
    let mut pc: u32 = 0x82C03518;
    'dispatch: loop {
        match pc {
            0x82C03518 => {
    //   block [0x82C03518..0x82C03560)
	// 82C03518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0351C: 480A5EED  bl 0x82ca9408
	ctx.lr = 0x82C03520;
	sub_82CA93D0(ctx, base);
	// 82C03520: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03524: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03528: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C0352C: 3BCB4BA4  addi r30, r11, 0x4ba4
	ctx.r[30].s64 = ctx.r[11].s64 + 19364;
	// 82C03530: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C03534: 896B4BA4  lbz r11, 0x4ba4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19364 as u32) ) } as u64;
	// 82C03538: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0353C: 409A0024  bne cr6, 0x82c03560
	if !ctx.cr[6].eq {
	pc = 0x82C03560; continue 'dispatch;
	}
	// 82C03540: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C03544: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03548: 388BA07C  addi r4, r11, -0x5f84
	ctx.r[4].s64 = ctx.r[11].s64 + -24452;
	// 82C0354C: 386A4BAC  addi r3, r10, 0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + 19372;
	// 82C03550: 4B671EE9  bl 0x82275438
	ctx.lr = 0x82C03554;
	sub_82275438(ctx, base);
	// 82C03554: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03558: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C0355C: 480A5EFC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C03560 => {
    //   block [0x82C03560..0x82C0359C)
	// 82C03560: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03564: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C03568: 3BAB4BAC  addi r29, r11, 0x4bac
	ctx.r[29].s64 = ctx.r[11].s64 + 19372;
	// 82C0356C: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 82C03570: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C03574: 4B671EC5  bl 0x82275438
	ctx.lr = 0x82C03578;
	sub_82275438(ctx, base);
	// 82C03578: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C0357C: 409A0020  bne cr6, 0x82c0359c
	if !ctx.cr[6].eq {
	pc = 0x82C0359C; continue 'dispatch;
	}
	// 82C03580: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C03584: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C03588: 388BA0C4  addi r4, r11, -0x5f3c
	ctx.r[4].s64 = ctx.r[11].s64 + -24380;
	// 82C0358C: 4B671EAD  bl 0x82275438
	ctx.lr = 0x82C03590;
	sub_82275438(ctx, base);
	// 82C03590: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03594: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C03598: 480A5EC0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C0359C => {
    //   block [0x82C0359C..0x82C035E0)
	// 82C0359C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C035A0: 48002CD1  bl 0x82c06270
	ctx.lr = 0x82C035A4;
	sub_82C06270(ctx, base);
	// 82C035A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C035A8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C035AC: 48002AED  bl 0x82c06098
	ctx.lr = 0x82C035B0;
	sub_82C06098(ctx, base);
	// 82C035B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C035B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C035B8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C035BC: 4BFFFA15  bl 0x82c02fd0
	ctx.lr = 0x82C035C0;
	sub_82C02FD0(ctx, base);
	// 82C035C0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C035C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C035C8: 409A0018  bne cr6, 0x82c035e0
	if !ctx.cr[6].eq {
	pc = 0x82C035E0; continue 'dispatch;
	}
	// 82C035CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C035D0: 48002DF1  bl 0x82c063c0
	ctx.lr = 0x82C035D4;
	sub_82C063C0(ctx, base);
	// 82C035D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C035D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C035DC: 480A5E7C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C035E0 => {
    //   block [0x82C035E0..0x82C035F4)
	// 82C035E0: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C035E4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82C035E8: 4198000C  blt cr6, 0x82c035f4
	if ctx.cr[6].lt {
	pc = 0x82C035F4; continue 'dispatch;
	}
	// 82C035EC: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C035F0: 48000008  b 0x82c035f8
	pc = 0x82C035F8; continue 'dispatch;
            }
            0x82C035F4 => {
    //   block [0x82C035F4..0x82C035F8)
	// 82C035F4: 389C0004  addi r4, r28, 4
	ctx.r[4].s64 = ctx.r[28].s64 + 4;
	pc = 0x82C035F8; continue 'dispatch;
            }
            0x82C035F8 => {
    //   block [0x82C035F8..0x82C03670)
	// 82C035F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C035FC: 4B6D2E0D  bl 0x822d6408
	ctx.lr = 0x82C03600;
	sub_822D6408(ctx, base);
	// 82C03600: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C03604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03608: 4BF3C291  bl 0x82b3f898
	ctx.lr = 0x82C0360C;
	sub_82B3F898(ctx, base);
	// 82C0360C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03610: 3FE0832D  lis r31, -0x7cd3
	ctx.r[31].s64 = -2094202880;
	// 82C03614: 394B4BB0  addi r10, r11, 0x4bb0
	ctx.r[10].s64 = ctx.r[11].s64 + 19376;
	// 82C03618: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0361C: 389F6118  addi r4, r31, 0x6118
	ctx.r[4].s64 = ctx.r[31].s64 + 24856;
	// 82C03620: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82C03624: 4B71DAD5  bl 0x823210f8
	ctx.lr = 0x82C03628;
	sub_823210F8(ctx, base);
	// 82C03628: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C0362C: 4B661B75  bl 0x822651a0
	ctx.lr = 0x82C03630;
	sub_822651A0(ctx, base);
	// 82C03630: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03634: 4B6117A5  bl 0x82214dd8
	ctx.lr = 0x82C03638;
	sub_82214DD8(ctx, base);
	// 82C03638: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C0363C: 4B5C31BD  bl 0x821c67f8
	ctx.lr = 0x82C03640;
	sub_821C67F8(ctx, base);
	// 82C03640: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82C03644: 807F6118  lwz r3, 0x6118(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24856 as u32) ) } as u64;
	// 82C03648: 48001E39  bl 0x82c05480
	ctx.lr = 0x82C0364C;
	sub_82C05480(ctx, base);
	// 82C0364C: 817F6118  lwz r11, 0x6118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24856 as u32) ) } as u64;
	// 82C03650: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C03654: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82C03658: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C0365C: 917F6118  stw r11, 0x6118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24856 as u32), ctx.r[11].u32 ) };
	// 82C03660: 48002D61  bl 0x82c063c0
	ctx.lr = 0x82C03664;
	sub_82C063C0(ctx, base);
	// 82C03664: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C03668: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C0366C: 480A5DEC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03670 size=380
    let mut pc: u32 = 0x82C03670;
    'dispatch: loop {
        match pc {
            0x82C03670 => {
    //   block [0x82C03670..0x82C036B4)
	// 82C03670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03674: 480A5D91  bl 0x82ca9404
	ctx.lr = 0x82C03678;
	sub_82CA93D0(ctx, base);
	// 82C03678: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0367C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03680: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C03684: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C03688: 894B4BA4  lbz r10, 0x4ba4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19364 as u32) ) } as u64;
	// 82C0368C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C03690: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03694: 386A4BAC  addi r3, r10, 0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + 19372;
	// 82C03698: 409A001C  bne cr6, 0x82c036b4
	if !ctx.cr[6].eq {
	pc = 0x82C036B4; continue 'dispatch;
	}
	// 82C0369C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C036A0: 388BA07C  addi r4, r11, -0x5f84
	ctx.r[4].s64 = ctx.r[11].s64 + -24452;
	// 82C036A4: 4B671D95  bl 0x82275438
	ctx.lr = 0x82C036A8;
	sub_82275438(ctx, base);
	// 82C036A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C036AC: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82C036B0: 480A5DA4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C036B4 => {
    //   block [0x82C036B4..0x82C03724)
	// 82C036B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C036B8: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 82C036BC: 4B671D7D  bl 0x82275438
	ctx.lr = 0x82C036C0;
	sub_82275438(ctx, base);
	// 82C036C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C036C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C036C8: 4B5779E1  bl 0x8217b0a8
	ctx.lr = 0x82C036CC;
	sub_8217B0A8(ctx, base);
	// 82C036CC: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 82C036D0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C036D4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C036D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C036DC: 80896124  lwz r4, 0x6124(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24868 as u32) ) } as u64;
	// 82C036E0: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C036E4: 80E80010  lwz r7, 0x10(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C036E8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C036EC: 4E800421  bctrl
	ctx.lr = 0x82C036F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C036F0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82C036F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C036F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C036FC: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03700: 4BF47711  bl 0x82b4ae10
	ctx.lr = 0x82C03704;
	sub_82B4AE10(ctx, base);
	// 82C03704: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03708: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C0370C: 419A0018  beq cr6, 0x82c03724
	if ctx.cr[6].eq {
	pc = 0x82C03724; continue 'dispatch;
	}
	// 82C03710: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03714: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C03718: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0371C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03720: 4E800421  bctrl
	ctx.lr = 0x82C03724;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C03724 => {
    //   block [0x82C03724..0x82C03740)
	// 82C03724: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C03728: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0372C: 419A0014  beq cr6, 0x82c03740
	if ctx.cr[6].eq {
	pc = 0x82C03740; continue 'dispatch;
	}
	// 82C03730: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03734: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03738: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0373C: 4E800421  bctrl
	ctx.lr = 0x82C03740;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C03740 => {
    //   block [0x82C03740..0x82C0375C)
	// 82C03740: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C03744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03748: 419A0014  beq cr6, 0x82c0375c
	if ctx.cr[6].eq {
	pc = 0x82C0375C; continue 'dispatch;
	}
	// 82C0374C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03750: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03754: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03758: 4E800421  bctrl
	ctx.lr = 0x82C0375C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C0375C => {
    //   block [0x82C0375C..0x82C03770)
	// 82C0375C: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C03760: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82C03764: 4198000C  blt cr6, 0x82c03770
	if ctx.cr[6].lt {
	pc = 0x82C03770; continue 'dispatch;
	}
	// 82C03768: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C0376C: 4BC42045  bl 0x828457b0
	ctx.lr = 0x82C03770;
	sub_828457B0(ctx, base);
	pc = 0x82C03770; continue 'dispatch;
            }
            0x82C03770 => {
    //   block [0x82C03770..0x82C037B8)
	// 82C03770: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C03774: 3BA00007  li r29, 7
	ctx.r[29].s64 = 7;
	// 82C03778: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82C0377C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C03780: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82C03784: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82C03788: B3E10064  sth r31, 0x64(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u16 ) };
	// 82C0378C: 4B57791D  bl 0x8217b0a8
	ctx.lr = 0x82C03790;
	sub_8217B0A8(ctx, base);
	// 82C03790: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82C03794: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C03798: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C0379C: 4BFFFD7D  bl 0x82c03518
	ctx.lr = 0x82C037A0;
	sub_82C03518(ctx, base);
	// 82C037A0: 816100A8  lwz r11, 0xa8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 82C037A4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C037A8: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82C037AC: 4198000C  blt cr6, 0x82c037b8
	if ctx.cr[6].lt {
	pc = 0x82C037B8; continue 'dispatch;
	}
	// 82C037B0: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C037B4: 4BC41FFD  bl 0x828457b0
	ctx.lr = 0x82C037B8;
	sub_828457B0(ctx, base);
	pc = 0x82C037B8; continue 'dispatch;
            }
            0x82C037B8 => {
    //   block [0x82C037B8..0x82C037E0)
	// 82C037B8: 93A100A8  stw r29, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 82C037BC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C037C0: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 82C037C4: B3E10094  sth r31, 0x94(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[31].u16 ) };
	// 82C037C8: 419A0018  beq cr6, 0x82c037e0
	if ctx.cr[6].eq {
	pc = 0x82C037E0; continue 'dispatch;
	}
	// 82C037CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C037D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C037D4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C037D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C037DC: 4E800421  bctrl
	ctx.lr = 0x82C037E0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C037E0 => {
    //   block [0x82C037E0..0x82C037EC)
	// 82C037E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C037E4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82C037E8: 480A5C6C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C037F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C037F0 size=108
    let mut pc: u32 = 0x82C037F0;
    'dispatch: loop {
        match pc {
            0x82C037F0 => {
    //   block [0x82C037F0..0x82C03818)
	// 82C037F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C037F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C037F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C037FC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03804: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03808: 409A0010  bne cr6, 0x82c03818
	if !ctx.cr[6].eq {
	pc = 0x82C03818; continue 'dispatch;
	}
	// 82C0380C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82C03810: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 82C03814: 48000008  b 0x82c0381c
	pc = 0x82C0381C; continue 'dispatch;
            }
            0x82C03818 => {
    //   block [0x82C03818..0x82C0381C)
	// 82C03818: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C0381C; continue 'dispatch;
            }
            0x82C0381C => {
    //   block [0x82C0381C..0x82C03844)
	// 82C0381C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03820: 4B577889  bl 0x8217b0a8
	ctx.lr = 0x82C03824;
	sub_8217B0A8(ctx, base);
	// 82C03824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03828: 4B6C5089  bl 0x822c88b0
	ctx.lr = 0x82C0382C;
	sub_822C88B0(ctx, base);
	// 82C0382C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C03830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C03834: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82C03838: 4198000C  blt cr6, 0x82c03844
	if ctx.cr[6].lt {
	pc = 0x82C03844; continue 'dispatch;
	}
	// 82C0383C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C03840: 4BC41F71  bl 0x828457b0
	ctx.lr = 0x82C03844;
	sub_828457B0(ctx, base);
	pc = 0x82C03844; continue 'dispatch;
            }
            0x82C03844 => {
    //   block [0x82C03844..0x82C0385C)
	// 82C03844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C03848: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C0384C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03860 size=96
    let mut pc: u32 = 0x82C03860;
    'dispatch: loop {
        match pc {
            0x82C03860 => {
    //   block [0x82C03860..0x82C03898)
	// 82C03860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0386C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03870: 814B4BCC  lwz r10, 0x4bcc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19404 as u32) ) } as u64;
	// 82C03874: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82C03878: 409A0020  bne cr6, 0x82c03898
	if !ctx.cr[6].eq {
	pc = 0x82C03898; continue 'dispatch;
	}
	// 82C0387C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03880: 386B4BD8  addi r3, r11, 0x4bd8
	ctx.r[3].s64 = ctx.r[11].s64 + 19416;
	// 82C03884: 4800666D  bl 0x82c09ef0
	ctx.lr = 0x82C03888;
	sub_82C09EF0(ctx, base);
	// 82C03888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0388C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03894: 4E800020  blr
	return;
            }
            0x82C03898 => {
    //   block [0x82C03898..0x82C038C0)
	// 82C03898: 4BFFF399  bl 0x82c02c30
	ctx.lr = 0x82C0389C;
	sub_82C02C30(ctx, base);
	// 82C0389C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C038A0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C038A4: 386A4BD8  addi r3, r10, 0x4bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 19416;
	// 82C038A8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C038AC: 4B58CE7D  bl 0x82190728
	ctx.lr = 0x82C038B0;
	sub_82190728(ctx, base);
	// 82C038B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C038B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C038B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C038BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C038C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C038C8 size=76
    let mut pc: u32 = 0x82C038C8;
    'dispatch: loop {
        match pc {
            0x82C038C8 => {
    //   block [0x82C038C8..0x82C03900)
	// 82C038C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C038CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C038D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C038D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C038D8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C038DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C038E0: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C038E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C038E8: 419A0018  beq cr6, 0x82c03900
	if ctx.cr[6].eq {
	pc = 0x82C03900; continue 'dispatch;
	}
	// 82C038EC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C038F0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C038F4: 4800284D  bl 0x82c06140
	ctx.lr = 0x82C038F8;
	sub_82C06140(ctx, base);
	// 82C038F8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C038FC: 4B63B06D  bl 0x8223e968
	ctx.lr = 0x82C03900;
	sub_8223E968(ctx, base);
	pc = 0x82C03900; continue 'dispatch;
            }
            0x82C03900 => {
    //   block [0x82C03900..0x82C03914)
	// 82C03900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C03904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0390C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03918 size=72
    let mut pc: u32 = 0x82C03918;
    'dispatch: loop {
        match pc {
            0x82C03918 => {
    //   block [0x82C03918..0x82C0394C)
	// 82C03918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0391C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C03924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03928: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0392C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C03930: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03934: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03938: 419A0014  beq cr6, 0x82c0394c
	if ctx.cr[6].eq {
	pc = 0x82C0394C; continue 'dispatch;
	}
	// 82C0393C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03940: 48002809  bl 0x82c06148
	ctx.lr = 0x82C03944;
	sub_82C06148(ctx, base);
	// 82C03944: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03948: 4B63B021  bl 0x8223e968
	ctx.lr = 0x82C0394C;
	sub_8223E968(ctx, base);
	pc = 0x82C0394C; continue 'dispatch;
            }
            0x82C0394C => {
    //   block [0x82C0394C..0x82C03960)
	// 82C0394C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C03950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03958: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0395C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03960 size=76
    let mut pc: u32 = 0x82C03960;
    'dispatch: loop {
        match pc {
            0x82C03960 => {
    //   block [0x82C03960..0x82C03998)
	// 82C03960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0396C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C03974: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C03978: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82C0397C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03980: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03984: 4800A23D  bl 0x82c0dbc0
	ctx.lr = 0x82C03988;
	sub_82C0DBC0(ctx, base);
	// 82C03988: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0398C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03990: 419A0008  beq cr6, 0x82c03998
	if ctx.cr[6].eq {
	pc = 0x82C03998; continue 'dispatch;
	}
	// 82C03994: 4B63AFD5  bl 0x8223e968
	ctx.lr = 0x82C03998;
	sub_8223E968(ctx, base);
	pc = 0x82C03998; continue 'dispatch;
            }
            0x82C03998 => {
    //   block [0x82C03998..0x82C039AC)
	// 82C03998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0399C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C039A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C039A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C039A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C039B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C039B0 size=68
    let mut pc: u32 = 0x82C039B0;
    'dispatch: loop {
        match pc {
            0x82C039B0 => {
    //   block [0x82C039B0..0x82C039E0)
	// 82C039B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C039B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C039B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C039BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C039C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C039C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C039C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C039CC: 48009CE5  bl 0x82c0d6b0
	ctx.lr = 0x82C039D0;
	sub_82C0D6B0(ctx, base);
	// 82C039D0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C039D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C039D8: 419A0008  beq cr6, 0x82c039e0
	if ctx.cr[6].eq {
	pc = 0x82C039E0; continue 'dispatch;
	}
	// 82C039DC: 4B63AF8D  bl 0x8223e968
	ctx.lr = 0x82C039E0;
	sub_8223E968(ctx, base);
	pc = 0x82C039E0; continue 'dispatch;
            }
            0x82C039E0 => {
    //   block [0x82C039E0..0x82C039F4)
	// 82C039E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C039E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C039E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C039EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C039F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C039F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C039F8 size=20
    let mut pc: u32 = 0x82C039F8;
    'dispatch: loop {
        match pc {
            0x82C039F8 => {
    //   block [0x82C039F8..0x82C03A0C)
	// 82C039F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C039FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C03A00: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82C03A04: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03A08: 4800A488  b 0x82c0de90
	sub_82C0DE90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03A10 size=12
    let mut pc: u32 = 0x82C03A10;
    'dispatch: loop {
        match pc {
            0x82C03A10 => {
    //   block [0x82C03A10..0x82C03A1C)
	// 82C03A10: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03A14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C03A18: 48009D90  b 0x82c0d7a8
	sub_82C0D7A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03A20 size=44
    let mut pc: u32 = 0x82C03A20;
    'dispatch: loop {
        match pc {
            0x82C03A20 => {
    //   block [0x82C03A20..0x82C03A4C)
	// 82C03A20: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03A24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C03A28: 894B4BD0  lbz r10, 0x4bd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C03A2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C03A30: 419A001C  beq cr6, 0x82c03a4c
	if ctx.cr[6].eq {
		sub_82C03A4C(ctx, base);
		return;
	}
	// 82C03A34: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03A38: 816B4BC8  lwz r11, 0x4bc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03A3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03A40: 419A000C  beq cr6, 0x82c03a4c
	if ctx.cr[6].eq {
		sub_82C03A4C(ctx, base);
		return;
	}
	// 82C03A44: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C03A48: 48002770  b 0x82c061b8
	sub_82C061B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03A4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03A4C size=8
    let mut pc: u32 = 0x82C03A4C;
    'dispatch: loop {
        match pc {
            0x82C03A4C => {
    //   block [0x82C03A4C..0x82C03A54)
	// 82C03A4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C03A58 size=36
    let mut pc: u32 = 0x82C03A58;
    'dispatch: loop {
        match pc {
            0x82C03A58 => {
    //   block [0x82C03A58..0x82C03A7C)
	// 82C03A58: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03A5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C03A60: 806A4BC8  lwz r3, 0x4bc8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03A64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03A68: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C03A6C: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C03A70: C02B0008  lfs f1, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C03A74: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03A78: 48002778  b 0x82c061f0
	sub_82C061F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03A7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03A7C size=4
    let mut pc: u32 = 0x82C03A7C;
    'dispatch: loop {
        match pc {
            0x82C03A7C => {
    //   block [0x82C03A7C..0x82C03A80)
	// 82C03A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03A80 size=32
    let mut pc: u32 = 0x82C03A80;
    'dispatch: loop {
        match pc {
            0x82C03A80 => {
    //   block [0x82C03A80..0x82C03AA0)
	// 82C03A80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03A84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C03A88: 806A4BC8  lwz r3, 0x4bc8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03A8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03A90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C03A94: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03A98: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03A9C: 48002714  b 0x82c061b0
	crate::recompiler::externs::call(ctx, base, 0x82C061B0);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03AA0 size=4
    let mut pc: u32 = 0x82C03AA0;
    'dispatch: loop {
        match pc {
            0x82C03AA0 => {
    //   block [0x82C03AA0..0x82C03AA4)
	// 82C03AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C03AA8 size=20
    let mut pc: u32 = 0x82C03AA8;
    'dispatch: loop {
        match pc {
            0x82C03AA8 => {
    //   block [0x82C03AA8..0x82C03ABC)
	// 82C03AA8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03AAC: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03AB0: C0230004  lfs f1, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C03AB4: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03AB8: 48002740  b 0x82c061f8
	sub_82C061F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C03AC0 size=24
    let mut pc: u32 = 0x82C03AC0;
    'dispatch: loop {
        match pc {
            0x82C03AC0 => {
    //   block [0x82C03AC0..0x82C03AD8)
	// 82C03AC0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03AC4: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C03AC8: C0230008  lfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C03ACC: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03AD0: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03AD4: 4800272C  b 0x82c06200
	sub_82C06200(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C03AD8 size=20
    let mut pc: u32 = 0x82C03AD8;
    'dispatch: loop {
        match pc {
            0x82C03AD8 => {
    //   block [0x82C03AD8..0x82C03AEC)
	// 82C03AD8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03ADC: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03AE0: C0230004  lfs f1, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C03AE4: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03AE8: 48002720  b 0x82c06208
	sub_82C06208(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C03AF0 size=24
    let mut pc: u32 = 0x82C03AF0;
    'dispatch: loop {
        match pc {
            0x82C03AF0 => {
    //   block [0x82C03AF0..0x82C03B08)
	// 82C03AF0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03AF4: C023000C  lfs f1, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C03AF8: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03AFC: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03B00: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03B04: 4800270C  b 0x82c06210
	sub_82C06210(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03B08 size=16
    let mut pc: u32 = 0x82C03B08;
    'dispatch: loop {
        match pc {
            0x82C03B08 => {
    //   block [0x82C03B08..0x82C03B18)
	// 82C03B08: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03B0C: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82C03B10: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03B14: 48002704  b 0x82c06218
	sub_82C06218(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03B18 size=16
    let mut pc: u32 = 0x82C03B18;
    'dispatch: loop {
        match pc {
            0x82C03B18 => {
    //   block [0x82C03B18..0x82C03B28)
	// 82C03B18: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03B1C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03B20: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03B24: 480026FC  b 0x82c06220
	sub_82C06220(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03B28 size=28
    let mut pc: u32 = 0x82C03B28;
    'dispatch: loop {
        match pc {
            0x82C03B28 => {
    //   block [0x82C03B28..0x82C03B44)
	// 82C03B28: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03B2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C03B30: 806A4BC8  lwz r3, 0x4bc8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03B34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03B38: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C03B3C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03B40: 480026A8  b 0x82c061e8
	sub_82C061E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03B44 size=4
    let mut pc: u32 = 0x82C03B44;
    'dispatch: loop {
        match pc {
            0x82C03B44 => {
    //   block [0x82C03B44..0x82C03B48)
	// 82C03B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03B48 size=52
    let mut pc: u32 = 0x82C03B48;
    'dispatch: loop {
        match pc {
            0x82C03B48 => {
    //   block [0x82C03B48..0x82C03B7C)
	// 82C03B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03B54: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03B58: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03B5C: 480025D5  bl 0x82c06130
	ctx.lr = 0x82C03B60;
	sub_82C06130(ctx, base);
	// 82C03B60: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03B64: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82C03B68: 912A4BCC  stw r9, 0x4bcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(19404 as u32), ctx.r[9].u32 ) };
	// 82C03B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C03B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03B80 size=80
    let mut pc: u32 = 0x82C03B80;
    'dispatch: loop {
        match pc {
            0x82C03B80 => {
    //   block [0x82C03B80..0x82C03BBC)
	// 82C03B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C03B8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03B90: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82C03B94: 817F4BCC  lwz r11, 0x4bcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19404 as u32) ) } as u64;
	// 82C03B98: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C03B9C: 409A0020  bne cr6, 0x82c03bbc
	if !ctx.cr[6].eq {
	pc = 0x82C03BBC; continue 'dispatch;
	}
	// 82C03BA0: 4800A239  bl 0x82c0ddd8
	ctx.lr = 0x82C03BA4;
	sub_82C0DDD8(ctx, base);
	// 82C03BA4: 4800A4C5  bl 0x82c0e068
	ctx.lr = 0x82C03BA8;
	sub_82C0E068(ctx, base);
	// 82C03BA8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03BAC: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03BB0: 48002589  bl 0x82c06138
	ctx.lr = 0x82C03BB4;
	sub_82C06138(ctx, base);
	// 82C03BB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C03BB8: 915F4BCC  stw r10, 0x4bcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19404 as u32), ctx.r[10].u32 ) };
	pc = 0x82C03BBC; continue 'dispatch;
            }
            0x82C03BBC => {
    //   block [0x82C03BBC..0x82C03BD0)
	// 82C03BBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C03BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03BC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03BD0 size=20
    let mut pc: u32 = 0x82C03BD0;
    'dispatch: loop {
        match pc {
            0x82C03BD0 => {
    //   block [0x82C03BD0..0x82C03BE4)
	// 82C03BD0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03BD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03BD8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C03BDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C03BE0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03BE8 size=280
    let mut pc: u32 = 0x82C03BE8;
    'dispatch: loop {
        match pc {
            0x82C03BE8 => {
    //   block [0x82C03BE8..0x82C03C08)
	// 82C03BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03BF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C03BF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03BF8: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82C03BFC: 897F4BD0  lbz r11, 0x4bd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C03C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03C04: 419A001C  beq cr6, 0x82c03c20
	if ctx.cr[6].eq {
	pc = 0x82C03C20; continue 'dispatch;
	}
	pc = 0x82C03C08; continue 'dispatch;
            }
            0x82C03C08 => {
    //   block [0x82C03C08..0x82C03C20)
	// 82C03C08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03C0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C03C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03C18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03C1C: 4E800020  blr
	return;
            }
            0x82C03C20 => {
    //   block [0x82C03C20..0x82C03C94)
	// 82C03C20: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03C24: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03C28: 4800A5A9  bl 0x82c0e1d0
	ctx.lr = 0x82C03C2C;
	sub_82C0E1D0(ctx, base);
	// 82C03C2C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C03C30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03C34: 419AFFD4  beq cr6, 0x82c03c08
	if ctx.cr[6].eq {
	pc = 0x82C03C08; continue 'dispatch;
	}
	// 82C03C38: 48006319  bl 0x82c09f50
	ctx.lr = 0x82C03C3C;
	sub_82C09F50(ctx, base);
	// 82C03C3C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C03C40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03C44: 419AFFC4  beq cr6, 0x82c03c08
	if ctx.cr[6].eq {
	pc = 0x82C03C08; continue 'dispatch;
	}
	// 82C03C48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03C4C: 4BF9322D  bl 0x82b96e78
	ctx.lr = 0x82C03C50;
	sub_82B96E78(ctx, base);
	// 82C03C50: 48001ED1  bl 0x82c05b20
	ctx.lr = 0x82C03C54;
	sub_82C05B20(ctx, base);
	// 82C03C54: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C03C58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03C5C: 419AFFAC  beq cr6, 0x82c03c08
	if ctx.cr[6].eq {
	pc = 0x82C03C08; continue 'dispatch;
	}
	// 82C03C60: 4800D681  bl 0x82c112e0
	ctx.lr = 0x82C03C64;
	sub_82C112E0(ctx, base);
	// 82C03C64: 4800AC65  bl 0x82c0e8c8
	ctx.lr = 0x82C03C68;
	sub_82C0E8C8(ctx, base);
	// 82C03C68: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C03C6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03C70: 419AFF98  beq cr6, 0x82c03c08
	if ctx.cr[6].eq {
	pc = 0x82C03C08; continue 'dispatch;
	}
	// 82C03C74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82C03C78: 4B61B5E1  bl 0x8221f258
	ctx.lr = 0x82C03C7C;
	sub_8221F258(ctx, base);
	// 82C03C7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03C80: 419A0014  beq cr6, 0x82c03c94
	if ctx.cr[6].eq {
	pc = 0x82C03C94; continue 'dispatch;
	}
	// 82C03C84: 480026AD  bl 0x82c06330
	ctx.lr = 0x82C03C88;
	sub_82C06330(ctx, base);
	// 82C03C88: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03C8C: 906B4BC8  stw r3, 0x4bc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(19400 as u32), ctx.r[3].u32 ) };
	// 82C03C90: 48000010  b 0x82c03ca0
	pc = 0x82C03CA0; continue 'dispatch;
            }
            0x82C03C94 => {
    //   block [0x82C03C94..0x82C03CA0)
	// 82C03C94: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C03C9C: 916A4BC8  stw r11, 0x4bc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(19400 as u32), ctx.r[11].u32 ) };
	pc = 0x82C03CA0; continue 'dispatch;
            }
            0x82C03CA0 => {
    //   block [0x82C03CA0..0x82C03D00)
	// 82C03CA0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03CA4: 38A01000  li r5, 0x1000
	ctx.r[5].s64 = 4096;
	// 82C03CA8: 386B4BD8  addi r3, r11, 0x4bd8
	ctx.r[3].s64 = ctx.r[11].s64 + 19416;
	// 82C03CAC: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 82C03CB0: 48006121  bl 0x82c09dd0
	ctx.lr = 0x82C03CB4;
	sub_82C09DD0(ctx, base);
	// 82C03CB4: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C03CB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C03CBC: 419AFF4C  beq cr6, 0x82c03c08
	if ctx.cr[6].eq {
	pc = 0x82C03C08; continue 'dispatch;
	}
	// 82C03CC0: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82C03CC4: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82C03CC8: 392A6168  addi r9, r10, 0x6168
	ctx.r[9].s64 = ctx.r[10].s64 + 24936;
	// 82C03CCC: 396B3860  addi r11, r11, 0x3860
	ctx.r[11].s64 = ctx.r[11].s64 + 14432;
	// 82C03CD0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82C03CD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C03CD8: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C03CDC: 4800A8A5  bl 0x82c0e580
	ctx.lr = 0x82C03CE0;
	sub_82C0E580(ctx, base);
	// 82C03CE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C03CE4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C03CE8: 997F4BD0  stb r11, 0x4bd0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(19408 as u32), ctx.r[11].u8 ) };
	// 82C03CEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C03CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03CF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C03D00 size=136
    let mut pc: u32 = 0x82C03D00;
    'dispatch: loop {
        match pc {
            0x82C03D00 => {
    //   block [0x82C03D00..0x82C03D70)
	// 82C03D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03D08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C03D0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03D10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C03D14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C03D18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03D1C: 4B6291B5  bl 0x8222ced0
	ctx.lr = 0x82C03D20;
	sub_8222CED0(ctx, base);
	// 82C03D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03D24: 4B686CB5  bl 0x8228a9d8
	ctx.lr = 0x82C03D28;
	sub_8228A9D8(ctx, base);
	// 82C03D28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C03D2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C03D30: 4B6110A9  bl 0x82214dd8
	ctx.lr = 0x82C03D34;
	sub_82214DD8(ctx, base);
	// 82C03D34: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03D38: 894B4BD0  lbz r10, 0x4bd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C03D3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C03D40: 419A0030  beq cr6, 0x82c03d70
	if ctx.cr[6].eq {
	pc = 0x82C03D70; continue 'dispatch;
	}
	// 82C03D44: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C03D48: 806B4BC8  lwz r3, 0x4bc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03D4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03D50: 419A0020  beq cr6, 0x82c03d70
	if ctx.cr[6].eq {
	pc = 0x82C03D70; continue 'dispatch;
	}
	// 82C03D54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C03D58: 48002461  bl 0x82c061b8
	ctx.lr = 0x82C03D5C;
	sub_82C061B8(ctx, base);
	// 82C03D5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C03D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03D6C: 4E800020  blr
	return;
            }
            0x82C03D70 => {
    //   block [0x82C03D70..0x82C03D88)
	// 82C03D70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C03D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C03D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C03D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C03D88 size=204
    let mut pc: u32 = 0x82C03D88;
    'dispatch: loop {
        match pc {
            0x82C03D88 => {
    //   block [0x82C03D88..0x82C03E44)
	// 82C03D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03D90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03D94: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C03D98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C03D9C: 806A4BC8  lwz r3, 0x4bc8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C03DA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C03DA4: 419A00A0  beq cr6, 0x82c03e44
	if ctx.cr[6].eq {
	pc = 0x82C03E44; continue 'dispatch;
	}
	// 82C03DA8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C03DAC: C02B0028  lfs f1, 0x28(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C03DB0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82C03DB4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C03DB8: 394B001C  addi r10, r11, 0x1c
	ctx.r[10].s64 = ctx.r[11].s64 + 28;
	// 82C03DBC: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82C03DC0: C0080C18  lfs f0, 0xc18(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C03DC4: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82C03DC8: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82C03DCC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C03DD0: 13E03407  vcmpneb. (lvlx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03DD4: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82C03DD8: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82C03DDC: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82C03DE0: 12802407  vcmpneb. (lvlx128) v20, v0, v4
	tmp.u32 = ctx.r[4].u32;
	// load shuffled into ctx.v[52] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03DE4: 13A05407  vcmpneb. (lvlx128) v29, v0, v10
	tmp.u32 = ctx.r[10].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03DE8: 13804C07  vcmpneb. (lvlx128) v28, v0, v9
	tmp.u32 = ctx.r[9].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03DEC: 13605C07  vcmpneb. (lvlx128) v27, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[59] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03DF0: 134A3C07  vcmpneb. (lvlx128) v26, v10, v7
	tmp.u32 = ctx.r[10].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[58] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03DF4: 13293C07  vcmpneb. (lvlx128) v25, v9, v7
	tmp.u32 = ctx.r[9].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[57] using VectorMaskL[(tmp.u32 & 0xF)]
	pc = 0x82C03E44; continue 'dispatch;
            }
            0x82C03E44 => {
    //   block [0x82C03E44..0x82C03E54)
	// 82C03E44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C03E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C03E58 size=124
    let mut pc: u32 = 0x82C03E58;
    'dispatch: loop {
        match pc {
            0x82C03E58 => {
    //   block [0x82C03E58..0x82C03EC4)
	// 82C03E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03E64: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03E68: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03E6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03E70: 419A0054  beq cr6, 0x82c03ec4
	if ctx.cr[6].eq {
	pc = 0x82C03EC4; continue 'dispatch;
	}
	// 82C03E74: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82C03E78: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03E7C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82C03E80: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C03E84: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82C03E88: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C03E8C: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03E90: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03E94: 13CB5407  vcmpneb. (lvlx128) v30, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	pc = 0x82C03EC4; continue 'dispatch;
            }
            0x82C03EC4 => {
    //   block [0x82C03EC4..0x82C03ED4)
	// 82C03EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C03EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03ED8 size=36
    let mut pc: u32 = 0x82C03ED8;
    'dispatch: loop {
        match pc {
            0x82C03ED8 => {
    //   block [0x82C03ED8..0x82C03EFC)
	// 82C03ED8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03EDC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03EE0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C03EE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C03EE8: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C03EEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03EF0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03EF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03EF8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03EFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03EFC size=4
    let mut pc: u32 = 0x82C03EFC;
    'dispatch: loop {
        match pc {
            0x82C03EFC => {
    //   block [0x82C03EFC..0x82C03F00)
	// 82C03EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03F00 size=48
    let mut pc: u32 = 0x82C03F00;
    'dispatch: loop {
        match pc {
            0x82C03F00 => {
    //   block [0x82C03F00..0x82C03F30)
	// 82C03F00: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03F04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03F08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C03F0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C03F10: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C03F14: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C03F18: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03F1C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C03F20: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03F24: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C03F28: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C03F2C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03F30 size=4
    let mut pc: u32 = 0x82C03F30;
    'dispatch: loop {
        match pc {
            0x82C03F30 => {
    //   block [0x82C03F30..0x82C03F34)
	// 82C03F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03F38 size=64
    let mut pc: u32 = 0x82C03F38;
    'dispatch: loop {
        match pc {
            0x82C03F38 => {
    //   block [0x82C03F38..0x82C03F64)
	// 82C03F38: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03F3C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03F40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03F44: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C03F48: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C03F4C: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03F54: 409A0010  bne cr6, 0x82c03f64
	if !ctx.cr[6].eq {
	pc = 0x82C03F64; continue 'dispatch;
	}
	// 82C03F58: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82C03F5C: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 82C03F60: 48000008  b 0x82c03f68
	pc = 0x82C03F68; continue 'dispatch;
            }
            0x82C03F64 => {
    //   block [0x82C03F64..0x82C03F68)
	// 82C03F64: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C03F68; continue 'dispatch;
            }
            0x82C03F68 => {
    //   block [0x82C03F68..0x82C03F78)
	// 82C03F68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03F6C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C03F70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C03F74: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C03F78 size=4
    let mut pc: u32 = 0x82C03F78;
    'dispatch: loop {
        match pc {
            0x82C03F78 => {
    //   block [0x82C03F78..0x82C03F7C)
	// 82C03F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C03F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C03F80 size=124
    let mut pc: u32 = 0x82C03F80;
    'dispatch: loop {
        match pc {
            0x82C03F80 => {
    //   block [0x82C03F80..0x82C03FEC)
	// 82C03F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C03F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C03F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C03F8C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C03F90: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C03F98: 419A0054  beq cr6, 0x82c03fec
	if ctx.cr[6].eq {
	pc = 0x82C03FEC; continue 'dispatch;
	}
	// 82C03F9C: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82C03FA0: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03FA4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82C03FA8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C03FAC: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82C03FB0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C03FB4: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C03FB8: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C03FBC: 13CB5407  vcmpneb. (lvlx128) v30, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	pc = 0x82C03FEC; continue 'dispatch;
            }
            0x82C03FEC => {
    //   block [0x82C03FEC..0x82C03FFC)
	// 82C03FEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C03FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C03FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C03FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04000 size=36
    let mut pc: u32 = 0x82C04000;
    'dispatch: loop {
        match pc {
            0x82C04000 => {
    //   block [0x82C04000..0x82C04024)
	// 82C04000: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C04004: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04008: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0400C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C04010: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C04014: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04018: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0401C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C04020: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04024(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04024 size=4
    let mut pc: u32 = 0x82C04024;
    'dispatch: loop {
        match pc {
            0x82C04024 => {
    //   block [0x82C04024..0x82C04028)
	// 82C04024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04028 size=48
    let mut pc: u32 = 0x82C04028;
    'dispatch: loop {
        match pc {
            0x82C04028 => {
    //   block [0x82C04028..0x82C04058)
	// 82C04028: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0402C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04030: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04034: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C04038: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C0403C: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C04040: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C04044: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C04048: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0404C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C04050: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C04054: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04058 size=4
    let mut pc: u32 = 0x82C04058;
    'dispatch: loop {
        match pc {
            0x82C04058 => {
    //   block [0x82C04058..0x82C0405C)
	// 82C04058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04060 size=64
    let mut pc: u32 = 0x82C04060;
    'dispatch: loop {
        match pc {
            0x82C04060 => {
    //   block [0x82C04060..0x82C0408C)
	// 82C04060: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C04064: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0406C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C04070: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C04074: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0407C: 409A0010  bne cr6, 0x82c0408c
	if !ctx.cr[6].eq {
	pc = 0x82C0408C; continue 'dispatch;
	}
	// 82C04080: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82C04084: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 82C04088: 48000008  b 0x82c04090
	pc = 0x82C04090; continue 'dispatch;
            }
            0x82C0408C => {
    //   block [0x82C0408C..0x82C04090)
	// 82C0408C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82C04090; continue 'dispatch;
            }
            0x82C04090 => {
    //   block [0x82C04090..0x82C040A0)
	// 82C04090: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04094: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C04098: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0409C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C040A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C040A0 size=4
    let mut pc: u32 = 0x82C040A0;
    'dispatch: loop {
        match pc {
            0x82C040A0 => {
    //   block [0x82C040A0..0x82C040A4)
	// 82C040A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C040A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C040A8 size=48
    let mut pc: u32 = 0x82C040A8;
    'dispatch: loop {
        match pc {
            0x82C040A8 => {
    //   block [0x82C040A8..0x82C040D8)
	// 82C040A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C040AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C040B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C040B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C040B8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C040BC: C0230008  lfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C040C0: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C040C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C040C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C040CC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C040D0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C040D4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C040D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C040D8 size=4
    let mut pc: u32 = 0x82C040D8;
    'dispatch: loop {
        match pc {
            0x82C040D8 => {
    //   block [0x82C040D8..0x82C040DC)
	// 82C040D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C040E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C040E0 size=48
    let mut pc: u32 = 0x82C040E0;
    'dispatch: loop {
        match pc {
            0x82C040E0 => {
    //   block [0x82C040E0..0x82C04110)
	// 82C040E0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C040E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C040E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C040EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C040F0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C040F4: C0230008  lfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C040F8: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C040FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C04100: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04104: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C04108: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C0410C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04110 size=4
    let mut pc: u32 = 0x82C04110;
    'dispatch: loop {
        match pc {
            0x82C04110 => {
    //   block [0x82C04110..0x82C04114)
	// 82C04110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04118 size=44
    let mut pc: u32 = 0x82C04118;
    'dispatch: loop {
        match pc {
            0x82C04118 => {
    //   block [0x82C04118..0x82C04144)
	// 82C04118: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0411C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04120: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04124: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C04128: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C0412C: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C04130: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C04134: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04138: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C0413C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C04140: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04144(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04144 size=4
    let mut pc: u32 = 0x82C04144;
    'dispatch: loop {
        match pc {
            0x82C04144 => {
    //   block [0x82C04144..0x82C04148)
	// 82C04144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04148 size=44
    let mut pc: u32 = 0x82C04148;
    'dispatch: loop {
        match pc {
            0x82C04148 => {
    //   block [0x82C04148..0x82C04174)
	// 82C04148: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0414C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04150: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04154: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C04158: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C0415C: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C04160: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C04164: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04168: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0416C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C04170: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04174 size=4
    let mut pc: u32 = 0x82C04174;
    'dispatch: loop {
        match pc {
            0x82C04174 => {
    //   block [0x82C04174..0x82C04178)
	// 82C04174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04178 size=36
    let mut pc: u32 = 0x82C04178;
    'dispatch: loop {
        match pc {
            0x82C04178 => {
    //   block [0x82C04178..0x82C0419C)
	// 82C04178: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0417C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04180: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04184: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C04188: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C0418C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04190: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C04194: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C04198: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0419C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0419C size=4
    let mut pc: u32 = 0x82C0419C;
    'dispatch: loop {
        match pc {
            0x82C0419C => {
    //   block [0x82C0419C..0x82C041A0)
	// 82C0419C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C041A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C041A0 size=48
    let mut pc: u32 = 0x82C041A0;
    'dispatch: loop {
        match pc {
            0x82C041A0 => {
    //   block [0x82C041A0..0x82C041D0)
	// 82C041A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C041A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C041A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C041AC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C041B0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C041B4: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C041B8: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C041BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C041C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C041C4: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C041C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C041CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C041D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C041D0 size=4
    let mut pc: u32 = 0x82C041D0;
    'dispatch: loop {
        match pc {
            0x82C041D0 => {
    //   block [0x82C041D0..0x82C041D4)
	// 82C041D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C041D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C041D8 size=168
    let mut pc: u32 = 0x82C041D8;
    'dispatch: loop {
        match pc {
            0x82C041D8 => {
    //   block [0x82C041D8..0x82C0423C)
	// 82C041D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C041DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C041E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C041E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C041E8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C041EC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C041F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C041F4: 894B4BD0  lbz r10, 0x4bd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C041F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C041FC: 419A006C  beq cr6, 0x82c04268
	if ctx.cr[6].eq {
	pc = 0x82C04268; continue 'dispatch;
	}
	// 82C04200: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C04204: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04208: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0420C: 4BF4B035  bl 0x82b4f240
	ctx.lr = 0x82C04210;
	sub_82B4F240(ctx, base);
	// 82C04210: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04214: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04218: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C0421C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04220: 4B63A751  bl 0x8223e970
	ctx.lr = 0x82C04224;
	sub_8223E970(ctx, base);
	// 82C04224: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C04228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0422C: 419A0010  beq cr6, 0x82c0423c
	if ctx.cr[6].eq {
	pc = 0x82C0423C; continue 'dispatch;
	}
	// 82C04230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C04234: 392AA0F8  addi r9, r10, -0x5f08
	ctx.r[9].s64 = ctx.r[10].s64 + -24328;
	// 82C04238: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C0423C; continue 'dispatch;
            }
            0x82C0423C => {
    //   block [0x82C0423C..0x82C04268)
	// 82C0423C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82C04240: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04248: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C0424C: 4B63A5ED  bl 0x8223e838
	ctx.lr = 0x82C04250;
	sub_8223E838(ctx, base);
	// 82C04250: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C04254: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82C04258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0425C: 4BF4B1A5  bl 0x82b4f400
	ctx.lr = 0x82C04260;
	sub_82B4F400(ctx, base);
	// 82C04260: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04264: 4BF4B195  bl 0x82b4f3f8
	ctx.lr = 0x82C04268;
	sub_82B4F3F8(ctx, base);
	pc = 0x82C04268; continue 'dispatch;
            }
            0x82C04268 => {
    //   block [0x82C04268..0x82C04280)
	// 82C04268: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82C0426C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04274: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04278: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0427C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04280 size=188
    let mut pc: u32 = 0x82C04280;
    'dispatch: loop {
        match pc {
            0x82C04280 => {
    //   block [0x82C04280..0x82C042E0)
	// 82C04280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04284: 480A517D  bl 0x82ca9400
	ctx.lr = 0x82C04288;
	sub_82CA93D0(ctx, base);
	// 82C04288: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0428C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04290: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C04294: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C04298: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0429C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C042A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C042A4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C042A8: 4BF4AF99  bl 0x82b4f240
	ctx.lr = 0x82C042AC;
	sub_82B4F240(ctx, base);
	// 82C042AC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C042B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C042B4: 3B4B4BD8  addi r26, r11, 0x4bd8
	ctx.r[26].s64 = ctx.r[11].s64 + 19416;
	// 82C042B8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C042BC: 4B63A6B5  bl 0x8223e970
	ctx.lr = 0x82C042C0;
	sub_8223E970(ctx, base);
	// 82C042C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C042C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C042C8: 419A0018  beq cr6, 0x82c042e0
	if ctx.cr[6].eq {
	pc = 0x82C042E0; continue 'dispatch;
	}
	// 82C042CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C042D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C042D4: 394BA104  addi r10, r11, -0x5efc
	ctx.r[10].s64 = ctx.r[11].s64 + -24316;
	// 82C042D8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C042DC: 4B5F3525  bl 0x821f7800
	ctx.lr = 0x82C042E0;
	sub_821F7800(ctx, base);
	pc = 0x82C042E0; continue 'dispatch;
            }
            0x82C042E0 => {
    //   block [0x82C042E0..0x82C04308)
	// 82C042E0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C042E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C042E8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82C042EC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C042F0: 4B671149  bl 0x82275438
	ctx.lr = 0x82C042F4;
	sub_82275438(ctx, base);
	// 82C042F4: 577E063E  clrlwi r30, r27, 0x18
	ctx.r[30].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82C042F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C042FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C04300: 409A0008  bne cr6, 0x82c04308
	if !ctx.cr[6].eq {
	pc = 0x82C04308; continue 'dispatch;
	}
	// 82C04304: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	pc = 0x82C04308; continue 'dispatch;
            }
            0x82C04308 => {
    //   block [0x82C04308..0x82C0432C)
	// 82C04308: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C0430C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C04310: 4B63A529  bl 0x8223e838
	ctx.lr = 0x82C04314;
	sub_8223E838(ctx, base);
	// 82C04314: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C04318: 409A0014  bne cr6, 0x82c0432c
	if !ctx.cr[6].eq {
	pc = 0x82C0432C; continue 'dispatch;
	}
	// 82C0431C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C04320: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82C04324: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04328: 4BF4B0D9  bl 0x82b4f400
	ctx.lr = 0x82C0432C;
	sub_82B4F400(ctx, base);
	pc = 0x82C0432C; continue 'dispatch;
            }
            0x82C0432C => {
    //   block [0x82C0432C..0x82C0433C)
	// 82C0432C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04330: 4BF4B0C9  bl 0x82b4f3f8
	ctx.lr = 0x82C04334;
	sub_82B4F3F8(ctx, base);
	// 82C04334: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82C04338: 480A5118  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04340 size=96
    let mut pc: u32 = 0x82C04340;
    'dispatch: loop {
        match pc {
            0x82C04340 => {
    //   block [0x82C04340..0x82C04388)
	// 82C04340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04348: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C0434C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C04358: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C0435C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C04360: 4B610A79  bl 0x82214dd8
	ctx.lr = 0x82C04364;
	sub_82214DD8(ctx, base);
	// 82C04364: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04368: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C0436C: 392BA0E8  addi r9, r11, -0x5f18
	ctx.r[9].s64 = ctx.r[11].s64 + -24344;
	// 82C04370: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04374: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C04378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0437C: 419A000C  beq cr6, 0x82c04388
	if ctx.cr[6].eq {
	pc = 0x82C04388; continue 'dispatch;
	}
	// 82C04380: 4BC41431  bl 0x828457b0
	ctx.lr = 0x82C04384;
	sub_828457B0(ctx, base);
	// 82C04384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C04388; continue 'dispatch;
            }
            0x82C04388 => {
    //   block [0x82C04388..0x82C043A0)
	// 82C04388: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0438C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04394: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0439C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C043A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C043A0 size=152
    let mut pc: u32 = 0x82C043A0;
    'dispatch: loop {
        match pc {
            0x82C043A0 => {
    //   block [0x82C043A0..0x82C043EC)
	// 82C043A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C043A4: 480A5069  bl 0x82ca940c
	ctx.lr = 0x82C043A8;
	sub_82CA93D0(ctx, base);
	// 82C043A8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C043AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C043B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C043B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C043B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C043BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C043C0: 4BF4AE81  bl 0x82b4f240
	ctx.lr = 0x82C043C4;
	sub_82B4F240(ctx, base);
	// 82C043C4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C043C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C043CC: 3BAB4BD8  addi r29, r11, 0x4bd8
	ctx.r[29].s64 = ctx.r[11].s64 + 19416;
	// 82C043D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C043D4: 4B63A59D  bl 0x8223e970
	ctx.lr = 0x82C043D8;
	sub_8223E970(ctx, base);
	// 82C043D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C043DC: 419A0010  beq cr6, 0x82c043ec
	if ctx.cr[6].eq {
	pc = 0x82C043EC; continue 'dispatch;
	}
	// 82C043E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C043E4: 394BA110  addi r10, r11, -0x5ef0
	ctx.r[10].s64 = ctx.r[11].s64 + -24304;
	// 82C043E8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C043EC; continue 'dispatch;
            }
            0x82C043EC => {
    //   block [0x82C043EC..0x82C04404)
	// 82C043EC: 57FF063E  clrlwi r31, r31, 0x18
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82C043F0: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C043F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C043F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C043FC: 409A0008  bne cr6, 0x82c04404
	if !ctx.cr[6].eq {
	pc = 0x82C04404; continue 'dispatch;
	}
	// 82C04400: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	pc = 0x82C04404; continue 'dispatch;
            }
            0x82C04404 => {
    //   block [0x82C04404..0x82C04428)
	// 82C04404: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C04408: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C0440C: 4B63A42D  bl 0x8223e838
	ctx.lr = 0x82C04410;
	sub_8223E838(ctx, base);
	// 82C04410: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C04414: 409A0014  bne cr6, 0x82c04428
	if !ctx.cr[6].eq {
	pc = 0x82C04428; continue 'dispatch;
	}
	// 82C04418: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0441C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82C04420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04424: 4BF4AFDD  bl 0x82b4f400
	ctx.lr = 0x82C04428;
	sub_82B4F400(ctx, base);
	pc = 0x82C04428; continue 'dispatch;
            }
            0x82C04428 => {
    //   block [0x82C04428..0x82C04438)
	// 82C04428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0442C: 4BF4AFCD  bl 0x82b4f3f8
	ctx.lr = 0x82C04430;
	sub_82B4F3F8(ctx, base);
	// 82C04430: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82C04434: 480A5028  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04438 size=104
    let mut pc: u32 = 0x82C04438;
    'dispatch: loop {
        match pc {
            0x82C04438 => {
    //   block [0x82C04438..0x82C04480)
	// 82C04438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0443C: 480A4FCD  bl 0x82ca9408
	ctx.lr = 0x82C04440;
	sub_82CA93D0(ctx, base);
	// 82C04440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04444: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04448: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C0444C: 3BCB4BD8  addi r30, r11, 0x4bd8
	ctx.r[30].s64 = ctx.r[11].s64 + 19416;
	// 82C04450: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C04454: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C0445C: 4B63A515  bl 0x8223e970
	ctx.lr = 0x82C04460;
	sub_8223E970(ctx, base);
	// 82C04460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C04464: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C04468: 419A0018  beq cr6, 0x82c04480
	if ctx.cr[6].eq {
	pc = 0x82C04480; continue 'dispatch;
	}
	// 82C0446C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04470: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C04474: 394BA11C  addi r10, r11, -0x5ee4
	ctx.r[10].s64 = ctx.r[11].s64 + -24292;
	// 82C04478: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0447C: 4B5F3385  bl 0x821f7800
	ctx.lr = 0x82C04480;
	sub_821F7800(ctx, base);
	pc = 0x82C04480; continue 'dispatch;
            }
            0x82C04480 => {
    //   block [0x82C04480..0x82C044A0)
	// 82C04480: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C04484: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C04488: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C0448C: 4B670FAD  bl 0x82275438
	ctx.lr = 0x82C04490;
	sub_82275438(ctx, base);
	// 82C04490: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C04494: 4B63A3A5  bl 0x8223e838
	ctx.lr = 0x82C04498;
	sub_8223E838(ctx, base);
	// 82C04498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0449C: 480A4FBC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C044A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C044A0 size=96
    let mut pc: u32 = 0x82C044A0;
    'dispatch: loop {
        match pc {
            0x82C044A0 => {
    //   block [0x82C044A0..0x82C044E8)
	// 82C044A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C044A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C044A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C044AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C044B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C044B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C044B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C044BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C044C0: 4B610919  bl 0x82214dd8
	ctx.lr = 0x82C044C4;
	sub_82214DD8(ctx, base);
	// 82C044C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C044C8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C044CC: 392BA0E8  addi r9, r11, -0x5f18
	ctx.r[9].s64 = ctx.r[11].s64 + -24344;
	// 82C044D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C044D4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C044D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C044DC: 419A000C  beq cr6, 0x82c044e8
	if ctx.cr[6].eq {
	pc = 0x82C044E8; continue 'dispatch;
	}
	// 82C044E0: 4BC412D1  bl 0x828457b0
	ctx.lr = 0x82C044E4;
	sub_828457B0(ctx, base);
	// 82C044E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C044E8; continue 'dispatch;
            }
            0x82C044E8 => {
    //   block [0x82C044E8..0x82C04500)
	// 82C044E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C044EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C044F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C044F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C044F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C044FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04500 size=100
    let mut pc: u32 = 0x82C04500;
    'dispatch: loop {
        match pc {
            0x82C04500 => {
    //   block [0x82C04500..0x82C04540)
	// 82C04500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C0450C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04514: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04518: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0451C: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04520: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04528: 4B63A449  bl 0x8223e970
	ctx.lr = 0x82C0452C;
	sub_8223E970(ctx, base);
	// 82C0452C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04530: 419A0010  beq cr6, 0x82c04540
	if ctx.cr[6].eq {
	pc = 0x82C04540; continue 'dispatch;
	}
	// 82C04534: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04538: 394BA128  addi r10, r11, -0x5ed8
	ctx.r[10].s64 = ctx.r[11].s64 + -24280;
	// 82C0453C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C04540; continue 'dispatch;
            }
            0x82C04540 => {
    //   block [0x82C04540..0x82C04564)
	// 82C04540: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04548: 4B63A2F1  bl 0x8223e838
	ctx.lr = 0x82C0454C;
	sub_8223E838(ctx, base);
	// 82C0454C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04558: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0455C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04568 size=88
    let mut pc: u32 = 0x82C04568;
    'dispatch: loop {
        match pc {
            0x82C04568 => {
    //   block [0x82C04568..0x82C045A8)
	// 82C04568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0456C: 480A4EA1  bl 0x82ca940c
	ctx.lr = 0x82C04570;
	sub_82CA93D0(ctx, base);
	// 82C04570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04574: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04578: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0457C: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04580: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C04584: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0458C: 4B63A3E5  bl 0x8223e970
	ctx.lr = 0x82C04590;
	sub_8223E970(ctx, base);
	// 82C04590: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C04594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C04598: 419A0010  beq cr6, 0x82c045a8
	if ctx.cr[6].eq {
	pc = 0x82C045A8; continue 'dispatch;
	}
	// 82C0459C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C045A0: 392AA134  addi r9, r10, -0x5ecc
	ctx.r[9].s64 = ctx.r[10].s64 + -24268;
	// 82C045A4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C045A8; continue 'dispatch;
            }
            0x82C045A8 => {
    //   block [0x82C045A8..0x82C045C0)
	// 82C045A8: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C045AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C045B0: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C045B4: 4B63A285  bl 0x8223e838
	ctx.lr = 0x82C045B8;
	sub_8223E838(ctx, base);
	// 82C045B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C045BC: 480A4EA0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C045C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C045C0 size=116
    let mut pc: u32 = 0x82C045C0;
    'dispatch: loop {
        match pc {
            0x82C045C0 => {
    //   block [0x82C045C0..0x82C04608)
	// 82C045C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C045C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C045C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C045CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C045D0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C045D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C045D8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C045DC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C045E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C045E4: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C045E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C045EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C045F0: 4B63A381  bl 0x8223e970
	ctx.lr = 0x82C045F4;
	sub_8223E970(ctx, base);
	// 82C045F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C045F8: 419A0010  beq cr6, 0x82c04608
	if ctx.cr[6].eq {
	pc = 0x82C04608; continue 'dispatch;
	}
	// 82C045FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04600: 394BA140  addi r10, r11, -0x5ec0
	ctx.r[10].s64 = ctx.r[11].s64 + -24256;
	// 82C04604: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C04608; continue 'dispatch;
            }
            0x82C04608 => {
    //   block [0x82C04608..0x82C04634)
	// 82C04608: D3E30004  stfs f31, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C0460C: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C04610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04614: 4B63A225  bl 0x8223e838
	ctx.lr = 0x82C04618;
	sub_8223E838(ctx, base);
	// 82C04618: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0461C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04624: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C04628: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0462C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C04638 size=100
    let mut pc: u32 = 0x82C04638;
    'dispatch: loop {
        match pc {
            0x82C04638 => {
    //   block [0x82C04638..0x82C0467C)
	// 82C04638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0463C: 480A4DD1  bl 0x82ca940c
	ctx.lr = 0x82C04640;
	sub_82CA93D0(ctx, base);
	// 82C04640: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C04644: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04648: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0464C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C04650: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04654: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04658: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C0465C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04660: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C04664: 4B63A30D  bl 0x8223e970
	ctx.lr = 0x82C04668;
	sub_8223E970(ctx, base);
	// 82C04668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0466C: 419A0010  beq cr6, 0x82c0467c
	if ctx.cr[6].eq {
	pc = 0x82C0467C; continue 'dispatch;
	}
	// 82C04670: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04674: 394BA14C  addi r10, r11, -0x5eb4
	ctx.r[10].s64 = ctx.r[11].s64 + -24244;
	// 82C04678: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C0467C; continue 'dispatch;
            }
            0x82C0467C => {
    //   block [0x82C0467C..0x82C0469C)
	// 82C0467C: D3E30008  stfs f31, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C04680: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04684: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82C04688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0468C: 4B63A1AD  bl 0x8223e838
	ctx.lr = 0x82C04690;
	sub_8223E838(ctx, base);
	// 82C04690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C04694: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C04698: 480A4DC4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C046A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C046A0 size=116
    let mut pc: u32 = 0x82C046A0;
    'dispatch: loop {
        match pc {
            0x82C046A0 => {
    //   block [0x82C046A0..0x82C046E8)
	// 82C046A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C046A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C046A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C046AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C046B0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C046B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C046B8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C046BC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C046C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C046C4: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C046C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C046CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C046D0: 4B63A2A1  bl 0x8223e970
	ctx.lr = 0x82C046D4;
	sub_8223E970(ctx, base);
	// 82C046D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C046D8: 419A0010  beq cr6, 0x82c046e8
	if ctx.cr[6].eq {
	pc = 0x82C046E8; continue 'dispatch;
	}
	// 82C046DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C046E0: 394BA158  addi r10, r11, -0x5ea8
	ctx.r[10].s64 = ctx.r[11].s64 + -24232;
	// 82C046E4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C046E8; continue 'dispatch;
            }
            0x82C046E8 => {
    //   block [0x82C046E8..0x82C04714)
	// 82C046E8: D3E30004  stfs f31, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C046EC: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C046F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C046F4: 4B63A145  bl 0x8223e838
	ctx.lr = 0x82C046F8;
	sub_8223E838(ctx, base);
	// 82C046F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C046FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04704: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C04708: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0470C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C04718 size=100
    let mut pc: u32 = 0x82C04718;
    'dispatch: loop {
        match pc {
            0x82C04718 => {
    //   block [0x82C04718..0x82C0475C)
	// 82C04718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0471C: 480A4CF1  bl 0x82ca940c
	ctx.lr = 0x82C04720;
	sub_82CA93D0(ctx, base);
	// 82C04720: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C04724: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04728: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0472C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C04730: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04734: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04738: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C0473C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04744: 4B63A22D  bl 0x8223e970
	ctx.lr = 0x82C04748;
	sub_8223E970(ctx, base);
	// 82C04748: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0474C: 419A0010  beq cr6, 0x82c0475c
	if ctx.cr[6].eq {
	pc = 0x82C0475C; continue 'dispatch;
	}
	// 82C04750: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04754: 394BA164  addi r10, r11, -0x5e9c
	ctx.r[10].s64 = ctx.r[11].s64 + -24220;
	// 82C04758: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C0475C; continue 'dispatch;
            }
            0x82C0475C => {
    //   block [0x82C0475C..0x82C0477C)
	// 82C0475C: D3E3000C  stfs f31, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C04760: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04764: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C04768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0476C: 4B63A0CD  bl 0x8223e838
	ctx.lr = 0x82C04770;
	sub_8223E838(ctx, base);
	// 82C04770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C04774: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C04778: 480A4CE4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04780 size=96
    let mut pc: u32 = 0x82C04780;
    'dispatch: loop {
        match pc {
            0x82C04780 => {
    //   block [0x82C04780..0x82C047C4)
	// 82C04780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04784: 480A4C89  bl 0x82ca940c
	ctx.lr = 0x82C04788;
	sub_82CA93D0(ctx, base);
	// 82C04788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0478C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04790: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C04794: 3BCB4BD8  addi r30, r11, 0x4bd8
	ctx.r[30].s64 = ctx.r[11].s64 + 19416;
	// 82C04798: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C0479C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C047A0: 4B63A1D1  bl 0x8223e970
	ctx.lr = 0x82C047A4;
	sub_8223E970(ctx, base);
	// 82C047A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C047A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C047AC: 419A0018  beq cr6, 0x82c047c4
	if ctx.cr[6].eq {
	pc = 0x82C047C4; continue 'dispatch;
	}
	// 82C047B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C047B4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C047B8: 394BA170  addi r10, r11, -0x5e90
	ctx.r[10].s64 = ctx.r[11].s64 + -24208;
	// 82C047BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C047C0: 4B5F3041  bl 0x821f7800
	ctx.lr = 0x82C047C4;
	sub_821F7800(ctx, base);
	pc = 0x82C047C4; continue 'dispatch;
            }
            0x82C047C4 => {
    //   block [0x82C047C4..0x82C047E0)
	// 82C047C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C047C8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C047CC: 4B670C6D  bl 0x82275438
	ctx.lr = 0x82C047D0;
	sub_82275438(ctx, base);
	// 82C047D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C047D4: 4B63A065  bl 0x8223e838
	ctx.lr = 0x82C047D8;
	sub_8223E838(ctx, base);
	// 82C047D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C047DC: 480A4C80  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C047E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C047E0 size=96
    let mut pc: u32 = 0x82C047E0;
    'dispatch: loop {
        match pc {
            0x82C047E0 => {
    //   block [0x82C047E0..0x82C04828)
	// 82C047E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C047E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C047E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C047EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C047F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C047F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C047F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C047FC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C04800: 4B6105D9  bl 0x82214dd8
	ctx.lr = 0x82C04804;
	sub_82214DD8(ctx, base);
	// 82C04804: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04808: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C0480C: 392BA0E8  addi r9, r11, -0x5f18
	ctx.r[9].s64 = ctx.r[11].s64 + -24344;
	// 82C04810: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04814: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C04818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0481C: 419A000C  beq cr6, 0x82c04828
	if ctx.cr[6].eq {
	pc = 0x82C04828; continue 'dispatch;
	}
	// 82C04820: 4BC40F91  bl 0x828457b0
	ctx.lr = 0x82C04824;
	sub_828457B0(ctx, base);
	// 82C04824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C04828; continue 'dispatch;
            }
            0x82C04828 => {
    //   block [0x82C04828..0x82C04840)
	// 82C04828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0482C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04834: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04840 size=112
    let mut pc: u32 = 0x82C04840;
    'dispatch: loop {
        match pc {
            0x82C04840 => {
    //   block [0x82C04840..0x82C0487C)
	// 82C04840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04844: 480A4BC9  bl 0x82ca940c
	ctx.lr = 0x82C04848;
	sub_82CA93D0(ctx, base);
	// 82C04848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0484C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04850: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C04854: 3BCB4BD8  addi r30, r11, 0x4bd8
	ctx.r[30].s64 = ctx.r[11].s64 + 19416;
	// 82C04858: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C0485C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C04860: 4B63A111  bl 0x8223e970
	ctx.lr = 0x82C04864;
	sub_8223E970(ctx, base);
	// 82C04864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C04868: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C0486C: 419A0010  beq cr6, 0x82c0487c
	if ctx.cr[6].eq {
	pc = 0x82C0487C; continue 'dispatch;
	}
	// 82C04870: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04874: 394BA17C  addi r10, r11, -0x5e84
	ctx.r[10].s64 = ctx.r[11].s64 + -24196;
	// 82C04878: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C0487C; continue 'dispatch;
            }
            0x82C0487C => {
    //   block [0x82C0487C..0x82C048B0)
	// 82C0487C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C04880: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C04884: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04888: 4B628649  bl 0x8222ced0
	ctx.lr = 0x82C0488C;
	sub_8222CED0(ctx, base);
	// 82C0488C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04890: 4B686149  bl 0x8228a9d8
	ctx.lr = 0x82C04894;
	sub_8228A9D8(ctx, base);
	// 82C04894: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C04898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0489C: 4B61053D  bl 0x82214dd8
	ctx.lr = 0x82C048A0;
	sub_82214DD8(ctx, base);
	// 82C048A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C048A4: 4B639F95  bl 0x8223e838
	ctx.lr = 0x82C048A8;
	sub_8223E838(ctx, base);
	// 82C048A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C048AC: 480A4BB0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C048B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C048B0 size=100
    let mut pc: u32 = 0x82C048B0;
    'dispatch: loop {
        match pc {
            0x82C048B0 => {
    //   block [0x82C048B0..0x82C048F0)
	// 82C048B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C048B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C048B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C048BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C048C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C048C4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C048C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C048CC: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C048D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C048D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C048D8: 4B63A099  bl 0x8223e970
	ctx.lr = 0x82C048DC;
	sub_8223E970(ctx, base);
	// 82C048DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C048E0: 419A0010  beq cr6, 0x82c048f0
	if ctx.cr[6].eq {
	pc = 0x82C048F0; continue 'dispatch;
	}
	// 82C048E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C048E8: 394BA188  addi r10, r11, -0x5e78
	ctx.r[10].s64 = ctx.r[11].s64 + -24184;
	// 82C048EC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C048F0; continue 'dispatch;
            }
            0x82C048F0 => {
    //   block [0x82C048F0..0x82C04914)
	// 82C048F0: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C048F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C048F8: 4B639F41  bl 0x8223e838
	ctx.lr = 0x82C048FC;
	sub_8223E838(ctx, base);
	// 82C048FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04908: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0490C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04918 size=172
    let mut pc: u32 = 0x82C04918;
    'dispatch: loop {
        match pc {
            0x82C04918 => {
    //   block [0x82C04918..0x82C049C4)
	// 82C04918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04928: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C049C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C049C8 size=196
    let mut pc: u32 = 0x82C049C8;
    'dispatch: loop {
        match pc {
            0x82C049C8 => {
    //   block [0x82C049C8..0x82C04A8C)
	// 82C049C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C049CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C049D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C049D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C049D8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C049DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C049E0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82C049E4: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82C049E8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82C049EC: C0090C18  lfs f0, 0xc18(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C049F0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C049F4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C049F8: 3CC08333  lis r6, -0x7ccd
	ctx.r[6].s64 = -2093809664;
	// 82C049FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04A00: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C04A04: 13CB5407  vcmpneb. (lvlx128) v30, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04A90 size=116
    let mut pc: u32 = 0x82C04A90;
    'dispatch: loop {
        match pc {
            0x82C04A90 => {
    //   block [0x82C04A90..0x82C04AE0)
	// 82C04A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04AA4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04AAC: 894B4BD0  lbz r10, 0x4bd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C04AB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04AB4: 419A0038  beq cr6, 0x82c04aec
	if ctx.cr[6].eq {
	pc = 0x82C04AEC; continue 'dispatch;
	}
	// 82C04AB8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04ABC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04AC0: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04AC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04AC8: 4B639EA9  bl 0x8223e970
	ctx.lr = 0x82C04ACC;
	sub_8223E970(ctx, base);
	// 82C04ACC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04AD0: 419A0010  beq cr6, 0x82c04ae0
	if ctx.cr[6].eq {
	pc = 0x82C04AE0; continue 'dispatch;
	}
	// 82C04AD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04AD8: 394BA1A0  addi r10, r11, -0x5e60
	ctx.r[10].s64 = ctx.r[11].s64 + -24160;
	// 82C04ADC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C04AE0; continue 'dispatch;
            }
            0x82C04AE0 => {
    //   block [0x82C04AE0..0x82C04AEC)
	// 82C04AE0: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04AE8: 4B639D51  bl 0x8223e838
	ctx.lr = 0x82C04AEC;
	sub_8223E838(ctx, base);
	pc = 0x82C04AEC; continue 'dispatch;
            }
            0x82C04AEC => {
    //   block [0x82C04AEC..0x82C04B04)
	// 82C04AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04AF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04AFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C04B08 size=172
    let mut pc: u32 = 0x82C04B08;
    'dispatch: loop {
        match pc {
            0x82C04B08 => {
    //   block [0x82C04B08..0x82C04BB4)
	// 82C04B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04B18: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04BB8 size=132
    let mut pc: u32 = 0x82C04BB8;
    'dispatch: loop {
        match pc {
            0x82C04BB8 => {
    //   block [0x82C04BB8..0x82C04C0C)
	// 82C04BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04BC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04BC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04BC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04BCC: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C04BD0: 817E4BCC  lwz r11, 0x4bcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19404 as u32) ) } as u64;
	// 82C04BD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C04BD8: 409A004C  bne cr6, 0x82c04c24
	if !ctx.cr[6].eq {
	pc = 0x82C04C24; continue 'dispatch;
	}
	// 82C04BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C04BE0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04BE4: 915E4BCC  stw r10, 0x4bcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(19404 as u32), ctx.r[10].u32 ) };
	// 82C04BE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04BEC: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04BF4: 4B639D7D  bl 0x8223e970
	ctx.lr = 0x82C04BF8;
	sub_8223E970(ctx, base);
	// 82C04BF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04BFC: 419A0010  beq cr6, 0x82c04c0c
	if ctx.cr[6].eq {
	pc = 0x82C04C0C; continue 'dispatch;
	}
	// 82C04C00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04C04: 394BA1B8  addi r10, r11, -0x5e48
	ctx.r[10].s64 = ctx.r[11].s64 + -24136;
	// 82C04C08: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C04C0C; continue 'dispatch;
            }
            0x82C04C0C => {
    //   block [0x82C04C0C..0x82C04C14)
	// 82C04C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04C10: 4B639C29  bl 0x8223e838
	ctx.lr = 0x82C04C14;
	sub_8223E838(ctx, base);
	pc = 0x82C04C14; continue 'dispatch;
            }
            0x82C04C14 => {
    //   block [0x82C04C14..0x82C04C24)
	// 82C04C14: 817E4BCC  lwz r11, 0x4bcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(19404 as u32) ) } as u64;
	// 82C04C18: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C04C1C: 409AFFF8  bne cr6, 0x82c04c14
	if !ctx.cr[6].eq {
	pc = 0x82C04C14; continue 'dispatch;
	}
	// 82C04C20: 48006591  bl 0x82c0b1b0
	ctx.lr = 0x82C04C24;
	sub_82C0B1B0(ctx, base);
	pc = 0x82C04C24; continue 'dispatch;
            }
            0x82C04C24 => {
    //   block [0x82C04C24..0x82C04C3C)
	// 82C04C24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04C30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04C34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04C40 size=100
    let mut pc: u32 = 0x82C04C40;
    'dispatch: loop {
        match pc {
            0x82C04C40 => {
    //   block [0x82C04C40..0x82C04C80)
	// 82C04C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04C54: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04C58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04C5C: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04C60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04C64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04C68: 4B639D09  bl 0x8223e970
	ctx.lr = 0x82C04C6C;
	sub_8223E970(ctx, base);
	// 82C04C6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04C70: 419A0010  beq cr6, 0x82c04c80
	if ctx.cr[6].eq {
	pc = 0x82C04C80; continue 'dispatch;
	}
	// 82C04C74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04C78: 394BA1C4  addi r10, r11, -0x5e3c
	ctx.r[10].s64 = ctx.r[11].s64 + -24124;
	// 82C04C7C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C04C80; continue 'dispatch;
            }
            0x82C04C80 => {
    //   block [0x82C04C80..0x82C04CA4)
	// 82C04C80: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04C88: 4B639BB1  bl 0x8223e838
	ctx.lr = 0x82C04C8C;
	sub_8223E838(ctx, base);
	// 82C04C8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04C98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04CA8 size=136
    let mut pc: u32 = 0x82C04CA8;
    'dispatch: loop {
        match pc {
            0x82C04CA8 => {
    //   block [0x82C04CA8..0x82C04CF4)
	// 82C04CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04CB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04CB8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04CBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C04CC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04CC4: 808B4BC8  lwz r4, 0x4bc8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19400 as u32) ) } as u64;
	// 82C04CC8: 48001759  bl 0x82c06420
	ctx.lr = 0x82C04CCC;
	sub_82C06420(ctx, base);
	// 82C04CCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C04CD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C04CD4: 4800CFF5  bl 0x82c11cc8
	ctx.lr = 0x82C04CD8;
	sub_82C11CC8(ctx, base);
	// 82C04CD8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C04CDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04CE0: 419A0014  beq cr6, 0x82c04cf4
	if ctx.cr[6].eq {
	pc = 0x82C04CF4; continue 'dispatch;
	}
	// 82C04CE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04CE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C04CEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C04CF0: 4E800421  bctrl
	ctx.lr = 0x82C04CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C04CF4 => {
    //   block [0x82C04CF4..0x82C04D1C)
	// 82C04CF4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C04CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C04CFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C04D00: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C04D04: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04D08: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C04D0C: 419A0010  beq cr6, 0x82c04d1c
	if ctx.cr[6].eq {
	pc = 0x82C04D1C; continue 'dispatch;
	}
	// 82C04D10: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C04D14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04D18: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	pc = 0x82C04D1C; continue 'dispatch;
            }
            0x82C04D1C => {
    //   block [0x82C04D1C..0x82C04D30)
	// 82C04D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04D30 size=116
    let mut pc: u32 = 0x82C04D30;
    'dispatch: loop {
        match pc {
            0x82C04D30 => {
    //   block [0x82C04D30..0x82C04D80)
	// 82C04D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04D44: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04D48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04D4C: 894B4BD0  lbz r10, 0x4bd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C04D50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04D54: 419A0038  beq cr6, 0x82c04d8c
	if ctx.cr[6].eq {
	pc = 0x82C04D8C; continue 'dispatch;
	}
	// 82C04D58: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04D5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04D60: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04D68: 4B639C09  bl 0x8223e970
	ctx.lr = 0x82C04D6C;
	sub_8223E970(ctx, base);
	// 82C04D6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04D70: 419A0010  beq cr6, 0x82c04d80
	if ctx.cr[6].eq {
	pc = 0x82C04D80; continue 'dispatch;
	}
	// 82C04D74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04D78: 394BA1D0  addi r10, r11, -0x5e30
	ctx.r[10].s64 = ctx.r[11].s64 + -24112;
	// 82C04D7C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C04D80; continue 'dispatch;
            }
            0x82C04D80 => {
    //   block [0x82C04D80..0x82C04D8C)
	// 82C04D80: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04D88: 4B639AB1  bl 0x8223e838
	ctx.lr = 0x82C04D8C;
	sub_8223E838(ctx, base);
	pc = 0x82C04D8C; continue 'dispatch;
            }
            0x82C04D8C => {
    //   block [0x82C04D8C..0x82C04DA4)
	// 82C04D8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04D98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04D9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04DA8 size=100
    let mut pc: u32 = 0x82C04DA8;
    'dispatch: loop {
        match pc {
            0x82C04DA8 => {
    //   block [0x82C04DA8..0x82C04DE8)
	// 82C04DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04DB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04DB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04DB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04DBC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04DC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04DC4: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04DC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04DD0: 4B639BA1  bl 0x8223e970
	ctx.lr = 0x82C04DD4;
	sub_8223E970(ctx, base);
	// 82C04DD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04DD8: 419A0010  beq cr6, 0x82c04de8
	if ctx.cr[6].eq {
	pc = 0x82C04DE8; continue 'dispatch;
	}
	// 82C04DDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04DE0: 394BA1DC  addi r10, r11, -0x5e24
	ctx.r[10].s64 = ctx.r[11].s64 + -24100;
	// 82C04DE4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C04DE8; continue 'dispatch;
            }
            0x82C04DE8 => {
    //   block [0x82C04DE8..0x82C04E0C)
	// 82C04DE8: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04DF0: 4B639A49  bl 0x8223e838
	ctx.lr = 0x82C04DF4;
	sub_8223E838(ctx, base);
	// 82C04DF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04E00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C04E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04E10 size=96
    let mut pc: u32 = 0x82C04E10;
    'dispatch: loop {
        match pc {
            0x82C04E10 => {
    //   block [0x82C04E10..0x82C04E54)
	// 82C04E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04E14: 480A45F5  bl 0x82ca9408
	ctx.lr = 0x82C04E18;
	sub_82CA93D0(ctx, base);
	// 82C04E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04E1C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04E20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04E24: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04E28: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C04E2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04E34: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C04E38: 4B639B39  bl 0x8223e970
	ctx.lr = 0x82C04E3C;
	sub_8223E970(ctx, base);
	// 82C04E3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C04E40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C04E44: 419A0010  beq cr6, 0x82c04e54
	if ctx.cr[6].eq {
	pc = 0x82C04E54; continue 'dispatch;
	}
	// 82C04E48: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C04E4C: 392AA1E8  addi r9, r10, -0x5e18
	ctx.r[9].s64 = ctx.r[10].s64 + -24088;
	// 82C04E50: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C04E54; continue 'dispatch;
            }
            0x82C04E54 => {
    //   block [0x82C04E54..0x82C04E70)
	// 82C04E54: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04E5C: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C04E60: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82C04E64: 4B6399D5  bl 0x8223e838
	ctx.lr = 0x82C04E68;
	sub_8223E838(ctx, base);
	// 82C04E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C04E6C: 480A45EC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04E70 size=104
    let mut pc: u32 = 0x82C04E70;
    'dispatch: loop {
        match pc {
            0x82C04E70 => {
    //   block [0x82C04E70..0x82C04EB8)
	// 82C04E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04E74: 480A4595  bl 0x82ca9408
	ctx.lr = 0x82C04E78;
	sub_82CA93D0(ctx, base);
	// 82C04E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04E7C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04E80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C04E84: 3BCB4BD8  addi r30, r11, 0x4bd8
	ctx.r[30].s64 = ctx.r[11].s64 + 19416;
	// 82C04E88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C04E8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04E90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C04E94: 4B639ADD  bl 0x8223e970
	ctx.lr = 0x82C04E98;
	sub_8223E970(ctx, base);
	// 82C04E98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C04E9C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C04EA0: 419A0018  beq cr6, 0x82c04eb8
	if ctx.cr[6].eq {
	pc = 0x82C04EB8; continue 'dispatch;
	}
	// 82C04EA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C04EA8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C04EAC: 394BA1F4  addi r10, r11, -0x5e0c
	ctx.r[10].s64 = ctx.r[11].s64 + -24076;
	// 82C04EB0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C04EB4: 4B5F294D  bl 0x821f7800
	ctx.lr = 0x82C04EB8;
	sub_821F7800(ctx, base);
	pc = 0x82C04EB8; continue 'dispatch;
            }
            0x82C04EB8 => {
    //   block [0x82C04EB8..0x82C04ED8)
	// 82C04EB8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C04EBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C04EC0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C04EC4: 4B670575  bl 0x82275438
	ctx.lr = 0x82C04EC8;
	sub_82275438(ctx, base);
	// 82C04EC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C04ECC: 4B63996D  bl 0x8223e838
	ctx.lr = 0x82C04ED0;
	sub_8223E838(ctx, base);
	// 82C04ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C04ED4: 480A4584  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04ED8 size=88
    let mut pc: u32 = 0x82C04ED8;
    'dispatch: loop {
        match pc {
            0x82C04ED8 => {
    //   block [0x82C04ED8..0x82C04F18)
	// 82C04ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04EDC: 480A4531  bl 0x82ca940c
	ctx.lr = 0x82C04EE0;
	sub_82CA93D0(ctx, base);
	// 82C04EE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04EE4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04EE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04EEC: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04EF0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C04EF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04EFC: 4B639A75  bl 0x8223e970
	ctx.lr = 0x82C04F00;
	sub_8223E970(ctx, base);
	// 82C04F00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C04F04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C04F08: 419A0010  beq cr6, 0x82c04f18
	if ctx.cr[6].eq {
	pc = 0x82C04F18; continue 'dispatch;
	}
	// 82C04F0C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C04F10: 392AA200  addi r9, r10, -0x5e00
	ctx.r[9].s64 = ctx.r[10].s64 + -24064;
	// 82C04F14: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C04F18; continue 'dispatch;
            }
            0x82C04F18 => {
    //   block [0x82C04F18..0x82C04F30)
	// 82C04F18: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C04F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C04F20: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C04F24: 4B639915  bl 0x8223e838
	ctx.lr = 0x82C04F28;
	sub_8223E838(ctx, base);
	// 82C04F28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04F2C: 480A4530  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04F30 size=148
    let mut pc: u32 = 0x82C04F30;
    'dispatch: loop {
        match pc {
            0x82C04F30 => {
    //   block [0x82C04F30..0x82C04F88)
	// 82C04F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04F3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C04F44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C04F48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C04F4C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04F50: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04F54: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04F58: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C04F5C: 4E800421  bctrl
	ctx.lr = 0x82C04F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C04F60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C04F64: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C04F68: 4800CD61  bl 0x82c11cc8
	ctx.lr = 0x82C04F6C;
	sub_82C11CC8(ctx, base);
	// 82C04F6C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C04F70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C04F74: 419A0014  beq cr6, 0x82c04f88
	if ctx.cr[6].eq {
	pc = 0x82C04F88; continue 'dispatch;
	}
	// 82C04F78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04F7C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C04F80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C04F84: 4E800421  bctrl
	ctx.lr = 0x82C04F88;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C04F88 => {
    //   block [0x82C04F88..0x82C04FB0)
	// 82C04F88: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C04F8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C04F90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C04F94: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C04F98: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04F9C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C04FA0: 419A0010  beq cr6, 0x82c04fb0
	if ctx.cr[6].eq {
	pc = 0x82C04FB0; continue 'dispatch;
	}
	// 82C04FA4: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C04FA8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C04FAC: 916A003C  stw r11, 0x3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	pc = 0x82C04FB0; continue 'dispatch;
            }
            0x82C04FB0 => {
    //   block [0x82C04FB0..0x82C04FC4)
	// 82C04FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C04FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C04FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C04FBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C04FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C04FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C04FC8 size=116
    let mut pc: u32 = 0x82C04FC8;
    'dispatch: loop {
        match pc {
            0x82C04FC8 => {
    //   block [0x82C04FC8..0x82C05018)
	// 82C04FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C04FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C04FD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C04FD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C04FD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C04FDC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04FE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C04FE4: 894B4BD0  lbz r10, 0x4bd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C04FE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C04FEC: 419A0038  beq cr6, 0x82c05024
	if ctx.cr[6].eq {
	pc = 0x82C05024; continue 'dispatch;
	}
	// 82C04FF0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C04FF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C04FF8: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C04FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05000: 4B639971  bl 0x8223e970
	ctx.lr = 0x82C05004;
	sub_8223E970(ctx, base);
	// 82C05004: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C05008: 419A0010  beq cr6, 0x82c05018
	if ctx.cr[6].eq {
	pc = 0x82C05018; continue 'dispatch;
	}
	// 82C0500C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05010: 394BA20C  addi r10, r11, -0x5df4
	ctx.r[10].s64 = ctx.r[11].s64 + -24052;
	// 82C05014: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C05018; continue 'dispatch;
            }
            0x82C05018 => {
    //   block [0x82C05018..0x82C05024)
	// 82C05018: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C0501C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05020: 4B639819  bl 0x8223e838
	ctx.lr = 0x82C05024;
	sub_8223E838(ctx, base);
	pc = 0x82C05024; continue 'dispatch;
            }
            0x82C05024 => {
    //   block [0x82C05024..0x82C0503C)
	// 82C05024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0502C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05030: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05034: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05040 size=100
    let mut pc: u32 = 0x82C05040;
    'dispatch: loop {
        match pc {
            0x82C05040 => {
    //   block [0x82C05040..0x82C05080)
	// 82C05040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C0504C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05054: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C05058: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0505C: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C05060: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C05064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05068: 4B639909  bl 0x8223e970
	ctx.lr = 0x82C0506C;
	sub_8223E970(ctx, base);
	// 82C0506C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C05070: 419A0010  beq cr6, 0x82c05080
	if ctx.cr[6].eq {
	pc = 0x82C05080; continue 'dispatch;
	}
	// 82C05074: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05078: 394BA218  addi r10, r11, -0x5de8
	ctx.r[10].s64 = ctx.r[11].s64 + -24040;
	// 82C0507C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C05080; continue 'dispatch;
            }
            0x82C05080 => {
    //   block [0x82C05080..0x82C050A4)
	// 82C05080: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C05084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05088: 4B6397B1  bl 0x8223e838
	ctx.lr = 0x82C0508C;
	sub_8223E838(ctx, base);
	// 82C0508C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05098: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0509C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C050A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C050A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C050A8 size=104
    let mut pc: u32 = 0x82C050A8;
    'dispatch: loop {
        match pc {
            0x82C050A8 => {
    //   block [0x82C050A8..0x82C050F0)
	// 82C050A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C050AC: 480A435D  bl 0x82ca9408
	ctx.lr = 0x82C050B0;
	sub_82CA93D0(ctx, base);
	// 82C050B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C050B4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C050B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C050BC: 3BCB4BD8  addi r30, r11, 0x4bd8
	ctx.r[30].s64 = ctx.r[11].s64 + 19416;
	// 82C050C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C050C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C050C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C050CC: 4B6398A5  bl 0x8223e970
	ctx.lr = 0x82C050D0;
	sub_8223E970(ctx, base);
	// 82C050D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C050D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C050D8: 419A0018  beq cr6, 0x82c050f0
	if ctx.cr[6].eq {
	pc = 0x82C050F0; continue 'dispatch;
	}
	// 82C050DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C050E0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C050E4: 394BA224  addi r10, r11, -0x5ddc
	ctx.r[10].s64 = ctx.r[11].s64 + -24028;
	// 82C050E8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C050EC: 4B5F2715  bl 0x821f7800
	ctx.lr = 0x82C050F0;
	sub_821F7800(ctx, base);
	pc = 0x82C050F0; continue 'dispatch;
            }
            0x82C050F0 => {
    //   block [0x82C050F0..0x82C05110)
	// 82C050F0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C050F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C050F8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C050FC: 4B67033D  bl 0x82275438
	ctx.lr = 0x82C05100;
	sub_82275438(ctx, base);
	// 82C05100: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C05104: 4B639735  bl 0x8223e838
	ctx.lr = 0x82C05108;
	sub_8223E838(ctx, base);
	// 82C05108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0510C: 480A434C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05110 size=200
    let mut pc: u32 = 0x82C05110;
    'dispatch: loop {
        match pc {
            0x82C05110 => {
    //   block [0x82C05110..0x82C05178)
	// 82C05110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0511C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05124: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05128: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0512C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C05130: 419A0094  beq cr6, 0x82c051c4
	if ctx.cr[6].eq {
	pc = 0x82C051C4; continue 'dispatch;
	}
	// 82C05134: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82C05138: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 82C0513C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05140: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C05144: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C05148: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0514C: 4E800421  bctrl
	ctx.lr = 0x82C05150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C05150: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C05154: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C05158: 4800CB71  bl 0x82c11cc8
	ctx.lr = 0x82C0515C;
	sub_82C11CC8(ctx, base);
	// 82C0515C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C05160: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C05164: 419A0014  beq cr6, 0x82c05178
	if ctx.cr[6].eq {
	pc = 0x82C05178; continue 'dispatch;
	}
	// 82C05168: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0516C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05170: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C05174: 4E800421  bctrl
	ctx.lr = 0x82C05178;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C05178 => {
    //   block [0x82C05178..0x82C051BC)
	// 82C05178: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0517C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C05180: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C05184: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C05188: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0518C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C05190: 419A002C  beq cr6, 0x82c051bc
	if ctx.cr[6].eq {
	pc = 0x82C051BC; continue 'dispatch;
	}
	// 82C05194: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C05198: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0519C: 91490084  stw r10, 0x84(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82C051A0: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C051A4: 99680011  stb r11, 0x11(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82C051A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C051AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C051B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C051B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C051B8: 4E800020  blr
	return;
            }
            0x82C051BC => {
    //   block [0x82C051BC..0x82C051C4)
	// 82C051BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C051C0: 996A0011  stb r11, 0x11(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	pc = 0x82C051C4; continue 'dispatch;
            }
            0x82C051C4 => {
    //   block [0x82C051C4..0x82C051D8)
	// 82C051C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C051C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C051CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C051D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C051D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C051D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C051D8 size=96
    let mut pc: u32 = 0x82C051D8;
    'dispatch: loop {
        match pc {
            0x82C051D8 => {
    //   block [0x82C051D8..0x82C05220)
	// 82C051D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C051DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C051E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C051E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C051E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C051EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C051F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C051F4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C051F8: 4B684851  bl 0x82289a48
	ctx.lr = 0x82C051FC;
	sub_82289A48(ctx, base);
	// 82C051FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05200: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C05204: 392BA0E8  addi r9, r11, -0x5f18
	ctx.r[9].s64 = ctx.r[11].s64 + -24344;
	// 82C05208: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0520C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C05210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05214: 419A000C  beq cr6, 0x82c05220
	if ctx.cr[6].eq {
	pc = 0x82C05220; continue 'dispatch;
	}
	// 82C05218: 4BC40599  bl 0x828457b0
	ctx.lr = 0x82C0521C;
	sub_828457B0(ctx, base);
	// 82C0521C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C05220; continue 'dispatch;
            }
            0x82C05220 => {
    //   block [0x82C05220..0x82C05238)
	// 82C05220: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0522C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05230: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C05238 size=100
    let mut pc: u32 = 0x82C05238;
    'dispatch: loop {
        match pc {
            0x82C05238 => {
    //   block [0x82C05238..0x82C0527C)
	// 82C05238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0523C: 480A41D1  bl 0x82ca940c
	ctx.lr = 0x82C05240;
	sub_82CA93D0(ctx, base);
	// 82C05240: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C05244: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05248: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0524C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C05250: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C05254: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C05258: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C0525C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05260: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C05264: 4B63970D  bl 0x8223e970
	ctx.lr = 0x82C05268;
	sub_8223E970(ctx, base);
	// 82C05268: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0526C: 419A0010  beq cr6, 0x82c0527c
	if ctx.cr[6].eq {
	pc = 0x82C0527C; continue 'dispatch;
	}
	// 82C05270: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05274: 394BA230  addi r10, r11, -0x5dd0
	ctx.r[10].s64 = ctx.r[11].s64 + -24016;
	// 82C05278: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C0527C; continue 'dispatch;
            }
            0x82C0527C => {
    //   block [0x82C0527C..0x82C0529C)
	// 82C0527C: D3E30008  stfs f31, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C05280: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C05284: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82C05288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0528C: 4B6395AD  bl 0x8223e838
	ctx.lr = 0x82C05290;
	sub_8223E838(ctx, base);
	// 82C05290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C05294: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C05298: 480A41C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C052A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C052A0 size=100
    let mut pc: u32 = 0x82C052A0;
    'dispatch: loop {
        match pc {
            0x82C052A0 => {
    //   block [0x82C052A0..0x82C052E4)
	// 82C052A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C052A4: 480A4169  bl 0x82ca940c
	ctx.lr = 0x82C052A8;
	sub_82CA93D0(ctx, base);
	// 82C052A8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C052AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C052B0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C052B4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C052B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C052BC: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C052C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C052C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C052C8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C052CC: 4B6396A5  bl 0x8223e970
	ctx.lr = 0x82C052D0;
	sub_8223E970(ctx, base);
	// 82C052D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C052D4: 419A0010  beq cr6, 0x82c052e4
	if ctx.cr[6].eq {
	pc = 0x82C052E4; continue 'dispatch;
	}
	// 82C052D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C052DC: 394BA23C  addi r10, r11, -0x5dc4
	ctx.r[10].s64 = ctx.r[11].s64 + -24004;
	// 82C052E0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C052E4; continue 'dispatch;
            }
            0x82C052E4 => {
    //   block [0x82C052E4..0x82C05304)
	// 82C052E4: D3E30008  stfs f31, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C052E8: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C052EC: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82C052F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C052F4: 4B639545  bl 0x8223e838
	ctx.lr = 0x82C052F8;
	sub_8223E838(ctx, base);
	// 82C052F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C052FC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C05300: 480A415C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05308 size=88
    let mut pc: u32 = 0x82C05308;
    'dispatch: loop {
        match pc {
            0x82C05308 => {
    //   block [0x82C05308..0x82C05348)
	// 82C05308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0530C: 480A4101  bl 0x82ca940c
	ctx.lr = 0x82C05310;
	sub_82CA93D0(ctx, base);
	// 82C05310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05314: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C05318: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0531C: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C05320: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C05324: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C05328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0532C: 4B639645  bl 0x8223e970
	ctx.lr = 0x82C05330;
	sub_8223E970(ctx, base);
	// 82C05330: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C05334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C05338: 419A0010  beq cr6, 0x82c05348
	if ctx.cr[6].eq {
	pc = 0x82C05348; continue 'dispatch;
	}
	// 82C0533C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C05340: 392AA248  addi r9, r10, -0x5db8
	ctx.r[9].s64 = ctx.r[10].s64 + -23992;
	// 82C05344: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C05348; continue 'dispatch;
            }
            0x82C05348 => {
    //   block [0x82C05348..0x82C05360)
	// 82C05348: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C0534C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05350: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C05354: 4B6394E5  bl 0x8223e838
	ctx.lr = 0x82C05358;
	sub_8223E838(ctx, base);
	// 82C05358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0535C: 480A4100  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05360 size=88
    let mut pc: u32 = 0x82C05360;
    'dispatch: loop {
        match pc {
            0x82C05360 => {
    //   block [0x82C05360..0x82C053A0)
	// 82C05360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05364: 480A40A9  bl 0x82ca940c
	ctx.lr = 0x82C05368;
	sub_82CA93D0(ctx, base);
	// 82C05368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0536C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C05370: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C05374: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C05378: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C0537C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C05380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05384: 4B6395ED  bl 0x8223e970
	ctx.lr = 0x82C05388;
	sub_8223E970(ctx, base);
	// 82C05388: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C0538C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C05390: 419A0010  beq cr6, 0x82c053a0
	if ctx.cr[6].eq {
	pc = 0x82C053A0; continue 'dispatch;
	}
	// 82C05394: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C05398: 392AA254  addi r9, r10, -0x5dac
	ctx.r[9].s64 = ctx.r[10].s64 + -23980;
	// 82C0539C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C053A0; continue 'dispatch;
            }
            0x82C053A0 => {
    //   block [0x82C053A0..0x82C053B8)
	// 82C053A0: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C053A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C053A8: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C053AC: 4B63948D  bl 0x8223e838
	ctx.lr = 0x82C053B0;
	sub_8223E838(ctx, base);
	// 82C053B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C053B4: 480A40A8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C053B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C053B8 size=100
    let mut pc: u32 = 0x82C053B8;
    'dispatch: loop {
        match pc {
            0x82C053B8 => {
    //   block [0x82C053B8..0x82C053F8)
	// 82C053B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C053BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C053C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C053C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C053C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C053CC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C053D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C053D4: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C053D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C053DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C053E0: 4B639591  bl 0x8223e970
	ctx.lr = 0x82C053E4;
	sub_8223E970(ctx, base);
	// 82C053E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C053E8: 419A0010  beq cr6, 0x82c053f8
	if ctx.cr[6].eq {
	pc = 0x82C053F8; continue 'dispatch;
	}
	// 82C053EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C053F0: 394BA260  addi r10, r11, -0x5da0
	ctx.r[10].s64 = ctx.r[11].s64 + -23968;
	// 82C053F4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C053F8; continue 'dispatch;
            }
            0x82C053F8 => {
    //   block [0x82C053F8..0x82C0541C)
	// 82C053F8: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C053FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05400: 4B639439  bl 0x8223e838
	ctx.lr = 0x82C05404;
	sub_8223E838(ctx, base);
	// 82C05404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0540C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05410: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05420 size=96
    let mut pc: u32 = 0x82C05420;
    'dispatch: loop {
        match pc {
            0x82C05420 => {
    //   block [0x82C05420..0x82C05464)
	// 82C05420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05424: 480A3FE5  bl 0x82ca9408
	ctx.lr = 0x82C05428;
	sub_82CA93D0(ctx, base);
	// 82C05428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0542C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C05430: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C05434: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C05438: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C0543C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C05440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05444: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C05448: 4B639529  bl 0x8223e970
	ctx.lr = 0x82C0544C;
	sub_8223E970(ctx, base);
	// 82C0544C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C05450: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C05454: 419A0010  beq cr6, 0x82c05464
	if ctx.cr[6].eq {
	pc = 0x82C05464; continue 'dispatch;
	}
	// 82C05458: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C0545C: 392AA26C  addi r9, r10, -0x5d94
	ctx.r[9].s64 = ctx.r[10].s64 + -23956;
	// 82C05460: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C05464; continue 'dispatch;
            }
            0x82C05464 => {
    //   block [0x82C05464..0x82C05480)
	// 82C05464: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C05468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0546C: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C05470: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82C05474: 4B6393C5  bl 0x8223e838
	ctx.lr = 0x82C05478;
	sub_8223E838(ctx, base);
	// 82C05478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0547C: 480A3FDC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05480 size=164
    let mut pc: u32 = 0x82C05480;
    'dispatch: loop {
        match pc {
            0x82C05480 => {
    //   block [0x82C05480..0x82C054E0)
	// 82C05480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05484: 480A3F89  bl 0x82ca940c
	ctx.lr = 0x82C05488;
	sub_82CA93D0(ctx, base);
	// 82C05488: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0548C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C05490: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C05494: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05498: 894B4BD0  lbz r10, 0x4bd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19408 as u32) ) } as u64;
	// 82C0549C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C054A0: 419A007C  beq cr6, 0x82c0551c
	if ctx.cr[6].eq {
	pc = 0x82C0551C; continue 'dispatch;
	}
	// 82C054A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C054A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C054AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C054B0: 4BF49D91  bl 0x82b4f240
	ctx.lr = 0x82C054B4;
	sub_82B4F240(ctx, base);
	// 82C054B4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C054B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C054BC: 3BEB4BD8  addi r31, r11, 0x4bd8
	ctx.r[31].s64 = ctx.r[11].s64 + 19416;
	// 82C054C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C054C4: 4B6394AD  bl 0x8223e970
	ctx.lr = 0x82C054C8;
	sub_8223E970(ctx, base);
	// 82C054C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C054CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C054D0: 419A0010  beq cr6, 0x82c054e0
	if ctx.cr[6].eq {
	pc = 0x82C054E0; continue 'dispatch;
	}
	// 82C054D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C054D8: 392AA278  addi r9, r10, -0x5d88
	ctx.r[9].s64 = ctx.r[10].s64 + -23944;
	// 82C054DC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82C054E0; continue 'dispatch;
            }
            0x82C054E0 => {
    //   block [0x82C054E0..0x82C0551C)
	// 82C054E0: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C054E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C054E8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C054EC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82C054F0: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C054F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C054F8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C054FC: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82C05500: 4B639339  bl 0x8223e838
	ctx.lr = 0x82C05504;
	sub_8223E838(ctx, base);
	// 82C05504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C05508: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82C0550C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05510: 4BF49EF1  bl 0x82b4f400
	ctx.lr = 0x82C05514;
	sub_82B4F400(ctx, base);
	// 82C05514: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05518: 4BF49EE1  bl 0x82b4f3f8
	ctx.lr = 0x82C0551C;
	sub_82B4F3F8(ctx, base);
	pc = 0x82C0551C; continue 'dispatch;
            }
            0x82C0551C => {
    //   block [0x82C0551C..0x82C05524)
	// 82C0551C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82C05520: 480A3F3C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05528 size=128
    let mut pc: u32 = 0x82C05528;
    'dispatch: loop {
        match pc {
            0x82C05528 => {
    //   block [0x82C05528..0x82C05558)
	// 82C05528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0552C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0553C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05540: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C05544: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C05548: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0554C: 419A000C  beq cr6, 0x82c05558
	if ctx.cr[6].eq {
	pc = 0x82C05558; continue 'dispatch;
	}
	// 82C05550: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C05554: 93CB0020  stw r30, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	pc = 0x82C05558; continue 'dispatch;
            }
            0x82C05558 => {
    //   block [0x82C05558..0x82C05580)
	// 82C05558: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0555C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C05560: 419A0030  beq cr6, 0x82c05590
	if ctx.cr[6].eq {
	pc = 0x82C05590; continue 'dispatch;
	}
	// 82C05564: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0556C: 419A0014  beq cr6, 0x82c05580
	if ctx.cr[6].eq {
	pc = 0x82C05580; continue 'dispatch;
	}
	// 82C05570: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C05574: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05578: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0557C: 4E800421  bctrl
	ctx.lr = 0x82C05580;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C05580 => {
    //   block [0x82C05580..0x82C05590)
	// 82C05580: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C05584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05588: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C0558C: 4BC40225  bl 0x828457b0
	ctx.lr = 0x82C05590;
	sub_828457B0(ctx, base);
	pc = 0x82C05590; continue 'dispatch;
            }
            0x82C05590 => {
    //   block [0x82C05590..0x82C055A8)
	// 82C05590: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0559C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C055A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C055A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C055A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C055A8 size=128
    let mut pc: u32 = 0x82C055A8;
    'dispatch: loop {
        match pc {
            0x82C055A8 => {
    //   block [0x82C055A8..0x82C055D8)
	// 82C055A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C055AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C055B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C055B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C055B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C055BC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C055C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C055C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C055C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C055CC: 419A000C  beq cr6, 0x82c055d8
	if ctx.cr[6].eq {
	pc = 0x82C055D8; continue 'dispatch;
	}
	// 82C055D0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C055D4: 93CB0038  stw r30, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	pc = 0x82C055D8; continue 'dispatch;
            }
            0x82C055D8 => {
    //   block [0x82C055D8..0x82C05600)
	// 82C055D8: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C055DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C055E0: 419A0030  beq cr6, 0x82c05610
	if ctx.cr[6].eq {
	pc = 0x82C05610; continue 'dispatch;
	}
	// 82C055E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C055E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C055EC: 419A0014  beq cr6, 0x82c05600
	if ctx.cr[6].eq {
	pc = 0x82C05600; continue 'dispatch;
	}
	// 82C055F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C055F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C055F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C055FC: 4E800421  bctrl
	ctx.lr = 0x82C05600;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C05600 => {
    //   block [0x82C05600..0x82C05610)
	// 82C05600: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C05604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05608: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C0560C: 4BC401A5  bl 0x828457b0
	ctx.lr = 0x82C05610;
	sub_828457B0(ctx, base);
	pc = 0x82C05610; continue 'dispatch;
            }
            0x82C05610 => {
    //   block [0x82C05610..0x82C05628)
	// 82C05610: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0561C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05628 size=136
    let mut pc: u32 = 0x82C05628;
    'dispatch: loop {
        match pc {
            0x82C05628 => {
    //   block [0x82C05628..0x82C05660)
	// 82C05628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0562C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0563C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05640: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C05644: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05648: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0564C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C05650: 419A0010  beq cr6, 0x82c05660
	if ctx.cr[6].eq {
	pc = 0x82C05660; continue 'dispatch;
	}
	// 82C05654: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C05658: 93C3003C  stw r30, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82C0565C: 4800E255  bl 0x82c138b0
	ctx.lr = 0x82C05660;
	sub_82C138B0(ctx, base);
	pc = 0x82C05660; continue 'dispatch;
            }
            0x82C05660 => {
    //   block [0x82C05660..0x82C05688)
	// 82C05660: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05664: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C05668: 419A0030  beq cr6, 0x82c05698
	if ctx.cr[6].eq {
	pc = 0x82C05698; continue 'dispatch;
	}
	// 82C0566C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C05674: 419A0014  beq cr6, 0x82c05688
	if ctx.cr[6].eq {
	pc = 0x82C05688; continue 'dispatch;
	}
	// 82C05678: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0567C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05680: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C05684: 4E800421  bctrl
	ctx.lr = 0x82C05688;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C05688 => {
    //   block [0x82C05688..0x82C05698)
	// 82C05688: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C0568C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05690: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C05694: 4BC4011D  bl 0x828457b0
	ctx.lr = 0x82C05698;
	sub_828457B0(ctx, base);
	pc = 0x82C05698; continue 'dispatch;
            }
            0x82C05698 => {
    //   block [0x82C05698..0x82C056B0)
	// 82C05698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0569C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C056A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C056A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C056A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C056AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C056B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C056B0 size=128
    let mut pc: u32 = 0x82C056B0;
    'dispatch: loop {
        match pc {
            0x82C056B0 => {
    //   block [0x82C056B0..0x82C056E0)
	// 82C056B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C056B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C056B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C056BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C056C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C056C4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C056C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C056CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C056D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C056D4: 419A000C  beq cr6, 0x82c056e0
	if ctx.cr[6].eq {
	pc = 0x82C056E0; continue 'dispatch;
	}
	// 82C056D8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C056DC: 93CB0084  stw r30, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	pc = 0x82C056E0; continue 'dispatch;
            }
            0x82C056E0 => {
    //   block [0x82C056E0..0x82C05708)
	// 82C056E0: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C056E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C056E8: 419A0030  beq cr6, 0x82c05718
	if ctx.cr[6].eq {
	pc = 0x82C05718; continue 'dispatch;
	}
	// 82C056EC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C056F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C056F4: 419A0014  beq cr6, 0x82c05708
	if ctx.cr[6].eq {
	pc = 0x82C05708; continue 'dispatch;
	}
	// 82C056F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C056FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05700: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C05704: 4E800421  bctrl
	ctx.lr = 0x82C05708;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C05708 => {
    //   block [0x82C05708..0x82C05718)
	// 82C05708: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C0570C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05710: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C05714: 4BC4009D  bl 0x828457b0
	ctx.lr = 0x82C05718;
	sub_828457B0(ctx, base);
	pc = 0x82C05718; continue 'dispatch;
            }
            0x82C05718 => {
    //   block [0x82C05718..0x82C05730)
	// 82C05718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0571C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05724: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0572C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05730 size=64
    let mut pc: u32 = 0x82C05730;
    'dispatch: loop {
        match pc {
            0x82C05730 => {
    //   block [0x82C05730..0x82C05770)
	// 82C05730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0573C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05744: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05748: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C0574C: 394BA284  addi r10, r11, -0x5d7c
	ctx.r[10].s64 = ctx.r[11].s64 + -23932;
	// 82C05750: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C05754: 4B5F20AD  bl 0x821f7800
	ctx.lr = 0x82C05758;
	sub_821F7800(ctx, base);
	// 82C05758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0575C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0576C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05770 size=24
    let mut pc: u32 = 0x82C05770;
    'dispatch: loop {
        match pc {
            0x82C05770 => {
    //   block [0x82C05770..0x82C05788)
	// 82C05770: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C05774: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C05778: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C0577C: 392AA284  addi r9, r10, -0x5d7c
	ctx.r[9].s64 = ctx.r[10].s64 + -23932;
	// 82C05780: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C05784: 4B60F654  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05788 size=52
    let mut pc: u32 = 0x82C05788;
    'dispatch: loop {
        match pc {
            0x82C05788 => {
    //   block [0x82C05788..0x82C057BC)
	// 82C05788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0578C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0579C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C057A0: 4B66FC99  bl 0x82275438
	ctx.lr = 0x82C057A4;
	sub_82275438(ctx, base);
	// 82C057A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C057A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C057AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C057B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C057B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C057B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C057C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C057C0 size=52
    let mut pc: u32 = 0x82C057C0;
    'dispatch: loop {
        match pc {
            0x82C057C0 => {
    //   block [0x82C057C0..0x82C057F4)
	// 82C057C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C057C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C057C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C057CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C057D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C057D4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C057D8: 4B5E7021  bl 0x821ec7f8
	ctx.lr = 0x82C057DC;
	sub_821EC7F8(ctx, base);
	// 82C057DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C057E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C057E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C057E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C057EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C057F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C057F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C057F8 size=52
    let mut pc: u32 = 0x82C057F8;
    'dispatch: loop {
        match pc {
            0x82C057F8 => {
    //   block [0x82C057F8..0x82C0582C)
	// 82C057F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C057FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05800: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05804: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0580C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C05810: 4B5D51B1  bl 0x821da9c0
	ctx.lr = 0x82C05814;
	sub_821DA9C0(ctx, base);
	// 82C05814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0581C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05830 size=52
    let mut pc: u32 = 0x82C05830;
    'dispatch: loop {
        match pc {
            0x82C05830 => {
    //   block [0x82C05830..0x82C05864)
	// 82C05830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0583C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05844: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C05848: 4B591A29  bl 0x82197270
	ctx.lr = 0x82C0584C;
	sub_82197270(ctx, base);
	// 82C0584C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0585C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05868 size=104
    let mut pc: u32 = 0x82C05868;
    'dispatch: loop {
        match pc {
            0x82C05868 => {
    //   block [0x82C05868..0x82C058D0)
	// 82C05868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0586C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0587C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05884: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05888: 4B5F1F79  bl 0x821f7800
	ctx.lr = 0x82C0588C;
	sub_821F7800(ctx, base);
	// 82C0588C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82C05890: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C05894: 388B8864  addi r4, r11, -0x779c
	ctx.r[4].s64 = ctx.r[11].s64 + -30620;
	// 82C05898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0589C: 4B5DE5B5  bl 0x821e3e50
	ctx.lr = 0x82C058A0;
	sub_821E3E50(ctx, base);
	// 82C058A0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C058A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C058A8: 4B5E6F51  bl 0x821ec7f8
	ctx.lr = 0x82C058AC;
	sub_821EC7F8(ctx, base);
	// 82C058AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C058B0: 4B60F529  bl 0x82214dd8
	ctx.lr = 0x82C058B4;
	sub_82214DD8(ctx, base);
	// 82C058B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C058B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C058BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C058C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C058C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C058C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C058CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C058D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C058D0 size=104
    let mut pc: u32 = 0x82C058D0;
    'dispatch: loop {
        match pc {
            0x82C058D0 => {
    //   block [0x82C058D0..0x82C05938)
	// 82C058D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C058D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C058D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C058DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C058E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C058E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C058E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C058EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C058F0: 4B5F1F11  bl 0x821f7800
	ctx.lr = 0x82C058F4;
	sub_821F7800(ctx, base);
	// 82C058F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C058F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C058FC: 388B681C  addi r4, r11, 0x681c
	ctx.r[4].s64 = ctx.r[11].s64 + 26652;
	// 82C05900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05904: 4B5DE54D  bl 0x821e3e50
	ctx.lr = 0x82C05908;
	sub_821E3E50(ctx, base);
	// 82C05908: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C0590C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C05910: 4B5E6EE9  bl 0x821ec7f8
	ctx.lr = 0x82C05914;
	sub_821EC7F8(ctx, base);
	// 82C05914: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05918: 4B60F4C1  bl 0x82214dd8
	ctx.lr = 0x82C0591C;
	sub_82214DD8(ctx, base);
	// 82C0591C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05920: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0592C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05930: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05938 size=104
    let mut pc: u32 = 0x82C05938;
    'dispatch: loop {
        match pc {
            0x82C05938 => {
    //   block [0x82C05938..0x82C059A0)
	// 82C05938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0594C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05954: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05958: 4B5F1EA9  bl 0x821f7800
	ctx.lr = 0x82C0595C;
	sub_821F7800(ctx, base);
	// 82C0595C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C05960: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C05964: 388B0B34  addi r4, r11, 0xb34
	ctx.r[4].s64 = ctx.r[11].s64 + 2868;
	// 82C05968: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0596C: 4B5DE4E5  bl 0x821e3e50
	ctx.lr = 0x82C05970;
	sub_821E3E50(ctx, base);
	// 82C05970: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C05974: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C05978: 4B5E6E81  bl 0x821ec7f8
	ctx.lr = 0x82C0597C;
	sub_821EC7F8(ctx, base);
	// 82C0597C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05980: 4B60F459  bl 0x82214dd8
	ctx.lr = 0x82C05984;
	sub_82214DD8(ctx, base);
	// 82C05984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0598C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05994: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0599C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C059A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C059A0 size=76
    let mut pc: u32 = 0x82C059A0;
    'dispatch: loop {
        match pc {
            0x82C059A0 => {
    //   block [0x82C059A0..0x82C059EC)
	// 82C059A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C059A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C059A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C059AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C059B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C059B4: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82C059B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C059BC: 4B5E8FD5  bl 0x821ee990
	ctx.lr = 0x82C059C0;
	sub_821EE990(ctx, base);
	// 82C059C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C059C4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C059C8: 4B5E6E31  bl 0x821ec7f8
	ctx.lr = 0x82C059CC;
	sub_821EC7F8(ctx, base);
	// 82C059CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C059D0: 4B60F409  bl 0x82214dd8
	ctx.lr = 0x82C059D4;
	sub_82214DD8(ctx, base);
	// 82C059D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C059D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C059DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C059E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C059E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C059E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C059F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C059F0 size=108
    let mut pc: u32 = 0x82C059F0;
    'dispatch: loop {
        match pc {
            0x82C059F0 => {
    //   block [0x82C059F0..0x82C05A24)
	// 82C059F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C059F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C059F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C059FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05A00: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82C05A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05A08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C05A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C05A10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05A14: 419A0010  beq cr6, 0x82c05a24
	if ctx.cr[6].eq {
	pc = 0x82C05A24; continue 'dispatch;
	}
	// 82C05A18: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82C05A1C: 388BFD40  addi r4, r11, -0x2c0
	ctx.r[4].s64 = ctx.r[11].s64 + -704;
	// 82C05A20: 4800000C  b 0x82c05a2c
	pc = 0x82C05A2C; continue 'dispatch;
            }
            0x82C05A24 => {
    //   block [0x82C05A24..0x82C05A2C)
	// 82C05A24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82C05A28: 388B83F0  addi r4, r11, -0x7c10
	ctx.r[4].s64 = ctx.r[11].s64 + -31760;
	pc = 0x82C05A2C; continue 'dispatch;
            }
            0x82C05A2C => {
    //   block [0x82C05A2C..0x82C05A5C)
	// 82C05A2C: 4B6274A5  bl 0x8222ced0
	ctx.lr = 0x82C05A30;
	sub_8222CED0(ctx, base);
	// 82C05A30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C05A34: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C05A38: 4B5E6DC1  bl 0x821ec7f8
	ctx.lr = 0x82C05A3C;
	sub_821EC7F8(ctx, base);
	// 82C05A3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C05A40: 4B60F399  bl 0x82214dd8
	ctx.lr = 0x82C05A44;
	sub_82214DD8(ctx, base);
	// 82C05A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05A48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05A60 size=96
    let mut pc: u32 = 0x82C05A60;
    'dispatch: loop {
        match pc {
            0x82C05A60 => {
    //   block [0x82C05A60..0x82C05AA8)
	// 82C05A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05A74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05A78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05A7C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C05A80: 394BA284  addi r10, r11, -0x5d7c
	ctx.r[10].s64 = ctx.r[11].s64 + -23932;
	// 82C05A84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05A88: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C05A8C: 4B60F34D  bl 0x82214dd8
	ctx.lr = 0x82C05A90;
	sub_82214DD8(ctx, base);
	// 82C05A90: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C05A94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05A98: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C05A9C: 419A000C  beq cr6, 0x82c05aa8
	if ctx.cr[6].eq {
	pc = 0x82C05AA8; continue 'dispatch;
	}
	// 82C05AA0: 4BC3FD11  bl 0x828457b0
	ctx.lr = 0x82C05AA4;
	sub_828457B0(ctx, base);
	// 82C05AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C05AA8; continue 'dispatch;
            }
            0x82C05AA8 => {
    //   block [0x82C05AA8..0x82C05AC0)
	// 82C05AA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05AB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05AC0 size=24
    let mut pc: u32 = 0x82C05AC0;
    'dispatch: loop {
        match pc {
            0x82C05AC0 => {
    //   block [0x82C05AC0..0x82C05AD8)
	// 82C05AC0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05AC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C05AC8: 409A0010  bne cr6, 0x82c05ad8
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82C05AD8);
		return;
	}
	// 82C05ACC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82C05AD0: 386BFFDF  addi r3, r11, -0x21
	ctx.r[3].s64 = ctx.r[11].s64 + -33;
	// 82C05AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05AE0 size=16
    let mut pc: u32 = 0x82C05AE0;
    'dispatch: loop {
        match pc {
            0x82C05AE0 => {
    //   block [0x82C05AE0..0x82C05AF0)
	// 82C05AE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05AE4: 394BA288  addi r10, r11, -0x5d78
	ctx.r[10].s64 = ctx.r[11].s64 + -23928;
	// 82C05AE8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C05AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05AF0 size=16
    let mut pc: u32 = 0x82C05AF0;
    'dispatch: loop {
        match pc {
            0x82C05AF0 => {
    //   block [0x82C05AF0..0x82C05B00)
	// 82C05AF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05AF4: 394BA290  addi r10, r11, -0x5d70
	ctx.r[10].s64 = ctx.r[11].s64 + -23920;
	// 82C05AF8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C05AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05B00 size=16
    let mut pc: u32 = 0x82C05B00;
    'dispatch: loop {
        match pc {
            0x82C05B00 => {
    //   block [0x82C05B00..0x82C05B10)
	// 82C05B00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05B04: 394BA29C  addi r10, r11, -0x5d64
	ctx.r[10].s64 = ctx.r[11].s64 + -23908;
	// 82C05B08: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C05B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05B10 size=16
    let mut pc: u32 = 0x82C05B10;
    'dispatch: loop {
        match pc {
            0x82C05B10 => {
    //   block [0x82C05B10..0x82C05B20)
	// 82C05B10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05B14: 394BA2C0  addi r10, r11, -0x5d40
	ctx.r[10].s64 = ctx.r[11].s64 + -23872;
	// 82C05B18: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C05B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05B20 size=8
    let mut pc: u32 = 0x82C05B20;
    'dispatch: loop {
        match pc {
            0x82C05B20 => {
    //   block [0x82C05B20..0x82C05B28)
	// 82C05B20: 48014A40  b 0x82c1a560
	sub_82C1A560(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05B28 size=36
    let mut pc: u32 = 0x82C05B28;
    'dispatch: loop {
        match pc {
            0x82C05B28 => {
    //   block [0x82C05B28..0x82C05B4C)
	// 82C05B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05B34: 480139E5  bl 0x82c19518
	ctx.lr = 0x82C05B38;
	sub_82C19518(ctx, base);
	// 82C05B38: 48014651  bl 0x82c1a188
	ctx.lr = 0x82C05B3C;
	sub_82C1A188(ctx, base);
	// 82C05B3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05B50 size=8
    let mut pc: u32 = 0x82C05B50;
    'dispatch: loop {
        match pc {
            0x82C05B50 => {
    //   block [0x82C05B50..0x82C05B58)
	// 82C05B50: 480139D8  b 0x82c19528
	crate::recompiler::externs::call(ctx, base, 0x82C19528);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05B58 size=68
    let mut pc: u32 = 0x82C05B58;
    'dispatch: loop {
        match pc {
            0x82C05B58 => {
    //   block [0x82C05B58..0x82C05B9C)
	// 82C05B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05B60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05B64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05B68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05B6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05B70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05B74: 480139A5  bl 0x82c19518
	ctx.lr = 0x82C05B78;
	sub_82C19518(ctx, base);
	// 82C05B78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C05B7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C05B80: 48014919  bl 0x82c1a498
	ctx.lr = 0x82C05B84;
	sub_82C1A498(ctx, base);
	// 82C05B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05BA0 size=36
    let mut pc: u32 = 0x82C05BA0;
    'dispatch: loop {
        match pc {
            0x82C05BA0 => {
    //   block [0x82C05BA0..0x82C05BC4)
	// 82C05BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05BAC: 4801396D  bl 0x82c19518
	ctx.lr = 0x82C05BB0;
	sub_82C19518(ctx, base);
	// 82C05BB0: 48014E39  bl 0x82c1a9e8
	ctx.lr = 0x82C05BB4;
	sub_82C1A9E8(ctx, base);
	// 82C05BB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05BC8 size=36
    let mut pc: u32 = 0x82C05BC8;
    'dispatch: loop {
        match pc {
            0x82C05BC8 => {
    //   block [0x82C05BC8..0x82C05BEC)
	// 82C05BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05BD4: 48013945  bl 0x82c19518
	ctx.lr = 0x82C05BD8;
	sub_82C19518(ctx, base);
	// 82C05BD8: 48013919  bl 0x82c194f0
	ctx.lr = 0x82C05BDC;
	sub_82C194F0(ctx, base);
	// 82C05BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05BF0 size=68
    let mut pc: u32 = 0x82C05BF0;
    'dispatch: loop {
        match pc {
            0x82C05BF0 => {
    //   block [0x82C05BF0..0x82C05C34)
	// 82C05BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05BF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05BFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05C08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05C0C: 4801390D  bl 0x82c19518
	ctx.lr = 0x82C05C10;
	sub_82C19518(ctx, base);
	// 82C05C10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C05C14: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C05C18: 48014FB9  bl 0x82c1abd0
	ctx.lr = 0x82C05C1C;
	sub_82C1ABD0(ctx, base);
	// 82C05C1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05C28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05C2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05C38 size=68
    let mut pc: u32 = 0x82C05C38;
    'dispatch: loop {
        match pc {
            0x82C05C38 => {
    //   block [0x82C05C38..0x82C05C7C)
	// 82C05C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05C40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05C44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05C48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05C4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05C50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05C54: 480138C5  bl 0x82c19518
	ctx.lr = 0x82C05C58;
	sub_82C19518(ctx, base);
	// 82C05C58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C05C5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C05C60: 48015049  bl 0x82c1aca8
	ctx.lr = 0x82C05C64;
	sub_82C1ACA8(ctx, base);
	// 82C05C64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05C70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05C74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05C80 size=124
    let mut pc: u32 = 0x82C05C80;
    'dispatch: loop {
        match pc {
            0x82C05C80 => {
    //   block [0x82C05C80..0x82C05CFC)
	// 82C05C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05C88: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82C05C8C: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05D00 size=68
    let mut pc: u32 = 0x82C05D00;
    'dispatch: loop {
        match pc {
            0x82C05D00 => {
    //   block [0x82C05D00..0x82C05D44)
	// 82C05D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05D08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05D0C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82C05D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05D14: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C05D18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C05D1C: 480137FD  bl 0x82c19518
	ctx.lr = 0x82C05D20;
	sub_82C19518(ctx, base);
	// 82C05D20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C05D24: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C05D28: 480137D1  bl 0x82c194f8
	ctx.lr = 0x82C05D2C;
	sub_82C194F8(ctx, base);
	// 82C05D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05D38: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05D48 size=84
    let mut pc: u32 = 0x82C05D48;
    'dispatch: loop {
        match pc {
            0x82C05D48 => {
    //   block [0x82C05D48..0x82C05D9C)
	// 82C05D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05D50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C05D54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05D58: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C05D5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05D64: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C05D68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05D6C: 480137AD  bl 0x82c19518
	ctx.lr = 0x82C05D70;
	sub_82C19518(ctx, base);
	// 82C05D70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C05D74: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C05D78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C05D7C: 48013785  bl 0x82c19500
	ctx.lr = 0x82C05D80;
	sub_82C19500(ctx, base);
	// 82C05D80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C05D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05D8C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C05D90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C05D94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05DA0 size=52
    let mut pc: u32 = 0x82C05DA0;
    'dispatch: loop {
        match pc {
            0x82C05DA0 => {
    //   block [0x82C05DA0..0x82C05DD4)
	// 82C05DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05DA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05DAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05DB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05DB4: 48013765  bl 0x82c19518
	ctx.lr = 0x82C05DB8;
	sub_82C19518(ctx, base);
	// 82C05DB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C05DBC: 48013755  bl 0x82c19510
	ctx.lr = 0x82C05DC0;
	sub_82C19510(ctx, base);
	// 82C05DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05DD8 size=72
    let mut pc: u32 = 0x82C05DD8;
    'dispatch: loop {
        match pc {
            0x82C05DD8 => {
    //   block [0x82C05DD8..0x82C05E0C)
	// 82C05DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05DEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05DF0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C05DF4: 392BA288  addi r9, r11, -0x5d78
	ctx.r[9].s64 = ctx.r[11].s64 + -23928;
	// 82C05DF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C05DFC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C05E00: 419A000C  beq cr6, 0x82c05e0c
	if ctx.cr[6].eq {
	pc = 0x82C05E0C; continue 'dispatch;
	}
	// 82C05E04: 4BC3F9AD  bl 0x828457b0
	ctx.lr = 0x82C05E08;
	sub_828457B0(ctx, base);
	// 82C05E08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C05E0C; continue 'dispatch;
            }
            0x82C05E0C => {
    //   block [0x82C05E0C..0x82C05E20)
	// 82C05E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05E18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05E20 size=72
    let mut pc: u32 = 0x82C05E20;
    'dispatch: loop {
        match pc {
            0x82C05E20 => {
    //   block [0x82C05E20..0x82C05E54)
	// 82C05E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05E34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05E38: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C05E3C: 392BA290  addi r9, r11, -0x5d70
	ctx.r[9].s64 = ctx.r[11].s64 + -23920;
	// 82C05E40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C05E44: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C05E48: 419A000C  beq cr6, 0x82c05e54
	if ctx.cr[6].eq {
	pc = 0x82C05E54; continue 'dispatch;
	}
	// 82C05E4C: 4BC3F965  bl 0x828457b0
	ctx.lr = 0x82C05E50;
	sub_828457B0(ctx, base);
	// 82C05E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C05E54; continue 'dispatch;
            }
            0x82C05E54 => {
    //   block [0x82C05E54..0x82C05E68)
	// 82C05E54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05E68 size=72
    let mut pc: u32 = 0x82C05E68;
    'dispatch: loop {
        match pc {
            0x82C05E68 => {
    //   block [0x82C05E68..0x82C05E9C)
	// 82C05E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05E70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05E74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05E78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05E7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05E80: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C05E84: 392BA29C  addi r9, r11, -0x5d64
	ctx.r[9].s64 = ctx.r[11].s64 + -23908;
	// 82C05E88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C05E8C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C05E90: 419A000C  beq cr6, 0x82c05e9c
	if ctx.cr[6].eq {
	pc = 0x82C05E9C; continue 'dispatch;
	}
	// 82C05E94: 4BC3F91D  bl 0x828457b0
	ctx.lr = 0x82C05E98;
	sub_828457B0(ctx, base);
	// 82C05E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C05E9C; continue 'dispatch;
            }
            0x82C05E9C => {
    //   block [0x82C05E9C..0x82C05EB0)
	// 82C05E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05EA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05EB0 size=72
    let mut pc: u32 = 0x82C05EB0;
    'dispatch: loop {
        match pc {
            0x82C05EB0 => {
    //   block [0x82C05EB0..0x82C05EE4)
	// 82C05EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05EBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05EC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C05EC8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C05ECC: 392BA2C0  addi r9, r11, -0x5d40
	ctx.r[9].s64 = ctx.r[11].s64 + -23872;
	// 82C05ED0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C05ED4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C05ED8: 419A000C  beq cr6, 0x82c05ee4
	if ctx.cr[6].eq {
	pc = 0x82C05EE4; continue 'dispatch;
	}
	// 82C05EDC: 4BC3F8D5  bl 0x828457b0
	ctx.lr = 0x82C05EE0;
	sub_828457B0(ctx, base);
	// 82C05EE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82C05EE4; continue 'dispatch;
            }
            0x82C05EE4 => {
    //   block [0x82C05EE4..0x82C05EF8)
	// 82C05EE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05EF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05EF8 size=80
    let mut pc: u32 = 0x82C05EF8;
    'dispatch: loop {
        match pc {
            0x82C05EF8 => {
    //   block [0x82C05EF8..0x82C05F28)
	// 82C05EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05F00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05F04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05F08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05F0C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05F10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C05F14: 419A0014  beq cr6, 0x82c05f28
	if ctx.cr[6].eq {
	pc = 0x82C05F28; continue 'dispatch;
	}
	// 82C05F18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C05F1C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C05F20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C05F24: 4E800421  bctrl
	ctx.lr = 0x82C05F28;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82C05F28 => {
    //   block [0x82C05F28..0x82C05F48)
	// 82C05F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C05F2C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C05F30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C05F34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05F48 size=84
    let mut pc: u32 = 0x82C05F48;
    'dispatch: loop {
        match pc {
            0x82C05F48 => {
    //   block [0x82C05F48..0x82C05F9C)
	// 82C05F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05F4C: 480A34B5  bl 0x82ca9400
	ctx.lr = 0x82C05F50;
	sub_82CA93D0(ctx, base);
	// 82C05F50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05F58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C05F5C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C05F60: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C05F64: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82C05F68: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82C05F6C: 480135AD  bl 0x82c19518
	ctx.lr = 0x82C05F70;
	sub_82C19518(ctx, base);
	// 82C05F70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C05F74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05F78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C05F7C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C05F80: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82C05F84: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82C05F88: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82C05F8C: 48014AA5  bl 0x82c1aa30
	ctx.lr = 0x82C05F90;
	sub_82C1AA30(ctx, base);
	// 82C05F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05F94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C05F98: 480A34B8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C05FA0 size=60
    let mut pc: u32 = 0x82C05FA0;
    'dispatch: loop {
        match pc {
            0x82C05FA0 => {
    //   block [0x82C05FA0..0x82C05FDC)
	// 82C05FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05FA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05FAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C05FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C05FB4: 48013565  bl 0x82c19518
	ctx.lr = 0x82C05FB8;
	sub_82C19518(ctx, base);
	// 82C05FB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C05FBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05FC0: 48015149  bl 0x82c1b108
	ctx.lr = 0x82C05FC4;
	sub_82C1B108(ctx, base);
	// 82C05FC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C05FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C05FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C05FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C05FD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C05FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C05FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C05FE0 size=84
    let mut pc: u32 = 0x82C05FE0;
    'dispatch: loop {
        match pc {
            0x82C05FE0 => {
    //   block [0x82C05FE0..0x82C06034)
	// 82C05FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C05FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C05FE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C05FEC: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C06038 size=76
    let mut pc: u32 = 0x82C06038;
    'dispatch: loop {
        match pc {
            0x82C06038 => {
    //   block [0x82C06038..0x82C06084)
	// 82C06038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0603C: 480A33D1  bl 0x82ca940c
	ctx.lr = 0x82C06040;
	sub_82CA93D0(ctx, base);
	// 82C06040: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C06044: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C06048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0604C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C06050: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C06054: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C06058: 480134C1  bl 0x82c19518
	ctx.lr = 0x82C0605C;
	sub_82C19518(ctx, base);
	// 82C0605C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C06060: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C06064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C06068: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C0606C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C06070: 48014321  bl 0x82c1a390
	ctx.lr = 0x82C06074;
	sub_82C1A390(ctx, base);
	// 82C06074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C06078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0607C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C06080: 480A33DC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06088 size=16
    let mut pc: u32 = 0x82C06088;
    'dispatch: loop {
        match pc {
            0x82C06088 => {
    //   block [0x82C06088..0x82C06098)
	// 82C06088: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82C0608C: 394BAB50  addi r10, r11, -0x54b0
	ctx.r[10].s64 = ctx.r[11].s64 + -21680;
	// 82C06090: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C06094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06098 size=12
    let mut pc: u32 = 0x82C06098;
    'dispatch: loop {
        match pc {
            0x82C06098 => {
    //   block [0x82C06098..0x82C060A4)
	// 82C06098: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0609C: 908B004C  stw r4, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[4].u32 ) };
	// 82C060A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C060A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C060A8 size=84
    let mut pc: u32 = 0x82C060A8;
    'dispatch: loop {
        match pc {
            0x82C060A8 => {
    //   block [0x82C060A8..0x82C060FC)
	// 82C060A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C060AC: 480A3361  bl 0x82ca940c
	ctx.lr = 0x82C060B0;
	sub_82CA93D0(ctx, base);
	// 82C060B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C060B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C060B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C060BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C060C0: 3BABA328  addi r29, r11, -0x5cd8
	ctx.r[29].s64 = ctx.r[11].s64 + -23768;
	// 82C060C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C060C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C060CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C060D0: 480022F9  bl 0x82c083c8
	ctx.lr = 0x82C060D4;
	sub_82C083C8(ctx, base);
	// 82C060D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C060D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C060DC: 4801E2FD  bl 0x82c243d8
	ctx.lr = 0x82C060E0;
	sub_82C243D8(ctx, base);
	// 82C060E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C060E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C060E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C060EC: 4800244D  bl 0x82c08538
	ctx.lr = 0x82C060F0;
	sub_82C08538(ctx, base);
	// 82C060F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C060F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C060F8: 480A3364  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06100 size=16
    let mut pc: u32 = 0x82C06100;
    'dispatch: loop {
        match pc {
            0x82C06100 => {
    //   block [0x82C06100..0x82C06110)
	// 82C06100: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C06104: 394BA31C  addi r10, r11, -0x5ce4
	ctx.r[10].s64 = ctx.r[11].s64 + -23780;
	// 82C06108: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0610C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06110 size=16
    let mut pc: u32 = 0x82C06110;
    'dispatch: loop {
        match pc {
            0x82C06110 => {
    //   block [0x82C06110..0x82C06120)
	// 82C06110: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C06114: 394BA334  addi r10, r11, -0x5ccc
	ctx.r[10].s64 = ctx.r[11].s64 + -23756;
	// 82C06118: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0611C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06120 size=8
    let mut pc: u32 = 0x82C06120;
    'dispatch: loop {
        match pc {
            0x82C06120 => {
    //   block [0x82C06120..0x82C06128)
	// 82C06120: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C06124: 4801EBE4  b 0x82c24d08
	sub_82C24D08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06128 size=8
    let mut pc: u32 = 0x82C06128;
    'dispatch: loop {
        match pc {
            0x82C06128 => {
    //   block [0x82C06128..0x82C06130)
	// 82C06128: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0612C: 4801EB2C  b 0x82c24c58
	sub_82C24C58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06130 size=8
    let mut pc: u32 = 0x82C06130;
    'dispatch: loop {
        match pc {
            0x82C06130 => {
    //   block [0x82C06130..0x82C06138)
	// 82C06130: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C06134: 4801015C  b 0x82c16290
	sub_82C16290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06138 size=8
    let mut pc: u32 = 0x82C06138;
    'dispatch: loop {
        match pc {
            0x82C06138 => {
    //   block [0x82C06138..0x82C06140)
	// 82C06138: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0613C: 480111F4  b 0x82c17330
	sub_82C17330(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06140 size=8
    let mut pc: u32 = 0x82C06140;
    'dispatch: loop {
        match pc {
            0x82C06140 => {
    //   block [0x82C06140..0x82C06148)
	// 82C06140: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C06144: 48012BBC  b 0x82c18d00
	sub_82C18D00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C06148 size=96
    let mut pc: u32 = 0x82C06148;
    'dispatch: loop {
        match pc {
            0x82C06148 => {
    //   block [0x82C06148..0x82C06190)
	// 82C06148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0614C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C06150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C06154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C06158: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0615C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C06160: 48012E31  bl 0x82c18f90
	ctx.lr = 0x82C06164;
	sub_82C18F90(ctx, base);
	// 82C06164: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C06168: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0616C: 419A0024  beq cr6, 0x82c06190
	if ctx.cr[6].eq {
	pc = 0x82C06190; continue 'dispatch;
	}
	// 82C06170: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C06174: 4800E865  bl 0x82c149d8
	ctx.lr = 0x82C06178;
	sub_82C149D8(ctx, base);
	// 82C06178: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C0617C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C06180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C06184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C06188: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0618C: 4E800020  blr
	return;
            }
            0x82C06190 => {
    //   block [0x82C06190..0x82C061A8)
	// 82C06190: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C06194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C06198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0619C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C061A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C061A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C061B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C061B8 size=8
    let mut pc: u32 = 0x82C061B8;
    'dispatch: loop {
        match pc {
            0x82C061B8 => {
    //   block [0x82C061B8..0x82C061C0)
	// 82C061B8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C061BC: 4B61ED04  b 0x82224ec0
	sub_82224EC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C061C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C061C0 size=36
    let mut pc: u32 = 0x82C061C0;
    'dispatch: loop {
        match pc {
            0x82C061C0 => {
    //   block [0x82C061C0..0x82C061E4)
	// 82C061C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C061C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C061C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C061CC: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C061D0: 4800B9A9  bl 0x82c11b78
	ctx.lr = 0x82C061D4;
	sub_82C11B78(ctx, base);
	// 82C061D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C061D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C061DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C061E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C061E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C061E8 size=8
    let mut pc: u32 = 0x82C061E8;
    'dispatch: loop {
        match pc {
            0x82C061E8 => {
    //   block [0x82C061E8..0x82C061F0)
	// 82C061E8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C061EC: 4800B9B4  b 0x82c11ba0
	sub_82C11BA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C061F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C061F0 size=8
    let mut pc: u32 = 0x82C061F0;
    'dispatch: loop {
        match pc {
            0x82C061F0 => {
    //   block [0x82C061F0..0x82C061F8)
	// 82C061F0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C061F4: 48010C54  b 0x82c16e48
	sub_82C16E48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C061F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C061F8 size=8
    let mut pc: u32 = 0x82C061F8;
    'dispatch: loop {
        match pc {
            0x82C061F8 => {
    //   block [0x82C061F8..0x82C06200)
	// 82C061F8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C061FC: 4800B9AC  b 0x82c11ba8
	sub_82C11BA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06200 size=8
    let mut pc: u32 = 0x82C06200;
    'dispatch: loop {
        match pc {
            0x82C06200 => {
    //   block [0x82C06200..0x82C06208)
	// 82C06200: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C06204: 4800B9BC  b 0x82c11bc0
	sub_82C11BC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06208 size=8
    let mut pc: u32 = 0x82C06208;
    'dispatch: loop {
        match pc {
            0x82C06208 => {
    //   block [0x82C06208..0x82C06210)
	// 82C06208: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0620C: 4800B9CC  b 0x82c11bd8
	sub_82C11BD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06210 size=8
    let mut pc: u32 = 0x82C06210;
    'dispatch: loop {
        match pc {
            0x82C06210 => {
    //   block [0x82C06210..0x82C06218)
	// 82C06210: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C06214: 4800B9CC  b 0x82c11be0
	sub_82C11BE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06218 size=8
    let mut pc: u32 = 0x82C06218;
    'dispatch: loop {
        match pc {
            0x82C06218 => {
    //   block [0x82C06218..0x82C06220)
	// 82C06218: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0621C: 4800B9CC  b 0x82c11be8
	sub_82C11BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06220 size=8
    let mut pc: u32 = 0x82C06220;
    'dispatch: loop {
        match pc {
            0x82C06220 => {
    //   block [0x82C06220..0x82C06228)
	// 82C06220: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C06224: 4800B9CC  b 0x82c11bf0
	sub_82C11BF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06228 size=16
    let mut pc: u32 = 0x82C06228;
    'dispatch: loop {
        match pc {
            0x82C06228 => {
    //   block [0x82C06228..0x82C06238)
	// 82C06228: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C0622C: 394BA368  addi r10, r11, -0x5c98
	ctx.r[10].s64 = ctx.r[11].s64 + -23704;
	// 82C06230: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C06234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06238 size=16
    let mut pc: u32 = 0x82C06238;
    'dispatch: loop {
        match pc {
            0x82C06238 => {
    //   block [0x82C06238..0x82C06248)
	// 82C06238: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C0623C: 394BA308  addi r10, r11, -0x5cf8
	ctx.r[10].s64 = ctx.r[11].s64 + -23800;
	// 82C06240: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C06244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C06248 size=36
    let mut pc: u32 = 0x82C06248;
    'dispatch: loop {
        match pc {
            0x82C06248 => {
    //   block [0x82C06248..0x82C0626C)
	// 82C06248: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C0624C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C06250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C06254: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C06258: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0625C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C06260: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82C06264: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C06268: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C06270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C06270 size=100
    let mut pc: u32 = 0x82C06270;
    'dispatch: loop {
        match pc {
            0x82C06270 => {
    //   block [0x82C06270..0x82C062B4)
	// 82C06270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C06274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C06278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0627C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C06280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C06284: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C06288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C0628C: 392BA378  addi r9, r11, -0x5c88
	ctx.r[9].s64 = ctx.r[11].s64 + -23688;
	// 82C06290: 386000A8  li r3, 0xa8
	ctx.r[3].s64 = 168;
	// 82C06294: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C06298: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0629C: 4B618FBD  bl 0x8221f258
	ctx.lr = 0x82C062A0;
	sub_8221F258(ctx, base);
	// 82C062A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C062A4: 419A0010  beq cr6, 0x82c062b4
	if ctx.cr[6].eq {
	pc = 0x82C062B4; continue 'dispatch;
	}
	// 82C062A8: 4801B8E9  bl 0x82c21b90
	ctx.lr = 0x82C062AC;
	sub_82C21B90(ctx, base);
	// 82C062AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C062B0: 48000008  b 0x82c062b8
	pc = 0x82C062B8; continue 'dispatch;
            }
            0x82C062B4 => {
    //   block [0x82C062B4..0x82C062B8)
	// 82C062B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82C062B8; continue 'dispatch;
            }
            0x82C062B8 => {
    //   block [0x82C062B8..0x82C062D4)
	// 82C062B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C062BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C062C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C062C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C062C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C062CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C062D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


