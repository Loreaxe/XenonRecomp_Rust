pub fn sub_82419470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419470 size=224
    let mut pc: u32 = 0x82419470;
    'dispatch: loop {
        match pc {
            0x82419470 => {
    //   block [0x82419470..0x82419550)
	// 82419470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419474: 4811BC41  bl 0x825350b4
	ctx.lr = 0x82419478;
	sub_82535080(ctx, base);
	// 82419478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241947C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419484: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82419488: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8241948C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82419490: 816BFF48  lwz r11, -0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-184 as u32) ) } as u64;
	// 82419494: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82419498: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241949C: 419A00A0  beq cr6, 0x8241953c
	if ctx.cr[6].eq {
	pc = 0x8241953C; continue 'dispatch;
	}
	// 824194A0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824194A4: 419A0098  beq cr6, 0x8241953c
	if ctx.cr[6].eq {
	pc = 0x8241953C; continue 'dispatch;
	}
	// 824194A8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824194AC: 40980010  bge cr6, 0x824194bc
	if !ctx.cr[6].lt {
	pc = 0x824194BC; continue 'dispatch;
	}
	// 824194B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824194B4: 386BF768  addi r3, r11, -0x898
	ctx.r[3].s64 = ctx.r[11].s64 + -2200;
	// 824194B8: 4800008C  b 0x82419544
	pc = 0x82419544; continue 'dispatch;
	// 824194BC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824194C0: 41990010  bgt cr6, 0x824194d0
	if ctx.cr[6].gt {
	pc = 0x824194D0; continue 'dispatch;
	}
	// 824194C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824194C8: 386BF73C  addi r3, r11, -0x8c4
	ctx.r[3].s64 = ctx.r[11].s64 + -2244;
	// 824194CC: 48000078  b 0x82419544
	pc = 0x82419544; continue 'dispatch;
	// 824194D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824194D4: 4BFFF45D  bl 0x82418930
	ctx.lr = 0x824194D8;
	sub_82418930(ctx, base);
	// 824194D8: 48015A79  bl 0x8242ef50
	ctx.lr = 0x824194DC;
	sub_8242EF50(ctx, base);
	// 824194DC: 815F01D0  lwz r10, 0x1d0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(464 as u32) ) } as u64;
	// 824194E0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824194E4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824194E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824194EC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824194F0: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 824194F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824194F8: 4082FFEC  bne 0x824194e4
	if !ctx.cr[0].eq {
	pc = 0x824194E4; continue 'dispatch;
	}
	// 824194FC: 809F0090  lwz r4, 0x90(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82419500: 817F01D0  lwz r11, 0x1d0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(464 as u32) ) } as u64;
	// 82419504: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82419508: 937F01D8  stw r27, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[27].u32 ) };
	// 8241950C: 93BF01DC  stw r29, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[29].u32 ) };
	// 82419510: 939F01E0  stw r28, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[28].u32 ) };
	// 82419514: 917F01D4  stw r11, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[11].u32 ) };
	// 82419518: 909F0094  stw r4, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[4].u32 ) };
	// 8241951C: 4800A285  bl 0x824237a0
	ctx.lr = 0x82419520;
	sub_824237A0(ctx, base);
	// 82419520: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82419524: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82419528: 997F01B6  stb r11, 0x1b6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(438 as u32), ctx.r[11].u8 ) };
	// 8241952C: 915F01E4  stw r10, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[10].u32 ) };
	// 82419530: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82419534: 48015A1D  bl 0x8242ef50
	ctx.lr = 0x82419538;
	sub_8242EF50(ctx, base);
	// 82419538: 48000010  b 0x82419548
	pc = 0x82419548; continue 'dispatch;
	// 8241953C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419540: 386BF714  addi r3, r11, -0x8ec
	ctx.r[3].s64 = ctx.r[11].s64 + -2284;
	// 82419544: 4800B195  bl 0x824246d8
	ctx.lr = 0x82419548;
	sub_824246D8(ctx, base);
	// 82419548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241954C: 4811BBB8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419550 size=56
    let mut pc: u32 = 0x82419550;
    'dispatch: loop {
        match pc {
            0x82419550 => {
    //   block [0x82419550..0x82419588)
	// 82419550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82419558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241955C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419564: 4800A0D5  bl 0x82423638
	ctx.lr = 0x82419568;
	sub_82423638(ctx, base);
	// 82419568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241956C: 4BFFF3C5  bl 0x82418930
	ctx.lr = 0x82419570;
	sub_82418930(ctx, base);
	// 82419570: 480159D9  bl 0x8242ef48
	ctx.lr = 0x82419574;
	sub_8242EF48(ctx, base);
	// 82419574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82419578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241957C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82419580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82419584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419588 size=148
    let mut pc: u32 = 0x82419588;
    'dispatch: loop {
        match pc {
            0x82419588 => {
    //   block [0x82419588..0x8241961C)
	// 82419588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241958C: 4811BB2D  bl 0x825350b8
	ctx.lr = 0x82419590;
	sub_82535080(ctx, base);
	// 82419590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419598: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8241959C: 4800A09D  bl 0x82423638
	ctx.lr = 0x824195A0;
	sub_82423638(ctx, base);
	// 824195A0: 480159B1  bl 0x8242ef50
	ctx.lr = 0x824195A4;
	sub_8242EF50(ctx, base);
	// 824195A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824195A8: 409A0014  bne cr6, 0x824195bc
	if !ctx.cr[6].eq {
	pc = 0x824195BC; continue 'dispatch;
	}
	// 824195AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824195B0: 386BF658  addi r3, r11, -0x9a8
	ctx.r[3].s64 = ctx.r[11].s64 + -2472;
	// 824195B4: 4800B125  bl 0x824246d8
	ctx.lr = 0x824195B8;
	sub_824246D8(ctx, base);
	// 824195B8: 48000054  b 0x8241960c
	pc = 0x8241960C; continue 'dispatch;
	// 824195BC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824195C0: 939F01AC  stw r28, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[28].u32 ) };
	// 824195C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824195C8: 419A0044  beq cr6, 0x8241960c
	if ctx.cr[6].eq {
	pc = 0x8241960C; continue 'dispatch;
	}
	// 824195CC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824195D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824195D4: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824195D8: 40810034  ble 0x8241960c
	if !ctx.cr[0].gt {
	pc = 0x8241960C; continue 'dispatch;
	}
	// 824195DC: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 824195E0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824195E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824195E8: 4182000C  beq 0x824195f4
	if ctx.cr[0].eq {
	pc = 0x824195F4; continue 'dispatch;
	}
	// 824195EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824195F0: 480018B9  bl 0x8241aea8
	ctx.lr = 0x824195F4;
	sub_8241AEA8(ctx, base);
	// 824195F4: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824195F8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824195FC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82419600: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419604: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419608: 4198FFD8  blt cr6, 0x824195e0
	if ctx.cr[6].lt {
	pc = 0x824195E0; continue 'dispatch;
	}
	// 8241960C: 48015945  bl 0x8242ef50
	ctx.lr = 0x82419610;
	sub_8242EF50(ctx, base);
	// 82419610: 48015939  bl 0x8242ef48
	ctx.lr = 0x82419614;
	sub_8242EF48(ctx, base);
	// 82419614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82419618: 4811BAF0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419620 size=136
    let mut pc: u32 = 0x82419620;
    'dispatch: loop {
        match pc {
            0x82419620 => {
    //   block [0x82419620..0x824196A8)
	// 82419620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419624: 4811BA99  bl 0x825350bc
	ctx.lr = 0x82419628;
	sub_82535080(ctx, base);
	// 82419628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241962C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82419630: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82419634: 4800A005  bl 0x82423638
	ctx.lr = 0x82419638;
	sub_82423638(ctx, base);
	// 82419638: 48015919  bl 0x8242ef50
	ctx.lr = 0x8241963C;
	sub_8242EF50(ctx, base);
	// 8241963C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82419640: 409A0014  bne cr6, 0x82419654
	if !ctx.cr[6].eq {
	pc = 0x82419654; continue 'dispatch;
	}
	// 82419644: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419648: 386BF6C4  addi r3, r11, -0x93c
	ctx.r[3].s64 = ctx.r[11].s64 + -2364;
	// 8241964C: 4800B08D  bl 0x824246d8
	ctx.lr = 0x82419650;
	sub_824246D8(ctx, base);
	// 82419650: 48000048  b 0x82419698
	pc = 0x82419698; continue 'dispatch;
	// 82419654: 897E0003  lbz r11, 3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419658: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241965C: 93FE01EC  stw r31, 0x1ec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(492 as u32), ctx.r[31].u32 ) };
	// 82419660: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82419664: 40810034  ble 0x82419698
	if !ctx.cr[0].gt {
	pc = 0x82419698; continue 'dispatch;
	}
	// 82419668: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 8241966C: 817F01E0  lwz r11, 0x1e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(480 as u32) ) } as u64;
	// 82419670: 815E01EC  lwz r10, 0x1ec(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(492 as u32) ) } as u64;
	// 82419674: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419678: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241967C: 48001425  bl 0x8241aaa0
	ctx.lr = 0x82419680;
	sub_8241AAA0(ctx, base);
	// 82419680: 897E0003  lbz r11, 3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419684: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82419688: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241968C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82419690: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419694: 4198FFD8  blt cr6, 0x8241966c
	if ctx.cr[6].lt {
	pc = 0x8241966C; continue 'dispatch;
	}
	// 82419698: 480158B9  bl 0x8242ef50
	ctx.lr = 0x8241969C;
	sub_8242EF50(ctx, base);
	// 8241969C: 480158AD  bl 0x8242ef48
	ctx.lr = 0x824196A0;
	sub_8242EF48(ctx, base);
	// 824196A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824196A4: 4811BA68  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824196A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824196A8 size=196
    let mut pc: u32 = 0x824196A8;
    'dispatch: loop {
        match pc {
            0x824196A8 => {
    //   block [0x824196A8..0x8241976C)
	// 824196A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824196AC: 4811BA11  bl 0x825350bc
	ctx.lr = 0x824196B0;
	sub_82535080(ctx, base);
	// 824196B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824196B4: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 824196B8: 816AF370  lwz r11, -0xc90(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-3216 as u32) ) } as u64;
	// 824196BC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824196C0: 916AF370  stw r11, -0xc90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3216 as u32), ctx.r[11].u32 ) };
	// 824196C4: 408200A0  bne 0x82419764
	if !ctx.cr[0].eq {
	pc = 0x82419764; continue 'dispatch;
	}
	// 824196C8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824196CC: 3BCBF820  addi r30, r11, -0x7e0
	ctx.r[30].s64 = ctx.r[11].s64 + -2016;
	// 824196D0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 824196D4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824196D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824196DC: 409A000C  bne cr6, 0x824196e8
	if !ctx.cr[6].eq {
	pc = 0x824196E8; continue 'dispatch;
	}
	// 824196E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824196E4: 4BFFFC3D  bl 0x82419320
	ctx.lr = 0x824196E8;
	sub_82419320(ctx, base);
	// 824196E8: 3BFF02A0  addi r31, r31, 0x2a0
	ctx.r[31].s64 = ctx.r[31].s64 + 672;
	// 824196EC: 397E0A80  addi r11, r30, 0xa80
	ctx.r[11].s64 = ctx.r[30].s64 + 2688;
	// 824196F0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824196F4: 4198FFE0  blt cr6, 0x824196d4
	if ctx.cr[6].lt {
	pc = 0x824196D4; continue 'dispatch;
	}
	// 824196F8: 38A00A80  li r5, 0xa80
	ctx.r[5].s64 = 2688;
	// 824196FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82419700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82419704: 4811BACD  bl 0x825351d0
	ctx.lr = 0x82419708;
	sub_825351D0(ctx, base);
	// 82419708: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241970C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82419710: 4800C7A1  bl 0x82425eb0
	ctx.lr = 0x82419714;
	sub_82425EB0(ctx, base);
	// 82419714: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82419718: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8241971C: 808BF374  lwz r4, -0xc8c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3212 as u32) ) } as u64;
	// 82419720: 4800C791  bl 0x82425eb0
	ctx.lr = 0x82419724;
	sub_82425EB0(ctx, base);
	// 82419724: 48009F7D  bl 0x824236a0
	ctx.lr = 0x82419728;
	sub_824236A0(ctx, base);
	// 82419728: 48002739  bl 0x8241be60
	ctx.lr = 0x8241972C;
	sub_8241BE60(ctx, base);
	// 8241972C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419730: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82419734: 3BABF7BC  addi r29, r11, -0x844
	ctx.r[29].s64 = ctx.r[11].s64 + -2116;
	// 82419738: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241973C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82419740: 419A0014  beq cr6, 0x82419754
	if ctx.cr[6].eq {
	pc = 0x82419754; continue 'dispatch;
	}
	// 82419744: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82419748: 4800C0D9  bl 0x82425820
	ctx.lr = 0x8241974C;
	sub_82425820(ctx, base);
	// 8241974C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419750: 4BFFFBD1  bl 0x82419320
	ctx.lr = 0x82419754;
	sub_82419320(ctx, base);
	// 82419754: 3BFF02A0  addi r31, r31, 0x2a0
	ctx.r[31].s64 = ctx.r[31].s64 + 672;
	// 82419758: 397E0A80  addi r11, r30, 0xa80
	ctx.r[11].s64 = ctx.r[30].s64 + 2688;
	// 8241975C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419760: 4198FFD8  blt cr6, 0x82419738
	if ctx.cr[6].lt {
	pc = 0x82419738; continue 'dispatch;
	}
	// 82419764: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82419768: 4811B9A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82419770 size=760
    let mut pc: u32 = 0x82419770;
    'dispatch: loop {
        match pc {
            0x82419770 => {
    //   block [0x82419770..0x82419A68)
	// 82419770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419774: 4811B925  bl 0x82535098
	ctx.lr = 0x82419778;
	sub_82535080(ctx, base);
	// 82419778: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241977C: 3964003F  addi r11, r4, 0x3f
	ctx.r[11].s64 = ctx.r[4].s64 + 63;
	// 82419780: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82419784: 55780032  rlwinm r24, r11, 0, 0, 0x19
	ctx.r[24].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82419788: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8241978C: 7D782050  subf r11, r24, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[24].s64;
	// 82419790: 7EAB2A14  add r21, r11, r5
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82419794: 409A0018  bne cr6, 0x824197ac
	if !ctx.cr[6].eq {
	pc = 0x824197AC; continue 'dispatch;
	}
	// 82419798: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241979C: 386BF7E8  addi r3, r11, -0x818
	ctx.r[3].s64 = ctx.r[11].s64 + -2072;
	// 824197A0: 4800AF39  bl 0x824246d8
	ctx.lr = 0x824197A4;
	sub_824246D8(ctx, base);
	// 824197A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824197A8: 480002B8  b 0x82419a60
	pc = 0x82419A60; continue 'dispatch;
	// 824197AC: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824197B0: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 824197B4: 394BF820  addi r10, r11, -0x7e0
	ctx.r[10].s64 = ctx.r[11].s64 + -2016;
	// 824197B8: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 824197BC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 824197C0: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824197C4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824197C8: 419A0018  beq cr6, 0x824197e0
	if ctx.cr[6].eq {
	pc = 0x824197E0; continue 'dispatch;
	}
	// 824197CC: 392902A0  addi r9, r9, 0x2a0
	ctx.r[9].s64 = ctx.r[9].s64 + 672;
	// 824197D0: 390A0A80  addi r8, r10, 0xa80
	ctx.r[8].s64 = ctx.r[10].s64 + 2688;
	// 824197D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824197D8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824197DC: 4198FFE4  blt cr6, 0x824197c0
	if ctx.cr[6].lt {
	pc = 0x824197C0; continue 'dispatch;
	}
	// 824197E0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824197E4: 419AFFC0  beq cr6, 0x824197a4
	if ctx.cr[6].eq {
	pc = 0x824197A4; continue 'dispatch;
	}
	// 824197E8: 1D6B02A0  mulli r11, r11, 0x2a0
	ctx.r[11].s64 = ctx.r[11].s64 * 672;
	// 824197EC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824197F0: 38A002A0  li r5, 0x2a0
	ctx.r[5].s64 = 672;
	// 824197F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824197F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824197FC: 4811B9D5  bl 0x825351d0
	ctx.lr = 0x82419800;
	sub_825351D0(ctx, base);
	// 82419800: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419804: 3A802000  li r20, 0x2000
	ctx.r[20].s64 = 8192;
	// 82419808: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8241980C: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82419810: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419814: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82419818: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241981C: 929F01A4  stw r20, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[20].u32 ) };
	// 82419820: 915F01A8  stw r10, 0x1a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[10].u32 ) };
	// 82419824: 917F0270  stw r11, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[11].u32 ) };
	// 82419828: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241982C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82419830: 409A0010  bne cr6, 0x82419840
	if !ctx.cr[6].eq {
	pc = 0x82419840; continue 'dispatch;
	}
	// 82419834: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419838: 1F2B4100  mulli r25, r11, 0x4100
	ctx.r[25].s64 = ctx.r[11].s64 * 16640;
	// 8241983C: 4800001C  b 0x82419858
	pc = 0x82419858; continue 'dispatch;
	// 82419840: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419844: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419848: 1D6B40C0  mulli r11, r11, 0x40c0
	ctx.r[11].s64 = ctx.r[11].s64 * 16576;
	// 8241984C: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82419850: 556B0032  rlwinm r11, r11, 0, 0, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82419854: 7F2B51D6  mullw r25, r11, r10
	ctx.r[25].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82419858: 3978003F  addi r11, r24, 0x3f
	ctx.r[11].s64 = ctx.r[24].s64 + 63;
	// 8241985C: 815F0270  lwz r10, 0x270(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82419860: 557C0032  rlwinm r28, r11, 0, 0, 0x19
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82419864: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82419868: 7D7CC850  subf r11, r28, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[28].s64;
	// 8241986C: 7F4BC214  add r26, r11, r24
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82419870: 409A000C  bne cr6, 0x8241987c
	if !ctx.cr[6].eq {
	pc = 0x8241987C; continue 'dispatch;
	}
	// 82419874: 3BA04100  li r29, 0x4100
	ctx.r[29].s64 = 16640;
	// 82419878: 48000018  b 0x82419890
	pc = 0x82419890; continue 'dispatch;
	// 8241987C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82419880: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419884: 1D6B40C0  mulli r11, r11, 0x40c0
	ctx.r[11].s64 = ctx.r[11].s64 * 16576;
	// 82419888: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 8241988C: 557D0032  rlwinm r29, r11, 0, 0, 0x19
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82419890: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419894: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 82419898: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241989C: 40810068  ble 0x82419904
	if !ctx.cr[0].gt {
	pc = 0x82419904; continue 'dispatch;
	}
	// 824198A0: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 824198A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824198A8: 7F5DD051  subf. r26, r29, r26
	ctx.r[26].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 824198AC: 7F9DE214  add r28, r29, r28
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 824198B0: 41800150  blt 0x82419a00
	if ctx.cr[0].lt {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 824198B4: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 824198B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824198BC: 409A0010  bne cr6, 0x824198cc
	if !ctx.cr[6].eq {
	pc = 0x824198CC; continue 'dispatch;
	}
	// 824198C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824198C4: 48001E95  bl 0x8241b758
	ctx.lr = 0x824198C8;
	sub_8241B758(ctx, base);
	// 824198C8: 48000018  b 0x824198e0
	pc = 0x824198E0; continue 'dispatch;
	// 824198CC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 824198D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824198D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824198D8: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 824198DC: 48001E3D  bl 0x8241b718
	ctx.lr = 0x824198E0;
	sub_8241B718(ctx, base);
	// 824198E0: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824198E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824198E8: 41820118  beq 0x82419a00
	if ctx.cr[0].eq {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 824198EC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824198F0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824198F4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824198F8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824198FC: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419900: 4198FFA4  blt cr6, 0x824198a4
	if ctx.cr[6].lt {
	pc = 0x824198A4; continue 'dispatch;
	}
	// 82419904: 7FB9A851  subf. r29, r25, r21
	ctx.r[29].s64 = ctx.r[21].s64 - ctx.r[25].s64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82419908: 7FD9C214  add r30, r25, r24
	ctx.r[30].u64 = ctx.r[25].u64 + ctx.r[24].u64;
	// 8241990C: 418000F4  blt 0x82419a00
	if ctx.cr[0].lt {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 82419910: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419914: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 82419918: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241991C: 40810054  ble 0x82419970
	if !ctx.cr[0].gt {
	pc = 0x82419970; continue 'dispatch;
	}
	// 82419920: 3B9F0124  addi r28, r31, 0x124
	ctx.r[28].s64 = ctx.r[31].s64 + 292;
	// 82419924: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82419928: 809F01A4  lwz r4, 0x1a4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 8241992C: 80BF01A8  lwz r5, 0x1a8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(424 as u32) ) } as u64;
	// 82419930: 7D44E850  subf r10, r4, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[4].s64;
	// 82419934: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 82419938: 7FA55051  subf. r29, r5, r10
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8241993C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82419940: 418000C0  blt 0x82419a00
	if ctx.cr[0].lt {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 82419944: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419948: 48009B01  bl 0x82423448
	ctx.lr = 0x8241994C;
	sub_82423448(ctx, base);
	// 8241994C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82419950: 907CFF74  stw r3, -0x8c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-140 as u32), ctx.r[3].u32 ) };
	// 82419954: 418200AC  beq 0x82419a00
	if ctx.cr[0].eq {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 82419958: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241995C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82419960: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82419964: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419968: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241996C: 4198FFB8  blt cr6, 0x82419924
	if ctx.cr[6].lt {
	pc = 0x82419924; continue 'dispatch;
	}
	// 82419970: 397DE000  addi r11, r29, -0x2000
	ctx.r[11].s64 = ctx.r[29].s64 + -8192;
	// 82419974: 929F0120  stw r20, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[20].u32 ) };
	// 82419978: 93DF0118  stw r30, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u32 ) };
	// 8241997C: 394BFF00  addi r10, r11, -0x100
	ctx.r[10].s64 = ctx.r[11].s64 + -256;
	// 82419980: 7D4A5E70  srawi r10, r10, 0xb
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 11) as i64;
	// 82419984: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82419988: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241998C: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82419990: 915F011C  stw r10, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[10].u32 ) };
	// 82419994: 4180006C  blt 0x82419a00
	if ctx.cr[0].lt {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 82419998: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8241999C: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 824199A0: 7D64F214  add r11, r4, r30
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[30].u64;
	// 824199A4: 396B2000  addi r11, r11, 0x2000
	ctx.r[11].s64 = ctx.r[11].s64 + 8192;
	// 824199A8: 917F01D0  stw r11, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[11].u32 ) };
	// 824199AC: 41980054  blt cr6, 0x82419a00
	if ctx.cr[6].lt {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 824199B0: 38A02000  li r5, 0x2000
	ctx.r[5].s64 = 8192;
	// 824199B4: 92FF0094  stw r23, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[23].u32 ) };
	// 824199B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824199BC: 48009A8D  bl 0x82423448
	ctx.lr = 0x824199C0;
	sub_82423448(ctx, base);
	// 824199C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824199C4: 907F0090  stw r3, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 824199C8: 41820038  beq 0x82419a00
	if ctx.cr[0].eq {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 824199CC: 38BF0098  addi r5, r31, 0x98
	ctx.r[5].s64 = ctx.r[31].s64 + 152;
	// 824199D0: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 824199D4: 80760000  lwz r3, 0(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 824199D8: 4800A511  bl 0x82423ee8
	ctx.lr = 0x824199DC;
	sub_82423EE8(ctx, base);
	// 824199DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824199E0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 824199E4: 4182001C  beq 0x82419a00
	if ctx.cr[0].eq {
	pc = 0x82419A00; continue 'dispatch;
	}
	// 824199E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824199EC: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 824199F0: 480087C1  bl 0x824221b0
	ctx.lr = 0x824199F4;
	sub_824221B0(ctx, base);
	// 824199F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824199F8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 824199FC: 40820010  bne 0x82419a0c
	if !ctx.cr[0].eq {
	pc = 0x82419A0C; continue 'dispatch;
	}
	// 82419A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419A04: 4BFFF91D  bl 0x82419320
	ctx.lr = 0x82419A08;
	sub_82419320(ctx, base);
	// 82419A08: 4BFFFD9C  b 0x824197a4
	pc = 0x824197A4; continue 'dispatch;
	// 82419A0C: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 82419A10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82419A14: 388B85E0  addi r4, r11, -0x7a20
	ctx.r[4].s64 = ctx.r[11].s64 + -31264;
	// 82419A18: 48007F51  bl 0x82421968
	ctx.lr = 0x82419A1C;
	sub_82421968(ctx, base);
	// 82419A1C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82419A20: 92FF01AC  stw r23, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[23].u32 ) };
	// 82419A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419A28: 9AFF01B4  stb r23, 0x1b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[23].u8 ) };
	// 82419A2C: 92FF01B8  stw r23, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[23].u32 ) };
	// 82419A30: 92FF01C4  stw r23, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[23].u32 ) };
	// 82419A34: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82419A38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82419A3C: D01F01E8  stfs f0, 0x1e8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(488 as u32), tmp.u32 ) };
	// 82419A40: 92FF01C0  stw r23, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[23].u32 ) };
	// 82419A44: 92FF01C8  stw r23, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[23].u32 ) };
	// 82419A48: 92FF0274  stw r23, 0x274(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), ctx.r[23].u32 ) };
	// 82419A4C: 92FF0278  stw r23, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[23].u32 ) };
	// 82419A50: 997F01B5  stb r11, 0x1b5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(437 as u32), ctx.r[11].u8 ) };
	// 82419A54: 917F01BC  stw r11, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[11].u32 ) };
	// 82419A58: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 82419A5C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82419A60: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82419A64: 4811B684  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419A68 size=68
    let mut pc: u32 = 0x82419A68;
    'dispatch: loop {
        match pc {
            0x82419A68 => {
    //   block [0x82419A68..0x82419AAC)
	// 82419A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419A6C: 4811B651  bl 0x825350bc
	ctx.lr = 0x82419A70;
	sub_82535080(ctx, base);
	// 82419A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419A74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419A78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82419A7C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82419A80: 48009BB9  bl 0x82423638
	ctx.lr = 0x82419A84;
	sub_82423638(ctx, base);
	// 82419A84: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 82419A88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82419A8C: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 82419A90: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82419A94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82419A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419A9C: 4BFFF9D5  bl 0x82419470
	ctx.lr = 0x82419AA0;
	sub_82419470(ctx, base);
	// 82419AA0: 480154A9  bl 0x8242ef48
	ctx.lr = 0x82419AA4;
	sub_8242EF48(ctx, base);
	// 82419AA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82419AA8: 4811B664  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419AB0 size=192
    let mut pc: u32 = 0x82419AB0;
    'dispatch: loop {
        match pc {
            0x82419AB0 => {
    //   block [0x82419AB0..0x82419B70)
	// 82419AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419AB4: 4811B609  bl 0x825350bc
	ctx.lr = 0x82419AB8;
	sub_82535080(ctx, base);
	// 82419AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419AC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82419AC4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82419AC8: 48009B71  bl 0x82423638
	ctx.lr = 0x82419ACC;
	sub_82423638(ctx, base);
	// 82419ACC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82419AD0: 409A0014  bne cr6, 0x82419ae4
	if !ctx.cr[6].eq {
	pc = 0x82419AE4; continue 'dispatch;
	}
	// 82419AD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419AD8: 386BF794  addi r3, r11, -0x86c
	ctx.r[3].s64 = ctx.r[11].s64 + -2156;
	// 82419ADC: 4800ABFD  bl 0x824246d8
	ctx.lr = 0x82419AE0;
	sub_824246D8(ctx, base);
	// 82419AE0: 48000084  b 0x82419b64
	pc = 0x82419B64; continue 'dispatch;
	// 82419AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419AE8: 4BFFEE49  bl 0x82418930
	ctx.lr = 0x82419AEC;
	sub_82418930(ctx, base);
	// 82419AEC: 48015465  bl 0x8242ef50
	ctx.lr = 0x82419AF0;
	sub_8242EF50(ctx, base);
	// 82419AF0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82419AF4: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82419AF8: 38ABF270  addi r5, r11, -0xd90
	ctx.r[5].s64 = ctx.r[11].s64 + -3472;
	// 82419AFC: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82419B00: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82419B04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82419B08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82419B0C: 4BFFE39D  bl 0x82417ea8
	ctx.lr = 0x82419B10;
	sub_82417EA8(ctx, base);
	// 82419B10: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82419B14: 4082004C  bne 0x82419b60
	if !ctx.cr[0].eq {
	pc = 0x82419B60; continue 'dispatch;
	}
	// 82419B18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82419B1C: 4BFFDC5D  bl 0x82417778
	ctx.lr = 0x82419B20;
	sub_82417778(ctx, base);
	// 82419B20: 809F0090  lwz r4, 0x90(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82419B24: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82419B28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82419B2C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82419B30: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82419B34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82419B38: 909F0094  stw r4, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[4].u32 ) };
	// 82419B3C: 917F01D4  stw r11, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[11].u32 ) };
	// 82419B40: 915F01D8  stw r10, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[10].u32 ) };
	// 82419B44: 913F01DC  stw r9, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[9].u32 ) };
	// 82419B48: 911F01E0  stw r8, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[8].u32 ) };
	// 82419B4C: 48009C55  bl 0x824237a0
	ctx.lr = 0x82419B50;
	sub_824237A0(ctx, base);
	// 82419B50: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82419B54: 917F01E4  stw r11, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[11].u32 ) };
	// 82419B58: 997F01B6  stb r11, 0x1b6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(438 as u32), ctx.r[11].u8 ) };
	// 82419B5C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82419B60: 480153F1  bl 0x8242ef50
	ctx.lr = 0x82419B64;
	sub_8242EF50(ctx, base);
	// 82419B64: 480153E5  bl 0x8242ef48
	ctx.lr = 0x82419B68;
	sub_8242EF48(ctx, base);
	// 82419B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82419B6C: 4811B5A0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419B70 size=180
    let mut pc: u32 = 0x82419B70;
    'dispatch: loop {
        match pc {
            0x82419B70 => {
    //   block [0x82419B70..0x82419C24)
	// 82419B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419B74: 4811B545  bl 0x825350b8
	ctx.lr = 0x82419B78;
	sub_82535080(ctx, base);
	// 82419B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419B7C: 48009ABD  bl 0x82423638
	ctx.lr = 0x82419B80;
	sub_82423638(ctx, base);
	// 82419B80: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82419B84: 3BCBF38C  addi r30, r11, -0xc74
	ctx.r[30].s64 = ctx.r[11].s64 + -3188;
	// 82419B88: 817EFFF4  lwz r11, -0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82419B8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82419B90: 419A0018  beq cr6, 0x82419ba8
	if ctx.cr[6].eq {
	pc = 0x82419BA8; continue 'dispatch;
	}
	// 82419B94: 397EFFF4  addi r11, r30, -0xc
	ctx.r[11].s64 = ctx.r[30].s64 + -12;
	// 82419B98: 807EFFF8  lwz r3, -8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82419B9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419BA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82419BA4: 4E800421  bctrl
	ctx.lr = 0x82419BA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82419BA8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82419BAC: 3B8BF820  addi r28, r11, -0x7e0
	ctx.r[28].s64 = ctx.r[11].s64 + -2016;
	// 82419BB0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82419BB4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82419BB8: 3BABF378  addi r29, r11, -0xc88
	ctx.r[29].s64 = ctx.r[11].s64 + -3208;
	// 82419BBC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419BC0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82419BC4: 409A0024  bne cr6, 0x82419be8
	if !ctx.cr[6].eq {
	pc = 0x82419BE8; continue 'dispatch;
	}
	// 82419BC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82419BCC: 4800C18D  bl 0x82425d58
	ctx.lr = 0x82419BD0;
	sub_82425D58(ctx, base);
	// 82419BD0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82419BD4: 41820014  beq 0x82419be8
	if ctx.cr[0].eq {
	pc = 0x82419BE8; continue 'dispatch;
	}
	// 82419BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419BDC: 4BFFF0CD  bl 0x82418ca8
	ctx.lr = 0x82419BE0;
	sub_82418CA8(ctx, base);
	// 82419BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82419BE4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82419BE8: 3BFF02A0  addi r31, r31, 0x2a0
	ctx.r[31].s64 = ctx.r[31].s64 + 672;
	// 82419BEC: 397C0A80  addi r11, r28, 0xa80
	ctx.r[11].s64 = ctx.r[28].s64 + 2688;
	// 82419BF0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419BF4: 4198FFC8  blt cr6, 0x82419bbc
	if ctx.cr[6].lt {
	pc = 0x82419BBC; continue 'dispatch;
	}
	// 82419BF8: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82419BFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82419C00: 419A0018  beq cr6, 0x82419c18
	if ctx.cr[6].eq {
	pc = 0x82419C18; continue 'dispatch;
	}
	// 82419C04: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 82419C08: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419C0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419C10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82419C14: 4E800421  bctrl
	ctx.lr = 0x82419C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82419C18: 48015331  bl 0x8242ef48
	ctx.lr = 0x82419C1C;
	sub_8242EF48(ctx, base);
	// 82419C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82419C20: 4811B4E8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419C28 size=48
    let mut pc: u32 = 0x82419C28;
    'dispatch: loop {
        match pc {
            0x82419C28 => {
    //   block [0x82419C28..0x82419C58)
	// 82419C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82419C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419C34: 48009A05  bl 0x82423638
	ctx.lr = 0x82419C38;
	sub_82423638(ctx, base);
	// 82419C38: 4BFFFF39  bl 0x82419b70
	ctx.lr = 0x82419C3C;
	sub_82419B70(ctx, base);
	// 82419C3C: 4800AA0D  bl 0x82424648
	ctx.lr = 0x82419C40;
	sub_82424648(ctx, base);
	// 82419C40: 48015309  bl 0x8242ef48
	ctx.lr = 0x82419C44;
	sub_8242EF48(ctx, base);
	// 82419C44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82419C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82419C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82419C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82419C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419C58 size=168
    let mut pc: u32 = 0x82419C58;
    'dispatch: loop {
        match pc {
            0x82419C58 => {
    //   block [0x82419C58..0x82419D00)
	// 82419C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82419C60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82419C64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419C68: 3FE08289  lis r31, -0x7d77
	ctx.r[31].s64 = -2104950784;
	// 82419C6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82419C70: 817FF370  lwz r11, -0xc90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-3216 as u32) ) } as u64;
	// 82419C74: 814AF3C4  lwz r10, -0xc3c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-3132 as u32) ) } as u64;
	// 82419C78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82419C7C: 409A0068  bne cr6, 0x82419ce4
	if !ctx.cr[6].eq {
	pc = 0x82419CE4; continue 'dispatch;
	}
	// 82419C80: 480020A1  bl 0x8241bd20
	ctx.lr = 0x82419C84;
	sub_8241BD20(ctx, base);
	// 82419C84: 480099BD  bl 0x82423640
	ctx.lr = 0x82419C88;
	sub_82423640(ctx, base);
	// 82419C88: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82419C8C: 38A00A80  li r5, 0xa80
	ctx.r[5].s64 = 2688;
	// 82419C90: 386BF820  addi r3, r11, -0x7e0
	ctx.r[3].s64 = ctx.r[11].s64 + -2016;
	// 82419C94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82419C98: 4811B539  bl 0x825351d0
	ctx.lr = 0x82419C9C;
	sub_825351D0(ctx, base);
	// 82419C9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419CA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82419CA4: 38EBF820  addi r7, r11, -0x7e0
	ctx.r[7].s64 = ctx.r[11].s64 + -2016;
	// 82419CA8: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 82419CAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82419CB0: 38AB9C28  addi r5, r11, -0x63d8
	ctx.r[5].s64 = ctx.r[11].s64 + -25560;
	// 82419CB4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82419CB8: 4800C2A9  bl 0x82425f60
	ctx.lr = 0x82419CBC;
	sub_82425F60(ctx, base);
	// 82419CBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419CC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82419CC4: 38CBF810  addi r6, r11, -0x7f0
	ctx.r[6].s64 = ctx.r[11].s64 + -2032;
	// 82419CC8: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 82419CCC: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82419CD0: 388B8508  addi r4, r11, -0x7af8
	ctx.r[4].s64 = ctx.r[11].s64 + -31480;
	// 82419CD4: 4800C105  bl 0x82425dd8
	ctx.lr = 0x82419CD8;
	sub_82425DD8(ctx, base);
	// 82419CD8: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82419CDC: 906BF374  stw r3, -0xc8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3212 as u32), ctx.r[3].u32 ) };
	// 82419CE0: 817FF370  lwz r11, -0xc90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-3216 as u32) ) } as u64;
	// 82419CE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82419CE8: 917FF370  stw r11, -0xc90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-3216 as u32), ctx.r[11].u32 ) };
	// 82419CEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82419CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82419CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82419CF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82419CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419D00 size=104
    let mut pc: u32 = 0x82419D00;
    'dispatch: loop {
        match pc {
            0x82419D00 => {
    //   block [0x82419D00..0x82419D68)
	// 82419D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419D04: 4811B3B5  bl 0x825350b8
	ctx.lr = 0x82419D08;
	sub_82535080(ctx, base);
	// 82419D08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419D10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82419D14: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82419D18: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82419D1C: 4800991D  bl 0x82423638
	ctx.lr = 0x82419D20;
	sub_82423638(ctx, base);
	// 82419D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82419D24: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82419D28: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82419D2C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82419D30: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82419D34: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82419D38: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82419D3C: 48015215  bl 0x8242ef50
	ctx.lr = 0x82419D40;
	sub_8242EF50(ctx, base);
	// 82419D40: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82419D44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82419D48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82419D4C: 4BFFFA25  bl 0x82419770
	ctx.lr = 0x82419D50;
	sub_82419770(ctx, base);
	// 82419D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419D54: 480151FD  bl 0x8242ef50
	ctx.lr = 0x82419D58;
	sub_8242EF50(ctx, base);
	// 82419D58: 480151F1  bl 0x8242ef48
	ctx.lr = 0x82419D5C;
	sub_8242EF48(ctx, base);
	// 82419D5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419D60: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82419D64: 4811B3A4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419D68 size=172
    let mut pc: u32 = 0x82419D68;
    'dispatch: loop {
        match pc {
            0x82419D68 => {
    //   block [0x82419D68..0x82419E14)
	// 82419D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82419D70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82419D74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82419D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419D7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419D80: 480151D1  bl 0x8242ef50
	ctx.lr = 0x82419D84;
	sub_8242EF50(ctx, base);
	// 82419D84: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82419D88: 4800AAA9  bl 0x82424830
	ctx.lr = 0x82419D8C;
	sub_82424830(ctx, base);
	// 82419D8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82419D90: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82419D94: 4800AAB5  bl 0x82424848
	ctx.lr = 0x82419D98;
	sub_82424848(ctx, base);
	// 82419D98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82419D9C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82419DA0: 4800AAC9  bl 0x82424868
	ctx.lr = 0x82419DA4;
	sub_82424868(ctx, base);
	// 82419DA4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419DA8: 4800C3E1  bl 0x82426188
	ctx.lr = 0x82419DAC;
	sub_82426188(ctx, base);
	// 82419DAC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82419DB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82419DB4: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82419DB8: 409A0024  bne cr6, 0x82419ddc
	if !ctx.cr[6].eq {
	pc = 0x82419DDC; continue 'dispatch;
	}
	// 82419DBC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82419DC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82419DC4: 41820018  beq 0x82419ddc
	if ctx.cr[0].eq {
	pc = 0x82419DDC; continue 'dispatch;
	}
	// 82419DC8: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82419DCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419DD0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82419DD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82419DD8: 4E800421  bctrl
	ctx.lr = 0x82419DDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82419DDC: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82419DE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82419DE4: 41820008  beq 0x82419dec
	if ctx.cr[0].eq {
	pc = 0x82419DEC; continue 'dispatch;
	}
	// 82419DE8: 480099A9  bl 0x82423790
	ctx.lr = 0x82419DEC;
	sub_82423790(ctx, base);
	// 82419DEC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82419DF0: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 82419DF4: 9BDF00A8  stb r30, 0xa8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u8 ) };
	// 82419DF8: 48015159  bl 0x8242ef50
	ctx.lr = 0x82419DFC;
	sub_8242EF50(ctx, base);
	// 82419DFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82419E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82419E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82419E08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82419E0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82419E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419E18 size=140
    let mut pc: u32 = 0x82419E18;
    'dispatch: loop {
        match pc {
            0x82419E18 => {
    //   block [0x82419E18..0x82419EA4)
	// 82419E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82419E20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82419E24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419E28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419E2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82419E30: 409A0014  bne cr6, 0x82419e44
	if !ctx.cr[6].eq {
	pc = 0x82419E44; continue 'dispatch;
	}
	// 82419E34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419E38: 386BF830  addi r3, r11, -0x7d0
	ctx.r[3].s64 = ctx.r[11].s64 + -2000;
	// 82419E3C: 4800745D  bl 0x82421298
	ctx.lr = 0x82419E40;
	sub_82421298(ctx, base);
	// 82419E40: 48000050  b 0x82419e90
	pc = 0x82419E90; continue 'dispatch;
	// 82419E44: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82419E48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82419E4C: 41820008  beq 0x82419e54
	if ctx.cr[0].eq {
	pc = 0x82419E54; continue 'dispatch;
	}
	// 82419E50: 48008751  bl 0x824225a0
	ctx.lr = 0x82419E54;
	sub_824225A0(ctx, base);
	// 82419E54: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82419E58: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82419E5C: 409A002C  bne cr6, 0x82419e88
	if !ctx.cr[6].eq {
	pc = 0x82419E88; continue 'dispatch;
	}
	// 82419E60: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82419E64: 4800DBDD  bl 0x82427a40
	ctx.lr = 0x82419E68;
	sub_82427A40(ctx, base);
	// 82419E68: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82419E6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82419E70: 419A0018  beq cr6, 0x82419e88
	if ctx.cr[6].eq {
	pc = 0x82419E88; continue 'dispatch;
	}
	// 82419E74: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82419E78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419E7C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82419E80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82419E84: 4E800421  bctrl
	ctx.lr = 0x82419E88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82419E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419E8C: 4BFFFEDD  bl 0x82419d68
	ctx.lr = 0x82419E90;
	sub_82419D68(ctx, base);
	// 82419E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82419E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82419E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82419E9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82419EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419EA8 size=172
    let mut pc: u32 = 0x82419EA8;
    'dispatch: loop {
        match pc {
            0x82419EA8 => {
    //   block [0x82419EA8..0x82419F54)
	// 82419EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419EAC: 4811B211  bl 0x825350bc
	ctx.lr = 0x82419EB0;
	sub_82535080(ctx, base);
	// 82419EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419EB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82419EB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82419EBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82419EC0: 897E0001  lbz r11, 1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 82419EC4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419EC8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82419ECC: 419A0060  beq cr6, 0x82419f2c
	if ctx.cr[6].eq {
	pc = 0x82419F2C; continue 'dispatch;
	}
	// 82419ED0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82419ED4: 419A0058  beq cr6, 0x82419f2c
	if ctx.cr[6].eq {
	pc = 0x82419F2C; continue 'dispatch;
	}
	// 82419ED8: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82419EDC: 409A003C  bne cr6, 0x82419f18
	if !ctx.cr[6].eq {
	pc = 0x82419F18; continue 'dispatch;
	}
	// 82419EE0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419EE4: 4800C90D  bl 0x824267f0
	ctx.lr = 0x82419EE8;
	sub_824267F0(ctx, base);
	// 82419EE8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82419EEC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419EF0: 4800C8E1  bl 0x824267d0
	ctx.lr = 0x82419EF4;
	sub_824267D0(ctx, base);
	// 82419EF4: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82419EF8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419EFC: 4800C8E5  bl 0x824267e0
	ctx.lr = 0x82419F00;
	sub_824267E0(ctx, base);
	// 82419F00: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82419F04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419F08: 7D4A1BD6  divw r10, r10, r3
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[3].s32;
	// 82419F0C: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82419F10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82419F14: 48000028  b 0x82419f3c
	pc = 0x82419F3C; continue 'dispatch;
	// 82419F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82419F1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82419F20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82419F24: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82419F28: 48000014  b 0x82419f3c
	pc = 0x82419F3C; continue 'dispatch;
	// 82419F2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82419F30: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82419F34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82419F38: 4800A951  bl 0x82424888
	ctx.lr = 0x82419F3C;
	sub_82424888(ctx, base);
	// 82419F3C: 817E0088  lwz r11, 0x88(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82419F40: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419F44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82419F48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82419F4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82419F50: 4811B1BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82419F58 size=648
    let mut pc: u32 = 0x82419F58;
    'dispatch: loop {
        match pc {
            0x82419F58 => {
    //   block [0x82419F58..0x8241A1E0)
	// 82419F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419F5C: 4811B155  bl 0x825350b0
	ctx.lr = 0x82419F60;
	sub_82535080(ctx, base);
	// 82419F60: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82419F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419F68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82419F6C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82419F70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82419F74: 419A0258  beq cr6, 0x8241a1cc
	if ctx.cr[6].eq {
	pc = 0x8241A1CC; continue 'dispatch;
	}
	// 82419F78: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82419F7C: 419A0250  beq cr6, 0x8241a1cc
	if ctx.cr[6].eq {
	pc = 0x8241A1CC; continue 'dispatch;
	}
	// 82419F80: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82419F84: 419A0248  beq cr6, 0x8241a1cc
	if ctx.cr[6].eq {
	pc = 0x8241A1CC; continue 'dispatch;
	}
	// 82419F88: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82419F8C: 816BF3C0  lwz r11, -0xc40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3136 as u32) ) } as u64;
	// 82419F90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82419F94: 409A000C  bne cr6, 0x82419fa0
	if !ctx.cr[6].eq {
	pc = 0x82419FA0; continue 'dispatch;
	}
	// 82419F98: 4BFFFF11  bl 0x82419ea8
	ctx.lr = 0x82419F9C;
	sub_82419EA8(ctx, base);
	// 82419F9C: 4800023C  b 0x8241a1d8
	pc = 0x8241A1D8; continue 'dispatch;
	// 82419FA0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82419FA4: 3F808313  lis r28, -0x7ced
	ctx.r[28].s64 = -2095906816;
	// 82419FA8: 3FA08313  lis r29, -0x7ced
	ctx.r[29].s64 = -2095906816;
	// 82419FAC: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82419FB0: D01CF800  stfs f0, -0x800(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-2048 as u32), tmp.u32 ) };
	// 82419FB4: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82419FB8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419FBC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82419FC0: 419A00B8  beq cr6, 0x8241a078
	if ctx.cr[6].eq {
	pc = 0x8241A078; continue 'dispatch;
	}
	// 82419FC4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82419FC8: 419A00B0  beq cr6, 0x8241a078
	if ctx.cr[6].eq {
	pc = 0x8241A078; continue 'dispatch;
	}
	// 82419FCC: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82419FD0: 409A009C  bne cr6, 0x8241a06c
	if !ctx.cr[6].eq {
	pc = 0x8241A06C; continue 'dispatch;
	}
	// 82419FD4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419FD8: 4800C819  bl 0x824267f0
	ctx.lr = 0x82419FDC;
	sub_824267F0(ctx, base);
	// 82419FDC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82419FE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419FE4: 4800C7ED  bl 0x824267d0
	ctx.lr = 0x82419FE8;
	sub_824267D0(ctx, base);
	// 82419FE8: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82419FEC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82419FF0: 4800C7F1  bl 0x824267e0
	ctx.lr = 0x82419FF4;
	sub_824267E0(ctx, base);
	// 82419FF4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82419FF8: 7D2B1BD6  divw r9, r11, r3
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[3].s32;
	// 82419FFC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241A000: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8241A004: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8241A008: E941005A  lwa r10, 0x58(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8241A00C: E91DF806  lwa r8, -0x7fc(r29)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-2044 as u32) ) } as i32) as i64;
	// 8241A010: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8241A014: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8241A018: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8241A01C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8241A020: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8241A024: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241A028: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 8241A02C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8241A030: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8241A034: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8241A038: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8241A03C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8241A040: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8241A044: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 8241A048: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8241A04C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8241A050: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8241A054: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8241A058: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241A05C: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8241A060: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241A064: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241A068: 48000008  b 0x8241a070
	pc = 0x8241A070; continue 'dispatch;
	// 8241A06C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241A070: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241A074: 4800013C  b 0x8241a1b0
	pc = 0x8241A1B0; continue 'dispatch;
	// 8241A078: 897F0072  lbz r11, 0x72(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(114 as u32) ) } as u64;
	// 8241A07C: 3F608289  lis r27, -0x7d77
	ctx.r[27].s64 = -2104950784;
	// 8241A080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241A084: 409A0020  bne cr6, 0x8241a0a4
	if !ctx.cr[6].eq {
	pc = 0x8241A0A4; continue 'dispatch;
	}
	// 8241A088: 813F00A0  lwz r9, 0xa0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8241A08C: 817BF40C  lwz r11, -0xbf4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-3060 as u32) ) } as u64;
	// 8241A090: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8241A094: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8241A098: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 8241A09C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241A0A0: 48000008  b 0x8241a0a8
	pc = 0x8241A0A8; continue 'dispatch;
	// 8241A0A4: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8241A0A8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8241A0AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241A0B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8241A0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A0B8: 4BFFFDF1  bl 0x82419ea8
	ctx.lr = 0x8241A0BC;
	sub_82419EA8(ctx, base);
	// 8241A0BC: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 8241A0C0: E921005A  lwa r9, 0x58(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8241A0C4: E91E0002  lwa r8, 0(r30)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i32) as i64;
	// 8241A0C8: E97DF806  lwa r11, -0x7fc(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-2044 as u32) ) } as i32) as i64;
	// 8241A0CC: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 8241A0D0: F9210068  std r9, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u64 ) };
	// 8241A0D4: F9010070  std r8, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u64 ) };
	// 8241A0D8: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 8241A0DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241A0E0: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8241A0E4: C9A10068  lfd f13, 0x68(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 8241A0E8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8241A0EC: C9810070  lfd f12, 0x70(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 8241A0F0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8241A0F4: C9610078  lfd f11, 0x78(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 8241A0F8: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8241A0FC: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8241A100: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8241A104: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8241A108: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8241A10C: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8241A110: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8241A114: EDAC5824  fdivs f13, f12, f11
	ctx.f[13].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 8241A118: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8241A11C: C00B204C  lfs f0, 0x204c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8241A120: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241A124: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8241A128: C1AB2054  lfs f13, 0x2054(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8276 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241A12C: D01CF800  stfs f0, -0x800(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-2048 as u32), tmp.u32 ) };
	// 8241A130: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8241A134: 41990014  bgt cr6, 0x8241a148
	if ctx.cr[6].gt {
	pc = 0x8241A148; continue 'dispatch;
	}
	// 8241A138: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241A13C: C1AB2800  lfs f13, 0x2800(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10240 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241A140: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8241A144: 4098006C  bge cr6, 0x8241a1b0
	if !ctx.cr[6].lt {
	pc = 0x8241A1B0; continue 'dispatch;
	}
	// 8241A148: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8241A14C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A150: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8241A154: 4800A735  bl 0x82424888
	ctx.lr = 0x8241A158;
	sub_82424888(ctx, base);
	// 8241A158: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 8241A15C: E921005A  lwa r9, 0x58(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8241A160: E97DF806  lwa r11, -0x7fc(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-2044 as u32) ) } as i32) as i64;
	// 8241A164: F9410078  std r10, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u64 ) };
	// 8241A168: F9210070  std r9, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u64 ) };
	// 8241A16C: F9610068  std r11, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 8241A170: C8010078  lfd f0, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 8241A174: C9A10070  lfd f13, 0x70(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 8241A178: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8241A17C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8241A180: C9810068  lfd f12, 0x68(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 8241A184: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8241A188: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8241A18C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8241A190: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8241A194: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8241A198: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8241A19C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8241A1A0: 3980009C  li r12, 0x9c
	ctx.r[12].s64 = 156;
	// 8241A1A4: 7C1F67AE  stfiwx f0, r31, r12
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[12].u32), tmp.u32) };
	// 8241A1A8: 817BF40C  lwz r11, -0xbf4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-3060 as u32) ) } as u64;
	// 8241A1AC: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8241A1B0: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241A1B4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A1B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241A1BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241A1C0: 817DF804  lwz r11, -0x7fc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-2044 as u32) ) } as u64;
	// 8241A1C4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241A1C8: 48000010  b 0x8241a1d8
	pc = 0x8241A1D8; continue 'dispatch;
	// 8241A1CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A1D0: 386BF880  addi r3, r11, -0x780
	ctx.r[3].s64 = ctx.r[11].s64 + -1920;
	// 8241A1D4: 480070C5  bl 0x82421298
	ctx.lr = 0x8241A1D8;
	sub_82421298(ctx, base);
	// 8241A1D8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8241A1DC: 4811AF24  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241A1E0 size=296
    let mut pc: u32 = 0x8241A1E0;
    'dispatch: loop {
        match pc {
            0x8241A1E0 => {
    //   block [0x8241A1E0..0x8241A308)
	// 8241A1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A1E4: 4811AED5  bl 0x825350b8
	ctx.lr = 0x8241A1E8;
	sub_82535080(ctx, base);
	// 8241A1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A1EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8241A1F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241A1F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8241A1F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8241A1FC: 409A0010  bne cr6, 0x8241a20c
	if !ctx.cr[6].eq {
	pc = 0x8241A20C; continue 'dispatch;
	}
	// 8241A200: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A204: 386BF984  addi r3, r11, -0x67c
	ctx.r[3].s64 = ctx.r[11].s64 + -1660;
	// 8241A208: 480000F4  b 0x8241a2fc
	pc = 0x8241A2FC; continue 'dispatch;
	// 8241A20C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241A210: 419A0018  beq cr6, 0x8241a228
	if ctx.cr[6].eq {
	pc = 0x8241A228; continue 'dispatch;
	}
	// 8241A214: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8241A218: 419A0010  beq cr6, 0x8241a228
	if ctx.cr[6].eq {
	pc = 0x8241A228; continue 'dispatch;
	}
	// 8241A21C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A220: 386BF958  addi r3, r11, -0x6a8
	ctx.r[3].s64 = ctx.r[11].s64 + -1704;
	// 8241A224: 480000D8  b 0x8241a2fc
	pc = 0x8241A2FC; continue 'dispatch;
	// 8241A228: 2F1CFF80  cmpwi cr6, r28, -0x80
	ctx.cr[6].compare_i32(ctx.r[28].s32, -128, &mut ctx.xer);
	// 8241A22C: 419A0020  beq cr6, 0x8241a24c
	if ctx.cr[6].eq {
	pc = 0x8241A24C; continue 'dispatch;
	}
	// 8241A230: 2F1CFFF1  cmpwi cr6, r28, -0xf
	ctx.cr[6].compare_i32(ctx.r[28].s32, -15, &mut ctx.xer);
	// 8241A234: 4098000C  bge cr6, 0x8241a240
	if !ctx.cr[6].lt {
	pc = 0x8241A240; continue 'dispatch;
	}
	// 8241A238: 3B80FFF1  li r28, -0xf
	ctx.r[28].s64 = -15;
	// 8241A23C: 48000010  b 0x8241a24c
	pc = 0x8241A24C; continue 'dispatch;
	// 8241A240: 2F1C000F  cmpwi cr6, r28, 0xf
	ctx.cr[6].compare_i32(ctx.r[28].s32, 15, &mut ctx.xer);
	// 8241A244: 40990008  ble cr6, 0x8241a24c
	if !ctx.cr[6].gt {
	pc = 0x8241A24C; continue 'dispatch;
	}
	// 8241A248: 3B80000F  li r28, 0xf
	ctx.r[28].s64 = 15;
	// 8241A24C: 897D00A9  lbz r11, 0xa9(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(169 as u32) ) } as u64;
	// 8241A250: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8241A254: 409A001C  bne cr6, 0x8241a270
	if !ctx.cr[6].eq {
	pc = 0x8241A270; continue 'dispatch;
	}
	// 8241A258: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241A25C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241A260: 4800C631  bl 0x82426890
	ctx.lr = 0x8241A264;
	sub_82426890(ctx, base);
	// 8241A264: 7C7F0734  extsh r31, r3
	ctx.r[31].s64 = ctx.r[3].s16 as i64;
	// 8241A268: 2F1FFF80  cmpwi cr6, r31, -0x80
	ctx.cr[6].compare_i32(ctx.r[31].s32, -128, &mut ctx.xer);
	// 8241A26C: 409A0008  bne cr6, 0x8241a274
	if !ctx.cr[6].eq {
	pc = 0x8241A274; continue 'dispatch;
	}
	// 8241A270: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241A274: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241A278: 816BF404  lwz r11, -0xbfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3068 as u32) ) } as u64;
	// 8241A27C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241A280: 409A0044  bne cr6, 0x8241a2c4
	if !ctx.cr[6].eq {
	pc = 0x8241A2C4; continue 'dispatch;
	}
	// 8241A284: 2F1CFF80  cmpwi cr6, r28, -0x80
	ctx.cr[6].compare_i32(ctx.r[28].s32, -128, &mut ctx.xer);
	// 8241A288: 409A0034  bne cr6, 0x8241a2bc
	if !ctx.cr[6].eq {
	pc = 0x8241A2BC; continue 'dispatch;
	}
	// 8241A28C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241A290: 4800C549  bl 0x824267d8
	ctx.lr = 0x8241A294;
	sub_824267D8(ctx, base);
	// 8241A294: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8241A298: 409A0018  bne cr6, 0x8241a2b0
	if !ctx.cr[6].eq {
	pc = 0x8241A2B0; continue 'dispatch;
	}
	// 8241A29C: 217E0000  subfic r11, r30, 0
	ctx.xer.ca = ctx.r[30].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[30].s64;
	// 8241A2A0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8241A2A4: 556B06FC  rlwinm r11, r11, 0, 0x1b, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8241A2A8: 396BFFF1  addi r11, r11, -0xf
	ctx.r[11].s64 = ctx.r[11].s64 + -15;
	// 8241A2AC: 48000008  b 0x8241a2b4
	pc = 0x8241A2B4; continue 'dispatch;
	// 8241A2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241A2B4: 7CABFA14  add r5, r11, r31
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8241A2B8: 48000010  b 0x8241a2c8
	pc = 0x8241A2C8; continue 'dispatch;
	// 8241A2BC: 7CBFE214  add r5, r31, r28
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 8241A2C0: 48000008  b 0x8241a2c8
	pc = 0x8241A2C8; continue 'dispatch;
	// 8241A2C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241A2C8: 397E0021  addi r11, r30, 0x21
	ctx.r[11].s64 = ctx.r[30].s64 + 33;
	// 8241A2CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241A2D0: 7F8BEB2E  sthx r28, r11, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u16) };
	// 8241A2D4: 897D0003  lbz r11, 3(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241A2D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241A2DC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241A2E0: 40980014  bge cr6, 0x8241a2f4
	if !ctx.cr[6].lt {
	pc = 0x8241A2F4; continue 'dispatch;
	}
	// 8241A2E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241A2E8: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A2EC: 4800A835  bl 0x82424b20
	ctx.lr = 0x8241A2F0;
	sub_82424B20(ctx, base);
	// 8241A2F0: 48000010  b 0x8241a300
	pc = 0x8241A300; continue 'dispatch;
	// 8241A2F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A2F8: 386BF92C  addi r3, r11, -0x6d4
	ctx.r[3].s64 = ctx.r[11].s64 + -1748;
	// 8241A2FC: 48006F9D  bl 0x82421298
	ctx.lr = 0x8241A300;
	sub_82421298(ctx, base);
	// 8241A300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241A304: 4811AE04  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A308 size=276
    let mut pc: u32 = 0x8241A308;
    'dispatch: loop {
        match pc {
            0x8241A308 => {
    //   block [0x8241A308..0x8241A41C)
	// 8241A308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A30C: 4811ADAD  bl 0x825350b8
	ctx.lr = 0x8241A310;
	sub_82535080(ctx, base);
	// 8241A310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A314: 48014C3D  bl 0x8242ef50
	ctx.lr = 0x8241A318;
	sub_8242EF50(ctx, base);
	// 8241A318: 3F808289  lis r28, -0x7d77
	ctx.r[28].s64 = -2104950784;
	// 8241A31C: 817CF3C4  lwz r11, -0xc3c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-3132 as u32) ) } as u64;
	// 8241A320: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241A324: 419A000C  beq cr6, 0x8241a330
	if ctx.cr[6].eq {
	pc = 0x8241A330; continue 'dispatch;
	}
	// 8241A328: 48014C29  bl 0x8242ef50
	ctx.lr = 0x8241A32C;
	sub_8242EF50(ctx, base);
	// 8241A32C: 480000E8  b 0x8241a414
	pc = 0x8241A414; continue 'dispatch;
	// 8241A330: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241A334: 917CF3C4  stw r11, -0xc3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-3132 as u32), ctx.r[11].u32 ) };
	// 8241A338: 48014C19  bl 0x8242ef50
	ctx.lr = 0x8241A33C;
	sub_8242EF50(ctx, base);
	// 8241A33C: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241A340: 3BEBF3AC  addi r31, r11, -0xc54
	ctx.r[31].s64 = ctx.r[11].s64 + -3156;
	// 8241A344: 817FFFF4  lwz r11, -0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241A348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241A34C: 419A0018  beq cr6, 0x8241a364
	if ctx.cr[6].eq {
	pc = 0x8241A364; continue 'dispatch;
	}
	// 8241A350: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8241A354: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241A358: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A35C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A360: 4E800421  bctrl
	ctx.lr = 0x8241A364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A364: 4800CDDD  bl 0x82427140
	ctx.lr = 0x8241A368;
	sub_82427140(ctx, base);
	// 8241A368: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8241A36C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8241A370: 3BABDF80  addi r29, r11, -0x2080
	ctx.r[29].s64 = ctx.r[11].s64 + -8320;
	// 8241A374: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8241A378: 915CF3C4  stw r10, -0xc3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-3132 as u32), ctx.r[10].u32 ) };
	// 8241A37C: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A380: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8241A384: 409A000C  bne cr6, 0x8241a390
	if !ctx.cr[6].eq {
	pc = 0x8241A390; continue 'dispatch;
	}
	// 8241A388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241A38C: 4800EACD  bl 0x82428e58
	ctx.lr = 0x8241A390;
	sub_82428E58(ctx, base);
	// 8241A390: 3BDE00C4  addi r30, r30, 0xc4
	ctx.r[30].s64 = ctx.r[30].s64 + 196;
	// 8241A394: 397D1880  addi r11, r29, 0x1880
	ctx.r[11].s64 = ctx.r[29].s64 + 6272;
	// 8241A398: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241A39C: 4198FFE0  blt cr6, 0x8241a37c
	if ctx.cr[6].lt {
	pc = 0x8241A37C; continue 'dispatch;
	}
	// 8241A3A0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8241A3A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241A3A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241A3AC: 915CF3C4  stw r10, -0xc3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-3132 as u32), ctx.r[10].u32 ) };
	// 8241A3B0: 419A0018  beq cr6, 0x8241a3c8
	if ctx.cr[6].eq {
	pc = 0x8241A3C8; continue 'dispatch;
	}
	// 8241A3B4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8241A3B8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241A3BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A3C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A3C4: 4E800421  bctrl
	ctx.lr = 0x8241A3C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A3C8: 4800A5A9  bl 0x82424970
	ctx.lr = 0x8241A3CC;
	sub_82424970(ctx, base);
	// 8241A3CC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A3D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241A3D4: 419A0018  beq cr6, 0x8241a3ec
	if ctx.cr[6].eq {
	pc = 0x8241A3EC; continue 'dispatch;
	}
	// 8241A3D8: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 8241A3DC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241A3E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A3E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A3E8: 4E800421  bctrl
	ctx.lr = 0x8241A3EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A3EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241A3F0: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241A3F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241A3F8: 915CF3C4  stw r10, -0xc3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-3132 as u32), ctx.r[10].u32 ) };
	// 8241A3FC: 419A0018  beq cr6, 0x8241a414
	if ctx.cr[6].eq {
	pc = 0x8241A414; continue 'dispatch;
	}
	// 8241A400: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241A404: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A408: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A40C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A410: 4E800421  bctrl
	ctx.lr = 0x8241A414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241A418: 4811ACF0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A420 size=132
    let mut pc: u32 = 0x8241A420;
    'dispatch: loop {
        match pc {
            0x8241A420 => {
    //   block [0x8241A420..0x8241A4A4)
	// 8241A420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241A428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241A42C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A430: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8241A434: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8241A438: 4098000C  bge cr6, 0x8241a444
	if !ctx.cr[6].lt {
	pc = 0x8241A444; continue 'dispatch;
	}
	// 8241A43C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241A440: 48000050  b 0x8241a490
	pc = 0x8241A490; continue 'dispatch;
	// 8241A444: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A448: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 8241A44C: 409AFFF0  bne cr6, 0x8241a43c
	if !ctx.cr[6].eq {
	pc = 0x8241A43C; continue 'dispatch;
	}
	// 8241A450: 38A1006C  addi r5, r1, 0x6c
	ctx.r[5].s64 = ctx.r[1].s64 + 108;
	// 8241A454: 39610068  addi r11, r1, 0x68
	ctx.r[11].s64 = ctx.r[1].s64 + 104;
	// 8241A458: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8241A45C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8241A460: 39010061  addi r8, r1, 0x61
	ctx.r[8].s64 = ctx.r[1].s64 + 97;
	// 8241A464: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8241A468: 38E10062  addi r7, r1, 0x62
	ctx.r[7].s64 = ctx.r[1].s64 + 98;
	// 8241A46C: 38C10063  addi r6, r1, 0x63
	ctx.r[6].s64 = ctx.r[1].s64 + 99;
	// 8241A470: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8241A474: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 8241A478: 4800D871  bl 0x82427ce8
	ctx.lr = 0x8241A47C;
	sub_82427CE8(ctx, base);
	// 8241A47C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241A480: 4180FFBC  blt 0x8241a43c
	if ctx.cr[0].lt {
	pc = 0x8241A43C; continue 'dispatch;
	}
	// 8241A484: A9610064  lha r11, 0x64(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as i16) as i64;
	// 8241A488: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241A48C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241A490: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241A494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241A498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241A49C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241A4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241A4A8 size=16
    let mut pc: u32 = 0x8241A4A8;
    'dispatch: loop {
        match pc {
            0x8241A4A8 => {
    //   block [0x8241A4A8..0x8241A4B8)
	// 8241A4A8: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8241A4AC: 4098000C  bge cr6, 0x8241a4b8
	if !ctx.cr[6].lt {
		sub_8241A4B8(ctx, base);
		return;
	}
	// 8241A4B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241A4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241A4B8 size=24
    let mut pc: u32 = 0x8241A4B8;
    'dispatch: loop {
        match pc {
            0x8241A4B8 => {
    //   block [0x8241A4B8..0x8241A4D0)
	// 8241A4B8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A4BC: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 8241A4C0: 409AFFF0  bne cr6, 0x8241a4b0
	if !ctx.cr[6].eq {
		sub_8241A4A8(ctx, base);
		return;
	}
	// 8241A4C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241A4C8: 90850000  stw r4, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8241A4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241A4D0 size=344
    let mut pc: u32 = 0x8241A4D0;
    'dispatch: loop {
        match pc {
            0x8241A4D0 => {
    //   block [0x8241A4D0..0x8241A628)
	// 8241A4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A4D4: 4811ABE1  bl 0x825350b4
	ctx.lr = 0x8241A4D8;
	sub_82535080(ctx, base);
	// 8241A4D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A4DC: 83E30014  lwz r31, 0x14(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241A4E0: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A4E4: 4082000C  bne 0x8241a4f0
	if !ctx.cr[0].eq {
	pc = 0x8241A4F0; continue 'dispatch;
	}
	// 8241A4E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241A4EC: 48000134  b 0x8241a620
	pc = 0x8241A620; continue 'dispatch;
	// 8241A4F0: 7CAA2E70  srawi r10, r5, 5
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[5].s32 >> 5) as i64;
	// 8241A4F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A4F8: 1FC40012  mulli r30, r4, 0x12
	ctx.r[30].s64 = ctx.r[4].s64 * 18;
	// 8241A4FC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241A500: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8241A504: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8241A508: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A50C: 7F8AF1D6  mullw r28, r10, r30
	ctx.r[28].s64 = (ctx.r[10].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8241A510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A514: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8241A518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A51C: 4E800421  bctrl
	ctx.lr = 0x8241A520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A520: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241A524: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A528: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241A52C: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 8241A530: 7FABF1D6  mullw r29, r11, r30
	ctx.r[29].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8241A534: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241A538: 4811AC99  bl 0x825351d0
	ctx.lr = 0x8241A53C;
	sub_825351D0(ctx, base);
	// 8241A53C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8241A540: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241A544: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241A548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8241A54C: 4800E9BD  bl 0x82428f08
	ctx.lr = 0x8241A550;
	sub_82428F08(ctx, base);
	// 8241A550: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A554: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241A558: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241A55C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A560: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8241A564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A568: 4E800421  bctrl
	ctx.lr = 0x8241A56C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A56C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A570: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8241A574: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A57C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241A580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A584: 4E800421  bctrl
	ctx.lr = 0x8241A588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A58C: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 8241A590: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8241A594: 7CBDE050  subf r5, r29, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 8241A598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A59C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241A5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A5A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A5A8: 4E800421  bctrl
	ctx.lr = 0x8241A5AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A5AC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241A5B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A5B4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241A5B8: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 8241A5BC: 7FABF1D6  mullw r29, r11, r30
	ctx.r[29].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8241A5C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241A5C4: 4811AC0D  bl 0x825351d0
	ctx.lr = 0x8241A5C8;
	sub_825351D0(ctx, base);
	// 8241A5C8: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8241A5CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241A5D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241A5D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8241A5D8: 4800E931  bl 0x82428f08
	ctx.lr = 0x8241A5DC;
	sub_82428F08(ctx, base);
	// 8241A5DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A5E0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241A5E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241A5E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A5EC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8241A5F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A5F4: 4E800421  bctrl
	ctx.lr = 0x8241A5F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A5F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A5FC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8241A600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A608: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241A60C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A610: 4E800421  bctrl
	ctx.lr = 0x8241A614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A614: 7D7DDA14  add r11, r29, r27
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[27].u64;
	// 8241A618: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 8241A61C: 55632834  slwi r3, r11, 5
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8241A620: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241A624: 4811AAE0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A628 size=424
    let mut pc: u32 = 0x8241A628;
    'dispatch: loop {
        match pc {
            0x8241A628 => {
    //   block [0x8241A628..0x8241A7D0)
	// 8241A628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A62C: 4811AA8D  bl 0x825350b8
	ctx.lr = 0x8241A630;
	sub_82535080(ctx, base);
	// 8241A630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A638: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241A63C: 409A0014  bne cr6, 0x8241a650
	if !ctx.cr[6].eq {
	pc = 0x8241A650; continue 'dispatch;
	}
	// 8241A640: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A644: 386BFBA4  addi r3, r11, -0x45c
	ctx.r[3].s64 = ctx.r[11].s64 + -1116;
	// 8241A648: 48006C51  bl 0x82421298
	ctx.lr = 0x8241A64C;
	sub_82421298(ctx, base);
	// 8241A64C: 4800017C  b 0x8241a7c8
	pc = 0x8241A7C8; continue 'dispatch;
	// 8241A650: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241A654: 814BF3D0  lwz r10, -0xc30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3120 as u32) ) } as u64;
	// 8241A658: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241A65C: 419A0014  beq cr6, 0x8241a670
	if ctx.cr[6].eq {
	pc = 0x8241A670; continue 'dispatch;
	}
	// 8241A660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A664: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241A668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A66C: 4E800421  bctrl
	ctx.lr = 0x8241A670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A670: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A674: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8241A678: 409A000C  bne cr6, 0x8241a684
	if !ctx.cr[6].eq {
	pc = 0x8241A684; continue 'dispatch;
	}
	// 8241A67C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A680: 4BFFF799  bl 0x82419e18
	ctx.lr = 0x8241A684;
	sub_82419E18(ctx, base);
	// 8241A684: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A688: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241A68C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A690: 4182000C  beq 0x8241a69c
	if ctx.cr[0].eq {
	pc = 0x8241A69C; continue 'dispatch;
	}
	// 8241A694: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8241A698: 4800A6E9  bl 0x82424d80
	ctx.lr = 0x8241A69C;
	sub_82424D80(ctx, base);
	// 8241A69C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241A6A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A6A4: 4182000C  beq 0x8241a6b0
	if ctx.cr[0].eq {
	pc = 0x8241A6B0; continue 'dispatch;
	}
	// 8241A6A8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8241A6AC: 4800BA0D  bl 0x824260b8
	ctx.lr = 0x8241A6B0;
	sub_824260B8(ctx, base);
	// 8241A6B0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241A6B4: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A6B8: 41820020  beq 0x8241a6d8
	if ctx.cr[0].eq {
	pc = 0x8241A6D8; continue 'dispatch;
	}
	// 8241A6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241A6C0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8241A6C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A6C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241A6CC: 4800729D  bl 0x82421968
	ctx.lr = 0x8241A6D0;
	sub_82421968(ctx, base);
	// 8241A6D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241A6D4: 48007F9D  bl 0x82422670
	ctx.lr = 0x8241A6D8;
	sub_82422670(ctx, base);
	// 8241A6D8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8241A6DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A6E0: 4182000C  beq 0x8241a6ec
	if ctx.cr[0].eq {
	pc = 0x8241A6EC; continue 'dispatch;
	}
	// 8241A6E4: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 8241A6E8: 4800D3F1  bl 0x82427ad8
	ctx.lr = 0x8241A6EC;
	sub_82427AD8(ctx, base);
	// 8241A6EC: 48014865  bl 0x8242ef50
	ctx.lr = 0x8241A6F0;
	sub_8242EF50(ctx, base);
	// 8241A6F0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241A6F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A6F8: 41820018  beq 0x8241a710
	if ctx.cr[0].eq {
	pc = 0x8241A710; continue 'dispatch;
	}
	// 8241A6FC: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8241A700: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A704: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A708: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A70C: 4E800421  bctrl
	ctx.lr = 0x8241A710;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A710: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241A714: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 8241A718: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241A71C: 40810080  ble 0x8241a79c
	if !ctx.cr[0].gt {
	pc = 0x8241A79C; continue 'dispatch;
	}
	// 8241A720: 3BDF0078  addi r30, r31, 0x78
	ctx.r[30].s64 = ctx.r[31].s64 + 120;
	// 8241A724: 807EFFA0  lwz r3, -0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-96 as u32) ) } as u64;
	// 8241A728: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A72C: 41820018  beq 0x8241a744
	if ctx.cr[0].eq {
	pc = 0x8241A744; continue 'dispatch;
	}
	// 8241A730: 93BEFFA0  stw r29, -0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-96 as u32), ctx.r[29].u32 ) };
	// 8241A734: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A738: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A73C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A740: 4E800421  bctrl
	ctx.lr = 0x8241A744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A744: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A748: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A74C: 41820018  beq 0x8241a764
	if ctx.cr[0].eq {
	pc = 0x8241A764; continue 'dispatch;
	}
	// 8241A750: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241A754: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A758: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A75C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A760: 4E800421  bctrl
	ctx.lr = 0x8241A764;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A764: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241A768: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A76C: 41820018  beq 0x8241a784
	if ctx.cr[0].eq {
	pc = 0x8241A784; continue 'dispatch;
	}
	// 8241A770: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8241A774: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241A778: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241A77C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241A780: 4E800421  bctrl
	ctx.lr = 0x8241A784;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241A784: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241A788: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8241A78C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8241A790: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241A794: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241A798: 4198FF8C  blt cr6, 0x8241a724
	if ctx.cr[6].lt {
	pc = 0x8241A724; continue 'dispatch;
	}
	// 8241A79C: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8241A7A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241A7A4: 4182000C  beq 0x8241a7b0
	if ctx.cr[0].eq {
	pc = 0x8241A7B0; continue 'dispatch;
	}
	// 8241A7A8: 93BF0074  stw r29, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 8241A7AC: 4800CA25  bl 0x824271d0
	ctx.lr = 0x8241A7B0;
	sub_824271D0(ctx, base);
	// 8241A7B0: 38A000C4  li r5, 0xc4
	ctx.r[5].s64 = 196;
	// 8241A7B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241A7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A7BC: 4811AA15  bl 0x825351d0
	ctx.lr = 0x8241A7C0;
	sub_825351D0(ctx, base);
	// 8241A7C0: 9BBF0000  stb r29, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8241A7C4: 4801478D  bl 0x8242ef50
	ctx.lr = 0x8241A7C8;
	sub_8242EF50(ctx, base);
	// 8241A7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241A7CC: 4811A93C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A7D0 size=56
    let mut pc: u32 = 0x8241A7D0;
    'dispatch: loop {
        match pc {
            0x8241A7D0 => {
    //   block [0x8241A7D0..0x8241A808)
	// 8241A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241A7D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241A7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A7E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A7E4: 4800684D  bl 0x82421030
	ctx.lr = 0x8241A7E8;
	sub_82421030(ctx, base);
	// 8241A7E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A7EC: 4BFFF62D  bl 0x82419e18
	ctx.lr = 0x8241A7F0;
	sub_82419E18(ctx, base);
	// 8241A7F0: 48006881  bl 0x82421070
	ctx.lr = 0x8241A7F4;
	sub_82421070(ctx, base);
	// 8241A7F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241A7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241A7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241A800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241A804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A808 size=88
    let mut pc: u32 = 0x8241A808;
    'dispatch: loop {
        match pc {
            0x8241A808 => {
    //   block [0x8241A808..0x8241A860)
	// 8241A808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241A810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241A814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A81C: 48006815  bl 0x82421030
	ctx.lr = 0x8241A820;
	sub_82421030(ctx, base);
	// 8241A820: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241A824: 409A0018  bne cr6, 0x8241a83c
	if !ctx.cr[6].eq {
	pc = 0x8241A83C; continue 'dispatch;
	}
	// 8241A828: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A82C: 386BF858  addi r3, r11, -0x7a8
	ctx.r[3].s64 = ctx.r[11].s64 + -1960;
	// 8241A830: 48006A69  bl 0x82421298
	ctx.lr = 0x8241A834;
	sub_82421298(ctx, base);
	// 8241A834: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241A838: 4800000C  b 0x8241a844
	pc = 0x8241A844; continue 'dispatch;
	// 8241A83C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8241A840: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 8241A844: 4800682D  bl 0x82421070
	ctx.lr = 0x8241A848;
	sub_82421070(ctx, base);
	// 8241A848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A84C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241A850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241A854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241A858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241A85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A860 size=56
    let mut pc: u32 = 0x8241A860;
    'dispatch: loop {
        match pc {
            0x8241A860 => {
    //   block [0x8241A860..0x8241A898)
	// 8241A860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A864: 4811A859  bl 0x825350bc
	ctx.lr = 0x8241A868;
	sub_82535080(ctx, base);
	// 8241A868: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A86C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A870: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241A874: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241A878: 480067B9  bl 0x82421030
	ctx.lr = 0x8241A87C;
	sub_82421030(ctx, base);
	// 8241A87C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241A880: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241A884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A888: 4BFFF6D1  bl 0x82419f58
	ctx.lr = 0x8241A88C;
	sub_82419F58(ctx, base);
	// 8241A88C: 480067E5  bl 0x82421070
	ctx.lr = 0x8241A890;
	sub_82421070(ctx, base);
	// 8241A890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241A894: 4811A878  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A898 size=116
    let mut pc: u32 = 0x8241A898;
    'dispatch: loop {
        match pc {
            0x8241A898 => {
    //   block [0x8241A898..0x8241A90C)
	// 8241A898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241A8A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241A8A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A8A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A8AC: 48006785  bl 0x82421030
	ctx.lr = 0x8241A8B0;
	sub_82421030(ctx, base);
	// 8241A8B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241A8B4: 409A0018  bne cr6, 0x8241a8cc
	if !ctx.cr[6].eq {
	pc = 0x8241A8CC; continue 'dispatch;
	}
	// 8241A8B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A8BC: 386BF8A8  addi r3, r11, -0x758
	ctx.r[3].s64 = ctx.r[11].s64 + -1880;
	// 8241A8C0: 480069D9  bl 0x82421298
	ctx.lr = 0x8241A8C4;
	sub_82421298(ctx, base);
	// 8241A8C4: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241A8C8: 48000028  b 0x8241a8f0
	pc = 0x8241A8F0; continue 'dispatch;
	// 8241A8CC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8241A8D0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241A8D4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8241A8D8: 41980014  blt cr6, 0x8241a8ec
	if ctx.cr[6].lt {
	pc = 0x8241A8EC; continue 'dispatch;
	}
	// 8241A8DC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241A8E0: 4800BF11  bl 0x824267f0
	ctx.lr = 0x8241A8E4;
	sub_824267F0(ctx, base);
	// 8241A8E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A8E8: 48000008  b 0x8241a8f0
	pc = 0x8241A8F0; continue 'dispatch;
	// 8241A8EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241A8F0: 48006781  bl 0x82421070
	ctx.lr = 0x8241A8F4;
	sub_82421070(ctx, base);
	// 8241A8F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241A8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241A900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241A904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241A908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A910 size=116
    let mut pc: u32 = 0x8241A910;
    'dispatch: loop {
        match pc {
            0x8241A910 => {
    //   block [0x8241A910..0x8241A984)
	// 8241A910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241A918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241A91C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A924: 4800670D  bl 0x82421030
	ctx.lr = 0x8241A928;
	sub_82421030(ctx, base);
	// 8241A928: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241A92C: 409A0018  bne cr6, 0x8241a944
	if !ctx.cr[6].eq {
	pc = 0x8241A944; continue 'dispatch;
	}
	// 8241A930: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A934: 386BF8D4  addi r3, r11, -0x72c
	ctx.r[3].s64 = ctx.r[11].s64 + -1836;
	// 8241A938: 48006961  bl 0x82421298
	ctx.lr = 0x8241A93C;
	sub_82421298(ctx, base);
	// 8241A93C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241A940: 48000028  b 0x8241a968
	pc = 0x8241A968; continue 'dispatch;
	// 8241A944: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8241A948: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241A94C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8241A950: 41980014  blt cr6, 0x8241a964
	if ctx.cr[6].lt {
	pc = 0x8241A964; continue 'dispatch;
	}
	// 8241A954: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241A958: 4800BE79  bl 0x824267d0
	ctx.lr = 0x8241A95C;
	sub_824267D0(ctx, base);
	// 8241A95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A960: 48000008  b 0x8241a968
	pc = 0x8241A968; continue 'dispatch;
	// 8241A964: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241A968: 48006709  bl 0x82421070
	ctx.lr = 0x8241A96C;
	sub_82421070(ctx, base);
	// 8241A96C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241A974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241A978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241A97C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241A980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241A988 size=116
    let mut pc: u32 = 0x8241A988;
    'dispatch: loop {
        match pc {
            0x8241A988 => {
    //   block [0x8241A988..0x8241A9FC)
	// 8241A988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241A98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241A990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241A994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241A998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A99C: 48006695  bl 0x82421030
	ctx.lr = 0x8241A9A0;
	sub_82421030(ctx, base);
	// 8241A9A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241A9A4: 409A0018  bne cr6, 0x8241a9bc
	if !ctx.cr[6].eq {
	pc = 0x8241A9BC; continue 'dispatch;
	}
	// 8241A9A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241A9AC: 386BF900  addi r3, r11, -0x700
	ctx.r[3].s64 = ctx.r[11].s64 + -1792;
	// 8241A9B0: 480068E9  bl 0x82421298
	ctx.lr = 0x8241A9B4;
	sub_82421298(ctx, base);
	// 8241A9B4: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241A9B8: 48000028  b 0x8241a9e0
	pc = 0x8241A9E0; continue 'dispatch;
	// 8241A9BC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8241A9C0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241A9C4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8241A9C8: 41980014  blt cr6, 0x8241a9dc
	if ctx.cr[6].lt {
	pc = 0x8241A9DC; continue 'dispatch;
	}
	// 8241A9CC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241A9D0: 4800BE09  bl 0x824267d8
	ctx.lr = 0x8241A9D4;
	sub_824267D8(ctx, base);
	// 8241A9D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241A9D8: 48000008  b 0x8241a9e0
	pc = 0x8241A9E0; continue 'dispatch;
	// 8241A9DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241A9E0: 48006691  bl 0x82421070
	ctx.lr = 0x8241A9E4;
	sub_82421070(ctx, base);
	// 8241A9E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241A9E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241A9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241A9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241A9F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241A9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AA00 size=56
    let mut pc: u32 = 0x8241AA00;
    'dispatch: loop {
        match pc {
            0x8241AA00 => {
    //   block [0x8241AA00..0x8241AA38)
	// 8241AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AA04: 4811A6B9  bl 0x825350bc
	ctx.lr = 0x8241AA08;
	sub_82535080(ctx, base);
	// 8241AA08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AA0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AA10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AA14: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241AA18: 48006619  bl 0x82421030
	ctx.lr = 0x8241AA1C;
	sub_82421030(ctx, base);
	// 8241AA1C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241AA20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241AA24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241AA28: 4BFFF7B9  bl 0x8241a1e0
	ctx.lr = 0x8241AA2C;
	sub_8241A1E0(ctx, base);
	// 8241AA2C: 48006645  bl 0x82421070
	ctx.lr = 0x8241AA30;
	sub_82421070(ctx, base);
	// 8241AA30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AA34: 4811A6D8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AA38 size=100
    let mut pc: u32 = 0x8241AA38;
    'dispatch: loop {
        match pc {
            0x8241AA38 => {
    //   block [0x8241AA38..0x8241AA9C)
	// 8241AA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AA40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241AA44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AA48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AA4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AA50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AA54: 480065DD  bl 0x82421030
	ctx.lr = 0x8241AA58;
	sub_82421030(ctx, base);
	// 8241AA58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AA5C: 409A0014  bne cr6, 0x8241aa70
	if !ctx.cr[6].eq {
	pc = 0x8241AA70; continue 'dispatch;
	}
	// 8241AA60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AA64: 386BF9B8  addi r3, r11, -0x648
	ctx.r[3].s64 = ctx.r[11].s64 + -1608;
	// 8241AA68: 48006831  bl 0x82421298
	ctx.lr = 0x8241AA6C;
	sub_82421298(ctx, base);
	// 8241AA6C: 48000010  b 0x8241aa7c
	pc = 0x8241AA7C; continue 'dispatch;
	// 8241AA70: 397E0021  addi r11, r30, 0x21
	ctx.r[11].s64 = ctx.r[30].s64 + 33;
	// 8241AA74: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241AA78: 7FEBFAAE  lhax r31, r11, r31
	ctx.r[31].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as i16) as i64;
	// 8241AA7C: 480065F5  bl 0x82421070
	ctx.lr = 0x8241AA80;
	sub_82421070(ctx, base);
	// 8241AA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241AA84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AA90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241AA94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AAA0 size=136
    let mut pc: u32 = 0x8241AAA0;
    'dispatch: loop {
        match pc {
            0x8241AAA0 => {
    //   block [0x8241AAA0..0x8241AB28)
	// 8241AAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AAA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241AAAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AAB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AAB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AAB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AABC: 48006575  bl 0x82421030
	ctx.lr = 0x8241AAC0;
	sub_82421030(ctx, base);
	// 8241AAC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AAC4: 409A0014  bne cr6, 0x8241aad8
	if !ctx.cr[6].eq {
	pc = 0x8241AAD8; continue 'dispatch;
	}
	// 8241AAC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AACC: 386BF9E4  addi r3, r11, -0x61c
	ctx.r[3].s64 = ctx.r[11].s64 + -1564;
	// 8241AAD0: 480067C9  bl 0x82421298
	ctx.lr = 0x8241AAD4;
	sub_82421298(ctx, base);
	// 8241AAD4: 48000038  b 0x8241ab0c
	pc = 0x8241AB0C; continue 'dispatch;
	// 8241AAD8: 895F00A9  lbz r10, 0xa9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(169 as u32) ) } as u64;
	// 8241AADC: B3DF0040  sth r30, 0x40(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u16 ) };
	// 8241AAE0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8241AAE4: 409A0014  bne cr6, 0x8241aaf8
	if !ctx.cr[6].eq {
	pc = 0x8241AAF8; continue 'dispatch;
	}
	// 8241AAE8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241AAEC: 4800BD45  bl 0x82426830
	ctx.lr = 0x8241AAF0;
	sub_82426830(ctx, base);
	// 8241AAF0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8241AAF4: 48000008  b 0x8241aafc
	pc = 0x8241AAFC; continue 'dispatch;
	// 8241AAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241AAFC: A95F0040  lha r10, 0x40(r31)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as i16) as i64;
	// 8241AB00: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241AB04: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241AB08: 48009FB1  bl 0x82424ab8
	ctx.lr = 0x8241AB0C;
	sub_82424AB8(ctx, base);
	// 8241AB0C: 48006565  bl 0x82421070
	ctx.lr = 0x8241AB10;
	sub_82421070(ctx, base);
	// 8241AB10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AB1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241AB20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AB28 size=80
    let mut pc: u32 = 0x8241AB28;
    'dispatch: loop {
        match pc {
            0x8241AB28 => {
    //   block [0x8241AB28..0x8241AB78)
	// 8241AB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AB38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AB3C: 480064F5  bl 0x82421030
	ctx.lr = 0x8241AB40;
	sub_82421030(ctx, base);
	// 8241AB40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AB44: 409A0014  bne cr6, 0x8241ab58
	if !ctx.cr[6].eq {
	pc = 0x8241AB58; continue 'dispatch;
	}
	// 8241AB48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AB4C: 386BFA10  addi r3, r11, -0x5f0
	ctx.r[3].s64 = ctx.r[11].s64 + -1520;
	// 8241AB50: 48006749  bl 0x82421298
	ctx.lr = 0x8241AB54;
	sub_82421298(ctx, base);
	// 8241AB54: 48000008  b 0x8241ab5c
	pc = 0x8241AB5C; continue 'dispatch;
	// 8241AB58: ABFF0040  lha r31, 0x40(r31)
	ctx.r[31].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as i16) as i64;
	// 8241AB5C: 48006515  bl 0x82421070
	ctx.lr = 0x8241AB60;
	sub_82421070(ctx, base);
	// 8241AB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241AB64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241AB68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AB6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AB70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AB78 size=64
    let mut pc: u32 = 0x8241AB78;
    'dispatch: loop {
        match pc {
            0x8241AB78 => {
    //   block [0x8241AB78..0x8241ABB8)
	// 8241AB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AB80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AB84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AB88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AB8C: 480064A5  bl 0x82421030
	ctx.lr = 0x8241AB90;
	sub_82421030(ctx, base);
	// 8241AB90: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241AB94: 93EBF3C8  stw r31, -0xc38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3128 as u32), ctx.r[31].u32 ) };
	// 8241AB98: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241AB9C: 93EBF3CC  stw r31, -0xc34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3124 as u32), ctx.r[31].u32 ) };
	// 8241ABA0: 480064D1  bl 0x82421070
	ctx.lr = 0x8241ABA4;
	sub_82421070(ctx, base);
	// 8241ABA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241ABA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241ABAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241ABB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241ABB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241ABB8 size=96
    let mut pc: u32 = 0x8241ABB8;
    'dispatch: loop {
        match pc {
            0x8241ABB8 => {
    //   block [0x8241ABB8..0x8241AC18)
	// 8241ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241ABBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241ABC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241ABC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241ABC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241ABCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241ABD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8241ABD4: 4800645D  bl 0x82421030
	ctx.lr = 0x8241ABD8;
	sub_82421030(ctx, base);
	// 8241ABD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8241ABDC: 409A0014  bne cr6, 0x8241abf0
	if !ctx.cr[6].eq {
	pc = 0x8241ABF0; continue 'dispatch;
	}
	// 8241ABE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241ABE4: 386BFA3C  addi r3, r11, -0x5c4
	ctx.r[3].s64 = ctx.r[11].s64 + -1476;
	// 8241ABE8: 480066B1  bl 0x82421298
	ctx.lr = 0x8241ABEC;
	sub_82421298(ctx, base);
	// 8241ABEC: 48000010  b 0x8241abfc
	pc = 0x8241ABFC; continue 'dispatch;
	// 8241ABF0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241ABF4: 93FE0038  stw r31, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 8241ABF8: 93EBF3CC  stw r31, -0xc34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3124 as u32), ctx.r[31].u32 ) };
	// 8241ABFC: 48006475  bl 0x82421070
	ctx.lr = 0x8241AC00;
	sub_82421070(ctx, base);
	// 8241AC00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AC04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AC08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AC0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241AC10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AC18 size=124
    let mut pc: u32 = 0x8241AC18;
    'dispatch: loop {
        match pc {
            0x8241AC18 => {
    //   block [0x8241AC18..0x8241AC94)
	// 8241AC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AC20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241AC24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AC28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AC2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AC30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AC34: 480063FD  bl 0x82421030
	ctx.lr = 0x8241AC38;
	sub_82421030(ctx, base);
	// 8241AC38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AC3C: 409A0014  bne cr6, 0x8241ac50
	if !ctx.cr[6].eq {
	pc = 0x8241AC50; continue 'dispatch;
	}
	// 8241AC40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AC44: 386BFA68  addi r3, r11, -0x598
	ctx.r[3].s64 = ctx.r[11].s64 + -1432;
	// 8241AC48: 48006651  bl 0x82421298
	ctx.lr = 0x8241AC4C;
	sub_82421298(ctx, base);
	// 8241AC4C: 4800002C  b 0x8241ac78
	pc = 0x8241AC78; continue 'dispatch;
	// 8241AC50: 7FCB0734  extsh r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	// 8241AC54: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241AC58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241AC5C: B17F003E  sth r11, 0x3e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[11].u16 ) };
	// 8241AC60: 41820018  beq 0x8241ac78
	if ctx.cr[0].eq {
	pc = 0x8241AC78; continue 'dispatch;
	}
	// 8241AC64: A95F003C  lha r10, 0x3c(r31)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as i16) as i64;
	// 8241AC68: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8241AC6C: 55645828  slwi r4, r11, 0xb
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8241AC70: 55455828  slwi r5, r10, 0xb
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8241AC74: 4800740D  bl 0x82422080
	ctx.lr = 0x8241AC78;
	sub_82422080(ctx, base);
	// 8241AC78: 480063F9  bl 0x82421070
	ctx.lr = 0x8241AC7C;
	sub_82421070(ctx, base);
	// 8241AC7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AC88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241AC8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AC98 size=64
    let mut pc: u32 = 0x8241AC98;
    'dispatch: loop {
        match pc {
            0x8241AC98 => {
    //   block [0x8241AC98..0x8241ACD8)
	// 8241AC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241ACA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241ACA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241ACA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241ACAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241ACB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241ACB4: 4800637D  bl 0x82421030
	ctx.lr = 0x8241ACB8;
	sub_82421030(ctx, base);
	// 8241ACB8: 9BDF006D  stb r30, 0x6d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(109 as u32), ctx.r[30].u8 ) };
	// 8241ACBC: 480063B5  bl 0x82421070
	ctx.lr = 0x8241ACC0;
	sub_82421070(ctx, base);
	// 8241ACC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241ACC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241ACC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241ACCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241ACD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241ACD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241ACD8 size=40
    let mut pc: u32 = 0x8241ACD8;
    'dispatch: loop {
        match pc {
            0x8241ACD8 => {
    //   block [0x8241ACD8..0x8241AD00)
	// 8241ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241ACE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241ACE4: 4800634D  bl 0x82421030
	ctx.lr = 0x8241ACE8;
	sub_82421030(ctx, base);
	// 8241ACE8: 4BFFF621  bl 0x8241a308
	ctx.lr = 0x8241ACEC;
	sub_8241A308(ctx, base);
	// 8241ACEC: 48006385  bl 0x82421070
	ctx.lr = 0x8241ACF0;
	sub_82421070(ctx, base);
	// 8241ACF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241ACF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241ACF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241ACFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AD00 size=84
    let mut pc: u32 = 0x8241AD00;
    'dispatch: loop {
        match pc {
            0x8241AD00 => {
    //   block [0x8241AD00..0x8241AD54)
	// 8241AD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AD08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AD0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AD10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AD14: 4800631D  bl 0x82421030
	ctx.lr = 0x8241AD18;
	sub_82421030(ctx, base);
	// 8241AD18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AD1C: 409A0018  bne cr6, 0x8241ad34
	if !ctx.cr[6].eq {
	pc = 0x8241AD34; continue 'dispatch;
	}
	// 8241AD20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AD24: 386BFA98  addi r3, r11, -0x568
	ctx.r[3].s64 = ctx.r[11].s64 + -1384;
	// 8241AD28: 48006571  bl 0x82421298
	ctx.lr = 0x8241AD2C;
	sub_82421298(ctx, base);
	// 8241AD2C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241AD30: 48000008  b 0x8241ad38
	pc = 0x8241AD38; continue 'dispatch;
	// 8241AD34: ABFF0060  lha r31, 0x60(r31)
	ctx.r[31].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as i16) as i64;
	// 8241AD38: 48006339  bl 0x82421070
	ctx.lr = 0x8241AD3C;
	sub_82421070(ctx, base);
	// 8241AD3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241AD40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241AD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AD4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241AD58 size=244
    let mut pc: u32 = 0x8241AD58;
    'dispatch: loop {
        match pc {
            0x8241AD58 => {
    //   block [0x8241AD58..0x8241AE4C)
	// 8241AD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AD5C: 4811A35D  bl 0x825350b8
	ctx.lr = 0x8241AD60;
	sub_82535080(ctx, base);
	// 8241AD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AD64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AD68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8241AD6C: 480062C5  bl 0x82421030
	ctx.lr = 0x8241AD70;
	sub_82421030(ctx, base);
	// 8241AD70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AD74: 409A0014  bne cr6, 0x8241ad88
	if !ctx.cr[6].eq {
	pc = 0x8241AD88; continue 'dispatch;
	}
	// 8241AD78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AD7C: 386BFAC4  addi r3, r11, -0x53c
	ctx.r[3].s64 = ctx.r[11].s64 + -1340;
	// 8241AD80: 48006519  bl 0x82421298
	ctx.lr = 0x8241AD84;
	sub_82421298(ctx, base);
	// 8241AD84: 480000BC  b 0x8241ae40
	pc = 0x8241AE40; continue 'dispatch;
	// 8241AD88: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241AD8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241AD90: 4082000C  bne 0x8241ad9c
	if !ctx.cr[0].eq {
	pc = 0x8241AD9C; continue 'dispatch;
	}
	// 8241AD94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241AD98: 4800001C  b 0x8241adb4
	pc = 0x8241ADB4; continue 'dispatch;
	// 8241AD9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241ADA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241ADA4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8241ADA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241ADAC: 4E800421  bctrl
	ctx.lr = 0x8241ADB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241ADB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241ADB4: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8241ADB8: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8241ADBC: 419A0080  beq cr6, 0x8241ae3c
	if ctx.cr[6].eq {
	pc = 0x8241AE3C; continue 'dispatch;
	}
	// 8241ADC0: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8241ADC4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8241ADC8: 409A0074  bne cr6, 0x8241ae3c
	if !ctx.cr[6].eq {
	pc = 0x8241AE3C; continue 'dispatch;
	}
	// 8241ADCC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8241ADD0: 409A006C  bne cr6, 0x8241ae3c
	if !ctx.cr[6].eq {
	pc = 0x8241AE3C; continue 'dispatch;
	}
	// 8241ADD4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241ADD8: 48013429  bl 0x8242e200
	ctx.lr = 0x8241ADDC;
	sub_8242E200(ctx, base);
	// 8241ADDC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8241ADE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241ADE4: 7FABF214  add r29, r11, r30
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8241ADE8: 4800BA21  bl 0x82426808
	ctx.lr = 0x8241ADEC;
	sub_82426808(ctx, base);
	// 8241ADEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8241ADF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241ADF4: 396B07FF  addi r11, r11, 0x7ff
	ctx.r[11].s64 = ctx.r[11].s64 + 2047;
	// 8241ADF8: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8241ADFC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8241AE00: 557E5828  slwi r30, r11, 0xb
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8241AE04: 4800BA25  bl 0x82426828
	ctx.lr = 0x8241AE08;
	sub_82426828(ctx, base);
	// 8241AE08: 396307FF  addi r11, r3, 0x7ff
	ctx.r[11].s64 = ctx.r[3].s64 + 2047;
	// 8241AE0C: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8241AE10: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8241AE14: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241AE18: 7D7E5851  subf. r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241AE1C: 4181000C  bgt 0x8241ae28
	if ctx.cr[0].gt {
	pc = 0x8241AE28; continue 'dispatch;
	}
	// 8241AE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241AE24: 48000014  b 0x8241ae38
	pc = 0x8241AE38; continue 'dispatch;
	// 8241AE28: 7D5EE850  subf r10, r30, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8241AE2C: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8241AE30: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8241AE34: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8241AE38: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8241AE3C: 9B9F006C  stb r28, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[28].u8 ) };
	// 8241AE40: 48006231  bl 0x82421070
	ctx.lr = 0x8241AE44;
	sub_82421070(ctx, base);
	// 8241AE44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241AE48: 4811A2C0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AE50 size=88
    let mut pc: u32 = 0x8241AE50;
    'dispatch: loop {
        match pc {
            0x8241AE50 => {
    //   block [0x8241AE50..0x8241AEA8)
	// 8241AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AE58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AE5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AE60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AE64: 480061CD  bl 0x82421030
	ctx.lr = 0x8241AE68;
	sub_82421030(ctx, base);
	// 8241AE68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AE6C: 409A0018  bne cr6, 0x8241ae84
	if !ctx.cr[6].eq {
	pc = 0x8241AE84; continue 'dispatch;
	}
	// 8241AE70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AE74: 386BFAF0  addi r3, r11, -0x510
	ctx.r[3].s64 = ctx.r[11].s64 + -1296;
	// 8241AE78: 48006421  bl 0x82421298
	ctx.lr = 0x8241AE7C;
	sub_82421298(ctx, base);
	// 8241AE7C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241AE80: 4800000C  b 0x8241ae8c
	pc = 0x8241AE8C; continue 'dispatch;
	// 8241AE84: 897F0071  lbz r11, 0x71(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(113 as u32) ) } as u64;
	// 8241AE88: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 8241AE8C: 480061E5  bl 0x82421070
	ctx.lr = 0x8241AE90;
	sub_82421070(ctx, base);
	// 8241AE90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241AE94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241AE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AEA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AEA8 size=72
    let mut pc: u32 = 0x8241AEA8;
    'dispatch: loop {
        match pc {
            0x8241AEA8 => {
    //   block [0x8241AEA8..0x8241AEF0)
	// 8241AEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AEB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241AEB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AEB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AEBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AEC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AEC4: 4800616D  bl 0x82421030
	ctx.lr = 0x8241AEC8;
	sub_82421030(ctx, base);
	// 8241AEC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241AECC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241AED0: 48009DB1  bl 0x82424c80
	ctx.lr = 0x8241AED4;
	sub_82424C80(ctx, base);
	// 8241AED4: 4800619D  bl 0x82421070
	ctx.lr = 0x8241AED8;
	sub_82421070(ctx, base);
	// 8241AED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AEE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241AEE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AEF0 size=80
    let mut pc: u32 = 0x8241AEF0;
    'dispatch: loop {
        match pc {
            0x8241AEF0 => {
    //   block [0x8241AEF0..0x8241AF40)
	// 8241AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AEF4: 4811A1C9  bl 0x825350bc
	ctx.lr = 0x8241AEF8;
	sub_82535080(ctx, base);
	// 8241AEF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AEFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AF00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AF04: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241AF08: 48006129  bl 0x82421030
	ctx.lr = 0x8241AF0C;
	sub_82421030(ctx, base);
	// 8241AF0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AF10: 409A0014  bne cr6, 0x8241af24
	if !ctx.cr[6].eq {
	pc = 0x8241AF24; continue 'dispatch;
	}
	// 8241AF14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AF18: 386BFB24  addi r3, r11, -0x4dc
	ctx.r[3].s64 = ctx.r[11].s64 + -1244;
	// 8241AF1C: 4800637D  bl 0x82421298
	ctx.lr = 0x8241AF20;
	sub_82421298(ctx, base);
	// 8241AF20: 48000014  b 0x8241af34
	pc = 0x8241AF34; continue 'dispatch;
	// 8241AF24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241AF28: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241AF2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241AF30: 48009AD1  bl 0x82424a00
	ctx.lr = 0x8241AF34;
	sub_82424A00(ctx, base);
	// 8241AF34: 4800613D  bl 0x82421070
	ctx.lr = 0x8241AF38;
	sub_82421070(ctx, base);
	// 8241AF38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AF3C: 4811A1D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AF40 size=80
    let mut pc: u32 = 0x8241AF40;
    'dispatch: loop {
        match pc {
            0x8241AF40 => {
    //   block [0x8241AF40..0x8241AF90)
	// 8241AF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AF44: 4811A179  bl 0x825350bc
	ctx.lr = 0x8241AF48;
	sub_82535080(ctx, base);
	// 8241AF48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AF4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AF50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AF54: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241AF58: 480060D9  bl 0x82421030
	ctx.lr = 0x8241AF5C;
	sub_82421030(ctx, base);
	// 8241AF5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241AF60: 409A0014  bne cr6, 0x8241af74
	if !ctx.cr[6].eq {
	pc = 0x8241AF74; continue 'dispatch;
	}
	// 8241AF64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241AF68: 386BFB4C  addi r3, r11, -0x4b4
	ctx.r[3].s64 = ctx.r[11].s64 + -1204;
	// 8241AF6C: 4800632D  bl 0x82421298
	ctx.lr = 0x8241AF70;
	sub_82421298(ctx, base);
	// 8241AF70: 48000014  b 0x8241af84
	pc = 0x8241AF84; continue 'dispatch;
	// 8241AF74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241AF78: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241AF7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241AF80: 48009B01  bl 0x82424a80
	ctx.lr = 0x8241AF84;
	sub_82424A80(ctx, base);
	// 8241AF84: 480060ED  bl 0x82421070
	ctx.lr = 0x8241AF88;
	sub_82421070(ctx, base);
	// 8241AF88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AF8C: 4811A180  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AF90 size=56
    let mut pc: u32 = 0x8241AF90;
    'dispatch: loop {
        match pc {
            0x8241AF90 => {
    //   block [0x8241AF90..0x8241AFC8)
	// 8241AF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AF98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AF9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AFA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AFA4: 4800608D  bl 0x82421030
	ctx.lr = 0x8241AFA8;
	sub_82421030(ctx, base);
	// 8241AFA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241AFAC: 4800B18D  bl 0x82426138
	ctx.lr = 0x8241AFB0;
	sub_82426138(ctx, base);
	// 8241AFB0: 480060C1  bl 0x82421070
	ctx.lr = 0x8241AFB4;
	sub_82421070(ctx, base);
	// 8241AFB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241AFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241AFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241AFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241AFC8 size=64
    let mut pc: u32 = 0x8241AFC8;
    'dispatch: loop {
        match pc {
            0x8241AFC8 => {
    //   block [0x8241AFC8..0x8241B008)
	// 8241AFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241AFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241AFD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241AFD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241AFD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241AFDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241AFE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241AFE4: 4800604D  bl 0x82421030
	ctx.lr = 0x8241AFE8;
	sub_82421030(ctx, base);
	// 8241AFE8: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 8241AFEC: 48006085  bl 0x82421070
	ctx.lr = 0x8241AFF0;
	sub_82421070(ctx, base);
	// 8241AFF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241AFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241AFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241AFFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241B000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B008 size=84
    let mut pc: u32 = 0x8241B008;
    'dispatch: loop {
        match pc {
            0x8241B008 => {
    //   block [0x8241B008..0x8241B05C)
	// 8241B008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B010: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241B014: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B018: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B01C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B020: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B024: 4800600D  bl 0x82421030
	ctx.lr = 0x8241B028;
	sub_82421030(ctx, base);
	// 8241B028: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241B02C: 9BDF0098  stb r30, 0x98(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u8 ) };
	// 8241B030: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B034: 4182000C  beq 0x8241b040
	if ctx.cr[0].eq {
	pc = 0x8241B040; continue 'dispatch;
	}
	// 8241B038: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B03C: 4800B745  bl 0x82426780
	ctx.lr = 0x8241B040;
	sub_82426780(ctx, base);
	// 8241B040: 48006031  bl 0x82421070
	ctx.lr = 0x8241B044;
	sub_82421070(ctx, base);
	// 8241B044: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B050: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241B054: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B060 size=56
    let mut pc: u32 = 0x8241B060;
    'dispatch: loop {
        match pc {
            0x8241B060 => {
    //   block [0x8241B060..0x8241B098)
	// 8241B060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B064: 4811A059  bl 0x825350bc
	ctx.lr = 0x8241B068;
	sub_82535080(ctx, base);
	// 8241B068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B074: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241B078: 48005FB9  bl 0x82421030
	ctx.lr = 0x8241B07C;
	sub_82421030(ctx, base);
	// 8241B07C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241B080: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241B084: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B088: 4800B709  bl 0x82426790
	ctx.lr = 0x8241B08C;
	sub_82426790(ctx, base);
	// 8241B08C: 48005FE5  bl 0x82421070
	ctx.lr = 0x8241B090;
	sub_82421070(ctx, base);
	// 8241B090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B094: 4811A078  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B098 size=64
    let mut pc: u32 = 0x8241B098;
    'dispatch: loop {
        match pc {
            0x8241B098 => {
    //   block [0x8241B098..0x8241B0D8)
	// 8241B098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B09C: 4811A021  bl 0x825350bc
	ctx.lr = 0x8241B0A0;
	sub_82535080(ctx, base);
	// 8241B0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B0A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B0A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B0AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241B0B0: 48005F81  bl 0x82421030
	ctx.lr = 0x8241B0B4;
	sub_82421030(ctx, base);
	// 8241B0B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241B0B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B0BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B0C0: 4BFFF411  bl 0x8241a4d0
	ctx.lr = 0x8241B0C4;
	sub_8241A4D0(ctx, base);
	// 8241B0C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B0C8: 48005FA9  bl 0x82421070
	ctx.lr = 0x8241B0CC;
	sub_82421070(ctx, base);
	// 8241B0CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B0D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B0D4: 4811A038  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B0D8 size=96
    let mut pc: u32 = 0x8241B0D8;
    'dispatch: loop {
        match pc {
            0x8241B0D8 => {
    //   block [0x8241B0D8..0x8241B138)
	// 8241B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B0E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241B0E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B0E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B0EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B0F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B0F4: 48005F3D  bl 0x82421030
	ctx.lr = 0x8241B0F8;
	sub_82421030(ctx, base);
	// 8241B0F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241B0FC: 409A0014  bne cr6, 0x8241b110
	if !ctx.cr[6].eq {
	pc = 0x8241B110; continue 'dispatch;
	}
	// 8241B100: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B104: 386BFB74  addi r3, r11, -0x48c
	ctx.r[3].s64 = ctx.r[11].s64 + -1164;
	// 8241B108: 48006191  bl 0x82421298
	ctx.lr = 0x8241B10C;
	sub_82421298(ctx, base);
	// 8241B10C: 48000010  b 0x8241b11c
	pc = 0x8241B11C; continue 'dispatch;
	// 8241B110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B114: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241B118: 4800B671  bl 0x82426788
	ctx.lr = 0x8241B11C;
	sub_82426788(ctx, base);
	// 8241B11C: 48005F55  bl 0x82421070
	ctx.lr = 0x8241B120;
	sub_82421070(ctx, base);
	// 8241B120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B12C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241B130: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241B138 size=720
    let mut pc: u32 = 0x8241B138;
    'dispatch: loop {
        match pc {
            0x8241B138 => {
    //   block [0x8241B138..0x8241B408)
	// 8241B138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B13C: 48119F79  bl 0x825350b4
	ctx.lr = 0x8241B140;
	sub_82535080(ctx, base);
	// 8241B140: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B144: 3964003F  addi r11, r4, 0x3f
	ctx.r[11].s64 = ctx.r[4].s64 + 63;
	// 8241B148: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241B14C: 557D0032  rlwinm r29, r11, 0, 0, 0x19
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8241B150: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241B154: 7D7D2050  subf r11, r29, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[29].s64;
	// 8241B158: 7F8B2A14  add r28, r11, r5
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8241B15C: 41980294  blt cr6, 0x8241b3f0
	if ctx.cr[6].lt {
	pc = 0x8241B3F0; continue 'dispatch;
	}
	// 8241B160: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8241B164: 419A028C  beq cr6, 0x8241b3f0
	if ctx.cr[6].eq {
	pc = 0x8241B3F0; continue 'dispatch;
	}
	// 8241B168: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8241B16C: 41980284  blt cr6, 0x8241b3f0
	if ctx.cr[6].lt {
	pc = 0x8241B3F0; continue 'dispatch;
	}
	// 8241B170: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8241B174: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8241B178: 394BDF80  addi r10, r11, -0x2080
	ctx.r[10].s64 = ctx.r[11].s64 + -8320;
	// 8241B17C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8241B180: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8241B184: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241B188: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8241B18C: 419A0018  beq cr6, 0x8241b1a4
	if ctx.cr[6].eq {
	pc = 0x8241B1A4; continue 'dispatch;
	}
	// 8241B190: 392900C4  addi r9, r9, 0xc4
	ctx.r[9].s64 = ctx.r[9].s64 + 196;
	// 8241B194: 390A1880  addi r8, r10, 0x1880
	ctx.r[8].s64 = ctx.r[10].s64 + 6272;
	// 8241B198: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241B19C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8241B1A0: 4198FFE4  blt cr6, 0x8241b184
	if ctx.cr[6].lt {
	pc = 0x8241B184; continue 'dispatch;
	}
	// 8241B1A4: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8241B1A8: 409A0010  bne cr6, 0x8241b1b8
	if !ctx.cr[6].eq {
	pc = 0x8241B1B8; continue 'dispatch;
	}
	// 8241B1AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B1B0: 386BFC24  addi r3, r11, -0x3dc
	ctx.r[3].s64 = ctx.r[11].s64 + -988;
	// 8241B1B4: 48000244  b 0x8241b3f8
	pc = 0x8241B3F8; continue 'dispatch;
	// 8241B1B8: 1D6B00C4  mulli r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 * 196;
	// 8241B1BC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241B1C0: 38A000C4  li r5, 0xc4
	ctx.r[5].s64 = 196;
	// 8241B1C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241B1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B1CC: 4811A005  bl 0x825351d0
	ctx.lr = 0x8241B1D0;
	sub_825351D0(ctx, base);
	// 8241B1D0: 1D7E40C0  mulli r11, r30, 0x40c0
	ctx.r[11].s64 = ctx.r[30].s64 * 16576;
	// 8241B1D4: 9BDF0003  stb r30, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[30].u8 ) };
	// 8241B1D8: 7D4BE050  subf r10, r11, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 8241B1DC: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8241B1E0: 396AFEDC  addi r11, r10, -0x124
	ctx.r[11].s64 = ctx.r[10].s64 + -292;
	// 8241B1E4: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8241B1E8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8241B1EC: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 8241B1F0: 55645829  rlwinm. r4, r11, 0xb, 0, 0x14
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8241B1F4: 909F0024  stw r4, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 8241B1F8: 40800010  bge 0x8241b208
	if !ctx.cr[0].lt {
	pc = 0x8241B208; continue 'dispatch;
	}
	// 8241B1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B200: 386BFBF4  addi r3, r11, -0x40c
	ctx.r[3].s64 = ctx.r[11].s64 + -1036;
	// 8241B204: 480001F4  b 0x8241b3f8
	pc = 0x8241B3F8; continue 'dispatch;
	// 8241B208: 7D641A14  add r11, r4, r3
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 8241B20C: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 8241B210: 39400024  li r10, 0x24
	ctx.r[10].s64 = 36;
	// 8241B214: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8241B218: 396B0024  addi r11, r11, 0x24
	ctx.r[11].s64 = ctx.r[11].s64 + 36;
	// 8241B21C: 39202000  li r9, 0x2000
	ctx.r[9].s64 = 8192;
	// 8241B220: 39002060  li r8, 0x2060
	ctx.r[8].s64 = 8288;
	// 8241B224: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 8241B228: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8241B22C: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 8241B230: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 8241B234: 911F0034  stw r8, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 8241B238: 48008211  bl 0x82423448
	ctx.lr = 0x8241B23C;
	sub_82423448(ctx, base);
	// 8241B23C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B240: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8241B244: 40820010  bne 0x8241b254
	if !ctx.cr[0].eq {
	pc = 0x8241B254; continue 'dispatch;
	}
	// 8241B248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B24C: 4BFFF3DD  bl 0x8241a628
	ctx.lr = 0x8241B250;
	sub_8241A628(ctx, base);
	// 8241B250: 480001AC  b 0x8241b3fc
	pc = 0x8241B3FC; continue 'dispatch;
	// 8241B254: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241B258: 48006F59  bl 0x824221b0
	ctx.lr = 0x8241B25C;
	sub_824221B0(ctx, base);
	// 8241B25C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B260: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8241B264: 4182FFE4  beq 0x8241b248
	if ctx.cr[0].eq {
	pc = 0x8241B248; continue 'dispatch;
	}
	// 8241B268: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8241B26C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241B270: 4099004C  ble cr6, 0x8241b2bc
	if !ctx.cr[6].gt {
	pc = 0x8241B2BC; continue 'dispatch;
	}
	// 8241B274: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 8241B278: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8241B27C: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8241B280: 7D2BE9D6  mullw r9, r11, r29
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8241B284: 811F002C  lwz r8, 0x2c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8241B288: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8241B28C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8241B290: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8241B294: 7C694214  add r3, r9, r8
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8241B298: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8241B29C: 480081AD  bl 0x82423448
	ctx.lr = 0x8241B2A0;
	sub_82423448(ctx, base);
	// 8241B2A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B2A4: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8241B2A8: 4182FFA0  beq 0x8241b248
	if ctx.cr[0].eq {
	pc = 0x8241B248; continue 'dispatch;
	}
	// 8241B2AC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8241B2B0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8241B2B4: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8241B2B8: 4198FFC0  blt cr6, 0x8241b278
	if ctx.cr[6].lt {
	pc = 0x8241B278; continue 'dispatch;
	}
	// 8241B2BC: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 8241B2C0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241B2C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B2C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241B2CC: 4800B64D  bl 0x82426918
	ctx.lr = 0x8241B2D0;
	sub_82426918(ctx, base);
	// 8241B2D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B2D4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8241B2D8: 4182FF70  beq 0x8241b248
	if ctx.cr[0].eq {
	pc = 0x8241B248; continue 'dispatch;
	}
	// 8241B2DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B2E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241B2E4: 4800946D  bl 0x82424750
	ctx.lr = 0x8241B2E8;
	sub_82424750(ctx, base);
	// 8241B2E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B2EC: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8241B2F0: 4182FF58  beq 0x8241b248
	if ctx.cr[0].eq {
	pc = 0x8241B248; continue 'dispatch;
	}
	// 8241B2F4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241B2F8: 4800C361  bl 0x82427658
	ctx.lr = 0x8241B2FC;
	sub_82427658(ctx, base);
	// 8241B2FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B300: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 8241B304: 4182FF44  beq 0x8241b248
	if ctx.cr[0].eq {
	pc = 0x8241B248; continue 'dispatch;
	}
	// 8241B308: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B30C: 4800C38D  bl 0x82427698
	ctx.lr = 0x8241B310;
	sub_82427698(ctx, base);
	// 8241B310: 48013C41  bl 0x8242ef50
	ctx.lr = 0x8241B314;
	sub_8242EF50(ctx, base);
	// 8241B314: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241B318: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8241B31C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241B320: B37F0040  sth r27, 0x40(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[27].u16 ) };
	// 8241B324: 814BF3C8  lwz r10, -0xc38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3128 as u32) ) } as u64;
	// 8241B328: 7D2B5E70  srawi r11, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 8241B32C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8241B330: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8241B334: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8241B338: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8241B33C: B17F003C  sth r11, 0x3c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u16 ) };
	// 8241B340: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241B344: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8241B348: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241B34C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8241B350: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8241B354: C00B2160  lfs f0, 0x2160(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8241B358: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8241B35C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8241B360: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8241B364: A1610056  lhz r11, 0x56(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8241B368: B17F003E  sth r11, 0x3e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[11].u16 ) };
	// 8241B36C: 40990024  ble cr6, 0x8241b390
	if !ctx.cr[6].gt {
	pc = 0x8241B390; continue 'dispatch;
	}
	// 8241B370: 397F0042  addi r11, r31, 0x42
	ctx.r[11].s64 = ctx.r[31].s64 + 66;
	// 8241B374: 3940FF80  li r10, -0x80
	ctx.r[10].s64 = -128;
	// 8241B378: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B37C: 41820014  beq 0x8241b390
	if ctx.cr[0].eq {
	pc = 0x8241B390; continue 'dispatch;
	}
	// 8241B380: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 8241B384: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8241B388: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8241B38C: 4200FFF8  bdnz 0x8241b384
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8241B384; continue 'dispatch;
	}
	// 8241B390: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8241B394: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241B398: B37F0046  sth r27, 0x46(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(70 as u32), ctx.r[27].u16 ) };
	// 8241B39C: 937F0054  stw r27, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8241B3A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B3A4: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8241B3A8: 937F005C  stw r27, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 8241B3AC: 9BDF006C  stb r30, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u8 ) };
	// 8241B3B0: B37F0060  sth r27, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[27].u16 ) };
	// 8241B3B4: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 8241B3B8: B37F0068  sth r27, 0x68(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[27].u16 ) };
	// 8241B3BC: B37F006A  sth r27, 0x6a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(106 as u32), ctx.r[27].u16 ) };
	// 8241B3C0: 9BDF006D  stb r30, 0x6d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(109 as u32), ctx.r[30].u8 ) };
	// 8241B3C4: 9B7F0072  stb r27, 0x72(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(114 as u32), ctx.r[27].u8 ) };
	// 8241B3C8: 937F0088  stw r27, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 8241B3CC: 9B7F0098  stb r27, 0x98(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[27].u8 ) };
	// 8241B3D0: 4182000C  beq 0x8241b3dc
	if ctx.cr[0].eq {
	pc = 0x8241B3DC; continue 'dispatch;
	}
	// 8241B3D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241B3D8: 4800B3A9  bl 0x82426780
	ctx.lr = 0x8241B3DC;
	sub_82426780(ctx, base);
	// 8241B3DC: 9BDF00A9  stb r30, 0xa9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(169 as u32), ctx.r[30].u8 ) };
	// 8241B3E0: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 8241B3E4: 48013B6D  bl 0x8242ef50
	ctx.lr = 0x8241B3E8;
	sub_8242EF50(ctx, base);
	// 8241B3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B3EC: 48000014  b 0x8241b400
	pc = 0x8241B400; continue 'dispatch;
	// 8241B3F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B3F4: 386BFBCC  addi r3, r11, -0x434
	ctx.r[3].s64 = ctx.r[11].s64 + -1076;
	// 8241B3F8: 48005EA1  bl 0x82421298
	ctx.lr = 0x8241B3FC;
	sub_82421298(ctx, base);
	// 8241B3FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241B400: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241B404: 48119D00  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B408 size=56
    let mut pc: u32 = 0x8241B408;
    'dispatch: loop {
        match pc {
            0x8241B408 => {
    //   block [0x8241B408..0x8241B440)
	// 8241B408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B410: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B414: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B418: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B41C: 48005C15  bl 0x82421030
	ctx.lr = 0x8241B420;
	sub_82421030(ctx, base);
	// 8241B420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B424: 4BFFF205  bl 0x8241a628
	ctx.lr = 0x8241B428;
	sub_8241A628(ctx, base);
	// 8241B428: 48005C49  bl 0x82421070
	ctx.lr = 0x8241B42C;
	sub_82421070(ctx, base);
	// 8241B42C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241B430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B440 size=100
    let mut pc: u32 = 0x8241B440;
    'dispatch: loop {
        match pc {
            0x8241B440 => {
    //   block [0x8241B440..0x8241B4A4)
	// 8241B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241B44C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B454: 48005BDD  bl 0x82421030
	ctx.lr = 0x8241B458;
	sub_82421030(ctx, base);
	// 8241B458: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8241B45C: 3BCBDF80  addi r30, r11, -0x2080
	ctx.r[30].s64 = ctx.r[11].s64 + -8320;
	// 8241B460: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8241B464: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241B468: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8241B46C: 409A000C  bne cr6, 0x8241b478
	if !ctx.cr[6].eq {
	pc = 0x8241B478; continue 'dispatch;
	}
	// 8241B470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B474: 4BFFF1B5  bl 0x8241a628
	ctx.lr = 0x8241B478;
	sub_8241A628(ctx, base);
	// 8241B478: 3BFF00C4  addi r31, r31, 0xc4
	ctx.r[31].s64 = ctx.r[31].s64 + 196;
	// 8241B47C: 397E1880  addi r11, r30, 0x1880
	ctx.r[11].s64 = ctx.r[30].s64 + 6272;
	// 8241B480: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241B484: 4198FFE0  blt cr6, 0x8241b464
	if ctx.cr[6].lt {
	pc = 0x8241B464; continue 'dispatch;
	}
	// 8241B488: 48005BE9  bl 0x82421070
	ctx.lr = 0x8241B48C;
	sub_82421070(ctx, base);
	// 8241B48C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B498: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241B49C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B4A8 size=220
    let mut pc: u32 = 0x8241B4A8;
    'dispatch: loop {
        match pc {
            0x8241B4A8 => {
    //   block [0x8241B4A8..0x8241B584)
	// 8241B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B4AC: 48119C09  bl 0x825350b4
	ctx.lr = 0x8241B4B0;
	sub_82535080(ctx, base);
	// 8241B4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B4B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241B4BC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241B4C0: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8241B4C4: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241B4C8: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241B4CC: 40810034  ble 0x8241b500
	if !ctx.cr[0].gt {
	pc = 0x8241B500; continue 'dispatch;
	}
	// 8241B4D0: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 8241B4D4: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241B4D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241B4DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241B4E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241B4E4: 4E800421  bctrl
	ctx.lr = 0x8241B4E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241B4E8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241B4EC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8241B4F0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241B4F4: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8241B4F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241B4FC: 4198FFD8  blt cr6, 0x8241b4d4
	if ctx.cr[6].lt {
	pc = 0x8241B4D4; continue 'dispatch;
	}
	// 8241B500: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8241B504: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241B508: 4800AC11  bl 0x82426118
	ctx.lr = 0x8241B50C;
	sub_82426118(ctx, base);
	// 8241B50C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241B510: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8241B514: 4800AC2D  bl 0x82426140
	ctx.lr = 0x8241B518;
	sub_82426140(ctx, base);
	// 8241B518: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241B51C: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 8241B520: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 8241B524: 9BDF0071  stb r30, 0x71(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(113 as u32), ctx.r[30].u8 ) };
	// 8241B528: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8241B52C: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 8241B530: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 8241B534: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8241B538: 895F0002  lbz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8241B53C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 8241B540: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241B544: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 8241B548: 911F0090  stw r8, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 8241B54C: 913F008C  stw r9, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 8241B550: 816BF40C  lwz r11, -0xbf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3060 as u32) ) } as u64;
	// 8241B554: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 8241B558: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8241B55C: 409A0010  bne cr6, 0x8241b56c
	if !ctx.cr[6].eq {
	pc = 0x8241B56C; continue 'dispatch;
	}
	// 8241B560: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241B564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B568: 4BFFF7F1  bl 0x8241ad58
	ctx.lr = 0x8241B56C;
	sub_8241AD58(ctx, base);
	// 8241B56C: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8241B570: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B574: 41820008  beq 0x8241b57c
	if ctx.cr[0].eq {
	pc = 0x8241B57C; continue 'dispatch;
	}
	// 8241B578: 4800BCA1  bl 0x82427218
	ctx.lr = 0x8241B57C;
	sub_82427218(ctx, base);
	// 8241B57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241B580: 48119B84  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B588 size=172
    let mut pc: u32 = 0x8241B588;
    'dispatch: loop {
        match pc {
            0x8241B588 => {
    //   block [0x8241B588..0x8241B634)
	// 8241B588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B58C: 48119B29  bl 0x825350b4
	ctx.lr = 0x8241B590;
	sub_82535080(ctx, base);
	// 8241B590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B59C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241B5A0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8241B5A4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8241B5A8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B5AC: A97F003C  lha r11, 0x3c(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as i16) as i64;
	// 8241B5B0: A95F003E  lha r10, 0x3e(r31)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(62 as u32) ) } as i16) as i64;
	// 8241B5B4: 55655828  slwi r5, r11, 0xb
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8241B5B8: 55445828  slwi r4, r10, 0xb
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8241B5BC: 48006AC5  bl 0x82422080
	ctx.lr = 0x8241B5C0;
	sub_82422080(ctx, base);
	// 8241B5C0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8241B5C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B5C8: 808B3BF8  lwz r4, 0x3bf8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15352 as u32) ) } as u64;
	// 8241B5CC: 480063CD  bl 0x82421998
	ctx.lr = 0x8241B5D0;
	sub_82421998(ctx, base);
	// 8241B5D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241B5D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B5D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241B5DC: 4800638D  bl 0x82421968
	ctx.lr = 0x8241B5E0;
	sub_82421968(ctx, base);
	// 8241B5E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241B5E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B5E8: 480062D9  bl 0x824218c0
	ctx.lr = 0x8241B5EC;
	sub_824218C0(ctx, base);
	// 8241B5EC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B5F0: 48006D81  bl 0x82422370
	ctx.lr = 0x8241B5F4;
	sub_82422370(ctx, base);
	// 8241B5F4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B5F8: 48006FA9  bl 0x824225a0
	ctx.lr = 0x8241B5FC;
	sub_824225A0(ctx, base);
	// 8241B5FC: 7F6B07B4  extsw r11, r27
	ctx.r[11].s64 = ctx.r[27].s32 as i64;
	// 8241B600: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8241B604: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B608: 79675D24  sldi r7, r11, 0xb
	ctx.r[7].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 8241B60C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241B610: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B614: 48006BED  bl 0x82422200
	ctx.lr = 0x8241B618;
	sub_82422200(ctx, base);
	// 8241B618: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241B61C: 48006C4D  bl 0x82422268
	ctx.lr = 0x8241B620;
	sub_82422268(ctx, base);
	// 8241B620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B624: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241B628: 4BFFFE81  bl 0x8241b4a8
	ctx.lr = 0x8241B62C;
	sub_8241B4A8(ctx, base);
	// 8241B62C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241B630: 48119AD4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241B638 size=220
    let mut pc: u32 = 0x8241B638;
    'dispatch: loop {
        match pc {
            0x8241B638 => {
    //   block [0x8241B638..0x8241B714)
	// 8241B638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B63C: 48119A7D  bl 0x825350b8
	ctx.lr = 0x8241B640;
	sub_82535080(ctx, base);
	// 8241B640: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B648: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B64C: 480059E5  bl 0x82421030
	ctx.lr = 0x8241B650;
	sub_82421030(ctx, base);
	// 8241B650: 897F0072  lbz r11, 0x72(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(114 as u32) ) } as u64;
	// 8241B654: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241B658: 409A000C  bne cr6, 0x8241b664
	if !ctx.cr[6].eq {
	pc = 0x8241B664; continue 'dispatch;
	}
	// 8241B65C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241B660: 480000A4  b 0x8241b704
	pc = 0x8241B704; continue 'dispatch;
	// 8241B664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B668: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241B66C: 4801C09D  bl 0x82437708
	ctx.lr = 0x8241B670;
	sub_82437708(ctx, base);
	// 8241B670: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8241B674: 480059BD  bl 0x82421030
	ctx.lr = 0x8241B678;
	sub_82421030(ctx, base);
	// 8241B678: 4BFFEC91  bl 0x8241a308
	ctx.lr = 0x8241B67C;
	sub_8241A308(ctx, base);
	// 8241B67C: 480059F5  bl 0x82421070
	ctx.lr = 0x8241B680;
	sub_82421070(ctx, base);
	// 8241B680: 3FC08289  lis r30, -0x7d77
	ctx.r[30].s64 = -2104950784;
	// 8241B684: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241B688: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8241B68C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8241B690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B694: 839EF3C0  lwz r28, -0xc40(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-3136 as u32) ) } as u64;
	// 8241B698: 917EF3C0  stw r11, -0xc40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-3136 as u32), ctx.r[11].u32 ) };
	// 8241B69C: 4BFFE8BD  bl 0x82419f58
	ctx.lr = 0x8241B6A0;
	sub_82419F58(ctx, base);
	// 8241B6A0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8241B6A4: 939EF3C0  stw r28, -0xc40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-3136 as u32), ctx.r[28].u32 ) };
	// 8241B6A8: 391F009C  addi r8, r31, 0x9c
	ctx.r[8].s64 = ctx.r[31].s64 + 156;
	// 8241B6AC: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 8241B6B0: E921005A  lwa r9, 0x58(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8241B6B4: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8241B6B8: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8241B6BC: E96BF806  lwa r11, -0x7fc(r11)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2044 as u32) ) } as i32) as i64;
	// 8241B6C0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8241B6C4: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8241B6C8: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241B6CC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8241B6D0: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8241B6D4: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241B6D8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8241B6DC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8241B6E0: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8241B6E4: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8241B6E8: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8241B6EC: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8241B6F0: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8241B6F4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8241B6F8: 7C0047AE  stfiwx f0, 0, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32, tmp.u32) };
	// 8241B6FC: 816BF40C  lwz r11, -0xbf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3060 as u32) ) } as u64;
	// 8241B700: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8241B704: 4800596D  bl 0x82421070
	ctx.lr = 0x8241B708;
	sub_82421070(ctx, base);
	// 8241B708: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241B70C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241B710: 481199F8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B718 size=64
    let mut pc: u32 = 0x8241B718;
    'dispatch: loop {
        match pc {
            0x8241B718 => {
    //   block [0x8241B718..0x8241B758)
	// 8241B718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B71C: 481199A1  bl 0x825350bc
	ctx.lr = 0x8241B720;
	sub_82535080(ctx, base);
	// 8241B720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B728: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B72C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241B730: 48005901  bl 0x82421030
	ctx.lr = 0x8241B734;
	sub_82421030(ctx, base);
	// 8241B734: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241B738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B73C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B740: 4BFFF9F9  bl 0x8241b138
	ctx.lr = 0x8241B744;
	sub_8241B138(ctx, base);
	// 8241B744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B748: 48005929  bl 0x82421070
	ctx.lr = 0x8241B74C;
	sub_82421070(ctx, base);
	// 8241B74C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B754: 481199B8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B758 size=84
    let mut pc: u32 = 0x8241B758;
    'dispatch: loop {
        match pc {
            0x8241B758 => {
    //   block [0x8241B758..0x8241B7AC)
	// 8241B758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B760: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241B764: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B76C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B770: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B774: 480058BD  bl 0x82421030
	ctx.lr = 0x8241B778;
	sub_82421030(ctx, base);
	// 8241B778: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241B77C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241B780: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241B784: 4BFFF9B5  bl 0x8241b138
	ctx.lr = 0x8241B788;
	sub_8241B138(ctx, base);
	// 8241B788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B78C: 480058E5  bl 0x82421070
	ctx.lr = 0x8241B790;
	sub_82421070(ctx, base);
	// 8241B790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B794: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B7A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241B7A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B7B0 size=156
    let mut pc: u32 = 0x8241B7B0;
    'dispatch: loop {
        match pc {
            0x8241B7B0 => {
    //   block [0x8241B7B0..0x8241B84C)
	// 8241B7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B7B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241B7BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B7C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B7C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B7C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B7CC: 48005865  bl 0x82421030
	ctx.lr = 0x8241B7D0;
	sub_82421030(ctx, base);
	// 8241B7D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241B7D4: 419A0050  beq cr6, 0x8241b824
	if ctx.cr[6].eq {
	pc = 0x8241B824; continue 'dispatch;
	}
	// 8241B7D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8241B7DC: 419A0048  beq cr6, 0x8241b824
	if ctx.cr[6].eq {
	pc = 0x8241B824; continue 'dispatch;
	}
	// 8241B7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B7E4: 4BFFE635  bl 0x82419e18
	ctx.lr = 0x8241B7E8;
	sub_82419E18(ctx, base);
	// 8241B7E8: 48013769  bl 0x8242ef50
	ctx.lr = 0x8241B7EC;
	sub_8242EF50(ctx, base);
	// 8241B7EC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8241B7F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241B7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B7F8: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 8241B7FC: 4BFFFCAD  bl 0x8241b4a8
	ctx.lr = 0x8241B800;
	sub_8241B4A8(ctx, base);
	// 8241B800: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241B804: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241B808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241B80C: 997F0098  stb r11, 0x98(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 8241B810: 4182000C  beq 0x8241b81c
	if ctx.cr[0].eq {
	pc = 0x8241B81C; continue 'dispatch;
	}
	// 8241B814: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241B818: 4800AF69  bl 0x82426780
	ctx.lr = 0x8241B81C;
	sub_82426780(ctx, base);
	// 8241B81C: 48013735  bl 0x8242ef50
	ctx.lr = 0x8241B820;
	sub_8242EF50(ctx, base);
	// 8241B820: 48000010  b 0x8241b830
	pc = 0x8241B830; continue 'dispatch;
	// 8241B824: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B828: 386BFC50  addi r3, r11, -0x3b0
	ctx.r[3].s64 = ctx.r[11].s64 + -944;
	// 8241B82C: 48005A6D  bl 0x82421298
	ctx.lr = 0x8241B830;
	sub_82421298(ctx, base);
	// 8241B830: 48005841  bl 0x82421070
	ctx.lr = 0x8241B834;
	sub_82421070(ctx, base);
	// 8241B834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241B838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241B844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B850 size=100
    let mut pc: u32 = 0x8241B850;
    'dispatch: loop {
        match pc {
            0x8241B850 => {
    //   block [0x8241B850..0x8241B8B4)
	// 8241B850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B85C: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241B860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8241B864: 38EBF3F0  addi r7, r11, -0xc10
	ctx.r[7].s64 = ctx.r[11].s64 + -3088;
	// 8241B868: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8241B86C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241B870: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8241B874: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241B878: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241B87C: 4082FFEC  bne 0x8241b868
	if !ctx.cr[0].eq {
	pc = 0x8241B868; continue 'dispatch;
	}
	// 8241B880: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8241B884: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241B888: 419A0018  beq cr6, 0x8241b8a0
	if ctx.cr[6].eq {
	pc = 0x8241B8A0; continue 'dispatch;
	}
	// 8241B88C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B890: 386BFCA8  addi r3, r11, -0x358
	ctx.r[3].s64 = ctx.r[11].s64 + -856;
	// 8241B894: 48001375  bl 0x8241cc08
	ctx.lr = 0x8241B898;
	sub_8241CC08(ctx, base);
	// 8241B898: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241B89C: 48000008  b 0x8241b8a4
	pc = 0x8241B8A4; continue 'dispatch;
	// 8241B8A0: 4800E019  bl 0x824298b8
	ctx.lr = 0x8241B8A4;
	sub_824298B8(ctx, base);
	// 8241B8A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241B8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B8B8 size=128
    let mut pc: u32 = 0x8241B8B8;
    'dispatch: loop {
        match pc {
            0x8241B8B8 => {
    //   block [0x8241B8B8..0x8241B938)
	// 8241B8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B8C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B8C4: 48000695  bl 0x8241bf58
	ctx.lr = 0x8241B8C8;
	sub_8241BF58(ctx, base);
	// 8241B8C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241B8CC: 41820018  beq 0x8241b8e4
	if ctx.cr[0].eq {
	pc = 0x8241B8E4; continue 'dispatch;
	}
	// 8241B8D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B8D4: 386BFD50  addi r3, r11, -0x2b0
	ctx.r[3].s64 = ctx.r[11].s64 + -688;
	// 8241B8D8: 48001331  bl 0x8241cc08
	ctx.lr = 0x8241B8DC;
	sub_8241CC08(ctx, base);
	// 8241B8DC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241B8E0: 48000048  b 0x8241b928
	pc = 0x8241B928; continue 'dispatch;
	// 8241B8E4: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241B8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241B8EC: 38EBF3F0  addi r7, r11, -0xc10
	ctx.r[7].s64 = ctx.r[11].s64 + -3088;
	// 8241B8F0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8241B8F4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241B8F8: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8241B8FC: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241B900: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241B904: 4082FFEC  bne 0x8241b8f0
	if !ctx.cr[0].eq {
	pc = 0x8241B8F0; continue 'dispatch;
	}
	// 8241B908: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8241B90C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241B910: 409A0010  bne cr6, 0x8241b920
	if !ctx.cr[6].eq {
	pc = 0x8241B920; continue 'dispatch;
	}
	// 8241B914: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B918: 386BFCF0  addi r3, r11, -0x310
	ctx.r[3].s64 = ctx.r[11].s64 + -784;
	// 8241B91C: 4BFFFFBC  b 0x8241b8d8
	pc = 0x8241B8D8; continue 'dispatch;
	// 8241B920: 4800D949  bl 0x82429268
	ctx.lr = 0x8241B924;
	sub_82429268(ctx, base);
	// 8241B924: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241B928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241B92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B938 size=56
    let mut pc: u32 = 0x8241B938;
    'dispatch: loop {
        match pc {
            0x8241B938 => {
    //   block [0x8241B938..0x8241B970)
	// 8241B938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B94C: 480056E5  bl 0x82421030
	ctx.lr = 0x8241B950;
	sub_82421030(ctx, base);
	// 8241B950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241B954: 4800DE2D  bl 0x82429780
	ctx.lr = 0x8241B958;
	sub_82429780(ctx, base);
	// 8241B958: 48005719  bl 0x82421070
	ctx.lr = 0x8241B95C;
	sub_82421070(ctx, base);
	// 8241B95C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241B960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B970 size=56
    let mut pc: u32 = 0x8241B970;
    'dispatch: loop {
        match pc {
            0x8241B970 => {
    //   block [0x8241B970..0x8241B9A8)
	// 8241B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B978: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8241B97C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B980: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8241B984: 480056AD  bl 0x82421030
	ctx.lr = 0x8241B988;
	sub_82421030(ctx, base);
	// 8241B988: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8241B98C: 4800DE45  bl 0x824297d0
	ctx.lr = 0x8241B990;
	sub_824297D0(ctx, base);
	// 8241B990: 480056E1  bl 0x82421070
	ctx.lr = 0x8241B994;
	sub_82421070(ctx, base);
	// 8241B994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241B998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241B99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241B9A0: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241B9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241B9A8 size=132
    let mut pc: u32 = 0x8241B9A8;
    'dispatch: loop {
        match pc {
            0x8241B9A8 => {
    //   block [0x8241B9A8..0x8241BA2C)
	// 8241B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241B9B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241B9B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241B9B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241B9BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241B9C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241B9C4: 4800566D  bl 0x82421030
	ctx.lr = 0x8241B9C8;
	sub_82421030(ctx, base);
	// 8241B9C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241B9CC: 409A0018  bne cr6, 0x8241b9e4
	if !ctx.cr[6].eq {
	pc = 0x8241B9E4; continue 'dispatch;
	}
	// 8241B9D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B9D4: 386BFDD8  addi r3, r11, -0x228
	ctx.r[3].s64 = ctx.r[11].s64 + -552;
	// 8241B9D8: 48001231  bl 0x8241cc08
	ctx.lr = 0x8241B9DC;
	sub_8241CC08(ctx, base);
	// 8241B9DC: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241B9E0: 4800002C  b 0x8241ba0c
	pc = 0x8241BA0C; continue 'dispatch;
	// 8241B9E4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241B9E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241B9EC: 409A0010  bne cr6, 0x8241b9fc
	if !ctx.cr[6].eq {
	pc = 0x8241B9FC; continue 'dispatch;
	}
	// 8241B9F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241B9F4: 386BFDB0  addi r3, r11, -0x250
	ctx.r[3].s64 = ctx.r[11].s64 + -592;
	// 8241B9F8: 4BFFFFE0  b 0x8241b9d8
	pc = 0x8241B9D8; continue 'dispatch;
	// 8241B9FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241BA00: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241BA04: 48009295  bl 0x82424c98
	ctx.lr = 0x8241BA08;
	sub_82424C98(ctx, base);
	// 8241BA08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241BA0C: 48005665  bl 0x82421070
	ctx.lr = 0x8241BA10;
	sub_82421070(ctx, base);
	// 8241BA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241BA14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241BA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241BA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241BA20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241BA24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241BA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BA30 size=236
    let mut pc: u32 = 0x8241BA30;
    'dispatch: loop {
        match pc {
            0x8241BA30 => {
    //   block [0x8241BA30..0x8241BB1C)
	// 8241BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BA34: 48119689  bl 0x825350bc
	ctx.lr = 0x8241BA38;
	sub_82535080(ctx, base);
	// 8241BA38: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BA3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241BA40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241BA44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241BA48: 480055E9  bl 0x82421030
	ctx.lr = 0x8241BA4C;
	sub_82421030(ctx, base);
	// 8241BA4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241BA50: 409A0014  bne cr6, 0x8241ba64
	if !ctx.cr[6].eq {
	pc = 0x8241BA64; continue 'dispatch;
	}
	// 8241BA54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BA58: 386BFE24  addi r3, r11, -0x1dc
	ctx.r[3].s64 = ctx.r[11].s64 + -476;
	// 8241BA5C: 4800583D  bl 0x82421298
	ctx.lr = 0x8241BA60;
	sub_82421298(ctx, base);
	// 8241BA60: 480000B0  b 0x8241bb10
	pc = 0x8241BB10; continue 'dispatch;
	// 8241BA64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241BA68: 4BFFED69  bl 0x8241a7d0
	ctx.lr = 0x8241BA6C;
	sub_8241A7D0(ctx, base);
	// 8241BA6C: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 8241BA70: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8241BA74: 80BF00AC  lwz r5, 0xac(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8241BA78: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8241BA7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241BA80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241BA84: 4BFFC425  bl 0x82417ea8
	ctx.lr = 0x8241BA88;
	sub_82417EA8(ctx, base);
	// 8241BA88: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241BA8C: 40820084  bne 0x8241bb10
	if !ctx.cr[0].eq {
	pc = 0x8241BB10; continue 'dispatch;
	}
	// 8241BA90: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241BA94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241BA98: 409A003C  bne cr6, 0x8241bad4
	if !ctx.cr[6].eq {
	pc = 0x8241BAD4; continue 'dispatch;
	}
	// 8241BA9C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8241BAA0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8241BAA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241BAA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241BAAC: 480059CD  bl 0x82421478
	ctx.lr = 0x8241BAB0;
	sub_82421478(ctx, base);
	// 8241BAB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BAB4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8241BAB8: 386BFE00  addi r3, r11, -0x200
	ctx.r[3].s64 = ctx.r[11].s64 + -512;
	// 8241BABC: 48005865  bl 0x82421320
	ctx.lr = 0x8241BAC0;
	sub_82421320(ctx, base);
	// 8241BAC0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241BAC4: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8241BAC8: B17F0060  sth r11, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u16 ) };
	// 8241BACC: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8241BAD0: 48000040  b 0x8241bb10
	pc = 0x8241BB10; continue 'dispatch;
	// 8241BAD4: 815F00AC  lwz r10, 0xac(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8241BAD8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241BADC: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241BAE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241BAE4: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241BAE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241BAEC: 80E10058  lwz r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8241BAF0: 915F00B0  stw r10, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8241BAF4: 913F00B4  stw r9, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[9].u32 ) };
	// 8241BAF8: 911F00B8  stw r8, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[8].u32 ) };
	// 8241BAFC: 90FF00BC  stw r7, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[7].u32 ) };
	// 8241BB00: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 8241BB04: 997F00A8  stb r11, 0xa8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u8 ) };
	// 8241BB08: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 8241BB0C: 4BFFF4FD  bl 0x8241b008
	ctx.lr = 0x8241BB10;
	sub_8241B008(ctx, base);
	// 8241BB10: 48005561  bl 0x82421070
	ctx.lr = 0x8241BB14;
	sub_82421070(ctx, base);
	// 8241BB14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241BB18: 481195F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BB20 size=204
    let mut pc: u32 = 0x8241BB20;
    'dispatch: loop {
        match pc {
            0x8241BB20 => {
    //   block [0x8241BB20..0x8241BBEC)
	// 8241BB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BB24: 4811957D  bl 0x825350a0
	ctx.lr = 0x8241BB28;
	sub_82535080(ctx, base);
	// 8241BB28: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BB2C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8241BB30: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241BB34: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241BB38: 895B0005  lbz r10, 5(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(5 as u32) ) } as u64;
	// 8241BB3C: 7D560774  extsb r22, r10
	ctx.r[22].s64 = ctx.r[10].s8 as i64;
	// 8241BB40: 7D770775  extsb. r23, r11
	ctx.r[23].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8241BB44: 408100A0  ble 0x8241bbe4
	if !ctx.cr[0].gt {
	pc = 0x8241BBE4; continue 'dispatch;
	}
	// 8241BB48: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 8241BB4C: 40990098  ble cr6, 0x8241bbe4
	if !ctx.cr[6].gt {
	pc = 0x8241BBE4; continue 'dispatch;
	}
	// 8241BB50: 7F16B9D6  mullw r24, r22, r23
	ctx.r[24].s64 = (ctx.r[22].s32 as i64) * (ctx.r[23].s32 as i64);
	// 8241BB54: 2F180006  cmpwi cr6, r24, 6
	ctx.cr[6].compare_i32(ctx.r[24].s32, 6, &mut ctx.xer);
	// 8241BB58: 41980008  blt cr6, 0x8241bb60
	if ctx.cr[6].lt {
	pc = 0x8241BB60; continue 'dispatch;
	}
	// 8241BB5C: 3B000006  li r24, 6
	ctx.r[24].s64 = 6;
	// 8241BB60: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8241BB64: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 8241BB68: 4099007C  ble cr6, 0x8241bbe4
	if !ctx.cr[6].gt {
	pc = 0x8241BBE4; continue 'dispatch;
	}
	// 8241BB6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BB70: 3B2BFE50  addi r25, r11, -0x1b0
	ctx.r[25].s64 = ctx.r[11].s64 + -432;
	// 8241BB74: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8241BB78: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8241BB7C: 4099005C  ble cr6, 0x8241bbd8
	if !ctx.cr[6].gt {
	pc = 0x8241BBD8; continue 'dispatch;
	}
	// 8241BB80: 1D780006  mulli r11, r24, 6
	ctx.r[11].s64 = ctx.r[24].s64 * 6;
	// 8241BB84: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8241BB88: 396BFFFA  addi r11, r11, -6
	ctx.r[11].s64 = ctx.r[11].s64 + -6;
	// 8241BB8C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241BB90: 7FEBCA14  add r31, r11, r25
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8241BB94: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8241BB98: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241BB9C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8241BBA0: 4BFFCF31  bl 0x82418ad0
	ctx.lr = 0x8241BBA4;
	sub_82418AD0(ctx, base);
	// 8241BBA4: 397E009F  addi r11, r30, 0x9f
	ctx.r[11].s64 = ctx.r[30].s64 + 159;
	// 8241BBA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241BBAC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241BBB0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8241BBB4: 7CCBD82E  lwzx r6, r11, r27
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8241BBB8: 4800F0C9  bl 0x8242ac80
	ctx.lr = 0x8241BBBC;
	sub_8242AC80(ctx, base);
	// 8241BBBC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8241BBC0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8241BBC4: 2F1D0006  cmpwi cr6, r29, 6
	ctx.cr[6].compare_i32(ctx.r[29].s32, 6, &mut ctx.xer);
	// 8241BBC8: 40980010  bge cr6, 0x8241bbd8
	if !ctx.cr[6].lt {
	pc = 0x8241BBD8; continue 'dispatch;
	}
	// 8241BBCC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8241BBD0: 7F1CB800  cmpw cr6, r28, r23
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[23].s32, &mut ctx.xer);
	// 8241BBD4: 4198FFC0  blt cr6, 0x8241bb94
	if ctx.cr[6].lt {
	pc = 0x8241BB94; continue 'dispatch;
	}
	// 8241BBD8: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8241BBDC: 7F1AB000  cmpw cr6, r26, r22
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[22].s32, &mut ctx.xer);
	// 8241BBE0: 4198FF94  blt cr6, 0x8241bb74
	if ctx.cr[6].lt {
	pc = 0x8241BB74; continue 'dispatch;
	}
	// 8241BBE4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8241BBE8: 48119508  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BBF0 size=116
    let mut pc: u32 = 0x8241BBF0;
    'dispatch: loop {
        match pc {
            0x8241BBF0 => {
    //   block [0x8241BBF0..0x8241BC64)
	// 8241BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241BBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BBFC: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241BC00: 89430003  lbz r10, 3(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241BC04: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8241BC08: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8241BC0C: 8964000C  lbz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241BC10: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 8241BC14: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8241BC18: 99630005  stb r11, 5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 8241BC1C: 41980028  blt cr6, 0x8241bc44
	if ctx.cr[6].lt {
	pc = 0x8241BC44; continue 'dispatch;
	}
	// 8241BC20: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 8241BC24: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241BC28: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8241BC2C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8241BC30: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8241BC34: 41980010  blt cr6, 0x8241bc44
	if ctx.cr[6].lt {
	pc = 0x8241BC44; continue 'dispatch;
	}
	// 8241BC38: 4BFFFEE9  bl 0x8241bb20
	ctx.lr = 0x8241BC3C;
	sub_8241BB20(ctx, base);
	// 8241BC3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241BC40: 48000014  b 0x8241bc54
	pc = 0x8241BC54; continue 'dispatch;
	// 8241BC44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BC48: 386BFEE0  addi r3, r11, -0x120
	ctx.r[3].s64 = ctx.r[11].s64 + -288;
	// 8241BC4C: 48000FBD  bl 0x8241cc08
	ctx.lr = 0x8241BC50;
	sub_8241CC08(ctx, base);
	// 8241BC50: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241BC54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241BC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241BC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241BC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BC68 size=92
    let mut pc: u32 = 0x8241BC68;
    'dispatch: loop {
        match pc {
            0x8241BC68 => {
    //   block [0x8241BC68..0x8241BCC4)
	// 8241BC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241BC70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241BC74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BC78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241BC7C: 480132D5  bl 0x8242ef50
	ctx.lr = 0x8241BC80;
	sub_8242EF50(ctx, base);
	// 8241BC80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241BC84: 409A0014  bne cr6, 0x8241bc98
	if !ctx.cr[6].eq {
	pc = 0x8241BC98; continue 'dispatch;
	}
	// 8241BC88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BC8C: 386BFF20  addi r3, r11, -0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + -224;
	// 8241BC90: 48000F79  bl 0x8241cc08
	ctx.lr = 0x8241BC94;
	sub_8241CC08(ctx, base);
	// 8241BC94: 48000018  b 0x8241bcac
	pc = 0x8241BCAC; continue 'dispatch;
	// 8241BC98: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241BC9C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241BCA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8241BCA4: 388BBBF0  addi r4, r11, -0x4410
	ctx.r[4].s64 = ctx.r[11].s64 + -17424;
	// 8241BCA8: 48007BA9  bl 0x82423850
	ctx.lr = 0x8241BCAC;
	sub_82423850(ctx, base);
	// 8241BCAC: 480132A5  bl 0x8242ef50
	ctx.lr = 0x8241BCB0;
	sub_8242EF50(ctx, base);
	// 8241BCB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241BCB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241BCB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241BCBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241BCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241BCC8 size=8
    let mut pc: u32 = 0x8241BCC8;
    'dispatch: loop {
        match pc {
            0x8241BCC8 => {
    //   block [0x8241BCC8..0x8241BCD0)
	// 8241BCC8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8241BCCC: 480055CC  b 0x82421298
	sub_82421298(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BCD0 size=40
    let mut pc: u32 = 0x8241BCD0;
    'dispatch: loop {
        match pc {
            0x8241BCD0 => {
    //   block [0x8241BCD0..0x8241BCF8)
	// 8241BCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241BCD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BCDC: 4BFFEFFD  bl 0x8241acd8
	ctx.lr = 0x8241BCE0;
	sub_8241ACD8(ctx, base);
	// 8241BCE0: 4800F011  bl 0x8242acf0
	ctx.lr = 0x8241BCE4;
	sub_8242ACF0(ctx, base);
	// 8241BCE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241BCE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241BCEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241BCF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241BCF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BCF8 size=36
    let mut pc: u32 = 0x8241BCF8;
    'dispatch: loop {
        match pc {
            0x8241BCF8 => {
    //   block [0x8241BCF8..0x8241BD1C)
	// 8241BCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241BD00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BD04: 480069CD  bl 0x824226d0
	ctx.lr = 0x8241BD08;
	sub_824226D0(ctx, base);
	// 8241BD08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241BD0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241BD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241BD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241BD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BD20 size=320
    let mut pc: u32 = 0x8241BD20;
    'dispatch: loop {
        match pc {
            0x8241BD20 => {
    //   block [0x8241BD20..0x8241BE60)
	// 8241BD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241BD28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241BD2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BD30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BD34: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 8241BD38: 396BFF48  addi r11, r11, -0xb8
	ctx.r[11].s64 = ctx.r[11].s64 + -184;
	// 8241BD3C: 916AF3F4  stw r11, -0xc0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3084 as u32), ctx.r[11].u32 ) };
	// 8241BD40: 480077D1  bl 0x82423510
	ctx.lr = 0x8241BD44;
	sub_82423510(ctx, base);
	// 8241BD44: 3FE08289  lis r31, -0x7d77
	ctx.r[31].s64 = -2104950784;
	// 8241BD48: 817FF3F8  lwz r11, -0xc08(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 8241BD4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241BD50: 409A00F4  bne cr6, 0x8241be44
	if !ctx.cr[6].eq {
	pc = 0x8241BE44; continue 'dispatch;
	}
	// 8241BD54: 480051E5  bl 0x82420f38
	ctx.lr = 0x8241BD58;
	sub_82420F38(ctx, base);
	// 8241BD58: 480131F9  bl 0x8242ef50
	ctx.lr = 0x8241BD5C;
	sub_8242EF50(ctx, base);
	// 8241BD5C: 4800F655  bl 0x8242b3b0
	ctx.lr = 0x8241BD60;
	sub_8242B3B0(ctx, base);
	// 8241BD60: 48007009  bl 0x82422d68
	ctx.lr = 0x8241BD64;
	sub_82422D68(ctx, base);
	// 8241BD64: 4800923D  bl 0x82424fa0
	ctx.lr = 0x8241BD68;
	sub_82424FA0(ctx, base);
	// 8241BD68: 480054E9  bl 0x82421250
	ctx.lr = 0x8241BD6C;
	sub_82421250(ctx, base);
	// 8241BD6C: 480057E5  bl 0x82421550
	ctx.lr = 0x8241BD70;
	sub_82421550(ctx, base);
	// 8241BD70: 4800A2B9  bl 0x82426028
	ctx.lr = 0x8241BD74;
	sub_82426028(ctx, base);
	// 8241BD74: 4800533D  bl 0x824210b0
	ctx.lr = 0x8241BD78;
	sub_824210B0(ctx, base);
	// 8241BD78: 48008969  bl 0x824246e0
	ctx.lr = 0x8241BD7C;
	sub_824246E0(ctx, base);
	// 8241BD7C: 4800F0B5  bl 0x8242ae30
	ctx.lr = 0x8241BD80;
	sub_8242AE30(ctx, base);
	// 8241BD80: 48009C09  bl 0x82425988
	ctx.lr = 0x8241BD84;
	sub_82425988(ctx, base);
	// 8241BD84: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241BD88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241BD8C: 386BBCC8  addi r3, r11, -0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + -17208;
	// 8241BD90: 480131C1  bl 0x8242ef50
	ctx.lr = 0x8241BD94;
	sub_8242EF50(ctx, base);
	// 8241BD94: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241BD98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241BD9C: 386BBCC8  addi r3, r11, -0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + -17208;
	// 8241BDA0: 4800EFC9  bl 0x8242ad68
	ctx.lr = 0x8241BDA4;
	sub_8242AD68(ctx, base);
	// 8241BDA4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8241BDA8: 38A01880  li r5, 0x1880
	ctx.r[5].s64 = 6272;
	// 8241BDAC: 386BDF80  addi r3, r11, -0x2080
	ctx.r[3].s64 = ctx.r[11].s64 + -8320;
	// 8241BDB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241BDB4: 4811941D  bl 0x825351d0
	ctx.lr = 0x8241BDB8;
	sub_825351D0(ctx, base);
	// 8241BDB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241BDC0: 38EBFFBC  addi r7, r11, -0x44
	ctx.r[7].s64 = ctx.r[11].s64 + -68;
	// 8241BDC4: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241BDC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241BDCC: 38ABBCD0  addi r5, r11, -0x4330
	ctx.r[5].s64 = ctx.r[11].s64 + -17200;
	// 8241BDD0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8241BDD4: 4800A18D  bl 0x82425f60
	ctx.lr = 0x8241BDD8;
	sub_82425F60(ctx, base);
	// 8241BDD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BDDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241BDE0: 38CBFFAC  addi r6, r11, -0x54
	ctx.r[6].s64 = ctx.r[11].s64 + -84;
	// 8241BDE4: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241BDE8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8241BDEC: 388BBCF8  addi r4, r11, -0x4308
	ctx.r[4].s64 = ctx.r[11].s64 + -17160;
	// 8241BDF0: 48009FE9  bl 0x82425dd8
	ctx.lr = 0x8241BDF4;
	sub_82425DD8(ctx, base);
	// 8241BDF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BDF8: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 8241BDFC: 38CBFF98  addi r6, r11, -0x68
	ctx.r[6].s64 = ctx.r[11].s64 + -104;
	// 8241BE00: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 8241BE04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241BE08: 388B7708  addi r4, r11, 0x7708
	ctx.r[4].s64 = ctx.r[11].s64 + 30472;
	// 8241BE0C: 906AF408  stw r3, -0xbf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3064 as u32), ctx.r[3].u32 ) };
	// 8241BE10: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8241BE14: 48009FC5  bl 0x82425dd8
	ctx.lr = 0x8241BE18;
	sub_82425DD8(ctx, base);
	// 8241BE18: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241BE1C: 3D208289  lis r9, -0x7d77
	ctx.r[9].s64 = -2104950784;
	// 8241BE20: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 8241BE24: 906BF400  stw r3, -0xc00(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3072 as u32), ctx.r[3].u32 ) };
	// 8241BE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241BE2C: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 8241BE30: 916AF404  stw r11, -0xbfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3068 as u32), ctx.r[11].u32 ) };
	// 8241BE34: 9169F40C  stw r11, -0xbf4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-3060 as u32), ctx.r[11].u32 ) };
	// 8241BE38: 4BFFED41  bl 0x8241ab78
	ctx.lr = 0x8241BE3C;
	sub_8241AB78(ctx, base);
	// 8241BE3C: 48013115  bl 0x8242ef50
	ctx.lr = 0x8241BE40;
	sub_8242EF50(ctx, base);
	// 8241BE40: 817FF3F8  lwz r11, -0xc08(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 8241BE44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241BE48: 917FF3F8  stw r11, -0xc08(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-3080 as u32), ctx.r[11].u32 ) };
	// 8241BE4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241BE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241BE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241BE58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241BE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BE60 size=248
    let mut pc: u32 = 0x8241BE60;
    'dispatch: loop {
        match pc {
            0x8241BE60 => {
    //   block [0x8241BE60..0x8241BF58)
	// 8241BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BE64: 48119259  bl 0x825350bc
	ctx.lr = 0x8241BE68;
	sub_82535080(ctx, base);
	// 8241BE68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BE6C: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 8241BE70: 816AF3F8  lwz r11, -0xc08(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 8241BE74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241BE78: 41990014  bgt cr6, 0x8241be8c
	if ctx.cr[6].gt {
	pc = 0x8241BE8C; continue 'dispatch;
	}
	// 8241BE7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BE80: 386BFFF8  addi r3, r11, -8
	ctx.r[3].s64 = ctx.r[11].s64 + -8;
	// 8241BE84: 4800999D  bl 0x82425820
	ctx.lr = 0x8241BE88;
	sub_82425820(ctx, base);
	// 8241BE88: 480000C8  b 0x8241bf50
	pc = 0x8241BF50; continue 'dispatch;
	// 8241BE8C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241BE90: 916AF3F8  stw r11, -0xc08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3080 as u32), ctx.r[11].u32 ) };
	// 8241BE94: 408200BC  bne 0x8241bf50
	if !ctx.cr[0].eq {
	pc = 0x8241BF50; continue 'dispatch;
	}
	// 8241BE98: 4BFFF5A9  bl 0x8241b440
	ctx.lr = 0x8241BE9C;
	sub_8241B440(ctx, base);
	// 8241BE9C: 48008F45  bl 0x82424de0
	ctx.lr = 0x8241BEA0;
	sub_82424DE0(ctx, base);
	// 8241BEA0: 480052F1  bl 0x82421190
	ctx.lr = 0x8241BEA4;
	sub_82421190(ctx, base);
	// 8241BEA4: 480056FD  bl 0x824215a0
	ctx.lr = 0x8241BEA8;
	sub_824215A0(ctx, base);
	// 8241BEA8: 4800F009  bl 0x8242aeb0
	ctx.lr = 0x8241BEAC;
	sub_8242AEB0(ctx, base);
	// 8241BEAC: 480130A5  bl 0x8242ef50
	ctx.lr = 0x8241BEB0;
	sub_8242EF50(ctx, base);
	// 8241BEB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241BEB4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8241BEB8: 48009FF9  bl 0x82425eb0
	ctx.lr = 0x8241BEBC;
	sub_82425EB0(ctx, base);
	// 8241BEBC: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241BEC0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8241BEC4: 808BF408  lwz r4, -0xbf8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3064 as u32) ) } as u64;
	// 8241BEC8: 48009FE9  bl 0x82425eb0
	ctx.lr = 0x8241BECC;
	sub_82425EB0(ctx, base);
	// 8241BECC: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241BED0: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8241BED4: 808BF400  lwz r4, -0xc00(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3072 as u32) ) } as u64;
	// 8241BED8: 48009FD9  bl 0x82425eb0
	ctx.lr = 0x8241BEDC;
	sub_82425EB0(ctx, base);
	// 8241BEDC: 48009B2D  bl 0x82425a08
	ctx.lr = 0x8241BEE0;
	sub_82425A08(ctx, base);
	// 8241BEE0: 4800A1A1  bl 0x82426080
	ctx.lr = 0x8241BEE4;
	sub_82426080(ctx, base);
	// 8241BEE4: 4800536D  bl 0x82421250
	ctx.lr = 0x8241BEE8;
	sub_82421250(ctx, base);
	// 8241BEE8: 48009121  bl 0x82425008
	ctx.lr = 0x8241BEEC;
	sub_82425008(ctx, base);
	// 8241BEEC: 48006EE5  bl 0x82422dd0
	ctx.lr = 0x8241BEF0;
	sub_82422DD0(ctx, base);
	// 8241BEF0: 4800F529  bl 0x8242b418
	ctx.lr = 0x8241BEF4;
	sub_8242B418(ctx, base);
	// 8241BEF4: 4801305D  bl 0x8242ef50
	ctx.lr = 0x8241BEF8;
	sub_8242EF50(ctx, base);
	// 8241BEF8: 3BE00400  li r31, 0x400
	ctx.r[31].s64 = 1024;
	// 8241BEFC: 48005135  bl 0x82421030
	ctx.lr = 0x8241BF00;
	sub_82421030(ctx, base);
	// 8241BF00: 48005171  bl 0x82421070
	ctx.lr = 0x8241BF04;
	sub_82421070(ctx, base);
	// 8241BF04: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8241BF08: 4082FFF4  bne 0x8241befc
	if !ctx.cr[0].eq {
	pc = 0x8241BEFC; continue 'dispatch;
	}
	// 8241BF0C: 480050AD  bl 0x82420fb8
	ctx.lr = 0x8241BF10;
	sub_82420FB8(ctx, base);
	// 8241BF10: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8241BF14: 3BCBDF80  addi r30, r11, -0x2080
	ctx.r[30].s64 = ctx.r[11].s64 + -8320;
	// 8241BF18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241BF1C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8241BF20: 3BABFFCC  addi r29, r11, -0x34
	ctx.r[29].s64 = ctx.r[11].s64 + -52;
	// 8241BF24: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241BF28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241BF2C: 419A0014  beq cr6, 0x8241bf40
	if ctx.cr[6].eq {
	pc = 0x8241BF40; continue 'dispatch;
	}
	// 8241BF30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241BF34: 480098ED  bl 0x82425820
	ctx.lr = 0x8241BF38;
	sub_82425820(ctx, base);
	// 8241BF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241BF3C: 4BFFF4CD  bl 0x8241b408
	ctx.lr = 0x8241BF40;
	sub_8241B408(ctx, base);
	// 8241BF40: 3BFF00C4  addi r31, r31, 0xc4
	ctx.r[31].s64 = ctx.r[31].s64 + 196;
	// 8241BF44: 397E1880  addi r11, r30, 0x1880
	ctx.r[11].s64 = ctx.r[30].s64 + 6272;
	// 8241BF48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241BF4C: 4198FFD8  blt cr6, 0x8241bf24
	if ctx.cr[6].lt {
	pc = 0x8241BF24; continue 'dispatch;
	}
	// 8241BF50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241BF54: 481191B8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241BF58 size=24
    let mut pc: u32 = 0x8241BF58;
    'dispatch: loop {
        match pc {
            0x8241BF58 => {
    //   block [0x8241BF58..0x8241BF70)
	// 8241BF58: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241BF5C: 816BF3F8  lwz r11, -0xc08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 8241BF60: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8241BF64: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8241BF68: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8241BF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241BF70 size=44
    let mut pc: u32 = 0x8241BF70;
    'dispatch: loop {
        match pc {
            0x8241BF70 => {
    //   block [0x8241BF70..0x8241BF9C)
	// 8241BF70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241BF74: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241BF78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241BF7C: 7D401828  lwarx r10, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241BF80: 7D60192D  stwcx. r11, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241BF84: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241BF88: 4082FFEC  bne 0x8241bf74
	if !ctx.cr[0].eq {
	pc = 0x8241BF74; continue 'dispatch;
	}
	// 8241BF8C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241BF90: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8241BF94: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8241BF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241BFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241BFA0 size=140
    let mut pc: u32 = 0x8241BFA0;
    'dispatch: loop {
        match pc {
            0x8241BFA0 => {
    //   block [0x8241BFA0..0x8241C02C)
	// 8241BFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241BFA4: 48119119  bl 0x825350bc
	ctx.lr = 0x8241BFA8;
	sub_82535080(ctx, base);
	// 8241BFA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241BFAC: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241BFB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8241BFB4: 3BABF474  addi r29, r11, -0xb8c
	ctx.r[29].s64 = ctx.r[11].s64 + -2956;
	// 8241BFB8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241BFBC: 397DFFBC  addi r11, r29, -0x44
	ctx.r[11].s64 = ctx.r[29].s64 + -68;
	// 8241BFC0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241BFC4: 3D60002D  lis r11, 0x2d
	ctx.r[11].s64 = 2949120;
	// 8241BFC8: 617EC6C0  ori r30, r11, 0xc6c0
	ctx.r[30].u64 = ctx.r[11].u64 | 50880;
	// 8241BFCC: 397DFFA4  addi r11, r29, -0x5c
	ctx.r[11].s64 = ctx.r[29].s64 + -92;
	// 8241BFD0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241BFD4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241BFD8: 4BFA5A09  bl 0x823c19e0
	ctx.lr = 0x8241BFDC;
	sub_823C19E0(ctx, base);
	// 8241BFDC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241BFE0: 4BFA5DB1  bl 0x823c1d90
	ctx.lr = 0x8241BFE4;
	sub_823C1D90(ctx, base);
	// 8241BFE4: 397DFFBC  addi r11, r29, -0x44
	ctx.r[11].s64 = ctx.r[29].s64 + -68;
	// 8241BFE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241BFEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241BFF0: 419A0010  beq cr6, 0x8241c000
	if ctx.cr[6].eq {
	pc = 0x8241C000; continue 'dispatch;
	}
	// 8241BFF4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8241BFF8: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8241BFFC: 4198FFD0  blt cr6, 0x8241bfcc
	if ctx.cr[6].lt {
	pc = 0x8241BFCC; continue 'dispatch;
	}
	// 8241C000: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8241C004: 409A0010  bne cr6, 0x8241c014
	if !ctx.cr[6].eq {
	pc = 0x8241C014; continue 'dispatch;
	}
	// 8241C008: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C00C: 386B0044  addi r3, r11, 0x44
	ctx.r[3].s64 = ctx.r[11].s64 + 68;
	// 8241C010: 48000BF9  bl 0x8241cc08
	ctx.lr = 0x8241C014;
	sub_8241CC08(ctx, base);
	// 8241C014: 397DFFA4  addi r11, r29, -0x5c
	ctx.r[11].s64 = ctx.r[29].s64 + -92;
	// 8241C018: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241C01C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C020: 4BFA59C1  bl 0x823c19e0
	ctx.lr = 0x8241C024;
	sub_823C19E0(ctx, base);
	// 8241C024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241C028: 481190E4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C030 size=224
    let mut pc: u32 = 0x8241C030;
    'dispatch: loop {
        match pc {
            0x8241C030 => {
    //   block [0x8241C030..0x8241C110)
	// 8241C030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C03C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C040: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C044: 3BEBF45C  addi r31, r11, -0xba4
	ctx.r[31].s64 = ctx.r[11].s64 + -2980;
	// 8241C048: 48000098  b 0x8241c0e0
	pc = 0x8241C0E0; continue 'dispatch;
	// 8241C04C: 397FFFDC  addi r11, r31, -0x24
	ctx.r[11].s64 = ctx.r[31].s64 + -36;
	// 8241C050: 395FFFDC  addi r10, r31, -0x24
	ctx.r[10].s64 = ctx.r[31].s64 + -36;
	// 8241C054: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C058: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241C05C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C060: 48009CE9  bl 0x82425d48
	ctx.lr = 0x8241C064;
	sub_82425D48(ctx, base);
	// 8241C064: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241C068: 41820014  beq 0x8241c07c
	if ctx.cr[0].eq {
	pc = 0x8241C07C; continue 'dispatch;
	}
	// 8241C06C: 397FFFD4  addi r11, r31, -0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + -44;
	// 8241C070: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C074: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241C078: 409A0068  bne cr6, 0x8241c0e0
	if !ctx.cr[6].eq {
	pc = 0x8241C0E0; continue 'dispatch;
	}
	// 8241C07C: 397FFFD4  addi r11, r31, -0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + -44;
	// 8241C080: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C084: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241C088: 409A0024  bne cr6, 0x8241c0ac
	if !ctx.cr[6].eq {
	pc = 0x8241C0AC; continue 'dispatch;
	}
	// 8241C08C: 397FFFD4  addi r11, r31, -0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + -44;
	// 8241C090: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8241C094: 395FFFBC  addi r10, r31, -0x44
	ctx.r[10].s64 = ctx.r[31].s64 + -68;
	// 8241C098: 393F0018  addi r9, r31, 0x18
	ctx.r[9].s64 = ctx.r[31].s64 + 24;
	// 8241C09C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8241C0A0: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241C0A4: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C0A8: 4BFA5939  bl 0x823c19e0
	ctx.lr = 0x8241C0AC;
	sub_823C19E0(ctx, base);
	// 8241C0AC: 397FFFE4  addi r11, r31, -0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + -28;
	// 8241C0B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C0B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C0B8: 419A001C  beq cr6, 0x8241c0d4
	if ctx.cr[6].eq {
	pc = 0x8241C0D4; continue 'dispatch;
	}
	// 8241C0BC: 397FFFE4  addi r11, r31, -0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + -28;
	// 8241C0C0: 395FFFE4  addi r10, r31, -0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + -28;
	// 8241C0C4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241C0C8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C0CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241C0D0: 4E800421  bctrl
	ctx.lr = 0x8241C0D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241C0D4: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 8241C0D8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C0DC: 4BFA5A05  bl 0x823c1ae0
	ctx.lr = 0x8241C0E0;
	sub_823C1AE0(ctx, base);
	// 8241C0E0: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241C0E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C0E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C0EC: 419AFF60  beq cr6, 0x8241c04c
	if ctx.cr[6].eq {
	pc = 0x8241C04C; continue 'dispatch;
	}
	// 8241C0F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241C0F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C0F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C0FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241C10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241C110 size=24
    let mut pc: u32 = 0x8241C110;
    'dispatch: loop {
        match pc {
            0x8241C110 => {
    //   block [0x8241C110..0x8241C128)
	// 8241C110: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C114: 816BF414  lwz r11, -0xbec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3052 as u32) ) } as u64;
	// 8241C118: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8241C11C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8241C120: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8241C124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241C128 size=4
    let mut pc: u32 = 0x8241C128;
    'dispatch: loop {
        match pc {
            0x8241C128 => {
    //   block [0x8241C128..0x8241C12C)
	// 8241C128: 48000B18  b 0x8241cc40
	sub_8241CC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241C130 size=4
    let mut pc: u32 = 0x8241C130;
    'dispatch: loop {
        match pc {
            0x8241C130 => {
    //   block [0x8241C130..0x8241C134)
	// 8241C130: 4800F7B0  b 0x8242b8e0
	sub_8242B8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C138 size=524
    let mut pc: u32 = 0x8241C138;
    'dispatch: loop {
        match pc {
            0x8241C138 => {
    //   block [0x8241C138..0x8241C344)
	// 8241C138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C13C: 48118F79  bl 0x825350b4
	ctx.lr = 0x8241C140;
	sub_82535080(ctx, base);
	// 8241C140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C144: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C148: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241C14C: 3BEBF470  addi r31, r11, -0xb90
	ctx.r[31].s64 = ctx.r[11].s64 + -2960;
	// 8241C150: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 8241C154: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8241C158: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8241C15C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C164: 419A0094  beq cr6, 0x8241c1f8
	if ctx.cr[6].eq {
	pc = 0x8241C1F8; continue 'dispatch;
	}
	// 8241C168: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8241C16C: 397FFFD8  addi r11, r31, -0x28
	ctx.r[11].s64 = ctx.r[31].s64 + -40;
	// 8241C170: 395FFFF4  addi r10, r31, -0xc
	ctx.r[10].s64 = ctx.r[31].s64 + -12;
	// 8241C174: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8241C178: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8241C17C: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C180: 4BFA5861  bl 0x823c19e0
	ctx.lr = 0x8241C184;
	sub_823C19E0(ctx, base);
	// 8241C184: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8241C188: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C18C: 4BFA5C05  bl 0x823c1d90
	ctx.lr = 0x8241C190;
	sub_823C1D90(ctx, base);
	// 8241C190: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8241C194: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 8241C198: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C19C: 4BFA537D  bl 0x823c1518
	ctx.lr = 0x8241C1A0;
	sub_823C1518(ctx, base);
	// 8241C1A0: 2B030102  cmplwi cr6, r3, 0x102
	ctx.cr[6].compare_u32(ctx.r[3].u32, 258 as u32, &mut ctx.xer);
	// 8241C1A4: 409A0010  bne cr6, 0x8241c1b4
	if !ctx.cr[6].eq {
	pc = 0x8241C1B4; continue 'dispatch;
	}
	// 8241C1A8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8241C1AC: 2F1E001E  cmpwi cr6, r30, 0x1e
	ctx.cr[6].compare_i32(ctx.r[30].s32, 30, &mut ctx.xer);
	// 8241C1B0: 4198FFBC  blt cr6, 0x8241c16c
	if ctx.cr[6].lt {
	pc = 0x8241C16C; continue 'dispatch;
	}
	// 8241C1B4: 2F1E001E  cmpwi cr6, r30, 0x1e
	ctx.cr[6].compare_i32(ctx.r[30].s32, 30, &mut ctx.xer);
	// 8241C1B8: 409A0014  bne cr6, 0x8241c1cc
	if !ctx.cr[6].eq {
	pc = 0x8241C1CC; continue 'dispatch;
	}
	// 8241C1BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C1C0: 386B00DC  addi r3, r11, 0xdc
	ctx.r[3].s64 = ctx.r[11].s64 + 220;
	// 8241C1C4: 48000A45  bl 0x8241cc08
	ctx.lr = 0x8241C1C8;
	sub_8241CC08(ctx, base);
	// 8241C1C8: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 8241C1CC: 397FFFDC  addi r11, r31, -0x24
	ctx.r[11].s64 = ctx.r[31].s64 + -36;
	// 8241C1D0: 395FFFD8  addi r10, r31, -0x28
	ctx.r[10].s64 = ctx.r[31].s64 + -40;
	// 8241C1D4: 393FFFF4  addi r9, r31, -0xc
	ctx.r[9].s64 = ctx.r[31].s64 + -12;
	// 8241C1D8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C1DC: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C1E0: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C1E4: 4BFA4775  bl 0x823c0958
	ctx.lr = 0x8241C1E8;
	sub_823C0958(ctx, base);
	// 8241C1E8: 395FFFF4  addi r10, r31, -0xc
	ctx.r[10].s64 = ctx.r[31].s64 + -12;
	// 8241C1EC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8241C1F0: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C1F4: 917FFFF0  stw r11, -0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 8241C1F8: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241C1FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C204: 419A0094  beq cr6, 0x8241c298
	if ctx.cr[6].eq {
	pc = 0x8241C298; continue 'dispatch;
	}
	// 8241C208: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8241C20C: 397FFFE0  addi r11, r31, -0x20
	ctx.r[11].s64 = ctx.r[31].s64 + -32;
	// 8241C210: 395FFFFC  addi r10, r31, -4
	ctx.r[10].s64 = ctx.r[31].s64 + -4;
	// 8241C214: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8241C218: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8241C21C: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C220: 4BFA57C1  bl 0x823c19e0
	ctx.lr = 0x8241C224;
	sub_823C19E0(ctx, base);
	// 8241C224: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241C228: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C22C: 4BFA5B65  bl 0x823c1d90
	ctx.lr = 0x8241C230;
	sub_823C1D90(ctx, base);
	// 8241C230: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241C234: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 8241C238: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C23C: 4BFA52DD  bl 0x823c1518
	ctx.lr = 0x8241C240;
	sub_823C1518(ctx, base);
	// 8241C240: 2B030102  cmplwi cr6, r3, 0x102
	ctx.cr[6].compare_u32(ctx.r[3].u32, 258 as u32, &mut ctx.xer);
	// 8241C244: 409A0010  bne cr6, 0x8241c254
	if !ctx.cr[6].eq {
	pc = 0x8241C254; continue 'dispatch;
	}
	// 8241C248: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8241C24C: 2F1E001E  cmpwi cr6, r30, 0x1e
	ctx.cr[6].compare_i32(ctx.r[30].s32, 30, &mut ctx.xer);
	// 8241C250: 4198FFBC  blt cr6, 0x8241c20c
	if ctx.cr[6].lt {
	pc = 0x8241C20C; continue 'dispatch;
	}
	// 8241C254: 2F1E001E  cmpwi cr6, r30, 0x1e
	ctx.cr[6].compare_i32(ctx.r[30].s32, 30, &mut ctx.xer);
	// 8241C258: 409A0014  bne cr6, 0x8241c26c
	if !ctx.cr[6].eq {
	pc = 0x8241C26C; continue 'dispatch;
	}
	// 8241C25C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C260: 386B00AC  addi r3, r11, 0xac
	ctx.r[3].s64 = ctx.r[11].s64 + 172;
	// 8241C264: 480009A5  bl 0x8241cc08
	ctx.lr = 0x8241C268;
	sub_8241CC08(ctx, base);
	// 8241C268: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 8241C26C: 397FFFE4  addi r11, r31, -0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + -28;
	// 8241C270: 395FFFE0  addi r10, r31, -0x20
	ctx.r[10].s64 = ctx.r[31].s64 + -32;
	// 8241C274: 393FFFFC  addi r9, r31, -4
	ctx.r[9].s64 = ctx.r[31].s64 + -4;
	// 8241C278: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C27C: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C280: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C284: 4BFA46D5  bl 0x823c0958
	ctx.lr = 0x8241C288;
	sub_823C0958(ctx, base);
	// 8241C288: 395FFFFC  addi r10, r31, -4
	ctx.r[10].s64 = ctx.r[31].s64 + -4;
	// 8241C28C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8241C290: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C294: 917FFFF8  stw r11, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 8241C298: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8241C29C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C2A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C2A4: 419A0094  beq cr6, 0x8241c338
	if ctx.cr[6].eq {
	pc = 0x8241C338; continue 'dispatch;
	}
	// 8241C2A8: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8241C2AC: 397FFFE8  addi r11, r31, -0x18
	ctx.r[11].s64 = ctx.r[31].s64 + -24;
	// 8241C2B0: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8241C2B4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8241C2B8: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8241C2BC: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C2C0: 4BFA5721  bl 0x823c19e0
	ctx.lr = 0x8241C2C4;
	sub_823C19E0(ctx, base);
	// 8241C2C4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8241C2C8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C2CC: 4BFA5AC5  bl 0x823c1d90
	ctx.lr = 0x8241C2D0;
	sub_823C1D90(ctx, base);
	// 8241C2D0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8241C2D4: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 8241C2D8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C2DC: 4BFA523D  bl 0x823c1518
	ctx.lr = 0x8241C2E0;
	sub_823C1518(ctx, base);
	// 8241C2E0: 2B030102  cmplwi cr6, r3, 0x102
	ctx.cr[6].compare_u32(ctx.r[3].u32, 258 as u32, &mut ctx.xer);
	// 8241C2E4: 409A0010  bne cr6, 0x8241c2f4
	if !ctx.cr[6].eq {
	pc = 0x8241C2F4; continue 'dispatch;
	}
	// 8241C2E8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8241C2EC: 2F1E001E  cmpwi cr6, r30, 0x1e
	ctx.cr[6].compare_i32(ctx.r[30].s32, 30, &mut ctx.xer);
	// 8241C2F0: 4198FFBC  blt cr6, 0x8241c2ac
	if ctx.cr[6].lt {
	pc = 0x8241C2AC; continue 'dispatch;
	}
	// 8241C2F4: 2F1E001E  cmpwi cr6, r30, 0x1e
	ctx.cr[6].compare_i32(ctx.r[30].s32, 30, &mut ctx.xer);
	// 8241C2F8: 409A0014  bne cr6, 0x8241c30c
	if !ctx.cr[6].eq {
	pc = 0x8241C30C; continue 'dispatch;
	}
	// 8241C2FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C300: 386B0078  addi r3, r11, 0x78
	ctx.r[3].s64 = ctx.r[11].s64 + 120;
	// 8241C304: 48000905  bl 0x8241cc08
	ctx.lr = 0x8241C308;
	sub_8241CC08(ctx, base);
	// 8241C308: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 8241C30C: 397FFFEC  addi r11, r31, -0x14
	ctx.r[11].s64 = ctx.r[31].s64 + -20;
	// 8241C310: 395FFFE8  addi r10, r31, -0x18
	ctx.r[10].s64 = ctx.r[31].s64 + -24;
	// 8241C314: 393F0004  addi r9, r31, 4
	ctx.r[9].s64 = ctx.r[31].s64 + 4;
	// 8241C318: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C31C: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C320: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C324: 4BFA4635  bl 0x823c0958
	ctx.lr = 0x8241C328;
	sub_823C0958(ctx, base);
	// 8241C328: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8241C32C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8241C330: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8241C334: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C338: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8241C33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241C340: 48118DC4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C348 size=188
    let mut pc: u32 = 0x8241C348;
    'dispatch: loop {
        match pc {
            0x8241C348 => {
    //   block [0x8241C348..0x8241C404)
	// 8241C348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241C354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C35C: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C360: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241C364: 3BCBF474  addi r30, r11, -0xb8c
	ctx.r[30].s64 = ctx.r[11].s64 + -2956;
	// 8241C368: 397EFFA4  addi r11, r30, -0x5c
	ctx.r[11].s64 = ctx.r[30].s64 + -92;
	// 8241C36C: 395EFFF0  addi r10, r30, -0x10
	ctx.r[10].s64 = ctx.r[30].s64 + -16;
	// 8241C370: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C374: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C378: 4BFA5669  bl 0x823c19e0
	ctx.lr = 0x8241C37C;
	sub_823C19E0(ctx, base);
	// 8241C37C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241C380: 40820014  bne 0x8241c394
	if !ctx.cr[0].eq {
	pc = 0x8241C394; continue 'dispatch;
	}
	// 8241C384: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C388: 386B01A0  addi r3, r11, 0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + 416;
	// 8241C38C: 4800087D  bl 0x8241cc08
	ctx.lr = 0x8241C390;
	sub_8241CC08(ctx, base);
	// 8241C390: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8241C394: 397EFFA4  addi r11, r30, -0x5c
	ctx.r[11].s64 = ctx.r[30].s64 + -92;
	// 8241C398: 395EFFF8  addi r10, r30, -8
	ctx.r[10].s64 = ctx.r[30].s64 + -8;
	// 8241C39C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241C3A0: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C3A4: 4BFA563D  bl 0x823c19e0
	ctx.lr = 0x8241C3A8;
	sub_823C19E0(ctx, base);
	// 8241C3A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241C3AC: 40820014  bne 0x8241c3c0
	if !ctx.cr[0].eq {
	pc = 0x8241C3C0; continue 'dispatch;
	}
	// 8241C3B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C3B4: 386B0158  addi r3, r11, 0x158
	ctx.r[3].s64 = ctx.r[11].s64 + 344;
	// 8241C3B8: 48000851  bl 0x8241cc08
	ctx.lr = 0x8241C3BC;
	sub_8241CC08(ctx, base);
	// 8241C3BC: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8241C3C0: 397EFFA4  addi r11, r30, -0x5c
	ctx.r[11].s64 = ctx.r[30].s64 + -92;
	// 8241C3C4: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241C3C8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C3CC: 4BFA5615  bl 0x823c19e0
	ctx.lr = 0x8241C3D0;
	sub_823C19E0(ctx, base);
	// 8241C3D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241C3D4: 40820014  bne 0x8241c3e8
	if !ctx.cr[0].eq {
	pc = 0x8241C3E8; continue 'dispatch;
	}
	// 8241C3D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C3DC: 386B0110  addi r3, r11, 0x110
	ctx.r[3].s64 = ctx.r[11].s64 + 272;
	// 8241C3E0: 48000829  bl 0x8241cc08
	ctx.lr = 0x8241C3E4;
	sub_8241CC08(ctx, base);
	// 8241C3E4: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8241C3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241C3EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241C3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C3F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241C3FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241C400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C408 size=164
    let mut pc: u32 = 0x8241C408;
    'dispatch: loop {
        match pc {
            0x8241C408 => {
    //   block [0x8241C408..0x8241C4AC)
	// 8241C408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C410: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C414: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C418: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C41C: 3BEBF474  addi r31, r11, -0xb8c
	ctx.r[31].s64 = ctx.r[11].s64 + -2956;
	// 8241C420: 397FFFA4  addi r11, r31, -0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + -92;
	// 8241C424: 395FFFF0  addi r10, r31, -0x10
	ctx.r[10].s64 = ctx.r[31].s64 + -16;
	// 8241C428: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241C42C: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C430: 4BFA5799  bl 0x823c1bc8
	ctx.lr = 0x8241C434;
	sub_823C1BC8(ctx, base);
	// 8241C434: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241C438: 409A0010  bne cr6, 0x8241c448
	if !ctx.cr[6].eq {
	pc = 0x8241C448; continue 'dispatch;
	}
	// 8241C43C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C440: 386B0288  addi r3, r11, 0x288
	ctx.r[3].s64 = ctx.r[11].s64 + 648;
	// 8241C444: 480007C5  bl 0x8241cc08
	ctx.lr = 0x8241C448;
	sub_8241CC08(ctx, base);
	// 8241C448: 397FFFA4  addi r11, r31, -0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + -92;
	// 8241C44C: 395FFFF8  addi r10, r31, -8
	ctx.r[10].s64 = ctx.r[31].s64 + -8;
	// 8241C450: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241C454: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C458: 4BFA5771  bl 0x823c1bc8
	ctx.lr = 0x8241C45C;
	sub_823C1BC8(ctx, base);
	// 8241C45C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241C460: 409A0010  bne cr6, 0x8241c470
	if !ctx.cr[6].eq {
	pc = 0x8241C470; continue 'dispatch;
	}
	// 8241C464: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C468: 386B0238  addi r3, r11, 0x238
	ctx.r[3].s64 = ctx.r[11].s64 + 568;
	// 8241C46C: 4800079D  bl 0x8241cc08
	ctx.lr = 0x8241C470;
	sub_8241CC08(ctx, base);
	// 8241C470: 397FFFA4  addi r11, r31, -0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + -92;
	// 8241C474: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241C478: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C47C: 4BFA574D  bl 0x823c1bc8
	ctx.lr = 0x8241C480;
	sub_823C1BC8(ctx, base);
	// 8241C480: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241C484: 409A0010  bne cr6, 0x8241c494
	if !ctx.cr[6].eq {
	pc = 0x8241C494; continue 'dispatch;
	}
	// 8241C488: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C48C: 386B01E8  addi r3, r11, 0x1e8
	ctx.r[3].s64 = ctx.r[11].s64 + 488;
	// 8241C490: 48000779  bl 0x8241cc08
	ctx.lr = 0x8241C494;
	sub_8241CC08(ctx, base);
	// 8241C494: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C4A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241C4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241C4B0 size=32
    let mut pc: u32 = 0x8241C4B0;
    'dispatch: loop {
        match pc {
            0x8241C4B0 => {
    //   block [0x8241C4B0..0x8241C4D0)
	// 8241C4B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8241C4B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C4B8: 419A0010  beq cr6, 0x8241c4c8
	if ctx.cr[6].eq {
	pc = 0x8241C4C8; continue 'dispatch;
	}
	// 8241C4BC: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8241C4C0: 814A3C14  lwz r10, 0x3c14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(15380 as u32) ) } as u64;
	// 8241C4C4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241C4C8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8241C4CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241C4D0 size=24
    let mut pc: u32 = 0x8241C4D0;
    'dispatch: loop {
        match pc {
            0x8241C4D0 => {
    //   block [0x8241C4D0..0x8241C4E8)
	// 8241C4D0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C4D4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8241C4D8: 396BF418  addi r11, r11, -0xbe8
	ctx.r[11].s64 = ctx.r[11].s64 + -3048;
	// 8241C4DC: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8241C4E0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8241C4E4: 4811866C  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241C4E8 size=4
    let mut pc: u32 = 0x8241C4E8;
    'dispatch: loop {
        match pc {
            0x8241C4E8 => {
    //   block [0x8241C4E8..0x8241C4EC)
	// 8241C4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C4F0 size=84
    let mut pc: u32 = 0x8241C4F0;
    'dispatch: loop {
        match pc {
            0x8241C4F0 => {
    //   block [0x8241C4F0..0x8241C544)
	// 8241C4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C4FC: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C500: 390BF414  addi r8, r11, -0xbec
	ctx.r[8].s64 = ctx.r[11].s64 + -3052;
	// 8241C504: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241C508: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241C50C: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241C510: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8241C514: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241C518: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241C51C: 4082FFE8  bne 0x8241c504
	if !ctx.cr[0].eq {
	pc = 0x8241C504; continue 'dispatch;
	}
	// 8241C520: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241C524: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241C528: 409A000C  bne cr6, 0x8241c534
	if !ctx.cr[6].eq {
	pc = 0x8241C534; continue 'dispatch;
	}
	// 8241C52C: 4BFFFC0D  bl 0x8241c138
	ctx.lr = 0x8241C530;
	sub_8241C138(ctx, base);
	// 8241C530: 480094D9  bl 0x82425a08
	ctx.lr = 0x8241C534;
	sub_82425A08(ctx, base);
	// 8241C534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C548 size=100
    let mut pc: u32 = 0x8241C548;
    'dispatch: loop {
        match pc {
            0x8241C548 => {
    //   block [0x8241C548..0x8241C5AC)
	// 8241C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C554: 48004ADD  bl 0x82421030
	ctx.lr = 0x8241C558;
	sub_82421030(ctx, base);
	// 8241C558: 4BFA3681  bl 0x823bfbd8
	ctx.lr = 0x8241C55C;
	sub_823BFBD8(ctx, base);
	// 8241C55C: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C560: 396BF47C  addi r11, r11, -0xb84
	ctx.r[11].s64 = ctx.r[11].s64 + -2948;
	// 8241C564: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 8241C568: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C56C: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8241C570: 40980010  bge cr6, 0x8241c580
	if !ctx.cr[6].lt {
	pc = 0x8241C580; continue 'dispatch;
	}
	// 8241C574: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C578: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241C57C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241C580: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 8241C584: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8241C588: 906A0000  stw r3, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8241C58C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C590: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8241C594: 48004ADD  bl 0x82421070
	ctx.lr = 0x8241C598;
	sub_82421070(ctx, base);
	// 8241C598: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241C59C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C5B0 size=172
    let mut pc: u32 = 0x8241C5B0;
    'dispatch: loop {
        match pc {
            0x8241C5B0 => {
    //   block [0x8241C5B0..0x8241C65C)
	// 8241C5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C5B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C5BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C5C0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C5C4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8241C5C8: 38EBF410  addi r7, r11, -0xbf0
	ctx.r[7].s64 = ctx.r[11].s64 + -3056;
	// 8241C5CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8241C5D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241C5D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241C5D8: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241C5DC: 7D00392D  stwcx. r8, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241C5E0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241C5E4: 4082FFEC  bne 0x8241c5d0
	if !ctx.cr[0].eq {
	pc = 0x8241C5D0; continue 'dispatch;
	}
	// 8241C5E8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241C5EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241C5F0: 409A0018  bne cr6, 0x8241c608
	if !ctx.cr[6].eq {
	pc = 0x8241C608; continue 'dispatch;
	}
	// 8241C5F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C5F8: 386B0300  addi r3, r11, 0x300
	ctx.r[3].s64 = ctx.r[11].s64 + 768;
	// 8241C5FC: 4800060D  bl 0x8241cc08
	ctx.lr = 0x8241C600;
	sub_8241CC08(ctx, base);
	// 8241C600: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C604: 48000044  b 0x8241c648
	pc = 0x8241C648; continue 'dispatch;
	// 8241C608: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8241C60C: 816B3C14  lwz r11, 0x3c14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15380 as u32) ) } as u64;
	// 8241C610: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8241C614: 41980024  blt cr6, 0x8241c638
	if ctx.cr[6].lt {
	pc = 0x8241C638; continue 'dispatch;
	}
	// 8241C618: 419A0024  beq cr6, 0x8241c63c
	if ctx.cr[6].eq {
	pc = 0x8241C63C; continue 'dispatch;
	}
	// 8241C61C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8241C620: 41980018  blt cr6, 0x8241c638
	if ctx.cr[6].lt {
	pc = 0x8241C638; continue 'dispatch;
	}
	// 8241C624: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C628: 386B02D4  addi r3, r11, 0x2d4
	ctx.r[3].s64 = ctx.r[11].s64 + 724;
	// 8241C62C: 480005DD  bl 0x8241cc08
	ctx.lr = 0x8241C630;
	sub_8241CC08(ctx, base);
	// 8241C630: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241C634: 48000008  b 0x8241c63c
	pc = 0x8241C63C; continue 'dispatch;
	// 8241C638: 4BFFFEB9  bl 0x8241c4f0
	ctx.lr = 0x8241C63C;
	sub_8241C4F0(ctx, base);
	// 8241C63C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241C640: 4800F251  bl 0x8242b890
	ctx.lr = 0x8241C644;
	sub_8242B890(ctx, base);
	// 8241C644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241C648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241C658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C660 size=104
    let mut pc: u32 = 0x8241C660;
    'dispatch: loop {
        match pc {
            0x8241C660 => {
    //   block [0x8241C660..0x8241C6C8)
	// 8241C660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C66C: 4BFFFEDD  bl 0x8241c548
	ctx.lr = 0x8241C670;
	sub_8241C548(ctx, base);
	// 8241C670: 1D23003C  mulli r9, r3, 0x3c
	ctx.r[9].s64 = ctx.r[3].s64 * 60;
	// 8241C674: 396003E8  li r11, 0x3e8
	ctx.r[11].s64 = 1000;
	// 8241C678: 3940003C  li r10, 0x3c
	ctx.r[10].s64 = 60;
	// 8241C67C: 7D695BD2  divd r11, r9, r11
	ctx.r[11].s64 = ctx.r[9].s64 / ctx.r[11].s64;
	// 8241C680: 5469003E  slwi r9, r3, 0
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8241C684: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241C688: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 8241C68C: 7D6B53D2  divd r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[10].s64;
	// 8241C690: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241C694: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8241C698: 346B0001  addic. r3, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8241C69C: 4082000C  bne 0x8241c6a8
	if !ctx.cr[0].eq {
	pc = 0x8241C6A8; continue 'dispatch;
	}
	// 8241C6A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241C6A4: 48000010  b 0x8241c6b4
	pc = 0x8241C6B4; continue 'dispatch;
	// 8241C6A8: 2B030011  cmplwi cr6, r3, 0x11
	ctx.cr[6].compare_u32(ctx.r[3].u32, 17 as u32, &mut ctx.xer);
	// 8241C6AC: 40990008  ble cr6, 0x8241c6b4
	if !ctx.cr[6].gt {
	pc = 0x8241C6B4; continue 'dispatch;
	}
	// 8241C6B0: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 8241C6B4: 4BFA3D15  bl 0x823c03c8
	ctx.lr = 0x8241C6B8;
	sub_823C03C8(ctx, base);
	// 8241C6B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C6C8 size=156
    let mut pc: u32 = 0x8241C6C8;
    'dispatch: loop {
        match pc {
            0x8241C6C8 => {
    //   block [0x8241C6C8..0x8241C764)
	// 8241C6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C6D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C6D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C6D8: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C6DC: 3BEBF44C  addi r31, r11, -0xbb4
	ctx.r[31].s64 = ctx.r[11].s64 + -2996;
	// 8241C6E0: 48000054  b 0x8241c734
	pc = 0x8241C734; continue 'dispatch;
	// 8241C6E4: 4BFFFF7D  bl 0x8241c660
	ctx.lr = 0x8241C6E8;
	sub_8241C660(ctx, base);
	// 8241C6E8: 397FFFE8  addi r11, r31, -0x18
	ctx.r[11].s64 = ctx.r[31].s64 + -24;
	// 8241C6EC: 395FFFE8  addi r10, r31, -0x18
	ctx.r[10].s64 = ctx.r[31].s64 + -24;
	// 8241C6F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C6F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241C6F8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C6FC: 4800962D  bl 0x82425d28
	ctx.lr = 0x8241C700;
	sub_82425D28(ctx, base);
	// 8241C700: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 8241C704: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C708: 4BFA5689  bl 0x823c1d90
	ctx.lr = 0x8241C70C;
	sub_823C1D90(ctx, base);
	// 8241C70C: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8241C710: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C718: 419A001C  beq cr6, 0x8241c734
	if ctx.cr[6].eq {
	pc = 0x8241C734; continue 'dispatch;
	}
	// 8241C71C: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8241C720: 395FFFF4  addi r10, r31, -0xc
	ctx.r[10].s64 = ctx.r[31].s64 + -12;
	// 8241C724: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241C728: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241C730: 4E800421  bctrl
	ctx.lr = 0x8241C734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241C734: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241C738: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C73C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C740: 419AFFA4  beq cr6, 0x8241c6e4
	if ctx.cr[6].eq {
	pc = 0x8241C6E4; continue 'dispatch;
	}
	// 8241C744: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241C748: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C74C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C75C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241C760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C768 size=84
    let mut pc: u32 = 0x8241C768;
    'dispatch: loop {
        match pc {
            0x8241C768 => {
    //   block [0x8241C768..0x8241C7BC)
	// 8241C768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C778: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C77C: 3BEBF454  addi r31, r11, -0xbac
	ctx.r[31].s64 = ctx.r[11].s64 + -2988;
	// 8241C780: 4800000C  b 0x8241c78c
	pc = 0x8241C78C; continue 'dispatch;
	// 8241C784: 4BFFFEDD  bl 0x8241c660
	ctx.lr = 0x8241C788;
	sub_8241C660(ctx, base);
	// 8241C788: 480095B1  bl 0x82425d38
	ctx.lr = 0x8241C78C;
	sub_82425D38(ctx, base);
	// 8241C78C: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241C790: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C794: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C798: 419AFFEC  beq cr6, 0x8241c784
	if ctx.cr[6].eq {
	pc = 0x8241C784; continue 'dispatch;
	}
	// 8241C79C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241C7A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C7A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C7A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C7B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241C7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C7C0 size=260
    let mut pc: u32 = 0x8241C7C0;
    'dispatch: loop {
        match pc {
            0x8241C7C0 => {
    //   block [0x8241C7C0..0x8241C8C4)
	// 8241C7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C7C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C7CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C7D0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C7D4: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8241C7D8: 3BEBF470  addi r31, r11, -0xb90
	ctx.r[31].s64 = ctx.r[11].s64 + -2960;
	// 8241C7DC: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241C7E0: 391FFFF0  addi r8, r31, -0x10
	ctx.r[8].s64 = ctx.r[31].s64 + -16;
	// 8241C7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241C7E8: 38ABC6C8  addi r5, r11, -0x3938
	ctx.r[5].s64 = ctx.r[11].s64 + -14648;
	// 8241C7EC: 38803000  li r4, 0x3000
	ctx.r[4].s64 = 12288;
	// 8241C7F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C7F4: 4BFA55DD  bl 0x823c1dd0
	ctx.lr = 0x8241C7F8;
	sub_823C1DD0(ctx, base);
	// 8241C7F8: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8241C7FC: 395FFFF4  addi r10, r31, -0xc
	ctx.r[10].s64 = ctx.r[31].s64 + -12;
	// 8241C800: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8241C804: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C808: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C80C: 409A0018  bne cr6, 0x8241c824
	if !ctx.cr[6].eq {
	pc = 0x8241C824; continue 'dispatch;
	}
	// 8241C810: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C814: 386B03E0  addi r3, r11, 0x3e0
	ctx.r[3].s64 = ctx.r[11].s64 + 992;
	// 8241C818: 480003F1  bl 0x8241cc08
	ctx.lr = 0x8241C81C;
	sub_8241CC08(ctx, base);
	// 8241C81C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241C820: 48000090  b 0x8241c8b0
	pc = 0x8241C8B0; continue 'dispatch;
	// 8241C824: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241C828: 391FFFF8  addi r8, r31, -8
	ctx.r[8].s64 = ctx.r[31].s64 + -8;
	// 8241C82C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8241C830: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241C834: 38ABC768  addi r5, r11, -0x3898
	ctx.r[5].s64 = ctx.r[11].s64 + -14488;
	// 8241C838: 38803000  li r4, 0x3000
	ctx.r[4].s64 = 12288;
	// 8241C83C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C840: 4BFA5591  bl 0x823c1dd0
	ctx.lr = 0x8241C844;
	sub_823C1DD0(ctx, base);
	// 8241C844: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8241C848: 395FFFFC  addi r10, r31, -4
	ctx.r[10].s64 = ctx.r[31].s64 + -4;
	// 8241C84C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8241C850: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C858: 409A0010  bne cr6, 0x8241c868
	if !ctx.cr[6].eq {
	pc = 0x8241C868; continue 'dispatch;
	}
	// 8241C85C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C860: 386B0398  addi r3, r11, 0x398
	ctx.r[3].s64 = ctx.r[11].s64 + 920;
	// 8241C864: 4BFFFFB4  b 0x8241c818
	pc = 0x8241C818; continue 'dispatch;
	// 8241C868: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241C86C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8241C870: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8241C874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241C878: 38ABC030  addi r5, r11, -0x3fd0
	ctx.r[5].s64 = ctx.r[11].s64 + -16336;
	// 8241C87C: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 8241C880: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C884: 4BFA554D  bl 0x823c1dd0
	ctx.lr = 0x8241C888;
	sub_823C1DD0(ctx, base);
	// 8241C888: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8241C88C: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8241C890: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8241C894: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241C898: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241C89C: 409A0010  bne cr6, 0x8241c8ac
	if !ctx.cr[6].eq {
	pc = 0x8241C8AC; continue 'dispatch;
	}
	// 8241C8A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241C8A4: 386B0350  addi r3, r11, 0x350
	ctx.r[3].s64 = ctx.r[11].s64 + 848;
	// 8241C8A8: 4BFFFF70  b 0x8241c818
	pc = 0x8241C818; continue 'dispatch;
	// 8241C8AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241C8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241C8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241C8BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241C8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241C8C8 size=412
    let mut pc: u32 = 0x8241C8C8;
    'dispatch: loop {
        match pc {
            0x8241C8C8 => {
    //   block [0x8241C8C8..0x8241CA64)
	// 8241C8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241C8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241C8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241C8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241C8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241C8DC: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241C8E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241C8E4: 3BEBF474  addi r31, r11, -0xb8c
	ctx.r[31].s64 = ctx.r[11].s64 + -2956;
	// 8241C8E8: 391FFFA0  addi r8, r31, -0x60
	ctx.r[8].s64 = ctx.r[31].s64 + -96;
	// 8241C8EC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241C8F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241C8F4: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241C8F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241C8FC: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241C900: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241C904: 4082FFE8  bne 0x8241c8ec
	if !ctx.cr[0].eq {
	pc = 0x8241C8EC; continue 'dispatch;
	}
	// 8241C908: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241C90C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241C910: 409A013C  bne cr6, 0x8241ca4c
	if !ctx.cr[6].eq {
	pc = 0x8241CA4C; continue 'dispatch;
	}
	// 8241C914: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241C918: 4BFA5299  bl 0x823c1bb0
	ctx.lr = 0x8241C91C;
	sub_823C1BB0(ctx, base);
	// 8241C91C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241C920: 395FFFD4  addi r10, r31, -0x2c
	ctx.r[10].s64 = ctx.r[31].s64 + -44;
	// 8241C924: 393FFFD8  addi r9, r31, -0x28
	ctx.r[9].s64 = ctx.r[31].s64 + -40;
	// 8241C928: 391FFFE4  addi r8, r31, -0x1c
	ctx.r[8].s64 = ctx.r[31].s64 + -28;
	// 8241C92C: 38FFFFE8  addi r7, r31, -0x18
	ctx.r[7].s64 = ctx.r[31].s64 + -24;
	// 8241C930: 38DFFFC0  addi r6, r31, -0x40
	ctx.r[6].s64 = ctx.r[31].s64 + -64;
	// 8241C934: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C938: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C93C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C940: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C944: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241C948: 48009041  bl 0x82425988
	ctx.lr = 0x8241C94C;
	sub_82425988(ctx, base);
	// 8241C94C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241C950: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C954: 48008F65  bl 0x824258b8
	ctx.lr = 0x8241C958;
	sub_824258B8(ctx, base);
	// 8241C958: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241C95C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241C960: 48008F71  bl 0x824258d0
	ctx.lr = 0x8241C964;
	sub_824258D0(ctx, base);
	// 8241C964: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241C968: 386BBF70  addi r3, r11, -0x4090
	ctx.r[3].s64 = ctx.r[11].s64 + -16528;
	// 8241C96C: 4800911D  bl 0x82425a88
	ctx.lr = 0x8241C970;
	sub_82425A88(ctx, base);
	// 8241C970: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8241C974: 409A004C  bne cr6, 0x8241c9c0
	if !ctx.cr[6].eq {
	pc = 0x8241C9C0; continue 'dispatch;
	}
	// 8241C978: 397FFFA4  addi r11, r31, -0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + -92;
	// 8241C97C: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8241C980: 395FFFA4  addi r10, r31, -0x5c
	ctx.r[10].s64 = ctx.r[31].s64 + -92;
	// 8241C984: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8241C988: 393FFFA4  addi r9, r31, -0x5c
	ctx.r[9].s64 = ctx.r[31].s64 + -92;
	// 8241C98C: 38FFFFA4  addi r7, r31, -0x5c
	ctx.r[7].s64 = ctx.r[31].s64 + -92;
	// 8241C990: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8241C994: 3900FFF1  li r8, -0xf
	ctx.r[8].s64 = -15;
	// 8241C998: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8241C99C: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8241C9A0: 38BFFFA4  addi r5, r31, -0x5c
	ctx.r[5].s64 = ctx.r[31].s64 + -92;
	// 8241C9A4: 389FFFA4  addi r4, r31, -0x5c
	ctx.r[4].s64 = ctx.r[31].s64 + -92;
	// 8241C9A8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8241C9AC: 91090008  stw r8, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8241C9B0: 9167000C  stw r11, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241C9B4: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8241C9B8: 91440014  stw r10, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8241C9BC: 48000014  b 0x8241c9d0
	pc = 0x8241C9D0; continue 'dispatch;
	// 8241C9C0: 387FFFA4  addi r3, r31, -0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + -92;
	// 8241C9C4: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8241C9C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241C9CC: 48118185  bl 0x82534b50
	ctx.lr = 0x8241C9D0;
	sub_82534B50(ctx, base);
	// 8241C9D0: 4BFFFDF1  bl 0x8241c7c0
	ctx.lr = 0x8241C9D4;
	sub_8241C7C0(ctx, base);
	// 8241C9D4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241C9D8: 40800010  bge 0x8241c9e8
	if !ctx.cr[0].lt {
	pc = 0x8241C9E8; continue 'dispatch;
	}
	// 8241C9DC: 4BFFF75D  bl 0x8241c138
	ctx.lr = 0x8241C9E0;
	sub_8241C138(ctx, base);
	// 8241C9E0: 48009029  bl 0x82425a08
	ctx.lr = 0x8241C9E4;
	sub_82425A08(ctx, base);
	// 8241C9E4: 48000068  b 0x8241ca4c
	pc = 0x8241CA4C; continue 'dispatch;
	// 8241C9E8: 4BFFF961  bl 0x8241c348
	ctx.lr = 0x8241C9EC;
	sub_8241C348(ctx, base);
	// 8241C9EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241C9F0: 4180FFEC  blt 0x8241c9dc
	if ctx.cr[0].lt {
	pc = 0x8241C9DC; continue 'dispatch;
	}
	// 8241C9F4: 397FFFA4  addi r11, r31, -0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + -92;
	// 8241C9F8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241C9FC: 480007B5  bl 0x8241d1b0
	ctx.lr = 0x8241CA00;
	sub_8241D1B0(ctx, base);
	// 8241CA00: 4BFFFA09  bl 0x8241c408
	ctx.lr = 0x8241CA04;
	sub_8241C408(ctx, base);
	// 8241CA04: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241CA08: 4180FFD4  blt 0x8241c9dc
	if ctx.cr[0].lt {
	pc = 0x8241C9DC; continue 'dispatch;
	}
	// 8241CA0C: 397FFFA4  addi r11, r31, -0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + -92;
	// 8241CA10: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241CA14: 48000805  bl 0x8241d218
	ctx.lr = 0x8241CA18;
	sub_8241D218(ctx, base);
	// 8241CA18: 397FFFF0  addi r11, r31, -0x10
	ctx.r[11].s64 = ctx.r[31].s64 + -16;
	// 8241CA1C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CA20: 4BFA5371  bl 0x823c1d90
	ctx.lr = 0x8241CA24;
	sub_823C1D90(ctx, base);
	// 8241CA24: 397FFFF8  addi r11, r31, -8
	ctx.r[11].s64 = ctx.r[31].s64 + -8;
	// 8241CA28: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CA2C: 4BFA5365  bl 0x823c1d90
	ctx.lr = 0x8241CA30;
	sub_823C1D90(ctx, base);
	// 8241CA30: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CA34: 4BFA535D  bl 0x823c1d90
	ctx.lr = 0x8241CA38;
	sub_823C1D90(ctx, base);
	// 8241CA38: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241CA3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241CA40: 388BBFA0  addi r4, r11, -0x4060
	ctx.r[4].s64 = ctx.r[11].s64 + -16480;
	// 8241CA44: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8241CA48: 480091E9  bl 0x82425c30
	ctx.lr = 0x8241CA4C;
	sub_82425C30(ctx, base);
	// 8241CA4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241CA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241CA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241CA58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241CA5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241CA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241CA68 size=184
    let mut pc: u32 = 0x8241CA68;
    'dispatch: loop {
        match pc {
            0x8241CA68 => {
    //   block [0x8241CA68..0x8241CB20)
	// 8241CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241CA6C: 48118651  bl 0x825350bc
	ctx.lr = 0x8241CA70;
	sub_82535080(ctx, base);
	// 8241CA70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241CA74: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241CA78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241CA7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241CA80: 390BF410  addi r8, r11, -0xbf0
	ctx.r[8].s64 = ctx.r[11].s64 + -3056;
	// 8241CA84: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8241CA88: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241CA8C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241CA90: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241CA94: 7FC0412D  stwcx. r30, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[30].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241CA98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241CA9C: 4082FFEC  bne 0x8241ca88
	if !ctx.cr[0].eq {
	pc = 0x8241CA88; continue 'dispatch;
	}
	// 8241CAA0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241CAA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CAA8: 419A0018  beq cr6, 0x8241cac0
	if ctx.cr[6].eq {
	pc = 0x8241CAC0; continue 'dispatch;
	}
	// 8241CAAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241CAB0: 386B044C  addi r3, r11, 0x44c
	ctx.r[3].s64 = ctx.r[11].s64 + 1100;
	// 8241CAB4: 48000155  bl 0x8241cc08
	ctx.lr = 0x8241CAB8;
	sub_8241CC08(ctx, base);
	// 8241CAB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241CABC: 4800005C  b 0x8241cb18
	pc = 0x8241CB18; continue 'dispatch;
	// 8241CAC0: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8241CAC4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8241CAC8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8241CACC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8241CAD0: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8241CAD4: 4800EDBD  bl 0x8242b890
	ctx.lr = 0x8241CAD8;
	sub_8242B890(ctx, base);
	// 8241CAD8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8241CADC: 93EB3C14  stw r31, 0x3c14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(15380 as u32), ctx.r[31].u32 ) };
	// 8241CAE0: 816B3C14  lwz r11, 0x3c14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15380 as u32) ) } as u64;
	// 8241CAE4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8241CAE8: 41980024  blt cr6, 0x8241cb0c
	if ctx.cr[6].lt {
	pc = 0x8241CB0C; continue 'dispatch;
	}
	// 8241CAEC: 419A0028  beq cr6, 0x8241cb14
	if ctx.cr[6].eq {
	pc = 0x8241CB14; continue 'dispatch;
	}
	// 8241CAF0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8241CAF4: 41980018  blt cr6, 0x8241cb0c
	if ctx.cr[6].lt {
	pc = 0x8241CB0C; continue 'dispatch;
	}
	// 8241CAF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241CAFC: 386B0424  addi r3, r11, 0x424
	ctx.r[3].s64 = ctx.r[11].s64 + 1060;
	// 8241CB00: 48000109  bl 0x8241cc08
	ctx.lr = 0x8241CB04;
	sub_8241CC08(ctx, base);
	// 8241CB04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241CB08: 4800000C  b 0x8241cb14
	pc = 0x8241CB14; continue 'dispatch;
	// 8241CB0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241CB10: 4BFFFDB9  bl 0x8241c8c8
	ctx.lr = 0x8241CB14;
	sub_8241C8C8(ctx, base);
	// 8241CB14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241CB18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241CB1C: 481185F0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CB20 size=48
    let mut pc: u32 = 0x8241CB20;
    'dispatch: loop {
        match pc {
            0x8241CB20 => {
    //   block [0x8241CB20..0x8241CB50)
	// 8241CB20: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241CB24: 390BF480  addi r8, r11, -0xb80
	ctx.r[8].s64 = ctx.r[11].s64 + -2944;
	// 8241CB28: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241CB2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241CB30: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241CB34: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241CB38: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241CB3C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241CB40: 4082FFE8  bne 0x8241cb28
	if !ctx.cr[0].eq {
	pc = 0x8241CB28; continue 'dispatch;
	}
	// 8241CB44: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241CB48: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241CB4C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CB50 size=8
    let mut pc: u32 = 0x8241CB50;
    'dispatch: loop {
        match pc {
            0x8241CB50 => {
    //   block [0x8241CB50..0x8241CB58)
	// 8241CB50: 4800F068  b 0x8242bbb8
	sub_8242BBB8(ctx, base);
	return;
	// 8241CB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CB58 size=48
    let mut pc: u32 = 0x8241CB58;
    'dispatch: loop {
        match pc {
            0x8241CB58 => {
    //   block [0x8241CB58..0x8241CB88)
	// 8241CB58: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241CB5C: 390BF480  addi r8, r11, -0xb80
	ctx.r[8].s64 = ctx.r[11].s64 + -2944;
	// 8241CB60: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241CB64: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241CB68: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241CB6C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8241CB70: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241CB74: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241CB78: 4082FFE8  bne 0x8241cb60
	if !ctx.cr[0].eq {
	pc = 0x8241CB60; continue 'dispatch;
	}
	// 8241CB7C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241CB80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CB84: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CB88 size=8
    let mut pc: u32 = 0x8241CB88;
    'dispatch: loop {
        match pc {
            0x8241CB88 => {
    //   block [0x8241CB88..0x8241CB90)
	// 8241CB88: 4800F0F0  b 0x8242bc78
	sub_8242BC78(ctx, base);
	return;
	// 8241CB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241CB90 size=40
    let mut pc: u32 = 0x8241CB90;
    'dispatch: loop {
        match pc {
            0x8241CB90 => {
    //   block [0x8241CB90..0x8241CBB8)
	// 8241CB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241CB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241CB98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241CB9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241CBA0: 4800F1A9  bl 0x8242bd48
	ctx.lr = 0x8241CBA4;
	sub_8242BD48(ctx, base);
	// 8241CBA4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241CBA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241CBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241CBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241CBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241CBB8 size=76
    let mut pc: u32 = 0x8241CBB8;
    'dispatch: loop {
        match pc {
            0x8241CBB8 => {
    //   block [0x8241CBB8..0x8241CC04)
	// 8241CBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241CBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241CBC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241CBC4: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241CBC8: 816BF480  lwz r11, -0xb80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2944 as u32) ) } as u64;
	// 8241CBCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CBD0: 41990018  bgt cr6, 0x8241cbe8
	if ctx.cr[6].gt {
	pc = 0x8241CBE8; continue 'dispatch;
	}
	// 8241CBD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241CBD8: 386B0490  addi r3, r11, 0x490
	ctx.r[3].s64 = ctx.r[11].s64 + 1168;
	// 8241CBDC: 4800002D  bl 0x8241cc08
	ctx.lr = 0x8241CBE0;
	sub_8241CC08(ctx, base);
	// 8241CBE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241CBE4: 48000010  b 0x8241cbf4
	pc = 0x8241CBF4; continue 'dispatch;
	// 8241CBE8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8241CBEC: 4800F33D  bl 0x8242bf28
	ctx.lr = 0x8241CBF0;
	sub_8242BF28(ctx, base);
	// 8241CBF0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241CBF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241CBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241CBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241CC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CC08 size=28
    let mut pc: u32 = 0x8241CC08;
    'dispatch: loop {
        match pc {
            0x8241CC08 => {
    //   block [0x8241CC08..0x8241CC24)
	// 8241CC08: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241CC0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8241CC10: 396BF488  addi r11, r11, -0xb78
	ctx.r[11].s64 = ctx.r[11].s64 + -2936;
	// 8241CC14: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 8241CC18: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CC1C: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8241CC20: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CC24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CC24 size=20
    let mut pc: u32 = 0x8241CC24;
    'dispatch: loop {
        match pc {
            0x8241CC24 => {
    //   block [0x8241CC24..0x8241CC38)
	// 8241CC24: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8241CC28: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CC2C: 816A3C18  lwz r11, 0x3c18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(15384 as u32) ) } as u64;
	// 8241CC30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241CC34: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CC38 size=4
    let mut pc: u32 = 0x8241CC38;
    'dispatch: loop {
        match pc {
            0x8241CC38 => {
    //   block [0x8241CC38..0x8241CC3C)
	// 8241CC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CC40 size=52
    let mut pc: u32 = 0x8241CC40;
    'dispatch: loop {
        match pc {
            0x8241CC40 => {
    //   block [0x8241CC40..0x8241CC74)
	// 8241CC40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241CC44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241CC48: 394B04F8  addi r10, r11, 0x4f8
	ctx.r[10].s64 = ctx.r[11].s64 + 1272;
	// 8241CC4C: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241CC50: 396BF484  addi r11, r11, -0xb7c
	ctx.r[11].s64 = ctx.r[11].s64 + -2940;
	// 8241CC54: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241CC58: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8241CC5C: 409A0018  bne cr6, 0x8241cc74
	if !ctx.cr[6].eq {
		sub_8241CC74(ctx, base);
		return;
	}
	// 8241CC60: 3D408243  lis r10, -0x7dbd
	ctx.r[10].s64 = -2109538304;
	// 8241CC64: 3D208273  lis r9, -0x7d8d
	ctx.r[9].s64 = -2106392576;
	// 8241CC68: 396AEF50  addi r11, r10, -0x10b0
	ctx.r[11].s64 = ctx.r[10].s64 + -4272;
	// 8241CC6C: 91693C18  stw r11, 0x3c18(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(15384 as u32), ctx.r[11].u32 ) };
	// 8241CC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CC74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CC74 size=12
    let mut pc: u32 = 0x8241CC74;
    'dispatch: loop {
        match pc {
            0x8241CC74 => {
    //   block [0x8241CC74..0x8241CC80)
	// 8241CC74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8241CC78: 906B3C18  stw r3, 0x3c18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(15384 as u32), ctx.r[3].u32 ) };
	// 8241CC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241CC80 size=12
    let mut pc: u32 = 0x8241CC80;
    'dispatch: loop {
        match pc {
            0x8241CC80 => {
    //   block [0x8241CC80..0x8241CC8C)
	// 8241CC80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241CC84: 386B0598  addi r3, r11, 0x598
	ctx.r[3].s64 = ctx.r[11].s64 + 1432;
	// 8241CC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241CC90 size=236
    let mut pc: u32 = 0x8241CC90;
    'dispatch: loop {
        match pc {
            0x8241CC90 => {
    //   block [0x8241CC90..0x8241CD7C)
	// 8241CC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241CC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241CC98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241CC9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241CCA0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241CCA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241CCA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8241CCAC: 3D006000  lis r8, 0x6000
	ctx.r[8].s64 = 1610612736;
	// 8241CCB0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8241CCB4: 816B87B4  lwz r11, -0x784c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30796 as u32) ) } as u64;
	// 8241CCB8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241CCBC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CCC0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8241CCC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CCC8: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 8241CCCC: 409A000C  bne cr6, 0x8241ccd8
	if !ctx.cr[6].eq {
	pc = 0x8241CCD8; continue 'dispatch;
	}
	// 8241CCD0: 4BFA4261  bl 0x823c0f30
	ctx.lr = 0x8241CCD4;
	sub_823C0F30(ctx, base);
	// 8241CCD4: 48000008  b 0x8241ccdc
	pc = 0x8241CCDC; continue 'dispatch;
	// 8241CCD8: 4BFA39B1  bl 0x823c0688
	ctx.lr = 0x8241CCDC;
	sub_823C0688(ctx, base);
	// 8241CCDC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8241CCE0: 5463003E  slwi r3, r3, 0
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8241CCE4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241CCE8: 409A001C  bne cr6, 0x8241cd04
	if !ctx.cr[6].eq {
	pc = 0x8241CD04; continue 'dispatch;
	}
	// 8241CCEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241CCF0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8241CCF4: 4BFA398D  bl 0x823c0680
	ctx.lr = 0x8241CCF8;
	sub_823C0680(ctx, base);
	// 8241CCF8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241CCFC: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8241CD00: 48000064  b 0x8241cd64
	pc = 0x8241CD64; continue 'dispatch;
	// 8241CD04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8241CD08: 4800F429  bl 0x8242c130
	ctx.lr = 0x8241CD0C;
	sub_8242C130(ctx, base);
	// 8241CD0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241CD10: 40820010  bne 0x8241cd20
	if !ctx.cr[0].eq {
	pc = 0x8241CD20; continue 'dispatch;
	}
	// 8241CD14: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241CD18: 4BFA3C41  bl 0x823c0958
	ctx.lr = 0x8241CD1C;
	sub_823C0958(ctx, base);
	// 8241CD1C: 4BFFFFD0  b 0x8241ccec
	pc = 0x8241CCEC; continue 'dispatch;
	// 8241CD20: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241CD24: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 8241CD28: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 8241CD2C: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8241CD30: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241CD34: 7D4A5E74  sradi r10, r10, 0xb
	ctx.xer.ca = (ctx.r[10].s64 < 0) && ((ctx.r[10].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[10].s64 >> 11;
	// 8241CD38: 7D695E74  sradi r9, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[9].s64 = ctx.r[11].s64 >> 11;
	// 8241CD3C: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8241CD40: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8241CD44: 79295D24  sldi r9, r9, 0xb
	ctx.r[9].u64 = ctx.r[9].u64.wrapping_shl(11);
	ctx.r[9].u32 = ctx.r[9].u64 as u32;
	// 8241CD48: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8241CD4C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8241CD50: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8241CD54: 4099000C  ble cr6, 0x8241cd60
	if !ctx.cr[6].gt {
	pc = 0x8241CD60; continue 'dispatch;
	}
	// 8241CD58: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8241CD5C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8241CD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241CD64: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8241CD68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241CD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241CD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241CD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241CD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241CD80 size=140
    let mut pc: u32 = 0x8241CD80;
    'dispatch: loop {
        match pc {
            0x8241CD80 => {
    //   block [0x8241CD80..0x8241CE0C)
	// 8241CD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241CD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241CD88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241CD8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241CD90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241CD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8241CD98: 3D006000  lis r8, 0x6000
	ctx.r[8].s64 = 1610612736;
	// 8241CD9C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8241CDA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241CDA4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8241CDA8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CDAC: 3C804000  lis r4, 0x4000
	ctx.r[4].s64 = 1073741824;
	// 8241CDB0: 4BFA38D9  bl 0x823c0688
	ctx.lr = 0x8241CDB4;
	sub_823C0688(ctx, base);
	// 8241CDB4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241CDB8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8241CDBC: 409A001C  bne cr6, 0x8241cdd8
	if !ctx.cr[6].eq {
	pc = 0x8241CDD8; continue 'dispatch;
	}
	// 8241CDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241CDC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8241CDC8: 4BFA38B9  bl 0x823c0680
	ctx.lr = 0x8241CDCC;
	sub_823C0680(ctx, base);
	// 8241CDCC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241CDD0: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8241CDD4: 48000020  b 0x8241cdf4
	pc = 0x8241CDF4; continue 'dispatch;
	// 8241CDD8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241CDDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241CDE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241CDE4: 48129585  bl 0x82546368
	ctx.lr = 0x8241CDE8;
	sub_82546368(ctx, base);
	// 8241CDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241CDEC: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8241CDF0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8241CDF4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8241CDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241CDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241CE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241CE04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241CE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241CE10 size=340
    let mut pc: u32 = 0x8241CE10;
    'dispatch: loop {
        match pc {
            0x8241CE10 => {
    //   block [0x8241CE10..0x8241CF64)
	// 8241CE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241CE14: 48118291  bl 0x825350a4
	ctx.lr = 0x8241CE18;
	sub_82535080(ctx, base);
	// 8241CE18: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241CE1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241CE20: 3EE00002  lis r23, 2
	ctx.r[23].s64 = 131072;
	// 8241CE24: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8241CE28: 83BF0024  lwz r29, 0x24(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8241CE2C: 839F0020  lwz r28, 0x20(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8241CE30: 7FAB8E70  srawi r11, r29, 0x11
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 17) as i64;
	// 8241CE34: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 8241CE38: 7F2B0194  addze r25, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[25].s64 = tmp.s64;
	// 8241CE3C: 7FAB8E70  srawi r11, r29, 0x11
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 17) as i64;
	// 8241CE40: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8241CE44: 7D6BB9D6  mullw r11, r11, r23
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[23].s32 as i64);
	// 8241CE48: 7D6BE851  subf. r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CE4C: 41820008  beq 0x8241ce54
	if ctx.cr[0].eq {
	pc = 0x8241CE54; continue 'dispatch;
	}
	// 8241CE50: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8241CE54: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 8241CE58: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8241CE5C: 409900E0  ble cr6, 0x8241cf3c
	if !ctx.cr[6].gt {
	pc = 0x8241CF3C; continue 'dispatch;
	}
	// 8241CE60: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8241CE64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CE68: 409A00E0  bne cr6, 0x8241cf48
	if !ctx.cr[6].eq {
	pc = 0x8241CF48; continue 'dispatch;
	}
	// 8241CE6C: 7F1DB800  cmpw cr6, r29, r23
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[23].s32, &mut ctx.xer);
	// 8241CE70: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8241CE74: 41980008  blt cr6, 0x8241ce7c
	if ctx.cr[6].lt {
	pc = 0x8241CE7C; continue 'dispatch;
	}
	// 8241CE78: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 8241CE7C: 396B07FF  addi r11, r11, 0x7ff
	ctx.r[11].s64 = ctx.r[11].s64 + 2047;
	// 8241CE80: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CE84: 3B7F0004  addi r27, r31, 4
	ctx.r[27].s64 = ctx.r[31].s64 + 4;
	// 8241CE88: 557E0028  rlwinm r30, r11, 0, 0, 0x14
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8241CE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241CE90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8241CE94: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8241CE98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241CE9C: 4BFA3535  bl 0x823c03d0
	ctx.lr = 0x8241CEA0;
	sub_823C03D0(ctx, base);
	// 8241CEA0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241CEA4: 40820020  bne 0x8241cec4
	if !ctx.cr[0].eq {
	pc = 0x8241CEC4; continue 'dispatch;
	}
	// 8241CEA8: 4BFA37D9  bl 0x823c0680
	ctx.lr = 0x8241CEAC;
	sub_823C0680(ctx, base);
	// 8241CEAC: 2B030026  cmplwi cr6, r3, 0x26
	ctx.cr[6].compare_u32(ctx.r[3].u32, 38 as u32, &mut ctx.xer);
	// 8241CEB0: 419A0014  beq cr6, 0x8241cec4
	if ctx.cr[6].eq {
	pc = 0x8241CEC4; continue 'dispatch;
	}
	// 8241CEB4: 2B0303E3  cmplwi cr6, r3, 0x3e3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 995 as u32, &mut ctx.xer);
	// 8241CEB8: 4099009C  ble cr6, 0x8241cf54
	if !ctx.cr[6].gt {
	pc = 0x8241CF54; continue 'dispatch;
	}
	// 8241CEBC: 2B0303E5  cmplwi cr6, r3, 0x3e5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 997 as u32, &mut ctx.xer);
	// 8241CEC0: 41990094  bgt cr6, 0x8241cf54
	if ctx.cr[6].gt {
	pc = 0x8241CF54; continue 'dispatch;
	}
	// 8241CEC4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8241CEC8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CECC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241CED0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8241CED4: 4812A02D  bl 0x82546f00
	ctx.lr = 0x8241CED8;
	sub_82546F00(ctx, base);
	// 8241CED8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241CEDC: 40820010  bne 0x8241ceec
	if !ctx.cr[0].eq {
	pc = 0x8241CEEC; continue 'dispatch;
	}
	// 8241CEE0: 4BFA37A1  bl 0x823c0680
	ctx.lr = 0x8241CEE4;
	sub_823C0680(ctx, base);
	// 8241CEE4: 2B030026  cmplwi cr6, r3, 0x26
	ctx.cr[6].compare_u32(ctx.r[3].u32, 38 as u32, &mut ctx.xer);
	// 8241CEE8: 409A0068  bne cr6, 0x8241cf50
	if !ctx.cr[6].eq {
	pc = 0x8241CF50; continue 'dispatch;
	}
	// 8241CEEC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241CEF0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241CEF4: 40990014  ble cr6, 0x8241cf08
	if !ctx.cr[6].gt {
	pc = 0x8241CF08; continue 'dispatch;
	}
	// 8241CEF8: 7CABF050  subf r5, r11, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8241CEFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241CF00: 7C7C5A14  add r3, r28, r11
	ctx.r[3].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 8241CF04: 481182CD  bl 0x825351d0
	ctx.lr = 0x8241CF08;
	sub_825351D0(ctx, base);
	// 8241CF08: E95F0018  ld r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 8241CF0C: 7BCB0020  clrldi r11, r30, 0x20
	ctx.r[11].u64 = ctx.r[30].u64 & 0x00000000FFFFFFFFu64;
	// 8241CF10: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8241CF14: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241CF18: 7F9EE214  add r28, r30, r28
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 8241CF1C: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8241CF20: 7F1AC800  cmpw cr6, r26, r25
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8241CF24: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 8241CF28: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241CF2C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241CF30: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241CF34: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8241CF38: 4198FF28  blt cr6, 0x8241ce60
	if ctx.cr[6].lt {
	pc = 0x8241CE60; continue 'dispatch;
	}
	// 8241CF3C: 931F0028  stw r24, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[24].u32 ) };
	// 8241CF40: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8241CF44: 481181B0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8241CF48: 931F0028  stw r24, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[24].u32 ) };
	// 8241CF4C: 4BFFFFF0  b 0x8241cf3c
	pc = 0x8241CF3C; continue 'dispatch;
	// 8241CF50: 4BFA3731  bl 0x823c0680
	ctx.lr = 0x8241CF54;
	sub_823C0680(ctx, base);
	// 8241CF54: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241CF58: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8241CF5C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8241CF60: 4BFFFFE0  b 0x8241cf40
	pc = 0x8241CF40; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241CF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241CF68 size=296
    let mut pc: u32 = 0x8241CF68;
    'dispatch: loop {
        match pc {
            0x8241CF68 => {
    //   block [0x8241CF68..0x8241D090)
	// 8241CF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241CF6C: 48118139  bl 0x825350a4
	ctx.lr = 0x8241CF70;
	sub_82535080(ctx, base);
	// 8241CF70: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241CF74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241CF78: 3EE00002  lis r23, 2
	ctx.r[23].s64 = 131072;
	// 8241CF7C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8241CF80: 83BF0024  lwz r29, 0x24(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8241CF84: 839F0020  lwz r28, 0x20(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8241CF88: 7FAB8E70  srawi r11, r29, 0x11
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 17) as i64;
	// 8241CF8C: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 8241CF90: 7F2B0194  addze r25, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[25].s64 = tmp.s64;
	// 8241CF94: 7FAB8E70  srawi r11, r29, 0x11
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 17) as i64;
	// 8241CF98: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8241CF9C: 7D6BB9D6  mullw r11, r11, r23
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[23].s32 as i64);
	// 8241CFA0: 7D6BE851  subf. r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CFA4: 41820008  beq 0x8241cfac
	if ctx.cr[0].eq {
	pc = 0x8241CFAC; continue 'dispatch;
	}
	// 8241CFA8: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8241CFAC: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 8241CFB0: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8241CFB4: 409900B4  ble cr6, 0x8241d068
	if !ctx.cr[6].gt {
	pc = 0x8241D068; continue 'dispatch;
	}
	// 8241CFB8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8241CFBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241CFC0: 409A00B4  bne cr6, 0x8241d074
	if !ctx.cr[6].eq {
	pc = 0x8241D074; continue 'dispatch;
	}
	// 8241CFC4: 7F1DB800  cmpw cr6, r29, r23
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[23].s32, &mut ctx.xer);
	// 8241CFC8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8241CFCC: 41980008  blt cr6, 0x8241cfd4
	if ctx.cr[6].lt {
	pc = 0x8241CFD4; continue 'dispatch;
	}
	// 8241CFD0: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 8241CFD4: 396B07FF  addi r11, r11, 0x7ff
	ctx.r[11].s64 = ctx.r[11].s64 + 2047;
	// 8241CFD8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241CFDC: 3B5F0004  addi r26, r31, 4
	ctx.r[26].s64 = ctx.r[31].s64 + 4;
	// 8241CFE0: 557E0028  rlwinm r30, r11, 0, 0, 0x14
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8241CFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241CFE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8241CFEC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8241CFF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241CFF4: 4BFA3565  bl 0x823c0558
	ctx.lr = 0x8241CFF8;
	sub_823C0558(ctx, base);
	// 8241CFF8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241CFFC: 40820018  bne 0x8241d014
	if !ctx.cr[0].eq {
	pc = 0x8241D014; continue 'dispatch;
	}
	// 8241D000: 4BFA3681  bl 0x823c0680
	ctx.lr = 0x8241D004;
	sub_823C0680(ctx, base);
	// 8241D004: 2B0303E4  cmplwi cr6, r3, 0x3e4
	ctx.cr[6].compare_u32(ctx.r[3].u32, 996 as u32, &mut ctx.xer);
	// 8241D008: 41980078  blt cr6, 0x8241d080
	if ctx.cr[6].lt {
	pc = 0x8241D080; continue 'dispatch;
	}
	// 8241D00C: 2B0303E5  cmplwi cr6, r3, 0x3e5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 997 as u32, &mut ctx.xer);
	// 8241D010: 41990070  bgt cr6, 0x8241d080
	if ctx.cr[6].gt {
	pc = 0x8241D080; continue 'dispatch;
	}
	// 8241D014: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8241D018: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D01C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241D020: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8241D024: 48129EDD  bl 0x82546f00
	ctx.lr = 0x8241D028;
	sub_82546F00(ctx, base);
	// 8241D028: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D02C: 41820050  beq 0x8241d07c
	if ctx.cr[0].eq {
	pc = 0x8241D07C; continue 'dispatch;
	}
	// 8241D030: 4BFADC71  bl 0x823caca0
	ctx.lr = 0x8241D034;
	sub_823CACA0(ctx, base);
	// 8241D034: E95F0018  ld r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 8241D038: 7BCB0020  clrldi r11, r30, 0x20
	ctx.r[11].u64 = ctx.r[30].u64 & 0x00000000FFFFFFFFu64;
	// 8241D03C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8241D040: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241D044: 7F9EE214  add r28, r30, r28
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 8241D048: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8241D04C: 7F1BC800  cmpw cr6, r27, r25
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8241D050: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 8241D054: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241D058: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241D05C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241D060: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8241D064: 4198FF54  blt cr6, 0x8241cfb8
	if ctx.cr[6].lt {
	pc = 0x8241CFB8; continue 'dispatch;
	}
	// 8241D068: 931F0028  stw r24, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[24].u32 ) };
	// 8241D06C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8241D070: 48118084  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8241D074: 931F0028  stw r24, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[24].u32 ) };
	// 8241D078: 4BFFFFF0  b 0x8241d068
	pc = 0x8241D068; continue 'dispatch;
	// 8241D07C: 4BFA3605  bl 0x823c0680
	ctx.lr = 0x8241D080;
	sub_823C0680(ctx, base);
	// 8241D080: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241D084: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8241D088: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8241D08C: 4BFFFFE0  b 0x8241d06c
	pc = 0x8241D06C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8241D090 size=172
    let mut pc: u32 = 0x8241D090;
    'dispatch: loop {
        match pc {
            0x8241D090 => {
    //   block [0x8241D090..0x8241D13C)
	// 8241D090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241D09C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D0A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241D0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241D0A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241D0AC: E89F0008  ld r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8241D0B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D0B4: 4800F0F5  bl 0x8242c1a8
	ctx.lr = 0x8241D0B8;
	sub_8242C1A8(ctx, base);
	// 8241D0B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D0BC: 40820014  bne 0x8241d0d0
	if !ctx.cr[0].eq {
	pc = 0x8241D0D0; continue 'dispatch;
	}
	// 8241D0C0: 4BFA35C1  bl 0x823c0680
	ctx.lr = 0x8241D0C4;
	sub_823C0680(ctx, base);
	// 8241D0C4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241D0C8: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8241D0CC: 48000058  b 0x8241d124
	pc = 0x8241D124; continue 'dispatch;
	// 8241D0D0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D0D4: 481291ED  bl 0x825462c0
	ctx.lr = 0x8241D0D8;
	sub_825462C0(ctx, base);
	// 8241D0D8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D0DC: 4182FFE4  beq 0x8241d0c0
	if ctx.cr[0].eq {
	pc = 0x8241D0C0; continue 'dispatch;
	}
	// 8241D0E0: 4BFADBC1  bl 0x823caca0
	ctx.lr = 0x8241D0E4;
	sub_823CACA0(ctx, base);
	// 8241D0E4: E97F0008  ld r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8241D0E8: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 8241D0EC: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 8241D0F0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241D0F4: 7D4A5E74  sradi r10, r10, 0xb
	ctx.xer.ca = (ctx.r[10].s64 < 0) && ((ctx.r[10].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[10].s64 >> 11;
	// 8241D0F8: 7D695E74  sradi r9, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[9].s64 = ctx.r[11].s64 >> 11;
	// 8241D0FC: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8241D100: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8241D104: 79295D24  sldi r9, r9, 0xb
	ctx.r[9].u64 = ctx.r[9].u64.wrapping_shl(11);
	ctx.r[9].u32 = ctx.r[9].u64 as u32;
	// 8241D108: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8241D10C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8241D110: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8241D114: 4099000C  ble cr6, 0x8241d120
	if !ctx.cr[6].gt {
	pc = 0x8241D120; continue 'dispatch;
	}
	// 8241D118: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8241D11C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8241D120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241D124: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8241D128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241D12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241D138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D140 size=88
    let mut pc: u32 = 0x8241D140;
    'dispatch: loop {
        match pc {
            0x8241D140 => {
    //   block [0x8241D140..0x8241D198)
	// 8241D140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241D14C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D150: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241D154: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241D158: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D15C: 4BFA4075  bl 0x823c11d0
	ctx.lr = 0x8241D160;
	sub_823C11D0(ctx, base);
	// 8241D160: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D164: 40820014  bne 0x8241d178
	if !ctx.cr[0].eq {
	pc = 0x8241D178; continue 'dispatch;
	}
	// 8241D168: 4BFA3519  bl 0x823c0680
	ctx.lr = 0x8241D16C;
	sub_823C0680(ctx, base);
	// 8241D16C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241D170: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8241D174: 4800000C  b 0x8241d180
	pc = 0x8241D180; continue 'dispatch;
	// 8241D178: 4BFADB29  bl 0x823caca0
	ctx.lr = 0x8241D17C;
	sub_823CACA0(ctx, base);
	// 8241D17C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241D180: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241D184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241D188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241D194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241D198 size=8
    let mut pc: u32 = 0x8241D198;
    'dispatch: loop {
        match pc {
            0x8241D198 => {
    //   block [0x8241D198..0x8241D1A0)
	// 8241D198: 38600800  li r3, 0x800
	ctx.r[3].s64 = 2048;
	// 8241D19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241D1A0 size=12
    let mut pc: u32 = 0x8241D1A0;
    'dispatch: loop {
        match pc {
            0x8241D1A0 => {
    //   block [0x8241D1A0..0x8241D1AC)
	// 8241D1A0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241D1A4: 906B87B4  stw r3, -0x784c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-30796 as u32), ctx.r[3].u32 ) };
	// 8241D1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D1B0 size=104
    let mut pc: u32 = 0x8241D1B0;
    'dispatch: loop {
        match pc {
            0x8241D1B0 => {
    //   block [0x8241D1B0..0x8241D218)
	// 8241D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D1B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241D1BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241D1C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D1C4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241D1C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241D1CC: 3BCB87BC  addi r30, r11, -0x7844
	ctx.r[30].s64 = ctx.r[11].s64 + -30788;
	// 8241D1D0: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8241D1D4: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241D1D8: 93EA3C1C  stw r31, 0x3c1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(15388 as u32), ctx.r[31].u32 ) };
	// 8241D1DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241D1E0: 419A000C  beq cr6, 0x8241d1ec
	if ctx.cr[6].eq {
	pc = 0x8241D1EC; continue 'dispatch;
	}
	// 8241D1E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241D1E8: 4800F9F1  bl 0x8242cbd8
	ctx.lr = 0x8241D1EC;
	sub_8242CBD8(ctx, base);
	// 8241D1EC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D1F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241D1F4: 419A000C  beq cr6, 0x8241d200
	if ctx.cr[6].eq {
	pc = 0x8241D200; continue 'dispatch;
	}
	// 8241D1F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241D1FC: 4800F9DD  bl 0x8242cbd8
	ctx.lr = 0x8241D200;
	sub_8242CBD8(ctx, base);
	// 8241D200: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241D204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D20C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241D210: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241D214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D218 size=104
    let mut pc: u32 = 0x8241D218;
    'dispatch: loop {
        match pc {
            0x8241D218 => {
    //   block [0x8241D218..0x8241D280)
	// 8241D218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241D224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241D228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D22C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241D230: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241D234: 3BEB87BC  addi r31, r11, -0x7844
	ctx.r[31].s64 = ctx.r[11].s64 + -30788;
	// 8241D238: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 8241D23C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241D240: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241D244: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8241D248: 419A000C  beq cr6, 0x8241d254
	if ctx.cr[6].eq {
	pc = 0x8241D254; continue 'dispatch;
	}
	// 8241D24C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241D250: 4800FA11  bl 0x8242cc60
	ctx.lr = 0x8241D254;
	sub_8242CC60(ctx, base);
	// 8241D254: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D258: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241D25C: 419A000C  beq cr6, 0x8241d268
	if ctx.cr[6].eq {
	pc = 0x8241D268; continue 'dispatch;
	}
	// 8241D260: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241D264: 4800F9FD  bl 0x8242cc60
	ctx.lr = 0x8241D268;
	sub_8242CC60(ctx, base);
	// 8241D268: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241D26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D274: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241D278: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241D27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241D280 size=4
    let mut pc: u32 = 0x8241D280;
    'dispatch: loop {
        match pc {
            0x8241D280 => {
    //   block [0x8241D280..0x8241D284)
	// 8241D280: 4BFA29B8  b 0x823bfc38
	sub_823BFC38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D288 size=224
    let mut pc: u32 = 0x8241D288;
    'dispatch: loop {
        match pc {
            0x8241D288 => {
    //   block [0x8241D288..0x8241D368)
	// 8241D288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D290: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241D294: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241D298: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D29C: 4800EF95  bl 0x8242c230
	ctx.lr = 0x8241D2A0;
	sub_8242C230(ctx, base);
	// 8241D2A0: 4800E919  bl 0x8242bbb8
	ctx.lr = 0x8241D2A4;
	sub_8242BBB8(ctx, base);
	// 8241D2A4: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241D2A8: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 8241D2AC: 386BF6A8  addi r3, r11, -0x958
	ctx.r[3].s64 = ctx.r[11].s64 + -2392;
	// 8241D2B0: 60A59100  ori r5, r5, 0x9100
	ctx.r[5].u64 = ctx.r[5].u64 | 37120;
	// 8241D2B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241D2B8: 48117F19  bl 0x825351d0
	ctx.lr = 0x8241D2BC;
	sub_825351D0(ctx, base);
	// 8241D2BC: 4800F36D  bl 0x8242c628
	ctx.lr = 0x8241D2C0;
	sub_8242C628(ctx, base);
	// 8241D2C0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241D2C4: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 8241D2C8: 3BCBF490  addi r30, r11, -0xb70
	ctx.r[30].s64 = ctx.r[11].s64 + -2928;
	// 8241D2CC: 387E01C0  addi r3, r30, 0x1c0
	ctx.r[3].s64 = ctx.r[30].s64 + 448;
	// 8241D2D0: 4800FA11  bl 0x8242cce0
	ctx.lr = 0x8241D2D4;
	sub_8242CCE0(ctx, base);
	// 8241D2D4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241D2D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241D2DC: 3BEB87C8  addi r31, r11, -0x7838
	ctx.r[31].s64 = ctx.r[11].s64 + -30776;
	// 8241D2E0: 907FFFF0  stw r3, -0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16 as u32), ctx.r[3].u32 ) };
	// 8241D2E4: 40820014  bne 0x8241d2f8
	if !ctx.cr[0].eq {
	pc = 0x8241D2F8; continue 'dispatch;
	}
	// 8241D2E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D2EC: 386B06E4  addi r3, r11, 0x6e4
	ctx.r[3].s64 = ctx.r[11].s64 + 1764;
	// 8241D2F0: 4BFFF919  bl 0x8241cc08
	ctx.lr = 0x8241D2F4;
	sub_8241CC08(ctx, base);
	// 8241D2F4: 4800005C  b 0x8241d350
	pc = 0x8241D350; continue 'dispatch;
	// 8241D2F8: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 8241D2FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241D300: 4800F9E1  bl 0x8242cce0
	ctx.lr = 0x8241D304;
	sub_8242CCE0(ctx, base);
	// 8241D304: 907FFFF4  stw r3, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[3].u32 ) };
	// 8241D308: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241D30C: 40820010  bne 0x8241d31c
	if !ctx.cr[0].eq {
	pc = 0x8241D31C; continue 'dispatch;
	}
	// 8241D310: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D314: 386B06BC  addi r3, r11, 0x6bc
	ctx.r[3].s64 = ctx.r[11].s64 + 1724;
	// 8241D318: 4BFFFFD8  b 0x8241d2f0
	pc = 0x8241D2F0; continue 'dispatch;
	// 8241D31C: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 8241D320: 807FFFF0  lwz r3, -0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8241D324: 809E3C1C  lwz r4, 0x3c1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(15388 as u32) ) } as u64;
	// 8241D328: 4800F8B1  bl 0x8242cbd8
	ctx.lr = 0x8241D32C;
	sub_8242CBD8(ctx, base);
	// 8241D32C: 809E3C1C  lwz r4, 0x3c1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(15388 as u32) ) } as u64;
	// 8241D330: 807FFFF4  lwz r3, -0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241D334: 4800F8A5  bl 0x8242cbd8
	ctx.lr = 0x8241D338;
	sub_8242CBD8(ctx, base);
	// 8241D338: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D33C: 807FFFF0  lwz r3, -0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8241D340: 4800F921  bl 0x8242cc60
	ctx.lr = 0x8241D344;
	sub_8242CC60(ctx, base);
	// 8241D344: 807FFFF4  lwz r3, -0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241D348: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D34C: 4800F915  bl 0x8242cc60
	ctx.lr = 0x8241D350;
	sub_8242CC60(ctx, base);
	// 8241D350: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241D354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D35C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241D360: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241D364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D368 size=124
    let mut pc: u32 = 0x8241D368;
    'dispatch: loop {
        match pc {
            0x8241D368 => {
    //   block [0x8241D368..0x8241D3E4)
	// 8241D368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D370: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241D378: 409A0018  bne cr6, 0x8241d390
	if !ctx.cr[6].eq {
	pc = 0x8241D390; continue 'dispatch;
	}
	// 8241D37C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D380: 386B08B0  addi r3, r11, 0x8b0
	ctx.r[3].s64 = ctx.r[11].s64 + 2224;
	// 8241D384: 4BFFF885  bl 0x8241cc08
	ctx.lr = 0x8241D388;
	sub_8241CC08(ctx, base);
	// 8241D388: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241D38C: 48000048  b 0x8241d3d4
	pc = 0x8241D3D4; continue 'dispatch;
	// 8241D390: 388101A0  addi r4, r1, 0x1a0
	ctx.r[4].s64 = ctx.r[1].s64 + 416;
	// 8241D394: 4800EF2D  bl 0x8242c2c0
	ctx.lr = 0x8241D398;
	sub_8242C2C0(ctx, base);
	// 8241D398: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8241D39C: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 8241D3A0: 4BFAE171  bl 0x823cb510
	ctx.lr = 0x8241D3A4;
	sub_823CB510(ctx, base);
	// 8241D3A4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241D3A8: 409A000C  bne cr6, 0x8241d3b4
	if !ctx.cr[6].eq {
	pc = 0x8241D3B4; continue 'dispatch;
	}
	// 8241D3AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241D3B0: 4800001C  b 0x8241d3cc
	pc = 0x8241D3CC; continue 'dispatch;
	// 8241D3B4: 4BFA35A5  bl 0x823c0958
	ctx.lr = 0x8241D3B8;
	sub_823C0958(ctx, base);
	// 8241D3B8: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8241D3BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8241D3C0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8241D3C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8241D3C8: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241D3CC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241D3D0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241D3D4: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 8241D3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D3E8 size=124
    let mut pc: u32 = 0x8241D3E8;
    'dispatch: loop {
        match pc {
            0x8241D3E8 => {
    //   block [0x8241D3E8..0x8241D464)
	// 8241D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D3F0: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D3F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241D3F8: 409A0018  bne cr6, 0x8241d410
	if !ctx.cr[6].eq {
	pc = 0x8241D410; continue 'dispatch;
	}
	// 8241D3FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D400: 386B08D8  addi r3, r11, 0x8d8
	ctx.r[3].s64 = ctx.r[11].s64 + 2264;
	// 8241D404: 4BFFF805  bl 0x8241cc08
	ctx.lr = 0x8241D408;
	sub_8241CC08(ctx, base);
	// 8241D408: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241D40C: 48000048  b 0x8241d454
	pc = 0x8241D454; continue 'dispatch;
	// 8241D410: 388101A0  addi r4, r1, 0x1a0
	ctx.r[4].s64 = ctx.r[1].s64 + 416;
	// 8241D414: 4800EEAD  bl 0x8242c2c0
	ctx.lr = 0x8241D418;
	sub_8242C2C0(ctx, base);
	// 8241D418: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8241D41C: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 8241D420: 4BFAE0F1  bl 0x823cb510
	ctx.lr = 0x8241D424;
	sub_823CB510(ctx, base);
	// 8241D424: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241D428: 409A000C  bne cr6, 0x8241d434
	if !ctx.cr[6].eq {
	pc = 0x8241D434; continue 'dispatch;
	}
	// 8241D42C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241D430: 4800001C  b 0x8241d44c
	pc = 0x8241D44C; continue 'dispatch;
	// 8241D434: 4BFA3525  bl 0x823c0958
	ctx.lr = 0x8241D438;
	sub_823C0958(ctx, base);
	// 8241D438: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8241D43C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8241D440: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8241D444: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8241D448: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241D44C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241D450: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241D454: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 8241D458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D468 size=124
    let mut pc: u32 = 0x8241D468;
    'dispatch: loop {
        match pc {
            0x8241D468 => {
    //   block [0x8241D468..0x8241D4E4)
	// 8241D468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D470: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D474: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241D478: 409A0018  bne cr6, 0x8241d490
	if !ctx.cr[6].eq {
	pc = 0x8241D490; continue 'dispatch;
	}
	// 8241D47C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D480: 386B0900  addi r3, r11, 0x900
	ctx.r[3].s64 = ctx.r[11].s64 + 2304;
	// 8241D484: 4BFFF785  bl 0x8241cc08
	ctx.lr = 0x8241D488;
	sub_8241CC08(ctx, base);
	// 8241D488: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241D48C: 48000048  b 0x8241d4d4
	pc = 0x8241D4D4; continue 'dispatch;
	// 8241D490: 388101A0  addi r4, r1, 0x1a0
	ctx.r[4].s64 = ctx.r[1].s64 + 416;
	// 8241D494: 4800EE2D  bl 0x8242c2c0
	ctx.lr = 0x8241D498;
	sub_8242C2C0(ctx, base);
	// 8241D498: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8241D49C: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 8241D4A0: 4BFAE071  bl 0x823cb510
	ctx.lr = 0x8241D4A4;
	sub_823CB510(ctx, base);
	// 8241D4A4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241D4A8: 409A000C  bne cr6, 0x8241d4b4
	if !ctx.cr[6].eq {
	pc = 0x8241D4B4; continue 'dispatch;
	}
	// 8241D4AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8241D4B0: 4800001C  b 0x8241d4cc
	pc = 0x8241D4CC; continue 'dispatch;
	// 8241D4B4: 4BFA34A5  bl 0x823c0958
	ctx.lr = 0x8241D4B8;
	sub_823C0958(ctx, base);
	// 8241D4B8: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8241D4BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8241D4C0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8241D4C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8241D4C8: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8241D4CC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241D4D0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241D4D4: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 8241D4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D4E8 size=180
    let mut pc: u32 = 0x8241D4E8;
    'dispatch: loop {
        match pc {
            0x8241D4E8 => {
    //   block [0x8241D4E8..0x8241D59C)
	// 8241D4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D4F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241D4F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241D4F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D4FC: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241D500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241D504: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241D508: 4800FAC1  bl 0x8242cfc8
	ctx.lr = 0x8241D50C;
	sub_8242CFC8(ctx, base);
	// 8241D50C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D510: 40800010  bge 0x8241d520
	if !ctx.cr[0].lt {
	pc = 0x8241D520; continue 'dispatch;
	}
	// 8241D514: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D518: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241D51C: 4BFFF6ED  bl 0x8241cc08
	ctx.lr = 0x8241D520;
	sub_8241CC08(ctx, base);
	// 8241D520: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241D524: 409A0018  bne cr6, 0x8241d53c
	if !ctx.cr[6].eq {
	pc = 0x8241D53C; continue 'dispatch;
	}
	// 8241D528: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D52C: 386B0950  addi r3, r11, 0x950
	ctx.r[3].s64 = ctx.r[11].s64 + 2384;
	// 8241D530: 4BFFF6D9  bl 0x8241cc08
	ctx.lr = 0x8241D534;
	sub_8241CC08(ctx, base);
	// 8241D534: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241D538: 48000028  b 0x8241d560
	pc = 0x8241D560; continue 'dispatch;
	// 8241D53C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D540: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241D544: 409A0010  bne cr6, 0x8241d554
	if !ctx.cr[6].eq {
	pc = 0x8241D554; continue 'dispatch;
	}
	// 8241D548: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D54C: 386B0928  addi r3, r11, 0x928
	ctx.r[3].s64 = ctx.r[11].s64 + 2344;
	// 8241D550: 4BFFFFE0  b 0x8241d530
	pc = 0x8241D530; continue 'dispatch;
	// 8241D554: E97F0118  ld r11, 0x118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) };
	// 8241D558: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241D55C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241D560: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241D564: 7D7F07B4  extsw r31, r11
	ctx.r[31].s64 = ctx.r[11].s32 as i64;
	// 8241D568: 4800FAF9  bl 0x8242d060
	ctx.lr = 0x8241D56C;
	sub_8242D060(ctx, base);
	// 8241D56C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D570: 40800010  bge 0x8241d580
	if !ctx.cr[0].lt {
	pc = 0x8241D580; continue 'dispatch;
	}
	// 8241D574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D578: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241D57C: 4BFFF68D  bl 0x8241cc08
	ctx.lr = 0x8241D580;
	sub_8241CC08(ctx, base);
	// 8241D580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241D584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241D588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D590: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241D594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241D598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D5A0 size=524
    let mut pc: u32 = 0x8241D5A0;
    'dispatch: loop {
        match pc {
            0x8241D5A0 => {
    //   block [0x8241D5A0..0x8241D7AC)
	// 8241D5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D5A4: 48117B0D  bl 0x825350b0
	ctx.lr = 0x8241D5A8;
	sub_82535080(ctx, base);
	// 8241D5A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D5AC: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241D5B0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8241D5B4: 3BABF6A8  addi r29, r11, -0x958
	ctx.r[29].s64 = ctx.r[11].s64 + -2392;
	// 8241D5B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241D5BC: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8241D5C0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8241D5C4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D5C8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8241D5CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8241D5D0: 419A001C  beq cr6, 0x8241d5ec
	if ctx.cr[6].eq {
	pc = 0x8241D5EC; continue 'dispatch;
	}
	// 8241D5D4: 3D3D0001  addis r9, r29, 1
	ctx.r[9].s64 = ctx.r[29].s64 + 65536;
	// 8241D5D8: 396B01D0  addi r11, r11, 0x1d0
	ctx.r[11].s64 = ctx.r[11].s64 + 464;
	// 8241D5DC: 39299100  addi r9, r9, -0x6f00
	ctx.r[9].s64 = ctx.r[9].s64 + -28416;
	// 8241D5E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241D5E4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8241D5E8: 4198FFDC  blt cr6, 0x8241d5c4
	if ctx.cr[6].lt {
	pc = 0x8241D5C4; continue 'dispatch;
	}
	// 8241D5EC: 2F0A0050  cmpwi cr6, r10, 0x50
	ctx.cr[6].compare_i32(ctx.r[10].s32, 80, &mut ctx.xer);
	// 8241D5F0: 409A0018  bne cr6, 0x8241d608
	if !ctx.cr[6].eq {
	pc = 0x8241D608; continue 'dispatch;
	}
	// 8241D5F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D5F8: 386B0A18  addi r3, r11, 0xa18
	ctx.r[3].s64 = ctx.r[11].s64 + 2584;
	// 8241D5FC: 4BFFF60D  bl 0x8241cc08
	ctx.lr = 0x8241D600;
	sub_8241CC08(ctx, base);
	// 8241D600: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241D604: 480001A0  b 0x8241d7a4
	pc = 0x8241D7A4; continue 'dispatch;
	// 8241D608: 38A001D0  li r5, 0x1d0
	ctx.r[5].s64 = 464;
	// 8241D60C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241D610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241D614: 48117BBD  bl 0x825351d0
	ctx.lr = 0x8241D618;
	sub_825351D0(ctx, base);
	// 8241D618: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 8241D61C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241D620: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8241D624: 4800EC9D  bl 0x8242c2c0
	ctx.lr = 0x8241D628;
	sub_8242C2C0(ctx, base);
	// 8241D628: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8241D62C: 4800EE1D  bl 0x8242c448
	ctx.lr = 0x8241D630;
	sub_8242C448(ctx, base);
	// 8241D630: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D634: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241D638: 40820010  bne 0x8241d648
	if !ctx.cr[0].eq {
	pc = 0x8241D648; continue 'dispatch;
	}
	// 8241D63C: 396B87BC  addi r11, r11, -0x7844
	ctx.r[11].s64 = ctx.r[11].s64 + -30788;
	// 8241D640: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241D644: 48000008  b 0x8241d64c
	pc = 0x8241D64C; continue 'dispatch;
	// 8241D648: 816B87BC  lwz r11, -0x7844(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30788 as u32) ) } as u64;
	// 8241D64C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241D650: 917F01C8  stw r11, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[11].u32 ) };
	// 8241D654: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241D658: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241D65C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241D660: 4BFA3E19  bl 0x823c1478
	ctx.lr = 0x8241D664;
	sub_823C1478(ctx, base);
	// 8241D664: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241D668: 907F0174  stw r3, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[3].u32 ) };
	// 8241D66C: 40820010  bne 0x8241d67c
	if !ctx.cr[0].eq {
	pc = 0x8241D67C; continue 'dispatch;
	}
	// 8241D670: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D674: 386B0A80  addi r3, r11, 0xa80
	ctx.r[3].s64 = ctx.r[11].s64 + 2688;
	// 8241D678: 4BFFFF84  b 0x8241d5fc
	pc = 0x8241D5FC; continue 'dispatch;
	// 8241D67C: 939F0140  stw r28, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[28].u32 ) };
	// 8241D680: 3B5F0140  addi r26, r31, 0x140
	ctx.r[26].s64 = ctx.r[31].s64 + 320;
	// 8241D684: 937F0158  stw r27, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[27].u32 ) };
	// 8241D688: 3BDD000C  addi r30, r29, 0xc
	ctx.r[30].s64 = ctx.r[29].s64 + 12;
	// 8241D68C: 937F015C  stw r27, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[27].u32 ) };
	// 8241D690: 937F0114  stw r27, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[27].u32 ) };
	// 8241D694: FB7F0118  std r27, 0x118(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[27].u64 ) };
	// 8241D698: 937F0120  stw r27, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[27].u32 ) };
	// 8241D69C: 817EFFF4  lwz r11, -0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241D6A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241D6A4: 419A0030  beq cr6, 0x8241d6d4
	if ctx.cr[6].eq {
	pc = 0x8241D6D4; continue 'dispatch;
	}
	// 8241D6A8: 817E0108  lwz r11, 0x108(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) } as u64;
	// 8241D6AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241D6B0: 419A0024  beq cr6, 0x8241d6d4
	if ctx.cr[6].eq {
	pc = 0x8241D6D4; continue 'dispatch;
	}
	// 8241D6B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D6B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241D6BC: 409A0018  bne cr6, 0x8241d6d4
	if !ctx.cr[6].eq {
	pc = 0x8241D6D4; continue 'dispatch;
	}
	// 8241D6C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8241D6C4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8241D6C8: 4811A239  bl 0x82537900
	ctx.lr = 0x8241D6CC;
	sub_82537900(ctx, base);
	// 8241D6CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D6D0: 4182001C  beq 0x8241d6ec
	if ctx.cr[0].eq {
	pc = 0x8241D6EC; continue 'dispatch;
	}
	// 8241D6D4: 3D7D0001  addis r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 65536;
	// 8241D6D8: 3BDE01D0  addi r30, r30, 0x1d0
	ctx.r[30].s64 = ctx.r[30].s64 + 464;
	// 8241D6DC: 396B910C  addi r11, r11, -0x6ef4
	ctx.r[11].s64 = ctx.r[11].s64 + -28404;
	// 8241D6E0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241D6E4: 4198FFB8  blt cr6, 0x8241d69c
	if ctx.cr[6].lt {
	pc = 0x8241D69C; continue 'dispatch;
	}
	// 8241D6E8: 4800001C  b 0x8241d704
	pc = 0x8241D704; continue 'dispatch;
	// 8241D6EC: 817E0108  lwz r11, 0x108(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) } as u64;
	// 8241D6F0: 917F0114  stw r11, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 8241D6F4: E97E010C  ld r11, 0x10c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(268 as u32) ) };
	// 8241D6F8: F97F0118  std r11, 0x118(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[11].u64 ) };
	// 8241D6FC: 817E0114  lwz r11, 0x114(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241D700: 917F0120  stw r11, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 8241D704: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241D708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241D70C: 409A0074  bne cr6, 0x8241d780
	if !ctx.cr[6].eq {
	pc = 0x8241D780; continue 'dispatch;
	}
	// 8241D710: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8241D714: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8241D718: 4800E741  bl 0x8242be58
	ctx.lr = 0x8241D71C;
	sub_8242BE58(ctx, base);
	// 8241D71C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241D720: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241D724: 419A005C  beq cr6, 0x8241d780
	if ctx.cr[6].eq {
	pc = 0x8241D780; continue 'dispatch;
	}
	// 8241D728: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8241D72C: 4BFFF565  bl 0x8241cc90
	ctx.lr = 0x8241D730;
	sub_8241CC90(ctx, base);
	// 8241D730: 817F0158  lwz r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 8241D734: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241D738: 419A0030  beq cr6, 0x8241d768
	if ctx.cr[6].eq {
	pc = 0x8241D768; continue 'dispatch;
	}
	// 8241D73C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D740: 80BF015C  lwz r5, 0x15c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8241D744: 387DFE40  addi r3, r29, -0x1c0
	ctx.r[3].s64 = ctx.r[29].s64 + -448;
	// 8241D748: 388B0A44  addi r4, r11, 0xa44
	ctx.r[4].s64 = ctx.r[11].s64 + 2628;
	// 8241D74C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8241D750: 48129859  bl 0x82546fa8
	ctx.lr = 0x8241D754;
	sub_82546FA8(ctx, base);
	// 8241D754: 387DFE40  addi r3, r29, -0x1c0
	ctx.r[3].s64 = ctx.r[29].s64 + -448;
	// 8241D758: 4BFFF4B1  bl 0x8241cc08
	ctx.lr = 0x8241D75C;
	sub_8241CC08(ctx, base);
	// 8241D75C: 807F0174  lwz r3, 0x174(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 8241D760: 4BFA31F9  bl 0x823c0958
	ctx.lr = 0x8241D764;
	sub_823C0958(ctx, base);
	// 8241D764: 4BFFFE9C  b 0x8241d600
	pc = 0x8241D600; continue 'dispatch;
	// 8241D768: 817F0144  lwz r11, 0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 8241D76C: E95F0148  ld r10, 0x148(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) };
	// 8241D770: 813F0150  lwz r9, 0x150(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 8241D774: 917F0114  stw r11, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 8241D778: F95F0118  std r10, 0x118(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[10].u64 ) };
	// 8241D77C: 913F0120  stw r9, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[9].u32 ) };
	// 8241D780: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241D784: 815F0114  lwz r10, 0x114(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241D788: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8241D78C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241D790: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 8241D794: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241D798: 409A0008  bne cr6, 0x8241d7a0
	if !ctx.cr[6].eq {
	pc = 0x8241D7A0; continue 'dispatch;
	}
	// 8241D79C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8241D7A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241D7A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241D7A8: 48117958  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D7B0 size=268
    let mut pc: u32 = 0x8241D7B0;
    'dispatch: loop {
        match pc {
            0x8241D7B0 => {
    //   block [0x8241D7B0..0x8241D8BC)
	// 8241D7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D7B4: 48117905  bl 0x825350b8
	ctx.lr = 0x8241D7B8;
	sub_82535080(ctx, base);
	// 8241D7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D7BC: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241D7C0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8241D7C4: 394BF6A8  addi r10, r11, -0x958
	ctx.r[10].s64 = ctx.r[11].s64 + -2392;
	// 8241D7C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8241D7CC: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8241D7D0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241D7D4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D7D8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8241D7DC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8241D7E0: 419A001C  beq cr6, 0x8241d7fc
	if ctx.cr[6].eq {
	pc = 0x8241D7FC; continue 'dispatch;
	}
	// 8241D7E4: 3D0A0001  addis r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 65536;
	// 8241D7E8: 396B01D0  addi r11, r11, 0x1d0
	ctx.r[11].s64 = ctx.r[11].s64 + 464;
	// 8241D7EC: 39089100  addi r8, r8, -0x6f00
	ctx.r[8].s64 = ctx.r[8].s64 + -28416;
	// 8241D7F0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8241D7F4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8241D7F8: 4198FFDC  blt cr6, 0x8241d7d4
	if ctx.cr[6].lt {
	pc = 0x8241D7D4; continue 'dispatch;
	}
	// 8241D7FC: 2F090050  cmpwi cr6, r9, 0x50
	ctx.cr[6].compare_i32(ctx.r[9].s32, 80, &mut ctx.xer);
	// 8241D800: 409A0018  bne cr6, 0x8241d818
	if !ctx.cr[6].eq {
	pc = 0x8241D818; continue 'dispatch;
	}
	// 8241D804: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D808: 386B0A18  addi r3, r11, 0xa18
	ctx.r[3].s64 = ctx.r[11].s64 + 2584;
	// 8241D80C: 4BFFF3FD  bl 0x8241cc08
	ctx.lr = 0x8241D810;
	sub_8241CC08(ctx, base);
	// 8241D810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241D814: 480000A0  b 0x8241d8b4
	pc = 0x8241D8B4; continue 'dispatch;
	// 8241D818: 38A001D0  li r5, 0x1d0
	ctx.r[5].s64 = 464;
	// 8241D81C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241D820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241D824: 481179AD  bl 0x825351d0
	ctx.lr = 0x8241D828;
	sub_825351D0(ctx, base);
	// 8241D828: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 8241D82C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241D830: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241D834: 4800EA8D  bl 0x8242c2c0
	ctx.lr = 0x8241D838;
	sub_8242C2C0(ctx, base);
	// 8241D838: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241D83C: 4800EC0D  bl 0x8242c448
	ctx.lr = 0x8241D840;
	sub_8242C448(ctx, base);
	// 8241D840: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D844: 40820010  bne 0x8241d854
	if !ctx.cr[0].eq {
	pc = 0x8241D854; continue 'dispatch;
	}
	// 8241D848: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D84C: 386B0AA4  addi r3, r11, 0xaa4
	ctx.r[3].s64 = ctx.r[11].s64 + 2724;
	// 8241D850: 4BFFFFBC  b 0x8241d80c
	pc = 0x8241D80C; continue 'dispatch;
	// 8241D854: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241D858: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241D85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241D860: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241D864: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241D868: 816B87BC  lwz r11, -0x7844(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30788 as u32) ) } as u64;
	// 8241D86C: 917F01C8  stw r11, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[11].u32 ) };
	// 8241D870: 4BFA3C09  bl 0x823c1478
	ctx.lr = 0x8241D874;
	sub_823C1478(ctx, base);
	// 8241D874: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241D878: 907F0174  stw r3, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[3].u32 ) };
	// 8241D87C: 40820010  bne 0x8241d88c
	if !ctx.cr[0].eq {
	pc = 0x8241D88C; continue 'dispatch;
	}
	// 8241D880: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D884: 386B0A80  addi r3, r11, 0xa80
	ctx.r[3].s64 = ctx.r[11].s64 + 2688;
	// 8241D888: 4BFFFF84  b 0x8241d80c
	pc = 0x8241D80C; continue 'dispatch;
	// 8241D88C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241D890: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 8241D894: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8241D898: 939F0158  stw r28, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[28].u32 ) };
	// 8241D89C: 939F015C  stw r28, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[28].u32 ) };
	// 8241D8A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241D8A4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8241D8A8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241D8AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241D8B0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8241D8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241D8B8: 48117850  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D8C0 size=276
    let mut pc: u32 = 0x8241D8C0;
    'dispatch: loop {
        match pc {
            0x8241D8C0 => {
    //   block [0x8241D8C0..0x8241D9D4)
	// 8241D8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241D8C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241D8CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241D8D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D8D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241D8D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241D8DC: 409A0014  bne cr6, 0x8241d8f0
	if !ctx.cr[6].eq {
	pc = 0x8241D8F0; continue 'dispatch;
	}
	// 8241D8E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D8E4: 386B0AF4  addi r3, r11, 0xaf4
	ctx.r[3].s64 = ctx.r[11].s64 + 2804;
	// 8241D8E8: 4BFFF321  bl 0x8241cc08
	ctx.lr = 0x8241D8EC;
	sub_8241CC08(ctx, base);
	// 8241D8EC: 480000D0  b 0x8241d9bc
	pc = 0x8241D9BC; continue 'dispatch;
	// 8241D8F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D8F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241D8F8: 409A0010  bne cr6, 0x8241d908
	if !ctx.cr[6].eq {
	pc = 0x8241D908; continue 'dispatch;
	}
	// 8241D8FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241D900: 386B0ACC  addi r3, r11, 0xacc
	ctx.r[3].s64 = ctx.r[11].s64 + 2764;
	// 8241D904: 4BFFFFE4  b 0x8241d8e8
	pc = 0x8241D8E8; continue 'dispatch;
	// 8241D908: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241D90C: 809F0138  lwz r4, 0x138(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 8241D910: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241D914: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241D918: 917F0190  stw r11, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 8241D91C: 40810014  ble 0x8241d930
	if !ctx.cr[0].gt {
	pc = 0x8241D930; continue 'dispatch;
	}
	// 8241D920: 807F01C8  lwz r3, 0x1c8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 8241D924: 4800F405  bl 0x8242cd28
	ctx.lr = 0x8241D928;
	sub_8242CD28(ctx, base);
	// 8241D928: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8241D92C: 93DF0138  stw r30, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[30].u32 ) };
	// 8241D930: 817F0144  lwz r11, 0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 8241D934: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241D938: 41820008  beq 0x8241d940
	if ctx.cr[0].eq {
	pc = 0x8241D940; continue 'dispatch;
	}
	// 8241D93C: 917F0114  stw r11, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 8241D940: 807F0174  lwz r3, 0x174(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 8241D944: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241D948: 4182000C  beq 0x8241d954
	if ctx.cr[0].eq {
	pc = 0x8241D954; continue 'dispatch;
	}
	// 8241D94C: 4BFA300D  bl 0x823c0958
	ctx.lr = 0x8241D950;
	sub_823C0958(ctx, base);
	// 8241D950: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 8241D954: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241D958: 392BF6A8  addi r9, r11, -0x958
	ctx.r[9].s64 = ctx.r[11].s64 + -2392;
	// 8241D95C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8241D960: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241D964: 419A0020  beq cr6, 0x8241d984
	if ctx.cr[6].eq {
	pc = 0x8241D984; continue 'dispatch;
	}
	// 8241D968: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D96C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8241D970: 419A0014  beq cr6, 0x8241d984
	if ctx.cr[6].eq {
	pc = 0x8241D984; continue 'dispatch;
	}
	// 8241D974: 814B0114  lwz r10, 0x114(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241D978: 811F0114  lwz r8, 0x114(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241D97C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8241D980: 419A002C  beq cr6, 0x8241d9ac
	if ctx.cr[6].eq {
	pc = 0x8241D9AC; continue 'dispatch;
	}
	// 8241D984: 3D490001  addis r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 65536;
	// 8241D988: 396B01D0  addi r11, r11, 0x1d0
	ctx.r[11].s64 = ctx.r[11].s64 + 464;
	// 8241D98C: 394A9100  addi r10, r10, -0x6f00
	ctx.r[10].s64 = ctx.r[10].s64 + -28416;
	// 8241D990: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8241D994: 4198FFCC  blt cr6, 0x8241d960
	if ctx.cr[6].lt {
	pc = 0x8241D960; continue 'dispatch;
	}
	// 8241D998: 807F0114  lwz r3, 0x114(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241D99C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241D9A0: 4182000C  beq 0x8241d9ac
	if ctx.cr[0].eq {
	pc = 0x8241D9AC; continue 'dispatch;
	}
	// 8241D9A4: 4BFA2FB5  bl 0x823c0958
	ctx.lr = 0x8241D9A8;
	sub_823C0958(ctx, base);
	// 8241D9A8: 93DF0114  stw r30, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[30].u32 ) };
	// 8241D9AC: 38A001D0  li r5, 0x1d0
	ctx.r[5].s64 = 464;
	// 8241D9B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241D9B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241D9B8: 48117819  bl 0x825351d0
	ctx.lr = 0x8241D9BC;
	sub_825351D0(ctx, base);
	// 8241D9BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241D9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241D9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241D9C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241D9CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241D9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241D9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241D9D8 size=344
    let mut pc: u32 = 0x8241D9D8;
    'dispatch: loop {
        match pc {
            0x8241D9D8 => {
    //   block [0x8241D9D8..0x8241DB30)
	// 8241D9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241D9DC: 481176E1  bl 0x825350bc
	ctx.lr = 0x8241D9E0;
	sub_82535080(ctx, base);
	// 8241D9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241D9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241D9E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241D9EC: 419A012C  beq cr6, 0x8241db18
	if ctx.cr[6].eq {
	pc = 0x8241DB18; continue 'dispatch;
	}
	// 8241D9F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8241D9F4: 419A0124  beq cr6, 0x8241db18
	if ctx.cr[6].eq {
	pc = 0x8241DB18; continue 'dispatch;
	}
	// 8241D9F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241D9FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241DA00: 409A0010  bne cr6, 0x8241da10
	if !ctx.cr[6].eq {
	pc = 0x8241DA10; continue 'dispatch;
	}
	// 8241DA04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DA08: 386B0C44  addi r3, r11, 0xc44
	ctx.r[3].s64 = ctx.r[11].s64 + 3140;
	// 8241DA0C: 48000114  b 0x8241db20
	pc = 0x8241DB20; continue 'dispatch;
	// 8241DA10: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8241DA14: 40980010  bge cr6, 0x8241da24
	if !ctx.cr[6].lt {
	pc = 0x8241DA24; continue 'dispatch;
	}
	// 8241DA18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DA1C: 386B0C20  addi r3, r11, 0xc20
	ctx.r[3].s64 = ctx.r[11].s64 + 3104;
	// 8241DA20: 48000100  b 0x8241db20
	pc = 0x8241DB20; continue 'dispatch;
	// 8241DA24: 813F0114  lwz r9, 0x114(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241DA28: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241DA2C: 418200F8  beq 0x8241db24
	if ctx.cr[0].eq {
	pc = 0x8241DB24; continue 'dispatch;
	}
	// 8241DA30: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241DA34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241DA38: 419A0010  beq cr6, 0x8241da48
	if ctx.cr[6].eq {
	pc = 0x8241DA48; continue 'dispatch;
	}
	// 8241DA3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DA40: 386B0BE4  addi r3, r11, 0xbe4
	ctx.r[3].s64 = ctx.r[11].s64 + 3044;
	// 8241DA44: 480000DC  b 0x8241db20
	pc = 0x8241DB20; continue 'dispatch;
	// 8241DA48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241DA4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241DA50: 409A00D4  bne cr6, 0x8241db24
	if !ctx.cr[6].eq {
	pc = 0x8241DB24; continue 'dispatch;
	}
	// 8241DA54: 815F0124  lwz r10, 0x124(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 8241DA58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241DA5C: 817F0120  lwz r11, 0x120(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 8241DA60: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8241DA64: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241DA68: 41980008  blt cr6, 0x8241da70
	if ctx.cr[6].lt {
	pc = 0x8241DA70; continue 'dispatch;
	}
	// 8241DA6C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8241DA70: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8241DA74: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241DA78: 409A0018  bne cr6, 0x8241da90
	if !ctx.cr[6].eq {
	pc = 0x8241DA90; continue 'dispatch;
	}
	// 8241DA7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241DA80: 93DF0128  stw r30, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[30].u32 ) };
	// 8241DA84: FBDF0130  std r30, 0x130(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[30].u64 ) };
	// 8241DA88: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8241DA8C: 48000098  b 0x8241db24
	pc = 0x8241DB24; continue 'dispatch;
	// 8241DA90: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8241DA94: 913F0160  stw r9, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[9].u32 ) };
	// 8241DA98: 57A85828  slwi r8, r29, 0xb
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(11);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8241DA9C: 90BF0180  stw r5, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[5].u32 ) };
	// 8241DAA0: 794A5D24  sldi r10, r10, 0xb
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(11);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8241DAA4: 807F01C8  lwz r3, 0x1c8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 8241DAA8: 397F0160  addi r11, r31, 0x160
	ctx.r[11].s64 = ctx.r[31].s64 + 352;
	// 8241DAAC: 93DF0190  stw r30, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[30].u32 ) };
	// 8241DAB0: 93DF0188  stw r30, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[30].u32 ) };
	// 8241DAB4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8241DAB8: 93DF018C  stw r30, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[30].u32 ) };
	// 8241DABC: 911F0184  stw r8, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[8].u32 ) };
	// 8241DAC0: F95F0178  std r10, 0x178(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[10].u64 ) };
	// 8241DAC4: 3D408242  lis r10, -0x7dbe
	ctx.r[10].s64 = -2109603840;
	// 8241DAC8: 813F017C  lwz r9, 0x17c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(380 as u32) ) } as u64;
	// 8241DACC: 80FF0178  lwz r7, 0x178(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(376 as u32) ) } as u64;
	// 8241DAD0: 388ACE10  addi r4, r10, -0x31f0
	ctx.r[4].s64 = ctx.r[10].s64 + -12784;
	// 8241DAD4: 913F016C  stw r9, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[9].u32 ) };
	// 8241DAD8: 90FF0170  stw r7, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[7].u32 ) };
	// 8241DADC: 4800EEB5  bl 0x8242c990
	ctx.lr = 0x8241DAE0;
	sub_8242C990(ctx, base);
	// 8241DAE0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DAE4: 907F0138  stw r3, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[3].u32 ) };
	// 8241DAE8: 4180003C  blt 0x8241db24
	if ctx.cr[0].lt {
	pc = 0x8241DB24; continue 'dispatch;
	}
	// 8241DAEC: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 8241DAF0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8241DAF4: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8241DAF8: 93BF0128  stw r29, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[29].u32 ) };
	// 8241DAFC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8241DB00: FBDF0130  std r30, 0x130(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[30].u64 ) };
	// 8241DB04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241DB08: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8241DB0C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8241DB10: 917F0124  stw r11, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[11].u32 ) };
	// 8241DB14: 48000014  b 0x8241db28
	pc = 0x8241DB28; continue 'dispatch;
	// 8241DB18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DB1C: 386B0BBC  addi r3, r11, 0xbbc
	ctx.r[3].s64 = ctx.r[11].s64 + 3004;
	// 8241DB20: 4BFFF0E9  bl 0x8241cc08
	ctx.lr = 0x8241DB24;
	sub_8241CC08(ctx, base);
	// 8241DB24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241DB28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241DB2C: 481175E0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241DB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241DB30 size=308
    let mut pc: u32 = 0x8241DB30;
    'dispatch: loop {
        match pc {
            0x8241DB30 => {
    //   block [0x8241DB30..0x8241DC64)
	// 8241DB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241DB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241DB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241DB3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8241DB40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241DB44: 419A0100  beq cr6, 0x8241dc44
	if ctx.cr[6].eq {
	pc = 0x8241DC44; continue 'dispatch;
	}
	// 8241DB48: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8241DB4C: 419A00F8  beq cr6, 0x8241dc44
	if ctx.cr[6].eq {
	pc = 0x8241DC44; continue 'dispatch;
	}
	// 8241DB50: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241DB54: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8241DB58: 409A0010  bne cr6, 0x8241db68
	if !ctx.cr[6].eq {
	pc = 0x8241DB68; continue 'dispatch;
	}
	// 8241DB5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DB60: 386B0CF4  addi r3, r11, 0xcf4
	ctx.r[3].s64 = ctx.r[11].s64 + 3316;
	// 8241DB64: 480000E8  b 0x8241dc4c
	pc = 0x8241DC4C; continue 'dispatch;
	// 8241DB68: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8241DB6C: 40980010  bge cr6, 0x8241db7c
	if !ctx.cr[6].lt {
	pc = 0x8241DB7C; continue 'dispatch;
	}
	// 8241DB70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DB74: 386B0CD0  addi r3, r11, 0xcd0
	ctx.r[3].s64 = ctx.r[11].s64 + 3280;
	// 8241DB78: 480000D4  b 0x8241dc4c
	pc = 0x8241DC4C; continue 'dispatch;
	// 8241DB7C: 810B0114  lwz r8, 0x114(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241DB80: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241DB84: 418200CC  beq 0x8241dc50
	if ctx.cr[0].eq {
	pc = 0x8241DC50; continue 'dispatch;
	}
	// 8241DB88: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241DB8C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8241DB90: 419A0010  beq cr6, 0x8241dba0
	if ctx.cr[6].eq {
	pc = 0x8241DBA0; continue 'dispatch;
	}
	// 8241DB94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DB98: 386B0C94  addi r3, r11, 0xc94
	ctx.r[3].s64 = ctx.r[11].s64 + 3220;
	// 8241DB9C: 480000B0  b 0x8241dc4c
	pc = 0x8241DC4C; continue 'dispatch;
	// 8241DBA0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241DBA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8241DBA8: 409A00A8  bne cr6, 0x8241dc50
	if !ctx.cr[6].eq {
	pc = 0x8241DC50; continue 'dispatch;
	}
	// 8241DBAC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8241DBB0: F94B0130  std r10, 0x130(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(304 as u32), ctx.r[10].u64 ) };
	// 8241DBB4: 409A0014  bne cr6, 0x8241dbc8
	if !ctx.cr[6].eq {
	pc = 0x8241DBC8; continue 'dispatch;
	}
	// 8241DBB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8241DBBC: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 8241DBC0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8241DBC4: 4800008C  b 0x8241dc50
	pc = 0x8241DC50; continue 'dispatch;
	// 8241DBC8: 812B0124  lwz r9, 0x124(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(292 as u32) ) } as u64;
	// 8241DBCC: 54875828  slwi r7, r4, 0xb
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shl(11);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8241DBD0: 914B0190  stw r10, 0x190(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(400 as u32), ctx.r[10].u32 ) };
	// 8241DBD4: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 8241DBD8: 914B018C  stw r10, 0x18c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(396 as u32), ctx.r[10].u32 ) };
	// 8241DBDC: 7D492214  add r10, r9, r4
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 8241DBE0: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 8241DBE4: 910B0160  stw r8, 0x160(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(352 as u32), ctx.r[8].u32 ) };
	// 8241DBE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8241DBEC: 90EB0184  stw r7, 0x184(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(388 as u32), ctx.r[7].u32 ) };
	// 8241DBF0: 79295D24  sldi r9, r9, 0xb
	ctx.r[9].u64 = ctx.r[9].u64.wrapping_shl(11);
	ctx.r[9].u32 = ctx.r[9].u64 as u32;
	// 8241DBF4: 80CB0120  lwz r6, 0x120(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 8241DBF8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8241DBFC: 90AB0180  stw r5, 0x180(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(384 as u32), ctx.r[5].u32 ) };
	// 8241DC00: 908B0128  stw r4, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[4].u32 ) };
	// 8241DC04: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 8241DC08: 914B0124  stw r10, 0x124(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(292 as u32), ctx.r[10].u32 ) };
	// 8241DC0C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8241DC10: F92B0178  std r9, 0x178(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(376 as u32), ctx.r[9].u64 ) };
	// 8241DC14: 812B017C  lwz r9, 0x17c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(380 as u32) ) } as u64;
	// 8241DC18: 810B0178  lwz r8, 0x178(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(376 as u32) ) } as u64;
	// 8241DC1C: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8241DC20: 912B016C  stw r9, 0x16c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(364 as u32), ctx.r[9].u32 ) };
	// 8241DC24: 910B0170  stw r8, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[8].u32 ) };
	// 8241DC28: 40990014  ble cr6, 0x8241dc3c
	if !ctx.cr[6].gt {
	pc = 0x8241DC3C; continue 'dispatch;
	}
	// 8241DC2C: 7D4907B4  extsw r9, r10
	ctx.r[9].s64 = ctx.r[10].s32 as i64;
	// 8241DC30: 914B0120  stw r10, 0x120(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(288 as u32), ctx.r[10].u32 ) };
	// 8241DC34: 79295D24  sldi r9, r9, 0xb
	ctx.r[9].u64 = ctx.r[9].u64.wrapping_shl(11);
	ctx.r[9].u32 = ctx.r[9].u64 as u32;
	// 8241DC38: F92B0118  std r9, 0x118(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(280 as u32), ctx.r[9].u64 ) };
	// 8241DC3C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8241DC40: 48000014  b 0x8241dc54
	pc = 0x8241DC54; continue 'dispatch;
	// 8241DC44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DC48: 386B0C6C  addi r3, r11, 0xc6c
	ctx.r[3].s64 = ctx.r[11].s64 + 3180;
	// 8241DC4C: 4BFFEFBD  bl 0x8241cc08
	ctx.lr = 0x8241DC50;
	sub_8241CC08(ctx, base);
	// 8241DC50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241DC54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241DC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241DC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241DC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241DC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241DC68 size=12
    let mut pc: u32 = 0x8241DC68;
    'dispatch: loop {
        match pc {
            0x8241DC68 => {
    //   block [0x8241DC68..0x8241DC74)
	// 8241DC68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DC6C: 386B0EE4  addi r3, r11, 0xee4
	ctx.r[3].s64 = ctx.r[11].s64 + 3812;
	// 8241DC70: 4BFFEF98  b 0x8241cc08
	sub_8241CC08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241DC78 size=116
    let mut pc: u32 = 0x8241DC78;
    'dispatch: loop {
        match pc {
            0x8241DC78 => {
    //   block [0x8241DC78..0x8241DCEC)
	// 8241DC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241DC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241DC80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241DC84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241DC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241DC8C: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8241DC90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241DC94: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241DC98: 4800F331  bl 0x8242cfc8
	ctx.lr = 0x8241DC9C;
	sub_8242CFC8(ctx, base);
	// 8241DC9C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DCA0: 40800010  bge 0x8241dcb0
	if !ctx.cr[0].lt {
	pc = 0x8241DCB0; continue 'dispatch;
	}
	// 8241DCA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DCA8: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241DCAC: 4BFFEF5D  bl 0x8241cc08
	ctx.lr = 0x8241DCB0;
	sub_8241CC08(ctx, base);
	// 8241DCB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241DCB4: 4800E7FD  bl 0x8242c4b0
	ctx.lr = 0x8241DCB8;
	sub_8242C4B0(ctx, base);
	// 8241DCB8: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241DCBC: 4800F3A5  bl 0x8242d060
	ctx.lr = 0x8241DCC0;
	sub_8242D060(ctx, base);
	// 8241DCC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DCC4: 40800010  bge 0x8241dcd4
	if !ctx.cr[0].lt {
	pc = 0x8241DCD4; continue 'dispatch;
	}
	// 8241DCC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DCCC: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241DCD0: 4BFFEF39  bl 0x8241cc08
	ctx.lr = 0x8241DCD4;
	sub_8241CC08(ctx, base);
	// 8241DCD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241DCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241DCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241DCE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241DCE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241DCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241DCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241DCF0 size=120
    let mut pc: u32 = 0x8241DCF0;
    'dispatch: loop {
        match pc {
            0x8241DCF0 => {
    //   block [0x8241DCF0..0x8241DD68)
	// 8241DCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241DCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241DCF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241DCFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241DD00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241DD04: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8241DD08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241DD0C: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241DD10: 4800F2B9  bl 0x8242cfc8
	ctx.lr = 0x8241DD14;
	sub_8242CFC8(ctx, base);
	// 8241DD14: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DD18: 40800010  bge 0x8241dd28
	if !ctx.cr[0].lt {
	pc = 0x8241DD28; continue 'dispatch;
	}
	// 8241DD1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DD20: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241DD24: 4BFFEEE5  bl 0x8241cc08
	ctx.lr = 0x8241DD28;
	sub_8241CC08(ctx, base);
	// 8241DD28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8241DD2C: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241DD30: 917E0190  stw r11, 0x190(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 8241DD34: 4800F32D  bl 0x8242d060
	ctx.lr = 0x8241DD38;
	sub_8242D060(ctx, base);
	// 8241DD38: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DD3C: 40800010  bge 0x8241dd4c
	if !ctx.cr[0].lt {
	pc = 0x8241DD4C; continue 'dispatch;
	}
	// 8241DD40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DD44: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241DD48: 4BFFEEC1  bl 0x8241cc08
	ctx.lr = 0x8241DD4C;
	sub_8241CC08(ctx, base);
	// 8241DD4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241DD50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241DD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241DD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241DD5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241DD60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241DD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241DD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241DD68 size=200
    let mut pc: u32 = 0x8241DD68;
    'dispatch: loop {
        match pc {
            0x8241DD68 => {
    //   block [0x8241DD68..0x8241DE30)
	// 8241DD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241DD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241DD70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241DD74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241DD78: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 8241DD7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DD80: 386AF62C  addi r3, r10, -0x9d4
	ctx.r[3].s64 = ctx.r[10].s64 + -2516;
	// 8241DD84: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8241DD88: 396B0548  addi r11, r11, 0x548
	ctx.r[11].s64 = ctx.r[11].s64 + 1352;
	// 8241DD8C: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 8241DD90: 3BEA87B0  addi r31, r10, -0x7850
	ctx.r[31].s64 = ctx.r[10].s64 + -30800;
	// 8241DD94: 395FFFFC  addi r10, r31, -4
	ctx.r[10].s64 = ctx.r[31].s64 + -4;
	// 8241DD98: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241DD9C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8241DDA0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241DDA4: 7D005028  lwarx r8, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8241DDA8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8241DDAC: 7D00512D  stwcx. r8, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241DDB0: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241DDB4: 4082FFE8  bne 0x8241dd9c
	if !ctx.cr[0].eq {
	pc = 0x8241DD9C; continue 'dispatch;
	}
	// 8241DDB8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8241DDBC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241DDC0: 409A005C  bne cr6, 0x8241de1c
	if !ctx.cr[6].eq {
	pc = 0x8241DE1C; continue 'dispatch;
	}
	// 8241DDC4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8241DDC8: 4800F091  bl 0x8242ce58
	ctx.lr = 0x8241DDCC;
	sub_8242CE58(ctx, base);
	// 8241DDCC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8241DDD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241DDD4: 40820010  bne 0x8241dde4
	if !ctx.cr[0].eq {
	pc = 0x8241DDE4; continue 'dispatch;
	}
	// 8241DDD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DDDC: 386B1088  addi r3, r11, 0x1088
	ctx.r[3].s64 = ctx.r[11].s64 + 4232;
	// 8241DDE0: 48000038  b 0x8241de18
	pc = 0x8241DE18; continue 'dispatch;
	// 8241DDE4: 4800F1E5  bl 0x8242cfc8
	ctx.lr = 0x8241DDE8;
	sub_8242CFC8(ctx, base);
	// 8241DDE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DDEC: 40800010  bge 0x8241ddfc
	if !ctx.cr[0].lt {
	pc = 0x8241DDFC; continue 'dispatch;
	}
	// 8241DDF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DDF4: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241DDF8: 4BFFEE11  bl 0x8241cc08
	ctx.lr = 0x8241DDFC;
	sub_8241CC08(ctx, base);
	// 8241DDFC: 4BFFF48D  bl 0x8241d288
	ctx.lr = 0x8241DE00;
	sub_8241D288(ctx, base);
	// 8241DE00: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241DE04: 4800F25D  bl 0x8242d060
	ctx.lr = 0x8241DE08;
	sub_8242D060(ctx, base);
	// 8241DE08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DE0C: 40800010  bge 0x8241de1c
	if !ctx.cr[0].lt {
	pc = 0x8241DE1C; continue 'dispatch;
	}
	// 8241DE10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DE14: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241DE18: 4BFFEDF1  bl 0x8241cc08
	ctx.lr = 0x8241DE1C;
	sub_8241CC08(ctx, base);
	// 8241DE1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241DE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241DE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241DE28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241DE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241DE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241DE30 size=988
    let mut pc: u32 = 0x8241DE30;
    'dispatch: loop {
        match pc {
            0x8241DE30 => {
    //   block [0x8241DE30..0x8241E20C)
	// 8241DE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241DE34: 48117275  bl 0x825350a8
	ctx.lr = 0x8241DE38;
	sub_82535080(ctx, base);
	// 8241DE38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241DE3C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241DE40: 3B8B87C4  addi r28, r11, -0x783c
	ctx.r[28].s64 = ctx.r[11].s64 + -30780;
	// 8241DE44: 807CFFF4  lwz r3, -0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241DE48: 4800EAB1  bl 0x8242c8f8
	ctx.lr = 0x8241DE4C;
	sub_8242C8F8(ctx, base);
	// 8241DE4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241DE50: 807CFFF8  lwz r3, -8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241DE54: 4800EAA5  bl 0x8242c8f8
	ctx.lr = 0x8241DE58;
	sub_8242C8F8(ctx, base);
	// 8241DE58: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 8241DE5C: 409A000C  bne cr6, 0x8241de68
	if !ctx.cr[6].eq {
	pc = 0x8241DE68; continue 'dispatch;
	}
	// 8241DE60: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241DE64: 419A03A0  beq cr6, 0x8241e204
	if ctx.cr[6].eq {
	pc = 0x8241E204; continue 'dispatch;
	}
	// 8241DE68: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241DE6C: 3B400003  li r26, 3
	ctx.r[26].s64 = 3;
	// 8241DE70: 3B6BF6A8  addi r27, r11, -0x958
	ctx.r[27].s64 = ctx.r[11].s64 + -2392;
	// 8241DE74: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241DE78: 3BFB0144  addi r31, r27, 0x144
	ctx.r[31].s64 = ctx.r[27].s64 + 324;
	// 8241DE7C: 3BABF4E8  addi r29, r11, -0xb18
	ctx.r[29].s64 = ctx.r[11].s64 + -2840;
	// 8241DE80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DE84: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8241DE88: 3BCB074C  addi r30, r11, 0x74c
	ctx.r[30].s64 = ctx.r[11].s64 + 1868;
	// 8241DE8C: 817FFEBC  lwz r11, -0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-324 as u32) ) } as u64;
	// 8241DE90: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241DE94: 409A0094  bne cr6, 0x8241df28
	if !ctx.cr[6].eq {
	pc = 0x8241DF28; continue 'dispatch;
	}
	// 8241DE98: 817FFEC4  lwz r11, -0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-316 as u32) ) } as u64;
	// 8241DE9C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241DEA0: 409A0088  bne cr6, 0x8241df28
	if !ctx.cr[6].eq {
	pc = 0x8241DF28; continue 'dispatch;
	}
	// 8241DEA4: 809FFFF4  lwz r4, -0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241DEA8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8241DEAC: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DEB0: 40810064  ble 0x8241df14
	if !ctx.cr[0].gt {
	pc = 0x8241DF14; continue 'dispatch;
	}
	// 8241DEB4: 4800EC95  bl 0x8242cb48
	ctx.lr = 0x8241DEB8;
	sub_8242CB48(ctx, base);
	// 8241DEB8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241DEBC: 409A006C  bne cr6, 0x8241df28
	if !ctx.cr[6].eq {
	pc = 0x8241DF28; continue 'dispatch;
	}
	// 8241DEC0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241DEC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241DEC8: 409A0020  bne cr6, 0x8241dee8
	if !ctx.cr[6].eq {
	pc = 0x8241DEE8; continue 'dispatch;
	}
	// 8241DECC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241DED0: E95F0004  ld r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8241DED4: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241DED8: 917FFFD0  stw r11, -0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-48 as u32), ctx.r[11].u32 ) };
	// 8241DEDC: F95FFFD4  std r10, -0x2c(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(-44 as u32), ctx.r[10].u64 ) };
	// 8241DEE0: 913FFFDC  stw r9, -0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-36 as u32), ctx.r[9].u32 ) };
	// 8241DEE4: 48000024  b 0x8241df08
	pc = 0x8241DF08; continue 'dispatch;
	// 8241DEE8: 38DFFECC  addi r6, r31, -0x134
	ctx.r[6].s64 = ctx.r[31].s64 + -308;
	// 8241DEEC: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241DEF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241DEF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241DEF8: 481290B1  bl 0x82546fa8
	ctx.lr = 0x8241DEFC;
	sub_82546FA8(ctx, base);
	// 8241DEFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241DF00: 4BFFED09  bl 0x8241cc08
	ctx.lr = 0x8241DF04;
	sub_8241CC08(ctx, base);
	// 8241DF04: 935FFEC0  stw r26, -0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-320 as u32), ctx.r[26].u32 ) };
	// 8241DF08: 931FFEC4  stw r24, -0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-316 as u32), ctx.r[24].u32 ) };
	// 8241DF0C: 931FFFF4  stw r24, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[24].u32 ) };
	// 8241DF10: 48000018  b 0x8241df28
	pc = 0x8241DF28; continue 'dispatch;
	// 8241DF14: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241DF18: 38BFFFFC  addi r5, r31, -4
	ctx.r[5].s64 = ctx.r[31].s64 + -4;
	// 8241DF1C: 388BCC90  addi r4, r11, -0x3370
	ctx.r[4].s64 = ctx.r[11].s64 + -13168;
	// 8241DF20: 4800EA71  bl 0x8242c990
	ctx.lr = 0x8241DF24;
	sub_8242C990(ctx, base);
	// 8241DF24: 907FFFF4  stw r3, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[3].u32 ) };
	// 8241DF28: 3D7B0001  addis r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 65536;
	// 8241DF2C: 3BFF01D0  addi r31, r31, 0x1d0
	ctx.r[31].s64 = ctx.r[31].s64 + 464;
	// 8241DF30: 396B9244  addi r11, r11, -0x6dbc
	ctx.r[11].s64 = ctx.r[11].s64 + -28092;
	// 8241DF34: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241DF38: 4198FF54  blt cr6, 0x8241de8c
	if ctx.cr[6].lt {
	pc = 0x8241DE8C; continue 'dispatch;
	}
	// 8241DF3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241DF40: 3BFB0128  addi r31, r27, 0x128
	ctx.r[31].s64 = ctx.r[27].s64 + 296;
	// 8241DF44: 3BCB0788  addi r30, r11, 0x788
	ctx.r[30].s64 = ctx.r[11].s64 + 1928;
	// 8241DF48: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8241DF4C: 817FFED8  lwz r11, -0x128(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-296 as u32) ) } as u64;
	// 8241DF50: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241DF54: 409A0094  bne cr6, 0x8241dfe8
	if !ctx.cr[6].eq {
	pc = 0x8241DFE8; continue 'dispatch;
	}
	// 8241DF58: 817FFEE0  lwz r11, -0x120(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-288 as u32) ) } as u64;
	// 8241DF5C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8241DF60: 409A0088  bne cr6, 0x8241dfe8
	if !ctx.cr[6].eq {
	pc = 0x8241DFE8; continue 'dispatch;
	}
	// 8241DF64: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241DF68: 807F00A0  lwz r3, 0xa0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8241DF6C: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241DF70: 40810064  ble 0x8241dfd4
	if !ctx.cr[0].gt {
	pc = 0x8241DFD4; continue 'dispatch;
	}
	// 8241DF74: 4800EBD5  bl 0x8242cb48
	ctx.lr = 0x8241DF78;
	sub_8242CB48(ctx, base);
	// 8241DF78: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241DF7C: 409A006C  bne cr6, 0x8241dfe8
	if !ctx.cr[6].eq {
	pc = 0x8241DFE8; continue 'dispatch;
	}
	// 8241DF80: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241DF84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241DF88: 409A001C  bne cr6, 0x8241dfa4
	if !ctx.cr[6].eq {
	pc = 0x8241DFA4; continue 'dispatch;
	}
	// 8241DF8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241DF90: 933FFEDC  stw r25, -0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-292 as u32), ctx.r[25].u32 ) };
	// 8241DF94: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241DF98: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8241DF9C: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8241DFA0: 48000028  b 0x8241dfc8
	pc = 0x8241DFC8; continue 'dispatch;
	// 8241DFA4: 38DFFEE8  addi r6, r31, -0x118
	ctx.r[6].s64 = ctx.r[31].s64 + -280;
	// 8241DFA8: 80BF0064  lwz r5, 0x64(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8241DFAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241DFB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241DFB4: 48128FF5  bl 0x82546fa8
	ctx.lr = 0x8241DFB8;
	sub_82546FA8(ctx, base);
	// 8241DFB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241DFBC: 4BFFEC4D  bl 0x8241cc08
	ctx.lr = 0x8241DFC0;
	sub_8241CC08(ctx, base);
	// 8241DFC0: 935FFEDC  stw r26, -0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-292 as u32), ctx.r[26].u32 ) };
	// 8241DFC4: FB1F0008  std r24, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 8241DFC8: 931FFEE0  stw r24, -0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-288 as u32), ctx.r[24].u32 ) };
	// 8241DFCC: 931F0010  stw r24, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[24].u32 ) };
	// 8241DFD0: 48000018  b 0x8241dfe8
	pc = 0x8241DFE8; continue 'dispatch;
	// 8241DFD4: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241DFD8: 38BF0038  addi r5, r31, 0x38
	ctx.r[5].s64 = ctx.r[31].s64 + 56;
	// 8241DFDC: 388BCF68  addi r4, r11, -0x3098
	ctx.r[4].s64 = ctx.r[11].s64 + -12440;
	// 8241DFE0: 4800E9B1  bl 0x8242c990
	ctx.lr = 0x8241DFE4;
	sub_8242C990(ctx, base);
	// 8241DFE4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8241DFE8: 3D7B0001  addis r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 65536;
	// 8241DFEC: 3BFF01D0  addi r31, r31, 0x1d0
	ctx.r[31].s64 = ctx.r[31].s64 + 464;
	// 8241DFF0: 396B9228  addi r11, r11, -0x6dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -28120;
	// 8241DFF4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241DFF8: 4198FF54  blt cr6, 0x8241df4c
	if ctx.cr[6].lt {
	pc = 0x8241DF4C; continue 'dispatch;
	}
	// 8241DFFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E000: 3BFB0144  addi r31, r27, 0x144
	ctx.r[31].s64 = ctx.r[27].s64 + 324;
	// 8241E004: 3BCB07C8  addi r30, r11, 0x7c8
	ctx.r[30].s64 = ctx.r[11].s64 + 1992;
	// 8241E008: 817FFEBC  lwz r11, -0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-324 as u32) ) } as u64;
	// 8241E00C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241E010: 409A0094  bne cr6, 0x8241e0a4
	if !ctx.cr[6].eq {
	pc = 0x8241E0A4; continue 'dispatch;
	}
	// 8241E014: 817FFEC4  lwz r11, -0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-316 as u32) ) } as u64;
	// 8241E018: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8241E01C: 409A0088  bne cr6, 0x8241e0a4
	if !ctx.cr[6].eq {
	pc = 0x8241E0A4; continue 'dispatch;
	}
	// 8241E020: 809FFFF4  lwz r4, -0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241E024: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8241E028: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E02C: 40810064  ble 0x8241e090
	if !ctx.cr[0].gt {
	pc = 0x8241E090; continue 'dispatch;
	}
	// 8241E030: 4800EB19  bl 0x8242cb48
	ctx.lr = 0x8241E034;
	sub_8242CB48(ctx, base);
	// 8241E034: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241E038: 409A006C  bne cr6, 0x8241e0a4
	if !ctx.cr[6].eq {
	pc = 0x8241E0A4; continue 'dispatch;
	}
	// 8241E03C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241E040: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E044: 409A0020  bne cr6, 0x8241e064
	if !ctx.cr[6].eq {
	pc = 0x8241E064; continue 'dispatch;
	}
	// 8241E048: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E04C: E95F0004  ld r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8241E050: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241E054: 917FFFD0  stw r11, -0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-48 as u32), ctx.r[11].u32 ) };
	// 8241E058: F95FFFD4  std r10, -0x2c(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(-44 as u32), ctx.r[10].u64 ) };
	// 8241E05C: 913FFFDC  stw r9, -0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-36 as u32), ctx.r[9].u32 ) };
	// 8241E060: 48000024  b 0x8241e084
	pc = 0x8241E084; continue 'dispatch;
	// 8241E064: 38DFFECC  addi r6, r31, -0x134
	ctx.r[6].s64 = ctx.r[31].s64 + -308;
	// 8241E068: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241E06C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241E070: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241E074: 48128F35  bl 0x82546fa8
	ctx.lr = 0x8241E078;
	sub_82546FA8(ctx, base);
	// 8241E078: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241E07C: 4BFFEB8D  bl 0x8241cc08
	ctx.lr = 0x8241E080;
	sub_8241CC08(ctx, base);
	// 8241E080: 935FFEC0  stw r26, -0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-320 as u32), ctx.r[26].u32 ) };
	// 8241E084: 931FFEC4  stw r24, -0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-316 as u32), ctx.r[24].u32 ) };
	// 8241E088: 931FFFF4  stw r24, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[24].u32 ) };
	// 8241E08C: 48000018  b 0x8241e0a4
	pc = 0x8241E0A4; continue 'dispatch;
	// 8241E090: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241E094: 38BFFFFC  addi r5, r31, -4
	ctx.r[5].s64 = ctx.r[31].s64 + -4;
	// 8241E098: 388BCD80  addi r4, r11, -0x3280
	ctx.r[4].s64 = ctx.r[11].s64 + -12928;
	// 8241E09C: 4800E8F5  bl 0x8242c990
	ctx.lr = 0x8241E0A0;
	sub_8242C990(ctx, base);
	// 8241E0A0: 907FFFF4  stw r3, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[3].u32 ) };
	// 8241E0A4: 3D7B0001  addis r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 65536;
	// 8241E0A8: 3BFF01D0  addi r31, r31, 0x1d0
	ctx.r[31].s64 = ctx.r[31].s64 + 464;
	// 8241E0AC: 396B9244  addi r11, r11, -0x6dbc
	ctx.r[11].s64 = ctx.r[11].s64 + -28092;
	// 8241E0B0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241E0B4: 4198FF54  blt cr6, 0x8241e008
	if ctx.cr[6].lt {
	pc = 0x8241E008; continue 'dispatch;
	}
	// 8241E0B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E0BC: 3BFB01A0  addi r31, r27, 0x1a0
	ctx.r[31].s64 = ctx.r[27].s64 + 416;
	// 8241E0C0: 3BCB0804  addi r30, r11, 0x804
	ctx.r[30].s64 = ctx.r[11].s64 + 2052;
	// 8241E0C4: 817FFE60  lwz r11, -0x1a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-416 as u32) ) } as u64;
	// 8241E0C8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241E0CC: 409A009C  bne cr6, 0x8241e168
	if !ctx.cr[6].eq {
	pc = 0x8241E168; continue 'dispatch;
	}
	// 8241E0D0: 817FFE68  lwz r11, -0x198(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-408 as u32) ) } as u64;
	// 8241E0D4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8241E0D8: 409A0090  bne cr6, 0x8241e168
	if !ctx.cr[6].eq {
	pc = 0x8241E168; continue 'dispatch;
	}
	// 8241E0DC: 809FFF98  lwz r4, -0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-104 as u32) ) } as u64;
	// 8241E0E0: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8241E0E4: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E0E8: 4081006C  ble 0x8241e154
	if !ctx.cr[0].gt {
	pc = 0x8241E154; continue 'dispatch;
	}
	// 8241E0EC: 4800EA5D  bl 0x8242cb48
	ctx.lr = 0x8241E0F0;
	sub_8242CB48(ctx, base);
	// 8241E0F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241E0F4: 409A0074  bne cr6, 0x8241e168
	if !ctx.cr[6].eq {
	pc = 0x8241E168; continue 'dispatch;
	}
	// 8241E0F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241E0FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E100: 409A0028  bne cr6, 0x8241e128
	if !ctx.cr[6].eq {
	pc = 0x8241E128; continue 'dispatch;
	}
	// 8241E104: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241E108: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8241E10C: 813FFF84  lwz r9, -0x7c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-124 as u32) ) } as u64;
	// 8241E110: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241E114: 917FFF80  stw r11, -0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-128 as u32), ctx.r[11].u32 ) };
	// 8241E118: F95FFF78  std r10, -0x88(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(-136 as u32), ctx.r[10].u64 ) };
	// 8241E11C: 4099002C  ble cr6, 0x8241e148
	if !ctx.cr[6].gt {
	pc = 0x8241E148; continue 'dispatch;
	}
	// 8241E120: 917FFF84  stw r11, -0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-124 as u32), ctx.r[11].u32 ) };
	// 8241E124: 48000024  b 0x8241e148
	pc = 0x8241E148; continue 'dispatch;
	// 8241E128: 38DFFE70  addi r6, r31, -0x190
	ctx.r[6].s64 = ctx.r[31].s64 + -400;
	// 8241E12C: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241E130: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8241E134: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241E138: 48128E71  bl 0x82546fa8
	ctx.lr = 0x8241E13C;
	sub_82546FA8(ctx, base);
	// 8241E13C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241E140: 4BFFEAC9  bl 0x8241cc08
	ctx.lr = 0x8241E144;
	sub_8241CC08(ctx, base);
	// 8241E144: 935FFE64  stw r26, -0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-412 as u32), ctx.r[26].u32 ) };
	// 8241E148: 931FFE68  stw r24, -0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-408 as u32), ctx.r[24].u32 ) };
	// 8241E14C: 931FFF98  stw r24, -0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-104 as u32), ctx.r[24].u32 ) };
	// 8241E150: 48000018  b 0x8241e168
	pc = 0x8241E168; continue 'dispatch;
	// 8241E154: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241E158: 38BFFFF8  addi r5, r31, -8
	ctx.r[5].s64 = ctx.r[31].s64 + -8;
	// 8241E15C: 388BD090  addi r4, r11, -0x2f70
	ctx.r[4].s64 = ctx.r[11].s64 + -12144;
	// 8241E160: 4800E831  bl 0x8242c990
	ctx.lr = 0x8241E164;
	sub_8242C990(ctx, base);
	// 8241E164: 907FFF98  stw r3, -0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-104 as u32), ctx.r[3].u32 ) };
	// 8241E168: 3D7B0001  addis r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 65536;
	// 8241E16C: 3BFF01D0  addi r31, r31, 0x1d0
	ctx.r[31].s64 = ctx.r[31].s64 + 464;
	// 8241E170: 396B92A0  addi r11, r11, -0x6d60
	ctx.r[11].s64 = ctx.r[11].s64 + -28000;
	// 8241E174: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241E178: 4198FF4C  blt cr6, 0x8241e0c4
	if ctx.cr[6].lt {
	pc = 0x8241E0C4; continue 'dispatch;
	}
	// 8241E17C: 80BCFFFC  lwz r5, -4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241E180: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8241E184: 419A0080  beq cr6, 0x8241e204
	if ctx.cr[6].eq {
	pc = 0x8241E204; continue 'dispatch;
	}
	// 8241E188: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E18C: 807CFFF8  lwz r3, -8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241E190: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8241E194: 40990060  ble cr6, 0x8241e1f4
	if !ctx.cr[6].gt {
	pc = 0x8241E1F4; continue 'dispatch;
	}
	// 8241E198: 4800E9B1  bl 0x8242cb48
	ctx.lr = 0x8241E19C;
	sub_8242CB48(ctx, base);
	// 8241E19C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241E1A0: 409A0064  bne cr6, 0x8241e204
	if !ctx.cr[6].eq {
	pc = 0x8241E204; continue 'dispatch;
	}
	// 8241E1A4: 817CFFFC  lwz r11, -4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241E1A8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241E1AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8241E1B0: 419A002C  beq cr6, 0x8241e1dc
	if ctx.cr[6].eq {
	pc = 0x8241E1DC; continue 'dispatch;
	}
	// 8241E1B4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241E1B8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241E1BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241E1C0: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E1C4: 388A0840  addi r4, r10, 0x840
	ctx.r[4].s64 = ctx.r[10].s64 + 2112;
	// 8241E1C8: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241E1CC: 48128DDD  bl 0x82546fa8
	ctx.lr = 0x8241E1D0;
	sub_82546FA8(ctx, base);
	// 8241E1D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241E1D4: 4BFFEA35  bl 0x8241cc08
	ctx.lr = 0x8241E1D8;
	sub_8241CC08(ctx, base);
	// 8241E1D8: 817CFFFC  lwz r11, -4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241E1DC: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8241E1E0: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 8241E1E4: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8241E1E8: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241E1EC: 917CFFFC  stw r11, -4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 8241E1F0: 48000014  b 0x8241e204
	pc = 0x8241E204; continue 'dispatch;
	// 8241E1F4: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241E1F8: 388BD140  addi r4, r11, -0x2ec0
	ctx.r[4].s64 = ctx.r[11].s64 + -11968;
	// 8241E1FC: 4800E795  bl 0x8242c990
	ctx.lr = 0x8241E200;
	sub_8242C990(ctx, base);
	// 8241E200: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8241E204: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8241E208: 48116EF0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E210 size=176
    let mut pc: u32 = 0x8241E210;
    'dispatch: loop {
        match pc {
            0x8241E210 => {
    //   block [0x8241E210..0x8241E2C0)
	// 8241E210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241E218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241E21C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241E220: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E224: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241E228: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E22C: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E230: 4800ED99  bl 0x8242cfc8
	ctx.lr = 0x8241E234;
	sub_8242CFC8(ctx, base);
	// 8241E234: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E238: 40800010  bge 0x8241e248
	if !ctx.cr[0].lt {
	pc = 0x8241E248; continue 'dispatch;
	}
	// 8241E23C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E240: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E244: 4BFFE9C5  bl 0x8241cc08
	ctx.lr = 0x8241E248;
	sub_8241CC08(ctx, base);
	// 8241E248: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E24C: 409A0018  bne cr6, 0x8241e264
	if !ctx.cr[6].eq {
	pc = 0x8241E264; continue 'dispatch;
	}
	// 8241E250: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E254: 386B0888  addi r3, r11, 0x888
	ctx.r[3].s64 = ctx.r[11].s64 + 2184;
	// 8241E258: 4BFFE9B1  bl 0x8241cc08
	ctx.lr = 0x8241E25C;
	sub_8241CC08(ctx, base);
	// 8241E25C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241E260: 48000028  b 0x8241e288
	pc = 0x8241E288; continue 'dispatch;
	// 8241E264: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8241E268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241E26C: 4800E055  bl 0x8242c2c0
	ctx.lr = 0x8241E270;
	sub_8242C2C0(ctx, base);
	// 8241E270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8241E274: 48128C25  bl 0x82546e98
	ctx.lr = 0x8241E278;
	sub_82546E98(ctx, base);
	// 8241E278: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8241E27C: 419AFFE0  beq cr6, 0x8241e25c
	if ctx.cr[6].eq {
	pc = 0x8241E25C; continue 'dispatch;
	}
	// 8241E280: 7C6B18F8  nor r11, r3, r3
	ctx.r[11].u64 = !(ctx.r[3].u64 | ctx.r[3].u64);
	// 8241E284: 557FE7FE  rlwinm r31, r11, 0x1c, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8241E288: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E28C: 4800EDD5  bl 0x8242d060
	ctx.lr = 0x8241E290;
	sub_8242D060(ctx, base);
	// 8241E290: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E294: 40800010  bge 0x8241e2a4
	if !ctx.cr[0].lt {
	pc = 0x8241E2A4; continue 'dispatch;
	}
	// 8241E298: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E29C: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E2A0: 4BFFE969  bl 0x8241cc08
	ctx.lr = 0x8241E2A4;
	sub_8241CC08(ctx, base);
	// 8241E2A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241E2A8: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 8241E2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241E2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241E2B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241E2B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241E2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E2C0 size=176
    let mut pc: u32 = 0x8241E2C0;
    'dispatch: loop {
        match pc {
            0x8241E2C0 => {
    //   block [0x8241E2C0..0x8241E370)
	// 8241E2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241E2C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241E2CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241E2D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E2D4: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241E2D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E2DC: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E2E0: 4800ECE9  bl 0x8242cfc8
	ctx.lr = 0x8241E2E4;
	sub_8242CFC8(ctx, base);
	// 8241E2E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E2E8: 40800010  bge 0x8241e2f8
	if !ctx.cr[0].lt {
	pc = 0x8241E2F8; continue 'dispatch;
	}
	// 8241E2EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E2F0: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E2F4: 4BFFE915  bl 0x8241cc08
	ctx.lr = 0x8241E2F8;
	sub_8241CC08(ctx, base);
	// 8241E2F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E2FC: 409A0018  bne cr6, 0x8241e314
	if !ctx.cr[6].eq {
	pc = 0x8241E314; continue 'dispatch;
	}
	// 8241E300: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E304: 386B0950  addi r3, r11, 0x950
	ctx.r[3].s64 = ctx.r[11].s64 + 2384;
	// 8241E308: 4BFFE901  bl 0x8241cc08
	ctx.lr = 0x8241E30C;
	sub_8241CC08(ctx, base);
	// 8241E30C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241E310: 48000028  b 0x8241e338
	pc = 0x8241E338; continue 'dispatch;
	// 8241E314: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E318: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E31C: 409A0010  bne cr6, 0x8241e32c
	if !ctx.cr[6].eq {
	pc = 0x8241E32C; continue 'dispatch;
	}
	// 8241E320: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E324: 386B0928  addi r3, r11, 0x928
	ctx.r[3].s64 = ctx.r[11].s64 + 2344;
	// 8241E328: 4BFFFFE0  b 0x8241e308
	pc = 0x8241E308; continue 'dispatch;
	// 8241E32C: E97F0118  ld r11, 0x118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) };
	// 8241E330: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241E334: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241E338: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E33C: 4800ED25  bl 0x8242d060
	ctx.lr = 0x8241E340;
	sub_8242D060(ctx, base);
	// 8241E340: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E344: 40800010  bge 0x8241e354
	if !ctx.cr[0].lt {
	pc = 0x8241E354; continue 'dispatch;
	}
	// 8241E348: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E34C: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E350: 4BFFE8B9  bl 0x8241cc08
	ctx.lr = 0x8241E354;
	sub_8241CC08(ctx, base);
	// 8241E354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241E358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241E35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241E360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241E364: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241E368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241E36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E370 size=176
    let mut pc: u32 = 0x8241E370;
    'dispatch: loop {
        match pc {
            0x8241E370 => {
    //   block [0x8241E370..0x8241E420)
	// 8241E370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241E378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241E37C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241E380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E384: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241E388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E38C: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E390: 4800EC39  bl 0x8242cfc8
	ctx.lr = 0x8241E394;
	sub_8242CFC8(ctx, base);
	// 8241E394: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E398: 40800010  bge 0x8241e3a8
	if !ctx.cr[0].lt {
	pc = 0x8241E3A8; continue 'dispatch;
	}
	// 8241E39C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E3A0: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E3A4: 4BFFE865  bl 0x8241cc08
	ctx.lr = 0x8241E3A8;
	sub_8241CC08(ctx, base);
	// 8241E3A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E3AC: 409A0018  bne cr6, 0x8241e3c4
	if !ctx.cr[6].eq {
	pc = 0x8241E3C4; continue 'dispatch;
	}
	// 8241E3B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E3B4: 386B09A0  addi r3, r11, 0x9a0
	ctx.r[3].s64 = ctx.r[11].s64 + 2464;
	// 8241E3B8: 4BFFE851  bl 0x8241cc08
	ctx.lr = 0x8241E3BC;
	sub_8241CC08(ctx, base);
	// 8241E3BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241E3C0: 48000028  b 0x8241e3e8
	pc = 0x8241E3E8; continue 'dispatch;
	// 8241E3C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E3C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E3CC: 409A0010  bne cr6, 0x8241e3dc
	if !ctx.cr[6].eq {
	pc = 0x8241E3DC; continue 'dispatch;
	}
	// 8241E3D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E3D4: 386B0978  addi r3, r11, 0x978
	ctx.r[3].s64 = ctx.r[11].s64 + 2424;
	// 8241E3D8: 4BFFFFE0  b 0x8241e3b8
	pc = 0x8241E3B8; continue 'dispatch;
	// 8241E3DC: E97F0118  ld r11, 0x118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) };
	// 8241E3E0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241E3E4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241E3E8: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E3EC: 4800EC75  bl 0x8242d060
	ctx.lr = 0x8241E3F0;
	sub_8242D060(ctx, base);
	// 8241E3F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E3F4: 40800010  bge 0x8241e404
	if !ctx.cr[0].lt {
	pc = 0x8241E404; continue 'dispatch;
	}
	// 8241E3F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E3FC: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E400: 4BFFE809  bl 0x8241cc08
	ctx.lr = 0x8241E404;
	sub_8241CC08(ctx, base);
	// 8241E404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241E408: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241E40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241E410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241E414: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241E418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241E41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E420 size=176
    let mut pc: u32 = 0x8241E420;
    'dispatch: loop {
        match pc {
            0x8241E420 => {
    //   block [0x8241E420..0x8241E4D0)
	// 8241E420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241E428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241E42C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241E430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E434: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241E438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E43C: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E440: 4800EB89  bl 0x8242cfc8
	ctx.lr = 0x8241E444;
	sub_8242CFC8(ctx, base);
	// 8241E444: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E448: 40800010  bge 0x8241e458
	if !ctx.cr[0].lt {
	pc = 0x8241E458; continue 'dispatch;
	}
	// 8241E44C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E450: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E454: 4BFFE7B5  bl 0x8241cc08
	ctx.lr = 0x8241E458;
	sub_8241CC08(ctx, base);
	// 8241E458: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E45C: 409A0018  bne cr6, 0x8241e474
	if !ctx.cr[6].eq {
	pc = 0x8241E474; continue 'dispatch;
	}
	// 8241E460: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E464: 386B09F0  addi r3, r11, 0x9f0
	ctx.r[3].s64 = ctx.r[11].s64 + 2544;
	// 8241E468: 4BFFE7A1  bl 0x8241cc08
	ctx.lr = 0x8241E46C;
	sub_8241CC08(ctx, base);
	// 8241E46C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241E470: 48000028  b 0x8241e498
	pc = 0x8241E498; continue 'dispatch;
	// 8241E474: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E478: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E47C: 409A0010  bne cr6, 0x8241e48c
	if !ctx.cr[6].eq {
	pc = 0x8241E48C; continue 'dispatch;
	}
	// 8241E480: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E484: 386B09C8  addi r3, r11, 0x9c8
	ctx.r[3].s64 = ctx.r[11].s64 + 2504;
	// 8241E488: 4BFFFFE0  b 0x8241e468
	pc = 0x8241E468; continue 'dispatch;
	// 8241E48C: E97F0118  ld r11, 0x118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) };
	// 8241E490: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241E494: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241E498: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E49C: 4800EBC5  bl 0x8242d060
	ctx.lr = 0x8241E4A0;
	sub_8242D060(ctx, base);
	// 8241E4A0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E4A4: 40800010  bge 0x8241e4b4
	if !ctx.cr[0].lt {
	pc = 0x8241E4B4; continue 'dispatch;
	}
	// 8241E4A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E4AC: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E4B0: 4BFFE759  bl 0x8241cc08
	ctx.lr = 0x8241E4B4;
	sub_8241CC08(ctx, base);
	// 8241E4B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241E4B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241E4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241E4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241E4C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241E4C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241E4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E4D0 size=116
    let mut pc: u32 = 0x8241E4D0;
    'dispatch: loop {
        match pc {
            0x8241E4D0 => {
    //   block [0x8241E4D0..0x8241E544)
	// 8241E4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241E4D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241E4DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241E4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E4E4: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8241E4E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241E4EC: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E4F0: 4800EAD9  bl 0x8242cfc8
	ctx.lr = 0x8241E4F4;
	sub_8242CFC8(ctx, base);
	// 8241E4F4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E4F8: 40800010  bge 0x8241e508
	if !ctx.cr[0].lt {
	pc = 0x8241E508; continue 'dispatch;
	}
	// 8241E4FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E500: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E504: 4BFFE705  bl 0x8241cc08
	ctx.lr = 0x8241E508;
	sub_8241CC08(ctx, base);
	// 8241E508: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E50C: 4BFFF3B5  bl 0x8241d8c0
	ctx.lr = 0x8241E510;
	sub_8241D8C0(ctx, base);
	// 8241E510: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E514: 4800EB4D  bl 0x8242d060
	ctx.lr = 0x8241E518;
	sub_8242D060(ctx, base);
	// 8241E518: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E51C: 40800010  bge 0x8241e52c
	if !ctx.cr[0].lt {
	pc = 0x8241E52C; continue 'dispatch;
	}
	// 8241E520: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E524: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E528: 4BFFE6E1  bl 0x8241cc08
	ctx.lr = 0x8241E52C;
	sub_8241CC08(ctx, base);
	// 8241E52C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241E530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241E534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241E538: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241E53C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241E540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E548 size=236
    let mut pc: u32 = 0x8241E548;
    'dispatch: loop {
        match pc {
            0x8241E548 => {
    //   block [0x8241E548..0x8241E634)
	// 8241E548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E54C: 48116B6D  bl 0x825350b8
	ctx.lr = 0x8241E550;
	sub_82535080(ctx, base);
	// 8241E550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E554: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8241E558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E55C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241E560: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241E564: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E568: 4800EA61  bl 0x8242cfc8
	ctx.lr = 0x8241E56C;
	sub_8242CFC8(ctx, base);
	// 8241E56C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E570: 40800010  bge 0x8241e580
	if !ctx.cr[0].lt {
	pc = 0x8241E580; continue 'dispatch;
	}
	// 8241E574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E578: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E57C: 4BFFE68D  bl 0x8241cc08
	ctx.lr = 0x8241E580;
	sub_8241CC08(ctx, base);
	// 8241E580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241E584: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E588: 409A0018  bne cr6, 0x8241e5a0
	if !ctx.cr[6].eq {
	pc = 0x8241E5A0; continue 'dispatch;
	}
	// 8241E58C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E590: 386B0B44  addi r3, r11, 0xb44
	ctx.r[3].s64 = ctx.r[11].s64 + 2884;
	// 8241E594: 4BFFE675  bl 0x8241cc08
	ctx.lr = 0x8241E598;
	sub_8241CC08(ctx, base);
	// 8241E598: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241E59C: 48000070  b 0x8241e60c
	pc = 0x8241E60C; continue 'dispatch;
	// 8241E5A0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E5A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8241E5A8: 409A0010  bne cr6, 0x8241e5b8
	if !ctx.cr[6].eq {
	pc = 0x8241E5B8; continue 'dispatch;
	}
	// 8241E5AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E5B0: 386B0B1C  addi r3, r11, 0xb1c
	ctx.r[3].s64 = ctx.r[11].s64 + 2844;
	// 8241E5B4: 4BFFFFE0  b 0x8241e594
	pc = 0x8241E594; continue 'dispatch;
	// 8241E5B8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8241E5BC: 409A000C  bne cr6, 0x8241e5c8
	if !ctx.cr[6].eq {
	pc = 0x8241E5C8; continue 'dispatch;
	}
	// 8241E5C0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8241E5C4: 48000024  b 0x8241e5e8
	pc = 0x8241E5E8; continue 'dispatch;
	// 8241E5C8: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8241E5CC: 409A000C  bne cr6, 0x8241e5d8
	if !ctx.cr[6].eq {
	pc = 0x8241E5D8; continue 'dispatch;
	}
	// 8241E5D0: 817F0120  lwz r11, 0x120(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 8241E5D4: 48000010  b 0x8241e5e4
	pc = 0x8241E5E4; continue 'dispatch;
	// 8241E5D8: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8241E5DC: 409A000C  bne cr6, 0x8241e5e8
	if !ctx.cr[6].eq {
	pc = 0x8241E5E8; continue 'dispatch;
	}
	// 8241E5E0: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 8241E5E4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8241E5E8: 815F0120  lwz r10, 0x120(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 8241E5EC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8241E5F0: 41980008  blt cr6, 0x8241e5f8
	if ctx.cr[6].lt {
	pc = 0x8241E5F8; continue 'dispatch;
	}
	// 8241E5F4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241E5F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E5FC: 41990008  bgt cr6, 0x8241e604
	if ctx.cr[6].gt {
	pc = 0x8241E604; continue 'dispatch;
	}
	// 8241E600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241E604: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8241E608: 917F0124  stw r11, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[11].u32 ) };
	// 8241E60C: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E610: 4800EA51  bl 0x8242d060
	ctx.lr = 0x8241E614;
	sub_8242D060(ctx, base);
	// 8241E614: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E618: 40800010  bge 0x8241e628
	if !ctx.cr[0].lt {
	pc = 0x8241E628; continue 'dispatch;
	}
	// 8241E61C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E620: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E624: 4BFFE5E5  bl 0x8241cc08
	ctx.lr = 0x8241E628;
	sub_8241CC08(ctx, base);
	// 8241E628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E62C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241E630: 48116AD8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E638 size=168
    let mut pc: u32 = 0x8241E638;
    'dispatch: loop {
        match pc {
            0x8241E638 => {
    //   block [0x8241E638..0x8241E6E0)
	// 8241E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241E640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241E644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241E648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E64C: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241E650: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E654: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E658: 4800E971  bl 0x8242cfc8
	ctx.lr = 0x8241E65C;
	sub_8242CFC8(ctx, base);
	// 8241E65C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E660: 40800010  bge 0x8241e670
	if !ctx.cr[0].lt {
	pc = 0x8241E670; continue 'dispatch;
	}
	// 8241E664: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E668: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E66C: 4BFFE59D  bl 0x8241cc08
	ctx.lr = 0x8241E670;
	sub_8241CC08(ctx, base);
	// 8241E670: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E674: 409A0018  bne cr6, 0x8241e68c
	if !ctx.cr[6].eq {
	pc = 0x8241E68C; continue 'dispatch;
	}
	// 8241E678: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E67C: 386B0B94  addi r3, r11, 0xb94
	ctx.r[3].s64 = ctx.r[11].s64 + 2964;
	// 8241E680: 4BFFE589  bl 0x8241cc08
	ctx.lr = 0x8241E684;
	sub_8241CC08(ctx, base);
	// 8241E684: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241E688: 48000020  b 0x8241e6a8
	pc = 0x8241E6A8; continue 'dispatch;
	// 8241E68C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E690: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E694: 409A0010  bne cr6, 0x8241e6a4
	if !ctx.cr[6].eq {
	pc = 0x8241E6A4; continue 'dispatch;
	}
	// 8241E698: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E69C: 386B0B6C  addi r3, r11, 0xb6c
	ctx.r[3].s64 = ctx.r[11].s64 + 2924;
	// 8241E6A0: 4BFFFFE0  b 0x8241e680
	pc = 0x8241E680; continue 'dispatch;
	// 8241E6A4: 83FF0124  lwz r31, 0x124(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 8241E6A8: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E6AC: 4800E9B5  bl 0x8242d060
	ctx.lr = 0x8241E6B0;
	sub_8242D060(ctx, base);
	// 8241E6B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E6B4: 40800010  bge 0x8241e6c4
	if !ctx.cr[0].lt {
	pc = 0x8241E6C4; continue 'dispatch;
	}
	// 8241E6B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E6BC: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E6C0: 4BFFE549  bl 0x8241cc08
	ctx.lr = 0x8241E6C4;
	sub_8241CC08(ctx, base);
	// 8241E6C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241E6C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241E6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241E6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241E6D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241E6D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241E6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E6E0 size=116
    let mut pc: u32 = 0x8241E6E0;
    'dispatch: loop {
        match pc {
            0x8241E6E0 => {
    //   block [0x8241E6E0..0x8241E754)
	// 8241E6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E6E4: 481169D5  bl 0x825350b8
	ctx.lr = 0x8241E6E8;
	sub_82535080(ctx, base);
	// 8241E6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E6EC: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8241E6F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241E6F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241E6F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8241E6FC: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E700: 4800E8C9  bl 0x8242cfc8
	ctx.lr = 0x8241E704;
	sub_8242CFC8(ctx, base);
	// 8241E704: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E708: 40800010  bge 0x8241e718
	if !ctx.cr[0].lt {
	pc = 0x8241E718; continue 'dispatch;
	}
	// 8241E70C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E710: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E714: 4BFFE4F5  bl 0x8241cc08
	ctx.lr = 0x8241E718;
	sub_8241CC08(ctx, base);
	// 8241E718: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8241E71C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241E720: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E724: 4BFFF2B5  bl 0x8241d9d8
	ctx.lr = 0x8241E728;
	sub_8241D9D8(ctx, base);
	// 8241E728: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241E72C: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E730: 4800E931  bl 0x8242d060
	ctx.lr = 0x8241E734;
	sub_8242D060(ctx, base);
	// 8241E734: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E738: 40800010  bge 0x8241e748
	if !ctx.cr[0].lt {
	pc = 0x8241E748; continue 'dispatch;
	}
	// 8241E73C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E740: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E744: 4BFFE4C5  bl 0x8241cc08
	ctx.lr = 0x8241E748;
	sub_8241CC08(ctx, base);
	// 8241E748: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241E750: 481169B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E758 size=116
    let mut pc: u32 = 0x8241E758;
    'dispatch: loop {
        match pc {
            0x8241E758 => {
    //   block [0x8241E758..0x8241E7CC)
	// 8241E758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E75C: 4811695D  bl 0x825350b8
	ctx.lr = 0x8241E760;
	sub_82535080(ctx, base);
	// 8241E760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E764: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8241E768: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241E76C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241E770: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8241E774: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E778: 4800E851  bl 0x8242cfc8
	ctx.lr = 0x8241E77C;
	sub_8242CFC8(ctx, base);
	// 8241E77C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E780: 40800010  bge 0x8241e790
	if !ctx.cr[0].lt {
	pc = 0x8241E790; continue 'dispatch;
	}
	// 8241E784: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E788: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E78C: 4BFFE47D  bl 0x8241cc08
	ctx.lr = 0x8241E790;
	sub_8241CC08(ctx, base);
	// 8241E790: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8241E794: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241E798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E79C: 4BFFF395  bl 0x8241db30
	ctx.lr = 0x8241E7A0;
	sub_8241DB30(ctx, base);
	// 8241E7A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241E7A4: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E7A8: 4800E8B9  bl 0x8242d060
	ctx.lr = 0x8241E7AC;
	sub_8242D060(ctx, base);
	// 8241E7AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E7B0: 40800010  bge 0x8241e7c0
	if !ctx.cr[0].lt {
	pc = 0x8241E7C0; continue 'dispatch;
	}
	// 8241E7B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E7B8: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E7BC: 4BFFE44D  bl 0x8241cc08
	ctx.lr = 0x8241E7C0;
	sub_8241CC08(ctx, base);
	// 8241E7C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241E7C8: 48116940  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E7D0 size=256
    let mut pc: u32 = 0x8241E7D0;
    'dispatch: loop {
        match pc {
            0x8241E7D0 => {
    //   block [0x8241E7D0..0x8241E8D0)
	// 8241E7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E7D4: 481168E5  bl 0x825350b8
	ctx.lr = 0x8241E7D8;
	sub_82535080(ctx, base);
	// 8241E7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E7DC: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8241E7E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E7E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241E7E8: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E7EC: 4800E7DD  bl 0x8242cfc8
	ctx.lr = 0x8241E7F0;
	sub_8242CFC8(ctx, base);
	// 8241E7F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E7F4: 40800010  bge 0x8241e804
	if !ctx.cr[0].lt {
	pc = 0x8241E804; continue 'dispatch;
	}
	// 8241E7F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E7FC: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E800: 4BFFE409  bl 0x8241cc08
	ctx.lr = 0x8241E804;
	sub_8241CC08(ctx, base);
	// 8241E804: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E808: 409A0018  bne cr6, 0x8241e820
	if !ctx.cr[6].eq {
	pc = 0x8241E820; continue 'dispatch;
	}
	// 8241E80C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E810: 386B0DA4  addi r3, r11, 0xda4
	ctx.r[3].s64 = ctx.r[11].s64 + 3492;
	// 8241E814: 4BFFE3F5  bl 0x8241cc08
	ctx.lr = 0x8241E818;
	sub_8241CC08(ctx, base);
	// 8241E818: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8241E81C: 4800008C  b 0x8241e8a8
	pc = 0x8241E8A8; continue 'dispatch;
	// 8241E820: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E824: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E828: 409A0010  bne cr6, 0x8241e838
	if !ctx.cr[6].eq {
	pc = 0x8241E838; continue 'dispatch;
	}
	// 8241E82C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E830: 386B0D7C  addi r3, r11, 0xd7c
	ctx.r[3].s64 = ctx.r[11].s64 + 3452;
	// 8241E834: 4BFFFFE0  b 0x8241e814
	pc = 0x8241E814; continue 'dispatch;
	// 8241E838: 2F3D0000  cmpdi cr6, r29, 0
	ctx.cr[6].compare_i64(ctx.r[29].s64, 0, &mut ctx.xer);
	// 8241E83C: 40980010  bge cr6, 0x8241e84c
	if !ctx.cr[6].lt {
	pc = 0x8241E84C; continue 'dispatch;
	}
	// 8241E840: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E844: 386B0D58  addi r3, r11, 0xd58
	ctx.r[3].s64 = ctx.r[11].s64 + 3416;
	// 8241E848: 4BFFFFCC  b 0x8241e814
	pc = 0x8241E814; continue 'dispatch;
	// 8241E84C: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241E850: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241E854: 4182FFC4  beq 0x8241e818
	if ctx.cr[0].eq {
	pc = 0x8241E818; continue 'dispatch;
	}
	// 8241E858: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241E85C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8241E860: 419A0018  beq cr6, 0x8241e878
	if ctx.cr[6].eq {
	pc = 0x8241E878; continue 'dispatch;
	}
	// 8241E864: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E868: 386B0D1C  addi r3, r11, 0xd1c
	ctx.r[3].s64 = ctx.r[11].s64 + 3356;
	// 8241E86C: 4BFFE39D  bl 0x8241cc08
	ctx.lr = 0x8241E870;
	sub_8241CC08(ctx, base);
	// 8241E870: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241E874: 48000034  b 0x8241e8a8
	pc = 0x8241E8A8; continue 'dispatch;
	// 8241E878: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241E87C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8241E880: 409AFFF0  bne cr6, 0x8241e870
	if !ctx.cr[6].eq {
	pc = 0x8241E870; continue 'dispatch;
	}
	// 8241E884: E95F0118  ld r10, 0x118(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) };
	// 8241E888: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241E88C: 7F2AE800  cmpd cr6, r10, r29
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[29].s64, &mut ctx.xer);
	// 8241E890: 419A0018  beq cr6, 0x8241e8a8
	if ctx.cr[6].eq {
	pc = 0x8241E8A8; continue 'dispatch;
	}
	// 8241E894: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8241E898: 917F0198  stw r11, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[11].u32 ) };
	// 8241E89C: FBBF01A0  std r29, 0x1a0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[29].u64 ) };
	// 8241E8A0: 93DF01AC  stw r30, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[30].u32 ) };
	// 8241E8A4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8241E8A8: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E8AC: 4800E7B5  bl 0x8242d060
	ctx.lr = 0x8241E8B0;
	sub_8242D060(ctx, base);
	// 8241E8B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E8B4: 40800010  bge 0x8241e8c4
	if !ctx.cr[0].lt {
	pc = 0x8241E8C4; continue 'dispatch;
	}
	// 8241E8B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E8BC: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E8C0: 4BFFE349  bl 0x8241cc08
	ctx.lr = 0x8241E8C4;
	sub_8241CC08(ctx, base);
	// 8241E8C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241E8CC: 4811683C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E8D0 size=196
    let mut pc: u32 = 0x8241E8D0;
    'dispatch: loop {
        match pc {
            0x8241E8D0 => {
    //   block [0x8241E8D0..0x8241E994)
	// 8241E8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E8D4: 481167E5  bl 0x825350b8
	ctx.lr = 0x8241E8D8;
	sub_82535080(ctx, base);
	// 8241E8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E8DC: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8241E8E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E8E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241E8E8: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E8EC: 4800E6DD  bl 0x8242cfc8
	ctx.lr = 0x8241E8F0;
	sub_8242CFC8(ctx, base);
	// 8241E8F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E8F4: 40800010  bge 0x8241e904
	if !ctx.cr[0].lt {
	pc = 0x8241E904; continue 'dispatch;
	}
	// 8241E8F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E8FC: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E900: 4BFFE309  bl 0x8241cc08
	ctx.lr = 0x8241E904;
	sub_8241CC08(ctx, base);
	// 8241E904: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E908: 409A0018  bne cr6, 0x8241e920
	if !ctx.cr[6].eq {
	pc = 0x8241E920; continue 'dispatch;
	}
	// 8241E90C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E910: 386B0DF4  addi r3, r11, 0xdf4
	ctx.r[3].s64 = ctx.r[11].s64 + 3572;
	// 8241E914: 4BFFE2F5  bl 0x8241cc08
	ctx.lr = 0x8241E918;
	sub_8241CC08(ctx, base);
	// 8241E918: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8241E91C: 48000050  b 0x8241e96c
	pc = 0x8241E96C; continue 'dispatch;
	// 8241E920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241E928: 409A0010  bne cr6, 0x8241e938
	if !ctx.cr[6].eq {
	pc = 0x8241E938; continue 'dispatch;
	}
	// 8241E92C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E930: 386B0DCC  addi r3, r11, 0xdcc
	ctx.r[3].s64 = ctx.r[11].s64 + 3532;
	// 8241E934: 4BFFFFE0  b 0x8241e914
	pc = 0x8241E914; continue 'dispatch;
	// 8241E938: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241E93C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241E940: 4182FFD8  beq 0x8241e918
	if ctx.cr[0].eq {
	pc = 0x8241E918; continue 'dispatch;
	}
	// 8241E944: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241E948: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241E94C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8241E950: 409A001C  bne cr6, 0x8241e96c
	if !ctx.cr[6].eq {
	pc = 0x8241E96C; continue 'dispatch;
	}
	// 8241E954: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8241E958: 917F01B8  stw r11, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[11].u32 ) };
	// 8241E95C: 93BF01BC  stw r29, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[29].u32 ) };
	// 8241E960: 93DF01C0  stw r30, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[30].u32 ) };
	// 8241E964: 93DF01C4  stw r30, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[30].u32 ) };
	// 8241E968: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8241E96C: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241E970: 4800E6F1  bl 0x8242d060
	ctx.lr = 0x8241E974;
	sub_8242D060(ctx, base);
	// 8241E974: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E978: 40800010  bge 0x8241e988
	if !ctx.cr[0].lt {
	pc = 0x8241E988; continue 'dispatch;
	}
	// 8241E97C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E980: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241E984: 4BFFE285  bl 0x8241cc08
	ctx.lr = 0x8241E988;
	sub_8241CC08(ctx, base);
	// 8241E988: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241E98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241E990: 48116778  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241E998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241E998 size=152
    let mut pc: u32 = 0x8241E998;
    'dispatch: loop {
        match pc {
            0x8241E998 => {
    //   block [0x8241E998..0x8241EA30)
	// 8241E998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241E99C: 48116721  bl 0x825350bc
	ctx.lr = 0x8241E9A0;
	sub_82535080(ctx, base);
	// 8241E9A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241E9A4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241E9A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241E9AC: 3BCB87C0  addi r30, r11, -0x7840
	ctx.r[30].s64 = ctx.r[11].s64 + -30784;
	// 8241E9B0: 807EFFF0  lwz r3, -0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8241E9B4: 4800E615  bl 0x8242cfc8
	ctx.lr = 0x8241E9B8;
	sub_8242CFC8(ctx, base);
	// 8241E9B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241E9BC: 40800010  bge 0x8241e9cc
	if !ctx.cr[0].lt {
	pc = 0x8241E9CC; continue 'dispatch;
	}
	// 8241E9C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E9C4: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241E9C8: 4BFFE241  bl 0x8241cc08
	ctx.lr = 0x8241E9CC;
	sub_8241CC08(ctx, base);
	// 8241E9CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241E9D0: 409A0018  bne cr6, 0x8241e9e8
	if !ctx.cr[6].eq {
	pc = 0x8241E9E8; continue 'dispatch;
	}
	// 8241E9D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241E9D8: 386B0E1C  addi r3, r11, 0xe1c
	ctx.r[3].s64 = ctx.r[11].s64 + 3612;
	// 8241E9DC: 4BFFE22D  bl 0x8241cc08
	ctx.lr = 0x8241E9E0;
	sub_8241CC08(ctx, base);
	// 8241E9E0: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 8241E9E4: 48000024  b 0x8241ea08
	pc = 0x8241EA08; continue 'dispatch;
	// 8241E9E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241E9EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241E9F0: 409AFFF0  bne cr6, 0x8241e9e0
	if !ctx.cr[6].eq {
	pc = 0x8241E9E0; continue 'dispatch;
	}
	// 8241E9F4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8241E9F8: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8241E9FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8241EA00: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241EA04: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8241EA08: 807EFFF0  lwz r3, -0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8241EA0C: 4800E655  bl 0x8242d060
	ctx.lr = 0x8241EA10;
	sub_8242D060(ctx, base);
	// 8241EA10: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EA14: 40800010  bge 0x8241ea24
	if !ctx.cr[0].lt {
	pc = 0x8241EA24; continue 'dispatch;
	}
	// 8241EA18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EA1C: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241EA20: 4BFFE1E9  bl 0x8241cc08
	ctx.lr = 0x8241EA24;
	sub_8241CC08(ctx, base);
	// 8241EA24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241EA28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241EA2C: 481166E0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241EA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241EA30 size=172
    let mut pc: u32 = 0x8241EA30;
    'dispatch: loop {
        match pc {
            0x8241EA30 => {
    //   block [0x8241EA30..0x8241EADC)
	// 8241EA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241EA34: 48116689  bl 0x825350bc
	ctx.lr = 0x8241EA38;
	sub_82535080(ctx, base);
	// 8241EA38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241EA3C: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8241EA40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241EA44: 807D87B0  lwz r3, -0x7850(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EA48: 4800E581  bl 0x8242cfc8
	ctx.lr = 0x8241EA4C;
	sub_8242CFC8(ctx, base);
	// 8241EA4C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EA50: 40800010  bge 0x8241ea60
	if !ctx.cr[0].lt {
	pc = 0x8241EA60; continue 'dispatch;
	}
	// 8241EA54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EA58: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241EA5C: 4BFFE1AD  bl 0x8241cc08
	ctx.lr = 0x8241EA60;
	sub_8241CC08(ctx, base);
	// 8241EA60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241EA64: 409A0014  bne cr6, 0x8241ea78
	if !ctx.cr[6].eq {
	pc = 0x8241EA78; continue 'dispatch;
	}
	// 8241EA68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EA6C: 386B0E6C  addi r3, r11, 0xe6c
	ctx.r[3].s64 = ctx.r[11].s64 + 3692;
	// 8241EA70: 4BFFE199  bl 0x8241cc08
	ctx.lr = 0x8241EA74;
	sub_8241CC08(ctx, base);
	// 8241EA74: 48000044  b 0x8241eab8
	pc = 0x8241EAB8; continue 'dispatch;
	// 8241EA78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EA7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EA80: 409A0010  bne cr6, 0x8241ea90
	if !ctx.cr[6].eq {
	pc = 0x8241EA90; continue 'dispatch;
	}
	// 8241EA84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EA88: 386B0E44  addi r3, r11, 0xe44
	ctx.r[3].s64 = ctx.r[11].s64 + 3652;
	// 8241EA8C: 4BFFFFE4  b 0x8241ea70
	pc = 0x8241EA70; continue 'dispatch;
	// 8241EA90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241EA94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241EA98: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8241EA9C: 409A0018  bne cr6, 0x8241eab4
	if !ctx.cr[6].eq {
	pc = 0x8241EAB4; continue 'dispatch;
	}
	// 8241EAA0: 809F0138  lwz r4, 0x138(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 8241EAA4: 807F01C8  lwz r3, 0x1c8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 8241EAA8: 4800E281  bl 0x8242cd28
	ctx.lr = 0x8241EAAC;
	sub_8242CD28(ctx, base);
	// 8241EAAC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8241EAB0: 93DF0138  stw r30, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[30].u32 ) };
	// 8241EAB4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8241EAB8: 807D87B0  lwz r3, -0x7850(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EABC: 4800E5A5  bl 0x8242d060
	ctx.lr = 0x8241EAC0;
	sub_8242D060(ctx, base);
	// 8241EAC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EAC4: 40800010  bge 0x8241ead4
	if !ctx.cr[0].lt {
	pc = 0x8241EAD4; continue 'dispatch;
	}
	// 8241EAC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EACC: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241EAD0: 4BFFE139  bl 0x8241cc08
	ctx.lr = 0x8241EAD4;
	sub_8241CC08(ctx, base);
	// 8241EAD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241EAD8: 48116634  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241EAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241EAE0 size=168
    let mut pc: u32 = 0x8241EAE0;
    'dispatch: loop {
        match pc {
            0x8241EAE0 => {
    //   block [0x8241EAE0..0x8241EB88)
	// 8241EAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241EAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241EAE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241EAEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241EAF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241EAF4: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241EAF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241EAFC: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EB00: 4800E4C9  bl 0x8242cfc8
	ctx.lr = 0x8241EB04;
	sub_8242CFC8(ctx, base);
	// 8241EB04: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EB08: 40800010  bge 0x8241eb18
	if !ctx.cr[0].lt {
	pc = 0x8241EB18; continue 'dispatch;
	}
	// 8241EB0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EB10: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241EB14: 4BFFE0F5  bl 0x8241cc08
	ctx.lr = 0x8241EB18;
	sub_8241CC08(ctx, base);
	// 8241EB18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241EB1C: 409A0018  bne cr6, 0x8241eb34
	if !ctx.cr[6].eq {
	pc = 0x8241EB34; continue 'dispatch;
	}
	// 8241EB20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EB24: 386B0EBC  addi r3, r11, 0xebc
	ctx.r[3].s64 = ctx.r[11].s64 + 3772;
	// 8241EB28: 4BFFE0E1  bl 0x8241cc08
	ctx.lr = 0x8241EB2C;
	sub_8241CC08(ctx, base);
	// 8241EB2C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241EB30: 48000020  b 0x8241eb50
	pc = 0x8241EB50; continue 'dispatch;
	// 8241EB34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EB38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EB3C: 409A0010  bne cr6, 0x8241eb4c
	if !ctx.cr[6].eq {
	pc = 0x8241EB4C; continue 'dispatch;
	}
	// 8241EB40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EB44: 386B0E94  addi r3, r11, 0xe94
	ctx.r[3].s64 = ctx.r[11].s64 + 3732;
	// 8241EB48: 4BFFFFE0  b 0x8241eb28
	pc = 0x8241EB28; continue 'dispatch;
	// 8241EB4C: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241EB50: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EB54: 4800E50D  bl 0x8242d060
	ctx.lr = 0x8241EB58;
	sub_8242D060(ctx, base);
	// 8241EB58: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EB5C: 40800010  bge 0x8241eb6c
	if !ctx.cr[0].lt {
	pc = 0x8241EB6C; continue 'dispatch;
	}
	// 8241EB60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EB64: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241EB68: 4BFFE0A1  bl 0x8241cc08
	ctx.lr = 0x8241EB6C;
	sub_8241CC08(ctx, base);
	// 8241EB6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241EB70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241EB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241EB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241EB7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241EB80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241EB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241EB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241EB88 size=176
    let mut pc: u32 = 0x8241EB88;
    'dispatch: loop {
        match pc {
            0x8241EB88 => {
    //   block [0x8241EB88..0x8241EC38)
	// 8241EB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241EB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241EB90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241EB94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241EB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241EB9C: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241EBA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241EBA4: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EBA8: 4800E421  bl 0x8242cfc8
	ctx.lr = 0x8241EBAC;
	sub_8242CFC8(ctx, base);
	// 8241EBAC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EBB0: 40800010  bge 0x8241ebc0
	if !ctx.cr[0].lt {
	pc = 0x8241EBC0; continue 'dispatch;
	}
	// 8241EBB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EBB8: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241EBBC: 4BFFE04D  bl 0x8241cc08
	ctx.lr = 0x8241EBC0;
	sub_8241CC08(ctx, base);
	// 8241EBC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241EBC4: 409A0018  bne cr6, 0x8241ebdc
	if !ctx.cr[6].eq {
	pc = 0x8241EBDC; continue 'dispatch;
	}
	// 8241EBC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EBCC: 386B0F38  addi r3, r11, 0xf38
	ctx.r[3].s64 = ctx.r[11].s64 + 3896;
	// 8241EBD0: 4BFFE039  bl 0x8241cc08
	ctx.lr = 0x8241EBD4;
	sub_8241CC08(ctx, base);
	// 8241EBD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241EBD8: 48000028  b 0x8241ec00
	pc = 0x8241EC00; continue 'dispatch;
	// 8241EBDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EBE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EBE4: 409A0010  bne cr6, 0x8241ebf4
	if !ctx.cr[6].eq {
	pc = 0x8241EBF4; continue 'dispatch;
	}
	// 8241EBE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EBEC: 386B0F10  addi r3, r11, 0xf10
	ctx.r[3].s64 = ctx.r[11].s64 + 3856;
	// 8241EBF0: 4BFFFFE0  b 0x8241ebd0
	pc = 0x8241EBD0; continue 'dispatch;
	// 8241EBF4: E97F0130  ld r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	// 8241EBF8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241EBFC: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241EC00: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EC04: 4800E45D  bl 0x8242d060
	ctx.lr = 0x8241EC08;
	sub_8242D060(ctx, base);
	// 8241EC08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EC0C: 40800010  bge 0x8241ec1c
	if !ctx.cr[0].lt {
	pc = 0x8241EC1C; continue 'dispatch;
	}
	// 8241EC10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EC14: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241EC18: 4BFFDFF1  bl 0x8241cc08
	ctx.lr = 0x8241EC1C;
	sub_8241CC08(ctx, base);
	// 8241EC1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241EC20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241EC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241EC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241EC2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241EC30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241EC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241EC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241EC38 size=176
    let mut pc: u32 = 0x8241EC38;
    'dispatch: loop {
        match pc {
            0x8241EC38 => {
    //   block [0x8241EC38..0x8241ECE8)
	// 8241EC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241EC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241EC40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241EC44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241EC48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241EC4C: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241EC50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241EC54: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EC58: 4800E371  bl 0x8242cfc8
	ctx.lr = 0x8241EC5C;
	sub_8242CFC8(ctx, base);
	// 8241EC5C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EC60: 40800010  bge 0x8241ec70
	if !ctx.cr[0].lt {
	pc = 0x8241EC70; continue 'dispatch;
	}
	// 8241EC64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EC68: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241EC6C: 4BFFDF9D  bl 0x8241cc08
	ctx.lr = 0x8241EC70;
	sub_8241CC08(ctx, base);
	// 8241EC70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241EC74: 409A0018  bne cr6, 0x8241ec8c
	if !ctx.cr[6].eq {
	pc = 0x8241EC8C; continue 'dispatch;
	}
	// 8241EC78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EC7C: 386B0F88  addi r3, r11, 0xf88
	ctx.r[3].s64 = ctx.r[11].s64 + 3976;
	// 8241EC80: 4BFFDF89  bl 0x8241cc08
	ctx.lr = 0x8241EC84;
	sub_8241CC08(ctx, base);
	// 8241EC84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241EC88: 48000028  b 0x8241ecb0
	pc = 0x8241ECB0; continue 'dispatch;
	// 8241EC8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EC90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EC94: 409A0010  bne cr6, 0x8241eca4
	if !ctx.cr[6].eq {
	pc = 0x8241ECA4; continue 'dispatch;
	}
	// 8241EC98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EC9C: 386B0F60  addi r3, r11, 0xf60
	ctx.r[3].s64 = ctx.r[11].s64 + 3936;
	// 8241ECA0: 4BFFFFE0  b 0x8241ec80
	pc = 0x8241EC80; continue 'dispatch;
	// 8241ECA4: E97F0130  ld r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	// 8241ECA8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241ECAC: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241ECB0: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241ECB4: 4800E3AD  bl 0x8242d060
	ctx.lr = 0x8241ECB8;
	sub_8242D060(ctx, base);
	// 8241ECB8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241ECBC: 40800010  bge 0x8241eccc
	if !ctx.cr[0].lt {
	pc = 0x8241ECCC; continue 'dispatch;
	}
	// 8241ECC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241ECC4: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241ECC8: 4BFFDF41  bl 0x8241cc08
	ctx.lr = 0x8241ECCC;
	sub_8241CC08(ctx, base);
	// 8241ECCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241ECD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241ECD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241ECD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241ECDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241ECE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241ECE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241ECE8 size=176
    let mut pc: u32 = 0x8241ECE8;
    'dispatch: loop {
        match pc {
            0x8241ECE8 => {
    //   block [0x8241ECE8..0x8241ED98)
	// 8241ECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241ECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241ECF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241ECF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241ECF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241ECFC: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241ED00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241ED04: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241ED08: 4800E2C1  bl 0x8242cfc8
	ctx.lr = 0x8241ED0C;
	sub_8242CFC8(ctx, base);
	// 8241ED0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241ED10: 40800010  bge 0x8241ed20
	if !ctx.cr[0].lt {
	pc = 0x8241ED20; continue 'dispatch;
	}
	// 8241ED14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241ED18: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241ED1C: 4BFFDEED  bl 0x8241cc08
	ctx.lr = 0x8241ED20;
	sub_8241CC08(ctx, base);
	// 8241ED20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241ED24: 409A0018  bne cr6, 0x8241ed3c
	if !ctx.cr[6].eq {
	pc = 0x8241ED3C; continue 'dispatch;
	}
	// 8241ED28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241ED2C: 386B0FD8  addi r3, r11, 0xfd8
	ctx.r[3].s64 = ctx.r[11].s64 + 4056;
	// 8241ED30: 4BFFDED9  bl 0x8241cc08
	ctx.lr = 0x8241ED34;
	sub_8241CC08(ctx, base);
	// 8241ED34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241ED38: 48000028  b 0x8241ed60
	pc = 0x8241ED60; continue 'dispatch;
	// 8241ED3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241ED40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241ED44: 409A0010  bne cr6, 0x8241ed54
	if !ctx.cr[6].eq {
	pc = 0x8241ED54; continue 'dispatch;
	}
	// 8241ED48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241ED4C: 386B0FB0  addi r3, r11, 0xfb0
	ctx.r[3].s64 = ctx.r[11].s64 + 4016;
	// 8241ED50: 4BFFFFE0  b 0x8241ed30
	pc = 0x8241ED30; continue 'dispatch;
	// 8241ED54: E97F0130  ld r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	// 8241ED58: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241ED5C: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8241ED60: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241ED64: 4800E2FD  bl 0x8242d060
	ctx.lr = 0x8241ED68;
	sub_8242D060(ctx, base);
	// 8241ED68: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241ED6C: 40800010  bge 0x8241ed7c
	if !ctx.cr[0].lt {
	pc = 0x8241ED7C; continue 'dispatch;
	}
	// 8241ED70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241ED74: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241ED78: 4BFFDE91  bl 0x8241cc08
	ctx.lr = 0x8241ED7C;
	sub_8241CC08(ctx, base);
	// 8241ED7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241ED80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241ED84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241ED88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241ED8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241ED90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241ED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241ED98 size=180
    let mut pc: u32 = 0x8241ED98;
    'dispatch: loop {
        match pc {
            0x8241ED98 => {
    //   block [0x8241ED98..0x8241EE4C)
	// 8241ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241EDA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241EDA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241EDA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241EDAC: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8241EDB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241EDB4: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EDB8: 4800E211  bl 0x8242cfc8
	ctx.lr = 0x8241EDBC;
	sub_8242CFC8(ctx, base);
	// 8241EDBC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EDC0: 40800010  bge 0x8241edd0
	if !ctx.cr[0].lt {
	pc = 0x8241EDD0; continue 'dispatch;
	}
	// 8241EDC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EDC8: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241EDCC: 4BFFDE3D  bl 0x8241cc08
	ctx.lr = 0x8241EDD0;
	sub_8241CC08(ctx, base);
	// 8241EDD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241EDD4: 409A0018  bne cr6, 0x8241edec
	if !ctx.cr[6].eq {
	pc = 0x8241EDEC; continue 'dispatch;
	}
	// 8241EDD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EDDC: 386B1028  addi r3, r11, 0x1028
	ctx.r[3].s64 = ctx.r[11].s64 + 4136;
	// 8241EDE0: 4BFFDE29  bl 0x8241cc08
	ctx.lr = 0x8241EDE4;
	sub_8241CC08(ctx, base);
	// 8241EDE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241EDE8: 4800002C  b 0x8241ee14
	pc = 0x8241EE14; continue 'dispatch;
	// 8241EDEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EDF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EDF4: 409A0010  bne cr6, 0x8241ee04
	if !ctx.cr[6].eq {
	pc = 0x8241EE04; continue 'dispatch;
	}
	// 8241EDF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EDFC: 386B1000  addi r3, r11, 0x1000
	ctx.r[3].s64 = ctx.r[11].s64 + 4096;
	// 8241EE00: 4BFFFFE0  b 0x8241ede0
	pc = 0x8241EDE0; continue 'dispatch;
	// 8241EE04: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8241EE08: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8241EE0C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8241EE10: 697F0001  xori r31, r11, 1
	ctx.r[31].u64 = ctx.r[11].u64 ^ 1;
	// 8241EE14: 807E87B0  lwz r3, -0x7850(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241EE18: 4800E249  bl 0x8242d060
	ctx.lr = 0x8241EE1C;
	sub_8242D060(ctx, base);
	// 8241EE1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EE20: 40800010  bge 0x8241ee30
	if !ctx.cr[0].lt {
	pc = 0x8241EE30; continue 'dispatch;
	}
	// 8241EE24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EE28: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241EE2C: 4BFFDDDD  bl 0x8241cc08
	ctx.lr = 0x8241EE30;
	sub_8241CC08(ctx, base);
	// 8241EE30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241EE34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241EE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241EE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241EE40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241EE44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241EE48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241EE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241EE50 size=196
    let mut pc: u32 = 0x8241EE50;
    'dispatch: loop {
        match pc {
            0x8241EE50 => {
    //   block [0x8241EE50..0x8241EF14)
	// 8241EE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241EE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241EE58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241EE5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241EE60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241EE64: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241EE68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241EE6C: 3BEB87B8  addi r31, r11, -0x7848
	ctx.r[31].s64 = ctx.r[11].s64 + -30792;
	// 8241EE70: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241EE74: 4800E155  bl 0x8242cfc8
	ctx.lr = 0x8241EE78;
	sub_8242CFC8(ctx, base);
	// 8241EE78: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EE7C: 40800010  bge 0x8241ee8c
	if !ctx.cr[0].lt {
	pc = 0x8241EE8C; continue 'dispatch;
	}
	// 8241EE80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EE84: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241EE88: 4BFFDD81  bl 0x8241cc08
	ctx.lr = 0x8241EE8C;
	sub_8241CC08(ctx, base);
	// 8241EE8C: 397FFFF0  addi r11, r31, -0x10
	ctx.r[11].s64 = ctx.r[31].s64 + -16;
	// 8241EE90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EE94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EE98: 409A0048  bne cr6, 0x8241eee0
	if !ctx.cr[6].eq {
	pc = 0x8241EEE0; continue 'dispatch;
	}
	// 8241EE9C: 397FFFF0  addi r11, r31, -0x10
	ctx.r[11].s64 = ctx.r[31].s64 + -16;
	// 8241EEA0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EEA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8241EEA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241EEAC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241EEB0: 409A0018  bne cr6, 0x8241eec8
	if !ctx.cr[6].eq {
	pc = 0x8241EEC8; continue 'dispatch;
	}
	// 8241EEB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EEB8: 386B1050  addi r3, r11, 0x1050
	ctx.r[3].s64 = ctx.r[11].s64 + 4176;
	// 8241EEBC: 4BFFDD4D  bl 0x8241cc08
	ctx.lr = 0x8241EEC0;
	sub_8241CC08(ctx, base);
	// 8241EEC0: 48000020  b 0x8241eee0
	pc = 0x8241EEE0; continue 'dispatch;
	// 8241EEC4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EEC8: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 8241EECC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241EED0: 388BD280  addi r4, r11, -0x2d80
	ctx.r[4].s64 = ctx.r[11].s64 + -11648;
	// 8241EED4: 4800DABD  bl 0x8242c990
	ctx.lr = 0x8241EED8;
	sub_8242C990(ctx, base);
	// 8241EED8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EEDC: 4081FFE8  ble 0x8241eec4
	if !ctx.cr[0].gt {
	pc = 0x8241EEC4; continue 'dispatch;
	}
	// 8241EEE0: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241EEE4: 4800E17D  bl 0x8242d060
	ctx.lr = 0x8241EEE8;
	sub_8242D060(ctx, base);
	// 8241EEE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EEEC: 40800010  bge 0x8241eefc
	if !ctx.cr[0].lt {
	pc = 0x8241EEFC; continue 'dispatch;
	}
	// 8241EEF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EEF4: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241EEF8: 4BFFDD11  bl 0x8241cc08
	ctx.lr = 0x8241EEFC;
	sub_8241CC08(ctx, base);
	// 8241EEFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241EF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241EF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241EF08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241EF0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241EF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241EF18 size=264
    let mut pc: u32 = 0x8241EF18;
    'dispatch: loop {
        match pc {
            0x8241EF18 => {
    //   block [0x8241EF18..0x8241F020)
	// 8241EF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241EF1C: 481161A1  bl 0x825350bc
	ctx.lr = 0x8241EF20;
	sub_82535080(ctx, base);
	// 8241EF20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241EF24: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241EF28: 3BEB87BC  addi r31, r11, -0x7844
	ctx.r[31].s64 = ctx.r[11].s64 + -30788;
	// 8241EF2C: 391FFFF0  addi r8, r31, -0x10
	ctx.r[8].s64 = ctx.r[31].s64 + -16;
	// 8241EF30: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241EF34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241EF38: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241EF3C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8241EF40: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241EF44: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241EF48: 4082FFE8  bne 0x8241ef30
	if !ctx.cr[0].eq {
	pc = 0x8241EF30; continue 'dispatch;
	}
	// 8241EF4C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241EF50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EF54: 409A00C4  bne cr6, 0x8241f018
	if !ctx.cr[6].eq {
	pc = 0x8241F018; continue 'dispatch;
	}
	// 8241EF58: 807FFFF4  lwz r3, -0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241EF5C: 4800E06D  bl 0x8242cfc8
	ctx.lr = 0x8241EF60;
	sub_8242CFC8(ctx, base);
	// 8241EF60: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EF64: 40800010  bge 0x8241ef74
	if !ctx.cr[0].lt {
	pc = 0x8241EF74; continue 'dispatch;
	}
	// 8241EF68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EF6C: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241EF70: 4BFFDC99  bl 0x8241cc08
	ctx.lr = 0x8241EF74;
	sub_8241CC08(ctx, base);
	// 8241EF74: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241EF78: 3BABF6A8  addi r29, r11, -0x958
	ctx.r[29].s64 = ctx.r[11].s64 + -2392;
	// 8241EF7C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8241EF80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EF84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241EF88: 419A000C  beq cr6, 0x8241ef94
	if ctx.cr[6].eq {
	pc = 0x8241EF94; continue 'dispatch;
	}
	// 8241EF8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241EF90: 4BFFE931  bl 0x8241d8c0
	ctx.lr = 0x8241EF94;
	sub_8241D8C0(ctx, base);
	// 8241EF94: 3D7D0001  addis r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 65536;
	// 8241EF98: 3BDE01D0  addi r30, r30, 0x1d0
	ctx.r[30].s64 = ctx.r[30].s64 + 464;
	// 8241EF9C: 396B9100  addi r11, r11, -0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + -28416;
	// 8241EFA0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241EFA4: 4198FFDC  blt cr6, 0x8241ef80
	if ctx.cr[6].lt {
	pc = 0x8241EF80; continue 'dispatch;
	}
	// 8241EFA8: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241EFAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241EFB0: 419A0010  beq cr6, 0x8241efc0
	if ctx.cr[6].eq {
	pc = 0x8241EFC0; continue 'dispatch;
	}
	// 8241EFB4: 4800DDB5  bl 0x8242cd68
	ctx.lr = 0x8241EFB8;
	sub_8242CD68(ctx, base);
	// 8241EFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241EFBC: 917FFFFC  stw r11, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 8241EFC0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241EFC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241EFC8: 419A0010  beq cr6, 0x8241efd8
	if ctx.cr[6].eq {
	pc = 0x8241EFD8; continue 'dispatch;
	}
	// 8241EFCC: 4800DD9D  bl 0x8242cd68
	ctx.lr = 0x8241EFD0;
	sub_8242CD68(ctx, base);
	// 8241EFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241EFD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241EFD8: 4800D6B1  bl 0x8242c688
	ctx.lr = 0x8241EFDC;
	sub_8242C688(ctx, base);
	// 8241EFDC: 807FFFF4  lwz r3, -0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241EFE0: 4800E081  bl 0x8242d060
	ctx.lr = 0x8241EFE4;
	sub_8242D060(ctx, base);
	// 8241EFE4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241EFE8: 40800010  bge 0x8241eff8
	if !ctx.cr[0].lt {
	pc = 0x8241EFF8; continue 'dispatch;
	}
	// 8241EFEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241EFF0: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241EFF4: 4BFFDC15  bl 0x8241cc08
	ctx.lr = 0x8241EFF8;
	sub_8241CC08(ctx, base);
	// 8241EFF8: 807FFFF4  lwz r3, -0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8241EFFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241F000: 419A0010  beq cr6, 0x8241f010
	if ctx.cr[6].eq {
	pc = 0x8241F010; continue 'dispatch;
	}
	// 8241F004: 4800DF35  bl 0x8242cf38
	ctx.lr = 0x8241F008;
	sub_8242CF38(ctx, base);
	// 8241F008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241F00C: 917FFFF4  stw r11, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 8241F010: 4800CC69  bl 0x8242bc78
	ctx.lr = 0x8241F014;
	sub_8242BC78(ctx, base);
	// 8241F014: 4800D265  bl 0x8242c278
	ctx.lr = 0x8241F018;
	sub_8242C278(ctx, base);
	// 8241F018: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241F01C: 481160F0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F020 size=264
    let mut pc: u32 = 0x8241F020;
    'dispatch: loop {
        match pc {
            0x8241F020 => {
    //   block [0x8241F020..0x8241F128)
	// 8241F020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F024: 4811608D  bl 0x825350b0
	ctx.lr = 0x8241F028;
	sub_82535080(ctx, base);
	// 8241F028: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F02C: 3F40828A  lis r26, -0x7d76
	ctx.r[26].s64 = -2104885248;
	// 8241F030: 807A87B0  lwz r3, -0x7850(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241F034: 4800DF95  bl 0x8242cfc8
	ctx.lr = 0x8241F038;
	sub_8242CFC8(ctx, base);
	// 8241F038: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241F03C: 40800010  bge 0x8241f04c
	if !ctx.cr[0].lt {
	pc = 0x8241F04C; continue 'dispatch;
	}
	// 8241F040: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F044: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241F048: 4BFFDBC1  bl 0x8241cc08
	ctx.lr = 0x8241F04C;
	sub_8241CC08(ctx, base);
	// 8241F04C: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241F050: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241F054: 3B6BF6A8  addi r27, r11, -0x958
	ctx.r[27].s64 = ctx.r[11].s64 + -2392;
	// 8241F058: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8241F05C: 3BFB0188  addi r31, r27, 0x188
	ctx.r[31].s64 = ctx.r[27].s64 + 392;
	// 8241F060: 3BCBF4E8  addi r30, r11, -0xb18
	ctx.r[30].s64 = ctx.r[11].s64 + -2840;
	// 8241F064: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F068: 3B8B070C  addi r28, r11, 0x70c
	ctx.r[28].s64 = ctx.r[11].s64 + 1804;
	// 8241F06C: 817FFE78  lwz r11, -0x188(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-392 as u32) ) } as u64;
	// 8241F070: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241F074: 409A007C  bne cr6, 0x8241f0f0
	if !ctx.cr[6].eq {
	pc = 0x8241F0F0; continue 'dispatch;
	}
	// 8241F078: 817FFE80  lwz r11, -0x180(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-384 as u32) ) } as u64;
	// 8241F07C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8241F080: 409A0070  bne cr6, 0x8241f0f0
	if !ctx.cr[6].eq {
	pc = 0x8241F0F0; continue 'dispatch;
	}
	// 8241F084: 809FFFB0  lwz r4, -0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-80 as u32) ) } as u64;
	// 8241F088: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8241F08C: 4800DABD  bl 0x8242cb48
	ctx.lr = 0x8241F090;
	sub_8242CB48(ctx, base);
	// 8241F090: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241F094: 409A005C  bne cr6, 0x8241f0f0
	if !ctx.cr[6].eq {
	pc = 0x8241F0F0; continue 'dispatch;
	}
	// 8241F098: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F09C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241F0A0: 409A0020  bne cr6, 0x8241f0c0
	if !ctx.cr[6].eq {
	pc = 0x8241F0C0; continue 'dispatch;
	}
	// 8241F0A4: 817FFFA0  lwz r11, -0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-96 as u32) ) } as u64;
	// 8241F0A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8241F0AC: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241F0B0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8241F0B4: 915FFE7C  stw r10, -0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-388 as u32), ctx.r[10].u32 ) };
	// 8241F0B8: F97FFFA8  std r11, -0x58(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(-88 as u32), ctx.r[11].u64 ) };
	// 8241F0BC: 4800002C  b 0x8241f0e8
	pc = 0x8241F0E8; continue 'dispatch;
	// 8241F0C0: 38DFFE88  addi r6, r31, -0x178
	ctx.r[6].s64 = ctx.r[31].s64 + -376;
	// 8241F0C4: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F0C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8241F0CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241F0D0: 48127ED9  bl 0x82546fa8
	ctx.lr = 0x8241F0D4;
	sub_82546FA8(ctx, base);
	// 8241F0D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241F0D8: 4BFFDB31  bl 0x8241cc08
	ctx.lr = 0x8241F0DC;
	sub_8241CC08(ctx, base);
	// 8241F0DC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8241F0E0: FBBFFFA8  std r29, -0x58(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(-88 as u32), ctx.r[29].u64 ) };
	// 8241F0E4: 917FFE7C  stw r11, -0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-388 as u32), ctx.r[11].u32 ) };
	// 8241F0E8: 93BFFE80  stw r29, -0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-384 as u32), ctx.r[29].u32 ) };
	// 8241F0EC: 93BFFFB0  stw r29, -0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-80 as u32), ctx.r[29].u32 ) };
	// 8241F0F0: 3D7B0001  addis r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 65536;
	// 8241F0F4: 3BFF01D0  addi r31, r31, 0x1d0
	ctx.r[31].s64 = ctx.r[31].s64 + 464;
	// 8241F0F8: 396B9288  addi r11, r11, -0x6d78
	ctx.r[11].s64 = ctx.r[11].s64 + -28024;
	// 8241F0FC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241F100: 4198FF6C  blt cr6, 0x8241f06c
	if ctx.cr[6].lt {
	pc = 0x8241F06C; continue 'dispatch;
	}
	// 8241F104: 807A87B0  lwz r3, -0x7850(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241F108: 4800DF59  bl 0x8242d060
	ctx.lr = 0x8241F10C;
	sub_8242D060(ctx, base);
	// 8241F10C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241F110: 40800010  bge 0x8241f120
	if !ctx.cr[0].lt {
	pc = 0x8241F120; continue 'dispatch;
	}
	// 8241F114: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F118: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241F11C: 4BFFDAED  bl 0x8241cc08
	ctx.lr = 0x8241F120;
	sub_8241CC08(ctx, base);
	// 8241F120: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241F124: 48115FDC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F128 size=100
    let mut pc: u32 = 0x8241F128;
    'dispatch: loop {
        match pc {
            0x8241F128 => {
    //   block [0x8241F128..0x8241F18C)
	// 8241F128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F130: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241F134: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F138: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8241F13C: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241F140: 4800DE89  bl 0x8242cfc8
	ctx.lr = 0x8241F144;
	sub_8242CFC8(ctx, base);
	// 8241F144: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241F148: 40800010  bge 0x8241f158
	if !ctx.cr[0].lt {
	pc = 0x8241F158; continue 'dispatch;
	}
	// 8241F14C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F150: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241F154: 4BFFDAB5  bl 0x8241cc08
	ctx.lr = 0x8241F158;
	sub_8241CC08(ctx, base);
	// 8241F158: 4BFFECD9  bl 0x8241de30
	ctx.lr = 0x8241F15C;
	sub_8241DE30(ctx, base);
	// 8241F15C: 807F87B0  lwz r3, -0x7850(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241F160: 4800DF01  bl 0x8242d060
	ctx.lr = 0x8241F164;
	sub_8242D060(ctx, base);
	// 8241F164: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241F168: 40800010  bge 0x8241f178
	if !ctx.cr[0].lt {
	pc = 0x8241F178; continue 'dispatch;
	}
	// 8241F16C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F170: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241F174: 4BFFDA95  bl 0x8241cc08
	ctx.lr = 0x8241F178;
	sub_8241CC08(ctx, base);
	// 8241F178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241F188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F190 size=216
    let mut pc: u32 = 0x8241F190;
    'dispatch: loop {
        match pc {
            0x8241F190 => {
    //   block [0x8241F190..0x8241F268)
	// 8241F190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F19C: 2F04012D  cmpwi cr6, r4, 0x12d
	ctx.cr[6].compare_i32(ctx.r[4].s32, 301, &mut ctx.xer);
	// 8241F1A0: 41990060  bgt cr6, 0x8241f200
	if ctx.cr[6].gt {
	pc = 0x8241F200; continue 'dispatch;
	}
	// 8241F1A4: 419A0054  beq cr6, 0x8241f1f8
	if ctx.cr[6].eq {
	pc = 0x8241F1F8; continue 'dispatch;
	}
	// 8241F1A8: 2F0400C8  cmpwi cr6, r4, 0xc8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 200, &mut ctx.xer);
	// 8241F1AC: 419A0044  beq cr6, 0x8241f1f0
	if ctx.cr[6].eq {
	pc = 0x8241F1F0; continue 'dispatch;
	}
	// 8241F1B0: 2F0400C9  cmpwi cr6, r4, 0xc9
	ctx.cr[6].compare_i32(ctx.r[4].s32, 201, &mut ctx.xer);
	// 8241F1B4: 419A0034  beq cr6, 0x8241f1e8
	if ctx.cr[6].eq {
	pc = 0x8241F1E8; continue 'dispatch;
	}
	// 8241F1B8: 2F0400CA  cmpwi cr6, r4, 0xca
	ctx.cr[6].compare_i32(ctx.r[4].s32, 202, &mut ctx.xer);
	// 8241F1BC: 419A0024  beq cr6, 0x8241f1e0
	if ctx.cr[6].eq {
	pc = 0x8241F1E0; continue 'dispatch;
	}
	// 8241F1C0: 2F0400CB  cmpwi cr6, r4, 0xcb
	ctx.cr[6].compare_i32(ctx.r[4].s32, 203, &mut ctx.xer);
	// 8241F1C4: 419A0014  beq cr6, 0x8241f1d8
	if ctx.cr[6].eq {
	pc = 0x8241F1D8; continue 'dispatch;
	}
	// 8241F1C8: 2F04012C  cmpwi cr6, r4, 0x12c
	ctx.cr[6].compare_i32(ctx.r[4].s32, 300, &mut ctx.xer);
	// 8241F1CC: 409A005C  bne cr6, 0x8241f228
	if !ctx.cr[6].eq {
	pc = 0x8241F228; continue 'dispatch;
	}
	// 8241F1D0: 4BFFF0F1  bl 0x8241e2c0
	ctx.lr = 0x8241F1D4;
	sub_8241E2C0(ctx, base);
	// 8241F1D4: 48000084  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F1D8: 4BFFE291  bl 0x8241d468
	ctx.lr = 0x8241F1DC;
	sub_8241D468(ctx, base);
	// 8241F1DC: 4800007C  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F1E0: 4BFFE209  bl 0x8241d3e8
	ctx.lr = 0x8241F1E4;
	sub_8241D3E8(ctx, base);
	// 8241F1E4: 48000074  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F1E8: 4BFFFB01  bl 0x8241ece8
	ctx.lr = 0x8241F1EC;
	sub_8241ECE8(ctx, base);
	// 8241F1EC: 4800006C  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F1F0: 4BFFFA49  bl 0x8241ec38
	ctx.lr = 0x8241F1F4;
	sub_8241EC38(ctx, base);
	// 8241F1F4: 48000064  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F1F8: 4BFFF179  bl 0x8241e370
	ctx.lr = 0x8241F1FC;
	sub_8241E370(ctx, base);
	// 8241F1FC: 4800005C  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F200: 2F04012E  cmpwi cr6, r4, 0x12e
	ctx.cr[6].compare_i32(ctx.r[4].s32, 302, &mut ctx.xer);
	// 8241F204: 419A0050  beq cr6, 0x8241f254
	if ctx.cr[6].eq {
	pc = 0x8241F254; continue 'dispatch;
	}
	// 8241F208: 2F040190  cmpwi cr6, r4, 0x190
	ctx.cr[6].compare_i32(ctx.r[4].s32, 400, &mut ctx.xer);
	// 8241F20C: 419A0040  beq cr6, 0x8241f24c
	if ctx.cr[6].eq {
	pc = 0x8241F24C; continue 'dispatch;
	}
	// 8241F210: 2F0401F4  cmpwi cr6, r4, 0x1f4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 500, &mut ctx.xer);
	// 8241F214: 419A002C  beq cr6, 0x8241f240
	if ctx.cr[6].eq {
	pc = 0x8241F240; continue 'dispatch;
	}
	// 8241F218: 2F0401F5  cmpwi cr6, r4, 0x1f5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 501, &mut ctx.xer);
	// 8241F21C: 419A001C  beq cr6, 0x8241f238
	if ctx.cr[6].eq {
	pc = 0x8241F238; continue 'dispatch;
	}
	// 8241F220: 2F040258  cmpwi cr6, r4, 0x258
	ctx.cr[6].compare_i32(ctx.r[4].s32, 600, &mut ctx.xer);
	// 8241F224: 419A000C  beq cr6, 0x8241f230
	if ctx.cr[6].eq {
	pc = 0x8241F230; continue 'dispatch;
	}
	// 8241F228: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241F22C: 4800002C  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F230: 4BFFFB69  bl 0x8241ed98
	ctx.lr = 0x8241F234;
	sub_8241ED98(ctx, base);
	// 8241F234: 48000024  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F238: 4BFFEAB9  bl 0x8241dcf0
	ctx.lr = 0x8241F23C;
	sub_8241DCF0(ctx, base);
	// 8241F23C: 4800001C  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F240: 4BFFFEE9  bl 0x8241f128
	ctx.lr = 0x8241F244;
	sub_8241F128(ctx, base);
	// 8241F244: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241F248: 48000010  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F24C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241F250: 48000008  b 0x8241f258
	pc = 0x8241F258; continue 'dispatch;
	// 8241F254: 4BFFF1CD  bl 0x8241e420
	ctx.lr = 0x8241F258;
	sub_8241E420(ctx, base);
	// 8241F258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F268 size=176
    let mut pc: u32 = 0x8241F268;
    'dispatch: loop {
        match pc {
            0x8241F268 => {
    //   block [0x8241F268..0x8241F318)
	// 8241F268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F26C: 48115E4D  bl 0x825350b8
	ctx.lr = 0x8241F270;
	sub_82535080(ctx, base);
	// 8241F270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F274: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 8241F278: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241F27C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241F280: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241F284: 4800DD45  bl 0x8242cfc8
	ctx.lr = 0x8241F288;
	sub_8242CFC8(ctx, base);
	// 8241F288: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241F28C: 40800010  bge 0x8241f29c
	if !ctx.cr[0].lt {
	pc = 0x8241F29C; continue 'dispatch;
	}
	// 8241F290: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F294: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 8241F298: 4BFFD971  bl 0x8241cc08
	ctx.lr = 0x8241F29C;
	sub_8241CC08(ctx, base);
	// 8241F29C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8241F2A0: 409A0018  bne cr6, 0x8241f2b8
	if !ctx.cr[6].eq {
	pc = 0x8241F2B8; continue 'dispatch;
	}
	// 8241F2A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F2A8: 386B10E4  addi r3, r11, 0x10e4
	ctx.r[3].s64 = ctx.r[11].s64 + 4324;
	// 8241F2AC: 4BFFD95D  bl 0x8241cc08
	ctx.lr = 0x8241F2B0;
	sub_8241CC08(ctx, base);
	// 8241F2B0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241F2B4: 4800003C  b 0x8241f2f0
	pc = 0x8241F2F0; continue 'dispatch;
	// 8241F2B8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241F2BC: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8241F2C0: 41980024  blt cr6, 0x8241f2e4
	if ctx.cr[6].lt {
	pc = 0x8241F2E4; continue 'dispatch;
	}
	// 8241F2C4: 419A0014  beq cr6, 0x8241f2d8
	if ctx.cr[6].eq {
	pc = 0x8241F2D8; continue 'dispatch;
	}
	// 8241F2C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F2CC: 386B10B8  addi r3, r11, 0x10b8
	ctx.r[3].s64 = ctx.r[11].s64 + 4280;
	// 8241F2D0: 4BFFD939  bl 0x8241cc08
	ctx.lr = 0x8241F2D4;
	sub_8241CC08(ctx, base);
	// 8241F2D4: 4800001C  b 0x8241f2f0
	pc = 0x8241F2F0; continue 'dispatch;
	// 8241F2D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241F2DC: 4BFFE4D5  bl 0x8241d7b0
	ctx.lr = 0x8241F2E0;
	sub_8241D7B0(ctx, base);
	// 8241F2E0: 4800000C  b 0x8241f2ec
	pc = 0x8241F2EC; continue 'dispatch;
	// 8241F2E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241F2E8: 4BFFE2B9  bl 0x8241d5a0
	ctx.lr = 0x8241F2EC;
	sub_8241D5A0(ctx, base);
	// 8241F2EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241F2F0: 807C87B0  lwz r3, -0x7850(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-30800 as u32) ) } as u64;
	// 8241F2F4: 4800DD6D  bl 0x8242d060
	ctx.lr = 0x8241F2F8;
	sub_8242D060(ctx, base);
	// 8241F2F8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241F2FC: 40800010  bge 0x8241f30c
	if !ctx.cr[0].lt {
	pc = 0x8241F30C; continue 'dispatch;
	}
	// 8241F300: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241F304: 386B0660  addi r3, r11, 0x660
	ctx.r[3].s64 = ctx.r[11].s64 + 1632;
	// 8241F308: 4BFFD901  bl 0x8241cc08
	ctx.lr = 0x8241F30C;
	sub_8241CC08(ctx, base);
	// 8241F30C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241F310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241F314: 48115DF4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F318 size=84
    let mut pc: u32 = 0x8241F318;
    'dispatch: loop {
        match pc {
            0x8241F318 => {
    //   block [0x8241F318..0x8241F36C)
	// 8241F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F324: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F328: 390B87CC  addi r8, r11, -0x7834
	ctx.r[8].s64 = ctx.r[11].s64 + -30772;
	// 8241F32C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241F330: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241F334: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241F338: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241F33C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241F340: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241F344: 4082FFE8  bne 0x8241f32c
	if !ctx.cr[0].eq {
	pc = 0x8241F32C; continue 'dispatch;
	}
	// 8241F348: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241F34C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241F350: 409A000C  bne cr6, 0x8241f35c
	if !ctx.cr[6].eq {
	pc = 0x8241F35C; continue 'dispatch;
	}
	// 8241F354: 48001BE5  bl 0x82420f38
	ctx.lr = 0x8241F358;
	sub_82420F38(ctx, base);
	// 8241F358: 4800E099  bl 0x8242d3f0
	ctx.lr = 0x8241F35C;
	sub_8242D3F0(ctx, base);
	// 8241F35C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F370 size=84
    let mut pc: u32 = 0x8241F370;
    'dispatch: loop {
        match pc {
            0x8241F370 => {
    //   block [0x8241F370..0x8241F3C4)
	// 8241F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F37C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F380: 390B87CC  addi r8, r11, -0x7834
	ctx.r[8].s64 = ctx.r[11].s64 + -30772;
	// 8241F384: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8241F388: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241F38C: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8241F390: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8241F394: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241F398: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241F39C: 4082FFE8  bne 0x8241f384
	if !ctx.cr[0].eq {
	pc = 0x8241F384; continue 'dispatch;
	}
	// 8241F3A0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241F3A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241F3A8: 409A000C  bne cr6, 0x8241f3b4
	if !ctx.cr[6].eq {
	pc = 0x8241F3B4; continue 'dispatch;
	}
	// 8241F3AC: 4800E275  bl 0x8242d620
	ctx.lr = 0x8241F3B0;
	sub_8242D620(ctx, base);
	// 8241F3B0: 48001C09  bl 0x82420fb8
	ctx.lr = 0x8241F3B4;
	sub_82420FB8(ctx, base);
	// 8241F3B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F3C8 size=40
    let mut pc: u32 = 0x8241F3C8;
    'dispatch: loop {
        match pc {
            0x8241F3C8 => {
    //   block [0x8241F3C8..0x8241F3F0)
	// 8241F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F3D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F3D4: 48001C5D  bl 0x82421030
	ctx.lr = 0x8241F3D8;
	sub_82421030(ctx, base);
	// 8241F3D8: 4800E339  bl 0x8242d710
	ctx.lr = 0x8241F3DC;
	sub_8242D710(ctx, base);
	// 8241F3DC: 48001C95  bl 0x82421070
	ctx.lr = 0x8241F3E0;
	sub_82421070(ctx, base);
	// 8241F3E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241F3F0 size=20
    let mut pc: u32 = 0x8241F3F0;
    'dispatch: loop {
        match pc {
            0x8241F3F0 => {
    //   block [0x8241F3F0..0x8241F404)
	// 8241F3F0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F3F4: 396B8D94  addi r11, r11, -0x726c
	ctx.r[11].s64 = ctx.r[11].s64 + -29292;
	// 8241F3F8: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8241F3FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F400: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F404(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241F404 size=20
    let mut pc: u32 = 0x8241F404;
    'dispatch: loop {
        match pc {
            0x8241F404 => {
    //   block [0x8241F404..0x8241F418)
	// 8241F404: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 8241F408: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F40C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F410: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F414: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241F418 size=4
    let mut pc: u32 = 0x8241F418;
    'dispatch: loop {
        match pc {
            0x8241F418 => {
    //   block [0x8241F418..0x8241F41C)
	// 8241F418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F420 size=108
    let mut pc: u32 = 0x8241F420;
    'dispatch: loop {
        match pc {
            0x8241F420 => {
    //   block [0x8241F420..0x8241F48C)
	// 8241F420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F424: 48115C99  bl 0x825350bc
	ctx.lr = 0x8241F428;
	sub_82535080(ctx, base);
	// 8241F428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F42C: 3FA0828A  lis r29, -0x7d76
	ctx.r[29].s64 = -2104885248;
	// 8241F430: 83DD8D98  lwz r30, -0x7268(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-29288 as u32) ) } as u64;
	// 8241F434: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241F438: 409A0044  bne cr6, 0x8241f47c
	if !ctx.cr[6].eq {
	pc = 0x8241F47C; continue 'dispatch;
	}
	// 8241F43C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F440: 38A00280  li r5, 0x280
	ctx.r[5].s64 = 640;
	// 8241F444: 3BEB87D0  addi r31, r11, -0x7830
	ctx.r[31].s64 = ctx.r[11].s64 + -30768;
	// 8241F448: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241F44C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8241F450: 48115D81  bl 0x825351d0
	ctx.lr = 0x8241F454;
	sub_825351D0(ctx, base);
	// 8241F454: 387F03C0  addi r3, r31, 0x3c0
	ctx.r[3].s64 = ctx.r[31].s64 + 960;
	// 8241F458: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8241F45C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241F460: 48115D71  bl 0x825351d0
	ctx.lr = 0x8241F464;
	sub_825351D0(ctx, base);
	// 8241F464: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 8241F468: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241F46C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241F470: 48115D61  bl 0x825351d0
	ctx.lr = 0x8241F474;
	sub_825351D0(ctx, base);
	// 8241F474: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241F478: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8241F47C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 8241F480: 917D8D98  stw r11, -0x7268(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-29288 as u32), ctx.r[11].u32 ) };
	// 8241F484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241F488: 48115C84  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F490 size=180
    let mut pc: u32 = 0x8241F490;
    'dispatch: loop {
        match pc {
            0x8241F490 => {
    //   block [0x8241F490..0x8241F544)
	// 8241F490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F498: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241F49C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241F4A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F4A4: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8241F4A8: 816A8D98  lwz r11, -0x7268(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29288 as u32) ) } as u64;
	// 8241F4AC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241F4B0: 916A8D98  stw r11, -0x7268(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29288 as u32), ctx.r[11].u32 ) };
	// 8241F4B4: 40820078  bne 0x8241f52c
	if !ctx.cr[0].eq {
	pc = 0x8241F52C; continue 'dispatch;
	}
	// 8241F4B8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F4BC: 3BCB87D0  addi r30, r11, -0x7830
	ctx.r[30].s64 = ctx.r[11].s64 + -30768;
	// 8241F4C0: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 8241F4C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F4C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241F4CC: 41820014  beq 0x8241f4e0
	if ctx.cr[0].eq {
	pc = 0x8241F4E0; continue 'dispatch;
	}
	// 8241F4D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F4D4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241F4D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F4DC: 4E800421  bctrl
	ctx.lr = 0x8241F4E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F4E0: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 8241F4E4: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8241F4E8: 396B0280  addi r11, r11, 0x280
	ctx.r[11].s64 = ctx.r[11].s64 + 640;
	// 8241F4EC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241F4F0: 4198FFD4  blt cr6, 0x8241f4c4
	if ctx.cr[6].lt {
	pc = 0x8241F4C4; continue 'dispatch;
	}
	// 8241F4F4: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 8241F4F8: 38A00280  li r5, 0x280
	ctx.r[5].s64 = 640;
	// 8241F4FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241F500: 48115CD1  bl 0x825351d0
	ctx.lr = 0x8241F504;
	sub_825351D0(ctx, base);
	// 8241F504: 387E03C0  addi r3, r30, 0x3c0
	ctx.r[3].s64 = ctx.r[30].s64 + 960;
	// 8241F508: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8241F50C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241F510: 48115CC1  bl 0x825351d0
	ctx.lr = 0x8241F514;
	sub_825351D0(ctx, base);
	// 8241F514: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 8241F518: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241F51C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241F520: 48115CB1  bl 0x825351d0
	ctx.lr = 0x8241F524;
	sub_825351D0(ctx, base);
	// 8241F524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241F528: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8241F52C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241F530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F538: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241F53C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241F540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F548 size=132
    let mut pc: u32 = 0x8241F548;
    'dispatch: loop {
        match pc {
            0x8241F548 => {
    //   block [0x8241F548..0x8241F5CC)
	// 8241F548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F54C: 48115B69  bl 0x825350b4
	ctx.lr = 0x8241F550;
	sub_82535080(ctx, base);
	// 8241F550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F554: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8241F558: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8241F55C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8241F560: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F564: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241F568: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8241F56C: 409AFFF4  bne cr6, 0x8241f560
	if !ctx.cr[6].eq {
	pc = 0x8241F560; continue 'dispatch;
	}
	// 8241F570: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8241F574: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8241F578: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8241F57C: 3BAA8B90  addi r29, r10, -0x7470
	ctx.r[29].s64 = ctx.r[10].s64 + -29808;
	// 8241F580: 557C003E  slwi r28, r11, 0
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8241F584: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241F588: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8241F58C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8241F590: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241F594: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8241F598: 48113BF9  bl 0x82533190
	ctx.lr = 0x8241F59C;
	sub_82533190(ctx, base);
	// 8241F59C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241F5A0: 41820020  beq 0x8241f5c0
	if ctx.cr[0].eq {
	pc = 0x8241F5C0; continue 'dispatch;
	}
	// 8241F5A4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8241F5A8: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 8241F5AC: 2B1E0020  cmplwi cr6, r30, 0x20
	ctx.cr[6].compare_u32(ctx.r[30].u32, 32 as u32, &mut ctx.xer);
	// 8241F5B0: 4198FFDC  blt cr6, 0x8241f58c
	if ctx.cr[6].lt {
	pc = 0x8241F58C; continue 'dispatch;
	}
	// 8241F5B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241F5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241F5BC: 48115B48  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8241F5C0: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241F5C4: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8241F5C8: 4BFFFFF0  b 0x8241f5b8
	pc = 0x8241F5B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F5D0 size=356
    let mut pc: u32 = 0x8241F5D0;
    'dispatch: loop {
        match pc {
            0x8241F5D0 => {
    //   block [0x8241F5D0..0x8241F734)
	// 8241F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F5D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241F5DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241F5E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F5E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241F5E8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8241F5EC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8241F5F0: 419A012C  beq cr6, 0x8241f71c
	if ctx.cr[6].eq {
	pc = 0x8241F71C; continue 'dispatch;
	}
	// 8241F5F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241F5F8: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8241F5FC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8241F600: 7CE5F850  subf r7, r5, r31
	ctx.r[7].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	// 8241F604: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F608: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 8241F60C: 2F08003A  cmpwi cr6, r8, 0x3a
	ctx.cr[6].compare_i32(ctx.r[8].s32, 58, &mut ctx.xer);
	// 8241F610: 419A0020  beq cr6, 0x8241f630
	if ctx.cr[6].eq {
	pc = 0x8241F630; continue 'dispatch;
	}
	// 8241F614: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8241F618: 419A0018  beq cr6, 0x8241f630
	if ctx.cr[6].eq {
	pc = 0x8241F630; continue 'dispatch;
	}
	// 8241F61C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241F620: 7D2751AE  stbx r9, r7, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 8241F624: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241F628: 2F0B0129  cmpwi cr6, r11, 0x129
	ctx.cr[6].compare_i32(ctx.r[11].s32, 297, &mut ctx.xer);
	// 8241F62C: 4198FFD8  blt cr6, 0x8241f604
	if ctx.cr[6].lt {
	pc = 0x8241F604; continue 'dispatch;
	}
	// 8241F630: 7D4B28AE  lbzx r10, r11, r5
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8241F634: 7FCBF9AE  stbx r30, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u8) };
	// 8241F638: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F63C: 409A003C  bne cr6, 0x8241f678
	if !ctx.cr[6].eq {
	pc = 0x8241F678; continue 'dispatch;
	}
	// 8241F640: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8241F644: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8241F648: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F64C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241F650: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8241F654: 409AFFF4  bne cr6, 0x8241f648
	if !ctx.cr[6].eq {
	pc = 0x8241F648; continue 'dispatch;
	}
	// 8241F658: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8241F65C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241F660: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8241F664: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241F668: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 8241F66C: 481154E5  bl 0x82534b50
	ctx.lr = 0x8241F670;
	sub_82534B50(ctx, base);
	// 8241F670: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 8241F674: 480000A8  b 0x8241f71c
	pc = 0x8241F71C; continue 'dispatch;
	// 8241F678: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 8241F67C: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 8241F680: 409A000C  bne cr6, 0x8241f68c
	if !ctx.cr[6].eq {
	pc = 0x8241F68C; continue 'dispatch;
	}
	// 8241F684: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8241F688: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 8241F68C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8241F690: 2F090129  cmpwi cr6, r9, 0x129
	ctx.cr[6].compare_i32(ctx.r[9].s32, 297, &mut ctx.xer);
	// 8241F694: 40980024  bge cr6, 0x8241f6b8
	if !ctx.cr[6].lt {
	pc = 0x8241F6B8; continue 'dispatch;
	}
	// 8241F698: 7D091850  subf r8, r9, r3
	ctx.r[8].s64 = ctx.r[3].s64 - ctx.r[9].s64;
	// 8241F69C: 7D6A28AE  lbzx r11, r10, r5
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8241F6A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241F6A4: 41820014  beq 0x8241f6b8
	if ctx.cr[0].eq {
	pc = 0x8241F6B8; continue 'dispatch;
	}
	// 8241F6A8: 7D6851AE  stbx r11, r8, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 8241F6AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241F6B0: 2F0A0129  cmpwi cr6, r10, 0x129
	ctx.cr[6].compare_i32(ctx.r[10].s32, 297, &mut ctx.xer);
	// 8241F6B4: 4198FFE8  blt cr6, 0x8241f69c
	if ctx.cr[6].lt {
	pc = 0x8241F69C; continue 'dispatch;
	}
	// 8241F6B8: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8241F6BC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8241F6C0: 7FCA19AE  stbx r30, r10, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[30].u8) };
	// 8241F6C4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8241F6C8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F6CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241F6D0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8241F6D4: 409AFFF4  bne cr6, 0x8241f6c8
	if !ctx.cr[6].eq {
	pc = 0x8241F6C8; continue 'dispatch;
	}
	// 8241F6D8: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8241F6DC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8241F6E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8241F6E4: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241F6E8: 352A0001  addic. r9, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8241F6EC: 41820030  beq 0x8241f71c
	if ctx.cr[0].eq {
	pc = 0x8241F71C; continue 'dispatch;
	}
	// 8241F6F0: 7D4BF8AE  lbzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8241F6F4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8241F6F8: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 8241F6FC: 41980014  blt cr6, 0x8241f710
	if ctx.cr[6].lt {
	pc = 0x8241F710; continue 'dispatch;
	}
	// 8241F700: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 8241F704: 4199000C  bgt cr6, 0x8241f710
	if ctx.cr[6].gt {
	pc = 0x8241F710; continue 'dispatch;
	}
	// 8241F708: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8241F70C: 7D4BF9AE  stbx r10, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u8) };
	// 8241F710: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241F714: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8241F718: 4198FFD8  blt cr6, 0x8241f6f0
	if ctx.cr[6].lt {
	pc = 0x8241F6F0; continue 'dispatch;
	}
	// 8241F71C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241F720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F728: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241F72C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241F730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F738 size=172
    let mut pc: u32 = 0x8241F738;
    'dispatch: loop {
        match pc {
            0x8241F738 => {
    //   block [0x8241F738..0x8241F7E4)
	// 8241F738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241F744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241F74C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241F750: 409A0024  bne cr6, 0x8241f774
	if !ctx.cr[6].eq {
	pc = 0x8241F774; continue 'dispatch;
	}
	// 8241F754: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F758: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F75C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F760: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F764: 419A006C  beq cr6, 0x8241f7d0
	if ctx.cr[6].eq {
	pc = 0x8241F7D0; continue 'dispatch;
	}
	// 8241F768: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F76C: 388A1158  addi r4, r10, 0x1158
	ctx.r[4].s64 = ctx.r[10].s64 + 4440;
	// 8241F770: 4800004C  b 0x8241f7bc
	pc = 0x8241F7BC; continue 'dispatch;
	// 8241F774: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F778: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241F77C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241F780: 41820020  beq 0x8241f7a0
	if ctx.cr[0].eq {
	pc = 0x8241F7A0; continue 'dispatch;
	}
	// 8241F784: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F78C: 4E800421  bctrl
	ctx.lr = 0x8241F790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241F794: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8241F798: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241F79C: 48000034  b 0x8241f7d0
	pc = 0x8241F7D0; continue 'dispatch;
	// 8241F7A0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F7A4: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F7A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F7AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F7B0: 419A0020  beq cr6, 0x8241f7d0
	if ctx.cr[6].eq {
	pc = 0x8241F7D0; continue 'dispatch;
	}
	// 8241F7B4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F7B8: 388A1140  addi r4, r10, 0x1140
	ctx.r[4].s64 = ctx.r[10].s64 + 4416;
	// 8241F7BC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F7C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241F7C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F7C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F7CC: 4E800421  bctrl
	ctx.lr = 0x8241F7D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F7D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F7DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241F7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F7E8 size=192
    let mut pc: u32 = 0x8241F7E8;
    'dispatch: loop {
        match pc {
            0x8241F7E8 => {
    //   block [0x8241F7E8..0x8241F8A8)
	// 8241F7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F7F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241F7F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F7F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241F7FC: 409A003C  bne cr6, 0x8241f838
	if !ctx.cr[6].eq {
	pc = 0x8241F838; continue 'dispatch;
	}
	// 8241F800: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F804: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F808: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F80C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F810: 419A0020  beq cr6, 0x8241f830
	if ctx.cr[6].eq {
	pc = 0x8241F830; continue 'dispatch;
	}
	// 8241F814: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F818: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F81C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F820: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241F824: 388A118C  addi r4, r10, 0x118c
	ctx.r[4].s64 = ctx.r[10].s64 + 4492;
	// 8241F828: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F82C: 4E800421  bctrl
	ctx.lr = 0x8241F830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241F834: 48000060  b 0x8241f894
	pc = 0x8241F894; continue 'dispatch;
	// 8241F838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F83C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241F840: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241F844: 41820018  beq 0x8241f85c
	if ctx.cr[0].eq {
	pc = 0x8241F85C; continue 'dispatch;
	}
	// 8241F848: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F850: 4E800421  bctrl
	ctx.lr = 0x8241F854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241F858: 48000038  b 0x8241f890
	pc = 0x8241F890; continue 'dispatch;
	// 8241F85C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F860: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241F864: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F868: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F86C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F870: 419A0020  beq cr6, 0x8241f890
	if ctx.cr[6].eq {
	pc = 0x8241F890; continue 'dispatch;
	}
	// 8241F874: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F878: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F87C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F880: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241F884: 388A1174  addi r4, r10, 0x1174
	ctx.r[4].s64 = ctx.r[10].s64 + 4468;
	// 8241F888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F88C: 4E800421  bctrl
	ctx.lr = 0x8241F890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241F894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F8A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241F8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F8A8 size=192
    let mut pc: u32 = 0x8241F8A8;
    'dispatch: loop {
        match pc {
            0x8241F8A8 => {
    //   block [0x8241F8A8..0x8241F968)
	// 8241F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F8B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241F8B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F8B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241F8BC: 409A003C  bne cr6, 0x8241f8f8
	if !ctx.cr[6].eq {
	pc = 0x8241F8F8; continue 'dispatch;
	}
	// 8241F8C0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F8C4: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F8C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F8CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F8D0: 419A0020  beq cr6, 0x8241f8f0
	if ctx.cr[6].eq {
	pc = 0x8241F8F0; continue 'dispatch;
	}
	// 8241F8D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F8D8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F8DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F8E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241F8E4: 388A11C0  addi r4, r10, 0x11c0
	ctx.r[4].s64 = ctx.r[10].s64 + 4544;
	// 8241F8E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F8EC: 4E800421  bctrl
	ctx.lr = 0x8241F8F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F8F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241F8F4: 48000060  b 0x8241f954
	pc = 0x8241F954; continue 'dispatch;
	// 8241F8F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F8FC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241F900: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241F904: 41820018  beq 0x8241f91c
	if ctx.cr[0].eq {
	pc = 0x8241F91C; continue 'dispatch;
	}
	// 8241F908: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F90C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F910: 4E800421  bctrl
	ctx.lr = 0x8241F914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241F918: 48000038  b 0x8241f950
	pc = 0x8241F950; continue 'dispatch;
	// 8241F91C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F920: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241F924: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F928: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F92C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F930: 419A0020  beq cr6, 0x8241f950
	if ctx.cr[6].eq {
	pc = 0x8241F950; continue 'dispatch;
	}
	// 8241F934: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F938: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F93C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F940: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241F944: 388A11A8  addi r4, r10, 0x11a8
	ctx.r[4].s64 = ctx.r[10].s64 + 4520;
	// 8241F948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F94C: 4E800421  bctrl
	ctx.lr = 0x8241F950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241F954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241F958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241F95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241F960: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241F964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241F968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241F968 size=192
    let mut pc: u32 = 0x8241F968;
    'dispatch: loop {
        match pc {
            0x8241F968 => {
    //   block [0x8241F968..0x8241FA28)
	// 8241F968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241F96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241F970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241F974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241F978: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241F97C: 409A003C  bne cr6, 0x8241f9b8
	if !ctx.cr[6].eq {
	pc = 0x8241F9B8; continue 'dispatch;
	}
	// 8241F980: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F984: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F988: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F98C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F990: 419A0020  beq cr6, 0x8241f9b0
	if ctx.cr[6].eq {
	pc = 0x8241F9B0; continue 'dispatch;
	}
	// 8241F994: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F998: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F99C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F9A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241F9A4: 388A11F4  addi r4, r10, 0x11f4
	ctx.r[4].s64 = ctx.r[10].s64 + 4596;
	// 8241F9A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F9AC: 4E800421  bctrl
	ctx.lr = 0x8241F9B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F9B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241F9B4: 48000060  b 0x8241fa14
	pc = 0x8241FA14; continue 'dispatch;
	// 8241F9B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F9BC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8241F9C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241F9C4: 41820018  beq 0x8241f9dc
	if ctx.cr[0].eq {
	pc = 0x8241F9DC; continue 'dispatch;
	}
	// 8241F9C8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F9CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241F9D0: 4E800421  bctrl
	ctx.lr = 0x8241F9D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241F9D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241F9D8: 48000038  b 0x8241fa10
	pc = 0x8241FA10; continue 'dispatch;
	// 8241F9DC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241F9E0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241F9E4: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241F9E8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241F9EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241F9F0: 419A0020  beq cr6, 0x8241fa10
	if ctx.cr[6].eq {
	pc = 0x8241FA10; continue 'dispatch;
	}
	// 8241F9F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241F9F8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241F9FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FA00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FA04: 388A11DC  addi r4, r10, 0x11dc
	ctx.r[4].s64 = ctx.r[10].s64 + 4572;
	// 8241FA08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FA0C: 4E800421  bctrl
	ctx.lr = 0x8241FA10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241FA14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241FA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FA20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241FA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FA28 size=104
    let mut pc: u32 = 0x8241FA28;
    'dispatch: loop {
        match pc {
            0x8241FA28 => {
    //   block [0x8241FA28..0x8241FA90)
	// 8241FA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241FA30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241FA34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241FA38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FA3C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FA40: 3BCB8B90  addi r30, r11, -0x7470
	ctx.r[30].s64 = ctx.r[11].s64 + -29808;
	// 8241FA44: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8241FA48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FA4C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FA50: 41820018  beq 0x8241fa68
	if ctx.cr[0].eq {
	pc = 0x8241FA68; continue 'dispatch;
	}
	// 8241FA54: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FA58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FA5C: 4182000C  beq 0x8241fa68
	if ctx.cr[0].eq {
	pc = 0x8241FA68; continue 'dispatch;
	}
	// 8241FA60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FA64: 4E800421  bctrl
	ctx.lr = 0x8241FA68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FA68: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 8241FA6C: 397E0200  addi r11, r30, 0x200
	ctx.r[11].s64 = ctx.r[30].s64 + 512;
	// 8241FA70: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241FA74: 4198FFD4  blt cr6, 0x8241fa48
	if ctx.cr[6].lt {
	pc = 0x8241FA48; continue 'dispatch;
	}
	// 8241FA78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241FA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FA84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241FA88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241FA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FA90 size=120
    let mut pc: u32 = 0x8241FA90;
    'dispatch: loop {
        match pc {
            0x8241FA90 => {
    //   block [0x8241FA90..0x8241FB08)
	// 8241FA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241FA98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241FA9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241FAA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FAA4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FAA8: 3BCB8B90  addi r30, r11, -0x7470
	ctx.r[30].s64 = ctx.r[11].s64 + -29808;
	// 8241FAAC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8241FAB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FAB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FAB8: 41820028  beq 0x8241fae0
	if ctx.cr[0].eq {
	pc = 0x8241FAE0; continue 'dispatch;
	}
	// 8241FABC: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241FAC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FAC4: 4182001C  beq 0x8241fae0
	if ctx.cr[0].eq {
	pc = 0x8241FAE0; continue 'dispatch;
	}
	// 8241FAC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241FACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FAD0: 388001F4  li r4, 0x1f4
	ctx.r[4].s64 = 500;
	// 8241FAD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241FAD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FADC: 4E800421  bctrl
	ctx.lr = 0x8241FAE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FAE0: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 8241FAE4: 397E0200  addi r11, r30, 0x200
	ctx.r[11].s64 = ctx.r[30].s64 + 512;
	// 8241FAE8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241FAEC: 4198FFC4  blt cr6, 0x8241fab0
	if ctx.cr[6].lt {
	pc = 0x8241FAB0; continue 'dispatch;
	}
	// 8241FAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241FAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FAFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241FB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241FB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FB08 size=192
    let mut pc: u32 = 0x8241FB08;
    'dispatch: loop {
        match pc {
            0x8241FB08 => {
    //   block [0x8241FB08..0x8241FBC8)
	// 8241FB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241FB10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241FB14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FB18: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8241FB1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241FB20: 409A003C  bne cr6, 0x8241fb5c
	if !ctx.cr[6].eq {
	pc = 0x8241FB5C; continue 'dispatch;
	}
	// 8241FB24: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FB28: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241FB2C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FB30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241FB34: 419A0020  beq cr6, 0x8241fb54
	if ctx.cr[6].eq {
	pc = 0x8241FB54; continue 'dispatch;
	}
	// 8241FB38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241FB3C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FB40: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FB48: 388A122C  addi r4, r10, 0x122c
	ctx.r[4].s64 = ctx.r[10].s64 + 4652;
	// 8241FB4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FB50: 4E800421  bctrl
	ctx.lr = 0x8241FB54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FB54: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8241FB58: 4800005C  b 0x8241fbb4
	pc = 0x8241FBB4; continue 'dispatch;
	// 8241FB5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FB60: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8241FB64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FB68: 41820018  beq 0x8241fb80
	if ctx.cr[0].eq {
	pc = 0x8241FB80; continue 'dispatch;
	}
	// 8241FB6C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FB70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FB74: 4E800421  bctrl
	ctx.lr = 0x8241FB78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FB78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241FB7C: 48000034  b 0x8241fbb0
	pc = 0x8241FBB0; continue 'dispatch;
	// 8241FB80: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FB84: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241FB88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FB8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241FB90: 419A0020  beq cr6, 0x8241fbb0
	if ctx.cr[6].eq {
	pc = 0x8241FBB0; continue 'dispatch;
	}
	// 8241FB94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241FB98: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FB9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FBA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FBA4: 388A1210  addi r4, r10, 0x1210
	ctx.r[4].s64 = ctx.r[10].s64 + 4624;
	// 8241FBA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FBAC: 4E800421  bctrl
	ctx.lr = 0x8241FBB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FBB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241FBB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241FBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FBC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241FBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FBC8 size=140
    let mut pc: u32 = 0x8241FBC8;
    'dispatch: loop {
        match pc {
            0x8241FBC8 => {
    //   block [0x8241FBC8..0x8241FC54)
	// 8241FBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241FBD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FBD4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8241FBD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241FBDC: 409A003C  bne cr6, 0x8241fc18
	if !ctx.cr[6].eq {
	pc = 0x8241FC18; continue 'dispatch;
	}
	// 8241FBE0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FBE4: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241FBE8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FBEC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241FBF0: 419A0020  beq cr6, 0x8241fc10
	if ctx.cr[6].eq {
	pc = 0x8241FC10; continue 'dispatch;
	}
	// 8241FBF4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241FBF8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FBFC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FC00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FC04: 388A1248  addi r4, r10, 0x1248
	ctx.r[4].s64 = ctx.r[10].s64 + 4680;
	// 8241FC08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FC0C: 4E800421  bctrl
	ctx.lr = 0x8241FC10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FC10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241FC14: 48000030  b 0x8241fc44
	pc = 0x8241FC44; continue 'dispatch;
	// 8241FC18: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FC1C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241FC20: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241FC24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FC28: 4182001C  beq 0x8241fc44
	if ctx.cr[0].eq {
	pc = 0x8241FC44; continue 'dispatch;
	}
	// 8241FC2C: 806A0004  lwz r3, 4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FC30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241FC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FC38: 38800258  li r4, 0x258
	ctx.r[4].s64 = 600;
	// 8241FC3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FC40: 4E800421  bctrl
	ctx.lr = 0x8241FC44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FC44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241FC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241FC58 size=16
    let mut pc: u32 = 0x8241FC58;
    'dispatch: loop {
        match pc {
            0x8241FC58 => {
    //   block [0x8241FC58..0x8241FC68)
	// 8241FC58: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8241FC5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241FC60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241FC64: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241FC68 size=16
    let mut pc: u32 = 0x8241FC68;
    'dispatch: loop {
        match pc {
            0x8241FC68 => {
    //   block [0x8241FC68..0x8241FC78)
	// 8241FC68: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FC6C: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241FC70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FC74: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241FC78 size=24
    let mut pc: u32 = 0x8241FC78;
    'dispatch: loop {
        match pc {
            0x8241FC78 => {
    //   block [0x8241FC78..0x8241FC90)
	// 8241FC78: 806A0004  lwz r3, 4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241FC80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FC84: 388001F5  li r4, 0x1f5
	ctx.r[4].s64 = 501;
	// 8241FC88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FC8C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241FC90 size=4
    let mut pc: u32 = 0x8241FC90;
    'dispatch: loop {
        match pc {
            0x8241FC90 => {
    //   block [0x8241FC90..0x8241FC94)
	// 8241FC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241FC98 size=32
    let mut pc: u32 = 0x8241FC98;
    'dispatch: loop {
        match pc {
            0x8241FC98 => {
    //   block [0x8241FC98..0x8241FCB8)
	// 8241FC98: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FC9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241FCA0: 396B8D94  addi r11, r11, -0x726c
	ctx.r[11].s64 = ctx.r[11].s64 + -29292;
	// 8241FCA4: 409A0014  bne cr6, 0x8241fcb8
	if !ctx.cr[6].eq {
		sub_8241FCB8(ctx, base);
		return;
	}
	// 8241FCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241FCAC: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 8241FCB0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241FCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241FCB8 size=12
    let mut pc: u32 = 0x8241FCB8;
    'dispatch: loop {
        match pc {
            0x8241FCB8 => {
    //   block [0x8241FCB8..0x8241FCC4)
	// 8241FCB8: 906BFFFC  stw r3, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[3].u32 ) };
	// 8241FCBC: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8241FCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FCC8 size=152
    let mut pc: u32 = 0x8241FCC8;
    'dispatch: loop {
        match pc {
            0x8241FCC8 => {
    //   block [0x8241FCC8..0x8241FD60)
	// 8241FCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FCCC: 481153F1  bl 0x825350bc
	ctx.lr = 0x8241FCD0;
	sub_82535080(ctx, base);
	// 8241FCD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FCD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241FCD8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FCDC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241FCE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241FCE4: 3BCB87D0  addi r30, r11, -0x7830
	ctx.r[30].s64 = ctx.r[11].s64 + -30768;
	// 8241FCE8: 409A0008  bne cr6, 0x8241fcf0
	if !ctx.cr[6].eq {
	pc = 0x8241FCF0; continue 'dispatch;
	}
	// 8241FCEC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8241FCF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241FCF4: 4BFFF855  bl 0x8241f548
	ctx.lr = 0x8241FCF8;
	sub_8241F548(ctx, base);
	// 8241FCF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FCFC: 4182005C  beq 0x8241fd58
	if ctx.cr[0].eq {
	pc = 0x8241FD58; continue 'dispatch;
	}
	// 8241FD00: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241FD04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241FD08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FD0C: 41820018  beq 0x8241fd24
	if ctx.cr[0].eq {
	pc = 0x8241FD24; continue 'dispatch;
	}
	// 8241FD10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241FD14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FD18: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 8241FD1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FD20: 4E800421  bctrl
	ctx.lr = 0x8241FD24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FD24: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8241FD28: 409A0030  bne cr6, 0x8241fd58
	if !ctx.cr[6].eq {
	pc = 0x8241FD58; continue 'dispatch;
	}
	// 8241FD2C: 387E0290  addi r3, r30, 0x290
	ctx.r[3].s64 = ctx.r[30].s64 + 656;
	// 8241FD30: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241FD34: 38800129  li r4, 0x129
	ctx.r[4].s64 = 297;
	// 8241FD38: 480037F1  bl 0x82423528
	ctx.lr = 0x8241FD3C;
	sub_82423528(ctx, base);
	// 8241FD3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241FD40: 38FE0290  addi r7, r30, 0x290
	ctx.r[7].s64 = ctx.r[30].s64 + 656;
	// 8241FD44: 38AB1270  addi r5, r11, 0x1270
	ctx.r[5].s64 = ctx.r[11].s64 + 4720;
	// 8241FD48: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8241FD4C: 38800129  li r4, 0x129
	ctx.r[4].s64 = 297;
	// 8241FD50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241FD54: 48003895  bl 0x824235e8
	ctx.lr = 0x8241FD58;
	sub_824235E8(ctx, base);
	// 8241FD58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241FD5C: 481153B0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FD60 size=208
    let mut pc: u32 = 0x8241FD60;
    'dispatch: loop {
        match pc {
            0x8241FD60 => {
    //   block [0x8241FD60..0x8241FE30)
	// 8241FD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241FD68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241FD6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241FD70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FD74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241FD78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241FD7C: 409A003C  bne cr6, 0x8241fdb8
	if !ctx.cr[6].eq {
	pc = 0x8241FDB8; continue 'dispatch;
	}
	// 8241FD80: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FD84: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241FD88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FD8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241FD90: 419A0020  beq cr6, 0x8241fdb0
	if ctx.cr[6].eq {
	pc = 0x8241FDB0; continue 'dispatch;
	}
	// 8241FD94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241FD98: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FD9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FDA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FDA4: 388A1278  addi r4, r10, 0x1278
	ctx.r[4].s64 = ctx.r[10].s64 + 4728;
	// 8241FDA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FDAC: 4E800421  bctrl
	ctx.lr = 0x8241FDB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FDB0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241FDB4: 48000064  b 0x8241fe18
	pc = 0x8241FE18; continue 'dispatch;
	// 8241FDB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FDBC: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 8241FDC0: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8241FDC4: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241FDC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FDCC: 4182004C  beq 0x8241fe18
	if ctx.cr[0].eq {
	pc = 0x8241FE18; continue 'dispatch;
	}
	// 8241FDD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FDD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241FDD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FDDC: 3880012D  li r4, 0x12d
	ctx.r[4].s64 = 301;
	// 8241FDE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FDE4: 4E800421  bctrl
	ctx.lr = 0x8241FDE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FDE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FDEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241FDF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241FDF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FDFC: 3880012E  li r4, 0x12e
	ctx.r[4].s64 = 302;
	// 8241FE00: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241FE04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FE08: 4E800421  bctrl
	ctx.lr = 0x8241FE0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FE0C: 7BCA07C6  sldi r10, r30, 0x20
	ctx.r[10].u64 = ctx.r[30].u64.wrapping_shl(32);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8241FE10: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 8241FE14: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241FE18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241FE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FE24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241FE28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241FE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FE30 size=140
    let mut pc: u32 = 0x8241FE30;
    'dispatch: loop {
        match pc {
            0x8241FE30 => {
    //   block [0x8241FE30..0x8241FEBC)
	// 8241FE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241FE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FE3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8241FE40: 409A003C  bne cr6, 0x8241fe7c
	if !ctx.cr[6].eq {
	pc = 0x8241FE7C; continue 'dispatch;
	}
	// 8241FE44: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FE48: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8241FE4C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FE50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241FE54: 419A0020  beq cr6, 0x8241fe74
	if ctx.cr[6].eq {
	pc = 0x8241FE74; continue 'dispatch;
	}
	// 8241FE58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8241FE5C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FE60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FE64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FE68: 388A12A8  addi r4, r10, 0x12a8
	ctx.r[4].s64 = ctx.r[10].s64 + 4776;
	// 8241FE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FE70: 4E800421  bctrl
	ctx.lr = 0x8241FE74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FE74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241FE78: 48000034  b 0x8241feac
	pc = 0x8241FEAC; continue 'dispatch;
	// 8241FE7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FE80: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241FE84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FE88: 41820020  beq 0x8241fea8
	if ctx.cr[0].eq {
	pc = 0x8241FEA8; continue 'dispatch;
	}
	// 8241FE8C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241FE90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8241FE94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241FE98: 38800190  li r4, 0x190
	ctx.r[4].s64 = 400;
	// 8241FE9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241FEA0: 4E800421  bctrl
	ctx.lr = 0x8241FEA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FEA4: 48000008  b 0x8241feac
	pc = 0x8241FEAC; continue 'dispatch;
	// 8241FEA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241FEAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241FEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FEC0 size=300
    let mut pc: u32 = 0x8241FEC0;
    'dispatch: loop {
        match pc {
            0x8241FEC0 => {
    //   block [0x8241FEC0..0x8241FFEC)
	// 8241FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8241FEC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241FECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241FED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241FED8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8241FEDC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8241FEE0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FEE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241FEE8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8241FEEC: 409AFFF4  bne cr6, 0x8241fee0
	if !ctx.cr[6].eq {
	pc = 0x8241FEE0; continue 'dispatch;
	}
	// 8241FEF0: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8241FEF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241FEF8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8241FEFC: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241FF00: 352A0001  addic. r9, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8241FF04: 41820030  beq 0x8241ff34
	if ctx.cr[0].eq {
	pc = 0x8241FF34; continue 'dispatch;
	}
	// 8241FF08: 7D4BF8AE  lbzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8241FF0C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8241FF10: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 8241FF14: 41980014  blt cr6, 0x8241ff28
	if ctx.cr[6].lt {
	pc = 0x8241FF28; continue 'dispatch;
	}
	// 8241FF18: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 8241FF1C: 4199000C  bgt cr6, 0x8241ff28
	if ctx.cr[6].gt {
	pc = 0x8241FF28; continue 'dispatch;
	}
	// 8241FF20: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8241FF24: 7D4BF9AE  stbx r10, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u8) };
	// 8241FF28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241FF2C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8241FF30: 4198FFD8  blt cr6, 0x8241ff08
	if ctx.cr[6].lt {
	pc = 0x8241FF08; continue 'dispatch;
	}
	// 8241FF34: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 8241FF38: 4E800421  bctrl
	ctx.lr = 0x8241FF3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241FF3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241FF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241FF44: 4BFFF605  bl 0x8241f548
	ctx.lr = 0x8241FF48;
	sub_8241F548(ctx, base);
	// 8241FF48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241FF4C: 40820084  bne 0x8241ffd0
	if !ctx.cr[0].eq {
	pc = 0x8241FFD0; continue 'dispatch;
	}
	// 8241FF50: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8241FF54: 394B8B90  addi r10, r11, -0x7470
	ctx.r[10].s64 = ctx.r[11].s64 + -29808;
	// 8241FF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241FF5C: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 8241FF60: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FF64: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8241FF68: 419A0018  beq cr6, 0x8241ff80
	if ctx.cr[6].eq {
	pc = 0x8241FF80; continue 'dispatch;
	}
	// 8241FF6C: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 8241FF70: 390A0204  addi r8, r10, 0x204
	ctx.r[8].s64 = ctx.r[10].s64 + 516;
	// 8241FF74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241FF78: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8241FF7C: 4198FFE4  blt cr6, 0x8241ff60
	if ctx.cr[6].lt {
	pc = 0x8241FF60; continue 'dispatch;
	}
	// 8241FF80: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8241FF84: 409A000C  bne cr6, 0x8241ff90
	if !ctx.cr[6].eq {
	pc = 0x8241FF90; continue 'dispatch;
	}
	// 8241FF88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241FF8C: 48000048  b 0x8241ffd4
	pc = 0x8241FFD4; continue 'dispatch;
	// 8241FF90: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8241FF94: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8241FF98: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8241FF9C: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 8241FFA0: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241FFA4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241FFA8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8241FFAC: 409AFFF4  bne cr6, 0x8241ffa0
	if !ctx.cr[6].eq {
	pc = 0x8241FFA0; continue 'dispatch;
	}
	// 8241FFB0: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8241FFB4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8241FFB8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8241FFBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241FFC0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241FFC4: 7C695214  add r3, r9, r10
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8241FFC8: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 8241FFCC: 48114B85  bl 0x82534b50
	ctx.lr = 0x8241FFD0;
	sub_82534B50(ctx, base);
	// 8241FFD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241FFD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241FFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241FFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241FFE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241FFE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241FFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241FFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8241FFF0 size=348
    let mut pc: u32 = 0x8241FFF0;
    'dispatch: loop {
        match pc {
            0x8241FFF0 => {
    //   block [0x8241FFF0..0x8242014C)
	// 8241FFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241FFF4: 481150C5  bl 0x825350b8
	ctx.lr = 0x8241FFF8;
	sub_82535080(ctx, base);
	// 8241FFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241FFFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82420000: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82420004: 409A0030  bne cr6, 0x82420034
	if !ctx.cr[6].eq {
	pc = 0x82420034; continue 'dispatch;
	}
	// 82420008: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242000C: 396B87D0  addi r11, r11, -0x7830
	ctx.r[11].s64 = ctx.r[11].s64 + -30768;
	// 82420010: 814B05C0  lwz r10, 0x5c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1472 as u32) ) } as u64;
	// 82420014: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82420018: 419A0118  beq cr6, 0x82420130
	if ctx.cr[6].eq {
	pc = 0x82420130; continue 'dispatch;
	}
	// 8242001C: 392B05C0  addi r9, r11, 0x5c0
	ctx.r[9].s64 = ctx.r[11].s64 + 1472;
	// 82420020: 806B05C4  lwz r3, 0x5c4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1476 as u32) ) } as u64;
	// 82420024: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82420028: 388A12FC  addi r4, r10, 0x12fc
	ctx.r[4].s64 = ctx.r[10].s64 + 4860;
	// 8242002C: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420030: 480000F4  b 0x82420124
	pc = 0x82420124; continue 'dispatch;
	// 82420034: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82420038: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242003C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420040: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82420044: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82420048: 409AFFF4  bne cr6, 0x8242003c
	if !ctx.cr[6].eq {
	pc = 0x8242003C; continue 'dispatch;
	}
	// 8242004C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82420050: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82420054: 557C003F  rotlwi. r28, r11, 0
	ctx.r[28].u64 = ((ctx.r[11].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82420058: 40820014  bne 0x8242006c
	if !ctx.cr[0].eq {
	pc = 0x8242006C; continue 'dispatch;
	}
	// 8242005C: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82420064: 996A87D0  stb r11, -0x7830(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-30768 as u32), ctx.r[11].u8 ) };
	// 82420068: 480000C8  b 0x82420130
	pc = 0x82420130; continue 'dispatch;
	// 8242006C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82420070: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82420074: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420078: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242007C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82420080: 409AFFF4  bne cr6, 0x82420074
	if !ctx.cr[6].eq {
	pc = 0x82420074; continue 'dispatch;
	}
	// 82420084: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82420088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242008C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82420090: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82420094: 352A0001  addic. r9, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82420098: 41820030  beq 0x824200c8
	if ctx.cr[0].eq {
	pc = 0x824200C8; continue 'dispatch;
	}
	// 8242009C: 7D4BE8AE  lbzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824200A0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824200A4: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 824200A8: 41980014  blt cr6, 0x824200bc
	if ctx.cr[6].lt {
	pc = 0x824200BC; continue 'dispatch;
	}
	// 824200AC: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 824200B0: 4199000C  bgt cr6, 0x824200bc
	if ctx.cr[6].gt {
	pc = 0x824200BC; continue 'dispatch;
	}
	// 824200B4: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 824200B8: 7D4BE9AE  stbx r10, r11, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u8) };
	// 824200BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824200C0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824200C4: 4198FFD8  blt cr6, 0x8242009c
	if ctx.cr[6].lt {
	pc = 0x8242009C; continue 'dispatch;
	}
	// 824200C8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824200CC: 3BEB87D0  addi r31, r11, -0x7830
	ctx.r[31].s64 = ctx.r[11].s64 + -30768;
	// 824200D0: 397F03C0  addi r11, r31, 0x3c0
	ctx.r[11].s64 = ctx.r[31].s64 + 960;
	// 824200D4: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 824200D8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824200DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824200E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824200E4: 481130AD  bl 0x82533190
	ctx.lr = 0x824200E8;
	sub_82533190(ctx, base);
	// 824200E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824200EC: 4182004C  beq 0x82420138
	if ctx.cr[0].eq {
	pc = 0x82420138; continue 'dispatch;
	}
	// 824200F0: 397F03C0  addi r11, r31, 0x3c0
	ctx.r[11].s64 = ctx.r[31].s64 + 960;
	// 824200F4: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 824200F8: 396B0204  addi r11, r11, 0x204
	ctx.r[11].s64 = ctx.r[11].s64 + 516;
	// 824200FC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82420100: 4198FFD8  blt cr6, 0x824200d8
	if ctx.cr[6].lt {
	pc = 0x824200D8; continue 'dispatch;
	}
	// 82420104: 817F05C0  lwz r11, 0x5c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1472 as u32) ) } as u64;
	// 82420108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242010C: 419A0024  beq cr6, 0x82420130
	if ctx.cr[6].eq {
	pc = 0x82420130; continue 'dispatch;
	}
	// 82420110: 395F05C0  addi r10, r31, 0x5c0
	ctx.r[10].s64 = ctx.r[31].s64 + 1472;
	// 82420114: 807F05C4  lwz r3, 0x5c4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1476 as u32) ) } as u64;
	// 82420118: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242011C: 388B12D4  addi r4, r11, 0x12d4
	ctx.r[4].s64 = ctx.r[11].s64 + 4820;
	// 82420120: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242012C: 4E800421  bctrl
	ctx.lr = 0x82420130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82420134: 48114FD4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82420138: 38BC0001  addi r5, r28, 1
	ctx.r[5].s64 = ctx.r[28].s64 + 1;
	// 8242013C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82420140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82420144: 48114A0D  bl 0x82534b50
	ctx.lr = 0x82420148;
	sub_82534B50(ctx, base);
	// 82420148: 4BFFFFE8  b 0x82420130
	pc = 0x82420130; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420150 size=284
    let mut pc: u32 = 0x82420150;
    'dispatch: loop {
        match pc {
            0x82420150 => {
    //   block [0x82420150..0x8242026C)
	// 82420150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420154: 48114F5D  bl 0x825350b0
	ctx.lr = 0x82420158;
	sub_82535080(ctx, base);
	// 82420158: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242015C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82420160: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82420164: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82420168: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8242016C: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420170: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420174: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420178: 3BEB87D0  addi r31, r11, -0x7830
	ctx.r[31].s64 = ctx.r[11].s64 + -30768;
	// 8242017C: 409A0060  bne cr6, 0x824201dc
	if !ctx.cr[6].eq {
	pc = 0x824201DC; continue 'dispatch;
	}
	// 82420180: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82420184: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82420188: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242018C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82420190: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82420194: 409AFFF4  bne cr6, 0x82420188
	if !ctx.cr[6].eq {
	pc = 0x82420188; continue 'dispatch;
	}
	// 82420198: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242019C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824201A0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824201A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824201A8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824201AC: 4082000C  bne 0x824201b8
	if !ctx.cr[0].eq {
	pc = 0x824201B8; continue 'dispatch;
	}
	// 824201B0: 9B7E0000  stb r27, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 824201B4: 48000014  b 0x824201c8
	pc = 0x824201C8; continue 'dispatch;
	// 824201B8: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 824201BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824201C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824201C4: 4811498D  bl 0x82534b50
	ctx.lr = 0x824201C8;
	sub_82534B50(ctx, base);
	// 824201C8: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824201CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824201D0: 409A000C  bne cr6, 0x824201dc
	if !ctx.cr[6].eq {
	pc = 0x824201DC; continue 'dispatch;
	}
	// 824201D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824201D8: 4800008C  b 0x82420264
	pc = 0x82420264; continue 'dispatch;
	// 824201DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824201E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824201E4: 4BFFFAE5  bl 0x8241fcc8
	ctx.lr = 0x824201E8;
	sub_8241FCC8(ctx, base);
	// 824201E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824201EC: 4BFFF35D  bl 0x8241f548
	ctx.lr = 0x824201F0;
	sub_8241F548(ctx, base);
	// 824201F0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824201F4: 4082006C  bne 0x82420260
	if !ctx.cr[0].eq {
	pc = 0x82420260; continue 'dispatch;
	}
	// 824201F8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824201FC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82420200: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420204: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82420208: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242020C: 409AFFF4  bne cr6, 0x82420200
	if !ctx.cr[6].eq {
	pc = 0x82420200; continue 'dispatch;
	}
	// 82420210: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82420214: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420218: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242021C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82420220: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82420224: 4082000C  bne 0x82420230
	if !ctx.cr[0].eq {
	pc = 0x82420230; continue 'dispatch;
	}
	// 82420228: 9B7E0000  stb r27, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8242022C: 48000014  b 0x82420240
	pc = 0x82420240; continue 'dispatch;
	// 82420230: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82420234: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82420238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242023C: 48114915  bl 0x82534b50
	ctx.lr = 0x82420240;
	sub_82534B50(ctx, base);
	// 82420240: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82420244: 4BFFF305  bl 0x8241f548
	ctx.lr = 0x82420248;
	sub_8241F548(ctx, base);
	// 82420248: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8242024C: 4182FF88  beq 0x824201d4
	if ctx.cr[0].eq {
	pc = 0x824201D4; continue 'dispatch;
	}
	// 82420250: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82420254: 38800129  li r4, 0x129
	ctx.r[4].s64 = 297;
	// 82420258: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8242025C: 480032CD  bl 0x82423528
	ctx.lr = 0x82420260;
	sub_82423528(ctx, base);
	// 82420260: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82420264: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82420268: 48114E98  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420270 size=408
    let mut pc: u32 = 0x82420270;
    'dispatch: loop {
        match pc {
            0x82420270 => {
    //   block [0x82420270..0x82420408)
	// 82420270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420274: 48114E3D  bl 0x825350b0
	ctx.lr = 0x82420278;
	sub_82535080(ctx, base);
	// 82420278: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242027C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82420280: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82420284: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82420288: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8242028C: 409A0030  bne cr6, 0x824202bc
	if !ctx.cr[6].eq {
	pc = 0x824202BC; continue 'dispatch;
	}
	// 82420290: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420294: 396B87E0  addi r11, r11, -0x7820
	ctx.r[11].s64 = ctx.r[11].s64 + -30752;
	// 82420298: 814B05B0  lwz r10, 0x5b0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 8242029C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824202A0: 419A015C  beq cr6, 0x824203fc
	if ctx.cr[6].eq {
	pc = 0x824203FC; continue 'dispatch;
	}
	// 824202A4: 392B05B0  addi r9, r11, 0x5b0
	ctx.r[9].s64 = ctx.r[11].s64 + 1456;
	// 824202A8: 806B05B4  lwz r3, 0x5b4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1460 as u32) ) } as u64;
	// 824202AC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824202B0: 388A13A0  addi r4, r10, 0x13a0
	ctx.r[4].s64 = ctx.r[10].s64 + 5024;
	// 824202B4: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824202B8: 48000138  b 0x824203f0
	pc = 0x824203F0; continue 'dispatch;
	// 824202BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824202C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824202C4: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 824202C8: 4BFFF309  bl 0x8241f5d0
	ctx.lr = 0x824202CC;
	sub_8241F5D0(ctx, base);
	// 824202CC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824202D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824202D4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824202D8: 4182FFBC  beq 0x82420294
	if ctx.cr[0].eq {
	pc = 0x82420294; continue 'dispatch;
	}
	// 824202DC: 3BEB87E0  addi r31, r11, -0x7820
	ctx.r[31].s64 = ctx.r[11].s64 + -30752;
	// 824202E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824202E4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 824202E8: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 824202EC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824202F0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824202F4: 419A0018  beq cr6, 0x8242030c
	if ctx.cr[6].eq {
	pc = 0x8242030C; continue 'dispatch;
	}
	// 824202F8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824202FC: 393F0284  addi r9, r31, 0x284
	ctx.r[9].s64 = ctx.r[31].s64 + 644;
	// 82420300: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82420304: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82420308: 4198FFE4  blt cr6, 0x824202ec
	if ctx.cr[6].lt {
	pc = 0x824202EC; continue 'dispatch;
	}
	// 8242030C: 2F0A0050  cmpwi cr6, r10, 0x50
	ctx.cr[6].compare_i32(ctx.r[10].s32, 80, &mut ctx.xer);
	// 82420310: 419A0010  beq cr6, 0x82420320
	if ctx.cr[6].eq {
	pc = 0x82420320; continue 'dispatch;
	}
	// 82420314: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82420318: 7FCBFA15  add. r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8242031C: 4082001C  bne 0x82420338
	if !ctx.cr[0].eq {
	pc = 0x82420338; continue 'dispatch;
	}
	// 82420320: 817F05B0  lwz r11, 0x5b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82420324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420328: 419A00D4  beq cr6, 0x824203fc
	if ctx.cr[6].eq {
	pc = 0x824203FC; continue 'dispatch;
	}
	// 8242032C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420330: 388B137C  addi r4, r11, 0x137c
	ctx.r[4].s64 = ctx.r[11].s64 + 4988;
	// 82420334: 480000B0  b 0x824203e4
	pc = 0x824203E4; continue 'dispatch;
	// 82420338: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8242033C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82420340: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 82420344: 4BFFFE0D  bl 0x82420150
	ctx.lr = 0x82420348;
	sub_82420150(ctx, base);
	// 82420348: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242034C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82420350: 40820024  bne 0x82420374
	if !ctx.cr[0].eq {
	pc = 0x82420374; continue 'dispatch;
	}
	// 82420354: 817F05B0  lwz r11, 0x5b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82420358: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8242035C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420360: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82420364: 419A0098  beq cr6, 0x824203fc
	if ctx.cr[6].eq {
	pc = 0x824203FC; continue 'dispatch;
	}
	// 82420368: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242036C: 388B135C  addi r4, r11, 0x135c
	ctx.r[4].s64 = ctx.r[11].s64 + 4956;
	// 82420370: 48000074  b 0x824203e4
	pc = 0x824203E4; continue 'dispatch;
	// 82420374: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82420378: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242037C: 4182004C  beq 0x824203c8
	if ctx.cr[0].eq {
	pc = 0x824203C8; continue 'dispatch;
	}
	// 82420380: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82420384: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82420388: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8242038C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420390: 4E800421  bctrl
	ctx.lr = 0x82420394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420394: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82420398: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8242039C: 40820024  bne 0x824203c0
	if !ctx.cr[0].eq {
	pc = 0x824203C0; continue 'dispatch;
	}
	// 824203A0: 817F05B0  lwz r11, 0x5b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 824203A4: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824203A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824203AC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824203B0: 419A004C  beq cr6, 0x824203fc
	if ctx.cr[6].eq {
	pc = 0x824203FC; continue 'dispatch;
	}
	// 824203B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824203B8: 388B133C  addi r4, r11, 0x133c
	ctx.r[4].s64 = ctx.r[11].s64 + 4924;
	// 824203BC: 48000028  b 0x824203e4
	pc = 0x824203E4; continue 'dispatch;
	// 824203C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824203C4: 4800003C  b 0x82420400
	pc = 0x82420400; continue 'dispatch;
	// 824203C8: 817F05B0  lwz r11, 0x5b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 824203CC: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824203D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824203D4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824203D8: 419A0024  beq cr6, 0x824203fc
	if ctx.cr[6].eq {
	pc = 0x824203FC; continue 'dispatch;
	}
	// 824203DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824203E0: 388B1324  addi r4, r11, 0x1324
	ctx.r[4].s64 = ctx.r[11].s64 + 4900;
	// 824203E4: 395F05B0  addi r10, r31, 0x5b0
	ctx.r[10].s64 = ctx.r[31].s64 + 1456;
	// 824203E8: 807F05B4  lwz r3, 0x5b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1460 as u32) ) } as u64;
	// 824203EC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824203F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824203F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824203F8: 4E800421  bctrl
	ctx.lr = 0x824203FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824203FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420400: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82420404: 48114CFC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420408 size=268
    let mut pc: u32 = 0x82420408;
    'dispatch: loop {
        match pc {
            0x82420408 => {
    //   block [0x82420408..0x82420514)
	// 82420408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242040C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82420414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82420418: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242041C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82420420: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82420424: 409A002C  bne cr6, 0x82420450
	if !ctx.cr[6].eq {
	pc = 0x82420450; continue 'dispatch;
	}
	// 82420428: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242042C: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 82420430: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420434: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82420438: 419A00C0  beq cr6, 0x824204f8
	if ctx.cr[6].eq {
	pc = 0x824204F8; continue 'dispatch;
	}
	// 8242043C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82420440: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82420444: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420448: 388A1404  addi r4, r10, 0x1404
	ctx.r[4].s64 = ctx.r[10].s64 + 5124;
	// 8242044C: 480000A0  b 0x824204ec
	pc = 0x824204EC; continue 'dispatch;
	// 82420450: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82420454: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82420458: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 8242045C: 4BFFF175  bl 0x8241f5d0
	ctx.lr = 0x82420460;
	sub_8241F5D0(ctx, base);
	// 82420460: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82420464: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82420468: 4182FFC0  beq 0x82420428
	if ctx.cr[0].eq {
	pc = 0x82420428; continue 'dispatch;
	}
	// 8242046C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82420470: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82420474: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 82420478: 4BFFFCD9  bl 0x82420150
	ctx.lr = 0x8242047C;
	sub_82420150(ctx, base);
	// 8242047C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420480: 3BEB8D90  addi r31, r11, -0x7270
	ctx.r[31].s64 = ctx.r[11].s64 + -29296;
	// 82420484: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82420488: 4082002C  bne 0x824204b4
	if !ctx.cr[0].eq {
	pc = 0x824204B4; continue 'dispatch;
	}
	// 8242048C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420494: 419A0020  beq cr6, 0x824204b4
	if ctx.cr[6].eq {
	pc = 0x824204B4; continue 'dispatch;
	}
	// 82420498: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242049C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824204A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824204A4: 388B13E0  addi r4, r11, 0x13e0
	ctx.r[4].s64 = ctx.r[11].s64 + 5088;
	// 824204A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824204AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824204B0: 4E800421  bctrl
	ctx.lr = 0x824204B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824204B4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824204B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824204BC: 41820014  beq 0x824204d0
	if ctx.cr[0].eq {
	pc = 0x824204D0; continue 'dispatch;
	}
	// 824204C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824204C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824204C8: 4E800421  bctrl
	ctx.lr = 0x824204CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824204CC: 48000030  b 0x824204fc
	pc = 0x824204FC; continue 'dispatch;
	// 824204D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824204D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824204D8: 419A0020  beq cr6, 0x824204f8
	if ctx.cr[6].eq {
	pc = 0x824204F8; continue 'dispatch;
	}
	// 824204DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824204E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824204E4: 388B13C0  addi r4, r11, 0x13c0
	ctx.r[4].s64 = ctx.r[11].s64 + 5056;
	// 824204E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824204EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824204F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824204F4: 4E800421  bctrl
	ctx.lr = 0x824204F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824204F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824204FC: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82420500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420508: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242050C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82420510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420518 size=204
    let mut pc: u32 = 0x82420518;
    'dispatch: loop {
        match pc {
            0x82420518 => {
    //   block [0x82420518..0x824205E4)
	// 82420518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242051C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420524: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420528: 409A0038  bne cr6, 0x82420560
	if !ctx.cr[6].eq {
	pc = 0x82420560; continue 'dispatch;
	}
	// 8242052C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420530: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 82420534: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420538: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8242053C: 419A0098  beq cr6, 0x824205d4
	if ctx.cr[6].eq {
	pc = 0x824205D4; continue 'dispatch;
	}
	// 82420540: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82420544: 388A1474  addi r4, r10, 0x1474
	ctx.r[4].s64 = ctx.r[10].s64 + 5236;
	// 82420548: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242054C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420550: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420558: 4E800421  bctrl
	ctx.lr = 0x8242055C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242055C: 48000078  b 0x824205d4
	pc = 0x824205D4; continue 'dispatch;
	// 82420560: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82420564: 409A0024  bne cr6, 0x82420588
	if !ctx.cr[6].eq {
	pc = 0x82420588; continue 'dispatch;
	}
	// 82420568: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242056C: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 82420570: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420574: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82420578: 419A005C  beq cr6, 0x824205d4
	if ctx.cr[6].eq {
	pc = 0x824205D4; continue 'dispatch;
	}
	// 8242057C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82420580: 388A1450  addi r4, r10, 0x1450
	ctx.r[4].s64 = ctx.r[10].s64 + 5200;
	// 82420584: 4BFFFFC4  b 0x82420548
	pc = 0x82420548; continue 'dispatch;
	// 82420588: 4BFFF939  bl 0x8241fec0
	ctx.lr = 0x8242058C;
	sub_8241FEC0(ctx, base);
	// 8242058C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82420590: 40820024  bne 0x824205b4
	if !ctx.cr[0].eq {
	pc = 0x824205B4; continue 'dispatch;
	}
	// 82420594: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420598: 396B8D90  addi r11, r11, -0x7270
	ctx.r[11].s64 = ctx.r[11].s64 + -29296;
	// 8242059C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824205A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824205A4: 419A0030  beq cr6, 0x824205d4
	if ctx.cr[6].eq {
	pc = 0x824205D4; continue 'dispatch;
	}
	// 824205A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824205AC: 388A142C  addi r4, r10, 0x142c
	ctx.r[4].s64 = ctx.r[10].s64 + 5164;
	// 824205B0: 4BFFFF98  b 0x82420548
	pc = 0x82420548; continue 'dispatch;
	// 824205B4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824205B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824205BC: 41820018  beq 0x824205d4
	if ctx.cr[0].eq {
	pc = 0x824205D4; continue 'dispatch;
	}
	// 824205C0: 3D408242  lis r10, -0x7dbe
	ctx.r[10].s64 = -2109603840;
	// 824205C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824205C8: 386AF3F0  addi r3, r10, -0xc10
	ctx.r[3].s64 = ctx.r[10].s64 + -3088;
	// 824205CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824205D0: 4E800421  bctrl
	ctx.lr = 0x824205D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824205D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824205D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824205DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824205E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824205E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824205E8 size=20
    let mut pc: u32 = 0x824205E8;
    'dispatch: loop {
        match pc {
            0x824205E8 => {
    //   block [0x824205E8..0x824205FC)
	// 824205E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824205EC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 824205F0: 386B3C20  addi r3, r11, 0x3c20
	ctx.r[3].s64 = ctx.r[11].s64 + 15392;
	// 824205F4: 816A14CC  lwz r11, 0x14cc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5324 as u32) ) } as u64;
	// 824205F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420600 size=40
    let mut pc: u32 = 0x82420600;
    'dispatch: loop {
        match pc {
            0x82420600 => {
    //   block [0x82420600..0x82420628)
	// 82420600: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82420604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420608: 89690000  lbz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242060C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82420610: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 82420614: 41980014  blt cr6, 0x82420628
	if ctx.cr[6].lt {
		sub_82420628(ctx, base);
		return;
	}
	// 82420618: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8242061C: 4199000C  bgt cr6, 0x82420628
	if ctx.cr[6].gt {
		sub_82420628(ctx, base);
		return;
	}
	// 82420620: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 82420624: 48000030  b 0x82420654
	sub_82420640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420628 size=24
    let mut pc: u32 = 0x82420628;
    'dispatch: loop {
        match pc {
            0x82420628 => {
    //   block [0x82420628..0x82420640)
	// 82420628: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 8242062C: 41980014  blt cr6, 0x82420640
	if ctx.cr[6].lt {
		sub_82420640(ctx, base);
		return;
	}
	// 82420630: 2F0B0066  cmpwi cr6, r11, 0x66
	ctx.cr[6].compare_i32(ctx.r[11].s32, 102, &mut ctx.xer);
	// 82420634: 4199000C  bgt cr6, 0x82420640
	if ctx.cr[6].gt {
		sub_82420640(ctx, base);
		return;
	}
	// 82420638: 396BFFA9  addi r11, r11, -0x57
	ctx.r[11].s64 = ctx.r[11].s64 + -87;
	// 8242063C: 48000018  b 0x82420654
	sub_82420640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420640 size=36
    let mut pc: u32 = 0x82420640;
    'dispatch: loop {
        match pc {
            0x82420640 => {
    //   block [0x82420640..0x82420664)
	// 82420640: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82420644: 41980020  blt cr6, 0x82420664
	if ctx.cr[6].lt {
		sub_82420664(ctx, base);
		return;
	}
	// 82420648: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 8242064C: 41990018  bgt cr6, 0x82420664
	if ctx.cr[6].gt {
		sub_82420664(ctx, base);
		return;
	}
	// 82420650: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 82420654: 7D4329D6  mullw r10, r3, r5
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82420658: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8242065C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82420660: 4BFFFFA8  b 0x82420608
	sub_82420600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420664 size=8
    let mut pc: u32 = 0x82420664;
    'dispatch: loop {
        match pc {
            0x82420664 => {
    //   block [0x82420664..0x8242066C)
	// 82420664: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82420668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420670 size=232
    let mut pc: u32 = 0x82420670;
    'dispatch: loop {
        match pc {
            0x82420670 => {
    //   block [0x82420670..0x82420758)
	// 82420670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420674: 48114A49  bl 0x825350bc
	ctx.lr = 0x82420678;
	sub_82535080(ctx, base);
	// 82420678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242067C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82420680: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82420684: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82420688: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242068C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420690: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82420694: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82420698: 409AFFF4  bne cr6, 0x8242068c
	if !ctx.cr[6].eq {
	pc = 0x8242068C; continue 'dispatch;
	}
	// 8242069C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824206A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824206A4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824206A8: 2B0B0012  cmplwi cr6, r11, 0x12
	ctx.cr[6].compare_u32(ctx.r[11].u32, 18 as u32, &mut ctx.xer);
	// 824206AC: 41980054  blt cr6, 0x82420700
	if ctx.cr[6].lt {
	pc = 0x82420700; continue 'dispatch;
	}
	// 824206B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824206B4: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 824206B8: 38AB14D0  addi r5, r11, 0x14d0
	ctx.r[5].s64 = ctx.r[11].s64 + 5328;
	// 824206BC: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824206C0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824206C4: 3BEBCCC0  addi r31, r11, -0x3340
	ctx.r[31].s64 = ctx.r[11].s64 + -13120;
	// 824206C8: 3880012C  li r4, 0x12c
	ctx.r[4].s64 = 300;
	// 824206CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824206D0: 48002F19  bl 0x824235e8
	ctx.lr = 0x824206D4;
	sub_824235E8(ctx, base);
	// 824206D4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824206D8: 814B8D9C  lwz r10, -0x7264(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 824206DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824206E0: 419A0020  beq cr6, 0x82420700
	if ctx.cr[6].eq {
	pc = 0x82420700; continue 'dispatch;
	}
	// 824206E4: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824206E8: 816B8D9C  lwz r11, -0x7264(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 824206EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824206F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824206F4: 806A8DA0  lwz r3, -0x7260(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 824206F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824206FC: 4E800421  bctrl
	ctx.lr = 0x82420700;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420700: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82420704: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82420708: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242070C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82420710: 4BFFFEF1  bl 0x82420600
	ctx.lr = 0x82420714;
	sub_82420600(ctx, base);
	// 82420714: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82420718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242071C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420720: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82420724: 419A000C  beq cr6, 0x82420730
	if ctx.cr[6].eq {
	pc = 0x82420730; continue 'dispatch;
	}
	// 82420728: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242072C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82420730: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82420734: 419A0018  beq cr6, 0x8242074c
	if ctx.cr[6].eq {
	pc = 0x8242074C; continue 'dispatch;
	}
	// 82420738: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8242073C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82420740: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82420744: 4811396D  bl 0x825340b0
	ctx.lr = 0x82420748;
	sub_825340B0(ctx, base);
	// 82420748: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8242074C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82420750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82420754: 481149B8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420758 size=20
    let mut pc: u32 = 0x82420758;
    'dispatch: loop {
        match pc {
            0x82420758 => {
    //   block [0x82420758..0x8242076C)
	// 82420758: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242075C: 906B8D9C  stw r3, -0x7264(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-29284 as u32), ctx.r[3].u32 ) };
	// 82420760: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420764: 908B8DA0  stw r4, -0x7260(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-29280 as u32), ctx.r[4].u32 ) };
	// 82420768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420770 size=40
    let mut pc: u32 = 0x82420770;
    'dispatch: loop {
        match pc {
            0x82420770 => {
    //   block [0x82420770..0x82420798)
	// 82420770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242077C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82420780: 4BFFFEF1  bl 0x82420670
	ctx.lr = 0x82420784;
	sub_82420670(ctx, base);
	// 82420784: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82420788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420798 size=56
    let mut pc: u32 = 0x82420798;
    'dispatch: loop {
        match pc {
            0x82420798 => {
    //   block [0x82420798..0x824207D0)
	// 82420798: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8242079C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824207A0: 396BCE00  addi r11, r11, -0x3200
	ctx.r[11].s64 = ctx.r[11].s64 + -12800;
	// 824207A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824207A8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824207AC: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824207B0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824207B4: 419A001C  beq cr6, 0x824207d0
	if ctx.cr[6].eq {
		sub_824207D0(ctx, base);
		return;
	}
	// 824207B8: 394A0038  addi r10, r10, 0x38
	ctx.r[10].s64 = ctx.r[10].s64 + 56;
	// 824207BC: 390B1180  addi r8, r11, 0x1180
	ctx.r[8].s64 = ctx.r[11].s64 + 4480;
	// 824207C0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824207C4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824207C8: 4198FFE4  blt cr6, 0x824207ac
	if ctx.cr[6].lt {
	pc = 0x824207AC; continue 'dispatch;
	}
	// 824207CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824207D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824207D0 size=12
    let mut pc: u32 = 0x824207D0;
    'dispatch: loop {
        match pc {
            0x824207D0 => {
    //   block [0x824207D0..0x824207DC)
	// 824207D0: 1D490038  mulli r10, r9, 0x38
	ctx.r[10].s64 = ctx.r[9].s64 * 56;
	// 824207D4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824207D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824207E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824207E0 size=116
    let mut pc: u32 = 0x824207E0;
    'dispatch: loop {
        match pc {
            0x824207E0 => {
    //   block [0x824207E0..0x82420854)
	// 824207E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824207E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824207E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824207EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824207F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824207F4: 39600800  li r11, 0x800
	ctx.r[11].s64 = 2048;
	// 824207F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824207FC: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82420800: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82420804: 4BFFFE6D  bl 0x82420670
	ctx.lr = 0x82420808;
	sub_82420670(ctx, base);
	// 82420808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242080C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82420810: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82420814: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82420818: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8242081C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82420820: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82420824: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82420828: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8242082C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82420830: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82420834: 991F0000  stb r8, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82420838: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8242083C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82420840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82420844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242084C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82420850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420858 size=208
    let mut pc: u32 = 0x82420858;
    'dispatch: loop {
        match pc {
            0x82420858 => {
    //   block [0x82420858..0x82420928)
	// 82420858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242085C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82420864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420868: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8242086C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82420870: 409A003C  bne cr6, 0x824208ac
	if !ctx.cr[6].eq {
	pc = 0x824208AC; continue 'dispatch;
	}
	// 82420874: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420878: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 8242087C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420880: 419A0024  beq cr6, 0x824208a4
	if ctx.cr[6].eq {
	pc = 0x824208A4; continue 'dispatch;
	}
	// 82420884: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420888: 388B1564  addi r4, r11, 0x1564
	ctx.r[4].s64 = ctx.r[11].s64 + 5476;
	// 8242088C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420890: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420894: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420898: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 8242089C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824208A0: 4E800421  bctrl
	ctx.lr = 0x824208A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824208A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824208A8: 4800006C  b 0x82420914
	pc = 0x82420914; continue 'dispatch;
	// 824208AC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824208B0: 419A0020  beq cr6, 0x824208d0
	if ctx.cr[6].eq {
	pc = 0x824208D0; continue 'dispatch;
	}
	// 824208B4: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824208B8: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 824208BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824208C0: 419AFFE4  beq cr6, 0x824208a4
	if ctx.cr[6].eq {
	pc = 0x824208A4; continue 'dispatch;
	}
	// 824208C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824208C8: 388B1540  addi r4, r11, 0x1540
	ctx.r[4].s64 = ctx.r[11].s64 + 5440;
	// 824208CC: 4BFFFFC0  b 0x8242088c
	pc = 0x8242088C; continue 'dispatch;
	// 824208D0: 4BFFFEC9  bl 0x82420798
	ctx.lr = 0x824208D4;
	sub_82420798(ctx, base);
	// 824208D4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824208D8: 40820020  bne 0x824208f8
	if !ctx.cr[0].eq {
	pc = 0x824208F8; continue 'dispatch;
	}
	// 824208DC: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824208E0: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 824208E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824208E8: 419AFFBC  beq cr6, 0x824208a4
	if ctx.cr[6].eq {
	pc = 0x824208A4; continue 'dispatch;
	}
	// 824208EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824208F0: 388B1510  addi r4, r11, 0x1510
	ctx.r[4].s64 = ctx.r[11].s64 + 5392;
	// 824208F4: 4BFFFF98  b 0x8242088c
	pc = 0x8242088C; continue 'dispatch;
	// 824208F8: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 824208FC: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 82420900: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82420904: 48002C25  bl 0x82423528
	ctx.lr = 0x82420908;
	sub_82423528(ctx, base);
	// 82420908: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242090C: 4BFFFED5  bl 0x824207e0
	ctx.lr = 0x82420910;
	sub_824207E0(ctx, base);
	// 82420910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82420914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242091C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82420924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420928 size=204
    let mut pc: u32 = 0x82420928;
    'dispatch: loop {
        match pc {
            0x82420928 => {
    //   block [0x82420928..0x824209F4)
	// 82420928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242092C: 48114791  bl 0x825350bc
	ctx.lr = 0x82420930;
	sub_82535080(ctx, base);
	// 82420930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82420938: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242093C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82420940: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82420944: 409A003C  bne cr6, 0x82420980
	if !ctx.cr[6].eq {
	pc = 0x82420980; continue 'dispatch;
	}
	// 82420948: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242094C: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420950: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420954: 419A0024  beq cr6, 0x82420978
	if ctx.cr[6].eq {
	pc = 0x82420978; continue 'dispatch;
	}
	// 82420958: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242095C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420960: 388B1588  addi r4, r11, 0x1588
	ctx.r[4].s64 = ctx.r[11].s64 + 5512;
	// 82420964: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420968: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 8242096C: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420970: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420974: 4E800421  bctrl
	ctx.lr = 0x82420978;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242097C: 48000070  b 0x824209ec
	pc = 0x824209EC; continue 'dispatch;
	// 82420980: 48004DE1  bl 0x82425760
	ctx.lr = 0x82420984;
	sub_82425760(ctx, base);
	// 82420984: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82420988: 409A000C  bne cr6, 0x82420994
	if !ctx.cr[6].eq {
	pc = 0x82420994; continue 'dispatch;
	}
	// 8242098C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82420990: 4800002C  b 0x824209bc
	pc = 0x824209BC; continue 'dispatch;
	// 82420994: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82420998: 409A0010  bne cr6, 0x824209a8
	if !ctx.cr[6].eq {
	pc = 0x824209A8; continue 'dispatch;
	}
	// 8242099C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824209A0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824209A4: 48000014  b 0x824209b8
	pc = 0x824209B8; continue 'dispatch;
	// 824209A8: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 824209AC: 409A0010  bne cr6, 0x824209bc
	if !ctx.cr[6].eq {
	pc = 0x824209BC; continue 'dispatch;
	}
	// 824209B0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824209B4: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 824209B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824209BC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824209C0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824209C4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824209C8: 41980008  blt cr6, 0x824209d0
	if ctx.cr[6].lt {
	pc = 0x824209D0; continue 'dispatch;
	}
	// 824209CC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824209D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824209D4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824209D8: 41990008  bgt cr6, 0x824209e0
	if ctx.cr[6].gt {
	pc = 0x824209E0; continue 'dispatch;
	}
	// 824209DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824209E0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824209E4: 48005245  bl 0x82425c28
	ctx.lr = 0x824209E8;
	sub_82425C28(ctx, base);
	// 824209E8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824209EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824209F0: 4811471C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824209F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824209F8 size=96
    let mut pc: u32 = 0x824209F8;
    'dispatch: loop {
        match pc {
            0x824209F8 => {
    //   block [0x824209F8..0x82420A58)
	// 824209F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824209FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420A04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420A08: 409A003C  bne cr6, 0x82420a44
	if !ctx.cr[6].eq {
	pc = 0x82420A44; continue 'dispatch;
	}
	// 82420A0C: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420A10: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420A14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420A18: 419A0024  beq cr6, 0x82420a3c
	if ctx.cr[6].eq {
	pc = 0x82420A3C; continue 'dispatch;
	}
	// 82420A1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420A20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420A24: 388B15A4  addi r4, r11, 0x15a4
	ctx.r[4].s64 = ctx.r[11].s64 + 5540;
	// 82420A28: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420A2C: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420A30: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420A34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420A38: 4E800421  bctrl
	ctx.lr = 0x82420A3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420A3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420A40: 48000008  b 0x82420a48
	pc = 0x82420A48; continue 'dispatch;
	// 82420A44: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82420A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420A58 size=432
    let mut pc: u32 = 0x82420A58;
    'dispatch: loop {
        match pc {
            0x82420A58 => {
    //   block [0x82420A58..0x82420C08)
	// 82420A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420A5C: 4811465D  bl 0x825350b8
	ctx.lr = 0x82420A60;
	sub_82535080(ctx, base);
	// 82420A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420A64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82420A68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82420A6C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82420A70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82420A74: 409A003C  bne cr6, 0x82420ab0
	if !ctx.cr[6].eq {
	pc = 0x82420AB0; continue 'dispatch;
	}
	// 82420A78: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420A7C: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420A84: 419A0024  beq cr6, 0x82420aa8
	if ctx.cr[6].eq {
	pc = 0x82420AA8; continue 'dispatch;
	}
	// 82420A88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420A90: 388B1604  addi r4, r11, 0x1604
	ctx.r[4].s64 = ctx.r[11].s64 + 5636;
	// 82420A94: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420A98: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420A9C: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420AA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420AA4: 4E800421  bctrl
	ctx.lr = 0x82420AA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420AA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420AAC: 48000154  b 0x82420c00
	pc = 0x82420C00; continue 'dispatch;
	// 82420AB0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82420AB4: 40980024  bge cr6, 0x82420ad8
	if !ctx.cr[6].lt {
	pc = 0x82420AD8; continue 'dispatch;
	}
	// 82420AB8: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420ABC: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420AC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420AC4: 419AFFE4  beq cr6, 0x82420aa8
	if ctx.cr[6].eq {
	pc = 0x82420AA8; continue 'dispatch;
	}
	// 82420AC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420ACC: 388B15E4  addi r4, r11, 0x15e4
	ctx.r[4].s64 = ctx.r[11].s64 + 5604;
	// 82420AD0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82420AD4: 4BFFFFC0  b 0x82420a94
	pc = 0x82420A94; continue 'dispatch;
	// 82420AD8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82420ADC: 409A0020  bne cr6, 0x82420afc
	if !ctx.cr[6].eq {
	pc = 0x82420AFC; continue 'dispatch;
	}
	// 82420AE0: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420AE4: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420AE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420AEC: 419AFFBC  beq cr6, 0x82420aa8
	if ctx.cr[6].eq {
	pc = 0x82420AA8; continue 'dispatch;
	}
	// 82420AF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420AF4: 388B15C0  addi r4, r11, 0x15c0
	ctx.r[4].s64 = ctx.r[11].s64 + 5568;
	// 82420AF8: 4BFFFFD8  b 0x82420ad0
	pc = 0x82420AD0; continue 'dispatch;
	// 82420AFC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82420B00: 409A0014  bne cr6, 0x82420b14
	if !ctx.cr[6].eq {
	pc = 0x82420B14; continue 'dispatch;
	}
	// 82420B04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82420B08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420B0C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82420B10: 480000F0  b 0x82420c00
	pc = 0x82420C00; continue 'dispatch;
	// 82420B14: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82420B18: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82420B1C: 419AFF8C  beq cr6, 0x82420aa8
	if ctx.cr[6].eq {
	pc = 0x82420AA8; continue 'dispatch;
	}
	// 82420B20: 48004C41  bl 0x82425760
	ctx.lr = 0x82420B24;
	sub_82425760(ctx, base);
	// 82420B24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82420B28: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82420B2C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82420B30: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82420B34: 7D495850  subf r10, r9, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82420B38: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82420B3C: 40980008  bge cr6, 0x82420b44
	if !ctx.cr[6].lt {
	pc = 0x82420B44; continue 'dispatch;
	}
	// 82420B40: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82420B44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82420B48: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82420B4C: 7D4B51D7  mullw. r10, r11, r10
	ctx.r[10].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82420B50: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82420B54: 40820014  bne 0x82420b68
	if !ctx.cr[0].eq {
	pc = 0x82420B68; continue 'dispatch;
	}
	// 82420B58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82420B5C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82420B60: 480050C9  bl 0x82425c28
	ctx.lr = 0x82420B64;
	sub_82425C28(ctx, base);
	// 82420B64: 4BFFFF44  b 0x82420aa8
	pc = 0x82420AA8; continue 'dispatch;
	// 82420B68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82420B6C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82420B70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82420B74: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82420B78: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82420B7C: 993F0001  stb r9, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 82420B80: 4BFFFAF1  bl 0x82420670
	ctx.lr = 0x82420B84;
	sub_82420670(ctx, base);
	// 82420B84: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82420B88: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82420B8C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82420B90: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82420B94: 7FCA4850  subf r30, r10, r9
	ctx.r[30].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82420B98: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82420B9C: 41990008  bgt cr6, 0x82420ba4
	if ctx.cr[6].gt {
	pc = 0x82420BA4; continue 'dispatch;
	}
	// 82420BA0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82420BA4: 48005085  bl 0x82425c28
	ctx.lr = 0x82420BA8;
	sub_82425C28(ctx, base);
	// 82420BA8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82420BAC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82420BB0: 80BF0034  lwz r5, 0x34(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82420BB4: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82420BB8: 48113F99  bl 0x82534b50
	ctx.lr = 0x82420BBC;
	sub_82534B50(ctx, base);
	// 82420BBC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82420BC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82420BC4: 7CBE5850  subf r5, r30, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82420BC8: 7C7EE214  add r3, r30, r28
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82420BCC: 48114605  bl 0x825351d0
	ctx.lr = 0x82420BD0;
	sub_825351D0(ctx, base);
	// 82420BD0: 48004B91  bl 0x82425760
	ctx.lr = 0x82420BD4;
	sub_82425760(ctx, base);
	// 82420BD4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82420BD8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82420BDC: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82420BE0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82420BE4: 993F0001  stb r9, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 82420BE8: 7D2859D6  mullw r9, r8, r11
	ctx.r[9].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82420BEC: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82420BF0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82420BF4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82420BF8: 48005031  bl 0x82425c28
	ctx.lr = 0x82420BFC;
	sub_82425C28(ctx, base);
	// 82420BFC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82420C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82420C04: 48114504  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420C08 size=116
    let mut pc: u32 = 0x82420C08;
    'dispatch: loop {
        match pc {
            0x82420C08 => {
    //   block [0x82420C08..0x82420C7C)
	// 82420C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420C10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82420C14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420C18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82420C1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82420C20: 409A0038  bne cr6, 0x82420c58
	if !ctx.cr[6].eq {
	pc = 0x82420C58; continue 'dispatch;
	}
	// 82420C24: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420C28: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420C2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420C30: 419A0038  beq cr6, 0x82420c68
	if ctx.cr[6].eq {
	pc = 0x82420C68; continue 'dispatch;
	}
	// 82420C34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420C38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420C3C: 388B1620  addi r4, r11, 0x1620
	ctx.r[4].s64 = ctx.r[11].s64 + 5664;
	// 82420C40: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420C44: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420C48: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420C4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420C50: 4E800421  bctrl
	ctx.lr = 0x82420C54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420C54: 48000014  b 0x82420c68
	pc = 0x82420C68; continue 'dispatch;
	// 82420C58: 48004B09  bl 0x82425760
	ctx.lr = 0x82420C5C;
	sub_82425760(ctx, base);
	// 82420C5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82420C60: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82420C64: 48004FC5  bl 0x82425c28
	ctx.lr = 0x82420C68;
	sub_82425C28(ctx, base);
	// 82420C68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420C74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82420C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420C80 size=100
    let mut pc: u32 = 0x82420C80;
    'dispatch: loop {
        match pc {
            0x82420C80 => {
    //   block [0x82420C80..0x82420CE4)
	// 82420C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420C8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420C90: 409A003C  bne cr6, 0x82420ccc
	if !ctx.cr[6].eq {
	pc = 0x82420CCC; continue 'dispatch;
	}
	// 82420C94: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420C98: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420C9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420CA0: 419A0024  beq cr6, 0x82420cc4
	if ctx.cr[6].eq {
	pc = 0x82420CC4; continue 'dispatch;
	}
	// 82420CA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420CA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420CAC: 388B1620  addi r4, r11, 0x1620
	ctx.r[4].s64 = ctx.r[11].s64 + 5664;
	// 82420CB0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420CB4: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420CB8: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420CBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420CC0: 4E800421  bctrl
	ctx.lr = 0x82420CC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420CC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420CC8: 4800000C  b 0x82420cd4
	pc = 0x82420CD4; continue 'dispatch;
	// 82420CCC: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 82420CD0: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82420CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420CE8 size=96
    let mut pc: u32 = 0x82420CE8;
    'dispatch: loop {
        match pc {
            0x82420CE8 => {
    //   block [0x82420CE8..0x82420D48)
	// 82420CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420CF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420CF8: 409A003C  bne cr6, 0x82420d34
	if !ctx.cr[6].eq {
	pc = 0x82420D34; continue 'dispatch;
	}
	// 82420CFC: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420D00: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420D04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420D08: 419A0024  beq cr6, 0x82420d2c
	if ctx.cr[6].eq {
	pc = 0x82420D2C; continue 'dispatch;
	}
	// 82420D0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420D10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420D14: 388B1638  addi r4, r11, 0x1638
	ctx.r[4].s64 = ctx.r[11].s64 + 5688;
	// 82420D18: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420D1C: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420D20: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420D28: 4E800421  bctrl
	ctx.lr = 0x82420D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420D2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420D30: 48000008  b 0x82420d38
	pc = 0x82420D38; continue 'dispatch;
	// 82420D34: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82420D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420D48 size=24
    let mut pc: u32 = 0x82420D48;
    'dispatch: loop {
        match pc {
            0x82420D48 => {
    //   block [0x82420D48..0x82420D60)
	// 82420D48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420D4C: 409A0034  bne cr6, 0x82420d80
	if !ctx.cr[6].eq {
		sub_82420D80(ctx, base);
		return;
	}
	// 82420D50: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420D54: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420D58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420D5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420D60 size=32
    let mut pc: u32 = 0x82420D60;
    'dispatch: loop {
        match pc {
            0x82420D60 => {
    //   block [0x82420D60..0x82420D80)
	// 82420D60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420D64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420D68: 388B1650  addi r4, r11, 0x1650
	ctx.r[4].s64 = ctx.r[11].s64 + 5712;
	// 82420D6C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420D70: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420D74: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420D78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420D7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82420D80 size=60
    let mut pc: u32 = 0x82420D80;
    'dispatch: loop {
        match pc {
            0x82420D80 => {
    //   block [0x82420D80..0x82420DBC)
	// 82420D80: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82420D84: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82420D88: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82420D8C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82420D90: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82420D94: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82420D98: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82420D9C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82420DA0: 7D2921D6  mullw r9, r9, r4
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82420DA4: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82420DA8: 7D4A23D6  divw r10, r10, r4
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[4].s32;
	// 82420DAC: 7D6B23D6  divw r11, r11, r4
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[4].s32;
	// 82420DB0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82420DB4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82420DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420DC0 size=96
    let mut pc: u32 = 0x82420DC0;
    'dispatch: loop {
        match pc {
            0x82420DC0 => {
    //   block [0x82420DC0..0x82420E20)
	// 82420DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420DCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420DD0: 409A003C  bne cr6, 0x82420e0c
	if !ctx.cr[6].eq {
	pc = 0x82420E0C; continue 'dispatch;
	}
	// 82420DD4: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420DD8: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420DDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82420DE0: 419A0024  beq cr6, 0x82420e04
	if ctx.cr[6].eq {
	pc = 0x82420E04; continue 'dispatch;
	}
	// 82420DE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420DE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82420DEC: 388B1620  addi r4, r11, 0x1620
	ctx.r[4].s64 = ctx.r[11].s64 + 5664;
	// 82420DF0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420DF4: 806B8DA0  lwz r3, -0x7260(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29280 as u32) ) } as u64;
	// 82420DF8: 816A8D9C  lwz r11, -0x7264(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29284 as u32) ) } as u64;
	// 82420DFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82420E00: 4E800421  bctrl
	ctx.lr = 0x82420E04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82420E04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420E08: 48000008  b 0x82420e10
	pc = 0x82420E10; continue 'dispatch;
	// 82420E0C: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82420E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420E20 size=188
    let mut pc: u32 = 0x82420E20;
    'dispatch: loop {
        match pc {
            0x82420E20 => {
    //   block [0x82420E20..0x82420EDC)
	// 82420E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420E2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420E30: 409A000C  bne cr6, 0x82420e3c
	if !ctx.cr[6].eq {
	pc = 0x82420E3C; continue 'dispatch;
	}
	// 82420E34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82420E38: 48000094  b 0x82420ecc
	pc = 0x82420ECC; continue 'dispatch;
	// 82420E3C: 2F04012B  cmpwi cr6, r4, 0x12b
	ctx.cr[6].compare_i32(ctx.r[4].s32, 299, &mut ctx.xer);
	// 82420E40: 41990050  bgt cr6, 0x82420e90
	if ctx.cr[6].gt {
	pc = 0x82420E90; continue 'dispatch;
	}
	// 82420E44: 419AFFF0  beq cr6, 0x82420e34
	if ctx.cr[6].eq {
	pc = 0x82420E34; continue 'dispatch;
	}
	// 82420E48: 2F0400C8  cmpwi cr6, r4, 0xc8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 200, &mut ctx.xer);
	// 82420E4C: 419AFFE8  beq cr6, 0x82420e34
	if ctx.cr[6].eq {
	pc = 0x82420E34; continue 'dispatch;
	}
	// 82420E50: 2F0400C9  cmpwi cr6, r4, 0xc9
	ctx.cr[6].compare_i32(ctx.r[4].s32, 201, &mut ctx.xer);
	// 82420E54: 419A0034  beq cr6, 0x82420e88
	if ctx.cr[6].eq {
	pc = 0x82420E88; continue 'dispatch;
	}
	// 82420E58: 2F0400CA  cmpwi cr6, r4, 0xca
	ctx.cr[6].compare_i32(ctx.r[4].s32, 202, &mut ctx.xer);
	// 82420E5C: 419AFFD8  beq cr6, 0x82420e34
	if ctx.cr[6].eq {
	pc = 0x82420E34; continue 'dispatch;
	}
	// 82420E60: 2F0400CB  cmpwi cr6, r4, 0xcb
	ctx.cr[6].compare_i32(ctx.r[4].s32, 203, &mut ctx.xer);
	// 82420E64: 419A0014  beq cr6, 0x82420e78
	if ctx.cr[6].eq {
	pc = 0x82420E78; continue 'dispatch;
	}
	// 82420E68: 2F0400CC  cmpwi cr6, r4, 0xcc
	ctx.cr[6].compare_i32(ctx.r[4].s32, 204, &mut ctx.xer);
	// 82420E6C: 419AFFC8  beq cr6, 0x82420e34
	if ctx.cr[6].eq {
	pc = 0x82420E34; continue 'dispatch;
	}
	// 82420E70: 2F0400CD  cmpwi cr6, r4, 0xcd
	ctx.cr[6].compare_i32(ctx.r[4].s32, 205, &mut ctx.xer);
	// 82420E74: 409A0044  bne cr6, 0x82420eb8
	if !ctx.cr[6].eq {
	pc = 0x82420EB8; continue 'dispatch;
	}
	// 82420E78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82420E7C: 4BFFF7F5  bl 0x82420670
	ctx.lr = 0x82420E80;
	sub_82420670(ctx, base);
	// 82420E80: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82420E84: 48000048  b 0x82420ecc
	pc = 0x82420ECC; continue 'dispatch;
	// 82420E88: 4BFFFF39  bl 0x82420dc0
	ctx.lr = 0x82420E8C;
	sub_82420DC0(ctx, base);
	// 82420E8C: 48000040  b 0x82420ecc
	pc = 0x82420ECC; continue 'dispatch;
	// 82420E90: 2F04012C  cmpwi cr6, r4, 0x12c
	ctx.cr[6].compare_i32(ctx.r[4].s32, 300, &mut ctx.xer);
	// 82420E94: 419A0034  beq cr6, 0x82420ec8
	if ctx.cr[6].eq {
	pc = 0x82420EC8; continue 'dispatch;
	}
	// 82420E98: 2F04012D  cmpwi cr6, r4, 0x12d
	ctx.cr[6].compare_i32(ctx.r[4].s32, 301, &mut ctx.xer);
	// 82420E9C: 419AFF98  beq cr6, 0x82420e34
	if ctx.cr[6].eq {
	pc = 0x82420E34; continue 'dispatch;
	}
	// 82420EA0: 2F04012E  cmpwi cr6, r4, 0x12e
	ctx.cr[6].compare_i32(ctx.r[4].s32, 302, &mut ctx.xer);
	// 82420EA4: 419A0024  beq cr6, 0x82420ec8
	if ctx.cr[6].eq {
	pc = 0x82420EC8; continue 'dispatch;
	}
	// 82420EA8: 2F040190  cmpwi cr6, r4, 0x190
	ctx.cr[6].compare_i32(ctx.r[4].s32, 400, &mut ctx.xer);
	// 82420EAC: 419AFF88  beq cr6, 0x82420e34
	if ctx.cr[6].eq {
	pc = 0x82420E34; continue 'dispatch;
	}
	// 82420EB0: 2F040258  cmpwi cr6, r4, 0x258
	ctx.cr[6].compare_i32(ctx.r[4].s32, 600, &mut ctx.xer);
	// 82420EB4: 419A000C  beq cr6, 0x82420ec0
	if ctx.cr[6].eq {
	pc = 0x82420EC0; continue 'dispatch;
	}
	// 82420EB8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82420EBC: 48000010  b 0x82420ecc
	pc = 0x82420ECC; continue 'dispatch;
	// 82420EC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82420EC4: 48000008  b 0x82420ecc
	pc = 0x82420ECC; continue 'dispatch;
	// 82420EC8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82420ECC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420EE0 size=88
    let mut pc: u32 = 0x82420EE0;
    'dispatch: loop {
        match pc {
            0x82420EE0 => {
    //   block [0x82420EE0..0x82420F38)
	// 82420EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420EE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82420EEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82420EF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82420EF8: 419A002C  beq cr6, 0x82420f24
	if ctx.cr[6].eq {
	pc = 0x82420F24; continue 'dispatch;
	}
	// 82420EFC: 4BFFFD0D  bl 0x82420c08
	ctx.lr = 0x82420F00;
	sub_82420C08(ctx, base);
	// 82420F00: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420F04: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82420F08: 409A001C  bne cr6, 0x82420f24
	if !ctx.cr[6].eq {
	pc = 0x82420F24; continue 'dispatch;
	}
	// 82420F0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82420F10: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 82420F14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82420F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82420F1C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82420F20: 481142B1  bl 0x825351d0
	ctx.lr = 0x82420F24;
	sub_825351D0(ctx, base);
	// 82420F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420F30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82420F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420F38 size=128
    let mut pc: u32 = 0x82420F38;
    'dispatch: loop {
        match pc {
            0x82420F38 => {
    //   block [0x82420F38..0x82420FB8)
	// 82420F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82420F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420F48: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420F4C: 3BEB8DA4  addi r31, r11, -0x725c
	ctx.r[31].s64 = ctx.r[11].s64 + -29276;
	// 82420F50: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 82420F54: 395F002C  addi r10, r31, 0x2c
	ctx.r[10].s64 = ctx.r[31].s64 + 44;
	// 82420F58: 393F002C  addi r9, r31, 0x2c
	ctx.r[9].s64 = ctx.r[31].s64 + 44;
	// 82420F5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420F60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82420F64: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82420F68: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420F6C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82420F70: 409A0034  bne cr6, 0x82420fa4
	if !ctx.cr[6].eq {
	pc = 0x82420FA4; continue 'dispatch;
	}
	// 82420F74: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82420F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82420F7C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82420F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82420F84: 916A8DC8  stw r11, -0x7238(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29240 as u32), ctx.r[11].u32 ) };
	// 82420F88: 4800BED1  bl 0x8242ce58
	ctx.lr = 0x82420F8C;
	sub_8242CE58(ctx, base);
	// 82420F8C: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82420F90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82420F94: 40820010  bne 0x82420fa4
	if !ctx.cr[0].eq {
	pc = 0x82420FA4; continue 'dispatch;
	}
	// 82420F98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82420F9C: 386B1668  addi r3, r11, 0x1668
	ctx.r[3].s64 = ctx.r[11].s64 + 5736;
	// 82420FA0: 4BFFBC69  bl 0x8241cc08
	ctx.lr = 0x82420FA4;
	sub_8241CC08(ctx, base);
	// 82420FA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82420FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82420FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82420FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82420FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82420FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82420FB8 size=116
    let mut pc: u32 = 0x82420FB8;
    'dispatch: loop {
        match pc {
            0x82420FB8 => {
    //   block [0x82420FB8..0x8242102C)
	// 82420FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82420FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82420FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82420FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82420FC8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82420FCC: 3BEB8DC4  addi r31, r11, -0x723c
	ctx.r[31].s64 = ctx.r[11].s64 + -29244;
	// 82420FD0: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82420FD4: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82420FD8: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 82420FDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420FE0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82420FE4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82420FE8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420FEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82420FF0: 409A0028  bne cr6, 0x82421018
	if !ctx.cr[6].eq {
	pc = 0x82421018; continue 'dispatch;
	}
	// 82420FF4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82420FF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82420FFC: 419A0010  beq cr6, 0x8242100c
	if ctx.cr[6].eq {
	pc = 0x8242100C; continue 'dispatch;
	}
	// 82421000: 4800BF39  bl 0x8242cf38
	ctx.lr = 0x82421004;
	sub_8242CF38(ctx, base);
	// 82421004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82421008: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242100C: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82421010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82421014: 916A8DC8  stw r11, -0x7238(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29240 as u32), ctx.r[11].u32 ) };
	// 82421018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242101C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82421028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421030 size=60
    let mut pc: u32 = 0x82421030;
    'dispatch: loop {
        match pc {
            0x82421030 => {
    //   block [0x82421030..0x8242106C)
	// 82421030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242103C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82421040: 806B8DC4  lwz r3, -0x723c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29244 as u32) ) } as u64;
	// 82421044: 4800BF85  bl 0x8242cfc8
	ctx.lr = 0x82421048;
	sub_8242CFC8(ctx, base);
	// 82421048: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242104C: 40800010  bge 0x8242105c
	if !ctx.cr[0].lt {
	pc = 0x8242105C; continue 'dispatch;
	}
	// 82421050: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82421054: 386B16A0  addi r3, r11, 0x16a0
	ctx.r[3].s64 = ctx.r[11].s64 + 5792;
	// 82421058: 4BFFBBB1  bl 0x8241cc08
	ctx.lr = 0x8242105C;
	sub_8241CC08(ctx, base);
	// 8242105C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82421060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421070 size=60
    let mut pc: u32 = 0x82421070;
    'dispatch: loop {
        match pc {
            0x82421070 => {
    //   block [0x82421070..0x824210AC)
	// 82421070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242107C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82421080: 806B8DC4  lwz r3, -0x723c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29244 as u32) ) } as u64;
	// 82421084: 4800BFDD  bl 0x8242d060
	ctx.lr = 0x82421088;
	sub_8242D060(ctx, base);
	// 82421088: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242108C: 40800010  bge 0x8242109c
	if !ctx.cr[0].lt {
	pc = 0x8242109C; continue 'dispatch;
	}
	// 82421090: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82421094: 386B1700  addi r3, r11, 0x1700
	ctx.r[3].s64 = ctx.r[11].s64 + 5888;
	// 82421098: 4BFFBB71  bl 0x8241cc08
	ctx.lr = 0x8242109C;
	sub_8241CC08(ctx, base);
	// 8242109C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824210A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824210A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824210A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824210B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824210B0 size=224
    let mut pc: u32 = 0x824210B0;
    'dispatch: loop {
        match pc {
            0x824210B0 => {
    //   block [0x824210B0..0x82421190)
	// 824210B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824210B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824210B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824210BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824210C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824210C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824210C8: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 824210CC: 396B1760  addi r11, r11, 0x1760
	ctx.r[11].s64 = ctx.r[11].s64 + 5984;
	// 824210D0: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824210D4: 83FE8DD8  lwz r31, -0x7228(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-29224 as u32) ) } as u64;
	// 824210D8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824210DC: 916A8DD4  stw r11, -0x722c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29228 as u32), ctx.r[11].u32 ) };
	// 824210E0: 409A0090  bne cr6, 0x82421170
	if !ctx.cr[6].eq {
	pc = 0x82421170; continue 'dispatch;
	}
	// 824210E4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824210E8: 38A00340  li r5, 0x340
	ctx.r[5].s64 = 832;
	// 824210EC: 386BC980  addi r3, r11, -0x3680
	ctx.r[3].s64 = ctx.r[11].s64 + -13952;
	// 824210F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824210F4: 481140DD  bl 0x825351d0
	ctx.lr = 0x824210F8;
	sub_825351D0(ctx, base);
	// 824210F8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824210FC: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 82421100: 386BC580  addi r3, r11, -0x3a80
	ctx.r[3].s64 = ctx.r[11].s64 + -14976;
	// 82421104: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421108: 481140C9  bl 0x825351d0
	ctx.lr = 0x8242110C;
	sub_825351D0(ctx, base);
	// 8242110C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421110: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82421114: 386BC420  addi r3, r11, -0x3be0
	ctx.r[3].s64 = ctx.r[11].s64 + -15328;
	// 82421118: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 8242111C: 481140B5  bl 0x825351d0
	ctx.lr = 0x82421120;
	sub_825351D0(ctx, base);
	// 82421120: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421124: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82421128: 386BC540  addi r3, r11, -0x3ac0
	ctx.r[3].s64 = ctx.r[11].s64 + -15040;
	// 8242112C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421130: 481140A1  bl 0x825351d0
	ctx.lr = 0x82421134;
	sub_825351D0(ctx, base);
	// 82421134: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82421138: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8242113C: 916AC520  stw r11, -0x3ae0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15072 as u32), ctx.r[11].u32 ) };
	// 82421140: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82421144: 916AC524  stw r11, -0x3adc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15068 as u32), ctx.r[11].u32 ) };
	// 82421148: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8242114C: 916AC564  stw r11, -0x3a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15004 as u32), ctx.r[11].u32 ) };
	// 82421150: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82421154: 916AC560  stw r11, -0x3aa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15008 as u32), ctx.r[11].u32 ) };
	// 82421158: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8242115C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82421160: 916AC400  stw r11, -0x3c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15360 as u32), ctx.r[11].u32 ) };
	// 82421164: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82421168: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242116C: 916AC404  stw r11, -0x3bfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82421170: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82421174: 917E8DD8  stw r11, -0x7228(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-29224 as u32), ctx.r[11].u32 ) };
	// 82421178: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242117C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421184: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82421188: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242118C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421190 size=192
    let mut pc: u32 = 0x82421190;
    'dispatch: loop {
        match pc {
            0x82421190 => {
    //   block [0x82421190..0x82421250)
	// 82421190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242119C: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824211A0: 816A8DD8  lwz r11, -0x7228(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29224 as u32) ) } as u64;
	// 824211A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824211A8: 916A8DD8  stw r11, -0x7228(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29224 as u32), ctx.r[11].u32 ) };
	// 824211AC: 40820094  bne 0x82421240
	if !ctx.cr[0].eq {
	pc = 0x82421240; continue 'dispatch;
	}
	// 824211B0: 4BFF6BD9  bl 0x82417d88
	ctx.lr = 0x824211B4;
	sub_82417D88(ctx, base);
	// 824211B4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824211B8: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824211BC: 386BC540  addi r3, r11, -0x3ac0
	ctx.r[3].s64 = ctx.r[11].s64 + -15040;
	// 824211C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824211C4: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 824211C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824211CC: 916AC404  stw r11, -0x3bfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 824211D0: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824211D4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824211D8: 916AC400  stw r11, -0x3c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15360 as u32), ctx.r[11].u32 ) };
	// 824211DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824211E0: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824211E4: 916AC560  stw r11, -0x3aa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15008 as u32), ctx.r[11].u32 ) };
	// 824211E8: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824211EC: 916AC564  stw r11, -0x3a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15004 as u32), ctx.r[11].u32 ) };
	// 824211F0: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824211F4: 916AC524  stw r11, -0x3adc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15068 as u32), ctx.r[11].u32 ) };
	// 824211F8: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824211FC: 916AC520  stw r11, -0x3ae0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15072 as u32), ctx.r[11].u32 ) };
	// 82421200: 48113FD1  bl 0x825351d0
	ctx.lr = 0x82421204;
	sub_825351D0(ctx, base);
	// 82421204: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421208: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8242120C: 386BC420  addi r3, r11, -0x3be0
	ctx.r[3].s64 = ctx.r[11].s64 + -15328;
	// 82421210: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82421214: 48113FBD  bl 0x825351d0
	ctx.lr = 0x82421218;
	sub_825351D0(ctx, base);
	// 82421218: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8242121C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 82421220: 386BC580  addi r3, r11, -0x3a80
	ctx.r[3].s64 = ctx.r[11].s64 + -14976;
	// 82421224: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421228: 48113FA9  bl 0x825351d0
	ctx.lr = 0x8242122C;
	sub_825351D0(ctx, base);
	// 8242122C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421230: 38A00340  li r5, 0x340
	ctx.r[5].s64 = 832;
	// 82421234: 386BC980  addi r3, r11, -0x3680
	ctx.r[3].s64 = ctx.r[11].s64 + -13952;
	// 82421238: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242123C: 48113F95  bl 0x825351d0
	ctx.lr = 0x82421240;
	sub_825351D0(ctx, base);
	// 82421240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82421244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242124C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421250 size=68
    let mut pc: u32 = 0x82421250;
    'dispatch: loop {
        match pc {
            0x82421250 => {
    //   block [0x82421250..0x82421294)
	// 82421250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242125C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421260: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82421264: 386BC300  addi r3, r11, -0x3d00
	ctx.r[3].s64 = ctx.r[11].s64 + -15616;
	// 82421268: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242126C: 48113F65  bl 0x825351d0
	ctx.lr = 0x82421270;
	sub_825351D0(ctx, base);
	// 82421270: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82421274: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82421278: 916A8DFC  stw r11, -0x7204(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29188 as u32), ctx.r[11].u32 ) };
	// 8242127C: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82421280: 916A8E00  stw r11, -0x7200(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29184 as u32), ctx.r[11].u32 ) };
	// 82421284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82421288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242128C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421298 size=132
    let mut pc: u32 = 0x82421298;
    'dispatch: loop {
        match pc {
            0x82421298 => {
    //   block [0x82421298..0x8242131C)
	// 82421298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242129C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824212A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824212A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824212A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824212AC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 824212B0: 409A0010  bne cr6, 0x824212c0
	if !ctx.cr[6].eq {
	pc = 0x824212C0; continue 'dispatch;
	}
	// 824212B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824212B8: 386B17AC  addi r3, r11, 0x17ac
	ctx.r[3].s64 = ctx.r[11].s64 + 6060;
	// 824212BC: 48000048  b 0x82421304
	pc = 0x82421304; continue 'dispatch;
	// 824212C0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824212C4: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 824212C8: 3BEBC300  addi r31, r11, -0x3d00
	ctx.r[31].s64 = ctx.r[11].s64 + -15616;
	// 824212CC: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 824212D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824212D4: 48002285  bl 0x82423558
	ctx.lr = 0x824212D8;
	sub_82423558(ctx, base);
	// 824212D8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824212DC: 814B8DFC  lwz r10, -0x7204(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29188 as u32) ) } as u64;
	// 824212E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824212E4: 419A001C  beq cr6, 0x82421300
	if ctx.cr[6].eq {
	pc = 0x82421300; continue 'dispatch;
	}
	// 824212E8: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824212EC: 816B8DFC  lwz r11, -0x7204(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29188 as u32) ) } as u64;
	// 824212F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824212F4: 806A8E00  lwz r3, -0x7200(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29184 as u32) ) } as u64;
	// 824212F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824212FC: 4E800421  bctrl
	ctx.lr = 0x82421300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421304: 48004465  bl 0x82425768
	ctx.lr = 0x82421308;
	sub_82425768(ctx, base);
	// 82421308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242130C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82421318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421320 size=172
    let mut pc: u32 = 0x82421320;
    'dispatch: loop {
        match pc {
            0x82421320 => {
    //   block [0x82421320..0x824213CC)
	// 82421320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242132C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82421330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82421334: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82421338: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242133C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82421340: 419A0068  beq cr6, 0x824213a8
	if ctx.cr[6].eq {
	pc = 0x824213A8; continue 'dispatch;
	}
	// 82421344: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82421348: 419A0060  beq cr6, 0x824213a8
	if ctx.cr[6].eq {
	pc = 0x824213A8; continue 'dispatch;
	}
	// 8242134C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421350: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 82421354: 3BEBC300  addi r31, r11, -0x3d00
	ctx.r[31].s64 = ctx.r[11].s64 + -15616;
	// 82421358: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 8242135C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421360: 480021F9  bl 0x82423558
	ctx.lr = 0x82421364;
	sub_82423558(ctx, base);
	// 82421364: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 82421368: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8242136C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82421370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421374: 48002215  bl 0x82423588
	ctx.lr = 0x82421378;
	sub_82423588(ctx, base);
	// 82421378: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242137C: 814B8DFC  lwz r10, -0x7204(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29188 as u32) ) } as u64;
	// 82421380: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82421384: 419A001C  beq cr6, 0x824213a0
	if ctx.cr[6].eq {
	pc = 0x824213A0; continue 'dispatch;
	}
	// 82421388: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242138C: 816B8DFC  lwz r11, -0x7204(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29188 as u32) ) } as u64;
	// 82421390: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82421394: 806A8E00  lwz r3, -0x7200(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29184 as u32) ) } as u64;
	// 82421398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242139C: 4E800421  bctrl
	ctx.lr = 0x824213A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824213A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824213A4: 4800000C  b 0x824213b0
	pc = 0x824213B0; continue 'dispatch;
	// 824213A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824213AC: 386B17AC  addi r3, r11, 0x17ac
	ctx.r[3].s64 = ctx.r[11].s64 + 6060;
	// 824213B0: 480043B9  bl 0x82425768
	ctx.lr = 0x824213B4;
	sub_82425768(ctx, base);
	// 824213B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824213B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824213BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824213C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824213C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824213C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824213D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824213D0 size=52
    let mut pc: u32 = 0x824213D0;
    'dispatch: loop {
        match pc {
            0x824213D0 => {
    //   block [0x824213D0..0x82421404)
	// 824213D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824213D4: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 824213D8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 824213DC: 7D2353D6  divw r9, r3, r10
	ctx.r[9].s32 = ctx.r[3].s32 / ctx.r[10].s32;
	// 824213E0: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 824213E4: 7D291850  subf r9, r9, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[9].s64;
	// 824213E8: 7C6353D7  divw. r3, r3, r10
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824213EC: 7D2B21AE  stbx r9, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u8) };
	// 824213F0: 41820014  beq 0x82421404
	if ctx.cr[0].eq {
		sub_82421404(ctx, base);
		return;
	}
	// 824213F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824213F8: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 824213FC: 4198FFE0  blt cr6, 0x824213dc
	if ctx.cr[6].lt {
	pc = 0x824213DC; continue 'dispatch;
	}
	// 82421400: 48000008  b 0x82421408
	sub_82421404(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421404(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82421404 size=116
    let mut pc: u32 = 0x82421404;
    'dispatch: loop {
        match pc {
            0x82421404 => {
    //   block [0x82421404..0x82421478)
	// 82421404: 7D0B21AE  stbx r8, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[8].u8) };
	// 82421408: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242140C: 392B8DDC  addi r9, r11, -0x7224
	ctx.r[9].s64 = ctx.r[11].s64 + -29220;
	// 82421410: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82421414: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82421418: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242141C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82421420: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82421424: 409AFFF4  bne cr6, 0x82421418
	if !ctx.cr[6].eq {
	pc = 0x82421418; continue 'dispatch;
	}
	// 82421428: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242142C: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 82421430: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82421434: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82421438: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8242143C: 41980008  blt cr6, 0x82421444
	if ctx.cr[6].lt {
	pc = 0x82421444; continue 'dispatch;
	}
	// 82421440: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82421444: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82421448: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242144C: 40990024  ble cr6, 0x82421470
	if !ctx.cr[6].gt {
	pc = 0x82421470; continue 'dispatch;
	}
	// 82421450: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82421454: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82421458: 88E90000  lbz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242145C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82421460: 7CEA21AE  stbx r7, r10, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[7].u8) };
	// 82421464: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82421468: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242146C: 4198FFEC  blt cr6, 0x82421458
	if ctx.cr[6].lt {
	pc = 0x82421458; continue 'dispatch;
	}
	// 82421470: 7D0A21AE  stbx r8, r10, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[8].u8) };
	// 82421474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421478 size=216
    let mut pc: u32 = 0x82421478;
    'dispatch: loop {
        match pc {
            0x82421478 => {
    //   block [0x82421478..0x82421550)
	// 82421478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82421484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82421488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242148C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82421490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82421494: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82421498: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8242149C: 4BFFFF35  bl 0x824213d0
	ctx.lr = 0x824214A0;
	sub_824213D0(ctx, base);
	// 824214A0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824214A4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824214A8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824214AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824214B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824214B4: 409AFFF4  bne cr6, 0x824214a8
	if !ctx.cr[6].eq {
	pc = 0x824214A8; continue 'dispatch;
	}
	// 824214B8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824214BC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824214C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824214C4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 824214C8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824214CC: 38AAA440  addi r5, r10, -0x5bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -23488;
	// 824214D0: 7D6B3050  subf r11, r11, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 824214D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824214D8: 38CBFFFF  addi r6, r11, -1
	ctx.r[6].s64 = ctx.r[11].s64 + -1;
	// 824214DC: 480020AD  bl 0x82423588
	ctx.lr = 0x824214E0;
	sub_82423588(ctx, base);
	// 824214E0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824214E4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 824214E8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824214EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824214F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824214F4: 409AFFF4  bne cr6, 0x824214e8
	if !ctx.cr[6].eq {
	pc = 0x824214E8; continue 'dispatch;
	}
	// 824214F8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824214FC: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82421500: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82421504: 5569003E  slwi r9, r11, 0
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82421508: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242150C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421510: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82421514: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82421518: 409AFFF4  bne cr6, 0x8242150c
	if !ctx.cr[6].eq {
	pc = 0x8242150C; continue 'dispatch;
	}
	// 8242151C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82421520: 20A90004  subfic r5, r9, 4
	ctx.xer.ca = ctx.r[9].u32 <= 4 as u32;
	ctx.r[5].s64 = (4 as i64) - ctx.r[9].s64;
	// 82421524: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82421528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242152C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82421530: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82421534: 4BFFFE9D  bl 0x824213d0
	ctx.lr = 0x82421538;
	sub_824213D0(ctx, base);
	// 82421538: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242153C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421544: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82421548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421550 size=76
    let mut pc: u32 = 0x82421550;
    'dispatch: loop {
        match pc {
            0x82421550 => {
    //   block [0x82421550..0x8242159C)
	// 82421550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242155C: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82421560: 816A8E04  lwz r11, -0x71fc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29180 as u32) ) } as u64;
	// 82421564: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82421568: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8242156C: 916A8E04  stw r11, -0x71fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29180 as u32), ctx.r[11].u32 ) };
	// 82421570: 409A0018  bne cr6, 0x82421588
	if !ctx.cr[6].eq {
	pc = 0x82421588; continue 'dispatch;
	}
	// 82421574: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421578: 38A02080  li r5, 0x2080
	ctx.r[5].s64 = 8320;
	// 8242157C: 386BA280  addi r3, r11, -0x5d80
	ctx.r[3].s64 = ctx.r[11].s64 + -23936;
	// 82421580: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421584: 48113C4D  bl 0x825351d0
	ctx.lr = 0x82421588;
	sub_825351D0(ctx, base);
	// 82421588: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242158C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82421590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824215A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824215A0 size=20
    let mut pc: u32 = 0x824215A0;
    'dispatch: loop {
        match pc {
            0x824215A0 => {
    //   block [0x824215A0..0x824215B4)
	// 824215A0: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824215A4: 816A8E04  lwz r11, -0x71fc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29180 as u32) ) } as u64;
	// 824215A8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824215AC: 916A8E04  stw r11, -0x71fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29180 as u32), ctx.r[11].u32 ) };
	// 824215B0: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824215B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824215B4 size=20
    let mut pc: u32 = 0x824215B4;
    'dispatch: loop {
        match pc {
            0x824215B4 => {
    //   block [0x824215B4..0x824215C8)
	// 824215B4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824215B8: 38A02080  li r5, 0x2080
	ctx.r[5].s64 = 8320;
	// 824215BC: 386BA280  addi r3, r11, -0x5d80
	ctx.r[3].s64 = ctx.r[11].s64 + -23936;
	// 824215C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824215C4: 48113C0C  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824215C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824215C8 size=4
    let mut pc: u32 = 0x824215C8;
    'dispatch: loop {
        match pc {
            0x824215C8 => {
    //   block [0x824215C8..0x824215CC)
	// 824215C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824215D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824215D0 size=20
    let mut pc: u32 = 0x824215D0;
    'dispatch: loop {
        match pc {
            0x824215D0 => {
    //   block [0x824215D0..0x824215E4)
	// 824215D0: 8963004A  lbz r11, 0x4a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(74 as u32) ) } as u64;
	// 824215D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824215D8: 419A000C  beq cr6, 0x824215e4
	if ctx.cr[6].eq {
		sub_824215E4(ctx, base);
		return;
	}
	// 824215DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824215E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824215E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824215E4 size=20
    let mut pc: u32 = 0x824215E4;
    'dispatch: loop {
        match pc {
            0x824215E4 => {
    //   block [0x824215E4..0x824215F8)
	// 824215E4: 89630049  lbz r11, 0x49(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(73 as u32) ) } as u64;
	// 824215E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824215EC: 419A000C  beq cr6, 0x824215f8
	if ctx.cr[6].eq {
		sub_824215F8(ctx, base);
		return;
	}
	// 824215F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824215F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824215F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824215F8 size=24
    let mut pc: u32 = 0x824215F8;
    'dispatch: loop {
        match pc {
            0x824215F8 => {
    //   block [0x824215F8..0x82421610)
	// 824215F8: 8963004D  lbz r11, 0x4d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(77 as u32) ) } as u64;
	// 824215FC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82421600: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82421604: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82421608: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8242160C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82421610 size=28
    let mut pc: u32 = 0x82421610;
    'dispatch: loop {
        match pc {
            0x82421610 => {
    //   block [0x82421610..0x8242162C)
	// 82421610: 89630049  lbz r11, 0x49(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(73 as u32) ) } as u64;
	// 82421614: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421618: 409A0014  bne cr6, 0x8242162c
	if !ctx.cr[6].eq {
		sub_8242162C(ctx, base);
		return;
	}
	// 8242161C: 8963004D  lbz r11, 0x4d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(77 as u32) ) } as u64;
	// 82421620: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82421624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421628: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242162C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242162C size=8
    let mut pc: u32 = 0x8242162C;
    'dispatch: loop {
        match pc {
            0x8242162C => {
    //   block [0x8242162C..0x82421634)
	// 8242162C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82421630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421638 size=36
    let mut pc: u32 = 0x82421638;
    'dispatch: loop {
        match pc {
            0x82421638 => {
    //   block [0x82421638..0x8242165C)
	// 82421638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82421644: 4BFFDD85  bl 0x8241f3c8
	ctx.lr = 0x82421648;
	sub_8241F3C8(ctx, base);
	// 82421648: 4BFFE449  bl 0x8241fa90
	ctx.lr = 0x8242164C;
	sub_8241FA90(ctx, base);
	// 8242164C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82421650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82421660 size=12
    let mut pc: u32 = 0x82421660;
    'dispatch: loop {
        match pc {
            0x82421660 => {
    //   block [0x82421660..0x8242166C)
	// 82421660: 8963004D  lbz r11, 0x4d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(77 as u32) ) } as u64;
	// 82421664: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82421668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82421670 size=240
    let mut pc: u32 = 0x82421670;
    'dispatch: loop {
        match pc {
            0x82421670 => {
    //   block [0x82421670..0x82421760)
	// 82421670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421674: 48113A39  bl 0x825350ac
	ctx.lr = 0x82421678;
	sub_82535080(ctx, base);
	// 82421678: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242167C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82421680: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82421684: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82421688: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8242168C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82421690: 4800D8C1  bl 0x8242ef50
	ctx.lr = 0x82421694;
	sub_8242EF50(ctx, base);
	// 82421694: 7FCB5E70  srawi r11, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 82421698: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8242169C: 7FCA07B4  extsw r10, r30
	ctx.r[10].s64 = ctx.r[30].s32 as i64;
	// 824216A0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 824216A4: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 824216A8: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 824216AC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824216B0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824216B4: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824216B8: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 824216BC: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 824216C0: 9B3F0001  stb r25, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[25].u8 ) };
	// 824216C4: 7D6BF051  subf. r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824216C8: 9B5F0002  stb r26, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[26].u8 ) };
	// 824216CC: 41810008  bgt 0x824216d4
	if ctx.cr[0].gt {
	pc = 0x824216D4; continue 'dispatch;
	}
	// 824216D0: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 824216D4: 7FCB5E70  srawi r11, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 824216D8: 935F005C  stw r26, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 824216DC: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 824216E0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824216E4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824216E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824216EC: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 824216F0: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 824216F4: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 824216F8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824216FC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82421700: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82421704: 419A0048  beq cr6, 0x8242174c
	if ctx.cr[6].eq {
	pc = 0x8242174C; continue 'dispatch;
	}
	// 82421708: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242170C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82421710: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421714: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82421718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242171C: 4E800421  bctrl
	ctx.lr = 0x82421720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421720: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421724: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82421728: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242172C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421730: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82421734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421738: 4E800421  bctrl
	ctx.lr = 0x8242173C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242173C: 7D7E1A14  add r11, r30, r3
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 82421740: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82421744: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82421748: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8242174C: 9B5F0048  stb r26, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u8 ) };
	// 82421750: 9B3F0000  stb r25, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82421754: 4800D7FD  bl 0x8242ef50
	ctx.lr = 0x82421758;
	sub_8242EF50(ctx, base);
	// 82421758: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8242175C: 481139A0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421760 size=284
    let mut pc: u32 = 0x82421760;
    'dispatch: loop {
        match pc {
            0x82421760 => {
    //   block [0x82421760..0x8242187C)
	// 82421760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421768: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242176C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82421770: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82421774: 2F040100  cmpwi cr6, r4, 0x100
	ctx.cr[6].compare_i32(ctx.r[4].s32, 256, &mut ctx.xer);
	// 82421778: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8242177C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82421780: 40980070  bge cr6, 0x824217f0
	if !ctx.cr[6].lt {
	pc = 0x824217F0; continue 'dispatch;
	}
	// 82421784: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82421788: 810B3C88  lwz r8, 0x3c88(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15496 as u32) ) } as u64;
	// 8242178C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82421790: 4099003C  ble cr6, 0x824217cc
	if !ctx.cr[6].gt {
	pc = 0x824217CC; continue 'dispatch;
	}
	// 82421794: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82421798: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8242179C: 396BA280  addi r11, r11, -0x5d80
	ctx.r[11].s64 = ctx.r[11].s64 + -23936;
	// 824217A0: 814A8E08  lwz r10, -0x71f8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29176 as u32) ) } as u64;
	// 824217A4: 1D4A0068  mulli r10, r10, 0x68
	ctx.r[10].s64 = ctx.r[10].s64 * 104;
	// 824217A8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824217AC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824217B0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824217B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824217B8: 419A0014  beq cr6, 0x824217cc
	if ctx.cr[6].eq {
	pc = 0x824217CC; continue 'dispatch;
	}
	// 824217BC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824217C0: 396B0068  addi r11, r11, 0x68
	ctx.r[11].s64 = ctx.r[11].s64 + 104;
	// 824217C4: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824217C8: 4198FFE4  blt cr6, 0x824217ac
	if ctx.cr[6].lt {
	pc = 0x824217AC; continue 'dispatch;
	}
	// 824217CC: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824217D0: 419A0070  beq cr6, 0x82421840
	if ctx.cr[6].eq {
	pc = 0x82421840; continue 'dispatch;
	}
	// 824217D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824217D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824217DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824217E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824217E4: 4BFFFE8D  bl 0x82421670
	ctx.lr = 0x824217E8;
	sub_82421670(ctx, base);
	// 824217E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824217EC: 48000074  b 0x82421860
	pc = 0x82421860; continue 'dispatch;
	// 824217F0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 824217F4: 810B3C90  lwz r8, 0x3c90(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15504 as u32) ) } as u64;
	// 824217F8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824217FC: 4099003C  ble cr6, 0x82421838
	if !ctx.cr[6].gt {
	pc = 0x82421838; continue 'dispatch;
	}
	// 82421800: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 82421804: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82421808: 396BA280  addi r11, r11, -0x5d80
	ctx.r[11].s64 = ctx.r[11].s64 + -23936;
	// 8242180C: 814A3C8C  lwz r10, 0x3c8c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(15500 as u32) ) } as u64;
	// 82421810: 1D4A0068  mulli r10, r10, 0x68
	ctx.r[10].s64 = ctx.r[10].s64 * 104;
	// 82421814: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82421818: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242181C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82421820: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82421824: 419A0014  beq cr6, 0x82421838
	if ctx.cr[6].eq {
	pc = 0x82421838; continue 'dispatch;
	}
	// 82421828: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8242182C: 396B0068  addi r11, r11, 0x68
	ctx.r[11].s64 = ctx.r[11].s64 + 104;
	// 82421830: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82421834: 4198FFE4  blt cr6, 0x82421818
	if ctx.cr[6].lt {
	pc = 0x82421818; continue 'dispatch;
	}
	// 82421838: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8242183C: 409A000C  bne cr6, 0x82421848
	if !ctx.cr[6].eq {
	pc = 0x82421848; continue 'dispatch;
	}
	// 82421840: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82421844: 48000024  b 0x82421868
	pc = 0x82421868; continue 'dispatch;
	// 82421848: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8242184C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82421850: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421858: 4BFFFE19  bl 0x82421670
	ctx.lr = 0x8242185C;
	sub_82421670(ctx, base);
	// 8242185C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82421860: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82421864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242186C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82421874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82421878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421880 size=60
    let mut pc: u32 = 0x82421880;
    'dispatch: loop {
        match pc {
            0x82421880 => {
    //   block [0x82421880..0x824218BC)
	// 82421880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242188C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82421890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82421894: 4BFFF79D  bl 0x82421030
	ctx.lr = 0x82421898;
	sub_82421030(ctx, base);
	// 82421898: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242189C: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 824218A0: 4BFFF7D1  bl 0x82421070
	ctx.lr = 0x824218A4;
	sub_82421070(ctx, base);
	// 824218A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824218A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824218AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824218B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824218B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824218B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824218C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824218C0 size=88
    let mut pc: u32 = 0x824218C0;
    'dispatch: loop {
        match pc {
            0x824218C0 => {
    //   block [0x824218C0..0x82421918)
	// 824218C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824218C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824218C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824218CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824218D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824218D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824218D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824218DC: 4BFFF755  bl 0x82421030
	ctx.lr = 0x824218E0;
	sub_82421030(ctx, base);
	// 824218E0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824218E4: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 824218E8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824218EC: 40990008  ble cr6, 0x824218f4
	if !ctx.cr[6].gt {
	pc = 0x824218F4; continue 'dispatch;
	}
	// 824218F0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824218F4: 83FF005C  lwz r31, 0x5c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 824218F8: 4BFFF779  bl 0x82421070
	ctx.lr = 0x824218FC;
	sub_82421070(ctx, base);
	// 824218FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421900: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82421904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242190C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82421910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82421914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421918 size=76
    let mut pc: u32 = 0x82421918;
    'dispatch: loop {
        match pc {
            0x82421918 => {
    //   block [0x82421918..0x82421964)
	// 82421918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242191C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82421920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82421924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82421928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242192C: 4BFFF705  bl 0x82421030
	ctx.lr = 0x82421930;
	sub_82421030(ctx, base);
	// 82421930: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421934: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421938: 419A000C  beq cr6, 0x82421944
	if ctx.cr[6].eq {
	pc = 0x82421944; continue 'dispatch;
	}
	// 8242193C: 83FF005C  lwz r31, 0x5c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82421940: 48000008  b 0x82421948
	pc = 0x82421948; continue 'dispatch;
	// 82421944: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82421948: 4BFFF729  bl 0x82421070
	ctx.lr = 0x8242194C;
	sub_82421070(ctx, base);
	// 8242194C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82421954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82421958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242195C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82421960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421968 size=48
    let mut pc: u32 = 0x82421968;
    'dispatch: loop {
        match pc {
            0x82421968 => {
    //   block [0x82421968..0x82421998)
	// 82421968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242196C: 48113751  bl 0x825350bc
	ctx.lr = 0x82421970;
	sub_82535080(ctx, base);
	// 82421970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82421974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82421978: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242197C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82421980: 4BFFF6B1  bl 0x82421030
	ctx.lr = 0x82421984;
	sub_82421030(ctx, base);
	// 82421984: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82421988: 93BF0040  stw r29, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 8242198C: 4BFFF6E5  bl 0x82421070
	ctx.lr = 0x82421990;
	sub_82421070(ctx, base);
	// 82421990: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82421994: 48113778  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421998 size=84
    let mut pc: u32 = 0x82421998;
    'dispatch: loop {
        match pc {
            0x82421998 => {
    //   block [0x82421998..0x824219EC)
	// 82421998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824219A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824219A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824219A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824219AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824219B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824219B4: 4BFFF67D  bl 0x82421030
	ctx.lr = 0x824219B8;
	sub_82421030(ctx, base);
	// 824219B8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824219BC: 4198000C  blt cr6, 0x824219c8
	if ctx.cr[6].lt {
	pc = 0x824219C8; continue 'dispatch;
	}
	// 824219C0: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 824219C4: 4800000C  b 0x824219d0
	pc = 0x824219D0; continue 'dispatch;
	// 824219C8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824219CC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824219D0: 4BFFF6A1  bl 0x82421070
	ctx.lr = 0x824219D4;
	sub_82421070(ctx, base);
	// 824219D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824219D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824219DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824219E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824219E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824219E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824219F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824219F0 size=972
    let mut pc: u32 = 0x824219F0;
    'dispatch: loop {
        match pc {
            0x824219F0 => {
    //   block [0x824219F0..0x82421DBC)
	// 824219F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824219F4: 481136B1  bl 0x825350a4
	ctx.lr = 0x824219F8;
	sub_82535080(ctx, base);
	// 824219F8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824219FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82421A00: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82421A04: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421A08: 4BFFE101  bl 0x8241fb08
	ctx.lr = 0x82421A0C;
	sub_8241FB08(ctx, base);
	// 82421A0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82421A10: 4800D541  bl 0x8242ef50
	ctx.lr = 0x82421A14;
	sub_8242EF50(ctx, base);
	// 82421A14: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82421A18: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82421A1C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82421A20: 3B400003  li r26, 3
	ctx.r[26].s64 = 3;
	// 82421A24: 6159FFFF  ori r25, r10, 0xffff
	ctx.r[25].u64 = ctx.r[10].u64 | 65535;
	// 82421A28: 3AE00004  li r23, 4
	ctx.r[23].s64 = 4;
	// 82421A2C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421A30: 3F00828A  lis r24, -0x7d76
	ctx.r[24].s64 = -2104885248;
	// 82421A34: 409A0164  bne cr6, 0x82421b98
	if !ctx.cr[6].eq {
	pc = 0x82421B98; continue 'dispatch;
	}
	// 82421A38: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82421A3C: 409A00E0  bne cr6, 0x82421b1c
	if !ctx.cr[6].eq {
	pc = 0x82421B1C; continue 'dispatch;
	}
	// 82421A40: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 82421A44: 4800D50D  bl 0x8242ef50
	ctx.lr = 0x82421A48;
	sub_8242EF50(ctx, base);
	// 82421A48: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82421A4C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82421A50: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82421A54: 557E5828  slwi r30, r11, 0xb
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82421A58: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82421A5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82421A60: 480074A9  bl 0x82428f08
	ctx.lr = 0x82421A64;
	sub_82428F08(ctx, base);
	// 82421A64: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421A68: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82421A6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82421A70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421A74: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82421A78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421A7C: 4E800421  bctrl
	ctx.lr = 0x82421A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421A80: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421A84: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82421A88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421A8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421A90: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82421A94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421A98: 4E800421  bctrl
	ctx.lr = 0x82421A9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421A9C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82421AA0: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82421AA4: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82421AA8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82421AAC: 813F0034  lwz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82421AB0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82421AB4: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82421AB8: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82421ABC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82421AC0: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82421AC4: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82421AC8: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82421ACC: 409A001C  bne cr6, 0x82421ae8
	if !ctx.cr[6].eq {
	pc = 0x82421AE8; continue 'dispatch;
	}
	// 82421AD0: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82421AD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82421AD8: 41820010  beq 0x82421ae8
	if ctx.cr[0].eq {
	pc = 0x82421AE8; continue 'dispatch;
	}
	// 82421ADC: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82421AE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421AE4: 4E800421  bctrl
	ctx.lr = 0x82421AE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421AE8: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82421AEC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82421AF0: 40980020  bge cr6, 0x82421b10
	if !ctx.cr[6].lt {
	pc = 0x82421B10; continue 'dispatch;
	}
	// 82421AF4: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82421AF8: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82421AFC: 554AAAFE  srwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82421B00: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82421B04: 41980010  blt cr6, 0x82421b14
	if ctx.cr[6].lt {
	pc = 0x82421B14; continue 'dispatch;
	}
	// 82421B08: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82421B0C: 40980008  bge cr6, 0x82421b14
	if !ctx.cr[6].lt {
	pc = 0x82421B14; continue 'dispatch;
	}
	// 82421B10: 9B5F0001  stb r26, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 82421B14: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82421B18: 48000084  b 0x82421b9c
	pc = 0x82421B9C; continue 'dispatch;
	// 82421B1C: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 82421B20: 409A0078  bne cr6, 0x82421b98
	if !ctx.cr[6].eq {
	pc = 0x82421B98; continue 'dispatch;
	}
	// 82421B24: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 82421B28: 4800D429  bl 0x8242ef50
	ctx.lr = 0x82421B2C;
	sub_8242EF50(ctx, base);
	// 82421B2C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421B30: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 82421B34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421B38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421B3C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82421B40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421B44: 4E800421  bctrl
	ctx.lr = 0x82421B48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421B48: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421B4C: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82421B50: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82421B54: 4BFFE2DD  bl 0x8241fe30
	ctx.lr = 0x82421B58;
	sub_8241FE30(ctx, base);
	// 82421B58: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82421B5C: 4182001C  beq 0x82421b78
	if ctx.cr[0].eq {
	pc = 0x82421B78; continue 'dispatch;
	}
	// 82421B60: 81788E14  lwz r11, -0x71ec(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-29164 as u32) ) } as u64;
	// 82421B64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82421B68: 41980018  blt cr6, 0x82421b80
	if ctx.cr[6].lt {
	pc = 0x82421B80; continue 'dispatch;
	}
	// 82421B6C: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82421B70: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82421B74: 4198000C  blt cr6, 0x82421b80
	if ctx.cr[6].lt {
	pc = 0x82421B80; continue 'dispatch;
	}
	// 82421B78: 9AFF0001  stb r23, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[23].u8 ) };
	// 82421B7C: 48000020  b 0x82421b9c
	pc = 0x82421B9C; continue 'dispatch;
	// 82421B80: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82421B84: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82421B88: 40980014  bge cr6, 0x82421b9c
	if !ctx.cr[6].lt {
	pc = 0x82421B9C; continue 'dispatch;
	}
	// 82421B8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82421B90: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82421B94: 48000008  b 0x82421b9c
	pc = 0x82421B9C; continue 'dispatch;
	// 82421B98: 4800D3B9  bl 0x8242ef50
	ctx.lr = 0x82421B9C;
	sub_8242EF50(ctx, base);
	// 82421B9C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82421BA0: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82421BA4: 419A0210  beq cr6, 0x82421db4
	if ctx.cr[6].eq {
	pc = 0x82421DB4; continue 'dispatch;
	}
	// 82421BA8: 4800D3A9  bl 0x8242ef50
	ctx.lr = 0x82421BAC;
	sub_8242EF50(ctx, base);
	// 82421BAC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82421BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421BB4: 409A01FC  bne cr6, 0x82421db0
	if !ctx.cr[6].eq {
	pc = 0x82421DB0; continue 'dispatch;
	}
	// 82421BB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82421BBC: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82421BC0: 3B7F0028  addi r27, r31, 0x28
	ctx.r[27].s64 = ctx.r[31].s64 + 40;
	// 82421BC4: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82421BC8: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82421BCC: 4800D385  bl 0x8242ef50
	ctx.lr = 0x82421BD0;
	sub_8242EF50(ctx, base);
	// 82421BD0: 897F0048  lbz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82421BD4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421BD8: 419A01D0  beq cr6, 0x82421da8
	if ctx.cr[6].eq {
	pc = 0x82421DA8; continue 'dispatch;
	}
	// 82421BDC: 897F004C  lbz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82421BE0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421BE4: 419A01C4  beq cr6, 0x82421da8
	if ctx.cr[6].eq {
	pc = 0x82421DA8; continue 'dispatch;
	}
	// 82421BE8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82421BEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82421BF0: 409A0014  bne cr6, 0x82421c04
	if !ctx.cr[6].eq {
	pc = 0x82421C04; continue 'dispatch;
	}
	// 82421BF4: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 82421BF8: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82421BFC: 9B5F0001  stb r26, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 82421C00: 480001B4  b 0x82421db4
	pc = 0x82421DB4; continue 'dispatch;
	// 82421C04: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82421C08: 419A0188  beq cr6, 0x82421d90
	if ctx.cr[6].eq {
	pc = 0x82421D90; continue 'dispatch;
	}
	// 82421C0C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421C10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82421C14: 4182017C  beq 0x82421d90
	if ctx.cr[0].eq {
	pc = 0x82421D90; continue 'dispatch;
	}
	// 82421C18: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82421C1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421C20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421C28: 4E800421  bctrl
	ctx.lr = 0x82421C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421C2C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82421C30: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82421C34: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82421C38: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82421C3C: 4098016C  bge cr6, 0x82421da8
	if !ctx.cr[6].lt {
	pc = 0x82421DA8; continue 'dispatch;
	}
	// 82421C40: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421C44: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82421C48: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82421C4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421C50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421C54: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82421C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421C5C: 4E800421  bctrl
	ctx.lr = 0x82421C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421C60: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82421C64: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82421C68: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82421C6C: 7D295E70  srawi r9, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 82421C70: 7FC90194  addze r30, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[30].s64 = tmp.s64;
	// 82421C74: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82421C78: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82421C7C: 41980008  blt cr6, 0x82421c84
	if ctx.cr[6].lt {
	pc = 0x82421C84; continue 'dispatch;
	}
	// 82421C80: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82421C84: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82421C88: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82421C8C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82421C90: 41980008  blt cr6, 0x82421c98
	if ctx.cr[6].lt {
	pc = 0x82421C98; continue 'dispatch;
	}
	// 82421C94: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82421C98: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82421C9C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82421CA0: 41980008  blt cr6, 0x82421ca8
	if ctx.cr[6].lt {
	pc = 0x82421CA8; continue 'dispatch;
	}
	// 82421CA4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82421CA8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82421CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82421CB0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421CB4: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82421CB8: 4BFFDBF1  bl 0x8241f8a8
	ctx.lr = 0x82421CBC;
	sub_8241F8A8(ctx, base);
	// 82421CBC: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82421CC0: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82421CC4: 419A0020  beq cr6, 0x82421ce4
	if ctx.cr[6].eq {
	pc = 0x82421CE4; continue 'dispatch;
	}
	// 82421CC8: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82421CCC: 7D4A5E70  srawi r10, r10, 0xb
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 11) as i64;
	// 82421CD0: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82421CD4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82421CD8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82421CDC: 41980008  blt cr6, 0x82421ce4
	if ctx.cr[6].lt {
	pc = 0x82421CE4; continue 'dispatch;
	}
	// 82421CE0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82421CE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82421CE8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82421CEC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421CF0: 4BFFDC79  bl 0x8241f968
	ctx.lr = 0x82421CF4;
	sub_8241F968(ctx, base);
	// 82421CF4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82421CF8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82421CFC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82421D00: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82421D04: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82421D08: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82421D0C: 418100A8  bgt 0x82421db4
	if ctx.cr[0].gt {
	pc = 0x82421DB4; continue 'dispatch;
	}
	// 82421D10: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82421D14: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82421D18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421D1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82421D20: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82421D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82421D28: 4E800421  bctrl
	ctx.lr = 0x82421D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82421D2C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421D30: 939B0000  stw r28, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82421D34: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82421D38: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 82421D3C: 4BFFDDCD  bl 0x8241fb08
	ctx.lr = 0x82421D40;
	sub_8241FB08(ctx, base);
	// 82421D40: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82421D44: 409A0070  bne cr6, 0x82421db4
	if !ctx.cr[6].eq {
	pc = 0x82421DB4; continue 'dispatch;
	}
	// 82421D48: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421D4C: 4BFFE0E5  bl 0x8241fe30
	ctx.lr = 0x82421D50;
	sub_8241FE30(ctx, base);
	// 82421D50: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82421D54: 4182001C  beq 0x82421d70
	if ctx.cr[0].eq {
	pc = 0x82421D70; continue 'dispatch;
	}
	// 82421D58: 81788E14  lwz r11, -0x71ec(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-29164 as u32) ) } as u64;
	// 82421D5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82421D60: 41980018  blt cr6, 0x82421d78
	if ctx.cr[6].lt {
	pc = 0x82421D78; continue 'dispatch;
	}
	// 82421D64: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82421D68: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82421D6C: 4198000C  blt cr6, 0x82421d78
	if ctx.cr[6].lt {
	pc = 0x82421D78; continue 'dispatch;
	}
	// 82421D70: 9AFF0001  stb r23, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[23].u8 ) };
	// 82421D74: 48000040  b 0x82421db4
	pc = 0x82421DB4; continue 'dispatch;
	// 82421D78: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82421D7C: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82421D80: 40980034  bge cr6, 0x82421db4
	if !ctx.cr[6].lt {
	pc = 0x82421DB4; continue 'dispatch;
	}
	// 82421D84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82421D88: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82421D8C: 48000028  b 0x82421db4
	pc = 0x82421DB4; continue 'dispatch;
	// 82421D90: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82421D94: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 82421D98: 814B8E18  lwz r10, -0x71e8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29160 as u32) ) } as u64;
	// 82421D9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82421DA0: 914B8E18  stw r10, -0x71e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-29160 as u32), ctx.r[10].u32 ) };
	// 82421DA4: 48000010  b 0x82421db4
	pc = 0x82421DB4; continue 'dispatch;
	// 82421DA8: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 82421DAC: 48000008  b 0x82421db4
	pc = 0x82421DB4; continue 'dispatch;
	// 82421DB0: 4800D1A1  bl 0x8242ef50
	ctx.lr = 0x82421DB4;
	sub_8242EF50(ctx, base);
	// 82421DB4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82421DB8: 4811333C  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82421DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82421DC0 size=660
    let mut pc: u32 = 0x82421DC0;
    'dispatch: loop {
        match pc {
            0x82421DC0 => {
    //   block [0x82421DC0..0x82422054)
	// 82421DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82421DC4: 481132F5  bl 0x825350b8
	ctx.lr = 0x82421DC8;
	sub_82535080(ctx, base);
	// 82421DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82421DCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82421DD0: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82421DD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421DD8: 409A0248  bne cr6, 0x82422020
	if !ctx.cr[6].eq {
	pc = 0x82422020; continue 'dispatch;
	}
	// 82421DDC: 897F004C  lbz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82421DE0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82421DE4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82421DE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421DEC: 409A0018  bne cr6, 0x82421e04
	if !ctx.cr[6].eq {
	pc = 0x82421E04; continue 'dispatch;
	}
	// 82421DF0: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 82421DF4: 9B9F004C  stb r28, 0x4c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[28].u8 ) };
	// 82421DF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421DFC: 409A0008  bne cr6, 0x82421e04
	if !ctx.cr[6].eq {
	pc = 0x82421E04; continue 'dispatch;
	}
	// 82421E00: 9BBF0001  stb r29, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 82421E04: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 82421E08: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421E0C: 409A0028  bne cr6, 0x82421e34
	if !ctx.cr[6].eq {
	pc = 0x82421E34; continue 'dispatch;
	}
	// 82421E10: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421E14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82421E18: 4182000C  beq 0x82421e24
	if ctx.cr[0].eq {
	pc = 0x82421E24; continue 'dispatch;
	}
	// 82421E1C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82421E20: 4BFFD919  bl 0x8241f738
	ctx.lr = 0x82421E24;
	sub_8241F738(ctx, base);
	// 82421E24: 4800D12D  bl 0x8242ef50
	ctx.lr = 0x82421E28;
	sub_8242EF50(ctx, base);
	// 82421E28: 9B9F004A  stb r28, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[28].u8 ) };
	// 82421E2C: 9B9F004D  stb r28, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[28].u8 ) };
	// 82421E30: 4800D121  bl 0x8242ef50
	ctx.lr = 0x82421E34;
	sub_8242EF50(ctx, base);
	// 82421E34: 4800D11D  bl 0x8242ef50
	ctx.lr = 0x82421E38;
	sub_8242EF50(ctx, base);
	// 82421E38: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 82421E3C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82421E40: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421E44: 409A01C8  bne cr6, 0x8242200c
	if !ctx.cr[6].eq {
	pc = 0x8242200C; continue 'dispatch;
	}
	// 82421E48: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 82421E4C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421E50: 419A01FC  beq cr6, 0x8242204c
	if ctx.cr[6].eq {
	pc = 0x8242204C; continue 'dispatch;
	}
	// 82421E54: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 82421E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421E5C: 409A005C  bne cr6, 0x82421eb8
	if !ctx.cr[6].eq {
	pc = 0x82421EB8; continue 'dispatch;
	}
	// 82421E60: 9BBF004D  stb r29, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[29].u8 ) };
	// 82421E64: 4800D0ED  bl 0x8242ef50
	ctx.lr = 0x82421E68;
	sub_8242EF50(ctx, base);
	// 82421E68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421E6C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82421E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421E74: 409A0044  bne cr6, 0x82421eb8
	if !ctx.cr[6].eq {
	pc = 0x82421EB8; continue 'dispatch;
	}
	// 82421E78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82421E7C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82421E80: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82421E84: 4BFFE3ED  bl 0x82420270
	ctx.lr = 0x82421E88;
	sub_82420270(ctx, base);
	// 82421E88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82421E8C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82421E90: 40820028  bne 0x82421eb8
	if !ctx.cr[0].eq {
	pc = 0x82421EB8; continue 'dispatch;
	}
	// 82421E94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82421E98: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82421E9C: 386B17E0  addi r3, r11, 0x17e0
	ctx.r[3].s64 = ctx.r[11].s64 + 6112;
	// 82421EA0: 4BFFF481  bl 0x82421320
	ctx.lr = 0x82421EA4;
	sub_82421320(ctx, base);
	// 82421EA4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82421EA8: 9B9F004D  stb r28, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[28].u8 ) };
	// 82421EAC: 9B9F0049  stb r28, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[28].u8 ) };
	// 82421EB0: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82421EB4: 48000198  b 0x8242204c
	pc = 0x8242204C; continue 'dispatch;
	// 82421EB8: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 82421EBC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421EC0: 409A0150  bne cr6, 0x82422010
	if !ctx.cr[6].eq {
	pc = 0x82422010; continue 'dispatch;
	}
	// 82421EC4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82421EC8: 409A0008  bne cr6, 0x82421ed0
	if !ctx.cr[6].eq {
	pc = 0x82421ED0; continue 'dispatch;
	}
	// 82421ECC: 4800D085  bl 0x8242ef50
	ctx.lr = 0x82421ED0;
	sub_8242EF50(ctx, base);
	// 82421ED0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421ED4: 4BFFDCF5  bl 0x8241fbc8
	ctx.lr = 0x82421ED8;
	sub_8241FBC8(ctx, base);
	// 82421ED8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82421EDC: 418200F8  beq 0x82421fd4
	if ctx.cr[0].eq {
	pc = 0x82421FD4; continue 'dispatch;
	}
	// 82421EE0: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 82421EE4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421EE8: 409A0010  bne cr6, 0x82421ef8
	if !ctx.cr[6].eq {
	pc = 0x82421EF8; continue 'dispatch;
	}
	// 82421EEC: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 82421EF0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82421EF4: 419A0158  beq cr6, 0x8242204c
	if ctx.cr[6].eq {
	pc = 0x8242204C; continue 'dispatch;
	}
	// 82421EF8: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82421EFC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421F00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82421F04: 409A003C  bne cr6, 0x82421f40
	if !ctx.cr[6].eq {
	pc = 0x82421F40; continue 'dispatch;
	}
	// 82421F08: 4BFFDE59  bl 0x8241fd60
	ctx.lr = 0x82421F0C;
	sub_8241FD60(ctx, base);
	// 82421F0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82421F10: 2F3D0000  cmpdi cr6, r29, 0
	ctx.cr[6].compare_i64(ctx.r[29].s64, 0, &mut ctx.xer);
	// 82421F14: 40980010  bge cr6, 0x82421f24
	if !ctx.cr[6].lt {
	pc = 0x82421F24; continue 'dispatch;
	}
	// 82421F18: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82421F1C: 4BFFE4ED  bl 0x82420408
	ctx.lr = 0x82421F20;
	sub_82420408(ctx, base);
	// 82421F20: 7C7D07B4  extsw r29, r3
	ctx.r[29].s64 = ctx.r[3].s32 as i64;
	// 82421F24: 397D07FF  addi r11, r29, 0x7ff
	ctx.r[11].s64 = ctx.r[29].s64 + 2047;
	// 82421F28: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 82421F2C: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 82421F30: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82421F34: 7D6B5E74  sradi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 11;
	// 82421F38: 7D7E07B4  extsw r30, r11
	ctx.r[30].s64 = ctx.r[11].s32 as i64;
	// 82421F3C: 48000034  b 0x82421f70
	pc = 0x82421F70; continue 'dispatch;
	// 82421F40: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82421F44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421F48: 4BFFD961  bl 0x8241f8a8
	ctx.lr = 0x82421F4C;
	sub_8241F8A8(ctx, base);
	// 82421F4C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421F50: 4BFFD899  bl 0x8241f7e8
	ctx.lr = 0x82421F54;
	sub_8241F7E8(ctx, base);
	// 82421F54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82421F58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82421F5C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421F60: 57CB5828  slwi r11, r30, 0xb
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82421F64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421F68: 7D7D07B4  extsw r29, r11
	ctx.r[29].s64 = ctx.r[11].s32 as i64;
	// 82421F6C: 4BFFD93D  bl 0x8241f8a8
	ctx.lr = 0x82421F70;
	sub_8241F8A8(ctx, base);
	// 82421F70: 3940F800  li r10, -0x800
	ctx.r[10].s64 = -2048;
	// 82421F74: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82421F78: 794A0580  clrldi r10, r10, 0x16
	ctx.r[10].u64 = ctx.r[10].u64 & 0x000003FFFFFFFFFFu64;
	// 82421F7C: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 82421F80: 409A000C  bne cr6, 0x82421f8c
	if !ctx.cr[6].eq {
	pc = 0x82421F8C; continue 'dispatch;
	}
	// 82421F84: FBBF0010  std r29, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u64 ) };
	// 82421F88: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82421F8C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82421F90: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82421F94: 40990008  ble cr6, 0x82421f9c
	if !ctx.cr[6].gt {
	pc = 0x82421F9C; continue 'dispatch;
	}
	// 82421F98: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82421F9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82421FA0: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82421FA4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82421FA8: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82421FAC: 40990018  ble cr6, 0x82421fc4
	if !ctx.cr[6].gt {
	pc = 0x82421FC4; continue 'dispatch;
	}
	// 82421FB0: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82421FB4: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 82421FB8: 794A5D24  sldi r10, r10, 0xb
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(11);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 82421FBC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82421FC0: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82421FC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82421FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82421FCC: 4BFFF8F5  bl 0x824218c0
	ctx.lr = 0x82421FD0;
	sub_824218C0(ctx, base);
	// 82421FD0: 9B9F0049  stb r28, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[28].u8 ) };
	// 82421FD4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421FD8: 4BFFDB31  bl 0x8241fb08
	ctx.lr = 0x82421FDC;
	sub_8241FB08(ctx, base);
	// 82421FDC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82421FE0: 409A0030  bne cr6, 0x82422010
	if !ctx.cr[6].eq {
	pc = 0x82422010; continue 'dispatch;
	}
	// 82421FE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82421FE8: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82421FEC: 386B17B4  addi r3, r11, 0x17b4
	ctx.r[3].s64 = ctx.r[11].s64 + 6068;
	// 82421FF0: 4BFFF331  bl 0x82421320
	ctx.lr = 0x82421FF4;
	sub_82421320(ctx, base);
	// 82421FF4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82421FF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82421FFC: 4182FEA8  beq 0x82421ea4
	if ctx.cr[0].eq {
	pc = 0x82421EA4; continue 'dispatch;
	}
	// 82422000: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82422004: 4BFFD735  bl 0x8241f738
	ctx.lr = 0x82422008;
	sub_8241F738(ctx, base);
	// 82422008: 4BFFFE9C  b 0x82421ea4
	pc = 0x82421EA4; continue 'dispatch;
	// 8242200C: 4800CF45  bl 0x8242ef50
	ctx.lr = 0x82422010;
	sub_8242EF50(ctx, base);
	// 82422010: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 82422014: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82422018: 409A0008  bne cr6, 0x82422020
	if !ctx.cr[6].eq {
	pc = 0x82422020; continue 'dispatch;
	}
	// 8242201C: 9B9F004B  stb r28, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[28].u8 ) };
	// 82422020: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82422024: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82422028: 409A0024  bne cr6, 0x8242204c
	if !ctx.cr[6].eq {
	pc = 0x8242204C; continue 'dispatch;
	}
	// 8242202C: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 82422030: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82422034: 409A0018  bne cr6, 0x8242204c
	if !ctx.cr[6].eq {
	pc = 0x8242204C; continue 'dispatch;
	}
	// 82422038: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 8242203C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82422040: 409A000C  bne cr6, 0x8242204c
	if !ctx.cr[6].eq {
	pc = 0x8242204C; continue 'dispatch;
	}
	// 82422044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422048: 4BFFF9A9  bl 0x824219f0
	ctx.lr = 0x8242204C;
	sub_824219F0(ctx, base);
	// 8242204C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82422050: 481130B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422058 size=40
    let mut pc: u32 = 0x82422058;
    'dispatch: loop {
        match pc {
            0x82422058 => {
    //   block [0x82422058..0x82422080)
	// 82422058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242205C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422064: 4BFFEFCD  bl 0x82421030
	ctx.lr = 0x82422068;
	sub_82421030(ctx, base);
	// 82422068: 4BFFD9C1  bl 0x8241fa28
	ctx.lr = 0x8242206C;
	sub_8241FA28(ctx, base);
	// 8242206C: 4BFFF005  bl 0x82421070
	ctx.lr = 0x82422070;
	sub_82421070(ctx, base);
	// 82422070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82422074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242207C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422080 size=52
    let mut pc: u32 = 0x82422080;
    'dispatch: loop {
        match pc {
            0x82422080 => {
    //   block [0x82422080..0x824220B4)
	// 82422080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422084: 48113039  bl 0x825350bc
	ctx.lr = 0x82422088;
	sub_82535080(ctx, base);
	// 82422088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242208C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422090: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82422094: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82422098: 4BFFEF99  bl 0x82421030
	ctx.lr = 0x8242209C;
	sub_82421030(ctx, base);
	// 8242209C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 824220A0: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 824220A4: 4BFFEFCD  bl 0x82421070
	ctx.lr = 0x824220A8;
	sub_82421070(ctx, base);
	// 824220A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824220AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824220B0: 4811305C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824220B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824220B8 size=68
    let mut pc: u32 = 0x824220B8;
    'dispatch: loop {
        match pc {
            0x824220B8 => {
    //   block [0x824220B8..0x824220FC)
	// 824220B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824220BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824220C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824220C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824220C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824220CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824220D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824220D4: 4BFFEF5D  bl 0x82421030
	ctx.lr = 0x824220D8;
	sub_82421030(ctx, base);
	// 824220D8: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 824220DC: 4BFFEF95  bl 0x82421070
	ctx.lr = 0x824220E0;
	sub_82421070(ctx, base);
	// 824220E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824220E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824220E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824220EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824220F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824220F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824220F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422100 size=56
    let mut pc: u32 = 0x82422100;
    'dispatch: loop {
        match pc {
            0x82422100 => {
    //   block [0x82422100..0x82422138)
	// 82422100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242210C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422114: 4BFFEF1D  bl 0x82421030
	ctx.lr = 0x82422118;
	sub_82421030(ctx, base);
	// 82422118: EBFF0010  ld r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 8242211C: 4BFFEF55  bl 0x82421070
	ctx.lr = 0x82422120;
	sub_82421070(ctx, base);
	// 82422120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82422128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242212C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422130: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


