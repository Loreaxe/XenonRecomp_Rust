pub fn sub_82E9B488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9B488 size=176
    let mut pc: u32 = 0x82E9B488;
    'dispatch: loop {
        match pc {
            0x82E9B488 => {
    //   block [0x82E9B488..0x82E9B538)
	// 82E9B488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9B48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9B490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9B494: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9B498: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9B49C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B4A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B4A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9B4A8: 4E800421  bctrl
	ctx.lr = 0x82E9B4AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9B4AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9B4B0: 41820038  beq 0x82e9b4e8
	if ctx.cr[0].eq {
	pc = 0x82E9B4E8; continue 'dispatch;
	}
	// 82E9B4B4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82E9B4B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9B4BC: 4BF8EC6D  bl 0x82e2a128
	ctx.lr = 0x82E9B4C0;
	sub_82E2A128(ctx, base);
	// 82E9B4C0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9B4C4: 4BFFFFC5  bl 0x82e9b488
	ctx.lr = 0x82E9B4C8;
	sub_82E9B488(ctx, base);
	// 82E9B4C8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E9B4CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9B4D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9B4D4: 419A000C  beq cr6, 0x82e9b4e0
	if ctx.cr[6].eq {
	pc = 0x82E9B4E0; continue 'dispatch;
	}
	// 82E9B4D8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E9B4DC: 4B4253B5  bl 0x822c0890
	ctx.lr = 0x82E9B4E0;
	sub_822C0890(ctx, base);
	// 82E9B4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9B4E4: 48000040  b 0x82e9b524
	pc = 0x82E9B524; continue 'dispatch;
	// 82E9B4E8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9B4EC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9B4F0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E9B4F4: 4082002C  bne 0x82e9b520
	if !ctx.cr[0].eq {
	pc = 0x82E9B520; continue 'dispatch;
	}
	// 82E9B4F8: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9B4FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9B500: 41820020  beq 0x82e9b520
	if ctx.cr[0].eq {
	pc = 0x82E9B520; continue 'dispatch;
	}
	// 82E9B504: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E9B508: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9B50C: 41820014  beq 0x82e9b520
	if ctx.cr[0].eq {
	pc = 0x82E9B520; continue 'dispatch;
	}
	// 82E9B510: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9B514: 4BFF8205  bl 0x82e93718
	ctx.lr = 0x82E9B518;
	sub_82E93718(ctx, base);
	// 82E9B518: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9B51C: 4BFFAFBD  bl 0x82e964d8
	ctx.lr = 0x82E9B520;
	sub_82E964D8(ctx, base);
	// 82E9B520: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9B524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9B528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9B52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9B530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9B534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9B538 size=240
    let mut pc: u32 = 0x82E9B538;
    'dispatch: loop {
        match pc {
            0x82E9B538 => {
    //   block [0x82E9B538..0x82E9B628)
	// 82E9B538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9B53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9B540: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9B544: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9B548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9B54C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9B550: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82E9B554: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9B558: 4BF8EBD1  bl 0x82e2a128
	ctx.lr = 0x82E9B55C;
	sub_82E2A128(ctx, base);
	// 82E9B55C: 48000050  b 0x82e9b5ac
	pc = 0x82E9B5AC; continue 'dispatch;
	// 82E9B560: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9B568: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B56C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9B570: 4E800421  bctrl
	ctx.lr = 0x82E9B574;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9B574: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82E9B578: 419A0078  beq cr6, 0x82e9b5f0
	if ctx.cr[6].eq {
	pc = 0x82E9B5F0; continue 'dispatch;
	}
	// 82E9B57C: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82E9B580: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E9B584: 4BF8EBA5  bl 0x82e2a128
	ctx.lr = 0x82E9B588;
	sub_82E2A128(ctx, base);
	// 82E9B588: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E9B58C: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E9B590: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E9B594: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E9B598: 4B428EC9  bl 0x822c4460
	ctx.lr = 0x82E9B59C;
	sub_822C4460(ctx, base);
	// 82E9B59C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E9B5A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9B5A4: 419A0008  beq cr6, 0x82e9b5ac
	if ctx.cr[6].eq {
	pc = 0x82E9B5AC; continue 'dispatch;
	}
	// 82E9B5A8: 4B4252E9  bl 0x822c0890
	ctx.lr = 0x82E9B5AC;
	sub_822C0890(ctx, base);
	// 82E9B5AC: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9B5B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E9B5B4: 409AFFAC  bne cr6, 0x82e9b560
	if !ctx.cr[6].eq {
	pc = 0x82E9B560; continue 'dispatch;
	}
	// 82E9B5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9B5BC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E9B5C0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B5C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9B5C8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9B5CC: 419A0008  beq cr6, 0x82e9b5d4
	if ctx.cr[6].eq {
	pc = 0x82E9B5D4; continue 'dispatch;
	}
	// 82E9B5D0: 4B4252C1  bl 0x822c0890
	ctx.lr = 0x82E9B5D4;
	sub_822C0890(ctx, base);
	// 82E9B5D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9B5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9B5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9B5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9B5E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9B5E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9B5EC: 4E800020  blr
	return;
	// 82E9B5F0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E9B5F4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E9B5F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9B5FC: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E9B600: 419AFFD4  beq cr6, 0x82e9b5d4
	if ctx.cr[6].eq {
	pc = 0x82E9B5D4; continue 'dispatch;
	}
	// 82E9B604: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82E9B608: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E9B60C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E9B610: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E9B614: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9B618: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E9B61C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E9B620: 4082FFE8  bne 0x82e9b608
	if !ctx.cr[0].eq {
	pc = 0x82E9B608; continue 'dispatch;
	}
	// 82E9B624: 4BFFFFAC  b 0x82e9b5d0
	pc = 0x82E9B5D0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B628 size=64
    let mut pc: u32 = 0x82E9B628;
    'dispatch: loop {
        match pc {
            0x82E9B628 => {
    //   block [0x82E9B628..0x82E9B668)
	// 82E9B628: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B62C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B630: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9B634: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B638: 892A0049  lbz r9, 0x49(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B63C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B640: 409A0008  bne cr6, 0x82e9b648
	if !ctx.cr[6].eq {
	pc = 0x82E9B648; continue 'dispatch;
	}
	// 82E9B644: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E9B648: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B64C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9B650: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B654: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B658: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B65C: 409A000C  bne cr6, 0x82e9b668
	if !ctx.cr[6].eq {
		sub_82E9B668(ctx, base);
		return;
	}
	// 82E9B660: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9B664: 48000020  b 0x82e9b684
	sub_82E9B680(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B668 size=24
    let mut pc: u32 = 0x82E9B668;
    'dispatch: loop {
        match pc {
            0x82E9B668 => {
    //   block [0x82E9B668..0x82E9B680)
	// 82E9B668: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B66C: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B670: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B674: 409A000C  bne cr6, 0x82e9b680
	if !ctx.cr[6].eq {
		sub_82E9B680(ctx, base);
		return;
	}
	// 82E9B678: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9B67C: 48000008  b 0x82e9b684
	sub_82E9B680(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B680 size=16
    let mut pc: u32 = 0x82E9B680;
    'dispatch: loop {
        match pc {
            0x82E9B680 => {
    //   block [0x82E9B680..0x82E9B690)
	// 82E9B680: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B684: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82E9B688: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9B68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B690 size=64
    let mut pc: u32 = 0x82E9B690;
    'dispatch: loop {
        match pc {
            0x82E9B690 => {
    //   block [0x82E9B690..0x82E9B6D0)
	// 82E9B690: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B694: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B698: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9B69C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B6A0: 892A0041  lbz r9, 0x41(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B6A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B6A8: 409A0008  bne cr6, 0x82e9b6b0
	if !ctx.cr[6].eq {
	pc = 0x82E9B6B0; continue 'dispatch;
	}
	// 82E9B6AC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E9B6B0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B6B4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9B6B8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B6BC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B6C0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B6C4: 409A000C  bne cr6, 0x82e9b6d0
	if !ctx.cr[6].eq {
		sub_82E9B6D0(ctx, base);
		return;
	}
	// 82E9B6C8: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9B6CC: 48000020  b 0x82e9b6ec
	sub_82E9B6E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B6D0 size=24
    let mut pc: u32 = 0x82E9B6D0;
    'dispatch: loop {
        match pc {
            0x82E9B6D0 => {
    //   block [0x82E9B6D0..0x82E9B6E8)
	// 82E9B6D0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B6D4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B6D8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B6DC: 409A000C  bne cr6, 0x82e9b6e8
	if !ctx.cr[6].eq {
		sub_82E9B6E8(ctx, base);
		return;
	}
	// 82E9B6E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9B6E4: 48000008  b 0x82e9b6ec
	sub_82E9B6E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B6E8 size=16
    let mut pc: u32 = 0x82E9B6E8;
    'dispatch: loop {
        match pc {
            0x82E9B6E8 => {
    //   block [0x82E9B6E8..0x82E9B6F8)
	// 82E9B6E8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B6EC: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82E9B6F0: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9B6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B6F8 size=16
    let mut pc: u32 = 0x82E9B6F8;
    'dispatch: loop {
        match pc {
            0x82E9B6F8 => {
    //   block [0x82E9B6F8..0x82E9B708)
	// 82E9B6F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B6FC: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B700: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B704: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B708 size=24
    let mut pc: u32 = 0x82E9B708;
    'dispatch: loop {
        match pc {
            0x82E9B708 => {
    //   block [0x82E9B708..0x82E9B720)
	// 82E9B708: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B70C: 892A0049  lbz r9, 0x49(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B710: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B714: 409A0040  bne cr6, 0x82e9b754
	if !ctx.cr[6].eq {
		sub_82E9B73C(ctx, base);
		return;
	}
	// 82E9B718: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B71C: 4800000C  b 0x82e9b728
	sub_82E9B720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B720 size=28
    let mut pc: u32 = 0x82E9B720;
    'dispatch: loop {
        match pc {
            0x82E9B720 => {
    //   block [0x82E9B720..0x82E9B73C)
	// 82E9B720: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9B724: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B728: 892B0049  lbz r9, 0x49(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B72C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B730: 419AFFF0  beq cr6, 0x82e9b720
	if ctx.cr[6].eq {
	pc = 0x82E9B720; continue 'dispatch;
	}
	// 82E9B734: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9B738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B73C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B73C size=48
    let mut pc: u32 = 0x82E9B73C;
    'dispatch: loop {
        match pc {
            0x82E9B73C => {
    //   block [0x82E9B73C..0x82E9B76C)
	// 82E9B73C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B740: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B744: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B748: 409A001C  bne cr6, 0x82e9b764
	if !ctx.cr[6].eq {
	pc = 0x82E9B764; continue 'dispatch;
	}
	// 82E9B74C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B750: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9B754: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B758: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B75C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B760: 419AFFDC  beq cr6, 0x82e9b73c
	if ctx.cr[6].eq {
	pc = 0x82E9B73C; continue 'dispatch;
	}
	// 82E9B764: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B770 size=24
    let mut pc: u32 = 0x82E9B770;
    'dispatch: loop {
        match pc {
            0x82E9B770 => {
    //   block [0x82E9B770..0x82E9B788)
	// 82E9B770: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B774: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B778: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B77C: 419A000C  beq cr6, 0x82e9b788
	if ctx.cr[6].eq {
		sub_82E9B788(ctx, base);
		return;
	}
	// 82E9B780: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B784: 48000070  b 0x82e9b7f4
	sub_82E9B7F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B788 size=24
    let mut pc: u32 = 0x82E9B788;
    'dispatch: loop {
        match pc {
            0x82E9B788 => {
    //   block [0x82E9B788..0x82E9B7A0)
	// 82E9B788: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B78C: 892A0041  lbz r9, 0x41(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B790: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B794: 409A0040  bne cr6, 0x82e9b7d4
	if !ctx.cr[6].eq {
		sub_82E9B7BC(ctx, base);
		return;
	}
	// 82E9B798: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B79C: 4800000C  b 0x82e9b7a8
	sub_82E9B7A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B7A0 size=28
    let mut pc: u32 = 0x82E9B7A0;
    'dispatch: loop {
        match pc {
            0x82E9B7A0 => {
    //   block [0x82E9B7A0..0x82E9B7BC)
	// 82E9B7A0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9B7A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B7A8: 892B0041  lbz r9, 0x41(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B7AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B7B0: 419AFFF0  beq cr6, 0x82e9b7a0
	if ctx.cr[6].eq {
	pc = 0x82E9B7A0; continue 'dispatch;
	}
	// 82E9B7B4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9B7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B7BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B7BC size=56
    let mut pc: u32 = 0x82E9B7BC;
    'dispatch: loop {
        match pc {
            0x82E9B7BC => {
    //   block [0x82E9B7BC..0x82E9B7F4)
	// 82E9B7BC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B7C0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B7C4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B7C8: 409A001C  bne cr6, 0x82e9b7e4
	if !ctx.cr[6].eq {
	pc = 0x82E9B7E4; continue 'dispatch;
	}
	// 82E9B7CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B7D0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9B7D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B7D8: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B7DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B7E0: 419AFFDC  beq cr6, 0x82e9b7bc
	if ctx.cr[6].eq {
	pc = 0x82E9B7BC; continue 'dispatch;
	}
	// 82E9B7E4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B7E8: 894A0041  lbz r10, 0x41(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B7EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B7F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B7F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B7F4 size=8
    let mut pc: u32 = 0x82E9B7F4;
    'dispatch: loop {
        match pc {
            0x82E9B7F4 => {
    //   block [0x82E9B7F4..0x82E9B7FC)
	// 82E9B7F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B800 size=24
    let mut pc: u32 = 0x82E9B800;
    'dispatch: loop {
        match pc {
            0x82E9B800 => {
    //   block [0x82E9B800..0x82E9B818)
	// 82E9B800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B804: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B808: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B80C: 419A000C  beq cr6, 0x82e9b818
	if ctx.cr[6].eq {
		sub_82E9B818(ctx, base);
		return;
	}
	// 82E9B810: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B814: 48000070  b 0x82e9b884
	sub_82E9B884(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B818 size=24
    let mut pc: u32 = 0x82E9B818;
    'dispatch: loop {
        match pc {
            0x82E9B818 => {
    //   block [0x82E9B818..0x82E9B830)
	// 82E9B818: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B81C: 892A0049  lbz r9, 0x49(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B820: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B824: 409A0040  bne cr6, 0x82e9b864
	if !ctx.cr[6].eq {
		sub_82E9B84C(ctx, base);
		return;
	}
	// 82E9B828: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B82C: 4800000C  b 0x82e9b838
	sub_82E9B830(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B830 size=28
    let mut pc: u32 = 0x82E9B830;
    'dispatch: loop {
        match pc {
            0x82E9B830 => {
    //   block [0x82E9B830..0x82E9B84C)
	// 82E9B830: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9B834: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B838: 892B0049  lbz r9, 0x49(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B83C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B840: 419AFFF0  beq cr6, 0x82e9b830
	if ctx.cr[6].eq {
	pc = 0x82E9B830; continue 'dispatch;
	}
	// 82E9B844: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9B848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B84C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B84C size=56
    let mut pc: u32 = 0x82E9B84C;
    'dispatch: loop {
        match pc {
            0x82E9B84C => {
    //   block [0x82E9B84C..0x82E9B884)
	// 82E9B84C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B850: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B854: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B858: 409A001C  bne cr6, 0x82e9b874
	if !ctx.cr[6].eq {
	pc = 0x82E9B874; continue 'dispatch;
	}
	// 82E9B85C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B860: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9B864: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B868: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B86C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B870: 419AFFDC  beq cr6, 0x82e9b84c
	if ctx.cr[6].eq {
	pc = 0x82E9B84C; continue 'dispatch;
	}
	// 82E9B874: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B878: 894A0049  lbz r10, 0x49(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B87C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B880: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B884 size=8
    let mut pc: u32 = 0x82E9B884;
    'dispatch: loop {
        match pc {
            0x82E9B884 => {
    //   block [0x82E9B884..0x82E9B88C)
	// 82E9B884: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B890 size=64
    let mut pc: u32 = 0x82E9B890;
    'dispatch: loop {
        match pc {
            0x82E9B890 => {
    //   block [0x82E9B890..0x82E9B8D0)
	// 82E9B890: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B894: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B898: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9B89C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B8A0: 892A0049  lbz r9, 0x49(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B8A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B8A8: 409A0008  bne cr6, 0x82e9b8b0
	if !ctx.cr[6].eq {
	pc = 0x82E9B8B0; continue 'dispatch;
	}
	// 82E9B8AC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E9B8B0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B8B4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9B8B8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B8BC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B8C0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B8C4: 409A000C  bne cr6, 0x82e9b8d0
	if !ctx.cr[6].eq {
		sub_82E9B8D0(ctx, base);
		return;
	}
	// 82E9B8C8: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9B8CC: 48000020  b 0x82e9b8ec
	sub_82E9B8E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B8D0 size=24
    let mut pc: u32 = 0x82E9B8D0;
    'dispatch: loop {
        match pc {
            0x82E9B8D0 => {
    //   block [0x82E9B8D0..0x82E9B8E8)
	// 82E9B8D0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B8D4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B8D8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9B8DC: 409A000C  bne cr6, 0x82e9b8e8
	if !ctx.cr[6].eq {
		sub_82E9B8E8(ctx, base);
		return;
	}
	// 82E9B8E0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9B8E4: 48000008  b 0x82e9b8ec
	sub_82E9B8E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B8E8 size=16
    let mut pc: u32 = 0x82E9B8E8;
    'dispatch: loop {
        match pc {
            0x82E9B8E8 => {
    //   block [0x82E9B8E8..0x82E9B8F8)
	// 82E9B8E8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9B8EC: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E9B8F0: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9B8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B8F8 size=12
    let mut pc: u32 = 0x82E9B8F8;
    'dispatch: loop {
        match pc {
            0x82E9B8F8 => {
    //   block [0x82E9B8F8..0x82E9B904)
	// 82E9B8F8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B8FC: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B900: 48000048  b 0x82e9b948
	sub_82E9B940(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B904 size=60
    let mut pc: u32 = 0x82E9B904;
    'dispatch: loop {
        match pc {
            0x82E9B904 => {
    //   block [0x82E9B904..0x82E9B940)
	// 82E9B904: 3969000C  addi r11, r9, 0xc
	ctx.r[11].s64 = ctx.r[9].s64 + 12;
	// 82E9B908: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82E9B90C: 38EB0034  addi r7, r11, 0x34
	ctx.r[7].s64 = ctx.r[11].s64 + 52;
	// 82E9B910: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B914: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B918: 7D064051  subf. r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9B91C: 40820014  bne 0x82e9b930
	if !ctx.cr[0].eq {
	pc = 0x82E9B930; continue 'dispatch;
	}
	// 82E9B920: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9B924: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9B928: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82E9B92C: 409AFFE4  bne cr6, 0x82e9b910
	if !ctx.cr[6].eq {
	pc = 0x82E9B910; continue 'dispatch;
	}
	// 82E9B930: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9B934: 4080000C  bge 0x82e9b940
	if !ctx.cr[0].lt {
		sub_82E9B940(ctx, base);
		return;
	}
	// 82E9B938: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B93C: 4800000C  b 0x82e9b948
	sub_82E9B940(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B940 size=24
    let mut pc: u32 = 0x82E9B940;
    'dispatch: loop {
        match pc {
            0x82E9B940 => {
    //   block [0x82E9B940..0x82E9B958)
	// 82E9B940: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E9B944: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B948: 89690049  lbz r11, 0x49(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9B94C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9B950: 419AFFB4  beq cr6, 0x82e9b904
	if ctx.cr[6].eq {
		sub_82E9B904(ctx, base);
		return;
	}
	// 82E9B954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B958 size=12
    let mut pc: u32 = 0x82E9B958;
    'dispatch: loop {
        match pc {
            0x82E9B958 => {
    //   block [0x82E9B958..0x82E9B964)
	// 82E9B958: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B95C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9B960: 48000048  b 0x82e9b9a8
	sub_82E9B9A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B964(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B964 size=60
    let mut pc: u32 = 0x82E9B964;
    'dispatch: loop {
        match pc {
            0x82E9B964 => {
    //   block [0x82E9B964..0x82E9B9A0)
	// 82E9B964: 3969000C  addi r11, r9, 0xc
	ctx.r[11].s64 = ctx.r[9].s64 + 12;
	// 82E9B968: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82E9B96C: 38EB0028  addi r7, r11, 0x28
	ctx.r[7].s64 = ctx.r[11].s64 + 40;
	// 82E9B970: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B974: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B978: 7D064051  subf. r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9B97C: 40820014  bne 0x82e9b990
	if !ctx.cr[0].eq {
	pc = 0x82E9B990; continue 'dispatch;
	}
	// 82E9B980: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9B984: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9B988: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82E9B98C: 409AFFE4  bne cr6, 0x82e9b970
	if !ctx.cr[6].eq {
	pc = 0x82E9B970; continue 'dispatch;
	}
	// 82E9B990: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9B994: 4080000C  bge 0x82e9b9a0
	if !ctx.cr[0].lt {
		sub_82E9B9A0(ctx, base);
		return;
	}
	// 82E9B998: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B99C: 4800000C  b 0x82e9b9a8
	sub_82E9B9A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B9A0 size=24
    let mut pc: u32 = 0x82E9B9A0;
    'dispatch: loop {
        match pc {
            0x82E9B9A0 => {
    //   block [0x82E9B9A0..0x82E9B9B8)
	// 82E9B9A0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E9B9A4: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B9A8: 89690041  lbz r11, 0x41(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B9AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9B9B0: 419AFFB4  beq cr6, 0x82e9b964
	if ctx.cr[6].eq {
		sub_82E9B964(ctx, base);
		return;
	}
	// 82E9B9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B9B8 size=16
    let mut pc: u32 = 0x82E9B9B8;
    'dispatch: loop {
        match pc {
            0x82E9B9B8 => {
    //   block [0x82E9B9B8..0x82E9B9C8)
	// 82E9B9B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B9BC: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B9C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9B9C4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B9C8 size=24
    let mut pc: u32 = 0x82E9B9C8;
    'dispatch: loop {
        match pc {
            0x82E9B9C8 => {
    //   block [0x82E9B9C8..0x82E9B9E0)
	// 82E9B9C8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9B9CC: 892A0041  lbz r9, 0x41(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B9D0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B9D4: 409A0040  bne cr6, 0x82e9ba14
	if !ctx.cr[6].eq {
		sub_82E9B9FC(ctx, base);
		return;
	}
	// 82E9B9D8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B9DC: 4800000C  b 0x82e9b9e8
	sub_82E9B9E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B9E0 size=28
    let mut pc: u32 = 0x82E9B9E0;
    'dispatch: loop {
        match pc {
            0x82E9B9E0 => {
    //   block [0x82E9B9E0..0x82E9B9FC)
	// 82E9B9E0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9B9E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9B9E8: 892B0041  lbz r9, 0x41(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9B9EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9B9F0: 419AFFF0  beq cr6, 0x82e9b9e0
	if ctx.cr[6].eq {
	pc = 0x82E9B9E0; continue 'dispatch;
	}
	// 82E9B9F4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9B9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9B9FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9B9FC size=48
    let mut pc: u32 = 0x82E9B9FC;
    'dispatch: loop {
        match pc {
            0x82E9B9FC => {
    //   block [0x82E9B9FC..0x82E9BA2C)
	// 82E9B9FC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BA00: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9BA04: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9BA08: 409A001C  bne cr6, 0x82e9ba24
	if !ctx.cr[6].eq {
	pc = 0x82E9BA24; continue 'dispatch;
	}
	// 82E9BA0C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9BA10: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9BA14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BA18: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9BA1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9BA20: 419AFFDC  beq cr6, 0x82e9b9fc
	if ctx.cr[6].eq {
	pc = 0x82E9B9FC; continue 'dispatch;
	}
	// 82E9BA24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9BA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9BA30 size=64
    let mut pc: u32 = 0x82E9BA30;
    'dispatch: loop {
        match pc {
            0x82E9BA30 => {
    //   block [0x82E9BA30..0x82E9BA70)
	// 82E9BA30: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9BA34: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BA38: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9BA3C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BA40: 892A0041  lbz r9, 0x41(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9BA44: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9BA48: 409A0008  bne cr6, 0x82e9ba50
	if !ctx.cr[6].eq {
	pc = 0x82E9BA50; continue 'dispatch;
	}
	// 82E9BA4C: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E9BA50: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BA54: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9BA58: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BA5C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BA60: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9BA64: 409A000C  bne cr6, 0x82e9ba70
	if !ctx.cr[6].eq {
		sub_82E9BA70(ctx, base);
		return;
	}
	// 82E9BA68: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9BA6C: 48000020  b 0x82e9ba8c
	sub_82E9BA88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9BA70 size=24
    let mut pc: u32 = 0x82E9BA70;
    'dispatch: loop {
        match pc {
            0x82E9BA70 => {
    //   block [0x82E9BA70..0x82E9BA88)
	// 82E9BA70: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BA74: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BA78: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9BA7C: 409A000C  bne cr6, 0x82e9ba88
	if !ctx.cr[6].eq {
		sub_82E9BA88(ctx, base);
		return;
	}
	// 82E9BA80: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9BA84: 48000008  b 0x82e9ba8c
	sub_82E9BA88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9BA88 size=16
    let mut pc: u32 = 0x82E9BA88;
    'dispatch: loop {
        match pc {
            0x82E9BA88 => {
    //   block [0x82E9BA88..0x82E9BA98)
	// 82E9BA88: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9BA8C: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E9BA90: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9BA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BA98 size=120
    let mut pc: u32 = 0x82E9BA98;
    'dispatch: loop {
        match pc {
            0x82E9BA98 => {
    //   block [0x82E9BA98..0x82E9BB10)
	// 82E9BA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BA9C: 4830C6C5  bl 0x831a8160
	ctx.lr = 0x82E9BAA0;
	sub_831A8130(ctx, base);
	// 82E9BAA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BAA4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9BAA8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E9BAAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9BAB0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E9BAB4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E9BAB8: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 82E9BABC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9BAC0: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82E9BAC4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E9BAC8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E9BACC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E9BAD0: 4BF565F9  bl 0x82df20c8
	ctx.lr = 0x82E9BAD4;
	sub_82DF20C8(ctx, base);
	// 82E9BAD4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E9BAD8: 4182002C  beq 0x82e9bb04
	if ctx.cr[0].eq {
	pc = 0x82E9BB04; continue 'dispatch;
	}
	// 82E9BADC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E9BAE0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E9BAE4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E9BAE8: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 82E9BAEC: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E9BAF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E9BAF4: 4830CA1D  bl 0x831a8510
	ctx.lr = 0x82E9BAF8;
	sub_831A8510(ctx, base);
	// 82E9BAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9BAFC: 9B5F0048  stb r26, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u8 ) };
	// 82E9BB00: 997F0049  stb r11, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[11].u8 ) };
	// 82E9BB04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9BB08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E9BB0C: 4830C6A4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BB10 size=108
    let mut pc: u32 = 0x82E9BB10;
    'dispatch: loop {
        match pc {
            0x82E9BB10 => {
    //   block [0x82E9BB10..0x82E9BB7C)
	// 82E9BB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9BB18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BB1C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9BB20: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E9BB24: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 82E9BB28: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82E9BB2C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E9BB30: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9BB34: 4BF56595  bl 0x82df20c8
	ctx.lr = 0x82E9BB38;
	sub_82DF20C8(ctx, base);
	// 82E9BB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9BB3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9BB40: 41820008  beq 0x82e9bb48
	if ctx.cr[0].eq {
	pc = 0x82E9BB48; continue 'dispatch;
	}
	// 82E9BB44: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9BB48: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9BB4C: 41820008  beq 0x82e9bb54
	if ctx.cr[0].eq {
	pc = 0x82E9BB54; continue 'dispatch;
	}
	// 82E9BB50: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9BB54: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9BB58: 41820008  beq 0x82e9bb60
	if ctx.cr[0].eq {
	pc = 0x82E9BB60; continue 'dispatch;
	}
	// 82E9BB5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9BB60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9BB64: 99430049  stb r10, 0x49(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(73 as u32), ctx.r[10].u8 ) };
	// 82E9BB68: 99630048  stb r11, 0x48(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 82E9BB6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9BB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9BB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9BB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BB80 size=108
    let mut pc: u32 = 0x82E9BB80;
    'dispatch: loop {
        match pc {
            0x82E9BB80 => {
    //   block [0x82E9BB80..0x82E9BBEC)
	// 82E9BB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9BB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BB8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9BB90: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E9BB94: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 82E9BB98: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82E9BB9C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E9BBA0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9BBA4: 4BF56525  bl 0x82df20c8
	ctx.lr = 0x82E9BBA8;
	sub_82DF20C8(ctx, base);
	// 82E9BBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9BBAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9BBB0: 41820008  beq 0x82e9bbb8
	if ctx.cr[0].eq {
	pc = 0x82E9BBB8; continue 'dispatch;
	}
	// 82E9BBB4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9BBB8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9BBBC: 41820008  beq 0x82e9bbc4
	if ctx.cr[0].eq {
	pc = 0x82E9BBC4; continue 'dispatch;
	}
	// 82E9BBC0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9BBC4: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9BBC8: 41820008  beq 0x82e9bbd0
	if ctx.cr[0].eq {
	pc = 0x82E9BBD0; continue 'dispatch;
	}
	// 82E9BBCC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9BBD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9BBD4: 99430041  stb r10, 0x41(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(65 as u32), ctx.r[10].u8 ) };
	// 82E9BBD8: 99630040  stb r11, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E9BBDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9BBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9BBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9BBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BBF0 size=140
    let mut pc: u32 = 0x82E9BBF0;
    'dispatch: loop {
        match pc {
            0x82E9BBF0 => {
    //   block [0x82E9BBF0..0x82E9BC7C)
	// 82E9BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BBF4: 4830C579  bl 0x831a816c
	ctx.lr = 0x82E9BBF8;
	sub_831A8130(ctx, base);
	// 82E9BBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BBFC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E9BC00: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E9BC04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9BC08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9BC0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9BC10: 4BFFFCE9  bl 0x82e9b8f8
	ctx.lr = 0x82E9BC14;
	sub_82E9B8F8(ctx, base);
	// 82E9BC14: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BC18: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82E9BC1C: 7F033840  cmplw cr6, r3, r7
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E9BC20: 419A0040  beq cr6, 0x82e9bc60
	if ctx.cr[6].eq {
	pc = 0x82E9BC60; continue 'dispatch;
	}
	// 82E9BC24: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82E9BC28: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9BC2C: 391F0034  addi r8, r31, 0x34
	ctx.r[8].s64 = ctx.r[31].s64 + 52;
	// 82E9BC30: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BC34: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BC38: 7D264851  subf. r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9BC3C: 40820014  bne 0x82e9bc50
	if !ctx.cr[0].eq {
	pc = 0x82E9BC50; continue 'dispatch;
	}
	// 82E9BC40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9BC44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9BC48: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9BC4C: 409AFFE4  bne cr6, 0x82e9bc30
	if !ctx.cr[6].eq {
	pc = 0x82E9BC30; continue 'dispatch;
	}
	// 82E9BC50: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9BC54: 4180000C  blt 0x82e9bc60
	if ctx.cr[0].lt {
	pc = 0x82E9BC60; continue 'dispatch;
	}
	// 82E9BC58: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E9BC5C: 4800000C  b 0x82e9bc68
	pc = 0x82E9BC68; continue 'dispatch;
	// 82E9BC60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82E9BC64: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82E9BC68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BC6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9BC70: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9BC74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9BC78: 4830C544  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BC80 size=140
    let mut pc: u32 = 0x82E9BC80;
    'dispatch: loop {
        match pc {
            0x82E9BC80 => {
    //   block [0x82E9BC80..0x82E9BD0C)
	// 82E9BC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BC84: 4830C4E9  bl 0x831a816c
	ctx.lr = 0x82E9BC88;
	sub_831A8130(ctx, base);
	// 82E9BC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BC8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E9BC90: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E9BC94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9BC98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9BC9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9BCA0: 4BFFFCB9  bl 0x82e9b958
	ctx.lr = 0x82E9BCA4;
	sub_82E9B958(ctx, base);
	// 82E9BCA4: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BCA8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82E9BCAC: 7F033840  cmplw cr6, r3, r7
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E9BCB0: 419A0040  beq cr6, 0x82e9bcf0
	if ctx.cr[6].eq {
	pc = 0x82E9BCF0; continue 'dispatch;
	}
	// 82E9BCB4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82E9BCB8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9BCBC: 391F0028  addi r8, r31, 0x28
	ctx.r[8].s64 = ctx.r[31].s64 + 40;
	// 82E9BCC0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BCC4: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BCC8: 7D264851  subf. r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9BCCC: 40820014  bne 0x82e9bce0
	if !ctx.cr[0].eq {
	pc = 0x82E9BCE0; continue 'dispatch;
	}
	// 82E9BCD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9BCD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9BCD8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9BCDC: 409AFFE4  bne cr6, 0x82e9bcc0
	if !ctx.cr[6].eq {
	pc = 0x82E9BCC0; continue 'dispatch;
	}
	// 82E9BCE0: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9BCE4: 4180000C  blt 0x82e9bcf0
	if ctx.cr[0].lt {
	pc = 0x82E9BCF0; continue 'dispatch;
	}
	// 82E9BCE8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E9BCEC: 4800000C  b 0x82e9bcf8
	pc = 0x82E9BCF8; continue 'dispatch;
	// 82E9BCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82E9BCF4: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82E9BCF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BCFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9BD00: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9BD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9BD08: 4830C4B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BD10 size=88
    let mut pc: u32 = 0x82E9BD10;
    'dispatch: loop {
        match pc {
            0x82E9BD10 => {
    //   block [0x82E9BD10..0x82E9BD68)
	// 82E9BD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9BD18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9BD1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BD20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9BD24: 4BFFFDED  bl 0x82e9bb10
	ctx.lr = 0x82E9BD28;
	sub_82E9BB10(ctx, base);
	// 82E9BD28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9BD2C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E9BD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9BD34: 99630049  stb r11, 0x49(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(73 as u32), ctx.r[11].u8 ) };
	// 82E9BD38: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BD3C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9BD40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BD44: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9BD48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BD4C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9BD50: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9BD54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9BD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9BD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9BD60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9BD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BD68 size=88
    let mut pc: u32 = 0x82E9BD68;
    'dispatch: loop {
        match pc {
            0x82E9BD68 => {
    //   block [0x82E9BD68..0x82E9BDC0)
	// 82E9BD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9BD70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9BD74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BD78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9BD7C: 4BFFFE05  bl 0x82e9bb80
	ctx.lr = 0x82E9BD80;
	sub_82E9BB80(ctx, base);
	// 82E9BD80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9BD84: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E9BD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9BD8C: 99630041  stb r11, 0x41(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(65 as u32), ctx.r[11].u8 ) };
	// 82E9BD90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BD94: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9BD98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BD9C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9BDA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BDA4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9BDA8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9BDAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9BDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9BDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9BDB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9BDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BDC0 size=128
    let mut pc: u32 = 0x82E9BDC0;
    'dispatch: loop {
        match pc {
            0x82E9BDC0 => {
    //   block [0x82E9BDC0..0x82E9BE40)
	// 82E9BDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BDC4: 4830C3A9  bl 0x831a816c
	ctx.lr = 0x82E9BDC8;
	sub_831A8130(ctx, base);
	// 82E9BDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BDCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9BDD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9BDD4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BDD8: 4BFF7941  bl 0x82e93718
	ctx.lr = 0x82E9BDDC;
	sub_82E93718(ctx, base);
	// 82E9BDDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9BDE0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E9BDE4: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82E9BDE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9BDEC: 4BFFFE95  bl 0x82e9bc80
	ctx.lr = 0x82E9BDF0;
	sub_82E9BC80(ctx, base);
	// 82E9BDF0: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9BDF4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9BDF8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9BDFC: 419A0030  beq cr6, 0x82e9be2c
	if ctx.cr[6].eq {
	pc = 0x82E9BE2C; continue 'dispatch;
	}
	// 82E9BE00: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E9BE04: 388B0034  addi r4, r11, 0x34
	ctx.r[4].s64 = ctx.r[11].s64 + 52;
	// 82E9BE08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9BE0C: 419A0020  beq cr6, 0x82e9be2c
	if ctx.cr[6].eq {
	pc = 0x82E9BE2C; continue 'dispatch;
	}
	// 82E9BE10: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BE14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9BE18: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BE1C: 83E50008  lwz r31, 8(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9BE20: 4BFC74B9  bl 0x82e632d8
	ctx.lr = 0x82E9BE24;
	sub_82E632D8(ctx, base);
	// 82E9BE24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9BE28: 48000010  b 0x82e9be38
	pc = 0x82E9BE38; continue 'dispatch;
	// 82E9BE2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9BE30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9BE34: 4BFFB0C5  bl 0x82e96ef8
	ctx.lr = 0x82E9BE38;
	sub_82E96EF8(ctx, base);
	// 82E9BE38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9BE3C: 4830C380  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BE40 size=84
    let mut pc: u32 = 0x82E9BE40;
    'dispatch: loop {
        match pc {
            0x82E9BE40 => {
    //   block [0x82E9BE40..0x82E9BE94)
	// 82E9BE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BE44: 4830C329  bl 0x831a816c
	ctx.lr = 0x82E9BE48;
	sub_831A8130(ctx, base);
	// 82E9BE48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BE4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9BE50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9BE54: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9BE58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9BE5C: 409A0030  bne cr6, 0x82e9be8c
	if !ctx.cr[6].eq {
	pc = 0x82E9BE8C; continue 'dispatch;
	}
	// 82E9BE60: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 82E9BE64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9BE68: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9BE6C: 4BFFFFD5  bl 0x82e9be40
	ctx.lr = 0x82E9BE70;
	sub_82E9BE40(ctx, base);
	// 82E9BE70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9BE74: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9BE78: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BE7C: 4BF5630D  bl 0x82df2188
	ctx.lr = 0x82E9BE80;
	sub_82DF2188(ctx, base);
	// 82E9BE80: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9BE84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9BE88: 419AFFDC  beq cr6, 0x82e9be64
	if ctx.cr[6].eq {
	pc = 0x82E9BE64; continue 'dispatch;
	}
	// 82E9BE8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9BE90: 4830C32C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BE98 size=84
    let mut pc: u32 = 0x82E9BE98;
    'dispatch: loop {
        match pc {
            0x82E9BE98 => {
    //   block [0x82E9BE98..0x82E9BEEC)
	// 82E9BE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9BEA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9BEA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BEA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9BEAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BEB0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BEB4: 4BFFFF8D  bl 0x82e9be40
	ctx.lr = 0x82E9BEB8;
	sub_82E9BE40(ctx, base);
	// 82E9BEB8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BEBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9BEC0: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9BEC4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9BEC8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BECC: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9BED0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9BED4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9BED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9BEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9BEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9BEE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9BEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BEF0 size=104
    let mut pc: u32 = 0x82E9BEF0;
    'dispatch: loop {
        match pc {
            0x82E9BEF0 => {
    //   block [0x82E9BEF0..0x82E9BF58)
	// 82E9BEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9BEF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9BEFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9BF00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BF04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9BF08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9BF0C: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 82E9BF10: 4BFC4CA9  bl 0x82e60bb8
	ctx.lr = 0x82E9BF14;
	sub_82E60BB8(ctx, base);
	// 82E9BF14: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9BF18: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E9BF1C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9BF20: 4BF56269  bl 0x82df2188
	ctx.lr = 0x82E9BF24;
	sub_82DF2188(ctx, base);
	// 82E9BF24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9BF28: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9BF2C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E9BF30: 4182000C  beq 0x82e9bf3c
	if ctx.cr[0].eq {
	pc = 0x82E9BF3C; continue 'dispatch;
	}
	// 82E9BF34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9BF38: 4B424331  bl 0x822c0268
	ctx.lr = 0x82E9BF3C;
	sub_822C0268(ctx, base);
	// 82E9BF3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9BF40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9BF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9BF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9BF4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9BF50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9BF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9BF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9BF58 size=1016
    let mut pc: u32 = 0x82E9BF58;
    'dispatch: loop {
        match pc {
            0x82E9BF58 => {
    //   block [0x82E9BF58..0x82E9C350)
	// 82E9BF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9BF5C: 4830C1FD  bl 0x831a8158
	ctx.lr = 0x82E9BF60;
	sub_831A8130(ctx, base);
	// 82E9BF60: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9BF64: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E9BF68: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E9BF6C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E9BF70: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82E9BF74: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9BF78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9BF7C: 419A0048  beq cr6, 0x82e9bfc4
	if ctx.cr[6].eq {
	pc = 0x82E9BFC4; continue 'dispatch;
	}
	// 82E9BF80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9BF84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9BF88: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82E9BF8C: 4B42993D  bl 0x822c58c8
	ctx.lr = 0x82E9BF90;
	sub_822C58C8(ctx, base);
	// 82E9BF90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9BF94: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9BF98: 4B42DF19  bl 0x822c9eb0
	ctx.lr = 0x82E9BF9C;
	sub_822C9EB0(ctx, base);
	// 82E9BF9C: 4B428315  bl 0x822c42b0
	ctx.lr = 0x82E9BFA0;
	sub_822C42B0(ctx, base);
	// 82E9BFA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9BFA4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9BFA8: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82E9BFAC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E9BFB0: 4B4294C1  bl 0x822c5470
	ctx.lr = 0x82E9BFB4;
	sub_822C5470(ctx, base);
	// 82E9BFB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9BFB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9BFBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9BFC0: 4B428D21  bl 0x822c4ce0
	ctx.lr = 0x82E9BFC4;
	sub_822C4CE0(ctx, base);
	// 82E9BFC4: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82E9BFC8: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82E9BFCC: 4BFFF72D  bl 0x82e9b6f8
	ctx.lr = 0x82E9BFD0;
	sub_82E9B6F8(ctx, base);
	// 82E9BFD0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9BFD4: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9BFD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9BFDC: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82E9BFE0: 419A000C  beq cr6, 0x82e9bfec
	if ctx.cr[6].eq {
	pc = 0x82E9BFEC; continue 'dispatch;
	}
	// 82E9BFE4: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9BFE8: 48000028  b 0x82e9c010
	pc = 0x82E9C010; continue 'dispatch;
	// 82E9BFEC: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9BFF0: 894A0049  lbz r10, 0x49(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9BFF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9BFF8: 419A000C  beq cr6, 0x82e9c004
	if ctx.cr[6].eq {
	pc = 0x82E9C004; continue 'dispatch;
	}
	// 82E9BFFC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82E9C000: 48000010  b 0x82e9c010
	pc = 0x82E9C010; continue 'dispatch;
	// 82E9C004: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C008: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9C00C: 409A00DC  bne cr6, 0x82e9c0e8
	if !ctx.cr[6].eq {
	pc = 0x82E9C0E8; continue 'dispatch;
	}
	// 82E9C010: 897C0049  lbz r11, 0x49(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C014: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C01C: 409A0008  bne cr6, 0x82e9c024
	if !ctx.cr[6].eq {
	pc = 0x82E9C024; continue 'dispatch;
	}
	// 82E9C020: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E9C024: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C028: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C02C: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9C030: 409A000C  bne cr6, 0x82e9c03c
	if !ctx.cr[6].eq {
	pc = 0x82E9C03C; continue 'dispatch;
	}
	// 82E9C034: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E9C038: 4800001C  b 0x82e9c054
	pc = 0x82E9C054; continue 'dispatch;
	// 82E9C03C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C040: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9C044: 409A000C  bne cr6, 0x82e9c050
	if !ctx.cr[6].eq {
	pc = 0x82E9C050; continue 'dispatch;
	}
	// 82E9C048: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C04C: 48000008  b 0x82e9c054
	pc = 0x82E9C054; continue 'dispatch;
	// 82E9C050: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E9C054: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C058: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C05C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9C060: 409A003C  bne cr6, 0x82e9c09c
	if !ctx.cr[6].eq {
	pc = 0x82E9C09C; continue 'dispatch;
	}
	// 82E9C064: 897C0049  lbz r11, 0x49(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C06C: 419A000C  beq cr6, 0x82e9c078
	if ctx.cr[6].eq {
	pc = 0x82E9C078; continue 'dispatch;
	}
	// 82E9C070: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9C074: 48000024  b 0x82e9c098
	pc = 0x82E9C098; continue 'dispatch;
	// 82E9C078: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C07C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E9C080: 4800000C  b 0x82e9c08c
	pc = 0x82E9C08C; continue 'dispatch;
	// 82E9C084: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9C088: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C08C: 890B0049  lbz r8, 0x49(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C090: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E9C094: 419AFFF0  beq cr6, 0x82e9c084
	if ctx.cr[6].eq {
	pc = 0x82E9C084; continue 'dispatch;
	}
	// 82E9C098: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9C09C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C0A0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C0A4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9C0A8: 409A00D4  bne cr6, 0x82e9c17c
	if !ctx.cr[6].eq {
	pc = 0x82E9C17C; continue 'dispatch;
	}
	// 82E9C0AC: 897C0049  lbz r11, 0x49(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C0B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C0B4: 419A000C  beq cr6, 0x82e9c0c0
	if ctx.cr[6].eq {
	pc = 0x82E9C0C0; continue 'dispatch;
	}
	// 82E9C0B8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9C0BC: 48000024  b 0x82e9c0e0
	pc = 0x82E9C0E0; continue 'dispatch;
	// 82E9C0C0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C0C4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E9C0C8: 4800000C  b 0x82e9c0d4
	pc = 0x82E9C0D4; continue 'dispatch;
	// 82E9C0CC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9C0D0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C0D4: 890B0049  lbz r8, 0x49(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C0D8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E9C0DC: 419AFFF0  beq cr6, 0x82e9c0cc
	if ctx.cr[6].eq {
	pc = 0x82E9C0CC; continue 'dispatch;
	}
	// 82E9C0E0: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9C0E4: 48000098  b 0x82e9c17c
	pc = 0x82E9C17C; continue 'dispatch;
	// 82E9C0E8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E9C0EC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C0F0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9C0F4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C0F8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C0FC: 409A000C  bne cr6, 0x82e9c108
	if !ctx.cr[6].eq {
	pc = 0x82E9C108; continue 'dispatch;
	}
	// 82E9C100: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E9C104: 4800002C  b 0x82e9c130
	pc = 0x82E9C130; continue 'dispatch;
	// 82E9C108: 897C0049  lbz r11, 0x49(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C10C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C114: 409A0008  bne cr6, 0x82e9c11c
	if !ctx.cr[6].eq {
	pc = 0x82E9C11C; continue 'dispatch;
	}
	// 82E9C118: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E9C11C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C120: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C124: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9C128: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C12C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E9C130: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C134: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C138: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9C13C: 409A000C  bne cr6, 0x82e9c148
	if !ctx.cr[6].eq {
	pc = 0x82E9C148; continue 'dispatch;
	}
	// 82E9C140: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E9C144: 48000020  b 0x82e9c164
	pc = 0x82E9C164; continue 'dispatch;
	// 82E9C148: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C14C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C150: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9C154: 409A000C  bne cr6, 0x82e9c160
	if !ctx.cr[6].eq {
	pc = 0x82E9C160; continue 'dispatch;
	}
	// 82E9C158: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E9C15C: 48000008  b 0x82e9c164
	pc = 0x82E9C164; continue 'dispatch;
	// 82E9C160: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82E9C164: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C168: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9C16C: 897A0048  lbz r11, 0x48(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C170: 89590048  lbz r10, 0x48(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C174: 99790048  stb r11, 0x48(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 82E9C178: 995A0048  stb r10, 0x48(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(72 as u32), ctx.r[10].u8 ) };
	// 82E9C17C: 897A0048  lbz r11, 0x48(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C180: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E9C184: 409A0198  bne cr6, 0x82e9c31c
	if !ctx.cr[6].eq {
	pc = 0x82E9C31C; continue 'dispatch;
	}
	// 82E9C188: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C18C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E9C190: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C194: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C198: 419A0180  beq cr6, 0x82e9c318
	if ctx.cr[6].eq {
	pc = 0x82E9C318; continue 'dispatch;
	}
	// 82E9C19C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E9C1A0: 897C0048  lbz r11, 0x48(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C1A4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E9C1A8: 409A0170  bne cr6, 0x82e9c318
	if !ctx.cr[6].eq {
	pc = 0x82E9C318; continue 'dispatch;
	}
	// 82E9C1AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C1B0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C1B4: 409A00A8  bne cr6, 0x82e9c25c
	if !ctx.cr[6].eq {
	pc = 0x82E9C25C; continue 'dispatch;
	}
	// 82E9C1B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C1BC: 894B0048  lbz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C1C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C1C4: 409A001C  bne cr6, 0x82e9c1e0
	if !ctx.cr[6].eq {
	pc = 0x82E9C1E0; continue 'dispatch;
	}
	// 82E9C1C8: 9BCB0048  stb r30, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C1CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9C1D0: 9BBF0048  stb r29, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C1D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9C1D8: 4BFFF6B9  bl 0x82e9b890
	ctx.lr = 0x82E9C1DC;
	sub_82E9B890(ctx, base);
	// 82E9C1DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C1E0: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C1E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C1E8: 409A00C8  bne cr6, 0x82e9c2b0
	if !ctx.cr[6].eq {
	pc = 0x82E9C2B0; continue 'dispatch;
	}
	// 82E9C1EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C1F0: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C1F4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9C1F8: 409A0014  bne cr6, 0x82e9c20c
	if !ctx.cr[6].eq {
	pc = 0x82E9C20C; continue 'dispatch;
	}
	// 82E9C1FC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C200: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C204: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9C208: 419A00A4  beq cr6, 0x82e9c2ac
	if ctx.cr[6].eq {
	pc = 0x82E9C2AC; continue 'dispatch;
	}
	// 82E9C20C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C210: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C214: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9C218: 409A0020  bne cr6, 0x82e9c238
	if !ctx.cr[6].eq {
	pc = 0x82E9C238; continue 'dispatch;
	}
	// 82E9C21C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C220: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E9C224: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9C228: 9BCA0048  stb r30, 0x48(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C22C: 9BAB0048  stb r29, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C230: 4BFFF3F9  bl 0x82e9b628
	ctx.lr = 0x82E9C234;
	sub_82E9B628(ctx, base);
	// 82E9C234: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C238: 895F0048  lbz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C23C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9C240: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9C244: 994B0048  stb r10, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u8 ) };
	// 82E9C248: 9BDF0048  stb r30, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C24C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C250: 9BCB0048  stb r30, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C254: 4BFFF63D  bl 0x82e9b890
	ctx.lr = 0x82E9C258;
	sub_82E9B890(ctx, base);
	// 82E9C258: 480000C0  b 0x82e9c318
	pc = 0x82E9C318; continue 'dispatch;
	// 82E9C25C: 894B0048  lbz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C260: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C264: 409A001C  bne cr6, 0x82e9c280
	if !ctx.cr[6].eq {
	pc = 0x82E9C280; continue 'dispatch;
	}
	// 82E9C268: 9BCB0048  stb r30, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C26C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9C270: 9BBF0048  stb r29, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C274: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9C278: 4BFFF3B1  bl 0x82e9b628
	ctx.lr = 0x82E9C27C;
	sub_82E9B628(ctx, base);
	// 82E9C27C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C280: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C284: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C288: 409A0028  bne cr6, 0x82e9c2b0
	if !ctx.cr[6].eq {
	pc = 0x82E9C2B0; continue 'dispatch;
	}
	// 82E9C28C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C290: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C294: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9C298: 409A0034  bne cr6, 0x82e9c2cc
	if !ctx.cr[6].eq {
	pc = 0x82E9C2CC; continue 'dispatch;
	}
	// 82E9C29C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C2A0: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C2A4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9C2A8: 409A0024  bne cr6, 0x82e9c2cc
	if !ctx.cr[6].eq {
	pc = 0x82E9C2CC; continue 'dispatch;
	}
	// 82E9C2AC: 9BAB0048  stb r29, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C2B0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C2B4: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82E9C2B8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C2BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C2C0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C2C4: 409AFEDC  bne cr6, 0x82e9c1a0
	if !ctx.cr[6].eq {
	pc = 0x82E9C1A0; continue 'dispatch;
	}
	// 82E9C2C8: 48000050  b 0x82e9c318
	pc = 0x82E9C318; continue 'dispatch;
	// 82E9C2CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C2D0: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C2D4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9C2D8: 409A0020  bne cr6, 0x82e9c2f8
	if !ctx.cr[6].eq {
	pc = 0x82E9C2F8; continue 'dispatch;
	}
	// 82E9C2DC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C2E0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E9C2E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9C2E8: 9BCA0048  stb r30, 0x48(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C2EC: 9BAB0048  stb r29, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C2F0: 4BFFF5A1  bl 0x82e9b890
	ctx.lr = 0x82E9C2F4;
	sub_82E9B890(ctx, base);
	// 82E9C2F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C2F8: 895F0048  lbz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C2FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9C300: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9C304: 994B0048  stb r10, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u8 ) };
	// 82E9C308: 9BDF0048  stb r30, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C30C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C310: 9BCB0048  stb r30, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C314: 4BFFF315  bl 0x82e9b628
	ctx.lr = 0x82E9C318;
	sub_82E9B628(ctx, base);
	// 82E9C318: 9BDC0048  stb r30, 0x48(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82E9C31C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9C320: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9C324: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9C328: 4BF55E61  bl 0x82df2188
	ctx.lr = 0x82E9C32C;
	sub_82DF2188(ctx, base);
	// 82E9C32C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C330: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C334: 419A000C  beq cr6, 0x82e9c340
	if ctx.cr[6].eq {
	pc = 0x82E9C340; continue 'dispatch;
	}
	// 82E9C338: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E9C33C: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9C340: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E9C344: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E9C348: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E9C34C: 4830BE5C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9C350 size=132
    let mut pc: u32 = 0x82E9C350;
    'dispatch: loop {
        match pc {
            0x82E9C350 => {
    //   block [0x82E9C350..0x82E9C3D4)
	// 82E9C350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9C354: 4830BE15  bl 0x831a8168
	ctx.lr = 0x82E9C358;
	sub_831A8130(ctx, base);
	// 82E9C358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9C35C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9C360: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82E9C364: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9C368: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E9C36C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C370: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C374: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9C378: 409A0044  bne cr6, 0x82e9c3bc
	if !ctx.cr[6].eq {
	pc = 0x82E9C3BC; continue 'dispatch;
	}
	// 82E9C37C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C380: 409A003C  bne cr6, 0x82e9c3bc
	if !ctx.cr[6].eq {
	pc = 0x82E9C3BC; continue 'dispatch;
	}
	// 82E9C384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9C388: 4BFFFB11  bl 0x82e9be98
	ctx.lr = 0x82E9C38C;
	sub_82E9BE98(ctx, base);
	// 82E9C38C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C390: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C394: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9C398: 48000030  b 0x82e9c3c8
	pc = 0x82E9C3C8; continue 'dispatch;
	// 82E9C39C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82E9C3A0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E9C3A4: 4BFFF355  bl 0x82e9b6f8
	ctx.lr = 0x82E9C3A8;
	sub_82E9B6F8(ctx, base);
	// 82E9C3A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E9C3AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9C3B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C3B4: 4BFFFBA5  bl 0x82e9bf58
	ctx.lr = 0x82E9C3B8;
	sub_82E9BF58(ctx, base);
	// 82E9C3B8: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E9C3BC: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E9C3C0: 409AFFDC  bne cr6, 0x82e9c39c
	if !ctx.cr[6].eq {
	pc = 0x82E9C39C; continue 'dispatch;
	}
	// 82E9C3C4: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82E9C3C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9C3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9C3D0: 4830BDE8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9C3D8 size=548
    let mut pc: u32 = 0x82E9C3D8;
    'dispatch: loop {
        match pc {
            0x82E9C3D8 => {
    //   block [0x82E9C3D8..0x82E9C5FC)
	// 82E9C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9C3DC: 4830BD85  bl 0x831a8160
	ctx.lr = 0x82E9C3E0;
	sub_831A8130(ctx, base);
	// 82E9C3E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9C3E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9C3E8: 3D600444  lis r11, 0x444
	ctx.r[11].s64 = 71565312;
	// 82E9C3EC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E9C3F0: 616B4443  ori r11, r11, 0x4443
	ctx.r[11].u64 = ctx.r[11].u64 | 17475;
	// 82E9C3F4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E9C3F8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C3FC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E9C400: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E9C404: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C408: 41980048  blt cr6, 0x82e9c450
	if ctx.cr[6].lt {
	pc = 0x82E9C450; continue 'dispatch;
	}
	// 82E9C40C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9C410: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C414: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82E9C418: 4B4294B1  bl 0x822c58c8
	ctx.lr = 0x82E9C41C;
	sub_822C58C8(ctx, base);
	// 82E9C41C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9C420: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9C424: 4B4293F5  bl 0x822c5818
	ctx.lr = 0x82E9C428;
	sub_822C5818(ctx, base);
	// 82E9C428: 4B427E89  bl 0x822c42b0
	ctx.lr = 0x82E9C42C;
	sub_822C42B0(ctx, base);
	// 82E9C42C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9C430: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9C434: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82E9C438: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E9C43C: 4B429035  bl 0x822c5470
	ctx.lr = 0x82E9C440;
	sub_822C5470(ctx, base);
	// 82E9C440: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9C444: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9C448: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C44C: 4B428895  bl 0x822c4ce0
	ctx.lr = 0x82E9C450;
	sub_822C4CE0(ctx, base);
	// 82E9C450: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E9C458: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E9C45C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82E9C460: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9C464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9C468: 4BFFF631  bl 0x82e9ba98
	ctx.lr = 0x82E9C46C;
	sub_82E9BA98(ctx, base);
	// 82E9C46C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C470: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C474: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9C478: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9C47C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C480: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9C484: 409A0018  bne cr6, 0x82e9c49c
	if !ctx.cr[6].eq {
	pc = 0x82E9C49C; continue 'dispatch;
	}
	// 82E9C488: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E9C48C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C490: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C494: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C498: 4800003C  b 0x82e9c4d4
	pc = 0x82E9C4D4; continue 'dispatch;
	// 82E9C49C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9C4A0: 41820020  beq 0x82e9c4c0
	if ctx.cr[0].eq {
	pc = 0x82E9C4C0; continue 'dispatch;
	}
	// 82E9C4A4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C4A8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C4AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C4B0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9C4B4: 409A0024  bne cr6, 0x82e9c4d8
	if !ctx.cr[6].eq {
	pc = 0x82E9C4D8; continue 'dispatch;
	}
	// 82E9C4B8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C4BC: 4800001C  b 0x82e9c4d8
	pc = 0x82E9C4D8; continue 'dispatch;
	// 82E9C4C0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E9C4C4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C4C8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C4CC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9C4D0: 409A0008  bne cr6, 0x82e9c4d8
	if !ctx.cr[6].eq {
	pc = 0x82E9C4D8; continue 'dispatch;
	}
	// 82E9C4D4: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E9C4D8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C4DC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82E9C4E0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82E9C4E4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82E9C4E8: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C4EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C4F0: 409A00F0  bne cr6, 0x82e9c5e0
	if !ctx.cr[6].eq {
	pc = 0x82E9C5E0; continue 'dispatch;
	}
	// 82E9C4F4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E9C4F8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C4FC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C500: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C504: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9C508: 409A0054  bne cr6, 0x82e9c55c
	if !ctx.cr[6].eq {
	pc = 0x82E9C55C; continue 'dispatch;
	}
	// 82E9C50C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C510: 892A0048  lbz r9, 0x48(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C514: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9C518: 419A0054  beq cr6, 0x82e9c56c
	if ctx.cr[6].eq {
	pc = 0x82E9C56C; continue 'dispatch;
	}
	// 82E9C51C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C520: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C524: 409A0010  bne cr6, 0x82e9c534
	if !ctx.cr[6].eq {
	pc = 0x82E9C534; continue 'dispatch;
	}
	// 82E9C528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9C52C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9C530: 4BFFF361  bl 0x82e9b890
	ctx.lr = 0x82E9C534;
	sub_82E9B890(ctx, base);
	// 82E9C534: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9C53C: 9BAB0048  stb r29, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C540: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C544: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C548: 9B6B0048  stb r27, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[27].u8 ) };
	// 82E9C54C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C550: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C554: 4BFFF0D5  bl 0x82e9b628
	ctx.lr = 0x82E9C558;
	sub_82E9B628(ctx, base);
	// 82E9C558: 48000074  b 0x82e9c5cc
	pc = 0x82E9C5CC; continue 'dispatch;
	// 82E9C55C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C560: 892A0048  lbz r9, 0x48(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C564: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9C568: 409A0028  bne cr6, 0x82e9c590
	if !ctx.cr[6].eq {
	pc = 0x82E9C590; continue 'dispatch;
	}
	// 82E9C56C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C570: 9BA90048  stb r29, 0x48(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C574: 9BAA0048  stb r29, 0x48(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C578: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C57C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C580: 9B6A0048  stb r27, 0x48(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(72 as u32), ctx.r[27].u8 ) };
	// 82E9C584: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C588: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C58C: 48000040  b 0x82e9c5cc
	pc = 0x82E9C5CC; continue 'dispatch;
	// 82E9C590: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C594: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C598: 409A0010  bne cr6, 0x82e9c5a8
	if !ctx.cr[6].eq {
	pc = 0x82E9C5A8; continue 'dispatch;
	}
	// 82E9C59C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9C5A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9C5A4: 4BFFF085  bl 0x82e9b628
	ctx.lr = 0x82E9C5A8;
	sub_82E9B628(ctx, base);
	// 82E9C5A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9C5B0: 9BAB0048  stb r29, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C5B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5BC: 9B6B0048  stb r27, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[27].u8 ) };
	// 82E9C5C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5C4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5C8: 4BFFF2C9  bl 0x82e9b890
	ctx.lr = 0x82E9C5CC;
	sub_82E9B890(ctx, base);
	// 82E9C5CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5D0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82E9C5D4: 894A0048  lbz r10, 0x48(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E9C5D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C5DC: 419AFF1C  beq cr6, 0x82e9c4f8
	if ctx.cr[6].eq {
	pc = 0x82E9C4F8; continue 'dispatch;
	}
	// 82E9C5E0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9C5E8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C5EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C5F0: 9BAB0048  stb r29, 0x48(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[29].u8 ) };
	// 82E9C5F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E9C5F8: 4830BBB8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9C600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9C600 size=140
    let mut pc: u32 = 0x82E9C600;
    'dispatch: loop {
        match pc {
            0x82E9C600 => {
    //   block [0x82E9C600..0x82E9C68C)
	// 82E9C600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9C604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9C608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9C60C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9C610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9C614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9C618: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E9C61C: 4BFFE875  bl 0x82e9ae90
	ctx.lr = 0x82E9C620;
	sub_82E9AE90(ctx, base);
	// 82E9C620: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E9C624: 89650014  lbz r11, 0x14(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9C628: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9C62C: 40820048  bne 0x82e9c674
	if !ctx.cr[0].eq {
	pc = 0x82E9C674; continue 'dispatch;
	}
	// 82E9C630: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82E9C634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C638: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9C63C: 4BFFF5B5  bl 0x82e9bbf0
	ctx.lr = 0x82E9C640;
	sub_82E9BBF0(ctx, base);
	// 82E9C640: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9C644: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9C648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9C64C: 419A0008  beq cr6, 0x82e9c654
	if ctx.cr[6].eq {
	pc = 0x82E9C654; continue 'dispatch;
	}
	// 82E9C650: 4BD42989  bl 0x82bdefd8
	ctx.lr = 0x82E9C654;
	sub_82BDEFD8(ctx, base);
	// 82E9C654: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E9C658: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9C65C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E9C660: 40820014  bne 0x82e9c674
	if !ctx.cr[0].eq {
	pc = 0x82E9C674; continue 'dispatch;
	}
	// 82E9C664: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9C668: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9C66C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C670: 4BFFF8E9  bl 0x82e9bf58
	ctx.lr = 0x82E9C674;
	sub_82E9BF58(ctx, base);
	// 82E9C674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9C678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9C67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9C680: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9C684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9C688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9C690 size=88
    let mut pc: u32 = 0x82E9C690;
    'dispatch: loop {
        match pc {
            0x82E9C690 => {
    //   block [0x82E9C690..0x82E9C6E8)
	// 82E9C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9C698: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9C69C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9C6A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9C6A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C6A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9C6AC: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C6B0: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C6B4: 4BFFFC9D  bl 0x82e9c350
	ctx.lr = 0x82E9C6B8;
	sub_82E9C350(ctx, base);
	// 82E9C6B8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9C6BC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C6C0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9C6C4: 4BF55AC5  bl 0x82df2188
	ctx.lr = 0x82E9C6C8;
	sub_82DF2188(ctx, base);
	// 82E9C6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9C6CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9C6D0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9C6D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9C6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9C6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9C6E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9C6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9C6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9C6E8 size=304
    let mut pc: u32 = 0x82E9C6E8;
    'dispatch: loop {
        match pc {
            0x82E9C6E8 => {
    //   block [0x82E9C6E8..0x82E9C818)
	// 82E9C6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9C6EC: 4830BA75  bl 0x831a8160
	ctx.lr = 0x82E9C6F0;
	sub_831A8130(ctx, base);
	// 82E9C6F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9C6F4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E9C6F8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82E9C6FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9C700: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E9C704: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82E9C708: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C70C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C710: 48000050  b 0x82e9c760
	pc = 0x82E9C760; continue 'dispatch;
	// 82E9C714: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E9C718: 392B000C  addi r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 + 12;
	// 82E9C71C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E9C720: 391D0034  addi r8, r29, 0x34
	ctx.r[8].s64 = ctx.r[29].s64 + 52;
	// 82E9C724: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C728: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C72C: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E9C730: 40820014  bne 0x82e9c744
	if !ctx.cr[0].eq {
	pc = 0x82E9C744; continue 'dispatch;
	}
	// 82E9C734: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9C738: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E9C73C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9C740: 409AFFE4  bne cr6, 0x82e9c724
	if !ctx.cr[6].eq {
	pc = 0x82E9C724; continue 'dispatch;
	}
	// 82E9C744: 7CEA0034  cntlzw r10, r7
	ctx.r[10].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 82E9C748: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E9C74C: 555CDFFF  rlwinm. r28, r10, 0x1b, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E9C750: 4182000C  beq 0x82e9c75c
	if ctx.cr[0].eq {
	pc = 0x82E9C75C; continue 'dispatch;
	}
	// 82E9C754: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C758: 48000008  b 0x82e9c760
	pc = 0x82E9C760; continue 'dispatch;
	// 82E9C75C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C760: 894B0049  lbz r10, 0x49(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9C764: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C768: 419AFFAC  beq cr6, 0x82e9c714
	if ctx.cr[6].eq {
	pc = 0x82E9C714; continue 'dispatch;
	}
	// 82E9C76C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9C770: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9C774: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82E9C778: 41820048  beq 0x82e9c7c0
	if ctx.cr[0].eq {
	pc = 0x82E9C7C0; continue 'dispatch;
	}
	// 82E9C77C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C784: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C788: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C78C: 409A002C  bne cr6, 0x82e9c7b8
	if !ctx.cr[6].eq {
	pc = 0x82E9C7B8; continue 'dispatch;
	}
	// 82E9C790: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E9C794: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E9C798: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E9C79C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E9C7A0: 4BFFFC39  bl 0x82e9c3d8
	ctx.lr = 0x82E9C7A4;
	sub_82E9C3D8(ctx, base);
	// 82E9C7A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E9C7A8: 9B5E0004  stb r26, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82E9C7AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C7B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9C7B4: 48000058  b 0x82e9c80c
	pc = 0x82E9C80C; continue 'dispatch;
	// 82E9C7B8: 4BFFF049  bl 0x82e9b800
	ctx.lr = 0x82E9C7BC;
	sub_82E9B800(ctx, base);
	// 82E9C7BC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9C7C0: 3967000C  addi r11, r7, 0xc
	ctx.r[11].s64 = ctx.r[7].s64 + 12;
	// 82E9C7C4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E9C7C8: 390B0034  addi r8, r11, 0x34
	ctx.r[8].s64 = ctx.r[11].s64 + 52;
	// 82E9C7CC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C7D0: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C7D4: 7D264851  subf. r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9C7D8: 40820014  bne 0x82e9c7ec
	if !ctx.cr[0].eq {
	pc = 0x82E9C7EC; continue 'dispatch;
	}
	// 82E9C7DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9C7E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9C7E4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9C7E8: 409AFFE4  bne cr6, 0x82e9c7cc
	if !ctx.cr[6].eq {
	pc = 0x82E9C7CC; continue 'dispatch;
	}
	// 82E9C7EC: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9C7F0: 40800010  bge 0x82e9c800
	if !ctx.cr[0].lt {
	pc = 0x82E9C800; continue 'dispatch;
	}
	// 82E9C7F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E9C7F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C7FC: 4BFFFF98  b 0x82e9c794
	pc = 0x82E9C794; continue 'dispatch;
	// 82E9C800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9C804: 90FE0000  stw r7, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E9C808: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82E9C80C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9C810: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E9C814: 4830B99C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9C818 size=1028
    let mut pc: u32 = 0x82E9C818;
    'dispatch: loop {
        match pc {
            0x82E9C818 => {
    //   block [0x82E9C818..0x82E9CC1C)
	// 82E9C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9C81C: 4830B93D  bl 0x831a8158
	ctx.lr = 0x82E9C820;
	sub_831A8130(ctx, base);
	// 82E9C820: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9C824: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E9C828: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E9C82C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E9C830: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82E9C834: 897F0041  lbz r11, 0x41(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C838: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C83C: 419A0048  beq cr6, 0x82e9c884
	if ctx.cr[6].eq {
	pc = 0x82E9C884; continue 'dispatch;
	}
	// 82E9C840: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9C844: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C848: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82E9C84C: 4B42907D  bl 0x822c58c8
	ctx.lr = 0x82E9C850;
	sub_822C58C8(ctx, base);
	// 82E9C850: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9C854: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9C858: 4B42D659  bl 0x822c9eb0
	ctx.lr = 0x82E9C85C;
	sub_822C9EB0(ctx, base);
	// 82E9C85C: 4B427A55  bl 0x822c42b0
	ctx.lr = 0x82E9C860;
	sub_822C42B0(ctx, base);
	// 82E9C860: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9C864: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9C868: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82E9C86C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E9C870: 4B428C01  bl 0x822c5470
	ctx.lr = 0x82E9C874;
	sub_822C5470(ctx, base);
	// 82E9C874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9C878: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9C87C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9C880: 4B428461  bl 0x822c4ce0
	ctx.lr = 0x82E9C884;
	sub_822C4CE0(ctx, base);
	// 82E9C884: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82E9C888: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82E9C88C: 4BFFF12D  bl 0x82e9b9b8
	ctx.lr = 0x82E9C890;
	sub_82E9B9B8(ctx, base);
	// 82E9C890: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C894: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C898: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C89C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82E9C8A0: 419A000C  beq cr6, 0x82e9c8ac
	if ctx.cr[6].eq {
	pc = 0x82E9C8AC; continue 'dispatch;
	}
	// 82E9C8A4: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C8A8: 48000028  b 0x82e9c8d0
	pc = 0x82E9C8D0; continue 'dispatch;
	// 82E9C8AC: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C8B0: 894A0041  lbz r10, 0x41(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C8B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9C8B8: 419A000C  beq cr6, 0x82e9c8c4
	if ctx.cr[6].eq {
	pc = 0x82E9C8C4; continue 'dispatch;
	}
	// 82E9C8BC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82E9C8C0: 48000010  b 0x82e9c8d0
	pc = 0x82E9C8D0; continue 'dispatch;
	// 82E9C8C4: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C8C8: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E9C8CC: 409A00DC  bne cr6, 0x82e9c9a8
	if !ctx.cr[6].eq {
	pc = 0x82E9C9A8; continue 'dispatch;
	}
	// 82E9C8D0: 897C0041  lbz r11, 0x41(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C8D4: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C8D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C8DC: 409A0008  bne cr6, 0x82e9c8e4
	if !ctx.cr[6].eq {
	pc = 0x82E9C8E4; continue 'dispatch;
	}
	// 82E9C8E0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E9C8E4: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C8E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C8EC: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E9C8F0: 409A000C  bne cr6, 0x82e9c8fc
	if !ctx.cr[6].eq {
	pc = 0x82E9C8FC; continue 'dispatch;
	}
	// 82E9C8F4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E9C8F8: 4800001C  b 0x82e9c914
	pc = 0x82E9C914; continue 'dispatch;
	// 82E9C8FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C900: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E9C904: 409A000C  bne cr6, 0x82e9c910
	if !ctx.cr[6].eq {
	pc = 0x82E9C910; continue 'dispatch;
	}
	// 82E9C908: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C90C: 48000008  b 0x82e9c914
	pc = 0x82E9C914; continue 'dispatch;
	// 82E9C910: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E9C914: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C918: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C91C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E9C920: 409A003C  bne cr6, 0x82e9c95c
	if !ctx.cr[6].eq {
	pc = 0x82E9C95C; continue 'dispatch;
	}
	// 82E9C924: 897C0041  lbz r11, 0x41(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C92C: 419A000C  beq cr6, 0x82e9c938
	if ctx.cr[6].eq {
	pc = 0x82E9C938; continue 'dispatch;
	}
	// 82E9C930: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9C934: 48000024  b 0x82e9c958
	pc = 0x82E9C958; continue 'dispatch;
	// 82E9C938: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C93C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E9C940: 4800000C  b 0x82e9c94c
	pc = 0x82E9C94C; continue 'dispatch;
	// 82E9C944: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9C948: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C94C: 890B0041  lbz r8, 0x41(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C950: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E9C954: 419AFFF0  beq cr6, 0x82e9c944
	if ctx.cr[6].eq {
	pc = 0x82E9C944; continue 'dispatch;
	}
	// 82E9C958: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9C95C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C960: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C964: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E9C968: 409A00D4  bne cr6, 0x82e9ca3c
	if !ctx.cr[6].eq {
	pc = 0x82E9CA3C; continue 'dispatch;
	}
	// 82E9C96C: 897C0041  lbz r11, 0x41(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C970: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C974: 419A000C  beq cr6, 0x82e9c980
	if ctx.cr[6].eq {
	pc = 0x82E9C980; continue 'dispatch;
	}
	// 82E9C978: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9C97C: 48000024  b 0x82e9c9a0
	pc = 0x82E9C9A0; continue 'dispatch;
	// 82E9C980: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C984: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E9C988: 4800000C  b 0x82e9c994
	pc = 0x82E9C994; continue 'dispatch;
	// 82E9C98C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E9C990: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C994: 890B0041  lbz r8, 0x41(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C998: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E9C99C: 419AFFF0  beq cr6, 0x82e9c98c
	if ctx.cr[6].eq {
	pc = 0x82E9C98C; continue 'dispatch;
	}
	// 82E9C9A0: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9C9A4: 48000098  b 0x82e9ca3c
	pc = 0x82E9CA3C; continue 'dispatch;
	// 82E9C9A8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E9C9AC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9C9B0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9C9B4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C9B8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9C9BC: 409A000C  bne cr6, 0x82e9c9c8
	if !ctx.cr[6].eq {
	pc = 0x82E9C9C8; continue 'dispatch;
	}
	// 82E9C9C0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E9C9C4: 4800002C  b 0x82e9c9f0
	pc = 0x82E9C9F0; continue 'dispatch;
	// 82E9C9C8: 897C0041  lbz r11, 0x41(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9C9CC: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C9D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9C9D4: 409A0008  bne cr6, 0x82e9c9dc
	if !ctx.cr[6].eq {
	pc = 0x82E9C9DC; continue 'dispatch;
	}
	// 82E9C9D8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E9C9DC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9C9E0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C9E4: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9C9E8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9C9EC: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E9C9F0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C9F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9C9F8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E9C9FC: 409A000C  bne cr6, 0x82e9ca08
	if !ctx.cr[6].eq {
	pc = 0x82E9CA08; continue 'dispatch;
	}
	// 82E9CA00: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E9CA04: 48000020  b 0x82e9ca24
	pc = 0x82E9CA24; continue 'dispatch;
	// 82E9CA08: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CA0C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CA10: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E9CA14: 409A000C  bne cr6, 0x82e9ca20
	if !ctx.cr[6].eq {
	pc = 0x82E9CA20; continue 'dispatch;
	}
	// 82E9CA18: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E9CA1C: 48000008  b 0x82e9ca24
	pc = 0x82E9CA24; continue 'dispatch;
	// 82E9CA20: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82E9CA24: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CA28: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9CA2C: 897B0040  lbz r11, 0x40(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CA30: 89590040  lbz r10, 0x40(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CA34: 99790040  stb r11, 0x40(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E9CA38: 995B0040  stb r10, 0x40(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 82E9CA3C: 897B0040  lbz r11, 0x40(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CA40: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E9CA44: 409A0198  bne cr6, 0x82e9cbdc
	if !ctx.cr[6].eq {
	pc = 0x82E9CBDC; continue 'dispatch;
	}
	// 82E9CA48: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CA4C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E9CA50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CA54: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9CA58: 419A0180  beq cr6, 0x82e9cbd8
	if ctx.cr[6].eq {
	pc = 0x82E9CBD8; continue 'dispatch;
	}
	// 82E9CA5C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E9CA60: 897C0040  lbz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CA64: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E9CA68: 409A0170  bne cr6, 0x82e9cbd8
	if !ctx.cr[6].eq {
	pc = 0x82E9CBD8; continue 'dispatch;
	}
	// 82E9CA6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CA70: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9CA74: 409A00A8  bne cr6, 0x82e9cb1c
	if !ctx.cr[6].eq {
	pc = 0x82E9CB1C; continue 'dispatch;
	}
	// 82E9CA78: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CA7C: 894B0040  lbz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CA80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9CA84: 409A001C  bne cr6, 0x82e9caa0
	if !ctx.cr[6].eq {
	pc = 0x82E9CAA0; continue 'dispatch;
	}
	// 82E9CA88: 9BCB0040  stb r30, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CA8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9CA90: 9BBF0040  stb r29, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9CA94: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9CA98: 4BFFEF99  bl 0x82e9ba30
	ctx.lr = 0x82E9CA9C;
	sub_82E9BA30(ctx, base);
	// 82E9CA9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CAA0: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9CAA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9CAA8: 409A00C8  bne cr6, 0x82e9cb70
	if !ctx.cr[6].eq {
	pc = 0x82E9CB70; continue 'dispatch;
	}
	// 82E9CAAC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CAB0: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CAB4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9CAB8: 409A0014  bne cr6, 0x82e9cacc
	if !ctx.cr[6].eq {
	pc = 0x82E9CACC; continue 'dispatch;
	}
	// 82E9CABC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CAC0: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CAC4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9CAC8: 419A00A4  beq cr6, 0x82e9cb6c
	if ctx.cr[6].eq {
	pc = 0x82E9CB6C; continue 'dispatch;
	}
	// 82E9CACC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CAD0: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CAD4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9CAD8: 409A0020  bne cr6, 0x82e9caf8
	if !ctx.cr[6].eq {
	pc = 0x82E9CAF8; continue 'dispatch;
	}
	// 82E9CADC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CAE0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E9CAE4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9CAE8: 9BCA0040  stb r30, 0x40(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CAEC: 9BAB0040  stb r29, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9CAF0: 4BFFEBA1  bl 0x82e9b690
	ctx.lr = 0x82E9CAF4;
	sub_82E9B690(ctx, base);
	// 82E9CAF4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CAF8: 895F0040  lbz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CAFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9CB00: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9CB04: 994B0040  stb r10, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 82E9CB08: 9BDF0040  stb r30, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CB0C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CB10: 9BCB0040  stb r30, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CB14: 4BFFEF1D  bl 0x82e9ba30
	ctx.lr = 0x82E9CB18;
	sub_82E9BA30(ctx, base);
	// 82E9CB18: 480000C0  b 0x82e9cbd8
	pc = 0x82E9CBD8; continue 'dispatch;
	// 82E9CB1C: 894B0040  lbz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CB20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9CB24: 409A001C  bne cr6, 0x82e9cb40
	if !ctx.cr[6].eq {
	pc = 0x82E9CB40; continue 'dispatch;
	}
	// 82E9CB28: 9BCB0040  stb r30, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CB2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9CB30: 9BBF0040  stb r29, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9CB34: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9CB38: 4BFFEB59  bl 0x82e9b690
	ctx.lr = 0x82E9CB3C;
	sub_82E9B690(ctx, base);
	// 82E9CB3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CB40: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9CB44: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9CB48: 409A0028  bne cr6, 0x82e9cb70
	if !ctx.cr[6].eq {
	pc = 0x82E9CB70; continue 'dispatch;
	}
	// 82E9CB4C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CB50: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CB54: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9CB58: 409A0034  bne cr6, 0x82e9cb8c
	if !ctx.cr[6].eq {
	pc = 0x82E9CB8C; continue 'dispatch;
	}
	// 82E9CB5C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CB60: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CB64: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9CB68: 409A0024  bne cr6, 0x82e9cb8c
	if !ctx.cr[6].eq {
	pc = 0x82E9CB8C; continue 'dispatch;
	}
	// 82E9CB6C: 9BAB0040  stb r29, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9CB70: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CB74: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82E9CB78: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CB7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CB80: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9CB84: 409AFEDC  bne cr6, 0x82e9ca60
	if !ctx.cr[6].eq {
	pc = 0x82E9CA60; continue 'dispatch;
	}
	// 82E9CB88: 48000050  b 0x82e9cbd8
	pc = 0x82E9CBD8; continue 'dispatch;
	// 82E9CB8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CB90: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CB94: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9CB98: 409A0020  bne cr6, 0x82e9cbb8
	if !ctx.cr[6].eq {
	pc = 0x82E9CBB8; continue 'dispatch;
	}
	// 82E9CB9C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CBA0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E9CBA4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9CBA8: 9BCA0040  stb r30, 0x40(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CBAC: 9BAB0040  stb r29, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9CBB0: 4BFFEE81  bl 0x82e9ba30
	ctx.lr = 0x82E9CBB4;
	sub_82E9BA30(ctx, base);
	// 82E9CBB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CBB8: 895F0040  lbz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9CBBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9CBC0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9CBC4: 994B0040  stb r10, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 82E9CBC8: 9BDF0040  stb r30, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CBCC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CBD0: 9BCB0040  stb r30, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CBD4: 4BFFEABD  bl 0x82e9b690
	ctx.lr = 0x82E9CBD8;
	sub_82E9B690(ctx, base);
	// 82E9CBD8: 9BDC0040  stb r30, 0x40(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82E9CBDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E9CBE0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9CBE4: 4BFFF30D  bl 0x82e9bef0
	ctx.lr = 0x82E9CBE8;
	sub_82E9BEF0(ctx, base);
	// 82E9CBE8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9CBEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E9CBF0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9CBF4: 4BF55595  bl 0x82df2188
	ctx.lr = 0x82E9CBF8;
	sub_82DF2188(ctx, base);
	// 82E9CBF8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CBFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9CC00: 419A000C  beq cr6, 0x82e9cc0c
	if ctx.cr[6].eq {
	pc = 0x82E9CC0C; continue 'dispatch;
	}
	// 82E9CC04: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E9CC08: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9CC0C: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E9CC10: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E9CC14: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E9CC18: 4830B590  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9CC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9CC20 size=104
    let mut pc: u32 = 0x82E9CC20;
    'dispatch: loop {
        match pc {
            0x82E9CC20 => {
    //   block [0x82E9CC20..0x82E9CC88)
	// 82E9CC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9CC24: 4830B545  bl 0x831a8168
	ctx.lr = 0x82E9CC28;
	sub_831A8130(ctx, base);
	// 82E9CC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9CC2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9CC30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9CC34: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82E9CC38: 897E0041  lbz r11, 0x41(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9CC3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9CC40: 409A0040  bne cr6, 0x82e9cc80
	if !ctx.cr[6].eq {
	pc = 0x82E9CC80; continue 'dispatch;
	}
	// 82E9CC44: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82E9CC48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9CC4C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CC50: 4BFFFFD1  bl 0x82e9cc20
	ctx.lr = 0x82E9CC54;
	sub_82E9CC20(ctx, base);
	// 82E9CC54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E9CC58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9CC5C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CC60: 4BFFF291  bl 0x82e9bef0
	ctx.lr = 0x82E9CC64;
	sub_82E9BEF0(ctx, base);
	// 82E9CC64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9CC68: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9CC6C: 4BF5551D  bl 0x82df2188
	ctx.lr = 0x82E9CC70;
	sub_82DF2188(ctx, base);
	// 82E9CC70: 897F0041  lbz r11, 0x41(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9CC74: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82E9CC78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9CC7C: 419AFFCC  beq cr6, 0x82e9cc48
	if ctx.cr[6].eq {
	pc = 0x82E9CC48; continue 'dispatch;
	}
	// 82E9CC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9CC84: 4830B534  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9CC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9CC88 size=592
    let mut pc: u32 = 0x82E9CC88;
    'dispatch: loop {
        match pc {
            0x82E9CC88 => {
    //   block [0x82E9CC88..0x82E9CED8)
	// 82E9CC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9CC8C: 4830B4D5  bl 0x831a8160
	ctx.lr = 0x82E9CC90;
	sub_831A8130(ctx, base);
	// 82E9CC90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9CC94: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E9CC98: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E9CC9C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E9CCA0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E9CCA4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CCA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9CCAC: 409A0020  bne cr6, 0x82e9cccc
	if !ctx.cr[6].eq {
	pc = 0x82E9CCCC; continue 'dispatch;
	}
	// 82E9CCB0: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CCB4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E9CCB8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9CCBC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9CCC0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9CCC4: 4BFFF715  bl 0x82e9c3d8
	ctx.lr = 0x82E9CCC8;
	sub_82E9C3D8(ctx, base);
	// 82E9CCC8: 48000204  b 0x82e9cecc
	pc = 0x82E9CECC; continue 'dispatch;
	// 82E9CCCC: 839A0004  lwz r28, 4(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CCD0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CCD4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9CCD8: 409A0040  bne cr6, 0x82e9cd18
	if !ctx.cr[6].eq {
	pc = 0x82E9CD18; continue 'dispatch;
	}
	// 82E9CCDC: 395D000C  addi r10, r29, 0xc
	ctx.r[10].s64 = ctx.r[29].s64 + 12;
	// 82E9CCE0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9CCE4: 391F0034  addi r8, r31, 0x34
	ctx.r[8].s64 = ctx.r[31].s64 + 52;
	// 82E9CCE8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CCEC: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CCF0: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9CCF4: 40820014  bne 0x82e9cd08
	if !ctx.cr[0].eq {
	pc = 0x82E9CD08; continue 'dispatch;
	}
	// 82E9CCF8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9CCFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9CD00: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9CD04: 409AFFE4  bne cr6, 0x82e9cce8
	if !ctx.cr[6].eq {
	pc = 0x82E9CCE8; continue 'dispatch;
	}
	// 82E9CD08: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9CD0C: 408001A4  bge 0x82e9ceb0
	if !ctx.cr[0].lt {
	pc = 0x82E9CEB0; continue 'dispatch;
	}
	// 82E9CD10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E9CD14: 4BFFFFA0  b 0x82e9ccb4
	pc = 0x82E9CCB4; continue 'dispatch;
	// 82E9CD18: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E9CD1C: 409A0044  bne cr6, 0x82e9cd60
	if !ctx.cr[6].eq {
	pc = 0x82E9CD60; continue 'dispatch;
	}
	// 82E9CD20: 80DC0008  lwz r6, 8(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CD24: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9CD28: 3966000C  addi r11, r6, 0xc
	ctx.r[11].s64 = ctx.r[6].s64 + 12;
	// 82E9CD2C: 392B0034  addi r9, r11, 0x34
	ctx.r[9].s64 = ctx.r[11].s64 + 52;
	// 82E9CD30: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CD34: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CD38: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9CD3C: 40820014  bne 0x82e9cd50
	if !ctx.cr[0].eq {
	pc = 0x82E9CD50; continue 'dispatch;
	}
	// 82E9CD40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9CD44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9CD48: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E9CD4C: 409AFFE4  bne cr6, 0x82e9cd30
	if !ctx.cr[6].eq {
	pc = 0x82E9CD30; continue 'dispatch;
	}
	// 82E9CD50: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9CD54: 4080015C  bge 0x82e9ceb0
	if !ctx.cr[0].lt {
	pc = 0x82E9CEB0; continue 'dispatch;
	}
	// 82E9CD58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9CD5C: 4BFFFF5C  b 0x82e9ccb8
	pc = 0x82E9CCB8; continue 'dispatch;
	// 82E9CD60: 3BDD000C  addi r30, r29, 0xc
	ctx.r[30].s64 = ctx.r[29].s64 + 12;
	// 82E9CD64: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9CD68: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82E9CD6C: 393F0034  addi r9, r31, 0x34
	ctx.r[9].s64 = ctx.r[31].s64 + 52;
	// 82E9CD70: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CD74: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CD78: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9CD7C: 40820014  bne 0x82e9cd90
	if !ctx.cr[0].eq {
	pc = 0x82E9CD90; continue 'dispatch;
	}
	// 82E9CD80: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9CD84: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9CD88: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E9CD8C: 409AFFE4  bne cr6, 0x82e9cd70
	if !ctx.cr[6].eq {
	pc = 0x82E9CD70; continue 'dispatch;
	}
	// 82E9CD90: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9CD94: 40800078  bge 0x82e9ce0c
	if !ctx.cr[0].lt {
	pc = 0x82E9CE0C; continue 'dispatch;
	}
	// 82E9CD98: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82E9CD9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9CDA0: 4BFFEA61  bl 0x82e9b800
	ctx.lr = 0x82E9CDA4;
	sub_82E9B800(ctx, base);
	// 82E9CDA4: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9CDA8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9CDAC: 3966000C  addi r11, r6, 0xc
	ctx.r[11].s64 = ctx.r[6].s64 + 12;
	// 82E9CDB0: 392B0034  addi r9, r11, 0x34
	ctx.r[9].s64 = ctx.r[11].s64 + 52;
	// 82E9CDB4: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CDB8: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CDBC: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9CDC0: 40820014  bne 0x82e9cdd4
	if !ctx.cr[0].eq {
	pc = 0x82E9CDD4; continue 'dispatch;
	}
	// 82E9CDC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9CDC8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9CDCC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E9CDD0: 409AFFE4  bne cr6, 0x82e9cdb4
	if !ctx.cr[6].eq {
	pc = 0x82E9CDB4; continue 'dispatch;
	}
	// 82E9CDD4: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9CDD8: 40800034  bge 0x82e9ce0c
	if !ctx.cr[0].lt {
	pc = 0x82E9CE0C; continue 'dispatch;
	}
	// 82E9CDDC: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CDE0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9CDE4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9CDE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9CDEC: 896B0049  lbz r11, 0x49(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9CDF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9CDF4: 419A000C  beq cr6, 0x82e9ce00
	if ctx.cr[6].eq {
	pc = 0x82E9CE00; continue 'dispatch;
	}
	// 82E9CDF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9CDFC: 4BFFFEC8  b 0x82e9ccc4
	pc = 0x82E9CCC4; continue 'dispatch;
	// 82E9CE00: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E9CE04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E9CE08: 4BFFFEBC  b 0x82e9ccc4
	pc = 0x82E9CCC4; continue 'dispatch;
	// 82E9CE0C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9CE10: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E9CE14: 391E0034  addi r8, r30, 0x34
	ctx.r[8].s64 = ctx.r[30].s64 + 52;
	// 82E9CE18: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CE1C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CE20: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9CE24: 40820014  bne 0x82e9ce38
	if !ctx.cr[0].eq {
	pc = 0x82E9CE38; continue 'dispatch;
	}
	// 82E9CE28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9CE2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9CE30: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9CE34: 409AFFE4  bne cr6, 0x82e9ce18
	if !ctx.cr[6].eq {
	pc = 0x82E9CE18; continue 'dispatch;
	}
	// 82E9CE38: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9CE3C: 40800074  bge 0x82e9ceb0
	if !ctx.cr[0].lt {
	pc = 0x82E9CEB0; continue 'dispatch;
	}
	// 82E9CE40: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82E9CE44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9CE48: 4BFFE8B1  bl 0x82e9b6f8
	ctx.lr = 0x82E9CE4C;
	sub_82E9B6F8(ctx, base);
	// 82E9CE4C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9CE50: 7F06E040  cmplw cr6, r6, r28
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E9CE54: 419A0038  beq cr6, 0x82e9ce8c
	if ctx.cr[6].eq {
	pc = 0x82E9CE8C; continue 'dispatch;
	}
	// 82E9CE58: 3946000C  addi r10, r6, 0xc
	ctx.r[10].s64 = ctx.r[6].s64 + 12;
	// 82E9CE5C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9CE60: 391F0034  addi r8, r31, 0x34
	ctx.r[8].s64 = ctx.r[31].s64 + 52;
	// 82E9CE64: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CE68: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CE6C: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9CE70: 40820014  bne 0x82e9ce84
	if !ctx.cr[0].eq {
	pc = 0x82E9CE84; continue 'dispatch;
	}
	// 82E9CE74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9CE78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9CE7C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9CE80: 409AFFE4  bne cr6, 0x82e9ce64
	if !ctx.cr[6].eq {
	pc = 0x82E9CE64; continue 'dispatch;
	}
	// 82E9CE84: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9CE88: 40800028  bge 0x82e9ceb0
	if !ctx.cr[0].lt {
	pc = 0x82E9CEB0; continue 'dispatch;
	}
	// 82E9CE8C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9CE90: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9CE94: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9CE98: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9CE9C: 896B0049  lbz r11, 0x49(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E9CEA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9CEA4: 419AFF60  beq cr6, 0x82e9ce04
	if ctx.cr[6].eq {
	pc = 0x82E9CE04; continue 'dispatch;
	}
	// 82E9CEA8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E9CEAC: 4BFFFF4C  b 0x82e9cdf8
	pc = 0x82E9CDF8; continue 'dispatch;
	// 82E9CEB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9CEB4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9CEB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9CEBC: 4BFFF82D  bl 0x82e9c6e8
	ctx.lr = 0x82E9CEC0;
	sub_82E9C6E8(ctx, base);
	// 82E9CEC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E9CEC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CEC8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9CECC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9CED0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E9CED4: 4830B2DC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9CED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9CED8 size=84
    let mut pc: u32 = 0x82E9CED8;
    'dispatch: loop {
        match pc {
            0x82E9CED8 => {
    //   block [0x82E9CED8..0x82E9CF2C)
	// 82E9CED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9CEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9CEE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9CEE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9CEE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9CEEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CEF0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CEF4: 4BFFFD2D  bl 0x82e9cc20
	ctx.lr = 0x82E9CEF8;
	sub_82E9CC20(ctx, base);
	// 82E9CEF8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CEFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9CF00: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9CF04: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9CF08: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CF0C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9CF10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CF14: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9CF18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9CF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9CF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9CF24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9CF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9CF30 size=164
    let mut pc: u32 = 0x82E9CF30;
    'dispatch: loop {
        match pc {
            0x82E9CF30 => {
    //   block [0x82E9CF30..0x82E9CFD4)
	// 82E9CF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9CF34: 4830B239  bl 0x831a816c
	ctx.lr = 0x82E9CF38;
	sub_831A8130(ctx, base);
	// 82E9CF38: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9CF3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9CF40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9CF44: 4BFFE9B5  bl 0x82e9b8f8
	ctx.lr = 0x82E9CF48;
	sub_82E9B8F8(ctx, base);
	// 82E9CF48: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CF4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9CF50: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9CF54: 419A0038  beq cr6, 0x82e9cf8c
	if ctx.cr[6].eq {
	pc = 0x82E9CF8C; continue 'dispatch;
	}
	// 82E9CF58: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82E9CF5C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E9CF60: 391E0034  addi r8, r30, 0x34
	ctx.r[8].s64 = ctx.r[30].s64 + 52;
	// 82E9CF64: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CF68: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CF6C: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9CF70: 40820014  bne 0x82e9cf84
	if !ctx.cr[0].eq {
	pc = 0x82E9CF84; continue 'dispatch;
	}
	// 82E9CF74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9CF78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9CF7C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9CF80: 409AFFE4  bne cr6, 0x82e9cf64
	if !ctx.cr[6].eq {
	pc = 0x82E9CF64; continue 'dispatch;
	}
	// 82E9CF84: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9CF88: 40800040  bge 0x82e9cfc8
	if !ctx.cr[0].lt {
	pc = 0x82E9CFC8; continue 'dispatch;
	}
	// 82E9CF8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9CF90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E9CF94: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E9CF98: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E9CF9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9CFA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E9CFA4: 4830B56D  bl 0x831a8510
	ctx.lr = 0x82E9CFA8;
	sub_831A8510(ctx, base);
	// 82E9CFA8: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9CFAC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E9CFB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9CFB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E9CFB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9CFBC: F9610094  std r11, 0x94(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u64 ) };
	// 82E9CFC0: 4BFFFCC9  bl 0x82e9cc88
	ctx.lr = 0x82E9CFC4;
	sub_82E9CC88(ctx, base);
	// 82E9CFC4: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CFC8: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82E9CFCC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E9CFD0: 4830B1EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9CFD8 size=132
    let mut pc: u32 = 0x82E9CFD8;
    'dispatch: loop {
        match pc {
            0x82E9CFD8 => {
    //   block [0x82E9CFD8..0x82E9D05C)
	// 82E9CFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9CFDC: 4830B18D  bl 0x831a8168
	ctx.lr = 0x82E9CFE0;
	sub_831A8130(ctx, base);
	// 82E9CFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9CFE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9CFE8: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82E9CFEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9CFF0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E9CFF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9CFF8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9CFFC: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9D000: 409A0044  bne cr6, 0x82e9d044
	if !ctx.cr[6].eq {
	pc = 0x82E9D044; continue 'dispatch;
	}
	// 82E9D004: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D008: 409A003C  bne cr6, 0x82e9d044
	if !ctx.cr[6].eq {
	pc = 0x82E9D044; continue 'dispatch;
	}
	// 82E9D00C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9D010: 4BFFFEC9  bl 0x82e9ced8
	ctx.lr = 0x82E9D014;
	sub_82E9CED8(ctx, base);
	// 82E9D014: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D018: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D01C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9D020: 48000030  b 0x82e9d050
	pc = 0x82E9D050; continue 'dispatch;
	// 82E9D024: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82E9D028: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E9D02C: 4BFFE98D  bl 0x82e9b9b8
	ctx.lr = 0x82E9D030;
	sub_82E9B9B8(ctx, base);
	// 82E9D030: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E9D034: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9D038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D03C: 4BFFF7DD  bl 0x82e9c818
	ctx.lr = 0x82E9D040;
	sub_82E9C818(ctx, base);
	// 82E9D040: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E9D044: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E9D048: 409AFFDC  bne cr6, 0x82e9d024
	if !ctx.cr[6].eq {
	pc = 0x82E9D024; continue 'dispatch;
	}
	// 82E9D04C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82E9D050: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9D054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9D058: 4830B160  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D060 size=152
    let mut pc: u32 = 0x82E9D060;
    'dispatch: loop {
        match pc {
            0x82E9D060 => {
    //   block [0x82E9D060..0x82E9D0F8)
	// 82E9D060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D064: 4830B105  bl 0x831a8168
	ctx.lr = 0x82E9D068;
	sub_831A8130(ctx, base);
	// 82E9D068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9D070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9D074: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D078: 4BFF66A1  bl 0x82e93718
	ctx.lr = 0x82E9D07C;
	sub_82E93718(ctx, base);
	// 82E9D07C: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 82E9D080: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9D084: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E9D088: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E9D08C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D090: 4BFFEB61  bl 0x82e9bbf0
	ctx.lr = 0x82E9D094;
	sub_82E9BBF0(ctx, base);
	// 82E9D094: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9D098: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9D09C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D0A0: 419A0028  beq cr6, 0x82e9d0c8
	if ctx.cr[6].eq {
	pc = 0x82E9D0C8; continue 'dispatch;
	}
	// 82E9D0A4: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9D0A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9D0AC: 419A0008  beq cr6, 0x82e9d0b4
	if ctx.cr[6].eq {
	pc = 0x82E9D0B4; continue 'dispatch;
	}
	// 82E9D0B0: 4BD41EB1  bl 0x82bdef60
	ctx.lr = 0x82E9D0B4;
	sub_82BDEF60(ctx, base);
	// 82E9D0B4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E9D0B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D0BC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E9D0C0: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9D0C4: 4800002C  b 0x82e9d0f0
	pc = 0x82E9D0F0; continue 'dispatch;
	// 82E9D0C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9D0CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9D0D0: 4BFFFE61  bl 0x82e9cf30
	ctx.lr = 0x82E9D0D4;
	sub_82E9CF30(ctx, base);
	// 82E9D0D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9D0D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9D0DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E9D0E0: 4BFF9DA1  bl 0x82e96e80
	ctx.lr = 0x82E9D0E4;
	sub_82E96E80(ctx, base);
	// 82E9D0E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9D0E8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E9D0EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9D0F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9D0F4: 4830B0C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D0F8 size=88
    let mut pc: u32 = 0x82E9D0F8;
    'dispatch: loop {
        match pc {
            0x82E9D0F8 => {
    //   block [0x82E9D0F8..0x82E9D150)
	// 82E9D0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9D100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9D104: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9D10C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D110: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9D114: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D118: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D11C: 4BFFFEBD  bl 0x82e9cfd8
	ctx.lr = 0x82E9D120;
	sub_82E9CFD8(ctx, base);
	// 82E9D120: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9D124: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D128: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9D12C: 4BF5505D  bl 0x82df2188
	ctx.lr = 0x82E9D130;
	sub_82DF2188(ctx, base);
	// 82E9D130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9D134: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9D138: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9D13C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9D140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9D144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9D148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9D14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D150 size=136
    let mut pc: u32 = 0x82E9D150;
    'dispatch: loop {
        match pc {
            0x82E9D150 => {
    //   block [0x82E9D150..0x82E9D1D8)
	// 82E9D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D154: 4830B00D  bl 0x831a8160
	ctx.lr = 0x82E9D158;
	sub_831A8130(ctx, base);
	// 82E9D158: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D15C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9D160: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E9D164: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9D168: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E9D16C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E9D170: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 82E9D174: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9D178: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82E9D17C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E9D180: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E9D184: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E9D188: 4BF54F41  bl 0x82df20c8
	ctx.lr = 0x82E9D18C;
	sub_82DF20C8(ctx, base);
	// 82E9D18C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E9D190: 4182003C  beq 0x82e9d1cc
	if ctx.cr[0].eq {
	pc = 0x82E9D1CC; continue 'dispatch;
	}
	// 82E9D194: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E9D198: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82E9D19C: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E9D1A0: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E9D1A4: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82E9D1A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E9D1AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D1B0: 4830B361  bl 0x831a8510
	ctx.lr = 0x82E9D1B4;
	sub_831A8510(ctx, base);
	// 82E9D1B4: 389D0028  addi r4, r29, 0x28
	ctx.r[4].s64 = ctx.r[29].s64 + 40;
	// 82E9D1B8: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 82E9D1BC: 4BFC683D  bl 0x82e639f8
	ctx.lr = 0x82E9D1C0;
	sub_82E639F8(ctx, base);
	// 82E9D1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9D1C4: 9B5F0040  stb r26, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[26].u8 ) };
	// 82E9D1C8: 997F0041  stb r11, 0x41(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(65 as u32), ctx.r[11].u8 ) };
	// 82E9D1CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9D1D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E9D1D4: 4830AFDC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D1D8 size=128
    let mut pc: u32 = 0x82E9D1D8;
    'dispatch: loop {
        match pc {
            0x82E9D1D8 => {
    //   block [0x82E9D1D8..0x82E9D258)
	// 82E9D1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D1DC: 4830AF8D  bl 0x831a8168
	ctx.lr = 0x82E9D1E0;
	sub_831A8130(ctx, base);
	// 82E9D1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D1E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9D1E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9D1EC: 396BE5DC  addi r11, r11, -0x1a24
	ctx.r[11].s64 = ctx.r[11].s64 + -6692;
	// 82E9D1F0: 83BC0018  lwz r29, 0x18(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9D1F4: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9D1F8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D1FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E9D200: 48000038  b 0x82e9d238
	pc = 0x82E9D238; continue 'dispatch;
	// 82E9D204: 83CB0038  lwz r30, 0x38(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E9D208: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D20C: 48000018  b 0x82e9d224
	pc = 0x82E9D224; continue 'dispatch;
	// 82E9D210: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D214: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9D218: 419A0008  beq cr6, 0x82e9d220
	if ctx.cr[6].eq {
	pc = 0x82E9D220; continue 'dispatch;
	}
	// 82E9D21C: 4BD41DBD  bl 0x82bdefd8
	ctx.lr = 0x82E9D220;
	sub_82BDEFD8(ctx, base);
	// 82E9D220: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D224: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E9D228: 409AFFE8  bne cr6, 0x82e9d210
	if !ctx.cr[6].eq {
	pc = 0x82E9D210; continue 'dispatch;
	}
	// 82E9D22C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D230: 4BFFE789  bl 0x82e9b9b8
	ctx.lr = 0x82E9D234;
	sub_82E9B9B8(ctx, base);
	// 82E9D234: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9D238: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E9D23C: 409AFFC8  bne cr6, 0x82e9d204
	if !ctx.cr[6].eq {
	pc = 0x82E9D204; continue 'dispatch;
	}
	// 82E9D240: 387C0014  addi r3, r28, 0x14
	ctx.r[3].s64 = ctx.r[28].s64 + 20;
	// 82E9D244: 4BFFFEB5  bl 0x82e9d0f8
	ctx.lr = 0x82E9D248;
	sub_82E9D0F8(ctx, base);
	// 82E9D248: 387C0008  addi r3, r28, 8
	ctx.r[3].s64 = ctx.r[28].s64 + 8;
	// 82E9D24C: 4BFFF445  bl 0x82e9c690
	ctx.lr = 0x82E9D250;
	sub_82E9C690(ctx, base);
	// 82E9D250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9D254: 4830AF64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D258 size=548
    let mut pc: u32 = 0x82E9D258;
    'dispatch: loop {
        match pc {
            0x82E9D258 => {
    //   block [0x82E9D258..0x82E9D47C)
	// 82E9D258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D25C: 4830AF05  bl 0x831a8160
	ctx.lr = 0x82E9D260;
	sub_831A8130(ctx, base);
	// 82E9D260: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D264: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9D268: 3D6004EC  lis r11, 0x4ec
	ctx.r[11].s64 = 82575360;
	// 82E9D26C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E9D270: 616B4EC3  ori r11, r11, 0x4ec3
	ctx.r[11].u64 = ctx.r[11].u64 | 20163;
	// 82E9D274: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E9D278: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D27C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E9D280: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E9D284: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D288: 41980048  blt cr6, 0x82e9d2d0
	if ctx.cr[6].lt {
	pc = 0x82E9D2D0; continue 'dispatch;
	}
	// 82E9D28C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9D290: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D294: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82E9D298: 4B428631  bl 0x822c58c8
	ctx.lr = 0x82E9D29C;
	sub_822C58C8(ctx, base);
	// 82E9D29C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9D2A0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9D2A4: 4B428575  bl 0x822c5818
	ctx.lr = 0x82E9D2A8;
	sub_822C5818(ctx, base);
	// 82E9D2A8: 4B427009  bl 0x822c42b0
	ctx.lr = 0x82E9D2AC;
	sub_822C42B0(ctx, base);
	// 82E9D2AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9D2B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9D2B4: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82E9D2B8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E9D2BC: 4B4281B5  bl 0x822c5470
	ctx.lr = 0x82E9D2C0;
	sub_822C5470(ctx, base);
	// 82E9D2C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9D2C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9D2C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D2CC: 4B427A15  bl 0x822c4ce0
	ctx.lr = 0x82E9D2D0;
	sub_822C4CE0(ctx, base);
	// 82E9D2D0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E9D2D8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E9D2DC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82E9D2E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9D2E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D2E8: 4BFFFE69  bl 0x82e9d150
	ctx.lr = 0x82E9D2EC;
	sub_82E9D150(ctx, base);
	// 82E9D2EC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D2F0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D2F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9D2F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D2FC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D300: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9D304: 409A0018  bne cr6, 0x82e9d31c
	if !ctx.cr[6].eq {
	pc = 0x82E9D31C; continue 'dispatch;
	}
	// 82E9D308: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E9D30C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D310: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9D314: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D318: 4800003C  b 0x82e9d354
	pc = 0x82E9D354; continue 'dispatch;
	// 82E9D31C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9D320: 41820020  beq 0x82e9d340
	if ctx.cr[0].eq {
	pc = 0x82E9D340; continue 'dispatch;
	}
	// 82E9D324: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9D328: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D32C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D330: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9D334: 409A0024  bne cr6, 0x82e9d358
	if !ctx.cr[6].eq {
	pc = 0x82E9D358; continue 'dispatch;
	}
	// 82E9D338: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9D33C: 4800001C  b 0x82e9d358
	pc = 0x82E9D358; continue 'dispatch;
	// 82E9D340: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E9D344: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D348: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D34C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9D350: 409A0008  bne cr6, 0x82e9d358
	if !ctx.cr[6].eq {
	pc = 0x82E9D358; continue 'dispatch;
	}
	// 82E9D354: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E9D358: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D35C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82E9D360: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82E9D364: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82E9D368: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9D36C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9D370: 409A00F0  bne cr6, 0x82e9d460
	if !ctx.cr[6].eq {
	pc = 0x82E9D460; continue 'dispatch;
	}
	// 82E9D374: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E9D378: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D37C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D380: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D384: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9D388: 409A0054  bne cr6, 0x82e9d3dc
	if !ctx.cr[6].eq {
	pc = 0x82E9D3DC; continue 'dispatch;
	}
	// 82E9D38C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D390: 892A0040  lbz r9, 0x40(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9D394: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9D398: 419A0054  beq cr6, 0x82e9d3ec
	if ctx.cr[6].eq {
	pc = 0x82E9D3EC; continue 'dispatch;
	}
	// 82E9D39C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D3A0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D3A4: 409A0010  bne cr6, 0x82e9d3b4
	if !ctx.cr[6].eq {
	pc = 0x82E9D3B4; continue 'dispatch;
	}
	// 82E9D3A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D3AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9D3B0: 4BFFE681  bl 0x82e9ba30
	ctx.lr = 0x82E9D3B4;
	sub_82E9BA30(ctx, base);
	// 82E9D3B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D3B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D3BC: 9BAB0040  stb r29, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9D3C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D3C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D3C8: 9B6B0040  stb r27, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[27].u8 ) };
	// 82E9D3CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D3D0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D3D4: 4BFFE2BD  bl 0x82e9b690
	ctx.lr = 0x82E9D3D8;
	sub_82E9B690(ctx, base);
	// 82E9D3D8: 48000074  b 0x82e9d44c
	pc = 0x82E9D44C; continue 'dispatch;
	// 82E9D3DC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D3E0: 892A0040  lbz r9, 0x40(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9D3E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9D3E8: 409A0028  bne cr6, 0x82e9d410
	if !ctx.cr[6].eq {
	pc = 0x82E9D410; continue 'dispatch;
	}
	// 82E9D3EC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D3F0: 9BA90040  stb r29, 0x40(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9D3F4: 9BAA0040  stb r29, 0x40(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9D3F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D3FC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D400: 9B6A0040  stb r27, 0x40(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(64 as u32), ctx.r[27].u8 ) };
	// 82E9D404: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D408: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D40C: 48000040  b 0x82e9d44c
	pc = 0x82E9D44C; continue 'dispatch;
	// 82E9D410: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D414: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D418: 409A0010  bne cr6, 0x82e9d428
	if !ctx.cr[6].eq {
	pc = 0x82E9D428; continue 'dispatch;
	}
	// 82E9D41C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D420: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9D424: 4BFFE26D  bl 0x82e9b690
	ctx.lr = 0x82E9D428;
	sub_82E9B690(ctx, base);
	// 82E9D428: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D42C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D430: 9BAB0040  stb r29, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9D434: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D438: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D43C: 9B6B0040  stb r27, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[27].u8 ) };
	// 82E9D440: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D444: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D448: 4BFFE5E9  bl 0x82e9ba30
	ctx.lr = 0x82E9D44C;
	sub_82E9BA30(ctx, base);
	// 82E9D44C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D450: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82E9D454: 894A0040  lbz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9D458: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9D45C: 419AFF1C  beq cr6, 0x82e9d378
	if ctx.cr[6].eq {
	pc = 0x82E9D378; continue 'dispatch;
	}
	// 82E9D460: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D464: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E9D468: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9D46C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D470: 9BAB0040  stb r29, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E9D474: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E9D478: 4830AD38  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D480 size=76
    let mut pc: u32 = 0x82E9D480;
    'dispatch: loop {
        match pc {
            0x82E9D480 => {
    //   block [0x82E9D480..0x82E9D4CC)
	// 82E9D480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9D488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9D48C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9D494: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9D498: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E9D49C: 396BE5DC  addi r11, r11, -0x1a24
	ctx.r[11].s64 = ctx.r[11].s64 + -6692;
	// 82E9D4A0: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E9D4A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9D4A8: 4BFFE869  bl 0x82e9bd10
	ctx.lr = 0x82E9D4AC;
	sub_82E9BD10(ctx, base);
	// 82E9D4AC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E9D4B0: 4BFFE8B9  bl 0x82e9bd68
	ctx.lr = 0x82E9D4B4;
	sub_82E9BD68(ctx, base);
	// 82E9D4B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9D4B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9D4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9D4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9D4C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9D4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D4D0 size=76
    let mut pc: u32 = 0x82E9D4D0;
    'dispatch: loop {
        match pc {
            0x82E9D4D0 => {
    //   block [0x82E9D4D0..0x82E9D51C)
	// 82E9D4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9D4D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9D4DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9D4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D4E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9D4E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9D4EC: 4BFFFCED  bl 0x82e9d1d8
	ctx.lr = 0x82E9D4F0;
	sub_82E9D1D8(ctx, base);
	// 82E9D4F0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9D4F4: 4182000C  beq 0x82e9d500
	if ctx.cr[0].eq {
	pc = 0x82E9D500; continue 'dispatch;
	}
	// 82E9D4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9D4FC: 4BF54EDD  bl 0x82df23d8
	ctx.lr = 0x82E9D500;
	sub_82DF23D8(ctx, base);
	// 82E9D500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9D504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9D508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9D50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9D510: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9D514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9D518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D520 size=304
    let mut pc: u32 = 0x82E9D520;
    'dispatch: loop {
        match pc {
            0x82E9D520 => {
    //   block [0x82E9D520..0x82E9D650)
	// 82E9D520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D524: 4830AC3D  bl 0x831a8160
	ctx.lr = 0x82E9D528;
	sub_831A8130(ctx, base);
	// 82E9D528: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D52C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E9D530: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82E9D534: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9D538: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E9D53C: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82E9D540: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D544: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D548: 48000050  b 0x82e9d598
	pc = 0x82E9D598; continue 'dispatch;
	// 82E9D54C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E9D550: 392B000C  addi r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 + 12;
	// 82E9D554: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E9D558: 391D0028  addi r8, r29, 0x28
	ctx.r[8].s64 = ctx.r[29].s64 + 40;
	// 82E9D55C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D560: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D564: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E9D568: 40820014  bne 0x82e9d57c
	if !ctx.cr[0].eq {
	pc = 0x82E9D57C; continue 'dispatch;
	}
	// 82E9D56C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D570: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E9D574: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9D578: 409AFFE4  bne cr6, 0x82e9d55c
	if !ctx.cr[6].eq {
	pc = 0x82E9D55C; continue 'dispatch;
	}
	// 82E9D57C: 7CEA0034  cntlzw r10, r7
	ctx.r[10].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 82E9D580: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E9D584: 555CDFFF  rlwinm. r28, r10, 0x1b, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E9D588: 4182000C  beq 0x82e9d594
	if ctx.cr[0].eq {
	pc = 0x82E9D594; continue 'dispatch;
	}
	// 82E9D58C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D590: 48000008  b 0x82e9d598
	pc = 0x82E9D598; continue 'dispatch;
	// 82E9D594: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D598: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9D59C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E9D5A0: 419AFFAC  beq cr6, 0x82e9d54c
	if ctx.cr[6].eq {
	pc = 0x82E9D54C; continue 'dispatch;
	}
	// 82E9D5A4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9D5A8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9D5AC: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82E9D5B0: 41820048  beq 0x82e9d5f8
	if ctx.cr[0].eq {
	pc = 0x82E9D5F8; continue 'dispatch;
	}
	// 82E9D5B4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D5B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D5BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D5C0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D5C4: 409A002C  bne cr6, 0x82e9d5f0
	if !ctx.cr[6].eq {
	pc = 0x82E9D5F0; continue 'dispatch;
	}
	// 82E9D5C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E9D5CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E9D5D0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E9D5D4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E9D5D8: 4BFFFC81  bl 0x82e9d258
	ctx.lr = 0x82E9D5DC;
	sub_82E9D258(ctx, base);
	// 82E9D5DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E9D5E0: 9B5E0004  stb r26, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82E9D5E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D5E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9D5EC: 48000058  b 0x82e9d644
	pc = 0x82E9D644; continue 'dispatch;
	// 82E9D5F0: 4BFFE181  bl 0x82e9b770
	ctx.lr = 0x82E9D5F4;
	sub_82E9B770(ctx, base);
	// 82E9D5F4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9D5F8: 3967000C  addi r11, r7, 0xc
	ctx.r[11].s64 = ctx.r[7].s64 + 12;
	// 82E9D5FC: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E9D600: 390B0028  addi r8, r11, 0x28
	ctx.r[8].s64 = ctx.r[11].s64 + 40;
	// 82E9D604: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D608: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D60C: 7D264851  subf. r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9D610: 40820014  bne 0x82e9d624
	if !ctx.cr[0].eq {
	pc = 0x82E9D624; continue 'dispatch;
	}
	// 82E9D614: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D618: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D61C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9D620: 409AFFE4  bne cr6, 0x82e9d604
	if !ctx.cr[6].eq {
	pc = 0x82E9D604; continue 'dispatch;
	}
	// 82E9D624: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D628: 40800010  bge 0x82e9d638
	if !ctx.cr[0].lt {
	pc = 0x82E9D638; continue 'dispatch;
	}
	// 82E9D62C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E9D630: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D634: 4BFFFF98  b 0x82e9d5cc
	pc = 0x82E9D5CC; continue 'dispatch;
	// 82E9D638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9D63C: 90FE0000  stw r7, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E9D640: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82E9D644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D648: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E9D64C: 4830AB64  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D650 size=592
    let mut pc: u32 = 0x82E9D650;
    'dispatch: loop {
        match pc {
            0x82E9D650 => {
    //   block [0x82E9D650..0x82E9D8A0)
	// 82E9D650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D654: 4830AB0D  bl 0x831a8160
	ctx.lr = 0x82E9D658;
	sub_831A8130(ctx, base);
	// 82E9D658: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D65C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E9D660: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E9D664: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E9D668: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E9D66C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D670: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9D674: 409A0020  bne cr6, 0x82e9d694
	if !ctx.cr[6].eq {
	pc = 0x82E9D694; continue 'dispatch;
	}
	// 82E9D678: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D67C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E9D680: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9D684: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9D688: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9D68C: 4BFFFBCD  bl 0x82e9d258
	ctx.lr = 0x82E9D690;
	sub_82E9D258(ctx, base);
	// 82E9D690: 48000204  b 0x82e9d894
	pc = 0x82E9D894; continue 'dispatch;
	// 82E9D694: 839A0004  lwz r28, 4(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D698: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D69C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D6A0: 409A0040  bne cr6, 0x82e9d6e0
	if !ctx.cr[6].eq {
	pc = 0x82E9D6E0; continue 'dispatch;
	}
	// 82E9D6A4: 395D000C  addi r10, r29, 0xc
	ctx.r[10].s64 = ctx.r[29].s64 + 12;
	// 82E9D6A8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9D6AC: 391F0028  addi r8, r31, 0x28
	ctx.r[8].s64 = ctx.r[31].s64 + 40;
	// 82E9D6B0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D6B4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D6B8: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9D6BC: 40820014  bne 0x82e9d6d0
	if !ctx.cr[0].eq {
	pc = 0x82E9D6D0; continue 'dispatch;
	}
	// 82E9D6C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D6C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D6C8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9D6CC: 409AFFE4  bne cr6, 0x82e9d6b0
	if !ctx.cr[6].eq {
	pc = 0x82E9D6B0; continue 'dispatch;
	}
	// 82E9D6D0: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D6D4: 408001A4  bge 0x82e9d878
	if !ctx.cr[0].lt {
	pc = 0x82E9D878; continue 'dispatch;
	}
	// 82E9D6D8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E9D6DC: 4BFFFFA0  b 0x82e9d67c
	pc = 0x82E9D67C; continue 'dispatch;
	// 82E9D6E0: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E9D6E4: 409A0044  bne cr6, 0x82e9d728
	if !ctx.cr[6].eq {
	pc = 0x82E9D728; continue 'dispatch;
	}
	// 82E9D6E8: 80DC0008  lwz r6, 8(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D6EC: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9D6F0: 3966000C  addi r11, r6, 0xc
	ctx.r[11].s64 = ctx.r[6].s64 + 12;
	// 82E9D6F4: 392B0028  addi r9, r11, 0x28
	ctx.r[9].s64 = ctx.r[11].s64 + 40;
	// 82E9D6F8: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D6FC: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D700: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9D704: 40820014  bne 0x82e9d718
	if !ctx.cr[0].eq {
	pc = 0x82E9D718; continue 'dispatch;
	}
	// 82E9D708: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D70C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D710: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E9D714: 409AFFE4  bne cr6, 0x82e9d6f8
	if !ctx.cr[6].eq {
	pc = 0x82E9D6F8; continue 'dispatch;
	}
	// 82E9D718: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D71C: 4080015C  bge 0x82e9d878
	if !ctx.cr[0].lt {
	pc = 0x82E9D878; continue 'dispatch;
	}
	// 82E9D720: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9D724: 4BFFFF5C  b 0x82e9d680
	pc = 0x82E9D680; continue 'dispatch;
	// 82E9D728: 3BDD000C  addi r30, r29, 0xc
	ctx.r[30].s64 = ctx.r[29].s64 + 12;
	// 82E9D72C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9D730: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82E9D734: 393F0028  addi r9, r31, 0x28
	ctx.r[9].s64 = ctx.r[31].s64 + 40;
	// 82E9D738: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D73C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D740: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9D744: 40820014  bne 0x82e9d758
	if !ctx.cr[0].eq {
	pc = 0x82E9D758; continue 'dispatch;
	}
	// 82E9D748: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D74C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D750: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E9D754: 409AFFE4  bne cr6, 0x82e9d738
	if !ctx.cr[6].eq {
	pc = 0x82E9D738; continue 'dispatch;
	}
	// 82E9D758: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D75C: 40800078  bge 0x82e9d7d4
	if !ctx.cr[0].lt {
	pc = 0x82E9D7D4; continue 'dispatch;
	}
	// 82E9D760: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82E9D764: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D768: 4BFFE009  bl 0x82e9b770
	ctx.lr = 0x82E9D76C;
	sub_82E9B770(ctx, base);
	// 82E9D76C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9D770: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9D774: 3966000C  addi r11, r6, 0xc
	ctx.r[11].s64 = ctx.r[6].s64 + 12;
	// 82E9D778: 392B0028  addi r9, r11, 0x28
	ctx.r[9].s64 = ctx.r[11].s64 + 40;
	// 82E9D77C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D780: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D784: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9D788: 40820014  bne 0x82e9d79c
	if !ctx.cr[0].eq {
	pc = 0x82E9D79C; continue 'dispatch;
	}
	// 82E9D78C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D790: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D794: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E9D798: 409AFFE4  bne cr6, 0x82e9d77c
	if !ctx.cr[6].eq {
	pc = 0x82E9D77C; continue 'dispatch;
	}
	// 82E9D79C: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D7A0: 40800034  bge 0x82e9d7d4
	if !ctx.cr[0].lt {
	pc = 0x82E9D7D4; continue 'dispatch;
	}
	// 82E9D7A4: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D7A8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9D7AC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9D7B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9D7B4: 896B0041  lbz r11, 0x41(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9D7B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9D7BC: 419A000C  beq cr6, 0x82e9d7c8
	if ctx.cr[6].eq {
	pc = 0x82E9D7C8; continue 'dispatch;
	}
	// 82E9D7C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9D7C4: 4BFFFEC8  b 0x82e9d68c
	pc = 0x82E9D68C; continue 'dispatch;
	// 82E9D7C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E9D7CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E9D7D0: 4BFFFEBC  b 0x82e9d68c
	pc = 0x82E9D68C; continue 'dispatch;
	// 82E9D7D4: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E9D7D8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E9D7DC: 391E0028  addi r8, r30, 0x28
	ctx.r[8].s64 = ctx.r[30].s64 + 40;
	// 82E9D7E0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D7E4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D7E8: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9D7EC: 40820014  bne 0x82e9d800
	if !ctx.cr[0].eq {
	pc = 0x82E9D800; continue 'dispatch;
	}
	// 82E9D7F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D7F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D7F8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9D7FC: 409AFFE4  bne cr6, 0x82e9d7e0
	if !ctx.cr[6].eq {
	pc = 0x82E9D7E0; continue 'dispatch;
	}
	// 82E9D800: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D804: 40800074  bge 0x82e9d878
	if !ctx.cr[0].lt {
	pc = 0x82E9D878; continue 'dispatch;
	}
	// 82E9D808: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82E9D80C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D810: 4BFFE1A9  bl 0x82e9b9b8
	ctx.lr = 0x82E9D814;
	sub_82E9B9B8(ctx, base);
	// 82E9D814: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E9D818: 7F06E040  cmplw cr6, r6, r28
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E9D81C: 419A0038  beq cr6, 0x82e9d854
	if ctx.cr[6].eq {
	pc = 0x82E9D854; continue 'dispatch;
	}
	// 82E9D820: 3946000C  addi r10, r6, 0xc
	ctx.r[10].s64 = ctx.r[6].s64 + 12;
	// 82E9D824: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E9D828: 391F0028  addi r8, r31, 0x28
	ctx.r[8].s64 = ctx.r[31].s64 + 40;
	// 82E9D82C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D830: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D834: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9D838: 40820014  bne 0x82e9d84c
	if !ctx.cr[0].eq {
	pc = 0x82E9D84C; continue 'dispatch;
	}
	// 82E9D83C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D840: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D844: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9D848: 409AFFE4  bne cr6, 0x82e9d82c
	if !ctx.cr[6].eq {
	pc = 0x82E9D82C; continue 'dispatch;
	}
	// 82E9D84C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D850: 40800028  bge 0x82e9d878
	if !ctx.cr[0].lt {
	pc = 0x82E9D878; continue 'dispatch;
	}
	// 82E9D854: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9D858: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E9D85C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9D860: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9D864: 896B0041  lbz r11, 0x41(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82E9D868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9D86C: 419AFF60  beq cr6, 0x82e9d7cc
	if ctx.cr[6].eq {
	pc = 0x82E9D7CC; continue 'dispatch;
	}
	// 82E9D870: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E9D874: 4BFFFF4C  b 0x82e9d7c0
	pc = 0x82E9D7C0; continue 'dispatch;
	// 82E9D878: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9D87C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E9D880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D884: 4BFFFC9D  bl 0x82e9d520
	ctx.lr = 0x82E9D888;
	sub_82E9D520(ctx, base);
	// 82E9D888: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E9D88C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D890: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9D894: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E9D898: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E9D89C: 4830A914  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D8A0 size=224
    let mut pc: u32 = 0x82E9D8A0;
    'dispatch: loop {
        match pc {
            0x82E9D8A0 => {
    //   block [0x82E9D8A0..0x82E9D980)
	// 82E9D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D8A4: 4830A8C9  bl 0x831a816c
	ctx.lr = 0x82E9D8A8;
	sub_831A8130(ctx, base);
	// 82E9D8A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D8AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9D8B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9D8B4: 4BFFE0A5  bl 0x82e9b958
	ctx.lr = 0x82E9D8B8;
	sub_82E9B958(ctx, base);
	// 82E9D8B8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D8BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9D8C0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9D8C4: 419A0038  beq cr6, 0x82e9d8fc
	if ctx.cr[6].eq {
	pc = 0x82E9D8FC; continue 'dispatch;
	}
	// 82E9D8C8: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82E9D8CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E9D8D0: 391E0028  addi r8, r30, 0x28
	ctx.r[8].s64 = ctx.r[30].s64 + 40;
	// 82E9D8D4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D8D8: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D8DC: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9D8E0: 40820014  bne 0x82e9d8f4
	if !ctx.cr[0].eq {
	pc = 0x82E9D8F4; continue 'dispatch;
	}
	// 82E9D8E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9D8E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9D8EC: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9D8F0: 409AFFE4  bne cr6, 0x82e9d8d4
	if !ctx.cr[6].eq {
	pc = 0x82E9D8D4; continue 'dispatch;
	}
	// 82E9D8F4: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9D8F8: 4080007C  bge 0x82e9d974
	if !ctx.cr[0].lt {
	pc = 0x82E9D974; continue 'dispatch;
	}
	// 82E9D8FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E9D900: 4BFC3499  bl 0x82e60d98
	ctx.lr = 0x82E9D904;
	sub_82E60D98(ctx, base);
	// 82E9D904: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82E9D908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9D90C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9D910: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E9D914: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E9D918: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9D91C: 4830ABF5  bl 0x831a8510
	ctx.lr = 0x82E9D920;
	sub_831A8510(ctx, base);
	// 82E9D920: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E9D924: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E9D928: 4BFC60D1  bl 0x82e639f8
	ctx.lr = 0x82E9D92C;
	sub_82E639F8(ctx, base);
	// 82E9D92C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82E9D930: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9D934: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E9D938: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9D93C: 4BFFFD15  bl 0x82e9d650
	ctx.lr = 0x82E9D940;
	sub_82E9D650(ctx, base);
	// 82E9D940: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E9D944: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E9D948: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D94C: 4BFC326D  bl 0x82e60bb8
	ctx.lr = 0x82E9D950;
	sub_82E60BB8(ctx, base);
	// 82E9D950: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 82E9D954: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E9D958: 807E110C  lwz r3, 0x110c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9D95C: 4BF5482D  bl 0x82df2188
	ctx.lr = 0x82E9D960;
	sub_82DF2188(ctx, base);
	// 82E9D960: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E9D964: 4BFC3255  bl 0x82e60bb8
	ctx.lr = 0x82E9D968;
	sub_82E60BB8(ctx, base);
	// 82E9D968: 807E110C  lwz r3, 0x110c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9D96C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E9D970: 4BF54819  bl 0x82df2188
	ctx.lr = 0x82E9D974;
	sub_82DF2188(ctx, base);
	// 82E9D974: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 82E9D978: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E9D97C: 4830A840  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9D980 size=140
    let mut pc: u32 = 0x82E9D980;
    'dispatch: loop {
        match pc {
            0x82E9D980 => {
    //   block [0x82E9D980..0x82E9DA0C)
	// 82E9D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9D984: 4830A7E5  bl 0x831a8168
	ctx.lr = 0x82E9D988;
	sub_831A8130(ctx, base);
	// 82E9D988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9D98C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9D990: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9D994: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E9D998: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9D99C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82E9D9A0: 419A0008  beq cr6, 0x82e9d9a8
	if ctx.cr[6].eq {
	pc = 0x82E9D9A8; continue 'dispatch;
	}
	// 82E9D9A4: 4BD415BD  bl 0x82bdef60
	ctx.lr = 0x82E9D9A8;
	sub_82BDEF60(ctx, base);
	// 82E9D9A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9D9AC: 4BFFD025  bl 0x82e9a9d0
	ctx.lr = 0x82E9D9B0;
	sub_82E9A9D0(ctx, base);
	// 82E9D9B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E9D9B4: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82E9D9B8: 4BFFFEE9  bl 0x82e9d8a0
	ctx.lr = 0x82E9D9BC;
	sub_82E9D8A0(ctx, base);
	// 82E9D9BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9D9C0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E9D9C4: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D9C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E9D9CC: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D9D0: 4BFC3419  bl 0x82e60de8
	ctx.lr = 0x82E9D9D4;
	sub_82E60DE8(ctx, base);
	// 82E9D9D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9D9D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9D9DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9D9E0: 4BF8B9C9  bl 0x82e293a8
	ctx.lr = 0x82E9D9E4;
	sub_82E293A8(ctx, base);
	// 82E9D9E4: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E9D9E8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9D9EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9D9F0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9D9F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9D9F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9D9FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9DA00: 4E800421  bctrl
	ctx.lr = 0x82E9DA04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9DA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9DA08: 4830A7B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9DA10 size=164
    let mut pc: u32 = 0x82E9DA10;
    'dispatch: loop {
        match pc {
            0x82E9DA10 => {
    //   block [0x82E9DA10..0x82E9DAB4)
	// 82E9DA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DA14: 4830A759  bl 0x831a816c
	ctx.lr = 0x82E9DA18;
	sub_831A8130(ctx, base);
	// 82E9DA18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DA1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9DA20: 4BFFD471  bl 0x82e9ae90
	ctx.lr = 0x82E9DA24;
	sub_82E9AE90(ctx, base);
	// 82E9DA24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9DA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9DA2C: 4BFFD465  bl 0x82e9ae90
	ctx.lr = 0x82E9DA30;
	sub_82E9AE90(ctx, base);
	// 82E9DA30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9DA34: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9DA38: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9DA3C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9DA40: 41990068  bgt cr6, 0x82e9daa8
	if ctx.cr[6].gt {
	pc = 0x82E9DAA8; continue 'dispatch;
	}
	// 82E9DA44: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E9DA48: 811E0028  lwz r8, 0x28(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E9DA4C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E9DA50: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E9DA54: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9DA58: 41990050  bgt cr6, 0x82e9daa8
	if ctx.cr[6].gt {
	pc = 0x82E9DAA8; continue 'dispatch;
	}
	// 82E9DA5C: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E9DA60: 480011E9  bl 0x82e9ec48
	ctx.lr = 0x82E9DA64;
	sub_82E9EC48(ctx, base);
	// 82E9DA64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9DA68: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E9DA6C: 480011DD  bl 0x82e9ec48
	ctx.lr = 0x82E9DA70;
	sub_82E9EC48(ctx, base);
	// 82E9DA70: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E9DA74: 409A0034  bne cr6, 0x82e9daa8
	if !ctx.cr[6].eq {
	pc = 0x82E9DAA8; continue 'dispatch;
	}
	// 82E9DA78: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E9DA7C: 48001135  bl 0x82e9ebb0
	ctx.lr = 0x82E9DA80;
	sub_82E9EBB0(ctx, base);
	// 82E9DA80: 547D063E  clrlwi r29, r3, 0x18
	ctx.r[29].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E9DA84: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E9DA88: 48001129  bl 0x82e9ebb0
	ctx.lr = 0x82E9DA8C;
	sub_82E9EBB0(ctx, base);
	// 82E9DA8C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E9DA90: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9DA94: 409A0014  bne cr6, 0x82e9daa8
	if !ctx.cr[6].eq {
	pc = 0x82E9DAA8; continue 'dispatch;
	}
	// 82E9DA98: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9DA9C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9DAA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9DAA4: 41820008  beq 0x82e9daac
	if ctx.cr[0].eq {
	pc = 0x82E9DAAC; continue 'dispatch;
	}
	// 82E9DAA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9DAAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9DAB0: 4830A70C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9DAB8 size=8
    let mut pc: u32 = 0x82E9DAB8;
    'dispatch: loop {
        match pc {
            0x82E9DAB8 => {
    //   block [0x82E9DAB8..0x82E9DAC0)
	// 82E9DAB8: 3863002C  addi r3, r3, 0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + 44;
	// 82E9DABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9DAC0 size=60
    let mut pc: u32 = 0x82E9DAC0;
    'dispatch: loop {
        match pc {
            0x82E9DAC0 => {
    //   block [0x82E9DAC0..0x82E9DAFC)
	// 82E9DAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9DAC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9DACC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DAD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9DAD4: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E9DAD8: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E9DADC: 4830AA35  bl 0x831a8510
	ctx.lr = 0x82E9DAE0;
	sub_831A8510(ctx, base);
	// 82E9DAE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9DAE4: 997F0095  stb r11, 0x95(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(149 as u32), ctx.r[11].u8 ) };
	// 82E9DAE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9DAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9DAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9DAF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9DAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9DB00 size=48
    let mut pc: u32 = 0x82E9DB00;
    'dispatch: loop {
        match pc {
            0x82E9DB00 => {
    //   block [0x82E9DB00..0x82E9DB30)
	// 82E9DB00: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DB04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E9DB08: 39230084  addi r9, r3, 0x84
	ctx.r[9].s64 = ctx.r[3].s64 + 132;
	// 82E9DB0C: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E9DB10: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9DB14: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82E9DB18: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9DB1C: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82E9DB20: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9DB24: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82E9DB28: 99430095  stb r10, 0x95(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(149 as u32), ctx.r[10].u8 ) };
	// 82E9DB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9DB30 size=76
    let mut pc: u32 = 0x82E9DB30;
    'dispatch: loop {
        match pc {
            0x82E9DB30 => {
    //   block [0x82E9DB30..0x82E9DB7C)
	// 82E9DB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DB34: 4830A639  bl 0x831a816c
	ctx.lr = 0x82E9DB38;
	sub_831A8130(ctx, base);
	// 82E9DB38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DB3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9DB40: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82E9DB44: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E9DB48: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DB4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9DB50: 419A0008  beq cr6, 0x82e9db58
	if ctx.cr[6].eq {
	pc = 0x82E9DB58; continue 'dispatch;
	}
	// 82E9DB54: 4BFFD6A5  bl 0x82e9b1f8
	ctx.lr = 0x82E9DB58;
	sub_82E9B1F8(ctx, base);
	// 82E9DB58: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E9DB5C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E9DB60: 4082FFE8  bne 0x82e9db48
	if !ctx.cr[0].eq {
	pc = 0x82E9DB48; continue 'dispatch;
	}
	// 82E9DB64: 807D0020  lwz r3, 0x20(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E9DB68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9DB6C: 419A0008  beq cr6, 0x82e9db74
	if ctx.cr[6].eq {
	pc = 0x82E9DB74; continue 'dispatch;
	}
	// 82E9DB70: 4BFFD689  bl 0x82e9b1f8
	ctx.lr = 0x82E9DB74;
	sub_82E9B1F8(ctx, base);
	// 82E9DB74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9DB78: 4830A644  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9DB80 size=108
    let mut pc: u32 = 0x82E9DB80;
    'dispatch: loop {
        match pc {
            0x82E9DB80 => {
    //   block [0x82E9DB80..0x82E9DBEC)
	// 82E9DB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9DB88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9DB8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9DB90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DB94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9DB98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9DB9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9DBA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9DBA4: 4BFF8E75  bl 0x82e96a18
	ctx.lr = 0x82E9DBA8;
	sub_82E96A18(ctx, base);
	// 82E9DBA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E9DBAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9DBB0: 4BFF8C01  bl 0x82e967b0
	ctx.lr = 0x82E9DBB4;
	sub_82E967B0(ctx, base);
	// 82E9DBB4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E9DBB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9DBBC: 419A0008  beq cr6, 0x82e9dbc4
	if ctx.cr[6].eq {
	pc = 0x82E9DBC4; continue 'dispatch;
	}
	// 82E9DBC0: 4B422CD1  bl 0x822c0890
	ctx.lr = 0x82E9DBC4;
	sub_822C0890(ctx, base);
	// 82E9DBC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DBC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9DBCC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DBD0: 4BD38759  bl 0x82bd6328
	ctx.lr = 0x82E9DBD4;
	sub_82BD6328(ctx, base);
	// 82E9DBD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9DBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9DBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9DBE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9DBE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9DBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9DBF0 size=84
    let mut pc: u32 = 0x82E9DBF0;
    'dispatch: loop {
        match pc {
            0x82E9DBF0 => {
    //   block [0x82E9DBF0..0x82E9DC44)
	// 82E9DBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9DBF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9DBFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9DC00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DC04: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82E9DC08: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 82E9DC0C: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82E9DC10: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82E9DC14: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DC18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9DC1C: 419A0008  beq cr6, 0x82e9dc24
	if ctx.cr[6].eq {
	pc = 0x82E9DC24; continue 'dispatch;
	}
	// 82E9DC20: 4B422C71  bl 0x822c0890
	ctx.lr = 0x82E9DC24;
	sub_822C0890(ctx, base);
	// 82E9DC24: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E9DC28: 4080FFE8  bge 0x82e9dc10
	if !ctx.cr[0].lt {
	pc = 0x82E9DC10; continue 'dispatch;
	}
	// 82E9DC2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9DC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9DC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9DC38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9DC3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9DC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9DC48 size=36
    let mut pc: u32 = 0x82E9DC48;
    'dispatch: loop {
        match pc {
            0x82E9DC48 => {
    //   block [0x82E9DC48..0x82E9DC6C)
	// 82E9DC48: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9DC4C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82E9DC50: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E9DC54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9DC58: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9DC5C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9DC60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9DC64: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9DC68: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DC6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9DC6C size=36
    let mut pc: u32 = 0x82E9DC6C;
    'dispatch: loop {
        match pc {
            0x82E9DC6C => {
    //   block [0x82E9DC6C..0x82E9DC90)
	// 82E9DC6C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E9DC70: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E9DC74: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E9DC78: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E9DC7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9DC80: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E9DC84: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E9DC88: 4082FFE8  bne 0x82e9dc70
	if !ctx.cr[0].eq {
	pc = 0x82E9DC70; continue 'dispatch;
	}
	// 82E9DC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9DC90 size=24
    let mut pc: u32 = 0x82E9DC90;
    'dispatch: loop {
        match pc {
            0x82E9DC90 => {
    //   block [0x82E9DC90..0x82E9DCA8)
	// 82E9DC90: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9DC94: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9DC98: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E9DC9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9DCA0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9DCA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9DCA8 size=36
    let mut pc: u32 = 0x82E9DCA8;
    'dispatch: loop {
        match pc {
            0x82E9DCA8 => {
    //   block [0x82E9DCA8..0x82E9DCCC)
	// 82E9DCA8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E9DCAC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E9DCB0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E9DCB4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E9DCB8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9DCBC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E9DCC0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E9DCC4: 4082FFE8  bne 0x82e9dcac
	if !ctx.cr[0].eq {
	pc = 0x82E9DCAC; continue 'dispatch;
	}
	// 82E9DCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9DCD0 size=228
    let mut pc: u32 = 0x82E9DCD0;
    'dispatch: loop {
        match pc {
            0x82E9DCD0 => {
    //   block [0x82E9DCD0..0x82E9DDB4)
	// 82E9DCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DCD4: 4830A495  bl 0x831a8168
	ctx.lr = 0x82E9DCD8;
	sub_831A8130(ctx, base);
	// 82E9DCD8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E9DCDC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DCE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9DCE4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E9DCE8: 3BDF0024  addi r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 + 36;
	// 82E9DCEC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82E9DCF0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9DCF4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DCF8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9DCFC: 419A0018  beq cr6, 0x82e9dd14
	if ctx.cr[6].eq {
	pc = 0x82E9DD14; continue 'dispatch;
	}
	// 82E9DD00: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9DD04: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 82E9DD08: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82E9DD0C: 4B426755  bl 0x822c4460
	ctx.lr = 0x82E9DD10;
	sub_822C4460(ctx, base);
	// 82E9DD10: 9B9F0095  stb r28, 0x95(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(149 as u32), ctx.r[28].u8 ) };
	// 82E9DD14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9DD18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9DD1C: 409A008C  bne cr6, 0x82e9dda8
	if !ctx.cr[6].eq {
	pc = 0x82E9DDA8; continue 'dispatch;
	}
	// 82E9DD20: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DD24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9DD28: 419A0080  beq cr6, 0x82e9dda8
	if ctx.cr[6].eq {
	pc = 0x82E9DDA8; continue 'dispatch;
	}
	// 82E9DD2C: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DD30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9DD34: 4BFFD165  bl 0x82e9ae98
	ctx.lr = 0x82E9DD38;
	sub_82E9AE98(ctx, base);
	// 82E9DD38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9DD3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9DD40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9DD44: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E9DD48: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E9DD4C: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E9DD50: 4822B589  bl 0x830c92d8
	ctx.lr = 0x82E9DD54;
	sub_830C92D8(ctx, base);
	// 82E9DD54: 7BAB0020  clrldi r11, r29, 0x20
	ctx.r[11].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 82E9DD58: D3E10070  stfs f31, 0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E9DD5C: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82E9DD60: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E9DD64: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9DD68: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82E9DD6C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9DD70: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 82E9DD74: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9DD78: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E9DD7C: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9DD80: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E9DD84: FC006018  frsp f0, f12
	ctx.f[0].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82E9DD88: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82E9DD8C: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E9DD90: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E9DD94: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E9DD98: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E9DD9C: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E9DDA0: 4830A771  bl 0x831a8510
	ctx.lr = 0x82E9DDA4;
	sub_831A8510(ctx, base);
	// 82E9DDA4: 9B9F0095  stb r28, 0x95(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(149 as u32), ctx.r[28].u8 ) };
	// 82E9DDA8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E9DDAC: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E9DDB0: 4830A408  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9DDB8 size=80
    let mut pc: u32 = 0x82E9DDB8;
    'dispatch: loop {
        match pc {
            0x82E9DDB8 => {
    //   block [0x82E9DDB8..0x82E9DE08)
	// 82E9DDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DDBC: 4830A3A9  bl 0x831a8164
	ctx.lr = 0x82E9DDC0;
	sub_831A8130(ctx, base);
	// 82E9DDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DDC4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9DDC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9DDCC: 3BFC0004  addi r31, r28, 4
	ctx.r[31].s64 = ctx.r[28].s64 + 4;
	// 82E9DDD0: 7F7CF050  subf r27, r28, r30
	ctx.r[27].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 82E9DDD4: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82E9DDD8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DDDC: 7C9BFA14  add r4, r27, r31
	ctx.r[4].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 82E9DDE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9DDE4: 917FFFFC  stw r11, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82E9DDE8: 4B426679  bl 0x822c4460
	ctx.lr = 0x82E9DDEC;
	sub_822C4460(ctx, base);
	// 82E9DDEC: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E9DDF0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E9DDF4: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E9DDF8: 4082FFE0  bne 0x82e9ddd8
	if !ctx.cr[0].eq {
	pc = 0x82E9DDD8; continue 'dispatch;
	}
	// 82E9DDFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E9DE00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9DE04: 4830A3B0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9DE08 size=944
    let mut pc: u32 = 0x82E9DE08;
    'dispatch: loop {
        match pc {
            0x82E9DE08 => {
    //   block [0x82E9DE08..0x82E9E1B8)
	// 82E9DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9DE0C: 4830A331  bl 0x831a813c
	ctx.lr = 0x82E9DE10;
	sub_831A8130(ctx, base);
	// 82E9DE10: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82E9DE14: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82E9DE18: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9DE1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9DE20: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E9DE24: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 82E9DE28: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82E9DE2C: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9DE30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9DE34: 419A0010  beq cr6, 0x82e9de44
	if ctx.cr[6].eq {
	pc = 0x82E9DE44; continue 'dispatch;
	}
	// 82E9DE38: 4BFF6981  bl 0x82e947b8
	ctx.lr = 0x82E9DE3C;
	sub_82E947B8(ctx, base);
	// 82E9DE3C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E9DE40: 4800000C  b 0x82e9de4c
	pc = 0x82E9DE4C; continue 'dispatch;
	// 82E9DE44: 3EC02D20  lis r22, 0x2d20
	ctx.r[22].s64 = 757071872;
	// 82E9DE48: 62D60196  ori r22, r22, 0x196
	ctx.r[22].u64 = ctx.r[22].u64 | 406;
	// 82E9DE4C: 809F0064  lwz r4, 0x64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9DE50: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82E9DE54: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82E9DE58: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E9DE5C: 419A00DC  beq cr6, 0x82e9df38
	if ctx.cr[6].eq {
	pc = 0x82E9DF38; continue 'dispatch;
	}
	// 82E9DE60: 89750000  lbz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DE64: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9DE68: 408200D0  bne 0x82e9df38
	if !ctx.cr[0].eq {
	pc = 0x82E9DF38; continue 'dispatch;
	}
	// 82E9DE6C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9DE70: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9DE74: 409A000C  bne cr6, 0x82e9de80
	if !ctx.cr[6].eq {
	pc = 0x82E9DE80; continue 'dispatch;
	}
	// 82E9DE78: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9DE7C: 418200BC  beq 0x82e9df38
	if ctx.cr[0].eq {
	pc = 0x82E9DF38; continue 'dispatch;
	}
	// 82E9DE80: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E9DE84: 4BFFD0D5  bl 0x82e9af58
	ctx.lr = 0x82E9DE88;
	sub_82E9AF58(ctx, base);
	// 82E9DE88: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E9DE8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9DE90: 419A0098  beq cr6, 0x82e9df28
	if ctx.cr[6].eq {
	pc = 0x82E9DF28; continue 'dispatch;
	}
	// 82E9DE94: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9DE98: 4BFFCFF9  bl 0x82e9ae90
	ctx.lr = 0x82E9DE9C;
	sub_82E9AE90(ctx, base);
	// 82E9DE9C: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E9DEA0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E9DEA4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E9DEA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DEAC: 83CA0058  lwz r30, 0x58(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E9DEB0: 83AA0054  lwz r29, 0x54(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E9DEB4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E9DEB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9DEBC: 4E800421  bctrl
	ctx.lr = 0x82E9DEC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9DEC0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9DEC4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E9DEC8: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E9DECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9DED0: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 82E9DED4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82E9DED8: 93010064  stw r24, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 82E9DEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E9DEE0: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82E9DEE4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E9DEE8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82E9DEEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9DEF0: 4BFF8931  bl 0x82e96820
	ctx.lr = 0x82E9DEF4;
	sub_82E96820(ctx, base);
	// 82E9DEF4: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9DEF8: 9AEB0094  stb r23, 0x94(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[23].u8 ) };
	// 82E9DEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E9DF00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9DF04: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E9DF08: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E9DF0C: 4BFFCB1D  bl 0x82e9aa28
	ctx.lr = 0x82E9DF10;
	sub_82E9AA28(ctx, base);
	// 82E9DF10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DF14: 9AEB0094  stb r23, 0x94(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[23].u8 ) };
	// 82E9DF18: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E9DF1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9DF20: 419A0008  beq cr6, 0x82e9df28
	if ctx.cr[6].eq {
	pc = 0x82E9DF28; continue 'dispatch;
	}
	// 82E9DF24: 4B42296D  bl 0x822c0890
	ctx.lr = 0x82E9DF28;
	sub_822C0890(ctx, base);
	// 82E9DF28: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E9DF2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9DF30: 419A0008  beq cr6, 0x82e9df38
	if ctx.cr[6].eq {
	pc = 0x82E9DF38; continue 'dispatch;
	}
	// 82E9DF34: 4B42295D  bl 0x822c0890
	ctx.lr = 0x82E9DF38;
	sub_822C0890(ctx, base);
	// 82E9DF38: 897F00A4  lbz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E9DF3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9DF40: 40820010  bne 0x82e9df50
	if !ctx.cr[0].eq {
	pc = 0x82E9DF50; continue 'dispatch;
	}
	// 82E9DF44: 89750000  lbz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DF48: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9DF4C: 4082025C  bne 0x82e9e1a8
	if !ctx.cr[0].eq {
	pc = 0x82E9E1A8; continue 'dispatch;
	}
	// 82E9DF50: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9DF54: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82E9DF58: 3B9F0044  addi r28, r31, 0x44
	ctx.r[28].s64 = ctx.r[31].s64 + 68;
	// 82E9DF5C: C3CB08A8  lfs f30, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E9DF60: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DF64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9DF68: 419A0230  beq cr6, 0x82e9e198
	if ctx.cr[6].eq {
	pc = 0x82E9E198; continue 'dispatch;
	}
	// 82E9DF6C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E9DF70: 817CFFC0  lwz r11, -0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-64 as u32) ) } as u64;
	// 82E9DF74: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9DF78: 409A000C  bne cr6, 0x82e9df84
	if !ctx.cr[6].eq {
	pc = 0x82E9DF84; continue 'dispatch;
	}
	// 82E9DF7C: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9DF80: 41820218  beq 0x82e9e198
	if ctx.cr[0].eq {
	pc = 0x82E9E198; continue 'dispatch;
	}
	// 82E9DF84: 4BFFCF0D  bl 0x82e9ae90
	ctx.lr = 0x82E9DF88;
	sub_82E9AE90(ctx, base);
	// 82E9DF88: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E9DF8C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9DF90: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DF94: 817B0034  lwz r11, 0x34(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E9DF98: 93010094  stw r24, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[24].u32 ) };
	// 82E9DF9C: 92C10098  stw r22, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[22].u32 ) };
	// 82E9DFA0: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82E9DFA4: 4BFFCFB5  bl 0x82e9af58
	ctx.lr = 0x82E9DFA8;
	sub_82E9AF58(ctx, base);
	// 82E9DFA8: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E9DFAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9DFB0: 419A01D8  beq cr6, 0x82e9e188
	if ctx.cr[6].eq {
	pc = 0x82E9E188; continue 'dispatch;
	}
	// 82E9DFB4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DFB8: 7EFEC830  slw r30, r23, r25
	if (ctx.r[25].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[23].u32) << ((ctx.r[25].u8 & 0x1F) as u32)) as u64;
	}
	// 82E9DFBC: 7FCB5839  and. r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 & ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9DFC0: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E9DFC4: 41820034  beq 0x82e9dff8
	if ctx.cr[0].eq {
	pc = 0x82E9DFF8; continue 'dispatch;
	}
	// 82E9DFC8: 809CFFC0  lwz r4, -0x40(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-64 as u32) ) } as u64;
	// 82E9DFCC: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DFD0: 4BFFFA41  bl 0x82e9da10
	ctx.lr = 0x82E9DFD4;
	sub_82E9DA10(ctx, base);
	// 82E9DFD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9DFD8: 41820020  beq 0x82e9dff8
	if ctx.cr[0].eq {
	pc = 0x82E9DFF8; continue 'dispatch;
	}
	// 82E9DFDC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9DFE0: 7D6BF078  andc r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[30].u64;
	// 82E9DFE4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9DFE8: 807CFFC0  lwz r3, -0x40(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-64 as u32) ) } as u64;
	// 82E9DFEC: 4BFF67CD  bl 0x82e947b8
	ctx.lr = 0x82E9DFF0;
	sub_82E947B8(ctx, base);
	// 82E9DFF0: 90610090  stw r3, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 82E9DFF4: 633D0100  ori r29, r25, 0x100
	ctx.r[29].u64 = ctx.r[25].u64 | 256;
	// 82E9DFF8: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9DFFC: C3FA0008  lfs f31, 8(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E9E000: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9E004: 419A0058  beq cr6, 0x82e9e05c
	if ctx.cr[6].eq {
	pc = 0x82E9E05C; continue 'dispatch;
	}
	// 82E9E008: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E00C: 556B06B7  rlwinm. r11, r11, 0, 0x1a, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E010: 4182004C  beq 0x82e9e05c
	if ctx.cr[0].eq {
	pc = 0x82E9E05C; continue 'dispatch;
	}
	// 82E9E014: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9E018: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9E01C: 4BFFF9F5  bl 0x82e9da10
	ctx.lr = 0x82E9E020;
	sub_82E9DA10(ctx, base);
	// 82E9E020: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E024: 41820038  beq 0x82e9e05c
	if ctx.cr[0].eq {
	pc = 0x82E9E05C; continue 'dispatch;
	}
	// 82E9E028: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E02C: 556B0732  rlwinm r11, r11, 0, 0x1c, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E9E030: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9E034: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9E038: 4BFF6781  bl 0x82e947b8
	ctx.lr = 0x82E9E03C;
	sub_82E947B8(ctx, base);
	// 82E9E03C: 3D601A22  lis r11, 0x1a22
	ctx.r[11].s64 = 438435840;
	// 82E9E040: 616B0197  ori r11, r11, 0x197
	ctx.r[11].u64 = ctx.r[11].u64 | 407;
	// 82E9E044: 90610098  stw r3, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[3].u32 ) };
	// 82E9E048: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9E04C: 409A0010  bne cr6, 0x82e9e05c
	if !ctx.cr[6].eq {
	pc = 0x82E9E05C; continue 'dispatch;
	}
	// 82E9E050: C01A0008  lfs f0, 8(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9E054: 63BD0200  ori r29, r29, 0x200
	ctx.r[29].u64 = ctx.r[29].u64 | 512;
	// 82E9E058: EFFE0028  fsubs f31, f30, f0
	ctx.f[31].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E9E05C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E060: 48000CB1  bl 0x82e9ed10
	ctx.lr = 0x82E9E064;
	sub_82E9ED10(ctx, base);
	// 82E9E064: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82E9E068: 895F00A4  lbz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E9E06C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9E1B8 size=340
    let mut pc: u32 = 0x82E9E1B8;
    'dispatch: loop {
        match pc {
            0x82E9E1B8 => {
    //   block [0x82E9E1B8..0x82E9E30C)
	// 82E9E1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E1BC: 48309FAD  bl 0x831a8168
	ctx.lr = 0x82E9E1C0;
	sub_831A8130(ctx, base);
	// 82E9E1C0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82E9E1C4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E9E1C8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E1CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9E1D0: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9E1D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E9E1D8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E9E1DC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82E9E1E0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E9E1E4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E1E8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E1EC: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E9E1F0: 419A0018  beq cr6, 0x82e9e208
	if ctx.cr[6].eq {
	pc = 0x82E9E208; continue 'dispatch;
	}
	// 82E9E1F4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9E1F8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82E9E1FC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E9E200: 4B426261  bl 0x822c4460
	ctx.lr = 0x82E9E204;
	sub_822C4460(ctx, base);
	// 82E9E204: 9BBF0095  stb r29, 0x95(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(149 as u32), ctx.r[29].u8 ) };
	// 82E9E208: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9E20C: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E210: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E9E214: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E9E218: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E9E21C: C3CA08A8  lfs f30, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E9E220: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E9E224: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E9E228: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E9E22C: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82E9E230: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E9E234: D3C10074  stfs f30, 0x74(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E9E238: 419A0050  beq cr6, 0x82e9e288
	if ctx.cr[6].eq {
	pc = 0x82E9E288; continue 'dispatch;
	}
	// 82E9E23C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9E240: 4BFFCC59  bl 0x82e9ae98
	ctx.lr = 0x82E9E244;
	sub_82E9AE98(ctx, base);
	// 82E9E244: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9E248: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9E24C: D3E10060  stfs f31, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E9E250: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E9E254: 4822B085  bl 0x830c92d8
	ctx.lr = 0x82E9E258;
	sub_830C92D8(ctx, base);
	// 82E9E258: 7B8B0020  clrldi r11, r28, 0x20
	ctx.r[11].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 82E9E25C: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82E9E260: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E9E264: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9E268: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82E9E26C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9E270: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E9E274: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E9E278: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E9E27C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E9E280: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E9E284: 48000058  b 0x82e9e2dc
	pc = 0x82E9E2DC; continue 'dispatch;
	// 82E9E288: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9E28C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E9E290: 419A0058  beq cr6, 0x82e9e2e8
	if ctx.cr[6].eq {
	pc = 0x82E9E2E8; continue 'dispatch;
	}
	// 82E9E294: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9E298: 4BFFCC01  bl 0x82e9ae98
	ctx.lr = 0x82E9E29C;
	sub_82E9AE98(ctx, base);
	// 82E9E29C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E9E2A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9E2A4: D3E10060  stfs f31, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E9E2A8: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E9E2AC: 4822B02D  bl 0x830c92d8
	ctx.lr = 0x82E9E2B0;
	sub_830C92D8(ctx, base);
	// 82E9E2B0: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82E9E2B4: 7B8B0020  clrldi r11, r28, 0x20
	ctx.r[11].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 82E9E2B8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82E9E2BC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82E9E2C0: C9A10058  lfd f13, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E9E2C4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E9E2C8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9E2CC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E9E2D0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E9E2D4: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E9E2D8: FC006818  frsp f0, f13
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E9E2DC: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82E9E2E0: D3C10074  stfs f30, 0x74(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E9E2E4: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E9E2E8: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E9E2EC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E9E2F0: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E9E2F4: 4830A21D  bl 0x831a8510
	ctx.lr = 0x82E9E2F8;
	sub_831A8510(ctx, base);
	// 82E9E2F8: 9BBF0095  stb r29, 0x95(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(149 as u32), ctx.r[29].u8 ) };
	// 82E9E2FC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E9E300: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E9E304: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E9E308: 48309EB0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9E310 size=104
    let mut pc: u32 = 0x82E9E310;
    'dispatch: loop {
        match pc {
            0x82E9E310 => {
    //   block [0x82E9E310..0x82E9E378)
	// 82E9E310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9E318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9E31C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9E320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9E328: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9E32C: 4BFFFA8D  bl 0x82e9ddb8
	ctx.lr = 0x82E9E330;
	sub_82E9DDB8(ctx, base);
	// 82E9E330: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82E9E334: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82E9E338: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E9E33C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E9E340: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E9E344: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E9E348: 4B426119  bl 0x822c4460
	ctx.lr = 0x82E9E34C;
	sub_822C4460(ctx, base);
	// 82E9E34C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82E9E350: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 82E9E354: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E9E358: 4830A1B9  bl 0x831a8510
	ctx.lr = 0x82E9E35C;
	sub_831A8510(ctx, base);
	// 82E9E35C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E360: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9E364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9E368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9E36C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9E370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9E374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E9E378 size=84
    let mut pc: u32 = 0x82E9E378;
    'dispatch: loop {
        match pc {
            0x82E9E378 => {
    //   block [0x82E9E378..0x82E9E3CC)
	// 82E9E378: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E9E37C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82E9E380: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E9E384: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E9E388: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9E38C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E9E390: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E9E394: 4080FFF0  bge 0x82e9e384
	if !ctx.cr[0].lt {
	pc = 0x82E9E384; continue 'dispatch;
	}
	// 82E9E398: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9E39C: 91230020  stw r9, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E9E3A0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E9E3A4: 91230024  stw r9, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 82E9E3A8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9E3AC: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E9E3B0: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E9E3B4: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E9E3B8: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E9E3BC: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E9E3C0: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E9E3C4: D1A3003C  stfs f13, 0x3c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82E9E3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9E3D0 size=120
    let mut pc: u32 = 0x82E9E3D0;
    'dispatch: loop {
        match pc {
            0x82E9E3D0 => {
    //   block [0x82E9E3D0..0x82E9E448)
	// 82E9E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E3D4: 48309D95  bl 0x831a8168
	ctx.lr = 0x82E9E3D8;
	sub_831A8130(ctx, base);
	// 82E9E3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E3DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9E3E0: 83850000  lwz r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E3E4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E9E3E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9E3EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E3F0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9E3F4: 419A0044  beq cr6, 0x82e9e438
	if ctx.cr[6].eq {
	pc = 0x82E9E438; continue 'dispatch;
	}
	// 82E9E3F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E3FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E9E400: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E408: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9E40C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E410: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E414: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9E418: 4BF60D79  bl 0x82dff190
	ctx.lr = 0x82E9E41C;
	sub_82DFF190(ctx, base);
	// 82E9E41C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9E420: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9E424: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9E428: 4BF53D61  bl 0x82df2188
	ctx.lr = 0x82E9E42C;
	sub_82DF2188(ctx, base);
	// 82E9E42C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9E430: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E9E434: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9E438: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9E43C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9E440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9E444: 48309D74  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9E448 size=112
    let mut pc: u32 = 0x82E9E448;
    'dispatch: loop {
        match pc {
            0x82E9E448 => {
    //   block [0x82E9E448..0x82E9E4B8)
	// 82E9E448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E44C: 48309D1D  bl 0x831a8168
	ctx.lr = 0x82E9E450;
	sub_831A8130(ctx, base);
	// 82E9E450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E454: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E9E458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9E45C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E460: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E464: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9E468: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E46C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9E470: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E474: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9E478: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E9E47C: 419A0034  beq cr6, 0x82e9e4b0
	if ctx.cr[6].eq {
	pc = 0x82E9E4B0; continue 'dispatch;
	}
	// 82E9E480: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82E9E484: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E9E488: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E48C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E490: 4BF60D01  bl 0x82dff190
	ctx.lr = 0x82E9E494;
	sub_82DFF190(ctx, base);
	// 82E9E494: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9E498: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9E49C: 4BF53CED  bl 0x82df2188
	ctx.lr = 0x82E9E4A0;
	sub_82DF2188(ctx, base);
	// 82E9E4A0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E4A4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82E9E4A8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9E4AC: 409AFFD8  bne cr6, 0x82e9e484
	if !ctx.cr[6].eq {
	pc = 0x82E9E484; continue 'dispatch;
	}
	// 82E9E4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9E4B4: 48309D04  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9E4B8 size=156
    let mut pc: u32 = 0x82E9E4B8;
    'dispatch: loop {
        match pc {
            0x82E9E4B8 => {
    //   block [0x82E9E4B8..0x82E9E554)
	// 82E9E4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9E4C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9E4C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9E4C8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E4CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9E4D0: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82E9E4D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9E4D8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82E9E4DC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9E4E0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E9E4E4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E9E4E8: 40980048  bge cr6, 0x82e9e530
	if !ctx.cr[6].lt {
	pc = 0x82E9E530; continue 'dispatch;
	}
	// 82E9E4EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9E4F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9E4F4: 388B960C  addi r4, r11, -0x69f4
	ctx.r[4].s64 = ctx.r[11].s64 + -27124;
	// 82E9E4F8: 4B4273D1  bl 0x822c58c8
	ctx.lr = 0x82E9E4FC;
	sub_822C58C8(ctx, base);
	// 82E9E4FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9E500: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9E504: 4B427315  bl 0x822c5818
	ctx.lr = 0x82E9E508;
	sub_822C5818(ctx, base);
	// 82E9E508: 4B425DA9  bl 0x822c42b0
	ctx.lr = 0x82E9E50C;
	sub_822C42B0(ctx, base);
	// 82E9E50C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9E510: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E9E514: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82E9E518: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E9E51C: 4B426F55  bl 0x822c5470
	ctx.lr = 0x82E9E520;
	sub_822C5470(ctx, base);
	// 82E9E520: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9E524: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9E528: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9E52C: 4B4267B5  bl 0x822c4ce0
	ctx.lr = 0x82E9E530;
	sub_822C4CE0(ctx, base);
	// 82E9E530: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9E534: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E9E538: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9E53C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E9E540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9E544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9E548: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9E54C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9E550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9E558 size=148
    let mut pc: u32 = 0x82E9E558;
    'dispatch: loop {
        match pc {
            0x82E9E558 => {
    //   block [0x82E9E558..0x82E9E5EC)
	// 82E9E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9E560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9E564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9E568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E56C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9E570: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9E574: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 82E9E578: 396BE5E0  addi r11, r11, -0x1a20
	ctx.r[11].s64 = ctx.r[11].s64 + -6688;
	// 82E9E57C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9E580: 4BFFFEC9  bl 0x82e9e448
	ctx.lr = 0x82E9E584;
	sub_82E9E448(ctx, base);
	// 82E9E584: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E9E588: 809F009C  lwz r4, 0x9c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E9E58C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E9E590: 4BF53BF9  bl 0x82df2188
	ctx.lr = 0x82E9E594;
	sub_82DF2188(ctx, base);
	// 82E9E594: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9E598: 3BDF0044  addi r30, r31, 0x44
	ctx.r[30].s64 = ctx.r[31].s64 + 68;
	// 82E9E59C: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82E9E5A0: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E9E5A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9E5A8: 419A0008  beq cr6, 0x82e9e5b0
	if ctx.cr[6].eq {
	pc = 0x82E9E5B0; continue 'dispatch;
	}
	// 82E9E5AC: 4B4222E5  bl 0x822c0890
	ctx.lr = 0x82E9E5B0;
	sub_822C0890(ctx, base);
	// 82E9E5B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9E5B4: 4BFFF63D  bl 0x82e9dbf0
	ctx.lr = 0x82E9E5B8;
	sub_82E9DBF0(ctx, base);
	// 82E9E5B8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E9E5BC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9E5C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9E5C4: 419A0008  beq cr6, 0x82e9e5cc
	if ctx.cr[6].eq {
	pc = 0x82E9E5CC; continue 'dispatch;
	}
	// 82E9E5C8: 4B4222C9  bl 0x822c0890
	ctx.lr = 0x82E9E5CC;
	sub_822C0890(ctx, base);
	// 82E9E5CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E5D0: 4BFFF621  bl 0x82e9dbf0
	ctx.lr = 0x82E9E5D4;
	sub_82E9DBF0(ctx, base);
	// 82E9E5D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9E5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9E5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9E5E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9E5E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9E5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9E5F0 size=120
    let mut pc: u32 = 0x82E9E5F0;
    'dispatch: loop {
        match pc {
            0x82E9E5F0 => {
    //   block [0x82E9E5F0..0x82E9E668)
	// 82E9E5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E5F4: 48309B79  bl 0x831a816c
	ctx.lr = 0x82E9E5F8;
	sub_831A8130(ctx, base);
	// 82E9E5F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E5FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9E600: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9E604: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E9E608: 396BE5E0  addi r11, r11, -0x1a20
	ctx.r[11].s64 = ctx.r[11].s64 + -6688;
	// 82E9E60C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E9E610: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9E614: 4BFFFD65  bl 0x82e9e378
	ctx.lr = 0x82E9E618;
	sub_82E9E378(ctx, base);
	// 82E9E618: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 82E9E61C: 4BFFFD5D  bl 0x82e9e378
	ctx.lr = 0x82E9E620;
	sub_82E9E378(ctx, base);
	// 82E9E620: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9E624: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E9E628: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 82E9E62C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82E9E630: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 82E9E634: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9E638: 93DF0090  stw r30, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82E9E63C: D01F008C  stfs f0, 0x8c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82E9E640: 9BDF0094  stb r30, 0x94(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u8 ) };
	// 82E9E644: 9BDF0095  stb r30, 0x95(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(149 as u32), ctx.r[30].u8 ) };
	// 82E9E648: 4BFB8649  bl 0x82e56c90
	ctx.lr = 0x82E9E64C;
	sub_82E56C90(ctx, base);
	// 82E9E64C: 907F009C  stw r3, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[3].u32 ) };
	// 82E9E650: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 82E9E654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E658: 9BDF00A4  stb r30, 0xa4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u8 ) };
	// 82E9E65C: 93BF00A8  stw r29, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 82E9E660: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9E664: 48309B58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9E668 size=76
    let mut pc: u32 = 0x82E9E668;
    'dispatch: loop {
        match pc {
            0x82E9E668 => {
    //   block [0x82E9E668..0x82E9E6B4)
	// 82E9E668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9E670: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9E674: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9E678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E67C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9E680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9E684: 4BFFFED5  bl 0x82e9e558
	ctx.lr = 0x82E9E688;
	sub_82E9E558(ctx, base);
	// 82E9E688: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E68C: 4182000C  beq 0x82e9e698
	if ctx.cr[0].eq {
	pc = 0x82E9E698; continue 'dispatch;
	}
	// 82E9E690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E694: 4BF53D45  bl 0x82df23d8
	ctx.lr = 0x82E9E698;
	sub_82DF23D8(ctx, base);
	// 82E9E698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E69C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9E6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9E6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9E6A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9E6AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9E6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9E6B8 size=184
    let mut pc: u32 = 0x82E9E6B8;
    'dispatch: loop {
        match pc {
            0x82E9E6B8 => {
    //   block [0x82E9E6B8..0x82E9E770)
	// 82E9E6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E6BC: 48309AA9  bl 0x831a8164
	ctx.lr = 0x82E9E6C0;
	sub_831A8130(ctx, base);
	// 82E9E6C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E6C4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E9E6C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9E6CC: 839B0000  lwz r28, 0(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E6D0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E9E6D4: 419A0088  beq cr6, 0x82e9e75c
	if ctx.cr[6].eq {
	pc = 0x82E9E75C; continue 'dispatch;
	}
	// 82E9E6D8: 83BE009C  lwz r29, 0x9c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E9E6DC: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E6E0: 4800003C  b 0x82e9e71c
	pc = 0x82E9E71C; continue 'dispatch;
	// 82E9E6E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9E6E8: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E9E6EC: 419A007C  beq cr6, 0x82e9e768
	if ctx.cr[6].eq {
	pc = 0x82E9E768; continue 'dispatch;
	}
	// 82E9E6F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E9E6F4: 4BFFC815  bl 0x82e9af08
	ctx.lr = 0x82E9E6F8;
	sub_82E9AF08(ctx, base);
	// 82E9E6F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E6FC: 4182001C  beq 0x82e9e718
	if ctx.cr[0].eq {
	pc = 0x82E9E718; continue 'dispatch;
	}
	// 82E9E700: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E9E704: 389E0098  addi r4, r30, 0x98
	ctx.r[4].s64 = ctx.r[30].s64 + 152;
	// 82E9E708: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E9E70C: 4BFFFCC5  bl 0x82e9e3d0
	ctx.lr = 0x82E9E710;
	sub_82E9E3D0(ctx, base);
	// 82E9E710: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E714: 48000008  b 0x82e9e71c
	pc = 0x82E9E71C; continue 'dispatch;
	// 82E9E718: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E71C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E9E720: 409AFFC4  bne cr6, 0x82e9e6e4
	if !ctx.cr[6].eq {
	pc = 0x82E9E6E4; continue 'dispatch;
	}
	// 82E9E724: 83BE009C  lwz r29, 0x9c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E9E728: 3BFE0098  addi r31, r30, 0x98
	ctx.r[31].s64 = ctx.r[30].s64 + 152;
	// 82E9E72C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E9E730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E734: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E9E738: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E73C: 4BFBBE75  bl 0x82e5a5b0
	ctx.lr = 0x82E9E740;
	sub_82E5A5B0(ctx, base);
	// 82E9E740: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9E744: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9E748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E74C: 4BFFFD6D  bl 0x82e9e4b8
	ctx.lr = 0x82E9E750;
	sub_82E9E4B8(ctx, base);
	// 82E9E750: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E9E754: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9E758: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E9E75C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9E760: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E9E764: 48309A50  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82E9E768: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9E76C: 4BFFFFF4  b 0x82e9e760
	pc = 0x82E9E760; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9E770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9E770 size=1088
    let mut pc: u32 = 0x82E9E770;
    'dispatch: loop {
        match pc {
            0x82E9E770 => {
    //   block [0x82E9E770..0x82E9EBB0)
	// 82E9E770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9E774: 483099D9  bl 0x831a814c
	ctx.lr = 0x82E9E778;
	sub_831A8130(ctx, base);
	// 82E9E778: DBC1FF90  stfd f30, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[30].u64 ) };
	// 82E9E77C: DBE1FF98  stfd f31, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82E9E780: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9E784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9E788: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82E9E78C: 897F0095  lbz r11, 0x95(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(149 as u32) ) } as u64;
	// 82E9E790: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9E794: 4182040C  beq 0x82e9eba0
	if ctx.cr[0].eq {
	pc = 0x82E9EBA0; continue 'dispatch;
	}
	// 82E9E798: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82E9E79C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E9E7A0: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E9E7A4: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82E9E7A8: 9B1F0095  stb r24, 0x95(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(149 as u32), ctx.r[24].u8 ) };
	// 82E9E7AC: 3ABF0004  addi r21, r31, 4
	ctx.r[21].s64 = ctx.r[31].s64 + 4;
	// 82E9E7B0: 811F008C  lwz r8, 0x8c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E9E7B4: 3B5F0084  addi r26, r31, 0x84
	ctx.r[26].s64 = ctx.r[31].s64 + 132;
	// 82E9E7B8: 80FF0090  lwz r7, 0x90(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E9E7BC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E9E7C0: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9E7C4: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9E7C8: 91090008  stw r8, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E9E7CC: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82E9E7D0: 4BFFF361  bl 0x82e9db30
	ctx.lr = 0x82E9E7D4;
	sub_82E9DB30(ctx, base);
	// 82E9E7D4: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82E9E7D8: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E9E7DC: 88BF0094  lbz r5, 0x94(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E9E7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E7E4: 4BFFF625  bl 0x82e9de08
	ctx.lr = 0x82E9E7E8;
	sub_82E9DE08(ctx, base);
	// 82E9E7E8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E9E7EC: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9E7F0: 3B7F0024  addi r27, r31, 0x24
	ctx.r[27].s64 = ctx.r[31].s64 + 36;
	// 82E9E7F4: 7F17C378  mr r23, r24
	ctx.r[23].u64 = ctx.r[24].u64;
	// 82E9E7F8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9E7FC: 409A0010  bne cr6, 0x82e9e80c
	if !ctx.cr[6].eq {
	pc = 0x82E9E80C; continue 'dispatch;
	}
	// 82E9E800: 897F0094  lbz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E9E804: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9E808: 418200A8  beq 0x82e9e8b0
	if ctx.cr[0].eq {
	pc = 0x82E9E8B0; continue 'dispatch;
	}
	// 82E9E80C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9E810: 419A0094  beq cr6, 0x82e9e8a4
	if ctx.cr[6].eq {
	pc = 0x82E9E8A4; continue 'dispatch;
	}
	// 82E9E814: 4BFD9D35  bl 0x82e78548
	ctx.lr = 0x82E9E818;
	sub_82E78548(ctx, base);
	// 82E9E818: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E9E81C: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E9E820: 4BFFF361  bl 0x82e9db80
	ctx.lr = 0x82E9E824;
	sub_82E9DB80(ctx, base);
	// 82E9E824: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E9E828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E82C: 4BFFFE8D  bl 0x82e9e6b8
	ctx.lr = 0x82E9E830;
	sub_82E9E6B8(ctx, base);
	// 82E9E830: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E834: 40820014  bne 0x82e9e848
	if !ctx.cr[0].eq {
	pc = 0x82E9E848; continue 'dispatch;
	}
	// 82E9E838: 89760000  lbz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E83C: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E840: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82E9E844: 41820008  beq 0x82e9e84c
	if ctx.cr[0].eq {
	pc = 0x82E9E84C; continue 'dispatch;
	}
	// 82E9E848: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9E84C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E850: 40820048  bne 0x82e9e898
	if !ctx.cr[0].eq {
	pc = 0x82E9E898; continue 'dispatch;
	}
	// 82E9E854: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E858: 89640094  lbz r11, 0x94(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E9E85C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9E860: 41820038  beq 0x82e9e898
	if ctx.cr[0].eq {
	pc = 0x82E9E898; continue 'dispatch;
	}
	// 82E9E864: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E868: 556B06B7  rlwinm. r11, r11, 0, 0x1a, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E86C: 4082002C  bne 0x82e9e898
	if !ctx.cr[0].eq {
	pc = 0x82E9E898; continue 'dispatch;
	}
	// 82E9E870: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E9E874: 4BFFC6E5  bl 0x82e9af58
	ctx.lr = 0x82E9E878;
	sub_82E9AF58(ctx, base);
	// 82E9E878: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E9E87C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9E880: 419A0008  beq cr6, 0x82e9e888
	if ctx.cr[6].eq {
	pc = 0x82E9E888; continue 'dispatch;
	}
	// 82E9E884: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82E9E888: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E9E88C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9E890: 419A0008  beq cr6, 0x82e9e898
	if ctx.cr[6].eq {
	pc = 0x82E9E898; continue 'dispatch;
	}
	// 82E9E894: 4B421FFD  bl 0x822c0890
	ctx.lr = 0x82E9E898;
	sub_822C0890(ctx, base);
	// 82E9E898: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E89C: 9B0B0094  stb r24, 0x94(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[24].u8 ) };
	// 82E9E8A0: 48000010  b 0x82e9e8b0
	pc = 0x82E9E8B0; continue 'dispatch;
	// 82E9E8A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E9E8A8: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E9E8AC: 4BFFF2D5  bl 0x82e9db80
	ctx.lr = 0x82E9E8B0;
	sub_82E9DB80(ctx, base);
	// 82E9E8B0: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82E9E8B4: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82E9E8B8: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 82E9E8BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E8C0: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E9E8C4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9E8C8: 409A0010  bne cr6, 0x82e9e8d8
	if !ctx.cr[6].eq {
	pc = 0x82E9E8D8; continue 'dispatch;
	}
	// 82E9E8CC: 897F0094  lbz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E9E8D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9E8D4: 4182010C  beq 0x82e9e9e0
	if ctx.cr[0].eq {
	pc = 0x82E9E9E0; continue 'dispatch;
	}
	// 82E9E8D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9E8DC: 419A00F0  beq cr6, 0x82e9e9cc
	if ctx.cr[6].eq {
	pc = 0x82E9E9CC; continue 'dispatch;
	}
	// 82E9E8E0: 4BFD9C69  bl 0x82e78548
	ctx.lr = 0x82E9E8E4;
	sub_82E78548(ctx, base);
	// 82E9E8E4: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E9E8E8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E9E8EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E9E8F0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E8F4: 4BF6FBFD  bl 0x82e0e4f0
	ctx.lr = 0x82E9E8F8;
	sub_82E0E4F0(ctx, base);
	// 82E9E8F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9E8FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9E900: 4BFFFDB9  bl 0x82e9e6b8
	ctx.lr = 0x82E9E904;
	sub_82E9E6B8(ctx, base);
	// 82E9E904: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E908: 40820014  bne 0x82e9e91c
	if !ctx.cr[0].eq {
	pc = 0x82E9E91C; continue 'dispatch;
	}
	// 82E9E90C: 89760000  lbz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E910: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82E9E914: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E918: 41820008  beq 0x82e9e920
	if ctx.cr[0].eq {
	pc = 0x82E9E920; continue 'dispatch;
	}
	// 82E9E91C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82E9E920: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E924: 4BFFC665  bl 0x82e9af88
	ctx.lr = 0x82E9E928;
	sub_82E9AF88(ctx, base);
	// 82E9E928: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E92C: 4182000C  beq 0x82e9e938
	if ctx.cr[0].eq {
	pc = 0x82E9E938; continue 'dispatch;
	}
	// 82E9E930: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82E9E934: 4800008C  b 0x82e9e9c0
	pc = 0x82E9E9C0; continue 'dispatch;
	// 82E9E938: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E93C: 40820084  bne 0x82e9e9c0
	if !ctx.cr[0].eq {
	pc = 0x82E9E9C0; continue 'dispatch;
	}
	// 82E9E940: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E944: 89640094  lbz r11, 0x94(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E9E948: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9E94C: 41820074  beq 0x82e9e9c0
	if ctx.cr[0].eq {
	pc = 0x82E9E9C0; continue 'dispatch;
	}
	// 82E9E950: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E954: 556B073F  clrlwi. r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E958: 40820068  bne 0x82e9e9c0
	if !ctx.cr[0].eq {
	pc = 0x82E9E9C0; continue 'dispatch;
	}
	// 82E9E95C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E9E960: 4BFFC5F9  bl 0x82e9af58
	ctx.lr = 0x82E9E964;
	sub_82E9AF58(ctx, base);
	// 82E9E964: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E9E968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9E96C: 419A0044  beq cr6, 0x82e9e9b0
	if ctx.cr[6].eq {
	pc = 0x82E9E9B0; continue 'dispatch;
	}
	// 82E9E970: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E9E974: 56EB063F  clrlwi. r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9E978: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E9E97C: 41820014  beq 0x82e9e990
	if ctx.cr[0].eq {
	pc = 0x82E9E990; continue 'dispatch;
	}
	// 82E9E980: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E9E984: 4BFF8C8D  bl 0x82e97610
	ctx.lr = 0x82E9E988;
	sub_82E97610(ctx, base);
	// 82E9E988: 7F17C378  mr r23, r24
	ctx.r[23].u64 = ctx.r[24].u64;
	// 82E9E98C: 48000024  b 0x82e9e9b0
	pc = 0x82E9E9B0; continue 'dispatch;
	// 82E9E990: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82E9E994: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E9E998: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 82E9E99C: 4BFF8C75  bl 0x82e97610
	ctx.lr = 0x82E9E9A0;
	sub_82E97610(ctx, base);
	// 82E9E9A0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E9E9A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9E9A8: 419A0008  beq cr6, 0x82e9e9b0
	if ctx.cr[6].eq {
	pc = 0x82E9E9B0; continue 'dispatch;
	}
	// 82E9E9AC: 4B421EE5  bl 0x822c0890
	ctx.lr = 0x82E9E9B0;
	sub_822C0890(ctx, base);
	// 82E9E9B0: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E9E9B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9E9B8: 419A0008  beq cr6, 0x82e9e9c0
	if ctx.cr[6].eq {
	pc = 0x82E9E9C0; continue 'dispatch;
	}
	// 82E9E9BC: 4B421ED5  bl 0x822c0890
	ctx.lr = 0x82E9E9C0;
	sub_822C0890(ctx, base);
	// 82E9E9C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E9C4: 9B0B0094  stb r24, 0x94(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[24].u8 ) };
	// 82E9E9C8: 48000018  b 0x82e9e9e0
	pc = 0x82E9E9E0; continue 'dispatch;
	// 82E9E9CC: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E9E9D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9E9D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E9E9D8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9E9DC: 4BF6FB15  bl 0x82e0e4f0
	ctx.lr = 0x82E9E9E0;
	sub_82E0E4F0(ctx, base);
	// 82E9E9E0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E9E9E4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E9E9E8: 2B1C0004  cmplwi cr6, r28, 4
	ctx.cr[6].compare_u32(ctx.r[28].u32, 4 as u32, &mut ctx.xer);
	// 82E9E9EC: 409AFED0  bne cr6, 0x82e9e8bc
	if !ctx.cr[6].eq {
	pc = 0x82E9E8BC; continue 'dispatch;
	}
	// 82E9E9F0: 897F00A4  lbz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E9E9F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9E9F8: 40820034  bne 0x82e9ea2c
	if !ctx.cr[0].eq {
	pc = 0x82E9EA2C; continue 'dispatch;
	}
	// 82E9E9FC: 56EB063F  clrlwi. r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9EA00: 4182002C  beq 0x82e9ea2c
	if ctx.cr[0].eq {
	pc = 0x82E9EA2C; continue 'dispatch;
	}
	// 82E9EA04: 93010058  stw r24, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[24].u32 ) };
	// 82E9EA08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E9EA0C: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 82E9EA10: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E9EA14: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E9EA18: 4BFF8BF9  bl 0x82e97610
	ctx.lr = 0x82E9EA1C;
	sub_82E97610(ctx, base);
	// 82E9EA1C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E9EA20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9EA24: 419A0008  beq cr6, 0x82e9ea2c
	if ctx.cr[6].eq {
	pc = 0x82E9EA2C; continue 'dispatch;
	}
	// 82E9EA28: 4B421E69  bl 0x822c0890
	ctx.lr = 0x82E9EA2C;
	sub_822C0890(ctx, base);
	// 82E9EA2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9EA30: 572A063F  clrlwi. r10, r25, 0x18
	ctx.r[10].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9EA34: C3CB08A8  lfs f30, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E9EA38: 418200A4  beq 0x82e9eadc
	if ctx.cr[0].eq {
	pc = 0x82E9EADC; continue 'dispatch;
	}
	// 82E9EA3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9EA40: 997F00A4  stb r11, 0xa4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u8 ) };
	// 82E9EA44: 80750000  lwz r3, 0(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9EA48: 4BFFC449  bl 0x82e9ae90
	ctx.lr = 0x82E9EA4C;
	sub_82E9AE90(ctx, base);
	// 82E9EA4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9EA50: 807F0088  lwz r3, 0x88(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E9EA54: 480002BD  bl 0x82e9ed10
	ctx.lr = 0x82E9EA58;
	sub_82E9ED10(ctx, base);
	// 82E9EA58: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E9EA5C: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9EA60: C3E10088  lfs f31, 0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E9EA64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EBB0 size=40
    let mut pc: u32 = 0x82E9EBB0;
    'dispatch: loop {
        match pc {
            0x82E9EBB0 => {
    //   block [0x82E9EBB0..0x82E9EBD8)
	// 82E9EBB0: 3D63E5DE  addis r11, r3, -0x1a22
	ctx.r[11].s64 = ctx.r[3].s64 + -438435840;
	// 82E9EBB4: 356BFEA8  addic. r11, r11, -0x158
	ctx.xer.ca = (ctx.r[11].u32 > (!(-344 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -344;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9EBB8: 41820020  beq 0x82e9ebd8
	if ctx.cr[0].eq {
		sub_82E9EBD8(ctx, base);
		return;
	}
	// 82E9EBBC: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 82E9EBC0: 419A0018  beq cr6, 0x82e9ebd8
	if ctx.cr[6].eq {
		sub_82E9EBD8(ctx, base);
		return;
	}
	// 82E9EBC4: 3D4012FE  lis r10, 0x12fe
	ctx.r[10].s64 = 318636032;
	// 82E9EBC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9EBCC: 614A003E  ori r10, r10, 0x3e
	ctx.r[10].u64 = ctx.r[10].u64 | 62;
	// 82E9EBD0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9EBD4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EBD8 size=8
    let mut pc: u32 = 0x82E9EBD8;
    'dispatch: loop {
        match pc {
            0x82E9EBD8 => {
    //   block [0x82E9EBD8..0x82E9EBE0)
	// 82E9EBD8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9EBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EBE0 size=76
    let mut pc: u32 = 0x82E9EBE0;
    'dispatch: loop {
        match pc {
            0x82E9EBE0 => {
    //   block [0x82E9EBE0..0x82E9EC2C)
	// 82E9EBE0: 3D63E5E0  addis r11, r3, -0x1a20
	ctx.r[11].s64 = ctx.r[3].s64 + -438304768;
	// 82E9EBE4: 356BFEA6  addic. r11, r11, -0x15a
	ctx.xer.ca = (ctx.r[11].u32 > (!(-346 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -346;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9EBE8: 41820050  beq 0x82e9ec38
	if ctx.cr[0].eq {
		sub_82E9EC38(ctx, base);
		return;
	}
	// 82E9EBEC: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82E9EBF0: 419A0048  beq cr6, 0x82e9ec38
	if ctx.cr[6].eq {
		sub_82E9EC38(ctx, base);
		return;
	}
	// 82E9EBF4: 3D6BFFFD  addis r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -196608;
	// 82E9EBF8: 356B55FD  addic. r11, r11, 0x55fd
	ctx.xer.ca = (ctx.r[11].u32 > (!(22013 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 22013;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9EBFC: 4182003C  beq 0x82e9ec38
	if ctx.cr[0].eq {
		sub_82E9EC38(ctx, base);
		return;
	}
	// 82E9EC00: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E9EC04: 419A0034  beq cr6, 0x82e9ec38
	if ctx.cr[6].eq {
		sub_82E9EC38(ctx, base);
		return;
	}
	// 82E9EC08: 3D4012FD  lis r10, 0x12fd
	ctx.r[10].s64 = 318570496;
	// 82E9EC0C: 614A563C  ori r10, r10, 0x563c
	ctx.r[10].u64 = ctx.r[10].u64 | 22076;
	// 82E9EC10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9EC14: 419A0018  beq cr6, 0x82e9ec2c
	if ctx.cr[6].eq {
		sub_82E9EC2C(ctx, base);
		return;
	}
	// 82E9EC18: 3D6BED00  addis r11, r11, -0x1300
	ctx.r[11].s64 = ctx.r[11].s64 + -318767104;
	// 82E9EC1C: 356BFFC1  addic. r11, r11, -0x3f
	ctx.xer.ca = (ctx.r[11].u32 > (!(-63 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -63;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9EC20: 4182000C  beq 0x82e9ec2c
	if ctx.cr[0].eq {
		sub_82E9EC2C(ctx, base);
		return;
	}
	// 82E9EC24: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E9EC28: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EC2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EC2C size=12
    let mut pc: u32 = 0x82E9EC2C;
    'dispatch: loop {
        match pc {
            0x82E9EC2C => {
    //   block [0x82E9EC2C..0x82E9EC38)
	// 82E9EC2C: 3C602D20  lis r3, 0x2d20
	ctx.r[3].s64 = 757071872;
	// 82E9EC30: 6063AB8D  ori r3, r3, 0xab8d
	ctx.r[3].u64 = ctx.r[3].u64 | 43917;
	// 82E9EC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EC38 size=12
    let mut pc: u32 = 0x82E9EC38;
    'dispatch: loop {
        match pc {
            0x82E9EC38 => {
    //   block [0x82E9EC38..0x82E9EC44)
	// 82E9EC38: 3C601A22  lis r3, 0x1a22
	ctx.r[3].s64 = 438435840;
	// 82E9EC3C: 606301BF  ori r3, r3, 0x1bf
	ctx.r[3].u64 = ctx.r[3].u64 | 447;
	// 82E9EC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EC48 size=60
    let mut pc: u32 = 0x82E9EC48;
    'dispatch: loop {
        match pc {
            0x82E9EC48 => {
    //   block [0x82E9EC48..0x82E9EC84)
	// 82E9EC48: 3D601A20  lis r11, 0x1a20
	ctx.r[11].s64 = 438304768;
	// 82E9EC4C: 616BAB55  ori r11, r11, 0xab55
	ctx.r[11].u64 = ctx.r[11].u64 | 43861;
	// 82E9EC50: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9EC54: 419A0040  beq cr6, 0x82e9ec94
	if ctx.cr[6].eq {
		sub_82E9EC94(ctx, base);
		return;
	}
	// 82E9EC58: 3D63E5DE  addis r11, r3, -0x1a22
	ctx.r[11].s64 = ctx.r[3].s64 + -438435840;
	// 82E9EC5C: 356BFEA8  addic. r11, r11, -0x158
	ctx.xer.ca = (ctx.r[11].u32 > (!(-344 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -344;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9EC60: 4182002C  beq 0x82e9ec8c
	if ctx.cr[0].eq {
		sub_82E9EC8C(ctx, base);
		return;
	}
	// 82E9EC64: 2B0BAA08  cmplwi cr6, r11, 0xaa08
	ctx.cr[6].compare_u32(ctx.r[11].u32, 43528 as u32, &mut ctx.xer);
	// 82E9EC68: 419A002C  beq cr6, 0x82e9ec94
	if ctx.cr[6].eq {
		sub_82E9EC94(ctx, base);
		return;
	}
	// 82E9EC6C: 3D400DDD  lis r10, 0xddd
	ctx.r[10].s64 = 232587264;
	// 82E9EC70: 614AFFAA  ori r10, r10, 0xffaa
	ctx.r[10].u64 = ctx.r[10].u64 | 65450;
	// 82E9EC74: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E9EC78: 419A000C  beq cr6, 0x82e9ec84
	if ctx.cr[6].eq {
		sub_82E9EC84(ctx, base);
		return;
	}
	// 82E9EC7C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E9EC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EC84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EC84 size=8
    let mut pc: u32 = 0x82E9EC84;
    'dispatch: loop {
        match pc {
            0x82E9EC84 => {
    //   block [0x82E9EC84..0x82E9EC8C)
	// 82E9EC84: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82E9EC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EC8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EC8C size=8
    let mut pc: u32 = 0x82E9EC8C;
    'dispatch: loop {
        match pc {
            0x82E9EC8C => {
    //   block [0x82E9EC8C..0x82E9EC94)
	// 82E9EC8C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E9EC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EC94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EC94 size=8
    let mut pc: u32 = 0x82E9EC94;
    'dispatch: loop {
        match pc {
            0x82E9EC94 => {
    //   block [0x82E9EC94..0x82E9EC9C)
	// 82E9EC94: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82E9EC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9ECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9ECA0 size=36
    let mut pc: u32 = 0x82E9ECA0;
    'dispatch: loop {
        match pc {
            0x82E9ECA0 => {
    //   block [0x82E9ECA0..0x82E9ECC4)
	// 82E9ECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9ECA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9ECA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9ECAC: 4BFFFF9D  bl 0x82e9ec48
	ctx.lr = 0x82E9ECB0;
	sub_82E9EC48(ctx, base);
	// 82E9ECB0: 5463E8FE  srwi r3, r3, 3
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E9ECB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9ECB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9ECBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9ECC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9ECC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9ECC8 size=24
    let mut pc: u32 = 0x82E9ECC8;
    'dispatch: loop {
        match pc {
            0x82E9ECC8 => {
    //   block [0x82E9ECC8..0x82E9ECE0)
	// 82E9ECC8: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82E9ECCC: 41980014  blt cr6, 0x82e9ece0
	if ctx.cr[6].lt {
		sub_82E9ECE0(ctx, base);
		return;
	}
	// 82E9ECD0: 419A0010  beq cr6, 0x82e9ece0
	if ctx.cr[6].eq {
		sub_82E9ECE0(ctx, base);
		return;
	}
	// 82E9ECD4: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82E9ECD8: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82E9ECDC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9ECE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9ECE0 size=8
    let mut pc: u32 = 0x82E9ECE0;
    'dispatch: loop {
        match pc {
            0x82E9ECE0 => {
    //   block [0x82E9ECE0..0x82E9ECE8)
	// 82E9ECE0: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82E9ECE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9ECE8 size=28
    let mut pc: u32 = 0x82E9ECE8;
    'dispatch: loop {
        match pc {
            0x82E9ECE8 => {
    //   block [0x82E9ECE8..0x82E9ED04)
	// 82E9ECE8: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82E9ECEC: 41980018  blt cr6, 0x82e9ed04
	if ctx.cr[6].lt {
		sub_82E9ED04(ctx, base);
		return;
	}
	// 82E9ECF0: 419A000C  beq cr6, 0x82e9ecfc
	if ctx.cr[6].eq {
	pc = 0x82E9ECFC; continue 'dispatch;
	}
	// 82E9ECF4: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82E9ECF8: 4098000C  bge cr6, 0x82e9ed04
	if !ctx.cr[6].lt {
		sub_82E9ED04(ctx, base);
		return;
	}
	// 82E9ECFC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82E9ED00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9ED04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9ED04 size=8
    let mut pc: u32 = 0x82E9ED04;
    'dispatch: loop {
        match pc {
            0x82E9ED04 => {
    //   block [0x82E9ED04..0x82E9ED0C)
	// 82E9ED04: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E9ED08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9ED10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E9ED10 size=132
    let mut pc: u32 = 0x82E9ED10;
    'dispatch: loop {
        match pc {
            0x82E9ED10 => {
    //   block [0x82E9ED10..0x82E9ED94)
	// 82E9ED10: 546B463E  srwi r11, r3, 0x18
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shr(24);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9ED14: 546A863E  rlwinm r10, r3, 0x10, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82E9ED18: 5469C63E  rlwinm r9, r3, 0x18, 0x18, 0x1f
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E9ED1C: F961FFE0  std r11, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[11].u64 ) };
	// 82E9ED20: C9A1FFE0  lfd f13, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E9ED24: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E9ED28: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82E9ED2C: C981FFF0  lfd f12, -0x10(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9ED30: F921FFE0  std r9, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[9].u64 ) };
	// 82E9ED34: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82E9ED38: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82E9ED3C: C961FFF0  lfd f11, -0x10(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9ED40: C801FFE0  lfd f0, -0x20(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E9ED44: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E9ED48: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82E9ED4C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82E9ED50: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E9ED54: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82E9ED58: FD40069C  fcfid f10, f0
	ctx.f[10].f64 = (ctx.f[0].s64 as f64);
	// 82E9ED5C: C00A94AC  lfs f0, -0x6b54(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27476 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9ED60: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82E9ED64: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82E9ED68: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82E9ED6C: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E9ED70: D181FFF0  stfs f12, -0x10(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82E9ED74: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E9ED78: D161FFF8  stfs f11, -8(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82E9ED7C: ED6A0032  fmuls f11, f10, f0
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E9ED80: D161FFF4  stfs f11, -0xc(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82E9ED84: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E9ED88: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9ED98 size=28
    let mut pc: u32 = 0x82E9ED98;
    'dispatch: loop {
        match pc {
            0x82E9ED98 => {
    //   block [0x82E9ED98..0x82E9EDB4)
	// 82E9ED98: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E9ED9C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E9EDA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9EDA4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9EDA8: 409A000C  bne cr6, 0x82e9edb4
	if !ctx.cr[6].eq {
		sub_82E9EDB4(ctx, base);
		return;
	}
	// 82E9EDAC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E9EDB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EDB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EDB4 size=40
    let mut pc: u32 = 0x82E9EDB4;
    'dispatch: loop {
        match pc {
            0x82E9EDB4 => {
    //   block [0x82E9EDB4..0x82E9EDDC)
	// 82E9EDB4: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E9EDB8: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9EDBC: 40980008  bge cr6, 0x82e9edc4
	if !ctx.cr[6].lt {
	pc = 0x82E9EDC4; continue 'dispatch;
	}
	// 82E9EDC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E9EDC4: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9EDC8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E9EDCC: 40980008  bge cr6, 0x82e9edd4
	if !ctx.cr[6].lt {
	pc = 0x82E9EDD4; continue 'dispatch;
	}
	// 82E9EDD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9EDD4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82E9EDD8: 4BFFFFCC  b 0x82e9eda4
	sub_82E9ED98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EDDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EDDC size=4
    let mut pc: u32 = 0x82E9EDDC;
    'dispatch: loop {
        match pc {
            0x82E9EDDC => {
    //   block [0x82E9EDDC..0x82E9EDE0)
	// 82E9EDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9EDE0 size=16
    let mut pc: u32 = 0x82E9EDE0;
    'dispatch: loop {
        match pc {
            0x82E9EDE0 => {
    //   block [0x82E9EDE0..0x82E9EDF0)
	// 82E9EDE0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9EDE4: 396BE65C  addi r11, r11, -0x19a4
	ctx.r[11].s64 = ctx.r[11].s64 + -6564;
	// 82E9EDE8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9EDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9EDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9EDF0 size=544
    let mut pc: u32 = 0x82E9EDF0;
    'dispatch: loop {
        match pc {
            0x82E9EDF0 => {
    //   block [0x82E9EDF0..0x82E9F010)
	// 82E9EDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9EDF4: 48309361  bl 0x831a8154
	ctx.lr = 0x82E9EDF8;
	sub_831A8130(ctx, base);
	// 82E9EDF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9EDFC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E9EE00: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82E9EE04: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E9EE08: 419A0034  beq cr6, 0x82e9ee3c
	if ctx.cr[6].eq {
	pc = 0x82E9EE3C; continue 'dispatch;
	}
	// 82E9EE0C: 39640008  addi r11, r4, 8
	ctx.r[11].s64 = ctx.r[4].s64 + 8;
	// 82E9EE10: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82E9EE14: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E9EE18: 930BFFFC  stw r24, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[24].u32 ) };
	// 82E9EE1C: B30B0000  sth r24, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 82E9EE20: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9EE24: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82E9EE28: B30B0002  sth r24, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[24].u16 ) };
	// 82E9EE2C: B30B0004  sth r24, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[24].u16 ) };
	// 82E9EE30: B30B0006  sth r24, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[24].u16 ) };
	// 82E9EE34: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82E9EE38: 4082FFDC  bne 0x82e9ee14
	if !ctx.cr[0].eq {
	pc = 0x82E9EE14; continue 'dispatch;
	}
	// 82E9EE3C: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82E9EE40: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E9EE44: 419A01C0  beq cr6, 0x82e9f004
	if ctx.cr[6].eq {
	pc = 0x82E9F004; continue 'dispatch;
	}
	// 82E9EE48: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9EE4C: 3BE40004  addi r31, r4, 4
	ctx.r[31].s64 = ctx.r[4].s64 + 4;
	// 82E9EE50: 7EE41850  subf r23, r4, r3
	ctx.r[23].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82E9EE54: 3B2BE5E8  addi r25, r11, -0x1a18
	ctx.r[25].s64 = ctx.r[11].s64 + -6680;
	// 82E9EE58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9EE5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E9EE60: 48326971  bl 0x831c57d0
	ctx.lr = 0x82E9EE64;
	sub_831C57D0(ctx, base);
	// 82E9EE64: 815FFFFC  lwz r10, -4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E9EE68: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E9EE6C: 3B7FFFFC  addi r27, r31, -4
	ctx.r[27].s64 = ctx.r[31].s64 + -4;
	// 82E9EE70: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E9EE74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9EE78: 419A0014  beq cr6, 0x82e9ee8c
	if ctx.cr[6].eq {
	pc = 0x82E9EE8C; continue 'dispatch;
	}
	// 82E9EE7C: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E9EE80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E9EE84: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E9EE88: 419A0008  beq cr6, 0x82e9ee90
	if ctx.cr[6].eq {
	pc = 0x82E9EE90; continue 'dispatch;
	}
	// 82E9EE8C: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82E9EE90: 557D063F  clrlwi. r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E9EE94: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E9EE98: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82E9EE9C: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82E9EEA0: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82E9EEA4: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82E9EEA8: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82E9EEAC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82E9EEB0: 41820114  beq 0x82e9efc4
	if ctx.cr[0].eq {
	pc = 0x82E9EFC4; continue 'dispatch;
	}
	// 82E9EEB4: 550B063F  clrlwi. r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9EEB8: 41820014  beq 0x82e9eecc
	if ctx.cr[0].eq {
	pc = 0x82E9EECC; continue 'dispatch;
	}
	// 82E9EEBC: 7CB7FA14  add r5, r23, r31
	ctx.r[5].u64 = ctx.r[23].u64 + ctx.r[31].u64;
	// 82E9EEC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9EEC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E9EEC8: 48326901  bl 0x831c57c8
	ctx.lr = 0x82E9EECC;
	sub_831C57C8(ctx, base);
	// 82E9EECC: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82E9EED0: 39790004  addi r11, r25, 4
	ctx.r[11].s64 = ctx.r[25].s64 + 4;
	// 82E9EED4: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82E9EED8: 3940000E  li r10, 0xe
	ctx.r[10].s64 = 14;
	// 82E9EEDC: A1210054  lhz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E9EEE0: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82E9EEE4: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E9EEE8: 808BFFFC  lwz r4, -4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E9EEEC: 7C844839  and. r4, r4, r9
	ctx.r[4].u64 = ctx.r[4].u64 & ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E9EEF0: 4182000C  beq 0x82e9eefc
	if ctx.cr[0].eq {
	pc = 0x82E9EEFC; continue 'dispatch;
	}
	// 82E9EEF4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9EEF8: 7C9EF378  or r30, r4, r30
	ctx.r[30].u64 = ctx.r[4].u64 | ctx.r[30].u64;
	// 82E9EEFC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9EF00: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E9EF04: 4082FFE4  bne 0x82e9eee8
	if !ctx.cr[0].eq {
	pc = 0x82E9EEE8; continue 'dispatch;
	}
	// 82E9EF08: 550B063E  clrlwi r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82E9EF0C: 2B0B001E  cmplwi cr6, r11, 0x1e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 30 as u32, &mut ctx.xer);
	// 82E9EF10: 40990008  ble cr6, 0x82e9ef18
	if !ctx.cr[6].gt {
	pc = 0x82E9EF18; continue 'dispatch;
	}
	// 82E9EF14: 63DE4000  ori r30, r30, 0x4000
	ctx.r[30].u64 = ctx.r[30].u64 | 16384;
	// 82E9EF18: 54EB063E  clrlwi r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82E9EF1C: 2B0B001E  cmplwi cr6, r11, 0x1e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 30 as u32, &mut ctx.xer);
	// 82E9EF20: 40990008  ble cr6, 0x82e9ef28
	if !ctx.cr[6].gt {
	pc = 0x82E9EF28; continue 'dispatch;
	}
	// 82E9EF24: 63DE8000  ori r30, r30, 0x8000
	ctx.r[30].u64 = ctx.r[30].u64 | 32768;
	// 82E9EF28: A9610058  lha r11, 0x58(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i16) as i64;
	// 82E9EF2C: 2F0B8001  cmpwi cr6, r11, -0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32767, &mut ctx.xer);
	// 82E9EF30: 40980008  bge cr6, 0x82e9ef38
	if !ctx.cr[6].lt {
	pc = 0x82E9EF38; continue 'dispatch;
	}
	// 82E9EF34: 39608001  li r11, -0x7fff
	ctx.r[11].s64 = -32767;
	// 82E9EF38: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E9EF3C: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82E9EF40: 40990008  ble cr6, 0x82e9ef48
	if !ctx.cr[6].gt {
	pc = 0x82E9EF48; continue 'dispatch;
	}
	// 82E9EF44: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82E9EF48: A941005A  lha r10, 0x5a(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as i16) as i64;
	// 82E9EF4C: 7D670734  extsh r7, r11
	ctx.r[7].s64 = ctx.r[11].s16 as i64;
	// 82E9EF50: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E9EF54: 2F0B8001  cmpwi cr6, r11, -0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32767, &mut ctx.xer);
	// 82E9EF58: 40980008  bge cr6, 0x82e9ef60
	if !ctx.cr[6].lt {
	pc = 0x82E9EF60; continue 'dispatch;
	}
	// 82E9EF5C: 39608001  li r11, -0x7fff
	ctx.r[11].s64 = -32767;
	// 82E9EF60: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E9EF64: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82E9EF68: 40990008  ble cr6, 0x82e9ef70
	if !ctx.cr[6].gt {
	pc = 0x82E9EF70; continue 'dispatch;
	}
	// 82E9EF6C: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82E9EF70: A941005C  lha r10, 0x5c(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as i16) as i64;
	// 82E9EF74: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 82E9EF78: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E9EF7C: 2F0B8001  cmpwi cr6, r11, -0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32767, &mut ctx.xer);
	// 82E9EF80: 40980008  bge cr6, 0x82e9ef88
	if !ctx.cr[6].lt {
	pc = 0x82E9EF88; continue 'dispatch;
	}
	// 82E9EF84: 39608001  li r11, -0x7fff
	ctx.r[11].s64 = -32767;
	// 82E9EF88: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E9EF8C: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82E9EF90: 40990008  ble cr6, 0x82e9ef98
	if !ctx.cr[6].gt {
	pc = 0x82E9EF98; continue 'dispatch;
	}
	// 82E9EF94: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82E9EF98: A901005E  lha r8, 0x5e(r1)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as i16) as i64;
	// 82E9EF9C: 7D6A0734  extsh r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	// 82E9EFA0: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82E9EFA4: 2F0B8001  cmpwi cr6, r11, -0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32767, &mut ctx.xer);
	// 82E9EFA8: 40980008  bge cr6, 0x82e9efb0
	if !ctx.cr[6].lt {
	pc = 0x82E9EFB0; continue 'dispatch;
	}
	// 82E9EFAC: 39608001  li r11, -0x7fff
	ctx.r[11].s64 = -32767;
	// 82E9EFB0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E9EFB4: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82E9EFB8: 40990008  ble cr6, 0x82e9efc0
	if !ctx.cr[6].gt {
	pc = 0x82E9EFC0; continue 'dispatch;
	}
	// 82E9EFBC: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82E9EFC0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E9EFC4: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 82E9EFC8: B15F0008  sth r10, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 82E9EFCC: 7FA80034  cntlzw r8, r29
	ctx.r[8].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82E9EFD0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E9EFD4: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	// 82E9EFD8: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82E9EFDC: 550ADFFE  rlwinm r10, r8, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82E9EFE0: B17F000A  sth r11, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82E9EFE4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E9EFE8: B0DF000C  sth r6, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u16 ) };
	// 82E9EFEC: B0BF000E  sth r5, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[5].u16 ) };
	// 82E9EFF0: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82E9EFF4: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 82E9EFF8: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E9EFFC: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E9F000: 4198FE58  blt cr6, 0x82e9ee58
	if ctx.cr[6].lt {
	pc = 0x82E9EE58; continue 'dispatch;
	}
	// 82E9F004: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9F008: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E9F00C: 48309198  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F010 size=40
    let mut pc: u32 = 0x82E9F010;
    'dispatch: loop {
        match pc {
            0x82E9F010 => {
    //   block [0x82E9F010..0x82E9F038)
	// 82E9F010: 394021F1  li r10, 0x21f1
	ctx.r[10].s64 = 8689;
	// 82E9F014: 39605E0E  li r11, 0x5e0e
	ctx.r[11].s64 = 24078;
	// 82E9F018: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 82E9F01C: B1450000  sth r10, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82E9F020: B1450002  sth r10, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 82E9F024: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9F028: B1650004  sth r11, 4(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E9F02C: B1650006  sth r11, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E9F030: B1250008  sth r9, 8(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u16 ) };
	// 82E9F034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F038 size=12
    let mut pc: u32 = 0x82E9F038;
    'dispatch: loop {
        match pc {
            0x82E9F038 => {
    //   block [0x82E9F038..0x82E9F044)
	// 82E9F038: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E9F03C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E9F040: 483267A0  b 0x831c57e0
	sub_831C57E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9F048 size=60
    let mut pc: u32 = 0x82E9F048;
    'dispatch: loop {
        match pc {
            0x82E9F048 => {
    //   block [0x82E9F048..0x82E9F084)
	// 82E9F048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9F050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F054: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E9F058: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E9F060: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E9F064: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E9F068: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9F06C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F070: 4E800421  bctrl
	ctx.lr = 0x82E9F074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9F074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9F078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9F07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9F080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F088 size=16
    let mut pc: u32 = 0x82E9F088;
    'dispatch: loop {
        match pc {
            0x82E9F088 => {
    //   block [0x82E9F088..0x82E9F098)
	// 82E9F088: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9F08C: 396BE67C  addi r11, r11, -0x1984
	ctx.r[11].s64 = ctx.r[11].s64 + -6532;
	// 82E9F090: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9F094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9F098 size=68
    let mut pc: u32 = 0x82E9F098;
    'dispatch: loop {
        match pc {
            0x82E9F098 => {
    //   block [0x82E9F098..0x82E9F0DC)
	// 82E9F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9F0A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9F0A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F0A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9F0AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9F0B0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F0B4: 396BE65C  addi r11, r11, -0x19a4
	ctx.r[11].s64 = ctx.r[11].s64 + -6564;
	// 82E9F0B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9F0BC: 41820008  beq 0x82e9f0c4
	if ctx.cr[0].eq {
	pc = 0x82E9F0C4; continue 'dispatch;
	}
	// 82E9F0C0: 4B4211A9  bl 0x822c0268
	ctx.lr = 0x82E9F0C4;
	sub_822C0268(ctx, base);
	// 82E9F0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9F0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9F0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9F0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9F0D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9F0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F0E0 size=84
    let mut pc: u32 = 0x82E9F0E0;
    'dispatch: loop {
        match pc {
            0x82E9F0E0 => {
    //   block [0x82E9F0E0..0x82E9F134)
	// 82E9F0E0: A1250000  lhz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F0E4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F0E8: 41820038  beq 0x82e9f120
	if ctx.cr[0].eq {
	pc = 0x82E9F120; continue 'dispatch;
	}
	// 82E9F0EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E9F0F0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82E9F0F4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E9F0F8: 419A001C  beq cr6, 0x82e9f114
	if ctx.cr[6].eq {
	pc = 0x82E9F114; continue 'dispatch;
	}
	// 82E9F0FC: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F100: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E9F104: 419A0030  beq cr6, 0x82e9f134
	if ctx.cr[6].eq {
		sub_82E9F134(ctx, base);
		return;
	}
	// 82E9F108: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F10C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82E9F110: 4082FFEC  bne 0x82e9f0fc
	if !ctx.cr[0].eq {
	pc = 0x82E9F0FC; continue 'dispatch;
	}
	// 82E9F114: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F118: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F11C: 41820020  beq 0x82e9f13c
	if ctx.cr[0].eq {
		sub_82E9F13C(ctx, base);
		return;
	}
	// 82E9F120: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82E9F124: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 82E9F128: 4082FFB8  bne 0x82e9f0e0
	if !ctx.cr[0].eq {
	pc = 0x82E9F0E0; continue 'dispatch;
	}
	// 82E9F12C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9F130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F134 size=8
    let mut pc: u32 = 0x82E9F134;
    'dispatch: loop {
        match pc {
            0x82E9F134 => {
    //   block [0x82E9F134..0x82E9F13C)
	// 82E9F134: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9F138: 4BFFFFE0  b 0x82e9f118
	sub_82E9F0E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F13C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F13C size=8
    let mut pc: u32 = 0x82E9F13C;
    'dispatch: loop {
        match pc {
            0x82E9F13C => {
    //   block [0x82E9F13C..0x82E9F144)
	// 82E9F13C: A0650000  lhz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F148 size=92
    let mut pc: u32 = 0x82E9F148;
    'dispatch: loop {
        match pc {
            0x82E9F148 => {
    //   block [0x82E9F148..0x82E9F1A4)
	// 82E9F148: 54AA043E  clrlwi r10, r5, 0x10
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 82E9F14C: 2B0A008C  cmplwi cr6, r10, 0x8c
	ctx.cr[6].compare_u32(ctx.r[10].u32, 140 as u32, &mut ctx.xer);
	// 82E9F150: 40980054  bge cr6, 0x82e9f1a4
	if !ctx.cr[6].lt {
		sub_82E9F1A4(ctx, base);
		return;
	}
	// 82E9F154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F158: 2B0A0053  cmplwi cr6, r10, 0x53
	ctx.cr[6].compare_u32(ctx.r[10].u32, 83 as u32, &mut ctx.xer);
	// 82E9F15C: 548907BD  rlwinm. r9, r4, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F160: 41980008  blt cr6, 0x82e9f168
	if ctx.cr[6].lt {
	pc = 0x82E9F168; continue 'dispatch;
	}
	// 82E9F164: 548907FF  clrlwi. r9, r4, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F168: 41820008  beq 0x82e9f170
	if ctx.cr[0].eq {
	pc = 0x82E9F170; continue 'dispatch;
	}
	// 82E9F16C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E9F170: 70690022  andi. r9, r3, 0x22
	ctx.r[9].u64 = ctx.r[3].u64 & 34;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F174: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F178: 41820010  beq 0x82e9f188
	if ctx.cr[0].eq {
	pc = 0x82E9F188; continue 'dispatch;
	}
	// 82E9F17C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E9F180: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E9F184: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E9F188: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F18C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9F190: 396BE698  addi r11, r11, -0x1968
	ctx.r[11].s64 = ctx.r[11].s64 + -6504;
	// 82E9F194: 41820008  beq 0x82e9f19c
	if ctx.cr[0].eq {
	pc = 0x82E9F19C; continue 'dispatch;
	}
	// 82E9F198: 396B0090  addi r11, r11, 0x90
	ctx.r[11].s64 = ctx.r[11].s64 + 144;
	// 82E9F19C: 7C6A58AE  lbzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E9F1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F1A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F1A4 size=8
    let mut pc: u32 = 0x82E9F1A4;
    'dispatch: loop {
        match pc {
            0x82E9F1A4 => {
    //   block [0x82E9F1A4..0x82E9F1AC)
	// 82E9F1A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9F1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F1B0 size=20
    let mut pc: u32 = 0x82E9F1B0;
    'dispatch: loop {
        match pc {
            0x82E9F1B0 => {
    //   block [0x82E9F1B0..0x82E9F1C4)
	// 82E9F1B0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9F1B4: 7D6A2079  andc. r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 & !ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F1B8: 4182000C  beq 0x82e9f1c4
	if ctx.cr[0].eq {
		sub_82E9F1C4(ctx, base);
		return;
	}
	// 82E9F1BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9F1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F1C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F1C4 size=116
    let mut pc: u32 = 0x82E9F1C4;
    'dispatch: loop {
        match pc {
            0x82E9F1C4 => {
    //   block [0x82E9F1C4..0x82E9F238)
	// 82E9F1C4: 708A0011  andi. r10, r4, 0x11
	ctx.r[10].u64 = ctx.r[4].u64 & 17;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F1C8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F1CC: 41820014  beq 0x82e9f1e0
	if ctx.cr[0].eq {
	pc = 0x82E9F1E0; continue 'dispatch;
	}
	// 82E9F1D0: 7D6A2038  and r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82E9F1D4: 714A0011  andi. r10, r10, 0x11
	ctx.r[10].u64 = ctx.r[10].u64 & 17;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F1D8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F1DC: 4182FFE0  beq 0x82e9f1bc
	if ctx.cr[0].eq {
		sub_82E9F1B0(ctx, base);
		return;
	}
	// 82E9F1E0: 708A0044  andi. r10, r4, 0x44
	ctx.r[10].u64 = ctx.r[4].u64 & 68;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F1E4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F1E8: 41820014  beq 0x82e9f1fc
	if ctx.cr[0].eq {
	pc = 0x82E9F1FC; continue 'dispatch;
	}
	// 82E9F1EC: 7D6A2038  and r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82E9F1F0: 714A0044  andi. r10, r10, 0x44
	ctx.r[10].u64 = ctx.r[10].u64 & 68;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F1F4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F1F8: 4182FFC4  beq 0x82e9f1bc
	if ctx.cr[0].eq {
		sub_82E9F1B0(ctx, base);
		return;
	}
	// 82E9F1FC: 708A0022  andi. r10, r4, 0x22
	ctx.r[10].u64 = ctx.r[4].u64 & 34;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F200: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F204: 41820014  beq 0x82e9f218
	if ctx.cr[0].eq {
	pc = 0x82E9F218; continue 'dispatch;
	}
	// 82E9F208: 7D6A2038  and r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82E9F20C: 714A0022  andi. r10, r10, 0x22
	ctx.r[10].u64 = ctx.r[10].u64 & 34;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F210: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F214: 4182FFA8  beq 0x82e9f1bc
	if ctx.cr[0].eq {
		sub_82E9F1B0(ctx, base);
		return;
	}
	// 82E9F218: 708A0088  andi. r10, r4, 0x88
	ctx.r[10].u64 = ctx.r[4].u64 & 136;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F21C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F220: 41820018  beq 0x82e9f238
	if ctx.cr[0].eq {
		sub_82E9F238(ctx, base);
		return;
	}
	// 82E9F224: 7D6B2038  and r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82E9F228: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9F22C: 716B0088  andi. r11, r11, 0x88
	ctx.r[11].u64 = ctx.r[11].u64 & 136;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F230: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F234: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F238 size=8
    let mut pc: u32 = 0x82E9F238;
    'dispatch: loop {
        match pc {
            0x82E9F238 => {
    //   block [0x82E9F238..0x82E9F240)
	// 82E9F238: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9F23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F240 size=60
    let mut pc: u32 = 0x82E9F240;
    'dispatch: loop {
        match pc {
            0x82E9F240 => {
    //   block [0x82E9F240..0x82E9F27C)
	// 82E9F240: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9F244: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9F248: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9F24C: 419A0028  beq cr6, 0x82e9f274
	if ctx.cr[6].eq {
	pc = 0x82E9F274; continue 'dispatch;
	}
	// 82E9F250: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82E9F254: 3963001C  addi r11, r3, 0x1c
	ctx.r[11].s64 = ctx.r[3].s64 + 28;
	// 82E9F258: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F25C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E9F260: 419A001C  beq cr6, 0x82e9f27c
	if ctx.cr[6].eq {
		sub_82E9F27C(ctx, base);
		return;
	}
	// 82E9F264: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E9F268: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82E9F26C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E9F270: 4198FFE8  blt cr6, 0x82e9f258
	if ctx.cr[6].lt {
	pc = 0x82E9F258; continue 'dispatch;
	}
	// 82E9F274: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E9F278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F27C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F27C size=8
    let mut pc: u32 = 0x82E9F27C;
    'dispatch: loop {
        match pc {
            0x82E9F27C => {
    //   block [0x82E9F27C..0x82E9F284)
	// 82E9F27C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9F280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F288 size=36
    let mut pc: u32 = 0x82E9F288;
    'dispatch: loop {
        match pc {
            0x82E9F288 => {
    //   block [0x82E9F288..0x82E9F2AC)
	// 82E9F288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F28C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82E9F290: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E9F294: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82E9F298: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82E9F29C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E9F2A0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E9F2A4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9F2A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F2B0 size=12
    let mut pc: u32 = 0x82E9F2B0;
    'dispatch: loop {
        match pc {
            0x82E9F2B0 => {
    //   block [0x82E9F2B0..0x82E9F2BC)
	// 82E9F2B0: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E9F2B4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82E9F2B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F2BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F2BC size=12
    let mut pc: u32 = 0x82E9F2BC;
    'dispatch: loop {
        match pc {
            0x82E9F2BC => {
    //   block [0x82E9F2BC..0x82E9F2C8)
	// 82E9F2BC: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9F2C0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E9F2C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F2C8 size=88
    let mut pc: u32 = 0x82E9F2C8;
    'dispatch: loop {
        match pc {
            0x82E9F2C8 => {
    //   block [0x82E9F2C8..0x82E9F320)
	// 82E9F2C8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9F2CC: 3943001C  addi r10, r3, 0x1c
	ctx.r[10].s64 = ctx.r[3].s64 + 28;
	// 82E9F2D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F2D4: 419A0020  beq cr6, 0x82e9f2f4
	if ctx.cr[6].eq {
	pc = 0x82E9F2F4; continue 'dispatch;
	}
	// 82E9F2D8: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9F2DC: A0CA0000  lhz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F2E0: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82E9F2E4: 419A003C  beq cr6, 0x82e9f320
	if ctx.cr[6].eq {
		sub_82E9F320(ctx, base);
		return;
	}
	// 82E9F2E8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F2EC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82E9F2F0: 4082FFEC  bne 0x82e9f2dc
	if !ctx.cr[0].eq {
	pc = 0x82E9F2DC; continue 'dispatch;
	}
	// 82E9F2F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9F2F8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9F2FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9F300: 409A0034  bne cr6, 0x82e9f334
	if !ctx.cr[6].eq {
		sub_82E9F328(ctx, base);
		return;
	}
	// 82E9F304: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F308: 4182002C  beq 0x82e9f334
	if ctx.cr[0].eq {
		sub_82E9F328(ctx, base);
		return;
	}
	// 82E9F30C: A1630002  lhz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E9F310: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F314: 41820014  beq 0x82e9f328
	if ctx.cr[0].eq {
		sub_82E9F328(ctx, base);
		return;
	}
	// 82E9F318: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82E9F31C: 48000018  b 0x82e9f334
	sub_82E9F328(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F320 size=8
    let mut pc: u32 = 0x82E9F320;
    'dispatch: loop {
        match pc {
            0x82E9F320 => {
    //   block [0x82E9F320..0x82E9F328)
	// 82E9F320: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E9F324: 4BFFFFD4  b 0x82e9f2f8
	sub_82E9F2C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F328 size=24
    let mut pc: u32 = 0x82E9F328;
    'dispatch: loop {
        match pc {
            0x82E9F328 => {
    //   block [0x82E9F328..0x82E9F340)
	// 82E9F328: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9F32C: 91030018  stw r8, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82E9F330: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82E9F334: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9F338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9F33C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F340 size=16
    let mut pc: u32 = 0x82E9F340;
    'dispatch: loop {
        match pc {
            0x82E9F340 => {
    //   block [0x82E9F340..0x82E9F350)
	// 82E9F340: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F344: 4082000C  bne 0x82e9f350
	if !ctx.cr[0].eq {
		sub_82E9F350(ctx, base);
		return;
	}
	// 82E9F348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F34C: 48000008  b 0x82e9f354
	sub_82E9F350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F350 size=12
    let mut pc: u32 = 0x82E9F350;
    'dispatch: loop {
        match pc {
            0x82E9F350 => {
    //   block [0x82E9F350..0x82E9F35C)
	// 82E9F350: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E9F354: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E9F358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F360 size=12
    let mut pc: u32 = 0x82E9F360;
    'dispatch: loop {
        match pc {
            0x82E9F360 => {
    //   block [0x82E9F360..0x82E9F36C)
	// 82E9F360: 1D640030  mulli r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 * 48;
	// 82E9F364: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E9F368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9F370 size=96
    let mut pc: u32 = 0x82E9F370;
    'dispatch: loop {
        match pc {
            0x82E9F370 => {
    //   block [0x82E9F370..0x82E9F3D0)
	// 82E9F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9F378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9F37C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9F380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9F388: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E9F38C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F390: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82E9F394: 419A0020  beq cr6, 0x82e9f3b4
	if ctx.cr[6].eq {
	pc = 0x82E9F3B4; continue 'dispatch;
	}
	// 82E9F398: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E9F39C: 41820018  beq 0x82e9f3b4
	if ctx.cr[0].eq {
	pc = 0x82E9F3B4; continue 'dispatch;
	}
	// 82E9F3A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F3A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9F3A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F3AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F3B0: 4E800421  bctrl
	ctx.lr = 0x82E9F3B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9F3B4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E9F3B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9F3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9F3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9F3C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9F3C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9F3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F3D0 size=112
    let mut pc: u32 = 0x82E9F3D0;
    'dispatch: loop {
        match pc {
            0x82E9F3D0 => {
    //   block [0x82E9F3D0..0x82E9F440)
	// 82E9F3D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E9F3D4: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82E9F3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9F3DC: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82E9F3E0: 994BFFFC  stb r10, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u8 ) };
	// 82E9F3E4: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82E9F3E8: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 82E9F3EC: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82E9F3F0: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82E9F3F4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E9F3F8: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F3FC: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E9F400: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82E9F404: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E9F408: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E9F40C: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82E9F410: 4080FFCC  bge 0x82e9f3dc
	if !ctx.cr[0].lt {
	pc = 0x82E9F3DC; continue 'dispatch;
	}
	// 82E9F414: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E9F418: 91430060  stw r10, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E9F41C: 3943002C  addi r10, r3, 0x2c
	ctx.r[10].s64 = ctx.r[3].s64 + 44;
	// 82E9F420: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E9F424: 906A0000  stw r3, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E9F428: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9F42C: 992AFFD5  stb r9, -0x2b(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-43 as u32), ctx.r[9].u8 ) };
	// 82E9F430: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 82E9F434: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E9F438: 4198FFE8  blt cr6, 0x82e9f420
	if ctx.cr[6].lt {
	pc = 0x82E9F420; continue 'dispatch;
	}
	// 82E9F43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F440 size=12
    let mut pc: u32 = 0x82E9F440;
    'dispatch: loop {
        match pc {
            0x82E9F440 => {
    //   block [0x82E9F440..0x82E9F44C)
	// 82E9F440: 80630060  lwz r3, 0x60(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E9F444: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9F448: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F44C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F44C size=20
    let mut pc: u32 = 0x82E9F44C;
    'dispatch: loop {
        match pc {
            0x82E9F44C => {
    //   block [0x82E9F44C..0x82E9F460)
	// 82E9F44C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F450: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9F454: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F45C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F460 size=4
    let mut pc: u32 = 0x82E9F460;
    'dispatch: loop {
        match pc {
            0x82E9F460 => {
    //   block [0x82E9F460..0x82E9F464)
	// 82E9F460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9F468 size=256
    let mut pc: u32 = 0x82E9F468;
    'dispatch: loop {
        match pc {
            0x82E9F468 => {
    //   block [0x82E9F468..0x82E9F568)
	// 82E9F468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F46C: 48308CF1  bl 0x831a815c
	ctx.lr = 0x82E9F470;
	sub_831A8130(ctx, base);
	// 82E9F470: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9F478: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82E9F47C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9F480: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E9F484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F488: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9F48C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F490: 4E800421  bctrl
	ctx.lr = 0x82E9F494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9F494: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F498: 418200C8  beq 0x82e9f560
	if ctx.cr[0].eq {
	pc = 0x82E9F560; continue 'dispatch;
	}
	// 82E9F49C: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82E9F4A0: 3BC1006C  addi r30, r1, 0x6c
	ctx.r[30].s64 = ctx.r[1].s64 + 108;
	// 82E9F4A4: 3B200002  li r25, 2
	ctx.r[25].s64 = 2;
	// 82E9F4A8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E9F4AC: 817EFFE4  lwz r11, -0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82E9F4B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F4B4: 409A0080  bne cr6, 0x82e9f534
	if !ctx.cr[6].eq {
	pc = 0x82E9F534; continue 'dispatch;
	}
	// 82E9F4B8: 83BEFFEC  lwz r29, -0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82E9F4BC: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F4C0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E9F4C4: 93BF0006  stw r29, 6(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[29].u32 ) };
	// 82E9F4C8: 419A0044  beq cr6, 0x82e9f50c
	if ctx.cr[6].eq {
	pc = 0x82E9F50C; continue 'dispatch;
	}
	// 82E9F4CC: 38FEFFF0  addi r7, r30, -0x10
	ctx.r[7].s64 = ctx.r[30].s64 + -16;
	// 82E9F4D0: 809F0012  lwz r4, 0x12(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82E9F4D4: 3B9F001A  addi r28, r31, 0x1a
	ctx.r[28].s64 = ctx.r[31].s64 + 26;
	// 82E9F4D8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E9F4DC: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E9F4E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E9F4E4: 4BFFFBFD  bl 0x82e9f0e0
	ctx.lr = 0x82E9F4E8;
	sub_82E9F0E0(ctx, base);
	// 82E9F4E8: 546B043F  clrlwi. r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F4EC: B07F0000  sth r3, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 82E9F4F0: 41820008  beq 0x82e9f4f8
	if ctx.cr[0].eq {
	pc = 0x82E9F4F8; continue 'dispatch;
	}
	// 82E9F4F4: B07F0002  sth r3, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[3].u16 ) };
	// 82E9F4F8: 5765083C  slwi r5, r27, 1
	ctx.r[5].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E9F4FC: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82E9F500: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E9F504: 4830900D  bl 0x831a8510
	ctx.lr = 0x82E9F508;
	sub_831A8510(ctx, base);
	// 82E9F508: 48000008  b 0x82e9f510
	pc = 0x82E9F510; continue 'dispatch;
	// 82E9F50C: B35F0000  sth r26, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 82E9F510: 937F0012  stw r27, 0x12(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[27].u32 ) };
	// 82E9F514: 387FFFFE  addi r3, r31, -2
	ctx.r[3].s64 = ctx.r[31].s64 + -2;
	// 82E9F518: 4BFFFD99  bl 0x82e9f2b0
	ctx.lr = 0x82E9F51C;
	sub_82E9F2B0(ctx, base);
	// 82E9F51C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E9F520: 809EFFE8  lwz r4, -0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82E9F524: A0BF0000  lhz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F528: 4BFFFC21  bl 0x82e9f148
	ctx.lr = 0x82E9F52C;
	sub_82E9F148(ctx, base);
	// 82E9F52C: 987FFFFE  stb r3, -2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-2 as u32), ctx.r[3].u8 ) };
	// 82E9F530: 48000020  b 0x82e9f550
	pc = 0x82E9F550; continue 'dispatch;
	// 82E9F534: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E9F538: 9B5FFFFE  stb r26, -2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-2 as u32), ctx.r[26].u8 ) };
	// 82E9F53C: B35F0000  sth r26, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 82E9F540: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82E9F544: 935F0012  stw r26, 0x12(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[26].u32 ) };
	// 82E9F548: 935F0016  stw r26, 0x16(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[26].u32 ) };
	// 82E9F54C: 935F0006  stw r26, 6(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[26].u32 ) };
	// 82E9F550: 3739FFFF  addic. r25, r25, -1
	ctx.xer.ca = (ctx.r[25].u32 > (!(-1 as u32)));
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82E9F554: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82E9F558: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 82E9F55C: 4082FF50  bne 0x82e9f4ac
	if !ctx.cr[0].eq {
	pc = 0x82E9F4AC; continue 'dispatch;
	}
	// 82E9F560: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E9F564: 48308C48  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9F568 size=92
    let mut pc: u32 = 0x82E9F568;
    'dispatch: loop {
        match pc {
            0x82E9F568 => {
    //   block [0x82E9F568..0x82E9F5C4)
	// 82E9F568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9F570: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9F574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F578: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9F57C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F580: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82E9F584: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F588: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9F58C: 4BFFFDE5  bl 0x82e9f370
	ctx.lr = 0x82E9F590;
	sub_82E9F370(ctx, base);
	// 82E9F590: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9F598: 419A0018  beq cr6, 0x82e9f5b0
	if ctx.cr[6].eq {
	pc = 0x82E9F5B0; continue 'dispatch;
	}
	// 82E9F59C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F5A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9F5A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F5A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F5AC: 4E800421  bctrl
	ctx.lr = 0x82E9F5B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9F5B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9F5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9F5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9F5BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9F5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F5C8 size=144
    let mut pc: u32 = 0x82E9F5C8;
    'dispatch: loop {
        match pc {
            0x82E9F5C8 => {
    //   block [0x82E9F5C8..0x82E9F658)
	// 82E9F5C8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E9F5CC: 88C3000E  lbz r6, 0xe(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82E9F5D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E9F5D4: 88A3000D  lbz r5, 0xd(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 82E9F5D8: 39630028  addi r11, r3, 0x28
	ctx.r[11].s64 = ctx.r[3].s64 + 40;
	// 82E9F5DC: 8883000C  lbz r4, 0xc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9F5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E9F5E4: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82E9F5E8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F5EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E9F5F0: 409A0028  bne cr6, 0x82e9f618
	if !ctx.cr[6].eq {
	pc = 0x82E9F618; continue 'dispatch;
	}
	// 82E9F5F4: 7CC95039  and. r9, r6, r10
	ctx.r[9].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F5F8: 4182000C  beq 0x82e9f604
	if ctx.cr[0].eq {
	pc = 0x82E9F604; continue 'dispatch;
	}
	// 82E9F5FC: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9F600: 48000014  b 0x82e9f614
	pc = 0x82E9F614; continue 'dispatch;
	// 82E9F604: 7C895039  and. r9, r4, r10
	ctx.r[9].u64 = ctx.r[4].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F608: 41820010  beq 0x82e9f618
	if ctx.cr[0].eq {
	pc = 0x82E9F618; continue 'dispatch;
	}
	// 82E9F60C: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9F610: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 82E9F614: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82E9F618: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F61C: 7D290775  extsb. r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F620: 4182001C  beq 0x82e9f63c
	if ctx.cr[0].eq {
	pc = 0x82E9F63C; continue 'dispatch;
	}
	// 82E9F624: 7CBF5039  and. r31, r5, r10
	ctx.r[31].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E9F628: 4182000C  beq 0x82e9f634
	if ctx.cr[0].eq {
	pc = 0x82E9F634; continue 'dispatch;
	}
	// 82E9F62C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E9F630: 48000008  b 0x82e9f638
	pc = 0x82E9F638; continue 'dispatch;
	// 82E9F634: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E9F638: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82E9F63C: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E9F640: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E9F644: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9F648: 4082FFA0  bne 0x82e9f5e8
	if !ctx.cr[0].eq {
	pc = 0x82E9F5E8; continue 'dispatch;
	}
	// 82E9F64C: 99030010  stb r8, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u8 ) };
	// 82E9F650: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82E9F654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F658 size=52
    let mut pc: u32 = 0x82E9F658;
    'dispatch: loop {
        match pc {
            0x82E9F658 => {
    //   block [0x82E9F658..0x82E9F68C)
	// 82E9F658: 88C3000E  lbz r6, 0xe(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82E9F65C: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82E9F660: 88A3000D  lbz r5, 0xd(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 82E9F664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E9F668: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E9F66C: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82E9F670: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F674: 7D290775  extsb. r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F678: 40820014  bne 0x82e9f68c
	if !ctx.cr[0].eq {
		sub_82E9F68C(ctx, base);
		return;
	}
	// 82E9F67C: 7D493039  and. r9, r10, r6
	ctx.r[9].u64 = ctx.r[10].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F680: 41820040  beq 0x82e9f6c0
	if ctx.cr[0].eq {
		sub_82E9F6B0(ctx, base);
		return;
	}
	// 82E9F684: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E9F688: 48000034  b 0x82e9f6bc
	sub_82E9F6B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F68C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F68C size=28
    let mut pc: u32 = 0x82E9F68C;
    'dispatch: loop {
        match pc {
            0x82E9F68C => {
    //   block [0x82E9F68C..0x82E9F6A8)
	// 82E9F68C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F690: 40990020  ble cr6, 0x82e9f6b0
	if !ctx.cr[6].gt {
		sub_82E9F6B0(ctx, base);
		return;
	}
	// 82E9F694: 7D443039  and. r4, r10, r6
	ctx.r[4].u64 = ctx.r[10].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E9F698: 41820010  beq 0x82e9f6a8
	if ctx.cr[0].eq {
		sub_82E9F6A8(ctx, base);
		return;
	}
	// 82E9F69C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82E9F6A0: 7D473B78  or r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82E9F6A4: 48000018  b 0x82e9f6bc
	sub_82E9F6B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F6A8 size=8
    let mut pc: u32 = 0x82E9F6A8;
    'dispatch: loop {
        match pc {
            0x82E9F6A8 => {
    //   block [0x82E9F6A8..0x82E9F6B0)
	// 82E9F6A8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E9F6AC: 48000010  b 0x82e9f6bc
	sub_82E9F6B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F6B0 size=40
    let mut pc: u32 = 0x82E9F6B0;
    'dispatch: loop {
        match pc {
            0x82E9F6B0 => {
    //   block [0x82E9F6B0..0x82E9F6D8)
	// 82E9F6B0: 7D492839  and. r9, r10, r5
	ctx.r[9].u64 = ctx.r[10].u64 & ctx.r[5].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9F6B4: 4182000C  beq 0x82e9f6c0
	if ctx.cr[0].eq {
	pc = 0x82E9F6C0; continue 'dispatch;
	}
	// 82E9F6B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E9F6BC: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82E9F6C0: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9F6C4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E9F6C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E9F6CC: 4082FFA4  bne 0x82e9f670
	if !ctx.cr[0].eq {
		sub_82E9F658(ctx, base);
		return;
	}
	// 82E9F6D0: 98E30011  stb r7, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[7].u8 ) };
	// 82E9F6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F6D8 size=16
    let mut pc: u32 = 0x82E9F6D8;
    'dispatch: loop {
        match pc {
            0x82E9F6D8 => {
    //   block [0x82E9F6D8..0x82E9F6E8)
	// 82E9F6D8: 3963003C  addi r11, r3, 0x3c
	ctx.r[11].s64 = ctx.r[3].s64 + 60;
	// 82E9F6DC: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E9F6E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9F6E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F6E8 size=20
    let mut pc: u32 = 0x82E9F6E8;
    'dispatch: loop {
        match pc {
            0x82E9F6E8 => {
    //   block [0x82E9F6E8..0x82E9F6FC)
	// 82E9F6E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F6EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9F6F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F6F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F6F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F6FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F6FC size=4
    let mut pc: u32 = 0x82E9F6FC;
    'dispatch: loop {
        match pc {
            0x82E9F6FC => {
    //   block [0x82E9F6FC..0x82E9F700)
	// 82E9F6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E9F700 size=76
    let mut pc: u32 = 0x82E9F700;
    'dispatch: loop {
        match pc {
            0x82E9F700 => {
    //   block [0x82E9F700..0x82E9F74C)
	// 82E9F700: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E9F704: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F708: 3940001E  li r10, 0x1e
	ctx.r[10].s64 = 30;
	// 82E9F70C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82E9F710: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9F714: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9F718: C009E7B4  lfs f0, -0x184c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-6220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9F71C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9F720: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E9F724: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E9F728: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E9F72C: 91030018  stw r8, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82E9F730: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E9F734: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E9F738: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E9F73C: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E9F740: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E9F744: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E9F748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9F750 size=100
    let mut pc: u32 = 0x82E9F750;
    'dispatch: loop {
        match pc {
            0x82E9F750 => {
    //   block [0x82E9F750..0x82E9F7B4)
	// 82E9F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9F758: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9F75C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F760: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9F764: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F768: 3945000E  addi r10, r5, 0xe
	ctx.r[10].s64 = ctx.r[5].s64 + 14;
	// 82E9F76C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F770: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9F774: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E9F778: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E9F77C: 4BFFFBF5  bl 0x82e9f370
	ctx.lr = 0x82E9F780;
	sub_82E9F370(ctx, base);
	// 82E9F780: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F784: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9F788: 419A0018  beq cr6, 0x82e9f7a0
	if ctx.cr[6].eq {
	pc = 0x82E9F7A0; continue 'dispatch;
	}
	// 82E9F78C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F790: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9F794: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F798: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F79C: 4E800421  bctrl
	ctx.lr = 0x82E9F7A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9F7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9F7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9F7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9F7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9F7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9F7B8 size=256
    let mut pc: u32 = 0x82E9F7B8;
    'dispatch: loop {
        match pc {
            0x82E9F7B8 => {
    //   block [0x82E9F7B8..0x82E9F8B8)
	// 82E9F7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9F7C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9F7C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F7C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9F7CC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E9F7D0: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E9F7D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9F7D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9F7DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9F7E0: 4E800421  bctrl
	ctx.lr = 0x82E9F7E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9F7E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9F7E8: 418200BC  beq 0x82e9f8a4
	if ctx.cr[0].eq {
	pc = 0x82E9F8A4; continue 'dispatch;
	}
	// 82E9F7EC: E9610066  lwa r11, 0x64(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as i32) as i64;
	// 82E9F7F0: C01F0024  lfs f0, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9F7F4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E9F7F8: C1BF0020  lfs f13, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E9F7FC: E941006A  lwa r10, 0x68(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as i32) as i64;
	// 82E9F800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9F804: E9210062  lwa r9, 0x60(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as i32) as i64;
	// 82E9F808: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9F80C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82E9F810: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82E9F814: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9F818: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E9F81C: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9F820: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82E9F824: 891F000C  lbz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9F828: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82E9F82C: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E9F830: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82E9F834: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E9F838: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82E9F83C: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82E9F840: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82E9F844: 7D6758F8  nor r7, r11, r11
	ctx.r[7].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82E9F848: 7D2A5A78  xor r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 ^ ctx.r[11].u64;
	// 82E9F84C: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82E9F850: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82E9F854: 98FF000D  stb r7, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[7].u8 ) };
	// 82E9F858: 7D465838  and r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82E9F85C: 7D4B4838  and r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[9].u64;
	// 82E9F860: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82E9F864: 98DF000E  stb r6, 0xe(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[6].u8 ) };
	// 82E9F868: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E9F86C: 997F000F  stb r11, 0xf(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 82E9F870: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82E9F874: 993F0011  stb r9, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[9].u8 ) };
	// 82E9F878: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E9F87C: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E9F880: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E9F884: FD80501E  fctiwz f12, f10
	ctx.f[12].s64 = if ctx.f[10].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[10].f64.trunc() as i32 as i64 };
	// 82E9F888: 7D80FFAE  stfiwx f12, 0, r31
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32, tmp.u32) };
	// 82E9F88C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82E9F890: 7C1F47AE  stfiwx f0, r31, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82E9F894: FC00681E  fctiwz f0, f13
	ctx.f[0].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82E9F898: 7C1F57AE  stfiwx f0, r31, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82E9F89C: 4BFFFD2D  bl 0x82e9f5c8
	ctx.lr = 0x82E9F8A0;
	sub_82E9F5C8(ctx, base);
	// 82E9F8A0: 4BFFFDB9  bl 0x82e9f658
	ctx.lr = 0x82E9F8A4;
	sub_82E9F658(ctx, base);
	// 82E9F8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9F8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9F8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9F8B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9F8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F8B8 size=40
    let mut pc: u32 = 0x82E9F8B8;
    'dispatch: loop {
        match pc {
            0x82E9F8B8 => {
    //   block [0x82E9F8B8..0x82E9F8E0)
	// 82E9F8B8: 7CCB0734  extsh r11, r6
	ctx.r[11].s64 = ctx.r[6].s16 as i64;
	// 82E9F8BC: A9430000  lha r10, 0(r3)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82E9F8C0: 7CEB00D0  neg r7, r11
	ctx.r[7].s64 = -ctx.r[11].s64;
	// 82E9F8C4: A9040000  lha r8, 0(r4)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82E9F8C8: 7F075000  cmpw cr6, r7, r10
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E9F8CC: 40980014  bge cr6, 0x82e9f8e0
	if !ctx.cr[6].lt {
		sub_82E9F8E0(ctx, base);
		return;
	}
	// 82E9F8D0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9F8D4: 4098000C  bge cr6, 0x82e9f8e0
	if !ctx.cr[6].lt {
		sub_82E9F8E0(ctx, base);
		return;
	}
	// 82E9F8D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E9F8DC: 48000014  b 0x82e9f8f0
	sub_82E9F8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F8E0 size=40
    let mut pc: u32 = 0x82E9F8E0;
    'dispatch: loop {
        match pc {
            0x82E9F8E0 => {
    //   block [0x82E9F8E0..0x82E9F908)
	// 82E9F8E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9F8E4: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E9F8E8: 41990008  bgt cr6, 0x82e9f8f0
	if ctx.cr[6].gt {
	pc = 0x82E9F8F0; continue 'dispatch;
	}
	// 82E9F8EC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E9F8F0: 7F074000  cmpw cr6, r7, r8
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9F8F4: 40980014  bge cr6, 0x82e9f908
	if !ctx.cr[6].lt {
		sub_82E9F908(ctx, base);
		return;
	}
	// 82E9F8F8: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E9F8FC: 4098000C  bge cr6, 0x82e9f908
	if !ctx.cr[6].lt {
		sub_82E9F908(ctx, base);
		return;
	}
	// 82E9F900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9F904: 48000018  b 0x82e9f91c
	sub_82E9F918(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F908 size=16
    let mut pc: u32 = 0x82E9F908;
    'dispatch: loop {
        match pc {
            0x82E9F908 => {
    //   block [0x82E9F908..0x82E9F918)
	// 82E9F908: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E9F90C: 4099000C  ble cr6, 0x82e9f918
	if !ctx.cr[6].gt {
		sub_82E9F918(ctx, base);
		return;
	}
	// 82E9F910: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82E9F914: 48000008  b 0x82e9f91c
	sub_82E9F918(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9F918 size=132
    let mut pc: u32 = 0x82E9F918;
    'dispatch: loop {
        match pc {
            0x82E9F918 => {
    //   block [0x82E9F918..0x82E9F99C)
	// 82E9F918: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E9F91C: 7CEB59D6  mullw r7, r11, r11
	ctx.r[7].s64 = (ctx.r[11].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82E9F920: 7D0949D6  mullw r8, r9, r9
	ctx.r[8].s64 = (ctx.r[9].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82E9F924: 7CAA0734  extsh r10, r5
	ctx.r[10].s64 = ctx.r[5].s16 as i64;
	// 82E9F928: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82E9F92C: 7CEA51D6  mullw r7, r10, r10
	ctx.r[7].s64 = (ctx.r[10].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82E9F930: 7F074000  cmpw cr6, r7, r8
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E9F934: 4098005C  bge cr6, 0x82e9f990
	if !ctx.cr[6].lt {
	pc = 0x82E9F990; continue 'dispatch;
	}
	// 82E9F938: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82E9F93C: 7CEA59D6  mullw r7, r10, r11
	ctx.r[7].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82E9F940: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 82E9F944: 7D2A49D6  mullw r9, r10, r9
	ctx.r[9].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82E9F948: 552A083E  rotlwi r10, r9, 1
	ctx.r[10].u64 = ((ctx.r[9].u32).rotate_left(1)) as u64;
	// 82E9F94C: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9F950: 54EB083E  rotlwi r11, r7, 1
	ctx.r[11].u64 = ((ctx.r[7].u32).rotate_left(1)) as u64;
	// 82E9F954: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E9F958: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E9F95C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E9F960: EC00002C  fsqrts f0, f0
	ctx.f[0].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 82E9F964: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82E9F968: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82E9F96C: 8101FFF4  lwz r8, -0xc(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82E9F970: 7D065878  andc r6, r8, r11
	ctx.r[6].u64 = ctx.r[8].u64 & !ctx.r[11].u64;
	// 82E9F974: 7D0A5078  andc r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 & !ctx.r[10].u64;
	// 82E9F978: 7D2943D6  divw r9, r9, r8
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[8].s32;
	// 82E9F97C: 0CC80000  twi 6, r8, 0
	// 82E9F980: 0CAAFFFF  twi 5, r10, -1
	// 82E9F984: 7D6743D6  divw r11, r7, r8
	ctx.r[11].s32 = ctx.r[7].s32 / ctx.r[8].s32;
	// 82E9F988: 0CC80000  twi 6, r8, 0
	// 82E9F98C: 0CA6FFFF  twi 5, r6, -1
	// 82E9F990: B1230000  sth r9, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82E9F994: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E9F998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9F9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9F9A0 size=160
    let mut pc: u32 = 0x82E9F9A0;
    'dispatch: loop {
        match pc {
            0x82E9F9A0 => {
    //   block [0x82E9F9A0..0x82E9FA40)
	// 82E9F9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9F9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9F9A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9F9AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9F9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9F9B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9F9B8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82E9F9BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E9F9C0: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82E9F9C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E9F9C8: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9F9CC: A15F000A  lhz r10, 0xa(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82E9F9D0: B1610052  sth r11, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 82E9F9D4: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82E9F9D8: 4BFFFEE1  bl 0x82e9f8b8
	ctx.lr = 0x82E9F9DC;
	sub_82E9F8B8(ctx, base);
	// 82E9F9DC: 7FCB0734  extsh r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	// 82E9F9E0: A9410052  lha r10, 0x52(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as i16) as i64;
	// 82E9F9E4: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82E9F9E8: A9610050  lha r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i16) as i64;
	// 82E9F9EC: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E9F9F0: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82E9F9F4: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E9F9F8: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82E9F9FC: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E9FA00: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E9FA04: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82E9FA08: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E9FA0C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E9FA10: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82E9FA14: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E9FA18: ED8C0024  fdivs f12, f12, f0
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 82E9FA1C: D19F0000  stfs f12, 0(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E9FA20: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82E9FA24: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E9FA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9FA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9FA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9FA34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9FA38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9FA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FA40 size=8
    let mut pc: u32 = 0x82E9FA40;
    'dispatch: loop {
        match pc {
            0x82E9FA40 => {
    //   block [0x82E9FA40..0x82E9FA48)
	// 82E9FA40: 806300E0  lwz r3, 0xe0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(224 as u32) ) } as u64;
	// 82E9FA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FA48 size=12
    let mut pc: u32 = 0x82E9FA48;
    'dispatch: loop {
        match pc {
            0x82E9FA48 => {
    //   block [0x82E9FA48..0x82E9FA54)
	// 82E9FA48: 1D640038  mulli r11, r4, 0x38
	ctx.r[11].s64 = ctx.r[4].s64 * 56;
	// 82E9FA4C: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E9FA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E9FA58 size=116
    let mut pc: u32 = 0x82E9FA58;
    'dispatch: loop {
        match pc {
            0x82E9FA58 => {
    //   block [0x82E9FA58..0x82E9FACC)
	// 82E9FA58: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E9FA5C: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82E9FA60: 39630018  addi r11, r3, 0x18
	ctx.r[11].s64 = ctx.r[3].s64 + 24;
	// 82E9FA64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E9FA68: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E9FA6C: 914BFFE8  stw r10, -0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24 as u32), ctx.r[10].u32 ) };
	// 82E9FA70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E9FA74: 914BFFEC  stw r10, -0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20 as u32), ctx.r[10].u32 ) };
	// 82E9FA78: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9FA7C: 914BFFF0  stw r10, -0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 82E9FA80: 914BFFF4  stw r10, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[10].u32 ) };
	// 82E9FA84: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82E9FA88: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82E9FA8C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82E9FA90: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 82E9FA94: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E9FA98: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E9FA9C: B14B000C  sth r10, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 82E9FAA0: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 82E9FAA4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E9FAA8: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E9FAAC: B14B0018  sth r10, 0x18(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 82E9FAB0: B14B001A  sth r10, 0x1a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 82E9FAB4: 910B001C  stw r8, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82E9FAB8: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82E9FABC: 4080FFB0  bge 0x82e9fa6c
	if !ctx.cr[0].lt {
	pc = 0x82E9FA6C; continue 'dispatch;
	}
	// 82E9FAC0: 914300E0  stw r10, 0xe0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82E9FAC4: 914300E4  stw r10, 0xe4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 82E9FAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FAD0 size=12
    let mut pc: u32 = 0x82E9FAD0;
    'dispatch: loop {
        match pc {
            0x82E9FAD0 => {
    //   block [0x82E9FAD0..0x82E9FADC)
	// 82E9FAD0: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 82E9FAD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9FAD8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FADC size=20
    let mut pc: u32 = 0x82E9FADC;
    'dispatch: loop {
        match pc {
            0x82E9FADC => {
    //   block [0x82E9FADC..0x82E9FAF0)
	// 82E9FADC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FAE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9FAE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FAE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FAEC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FAF0 size=4
    let mut pc: u32 = 0x82E9FAF0;
    'dispatch: loop {
        match pc {
            0x82E9FAF0 => {
    //   block [0x82E9FAF0..0x82E9FAF4)
	// 82E9FAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E9FAF8 size=552
    let mut pc: u32 = 0x82E9FAF8;
    'dispatch: loop {
        match pc {
            0x82E9FAF8 => {
    //   block [0x82E9FAF8..0x82E9FD20)
	// 82E9FAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9FAFC: 48308669  bl 0x831a8164
	ctx.lr = 0x82E9FB00;
	sub_831A8130(ctx, base);
	// 82E9FB00: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E9FB04: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9FB08: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E9FB0C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82E9FB10: 39610088  addi r11, r1, 0x88
	ctx.r[11].s64 = ctx.r[1].s64 + 136;
	// 82E9FB14: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E9FB18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E9FB1C: 93CBFFFC  stw r30, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[30].u32 ) };
	// 82E9FB20: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82E9FB24: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9FB28: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82E9FB2C: B3CB0002  sth r30, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[30].u16 ) };
	// 82E9FB30: B3CB0004  sth r30, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 82E9FB34: B3CB0006  sth r30, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[30].u16 ) };
	// 82E9FB38: B3CB0008  sth r30, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u16 ) };
	// 82E9FB3C: B3CB000A  sth r30, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[30].u16 ) };
	// 82E9FB40: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82E9FB44: 4080FFD4  bge 0x82e9fb18
	if !ctx.cr[0].lt {
	pc = 0x82E9FB18; continue 'dispatch;
	}
	// 82E9FB48: 807B00E4  lwz r3, 0xe4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(228 as u32) ) } as u64;
	// 82E9FB4C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E9FB50: 80BB00E0  lwz r5, 0xe0(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(224 as u32) ) } as u64;
	// 82E9FB54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FB58: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E9FB5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FB60: 4E800421  bctrl
	ctx.lr = 0x82E9FB64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FB64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E9FB68: 418201AC  beq 0x82e9fd14
	if ctx.cr[0].eq {
	pc = 0x82E9FD14; continue 'dispatch;
	}
	// 82E9FB6C: 817B00E0  lwz r11, 0xe0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(224 as u32) ) } as u64;
	// 82E9FB70: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82E9FB74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E9FB78: 4099019C  ble cr6, 0x82e9fd14
	if !ctx.cr[6].gt {
	pc = 0x82E9FD14; continue 'dispatch;
	}
	// 82E9FB7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E9FB80: 3BA10088  addi r29, r1, 0x88
	ctx.r[29].s64 = ctx.r[1].s64 + 136;
	// 82E9FB84: 3BFB001A  addi r31, r27, 0x1a
	ctx.r[31].s64 = ctx.r[27].s64 + 26;
	// 82E9FB88: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E9FB8C: 815DFFF8  lwz r10, -8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9FB90: 813F001A  lwz r9, 0x1a(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82E9FB94: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E9FB98: 41820064  beq 0x82e9fbfc
	if ctx.cr[0].eq {
	pc = 0x82E9FBFC; continue 'dispatch;
	}
	// 82E9FB9C: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82E9FBA0: 419A0054  beq cr6, 0x82e9fbf4
	if ctx.cr[6].eq {
	pc = 0x82E9FBF4; continue 'dispatch;
	}
	// 82E9FBA4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82E9FBA8: 915F001A  stw r10, 0x1a(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[10].u32 ) };
	// 82E9FBAC: 93DFFFF2  stw r30, -0xe(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-14 as u32), ctx.r[30].u32 ) };
	// 82E9FBB0: 93DFFFEE  stw r30, -0x12(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18 as u32), ctx.r[30].u32 ) };
	// 82E9FBB4: 419A0148  beq cr6, 0x82e9fcfc
	if ctx.cr[6].eq {
	pc = 0x82E9FCFC; continue 'dispatch;
	}
	// 82E9FBB8: 93DFFFEA  stw r30, -0x16(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22 as u32), ctx.r[30].u32 ) };
	// 82E9FBBC: 93DFFFE6  stw r30, -0x1a(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26 as u32), ctx.r[30].u32 ) };
	// 82E9FBC0: D3FFFFFA  stfs f31, -6(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-6 as u32), tmp.u32 ) };
	// 82E9FBC4: D3FFFFF6  stfs f31, -0xa(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-10 as u32), tmp.u32 ) };
	// 82E9FBC8: B3DF0000  sth r30, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82E9FBCC: B3DFFFFE  sth r30, -2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(-2 as u32), ctx.r[30].u16 ) };
	// 82E9FBD0: D3FF0006  stfs f31, 6(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), tmp.u32 ) };
	// 82E9FBD4: D3FF0002  stfs f31, 2(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), tmp.u32 ) };
	// 82E9FBD8: B3DF000C  sth r30, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 82E9FBDC: B3DF000A  sth r30, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[30].u16 ) };
	// 82E9FBE0: D3FF0012  stfs f31, 0x12(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), tmp.u32 ) };
	// 82E9FBE4: D3FF000E  stfs f31, 0xe(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), tmp.u32 ) };
	// 82E9FBE8: B3DF0018  sth r30, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u16 ) };
	// 82E9FBEC: B3DF0016  sth r30, 0x16(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[30].u16 ) };
	// 82E9FBF0: 4800010C  b 0x82e9fcfc
	pc = 0x82E9FCFC; continue 'dispatch;
	// 82E9FBF4: 93DF001A  stw r30, 0x1a(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[30].u32 ) };
	// 82E9FBF8: 48000104  b 0x82e9fcfc
	pc = 0x82E9FCFC; continue 'dispatch;
	// 82E9FBFC: 817DFFFC  lwz r11, -4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E9FC00: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E9FC04: 813FFFE6  lwz r9, -0x1a(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26 as u32) ) } as u64;
	// 82E9FC08: 7D6858F8  nor r8, r11, r11
	ctx.r[8].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82E9FC0C: 915F001A  stw r10, 0x1a(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[10].u32 ) };
	// 82E9FC10: 911FFFEA  stw r8, -0x16(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22 as u32), ctx.r[8].u32 ) };
	// 82E9FC14: 917FFFE6  stw r11, -0x1a(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26 as u32), ctx.r[11].u32 ) };
	// 82E9FC18: 419A0010  beq cr6, 0x82e9fc28
	if ctx.cr[6].eq {
	pc = 0x82E9FC28; continue 'dispatch;
	}
	// 82E9FC1C: 93DFFFF2  stw r30, -0xe(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-14 as u32), ctx.r[30].u32 ) };
	// 82E9FC20: 93DFFFEE  stw r30, -0x12(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18 as u32), ctx.r[30].u32 ) };
	// 82E9FC24: 48000018  b 0x82e9fc3c
	pc = 0x82E9FC3C; continue 'dispatch;
	// 82E9FC28: 7D2A5A78  xor r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 ^ ctx.r[11].u64;
	// 82E9FC2C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82E9FC30: 7D4A4838  and r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[9].u64;
	// 82E9FC34: 917FFFEE  stw r11, -0x12(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18 as u32), ctx.r[11].u32 ) };
	// 82E9FC38: 915FFFF2  stw r10, -0xe(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-14 as u32), ctx.r[10].u32 ) };
	// 82E9FC3C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FC40: 39407FFF  li r10, 0x7fff
	ctx.r[10].s64 = 32767;
	// 82E9FC44: A07D000A  lhz r3, 0xa(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(10 as u32) ) } as u64;
	// 82E9FC48: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82E9FC4C: A13D0002  lhz r9, 2(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E9FC50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E9FC54: A11D0004  lhz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9FC58: A0FD0006  lhz r7, 6(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E9FC5C: A0DD0008  lhz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9FC60: B17FFFFE  sth r11, -2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(-2 as u32), ctx.r[11].u16 ) };
	// 82E9FC64: B13F0000  sth r9, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82E9FC68: B11F000A  sth r8, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[8].u16 ) };
	// 82E9FC6C: B0FF000C  sth r7, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u16 ) };
	// 82E9FC70: B0DF0016  sth r6, 0x16(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[6].u16 ) };
	// 82E9FC74: B07F0018  sth r3, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u16 ) };
	// 82E9FC78: 807B00E4  lwz r3, 0xe4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(228 as u32) ) } as u64;
	// 82E9FC7C: B1410070  sth r10, 0x70(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 82E9FC80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FC84: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E9FC88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FC8C: 4E800421  bctrl
	ctx.lr = 0x82E9FC90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FC90: 387FFFF6  addi r3, r31, -0xa
	ctx.r[3].s64 = ctx.r[31].s64 + -10;
	// 82E9FC94: A0A1006C  lhz r5, 0x6c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E9FC98: A0810068  lhz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E9FC9C: 4BFFFD05  bl 0x82e9f9a0
	ctx.lr = 0x82E9FCA0;
	sub_82E9F9A0(ctx, base);
	// 82E9FCA0: 387F0002  addi r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 2;
	// 82E9FCA4: A0A1006E  lhz r5, 0x6e(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(110 as u32) ) } as u64;
	// 82E9FCA8: A081006A  lhz r4, 0x6a(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(106 as u32) ) } as u64;
	// 82E9FCAC: 4BFFFCF5  bl 0x82e9f9a0
	ctx.lr = 0x82E9FCB0;
	sub_82E9F9A0(ctx, base);
	// 82E9FCB0: A9610070  lha r11, 0x70(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as i16) as i64;
	// 82E9FCB4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E9FCB8: A95F0016  lha r10, 0x16(r31)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as i16) as i64;
	// 82E9FCBC: A97F0018  lha r11, 0x18(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as i16) as i64;
	// 82E9FCC0: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82E9FCC4: C9A10058  lfd f13, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E9FCC8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E9FCCC: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 82E9FCD0: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E9FCD4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E9FCD8: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82E9FCDC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E9FCE0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E9FCE4: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82E9FCE8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E9FCEC: ED8C0024  fdivs f12, f12, f0
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 82E9FCF0: D19F000E  stfs f12, 0xe(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), tmp.u32 ) };
	// 82E9FCF4: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82E9FCF8: D01F0012  stfs f0, 0x12(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), tmp.u32 ) };
	// 82E9FCFC: 817B00E0  lwz r11, 0xe0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(224 as u32) ) } as u64;
	// 82E9FD00: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E9FD04: 3BFF0038  addi r31, r31, 0x38
	ctx.r[31].s64 = ctx.r[31].s64 + 56;
	// 82E9FD08: 3BBD0014  addi r29, r29, 0x14
	ctx.r[29].s64 = ctx.r[29].s64 + 20;
	// 82E9FD0C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E9FD10: 4198FE7C  blt cr6, 0x82e9fb8c
	if ctx.cr[6].lt {
	pc = 0x82E9FB8C; continue 'dispatch;
	}
	// 82E9FD14: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82E9FD18: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E9FD1C: 48308498  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9FD20 size=160
    let mut pc: u32 = 0x82E9FD20;
    'dispatch: loop {
        match pc {
            0x82E9FD20 => {
    //   block [0x82E9FD20..0x82E9FDC0)
	// 82E9FD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9FD24: 48308445  bl 0x831a8168
	ctx.lr = 0x82E9FD28;
	sub_831A8130(ctx, base);
	// 82E9FD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9FD2C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E9FD30: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E9FD34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9FD38: 3BFE00E4  addi r31, r30, 0xe4
	ctx.r[31].s64 = ctx.r[30].s64 + 228;
	// 82E9FD3C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FD40: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E9FD44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9FD48: 4BFFF629  bl 0x82e9f370
	ctx.lr = 0x82E9FD4C;
	sub_82E9F370(ctx, base);
	// 82E9FD4C: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82E9FD50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E9FD54: 419A0040  beq cr6, 0x82e9fd94
	if ctx.cr[6].eq {
	pc = 0x82E9FD94; continue 'dispatch;
	}
	// 82E9FD58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FD5C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9FD60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FD64: 4E800421  bctrl
	ctx.lr = 0x82E9FD68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FD68: 2B030004  cmplwi cr6, r3, 4
	ctx.cr[6].compare_u32(ctx.r[3].u32, 4 as u32, &mut ctx.xer);
	// 82E9FD6C: 4098001C  bge cr6, 0x82e9fd88
	if !ctx.cr[6].lt {
	pc = 0x82E9FD88; continue 'dispatch;
	}
	// 82E9FD70: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FD74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FD78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9FD7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FD80: 4E800421  bctrl
	ctx.lr = 0x82E9FD84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FD84: 48000008  b 0x82e9fd8c
	pc = 0x82E9FD8C; continue 'dispatch;
	// 82E9FD88: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82E9FD8C: 907E00E0  stw r3, 0xe0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(224 as u32), ctx.r[3].u32 ) };
	// 82E9FD90: 48000008  b 0x82e9fd98
	pc = 0x82E9FD98; continue 'dispatch;
	// 82E9FD94: 939E00E0  stw r28, 0xe0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(224 as u32), ctx.r[28].u32 ) };
	// 82E9FD98: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FD9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E9FDA0: 419A0018  beq cr6, 0x82e9fdb8
	if ctx.cr[6].eq {
	pc = 0x82E9FDB8; continue 'dispatch;
	}
	// 82E9FDA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FDA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E9FDAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FDB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FDB4: 4E800421  bctrl
	ctx.lr = 0x82E9FDB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E9FDBC: 483083FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FDC0 size=20
    let mut pc: u32 = 0x82E9FDC0;
    'dispatch: loop {
        match pc {
            0x82E9FDC0 => {
    //   block [0x82E9FDC0..0x82E9FDD4)
	// 82E9FDC0: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 82E9FDC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FDC8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E9FDCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FDD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FDD8 size=20
    let mut pc: u32 = 0x82E9FDD8;
    'dispatch: loop {
        match pc {
            0x82E9FDD8 => {
    //   block [0x82E9FDD8..0x82E9FDEC)
	// 82E9FDD8: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 82E9FDDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FDE0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9FDE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E9FDE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FDF0 size=36
    let mut pc: u32 = 0x82E9FDF0;
    'dispatch: loop {
        match pc {
            0x82E9FDF0 => {
    //   block [0x82E9FDF0..0x82E9FE14)
	// 82E9FDF0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E9FDF4: 419A0018  beq cr6, 0x82e9fe0c
	if ctx.cr[6].eq {
	pc = 0x82E9FE0C; continue 'dispatch;
	}
	// 82E9FDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E9FDFC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9FE00: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E9FE04: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E9FE08: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E9FE0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9FE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FE18 size=16
    let mut pc: u32 = 0x82E9FE18;
    'dispatch: loop {
        match pc {
            0x82E9FE18 => {
    //   block [0x82E9FE18..0x82E9FE28)
	// 82E9FE18: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9FE1C: 396BE7CC  addi r11, r11, -0x1834
	ctx.r[11].s64 = ctx.r[11].s64 + -6196;
	// 82E9FE20: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9FE28 size=68
    let mut pc: u32 = 0x82E9FE28;
    'dispatch: loop {
        match pc {
            0x82E9FE28 => {
    //   block [0x82E9FE28..0x82E9FE6C)
	// 82E9FE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9FE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9FE30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9FE34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9FE38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9FE3C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9FE40: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9FE44: 396BE7C0  addi r11, r11, -0x1840
	ctx.r[11].s64 = ctx.r[11].s64 + -6208;
	// 82E9FE48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9FE4C: 41820008  beq 0x82e9fe54
	if ctx.cr[0].eq {
	pc = 0x82E9FE54; continue 'dispatch;
	}
	// 82E9FE50: 4B420419  bl 0x822c0268
	ctx.lr = 0x82E9FE54;
	sub_822C0268(ctx, base);
	// 82E9FE54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9FE58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9FE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9FE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9FE64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9FE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FE70 size=88
    let mut pc: u32 = 0x82E9FE70;
    'dispatch: loop {
        match pc {
            0x82E9FE70 => {
    //   block [0x82E9FE70..0x82E9FEC8)
	// 82E9FE70: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E9FE74: 419A004C  beq cr6, 0x82e9fec0
	if ctx.cr[6].eq {
	pc = 0x82E9FEC0; continue 'dispatch;
	}
	// 82E9FE78: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E9FE7C: 419A0044  beq cr6, 0x82e9fec0
	if ctx.cr[6].eq {
	pc = 0x82E9FEC0; continue 'dispatch;
	}
	// 82E9FE80: 39640008  addi r11, r4, 8
	ctx.r[11].s64 = ctx.r[4].s64 + 8;
	// 82E9FE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E9FE88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E9FE8C: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 82E9FE90: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E9FE94: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E9FE98: 910BFFF8  stw r8, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	// 82E9FE9C: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82E9FEA0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82E9FEA4: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82E9FEA8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82E9FEAC: 4200FFF8  bdnz 0x82e9fea4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E9FEA4; continue 'dispatch;
	}
	// 82E9FEB0: 912B0014  stw r9, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82E9FEB4: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E9FEB8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82E9FEBC: 4082FFCC  bne 0x82e9fe88
	if !ctx.cr[0].eq {
	pc = 0x82E9FE88; continue 'dispatch;
	}
	// 82E9FEC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E9FEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FEC8 size=16
    let mut pc: u32 = 0x82E9FEC8;
    'dispatch: loop {
        match pc {
            0x82E9FEC8 => {
    //   block [0x82E9FEC8..0x82E9FED8)
	// 82E9FEC8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9FECC: 396BE7E8  addi r11, r11, -0x1818
	ctx.r[11].s64 = ctx.r[11].s64 + -6168;
	// 82E9FED0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9FED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9FED8 size=68
    let mut pc: u32 = 0x82E9FED8;
    'dispatch: loop {
        match pc {
            0x82E9FED8 => {
    //   block [0x82E9FED8..0x82E9FF1C)
	// 82E9FED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9FEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9FEE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9FEE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9FEE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E9FEEC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E9FEF0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E9FEF4: 396BE7D8  addi r11, r11, -0x1828
	ctx.r[11].s64 = ctx.r[11].s64 + -6184;
	// 82E9FEF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E9FEFC: 41820008  beq 0x82e9ff04
	if ctx.cr[0].eq {
	pc = 0x82E9FF04; continue 'dispatch;
	}
	// 82E9FF00: 4B420369  bl 0x822c0268
	ctx.lr = 0x82E9FF04;
	sub_822C0268(ctx, base);
	// 82E9FF04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9FF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E9FF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9FF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9FF14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E9FF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FF20 size=4
    let mut pc: u32 = 0x82E9FF20;
    'dispatch: loop {
        match pc {
            0x82E9FF20 => {
    //   block [0x82E9FF20..0x82E9FF24)
	// 82E9FF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FF28 size=4
    let mut pc: u32 = 0x82E9FF28;
    'dispatch: loop {
        match pc {
            0x82E9FF28 => {
    //   block [0x82E9FF28..0x82E9FF2C)
	// 82E9FF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E9FF30 size=12
    let mut pc: u32 = 0x82E9FF30;
    'dispatch: loop {
        match pc {
            0x82E9FF30 => {
    //   block [0x82E9FF30..0x82E9FF3C)
	// 82E9FF30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E9FF34: 386B9EAC  addi r3, r11, -0x6154
	ctx.r[3].s64 = ctx.r[11].s64 + -24916;
	// 82E9FF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E9FF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E9FF40 size=196
    let mut pc: u32 = 0x82E9FF40;
    'dispatch: loop {
        match pc {
            0x82E9FF40 => {
    //   block [0x82E9FF40..0x82EA0004)
	// 82E9FF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E9FF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E9FF48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E9FF4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E9FF50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E9FF54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9FF58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E9FF5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E9FF60: 4800A0C9  bl 0x82eaa028
	ctx.lr = 0x82E9FF64;
	sub_82EAA028(ctx, base);
	// 82E9FF64: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E9FF68: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E9FF6C: 419A0030  beq cr6, 0x82e9ff9c
	if ctx.cr[6].eq {
	pc = 0x82E9FF9C; continue 'dispatch;
	}
	// 82E9FF70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FF74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9FF78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9FF7C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9FF80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E9FF84: 4E800421  bctrl
	ctx.lr = 0x82E9FF88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FF88: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9FF90: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9FF94: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82E9FF98: 48000050  b 0x82e9ffe8
	pc = 0x82E9FFE8; continue 'dispatch;
	// 82E9FF9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FFA0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E9FFA4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E9FFA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E9FFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9FFB0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E9FFB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E9FFB8: 4E800421  bctrl
	ctx.lr = 0x82E9FFBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FFBC: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FFC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E9FFC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E9FFC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E9FFCC: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E9FFD0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82E9FFD4: 4E800421  bctrl
	ctx.lr = 0x82E9FFD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FFD8: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E9FFDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E9FFE0: 80C70018  lwz r6, 0x18(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E9FFE4: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82E9FFE8: 4E800421  bctrl
	ctx.lr = 0x82E9FFEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E9FFEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E9FFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E9FFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E9FFF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E9FFFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA0000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0008 size=12
    let mut pc: u32 = 0x82EA0008;
    'dispatch: loop {
        match pc {
            0x82EA0008 => {
    //   block [0x82EA0008..0x82EA0014)
	// 82EA0008: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82EA000C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA0010: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0014 size=16
    let mut pc: u32 = 0x82EA0014;
    'dispatch: loop {
        match pc {
            0x82EA0014 => {
    //   block [0x82EA0014..0x82EA0024)
	// 82EA0014: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EA0018: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EA001C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EA0020: 48009F08  b 0x82ea9f28
	sub_82EA9F28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0024(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0024 size=4
    let mut pc: u32 = 0x82EA0024;
    'dispatch: loop {
        match pc {
            0x82EA0024 => {
    //   block [0x82EA0024..0x82EA0028)
	// 82EA0024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0028 size=16
    let mut pc: u32 = 0x82EA0028;
    'dispatch: loop {
        match pc {
            0x82EA0028 => {
    //   block [0x82EA0028..0x82EA0038)
	// 82EA0028: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA002C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA0030: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EA0034: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0038 size=20
    let mut pc: u32 = 0x82EA0038;
    'dispatch: loop {
        match pc {
            0x82EA0038 => {
    //   block [0x82EA0038..0x82EA004C)
	// 82EA0038: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA003C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA0040: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA0044: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA0048: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA004C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA004C size=4
    let mut pc: u32 = 0x82EA004C;
    'dispatch: loop {
        match pc {
            0x82EA004C => {
    //   block [0x82EA004C..0x82EA0050)
	// 82EA004C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0050 size=16
    let mut pc: u32 = 0x82EA0050;
    'dispatch: loop {
        match pc {
            0x82EA0050 => {
    //   block [0x82EA0050..0x82EA0060)
	// 82EA0050: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA0054: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA0058: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EA005C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EA0060 size=280
    let mut pc: u32 = 0x82EA0060;
    'dispatch: loop {
        match pc {
            0x82EA0060 => {
    //   block [0x82EA0060..0x82EA0178)
	// 82EA0060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0064: 483080FD  bl 0x831a8160
	ctx.lr = 0x82EA0068;
	sub_831A8130(ctx, base);
	// 82EA0068: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA006C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA0070: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82EA0074: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EA0078: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA007C: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA0080: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA0084: 7F6AE82E  lwzx r27, r10, r29
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EA0088: 409A0028  bne cr6, 0x82ea00b0
	if !ctx.cr[6].eq {
	pc = 0x82EA00B0; continue 'dispatch;
	}
	// 82EA008C: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA0090: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EA0094: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EA0098: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA009C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA00A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA00A4: 4E800421  bctrl
	ctx.lr = 0x82EA00A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA00A8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EA00AC: 48308104  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82EA00B0: 39440007  addi r10, r4, 7
	ctx.r[10].s64 = ctx.r[4].s64 + 7;
	// 82EA00B4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82EA00B8: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA00BC: 7F890194  addze r28, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[28].s64 = tmp.s64;
	// 82EA00C0: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EA00C4: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA00C8: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EA00CC: 4099008C  ble cr6, 0x82ea0158
	if !ctx.cr[6].gt {
	pc = 0x82EA0158; continue 'dispatch;
	}
	// 82EA00D0: 7D7CF050  subf r11, r28, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 82EA00D4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82EA00D8: 4099000C  ble cr6, 0x82ea00e4
	if !ctx.cr[6].gt {
	pc = 0x82EA00E4; continue 'dispatch;
	}
	// 82EA00DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EA00E0: 48000010  b 0x82ea00f0
	pc = 0x82EA00F0; continue 'dispatch;
	// 82EA00E4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82EA00E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA00EC: 40990044  ble cr6, 0x82ea0130
	if !ctx.cr[6].gt {
	pc = 0x82EA0130; continue 'dispatch;
	}
	// 82EA00F0: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82EA00F4: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82EA00F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA00FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA0100: 419A001C  beq cr6, 0x82ea011c
	if ctx.cr[6].eq {
	pc = 0x82EA011C; continue 'dispatch;
	}
	// 82EA0104: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA0108: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EA010C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EA0110: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0114: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EA0118: 48000008  b 0x82ea0120
	pc = 0x82EA0120; continue 'dispatch;
	// 82EA011C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA0120: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA0124: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA0128: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82EA012C: 4082FFCC  bne 0x82ea00f8
	if !ctx.cr[0].eq {
	pc = 0x82EA00F8; continue 'dispatch;
	}
	// 82EA0130: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA0134: 7FC5F050  subf r30, r5, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[5].s64;
	// 82EA0138: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EA013C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA0140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0144: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA0148: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA014C: 4E800421  bctrl
	ctx.lr = 0x82EA0150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA0150: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EA0154: 4199FF7C  bgt cr6, 0x82ea00d0
	if ctx.cr[6].gt {
	pc = 0x82EA00D0; continue 'dispatch;
	}
	// 82EA0158: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA015C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0160: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EA0164: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA0168: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA016C: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82EA0170: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EA0174: 4830803C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0178 size=192
    let mut pc: u32 = 0x82EA0178;
    'dispatch: loop {
        match pc {
            0x82EA0178 => {
    //   block [0x82EA0178..0x82EA0238)
	// 82EA0178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA017C: 48307FED  bl 0x831a8168
	ctx.lr = 0x82EA0180;
	sub_831A8130(ctx, base);
	// 82EA0180: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA0184: 39640007  addi r11, r4, 7
	ctx.r[11].s64 = ctx.r[4].s64 + 7;
	// 82EA0188: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA018C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA0190: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82EA0194: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EA0198: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA019C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA01A0: 7F89E82E  lwzx r28, r9, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EA01A4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA01A8: 40990088  ble cr6, 0x82ea0230
	if !ctx.cr[6].gt {
	pc = 0x82EA0230; continue 'dispatch;
	}
	// 82EA01AC: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82EA01B0: 4099000C  ble cr6, 0x82ea01bc
	if !ctx.cr[6].gt {
	pc = 0x82EA01BC; continue 'dispatch;
	}
	// 82EA01B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EA01B8: 48000010  b 0x82ea01c8
	pc = 0x82EA01C8; continue 'dispatch;
	// 82EA01BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA01C0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA01C4: 40990044  ble cr6, 0x82ea0208
	if !ctx.cr[6].gt {
	pc = 0x82EA0208; continue 'dispatch;
	}
	// 82EA01C8: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82EA01CC: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82EA01D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA01D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA01D8: 419A001C  beq cr6, 0x82ea01f4
	if ctx.cr[6].eq {
	pc = 0x82EA01F4; continue 'dispatch;
	}
	// 82EA01DC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA01E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EA01E4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EA01E8: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA01EC: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EA01F0: 48000008  b 0x82ea01f8
	pc = 0x82EA01F8; continue 'dispatch;
	// 82EA01F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA01F8: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA01FC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA0200: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82EA0204: 4082FFCC  bne 0x82ea01d0
	if !ctx.cr[0].eq {
	pc = 0x82EA01D0; continue 'dispatch;
	}
	// 82EA0208: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA020C: 7FC5F050  subf r30, r5, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[5].s64;
	// 82EA0210: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA0214: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA0218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA021C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA0220: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA0224: 4E800421  bctrl
	ctx.lr = 0x82EA0228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA0228: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA022C: 4199FF80  bgt cr6, 0x82ea01ac
	if ctx.cr[6].gt {
	pc = 0x82EA01AC; continue 'dispatch;
	}
	// 82EA0230: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA0234: 48307F84  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0238 size=36
    let mut pc: u32 = 0x82EA0238;
    'dispatch: loop {
        match pc {
            0x82EA0238 => {
    //   block [0x82EA0238..0x82EA025C)
	// 82EA0238: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82EA023C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0240: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EA0244: 38CBFFF0  addi r6, r11, -0x10
	ctx.r[6].s64 = ctx.r[11].s64 + -16;
	// 82EA0248: 816BFFF4  lwz r11, -0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EA024C: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA0250: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 82EA0254: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA0258: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0260 size=52
    let mut pc: u32 = 0x82EA0260;
    'dispatch: loop {
        match pc {
            0x82EA0260 => {
    //   block [0x82EA0260..0x82EA0294)
	// 82EA0260: 548B073E  clrlwi r11, r4, 0x1c
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000000Fu64;
	// 82EA0264: 90A30030  stw r5, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82EA0268: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EA026C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA0270: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82EA0274: 419A0020  beq cr6, 0x82ea0294
	if ctx.cr[6].eq {
		sub_82EA0294(ctx, base);
		return;
	}
	// 82EA0278: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82EA027C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82EA0280: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EA0284: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EA0288: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82EA028C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EA0290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0294(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0294 size=16
    let mut pc: u32 = 0x82EA0294;
    'dispatch: loop {
        match pc {
            0x82EA0294 => {
    //   block [0x82EA0294..0x82EA02A4)
	// 82EA0294: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 82EA0298: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82EA029C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EA02A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA02A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA02A8 size=104
    let mut pc: u32 = 0x82EA02A8;
    'dispatch: loop {
        match pc {
            0x82EA02A8 => {
    //   block [0x82EA02A8..0x82EA0310)
	// 82EA02A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA02AC: 48307EC1  bl 0x831a816c
	ctx.lr = 0x82EA02B0;
	sub_831A8130(ctx, base);
	// 82EA02B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA02B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA02B8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA02BC: 419A0010  beq cr6, 0x82ea02cc
	if ctx.cr[6].eq {
	pc = 0x82EA02CC; continue 'dispatch;
	}
	// 82EA02C0: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA02C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA02C8: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EA02CC: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA02D0: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 82EA02D4: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA02D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA02DC: 419A0028  beq cr6, 0x82ea0304
	if ctx.cr[6].eq {
	pc = 0x82EA0304; continue 'dispatch;
	}
	// 82EA02E0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA02E4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA02E8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EA02EC: 40820018  bne 0x82ea0304
	if !ctx.cr[0].eq {
	pc = 0x82EA0304; continue 'dispatch;
	}
	// 82EA02F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA02F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA02F8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA02FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA0300: 4E800421  bctrl
	ctx.lr = 0x82EA0304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA0304: 7FBFF12E  stwx r29, r31, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 82EA0308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA030C: 48307EB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0310 size=772
    let mut pc: u32 = 0x82EA0310;
    'dispatch: loop {
        match pc {
            0x82EA0310 => {
    //   block [0x82EA0310..0x82EA0614)
	// 82EA0310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0314: 48307E55  bl 0x831a8168
	ctx.lr = 0x82EA0318;
	sub_831A8130(ctx, base);
	// 82EA0318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA031C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA0320: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA0324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA0328: 392BBB5C  addi r9, r11, -0x44a4
	ctx.r[9].s64 = ctx.r[11].s64 + -17572;
	// 82EA032C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EA0330: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82EA0334: 915C0014  stw r10, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EA0338: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA033C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA0340: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA0344: 93FC0020  stw r31, 0x20(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82EA0348: 397C0038  addi r11, r28, 0x38
	ctx.r[11].s64 = ctx.r[28].s64 + 56;
	// 82EA034C: 93FC0024  stw r31, 0x24(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 82EA0350: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82EA0354: 911C0028  stw r8, 0x28(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 82EA0358: 93FC002C  stw r31, 0x2c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 82EA035C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EA0360: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA0364: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EA0368: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EA036C: 4080FFF0  bge 0x82ea035c
	if !ctx.cr[0].lt {
	pc = 0x82EA035C; continue 'dispatch;
	}
	// 82EA0370: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0374: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA0378: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EA037C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA0380: 4E800421  bctrl
	ctx.lr = 0x82EA0384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA0384: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EA0388: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA038C: 419A0008  beq cr6, 0x82ea0394
	if ctx.cr[6].eq {
	pc = 0x82EA0394; continue 'dispatch;
	}
	// 82EA0390: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82EA0394: 3D600FFF  lis r11, 0xfff
	ctx.r[11].s64 = 268369920;
	// 82EA0398: 93BC0010  stw r29, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82EA039C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82EA03A0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA03A4: 40980008  bge cr6, 0x82ea03ac
	if !ctx.cr[6].lt {
	pc = 0x82EA03AC; continue 'dispatch;
	}
	// 82EA03A8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EA03AC: 917C0034  stw r11, 0x34(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82EA03B0: 395C0104  addi r10, r28, 0x104
	ctx.r[10].s64 = ctx.r[28].s64 + 260;
	// 82EA03B4: 2F1F0008  cmpwi cr6, r31, 8
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8, &mut ctx.xer);
	// 82EA03B8: 4199000C  bgt cr6, 0x82ea03c4
	if ctx.cr[6].gt {
	pc = 0x82EA03C4; continue 'dispatch;
	}
	// 82EA03BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA03C0: 480000FC  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA03C4: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 82EA03C8: 4199000C  bgt cr6, 0x82ea03d4
	if ctx.cr[6].gt {
	pc = 0x82EA03D4; continue 'dispatch;
	}
	// 82EA03CC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EA03D0: 480000EC  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA03D4: 2F1F0020  cmpwi cr6, r31, 0x20
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32, &mut ctx.xer);
	// 82EA03D8: 4199000C  bgt cr6, 0x82ea03e4
	if ctx.cr[6].gt {
	pc = 0x82EA03E4; continue 'dispatch;
	}
	// 82EA03DC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82EA03E0: 480000DC  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA03E4: 2F1F0030  cmpwi cr6, r31, 0x30
	ctx.cr[6].compare_i32(ctx.r[31].s32, 48, &mut ctx.xer);
	// 82EA03E8: 4199000C  bgt cr6, 0x82ea03f4
	if ctx.cr[6].gt {
	pc = 0x82EA03F4; continue 'dispatch;
	}
	// 82EA03EC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EA03F0: 480000CC  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA03F4: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 82EA03F8: 4199000C  bgt cr6, 0x82ea0404
	if ctx.cr[6].gt {
	pc = 0x82EA0404; continue 'dispatch;
	}
	// 82EA03FC: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82EA0400: 480000BC  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0404: 2F1F0060  cmpwi cr6, r31, 0x60
	ctx.cr[6].compare_i32(ctx.r[31].s32, 96, &mut ctx.xer);
	// 82EA0408: 4199000C  bgt cr6, 0x82ea0414
	if ctx.cr[6].gt {
	pc = 0x82EA0414; continue 'dispatch;
	}
	// 82EA040C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82EA0410: 480000AC  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0414: 2F1F0080  cmpwi cr6, r31, 0x80
	ctx.cr[6].compare_i32(ctx.r[31].s32, 128, &mut ctx.xer);
	// 82EA0418: 4199000C  bgt cr6, 0x82ea0424
	if ctx.cr[6].gt {
	pc = 0x82EA0424; continue 'dispatch;
	}
	// 82EA041C: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82EA0420: 4800009C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0424: 2F1F00A0  cmpwi cr6, r31, 0xa0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 160, &mut ctx.xer);
	// 82EA0428: 4199000C  bgt cr6, 0x82ea0434
	if ctx.cr[6].gt {
	pc = 0x82EA0434; continue 'dispatch;
	}
	// 82EA042C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82EA0430: 4800008C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0434: 2F1F00C0  cmpwi cr6, r31, 0xc0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 192, &mut ctx.xer);
	// 82EA0438: 4199000C  bgt cr6, 0x82ea0444
	if ctx.cr[6].gt {
	pc = 0x82EA0444; continue 'dispatch;
	}
	// 82EA043C: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82EA0440: 4800007C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0444: 2F1F0100  cmpwi cr6, r31, 0x100
	ctx.cr[6].compare_i32(ctx.r[31].s32, 256, &mut ctx.xer);
	// 82EA0448: 4199000C  bgt cr6, 0x82ea0454
	if ctx.cr[6].gt {
	pc = 0x82EA0454; continue 'dispatch;
	}
	// 82EA044C: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82EA0450: 4800006C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0454: 2F1F0140  cmpwi cr6, r31, 0x140
	ctx.cr[6].compare_i32(ctx.r[31].s32, 320, &mut ctx.xer);
	// 82EA0458: 4199000C  bgt cr6, 0x82ea0464
	if ctx.cr[6].gt {
	pc = 0x82EA0464; continue 'dispatch;
	}
	// 82EA045C: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82EA0460: 4800005C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0464: 2F1F0200  cmpwi cr6, r31, 0x200
	ctx.cr[6].compare_i32(ctx.r[31].s32, 512, &mut ctx.xer);
	// 82EA0468: 4199000C  bgt cr6, 0x82ea0474
	if ctx.cr[6].gt {
	pc = 0x82EA0474; continue 'dispatch;
	}
	// 82EA046C: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82EA0470: 4800004C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0474: 2F1F0400  cmpwi cr6, r31, 0x400
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1024, &mut ctx.xer);
	// 82EA0478: 4199000C  bgt cr6, 0x82ea0484
	if ctx.cr[6].gt {
	pc = 0x82EA0484; continue 'dispatch;
	}
	// 82EA047C: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82EA0480: 4800003C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0484: 2F1F0800  cmpwi cr6, r31, 0x800
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2048, &mut ctx.xer);
	// 82EA0488: 4199000C  bgt cr6, 0x82ea0494
	if ctx.cr[6].gt {
	pc = 0x82EA0494; continue 'dispatch;
	}
	// 82EA048C: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 82EA0490: 4800002C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA0494: 2F1F1000  cmpwi cr6, r31, 0x1000
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4096, &mut ctx.xer);
	// 82EA0498: 4199000C  bgt cr6, 0x82ea04a4
	if ctx.cr[6].gt {
	pc = 0x82EA04A4; continue 'dispatch;
	}
	// 82EA049C: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82EA04A0: 4800001C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA04A4: 2F1F2000  cmpwi cr6, r31, 0x2000
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8192, &mut ctx.xer);
	// 82EA04A8: 4199000C  bgt cr6, 0x82ea04b4
	if ctx.cr[6].gt {
	pc = 0x82EA04B4; continue 'dispatch;
	}
	// 82EA04AC: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82EA04B0: 4800000C  b 0x82ea04bc
	pc = 0x82EA04BC; continue 'dispatch;
	// 82EA04B4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA04B8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA04BC: 392B0030  addi r9, r11, 0x30
	ctx.r[9].s64 = ctx.r[11].s64 + 48;
	// 82EA04C0: 7D6AF9AE  stbx r11, r10, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 82EA04C4: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA04C8: 7FE7E12E  stwx r31, r7, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[28].u32), ctx.r[31].u32) };
	// 82EA04CC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EA04D0: 2F1F0200  cmpwi cr6, r31, 0x200
	ctx.cr[6].compare_i32(ctx.r[31].s32, 512, &mut ctx.xer);
	// 82EA04D4: 4099FEE0  ble cr6, 0x82ea03b4
	if !ctx.cr[6].gt {
	pc = 0x82EA03B4; continue 'dispatch;
	}
	// 82EA04D8: 39400400  li r10, 0x400
	ctx.r[10].s64 = 1024;
	// 82EA04DC: 393C0308  addi r9, r28, 0x308
	ctx.r[9].s64 = ctx.r[28].s64 + 776;
	// 82EA04E0: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82EA04E4: 4199000C  bgt cr6, 0x82ea04f0
	if ctx.cr[6].gt {
	pc = 0x82EA04F0; continue 'dispatch;
	}
	// 82EA04E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA04EC: 480000FC  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA04F0: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82EA04F4: 4199000C  bgt cr6, 0x82ea0500
	if ctx.cr[6].gt {
	pc = 0x82EA0500; continue 'dispatch;
	}
	// 82EA04F8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EA04FC: 480000EC  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0500: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82EA0504: 4199000C  bgt cr6, 0x82ea0510
	if ctx.cr[6].gt {
	pc = 0x82EA0510; continue 'dispatch;
	}
	// 82EA0508: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82EA050C: 480000DC  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0510: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 82EA0514: 4199000C  bgt cr6, 0x82ea0520
	if ctx.cr[6].gt {
	pc = 0x82EA0520; continue 'dispatch;
	}
	// 82EA0518: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EA051C: 480000CC  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0520: 2F0A0040  cmpwi cr6, r10, 0x40
	ctx.cr[6].compare_i32(ctx.r[10].s32, 64, &mut ctx.xer);
	// 82EA0524: 4199000C  bgt cr6, 0x82ea0530
	if ctx.cr[6].gt {
	pc = 0x82EA0530; continue 'dispatch;
	}
	// 82EA0528: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82EA052C: 480000BC  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0530: 2F0A0060  cmpwi cr6, r10, 0x60
	ctx.cr[6].compare_i32(ctx.r[10].s32, 96, &mut ctx.xer);
	// 82EA0534: 4199000C  bgt cr6, 0x82ea0540
	if ctx.cr[6].gt {
	pc = 0x82EA0540; continue 'dispatch;
	}
	// 82EA0538: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82EA053C: 480000AC  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0540: 2F0A0080  cmpwi cr6, r10, 0x80
	ctx.cr[6].compare_i32(ctx.r[10].s32, 128, &mut ctx.xer);
	// 82EA0544: 4199000C  bgt cr6, 0x82ea0550
	if ctx.cr[6].gt {
	pc = 0x82EA0550; continue 'dispatch;
	}
	// 82EA0548: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82EA054C: 4800009C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0550: 2F0A00A0  cmpwi cr6, r10, 0xa0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 160, &mut ctx.xer);
	// 82EA0554: 4199000C  bgt cr6, 0x82ea0560
	if ctx.cr[6].gt {
	pc = 0x82EA0560; continue 'dispatch;
	}
	// 82EA0558: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82EA055C: 4800008C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0560: 2F0A00C0  cmpwi cr6, r10, 0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 192, &mut ctx.xer);
	// 82EA0564: 4199000C  bgt cr6, 0x82ea0570
	if ctx.cr[6].gt {
	pc = 0x82EA0570; continue 'dispatch;
	}
	// 82EA0568: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82EA056C: 4800007C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0570: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 82EA0574: 4199000C  bgt cr6, 0x82ea0580
	if ctx.cr[6].gt {
	pc = 0x82EA0580; continue 'dispatch;
	}
	// 82EA0578: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82EA057C: 4800006C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0580: 2F0A0140  cmpwi cr6, r10, 0x140
	ctx.cr[6].compare_i32(ctx.r[10].s32, 320, &mut ctx.xer);
	// 82EA0584: 4199000C  bgt cr6, 0x82ea0590
	if ctx.cr[6].gt {
	pc = 0x82EA0590; continue 'dispatch;
	}
	// 82EA0588: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82EA058C: 4800005C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA0590: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 82EA0594: 4199000C  bgt cr6, 0x82ea05a0
	if ctx.cr[6].gt {
	pc = 0x82EA05A0; continue 'dispatch;
	}
	// 82EA0598: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82EA059C: 4800004C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA05A0: 2F0A0400  cmpwi cr6, r10, 0x400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1024, &mut ctx.xer);
	// 82EA05A4: 4199000C  bgt cr6, 0x82ea05b0
	if ctx.cr[6].gt {
	pc = 0x82EA05B0; continue 'dispatch;
	}
	// 82EA05A8: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82EA05AC: 4800003C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA05B0: 2F0A0800  cmpwi cr6, r10, 0x800
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2048, &mut ctx.xer);
	// 82EA05B4: 4199000C  bgt cr6, 0x82ea05c0
	if ctx.cr[6].gt {
	pc = 0x82EA05C0; continue 'dispatch;
	}
	// 82EA05B8: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 82EA05BC: 4800002C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA05C0: 2F0A1000  cmpwi cr6, r10, 0x1000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4096, &mut ctx.xer);
	// 82EA05C4: 4199000C  bgt cr6, 0x82ea05d0
	if ctx.cr[6].gt {
	pc = 0x82EA05D0; continue 'dispatch;
	}
	// 82EA05C8: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82EA05CC: 4800001C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA05D0: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82EA05D4: 4199000C  bgt cr6, 0x82ea05e0
	if ctx.cr[6].gt {
	pc = 0x82EA05E0; continue 'dispatch;
	}
	// 82EA05D8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82EA05DC: 4800000C  b 0x82ea05e8
	pc = 0x82EA05E8; continue 'dispatch;
	// 82EA05E0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA05E4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA05E8: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82EA05EC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA05F0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82EA05F4: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA05F8: 7D47E12E  stwx r10, r7, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82EA05FC: 394A0400  addi r10, r10, 0x400
	ctx.r[10].s64 = ctx.r[10].s64 + 1024;
	// 82EA0600: 2F0A2400  cmpwi cr6, r10, 0x2400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 9216, &mut ctx.xer);
	// 82EA0604: 4198FEDC  blt cr6, 0x82ea04e0
	if ctx.cr[6].lt {
	pc = 0x82EA04E0; continue 'dispatch;
	}
	// 82EA0608: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA060C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA0610: 48307BA8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0618 size=68
    let mut pc: u32 = 0x82EA0618;
    'dispatch: loop {
        match pc {
            0x82EA0618 => {
    //   block [0x82EA0618..0x82EA065C)
	// 82EA0618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA061C: 48307B51  bl 0x831a816c
	ctx.lr = 0x82EA0620;
	sub_831A8130(ctx, base);
	// 82EA0620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA0624: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA0628: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82EA062C: 3BDD00BC  addi r30, r29, 0xbc
	ctx.r[30].s64 = ctx.r[29].s64 + 188;
	// 82EA0630: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0634: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA0638: 419A0010  beq cr6, 0x82ea0648
	if ctx.cr[6].eq {
	pc = 0x82EA0648; continue 'dispatch;
	}
	// 82EA063C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA0640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA0644: 4BFFFB35  bl 0x82ea0178
	ctx.lr = 0x82EA0648;
	sub_82EA0178(ctx, base);
	// 82EA0648: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EA064C: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82EA0650: 4080FFE0  bge 0x82ea0630
	if !ctx.cr[0].lt {
	pc = 0x82EA0630; continue 'dispatch;
	}
	// 82EA0654: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA0658: 48307B64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0660 size=204
    let mut pc: u32 = 0x82EA0660;
    'dispatch: loop {
        match pc {
            0x82EA0660 => {
    //   block [0x82EA0660..0x82EA072C)
	// 82EA0660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0664: 48307B09  bl 0x831a816c
	ctx.lr = 0x82EA0668;
	sub_831A8130(ctx, base);
	// 82EA0668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA066C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA0670: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA0674: 83DF0034  lwz r30, 0x34(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA0678: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA067C: 409A002C  bne cr6, 0x82ea06a8
	if !ctx.cr[6].eq {
	pc = 0x82EA06A8; continue 'dispatch;
	}
	// 82EA0680: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA0684: 397D0030  addi r11, r29, 0x30
	ctx.r[11].s64 = ctx.r[29].s64 + 48;
	// 82EA0688: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA068C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0690: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA0694: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA0698: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA069C: 4E800421  bctrl
	ctx.lr = 0x82EA06A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA06A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA06A4: 48307B18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EA06A8: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82EA06AC: 41980008  blt cr6, 0x82ea06b4
	if ctx.cr[6].lt {
	pc = 0x82EA06B4; continue 'dispatch;
	}
	// 82EA06B0: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82EA06B4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA06B8: 397D0030  addi r11, r29, 0x30
	ctx.r[11].s64 = ctx.r[29].s64 + 48;
	// 82EA06BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA06C0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA06C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA06C8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA06CC: 7CCAF82E  lwzx r6, r10, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA06D0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA06D4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA06D8: 4E800421  bctrl
	ctx.lr = 0x82EA06DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA06DC: 38FD0007  addi r7, r29, 7
	ctx.r[7].s64 = ctx.r[29].s64 + 7;
	// 82EA06E0: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82EA06E4: 54EB1838  slwi r11, r7, 3
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA06E8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA06EC: 40990034  ble cr6, 0x82ea0720
	if !ctx.cr[6].gt {
	pc = 0x82EA0720; continue 'dispatch;
	}
	// 82EA06F0: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82EA06F4: 395EFFFF  addi r10, r30, -1
	ctx.r[10].s64 = ctx.r[30].s64 + -1;
	// 82EA06F8: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA06FC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA0700: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0704: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82EA0708: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA070C: 38A80001  addi r5, r8, 1
	ctx.r[5].s64 = ctx.r[8].s64 + 1;
	// 82EA0710: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA0714: 90C70000  stw r6, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EA0718: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EA071C: 4082FFDC  bne 0x82ea06f8
	if !ctx.cr[0].eq {
	pc = 0x82EA06F8; continue 'dispatch;
	}
	// 82EA0720: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA0724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA0728: 48307A94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0730 size=32
    let mut pc: u32 = 0x82EA0730;
    'dispatch: loop {
        match pc {
            0x82EA0730 => {
    //   block [0x82EA0730..0x82EA0750)
	// 82EA0730: 2F042000  cmpwi cr6, r4, 0x2000
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8192, &mut ctx.xer);
	// 82EA0734: 41990068  bgt cr6, 0x82ea079c
	if ctx.cr[6].gt {
		sub_82EA079C(ctx, base);
		return;
	}
	// 82EA0738: 2F040200  cmpwi cr6, r4, 0x200
	ctx.cr[6].compare_i32(ctx.r[4].s32, 512, &mut ctx.xer);
	// 82EA073C: 41990014  bgt cr6, 0x82ea0750
	if ctx.cr[6].gt {
		sub_82EA0750(ctx, base);
		return;
	}
	// 82EA0740: 7D641A14  add r11, r4, r3
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 82EA0744: 894B0104  lbz r10, 0x104(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82EA0748: 7D440774  extsb r4, r10
	ctx.r[4].s64 = ctx.r[10].s8 as i64;
	// 82EA074C: 48000018  b 0x82ea0764
	sub_82EA0750(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0750 size=72
    let mut pc: u32 = 0x82EA0750;
    'dispatch: loop {
        match pc {
            0x82EA0750 => {
    //   block [0x82EA0750..0x82EA0798)
	// 82EA0750: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EA0754: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82EA0758: 394B00C2  addi r10, r11, 0xc2
	ctx.r[10].s64 = ctx.r[11].s64 + 194;
	// 82EA075C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA0760: 7C89182E  lwzx r4, r9, r3
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EA0764: 39640007  addi r11, r4, 7
	ctx.r[11].s64 = ctx.r[4].s64 + 7;
	// 82EA0768: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA076C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EA0770: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0774: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA0778: 419A0020  beq cr6, 0x82ea0798
	if ctx.cr[6].eq {
		sub_82EA0798(ctx, base);
		return;
	}
	// 82EA077C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA0780: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82EA0784: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82EA0788: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA078C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0790: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA0794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0798 size=4
    let mut pc: u32 = 0x82EA0798;
    'dispatch: loop {
        match pc {
            0x82EA0798 => {
    //   block [0x82EA0798..0x82EA079C)
	// 82EA0798: 4BFFFEC8  b 0x82ea0660
	sub_82EA0660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA079C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA079C size=20
    let mut pc: u32 = 0x82EA079C;
    'dispatch: loop {
        match pc {
            0x82EA079C => {
    //   block [0x82EA079C..0x82EA07B0)
	// 82EA079C: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA07A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA07A4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA07A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA07AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA07B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA07B0 size=8
    let mut pc: u32 = 0x82EA07B0;
    'dispatch: loop {
        match pc {
            0x82EA07B0 => {
    //   block [0x82EA07B0..0x82EA07B8)
	// 82EA07B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA07B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA07B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA07B8 size=32
    let mut pc: u32 = 0x82EA07B8;
    'dispatch: loop {
        match pc {
            0x82EA07B8 => {
    //   block [0x82EA07B8..0x82EA07D8)
	// 82EA07B8: 2F052000  cmpwi cr6, r5, 0x2000
	ctx.cr[6].compare_i32(ctx.r[5].s32, 8192, &mut ctx.xer);
	// 82EA07BC: 4199007C  bgt cr6, 0x82ea0838
	if ctx.cr[6].gt {
		sub_82EA0838(ctx, base);
		return;
	}
	// 82EA07C0: 2F050200  cmpwi cr6, r5, 0x200
	ctx.cr[6].compare_i32(ctx.r[5].s32, 512, &mut ctx.xer);
	// 82EA07C4: 41990014  bgt cr6, 0x82ea07d8
	if ctx.cr[6].gt {
		sub_82EA07D8(ctx, base);
		return;
	}
	// 82EA07C8: 7D651A14  add r11, r5, r3
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[3].u64;
	// 82EA07CC: 894B0104  lbz r10, 0x104(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82EA07D0: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82EA07D4: 48000018  b 0x82ea07ec
	sub_82EA07D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA07D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA07D8 size=56
    let mut pc: u32 = 0x82EA07D8;
    'dispatch: loop {
        match pc {
            0x82EA07D8 => {
    //   block [0x82EA07D8..0x82EA0810)
	// 82EA07D8: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82EA07DC: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82EA07E0: 394B00C2  addi r10, r11, 0xc2
	ctx.r[10].s64 = ctx.r[11].s64 + 194;
	// 82EA07E4: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA07E8: 7D69182E  lwzx r11, r9, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EA07EC: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA07F0: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA07F4: 7D0A1A14  add r8, r10, r3
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82EA07F8: 80E8003C  lwz r7, 0x3c(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EA07FC: 7F074800  cmpw cr6, r7, r9
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EA0800: 41980010  blt cr6, 0x82ea0810
	if ctx.cr[6].lt {
		sub_82EA0810(ctx, base);
		return;
	}
	// 82EA0804: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EA0808: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EA080C: 4BFFF854  b 0x82ea0060
	sub_82EA0060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0810 size=40
    let mut pc: u32 = 0x82EA0810;
    'dispatch: loop {
        match pc {
            0x82EA0810 => {
    //   block [0x82EA0810..0x82EA0838)
	// 82EA0810: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 82EA0814: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA0818: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EA081C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA0820: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0824: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82EA0828: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EA082C: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA0830: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EA0834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0838 size=20
    let mut pc: u32 = 0x82EA0838;
    'dispatch: loop {
        match pc {
            0x82EA0838 => {
    //   block [0x82EA0838..0x82EA084C)
	// 82EA0838: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA083C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0840: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA0844: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA0848: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0850 size=220
    let mut pc: u32 = 0x82EA0850;
    'dispatch: loop {
        match pc {
            0x82EA0850 => {
    //   block [0x82EA0850..0x82EA092C)
	// 82EA0850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0854: 48307919  bl 0x831a816c
	ctx.lr = 0x82EA0858;
	sub_831A8130(ctx, base);
	// 82EA0858: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA085C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA0860: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA0864: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA0868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA086C: 409A0060  bne cr6, 0x82ea08cc
	if !ctx.cr[6].eq {
	pc = 0x82EA08CC; continue 'dispatch;
	}
	// 82EA0870: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82EA0874: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EA0878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA087C: 48005B15  bl 0x82ea6390
	ctx.lr = 0x82EA0880;
	sub_82EA6390(ctx, base);
	// 82EA0880: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA0884: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA0888: 388BE820  addi r4, r11, -0x17e0
	ctx.r[4].s64 = ctx.r[11].s64 + -6112;
	// 82EA088C: 48004F4D  bl 0x82ea57d8
	ctx.lr = 0x82EA0890;
	sub_82EA57D8(ctx, base);
	// 82EA0890: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82EA0894: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA0898: 3CA02BEB  lis r5, 0x2beb
	ctx.r[5].s64 = 736821248;
	// 82EA089C: 3900012D  li r8, 0x12d
	ctx.r[8].s64 = 301;
	// 82EA08A0: 38E9E804  addi r7, r9, -0x17fc
	ctx.r[7].s64 = ctx.r[9].s64 + -6140;
	// 82EA08A4: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82EA08A8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EA08AC: 60A5BA62  ori r5, r5, 0xba62
	ctx.r[5].u64 = ctx.r[5].u64 | 47714;
	// 82EA08B0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82EA08B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA08B8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA08BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA08C0: 4E800421  bctrl
	ctx.lr = 0x82EA08C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA08C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA08C8: 48005551  bl 0x82ea5e18
	ctx.lr = 0x82EA08CC;
	sub_82EA5E18(ctx, base);
	// 82EA08CC: 3BDD0400  addi r30, r29, 0x400
	ctx.r[30].s64 = ctx.r[29].s64 + 1024;
	// 82EA08D0: 2F1E1000  cmpwi cr6, r30, 0x1000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4096, &mut ctx.xer);
	// 82EA08D4: 41990008  bgt cr6, 0x82ea08dc
	if ctx.cr[6].gt {
	pc = 0x82EA08DC; continue 'dispatch;
	}
	// 82EA08D8: 3BC01000  li r30, 0x1000
	ctx.r[30].s64 = 4096;
	// 82EA08DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA08E0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA08E4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EA08E8: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82EA08EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA08F0: 4BFFFE41  bl 0x82ea0730
	ctx.lr = 0x82EA08F4;
	sub_82EA0730(ctx, base);
	// 82EA08F4: E93F0020  ld r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 82EA08F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA08FC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EA0900: 7D03EA14  add r8, r3, r29
	ctx.r[8].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82EA0904: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EA0908: 7CE3F214  add r7, r3, r30
	ctx.r[7].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82EA090C: E8DF0028  ld r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 82EA0910: F8CB0008  std r6, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82EA0914: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82EA0918: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82EA091C: 90FF002C  stw r7, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82EA0920: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EA0924: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82EA0928: 48307894  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0930 size=48
    let mut pc: u32 = 0x82EA0930;
    'dispatch: loop {
        match pc {
            0x82EA0930 => {
    //   block [0x82EA0930..0x82EA0960)
	// 82EA0930: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82EA0934: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA0938: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EA093C: 8109002C  lwz r8, 0x2c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EA0940: 388BFFF0  addi r4, r11, -0x10
	ctx.r[4].s64 = ctx.r[11].s64 + -16;
	// 82EA0944: 7D4B4050  subf r10, r11, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82EA0948: E8EBFFF0  ld r7, -0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	// 82EA094C: 38AA0010  addi r5, r10, 0x10
	ctx.r[5].s64 = ctx.r[10].s64 + 16;
	// 82EA0950: F8E90020  std r7, 0x20(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[7].u64 ) };
	// 82EA0954: E96BFFF8  ld r11, -8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	// 82EA0958: F9690028  std r11, 0x28(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 82EA095C: 4BFFFE54  b 0x82ea07b0
	sub_82EA07B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0960 size=88
    let mut pc: u32 = 0x82EA0960;
    'dispatch: loop {
        match pc {
            0x82EA0960 => {
    //   block [0x82EA0960..0x82EA09B8)
	// 82EA0960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA0968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA096C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA0970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA0974: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA0978: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA097C: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EA0980: 4BFFFDB1  bl 0x82ea0730
	ctx.lr = 0x82EA0984;
	sub_82EA0730(ctx, base);
	// 82EA0984: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA0988: 3D400234  lis r10, 0x234
	ctx.r[10].s64 = 36962304;
	// 82EA098C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EA0990: 61495656  ori r9, r10, 0x5656
	ctx.r[9].u64 = ctx.r[10].u64 | 22102;
	// 82EA0994: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA0998: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EA099C: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EA09A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA09A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA09A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA09AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA09B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA09B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA09B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA09B8 size=12
    let mut pc: u32 = 0x82EA09B8;
    'dispatch: loop {
        match pc {
            0x82EA09B8 => {
    //   block [0x82EA09B8..0x82EA09C4)
	// 82EA09B8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA09BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA09C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA09C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA09C4 size=32
    let mut pc: u32 = 0x82EA09C4;
    'dispatch: loop {
        match pc {
            0x82EA09C4 => {
    //   block [0x82EA09C4..0x82EA09E4)
	// 82EA09C4: 3D20DEAD  lis r9, -0x2153
	ctx.r[9].s64 = -559087616;
	// 82EA09C8: 814BFFF4  lwz r10, -0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EA09CC: 80CBFFF8  lwz r6, -8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA09D0: 388BFFF0  addi r4, r11, -0x10
	ctx.r[4].s64 = ctx.r[11].s64 + -16;
	// 82EA09D4: 6128BEEF  ori r8, r9, 0xbeef
	ctx.r[8].u64 = ctx.r[9].u64 | 48879;
	// 82EA09D8: 38AA0010  addi r5, r10, 0x10
	ctx.r[5].s64 = ctx.r[10].s64 + 16;
	// 82EA09DC: 910BFFF0  stw r8, -0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), ctx.r[8].u32 ) };
	// 82EA09E0: 4BFFFDD0  b 0x82ea07b0
	sub_82EA07B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA09E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA09E4 size=4
    let mut pc: u32 = 0x82EA09E4;
    'dispatch: loop {
        match pc {
            0x82EA09E4 => {
    //   block [0x82EA09E4..0x82EA09E8)
	// 82EA09E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA09E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA09E8 size=92
    let mut pc: u32 = 0x82EA09E8;
    'dispatch: loop {
        match pc {
            0x82EA09E8 => {
    //   block [0x82EA09E8..0x82EA0A44)
	// 82EA09E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA09EC: 48307781  bl 0x831a816c
	ctx.lr = 0x82EA09F0;
	sub_831A8130(ctx, base);
	// 82EA09F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA09F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA09F8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EA09FC: 7FDF2A14  add r30, r31, r5
	ctx.r[30].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 82EA0A00: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EA0A04: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82EA0A08: 4BFFFD29  bl 0x82ea0730
	ctx.lr = 0x82EA0A0C;
	sub_82EA0730(ctx, base);
	// 82EA0A0C: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EA0A10: 395FFFFF  addi r10, r31, -1
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	// 82EA0A14: 390B000F  addi r8, r11, 0xf
	ctx.r[8].s64 = ctx.r[11].s64 + 15;
	// 82EA0A18: 3D200234  lis r9, 0x234
	ctx.r[9].s64 = 36962304;
	// 82EA0A1C: 7D065078  andc r6, r8, r10
	ctx.r[6].u64 = ctx.r[8].u64 & !ctx.r[10].u64;
	// 82EA0A20: 61275656  ori r7, r9, 0x5656
	ctx.r[7].u64 = ctx.r[9].u64 | 22102;
	// 82EA0A24: 7CA33050  subf r5, r3, r6
	ctx.r[5].s64 = ctx.r[6].s64 - ctx.r[3].s64;
	// 82EA0A28: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82EA0A2C: 90E6FFF0  stw r7, -0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-16 as u32), ctx.r[7].u32 ) };
	// 82EA0A30: 93C6FFF4  stw r30, -0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-12 as u32), ctx.r[30].u32 ) };
	// 82EA0A34: 93A6FFF8  stw r29, -8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-8 as u32), ctx.r[29].u32 ) };
	// 82EA0A38: 90A6FFFC  stw r5, -4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-4 as u32), ctx.r[5].u32 ) };
	// 82EA0A3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA0A40: 4830777C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0A48 size=12
    let mut pc: u32 = 0x82EA0A48;
    'dispatch: loop {
        match pc {
            0x82EA0A48 => {
    //   block [0x82EA0A48..0x82EA0A54)
	// 82EA0A48: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA0A4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA0A50: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0A54 size=36
    let mut pc: u32 = 0x82EA0A54;
    'dispatch: loop {
        match pc {
            0x82EA0A54 => {
    //   block [0x82EA0A54..0x82EA0A78)
	// 82EA0A54: 3D20DEAD  lis r9, -0x2153
	ctx.r[9].s64 = -559087616;
	// 82EA0A58: 814BFFF4  lwz r10, -0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EA0A5C: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EA0A60: 6127BEEF  ori r7, r9, 0xbeef
	ctx.r[7].u64 = ctx.r[9].u64 | 48879;
	// 82EA0A64: 80CBFFF8  lwz r6, -8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA0A68: 38AA0010  addi r5, r10, 0x10
	ctx.r[5].s64 = ctx.r[10].s64 + 16;
	// 82EA0A6C: 90EBFFF0  stw r7, -0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), ctx.r[7].u32 ) };
	// 82EA0A70: 7C885850  subf r4, r8, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82EA0A74: 4BFFFD3C  b 0x82ea07b0
	sub_82EA07B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0A78 size=4
    let mut pc: u32 = 0x82EA0A78;
    'dispatch: loop {
        match pc {
            0x82EA0A78 => {
    //   block [0x82EA0A78..0x82EA0A7C)
	// 82EA0A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0A80 size=136
    let mut pc: u32 = 0x82EA0A80;
    'dispatch: loop {
        match pc {
            0x82EA0A80 => {
    //   block [0x82EA0A80..0x82EA0B08)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0B08 size=184
    let mut pc: u32 = 0x82EA0B08;
    'dispatch: loop {
        match pc {
            0x82EA0B08 => {
    //   block [0x82EA0B08..0x82EA0BC0)
	// 82EA0B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA0B10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA0B14: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA0B18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA0B1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA0B20: 48006631  bl 0x82ea7150
	ctx.lr = 0x82EA0B24;
	sub_82EA7150(ctx, base);
	// 82EA0B24: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA0B28: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA0B2C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82EA0B30: 390AFFA0  addi r8, r10, -0x60
	ctx.r[8].s64 = ctx.r[10].s64 + -96;
	// 82EA0B34: 38E96E60  addi r7, r9, 0x6e60
	ctx.r[7].s64 = ctx.r[9].s64 + 28256;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0BC0 size=56
    let mut pc: u32 = 0x82EA0BC0;
    'dispatch: loop {
        match pc {
            0x82EA0BC0 => {
    //   block [0x82EA0BC0..0x82EA0BF8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0BF8 size=200
    let mut pc: u32 = 0x82EA0BF8;
    'dispatch: loop {
        match pc {
            0x82EA0BF8 => {
    //   block [0x82EA0BF8..0x82EA0CC0)
	// 82EA0BF8: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 82EA0BFC: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0CC0 size=188
    let mut pc: u32 = 0x82EA0CC0;
    'dispatch: loop {
        match pc {
            0x82EA0CC0 => {
    //   block [0x82EA0CC0..0x82EA0D7C)
	// 82EA0CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA0CC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA0CCC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA0CD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA0CD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA0CD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA0CDC: 4800677D  bl 0x82ea7458
	ctx.lr = 0x82EA0CE0;
	sub_82EA7458(ctx, base);
	// 82EA0CE0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA0CE4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA0CE8: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82EA0CEC: 390AFFA0  addi r8, r10, -0x60
	ctx.r[8].s64 = ctx.r[10].s64 + -96;
	// 82EA0CF0: 38E96E60  addi r7, r9, 0x6e60
	ctx.r[7].s64 = ctx.r[9].s64 + 28256;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0D80 size=8
    let mut pc: u32 = 0x82EA0D80;
    'dispatch: loop {
        match pc {
            0x82EA0D80 => {
    //   block [0x82EA0D80..0x82EA0D88)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0D88 size=8
    let mut pc: u32 = 0x82EA0D88;
    'dispatch: loop {
        match pc {
            0x82EA0D88 => {
    //   block [0x82EA0D88..0x82EA0D90)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0D90 size=80
    let mut pc: u32 = 0x82EA0D90;
    'dispatch: loop {
        match pc {
            0x82EA0D90 => {
    //   block [0x82EA0D90..0x82EA0DE0)
	// 82EA0D90: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0DE0 size=80
    let mut pc: u32 = 0x82EA0DE0;
    'dispatch: loop {
        match pc {
            0x82EA0DE0 => {
    //   block [0x82EA0DE0..0x82EA0E30)
	// 82EA0DE0: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA0E30 size=64
    let mut pc: u32 = 0x82EA0E30;
    'dispatch: loop {
        match pc {
            0x82EA0E30 => {
    //   block [0x82EA0E30..0x82EA0E70)
	// 82EA0E30: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA0E34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA0E38: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82EA0E3C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA0E40: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA0E44: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA0E48: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA0E4C: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA0E50: 419A0020  beq cr6, 0x82ea0e70
	if ctx.cr[6].eq {
		sub_82EA0E70(ctx, base);
		return;
	}
	// 82EA0E54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA0E58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA0E5C: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82EA0E60: 4198FFDC  blt cr6, 0x82ea0e3c
	if ctx.cr[6].lt {
	pc = 0x82EA0E3C; continue 'dispatch;
	}
	// 82EA0E64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA0E68: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA0E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0E70 size=12
    let mut pc: u32 = 0x82EA0E70;
    'dispatch: loop {
        match pc {
            0x82EA0E70 => {
    //   block [0x82EA0E70..0x82EA0E7C)
	// 82EA0E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA0E74: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA0E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EA0E80 size=164
    let mut pc: u32 = 0x82EA0E80;
    'dispatch: loop {
        match pc {
            0x82EA0E80 => {
    //   block [0x82EA0E80..0x82EA0F24)
	// 82EA0E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0E84: 483072E9  bl 0x831a816c
	ctx.lr = 0x82EA0E88;
	sub_831A8130(ctx, base);
	// 82EA0E88: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82EA0E8C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA0E90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA0E94: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EA0E98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA0E9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA0EA0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA0EA4: 4800823D  bl 0x82ea90e0
	ctx.lr = 0x82EA0EA8;
	sub_82EA90E0(ctx, base);
	// 82EA0EA8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA0EAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA0EB0: 419A005C  beq cr6, 0x82ea0f0c
	if ctx.cr[6].eq {
	pc = 0x82EA0F0C; continue 'dispatch;
	}
	// 82EA0EB4: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82EA0EB8: D3E10060  stfs f31, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82EA0EBC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EA0EC0: 11BF038C  vspltisw v13, -1
	for i in 0..4 {
		ctx.v[13].u32[i] = 4294967295;
	}
	// 82EA0EC4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82EA0EC8: 390ABD40  addi r8, r10, -0x42c0
	ctx.r[8].s64 = ctx.r[10].s64 + -17088;
	// 82EA0ECC: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA0F28 size=128
    let mut pc: u32 = 0x82EA0F28;
    'dispatch: loop {
        match pc {
            0x82EA0F28 => {
    //   block [0x82EA0F28..0x82EA0FA8)
	// 82EA0F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA0F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA0F30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA0F34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA0F38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA0F3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA0F40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA0F44: 480080AD  bl 0x82ea8ff0
	ctx.lr = 0x82EA0F48;
	sub_82EA8FF0(ctx, base);
	// 82EA0F48: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA0FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA0FA8 size=104
    let mut pc: u32 = 0x82EA0FA8;
    'dispatch: loop {
        match pc {
            0x82EA0FA8 => {
    //   block [0x82EA0FA8..0x82EA1010)
	// 82EA0FA8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1010 size=80
    let mut pc: u32 = 0x82EA1010;
    'dispatch: loop {
        match pc {
            0x82EA1010 => {
    //   block [0x82EA1010..0x82EA1060)
	// 82EA1010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA1014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA1018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA101C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA1020: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA1024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA1028: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EA102C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA1030: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA1034: 48007F1D  bl 0x82ea8f50
	ctx.lr = 0x82EA1038;
	sub_82EA8F50(ctx, base);
	// 82EA1038: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA103C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA1040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA1044: 4BFFFF65  bl 0x82ea0fa8
	ctx.lr = 0x82EA1048;
	sub_82EA0FA8(ctx, base);
	// 82EA1048: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EA104C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA1050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA1054: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA1058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA1060 size=152
    let mut pc: u32 = 0x82EA1060;
    'dispatch: loop {
        match pc {
            0x82EA1060 => {
    //   block [0x82EA1060..0x82EA10F8)
	// 82EA1060: 38E40010  addi r7, r4, 0x10
	ctx.r[7].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA10F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA10F8 size=100
    let mut pc: u32 = 0x82EA10F8;
    'dispatch: loop {
        match pc {
            0x82EA10F8 => {
    //   block [0x82EA10F8..0x82EA115C)
	// 82EA10F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA10FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA1100: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA1104: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1160 size=80
    let mut pc: u32 = 0x82EA1160;
    'dispatch: loop {
        match pc {
            0x82EA1160 => {
    //   block [0x82EA1160..0x82EA11B0)
	// 82EA1160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA1164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA1168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA116C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA1170: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA1174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA1178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA117C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EA1180: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA1184: 4BFFFDA5  bl 0x82ea0f28
	ctx.lr = 0x82EA1188;
	sub_82EA0F28(ctx, base);
	// 82EA1188: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EA118C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA1190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA1194: 4BFFFE15  bl 0x82ea0fa8
	ctx.lr = 0x82EA1198;
	sub_82EA0FA8(ctx, base);
	// 82EA1198: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EA119C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA11A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA11A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA11A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA11AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA11B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA11B0 size=160
    let mut pc: u32 = 0x82EA11B0;
    'dispatch: loop {
        match pc {
            0x82EA11B0 => {
    //   block [0x82EA11B0..0x82EA1250)
	// 82EA11B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA11B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA11B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA11BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA11C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA11C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA11C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA11CC: 4800145D  bl 0x82ea2628
	ctx.lr = 0x82EA11D0;
	sub_82EA2628(ctx, base);
	// 82EA11D0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA11D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA11D8: 419A005C  beq cr6, 0x82ea1234
	if ctx.cr[6].eq {
	pc = 0x82EA1234; continue 'dispatch;
	}
	// 82EA11DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA11E0: 419A0054  beq cr6, 0x82ea1234
	if ctx.cr[6].eq {
	pc = 0x82EA1234; continue 'dispatch;
	}
	// 82EA11E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA11E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA11EC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA11F0: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EA11F4: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA11F8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EA11FC: 4198001C  blt cr6, 0x82ea1218
	if ctx.cr[6].lt {
	pc = 0x82EA1218; continue 'dispatch;
	}
	// 82EA1200: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EA1204: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA1208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA120C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA1210: 4BFFEE51  bl 0x82ea0060
	ctx.lr = 0x82EA1214;
	sub_82EA0060(ctx, base);
	// 82EA1214: 48000020  b 0x82ea1234
	pc = 0x82EA1234; continue 'dispatch;
	// 82EA1218: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EA121C: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82EA1220: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EA1224: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EA1228: 912B0044  stw r9, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82EA122C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA1230: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82EA1234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA1238: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA123C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA1240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA1244: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA1248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA124C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA1250 size=12
    let mut pc: u32 = 0x82EA1250;
    'dispatch: loop {
        match pc {
            0x82EA1250 => {
    //   block [0x82EA1250..0x82EA125C)
	// 82EA1250: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA1254: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA1258: 483A1704  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1260 size=116
    let mut pc: u32 = 0x82EA1260;
    'dispatch: loop {
        match pc {
            0x82EA1260 => {
    //   block [0x82EA1260..0x82EA12D4)
	// 82EA1260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA1264: 48306F09  bl 0x831a816c
	ctx.lr = 0x82EA1268;
	sub_831A8130(ctx, base);
	// 82EA1268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA126C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA1270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA1274: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA1278: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA127C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA1280: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA1284: 40990018  ble cr6, 0x82ea129c
	if !ctx.cr[6].gt {
	pc = 0x82EA129C; continue 'dispatch;
	}
	// 82EA1288: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA128C: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA1290: 480013D1  bl 0x82ea2660
	ctx.lr = 0x82EA1294;
	sub_82EA2660(ctx, base);
	// 82EA1294: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA1298: 4181FFF0  bgt 0x82ea1288
	if ctx.cr[0].gt {
	pc = 0x82EA1288; continue 'dispatch;
	}
	// 82EA129C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA12A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA12A4: 419A0028  beq cr6, 0x82ea12cc
	if ctx.cr[6].eq {
	pc = 0x82EA12CC; continue 'dispatch;
	}
	// 82EA12A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA12AC: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA12B0: 480013B1  bl 0x82ea2660
	ctx.lr = 0x82EA12B4;
	sub_82EA2660(ctx, base);
	// 82EA12B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA12B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA12BC: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA12C0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA12C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA12C8: 409AFFE0  bne cr6, 0x82ea12a8
	if !ctx.cr[6].eq {
	pc = 0x82EA12A8; continue 'dispatch;
	}
	// 82EA12CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA12D0: 48306EEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA12D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA12D8 size=52
    let mut pc: u32 = 0x82EA12D8;
    'dispatch: loop {
        match pc {
            0x82EA12D8 => {
    //   block [0x82EA12D8..0x82EA130C)
	// 82EA12D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EA12DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA12E0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EA12E4: 9923003D  stb r9, 0x3d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(61 as u32), ctx.r[9].u8 ) };
	// 82EA12E8: 9943003E  stb r10, 0x3e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(62 as u32), ctx.r[10].u8 ) };
	// 82EA12EC: 3883003C  addi r4, r3, 0x3c
	ctx.r[4].s64 = ctx.r[3].s64 + 60;
	// 82EA12F0: 9963003C  stb r11, 0x3c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 82EA12F4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EA12F8: 99430041  stb r10, 0x41(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(65 as u32), ctx.r[10].u8 ) };
	// 82EA12FC: 99230042  stb r9, 0x42(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(66 as u32), ctx.r[9].u8 ) };
	// 82EA1300: 99630040  stb r11, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82EA1304: 38630044  addi r3, r3, 0x44
	ctx.r[3].s64 = ctx.r[3].s64 + 68;
	// 82EA1308: 48009440  b 0x82eaa748
	sub_82EAA748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1310 size=48
    let mut pc: u32 = 0x82EA1310;
    'dispatch: loop {
        match pc {
            0x82EA1310 => {
    //   block [0x82EA1310..0x82EA1340)
	// 82EA1310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA1314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA1318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA131C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA1320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA1324: 48000D8D  bl 0x82ea20b0
	ctx.lr = 0x82EA1328;
	sub_82EA20B0(ctx, base);
	// 82EA1328: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA132C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA1330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA1334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA1338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA1340 size=16
    let mut pc: u32 = 0x82EA1340;
    'dispatch: loop {
        match pc {
            0x82EA1340 => {
    //   block [0x82EA1340..0x82EA1350)
	// 82EA1340: 81650038  lwz r11, 0x38(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1344: 81450024  lwz r10, 0x24(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA1348: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA134C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA1350 size=32
    let mut pc: u32 = 0x82EA1350;
    'dispatch: loop {
        match pc {
            0x82EA1350 => {
    //   block [0x82EA1350..0x82EA1370)
	// 82EA1350: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1354: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1358: 419A0018  beq cr6, 0x82ea1370
	if ctx.cr[6].eq {
		sub_82EA1370(ctx, base);
		return;
	}
	// 82EA135C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1360: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1364: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA1368: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA136C: 480012F4  b 0x82ea2660
	sub_82EA2660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA1370 size=12
    let mut pc: u32 = 0x82EA1370;
    'dispatch: loop {
        match pc {
            0x82EA1370 => {
    //   block [0x82EA1370..0x82EA137C)
	// 82EA1370: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1374: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1378: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA137C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA137C size=20
    let mut pc: u32 = 0x82EA137C;
    'dispatch: loop {
        match pc {
            0x82EA137C => {
    //   block [0x82EA137C..0x82EA1390)
	// 82EA137C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1380: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1384: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA1388: 80630058  lwz r3, 0x58(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA138C: 480012D4  b 0x82ea2660
	sub_82EA2660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA1390 size=4
    let mut pc: u32 = 0x82EA1390;
    'dispatch: loop {
        match pc {
            0x82EA1390 => {
    //   block [0x82EA1390..0x82EA1394)
	// 82EA1390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1398 size=104
    let mut pc: u32 = 0x82EA1398;
    'dispatch: loop {
        match pc {
            0x82EA1398 => {
    //   block [0x82EA1398..0x82EA1400)
	// 82EA1398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA139C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA13A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA13A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA13A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA13AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA13B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA13B4: 48000CFD  bl 0x82ea20b0
	ctx.lr = 0x82EA13B8;
	sub_82EA20B0(ctx, base);
	// 82EA13B8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA13BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA13C0: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EA13C4: 48000EC5  bl 0x82ea2288
	ctx.lr = 0x82EA13C8;
	sub_82EA2288(ctx, base);
	// 82EA13C8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA13CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA13D0: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82EA13D4: 48000EB5  bl 0x82ea2288
	ctx.lr = 0x82EA13D8;
	sub_82EA2288(ctx, base);
	// 82EA13D8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA13DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA13E0: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA13E4: 483A1579  bl 0x8324295c
	ctx.lr = 0x82EA13E8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA13E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA13EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA13F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA13F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA13F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA13FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1400 size=532
    let mut pc: u32 = 0x82EA1400;
    'dispatch: loop {
        match pc {
            0x82EA1400 => {
    //   block [0x82EA1400..0x82EA1614)
	// 82EA1400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA1404: 48306D55  bl 0x831a8158
	ctx.lr = 0x82EA1408;
	sub_831A8130(ctx, base);
	// 82EA1408: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA140C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA1410: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EA1414: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA1418: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EA141C: 3BAB70B0  addi r29, r11, 0x70b0
	ctx.r[29].s64 = ctx.r[11].s64 + 28848;
	// 82EA1420: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82EA1424: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82EA1428: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82EA142C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1430: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82EA1434: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82EA1438: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82EA143C: 48000C75  bl 0x82ea20b0
	ctx.lr = 0x82EA1440;
	sub_82EA20B0(ctx, base);
	// 82EA1440: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EA1444: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EA1448: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA144C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA1450: 419A0008  beq cr6, 0x82ea1458
	if ctx.cr[6].eq {
	pc = 0x82EA1458; continue 'dispatch;
	}
	// 82EA1454: 93EB002C  stw r31, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 82EA1458: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82EA145C: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EA1460: 93FD0030  stw r31, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82EA1464: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1468: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82EA146C: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA1470: 483A14ED  bl 0x8324295c
	ctx.lr = 0x82EA1474;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1474: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA1478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA147C: 483A19D1  bl 0x83242e4c
	ctx.lr = 0x82EA1480;
	// extern call 0x83242E4C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82EA1480: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1484: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82EA1488: FB7F0020  std r27, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u64 ) };
	// 82EA148C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1490: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EA1494: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA1498: 419A001C  beq cr6, 0x82ea14b4
	if ctx.cr[6].eq {
	pc = 0x82EA14B4; continue 'dispatch;
	}
	// 82EA149C: 8143006C  lwz r10, 0x6c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EA14A0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EA14A4: 9143006C  stw r10, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82EA14A8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA14AC: 91230068  stw r9, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82EA14B0: 48000014  b 0x82ea14c4
	pc = 0x82EA14C4; continue 'dispatch;
	// 82EA14B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EA14B8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82EA14BC: 4BFFF1A5  bl 0x82ea0660
	ctx.lr = 0x82EA14C0;
	sub_82EA0660(ctx, base);
	// 82EA14C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA14C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA14C8: 419A0044  beq cr6, 0x82ea150c
	if ctx.cr[6].eq {
	pc = 0x82EA150C; continue 'dispatch;
	}
	// 82EA14CC: 93CB0014  stw r30, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EA14D0: 93CB0018  stw r30, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EA14D4: 93CB001C  stw r30, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82EA14D8: 93CB0020  stw r30, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EA14DC: 93CB0024  stw r30, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EA14E0: 93CB0028  stw r30, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82EA14E4: 93CB002C  stw r30, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82EA14E8: 93CB0030  stw r30, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82EA14EC: 93CB0034  stw r30, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82EA14F0: 93CB0038  stw r30, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82EA14F4: 93CB003C  stw r30, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82EA14F8: 93CB0040  stw r30, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82EA14FC: 93CB0044  stw r30, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82EA1500: 93CB0048  stw r30, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82EA1504: 93CB004C  stw r30, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82EA1508: 48000008  b 0x82ea1510
	pc = 0x82EA1510; continue 'dispatch;
	// 82EA150C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EA1510: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82EA1514: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1518: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EA151C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA1520: 419A0020  beq cr6, 0x82ea1540
	if ctx.cr[6].eq {
	pc = 0x82EA1540; continue 'dispatch;
	}
	// 82EA1524: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EA1528: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82EA152C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82EA1530: 912B0044  stw r9, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82EA1534: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1538: 910B0040  stw r8, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 82EA153C: 48000014  b 0x82ea1550
	pc = 0x82EA1550; continue 'dispatch;
	// 82EA1540: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EA1544: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1548: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA154C: 4BFFF115  bl 0x82ea0660
	ctx.lr = 0x82EA1550;
	sub_82EA0660(ctx, base);
	// 82EA1550: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA1554: 419A0014  beq cr6, 0x82ea1568
	if ctx.cr[6].eq {
	pc = 0x82EA1568; continue 'dispatch;
	}
	// 82EA1558: 38A003E8  li r5, 0x3e8
	ctx.r[5].s64 = 1000;
	// 82EA155C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA1560: 48001089  bl 0x82ea25e8
	ctx.lr = 0x82EA1564;
	sub_82EA25E8(ctx, base);
	// 82EA1564: 48000008  b 0x82ea156c
	pc = 0x82EA156C; continue 'dispatch;
	// 82EA1568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA156C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1570: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82EA1574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA1578: 935F0058  stw r26, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 82EA157C: 933F005C  stw r25, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 82EA1580: 931F0060  stw r24, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82EA1584: 9BCB0010  stb r30, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82EA1588: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA158C: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EA1590: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1594: 93C90008  stw r30, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EA1598: 811F0038  lwz r8, 0x38(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA159C: 93C8000C  stw r30, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82EA15A0: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA15A4: 93C70000  stw r30, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EA15A8: 48000B09  bl 0x82ea20b0
	ctx.lr = 0x82EA15AC;
	sub_82EA20B0(ctx, base);
	// 82EA15AC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA15B0: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82EA15B4: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EA15B8: 48000CD1  bl 0x82ea2288
	ctx.lr = 0x82EA15BC;
	sub_82EA2288(ctx, base);
	// 82EA15BC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA15C0: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82EA15C4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82EA15C8: 48000CC1  bl 0x82ea2288
	ctx.lr = 0x82EA15CC;
	sub_82EA2288(ctx, base);
	// 82EA15CC: FB7F0020  std r27, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u64 ) };
	// 82EA15D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA15D4: 483A1389  bl 0x8324295c
	ctx.lr = 0x82EA15D8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA15D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA15DC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EA15E0: 9BDF003D  stb r30, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[30].u8 ) };
	// 82EA15E4: 995F003E  stb r10, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[10].u8 ) };
	// 82EA15E8: 389F003C  addi r4, r31, 0x3c
	ctx.r[4].s64 = ctx.r[31].s64 + 60;
	// 82EA15EC: 997F003C  stb r11, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 82EA15F0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EA15F4: 995F0041  stb r10, 0x41(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(65 as u32), ctx.r[10].u8 ) };
	// 82EA15F8: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 82EA15FC: 9BDF0042  stb r30, 0x42(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(66 as u32), ctx.r[30].u8 ) };
	// 82EA1600: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82EA1604: 48009145  bl 0x82eaa748
	ctx.lr = 0x82EA1608;
	sub_82EAA748(ctx, base);
	// 82EA1608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA160C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EA1610: 48306B98  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1618 size=180
    let mut pc: u32 = 0x82EA1618;
    'dispatch: loop {
        match pc {
            0x82EA1618 => {
    //   block [0x82EA1618..0x82EA16CC)
	// 82EA1618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA161C: 48306B4D  bl 0x831a8168
	ctx.lr = 0x82EA1620;
	sub_831A8130(ctx, base);
	// 82EA1620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA1624: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA1628: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA162C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EA1630: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EA1634: 48000A7D  bl 0x82ea20b0
	ctx.lr = 0x82EA1638;
	sub_82EA20B0(ctx, base);
	// 82EA1638: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82EA163C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EA1640: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA1644: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA1648: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA164C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA1650: 83FC0038  lwz r31, 0x38(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1654: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA1658: 409A000C  bne cr6, 0x82ea1664
	if !ctx.cr[6].eq {
	pc = 0x82EA1664; continue 'dispatch;
	}
	// 82EA165C: 48000E3D  bl 0x82ea2498
	ctx.lr = 0x82EA1660;
	sub_82EA2498(ctx, base);
	// 82EA1660: 48000008  b 0x82ea1668
	pc = 0x82EA1668; continue 'dispatch;
	// 82EA1664: 48000D7D  bl 0x82ea23e0
	ctx.lr = 0x82EA1668;
	sub_82EA23E0(ctx, base);
	// 82EA1668: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA166C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA1670: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1674: 41820040  beq 0x82ea16b4
	if ctx.cr[0].eq {
	pc = 0x82EA16B4; continue 'dispatch;
	}
	// 82EA1678: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA167C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1680: 419A0014  beq cr6, 0x82ea1694
	if ctx.cr[6].eq {
	pc = 0x82EA1694; continue 'dispatch;
	}
	// 82EA1684: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1688: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA168C: 807C0054  lwz r3, 0x54(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1690: 4800001C  b 0x82ea16ac
	pc = 0x82EA16AC; continue 'dispatch;
	// 82EA1694: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1698: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA169C: 419A0018  beq cr6, 0x82ea16b4
	if ctx.cr[6].eq {
	pc = 0x82EA16B4; continue 'dispatch;
	}
	// 82EA16A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA16A4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA16A8: 807C0058  lwz r3, 0x58(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA16AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA16B0: 48000FB1  bl 0x82ea2660
	ctx.lr = 0x82EA16B4;
	sub_82EA2660(ctx, base);
	// 82EA16B4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA16B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA16BC: F97C0020  std r11, 0x20(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA16C0: 483A129D  bl 0x8324295c
	ctx.lr = 0x82EA16C4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA16C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA16C8: 48306AF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


