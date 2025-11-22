pub fn sub_824425E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824425E8 size=340
    let mut pc: u32 = 0x824425E8;
    'dispatch: loop {
        match pc {
            0x824425E8 => {
    //   block [0x824425E8..0x8244273C)
	// 824425E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824425EC: 480F2AC5  bl 0x825350b0
	ctx.lr = 0x824425F0;
	sub_82535080(ctx, base);
	// 824425F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824425F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824425F8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824425FC: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 82442600: 835E2048  lwz r26, 0x2048(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82442604: 817F0118  lwz r11, 0x118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 82442608: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244260C: 409A000C  bne cr6, 0x82442618
	if !ctx.cr[6].eq {
	pc = 0x82442618; continue 'dispatch;
	}
	// 82442610: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82442614: 48000018  b 0x8244262c
	pc = 0x8244262C; continue 'dispatch;
	// 82442618: 817F0164  lwz r11, 0x164(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8244261C: 813F013C  lwz r9, 0x13c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82442620: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82442624: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82442628: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244262C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82442630: 839F00E8  lwz r28, 0xe8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82442634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82442638: 419A0024  beq cr6, 0x8244265c
	if ctx.cr[6].eq {
	pc = 0x8244265C; continue 'dispatch;
	}
	// 8244263C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82442640: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82442644: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82442648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244264C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82442650: 4E800421  bctrl
	ctx.lr = 0x82442654;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82442654: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82442658: 480F2AA8  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 8244265C: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 82442660: 409A0014  bne cr6, 0x82442674
	if !ctx.cr[6].eq {
	pc = 0x82442674; continue 'dispatch;
	}
	// 82442664: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82442668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244266C: 4BFFBECD  bl 0x8243e538
	ctx.lr = 0x82442670;
	sub_8243E538(ctx, base);
	// 82442670: 4800000C  b 0x8244267c
	pc = 0x8244267C; continue 'dispatch;
	// 82442674: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 82442678: 409A0014  bne cr6, 0x8244268c
	if !ctx.cr[6].eq {
	pc = 0x8244268C; continue 'dispatch;
	}
	// 8244267C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82442680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442684: 4BFFBF45  bl 0x8243e5c8
	ctx.lr = 0x82442688;
	sub_8243E5C8(ctx, base);
	// 82442688: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8244268C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82442690: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82442694: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82442698: 4BFFCB91  bl 0x8243f228
	ctx.lr = 0x8244269C;
	sub_8243F228(ctx, base);
	// 8244269C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824426A0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824426A4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824426A8: 41990020  bgt cr6, 0x824426c8
	if ctx.cr[6].gt {
	pc = 0x824426C8; continue 'dispatch;
	}
	// 824426AC: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 824426B0: 815E0AA4  lwz r10, 0xaa4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2724 as u32) ) } as u64;
	// 824426B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824426B8: 41980010  blt cr6, 0x824426c8
	if ctx.cr[6].lt {
	pc = 0x824426C8; continue 'dispatch;
	}
	// 824426BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824426C0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824426C4: 480F2A3C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824426C8: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 824426CC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 824426D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824426D4: 4BFFC1BD  bl 0x8243e890
	ctx.lr = 0x824426D8;
	sub_8243E890(ctx, base);
	// 824426D8: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824426DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824426E0: 4198FFDC  blt cr6, 0x824426bc
	if ctx.cr[6].lt {
	pc = 0x824426BC; continue 'dispatch;
	}
	// 824426E4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824426E8: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 824426EC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824426F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824426F4: 4BFFEB1D  bl 0x82441210
	ctx.lr = 0x824426F8;
	sub_82441210(ctx, base);
	// 824426F8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824426FC: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82442700: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82442704: 7D6BE1D6  mullw r11, r11, r28
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82442708: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8244270C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82442710: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82442714: 7CABE850  subf r5, r11, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82442718: 480074A9  bl 0x82449bc0
	ctx.lr = 0x8244271C;
	sub_82449BC0(ctx, base);
	// 8244271C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442720: 409AFF9C  bne cr6, 0x824426bc
	if !ctx.cr[6].eq {
	pc = 0x824426BC; continue 'dispatch;
	}
	// 82442724: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82442728: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244272C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82442730: 917A000C  stw r11, 0xc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82442734: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82442738: 480F29C8  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442740 size=196
    let mut pc: u32 = 0x82442740;
    'dispatch: loop {
        match pc {
            0x82442740 => {
    //   block [0x82442740..0x82442804)
	// 82442740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442744: 480F2971  bl 0x825350b4
	ctx.lr = 0x82442748;
	sub_82535080(ctx, base);
	// 82442748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244274C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442750: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82442754: 83BF2048  lwz r29, 0x2048(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82442758: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244275C: 4BFFEACD  bl 0x82441228
	ctx.lr = 0x82442760;
	sub_82441228(ctx, base);
	// 82442760: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82442764: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82442768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244276C: 4BFE0C55  bl 0x824233c0
	ctx.lr = 0x82442770;
	sub_824233C0(ctx, base);
	// 82442770: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82442774: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82442778: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244277C: 4800D5E5  bl 0x8244fd60
	ctx.lr = 0x82442780;
	sub_8244FD60(ctx, base);
	// 82442780: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82442784: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82442788: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8244278C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82442790: 4BFE0C31  bl 0x824233c0
	ctx.lr = 0x82442794;
	sub_824233C0(ctx, base);
	// 82442794: 7FDB1850  subf r30, r27, r3
	ctx.r[30].s64 = ctx.r[3].s64 - ctx.r[27].s64;
	// 82442798: 3CC0FF00  lis r6, -0x100
	ctx.r[6].s64 = -16777216;
	// 8244279C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824427A0: 60C60F07  ori r6, r6, 0xf07
	ctx.r[6].u64 = ctx.r[6].u64 | 3847;
	// 824427A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824427A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824427AC: 4BFFE0FD  bl 0x824408a8
	ctx.lr = 0x824427B0;
	sub_824408A8(ctx, base);
	// 824427B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824427B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824427B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824427BC: 4BFFE095  bl 0x82440850
	ctx.lr = 0x824427C0;
	sub_82440850(ctx, base);
	// 824427C0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824427C4: 419A0010  beq cr6, 0x824427d4
	if ctx.cr[6].eq {
	pc = 0x824427D4; continue 'dispatch;
	}
	// 824427C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824427CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824427D0: 480F2934  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824427D4: 897D006C  lbz r11, 0x6c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 824427D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824427DC: 409A000C  bne cr6, 0x824427e8
	if !ctx.cr[6].eq {
	pc = 0x824427E8; continue 'dispatch;
	}
	// 824427E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824427E4: 917D00F4  stw r11, 0xf4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 824427E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824427EC: 80BD002C  lwz r5, 0x2c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 824427F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824427F4: 4BFF745D  bl 0x82439c50
	ctx.lr = 0x824427F8;
	sub_82439C50(ctx, base);
	// 824427F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824427FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82442800: 480F2904  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442808 size=80
    let mut pc: u32 = 0x82442808;
    'dispatch: loop {
        match pc {
            0x82442808 => {
    //   block [0x82442808..0x82442858)
	// 82442808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244280C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82442810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82442814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82442818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244281C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442820: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82442824: 389F0E48  addi r4, r31, 0xe48
	ctx.r[4].s64 = ctx.r[31].s64 + 3656;
	// 82442828: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 8244282C: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82442830: 480F2321  bl 0x82534b50
	ctx.lr = 0x82442834;
	sub_82534B50(ctx, base);
	// 82442834: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82442838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244283C: 4BFFEE25  bl 0x82441660
	ctx.lr = 0x82442840;
	sub_82441660(ctx, base);
	// 82442840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82442844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82442848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244284C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82442850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82442854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442858 size=344
    let mut pc: u32 = 0x82442858;
    'dispatch: loop {
        match pc {
            0x82442858 => {
    //   block [0x82442858..0x824429B0)
	// 82442858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244285C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82442860: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82442864: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82442868: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244286C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82442870: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82442874: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82442878: 809E2050  lwz r4, 0x2050(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8272 as u32) ) } as u64;
	// 8244287C: 48006215  bl 0x82448a90
	ctx.lr = 0x82442880;
	sub_82448A90(ctx, base);
	// 82442880: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442884: 419A000C  beq cr6, 0x82442890
	if ctx.cr[6].eq {
	pc = 0x82442890; continue 'dispatch;
	}
	// 82442888: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244288C: 4800010C  b 0x82442998
	pc = 0x82442998; continue 'dispatch;
	// 82442890: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82442894: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442898: 419AFFF0  beq cr6, 0x82442888
	if ctx.cr[6].eq {
	pc = 0x82442888; continue 'dispatch;
	}
	// 8244289C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824428A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824428A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824428A8: 4BFFDBA9  bl 0x82440450
	ctx.lr = 0x824428AC;
	sub_82440450(ctx, base);
	// 824428AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824428B0: 409A0078  bne cr6, 0x82442928
	if !ctx.cr[6].eq {
	pc = 0x82442928; continue 'dispatch;
	}
	// 824428B4: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 824428B8: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824428BC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824428C0: 37EBFFFD  addic. r31, r11, -3
	ctx.xer.ca = (ctx.r[11].u32 > (!(-3 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -3;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824428C4: 41810008  bgt 0x824428cc
	if ctx.cr[0].gt {
	pc = 0x824428CC; continue 'dispatch;
	}
	// 824428C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824428CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824428D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824428D4: 4BFFEE3D  bl 0x82441710
	ctx.lr = 0x824428D8;
	sub_82441710(ctx, base);
	// 824428D8: 80E10064  lwz r7, 0x64(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824428DC: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824428E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824428E4: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824428E8: 7D275050  subf r9, r7, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 824428EC: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 824428F0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 824428F4: 41980008  blt cr6, 0x824428fc
	if ctx.cr[6].lt {
	pc = 0x824428FC; continue 'dispatch;
	}
	// 824428F8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 824428FC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82442900: 40980084  bge cr6, 0x82442984
	if !ctx.cr[6].lt {
	pc = 0x82442984; continue 'dispatch;
	}
	// 82442904: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82442908: 7D4B4214  add r10, r11, r8
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8244290C: 41980008  blt cr6, 0x82442914
	if ctx.cr[6].lt {
	pc = 0x82442914; continue 'dispatch;
	}
	// 82442910: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82442914: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442918: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8244291C: 409A0058  bne cr6, 0x82442974
	if !ctx.cr[6].eq {
	pc = 0x82442974; continue 'dispatch;
	}
	// 82442920: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82442924: 4BFFFFC8  b 0x824428ec
	pc = 0x824428EC; continue 'dispatch;
	// 82442928: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8244292C: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82442930: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82442934: 41990018  bgt cr6, 0x8244294c
	if ctx.cr[6].gt {
	pc = 0x8244294C; continue 'dispatch;
	}
	// 82442938: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8244293C: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82442940: 4098000C  bge cr6, 0x8244294c
	if !ctx.cr[6].lt {
	pc = 0x8244294C; continue 'dispatch;
	}
	// 82442944: 7FEB1850  subf r31, r11, r3
	ctx.r[31].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82442948: 4BFFFF84  b 0x824428cc
	pc = 0x824428CC; continue 'dispatch;
	// 8244294C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82442950: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82442954: 4199FF74  bgt cr6, 0x824428c8
	if ctx.cr[6].gt {
	pc = 0x824428C8; continue 'dispatch;
	}
	// 82442958: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8244295C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82442960: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82442964: 4098FF64  bge cr6, 0x824428c8
	if !ctx.cr[6].lt {
	pc = 0x824428C8; continue 'dispatch;
	}
	// 82442968: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8244296C: 7FEB4A14  add r31, r11, r9
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82442970: 4BFFFF5C  b 0x824428cc
	pc = 0x824428CC; continue 'dispatch;
	// 82442974: E95E09B0  ld r10, 0x9b0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(2480 as u32) ) };
	// 82442978: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 8244297C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82442980: F97E09B0  std r11, 0x9b0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(2480 as u32), ctx.r[11].u64 ) };
	// 82442984: E95E09A8  ld r10, 0x9a8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(2472 as u32) ) };
	// 82442988: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 8244298C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442990: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82442994: F97E09A8  std r11, 0x9a8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(2472 as u32), ctx.r[11].u64 ) };
	// 82442998: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8244299C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824429A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824429A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824429A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824429AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824429B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824429B0 size=184
    let mut pc: u32 = 0x824429B0;
    'dispatch: loop {
        match pc {
            0x824429B0 => {
    //   block [0x824429B0..0x82442A68)
	// 824429B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824429B4: 480F26F9  bl 0x825350ac
	ctx.lr = 0x824429B8;
	sub_82535080(ctx, base);
	// 824429B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824429BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824429C0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824429C4: 4099009C  ble cr6, 0x82442a60
	if !ctx.cr[6].gt {
	pc = 0x82442A60; continue 'dispatch;
	}
	// 824429C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824429CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824429D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824429D4: 3B200005  li r25, 5
	ctx.r[25].s64 = 5;
	// 824429D8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 824429DC: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 824429E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824429E4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824429E8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 824429EC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 824429F0: B3DF000C  sth r30, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 824429F4: B3DF000E  sth r30, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 824429F8: B3DF0010  sth r30, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u16 ) };
	// 824429FC: B3DF0012  sth r30, 0x12(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[30].u16 ) };
	// 82442A00: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 82442A04: 4BFFCBBD  bl 0x8243f5c0
	ctx.lr = 0x82442A08;
	sub_8243F5C0(ctx, base);
	// 82442A08: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442A0C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82442A10: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82442A14: 935F0048  stw r26, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 82442A18: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82442A1C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82442A20: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82442A24: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82442A28: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82442A2C: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82442A30: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82442A34: 4BFFED4D  bl 0x82441780
	ctx.lr = 0x82442A38;
	sub_82441780(ctx, base);
	// 82442A38: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82442A3C: FB7F00E8  std r27, 0xe8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[27].u64 ) };
	// 82442A40: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 82442A44: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82442A48: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 82442A4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82442A50: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 82442A54: B3DF00FC  sth r30, 0xfc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u16 ) };
	// 82442A58: 3BFF0100  addi r31, r31, 0x100
	ctx.r[31].s64 = ctx.r[31].s64 + 256;
	// 82442A5C: 409AFF84  bne cr6, 0x824429e0
	if !ctx.cr[6].eq {
	pc = 0x824429E0; continue 'dispatch;
	}
	// 82442A60: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82442A64: 480F2698  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442A68 size=96
    let mut pc: u32 = 0x82442A68;
    'dispatch: loop {
        match pc {
            0x82442A68 => {
    //   block [0x82442A68..0x82442AC8)
	// 82442A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442A6C: 480F2651  bl 0x825350bc
	ctx.lr = 0x82442A70;
	sub_82535080(ctx, base);
	// 82442A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442A74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442A78: 83BF2048  lwz r29, 0x2048(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82442A7C: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442A80: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82442A84: 419A0038  beq cr6, 0x82442abc
	if ctx.cr[6].eq {
	pc = 0x82442ABC; continue 'dispatch;
	}
	// 82442A88: 48001D49  bl 0x824447d0
	ctx.lr = 0x82442A8C;
	sub_824447D0(ctx, base);
	// 82442A8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82442A90: 4BFFED21  bl 0x824417b0
	ctx.lr = 0x82442A94;
	sub_824417B0(ctx, base);
	// 82442A94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442A98: 419A001C  beq cr6, 0x82442ab4
	if ctx.cr[6].eq {
	pc = 0x82442AB4; continue 'dispatch;
	}
	// 82442A9C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82442AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442AA4: 60840F0C  ori r4, r4, 0xf0c
	ctx.r[4].u64 = ctx.r[4].u64 | 3852;
	// 82442AA8: 48004E61  bl 0x82447908
	ctx.lr = 0x82442AAC;
	sub_82447908(ctx, base);
	// 82442AAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82442AB0: 480F265C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82442AB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82442AB8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82442ABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82442AC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82442AC4: 480F2648  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82442AC8 size=12
    let mut pc: u32 = 0x82442AC8;
    'dispatch: loop {
        match pc {
            0x82442AC8 => {
    //   block [0x82442AC8..0x82442AD4)
	// 82442AC8: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 82442ACC: 38CB1800  addi r6, r11, 0x1800
	ctx.r[6].s64 = ctx.r[11].s64 + 6144;
	// 82442AD0: 48002628  b 0x824450f8
	sub_824450F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442AD8 size=164
    let mut pc: u32 = 0x82442AD8;
    'dispatch: loop {
        match pc {
            0x82442AD8 => {
    //   block [0x82442AD8..0x82442B7C)
	// 82442AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442ADC: 480F25DD  bl 0x825350b8
	ctx.lr = 0x82442AE0;
	sub_82535080(ctx, base);
	// 82442AE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442AE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82442AE8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82442AEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82442AF0: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82442AF4: 4BFFEE75  bl 0x82441968
	ctx.lr = 0x82442AF8;
	sub_82441968(ctx, base);
	// 82442AF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82442AFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82442B00: 419A0070  beq cr6, 0x82442b70
	if ctx.cr[6].eq {
	pc = 0x82442B70; continue 'dispatch;
	}
	// 82442B04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442B08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82442B0C: 419A0064  beq cr6, 0x82442b70
	if ctx.cr[6].eq {
	pc = 0x82442B70; continue 'dispatch;
	}
	// 82442B10: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 82442B14: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442B18: 387E0DC4  addi r3, r30, 0xdc4
	ctx.r[3].s64 = ctx.r[30].s64 + 3524;
	// 82442B1C: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82442B20: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82442B24: 3BEB0038  addi r31, r11, 0x38
	ctx.r[31].s64 = ctx.r[11].s64 + 56;
	// 82442B28: 480F2029  bl 0x82534b50
	ctx.lr = 0x82442B2C;
	sub_82534B50(ctx, base);
	// 82442B2C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82442B30: 817F0200  lwz r11, 0x200(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(512 as u32) ) } as u64;
	// 82442B34: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82442B38: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82442B3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82442B40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82442B44: 4800C57D  bl 0x8244f0c0
	ctx.lr = 0x82442B48;
	sub_8244F0C0(ctx, base);
	// 82442B48: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442B4C: 419A001C  beq cr6, 0x82442b68
	if ctx.cr[6].eq {
	pc = 0x82442B68; continue 'dispatch;
	}
	// 82442B50: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82442B54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82442B58: 60840F1B  ori r4, r4, 0xf1b
	ctx.r[4].u64 = ctx.r[4].u64 | 3867;
	// 82442B5C: 48004DAD  bl 0x82447908
	ctx.lr = 0x82442B60;
	sub_82447908(ctx, base);
	// 82442B60: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82442B64: 480F25A4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82442B68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82442B6C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82442B70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82442B74: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82442B78: 480F2590  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442B80 size=116
    let mut pc: u32 = 0x82442B80;
    'dispatch: loop {
        match pc {
            0x82442B80 => {
    //   block [0x82442B80..0x82442BF4)
	// 82442B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82442B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82442B8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442B90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442B94: 48001E95  bl 0x82444a28
	ctx.lr = 0x82442B98;
	sub_82444A28(ctx, base);
	// 82442B98: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442B9C: 419A001C  beq cr6, 0x82442bb8
	if ctx.cr[6].eq {
	pc = 0x82442BB8; continue 'dispatch;
	}
	// 82442BA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82442BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82442BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82442BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82442BB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82442BB4: 4E800020  blr
	return;
	// 82442BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442BBC: 4BFFD6B5  bl 0x82440270
	ctx.lr = 0x82442BC0;
	sub_82440270(ctx, base);
	// 82442BC0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442BC4: 419A0018  beq cr6, 0x82442bdc
	if ctx.cr[6].eq {
	pc = 0x82442BDC; continue 'dispatch;
	}
	// 82442BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442BCC: 4BFFEEAD  bl 0x82441a78
	ctx.lr = 0x82442BD0;
	sub_82441A78(ctx, base);
	// 82442BD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442BD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82442BD8: 409A0008  bne cr6, 0x82442be0
	if !ctx.cr[6].eq {
	pc = 0x82442BE0; continue 'dispatch;
	}
	// 82442BDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82442BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82442BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82442BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82442BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82442BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442BF8 size=552
    let mut pc: u32 = 0x82442BF8;
    'dispatch: loop {
        match pc {
            0x82442BF8 => {
    //   block [0x82442BF8..0x82442E20)
	// 82442BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442BFC: 480F24B5  bl 0x825350b0
	ctx.lr = 0x82442C00;
	sub_82535080(ctx, base);
	// 82442C00: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442C04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82442C08: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82442C0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82442C10: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82442C14: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82442C18: 83BC2050  lwz r29, 0x2050(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82442C1C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82442C20: 93DA0000  stw r30, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82442C24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82442C28: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82442C2C: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82442C30: 48005E61  bl 0x82448a90
	ctx.lr = 0x82442C34;
	sub_82448A90(ctx, base);
	// 82442C34: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442C38: 409A01E0  bne cr6, 0x82442e18
	if !ctx.cr[6].eq {
	pc = 0x82442E18; continue 'dispatch;
	}
	// 82442C3C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82442C40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442C44: 419A01D0  beq cr6, 0x82442e14
	if ctx.cr[6].eq {
	pc = 0x82442E14; continue 'dispatch;
	}
	// 82442C48: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82442C4C: 388000CE  li r4, 0xce
	ctx.r[4].s64 = 206;
	// 82442C50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82442C54: 4BFFD7FD  bl 0x82440450
	ctx.lr = 0x82442C58;
	sub_82440450(ctx, base);
	// 82442C58: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82442C5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82442C60: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82442C64: 419A0058  beq cr6, 0x82442cbc
	if ctx.cr[6].eq {
	pc = 0x82442CBC; continue 'dispatch;
	}
	// 82442C68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82442C6C: 419A0014  beq cr6, 0x82442c80
	if ctx.cr[6].eq {
	pc = 0x82442C80; continue 'dispatch;
	}
	// 82442C70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82442C74: 4BFFD725  bl 0x82440398
	ctx.lr = 0x82442C78;
	sub_82440398(ctx, base);
	// 82442C78: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82442C7C: 48000020  b 0x82442c9c
	pc = 0x82442C9C; continue 'dispatch;
	// 82442C80: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82442C84: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82442C88: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82442C8C: 356BFFFD  addic. r11, r11, -3
	ctx.xer.ca = (ctx.r[11].u32 > (!(-3 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442C90: 40800008  bge 0x82442c98
	if !ctx.cr[0].lt {
	pc = 0x82442C98; continue 'dispatch;
	}
	// 82442C94: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82442C98: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82442C9C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442CA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442CA4: 40990170  ble cr6, 0x82442e14
	if !ctx.cr[6].gt {
	pc = 0x82442E14; continue 'dispatch;
	}
	// 82442CA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82442CAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82442CB0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82442CB4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82442CB8: 480F2448  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82442CBC: 83610058  lwz r27, 0x58(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82442CC0: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82442CC4: 576A0630  rlwinm r10, r27, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 82442CC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82442CCC: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82442CD0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82442CD4: 409A0140  bne cr6, 0x82442e14
	if !ctx.cr[6].eq {
	pc = 0x82442E14; continue 'dispatch;
	}
	// 82442CD8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82442CDC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82442CE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82442CE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82442CE8: 480054B9  bl 0x824481a0
	ctx.lr = 0x82442CEC;
	sub_824481A0(ctx, base);
	// 82442CEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82442CF0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82442CF4: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82442CF8: 4BFFEEC1  bl 0x82441bb8
	ctx.lr = 0x82442CFC;
	sub_82441BB8(ctx, base);
	// 82442CFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442D00: 419A0060  beq cr6, 0x82442d60
	if ctx.cr[6].eq {
	pc = 0x82442D60; continue 'dispatch;
	}
	// 82442D04: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82442D08: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82442D0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82442D10: 409A0010  bne cr6, 0x82442d20
	if !ctx.cr[6].eq {
	pc = 0x82442D20; continue 'dispatch;
	}
	// 82442D14: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82442D18: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82442D1C: 48000008  b 0x82442d24
	pc = 0x82442D24; continue 'dispatch;
	// 82442D20: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82442D24: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82442D28: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82442D2C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82442D30: 419A0080  beq cr6, 0x82442db0
	if ctx.cr[6].eq {
	pc = 0x82442DB0; continue 'dispatch;
	}
	// 82442D34: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82442D38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82442D3C: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 82442D40: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82442D44: 4BFFD855  bl 0x82440598
	ctx.lr = 0x82442D48;
	sub_82440598(ctx, base);
	// 82442D48: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82442D4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82442D50: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82442D54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82442D58: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82442D5C: 4800548D  bl 0x824481e8
	ctx.lr = 0x82442D60;
	sub_824481E8(ctx, base);
	// 82442D60: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82442D64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82442D68: 419A0048  beq cr6, 0x82442db0
	if ctx.cr[6].eq {
	pc = 0x82442DB0; continue 'dispatch;
	}
	// 82442D6C: 4800C3ED  bl 0x8244f158
	ctx.lr = 0x82442D70;
	sub_8244F158(ctx, base);
	// 82442D70: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82442D74: 419A004C  beq cr6, 0x82442dc0
	if ctx.cr[6].eq {
	pc = 0x82442DC0; continue 'dispatch;
	}
	// 82442D78: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 82442D7C: 409A0084  bne cr6, 0x82442e00
	if !ctx.cr[6].eq {
	pc = 0x82442E00; continue 'dispatch;
	}
	// 82442D80: 576B0672  rlwinm r11, r27, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 82442D84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442D88: 419A0078  beq cr6, 0x82442e00
	if ctx.cr[6].eq {
	pc = 0x82442E00; continue 'dispatch;
	}
	// 82442D8C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82442D90: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82442D94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82442D98: 4BFFD6B9  bl 0x82440450
	ctx.lr = 0x82442D9C;
	sub_82440450(ctx, base);
	// 82442D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82442DA0: 419A0010  beq cr6, 0x82442db0
	if ctx.cr[6].eq {
	pc = 0x82442DB0; continue 'dispatch;
	}
	// 82442DA4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82442DA8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82442DAC: 409A0054  bne cr6, 0x82442e00
	if !ctx.cr[6].eq {
	pc = 0x82442E00; continue 'dispatch;
	}
	// 82442DB0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82442DB4: 4BFFD63D  bl 0x824403f0
	ctx.lr = 0x82442DB8;
	sub_824403F0(ctx, base);
	// 82442DB8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82442DBC: 480F2344  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82442DC0: 736B0048  andi. r11, r27, 0x48
	ctx.r[11].u64 = ctx.r[27].u64 & 72;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442DC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442DC8: 419A0038  beq cr6, 0x82442e00
	if ctx.cr[6].eq {
	pc = 0x82442E00; continue 'dispatch;
	}
	// 82442DCC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82442DD0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82442DD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82442DD8: 4BFFD679  bl 0x82440450
	ctx.lr = 0x82442DDC;
	sub_82440450(ctx, base);
	// 82442DDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82442DE0: 419A0010  beq cr6, 0x82442df0
	if ctx.cr[6].eq {
	pc = 0x82442DF0; continue 'dispatch;
	}
	// 82442DE4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82442DE8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82442DEC: 409A0014  bne cr6, 0x82442e00
	if !ctx.cr[6].eq {
	pc = 0x82442E00; continue 'dispatch;
	}
	// 82442DF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82442DF4: 4BFFD5FD  bl 0x824403f0
	ctx.lr = 0x82442DF8;
	sub_824403F0(ctx, base);
	// 82442DF8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82442DFC: 480F2304  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82442E00: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82442E04: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82442E08: 4BFFD591  bl 0x82440398
	ctx.lr = 0x82442E0C;
	sub_82440398(ctx, base);
	// 82442E0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82442E10: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82442E14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82442E18: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82442E1C: 480F22E4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442E20 size=152
    let mut pc: u32 = 0x82442E20;
    'dispatch: loop {
        match pc {
            0x82442E20 => {
    //   block [0x82442E20..0x82442EB8)
	// 82442E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442E24: 480F2299  bl 0x825350bc
	ctx.lr = 0x82442E28;
	sub_82535080(ctx, base);
	// 82442E28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442E30: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82442E34: 3BBF0D88  addi r29, r31, 0xd88
	ctx.r[29].s64 = ctx.r[31].s64 + 3464;
	// 82442E38: 83DF2048  lwz r30, 0x2048(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82442E3C: 4BFF98CD  bl 0x8243c708
	ctx.lr = 0x82442E40;
	sub_8243C708(ctx, base);
	// 82442E40: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442E48: 409A0058  bne cr6, 0x82442ea0
	if !ctx.cr[6].eq {
	pc = 0x82442EA0; continue 'dispatch;
	}
	// 82442E4C: 4BFFF0CD  bl 0x82441f18
	ctx.lr = 0x82442E50;
	sub_82441F18(ctx, base);
	// 82442E50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442E54: 4099001C  ble cr6, 0x82442e70
	if !ctx.cr[6].gt {
	pc = 0x82442E70; continue 'dispatch;
	}
	// 82442E58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82442E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442E60: 48001551  bl 0x824443b0
	ctx.lr = 0x82442E64;
	sub_824443B0(ctx, base);
	// 82442E64: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82442E68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82442E6C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82442E70: 3C807FFF  lis r4, 0x7fff
	ctx.r[4].s64 = 2147418112;
	// 82442E74: 387D003C  addi r3, r29, 0x3c
	ctx.r[3].s64 = ctx.r[29].s64 + 60;
	// 82442E78: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 82442E7C: 4BFFC745  bl 0x8243f5c0
	ctx.lr = 0x82442E80;
	sub_8243F5C0(ctx, base);
	// 82442E80: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82442E84: 387D0068  addi r3, r29, 0x68
	ctx.r[3].s64 = ctx.r[29].s64 + 104;
	// 82442E88: 4BFFC739  bl 0x8243f5c0
	ctx.lr = 0x82442E8C;
	sub_8243F5C0(ctx, base);
	// 82442E8C: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 82442E90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82442E94: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82442E98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82442E9C: 480F2270  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82442EA0: 4BFFD8F9  bl 0x82440798
	ctx.lr = 0x82442EA4;
	sub_82440798(ctx, base);
	// 82442EA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442EA8: 4098FFAC  bge cr6, 0x82442e54
	if !ctx.cr[6].lt {
	pc = 0x82442E54; continue 'dispatch;
	}
	// 82442EAC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82442EB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82442EB4: 480F2258  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442EB8 size=120
    let mut pc: u32 = 0x82442EB8;
    'dispatch: loop {
        match pc {
            0x82442EB8 => {
    //   block [0x82442EB8..0x82442F30)
	// 82442EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442EBC: 480F21F5  bl 0x825350b0
	ctx.lr = 0x82442EC0;
	sub_82535080(ctx, base);
	// 82442EC0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442EC8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82442ECC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82442ED0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82442ED4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82442ED8: 835F2048  lwz r26, 0x2048(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82442EDC: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82442EE0: 809F2050  lwz r4, 0x2050(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82442EE4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82442EE8: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82442EEC: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82442EF0: 419A0038  beq cr6, 0x82442f28
	if ctx.cr[6].eq {
	pc = 0x82442F28; continue 'dispatch;
	}
	// 82442EF4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82442EF8: 48007CB1  bl 0x8244aba8
	ctx.lr = 0x82442EFC;
	sub_8244ABA8(ctx, base);
	// 82442EFC: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82442F00: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82442F04: 41980024  blt cr6, 0x82442f28
	if ctx.cr[6].lt {
	pc = 0x82442F28; continue 'dispatch;
	}
	// 82442F08: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82442F0C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82442F10: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82442F14: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82442F18: 389A00A0  addi r4, r26, 0xa0
	ctx.r[4].s64 = ctx.r[26].s64 + 160;
	// 82442F1C: 387F0D88  addi r3, r31, 0xd88
	ctx.r[3].s64 = ctx.r[31].s64 + 3464;
	// 82442F20: 4BFFF1B1  bl 0x824420d0
	ctx.lr = 0x82442F24;
	sub_824420D0(ctx, base);
	// 82442F24: F87E0000  std r3, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82442F28: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82442F2C: 480F21D4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442F30 size=160
    let mut pc: u32 = 0x82442F30;
    'dispatch: loop {
        match pc {
            0x82442F30 => {
    //   block [0x82442F30..0x82442FD0)
	// 82442F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442F34: 480F2185  bl 0x825350b8
	ctx.lr = 0x82442F38;
	sub_82535080(ctx, base);
	// 82442F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442F3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82442F40: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 82442F44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442F48: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82442F4C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82442F50: 4BFF97B9  bl 0x8243c708
	ctx.lr = 0x82442F54;
	sub_8243C708(ctx, base);
	// 82442F54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442F58: 409A0054  bne cr6, 0x82442fac
	if !ctx.cr[6].eq {
	pc = 0x82442FAC; continue 'dispatch;
	}
	// 82442F5C: 2F3D0000  cmpdi cr6, r29, 0
	ctx.cr[6].compare_i64(ctx.r[29].s64, 0, &mut ctx.xer);
	// 82442F60: 40980038  bge cr6, 0x82442f98
	if !ctx.cr[6].lt {
	pc = 0x82442F98; continue 'dispatch;
	}
	// 82442F64: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82442F68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442F6C: 419A002C  beq cr6, 0x82442f98
	if ctx.cr[6].eq {
	pc = 0x82442F98; continue 'dispatch;
	}
	// 82442F70: 897E0057  lbz r11, 0x57(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(87 as u32) ) } as u64;
	// 82442F74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82442F78: 409A0020  bne cr6, 0x82442f98
	if !ctx.cr[6].eq {
	pc = 0x82442F98; continue 'dispatch;
	}
	// 82442F7C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82442F80: 419A0048  beq cr6, 0x82442fc8
	if ctx.cr[6].eq {
	pc = 0x82442FC8; continue 'dispatch;
	}
	// 82442F84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82442F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442F8C: 4BFFDA3D  bl 0x824409c8
	ctx.lr = 0x82442F90;
	sub_824409C8(ctx, base);
	// 82442F90: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442F94: 419A0034  beq cr6, 0x82442fc8
	if ctx.cr[6].eq {
	pc = 0x82442FC8; continue 'dispatch;
	}
	// 82442F98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82442F9C: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 82442FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442FA4: 4BFF97AD  bl 0x8243c750
	ctx.lr = 0x82442FA8;
	sub_8243C750(ctx, base);
	// 82442FA8: 4800000C  b 0x82442fb4
	pc = 0x82442FB4; continue 'dispatch;
	// 82442FAC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82442FB0: 409A0018  bne cr6, 0x82442fc8
	if !ctx.cr[6].eq {
	pc = 0x82442FC8; continue 'dispatch;
	}
	// 82442FB4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82442FB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82442FBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82442FC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442FC4: 4BFFF475  bl 0x82442438
	ctx.lr = 0x82442FC8;
	sub_82442438(ctx, base);
	// 82442FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82442FCC: 480F213C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82442FD0 size=756
    let mut pc: u32 = 0x82442FD0;
    'dispatch: loop {
        match pc {
            0x82442FD0 => {
    //   block [0x82442FD0..0x824432C4)
	// 82442FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442FD4: 480F20D1  bl 0x825350a4
	ctx.lr = 0x82442FD8;
	sub_82535080(ctx, base);
	// 82442FD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442FDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442FE0: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82442FE4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442FE8: 397F2768  addi r11, r31, 0x2768
	ctx.r[11].s64 = ctx.r[31].s64 + 10088;
	// 82442FEC: 38A9000F  addi r5, r9, 0xf
	ctx.r[5].s64 = ctx.r[9].s64 + 15;
	// 82442FF0: 38CA000F  addi r6, r10, 0xf
	ctx.r[6].s64 = ctx.r[10].s64 + 15;
	// 82442FF4: 83DF2048  lwz r30, 0x2048(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82442FF8: 3B3F27E8  addi r25, r31, 0x27e8
	ctx.r[25].s64 = ctx.r[31].s64 + 10216;
	// 82442FFC: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82443000: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82443004: 3928000F  addi r9, r8, 0xf
	ctx.r[9].s64 = ctx.r[8].s64 + 15;
	// 82443008: 80EB001C  lwz r7, 0x1c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8244300C: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 82443010: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 82443014: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82443018: 7D482670  srawi r8, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 8244301C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82443020: 7D280194  addze r9, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82443024: 7CC82670  srawi r8, r6, 4
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[6].s32 >> 4) as i64;
	// 82443028: 38CA007F  addi r6, r10, 0x7f
	ctx.r[6].s64 = ctx.r[10].s64 + 127;
	// 8244302C: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82443030: 553D2036  slwi r29, r9, 4
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82443034: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82443038: 7D040E70  srawi r4, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 8244303C: 3868007F  addi r3, r8, 0x7f
	ctx.r[3].s64 = ctx.r[8].s64 + 127;
	// 82443040: 7D040194  addze r8, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82443044: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 82443048: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 8244304C: 7C880194  addze r4, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82443050: 7CA82670  srawi r8, r5, 4
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[5].s32 >> 4) as i64;
	// 82443054: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82443058: 55052036  slwi r5, r8, 4
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8244305C: 7CA50E70  srawi r5, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 82443060: 7CA50194  addze r5, r5
	tmp.s64 = ctx.r[5].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[5].u32);
	ctx.r[5].s64 = tmp.s64;
	// 82443064: 7C7C3E70  srawi r28, r3, 7
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[3].s32 >> 7) as i64;
	// 82443068: 7C6521D6  mullw r3, r5, r4
	ctx.r[3].s64 = (ctx.r[5].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8244306C: 7F9C0194  addze r28, r28
	tmp.s64 = ctx.r[28].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[28].u32);
	ctx.r[28].s64 = tmp.s64;
	// 82443070: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82443074: 7F7C41D6  mullw r27, r28, r8
	ctx.r[27].s64 = (ctx.r[28].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82443078: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8244307C: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 82443080: 7D4A3E70  srawi r10, r10, 7
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 7) as i64;
	// 82443084: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82443088: 7FA50E70  srawi r5, r29, 1
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[29].s32 >> 1) as i64;
	// 8244308C: 7CA50194  addze r5, r5
	tmp.s64 = ctx.r[5].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[5].u32);
	ctx.r[5].s64 = tmp.s64;
	// 82443090: 7CC63E70  srawi r6, r6, 7
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> 7) as i64;
	// 82443094: 7D4A29D6  mullw r10, r10, r5
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82443098: 7CC60194  addze r6, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[6].s64 = tmp.s64;
	// 8244309C: 7D2649D6  mullw r9, r6, r9
	ctx.r[9].s64 = (ctx.r[6].s32 as i64) * (ctx.r[9].s32 as i64);
	// 824430A0: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824430A4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824430A8: 554A482C  slwi r10, r10, 9
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(9);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824430AC: 38CA0100  addi r6, r10, 0x100
	ctx.r[6].s64 = ctx.r[10].s64 + 256;
	// 824430B0: 576A1838  slwi r10, r27, 3
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824430B4: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 824430B8: 5549482C  slwi r9, r10, 9
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(9);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824430BC: 39290100  addi r9, r9, 0x100
	ctx.r[9].s64 = ctx.r[9].s64 + 256;
	// 824430C0: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824430C4: 4099001C  ble cr6, 0x824430e0
	if !ctx.cr[6].gt {
	pc = 0x824430E0; continue 'dispatch;
	}
	// 824430C8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824430CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824430D0: 60840F17  ori r4, r4, 0xf17
	ctx.r[4].u64 = ctx.r[4].u64 | 3863;
	// 824430D4: 48004835  bl 0x82447908
	ctx.lr = 0x824430D8;
	sub_82447908(ctx, base);
	// 824430D8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824430DC: 480F2018  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 824430E0: 834B0020  lwz r26, 0x20(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824430E4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824430E8: 409A000C  bne cr6, 0x824430f4
	if !ctx.cr[6].eq {
	pc = 0x824430F4; continue 'dispatch;
	}
	// 824430EC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 824430F0: 480000E8  b 0x824431d8
	pc = 0x824431D8; continue 'dispatch;
	// 824430F4: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824430F8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824430FC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82443100: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82443104: 3908000F  addi r8, r8, 0xf
	ctx.r[8].s64 = ctx.r[8].s64 + 15;
	// 82443108: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8244310C: 7D062670  srawi r6, r8, 4
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[8].s32 >> 4) as i64;
	// 82443110: 390A0080  addi r8, r10, 0x80
	ctx.r[8].s64 = ctx.r[10].s64 + 128;
	// 82443114: 7D460194  addze r10, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82443118: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 8244311C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82443120: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82443124: 7D450E70  srawi r5, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82443128: 55382036  slwi r24, r9, 4
	ctx.r[24].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 8244312C: 7CA50194  addze r5, r5
	tmp.s64 = ctx.r[5].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[5].u32);
	ctx.r[5].s64 = tmp.s64;
	// 82443130: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 82443134: 38A5007F  addi r5, r5, 0x7f
	ctx.r[5].s64 = ctx.r[5].s64 + 127;
	// 82443138: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8244313C: 7CA53E70  srawi r5, r5, 7
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 7) as i64;
	// 82443140: 7CA50194  addze r5, r5
	tmp.s64 = ctx.r[5].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[5].u32);
	ctx.r[5].s64 = tmp.s64;
	// 82443144: 7F180E70  srawi r24, r24, 1
	ctx.xer.ca = (ctx.r[24].s32 < 0) && ((ctx.r[24].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[24].s32 >> 1) as i64;
	// 82443148: 7F180194  addze r24, r24
	tmp.s64 = ctx.r[24].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[24].u32);
	ctx.r[24].s64 = tmp.s64;
	// 8244314C: 7D573E70  srawi r23, r10, 7
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[23].s64 = (ctx.r[10].s32 >> 7) as i64;
	// 82443150: 7D45C1D6  mullw r10, r5, r24
	ctx.r[10].s64 = (ctx.r[5].s32 as i64) * (ctx.r[24].s32 as i64);
	// 82443154: 7CB70194  addze r5, r23
	tmp.s64 = ctx.r[23].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[23].u32);
	ctx.r[5].s64 = tmp.s64;
	// 82443158: 7D2549D6  mullw r9, r5, r9
	ctx.r[9].s64 = (ctx.r[5].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8244315C: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82443160: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82443164: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82443168: 394A0080  addi r10, r10, 0x80
	ctx.r[10].s64 = ctx.r[10].s64 + 128;
	// 8244316C: 7D4A39D6  mullw r10, r10, r7
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82443170: 7F065000  cmpw cr6, r6, r10
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82443174: 41990018  bgt cr6, 0x8244318c
	if ctx.cr[6].gt {
	pc = 0x8244318C; continue 'dispatch;
	}
	// 82443178: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8244317C: 7CC64214  add r6, r6, r8
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 82443180: 2F1D0010  cmpwi cr6, r29, 0x10
	ctx.cr[6].compare_i32(ctx.r[29].s32, 16, &mut ctx.xer);
	// 82443184: 4099FFEC  ble cr6, 0x82443170
	if !ctx.cr[6].gt {
	pc = 0x82443170; continue 'dispatch;
	}
	// 82443188: 48000008  b 0x82443190
	pc = 0x82443190; continue 'dispatch;
	// 8244318C: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82443190: 7F1D3800  cmpw cr6, r29, r7
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82443194: 4198FF34  blt cr6, 0x824430c8
	if ctx.cr[6].lt {
	pc = 0x824430C8; continue 'dispatch;
	}
	// 82443198: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244319C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824431A0: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824431A4: 917F278C  stw r11, 0x278c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10124 as u32), ctx.r[11].u32 ) };
	// 824431A8: 915F2790  stw r10, 0x2790(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10128 as u32), ctx.r[10].u32 ) };
	// 824431AC: 4099002C  ble cr6, 0x824431d8
	if !ctx.cr[6].gt {
	pc = 0x824431D8; continue 'dispatch;
	}
	// 824431B0: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 824431B4: 393F2794  addi r9, r31, 0x2794
	ctx.r[9].s64 = ctx.r[31].s64 + 10132;
	// 824431B8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 824431BC: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 824431C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824431C4: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 824431C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824431CC: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824431D0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824431D4: 409AFFE8  bne cr6, 0x824431bc
	if !ctx.cr[6].eq {
	pc = 0x824431BC; continue 'dispatch;
	}
	// 824431D8: 548A3830  slwi r10, r4, 7
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824431DC: 578B3830  slwi r11, r28, 7
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824431E0: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 824431E4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824431E8: 57695828  slwi r9, r27, 0xb
	ctx.r[9].u32 = ctx.r[27].u32.wrapping_shl(11);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824431EC: 54683830  slwi r8, r3, 7
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824431F0: 389F278C  addi r4, r31, 0x278c
	ctx.r[4].s64 = ctx.r[31].s64 + 10124;
	// 824431F4: B15E00D4  sth r10, 0xd4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(212 as u32), ctx.r[10].u16 ) };
	// 824431F8: B17E00D6  sth r11, 0xd6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(214 as u32), ctx.r[11].u16 ) };
	// 824431FC: 80FF278C  lwz r7, 0x278c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10124 as u32) ) } as u64;
	// 82443200: B17E00E6  sth r11, 0xe6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(230 as u32), ctx.r[11].u16 ) };
	// 82443204: 7D693A14  add r11, r9, r7
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82443208: B15E00E4  sth r10, 0xe4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[10].u16 ) };
	// 8244320C: 7D4B4214  add r10, r11, r8
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82443210: 90FE00D0  stw r7, 0xd0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(208 as u32), ctx.r[7].u32 ) };
	// 82443214: 917E00C8  stw r11, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82443218: 915E00CC  stw r10, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[10].u32 ) };
	// 8244321C: 817F2790  lwz r11, 0x2790(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10128 as u32) ) } as u64;
	// 82443220: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82443224: 917E00E0  stw r11, 0xe0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82443228: 7D6A4214  add r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8244322C: 915E00D8  stw r10, 0xd8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(216 as u32), ctx.r[10].u32 ) };
	// 82443230: 917E00DC  stw r11, 0xdc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 82443234: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82443238: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8244323C: 409A005C  bne cr6, 0x82443298
	if !ctx.cr[6].eq {
	pc = 0x82443298; continue 'dispatch;
	}
	// 82443240: 2F1D000E  cmpwi cr6, r29, 0xe
	ctx.cr[6].compare_i32(ctx.r[29].s32, 14, &mut ctx.xer);
	// 82443244: 41980008  blt cr6, 0x8244324c
	if ctx.cr[6].lt {
	pc = 0x8244324C; continue 'dispatch;
	}
	// 82443248: 3BA0000E  li r29, 0xe
	ctx.r[29].s64 = 14;
	// 8244324C: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82443250: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82443254: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82443258: 917F27D4  stw r11, 0x27d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10196 as u32), ctx.r[11].u32 ) };
	// 8244325C: 4BFFF755  bl 0x824429b0
	ctx.lr = 0x82443260;
	sub_824429B0(ctx, base);
	// 82443260: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82443264: 389F2794  addi r4, r31, 0x2794
	ctx.r[4].s64 = ctx.r[31].s64 + 10132;
	// 82443268: 38790200  addi r3, r25, 0x200
	ctx.r[3].s64 = ctx.r[25].s64 + 512;
	// 8244326C: 4BFFF745  bl 0x824429b0
	ctx.lr = 0x82443270;
	sub_824429B0(ctx, base);
	// 82443270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443274: 4800186D  bl 0x82444ae0
	ctx.lr = 0x82443278;
	sub_82444AE0(ctx, base);
	// 82443278: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244327C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443280: 917E00E8  stw r11, 0xe8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 82443284: 4800185D  bl 0x82444ae0
	ctx.lr = 0x82443288;
	sub_82444AE0(ctx, base);
	// 82443288: 907E00EC  stw r3, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[3].u32 ) };
	// 8244328C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443290: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82443294: 480F1E60  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 82443298: 2F1D0010  cmpwi cr6, r29, 0x10
	ctx.cr[6].compare_i32(ctx.r[29].s32, 16, &mut ctx.xer);
	// 8244329C: 41980008  blt cr6, 0x824432a4
	if ctx.cr[6].lt {
	pc = 0x824432A4; continue 'dispatch;
	}
	// 824432A0: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 824432A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824432A8: 93BF27D4  stw r29, 0x27d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10196 as u32), ctx.r[29].u32 ) };
	// 824432AC: 389F2794  addi r4, r31, 0x2794
	ctx.r[4].s64 = ctx.r[31].s64 + 10132;
	// 824432B0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824432B4: 4BFFF6FD  bl 0x824429b0
	ctx.lr = 0x824432B8;
	sub_824429B0(ctx, base);
	// 824432B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824432BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824432C0: 480F1E34  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824432C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824432C8 size=272
    let mut pc: u32 = 0x824432C8;
    'dispatch: loop {
        match pc {
            0x824432C8 => {
    //   block [0x824432C8..0x824433D8)
	// 824432C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824432CC: 480F1DED  bl 0x825350b8
	ctx.lr = 0x824432D0;
	sub_82535080(ctx, base);
	// 824432D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824432D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824432D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824432DC: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 824432E0: 83DF2048  lwz r30, 0x2048(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 824432E4: 3B9E0014  addi r28, r30, 0x14
	ctx.r[28].s64 = ctx.r[30].s64 + 20;
	// 824432E8: 4BFF9421  bl 0x8243c708
	ctx.lr = 0x824432EC;
	sub_8243C708(ctx, base);
	// 824432EC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824432F0: 409A000C  bne cr6, 0x824432fc
	if !ctx.cr[6].eq {
	pc = 0x824432FC; continue 'dispatch;
	}
	// 824432F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824432F8: 480F1E10  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824432FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82443300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443304: 48000E15  bl 0x82444118
	ctx.lr = 0x82443308;
	sub_82444118(ctx, base);
	// 82443308: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244330C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443310: 409A00C0  bne cr6, 0x824433d0
	if !ctx.cr[6].eq {
	pc = 0x824433D0; continue 'dispatch;
	}
	// 82443314: 38800027  li r4, 0x27
	ctx.r[4].s64 = 39;
	// 82443318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244331C: 4BFF93ED  bl 0x8243c708
	ctx.lr = 0x82443320;
	sub_8243C708(ctx, base);
	// 82443320: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82443324: 409A0010  bne cr6, 0x82443334
	if !ctx.cr[6].eq {
	pc = 0x82443334; continue 'dispatch;
	}
	// 82443328: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244332C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82443330: 480F1DD8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82443334: 897C0058  lbz r11, 0x58(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82443338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244333C: 419A0010  beq cr6, 0x8244334c
	if ctx.cr[6].eq {
	pc = 0x8244334C; continue 'dispatch;
	}
	// 82443340: 807E00F4  lwz r3, 0xf4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(244 as u32) ) } as u64;
	// 82443344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82443348: 480F1DC0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8244334C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443350: 83DC0018  lwz r30, 0x18(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82443354: 4BFFDD55  bl 0x824410a8
	ctx.lr = 0x82443358;
	sub_824410A8(ctx, base);
	// 82443358: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244335C: 409A0040  bne cr6, 0x8244339c
	if !ctx.cr[6].eq {
	pc = 0x8244339C; continue 'dispatch;
	}
	// 82443360: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82443364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443368: 4BFFDDA9  bl 0x82441110
	ctx.lr = 0x8244336C;
	sub_82441110(ctx, base);
	// 8244336C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443370: 409A002C  bne cr6, 0x8244339c
	if !ctx.cr[6].eq {
	pc = 0x8244339C; continue 'dispatch;
	}
	// 82443374: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82443378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244337C: 4BFFDDE5  bl 0x82441160
	ctx.lr = 0x82443380;
	sub_82441160(ctx, base);
	// 82443380: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443384: 409A0018  bne cr6, 0x8244339c
	if !ctx.cr[6].eq {
	pc = 0x8244339C; continue 'dispatch;
	}
	// 82443388: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244338C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443390: 4BFFDC11  bl 0x82440fa0
	ctx.lr = 0x82443394;
	sub_82440FA0(ctx, base);
	// 82443394: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443398: 419A000C  beq cr6, 0x824433a4
	if ctx.cr[6].eq {
	pc = 0x824433A4; continue 'dispatch;
	}
	// 8244339C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824433A0: 4800001C  b 0x824433bc
	pc = 0x824433BC; continue 'dispatch;
	// 824433A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824433A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824433AC: 4BFFF23D  bl 0x824425e8
	ctx.lr = 0x824433B0;
	sub_824425E8(ctx, base);
	// 824433B0: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824433B4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824433B8: 697E0001  xori r30, r11, 1
	ctx.r[30].u64 = ctx.r[11].u64 ^ 1;
	// 824433BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824433C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824433C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824433C8: 4BFFDC11  bl 0x82440fd8
	ctx.lr = 0x824433CC;
	sub_82440FD8(ctx, base);
	// 824433CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824433D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824433D4: 480F1D34  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824433D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824433D8 size=600
    let mut pc: u32 = 0x824433D8;
    'dispatch: loop {
        match pc {
            0x824433D8 => {
    //   block [0x824433D8..0x82443630)
	// 824433D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824433DC: 480F1CC5  bl 0x825350a0
	ctx.lr = 0x824433E0;
	sub_82535080(ctx, base);
	// 824433E0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824433E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824433E8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824433EC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824433F0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824433F4: 3ADF0950  addi r22, r31, 0x950
	ctx.r[22].s64 = ctx.r[31].s64 + 2384;
	// 824433F8: 83DF2048  lwz r30, 0x2048(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 824433FC: 3B1E0014  addi r24, r30, 0x14
	ctx.r[24].s64 = ctx.r[30].s64 + 20;
	// 82443400: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82443404: 82FE0000  lwz r23, 0(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443408: 4BFFDE49  bl 0x82441250
	ctx.lr = 0x8244340C;
	sub_82441250(ctx, base);
	// 8244340C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443410: 409A0214  bne cr6, 0x82443624
	if !ctx.cr[6].eq {
	pc = 0x82443624; continue 'dispatch;
	}
	// 82443414: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82443418: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8244341C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82443420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443424: 4BFFE0CD  bl 0x824414f0
	ctx.lr = 0x82443428;
	sub_824414F0(ctx, base);
	// 82443428: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244342C: 389F37F4  addi r4, r31, 0x37f4
	ctx.r[4].s64 = ctx.r[31].s64 + 14324;
	// 82443430: 807D0060  lwz r3, 0x60(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82443434: 4BFFE28D  bl 0x824416c0
	ctx.lr = 0x82443438;
	sub_824416C0(ctx, base);
	// 82443438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244343C: 4BFFE135  bl 0x82441570
	ctx.lr = 0x82443440;
	sub_82441570(ctx, base);
	// 82443440: 48006879  bl 0x82449cb8
	ctx.lr = 0x82443444;
	sub_82449CB8(ctx, base);
	// 82443444: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82443448: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8244344C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82443450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82443454: 4BFDFF6D  bl 0x824233c0
	ctx.lr = 0x82443458;
	sub_824233C0(ctx, base);
	// 82443458: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8244345C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82443460: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82443464: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82443468: 4800C801  bl 0x8244fc68
	ctx.lr = 0x8244346C;
	sub_8244FC68(ctx, base);
	// 8244346C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82443470: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82443474: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82443478: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244347C: 4BFDFF45  bl 0x824233c0
	ctx.lr = 0x82443480;
	sub_824233C0(ctx, base);
	// 82443480: 7F9A1850  subf r28, r26, r3
	ctx.r[28].s64 = ctx.r[3].s64 - ctx.r[26].s64;
	// 82443484: 48006835  bl 0x82449cb8
	ctx.lr = 0x82443488;
	sub_82449CB8(ctx, base);
	// 82443488: 81780018  lwz r11, 0x18(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 8244348C: 7C9B1850  subf r4, r27, r3
	ctx.r[4].s64 = ctx.r[3].s64 - ctx.r[27].s64;
	// 82443490: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82443494: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82443498: 386B2688  addi r3, r11, 0x2688
	ctx.r[3].s64 = ctx.r[11].s64 + 9864;
	// 8244349C: 48006955  bl 0x82449df0
	ctx.lr = 0x824434A0;
	sub_82449DF0(ctx, base);
	// 824434A0: 815F0A04  lwz r10, 0xa04(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2564 as u32) ) } as u64;
	// 824434A4: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 824434A8: 3CC0FF00  lis r6, -0x100
	ctx.r[6].s64 = -16777216;
	// 824434AC: 817F0A08  lwz r11, 0xa08(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2568 as u32) ) } as u64;
	// 824434B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824434B4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824434B8: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 824434BC: 60C60F06  ori r6, r6, 0xf06
	ctx.r[6].u64 = ctx.r[6].u64 | 3846;
	// 824434C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824434C4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824434C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824434CC: 913F0A04  stw r9, 0xa04(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2564 as u32), ctx.r[9].u32 ) };
	// 824434D0: 917F0A08  stw r11, 0xa08(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2568 as u32), ctx.r[11].u32 ) };
	// 824434D4: 4BFFD3D5  bl 0x824408a8
	ctx.lr = 0x824434D8;
	sub_824408A8(ctx, base);
	// 824434D8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824434DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824434E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824434E4: 4BFFD36D  bl 0x82440850
	ctx.lr = 0x824434E8;
	sub_82440850(ctx, base);
	// 824434E8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824434EC: 419A0018  beq cr6, 0x82443504
	if ctx.cr[6].eq {
	pc = 0x82443504; continue 'dispatch;
	}
	// 824434F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824434F4: 4800167D  bl 0x82444b70
	ctx.lr = 0x824434F8;
	sub_82444B70(ctx, base);
	// 824434F8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824434FC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82443500: 480F1BF0  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 82443504: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82443508: 40990108  ble cr6, 0x82443610
	if !ctx.cr[6].gt {
	pc = 0x82443610; continue 'dispatch;
	}
	// 8244350C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82443510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443514: 4800151D  bl 0x82444a30
	ctx.lr = 0x82443518;
	sub_82444A30(ctx, base);
	// 82443518: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244351C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443520: 4BFFF2E9  bl 0x82442808
	ctx.lr = 0x82443524;
	sub_82442808(ctx, base);
	// 82443524: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82443528: 917D0054  stw r11, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8244352C: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82443530: 817E00FC  lwz r11, 0xfc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(252 as u32) ) } as u64;
	// 82443534: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82443538: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244353C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82443540: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82443544: 917D004C  stw r11, 0x4c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82443548: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8244354C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82443550: 917D0050  stw r11, 0x50(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82443554: A16100A0  lhz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82443558: B17D00FC  sth r11, 0xfc(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(252 as u32), ctx.r[11].u16 ) };
	// 8244355C: 815E00F0  lwz r10, 0xf0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 82443560: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82443564: 409A0010  bne cr6, 0x82443574
	if !ctx.cr[6].eq {
	pc = 0x82443574; continue 'dispatch;
	}
	// 82443568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244356C: 4BFFDB25  bl 0x82441090
	ctx.lr = 0x82443570;
	sub_82441090(ctx, base);
	// 82443570: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82443574: 81780038  lwz r11, 0x38(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(56 as u32) ) } as u64;
	// 82443578: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8244357C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82443580: 419A0014  beq cr6, 0x82443594
	if ctx.cr[6].eq {
	pc = 0x82443594; continue 'dispatch;
	}
	// 82443584: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82443588: 409A000C  bne cr6, 0x82443594
	if !ctx.cr[6].eq {
	pc = 0x82443594; continue 'dispatch;
	}
	// 8244358C: 93BE00F0  stw r29, 0xf0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(240 as u32), ctx.r[29].u32 ) };
	// 82443590: 48000008  b 0x82443598
	pc = 0x82443598; continue 'dispatch;
	// 82443594: 939E00F0  stw r28, 0xf0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(240 as u32), ctx.r[28].u32 ) };
	// 82443598: 817E00F0  lwz r11, 0xf0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 8244359C: 939E00F4  stw r28, 0xf4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(244 as u32), ctx.r[28].u32 ) };
	// 824435A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824435A4: 939E00F8  stw r28, 0xf8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(248 as u32), ctx.r[28].u32 ) };
	// 824435A8: 409A004C  bne cr6, 0x824435f4
	if !ctx.cr[6].eq {
	pc = 0x824435F4; continue 'dispatch;
	}
	// 824435AC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 824435B0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824435B4: 409A0024  bne cr6, 0x824435d8
	if !ctx.cr[6].eq {
	pc = 0x824435D8; continue 'dispatch;
	}
	// 824435B8: 81780018  lwz r11, 0x18(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 824435BC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824435C0: 419A000C  beq cr6, 0x824435cc
	if ctx.cr[6].eq {
	pc = 0x824435CC; continue 'dispatch;
	}
	// 824435C4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824435C8: 409A0010  bne cr6, 0x824435d8
	if !ctx.cr[6].eq {
	pc = 0x824435D8; continue 'dispatch;
	}
	// 824435CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824435D0: 480015D1  bl 0x82444ba0
	ctx.lr = 0x824435D4;
	sub_82444BA0(ctx, base);
	// 824435D4: 4800000C  b 0x824435e0
	pc = 0x824435E0; continue 'dispatch;
	// 824435D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824435DC: 480015AD  bl 0x82444b88
	ctx.lr = 0x824435E0;
	sub_82444B88(ctx, base);
	// 824435E0: 38B6000C  addi r5, r22, 0xc
	ctx.r[5].s64 = ctx.r[22].s64 + 12;
	// 824435E4: 38960008  addi r4, r22, 8
	ctx.r[4].s64 = ctx.r[22].s64 + 8;
	// 824435E8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 824435EC: 48009295  bl 0x8244c880
	ctx.lr = 0x824435F0;
	sub_8244C880(ctx, base);
	// 824435F0: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 824435F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824435F8: 80B80018  lwz r5, 0x18(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 824435FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443600: 4BFF6619  bl 0x82439c18
	ctx.lr = 0x82443604;
	sub_82439C18(ctx, base);
	// 82443604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443608: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 8244360C: 480F1AE4  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 82443610: 817E00F0  lwz r11, 0xf0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 82443614: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82443618: 409A000C  bne cr6, 0x82443624
	if !ctx.cr[6].eq {
	pc = 0x82443624; continue 'dispatch;
	}
	// 8244361C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82443620: 48001551  bl 0x82444b70
	ctx.lr = 0x82443624;
	sub_82444B70(ctx, base);
	// 82443624: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443628: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 8244362C: 480F1AC4  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443630 size=232
    let mut pc: u32 = 0x82443630;
    'dispatch: loop {
        match pc {
            0x82443630 => {
    //   block [0x82443630..0x82443718)
	// 82443630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82443634: 480F1A85  bl 0x825350b8
	ctx.lr = 0x82443638;
	sub_82535080(ctx, base);
	// 82443638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244363C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82443640: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82443644: 480010F5  bl 0x82444738
	ctx.lr = 0x82443648;
	sub_82444738(ctx, base);
	// 82443648: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244364C: 409A00C4  bne cr6, 0x82443710
	if !ctx.cr[6].eq {
	pc = 0x82443710; continue 'dispatch;
	}
	// 82443650: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82443654: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82443658: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 8244365C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82443660: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82443664: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82443668: 389D2794  addi r4, r29, 0x2794
	ctx.r[4].s64 = ctx.r[29].s64 + 10132;
	// 8244366C: 93DD27E0  stw r30, 0x27e0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(10208 as u32), ctx.r[30].u32 ) };
	// 82443670: 387D27E8  addi r3, r29, 0x27e8
	ctx.r[3].s64 = ctx.r[29].s64 + 10216;
	// 82443674: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82443678: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8244367C: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82443680: 913F00C4  stw r9, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[9].u32 ) };
	// 82443684: 93DD27D8  stw r30, 0x27d8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(10200 as u32), ctx.r[30].u32 ) };
	// 82443688: 93DD27DC  stw r30, 0x27dc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(10204 as u32), ctx.r[30].u32 ) };
	// 8244368C: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 82443690: 93DF00EC  stw r30, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u32 ) };
	// 82443694: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 82443698: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 8244369C: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 824436A0: 93DF00FC  stw r30, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 824436A4: 4BFFF30D  bl 0x824429b0
	ctx.lr = 0x824436A8;
	sub_824429B0(ctx, base);
	// 824436A8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 824436AC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 824436B0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 824436B4: 4BFFE0CD  bl 0x82441780
	ctx.lr = 0x824436B8;
	sub_82441780(ctx, base);
	// 824436B8: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 824436BC: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 824436C0: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 824436C4: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 824436C8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 824436CC: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 824436D0: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 824436D4: 4BFFD225  bl 0x824408f8
	ctx.lr = 0x824436D8;
	sub_824408F8(ctx, base);
	// 824436D8: 387D37E8  addi r3, r29, 0x37e8
	ctx.r[3].s64 = ctx.r[29].s64 + 14312;
	// 824436DC: 48001205  bl 0x824448e0
	ctx.lr = 0x824436E0;
	sub_824448E0(ctx, base);
	// 824436E0: 393D2848  addi r9, r29, 0x2848
	ctx.r[9].s64 = ctx.r[29].s64 + 10312;
	// 824436E4: 395D37FC  addi r10, r29, 0x37fc
	ctx.r[10].s64 = ctx.r[29].s64 + 14332;
	// 824436E8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824436EC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824436F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824436F4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824436F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824436FC: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82443700: 39290100  addi r9, r9, 0x100
	ctx.r[9].s64 = ctx.r[9].s64 + 256;
	// 82443704: 409AFFE8  bne cr6, 0x824436ec
	if !ctx.cr[6].eq {
	pc = 0x824436EC; continue 'dispatch;
	}
	// 82443708: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244370C: FB9F0100  std r28, 0x100(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[28].u64 ) };
	// 82443710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82443714: 480F19F4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443718 size=128
    let mut pc: u32 = 0x82443718;
    'dispatch: loop {
        match pc {
            0x82443718 => {
    //   block [0x82443718..0x82443798)
	// 82443718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244371C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82443720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82443724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82443728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244372C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82443730: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82443734: 83DF2048  lwz r30, 0x2048(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82443738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244373C: 4BFFF39D  bl 0x82442ad8
	ctx.lr = 0x82443740;
	sub_82442AD8(ctx, base);
	// 82443740: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443744: 409A003C  bne cr6, 0x82443780
	if !ctx.cr[6].eq {
	pc = 0x82443780; continue 'dispatch;
	}
	// 82443748: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8244374C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443750: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82443754: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82443758: 419A001C  beq cr6, 0x82443774
	if ctx.cr[6].eq {
	pc = 0x82443774; continue 'dispatch;
	}
	// 8244375C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82443760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443764: 4BFF8FA5  bl 0x8243c708
	ctx.lr = 0x82443768;
	sub_8243C708(ctx, base);
	// 82443768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244376C: 396000C8  li r11, 0xc8
	ctx.r[11].s64 = 200;
	// 82443770: 409A0008  bne cr6, 0x82443778
	if !ctx.cr[6].eq {
	pc = 0x82443778; continue 'dispatch;
	}
	// 82443774: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 82443778: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244377C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82443784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82443788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244378C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82443790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82443794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443798 size=108
    let mut pc: u32 = 0x82443798;
    'dispatch: loop {
        match pc {
            0x82443798 => {
    //   block [0x82443798..0x82443804)
	// 82443798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244379C: 480F1921  bl 0x825350bc
	ctx.lr = 0x824437A0;
	sub_82535080(ctx, base);
	// 824437A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824437A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824437A8: 83DF2054  lwz r30, 0x2054(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8276 as u32) ) } as u64;
	// 824437AC: 83BF2050  lwz r29, 0x2050(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 824437B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824437B4: 48004C15  bl 0x824483c8
	ctx.lr = 0x824437B8;
	sub_824483C8(ctx, base);
	// 824437B8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824437BC: 419A0040  beq cr6, 0x824437fc
	if ctx.cr[6].eq {
	pc = 0x824437FC; continue 'dispatch;
	}
	// 824437C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824437C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824437C8: 48004C01  bl 0x824483c8
	ctx.lr = 0x824437CC;
	sub_824483C8(ctx, base);
	// 824437CC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824437D0: 409A002C  bne cr6, 0x824437fc
	if !ctx.cr[6].eq {
	pc = 0x824437FC; continue 'dispatch;
	}
	// 824437D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824437D8: 4BFFF3A9  bl 0x82442b80
	ctx.lr = 0x824437DC;
	sub_82442B80(ctx, base);
	// 824437DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824437E0: 419A001C  beq cr6, 0x824437fc
	if ctx.cr[6].eq {
	pc = 0x824437FC; continue 'dispatch;
	}
	// 824437E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824437E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824437EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824437F0: 48004BC1  bl 0x824483b0
	ctx.lr = 0x824437F4;
	sub_824483B0(ctx, base);
	// 824437F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824437F8: 4BFFCAF1  bl 0x824402e8
	ctx.lr = 0x824437FC;
	sub_824402E8(ctx, base);
	// 824437FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82443800: 480F190C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443808 size=80
    let mut pc: u32 = 0x82443808;
    'dispatch: loop {
        match pc {
            0x82443808 => {
    //   block [0x82443808..0x82443858)
	// 82443808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244380C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82443810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82443814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82443818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244381C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82443820: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82443824: 4BFFF5FD  bl 0x82442e20
	ctx.lr = 0x82443828;
	sub_82442E20(ctx, base);
	// 82443828: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8244382C: 419A0014  beq cr6, 0x82443840
	if ctx.cr[6].eq {
	pc = 0x82443840; continue 'dispatch;
	}
	// 82443830: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82443834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443838: 4BFFE761  bl 0x82441f98
	ctx.lr = 0x8244383C;
	sub_82441F98(ctx, base);
	// 8244383C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82443844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82443848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244384C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82443850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82443854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443858 size=256
    let mut pc: u32 = 0x82443858;
    'dispatch: loop {
        match pc {
            0x82443858 => {
    //   block [0x82443858..0x82443958)
	// 82443858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244385C: 480F1855  bl 0x825350b0
	ctx.lr = 0x82443860;
	sub_82535080(ctx, base);
	// 82443860: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82443864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82443868: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244386C: 3BBF090C  addi r29, r31, 0x90c
	ctx.r[29].s64 = ctx.r[31].s64 + 2316;
	// 82443870: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82443874: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82443878: 839F2048  lwz r28, 0x2048(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 8244387C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82443880: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443884: 419A0010  beq cr6, 0x82443894
	if ctx.cr[6].eq {
	pc = 0x82443894; continue 'dispatch;
	}
	// 82443888: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244388C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82443890: 480F1870  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82443894: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82443898: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244389C: 4800BC3D  bl 0x8244f4d8
	ctx.lr = 0x824438A0;
	sub_8244F4D8(ctx, base);
	// 824438A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824438A4: 419A001C  beq cr6, 0x824438c0
	if ctx.cr[6].eq {
	pc = 0x824438C0; continue 'dispatch;
	}
	// 824438A8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824438AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824438B0: 60840F16  ori r4, r4, 0xf16
	ctx.r[4].u64 = ctx.r[4].u64 | 3862;
	// 824438B4: 48004055  bl 0x82447908
	ctx.lr = 0x824438B8;
	sub_82447908(ctx, base);
	// 824438B8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824438BC: 480F1844  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824438C0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824438C4: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 824438C8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824438CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824438D0: 4800BC69  bl 0x8244f538
	ctx.lr = 0x824438D4;
	sub_8244F538(ctx, base);
	// 824438D4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 824438D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824438DC: 4BFF8E2D  bl 0x8243c708
	ctx.lr = 0x824438E0;
	sub_8243C708(ctx, base);
	// 824438E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824438E4: 409A000C  bne cr6, 0x824438f0
	if !ctx.cr[6].eq {
	pc = 0x824438F0; continue 'dispatch;
	}
	// 824438E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824438EC: 48000030  b 0x8244391c
	pc = 0x8244391C; continue 'dispatch;
	// 824438F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824438F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824438F8: 48004931  bl 0x82448228
	ctx.lr = 0x824438FC;
	sub_82448228(ctx, base);
	// 824438FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443900: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82443904: 409A000C  bne cr6, 0x82443910
	if !ctx.cr[6].eq {
	pc = 0x82443910; continue 'dispatch;
	}
	// 82443908: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244390C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82443910: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82443914: 41980008  blt cr6, 0x8244391c
	if ctx.cr[6].lt {
	pc = 0x8244391C; continue 'dispatch;
	}
	// 82443918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244391C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82443920: 917C009C  stw r11, 0x9c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82443924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443928: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8244392C: 4BFFEC15  bl 0x82442540
	ctx.lr = 0x82443930;
	sub_82442540(ctx, base);
	// 82443930: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82443934: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82443938: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244393C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82443940: 4BFFD629  bl 0x82440f68
	ctx.lr = 0x82443944;
	sub_82440F68(ctx, base);
	// 82443944: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82443948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244394C: 4BFFF685  bl 0x82442fd0
	ctx.lr = 0x82443950;
	sub_82442FD0(ctx, base);
	// 82443950: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82443954: 480F17AC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443958 size=264
    let mut pc: u32 = 0x82443958;
    'dispatch: loop {
        match pc {
            0x82443958 => {
    //   block [0x82443958..0x82443A60)
	// 82443958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244395C: 480F1761  bl 0x825350bc
	ctx.lr = 0x82443960;
	sub_82535080(ctx, base);
	// 82443960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82443964: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82443968: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244396C: 4BFF8D9D  bl 0x8243c708
	ctx.lr = 0x82443970;
	sub_8243C708(ctx, base);
	// 82443970: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443974: 419A00E0  beq cr6, 0x82443a54
	if ctx.cr[6].eq {
	pc = 0x82443A54; continue 'dispatch;
	}
	// 82443978: 3BBF2430  addi r29, r31, 0x2430
	ctx.r[29].s64 = ctx.r[31].s64 + 9264;
	// 8244397C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443980: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82443984: 93BF2048  stw r29, 0x2048(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8264 as u32), ctx.r[29].u32 ) };
	// 82443988: 4BFFFCA9  bl 0x82443630
	ctx.lr = 0x8244398C;
	sub_82443630(ctx, base);
	// 8244398C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443990: 409A00C8  bne cr6, 0x82443a58
	if !ctx.cr[6].eq {
	pc = 0x82443A58; continue 'dispatch;
	}
	// 82443994: 48009485  bl 0x8244ce18
	ctx.lr = 0x82443998;
	sub_8244CE18(ctx, base);
	// 82443998: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244399C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824439A0: 409A0018  bne cr6, 0x824439b8
	if !ctx.cr[6].eq {
	pc = 0x824439B8; continue 'dispatch;
	}
	// 824439A4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824439A8: 60840F0A  ori r4, r4, 0xf0a
	ctx.r[4].u64 = ctx.r[4].u64 | 3850;
	// 824439AC: 48003F5D  bl 0x82447908
	ctx.lr = 0x824439B0;
	sub_82447908(ctx, base);
	// 824439B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824439B4: 480F1758  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824439B8: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824439BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824439C0: 388B1790  addi r4, r11, 0x1790
	ctx.r[4].s64 = ctx.r[11].s64 + 6032;
	// 824439C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824439C8: 4800C571  bl 0x8244ff38
	ctx.lr = 0x824439CC;
	sub_8244FF38(ctx, base);
	// 824439CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824439D0: 419A0024  beq cr6, 0x824439f4
	if ctx.cr[6].eq {
	pc = 0x824439F4; continue 'dispatch;
	}
	// 824439D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824439D8: 4BFFDDD9  bl 0x824417b0
	ctx.lr = 0x824439DC;
	sub_824417B0(ctx, base);
	// 824439DC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824439E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824439E4: 60840F0B  ori r4, r4, 0xf0b
	ctx.r[4].u64 = ctx.r[4].u64 | 3851;
	// 824439E8: 48003F21  bl 0x82447908
	ctx.lr = 0x824439EC;
	sub_82447908(ctx, base);
	// 824439EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824439F0: 480F171C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824439F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824439F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824439FC: 4BFF8D0D  bl 0x8243c708
	ctx.lr = 0x82443A00;
	sub_8243C708(ctx, base);
	// 82443A00: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82443A04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82443A08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82443A0C: 48009165  bl 0x8244cb70
	ctx.lr = 0x82443A10;
	sub_8244CB70(ctx, base);
	// 82443A10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82443A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443A18: 4BFF8CF1  bl 0x8243c708
	ctx.lr = 0x82443A1C;
	sub_8243C708(ctx, base);
	// 82443A1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82443A20: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82443A24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82443A28: 48009149  bl 0x8244cb70
	ctx.lr = 0x82443A2C;
	sub_8244CB70(ctx, base);
	// 82443A2C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82443A30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82443A34: 80BF0038  lwz r5, 0x38(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82443A38: 48009139  bl 0x8244cb70
	ctx.lr = 0x82443A3C;
	sub_8244CB70(ctx, base);
	// 82443A3C: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82443A40: 4BFF62A9  bl 0x82439ce8
	ctx.lr = 0x82443A44;
	sub_82439CE8(ctx, base);
	// 82443A44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443A48: 419A000C  beq cr6, 0x82443a54
	if ctx.cr[6].eq {
	pc = 0x82443A54; continue 'dispatch;
	}
	// 82443A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443A50: 48001691  bl 0x824450e0
	ctx.lr = 0x82443A54;
	sub_824450E0(ctx, base);
	// 82443A54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443A58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82443A5C: 480F16B0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443A60 size=712
    let mut pc: u32 = 0x82443A60;
    'dispatch: loop {
        match pc {
            0x82443A60 => {
    //   block [0x82443A60..0x82443D28)
	// 82443A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82443A64: 480F1639  bl 0x8253509c
	ctx.lr = 0x82443A68;
	sub_82535080(ctx, base);
	// 82443A68: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82443A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82443A70: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82443A74: 397F37E8  addi r11, r31, 0x37e8
	ctx.r[11].s64 = ctx.r[31].s64 + 14312;
	// 82443A78: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82443A7C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82443A80: 83DF2048  lwz r30, 0x2048(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82443A84: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82443A88: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82443A8C: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82443A90: 3B7E0014  addi r27, r30, 0x14
	ctx.r[27].s64 = ctx.r[30].s64 + 20;
	// 82443A94: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82443A98: 3AEB0010  addi r23, r11, 0x10
	ctx.r[23].s64 = ctx.r[11].s64 + 16;
	// 82443A9C: 833E0000  lwz r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443AA0: 92CB0010  stw r22, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[22].u32 ) };
	// 82443AA4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82443AA8: 48009529  bl 0x8244cfd0
	ctx.lr = 0x82443AAC;
	sub_8244CFD0(ctx, base);
	// 82443AAC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82443AB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82443AB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82443AB8: 4BFDF909  bl 0x824233c0
	ctx.lr = 0x82443ABC;
	sub_824233C0(ctx, base);
	// 82443ABC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82443AC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82443AC4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82443AC8: 4800B341  bl 0x8244ee08
	ctx.lr = 0x82443ACC;
	sub_8244EE08(ctx, base);
	// 82443ACC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82443AD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82443AD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82443AD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82443ADC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443AE0: 4BFDF8E1  bl 0x824233c0
	ctx.lr = 0x82443AE4;
	sub_824233C0(ctx, base);
	// 82443AE4: 7FBA1850  subf r29, r26, r3
	ctx.r[29].s64 = ctx.r[3].s64 - ctx.r[26].s64;
	// 82443AE8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443AEC: 3CC0FF00  lis r6, -0x100
	ctx.r[6].s64 = -16777216;
	// 82443AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443AF4: 60C60F04  ori r6, r6, 0xf04
	ctx.r[6].u64 = ctx.r[6].u64 | 3844;
	// 82443AF8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82443AFC: 4BFFCDAD  bl 0x824408a8
	ctx.lr = 0x82443B00;
	sub_824408A8(ctx, base);
	// 82443B00: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82443B04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82443B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443B0C: 4BFFCD45  bl 0x82440850
	ctx.lr = 0x82443B10;
	sub_82440850(ctx, base);
	// 82443B10: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82443B14: 419A0010  beq cr6, 0x82443b24
	if ctx.cr[6].eq {
	pc = 0x82443B24; continue 'dispatch;
	}
	// 82443B18: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82443B1C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82443B20: 480F15CC  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 82443B24: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443B28: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82443B2C: 409A0010  bne cr6, 0x82443b3c
	if !ctx.cr[6].eq {
	pc = 0x82443B3C; continue 'dispatch;
	}
	// 82443B30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443B34: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82443B38: 480F15B4  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 82443B3C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82443B40: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82443B44: 4800B92D  bl 0x8244f470
	ctx.lr = 0x82443B48;
	sub_8244F470(ctx, base);
	// 82443B48: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443B4C: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82443B50: 419A001C  beq cr6, 0x82443b6c
	if ctx.cr[6].eq {
	pc = 0x82443B6C; continue 'dispatch;
	}
	// 82443B54: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82443B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443B5C: 60840F05  ori r4, r4, 0xf05
	ctx.r[4].u64 = ctx.r[4].u64 | 3845;
	// 82443B60: 48003DA9  bl 0x82447908
	ctx.lr = 0x82443B64;
	sub_82447908(ctx, base);
	// 82443B64: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82443B68: 480F1584  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 82443B6C: 56BA0672  rlwinm r26, r21, 0, 0x19, 0x19
	ctx.r[26].u64 = ctx.r[21].u32 as u64 & 0xFFFFFFFFu64;
	// 82443B70: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82443B74: 419A0030  beq cr6, 0x82443ba4
	if ctx.cr[6].eq {
	pc = 0x82443BA4; continue 'dispatch;
	}
	// 82443B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443B7C: 80BB0004  lwz r5, 4(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82443B80: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443B84: 4800110D  bl 0x82444c90
	ctx.lr = 0x82443B88;
	sub_82444C90(ctx, base);
	// 82443B88: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443B8C: 419A0018  beq cr6, 0x82443ba4
	if ctx.cr[6].eq {
	pc = 0x82443BA4; continue 'dispatch;
	}
	// 82443B90: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 82443B94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443B98: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443B9C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82443BA0: 480F154C  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 82443BA4: 815B0018  lwz r10, 0x18(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82443BA8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82443BAC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82443BB0: 409A000C  bne cr6, 0x82443bbc
	if !ctx.cr[6].eq {
	pc = 0x82443BBC; continue 'dispatch;
	}
	// 82443BB4: 92DE00FC  stw r22, 0xfc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(252 as u32), ctx.r[22].u32 ) };
	// 82443BB8: 48000058  b 0x82443c10
	pc = 0x82443C10; continue 'dispatch;
	// 82443BBC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82443BC0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82443BC4: 409A004C  bne cr6, 0x82443c10
	if !ctx.cr[6].eq {
	pc = 0x82443C10; continue 'dispatch;
	}
	// 82443BC8: 817E00EC  lwz r11, 0xec(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 82443BCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82443BD0: 419A0040  beq cr6, 0x82443c10
	if ctx.cr[6].eq {
	pc = 0x82443C10; continue 'dispatch;
	}
	// 82443BD4: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82443BD8: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82443BDC: 409A001C  bne cr6, 0x82443bf8
	if !ctx.cr[6].eq {
	pc = 0x82443BF8; continue 'dispatch;
	}
	// 82443BE0: 815B0014  lwz r10, 0x14(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82443BE4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82443BE8: 40980028  bge cr6, 0x82443c10
	if !ctx.cr[6].lt {
	pc = 0x82443C10; continue 'dispatch;
	}
	// 82443BEC: 2F0B0200  cmpwi cr6, r11, 0x200
	ctx.cr[6].compare_i32(ctx.r[11].s32, 512, &mut ctx.xer);
	// 82443BF0: 40980020  bge cr6, 0x82443c10
	if !ctx.cr[6].lt {
	pc = 0x82443C10; continue 'dispatch;
	}
	// 82443BF4: 48000018  b 0x82443c0c
	pc = 0x82443C0C; continue 'dispatch;
	// 82443BF8: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82443BFC: 409A0014  bne cr6, 0x82443c10
	if !ctx.cr[6].eq {
	pc = 0x82443C10; continue 'dispatch;
	}
	// 82443C00: 815B0014  lwz r10, 0x14(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82443C04: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82443C08: 41980008  blt cr6, 0x82443c10
	if ctx.cr[6].lt {
	pc = 0x82443C10; continue 'dispatch;
	}
	// 82443C0C: 93BE00FC  stw r29, 0xfc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(252 as u32), ctx.r[29].u32 ) };
	// 82443C10: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82443C14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82443C18: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82443C1C: 480093CD  bl 0x8244cfe8
	ctx.lr = 0x82443C20;
	sub_8244CFE8(ctx, base);
	// 82443C20: 817B0030  lwz r11, 0x30(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82443C24: 815E0094  lwz r10, 0x94(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 82443C28: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82443C2C: 419A0010  beq cr6, 0x82443c3c
	if ctx.cr[6].eq {
	pc = 0x82443C3C; continue 'dispatch;
	}
	// 82443C30: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82443C34: 93BE0098  stw r29, 0x98(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), ctx.r[29].u32 ) };
	// 82443C38: 48000008  b 0x82443c40
	pc = 0x82443C40; continue 'dispatch;
	// 82443C3C: 92DE0098  stw r22, 0x98(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), ctx.r[22].u32 ) };
	// 82443C40: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82443C44: 419A0048  beq cr6, 0x82443c8c
	if ctx.cr[6].eq {
	pc = 0x82443C8C; continue 'dispatch;
	}
	// 82443C48: 83BF0D54  lwz r29, 0xd54(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3412 as u32) ) } as u64;
	// 82443C4C: 839F0D58  lwz r28, 0xd58(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3416 as u32) ) } as u64;
	// 82443C50: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82443C54: 419A0038  beq cr6, 0x82443c8c
	if ctx.cr[6].eq {
	pc = 0x82443C8C; continue 'dispatch;
	}
	// 82443C58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82443C5C: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82443C60: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443C64: 4800B69D  bl 0x8244f300
	ctx.lr = 0x82443C68;
	sub_8244F300(ctx, base);
	// 82443C68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82443C6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82443C70: 419A001C  beq cr6, 0x82443c8c
	if ctx.cr[6].eq {
	pc = 0x82443C8C; continue 'dispatch;
	}
	// 82443C74: 80980000  lwz r4, 0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443C78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82443C7C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82443C80: 38AB0004  addi r5, r11, 4
	ctx.r[5].s64 = ctx.r[11].s64 + 4;
	// 82443C84: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82443C88: 4E800421  bctrl
	ctx.lr = 0x82443C8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82443C8C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82443C90: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82443C94: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82443C98: 4800B669  bl 0x8244f300
	ctx.lr = 0x82443C9C;
	sub_8244F300(ctx, base);
	// 82443C9C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82443CA0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82443CA4: 811E0098  lwz r8, 0x98(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82443CA8: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82443CAC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82443CB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443CB4: 4BFFF205  bl 0x82442eb8
	ctx.lr = 0x82443CB8;
	sub_82442EB8(ctx, base);
	// 82443CB8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82443CBC: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82443CC0: 7D6BA838  and r11, r11, r21
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[21].u64;
	// 82443CC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443CC8: F95E0100  std r10, 0x100(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(256 as u32), ctx.r[10].u64 ) };
	// 82443CCC: 419AFE64  beq cr6, 0x82443b30
	if ctx.cr[6].eq {
	pc = 0x82443B30; continue 'dispatch;
	}
	// 82443CD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82443CD4: 80BE0098  lwz r5, 0x98(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82443CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443CDC: 4BFFE55D  bl 0x82442238
	ctx.lr = 0x82443CE0;
	sub_82442238(ctx, base);
	// 82443CE0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82443CE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443CE8: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82443CEC: 80DE0098  lwz r6, 0x98(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82443CF0: 4BFFF241  bl 0x82442f30
	ctx.lr = 0x82443CF4;
	sub_82442F30(ctx, base);
	// 82443CF4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82443CF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443CFC: 4BFFD0B5  bl 0x82440db0
	ctx.lr = 0x82443D00;
	sub_82440DB0(ctx, base);
	// 82443D00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82443D04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443D08: 4BFFD159  bl 0x82440e60
	ctx.lr = 0x82443D0C;
	sub_82440E60(ctx, base);
	// 82443D0C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82443D10: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82443D14: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82443D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443D1C: 4BFFFB3D  bl 0x82443858
	ctx.lr = 0x82443D20;
	sub_82443858(ctx, base);
	// 82443D20: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82443D24: 480F13C8  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443D28 size=648
    let mut pc: u32 = 0x82443D28;
    'dispatch: loop {
        match pc {
            0x82443D28 => {
    //   block [0x82443D28..0x82443FB0)
	// 82443D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82443D2C: 480F137D  bl 0x825350a8
	ctx.lr = 0x82443D30;
	sub_82535080(ctx, base);
	// 82443D30: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82443D34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82443D38: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82443D3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82443D40: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82443D44: 3BBF0950  addi r29, r31, 0x950
	ctx.r[29].s64 = ctx.r[31].s64 + 2384;
	// 82443D48: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82443D4C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443D50: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82443D54: 837F2048  lwz r27, 0x2048(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82443D58: 809F2050  lwz r4, 0x2050(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82443D5C: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82443D60: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82443D64: 2F0B00CC  cmpwi cr6, r11, 0xcc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 204, &mut ctx.xer);
	// 82443D68: 409A0010  bne cr6, 0x82443d78
	if !ctx.cr[6].eq {
	pc = 0x82443D78; continue 'dispatch;
	}
	// 82443D6C: 817B00F8  lwz r11, 0xf8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(248 as u32) ) } as u64;
	// 82443D70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443D74: 409A0008  bne cr6, 0x82443d7c
	if !ctx.cr[6].eq {
	pc = 0x82443D7C; continue 'dispatch;
	}
	// 82443D78: 73DE00CC  andi. r30, r30, 0xcc
	ctx.r[30].u64 = ctx.r[30].u64 & 204;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82443D7C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82443D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443D84: 4800453D  bl 0x824482c0
	ctx.lr = 0x82443D88;
	sub_824482C0(ctx, base);
	// 82443D88: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82443D8C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82443D90: 419A0010  beq cr6, 0x82443da0
	if ctx.cr[6].eq {
	pc = 0x82443DA0; continue 'dispatch;
	}
	// 82443D94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82443D98: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443D9C: 480F135C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443DA0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82443DA8: 419AFFEC  beq cr6, 0x82443d94
	if ctx.cr[6].eq {
	pc = 0x82443D94; continue 'dispatch;
	}
	// 82443DAC: 73CB00C8  andi. r11, r30, 0xc8
	ctx.r[11].u64 = ctx.r[30].u64 & 200;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443DB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443DB4: 419A0010  beq cr6, 0x82443dc4
	if ctx.cr[6].eq {
	pc = 0x82443DC4; continue 'dispatch;
	}
	// 82443DB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82443DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443DC0: 48000C71  bl 0x82444a30
	ctx.lr = 0x82443DC4;
	sub_82444A30(ctx, base);
	// 82443DC4: 2F1E0080  cmpwi cr6, r30, 0x80
	ctx.cr[6].compare_i32(ctx.r[30].s32, 128, &mut ctx.xer);
	// 82443DC8: 409A0070  bne cr6, 0x82443e38
	if !ctx.cr[6].eq {
	pc = 0x82443E38; continue 'dispatch;
	}
	// 82443DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443DD0: 4BFFC971  bl 0x82440740
	ctx.lr = 0x82443DD4;
	sub_82440740(ctx, base);
	// 82443DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443DD8: 480004F9  bl 0x824442d0
	ctx.lr = 0x82443DDC;
	sub_824442D0(ctx, base);
	// 82443DDC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443DE4: 419A0028  beq cr6, 0x82443e0c
	if ctx.cr[6].eq {
	pc = 0x82443E0C; continue 'dispatch;
	}
	// 82443DE8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443DEC: 4BFFFA1D  bl 0x82443808
	ctx.lr = 0x82443DF0;
	sub_82443808(ctx, base);
	// 82443DF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443DF4: 409A01B0  bne cr6, 0x82443fa4
	if !ctx.cr[6].eq {
	pc = 0x82443FA4; continue 'dispatch;
	}
	// 82443DF8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82443DFC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82443E00: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443E04: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443E08: 480F12F0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443E0C: 4800054D  bl 0x82444358
	ctx.lr = 0x82443E10;
	sub_82444358(ctx, base);
	// 82443E10: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443E14: 419A0024  beq cr6, 0x82443e38
	if ctx.cr[6].eq {
	pc = 0x82443E38; continue 'dispatch;
	}
	// 82443E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443E1C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443E20: 4BFFE179  bl 0x82441f98
	ctx.lr = 0x82443E24;
	sub_82441F98(ctx, base);
	// 82443E24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82443E28: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82443E2C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443E30: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443E34: 480F12C4  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443E38: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82443E3C: 409A004C  bne cr6, 0x82443e88
	if !ctx.cr[6].eq {
	pc = 0x82443E88; continue 'dispatch;
	}
	// 82443E40: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82443E44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82443E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443E4C: 4BFFE22D  bl 0x82442078
	ctx.lr = 0x82443E50;
	sub_82442078(ctx, base);
	// 82443E50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443E54: 419A0018  beq cr6, 0x82443e6c
	if ctx.cr[6].eq {
	pc = 0x82443E6C; continue 'dispatch;
	}
	// 82443E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443E5C: 48000BBD  bl 0x82444a18
	ctx.lr = 0x82443E60;
	sub_82444A18(ctx, base);
	// 82443E60: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82443E64: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443E68: 480F1290  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443E6C: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82443E70: 41990018  bgt cr6, 0x82443e88
	if ctx.cr[6].gt {
	pc = 0x82443E88; continue 'dispatch;
	}
	// 82443E74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82443E78: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82443E7C: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82443E80: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443E84: 480F1274  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443E88: 73CB004C  andi. r11, r30, 0x4c
	ctx.r[11].u64 = ctx.r[30].u64 & 76;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443E8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443E90: 419A0088  beq cr6, 0x82443f18
	if ctx.cr[6].eq {
	pc = 0x82443F18; continue 'dispatch;
	}
	// 82443E94: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82443E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443E9C: 4BFFC84D  bl 0x824406e8
	ctx.lr = 0x82443EA0;
	sub_824406E8(ctx, base);
	// 82443EA0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82443EA4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82443EA8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443EAC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82443EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443EB4: 4BFFFBAD  bl 0x82443a60
	ctx.lr = 0x82443EB8;
	sub_82443A60(ctx, base);
	// 82443EB8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82443EBC: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82443EC0: 409A00E4  bne cr6, 0x82443fa4
	if !ctx.cr[6].eq {
	pc = 0x82443FA4; continue 'dispatch;
	}
	// 82443EC4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82443EC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82443ECC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82443ED0: 409A0020  bne cr6, 0x82443ef0
	if !ctx.cr[6].eq {
	pc = 0x82443EF0; continue 'dispatch;
	}
	// 82443ED4: 813B0008  lwz r9, 8(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82443ED8: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 82443EDC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82443EE0: 419A000C  beq cr6, 0x82443eec
	if ctx.cr[6].eq {
	pc = 0x82443EEC; continue 'dispatch;
	}
	// 82443EE4: 392000CC  li r9, 0xcc
	ctx.r[9].s64 = 204;
	// 82443EE8: 913B0008  stw r9, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82443EEC: 917B00F8  stw r11, 0xf8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 82443EF0: 2F1E0040  cmpwi cr6, r30, 0x40
	ctx.cr[6].compare_i32(ctx.r[30].s32, 64, &mut ctx.xer);
	// 82443EF4: 409A00AC  bne cr6, 0x82443fa0
	if !ctx.cr[6].eq {
	pc = 0x82443FA0; continue 'dispatch;
	}
	// 82443EF8: 2F0AFFFE  cmpwi cr6, r10, -2
	ctx.cr[6].compare_i32(ctx.r[10].s32, -2, &mut ctx.xer);
	// 82443EFC: 409A00A4  bne cr6, 0x82443fa0
	if !ctx.cr[6].eq {
	pc = 0x82443FA0; continue 'dispatch;
	}
	// 82443F00: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 82443F04: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82443F08: 915B0008  stw r10, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82443F0C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443F10: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443F14: 480F11E4  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443F18: 57CB07BC  rlwinm r11, r30, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82443F1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443F20: 419A005C  beq cr6, 0x82443f7c
	if ctx.cr[6].eq {
	pc = 0x82443F7C; continue 'dispatch;
	}
	// 82443F24: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82443F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443F2C: 4BFFC7BD  bl 0x824406e8
	ctx.lr = 0x82443F30;
	sub_824406E8(ctx, base);
	// 82443F30: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82443F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443F38: 4BFFF391  bl 0x824432c8
	ctx.lr = 0x82443F3C;
	sub_824432C8(ctx, base);
	// 82443F3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443F40: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443F48: 419A0024  beq cr6, 0x82443f6c
	if ctx.cr[6].eq {
	pc = 0x82443F6C; continue 'dispatch;
	}
	// 82443F4C: 4BFFE7F5  bl 0x82442740
	ctx.lr = 0x82443F50;
	sub_82442740(ctx, base);
	// 82443F50: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82443F54: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82443F58: 409A004C  bne cr6, 0x82443fa4
	if !ctx.cr[6].eq {
	pc = 0x82443FA4; continue 'dispatch;
	}
	// 82443F5C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82443F60: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443F64: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443F68: 480F1190  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443F6C: 4BFFF46D  bl 0x824433d8
	ctx.lr = 0x82443F70;
	sub_824433D8(ctx, base);
	// 82443F70: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82443F74: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443F78: 480F1180  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82443F7C: 2F1E0080  cmpwi cr6, r30, 0x80
	ctx.cr[6].compare_i32(ctx.r[30].s32, 128, &mut ctx.xer);
	// 82443F80: 419A0024  beq cr6, 0x82443fa4
	if ctx.cr[6].eq {
	pc = 0x82443FA4; continue 'dispatch;
	}
	// 82443F84: 38A000CC  li r5, 0xcc
	ctx.r[5].s64 = 204;
	// 82443F88: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82443F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82443F90: 4BFFE8C9  bl 0x82442858
	ctx.lr = 0x82443F94;
	sub_82442858(ctx, base);
	// 82443F94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82443F98: 4099000C  ble cr6, 0x82443fa4
	if !ctx.cr[6].gt {
	pc = 0x82443FA4; continue 'dispatch;
	}
	// 82443F9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82443FA0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82443FA4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82443FA8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82443FAC: 480F114C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82443FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82443FB0 size=156
    let mut pc: u32 = 0x82443FB0;
    'dispatch: loop {
        match pc {
            0x82443FB0 => {
    //   block [0x82443FB0..0x8244404C)
	// 82443FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82443FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82443FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82443FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82443FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82443FC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82443FC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82443FCC: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82443FD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82443FD4: 409A0054  bne cr6, 0x82444028
	if !ctx.cr[6].eq {
	pc = 0x82444028; continue 'dispatch;
	}
	// 82443FD8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82443FDC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82443FE0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82443FE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82443FE8: 4BFFEC11  bl 0x82442bf8
	ctx.lr = 0x82443FEC;
	sub_82442BF8(ctx, base);
	// 82443FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82443FF0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82443FF4: 409A0034  bne cr6, 0x82444028
	if !ctx.cr[6].eq {
	pc = 0x82444028; continue 'dispatch;
	}
	// 82443FF8: 38E1005C  addi r7, r1, 0x5c
	ctx.r[7].s64 = ctx.r[1].s64 + 92;
	// 82443FFC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82444000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82444004: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82444008: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8244400C: 4BFFFD1D  bl 0x82443d28
	ctx.lr = 0x82444010;
	sub_82443D28(ctx, base);
	// 82444010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82444014: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82444018: 409A0010  bne cr6, 0x82444028
	if !ctx.cr[6].eq {
	pc = 0x82444028; continue 'dispatch;
	}
	// 8244401C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82444020: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444024: 409AFFA8  bne cr6, 0x82443fcc
	if !ctx.cr[6].eq {
	pc = 0x82443FCC; continue 'dispatch;
	}
	// 82444028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244402C: 4BFFD6F5  bl 0x82441720
	ctx.lr = 0x82444030;
	sub_82441720(ctx, base);
	// 82444030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82444034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82444038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244403C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444040: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82444044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82444048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444050 size=148
    let mut pc: u32 = 0x82444050;
    'dispatch: loop {
        match pc {
            0x82444050 => {
    //   block [0x82444050..0x824440E4)
	// 82444050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244405C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82444060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444064: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82444068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244406C: 4BFF869D  bl 0x8243c708
	ctx.lr = 0x82444070;
	sub_8243C708(ctx, base);
	// 82444070: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82444074: 409A000C  bne cr6, 0x82444080
	if !ctx.cr[6].eq {
	pc = 0x82444080; continue 'dispatch;
	}
	// 82444078: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244407C: 48000050  b 0x824440cc
	pc = 0x824440CC; continue 'dispatch;
	// 82444080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82444084: 4BFFC1E5  bl 0x82440268
	ctx.lr = 0x82444088;
	sub_82440268(ctx, base);
	// 82444088: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8244408C: 419AFFEC  beq cr6, 0x82444078
	if ctx.cr[6].eq {
	pc = 0x82444078; continue 'dispatch;
	}
	// 82444090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82444094: 4BFFC0F5  bl 0x82440188
	ctx.lr = 0x82444098;
	sub_82440188(ctx, base);
	// 82444098: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8244409C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824440A0: 409A000C  bne cr6, 0x824440ac
	if !ctx.cr[6].eq {
	pc = 0x824440AC; continue 'dispatch;
	}
	// 824440A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824440A8: 4BFFC141  bl 0x824401e8
	ctx.lr = 0x824440AC;
	sub_824401E8(ctx, base);
	// 824440AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824440B0: 4BFFFF01  bl 0x82443fb0
	ctx.lr = 0x824440B4;
	sub_82443FB0(ctx, base);
	// 824440B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824440B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824440BC: 4BFFF6DD  bl 0x82443798
	ctx.lr = 0x824440C0;
	sub_82443798(ctx, base);
	// 824440C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824440C4: 4BFFDA85  bl 0x82441b48
	ctx.lr = 0x824440C8;
	sub_82441B48(ctx, base);
	// 824440C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824440CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824440D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824440D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824440D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824440DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824440E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824440E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824440E8 size=4
    let mut pc: u32 = 0x824440E8;
    'dispatch: loop {
        match pc {
            0x824440E8 => {
    //   block [0x824440E8..0x824440EC)
	// 824440E8: 4BFFFF68  b 0x82444050
	sub_82444050(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824440F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824440F0 size=36
    let mut pc: u32 = 0x824440F0;
    'dispatch: loop {
        match pc {
            0x824440F0 => {
    //   block [0x824440F0..0x82444114)
	// 824440F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824440F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824440F8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824440FC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82444100: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82444104: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82444108: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8244410C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82444110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444118 size=28
    let mut pc: u32 = 0x82444118;
    'dispatch: loop {
        match pc {
            0x82444118 => {
    //   block [0x82444118..0x82444134)
	// 82444118: 81632668  lwz r11, 0x2668(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9832 as u32) ) } as u64;
	// 8244411C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444120: 81632668  lwz r11, 0x2668(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9832 as u32) ) } as u64;
	// 82444124: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444128: 409A000C  bne cr6, 0x82444134
	if !ctx.cr[6].eq {
		sub_82444134(ctx, base);
		return;
	}
	// 8244412C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444134 size=24
    let mut pc: u32 = 0x82444134;
    'dispatch: loop {
        match pc {
            0x82444134 => {
    //   block [0x82444134..0x8244414C)
	// 82444134: 81630954  lwz r11, 0x954(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2388 as u32) ) } as u64;
	// 82444138: 81432680  lwz r10, 0x2680(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9856 as u32) ) } as u64;
	// 8244413C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82444140: 4098000C  bge cr6, 0x8244414c
	if !ctx.cr[6].lt {
		sub_8244414C(ctx, base);
		return;
	}
	// 82444144: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82444148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244414C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8244414C size=24
    let mut pc: u32 = 0x8244414C;
    'dispatch: loop {
        match pc {
            0x8244414C => {
    //   block [0x8244414C..0x82444164)
	// 8244414C: 81630950  lwz r11, 0x950(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2384 as u32) ) } as u64;
	// 82444150: 81432670  lwz r10, 0x2670(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9840 as u32) ) } as u64;
	// 82444154: 7D6A5810  subfc r11, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82444158: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8244415C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82444160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444168 size=116
    let mut pc: u32 = 0x82444168;
    'dispatch: loop {
        match pc {
            0x82444168 => {
    //   block [0x82444168..0x824441DC)
	// 82444168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244416C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444170: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82444174: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444178: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 8244417C: 3BEBC7F0  addi r31, r11, -0x3810
	ctx.r[31].s64 = ctx.r[11].s64 + -14352;
	// 82444180: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82444184: 395F0024  addi r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 + 36;
	// 82444188: 393F0024  addi r9, r31, 0x24
	ctx.r[9].s64 = ctx.r[31].s64 + 36;
	// 8244418C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444190: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82444194: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444198: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244419C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824441A0: 409A0028  bne cr6, 0x824441c8
	if !ctx.cr[6].eq {
	pc = 0x824441C8; continue 'dispatch;
	}
	// 824441A4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 824441A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824441AC: 4BFE8CAD  bl 0x8242ce58
	ctx.lr = 0x824441B0;
	sub_8242CE58(ctx, base);
	// 824441B0: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 824441B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824441B8: 409A0010  bne cr6, 0x824441c8
	if !ctx.cr[6].eq {
	pc = 0x824441C8; continue 'dispatch;
	}
	// 824441BC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824441C0: 60840F61  ori r4, r4, 0xf61
	ctx.r[4].u64 = ctx.r[4].u64 | 3937;
	// 824441C4: 48003745  bl 0x82447908
	ctx.lr = 0x824441C8;
	sub_82447908(ctx, base);
	// 824441C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824441CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824441D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824441D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824441D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824441E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824441E0 size=64
    let mut pc: u32 = 0x824441E0;
    'dispatch: loop {
        match pc {
            0x824441E0 => {
    //   block [0x824441E0..0x82444220)
	// 824441E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824441E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824441E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824441EC: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824441F0: 806BC810  lwz r3, -0x37f0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14320 as u32) ) } as u64;
	// 824441F4: 4BFE8DD5  bl 0x8242cfc8
	ctx.lr = 0x824441F8;
	sub_8242CFC8(ctx, base);
	// 824441F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824441FC: 40980014  bge cr6, 0x82444210
	if !ctx.cr[6].lt {
	pc = 0x82444210; continue 'dispatch;
	}
	// 82444200: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444204: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444208: 60840F62  ori r4, r4, 0xf62
	ctx.r[4].u64 = ctx.r[4].u64 | 3938;
	// 8244420C: 480036FD  bl 0x82447908
	ctx.lr = 0x82444210;
	sub_82447908(ctx, base);
	// 82444210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244421C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444220 size=64
    let mut pc: u32 = 0x82444220;
    'dispatch: loop {
        match pc {
            0x82444220 => {
    //   block [0x82444220..0x82444260)
	// 82444220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244422C: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82444230: 806BC810  lwz r3, -0x37f0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14320 as u32) ) } as u64;
	// 82444234: 4BFE8E2D  bl 0x8242d060
	ctx.lr = 0x82444238;
	sub_8242D060(ctx, base);
	// 82444238: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244423C: 40980014  bge cr6, 0x82444250
	if !ctx.cr[6].lt {
	pc = 0x82444250; continue 'dispatch;
	}
	// 82444240: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444244: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444248: 60840F63  ori r4, r4, 0xf63
	ctx.r[4].u64 = ctx.r[4].u64 | 3939;
	// 8244424C: 480036BD  bl 0x82447908
	ctx.lr = 0x82444250;
	sub_82447908(ctx, base);
	// 82444250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244425C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444260 size=108
    let mut pc: u32 = 0x82444260;
    'dispatch: loop {
        match pc {
            0x82444260 => {
    //   block [0x82444260..0x824442CC)
	// 82444260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244426C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82444274: 480035CD  bl 0x82447840
	ctx.lr = 0x82444278;
	sub_82447840(ctx, base);
	// 82444278: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244427C: 419A0028  beq cr6, 0x824442a4
	if ctx.cr[6].eq {
	pc = 0x824442A4; continue 'dispatch;
	}
	// 82444280: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444284: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444288: 60840161  ori r4, r4, 0x161
	ctx.r[4].u64 = ctx.r[4].u64 | 353;
	// 8244428C: 4800367D  bl 0x82447908
	ctx.lr = 0x82444290;
	sub_82447908(ctx, base);
	// 82444290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244429C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824442A0: 4E800020  blr
	return;
	// 824442A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824442A8: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 824442AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824442B0: 4BFF84A1  bl 0x8243c750
	ctx.lr = 0x824442B4;
	sub_8243C750(ctx, base);
	// 824442B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824442B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824442BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824442C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824442C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824442C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824442D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824442D0 size=48
    let mut pc: u32 = 0x824442D0;
    'dispatch: loop {
        match pc {
            0x824442D0 => {
    //   block [0x824442D0..0x82444300)
	// 824442D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824442D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824442D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824442DC: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 824442E0: 4BFF8429  bl 0x8243c708
	ctx.lr = 0x824442E4;
	sub_8243C708(ctx, base);
	// 824442E4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824442E8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824442EC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 824442F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824442F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824442F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824442FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444300 size=84
    let mut pc: u32 = 0x82444300;
    'dispatch: loop {
        match pc {
            0x82444300 => {
    //   block [0x82444300..0x82444354)
	// 82444300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244430C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444310: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 82444314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82444318: 4BFF83F1  bl 0x8243c708
	ctx.lr = 0x8244431C;
	sub_8243C708(ctx, base);
	// 8244431C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82444320: 409A001C  bne cr6, 0x8244433c
	if !ctx.cr[6].eq {
	pc = 0x8244433C; continue 'dispatch;
	}
	// 82444324: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82444328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244432C: 4BFF83DD  bl 0x8243c708
	ctx.lr = 0x82444330;
	sub_8243C708(ctx, base);
	// 82444330: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82444334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444338: 419A0008  beq cr6, 0x82444340
	if ctx.cr[6].eq {
	pc = 0x82444340; continue 'dispatch;
	}
	// 8244433C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82444340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244434C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82444350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444358 size=84
    let mut pc: u32 = 0x82444358;
    'dispatch: loop {
        match pc {
            0x82444358 => {
    //   block [0x82444358..0x824443AC)
	// 82444358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244435C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444360: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82444364: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444368: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 8244436C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82444370: 4BFF8399  bl 0x8243c708
	ctx.lr = 0x82444374;
	sub_8243C708(ctx, base);
	// 82444374: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82444378: 409A001C  bne cr6, 0x82444394
	if !ctx.cr[6].eq {
	pc = 0x82444394; continue 'dispatch;
	}
	// 8244437C: 38800039  li r4, 0x39
	ctx.r[4].s64 = 57;
	// 82444380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82444384: 4BFF8385  bl 0x8243c708
	ctx.lr = 0x82444388;
	sub_8243C708(ctx, base);
	// 82444388: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244438C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444390: 419A0008  beq cr6, 0x82444398
	if ctx.cr[6].eq {
	pc = 0x82444398; continue 'dispatch;
	}
	// 82444394: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82444398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244439C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824443A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824443A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824443A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824443B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824443B0 size=124
    let mut pc: u32 = 0x824443B0;
    'dispatch: loop {
        match pc {
            0x824443B0 => {
    //   block [0x824443B0..0x8244442C)
	// 824443B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824443B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824443B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824443BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824443C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824443C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824443C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824443CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824443D0: 3BEB0D88  addi r31, r11, 0xd88
	ctx.r[31].s64 = ctx.r[11].s64 + 3464;
	// 824443D4: 4BFFFE0D  bl 0x824441e0
	ctx.lr = 0x824443D8;
	sub_824441E0(ctx, base);
	// 824443D8: 817F0168  lwz r11, 0x168(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 824443DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824443E0: 815F0164  lwz r10, 0x164(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 824443E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824443E8: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824443EC: 7D692E70  srawi r9, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 824443F0: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824443F4: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824443F8: 915F0164  stw r10, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 824443FC: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82444400: 3929005B  addi r9, r9, 0x5b
	ctx.r[9].s64 = ctx.r[9].s64 + 91;
	// 82444404: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82444408: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8244440C: 917F0168  stw r11, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 82444410: 4BFFFE11  bl 0x82444220
	ctx.lr = 0x82444414;
	sub_82444220(ctx, base);
	// 82444414: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82444418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244441C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444420: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82444424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82444428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82444430 size=160
    let mut pc: u32 = 0x82444430;
    'dispatch: loop {
        match pc {
            0x82444430 => {
    //   block [0x82444430..0x824444D0)
	// 82444430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444434: 480F0C85  bl 0x825350b8
	ctx.lr = 0x82444438;
	sub_82535080(ctx, base);
	// 82444438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244443C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82444440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82444444: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82444448: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244444C: 3BEB0D88  addi r31, r11, 0xd88
	ctx.r[31].s64 = ctx.r[11].s64 + 3464;
	// 82444450: 4BFFFD91  bl 0x824441e0
	ctx.lr = 0x82444454;
	sub_824441E0(ctx, base);
	// 82444454: 817F01F8  lwz r11, 0x1f8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 82444458: 815F01FC  lwz r10, 0x1fc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8244445C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82444460: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82444464: 4198001C  blt cr6, 0x82444480
	if ctx.cr[6].lt {
	pc = 0x82444480; continue 'dispatch;
	}
	// 82444468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8244446C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82444470: 4BFFFDB1  bl 0x82444220
	ctx.lr = 0x82444474;
	sub_82444220(ctx, base);
	// 82444474: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82444478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244447C: 480F0C8C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82444480: 7D6A2E70  srawi r10, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82444484: 93BF01EC  stw r29, 0x1ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(492 as u32), ctx.r[29].u32 ) };
	// 82444488: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8244448C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82444490: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82444494: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82444498: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8244449C: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 824444A0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824444A4: 7FCBF92E  stwx r30, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 824444A8: 815F01F8  lwz r10, 0x1f8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 824444AC: 817F01F0  lwz r11, 0x1f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 824444B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824444B4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824444B8: 915F01F8  stw r10, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[10].u32 ) };
	// 824444BC: 917F01F0  stw r11, 0x1f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 824444C0: 4BFFFD61  bl 0x82444220
	ctx.lr = 0x824444C4;
	sub_82444220(ctx, base);
	// 824444C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824444C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824444CC: 480F0C3C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824444D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824444D0 size=164
    let mut pc: u32 = 0x824444D0;
    'dispatch: loop {
        match pc {
            0x824444D0 => {
    //   block [0x824444D0..0x82444574)
	// 824444D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824444D4: 480F0BE5  bl 0x825350b8
	ctx.lr = 0x824444D8;
	sub_82535080(ctx, base);
	// 824444D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824444DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824444E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824444E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824444E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824444EC: 3BEB0D88  addi r31, r11, 0xd88
	ctx.r[31].s64 = ctx.r[11].s64 + 3464;
	// 824444F0: 4BFFFCF1  bl 0x824441e0
	ctx.lr = 0x824444F4;
	sub_824441E0(ctx, base);
	// 824444F4: 817F01F8  lwz r11, 0x1f8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 824444F8: 815F01FC  lwz r10, 0x1fc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 824444FC: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444500: 41810024  bgt 0x82444524
	if ctx.cr[0].gt {
	pc = 0x82444524; continue 'dispatch;
	}
	// 82444504: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82444508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8244450C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82444510: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444514: 4BFFFD0D  bl 0x82444220
	ctx.lr = 0x82444518;
	sub_82444220(ctx, base);
	// 82444518: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244451C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82444520: 480F0BE8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82444524: 817F01EC  lwz r11, 0x1ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(492 as u32) ) } as u64;
	// 82444528: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8244452C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82444530: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444534: 817F01FC  lwz r11, 0x1fc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82444538: 7D6A2E70  srawi r10, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 8244453C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82444540: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82444544: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82444548: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 8244454C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82444550: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82444554: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444558: 817F01FC  lwz r11, 0x1fc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8244455C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82444560: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	// 82444564: 4BFFFCBD  bl 0x82444220
	ctx.lr = 0x82444568;
	sub_82444220(ctx, base);
	// 82444568: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244456C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82444570: 480F0B98  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444578 size=32
    let mut pc: u32 = 0x82444578;
    'dispatch: loop {
        match pc {
            0x82444578 => {
    //   block [0x82444578..0x82444598)
	// 82444578: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8244457C: 409A0024  bne cr6, 0x824445a0
	if !ctx.cr[6].eq {
		sub_824445A0(ctx, base);
		return;
	}
	// 82444580: 7C6B2278  xor r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 ^ ctx.r[4].u64;
	// 82444584: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444588: 41980010  blt cr6, 0x82444598
	if ctx.cr[6].lt {
		sub_82444598(ctx, base);
		return;
	}
	// 8244458C: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 82444590: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 82444594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444598 size=8
    let mut pc: u32 = 0x82444598;
    'dispatch: loop {
        match pc {
            0x82444598 => {
    //   block [0x82444598..0x824445A0)
	// 82444598: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8244459C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824445A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824445A0 size=28
    let mut pc: u32 = 0x824445A0;
    'dispatch: loop {
        match pc {
            0x824445A0 => {
    //   block [0x824445A0..0x824445BC)
	// 824445A0: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 824445A4: 7C8A07B4  extsw r10, r4
	ctx.r[10].s64 = ctx.r[4].s32 as i64;
	// 824445A8: 7CA907B4  extsw r9, r5
	ctx.r[9].s64 = ctx.r[5].s32 as i64;
	// 824445AC: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 824445B0: 7D6B4BD2  divd r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 824445B4: 7D6307B4  extsw r3, r11
	ctx.r[3].s64 = ctx.r[11].s32 as i64;
	// 824445B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824445C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824445C0 size=60
    let mut pc: u32 = 0x824445C0;
    'dispatch: loop {
        match pc {
            0x824445C0 => {
    //   block [0x824445C0..0x824445FC)
	// 824445C0: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824445C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824445C8: 386BC818  addi r3, r11, -0x37e8
	ctx.r[3].s64 = ctx.r[11].s64 + -14312;
	// 824445CC: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 824445D0: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 824445D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824445D8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824445DC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824445E0: 4200FFF8  bdnz 0x824445d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824445D8; continue 'dispatch;
	}
	// 824445E4: 39630044  addi r11, r3, 0x44
	ctx.r[11].s64 = ctx.r[3].s64 + 68;
	// 824445E8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 824445EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824445F0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824445F4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824445F8: 480F0BD8  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444600 size=136
    let mut pc: u32 = 0x82444600;
    'dispatch: loop {
        match pc {
            0x82444600 => {
    //   block [0x82444600..0x82444688)
	// 82444600: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82444604: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82444608: 396BC818  addi r11, r11, -0x37e8
	ctx.r[11].s64 = ctx.r[11].s64 + -14312;
	// 8244460C: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82444610: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82444614: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82444618: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244461C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82444620: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82444624: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82444628: 4200FFF0  bdnz 0x82444618
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82444618; continue 'dispatch;
	}
	// 8244462C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82444630: 7CEB2850  subf r7, r11, r5
	ctx.r[7].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82444634: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82444638: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 8244463C: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82444640: 914B0070  stw r10, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82444644: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444648: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 8244464C: 554A0030  rlwinm r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82444650: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82444654: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82444658: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 8244465C: 554A0030  rlwinm r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82444660: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82444664: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82444668: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8244466C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82444670: 40980018  bge cr6, 0x82444688
	if !ctx.cr[6].lt {
		sub_82444688(ctx, base);
		return;
	}
	// 82444674: 7D07502E  lwzx r8, r7, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82444678: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 8244467C: 55080030  rlwinm r8, r8, 0, 0, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82444680: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82444684: 48000008  b 0x8244468c
	sub_82444688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444688 size=28
    let mut pc: u32 = 0x82444688;
    'dispatch: loop {
        match pc {
            0x82444688 => {
    //   block [0x82444688..0x824446A4)
	// 82444688: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8244468C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82444690: 390B0040  addi r8, r11, 0x40
	ctx.r[8].s64 = ctx.r[11].s64 + 64;
	// 82444694: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82444698: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8244469C: 4198FFCC  blt cr6, 0x82444668
	if ctx.cr[6].lt {
		sub_82444600(ctx, base);
		return;
	}
	// 824446A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824446A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824446A8 size=132
    let mut pc: u32 = 0x824446A8;
    'dispatch: loop {
        match pc {
            0x824446A8 => {
    //   block [0x824446A8..0x8244472C)
	// 824446A8: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824446AC: 394BC818  addi r10, r11, -0x37e8
	ctx.r[10].s64 = ctx.r[11].s64 + -14312;
	// 824446B0: 810A006C  lwz r8, 0x6c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(108 as u32) ) } as u64;
	// 824446B4: 3968FFFF  addi r11, r8, -1
	ctx.r[11].s64 = ctx.r[8].s64 + -1;
	// 824446B8: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 824446BC: 41990070  bgt cr6, 0x8244472c
	if ctx.cr[6].gt {
		sub_8244472C(ctx, base);
		return;
	}
	// 824446C0: 816A0060  lwz r11, 0x60(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 824446C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824446C8: 419A0010  beq cr6, 0x824446d8
	if ctx.cr[6].eq {
	pc = 0x824446D8; continue 'dispatch;
	}
	// 824446CC: 816A0070  lwz r11, 0x70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(112 as u32) ) } as u64;
	// 824446D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824446D4: 409A0050  bne cr6, 0x82444724
	if !ctx.cr[6].eq {
	pc = 0x82444724; continue 'dispatch;
	}
	// 824446D8: 396A0044  addi r11, r10, 0x44
	ctx.r[11].s64 = ctx.r[10].s64 + 68;
	// 824446DC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824446E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824446E4: 419A0048  beq cr6, 0x8244472c
	if ctx.cr[6].eq {
		sub_8244472C(ctx, base);
		return;
	}
	// 824446E8: 392A0044  addi r9, r10, 0x44
	ctx.r[9].s64 = ctx.r[10].s64 + 68;
	// 824446EC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824446F0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824446F4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824446F8: 4198FFE4  blt cr6, 0x824446dc
	if ctx.cr[6].lt {
	pc = 0x824446DC; continue 'dispatch;
	}
	// 824446FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82444700: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82444704: 40990020  ble cr6, 0x82444724
	if !ctx.cr[6].gt {
	pc = 0x82444724; continue 'dispatch;
	}
	// 82444708: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244470C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82444710: 419A001C  beq cr6, 0x8244472c
	if ctx.cr[6].eq {
		sub_8244472C(ctx, base);
		return;
	}
	// 82444714: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82444718: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8244471C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82444720: 4198FFE8  blt cr6, 0x82444708
	if ctx.cr[6].lt {
	pc = 0x82444708; continue 'dispatch;
	}
	// 82444724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244472C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244472C size=8
    let mut pc: u32 = 0x8244472C;
    'dispatch: loop {
        match pc {
            0x8244472C => {
    //   block [0x8244472C..0x82444734)
	// 8244472C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82444730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444738 size=148
    let mut pc: u32 = 0x82444738;
    'dispatch: loop {
        match pc {
            0x82444738 => {
    //   block [0x82444738..0x824447CC)
	// 82444738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244473C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444744: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82444748: 4BFFFF61  bl 0x824446a8
	ctx.lr = 0x8244474C;
	sub_824446A8(ctx, base);
	// 8244474C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82444750: 419A0024  beq cr6, 0x82444774
	if ctx.cr[6].eq {
	pc = 0x82444774; continue 'dispatch;
	}
	// 82444754: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244475C: 60840F15  ori r4, r4, 0xf15
	ctx.r[4].u64 = ctx.r[4].u64 | 3861;
	// 82444760: 480031A9  bl 0x82447908
	ctx.lr = 0x82444764;
	sub_82447908(ctx, base);
	// 82444764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244476C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444770: 4E800020  blr
	return;
	// 82444774: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82444778: 39472768  addi r10, r7, 0x2768
	ctx.r[10].s64 = ctx.r[7].s64 + 10088;
	// 8244477C: 392BC818  addi r9, r11, -0x37e8
	ctx.r[9].s64 = ctx.r[11].s64 + -14312;
	// 82444780: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82444784: 39690050  addi r11, r9, 0x50
	ctx.r[11].s64 = ctx.r[9].s64 + 80;
	// 82444788: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8244478C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444790: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82444794: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82444798: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8244479C: 4200FFF0  bdnz 0x8244478c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8244478C; continue 'dispatch;
	}
	// 824447A0: E9690044  ld r11, 0x44(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(68 as u32) ) };
	// 824447A4: 38672794  addi r3, r7, 0x2794
	ctx.r[3].s64 = ctx.r[7].s64 + 10132;
	// 824447A8: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 824447AC: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 824447B0: F967278C  std r11, 0x278c(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(10124 as u32), ctx.r[11].u64 ) };
	// 824447B4: 480F039D  bl 0x82534b50
	ctx.lr = 0x824447B8;
	sub_82534B50(ctx, base);
	// 824447B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824447BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824447C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824447C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824447C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824447D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824447D0 size=144
    let mut pc: u32 = 0x824447D0;
    'dispatch: loop {
        match pc {
            0x824447D0 => {
    //   block [0x824447D0..0x82444860)
	// 824447D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824447D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824447D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824447DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824447E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824447E4: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824447E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824447EC: 3BCBC818  addi r30, r11, -0x37e8
	ctx.r[30].s64 = ctx.r[11].s64 + -14312;
	// 824447F0: 397F2768  addi r11, r31, 0x2768
	ctx.r[11].s64 = ctx.r[31].s64 + 10088;
	// 824447F4: 393E0050  addi r9, r30, 0x50
	ctx.r[9].s64 = ctx.r[30].s64 + 80;
	// 824447F8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 824447FC: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82444800: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82444804: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444808: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8244480C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82444810: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82444814: 4200FFF0  bdnz 0x82444804
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82444804; continue 'dispatch;
	}
	// 82444818: E97F278C  ld r11, 0x278c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(10124 as u32) ) };
	// 8244481C: 389F2794  addi r4, r31, 0x2794
	ctx.r[4].s64 = ctx.r[31].s64 + 10132;
	// 82444820: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82444824: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82444828: F97E0044  std r11, 0x44(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u64 ) };
	// 8244482C: 480F0325  bl 0x82534b50
	ctx.lr = 0x82444830;
	sub_82534B50(ctx, base);
	// 82444830: 817F37E8  lwz r11, 0x37e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14312 as u32) ) } as u64;
	// 82444834: 917E0040  stw r11, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82444838: 817F37EC  lwz r11, 0x37ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14316 as u32) ) } as u64;
	// 8244483C: 917E0074  stw r11, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82444840: 817F37F0  lwz r11, 0x37f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14320 as u32) ) } as u64;
	// 82444844: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82444848: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244484C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444854: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82444858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244485C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82444860 size=128
    let mut pc: u32 = 0x82444860;
    'dispatch: loop {
        match pc {
            0x82444860 => {
    //   block [0x82444860..0x824448E0)
	// 82444860: 3964000F  addi r11, r4, 0xf
	ctx.r[11].s64 = ctx.r[4].s64 + 15;
	// 82444864: 90660008  stw r3, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82444868: 3925000F  addi r9, r5, 0xf
	ctx.r[9].s64 = ctx.r[5].s64 + 15;
	// 8244486C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82444870: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82444874: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82444878: 394B007F  addi r10, r11, 0x7f
	ctx.r[10].s64 = ctx.r[11].s64 + 127;
	// 8244487C: 7D4A3E70  srawi r10, r10, 7
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 7) as i64;
	// 82444880: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82444884: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82444888: 55483830  slwi r8, r10, 7
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244488C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82444890: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82444894: 7D6B3E70  srawi r11, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 82444898: B106000E  sth r8, 0xe(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(14 as u32), ctx.r[8].u16 ) };
	// 8244489C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824448A0: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 824448A4: 55683830  slwi r8, r11, 7
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824448A8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824448AC: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 824448B0: B106000C  sth r8, 0xc(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 824448B4: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824448B8: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824448BC: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 824448C0: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 824448C4: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824448C8: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 824448CC: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824448D0: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824448D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824448D8: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824448DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824448E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824448E0 size=60
    let mut pc: u32 = 0x824448E0;
    'dispatch: loop {
        match pc {
            0x824448E0 => {
    //   block [0x824448E0..0x8244491C)
	// 824448E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824448E4: 39430018  addi r10, r3, 0x18
	ctx.r[10].s64 = ctx.r[3].s64 + 24;
	// 824448E8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824448EC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824448F0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824448F4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824448F8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824448FC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82444900: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82444904: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82444908: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244490C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82444910: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82444914: 409AFFEC  bne cr6, 0x82444900
	if !ctx.cr[6].eq {
	pc = 0x82444900; continue 'dispatch;
	}
	// 82444918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444920 size=48
    let mut pc: u32 = 0x82444920;
    'dispatch: loop {
        match pc {
            0x82444920 => {
    //   block [0x82444920..0x82444950)
	// 82444920: 39631738  addi r11, r3, 0x1738
	ctx.r[11].s64 = ctx.r[3].s64 + 5944;
	// 82444924: 392327E8  addi r9, r3, 0x27e8
	ctx.r[9].s64 = ctx.r[3].s64 + 10216;
	// 82444928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244492C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82444930: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82444934: 419A001C  beq cr6, 0x82444950
	if ctx.cr[6].eq {
		sub_82444950(ctx, base);
		return;
	}
	// 82444938: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244493C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 82444940: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82444944: 4198FFEC  blt cr6, 0x82444930
	if ctx.cr[6].lt {
	pc = 0x82444930; continue 'dispatch;
	}
	// 82444948: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244494C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444950 size=12
    let mut pc: u32 = 0x82444950;
    'dispatch: loop {
        match pc {
            0x82444950 => {
    //   block [0x82444950..0x8244495C)
	// 82444950: 554B402E  slwi r11, r10, 8
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82444954: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82444958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444960 size=20
    let mut pc: u32 = 0x82444960;
    'dispatch: loop {
        match pc {
            0x82444960 => {
    //   block [0x82444960..0x82444974)
	// 82444960: 386327E8  addi r3, r3, 0x27e8
	ctx.r[3].s64 = ctx.r[3].s64 + 10216;
	// 82444964: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82444968: 81430064  lwz r10, 0x64(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 8244496C: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82444970: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444974(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444974 size=24
    let mut pc: u32 = 0x82444974;
    'dispatch: loop {
        match pc {
            0x82444974 => {
    //   block [0x82444974..0x8244498C)
	// 82444974: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82444978: 38630100  addi r3, r3, 0x100
	ctx.r[3].s64 = ctx.r[3].s64 + 256;
	// 8244497C: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82444980: 4198FFE8  blt cr6, 0x82444968
	if ctx.cr[6].lt {
		sub_82444960(ctx, base);
		return;
	}
	// 82444984: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444990 size=52
    let mut pc: u32 = 0x82444990;
    'dispatch: loop {
        match pc {
            0x82444990 => {
    //   block [0x82444990..0x824449C4)
	// 82444990: 812327D4  lwz r9, 0x27d4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10196 as u32) ) } as u64;
	// 82444994: 394327E8  addi r10, r3, 0x27e8
	ctx.r[10].s64 = ctx.r[3].s64 + 10216;
	// 82444998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244499C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824449A0: 4099001C  ble cr6, 0x824449bc
	if !ctx.cr[6].gt {
	pc = 0x824449BC; continue 'dispatch;
	}
	// 824449A4: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824449A8: 419A001C  beq cr6, 0x824449c4
	if ctx.cr[6].eq {
		sub_824449C4(ctx, base);
		return;
	}
	// 824449AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824449B0: 394A0100  addi r10, r10, 0x100
	ctx.r[10].s64 = ctx.r[10].s64 + 256;
	// 824449B4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824449B8: 4198FFEC  blt cr6, 0x824449a4
	if ctx.cr[6].lt {
	pc = 0x824449A4; continue 'dispatch;
	}
	// 824449BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824449C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824449C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824449C4 size=16
    let mut pc: u32 = 0x824449C4;
    'dispatch: loop {
        match pc {
            0x824449C4 => {
    //   block [0x824449C4..0x824449D4)
	// 824449C4: 1D6B0088  mulli r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 * 136;
	// 824449C8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824449CC: 386B1738  addi r3, r11, 0x1738
	ctx.r[3].s64 = ctx.r[11].s64 + 5944;
	// 824449D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824449D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824449D8 size=52
    let mut pc: u32 = 0x824449D8;
    'dispatch: loop {
        match pc {
            0x824449D8 => {
    //   block [0x824449D8..0x82444A0C)
	// 824449D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824449DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824449E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824449E4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 824449E8: 4BFFFFA9  bl 0x82444990
	ctx.lr = 0x824449EC;
	sub_82444990(ctx, base);
	// 824449EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824449F0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824449F4: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 824449F8: 908827E0  stw r4, 0x27e0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(10208 as u32), ctx.r[4].u32 ) };
	// 824449FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444A10 size=8
    let mut pc: u32 = 0x82444A10;
    'dispatch: loop {
        match pc {
            0x82444A10 => {
    //   block [0x82444A10..0x82444A18)
	// 82444A10: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 82444A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444A18 size=12
    let mut pc: u32 = 0x82444A18;
    'dispatch: loop {
        match pc {
            0x82444A18 => {
    //   block [0x82444A18..0x82444A24)
	// 82444A18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82444A1C: 916327D8  stw r11, 0x27d8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10200 as u32), ctx.r[11].u32 ) };
	// 82444A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444A28 size=8
    let mut pc: u32 = 0x82444A28;
    'dispatch: loop {
        match pc {
            0x82444A28 => {
    //   block [0x82444A28..0x82444A30)
	// 82444A28: 806327D8  lwz r3, 0x27d8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10200 as u32) ) } as u64;
	// 82444A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444A30 size=8
    let mut pc: u32 = 0x82444A30;
    'dispatch: loop {
        match pc {
            0x82444A30 => {
    //   block [0x82444A30..0x82444A38)
	// 82444A30: 908327DC  stw r4, 0x27dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10204 as u32), ctx.r[4].u32 ) };
	// 82444A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444A38 size=168
    let mut pc: u32 = 0x82444A38;
    'dispatch: loop {
        match pc {
            0x82444A38 => {
    //   block [0x82444A38..0x82444AE0)
	// 82444A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444A40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82444A44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82444A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444A4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82444A50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82444A54: 48002E8D  bl 0x824478e0
	ctx.lr = 0x82444A58;
	sub_824478E0(ctx, base);
	// 82444A58: 815F27D4  lwz r10, 0x27d4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10196 as u32) ) } as u64;
	// 82444A5C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82444A60: 397F27E8  addi r11, r31, 0x27e8
	ctx.r[11].s64 = ctx.r[31].s64 + 10216;
	// 82444A64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82444A68: 4099003C  ble cr6, 0x82444aa4
	if !ctx.cr[6].gt {
	pc = 0x82444AA4; continue 'dispatch;
	}
	// 82444A6C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82444A70: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444A74: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82444A78: 419A000C  beq cr6, 0x82444a84
	if ctx.cr[6].eq {
	pc = 0x82444A84; continue 'dispatch;
	}
	// 82444A7C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82444A80: 409A0014  bne cr6, 0x82444a94
	if !ctx.cr[6].eq {
	pc = 0x82444A94; continue 'dispatch;
	}
	// 82444A84: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82444A88: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82444A8C: 409A0008  bne cr6, 0x82444a94
	if !ctx.cr[6].eq {
	pc = 0x82444A94; continue 'dispatch;
	}
	// 82444A90: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82444A94: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82444A98: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 82444A9C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82444AA0: 409AFFD0  bne cr6, 0x82444a70
	if !ctx.cr[6].eq {
	pc = 0x82444A70; continue 'dispatch;
	}
	// 82444AA4: 817F27D8  lwz r11, 0x27d8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10200 as u32) ) } as u64;
	// 82444AA8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82444AAC: 409A0010  bne cr6, 0x82444abc
	if !ctx.cr[6].eq {
	pc = 0x82444ABC; continue 'dispatch;
	}
	// 82444AB0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82444AB4: 409A0008  bne cr6, 0x82444abc
	if !ctx.cr[6].eq {
	pc = 0x82444ABC; continue 'dispatch;
	}
	// 82444AB8: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82444ABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82444AC0: 48002E31  bl 0x824478f0
	ctx.lr = 0x82444AC4;
	sub_824478F0(ctx, base);
	// 82444AC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82444AC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82444ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444AD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82444AD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82444ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444AE0 size=144
    let mut pc: u32 = 0x82444AE0;
    'dispatch: loop {
        match pc {
            0x82444AE0 => {
    //   block [0x82444AE0..0x82444B70)
	// 82444AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444AE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82444AEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444AF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82444AF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82444AF8: 48002DE9  bl 0x824478e0
	ctx.lr = 0x82444AFC;
	sub_824478E0(ctx, base);
	// 82444AFC: 815F27D4  lwz r10, 0x27d4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10196 as u32) ) } as u64;
	// 82444B00: 3BFF27E8  addi r31, r31, 0x27e8
	ctx.r[31].s64 = ctx.r[31].s64 + 10216;
	// 82444B04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82444B08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82444B0C: 40990038  ble cr6, 0x82444b44
	if !ctx.cr[6].gt {
	pc = 0x82444B44; continue 'dispatch;
	}
	// 82444B10: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444B14: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82444B18: 409A0010  bne cr6, 0x82444b28
	if !ctx.cr[6].eq {
	pc = 0x82444B28; continue 'dispatch;
	}
	// 82444B1C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82444B20: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82444B24: 419A0018  beq cr6, 0x82444b3c
	if ctx.cr[6].eq {
	pc = 0x82444B3C; continue 'dispatch;
	}
	// 82444B28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82444B2C: 3BFF0100  addi r31, r31, 0x100
	ctx.r[31].s64 = ctx.r[31].s64 + 256;
	// 82444B30: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82444B34: 4198FFDC  blt cr6, 0x82444b10
	if ctx.cr[6].lt {
	pc = 0x82444B10; continue 'dispatch;
	}
	// 82444B38: 4800000C  b 0x82444b44
	pc = 0x82444B44; continue 'dispatch;
	// 82444B3C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82444B40: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82444B44: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82444B48: 409A0008  bne cr6, 0x82444b50
	if !ctx.cr[6].eq {
	pc = 0x82444B50; continue 'dispatch;
	}
	// 82444B4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82444B50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82444B54: 48002D9D  bl 0x824478f0
	ctx.lr = 0x82444B58;
	sub_824478F0(ctx, base);
	// 82444B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82444B5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82444B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444B68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82444B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444B70 size=8
    let mut pc: u32 = 0x82444B70;
    'dispatch: loop {
        match pc {
            0x82444B70 => {
    //   block [0x82444B70..0x82444B78)
	// 82444B70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82444B74: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444B78 size=12
    let mut pc: u32 = 0x82444B78;
    'dispatch: loop {
        match pc {
            0x82444B78 => {
    //   block [0x82444B78..0x82444B84)
	// 82444B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82444B7C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444B88 size=8
    let mut pc: u32 = 0x82444B88;
    'dispatch: loop {
        match pc {
            0x82444B88 => {
    //   block [0x82444B88..0x82444B90)
	// 82444B88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82444B8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444B90 size=12
    let mut pc: u32 = 0x82444B90;
    'dispatch: loop {
        match pc {
            0x82444B90 => {
    //   block [0x82444B90..0x82444B9C)
	// 82444B90: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82444B94: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444BA0 size=8
    let mut pc: u32 = 0x82444BA0;
    'dispatch: loop {
        match pc {
            0x82444BA0 => {
    //   block [0x82444BA0..0x82444BA8)
	// 82444BA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82444BA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444BA8 size=12
    let mut pc: u32 = 0x82444BA8;
    'dispatch: loop {
        match pc {
            0x82444BA8 => {
    //   block [0x82444BA8..0x82444BB4)
	// 82444BA8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82444BAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444BB8 size=8
    let mut pc: u32 = 0x82444BB8;
    'dispatch: loop {
        match pc {
            0x82444BB8 => {
    //   block [0x82444BB8..0x82444BC0)
	// 82444BB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82444BBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444BC0 size=36
    let mut pc: u32 = 0x82444BC0;
    'dispatch: loop {
        match pc {
            0x82444BC0 => {
    //   block [0x82444BC0..0x82444BE4)
	// 82444BC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444BC4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82444BC8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82444BCC: 419A0008  beq cr6, 0x82444bd4
	if ctx.cr[6].eq {
	pc = 0x82444BD4; continue 'dispatch;
	}
	// 82444BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82444BD4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444BD8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82444BDC: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82444BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444BE8 size=8
    let mut pc: u32 = 0x82444BE8;
    'dispatch: loop {
        match pc {
            0x82444BE8 => {
    //   block [0x82444BE8..0x82444BF0)
	// 82444BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82444BEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444BF0 size=24
    let mut pc: u32 = 0x82444BF0;
    'dispatch: loop {
        match pc {
            0x82444BF0 => {
    //   block [0x82444BF0..0x82444C08)
	// 82444BF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444BF4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82444BF8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82444BFC: 556BE7BC  rlwinm r11, r11, 0x1c, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82444C00: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444C08 size=16
    let mut pc: u32 = 0x82444C08;
    'dispatch: loop {
        match pc {
            0x82444C08 => {
    //   block [0x82444C08..0x82444C18)
	// 82444C08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82444C0C: 409A000C  bne cr6, 0x82444c18
	if !ctx.cr[6].eq {
		sub_82444C18(ctx, base);
		return;
	}
	// 82444C10: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82444C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444C18 size=80
    let mut pc: u32 = 0x82444C18;
    'dispatch: loop {
        match pc {
            0x82444C18 => {
    //   block [0x82444C18..0x82444C68)
	// 82444C18: 81640054  lwz r11, 0x54(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 82444C1C: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82444C20: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82444C24: 4198FFEC  blt cr6, 0x82444c10
	if ctx.cr[6].lt {
		sub_82444C08(ctx, base);
		return;
	}
	// 82444C28: 41990040  bgt cr6, 0x82444c68
	if ctx.cr[6].gt {
		sub_82444C68(ctx, base);
		return;
	}
	// 82444C2C: 816400F0  lwz r11, 0xf0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(240 as u32) ) } as u64;
	// 82444C30: 814300F0  lwz r10, 0xf0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 82444C34: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82444C38: 4198FFD8  blt cr6, 0x82444c10
	if ctx.cr[6].lt {
		sub_82444C08(ctx, base);
		return;
	}
	// 82444C3C: 4199002C  bgt cr6, 0x82444c68
	if ctx.cr[6].gt {
		sub_82444C68(ctx, base);
		return;
	}
	// 82444C40: 816400F4  lwz r11, 0xf4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(244 as u32) ) } as u64;
	// 82444C44: 814300F4  lwz r10, 0xf4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(244 as u32) ) } as u64;
	// 82444C48: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82444C4C: 4198FFC4  blt cr6, 0x82444c10
	if ctx.cr[6].lt {
		sub_82444C08(ctx, base);
		return;
	}
	// 82444C50: 41990018  bgt cr6, 0x82444c68
	if ctx.cr[6].gt {
		sub_82444C68(ctx, base);
		return;
	}
	// 82444C54: 814300F8  lwz r10, 0xf8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(248 as u32) ) } as u64;
	// 82444C58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82444C5C: 816400F8  lwz r11, 0xf8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(248 as u32) ) } as u64;
	// 82444C60: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82444C64: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444C68 size=8
    let mut pc: u32 = 0x82444C68;
    'dispatch: loop {
        match pc {
            0x82444C68 => {
    //   block [0x82444C68..0x82444C70)
	// 82444C68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444C70 size=20
    let mut pc: u32 = 0x82444C70;
    'dispatch: loop {
        match pc {
            0x82444C70 => {
    //   block [0x82444C70..0x82444C84)
	// 82444C70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82444C74: 806B005C  lwz r3, 0x5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82444C78: 35430001  addic. r10, r3, 1
	ctx.xer.ca = (ctx.r[3].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82444C7C: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82444C80: 4C800020  bgelr
	if !ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82444C84 size=12
    let mut pc: u32 = 0x82444C84;
    'dispatch: loop {
        match pc {
            0x82444C84 => {
    //   block [0x82444C84..0x82444C90)
	// 82444C84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82444C88: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82444C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444C90 size=120
    let mut pc: u32 = 0x82444C90;
    'dispatch: loop {
        match pc {
            0x82444C90 => {
    //   block [0x82444C90..0x82444D08)
	// 82444C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444C9C: 8163090C  lwz r11, 0x90c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2316 as u32) ) } as u64;
	// 82444CA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444CA4: 4099002C  ble cr6, 0x82444cd0
	if !ctx.cr[6].gt {
	pc = 0x82444CD0; continue 'dispatch;
	}
	// 82444CA8: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82444CAC: 409A0010  bne cr6, 0x82444cbc
	if !ctx.cr[6].eq {
	pc = 0x82444CBC; continue 'dispatch;
	}
	// 82444CB0: 81630910  lwz r11, 0x910(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2320 as u32) ) } as u64;
	// 82444CB4: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82444CB8: 419A0018  beq cr6, 0x82444cd0
	if ctx.cr[6].eq {
	pc = 0x82444CD0; continue 'dispatch;
	}
	// 82444CBC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82444CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444CCC: 4E800020  blr
	return;
	// 82444CD0: 81630D6C  lwz r11, 0xd6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3436 as u32) ) } as u64;
	// 82444CD4: 80630D70  lwz r3, 0xd70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3440 as u32) ) } as u64;
	// 82444CD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82444CDC: 419A0018  beq cr6, 0x82444cf4
	if ctx.cr[6].eq {
	pc = 0x82444CF4; continue 'dispatch;
	}
	// 82444CE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82444CE4: 4E800421  bctrl
	ctx.lr = 0x82444CE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82444CE8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82444CEC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82444CF0: 409A0008  bne cr6, 0x82444cf8
	if !ctx.cr[6].eq {
	pc = 0x82444CF8; continue 'dispatch;
	}
	// 82444CF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444D08 size=204
    let mut pc: u32 = 0x82444D08;
    'dispatch: loop {
        match pc {
            0x82444D08 => {
    //   block [0x82444D08..0x82444DD4)
	// 82444D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444D14: 396337E8  addi r11, r3, 0x37e8
	ctx.r[11].s64 = ctx.r[3].s64 + 14312;
	// 82444D18: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82444D1C: 419A009C  beq cr6, 0x82444db8
	if ctx.cr[6].eq {
	pc = 0x82444DB8; continue 'dispatch;
	}
	// 82444D20: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82444D24: 419A0094  beq cr6, 0x82444db8
	if ctx.cr[6].eq {
	pc = 0x82444DB8; continue 'dispatch;
	}
	// 82444D28: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82444D2C: 419A008C  beq cr6, 0x82444db8
	if ctx.cr[6].eq {
	pc = 0x82444DB8; continue 'dispatch;
	}
	// 82444D30: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82444D34: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 82444D38: 7F055000  cmpw cr6, r5, r10
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82444D3C: 40980020  bge cr6, 0x82444d5c
	if !ctx.cr[6].lt {
	pc = 0x82444D5C; continue 'dispatch;
	}
	// 82444D40: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444D44: 60840F1D  ori r4, r4, 0xf1d
	ctx.r[4].u64 = ctx.r[4].u64 | 3869;
	// 82444D48: 48002BC1  bl 0x82447908
	ctx.lr = 0x82444D4C;
	sub_82447908(ctx, base);
	// 82444D4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444D58: 4E800020  blr
	return;
	// 82444D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82444D60: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82444D64: 7D243214  add r9, r4, r6
	ctx.r[9].u64 = ctx.r[4].u64 + ctx.r[6].u64;
	// 82444D68: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82444D6C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82444D70: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82444D74: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82444D78: 908B000C  stw r4, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82444D7C: 394B0018  addi r10, r11, 0x18
	ctx.r[10].s64 = ctx.r[11].s64 + 24;
	// 82444D80: 90EB0010  stw r7, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82444D84: 2F050010  cmpwi cr6, r5, 0x10
	ctx.cr[6].compare_i32(ctx.r[5].s32, 16, &mut ctx.xer);
	// 82444D88: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82444D8C: 41980008  blt cr6, 0x82444d94
	if ctx.cr[6].lt {
	pc = 0x82444D94; continue 'dispatch;
	}
	// 82444D90: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82444D94: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82444D98: 40980028  bge cr6, 0x82444dc0
	if !ctx.cr[6].lt {
	pc = 0x82444DC0; continue 'dispatch;
	}
	// 82444D9C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82444DA0: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82444DA4: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 82444DA8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82444DAC: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82444DB0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82444DB4: 4BFFFFD0  b 0x82444d84
	pc = 0x82444D84; continue 'dispatch;
	// 82444DB8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82444DBC: 4BFFFB25  bl 0x824448e0
	ctx.lr = 0x82444DC0;
	sub_824448E0(ctx, base);
	// 82444DC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444DD8 size=228
    let mut pc: u32 = 0x82444DD8;
    'dispatch: loop {
        match pc {
            0x82444DD8 => {
    //   block [0x82444DD8..0x82444EBC)
	// 82444DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444DE4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82444DE8: 81680058  lwz r11, 0x58(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(88 as u32) ) } as u64;
	// 82444DEC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82444DF0: 409A0064  bne cr6, 0x82444e54
	if !ctx.cr[6].eq {
	pc = 0x82444E54; continue 'dispatch;
	}
	// 82444DF4: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444DF8: 4BFFFB69  bl 0x82444960
	ctx.lr = 0x82444DFC;
	sub_82444960(ctx, base);
	// 82444DFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82444E00: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82444E04: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82444E08: 409A0020  bne cr6, 0x82444e28
	if !ctx.cr[6].eq {
	pc = 0x82444E28; continue 'dispatch;
	}
	// 82444E0C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444E10: 60840F1F  ori r4, r4, 0xf1f
	ctx.r[4].u64 = ctx.r[4].u64 | 3871;
	// 82444E14: 48002AF5  bl 0x82447908
	ctx.lr = 0x82444E18;
	sub_82447908(ctx, base);
	// 82444E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444E24: 4E800020  blr
	return;
	// 82444E28: 4BFFFB69  bl 0x82444990
	ctx.lr = 0x82444E2C;
	sub_82444990(ctx, base);
	// 82444E2C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82444E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82444E34: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82444E38: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82444E3C: 4BFFFD7D  bl 0x82444bb8
	ctx.lr = 0x82444E40;
	sub_82444BB8(ctx, base);
	// 82444E40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82444E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444E50: 4E800020  blr
	return;
	// 82444E54: 4BFFFBBD  bl 0x82444a10
	ctx.lr = 0x82444E58;
	sub_82444A10(ctx, base);
	// 82444E58: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82444E5C: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82444E60: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444E64: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82444E68: 419A0020  beq cr6, 0x82444e88
	if ctx.cr[6].eq {
	pc = 0x82444E88; continue 'dispatch;
	}
	// 82444E6C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444E70: 60840F0E  ori r4, r4, 0xf0e
	ctx.r[4].u64 = ctx.r[4].u64 | 3854;
	// 82444E74: 48002A95  bl 0x82447908
	ctx.lr = 0x82444E78;
	sub_82447908(ctx, base);
	// 82444E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444E84: 4E800020  blr
	return;
	// 82444E88: 4BFFFA99  bl 0x82444920
	ctx.lr = 0x82444E8C;
	sub_82444920(ctx, base);
	// 82444E8C: 816827E0  lwz r11, 0x27e0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(10208 as u32) ) } as u64;
	// 82444E90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82444E94: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82444E98: 419AFF98  beq cr6, 0x82444e30
	if ctx.cr[6].eq {
	pc = 0x82444E30; continue 'dispatch;
	}
	// 82444E9C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82444EA0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82444EA4: 60840F0F  ori r4, r4, 0xf0f
	ctx.r[4].u64 = ctx.r[4].u64 | 3855;
	// 82444EA8: 48002A61  bl 0x82447908
	ctx.lr = 0x82444EAC;
	sub_82447908(ctx, base);
	// 82444EAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82444EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444EC0 size=204
    let mut pc: u32 = 0x82444EC0;
    'dispatch: loop {
        match pc {
            0x82444EC0 => {
    //   block [0x82444EC0..0x82444F8C)
	// 82444EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82444ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82444ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444ED4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82444ED8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82444EDC: 48002A05  bl 0x824478e0
	ctx.lr = 0x82444EE0;
	sub_824478E0(ctx, base);
	// 82444EE0: 817E27D4  lwz r11, 0x27d4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10196 as u32) ) } as u64;
	// 82444EE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82444EE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82444EEC: 389E27E8  addi r4, r30, 0x27e8
	ctx.r[4].s64 = ctx.r[30].s64 + 10216;
	// 82444EF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444EF4: 40990074  ble cr6, 0x82444f68
	if !ctx.cr[6].gt {
	pc = 0x82444F68; continue 'dispatch;
	}
	// 82444EF8: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82444EFC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444F00: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82444F04: 419A000C  beq cr6, 0x82444f10
	if ctx.cr[6].eq {
	pc = 0x82444F10; continue 'dispatch;
	}
	// 82444F08: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82444F0C: 409A0028  bne cr6, 0x82444f34
	if !ctx.cr[6].eq {
	pc = 0x82444F34; continue 'dispatch;
	}
	// 82444F10: 81640064  lwz r11, 0x64(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) } as u64;
	// 82444F14: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82444F18: 409A001C  bne cr6, 0x82444f34
	if !ctx.cr[6].eq {
	pc = 0x82444F34; continue 'dispatch;
	}
	// 82444F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82444F20: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82444F24: 4BFFFCE5  bl 0x82444c08
	ctx.lr = 0x82444F28;
	sub_82444C08(ctx, base);
	// 82444F28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82444F2C: 419A0008  beq cr6, 0x82444f34
	if ctx.cr[6].eq {
	pc = 0x82444F34; continue 'dispatch;
	}
	// 82444F30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82444F34: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82444F38: 38840100  addi r4, r4, 0x100
	ctx.r[4].s64 = ctx.r[4].s64 + 256;
	// 82444F3C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82444F40: 409AFFBC  bne cr6, 0x82444efc
	if !ctx.cr[6].eq {
	pc = 0x82444EFC; continue 'dispatch;
	}
	// 82444F44: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82444F48: 409A0020  bne cr6, 0x82444f68
	if !ctx.cr[6].eq {
	pc = 0x82444F68; continue 'dispatch;
	}
	// 82444F4C: 817E27D8  lwz r11, 0x27d8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10200 as u32) ) } as u64;
	// 82444F50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444F54: 409A0014  bne cr6, 0x82444f68
	if !ctx.cr[6].eq {
	pc = 0x82444F68; continue 'dispatch;
	}
	// 82444F58: 817E27DC  lwz r11, 0x27dc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10204 as u32) ) } as u64;
	// 82444F5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444F60: 409A0008  bne cr6, 0x82444f68
	if !ctx.cr[6].eq {
	pc = 0x82444F68; continue 'dispatch;
	}
	// 82444F64: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82444F68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82444F6C: 48002985  bl 0x824478f0
	ctx.lr = 0x82444F70;
	sub_824478F0(ctx, base);
	// 82444F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82444F74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82444F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82444F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82444F80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82444F84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82444F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82444F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82444F90 size=236
    let mut pc: u32 = 0x82444F90;
    'dispatch: loop {
        match pc {
            0x82444F90 => {
    //   block [0x82444F90..0x8244507C)
	// 82444F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82444F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82444F98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82444F9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82444FA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82444FA4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82444FA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82444FAC: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 82444FB0: 388627E8  addi r4, r6, 0x27e8
	ctx.r[4].s64 = ctx.r[6].s64 + 10216;
	// 82444FB4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82444FB8: 816627D4  lwz r11, 0x27d4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(10196 as u32) ) } as u64;
	// 82444FBC: 93C80000  stw r30, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82444FC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82444FC4: 93C50000  stw r30, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82444FC8: 40990070  ble cr6, 0x82445038
	if !ctx.cr[6].gt {
	pc = 0x82445038; continue 'dispatch;
	}
	// 82444FCC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82444FD0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444FD4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82444FD8: 419A000C  beq cr6, 0x82444fe4
	if ctx.cr[6].eq {
	pc = 0x82444FE4; continue 'dispatch;
	}
	// 82444FDC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82444FE0: 409A0048  bne cr6, 0x82445028
	if !ctx.cr[6].eq {
	pc = 0x82445028; continue 'dispatch;
	}
	// 82444FE4: 81640064  lwz r11, 0x64(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) } as u64;
	// 82444FE8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82444FEC: 409A003C  bne cr6, 0x82445028
	if !ctx.cr[6].eq {
	pc = 0x82445028; continue 'dispatch;
	}
	// 82444FF0: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82444FF4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82444FF8: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82444FFC: 4BFFFC0D  bl 0x82444c08
	ctx.lr = 0x82445000;
	sub_82444C08(ctx, base);
	// 82445000: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445004: 419A0010  beq cr6, 0x82445014
	if ctx.cr[6].eq {
	pc = 0x82445014; continue 'dispatch;
	}
	// 82445008: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8244500C: 90880000  stw r4, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82445010: 48000018  b 0x82445028
	pc = 0x82445028; continue 'dispatch;
	// 82445014: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82445018: 4BFFFBF1  bl 0x82444c08
	ctx.lr = 0x8244501C;
	sub_82444C08(ctx, base);
	// 8244501C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445020: 419A0008  beq cr6, 0x82445028
	if ctx.cr[6].eq {
	pc = 0x82445028; continue 'dispatch;
	}
	// 82445024: 90850000  stw r4, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82445028: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8244502C: 38840100  addi r4, r4, 0x100
	ctx.r[4].s64 = ctx.r[4].s64 + 256;
	// 82445030: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82445034: 409AFF9C  bne cr6, 0x82444fd0
	if !ctx.cr[6].eq {
	pc = 0x82444FD0; continue 'dispatch;
	}
	// 82445038: 816627D8  lwz r11, 0x27d8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(10200 as u32) ) } as u64;
	// 8244503C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82445040: 409A0008  bne cr6, 0x82445048
	if !ctx.cr[6].eq {
	pc = 0x82445048; continue 'dispatch;
	}
	// 82445044: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82445048: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8244504C: 4199000C  bgt cr6, 0x82445058
	if ctx.cr[6].gt {
	pc = 0x82445058; continue 'dispatch;
	}
	// 82445050: 93C80000  stw r30, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82445054: 4800000C  b 0x82445060
	pc = 0x82445060; continue 'dispatch;
	// 82445058: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 8244505C: 409A0008  bne cr6, 0x82445064
	if !ctx.cr[6].eq {
	pc = 0x82445064; continue 'dispatch;
	}
	// 82445060: 93C50000  stw r30, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82445064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82445068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244506C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445070: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82445074: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445080 size=92
    let mut pc: u32 = 0x82445080;
    'dispatch: loop {
        match pc {
            0x82445080 => {
    //   block [0x82445080..0x824450DC)
	// 82445080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445084: 480F0035  bl 0x825350b8
	ctx.lr = 0x82445088;
	sub_82535080(ctx, base);
	// 82445088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244508C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445090: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82445094: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82445098: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8244509C: 480027A5  bl 0x82447840
	ctx.lr = 0x824450A0;
	sub_82447840(ctx, base);
	// 824450A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824450A4: 419A001C  beq cr6, 0x824450c0
	if ctx.cr[6].eq {
	pc = 0x824450C0; continue 'dispatch;
	}
	// 824450A8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824450AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824450B0: 60840185  ori r4, r4, 0x185
	ctx.r[4].u64 = ctx.r[4].u64 | 389;
	// 824450B4: 48002855  bl 0x82447908
	ctx.lr = 0x824450B8;
	sub_82447908(ctx, base);
	// 824450B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824450BC: 480F004C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824450C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824450C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824450C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824450CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824450D0: 4BFFFC39  bl 0x82444d08
	ctx.lr = 0x824450D4;
	sub_82444D08(ctx, base);
	// 824450D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824450D8: 480F0030  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824450E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824450E0 size=24
    let mut pc: u32 = 0x824450E0;
    'dispatch: loop {
        match pc {
            0x824450E0 => {
    //   block [0x824450E0..0x824450F8)
	// 824450E0: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824450E4: 396BC858  addi r11, r11, -0x37a8
	ctx.r[11].s64 = ctx.r[11].s64 + -14248;
	// 824450E8: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824450EC: 80AB0034  lwz r5, 0x34(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824450F0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824450F4: 4BFFFC14  b 0x82444d08
	sub_82444D08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824450F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824450F8 size=196
    let mut pc: u32 = 0x824450F8;
    'dispatch: loop {
        match pc {
            0x824450F8 => {
    //   block [0x824450F8..0x824451BC)
	// 824450F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824450FC: 480EFFB9  bl 0x825350b4
	ctx.lr = 0x82445100;
	sub_82535080(ctx, base);
	// 82445100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445104: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445108: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244510C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82445110: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82445114: 4BFFFDAD  bl 0x82444ec0
	ctx.lr = 0x82445118;
	sub_82444EC0(ctx, base);
	// 82445118: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8244511C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82445120: 409A0018  bne cr6, 0x82445138
	if !ctx.cr[6].eq {
	pc = 0x82445138; continue 'dispatch;
	}
	// 82445124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82445128: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244512C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82445130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82445134: 480EFFD0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82445138: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244513C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445140: 4BFFF899  bl 0x824449d8
	ctx.lr = 0x82445144;
	sub_824449D8(ctx, base);
	// 82445144: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82445148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244514C: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82445150: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 82445154: 4E800421  bctrl
	ctx.lr = 0x82445158;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82445158: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244515C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445160: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82445164: 917F1008  stw r11, 0x1008(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4104 as u32), ctx.r[11].u32 ) };
	// 82445168: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244516C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82445170: 917F100C  stw r11, 0x100c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4108 as u32), ctx.r[11].u32 ) };
	// 82445174: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82445178: 4BFFAD81  bl 0x8243fef8
	ctx.lr = 0x8244517C;
	sub_8243FEF8(ctx, base);
	// 8244517C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445180: 419AFFA4  beq cr6, 0x82445124
	if ctx.cr[6].eq {
	pc = 0x82445124; continue 'dispatch;
	}
	// 82445184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445188: 4BFFA129  bl 0x8243f2b0
	ctx.lr = 0x8244518C;
	sub_8243F2B0(ctx, base);
	// 8244518C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445190: 419AFF94  beq cr6, 0x82445124
	if ctx.cr[6].eq {
	pc = 0x82445124; continue 'dispatch;
	}
	// 82445194: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82445198: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8244519C: 409A0014  bne cr6, 0x824451b0
	if !ctx.cr[6].eq {
	pc = 0x824451B0; continue 'dispatch;
	}
	// 824451A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824451A4: 4BFFFACD  bl 0x82444c70
	ctx.lr = 0x824451A8;
	sub_82444C70(ctx, base);
	// 824451A8: 907D0064  stw r3, 0x64(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 824451AC: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824451B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824451B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824451B8: 480EFF4C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824451C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824451C0 size=72
    let mut pc: u32 = 0x824451C0;
    'dispatch: loop {
        match pc {
            0x824451C0 => {
    //   block [0x824451C0..0x82445208)
	// 824451C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824451C4: 480EFEF9  bl 0x825350bc
	ctx.lr = 0x824451C8;
	sub_82535080(ctx, base);
	// 824451C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824451CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824451D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824451D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824451D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824451DC: 48002705  bl 0x824478e0
	ctx.lr = 0x824451E0;
	sub_824478E0(ctx, base);
	// 824451E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824451E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824451E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824451EC: 4BFFFBED  bl 0x82444dd8
	ctx.lr = 0x824451F0;
	sub_82444DD8(ctx, base);
	// 824451F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824451F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824451F8: 480026F9  bl 0x824478f0
	ctx.lr = 0x824451FC;
	sub_824478F0(ctx, base);
	// 824451FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82445204: 480EFF08  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445208 size=156
    let mut pc: u32 = 0x82445208;
    'dispatch: loop {
        match pc {
            0x82445208 => {
    //   block [0x82445208..0x824452A4)
	// 82445208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82445214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244521C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82445220: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82445224: 480026BD  bl 0x824478e0
	ctx.lr = 0x82445228;
	sub_824478E0(ctx, base);
	// 82445228: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 8244522C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82445230: 409A004C  bne cr6, 0x8244527c
	if !ctx.cr[6].eq {
	pc = 0x8244527C; continue 'dispatch;
	}
	// 82445234: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82445238: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8244523C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82445240: 4BFFFD51  bl 0x82444f90
	ctx.lr = 0x82445244;
	sub_82444F90(ctx, base);
	// 82445244: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82445248: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8244524C: 419A0034  beq cr6, 0x82445280
	if ctx.cr[6].eq {
	pc = 0x82445280; continue 'dispatch;
	}
	// 82445250: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82445254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82445258: 4BFF74B1  bl 0x8243c708
	ctx.lr = 0x8244525C;
	sub_8243C708(ctx, base);
	// 8244525C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445260: 419A0020  beq cr6, 0x82445280
	if ctx.cr[6].eq {
	pc = 0x82445280; continue 'dispatch;
	}
	// 82445264: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82445268: 80BF0048  lwz r5, 0x48(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8244526C: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82445270: 4BFFAB21  bl 0x8243fd90
	ctx.lr = 0x82445274;
	sub_8243FD90(ctx, base);
	// 82445274: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445278: 409A0008  bne cr6, 0x82445280
	if !ctx.cr[6].eq {
	pc = 0x82445280; continue 'dispatch;
	}
	// 8244527C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82445280: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82445284: 4800266D  bl 0x824478f0
	ctx.lr = 0x82445288;
	sub_824478F0(ctx, base);
	// 82445288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244528C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82445290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244529C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824452A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824452A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824452A8 size=112
    let mut pc: u32 = 0x824452A8;
    'dispatch: loop {
        match pc {
            0x824452A8 => {
    //   block [0x824452A8..0x82445318)
	// 824452A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824452AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824452B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824452B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824452B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824452BC: 48002585  bl 0x82447840
	ctx.lr = 0x824452C0;
	sub_82447840(ctx, base);
	// 824452C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824452C4: 419A002C  beq cr6, 0x824452f0
	if ctx.cr[6].eq {
	pc = 0x824452F0; continue 'dispatch;
	}
	// 824452C8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824452CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824452D0: 60840183  ori r4, r4, 0x183
	ctx.r[4].u64 = ctx.r[4].u64 | 387;
	// 824452D4: 48002635  bl 0x82447908
	ctx.lr = 0x824452D8;
	sub_82447908(ctx, base);
	// 824452D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824452DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824452E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824452E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824452E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824452EC: 4E800020  blr
	return;
	// 824452F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824452F4: 4BFFFF15  bl 0x82445208
	ctx.lr = 0x824452F8;
	sub_82445208(ctx, base);
	// 824452F8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824452FC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82445300: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82445304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445318 size=12
    let mut pc: u32 = 0x82445318;
    'dispatch: loop {
        match pc {
            0x82445318 => {
    //   block [0x82445318..0x82445324)
	// 82445318: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8244531C: 386B619C  addi r3, r11, 0x619c
	ctx.r[3].s64 = ctx.r[11].s64 + 24988;
	// 82445320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445328 size=92
    let mut pc: u32 = 0x82445328;
    'dispatch: loop {
        match pc {
            0x82445328 => {
    //   block [0x82445328..0x82445384)
	// 82445328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244532C: 480EFD91  bl 0x825350bc
	ctx.lr = 0x82445330;
	sub_82535080(ctx, base);
	// 82445330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445334: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82445338: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244533C: 3BEBC890  addi r31, r11, -0x3770
	ctx.r[31].s64 = ctx.r[11].s64 + -14192;
	// 82445340: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82445344: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82445348: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244534C: 41990030  bgt cr6, 0x8244537c
	if ctx.cr[6].gt {
	pc = 0x8244537C; continue 'dispatch;
	}
	// 82445350: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82445354: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82445358: 4BFFFFC1  bl 0x82445318
	ctx.lr = 0x8244535C;
	sub_82445318(ctx, base);
	// 8244535C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82445360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82445364: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82445368: 4800AC79  bl 0x8244ffe0
	ctx.lr = 0x8244536C;
	sub_8244FFE0(ctx, base);
	// 8244536C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82445370: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82445374: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82445378: 4800AC39  bl 0x8244ffb0
	ctx.lr = 0x8244537C;
	sub_8244FFB0(ctx, base);
	// 8244537C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82445380: 480EFD8C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445388 size=152
    let mut pc: u32 = 0x82445388;
    'dispatch: loop {
        match pc {
            0x82445388 => {
    //   block [0x82445388..0x82445420)
	// 82445388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244538C: 480EFD21  bl 0x825350ac
	ctx.lr = 0x82445390;
	sub_82535080(ctx, base);
	// 82445390: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445394: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82445398: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8244539C: 3BEBC898  addi r31, r11, -0x3768
	ctx.r[31].s64 = ctx.r[11].s64 + -14184;
	// 824453A0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824453A4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824453A8: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824453AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824453B0: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824453B4: 41980010  blt cr6, 0x824453c4
	if ctx.cr[6].lt {
	pc = 0x824453C4; continue 'dispatch;
	}
	// 824453B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824453BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824453C0: 480EFD3C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 824453C4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824453C8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824453CC: 4099002C  ble cr6, 0x824453f8
	if !ctx.cr[6].gt {
	pc = 0x824453F8; continue 'dispatch;
	}
	// 824453D0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824453D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824453D8: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 824453DC: 4800AC4D  bl 0x82450028
	ctx.lr = 0x824453E0;
	sub_82450028(ctx, base);
	// 824453E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824453E4: 409A0014  bne cr6, 0x824453f8
	if !ctx.cr[6].eq {
	pc = 0x824453F8; continue 'dispatch;
	}
	// 824453E8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824453EC: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 824453F0: 7F1CE800  cmpw cr6, r28, r29
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824453F4: 4198FFE0  blt cr6, 0x824453d4
	if ctx.cr[6].lt {
	pc = 0x824453D4; continue 'dispatch;
	}
	// 824453F8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 824453FC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82445400: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82445404: 4800ABC5  bl 0x8244ffc8
	ctx.lr = 0x82445408;
	sub_8244FFC8(ctx, base);
	// 82445408: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244540C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82445410: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82445414: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82445418: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8244541C: 480EFCE0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445420 size=52
    let mut pc: u32 = 0x82445420;
    'dispatch: loop {
        match pc {
            0x82445420 => {
    //   block [0x82445420..0x82445454)
	// 82445420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244542C: 4801E4DD  bl 0x82463908
	ctx.lr = 0x82445430;
	sub_82463908(ctx, base);
	// 82445430: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82445434: 396BC898  addi r11, r11, -0x3768
	ctx.r[11].s64 = ctx.r[11].s64 + -14184;
	// 82445438: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244543C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82445440: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82445444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244544C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445458 size=392
    let mut pc: u32 = 0x82445458;
    'dispatch: loop {
        match pc {
            0x82445458 => {
    //   block [0x82445458..0x824455E0)
	// 82445458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244545C: 480EFC61  bl 0x825350bc
	ctx.lr = 0x82445460;
	sub_82535080(ctx, base);
	// 82445460: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445464: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82445468: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8244546C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445470: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82445474: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82445478: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8244547C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82445480: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82445484: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82445488: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8244548C: 4800AB9D  bl 0x82450028
	ctx.lr = 0x82445490;
	sub_82450028(ctx, base);
	// 82445490: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445494: 419A00E4  beq cr6, 0x82445578
	if ctx.cr[6].eq {
	pc = 0x82445578; continue 'dispatch;
	}
	// 82445498: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244549C: 2B0B0800  cmplwi cr6, r11, 0x800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2048 as u32, &mut ctx.xer);
	// 824454A0: 40980018  bge cr6, 0x824454b8
	if !ctx.cr[6].lt {
	pc = 0x824454B8; continue 'dispatch;
	}
	// 824454A4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824454A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824454AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824454B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824454B4: 480EFC58  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824454B8: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 824454BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824454C0: 4800B919  bl 0x82450dd8
	ctx.lr = 0x824454C4;
	sub_82450DD8(ctx, base);
	// 824454C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824454C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824454CC: 419A005C  beq cr6, 0x82445528
	if ctx.cr[6].eq {
	pc = 0x82445528; continue 'dispatch;
	}
	// 824454D0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824454D4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 824454D8: 4800C681  bl 0x82451b58
	ctx.lr = 0x824454DC;
	sub_82451B58(ctx, base);
	// 824454DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824454E0: 419A0098  beq cr6, 0x82445578
	if ctx.cr[6].eq {
	pc = 0x82445578; continue 'dispatch;
	}
	// 824454E4: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 824454E8: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 824454EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824454F0: 4800B9F9  bl 0x82450ee8
	ctx.lr = 0x824454F4;
	sub_82450EE8(ctx, base);
	// 824454F4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824454F8: 419A0080  beq cr6, 0x82445578
	if ctx.cr[6].eq {
	pc = 0x82445578; continue 'dispatch;
	}
	// 824454FC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82445500: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82445504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445508: 4800B9F9  bl 0x82450f00
	ctx.lr = 0x8244550C;
	sub_82450F00(ctx, base);
	// 8244550C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445510: 409A000C  bne cr6, 0x8244551c
	if !ctx.cr[6].eq {
	pc = 0x8244551C; continue 'dispatch;
	}
	// 82445514: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82445518: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8244551C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82445520: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82445524: 4800006C  b 0x82445590
	pc = 0x82445590; continue 'dispatch;
	// 82445528: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8244552C: 4800AC15  bl 0x82450140
	ctx.lr = 0x82445530;
	sub_82450140(ctx, base);
	// 82445530: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445534: 419A0050  beq cr6, 0x82445584
	if ctx.cr[6].eq {
	pc = 0x82445584; continue 'dispatch;
	}
	// 82445538: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8244553C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82445540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445544: 4800B80D  bl 0x82450d50
	ctx.lr = 0x82445548;
	sub_82450D50(ctx, base);
	// 82445548: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 8244554C: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82445550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445554: 4800AD9D  bl 0x824502f0
	ctx.lr = 0x82445558;
	sub_824502F0(ctx, base);
	// 82445558: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244555C: 419A001C  beq cr6, 0x82445578
	if ctx.cr[6].eq {
	pc = 0x82445578; continue 'dispatch;
	}
	// 82445560: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82445564: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82445568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244556C: 4800ADB5  bl 0x82450320
	ctx.lr = 0x82445570;
	sub_82450320(ctx, base);
	// 82445570: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445574: 409AFFA8  bne cr6, 0x8244551c
	if !ctx.cr[6].eq {
	pc = 0x8244551C; continue 'dispatch;
	}
	// 82445578: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244557C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82445580: 480EFB8C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82445584: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82445588: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8244558C: 409AFF18  bne cr6, 0x824454a4
	if !ctx.cr[6].eq {
	pc = 0x824454A4; continue 'dispatch;
	}
	// 82445590: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82445594: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82445598: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8244559C: 1D2B0064  mulli r9, r11, 0x64
	ctx.r[9].s64 = ctx.r[11].s64 * 100;
	// 824455A0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824455A4: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824455A8: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824455AC: 1D4B0064  mulli r10, r11, 0x64
	ctx.r[10].s64 = ctx.r[11].s64 * 100;
	// 824455B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824455B4: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824455B8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824455BC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824455C0: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 824455C4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824455C8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824455CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824455D0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824455D4: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824455D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824455DC: 480EFB30  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824455E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824455E0 size=16
    let mut pc: u32 = 0x824455E0;
    'dispatch: loop {
        match pc {
            0x824455E0 => {
    //   block [0x824455E0..0x824455F0)
	// 824455E0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824455E4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824455E8: 40980008  bge cr6, 0x824455f0
	if !ctx.cr[6].lt {
		sub_824455F0(ctx, base);
		return;
	}
	// 824455EC: 4800C424  b 0x82451a10
	sub_82451A10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824455F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824455F0 size=12
    let mut pc: u32 = 0x824455F0;
    'dispatch: loop {
        match pc {
            0x824455F0 => {
    //   block [0x824455F0..0x824455FC)
	// 824455F0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824455F4: 40980008  bge cr6, 0x824455fc
	if !ctx.cr[6].lt {
		sub_824455FC(ctx, base);
		return;
	}
	// 824455F8: 4800B648  b 0x82450c40
	sub_82450C40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824455FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824455FC size=8
    let mut pc: u32 = 0x824455FC;
    'dispatch: loop {
        match pc {
            0x824455FC => {
    //   block [0x824455FC..0x82445604)
	// 824455FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445608 size=16
    let mut pc: u32 = 0x82445608;
    'dispatch: loop {
        match pc {
            0x82445608 => {
    //   block [0x82445608..0x82445618)
	// 82445608: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244560C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445610: 40980008  bge cr6, 0x82445618
	if !ctx.cr[6].lt {
		sub_82445618(ctx, base);
		return;
	}
	// 82445614: 4800C49C  b 0x82451ab0
	sub_82451AB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445618 size=12
    let mut pc: u32 = 0x82445618;
    'dispatch: loop {
        match pc {
            0x82445618 => {
    //   block [0x82445618..0x82445624)
	// 82445618: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244561C: 40980008  bge cr6, 0x82445624
	if !ctx.cr[6].lt {
		sub_82445624(ctx, base);
		return;
	}
	// 82445620: 4800B660  b 0x82450c80
	sub_82450C80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445624(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445624 size=8
    let mut pc: u32 = 0x82445624;
    'dispatch: loop {
        match pc {
            0x82445624 => {
    //   block [0x82445624..0x8244562C)
	// 82445624: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445630 size=16
    let mut pc: u32 = 0x82445630;
    'dispatch: loop {
        match pc {
            0x82445630 => {
    //   block [0x82445630..0x82445640)
	// 82445630: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445634: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445638: 40980008  bge cr6, 0x82445640
	if !ctx.cr[6].lt {
		sub_82445640(ctx, base);
		return;
	}
	// 8244563C: 4800C51C  b 0x82451b58
	sub_82451B58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445640 size=12
    let mut pc: u32 = 0x82445640;
    'dispatch: loop {
        match pc {
            0x82445640 => {
    //   block [0x82445640..0x8244564C)
	// 82445640: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445644: 40980008  bge cr6, 0x8244564c
	if !ctx.cr[6].lt {
		sub_8244564C(ctx, base);
		return;
	}
	// 82445648: 4800B708  b 0x82450d50
	sub_82450D50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244564C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244564C size=8
    let mut pc: u32 = 0x8244564C;
    'dispatch: loop {
        match pc {
            0x8244564C => {
    //   block [0x8244564C..0x82445654)
	// 8244564C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445658 size=16
    let mut pc: u32 = 0x82445658;
    'dispatch: loop {
        match pc {
            0x82445658 => {
    //   block [0x82445658..0x82445668)
	// 82445658: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244565C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445660: 40980008  bge cr6, 0x82445668
	if !ctx.cr[6].lt {
		sub_82445668(ctx, base);
		return;
	}
	// 82445664: 4800B8B4  b 0x82450f18
	sub_82450F18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445668 size=12
    let mut pc: u32 = 0x82445668;
    'dispatch: loop {
        match pc {
            0x82445668 => {
    //   block [0x82445668..0x82445674)
	// 82445668: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244566C: 40980008  bge cr6, 0x82445674
	if !ctx.cr[6].lt {
		sub_82445674(ctx, base);
		return;
	}
	// 82445670: 4800ACE0  b 0x82450350
	sub_82450350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445674 size=8
    let mut pc: u32 = 0x82445674;
    'dispatch: loop {
        match pc {
            0x82445674 => {
    //   block [0x82445674..0x8244567C)
	// 82445674: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445680 size=16
    let mut pc: u32 = 0x82445680;
    'dispatch: loop {
        match pc {
            0x82445680 => {
    //   block [0x82445680..0x82445690)
	// 82445680: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445684: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445688: 40980008  bge cr6, 0x82445690
	if !ctx.cr[6].lt {
		sub_82445690(ctx, base);
		return;
	}
	// 8244568C: 4800B8DC  b 0x82450f68
	sub_82450F68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445690 size=12
    let mut pc: u32 = 0x82445690;
    'dispatch: loop {
        match pc {
            0x82445690 => {
    //   block [0x82445690..0x8244569C)
	// 82445690: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445694: 40980008  bge cr6, 0x8244569c
	if !ctx.cr[6].lt {
		sub_8244569C(ctx, base);
		return;
	}
	// 82445698: 4800ACE8  b 0x82450380
	sub_82450380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244569C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244569C size=8
    let mut pc: u32 = 0x8244569C;
    'dispatch: loop {
        match pc {
            0x8244569C => {
    //   block [0x8244569C..0x824456A4)
	// 8244569C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824456A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824456A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824456A8 size=16
    let mut pc: u32 = 0x824456A8;
    'dispatch: loop {
        match pc {
            0x824456A8 => {
    //   block [0x824456A8..0x824456B8)
	// 824456A8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824456AC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824456B0: 40980008  bge cr6, 0x824456b8
	if !ctx.cr[6].lt {
		sub_824456B8(ctx, base);
		return;
	}
	// 824456B4: 4800B8D4  b 0x82450f88
	sub_82450F88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824456B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824456B8 size=12
    let mut pc: u32 = 0x824456B8;
    'dispatch: loop {
        match pc {
            0x824456B8 => {
    //   block [0x824456B8..0x824456C4)
	// 824456B8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824456BC: 40980008  bge cr6, 0x824456c4
	if !ctx.cr[6].lt {
		sub_824456C4(ctx, base);
		return;
	}
	// 824456C0: 4800ACD0  b 0x82450390
	sub_82450390(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824456C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824456C4 size=8
    let mut pc: u32 = 0x824456C4;
    'dispatch: loop {
        match pc {
            0x824456C4 => {
    //   block [0x824456C4..0x824456CC)
	// 824456C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824456C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824456D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824456D0 size=16
    let mut pc: u32 = 0x824456D0;
    'dispatch: loop {
        match pc {
            0x824456D0 => {
    //   block [0x824456D0..0x824456E0)
	// 824456D0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824456D4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824456D8: 40980008  bge cr6, 0x824456e0
	if !ctx.cr[6].lt {
		sub_824456E0(ctx, base);
		return;
	}
	// 824456DC: 4800B8E4  b 0x82450fc0
	sub_82450FC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824456E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824456E0 size=12
    let mut pc: u32 = 0x824456E0;
    'dispatch: loop {
        match pc {
            0x824456E0 => {
    //   block [0x824456E0..0x824456EC)
	// 824456E0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824456E4: 40980008  bge cr6, 0x824456ec
	if !ctx.cr[6].lt {
		sub_824456EC(ctx, base);
		return;
	}
	// 824456E8: 4800AC68  b 0x82450350
	sub_82450350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824456EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824456EC size=8
    let mut pc: u32 = 0x824456EC;
    'dispatch: loop {
        match pc {
            0x824456EC => {
    //   block [0x824456EC..0x824456F4)
	// 824456EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824456F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824456F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824456F8 size=16
    let mut pc: u32 = 0x824456F8;
    'dispatch: loop {
        match pc {
            0x824456F8 => {
    //   block [0x824456F8..0x82445708)
	// 824456F8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824456FC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445700: 40980008  bge cr6, 0x82445708
	if !ctx.cr[6].lt {
		sub_82445708(ctx, base);
		return;
	}
	// 82445704: 4800B96C  b 0x82451070
	sub_82451070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445708 size=12
    let mut pc: u32 = 0x82445708;
    'dispatch: loop {
        match pc {
            0x82445708 => {
    //   block [0x82445708..0x82445714)
	// 82445708: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244570C: 40980008  bge cr6, 0x82445714
	if !ctx.cr[6].lt {
		sub_82445714(ctx, base);
		return;
	}
	// 82445710: 4800AC90  b 0x824503a0
	sub_824503A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445714 size=8
    let mut pc: u32 = 0x82445714;
    'dispatch: loop {
        match pc {
            0x82445714 => {
    //   block [0x82445714..0x8244571C)
	// 82445714: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445720 size=16
    let mut pc: u32 = 0x82445720;
    'dispatch: loop {
        match pc {
            0x82445720 => {
    //   block [0x82445720..0x82445730)
	// 82445720: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445724: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445728: 40980008  bge cr6, 0x82445730
	if !ctx.cr[6].lt {
		sub_82445730(ctx, base);
		return;
	}
	// 8244572C: 4800B95C  b 0x82451088
	sub_82451088(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445730 size=12
    let mut pc: u32 = 0x82445730;
    'dispatch: loop {
        match pc {
            0x82445730 => {
    //   block [0x82445730..0x8244573C)
	// 82445730: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445734: 40980008  bge cr6, 0x8244573c
	if !ctx.cr[6].lt {
		sub_8244573C(ctx, base);
		return;
	}
	// 82445738: 4800AC80  b 0x824503b8
	sub_824503B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244573C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244573C size=8
    let mut pc: u32 = 0x8244573C;
    'dispatch: loop {
        match pc {
            0x8244573C => {
    //   block [0x8244573C..0x82445744)
	// 8244573C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445748 size=16
    let mut pc: u32 = 0x82445748;
    'dispatch: loop {
        match pc {
            0x82445748 => {
    //   block [0x82445748..0x82445758)
	// 82445748: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244574C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445750: 40980008  bge cr6, 0x82445758
	if !ctx.cr[6].lt {
		sub_82445758(ctx, base);
		return;
	}
	// 82445754: 4800B94C  b 0x824510a0
	sub_824510A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445758 size=12
    let mut pc: u32 = 0x82445758;
    'dispatch: loop {
        match pc {
            0x82445758 => {
    //   block [0x82445758..0x82445764)
	// 82445758: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244575C: 40980008  bge cr6, 0x82445764
	if !ctx.cr[6].lt {
		sub_82445764(ctx, base);
		return;
	}
	// 82445760: 4800AC70  b 0x824503d0
	sub_824503D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445764 size=8
    let mut pc: u32 = 0x82445764;
    'dispatch: loop {
        match pc {
            0x82445764 => {
    //   block [0x82445764..0x8244576C)
	// 82445764: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445770 size=16
    let mut pc: u32 = 0x82445770;
    'dispatch: loop {
        match pc {
            0x82445770 => {
    //   block [0x82445770..0x82445780)
	// 82445770: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445774: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445778: 40980008  bge cr6, 0x82445780
	if !ctx.cr[6].lt {
		sub_82445780(ctx, base);
		return;
	}
	// 8244577C: 4800B93C  b 0x824510b8
	sub_824510B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445780 size=12
    let mut pc: u32 = 0x82445780;
    'dispatch: loop {
        match pc {
            0x82445780 => {
    //   block [0x82445780..0x8244578C)
	// 82445780: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445784: 40980008  bge cr6, 0x8244578c
	if !ctx.cr[6].lt {
		sub_8244578C(ctx, base);
		return;
	}
	// 82445788: 4800AC60  b 0x824503e8
	sub_824503E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244578C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244578C size=8
    let mut pc: u32 = 0x8244578C;
    'dispatch: loop {
        match pc {
            0x8244578C => {
    //   block [0x8244578C..0x82445794)
	// 8244578C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445798 size=16
    let mut pc: u32 = 0x82445798;
    'dispatch: loop {
        match pc {
            0x82445798 => {
    //   block [0x82445798..0x824457A8)
	// 82445798: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244579C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824457A0: 40980008  bge cr6, 0x824457a8
	if !ctx.cr[6].lt {
		sub_824457A8(ctx, base);
		return;
	}
	// 824457A4: 4800B92C  b 0x824510d0
	sub_824510D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824457A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824457A8 size=12
    let mut pc: u32 = 0x824457A8;
    'dispatch: loop {
        match pc {
            0x824457A8 => {
    //   block [0x824457A8..0x824457B4)
	// 824457A8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824457AC: 40980008  bge cr6, 0x824457b4
	if !ctx.cr[6].lt {
		sub_824457B4(ctx, base);
		return;
	}
	// 824457B0: 4800AC50  b 0x82450400
	sub_82450400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824457B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824457B4 size=8
    let mut pc: u32 = 0x824457B4;
    'dispatch: loop {
        match pc {
            0x824457B4 => {
    //   block [0x824457B4..0x824457BC)
	// 824457B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824457B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824457C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824457C0 size=16
    let mut pc: u32 = 0x824457C0;
    'dispatch: loop {
        match pc {
            0x824457C0 => {
    //   block [0x824457C0..0x824457D0)
	// 824457C0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824457C4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824457C8: 40980008  bge cr6, 0x824457d0
	if !ctx.cr[6].lt {
		sub_824457D0(ctx, base);
		return;
	}
	// 824457CC: 4800B95C  b 0x82451128
	sub_82451128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824457D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824457D0 size=12
    let mut pc: u32 = 0x824457D0;
    'dispatch: loop {
        match pc {
            0x824457D0 => {
    //   block [0x824457D0..0x824457DC)
	// 824457D0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824457D4: 40980008  bge cr6, 0x824457dc
	if !ctx.cr[6].lt {
		sub_824457DC(ctx, base);
		return;
	}
	// 824457D8: 4800AC60  b 0x82450438
	sub_82450438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824457DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824457DC size=8
    let mut pc: u32 = 0x824457DC;
    'dispatch: loop {
        match pc {
            0x824457DC => {
    //   block [0x824457DC..0x824457E4)
	// 824457DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824457E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824457E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824457E8 size=16
    let mut pc: u32 = 0x824457E8;
    'dispatch: loop {
        match pc {
            0x824457E8 => {
    //   block [0x824457E8..0x824457F8)
	// 824457E8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824457EC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824457F0: 40980008  bge cr6, 0x824457f8
	if !ctx.cr[6].lt {
		sub_824457F8(ctx, base);
		return;
	}
	// 824457F4: 4800B974  b 0x82451168
	sub_82451168(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824457F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824457F8 size=12
    let mut pc: u32 = 0x824457F8;
    'dispatch: loop {
        match pc {
            0x824457F8 => {
    //   block [0x824457F8..0x82445804)
	// 824457F8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824457FC: 40980008  bge cr6, 0x82445804
	if !ctx.cr[6].lt {
		sub_82445804(ctx, base);
		return;
	}
	// 82445800: 4800AC70  b 0x82450470
	sub_82450470(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445804 size=8
    let mut pc: u32 = 0x82445804;
    'dispatch: loop {
        match pc {
            0x82445804 => {
    //   block [0x82445804..0x8244580C)
	// 82445804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445810 size=16
    let mut pc: u32 = 0x82445810;
    'dispatch: loop {
        match pc {
            0x82445810 => {
    //   block [0x82445810..0x82445820)
	// 82445810: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445814: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445818: 40980008  bge cr6, 0x82445820
	if !ctx.cr[6].lt {
		sub_82445820(ctx, base);
		return;
	}
	// 8244581C: 4800B98C  b 0x824511a8
	sub_824511A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445820 size=12
    let mut pc: u32 = 0x82445820;
    'dispatch: loop {
        match pc {
            0x82445820 => {
    //   block [0x82445820..0x8244582C)
	// 82445820: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445824: 40980008  bge cr6, 0x8244582c
	if !ctx.cr[6].lt {
		sub_8244582C(ctx, base);
		return;
	}
	// 82445828: 4800AC80  b 0x824504a8
	sub_824504A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244582C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244582C size=8
    let mut pc: u32 = 0x8244582C;
    'dispatch: loop {
        match pc {
            0x8244582C => {
    //   block [0x8244582C..0x82445834)
	// 8244582C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445838 size=16
    let mut pc: u32 = 0x82445838;
    'dispatch: loop {
        match pc {
            0x82445838 => {
    //   block [0x82445838..0x82445848)
	// 82445838: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244583C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445840: 40980008  bge cr6, 0x82445848
	if !ctx.cr[6].lt {
		sub_82445848(ctx, base);
		return;
	}
	// 82445844: 4800B9A4  b 0x824511e8
	sub_824511E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445848 size=12
    let mut pc: u32 = 0x82445848;
    'dispatch: loop {
        match pc {
            0x82445848 => {
    //   block [0x82445848..0x82445854)
	// 82445848: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244584C: 40980008  bge cr6, 0x82445854
	if !ctx.cr[6].lt {
		sub_82445854(ctx, base);
		return;
	}
	// 82445850: 4800AC90  b 0x824504e0
	sub_824504E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445854(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445854 size=8
    let mut pc: u32 = 0x82445854;
    'dispatch: loop {
        match pc {
            0x82445854 => {
    //   block [0x82445854..0x8244585C)
	// 82445854: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445860 size=16
    let mut pc: u32 = 0x82445860;
    'dispatch: loop {
        match pc {
            0x82445860 => {
    //   block [0x82445860..0x82445870)
	// 82445860: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445864: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445868: 40980008  bge cr6, 0x82445870
	if !ctx.cr[6].lt {
		sub_82445870(ctx, base);
		return;
	}
	// 8244586C: 4800BA24  b 0x82451290
	sub_82451290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445870 size=12
    let mut pc: u32 = 0x82445870;
    'dispatch: loop {
        match pc {
            0x82445870 => {
    //   block [0x82445870..0x8244587C)
	// 82445870: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445874: 40980008  bge cr6, 0x8244587c
	if !ctx.cr[6].lt {
		sub_8244587C(ctx, base);
		return;
	}
	// 82445878: 4800ACB0  b 0x82450528
	sub_82450528(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244587C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244587C size=8
    let mut pc: u32 = 0x8244587C;
    'dispatch: loop {
        match pc {
            0x8244587C => {
    //   block [0x8244587C..0x82445884)
	// 8244587C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445888 size=16
    let mut pc: u32 = 0x82445888;
    'dispatch: loop {
        match pc {
            0x82445888 => {
    //   block [0x82445888..0x82445898)
	// 82445888: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244588C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445890: 40980008  bge cr6, 0x82445898
	if !ctx.cr[6].lt {
		sub_82445898(ctx, base);
		return;
	}
	// 82445894: 4800BAB4  b 0x82451348
	sub_82451348(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445898 size=12
    let mut pc: u32 = 0x82445898;
    'dispatch: loop {
        match pc {
            0x82445898 => {
    //   block [0x82445898..0x824458A4)
	// 82445898: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244589C: 40980008  bge cr6, 0x824458a4
	if !ctx.cr[6].lt {
		sub_824458A4(ctx, base);
		return;
	}
	// 824458A0: 4800AD68  b 0x82450608
	sub_82450608(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824458A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824458A4 size=8
    let mut pc: u32 = 0x824458A4;
    'dispatch: loop {
        match pc {
            0x824458A4 => {
    //   block [0x824458A4..0x824458AC)
	// 824458A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824458A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824458B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824458B0 size=16
    let mut pc: u32 = 0x824458B0;
    'dispatch: loop {
        match pc {
            0x824458B0 => {
    //   block [0x824458B0..0x824458C0)
	// 824458B0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824458B4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824458B8: 40980008  bge cr6, 0x824458c0
	if !ctx.cr[6].lt {
		sub_824458C0(ctx, base);
		return;
	}
	// 824458BC: 4800BAF4  b 0x824513b0
	sub_824513B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824458C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824458C0 size=12
    let mut pc: u32 = 0x824458C0;
    'dispatch: loop {
        match pc {
            0x824458C0 => {
    //   block [0x824458C0..0x824458CC)
	// 824458C0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824458C4: 40980008  bge cr6, 0x824458cc
	if !ctx.cr[6].lt {
		sub_824458CC(ctx, base);
		return;
	}
	// 824458C8: 4800ADD8  b 0x824506a0
	sub_824506A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824458CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824458CC size=8
    let mut pc: u32 = 0x824458CC;
    'dispatch: loop {
        match pc {
            0x824458CC => {
    //   block [0x824458CC..0x824458D4)
	// 824458CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824458D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824458D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824458D8 size=16
    let mut pc: u32 = 0x824458D8;
    'dispatch: loop {
        match pc {
            0x824458D8 => {
    //   block [0x824458D8..0x824458E8)
	// 824458D8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824458DC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824458E0: 40980008  bge cr6, 0x824458e8
	if !ctx.cr[6].lt {
		sub_824458E8(ctx, base);
		return;
	}
	// 824458E4: 4800BB24  b 0x82451408
	sub_82451408(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824458E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824458E8 size=12
    let mut pc: u32 = 0x824458E8;
    'dispatch: loop {
        match pc {
            0x824458E8 => {
    //   block [0x824458E8..0x824458F4)
	// 824458E8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824458EC: 40980008  bge cr6, 0x824458f4
	if !ctx.cr[6].lt {
		sub_824458F4(ctx, base);
		return;
	}
	// 824458F0: 4800AE00  b 0x824506f0
	sub_824506F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824458F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824458F4 size=8
    let mut pc: u32 = 0x824458F4;
    'dispatch: loop {
        match pc {
            0x824458F4 => {
    //   block [0x824458F4..0x824458FC)
	// 824458F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824458F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445900 size=16
    let mut pc: u32 = 0x82445900;
    'dispatch: loop {
        match pc {
            0x82445900 => {
    //   block [0x82445900..0x82445910)
	// 82445900: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445904: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445908: 40980008  bge cr6, 0x82445910
	if !ctx.cr[6].lt {
		sub_82445910(ctx, base);
		return;
	}
	// 8244590C: 4800BB84  b 0x82451490
	sub_82451490(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445910 size=12
    let mut pc: u32 = 0x82445910;
    'dispatch: loop {
        match pc {
            0x82445910 => {
    //   block [0x82445910..0x8244591C)
	// 82445910: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445914: 40980008  bge cr6, 0x8244591c
	if !ctx.cr[6].lt {
		sub_8244591C(ctx, base);
		return;
	}
	// 82445918: 4800AE40  b 0x82450758
	sub_82450758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244591C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244591C size=8
    let mut pc: u32 = 0x8244591C;
    'dispatch: loop {
        match pc {
            0x8244591C => {
    //   block [0x8244591C..0x82445924)
	// 8244591C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445928 size=16
    let mut pc: u32 = 0x82445928;
    'dispatch: loop {
        match pc {
            0x82445928 => {
    //   block [0x82445928..0x82445938)
	// 82445928: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244592C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445930: 40980008  bge cr6, 0x82445938
	if !ctx.cr[6].lt {
		sub_82445938(ctx, base);
		return;
	}
	// 82445934: 4800BC04  b 0x82451538
	sub_82451538(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445938 size=12
    let mut pc: u32 = 0x82445938;
    'dispatch: loop {
        match pc {
            0x82445938 => {
    //   block [0x82445938..0x82445944)
	// 82445938: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244593C: 40980008  bge cr6, 0x82445944
	if !ctx.cr[6].lt {
		sub_82445944(ctx, base);
		return;
	}
	// 82445940: 4800AEE0  b 0x82450820
	sub_82450820(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445944 size=8
    let mut pc: u32 = 0x82445944;
    'dispatch: loop {
        match pc {
            0x82445944 => {
    //   block [0x82445944..0x8244594C)
	// 82445944: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445950 size=16
    let mut pc: u32 = 0x82445950;
    'dispatch: loop {
        match pc {
            0x82445950 => {
    //   block [0x82445950..0x82445960)
	// 82445950: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445954: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445958: 40980008  bge cr6, 0x82445960
	if !ctx.cr[6].lt {
		sub_82445960(ctx, base);
		return;
	}
	// 8244595C: 4800BC5C  b 0x824515b8
	sub_824515B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445960 size=12
    let mut pc: u32 = 0x82445960;
    'dispatch: loop {
        match pc {
            0x82445960 => {
    //   block [0x82445960..0x8244596C)
	// 82445960: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445964: 40980008  bge cr6, 0x8244596c
	if !ctx.cr[6].lt {
		sub_8244596C(ctx, base);
		return;
	}
	// 82445968: 4800AF30  b 0x82450898
	sub_82450898(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244596C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244596C size=8
    let mut pc: u32 = 0x8244596C;
    'dispatch: loop {
        match pc {
            0x8244596C => {
    //   block [0x8244596C..0x82445974)
	// 8244596C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445978 size=16
    let mut pc: u32 = 0x82445978;
    'dispatch: loop {
        match pc {
            0x82445978 => {
    //   block [0x82445978..0x82445988)
	// 82445978: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244597C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445980: 40980008  bge cr6, 0x82445988
	if !ctx.cr[6].lt {
		sub_82445988(ctx, base);
		return;
	}
	// 82445984: 4800BCBC  b 0x82451640
	sub_82451640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445988 size=12
    let mut pc: u32 = 0x82445988;
    'dispatch: loop {
        match pc {
            0x82445988 => {
    //   block [0x82445988..0x82445994)
	// 82445988: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8244598C: 40980008  bge cr6, 0x82445994
	if !ctx.cr[6].lt {
		sub_82445994(ctx, base);
		return;
	}
	// 82445990: 4800AF80  b 0x82450910
	sub_82450910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445994 size=8
    let mut pc: u32 = 0x82445994;
    'dispatch: loop {
        match pc {
            0x82445994 => {
    //   block [0x82445994..0x8244599C)
	// 82445994: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824459A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824459A0 size=16
    let mut pc: u32 = 0x824459A0;
    'dispatch: loop {
        match pc {
            0x824459A0 => {
    //   block [0x824459A0..0x824459B0)
	// 824459A0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824459A4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824459A8: 40980008  bge cr6, 0x824459b0
	if !ctx.cr[6].lt {
		sub_824459B0(ctx, base);
		return;
	}
	// 824459AC: 4800BCF4  b 0x824516a0
	sub_824516A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824459B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824459B0 size=12
    let mut pc: u32 = 0x824459B0;
    'dispatch: loop {
        match pc {
            0x824459B0 => {
    //   block [0x824459B0..0x824459BC)
	// 824459B0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824459B4: 40980008  bge cr6, 0x824459bc
	if !ctx.cr[6].lt {
		sub_824459BC(ctx, base);
		return;
	}
	// 824459B8: 4800AFB8  b 0x82450970
	sub_82450970(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824459BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824459BC size=8
    let mut pc: u32 = 0x824459BC;
    'dispatch: loop {
        match pc {
            0x824459BC => {
    //   block [0x824459BC..0x824459C4)
	// 824459BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824459C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824459C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824459C8 size=16
    let mut pc: u32 = 0x824459C8;
    'dispatch: loop {
        match pc {
            0x824459C8 => {
    //   block [0x824459C8..0x824459D8)
	// 824459C8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824459CC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824459D0: 40980008  bge cr6, 0x824459d8
	if !ctx.cr[6].lt {
		sub_824459D8(ctx, base);
		return;
	}
	// 824459D4: 4800BD24  b 0x824516f8
	sub_824516F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824459D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824459D8 size=12
    let mut pc: u32 = 0x824459D8;
    'dispatch: loop {
        match pc {
            0x824459D8 => {
    //   block [0x824459D8..0x824459E4)
	// 824459D8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 824459DC: 40980008  bge cr6, 0x824459e4
	if !ctx.cr[6].lt {
		sub_824459E4(ctx, base);
		return;
	}
	// 824459E0: 4800AFD8  b 0x824509b8
	sub_824509B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824459E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824459E4 size=8
    let mut pc: u32 = 0x824459E4;
    'dispatch: loop {
        match pc {
            0x824459E4 => {
    //   block [0x824459E4..0x824459EC)
	// 824459E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824459E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824459F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824459F0 size=16
    let mut pc: u32 = 0x824459F0;
    'dispatch: loop {
        match pc {
            0x824459F0 => {
    //   block [0x824459F0..0x82445A00)
	// 824459F0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824459F4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 824459F8: 40980008  bge cr6, 0x82445a00
	if !ctx.cr[6].lt {
		sub_82445A00(ctx, base);
		return;
	}
	// 824459FC: 4800BD54  b 0x82451750
	sub_82451750(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A00 size=12
    let mut pc: u32 = 0x82445A00;
    'dispatch: loop {
        match pc {
            0x82445A00 => {
    //   block [0x82445A00..0x82445A0C)
	// 82445A00: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445A04: 40980008  bge cr6, 0x82445a0c
	if !ctx.cr[6].lt {
		sub_82445A0C(ctx, base);
		return;
	}
	// 82445A08: 4800AFF8  b 0x82450a00
	sub_82450A00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A0C size=8
    let mut pc: u32 = 0x82445A0C;
    'dispatch: loop {
        match pc {
            0x82445A0C => {
    //   block [0x82445A0C..0x82445A14)
	// 82445A0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A18 size=16
    let mut pc: u32 = 0x82445A18;
    'dispatch: loop {
        match pc {
            0x82445A18 => {
    //   block [0x82445A18..0x82445A28)
	// 82445A18: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445A1C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445A20: 40980008  bge cr6, 0x82445a28
	if !ctx.cr[6].lt {
		sub_82445A28(ctx, base);
		return;
	}
	// 82445A24: 4800BD8C  b 0x824517b0
	sub_824517B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A28 size=12
    let mut pc: u32 = 0x82445A28;
    'dispatch: loop {
        match pc {
            0x82445A28 => {
    //   block [0x82445A28..0x82445A34)
	// 82445A28: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445A2C: 40980008  bge cr6, 0x82445a34
	if !ctx.cr[6].lt {
		sub_82445A34(ctx, base);
		return;
	}
	// 82445A30: 4800B020  b 0x82450a50
	sub_82450A50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A34 size=8
    let mut pc: u32 = 0x82445A34;
    'dispatch: loop {
        match pc {
            0x82445A34 => {
    //   block [0x82445A34..0x82445A3C)
	// 82445A34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A40 size=16
    let mut pc: u32 = 0x82445A40;
    'dispatch: loop {
        match pc {
            0x82445A40 => {
    //   block [0x82445A40..0x82445A50)
	// 82445A40: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445A44: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445A48: 40980008  bge cr6, 0x82445a50
	if !ctx.cr[6].lt {
		sub_82445A50(ctx, base);
		return;
	}
	// 82445A4C: 4800BDC4  b 0x82451810
	sub_82451810(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A50 size=12
    let mut pc: u32 = 0x82445A50;
    'dispatch: loop {
        match pc {
            0x82445A50 => {
    //   block [0x82445A50..0x82445A5C)
	// 82445A50: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445A54: 40980008  bge cr6, 0x82445a5c
	if !ctx.cr[6].lt {
		sub_82445A5C(ctx, base);
		return;
	}
	// 82445A58: 4800B048  b 0x82450aa0
	sub_82450AA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A5C size=8
    let mut pc: u32 = 0x82445A5C;
    'dispatch: loop {
        match pc {
            0x82445A5C => {
    //   block [0x82445A5C..0x82445A64)
	// 82445A5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A68 size=16
    let mut pc: u32 = 0x82445A68;
    'dispatch: loop {
        match pc {
            0x82445A68 => {
    //   block [0x82445A68..0x82445A78)
	// 82445A68: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445A6C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445A70: 40980008  bge cr6, 0x82445a78
	if !ctx.cr[6].lt {
		sub_82445A78(ctx, base);
		return;
	}
	// 82445A74: 4800BDF4  b 0x82451868
	sub_82451868(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A78 size=12
    let mut pc: u32 = 0x82445A78;
    'dispatch: loop {
        match pc {
            0x82445A78 => {
    //   block [0x82445A78..0x82445A84)
	// 82445A78: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445A7C: 40980008  bge cr6, 0x82445a84
	if !ctx.cr[6].lt {
		sub_82445A84(ctx, base);
		return;
	}
	// 82445A80: 4800B068  b 0x82450ae8
	sub_82450AE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A84 size=8
    let mut pc: u32 = 0x82445A84;
    'dispatch: loop {
        match pc {
            0x82445A84 => {
    //   block [0x82445A84..0x82445A8C)
	// 82445A84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445A90 size=16
    let mut pc: u32 = 0x82445A90;
    'dispatch: loop {
        match pc {
            0x82445A90 => {
    //   block [0x82445A90..0x82445AA0)
	// 82445A90: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445A94: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445A98: 40980008  bge cr6, 0x82445aa0
	if !ctx.cr[6].lt {
		sub_82445AA0(ctx, base);
		return;
	}
	// 82445A9C: 4800BE34  b 0x824518d0
	sub_824518D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445AA0 size=12
    let mut pc: u32 = 0x82445AA0;
    'dispatch: loop {
        match pc {
            0x82445AA0 => {
    //   block [0x82445AA0..0x82445AAC)
	// 82445AA0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445AA4: 40980008  bge cr6, 0x82445aac
	if !ctx.cr[6].lt {
		sub_82445AAC(ctx, base);
		return;
	}
	// 82445AA8: 4800B098  b 0x82450b40
	sub_82450B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445AAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445AAC size=8
    let mut pc: u32 = 0x82445AAC;
    'dispatch: loop {
        match pc {
            0x82445AAC => {
    //   block [0x82445AAC..0x82445AB4)
	// 82445AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445AB8 size=16
    let mut pc: u32 = 0x82445AB8;
    'dispatch: loop {
        match pc {
            0x82445AB8 => {
    //   block [0x82445AB8..0x82445AC8)
	// 82445AB8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82445ABC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 82445AC0: 40980008  bge cr6, 0x82445ac8
	if !ctx.cr[6].lt {
		sub_82445AC8(ctx, base);
		return;
	}
	// 82445AC4: 4800BE74  b 0x82451938
	sub_82451938(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445AC8 size=12
    let mut pc: u32 = 0x82445AC8;
    'dispatch: loop {
        match pc {
            0x82445AC8 => {
    //   block [0x82445AC8..0x82445AD4)
	// 82445AC8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 82445ACC: 40980008  bge cr6, 0x82445ad4
	if !ctx.cr[6].lt {
		sub_82445AD4(ctx, base);
		return;
	}
	// 82445AD0: 4800B0C8  b 0x82450b98
	sub_82450B98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445AD4 size=8
    let mut pc: u32 = 0x82445AD4;
    'dispatch: loop {
        match pc {
            0x82445AD4 => {
    //   block [0x82445AD4..0x82445ADC)
	// 82445AD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445AE0 size=84
    let mut pc: u32 = 0x82445AE0;
    'dispatch: loop {
        match pc {
            0x82445AE0 => {
    //   block [0x82445AE0..0x82445B34)
	// 82445AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445AE4: 480EF5D9  bl 0x825350bc
	ctx.lr = 0x82445AE8;
	sub_82535080(ctx, base);
	// 82445AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445AEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82445AF0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445AF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445AF8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82445AFC: 4BFF6C0D  bl 0x8243c708
	ctx.lr = 0x82445B00;
	sub_8243C708(ctx, base);
	// 82445B00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445B04: 419A0028  beq cr6, 0x82445b2c
	if ctx.cr[6].eq {
	pc = 0x82445B2C; continue 'dispatch;
	}
	// 82445B08: 817F219C  lwz r11, 0x219c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8604 as u32) ) } as u64;
	// 82445B0C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82445B10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82445B14: 419A0018  beq cr6, 0x82445b2c
	if ctx.cr[6].eq {
	pc = 0x82445B2C; continue 'dispatch;
	}
	// 82445B18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82445B1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82445B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445B24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82445B28: 4E800421  bctrl
	ctx.lr = 0x82445B2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82445B2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82445B30: 480EF5DC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445B38 size=92
    let mut pc: u32 = 0x82445B38;
    'dispatch: loop {
        match pc {
            0x82445B38 => {
    //   block [0x82445B38..0x82445B94)
	// 82445B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445B40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445B44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445B48: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82445B4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445B50: 480033F9  bl 0x82448f48
	ctx.lr = 0x82445B54;
	sub_82448F48(ctx, base);
	// 82445B54: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445B58: 419A0028  beq cr6, 0x82445b80
	if ctx.cr[6].eq {
	pc = 0x82445B80; continue 'dispatch;
	}
	// 82445B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445B60: 809F21A4  lwz r4, 0x21a4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8612 as u32) ) } as u64;
	// 82445B64: 48002865  bl 0x824483c8
	ctx.lr = 0x82445B68;
	sub_824483C8(ctx, base);
	// 82445B68: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445B6C: 409A0014  bne cr6, 0x82445b80
	if !ctx.cr[6].eq {
	pc = 0x82445B80; continue 'dispatch;
	}
	// 82445B70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82445B74: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82445B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445B7C: 480033BD  bl 0x82448f38
	ctx.lr = 0x82445B80;
	sub_82448F38(ctx, base);
	// 82445B80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445B98 size=92
    let mut pc: u32 = 0x82445B98;
    'dispatch: loop {
        match pc {
            0x82445B98 => {
    //   block [0x82445B98..0x82445BF4)
	// 82445B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445BA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445BA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445BA8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82445BAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445BB0: 480033B9  bl 0x82448f68
	ctx.lr = 0x82445BB4;
	sub_82448F68(ctx, base);
	// 82445BB4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445BB8: 419A0028  beq cr6, 0x82445be0
	if ctx.cr[6].eq {
	pc = 0x82445BE0; continue 'dispatch;
	}
	// 82445BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445BC0: 809F21A4  lwz r4, 0x21a4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8612 as u32) ) } as u64;
	// 82445BC4: 4800283D  bl 0x82448400
	ctx.lr = 0x82445BC8;
	sub_82448400(ctx, base);
	// 82445BC8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445BCC: 409A0014  bne cr6, 0x82445be0
	if !ctx.cr[6].eq {
	pc = 0x82445BE0; continue 'dispatch;
	}
	// 82445BD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82445BD4: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82445BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445BDC: 4800337D  bl 0x82448f58
	ctx.lr = 0x82445BE0;
	sub_82448F58(ctx, base);
	// 82445BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445BF8 size=40
    let mut pc: u32 = 0x82445BF8;
    'dispatch: loop {
        match pc {
            0x82445BF8 => {
    //   block [0x82445BF8..0x82445C20)
	// 82445BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445C04: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445C08: 4BFF6B01  bl 0x8243c708
	ctx.lr = 0x82445C0C;
	sub_8243C708(ctx, base);
	// 82445C0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445C20 size=100
    let mut pc: u32 = 0x82445C20;
    'dispatch: loop {
        match pc {
            0x82445C20 => {
    //   block [0x82445C20..0x82445C84)
	// 82445C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445C28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445C2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445C30: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445C38: 4BFF6AD1  bl 0x8243c708
	ctx.lr = 0x82445C3C;
	sub_8243C708(ctx, base);
	// 82445C3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445C40: 409A0018  bne cr6, 0x82445c58
	if !ctx.cr[6].eq {
	pc = 0x82445C58; continue 'dispatch;
	}
	// 82445C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445C50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445C54: 4E800020  blr
	return;
	// 82445C58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82445C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82445C60: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82445C64: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82445C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445C6C: 4800328D  bl 0x82448ef8
	ctx.lr = 0x82445C70;
	sub_82448EF8(ctx, base);
	// 82445C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445C7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445C88 size=100
    let mut pc: u32 = 0x82445C88;
    'dispatch: loop {
        match pc {
            0x82445C88 => {
    //   block [0x82445C88..0x82445CEC)
	// 82445C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445C90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445C94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445C98: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445C9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445CA0: 4BFF6A69  bl 0x8243c708
	ctx.lr = 0x82445CA4;
	sub_8243C708(ctx, base);
	// 82445CA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445CA8: 409A0018  bne cr6, 0x82445cc0
	if !ctx.cr[6].eq {
	pc = 0x82445CC0; continue 'dispatch;
	}
	// 82445CAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445CB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445CBC: 4E800020  blr
	return;
	// 82445CC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82445CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82445CC8: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82445CCC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82445CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445CD4: 48003225  bl 0x82448ef8
	ctx.lr = 0x82445CD8;
	sub_82448EF8(ctx, base);
	// 82445CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445CF0 size=92
    let mut pc: u32 = 0x82445CF0;
    'dispatch: loop {
        match pc {
            0x82445CF0 => {
    //   block [0x82445CF0..0x82445D4C)
	// 82445CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82445CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445D04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82445D08: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445D10: 4BFF69F9  bl 0x8243c708
	ctx.lr = 0x82445D14;
	sub_8243C708(ctx, base);
	// 82445D14: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445D18: 419A001C  beq cr6, 0x82445d34
	if ctx.cr[6].eq {
	pc = 0x82445D34; continue 'dispatch;
	}
	// 82445D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82445D20: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82445D24: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82445D28: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82445D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445D30: 480031C9  bl 0x82448ef8
	ctx.lr = 0x82445D34;
	sub_82448EF8(ctx, base);
	// 82445D34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82445D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445D40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82445D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445D50 size=12
    let mut pc: u32 = 0x82445D50;
    'dispatch: loop {
        match pc {
            0x82445D50 => {
    //   block [0x82445D50..0x82445D5C)
	// 82445D50: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82445D54: 60840A01  ori r4, r4, 0xa01
	ctx.r[4].u64 = ctx.r[4].u64 | 2561;
	// 82445D58: 48001BB0  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445D60 size=56
    let mut pc: u32 = 0x82445D60;
    'dispatch: loop {
        match pc {
            0x82445D60 => {
    //   block [0x82445D60..0x82445D98)
	// 82445D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445D70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445D74: 4BFFFDC5  bl 0x82445b38
	ctx.lr = 0x82445D78;
	sub_82445B38(ctx, base);
	// 82445D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445D7C: 4BFFFE1D  bl 0x82445b98
	ctx.lr = 0x82445D80;
	sub_82445B98(ctx, base);
	// 82445D80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445D84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445D98 size=68
    let mut pc: u32 = 0x82445D98;
    'dispatch: loop {
        match pc {
            0x82445D98 => {
    //   block [0x82445D98..0x82445DDC)
	// 82445D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445DA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445DA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445DA8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445DAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445DB0: 4BFF6959  bl 0x8243c708
	ctx.lr = 0x82445DB4;
	sub_8243C708(ctx, base);
	// 82445DB4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445DB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445DBC: 419A000C  beq cr6, 0x82445dc8
	if ctx.cr[6].eq {
	pc = 0x82445DC8; continue 'dispatch;
	}
	// 82445DC0: 397F2604  addi r11, r31, 0x2604
	ctx.r[11].s64 = ctx.r[31].s64 + 9732;
	// 82445DC4: 917F219C  stw r11, 0x219c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8604 as u32), ctx.r[11].u32 ) };
	// 82445DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445DD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445DE0 size=84
    let mut pc: u32 = 0x82445DE0;
    'dispatch: loop {
        match pc {
            0x82445DE0 => {
    //   block [0x82445DE0..0x82445E34)
	// 82445DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445DF0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445DF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445DF8: 4BFF6911  bl 0x8243c708
	ctx.lr = 0x82445DFC;
	sub_8243C708(ctx, base);
	// 82445DFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445E00: 409A0018  bne cr6, 0x82445e18
	if !ctx.cr[6].eq {
	pc = 0x82445E18; continue 'dispatch;
	}
	// 82445E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445E10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445E14: 4E800020  blr
	return;
	// 82445E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445E1C: 4BFFFF45  bl 0x82445d60
	ctx.lr = 0x82445E20;
	sub_82445D60(ctx, base);
	// 82445E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445E2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445E38 size=100
    let mut pc: u32 = 0x82445E38;
    'dispatch: loop {
        match pc {
            0x82445E38 => {
    //   block [0x82445E38..0x82445E9C)
	// 82445E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445E48: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82445E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445E50: 4BFF68B9  bl 0x8243c708
	ctx.lr = 0x82445E54;
	sub_8243C708(ctx, base);
	// 82445E54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445E58: 409A001C  bne cr6, 0x82445e74
	if !ctx.cr[6].eq {
	pc = 0x82445E74; continue 'dispatch;
	}
	// 82445E5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82445E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445E70: 4E800020  blr
	return;
	// 82445E74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445E78: 4BFF92F1  bl 0x8243f168
	ctx.lr = 0x82445E7C;
	sub_8243F168(ctx, base);
	// 82445E7C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82445E80: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82445E84: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82445E88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445E94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445EA0 size=12
    let mut pc: u32 = 0x82445EA0;
    'dispatch: loop {
        match pc {
            0x82445EA0 => {
    //   block [0x82445EA0..0x82445EAC)
	// 82445EA0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82445EA4: 60840701  ori r4, r4, 0x701
	ctx.r[4].u64 = ctx.r[4].u64 | 1793;
	// 82445EA8: 48001A60  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445EB0 size=40
    let mut pc: u32 = 0x82445EB0;
    'dispatch: loop {
        match pc {
            0x82445EB0 => {
    //   block [0x82445EB0..0x82445ED8)
	// 82445EB0: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82445EB4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82445EB8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82445EBC: 419A001C  beq cr6, 0x82445ed8
	if ctx.cr[6].eq {
		sub_82445ED8(ctx, base);
		return;
	}
	// 82445EC0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82445EC4: 419A0014  beq cr6, 0x82445ed8
	if ctx.cr[6].eq {
		sub_82445ED8(ctx, base);
		return;
	}
	// 82445EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82445ECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82445ED0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82445ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445ED8 size=16
    let mut pc: u32 = 0x82445ED8;
    'dispatch: loop {
        match pc {
            0x82445ED8 => {
    //   block [0x82445ED8..0x82445EE8)
	// 82445ED8: 81632160  lwz r11, 0x2160(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8544 as u32) ) } as u64;
	// 82445EDC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82445EE0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82445EE4: 4800243C  b 0x82448320
	sub_82448320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82445EE8 size=20
    let mut pc: u32 = 0x82445EE8;
    'dispatch: loop {
        match pc {
            0x82445EE8 => {
    //   block [0x82445EE8..0x82445EFC)
	// 82445EE8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82445EEC: 80832160  lwz r4, 0x2160(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8544 as u32) ) } as u64;
	// 82445EF0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82445EF4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82445EF8: 48002458  b 0x82448350
	sub_82448350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445F00 size=108
    let mut pc: u32 = 0x82445F00;
    'dispatch: loop {
        match pc {
            0x82445F00 => {
    //   block [0x82445F00..0x82445F6C)
	// 82445F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445F08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445F10: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445F18: 48003051  bl 0x82448f68
	ctx.lr = 0x82445F1C;
	sub_82448F68(ctx, base);
	// 82445F1C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445F20: 419A0038  beq cr6, 0x82445f58
	if ctx.cr[6].eq {
	pc = 0x82445F58; continue 'dispatch;
	}
	// 82445F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445F28: 809F2160  lwz r4, 0x2160(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8544 as u32) ) } as u64;
	// 82445F2C: 480024D5  bl 0x82448400
	ctx.lr = 0x82445F30;
	sub_82448400(ctx, base);
	// 82445F30: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445F34: 409A0024  bne cr6, 0x82445f58
	if !ctx.cr[6].eq {
	pc = 0x82445F58; continue 'dispatch;
	}
	// 82445F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445F3C: 4BFFFEFD  bl 0x82445e38
	ctx.lr = 0x82445F40;
	sub_82445E38(ctx, base);
	// 82445F40: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445F44: 419A0014  beq cr6, 0x82445f58
	if ctx.cr[6].eq {
	pc = 0x82445F58; continue 'dispatch;
	}
	// 82445F48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82445F4C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445F54: 48003005  bl 0x82448f58
	ctx.lr = 0x82445F58;
	sub_82448F58(ctx, base);
	// 82445F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445F70 size=108
    let mut pc: u32 = 0x82445F70;
    'dispatch: loop {
        match pc {
            0x82445F70 => {
    //   block [0x82445F70..0x82445FDC)
	// 82445F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445F78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445F7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445F80: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445F88: 48002FC1  bl 0x82448f48
	ctx.lr = 0x82445F8C;
	sub_82448F48(ctx, base);
	// 82445F8C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445F90: 419A0038  beq cr6, 0x82445fc8
	if ctx.cr[6].eq {
	pc = 0x82445FC8; continue 'dispatch;
	}
	// 82445F94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445F98: 809F2160  lwz r4, 0x2160(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8544 as u32) ) } as u64;
	// 82445F9C: 4800242D  bl 0x824483c8
	ctx.lr = 0x82445FA0;
	sub_824483C8(ctx, base);
	// 82445FA0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82445FA4: 409A0024  bne cr6, 0x82445fc8
	if !ctx.cr[6].eq {
	pc = 0x82445FC8; continue 'dispatch;
	}
	// 82445FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445FAC: 4BFF173D  bl 0x824376e8
	ctx.lr = 0x82445FB0;
	sub_824376E8(ctx, base);
	// 82445FB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82445FB4: 419A0014  beq cr6, 0x82445fc8
	if ctx.cr[6].eq {
	pc = 0x82445FC8; continue 'dispatch;
	}
	// 82445FB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82445FBC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82445FC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82445FC4: 48002F75  bl 0x82448f38
	ctx.lr = 0x82445FC8;
	sub_82448F38(ctx, base);
	// 82445FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82445FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82445FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82445FD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82445FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82445FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82445FE0 size=100
    let mut pc: u32 = 0x82445FE0;
    'dispatch: loop {
        match pc {
            0x82445FE0 => {
    //   block [0x82445FE0..0x82446044)
	// 82445FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82445FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82445FE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82445FEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82445FF0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82445FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82445FF8: 4BFF6711  bl 0x8243c708
	ctx.lr = 0x82445FFC;
	sub_8243C708(ctx, base);
	// 82445FFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446000: 409A0018  bne cr6, 0x82446018
	if !ctx.cr[6].eq {
	pc = 0x82446018; continue 'dispatch;
	}
	// 82446004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446010: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446014: 4E800020  blr
	return;
	// 82446018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244601C: 4BFFFEE5  bl 0x82445f00
	ctx.lr = 0x82446020;
	sub_82445F00(ctx, base);
	// 82446020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446024: 4BFFFF4D  bl 0x82445f70
	ctx.lr = 0x82446028;
	sub_82445F70(ctx, base);
	// 82446028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244602C: 4BFF16DD  bl 0x82437708
	ctx.lr = 0x82446030;
	sub_82437708(ctx, base);
	// 82446030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244603C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446048 size=12
    let mut pc: u32 = 0x82446048;
    'dispatch: loop {
        match pc {
            0x82446048 => {
    //   block [0x82446048..0x82446054)
	// 82446048: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 8244604C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446050: 4BFD49B0  b 0x8241aa00
	sub_8241AA00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446058 size=12
    let mut pc: u32 = 0x82446058;
    'dispatch: loop {
        match pc {
            0x82446058 => {
    //   block [0x82446058..0x82446064)
	// 82446058: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 8244605C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446060: 4BFD49D8  b 0x8241aa38
	sub_8241AA38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446068 size=12
    let mut pc: u32 = 0x82446068;
    'dispatch: loop {
        match pc {
            0x82446068 => {
    //   block [0x82446068..0x82446074)
	// 82446068: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 8244606C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446070: 4BFD4A30  b 0x8241aaa0
	sub_8241AAA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446078 size=12
    let mut pc: u32 = 0x82446078;
    'dispatch: loop {
        match pc {
            0x82446078 => {
    //   block [0x82446078..0x82446084)
	// 82446078: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 8244607C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446080: 4BFD4AA8  b 0x8241ab28
	sub_8241AB28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82446088 size=336
    let mut pc: u32 = 0x82446088;
    'dispatch: loop {
        match pc {
            0x82446088 => {
    //   block [0x82446088..0x824461D8)
	// 82446088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244608C: 480EF02D  bl 0x825350b8
	ctx.lr = 0x82446090;
	sub_82535080(ctx, base);
	// 82446090: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446094: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82446098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244609C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824460A0: 817D208C  lwz r11, 0x208c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824460A4: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824460A8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824460AC: 419A0124  beq cr6, 0x824461d0
	if ctx.cr[6].eq {
	pc = 0x824461D0; continue 'dispatch;
	}
	// 824460B0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824460B4: 409A0014  bne cr6, 0x824460c8
	if !ctx.cr[6].eq {
	pc = 0x824460C8; continue 'dispatch;
	}
	// 824460B8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824460BC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 824460C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824460C4: 480000F4  b 0x824461b8
	pc = 0x824461B8; continue 'dispatch;
	// 824460C8: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824460CC: 409A0010  bne cr6, 0x824460dc
	if !ctx.cr[6].eq {
	pc = 0x824460DC; continue 'dispatch;
	}
	// 824460D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824460D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824460D8: 480000E0  b 0x824461b8
	pc = 0x824461B8; continue 'dispatch;
	// 824460DC: 7FEA07B4  extsw r10, r31
	ctx.r[10].s64 = ctx.r[31].s32 as i64;
	// 824460E0: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 824460E4: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 824460E8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824460EC: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824460F0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824460F4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824460F8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 824460FC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82446100: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82446104: FC206824  fdiv f1, f0, f13
	ctx.f[1].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 82446108: 480ECED1  bl 0x82532fd8
	ctx.lr = 0x8244610C;
	sub_82532FD8(ctx, base);
	// 8244610C: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82446110: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82446114: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82446118: C00B6290  lfs f0, 0x6290(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25232 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8244611C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82446120: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82446124: C1AB20B0  lfs f13, 0x20b0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8368 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82446128: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8244612C: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82446130: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82446134: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446138: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8244613C: 1D640064  mulli r11, r4, 0x64
	ctx.r[11].s64 = ctx.r[4].s64 * 100;
	// 82446140: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82446144: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82446148: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8244614C: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82446150: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82446154: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82446158: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8244615C: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82446160: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82446164: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82446168: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8244616C: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 82446170: 7D8057AE  stfiwx f12, 0, r10
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82446174: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446178: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8244617C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82446180: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82446184: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82446188: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8244618C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82446190: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 82446194: 40990018  ble cr6, 0x824461ac
	if !ctx.cr[6].gt {
	pc = 0x824461AC; continue 'dispatch;
	}
	// 82446198: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8244619C: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 824461A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824461A4: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 824461A8: 48000010  b 0x824461b8
	pc = 0x824461B8; continue 'dispatch;
	// 824461AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824461B0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 824461B4: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824461B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824461BC: 4BFD4D35  bl 0x8241aef0
	ctx.lr = 0x824461C0;
	sub_8241AEF0(ctx, base);
	// 824461C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824461C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824461C8: 387D1088  addi r3, r29, 0x1088
	ctx.r[3].s64 = ctx.r[29].s64 + 4232;
	// 824461CC: 4800BB3D  bl 0x82451d08
	ctx.lr = 0x824461D0;
	sub_82451D08(ctx, base);
	// 824461D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824461D4: 480EEF34  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824461D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824461D8 size=84
    let mut pc: u32 = 0x824461D8;
    'dispatch: loop {
        match pc {
            0x824461D8 => {
    //   block [0x824461D8..0x8244622C)
	// 824461D8: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824461DC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824461E0: 396BC8A4  addi r11, r11, -0x375c
	ctx.r[11].s64 = ctx.r[11].s64 + -14172;
	// 824461E4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824461E8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824461EC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824461F0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824461F4: 394A001F  addi r10, r10, 0x1f
	ctx.r[10].s64 = ctx.r[10].s64 + 31;
	// 824461F8: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824461FC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82446200: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82446204: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82446208: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244620C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82446210: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82446214: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82446218: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8244621C: 394A001F  addi r10, r10, 0x1f
	ctx.r[10].s64 = ctx.r[10].s64 + 31;
	// 82446220: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82446224: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82446228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446230 size=60
    let mut pc: u32 = 0x82446230;
    'dispatch: loop {
        match pc {
            0x82446230 => {
    //   block [0x82446230..0x8244626C)
	// 82446230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244623C: 4BFD5AE5  bl 0x8241bd20
	ctx.lr = 0x82446240;
	sub_8241BD20(ctx, base);
	// 82446240: 4815FB51  bl 0x825a5d90
	ctx.lr = 0x82446244;
	sub_825A5D90(ctx, base);
	// 82446244: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82446248: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8244624C: 386BC8A4  addi r3, r11, -0x375c
	ctx.r[3].s64 = ctx.r[11].s64 + -14172;
	// 82446250: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82446254: 4800399D  bl 0x82449bf0
	ctx.lr = 0x82446258;
	sub_82449BF0(ctx, base);
	// 82446258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244625C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446270 size=40
    let mut pc: u32 = 0x82446270;
    'dispatch: loop {
        match pc {
            0x82446270 => {
    //   block [0x82446270..0x82446298)
	// 82446270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244627C: 4815FB2D  bl 0x825a5da8
	ctx.lr = 0x82446280;
	sub_825A5DA8(ctx, base);
	// 82446280: 4BFD5BE1  bl 0x8241be60
	ctx.lr = 0x82446284;
	sub_8241BE60(ctx, base);
	// 82446284: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244628C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446298 size=12
    let mut pc: u32 = 0x82446298;
    'dispatch: loop {
        match pc {
            0x82446298 => {
    //   block [0x82446298..0x824462A4)
	// 82446298: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8244629C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 824462A0: 4BFFE190  b 0x82444430
	sub_82444430(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824462A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824462A8 size=28
    let mut pc: u32 = 0x824462A8;
    'dispatch: loop {
        match pc {
            0x824462A8 => {
    //   block [0x824462A8..0x824462C4)
	// 824462A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824462AC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 824462B0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824462B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824462B8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824462BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824462C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824462C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824462C8 size=128
    let mut pc: u32 = 0x824462C8;
    'dispatch: loop {
        match pc {
            0x824462C8 => {
    //   block [0x824462C8..0x82446348)
	// 824462C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824462CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824462D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824462D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824462D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824462DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824462E0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824462E4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824462E8: 419A0030  beq cr6, 0x82446318
	if ctx.cr[6].eq {
	pc = 0x82446318; continue 'dispatch;
	}
	// 824462EC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824462F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824462F4: 4BFE2C15  bl 0x82428f08
	ctx.lr = 0x824462F8;
	sub_82428F08(ctx, base);
	// 824462F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824462FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82446300: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82446304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446308: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8244630C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82446310: 4E800421  bctrl
	ctx.lr = 0x82446314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82446314: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82446318: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244631C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446320: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82446324: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82446328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244632C: 4E800421  bctrl
	ctx.lr = 0x82446330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82446330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244633C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82446340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446348 size=140
    let mut pc: u32 = 0x82446348;
    'dispatch: loop {
        match pc {
            0x82446348 => {
    //   block [0x82446348..0x824463D4)
	// 82446348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244634C: 480EED69  bl 0x825350b4
	ctx.lr = 0x82446350;
	sub_82535080(ctx, base);
	// 82446350: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446354: 83C3208C  lwz r30, 0x208c(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446358: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8244635C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82446360: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82446364: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82446368: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244636C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82446370: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446374: 4BFFFF35  bl 0x824462a8
	ctx.lr = 0x82446378;
	sub_824462A8(ctx, base);
	// 82446378: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244637C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82446380: 41980008  blt cr6, 0x82446388
	if ctx.cr[6].lt {
	pc = 0x82446388; continue 'dispatch;
	}
	// 82446384: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82446388: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8244638C: 616B9000  ori r11, r11, 0x9000
	ctx.r[11].u64 = ctx.r[11].u64 | 36864;
	// 82446390: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82446394: 41980008  blt cr6, 0x8244639c
	if ctx.cr[6].lt {
	pc = 0x8244639C; continue 'dispatch;
	}
	// 82446398: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8244639C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824463A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824463A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824463A8: 48003A91  bl 0x82449e38
	ctx.lr = 0x824463AC;
	sub_82449E38(ctx, base);
	// 824463AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824463B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824463B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824463B8: 4BFFFF11  bl 0x824462c8
	ctx.lr = 0x824463BC;
	sub_824462C8(ctx, base);
	// 824463BC: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824463C0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824463C4: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 824463C8: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824463CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824463D0: 480EED34  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824463D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824463D8 size=112
    let mut pc: u32 = 0x824463D8;
    'dispatch: loop {
        match pc {
            0x824463D8 => {
    //   block [0x824463D8..0x82446448)
	// 824463D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824463DC: 480EECDD  bl 0x825350b8
	ctx.lr = 0x824463E0;
	sub_82535080(ctx, base);
	// 824463E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824463E4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824463E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824463EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824463F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824463F4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824463F8: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824463FC: 40990040  ble cr6, 0x8244643c
	if !ctx.cr[6].gt {
	pc = 0x8244643C; continue 'dispatch;
	}
	// 82446400: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82446404: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 82446408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244640C: 4BFD409D  bl 0x8241a4a8
	ctx.lr = 0x82446410;
	sub_8241A4A8(ctx, base);
	// 82446410: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446414: 409A0020  bne cr6, 0x82446434
	if !ctx.cr[6].eq {
	pc = 0x82446434; continue 'dispatch;
	}
	// 82446418: 3BFF0012  addi r31, r31, 0x12
	ctx.r[31].s64 = ctx.r[31].s64 + 18;
	// 8244641C: 3BDE0012  addi r30, r30, 0x12
	ctx.r[30].s64 = ctx.r[30].s64 + 18;
	// 82446420: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82446424: 4198FFDC  blt cr6, 0x82446400
	if ctx.cr[6].lt {
	pc = 0x82446400; continue 'dispatch;
	}
	// 82446428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244642C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82446430: 480EECD8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82446434: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82446438: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244643C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82446444: 480EECC4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446448 size=12
    let mut pc: u32 = 0x82446448;
    'dispatch: loop {
        match pc {
            0x82446448 => {
    //   block [0x82446448..0x82446454)
	// 82446448: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 8244644C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446450: 4BFD4C48  b 0x8241b098
	sub_8241B098(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446458 size=200
    let mut pc: u32 = 0x82446458;
    'dispatch: loop {
        match pc {
            0x82446458 => {
    //   block [0x82446458..0x82446520)
	// 82446458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244645C: 480EEC4D  bl 0x825350a8
	ctx.lr = 0x82446460;
	sub_82535080(ctx, base);
	// 82446460: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446464: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82446468: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8244646C: 3B3D0024  addi r25, r29, 0x24
	ctx.r[25].s64 = ctx.r[29].s64 + 36;
	// 82446470: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82446474: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82446478: 7F9D2A14  add r28, r29, r5
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 8244647C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82446480: 7F1DC840  cmplw cr6, r29, r25
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82446484: 40980090  bge cr6, 0x82446514
	if !ctx.cr[6].lt {
	pc = 0x82446514; continue 'dispatch;
	}
	// 82446488: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8244648C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82446490: 40980084  bge cr6, 0x82446514
	if !ctx.cr[6].lt {
	pc = 0x82446514; continue 'dispatch;
	}
	// 82446494: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446498: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 8244649C: 4098001C  bge cr6, 0x824464b8
	if !ctx.cr[6].lt {
	pc = 0x824464B8; continue 'dispatch;
	}
	// 824464A0: 3BFF0012  addi r31, r31, 0x12
	ctx.r[31].s64 = ctx.r[31].s64 + 18;
	// 824464A4: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 824464A8: 4198FFEC  blt cr6, 0x82446494
	if ctx.cr[6].lt {
	pc = 0x82446494; continue 'dispatch;
	}
	// 824464AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824464B0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824464B4: 480EEC44  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 824464B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824464BC: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 824464C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824464C4: 4BFD3FE5  bl 0x8241a4a8
	ctx.lr = 0x824464C8;
	sub_8241A4A8(ctx, base);
	// 824464C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824464CC: 419A0014  beq cr6, 0x824464e0
	if ctx.cr[6].eq {
	pc = 0x824464E0; continue 'dispatch;
	}
	// 824464D0: 7F1BF840  cmplw cr6, r27, r31
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824464D4: 4098000C  bge cr6, 0x824464e0
	if !ctx.cr[6].lt {
	pc = 0x824464E0; continue 'dispatch;
	}
	// 824464D8: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 824464DC: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 824464E0: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 824464E4: 7F1EC840  cmplw cr6, r30, r25
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[25].u32, &mut ctx.xer);
	// 824464E8: 4198FFA0  blt cr6, 0x82446488
	if ctx.cr[6].lt {
	pc = 0x82446488; continue 'dispatch;
	}
	// 824464EC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824464F0: 409A0020  bne cr6, 0x82446510
	if !ctx.cr[6].eq {
	pc = 0x82446510; continue 'dispatch;
	}
	// 824464F4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824464F8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824464FC: 60840C0A  ori r4, r4, 0xc0a
	ctx.r[4].u64 = ctx.r[4].u64 | 3082;
	// 82446500: 48001409  bl 0x82447908
	ctx.lr = 0x82446504;
	sub_82447908(ctx, base);
	// 82446504: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446508: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8244650C: 480EEBEC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82446510: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82446514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446518: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8244651C: 480EEBDC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446520 size=52
    let mut pc: u32 = 0x82446520;
    'dispatch: loop {
        match pc {
            0x82446520 => {
    //   block [0x82446520..0x82446554)
	// 82446520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244652C: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446530: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446534: 4BFD42D5  bl 0x8241a808
	ctx.lr = 0x82446538;
	sub_8241A808(ctx, base);
	// 82446538: 3963FFFD  addi r11, r3, -3
	ctx.r[11].s64 = ctx.r[3].s64 + -3;
	// 8244653C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82446540: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82446544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244654C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446558 size=80
    let mut pc: u32 = 0x82446558;
    'dispatch: loop {
        match pc {
            0x82446558 => {
    //   block [0x82446558..0x824465A8)
	// 82446558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82446564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82446568: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244656C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82446570: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82446574: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82446578: 80832094  lwz r4, 0x2094(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8340 as u32) ) } as u64;
	// 8244657C: 48002515  bl 0x82448a90
	ctx.lr = 0x82446580;
	sub_82448A90(ctx, base);
	// 82446580: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446584: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82446588: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244658C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82446590: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82446594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244659C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824465A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824465A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824465A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824465A8 size=12
    let mut pc: u32 = 0x824465A8;
    'dispatch: loop {
        match pc {
            0x824465A8 => {
    //   block [0x824465A8..0x824465B4)
	// 824465A8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824465AC: 80832094  lwz r4, 0x2094(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8340 as u32) ) } as u64;
	// 824465B0: 480027F0  b 0x82448da0
	sub_82448DA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824465B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824465B8 size=176
    let mut pc: u32 = 0x824465B8;
    'dispatch: loop {
        match pc {
            0x824465B8 => {
    //   block [0x824465B8..0x82446668)
	// 824465B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824465BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824465C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824465C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824465C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824465CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824465D0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824465D4: 83DF208C  lwz r30, 0x208c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824465D8: 809F2094  lwz r4, 0x2094(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8340 as u32) ) } as u64;
	// 824465DC: 48001CE5  bl 0x824482c0
	ctx.lr = 0x824465E0;
	sub_824482C0(ctx, base);
	// 824465E0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824465E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824465E8: 419A0068  beq cr6, 0x82446650
	if ctx.cr[6].eq {
	pc = 0x82446650; continue 'dispatch;
	}
	// 824465EC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 824465F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824465F4: 480022C5  bl 0x824488b8
	ctx.lr = 0x824465F8;
	sub_824488B8(ctx, base);
	// 824465F8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824465FC: E87F09B8  ld r3, 0x9b8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2488 as u32) ) };
	// 82446600: 48001F49  bl 0x82448548
	ctx.lr = 0x82446604;
	sub_82448548(ctx, base);
	// 82446604: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82446608: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244660C: E87F09C0  ld r3, 0x9c0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2496 as u32) ) };
	// 82446610: F97F09B8  std r11, 0x9b8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2488 as u32), ctx.r[11].u64 ) };
	// 82446614: 48001F35  bl 0x82448548
	ctx.lr = 0x82446618;
	sub_82448548(ctx, base);
	// 82446618: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8244661C: F87F09C0  std r3, 0x9c0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2496 as u32), ctx.r[3].u64 ) };
	// 82446620: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82446624: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82446628: 48002291  bl 0x824488b8
	ctx.lr = 0x8244662C;
	sub_824488B8(ctx, base);
	// 8244662C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446630: E87F09D0  ld r3, 0x9d0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2512 as u32) ) };
	// 82446634: 48001F15  bl 0x82448548
	ctx.lr = 0x82446638;
	sub_82448548(ctx, base);
	// 82446638: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244663C: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82446640: E87F09D8  ld r3, 0x9d8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2520 as u32) ) };
	// 82446644: F97F09D0  std r11, 0x9d0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2512 as u32), ctx.r[11].u64 ) };
	// 82446648: 48001F01  bl 0x82448548
	ctx.lr = 0x8244664C;
	sub_82448548(ctx, base);
	// 8244664C: F87F09D8  std r3, 0x9d8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2520 as u32), ctx.r[3].u64 ) };
	// 82446650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82446654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244665C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82446660: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446668 size=8
    let mut pc: u32 = 0x82446668;
    'dispatch: loop {
        match pc {
            0x82446668 => {
    //   block [0x82446668..0x82446670)
	// 82446668: 80832094  lwz r4, 0x2094(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8340 as u32) ) } as u64;
	// 8244666C: 48001D94  b 0x82448400
	sub_82448400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446670 size=8
    let mut pc: u32 = 0x82446670;
    'dispatch: loop {
        match pc {
            0x82446670 => {
    //   block [0x82446670..0x82446678)
	// 82446670: 80832098  lwz r4, 0x2098(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8344 as u32) ) } as u64;
	// 82446674: 48001D8C  b 0x82448400
	sub_82448400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446678 size=12
    let mut pc: u32 = 0x82446678;
    'dispatch: loop {
        match pc {
            0x82446678 => {
    //   block [0x82446678..0x82446684)
	// 82446678: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8244667C: 80832098  lwz r4, 0x2098(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8344 as u32) ) } as u64;
	// 82446680: 48001D68  b 0x824483e8
	sub_824483E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446688 size=56
    let mut pc: u32 = 0x82446688;
    'dispatch: loop {
        match pc {
            0x82446688 => {
    //   block [0x82446688..0x824466C0)
	// 82446688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244668C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446694: 4BFD4175  bl 0x8241a808
	ctx.lr = 0x82446698;
	sub_8241A808(ctx, base);
	// 82446698: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244669C: 419A0010  beq cr6, 0x824466ac
	if ctx.cr[6].eq {
	pc = 0x824466AC; continue 'dispatch;
	}
	// 824466A0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824466A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824466A8: 409A0008  bne cr6, 0x824466b0
	if !ctx.cr[6].eq {
	pc = 0x824466B0; continue 'dispatch;
	}
	// 824466AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824466B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824466B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824466B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824466BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824466C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824466C0 size=88
    let mut pc: u32 = 0x824466C0;
    'dispatch: loop {
        match pc {
            0x824466C0 => {
    //   block [0x824466C0..0x82446718)
	// 824466C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824466C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824466C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824466CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824466D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824466D4: 83E3208C  lwz r31, 0x208c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824466D8: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 824466DC: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824466E0: 4BFF6029  bl 0x8243c708
	ctx.lr = 0x824466E4;
	sub_8243C708(ctx, base);
	// 824466E4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 824466E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824466EC: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 824466F0: 419A0010  beq cr6, 0x82446700
	if ctx.cr[6].eq {
	pc = 0x82446700; continue 'dispatch;
	}
	// 824466F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824466F8: 909F0044  stw r4, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[4].u32 ) };
	// 824466FC: 4BFD44BD  bl 0x8241abb8
	ctx.lr = 0x82446700;
	sub_8241ABB8(ctx, base);
	// 82446700: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244670C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82446710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446718 size=452
    let mut pc: u32 = 0x82446718;
    'dispatch: loop {
        match pc {
            0x82446718 => {
    //   block [0x82446718..0x824468DC)
	// 82446718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244671C: 480EE9A1  bl 0x825350bc
	ctx.lr = 0x82446720;
	sub_82535080(ctx, base);
	// 82446720: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446724: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 82446728: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8244672C: 394AC8A4  addi r10, r10, -0x375c
	ctx.r[10].s64 = ctx.r[10].s64 + -14172;
	// 82446730: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82446734: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82446738: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8244673C: 419A0188  beq cr6, 0x824468c4
	if ctx.cr[6].eq {
	pc = 0x824468C4; continue 'dispatch;
	}
	// 82446740: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82446744: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82446748: 419A017C  beq cr6, 0x824468c4
	if ctx.cr[6].eq {
	pc = 0x824468C4; continue 'dispatch;
	}
	// 8244674C: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 82446750: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82446754: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82446758: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244675C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82446760: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82446764: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82446768: 4200FFF0  bdnz 0x82446758
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82446758; continue 'dispatch;
	}
	// 8244676C: 3D208244  lis r9, -0x7dbc
	ctx.r[9].s64 = -2109472768;
	// 82446770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82446774: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82446778: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8244677C: 39296348  addi r9, r9, 0x6348
	ctx.r[9].s64 = ctx.r[9].s64 + 25416;
	// 82446780: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82446784: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446788: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244678C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82446790: 3BFD1088  addi r31, r29, 0x1088
	ctx.r[31].s64 = ctx.r[29].s64 + 4232;
	// 82446794: 910B0024  stw r8, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 82446798: 90EB0028  stw r7, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 8244679C: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 824467A0: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 824467A4: 914B0034  stw r10, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 824467A8: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 824467AC: 912B003C  stw r9, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 824467B0: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 824467B4: 910B0044  stw r8, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[8].u32 ) };
	// 824467B8: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 824467BC: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 824467C0: 4BFF5F49  bl 0x8243c708
	ctx.lr = 0x824467C4;
	sub_8243C708(ctx, base);
	// 824467C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824467C8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 824467CC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824467D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824467D4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824467D8: 3D60000F  lis r11, 0xf
	ctx.r[11].s64 = 983040;
	// 824467DC: 617E4240  ori r30, r11, 0x4240
	ctx.r[30].u64 = ctx.r[11].u64 | 16960;
	// 824467E0: FBC10058  std r30, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u64 ) };
	// 824467E4: 4BFF5F25  bl 0x8243c708
	ctx.lr = 0x824467E8;
	sub_8243C708(ctx, base);
	// 824467E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824467EC: FBC10068  std r30, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u64 ) };
	// 824467F0: 38800041  li r4, 0x41
	ctx.r[4].s64 = 65;
	// 824467F4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824467F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824467FC: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82446800: 4BFF5F09  bl 0x8243c708
	ctx.lr = 0x82446804;
	sub_8243C708(ctx, base);
	// 82446804: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82446808: FBC10078  std r30, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u64 ) };
	// 8244680C: 38800042  li r4, 0x42
	ctx.r[4].s64 = 66;
	// 82446810: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82446814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446818: F9610070  std r11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u64 ) };
	// 8244681C: 4BFF5EED  bl 0x8243c708
	ctx.lr = 0x82446820;
	sub_8243C708(ctx, base);
	// 82446820: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82446824: FBC10088  std r30, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[30].u64 ) };
	// 82446828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244682C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82446830: F9610080  std r11, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u64 ) };
	// 82446834: 4800B73D  bl 0x82451f70
	ctx.lr = 0x82446838;
	sub_82451F70(ctx, base);
	// 82446838: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 8244683C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446840: 4BFF5EC9  bl 0x8243c708
	ctx.lr = 0x82446844;
	sub_8243C708(ctx, base);
	// 82446844: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82446848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244684C: 4800B405  bl 0x82451c50
	ctx.lr = 0x82446850;
	sub_82451C50(ctx, base);
	// 82446850: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82446854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446858: 4800B401  bl 0x82451c58
	ctx.lr = 0x8244685C;
	sub_82451C58(ctx, base);
	// 8244685C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82446860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446864: 4800B40D  bl 0x82451c70
	ctx.lr = 0x82446868;
	sub_82451C70(ctx, base);
	// 82446868: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8244686C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446870: 4800B419  bl 0x82451c88
	ctx.lr = 0x82446874;
	sub_82451C88(ctx, base);
	// 82446874: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82446878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244687C: 4800B425  bl 0x82451ca0
	ctx.lr = 0x82446880;
	sub_82451CA0(ctx, base);
	// 82446880: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82446884: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446888: 4BFF5E81  bl 0x8243c708
	ctx.lr = 0x8244688C;
	sub_8243C708(ctx, base);
	// 8244688C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82446890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446894: 4800B425  bl 0x82451cb8
	ctx.lr = 0x82446898;
	sub_82451CB8(ctx, base);
	// 82446898: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8244689C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824468A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824468A4: 4800B465  bl 0x82451d08
	ctx.lr = 0x824468A8;
	sub_82451D08(ctx, base);
	// 824468A8: 3880003D  li r4, 0x3d
	ctx.r[4].s64 = 61;
	// 824468AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824468B0: 4BFF5E59  bl 0x8243c708
	ctx.lr = 0x824468B4;
	sub_8243C708(ctx, base);
	// 824468B4: 4800324D  bl 0x82449b00
	ctx.lr = 0x824468B8;
	sub_82449B00(ctx, base);
	// 824468B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824468BC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824468C0: 480EE84C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824468C4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824468C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824468CC: 60840C06  ori r4, r4, 0xc06
	ctx.r[4].u64 = ctx.r[4].u64 | 3078;
	// 824468D0: 48001039  bl 0x82447908
	ctx.lr = 0x824468D4;
	sub_82447908(ctx, base);
	// 824468D4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824468D8: 480EE834  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824468E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824468E0 size=16
    let mut pc: u32 = 0x824468E0;
    'dispatch: loop {
        match pc {
            0x824468E0 => {
    //   block [0x824468E0..0x824468F0)
	// 824468E0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824468E4: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 824468E8: 806B0208  lwz r3, 0x208(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(520 as u32) ) } as u64;
	// 824468EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824468F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824468F0 size=144
    let mut pc: u32 = 0x824468F0;
    'dispatch: loop {
        match pc {
            0x824468F0 => {
    //   block [0x824468F0..0x82446980)
	// 824468F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824468F4: 480EE7C5  bl 0x825350b8
	ctx.lr = 0x824468F8;
	sub_82535080(ctx, base);
	// 824468F8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824468FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82446900: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82446904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446908: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244690C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82446910: 4BFD3EF9  bl 0x8241a808
	ctx.lr = 0x82446914;
	sub_8241A808(ctx, base);
	// 82446914: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82446918: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8244691C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82446920: 906B06D8  stw r3, 0x6d8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1752 as u32), ctx.r[3].u32 ) };
	// 82446924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446928: 4BFD3F39  bl 0x8241a860
	ctx.lr = 0x8244692C;
	sub_8241A860(ctx, base);
	// 8244692C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446930: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82446934: F9610070  std r11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u64 ) };
	// 82446938: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244693C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82446940: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 82446944: 480030E5  bl 0x82449a28
	ctx.lr = 0x82446948;
	sub_82449A28(ctx, base);
	// 82446948: F8610060  std r3, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u64 ) };
	// 8244694C: 48003195  bl 0x82449ae0
	ctx.lr = 0x82446950;
	sub_82449AE0(ctx, base);
	// 82446950: F8610068  std r3, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u64 ) };
	// 82446954: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82446958: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8244695C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82446960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446964: 4800BBBD  bl 0x82452520
	ctx.lr = 0x82446968;
	sub_82452520(ctx, base);
	// 82446968: E9610080  ld r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	// 8244696C: E9410088  ld r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	// 82446970: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82446974: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82446978: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8244697C: 480EE78C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446980 size=36
    let mut pc: u32 = 0x82446980;
    'dispatch: loop {
        match pc {
            0x82446980 => {
    //   block [0x82446980..0x824469A4)
	// 82446980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244698C: 4BFD4A7D  bl 0x8241b408
	ctx.lr = 0x82446990;
	sub_8241B408(ctx, base);
	// 82446990: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244699C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824469A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824469A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824469A8 size=20
    let mut pc: u32 = 0x824469A8;
    'dispatch: loop {
        match pc {
            0x824469A8 => {
    //   block [0x824469A8..0x824469BC)
	// 824469A8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824469AC: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 824469B0: 906B0208  stw r3, 0x208(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(520 as u32), ctx.r[3].u32 ) };
	// 824469B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824469B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824469C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824469C0 size=44
    let mut pc: u32 = 0x824469C0;
    'dispatch: loop {
        match pc {
            0x824469C0 => {
    //   block [0x824469C0..0x824469EC)
	// 824469C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824469C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824469C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824469CC: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824469D0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824469D4: 4BFD3DFD  bl 0x8241a7d0
	ctx.lr = 0x824469D8;
	sub_8241A7D0(ctx, base);
	// 824469D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824469DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824469E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824469E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824469E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824469F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824469F0 size=72
    let mut pc: u32 = 0x824469F0;
    'dispatch: loop {
        match pc {
            0x824469F0 => {
    //   block [0x824469F0..0x82446A38)
	// 824469F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824469F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824469F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824469FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82446A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446A04: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82446A08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82446A0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82446A10: 4BFD4499  bl 0x8241aea8
	ctx.lr = 0x82446A14;
	sub_8241AEA8(ctx, base);
	// 82446A14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82446A18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446A1C: 4800B2AD  bl 0x82451cc8
	ctx.lr = 0x82446A20;
	sub_82451CC8(ctx, base);
	// 82446A20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446A2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82446A30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446A38 size=84
    let mut pc: u32 = 0x82446A38;
    'dispatch: loop {
        match pc {
            0x82446A38 => {
    //   block [0x82446A38..0x82446A8C)
	// 82446A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446A40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82446A44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82446A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446A4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82446A50: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82446A54: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82446A58: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82446A5C: 4BFF8465  bl 0x8243eec0
	ctx.lr = 0x82446A60;
	sub_8243EEC0(ctx, base);
	// 82446A60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446A64: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446A68: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82446A6C: 4BFFDB0D  bl 0x82444578
	ctx.lr = 0x82446A70;
	sub_82444578(ctx, base);
	// 82446A70: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82446A74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446A80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82446A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446A90 size=12
    let mut pc: u32 = 0x82446A90;
    'dispatch: loop {
        match pc {
            0x82446A90 => {
    //   block [0x82446A90..0x82446A9C)
	// 82446A90: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82446A94: 60840C03  ori r4, r4, 0xc03
	ctx.r[4].u64 = ctx.r[4].u64 | 3075;
	// 82446A98: 48000E70  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446AA0 size=20
    let mut pc: u32 = 0x82446AA0;
    'dispatch: loop {
        match pc {
            0x82446AA0 => {
    //   block [0x82446AA0..0x82446AB4)
	// 82446AA0: 81632658  lwz r11, 0x2658(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82446AA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82446AA8: 409A000C  bne cr6, 0x82446ab4
	if !ctx.cr[6].eq {
		sub_82446AB4(ctx, base);
		return;
	}
	// 82446AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446AB4 size=20
    let mut pc: u32 = 0x82446AB4;
    'dispatch: loop {
        match pc {
            0x82446AB4 => {
    //   block [0x82446AB4..0x82446AC8)
	// 82446AB4: 8143208C  lwz r10, 0x208c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446AB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446ABC: 814A0040  lwz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82446AC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82446AC4: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446AC8 size=8
    let mut pc: u32 = 0x82446AC8;
    'dispatch: loop {
        match pc {
            0x82446AC8 => {
    //   block [0x82446AC8..0x82446AD0)
	// 82446AC8: 386B0D0C  addi r3, r11, 0xd0c
	ctx.r[3].s64 = ctx.r[11].s64 + 3340;
	// 82446ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446AD0 size=124
    let mut pc: u32 = 0x82446AD0;
    'dispatch: loop {
        match pc {
            0x82446AD0 => {
    //   block [0x82446AD0..0x82446B4C)
	// 82446AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446AD4: 480EE5E9  bl 0x825350bc
	ctx.lr = 0x82446AD8;
	sub_82535080(ctx, base);
	// 82446AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446ADC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82446AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82446AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82446AE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82446AEC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82446AF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82446AF4: 83BF208C  lwz r29, 0x208c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446AF8: 4BFFFA61  bl 0x82446558
	ctx.lr = 0x82446AFC;
	sub_82446558(ctx, base);
	// 82446AFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446B00: 409A0044  bne cr6, 0x82446b44
	if !ctx.cr[6].eq {
	pc = 0x82446B44; continue 'dispatch;
	}
	// 82446B04: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446B08: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82446B0C: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82446B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446B14: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82446B18: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82446B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82446B20: 4E800421  bctrl
	ctx.lr = 0x82446B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82446B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446B28: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82446B2C: 4BFFFA7D  bl 0x824465a8
	ctx.lr = 0x82446B30;
	sub_824465A8(ctx, base);
	// 82446B30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446B34: 409A0010  bne cr6, 0x82446b44
	if !ctx.cr[6].eq {
	pc = 0x82446B44; continue 'dispatch;
	}
	// 82446B38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446B3C: 4BFFFA7D  bl 0x824465b8
	ctx.lr = 0x82446B40;
	sub_824465B8(ctx, base);
	// 82446B40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82446B48: 480EE5C4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446B50 size=84
    let mut pc: u32 = 0x82446B50;
    'dispatch: loop {
        match pc {
            0x82446B50 => {
    //   block [0x82446B50..0x82446BA4)
	// 82446B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446B58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446B5C: 4BFFFF45  bl 0x82446aa0
	ctx.lr = 0x82446B60;
	sub_82446AA0(ctx, base);
	// 82446B60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82446B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82446B68: 409A0018  bne cr6, 0x82446b80
	if !ctx.cr[6].eq {
	pc = 0x82446B80; continue 'dispatch;
	}
	// 82446B6C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82446B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446B7C: 4E800020  blr
	return;
	// 82446B80: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82446B84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446B88: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82446B8C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82446B90: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82446B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


