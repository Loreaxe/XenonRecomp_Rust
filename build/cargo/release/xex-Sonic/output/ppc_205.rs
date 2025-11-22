pub fn sub_82EA82E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA82E8 size=588
    let mut pc: u32 = 0x82EA82E8;
    'dispatch: loop {
        match pc {
            0x82EA82E8 => {
    //   block [0x82EA82E8..0x82EA8534)
	// 82EA82E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA82EC: 482FFE65  bl 0x831a8150
	ctx.lr = 0x82EA82F0;
	sub_831A8130(ctx, base);
	// 82EA82F0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA82F4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA82F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA82FC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EA8300: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA8304: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82EA8308: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82EA830C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8310: 48006D41  bl 0x82eaf050
	ctx.lr = 0x82EA8314;
	sub_82EAF050(ctx, base);
	// 82EA8314: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA8318: 809A000C  lwz r4, 0xc(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA831C: 48006FB5  bl 0x82eaf2d0
	ctx.lr = 0x82EA8320;
	sub_82EAF2D0(ctx, base);
	// 82EA8320: 813A0014  lwz r9, 0x14(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA8324: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EA8328: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA832C: 4099002C  ble cr6, 0x82ea8358
	if !ctx.cr[6].gt {
	pc = 0x82EA8358; continue 'dispatch;
	}
	// 82EA8330: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EA8334: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA8338: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EA833C: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EA8340: 480073D9  bl 0x82eaf718
	ctx.lr = 0x82EA8344;
	sub_82EAF718(ctx, base);
	// 82EA8344: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA8348: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EA834C: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 82EA8350: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA8354: 4198FFE0  blt cr6, 0x82ea8334
	if ctx.cr[6].lt {
	pc = 0x82EA8334; continue 'dispatch;
	}
	// 82EA8358: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA835C: 809A0014  lwz r4, 0x14(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA8360: 48006F71  bl 0x82eaf2d0
	ctx.lr = 0x82EA8364;
	sub_82EAF2D0(ctx, base);
	// 82EA8364: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA8368: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82EA836C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA8370: 409901A8  ble cr6, 0x82ea8518
	if !ctx.cr[6].gt {
	pc = 0x82EA8518; continue 'dispatch;
	}
	// 82EA8374: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82EA8378: 817A0018  lwz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA837C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82EA8380: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82EA8384: 7F6BC214  add r27, r11, r24
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82EA8388: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82EA838C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA8390: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8394: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA8398: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA839C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EA83A0: 4200FFF0  bdnz 0x82ea8390
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EA8390; continue 'dispatch;
	}
	// 82EA83A4: 8B21007C  lbz r25, 0x7c(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EA83A8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82EA83AC: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82EA83B0: 419A000C  beq cr6, 0x82ea83bc
	if ctx.cr[6].eq {
	pc = 0x82EA83BC; continue 'dispatch;
	}
	// 82EA83B4: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 82EA83B8: 409A0034  bne cr6, 0x82ea83ec
	if !ctx.cr[6].eq {
	pc = 0x82EA83EC; continue 'dispatch;
	}
	// 82EA83BC: 8861007D  lbz r3, 0x7d(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(125 as u32) ) } as u64;
	// 82EA83C0: 48005AE9  bl 0x82eadea8
	ctx.lr = 0x82EA83C4;
	sub_82EADEA8(ctx, base);
	// 82EA83C4: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA83C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EA83CC: A0E10080  lhz r7, 0x80(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EA83D0: 7D480734  extsh r8, r10
	ctx.r[8].s64 = ctx.r[10].s16 as i64;
	// 82EA83D4: 9BC1007D  stb r30, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[30].u8 ) };
	// 82EA83D8: 55061838  slwi r6, r8, 3
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EA83DC: 7CC53A78  xor r5, r6, r7
	ctx.r[5].u64 = ctx.r[6].u64 ^ ctx.r[7].u64;
	// 82EA83E0: 54BD043E  clrlwi r29, r5, 0x10
	ctx.r[29].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 82EA83E4: B3A10080  sth r29, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u16 ) };
	// 82EA83E8: 4800000C  b 0x82ea83f4
	pc = 0x82EA83F4; continue 'dispatch;
	// 82EA83EC: A3A10080  lhz r29, 0x80(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EA83F0: 8BC1007D  lbz r30, 0x7d(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(125 as u32) ) } as u64;
	// 82EA83F4: 57AA056A  rlwinm r10, r29, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA83F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EA83FC: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 82EA8400: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA8404: 419A0030  beq cr6, 0x82ea8434
	if ctx.cr[6].eq {
	pc = 0x82EA8434; continue 'dispatch;
	}
	// 82EA8408: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82EA840C: 57CA063E  clrlwi r10, r30, 0x18
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82EA8410: 697D0400  xori r29, r11, 0x400
	ctx.r[29].u64 = ctx.r[11].u64 ^ 1024;
	// 82EA8414: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA8418: B3A10080  sth r29, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u16 ) };
	// 82EA841C: 419A0008  beq cr6, 0x82ea8424
	if ctx.cr[6].eq {
	pc = 0x82EA8424; continue 'dispatch;
	}
	// 82EA8420: 57DC063E  clrlwi r28, r30, 0x18
	ctx.r[28].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82EA8424: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82EA8428: 3B200013  li r25, 0x13
	ctx.r[25].s64 = 19;
	// 82EA842C: 9BC1007D  stb r30, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[30].u8 ) };
	// 82EA8430: 9B21007C  stb r25, 0x7c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[25].u8 ) };
	// 82EA8434: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EA8438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA843C: 419A0048  beq cr6, 0x82ea8484
	if ctx.cr[6].eq {
	pc = 0x82EA8484; continue 'dispatch;
	}
	// 82EA8440: 897B000C  lbz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA8444: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82EA8448: 419A003C  beq cr6, 0x82ea8484
	if ctx.cr[6].eq {
	pc = 0x82EA8484; continue 'dispatch;
	}
	// 82EA844C: 897B000D  lbz r11, 0xd(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(13 as u32) ) } as u64;
	// 82EA8450: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82EA8454: 419A0030  beq cr6, 0x82ea8484
	if ctx.cr[6].eq {
	pc = 0x82EA8484; continue 'dispatch;
	}
	// 82EA8458: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EA845C: 48005A75  bl 0x82eaded0
	ctx.lr = 0x82EA8460;
	sub_82EADED0(ctx, base);
	// 82EA8460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8464: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA8468: 419A001C  beq cr6, 0x82ea8484
	if ctx.cr[6].eq {
	pc = 0x82EA8484; continue 'dispatch;
	}
	// 82EA846C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EA8470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA8474: 4BFFFE75  bl 0x82ea82e8
	ctx.lr = 0x82EA8478;
	sub_82EA82E8(ctx, base);
	// 82EA8478: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA847C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA8480: 409AFFEC  bne cr6, 0x82ea846c
	if !ctx.cr[6].eq {
	pc = 0x82EA846C; continue 'dispatch;
	}
	// 82EA8484: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EA8488: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA848C: 419A0014  beq cr6, 0x82ea84a0
	if ctx.cr[6].eq {
	pc = 0x82EA84A0; continue 'dispatch;
	}
	// 82EA8490: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EA8494: 48005A45  bl 0x82eaded8
	ctx.lr = 0x82EA8498;
	sub_82EADED8(ctx, base);
	// 82EA8498: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EA849C: 4800727D  bl 0x82eaf718
	ctx.lr = 0x82EA84A0;
	sub_82EAF718(ctx, base);
	// 82EA84A0: 83E10070  lwz r31, 0x70(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EA84A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA84A8: 48002199  bl 0x82eaa640
	ctx.lr = 0x82EA84AC;
	sub_82EAA640(ctx, base);
	// 82EA84AC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA84B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA84B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA84B8: 48006A91  bl 0x82eaef48
	ctx.lr = 0x82EA84BC;
	sub_82EAEF48(ctx, base);
	// 82EA84BC: 5724063E  clrlwi r4, r25, 0x18
	ctx.r[4].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 82EA84C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA84C4: 48006DAD  bl 0x82eaf270
	ctx.lr = 0x82EA84C8;
	sub_82EAF270(ctx, base);
	// 82EA84C8: 57C4063E  clrlwi r4, r30, 0x18
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82EA84CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA84D0: 48006DA1  bl 0x82eaf270
	ctx.lr = 0x82EA84D4;
	sub_82EAF270(ctx, base);
	// 82EA84D4: 7F8B0734  extsh r11, r28
	ctx.r[11].s64 = ctx.r[28].s16 as i64;
	// 82EA84D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA84DC: 419A0010  beq cr6, 0x82ea84ec
	if ctx.cr[6].eq {
	pc = 0x82EA84EC; continue 'dispatch;
	}
	// 82EA84E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA84E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA84E8: 48006D89  bl 0x82eaf270
	ctx.lr = 0x82EA84EC;
	sub_82EAF270(ctx, base);
	// 82EA84EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA84F0: A081007E  lhz r4, 0x7e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(126 as u32) ) } as u64;
	// 82EA84F4: 48006D7D  bl 0x82eaf270
	ctx.lr = 0x82EA84F8;
	sub_82EAF270(ctx, base);
	// 82EA84F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA84FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA8500: 48006D71  bl 0x82eaf270
	ctx.lr = 0x82EA8504;
	sub_82EAF270(ctx, base);
	// 82EA8504: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA8508: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82EA850C: 3B180018  addi r24, r24, 0x18
	ctx.r[24].s64 = ctx.r[24].s64 + 24;
	// 82EA8510: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA8514: 4198FE64  blt cr6, 0x82ea8378
	if ctx.cr[6].lt {
	pc = 0x82EA8378; continue 'dispatch;
	}
	// 82EA8518: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA851C: 809A001C  lwz r4, 0x1c(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA8520: 48006DB1  bl 0x82eaf2d0
	ctx.lr = 0x82EA8524;
	sub_82EAF2D0(ctx, base);
	// 82EA8524: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA8528: 48006CC9  bl 0x82eaf1f0
	ctx.lr = 0x82EA852C;
	sub_82EAF1F0(ctx, base);
	// 82EA852C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82EA8530: 482FFC70  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8538 size=136
    let mut pc: u32 = 0x82EA8538;
    'dispatch: loop {
        match pc {
            0x82EA8538 => {
    //   block [0x82EA8538..0x82EA85C0)
	// 82EA8538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8540: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA8544: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA8548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA854C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8550: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA8554: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA8558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA855C: 480074B5  bl 0x82eafa10
	ctx.lr = 0x82EA8560;
	sub_82EAFA10(ctx, base);
	// 82EA8560: 7FCBF0F8  nor r11, r30, r30
	ctx.r[11].u64 = !(ctx.r[30].u64 | ctx.r[30].u64);
	// 82EA8564: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA8568: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82EA856C: 419A0034  beq cr6, 0x82ea85a0
	if ctx.cr[6].eq {
	pc = 0x82EA85A0; continue 'dispatch;
	}
	// 82EA8570: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EA8574: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA8578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA857C: 4BFFFD6D  bl 0x82ea82e8
	ctx.lr = 0x82EA8580;
	sub_82EA82E8(ctx, base);
	// 82EA8580: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA8584: 419A000C  beq cr6, 0x82ea8590
	if ctx.cr[6].eq {
	pc = 0x82EA8590; continue 'dispatch;
	}
	// 82EA8588: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA858C: 48000008  b 0x82ea8594
	pc = 0x82EA8594; continue 'dispatch;
	// 82EA8590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA8594: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82EA8598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA859C: 409AFFD8  bne cr6, 0x82ea8574
	if !ctx.cr[6].eq {
	pc = 0x82EA8574; continue 'dispatch;
	}
	// 82EA85A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA85A4: 4800745D  bl 0x82eafa00
	ctx.lr = 0x82EA85A8;
	sub_82EAFA00(ctx, base);
	// 82EA85A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA85AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA85B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA85B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA85B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA85BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA85C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA85C0 size=92
    let mut pc: u32 = 0x82EA85C0;
    'dispatch: loop {
        match pc {
            0x82EA85C0 => {
    //   block [0x82EA85C0..0x82EA861C)
	// 82EA85C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA85C4: 482FFBA9  bl 0x831a816c
	ctx.lr = 0x82EA85C8;
	sub_831A8130(ctx, base);
	// 82EA85C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA85CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA85D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA85D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA85D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA85DC: 480058CD  bl 0x82eadea8
	ctx.lr = 0x82EA85E0;
	sub_82EADEA8(ctx, base);
	// 82EA85E0: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA85E4: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 82EA85E8: 41990028  bgt cr6, 0x82ea8610
	if ctx.cr[6].gt {
	pc = 0x82EA8610; continue 'dispatch;
	}
	// 82EA85EC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EA85F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA85F4: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA85F8: 7D650734  extsh r5, r11
	ctx.r[5].s64 = ctx.r[11].s16 as i64;
	// 82EA85FC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82EA8600: 48002149  bl 0x82eaa748
	ctx.lr = 0x82EA8604;
	sub_82EAA748(ctx, base);
	// 82EA8604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA8608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA860C: 482FFBB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EA8610: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA8614: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA8618: 482FFBA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8620 size=36
    let mut pc: u32 = 0x82EA8620;
    'dispatch: loop {
        match pc {
            0x82EA8620 => {
    //   block [0x82EA8620..0x82EA8644)
	// 82EA8620: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA8624: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA8628: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EA862C: 7D0A4850  subf r8, r10, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82EA8630: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82EA8634: 54E6DFFE  rlwinm r6, r7, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82EA8638: 68C50001  xori r5, r6, 1
	ctx.r[5].u64 = ctx.r[6].u64 ^ 1;
	// 82EA863C: 98A30000  stb r5, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 82EA8640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8648 size=24
    let mut pc: u32 = 0x82EA8648;
    'dispatch: loop {
        match pc {
            0x82EA8648 => {
    //   block [0x82EA8648..0x82EA8660)
	// 82EA8648: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA864C: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EA8650: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EA8654: 69280001  xori r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 ^ 1;
	// 82EA8658: 99030000  stb r8, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82EA865C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8660 size=20
    let mut pc: u32 = 0x82EA8660;
    'dispatch: loop {
        match pc {
            0x82EA8660 => {
    //   block [0x82EA8660..0x82EA8674)
	// 82EA8660: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA8664: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA8668: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA866C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EA8670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8678 size=24
    let mut pc: u32 = 0x82EA8678;
    'dispatch: loop {
        match pc {
            0x82EA8678 => {
    //   block [0x82EA8678..0x82EA8690)
	// 82EA8678: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA867C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA8680: 41980010  blt cr6, 0x82ea8690
	if ctx.cr[6].lt {
		sub_82EA8690(ctx, base);
		return;
	}
	// 82EA8684: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA8688: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8690 size=8
    let mut pc: u32 = 0x82EA8690;
    'dispatch: loop {
        match pc {
            0x82EA8690 => {
    //   block [0x82EA8690..0x82EA8698)
	// 82EA8690: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA8694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8698 size=12
    let mut pc: u32 = 0x82EA8698;
    'dispatch: loop {
        match pc {
            0x82EA8698 => {
    //   block [0x82EA8698..0x82EA86A4)
	// 82EA8698: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA869C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA86A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA86A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA86A8 size=40
    let mut pc: u32 = 0x82EA86A8;
    'dispatch: loop {
        match pc {
            0x82EA86A8 => {
    //   block [0x82EA86A8..0x82EA86D0)
	// 82EA86A8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EA86AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA86B0: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82EA86B4: 41980028  blt cr6, 0x82ea86dc
	if ctx.cr[6].lt {
		sub_82EA86DC(ctx, base);
		return;
	}
	// 82EA86B8: 419A0018  beq cr6, 0x82ea86d0
	if ctx.cr[6].eq {
		sub_82EA86D0(ctx, base);
		return;
	}
	// 82EA86BC: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 82EA86C0: 40980020  bge cr6, 0x82ea86e0
	if !ctx.cr[6].lt {
		sub_82EA86DC(ctx, base);
		return;
	}
	// 82EA86C4: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA86C8: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82EA86CC: 48000014  b 0x82ea86e0
	sub_82EA86DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA86D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA86D0 size=12
    let mut pc: u32 = 0x82EA86D0;
    'dispatch: loop {
        match pc {
            0x82EA86D0 => {
    //   block [0x82EA86D0..0x82EA86DC)
	// 82EA86D0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA86D4: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EA86D8: 48000008  b 0x82ea86e0
	sub_82EA86DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA86DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA86DC size=32
    let mut pc: u32 = 0x82EA86DC;
    'dispatch: loop {
        match pc {
            0x82EA86DC => {
    //   block [0x82EA86DC..0x82EA86FC)
	// 82EA86DC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA86E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA86E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA86E8: 40980014  bge cr6, 0x82ea86fc
	if !ctx.cr[6].lt {
		sub_82EA86FC(ctx, base);
		return;
	}
	// 82EA86EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA86F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA86F4: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA86F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA86FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA86FC size=24
    let mut pc: u32 = 0x82EA86FC;
    'dispatch: loop {
        match pc {
            0x82EA86FC => {
    //   block [0x82EA86FC..0x82EA8714)
	// 82EA86FC: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA8700: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EA8704: 4099FFF0  ble cr6, 0x82ea86f4
	if !ctx.cr[6].gt {
		sub_82EA86DC(ctx, base);
		return;
	}
	// 82EA8708: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA870C: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82EA8710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8718 size=8
    let mut pc: u32 = 0x82EA8718;
    'dispatch: loop {
        match pc {
            0x82EA8718 => {
    //   block [0x82EA8718..0x82EA8720)
	// 82EA8718: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA871C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8720 size=168
    let mut pc: u32 = 0x82EA8720;
    'dispatch: loop {
        match pc {
            0x82EA8720 => {
    //   block [0x82EA8720..0x82EA87C8)
	// 82EA8720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA8724: 482FFA45  bl 0x831a8168
	ctx.lr = 0x82EA8728;
	sub_831A8130(ctx, base);
	// 82EA8728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA872C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8730: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA8734: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA8738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA873C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA8740: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8744: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA8748: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA874C: 4E800421  bctrl
	ctx.lr = 0x82EA8750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA8750: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8754: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA8758: 419A0064  beq cr6, 0x82ea87bc
	if ctx.cr[6].eq {
	pc = 0x82EA87BC; continue 'dispatch;
	}
	// 82EA875C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA8760: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA8764: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EA8768: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EA876C: 40980008  bge cr6, 0x82ea8774
	if !ctx.cr[6].lt {
	pc = 0x82EA8774; continue 'dispatch;
	}
	// 82EA8770: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82EA8774: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA8778: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA877C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA8780: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA8784: 48001FC5  bl 0x82eaa748
	ctx.lr = 0x82EA8788;
	sub_82EAA748(ctx, base);
	// 82EA8788: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA878C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA8790: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA8794: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA8798: 409A0018  bne cr6, 0x82ea87b0
	if !ctx.cr[6].eq {
	pc = 0x82EA87B0; continue 'dispatch;
	}
	// 82EA879C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EA87A0: 419A0010  beq cr6, 0x82ea87b0
	if ctx.cr[6].eq {
	pc = 0x82EA87B0; continue 'dispatch;
	}
	// 82EA87A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA87A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA87AC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA87B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA87B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA87B8: 482FFA00  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA87BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA87C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA87C4: 482FF9F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA87C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA87C8 size=156
    let mut pc: u32 = 0x82EA87C8;
    'dispatch: loop {
        match pc {
            0x82EA87C8 => {
    //   block [0x82EA87C8..0x82EA8864)
	// 82EA87C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA87CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA87D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA87D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA87D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA87DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA87E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA87E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA87E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA87EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA87F0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA87F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA87F8: 4E800421  bctrl
	ctx.lr = 0x82EA87FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA87FC: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8800: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA8804: 419A0044  beq cr6, 0x82ea8848
	if ctx.cr[6].eq {
	pc = 0x82EA8848; continue 'dispatch;
	}
	// 82EA8808: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA880C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA8810: 7D6A4850  subf r11, r10, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82EA8814: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA8818: 40980008  bge cr6, 0x82ea8820
	if !ctx.cr[6].lt {
	pc = 0x82EA8820; continue 'dispatch;
	}
	// 82EA881C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EA8820: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EA8824: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA8828: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EA882C: 409A0014  bne cr6, 0x82ea8840
	if !ctx.cr[6].eq {
	pc = 0x82EA8840; continue 'dispatch;
	}
	// 82EA8830: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA8834: 419A000C  beq cr6, 0x82ea8840
	if ctx.cr[6].eq {
	pc = 0x82EA8840; continue 'dispatch;
	}
	// 82EA8838: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 82EA883C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EA8840: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA8844: 48000008  b 0x82ea884c
	pc = 0x82EA884C; continue 'dispatch;
	// 82EA8848: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA884C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA8850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA8854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA8858: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA885C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA8860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8868 size=144
    let mut pc: u32 = 0x82EA8868;
    'dispatch: loop {
        match pc {
            0x82EA8868 => {
    //   block [0x82EA8868..0x82EA88F8)
	// 82EA8868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA886C: 482FF901  bl 0x831a816c
	ctx.lr = 0x82EA8870;
	sub_831A8130(ctx, base);
	// 82EA8870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8878: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA887C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA8880: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA8884: 392B082C  addi r9, r11, 0x82c
	ctx.r[9].s64 = ctx.r[11].s64 + 2092;
	// 82EA8888: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EA888C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EA8890: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82EA8894: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82EA8898: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA889C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EA88A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA88A4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82EA88A8: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82EA88AC: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82EA88B0: 409A0038  bne cr6, 0x82ea88e8
	if !ctx.cr[6].eq {
	pc = 0x82EA88E8; continue 'dispatch;
	}
	// 82EA88B4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA88B8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA88BC: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82EA88C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA88C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA88C8: 4BFF8099  bl 0x82ea0960
	ctx.lr = 0x82EA88CC;
	sub_82EA0960(ctx, base);
	// 82EA88CC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EA88D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA88D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA88D8: 48001E71  bl 0x82eaa748
	ctx.lr = 0x82EA88DC;
	sub_82EAA748(ctx, base);
	// 82EA88DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA88E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA88E4: 482FF8D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EA88E8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EA88EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA88F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA88F4: 482FF8C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA88F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA88F8 size=104
    let mut pc: u32 = 0x82EA88F8;
    'dispatch: loop {
        match pc {
            0x82EA88F8 => {
    //   block [0x82EA88F8..0x82EA8960)
	// 82EA88F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA88FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA8904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA890C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA8910: 392A082C  addi r9, r10, 0x82c
	ctx.r[9].s64 = ctx.r[10].s64 + 2092;
	// 82EA8914: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA8918: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA891C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA8920: 419A000C  beq cr6, 0x82ea892c
	if ctx.cr[6].eq {
	pc = 0x82EA892C; continue 'dispatch;
	}
	// 82EA8924: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EA8928: 409A0018  bne cr6, 0x82ea8940
	if !ctx.cr[6].eq {
	pc = 0x82EA8940; continue 'dispatch;
	}
	// 82EA892C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8930: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA8934: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA8938: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA893C: 4BFF807D  bl 0x82ea09b8
	ctx.lr = 0x82EA8940;
	sub_82EA09B8(ctx, base);
	// 82EA8940: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA8944: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EA8948: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA894C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA8950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA8954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA8958: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA895C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8960 size=160
    let mut pc: u32 = 0x82EA8960;
    'dispatch: loop {
        match pc {
            0x82EA8960 => {
    //   block [0x82EA8960..0x82EA8A00)
	// 82EA8960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA8964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA896C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA8970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8978: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA897C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA8980: 392A082C  addi r9, r10, 0x82c
	ctx.r[9].s64 = ctx.r[10].s64 + 2092;
	// 82EA8984: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA8988: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA898C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA8990: 419A000C  beq cr6, 0x82ea899c
	if ctx.cr[6].eq {
	pc = 0x82EA899C; continue 'dispatch;
	}
	// 82EA8994: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EA8998: 409A0018  bne cr6, 0x82ea89b0
	if !ctx.cr[6].eq {
	pc = 0x82EA89B0; continue 'dispatch;
	}
	// 82EA899C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA89A0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA89A4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA89A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA89AC: 4BFF800D  bl 0x82ea09b8
	ctx.lr = 0x82EA89B0;
	sub_82EA09B8(ctx, base);
	// 82EA89B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA89B4: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA89B8: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EA89BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA89C0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA89C4: 419A0020  beq cr6, 0x82ea89e4
	if ctx.cr[6].eq {
	pc = 0x82EA89E4; continue 'dispatch;
	}
	// 82EA89C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA89CC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA89D0: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EA89D4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA89D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA89DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA89E0: 4BFF7DD1  bl 0x82ea07b0
	ctx.lr = 0x82EA89E4;
	sub_82EA07B0(ctx, base);
	// 82EA89E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA89E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA89EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA89F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA89F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA89F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA89FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA8A00 size=320
    let mut pc: u32 = 0x82EA8A00;
    'dispatch: loop {
        match pc {
            0x82EA8A00 => {
    //   block [0x82EA8A00..0x82EA8B40)
	// 82EA8A00: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EA8A04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA8A08: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 82EA8A0C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EA8A10: C00A08A8  lfs f0, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA8A14: 419A006C  beq cr6, 0x82ea8a80
	if ctx.cr[6].eq {
	pc = 0x82EA8A80; continue 'dispatch;
	}
	// 82EA8A18: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82EA8A1C: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82EA8A20: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82EA8A24: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82EA8A28: 38C7FFA0  addi r6, r7, -0x60
	ctx.r[6].s64 = ctx.r[7].s64 + -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8B40 size=80
    let mut pc: u32 = 0x82EA8B40;
    'dispatch: loop {
        match pc {
            0x82EA8B40 => {
    //   block [0x82EA8B40..0x82EA8B90)
	// 82EA8B40: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8B90 size=92
    let mut pc: u32 = 0x82EA8B90;
    'dispatch: loop {
        match pc {
            0x82EA8B90 => {
    //   block [0x82EA8B90..0x82EA8BEC)
	// 82EA8B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA8B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8B98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA8B9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA8BA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8BA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8BA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA8BAC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82EA8BB0: 4BFFE8A1  bl 0x82ea7450
	ctx.lr = 0x82EA8BB4;
	sub_82EA7450(ctx, base);
	// 82EA8BB4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA8BB8: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82EA8BBC: 392BFFA0  addi r9, r11, -0x60
	ctx.r[9].s64 = ctx.r[11].s64 + -96;
	// 82EA8BC0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8BF0 size=76
    let mut pc: u32 = 0x82EA8BF0;
    'dispatch: loop {
        match pc {
            0x82EA8BF0 => {
    //   block [0x82EA8BF0..0x82EA8C3C)
	// 82EA8BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA8BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8BF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA8BFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA8C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8C08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA8C0C: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EA8C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA8C14: 4BFF7E6D  bl 0x82ea0a80
	ctx.lr = 0x82EA8C18;
	sub_82EA0A80(ctx, base);
	// 82EA8C18: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA8C40 size=80
    let mut pc: u32 = 0x82EA8C40;
    'dispatch: loop {
        match pc {
            0x82EA8C40 => {
    //   block [0x82EA8C40..0x82EA8C90)
	// 82EA8C40: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82EA8C44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA8C48: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82EA8C4C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82EA8C50: 419A0024  beq cr6, 0x82ea8c74
	if ctx.cr[6].eq {
	pc = 0x82EA8C74; continue 'dispatch;
	}
	// 82EA8C54: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 82EA8C58: 419A001C  beq cr6, 0x82ea8c74
	if ctx.cr[6].eq {
	pc = 0x82EA8C74; continue 'dispatch;
	}
	// 82EA8C5C: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA8C60: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA8C64: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA8C68: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA8C6C: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA8C70: 419A0020  beq cr6, 0x82ea8c90
	if ctx.cr[6].eq {
		sub_82EA8C90(ctx, base);
		return;
	}
	// 82EA8C74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA8C78: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EA8C7C: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82EA8C80: 4198FFCC  blt cr6, 0x82ea8c4c
	if ctx.cr[6].lt {
	pc = 0x82EA8C4C; continue 'dispatch;
	}
	// 82EA8C84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA8C88: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA8C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8C90 size=12
    let mut pc: u32 = 0x82EA8C90;
    'dispatch: loop {
        match pc {
            0x82EA8C90 => {
    //   block [0x82EA8C90..0x82EA8C9C)
	// 82EA8C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA8C94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA8C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8CA0 size=92
    let mut pc: u32 = 0x82EA8CA0;
    'dispatch: loop {
        match pc {
            0x82EA8CA0 => {
    //   block [0x82EA8CA0..0x82EA8CFC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8CFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA8CFC size=196
    let mut pc: u32 = 0x82EA8CFC;
    'dispatch: loop {
        match pc {
            0x82EA8CFC => {
    //   block [0x82EA8CFC..0x82EA8DC0)
	// 82EA8CFC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82EA8D00: D021FFE0  stfs f1, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82EA8D04: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EA8DC0 size=212
    let mut pc: u32 = 0x82EA8DC0;
    'dispatch: loop {
        match pc {
            0x82EA8DC0 => {
    //   block [0x82EA8DC0..0x82EA8E94)
	// 82EA8DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA8DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA8DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA8DD0: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8DD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8DD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA8DDC: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EA8DE0: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82EA8DE4: 4BFF7C9D  bl 0x82ea0a80
	ctx.lr = 0x82EA8DE8;
	sub_82EA0A80(ctx, base);
	// 82EA8DE8: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 82EA8DEC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EA8DF0: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA8DF4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82EA8DF8: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA8DFC: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82EA8E00: C19F0028  lfs f12, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA8E04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8E98 size=132
    let mut pc: u32 = 0x82EA8E98;
    'dispatch: loop {
        match pc {
            0x82EA8E98 => {
    //   block [0x82EA8E98..0x82EA8F1C)
	// 82EA8E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA8E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8EA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA8EA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA8EA8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8EAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA8EB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA8EB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA8EB8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82EA8EBC: 48006CE5  bl 0x82eafba0
	ctx.lr = 0x82EA8EC0;
	sub_82EAFBA0(ctx, base);
	// 82EA8EC0: 88C10081  lbz r6, 0x81(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(129 as u32) ) } as u64;
	// 82EA8EC4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA8EC8: 7CC50774  extsb r5, r6
	ctx.r[5].s64 = ctx.r[6].s8 as i64;
	// 82EA8ECC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82EA8ED0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82EA8ED4: 7CA40034  cntlzw r4, r5
	ctx.r[4].u64 = if ctx.r[5].u32 == 0 { 32 } else { ctx.r[5].u32.leading_zeros() as u64 };
	// 82EA8ED8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8F20 size=44
    let mut pc: u32 = 0x82EA8F20;
    'dispatch: loop {
        match pc {
            0x82EA8F20 => {
    //   block [0x82EA8F20..0x82EA8F4C)
	// 82EA8F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA8F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA8F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8F2C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EA8F30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EA8F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA8F38: 4BFFFF61  bl 0x82ea8e98
	ctx.lr = 0x82EA8F3C;
	sub_82EA8E98(ctx, base);
	// 82EA8F3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA8F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA8F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA8F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8F50 size=4
    let mut pc: u32 = 0x82EA8F50;
    'dispatch: loop {
        match pc {
            0x82EA8F50 => {
    //   block [0x82EA8F50..0x82EA8F54)
	// 82EA8F50: 4BFFFE70  b 0x82ea8dc0
	sub_82EA8DC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA8F58 size=64
    let mut pc: u32 = 0x82EA8F58;
    'dispatch: loop {
        match pc {
            0x82EA8F58 => {
    //   block [0x82EA8F58..0x82EA8F98)
	// 82EA8F58: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA8F5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA8F60: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82EA8F64: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA8F68: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA8F6C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA8F70: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA8F74: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA8F78: 419A0020  beq cr6, 0x82ea8f98
	if ctx.cr[6].eq {
		sub_82EA8F98(ctx, base);
		return;
	}
	// 82EA8F7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA8F80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA8F84: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 82EA8F88: 4198FFDC  blt cr6, 0x82ea8f64
	if ctx.cr[6].lt {
	pc = 0x82EA8F64; continue 'dispatch;
	}
	// 82EA8F8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA8F90: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA8F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8F98 size=12
    let mut pc: u32 = 0x82EA8F98;
    'dispatch: loop {
        match pc {
            0x82EA8F98 => {
    //   block [0x82EA8F98..0x82EA8FA4)
	// 82EA8F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA8F9C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA8FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8FA8 size=12
    let mut pc: u32 = 0x82EA8FA8;
    'dispatch: loop {
        match pc {
            0x82EA8FA8 => {
    //   block [0x82EA8FA8..0x82EA8FB4)
	// 82EA8FA8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA8FAC: 386B69F0  addi r3, r11, 0x69f0
	ctx.r[3].s64 = ctx.r[11].s64 + 27120;
	// 82EA8FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA8FB8 size=52
    let mut pc: u32 = 0x82EA8FB8;
    'dispatch: loop {
        match pc {
            0x82EA8FB8 => {
    //   block [0x82EA8FB8..0x82EA8FEC)
	// 82EA8FB8: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA8FBC: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA8FC0: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EA8FC4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EA8FC8: C1830020  lfs f12, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA8FCC: C1630008  lfs f11, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EA8FD0: D1630020  stfs f11, 0x20(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82EA8FD4: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EA8FD8: C1430024  lfs f10, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EA8FDC: C1230018  lfs f9, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EA8FE0: D1230024  stfs f9, 0x24(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82EA8FE4: D1430018  stfs f10, 0x18(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EA8FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA8FF0 size=96
    let mut pc: u32 = 0x82EA8FF0;
    'dispatch: loop {
        match pc {
            0x82EA8FF0 => {
    //   block [0x82EA8FF0..0x82EA9050)
	// 82EA8FF0: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA8FF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA8FF8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EA8FFC: C1A40014  lfs f13, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA9000: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EA9004: C1840028  lfs f12, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA9008: D1830028  stfs f12, 0x28(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82EA900C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA9010: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EA9014: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82EA9018: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82EA901C: C1640010  lfs f11, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EA9020: D1630004  stfs f11, 4(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EA9024: C1440004  lfs f10, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EA9028: D1430010  stfs f10, 0x10(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EA902C: C1240020  lfs f9, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EA9030: D1230008  stfs f9, 8(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EA9034: C1040008  lfs f8, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82EA9038: D1030020  stfs f8, 0x20(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82EA903C: C0E40024  lfs f7, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82EA9040: D0E30018  stfs f7, 0x18(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EA9044: C0C40018  lfs f6, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82EA9048: D0C30024  stfs f6, 0x24(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82EA904C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9050 size=60
    let mut pc: u32 = 0x82EA9050;
    'dispatch: loop {
        match pc {
            0x82EA9050 => {
    //   block [0x82EA9050..0x82EA908C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9090 size=80
    let mut pc: u32 = 0x82EA9090;
    'dispatch: loop {
        match pc {
            0x82EA9090 => {
    //   block [0x82EA9090..0x82EA90E0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA90E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA90E0 size=200
    let mut pc: u32 = 0x82EA90E0;
    'dispatch: loop {
        match pc {
            0x82EA90E0 => {
    //   block [0x82EA90E0..0x82EA91A8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA91A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA91A8 size=140
    let mut pc: u32 = 0x82EA91A8;
    'dispatch: loop {
        match pc {
            0x82EA91A8 => {
    //   block [0x82EA91A8..0x82EA9234)
	// 82EA91A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA91AC: C0040004  lfs f0, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA91B0: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82EA91B4: FD800050  fneg f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82EA91B8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82EA91BC: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA91C0: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 82EA91C4: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82EA91C8: D181FFF8  stfs f12, -8(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82EA91CC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EA91D0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA91D4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82EA91D8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA91DC: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9238 size=284
    let mut pc: u32 = 0x82EA9238;
    'dispatch: loop {
        match pc {
            0x82EA9238 => {
    //   block [0x82EA9238..0x82EA9354)
	// 82EA9238: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA923C: EC010072  fmuls f0, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82EA9240: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 82EA9244: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 82EA9248: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9354(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9354 size=8
    let mut pc: u32 = 0x82EA9354;
    'dispatch: loop {
        match pc {
            0x82EA9354 => {
    //   block [0x82EA9354..0x82EA935C)
	// 82EA9354: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA9358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9360 size=196
    let mut pc: u32 = 0x82EA9360;
    'dispatch: loop {
        match pc {
            0x82EA9360 => {
    //   block [0x82EA9360..0x82EA9424)
	// 82EA9360: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9428 size=68
    let mut pc: u32 = 0x82EA9428;
    'dispatch: loop {
        match pc {
            0x82EA9428 => {
    //   block [0x82EA9428..0x82EA946C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9470 size=68
    let mut pc: u32 = 0x82EA9470;
    'dispatch: loop {
        match pc {
            0x82EA9470 => {
    //   block [0x82EA9470..0x82EA94B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA94B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA94B8 size=60
    let mut pc: u32 = 0x82EA94B8;
    'dispatch: loop {
        match pc {
            0x82EA94B8 => {
    //   block [0x82EA94B8..0x82EA94F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA94F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA94F8 size=80
    let mut pc: u32 = 0x82EA94F8;
    'dispatch: loop {
        match pc {
            0x82EA94F8 => {
    //   block [0x82EA94F8..0x82EA9548)
	// 82EA94F8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9548 size=4
    let mut pc: u32 = 0x82EA9548;
    'dispatch: loop {
        match pc {
            0x82EA9548 => {
    //   block [0x82EA9548..0x82EA954C)
	// 82EA9548: 4BFFFE18  b 0x82ea9360
	sub_82EA9360(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9550 size=80
    let mut pc: u32 = 0x82EA9550;
    'dispatch: loop {
        match pc {
            0x82EA9550 => {
    //   block [0x82EA9550..0x82EA95A0)
	// 82EA9550: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA95A0 size=176
    let mut pc: u32 = 0x82EA95A0;
    'dispatch: loop {
        match pc {
            0x82EA95A0 => {
    //   block [0x82EA95A0..0x82EA9650)
	// 82EA95A0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EA95A4: C1A50000  lfs f13, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA95A8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EA95AC: C1850014  lfs f12, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA95B0: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82EA95B4: C1650028  lfs f11, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EA95B8: C1450010  lfs f10, 0x10(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EA95BC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EA95C0: C1250004  lfs f9, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EA95C4: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82EA95C8: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA95CC: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82EA95D0: C1050020  lfs f8, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82EA95D4: C0E50008  lfs f7, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82EA95D8: C0C50024  lfs f6, 0x24(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82EA95DC: C0A50018  lfs f5, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9650 size=92
    let mut pc: u32 = 0x82EA9650;
    'dispatch: loop {
        match pc {
            0x82EA9650 => {
    //   block [0x82EA9650..0x82EA96AC)
	// 82EA9650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA9658: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA965C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82EA9660: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EA9664: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82EA9668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA966C: 4BFFFEE5  bl 0x82ea9550
	ctx.lr = 0x82EA9670;
	sub_82EA9550(ctx, base);
	// 82EA9670: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EA9674: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EA9678: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82EA967C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82EA9680: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA96B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA96B0 size=76
    let mut pc: u32 = 0x82EA96B0;
    'dispatch: loop {
        match pc {
            0x82EA96B0 => {
    //   block [0x82EA96B0..0x82EA96FC)
	// 82EA96B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA96B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA96B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA96BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA96C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82EA96C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA96C8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82EA96CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA96D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA96D4: 4BFFFECD  bl 0x82ea95a0
	ctx.lr = 0x82EA96D8;
	sub_82EA95A0(ctx, base);
	// 82EA96D8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EA96DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA96E0: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82EA96E4: 4BFFFE6D  bl 0x82ea9550
	ctx.lr = 0x82EA96E8;
	sub_82EA9550(ctx, base);
	// 82EA96E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA96EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA96F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA96F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA96F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9700 size=124
    let mut pc: u32 = 0x82EA9700;
    'dispatch: loop {
        match pc {
            0x82EA9700 => {
    //   block [0x82EA9700..0x82EA977C)
	// 82EA9700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9704: 482FEA61  bl 0x831a8164
	ctx.lr = 0x82EA9708;
	sub_831A8130(ctx, base);
	// 82EA9708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA970C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA9710: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA9714: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EA9718: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EA971C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA9720: 419A0054  beq cr6, 0x82ea9774
	if ctx.cr[6].eq {
	pc = 0x82EA9774; continue 'dispatch;
	}
	// 82EA9724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9728: 48000F19  bl 0x82eaa640
	ctx.lr = 0x82EA972C;
	sub_82EAA640(ctx, base);
	// 82EA972C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9730: 3B630001  addi r27, r3, 1
	ctx.r[27].s64 = ctx.r[3].s64 + 1;
	// 82EA9734: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA9738: 806B6A30  lwz r3, 0x6a30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EA973C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9740: 812A003C  lwz r9, 0x3c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EA9744: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA9748: 4E800421  bctrl
	ctx.lr = 0x82EA974C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA974C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9750: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82EA9754: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9758: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EA975C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EA9760: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA9764: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA9768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA976C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9770: 4E800421  bctrl
	ctx.lr = 0x82EA9774;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA9778: 482FEA3C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9780 size=112
    let mut pc: u32 = 0x82EA9780;
    'dispatch: loop {
        match pc {
            0x82EA9780 => {
    //   block [0x82EA9780..0x82EA97F0)
	// 82EA9780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9784: 482FE9DD  bl 0x831a8160
	ctx.lr = 0x82EA9788;
	sub_831A8130(ctx, base);
	// 82EA9788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA978C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA9794: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EA9798: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA979C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82EA97A0: 806B6A30  lwz r3, 0x6a30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EA97A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA97A8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EA97AC: 7C86E1D6  mullw r4, r6, r28
	ctx.r[4].s64 = (ctx.r[6].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82EA97B0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA97B4: 812A003C  lwz r9, 0x3c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EA97B8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA97BC: 4E800421  bctrl
	ctx.lr = 0x82EA97C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA97C0: 80DA0000  lwz r6, 0(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA97C4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82EA97C8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA97CC: 7CFDE1D6  mullw r7, r29, r28
	ctx.r[7].s64 = (ctx.r[29].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82EA97D0: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA97D4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EA97D8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EA97DC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EA97E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA97E4: 4E800421  bctrl
	ctx.lr = 0x82EA97E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA97E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA97EC: 482FE9C4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA97F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA97F0 size=212
    let mut pc: u32 = 0x82EA97F0;
    'dispatch: loop {
        match pc {
            0x82EA97F0 => {
    //   block [0x82EA97F0..0x82EA98C4)
	// 82EA97F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA97F4: 482FE969  bl 0x831a815c
	ctx.lr = 0x82EA97F8;
	sub_831A8130(ctx, base);
	// 82EA97F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA97FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA9800: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EA9804: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EA9808: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EA980C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA9810: 419A00AC  beq cr6, 0x82ea98bc
	if ctx.cr[6].eq {
	pc = 0x82EA98BC; continue 'dispatch;
	}
	// 82EA9814: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9818: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA981C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA9820: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA9824: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9828: 4E800421  bctrl
	ctx.lr = 0x82EA982C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA982C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EA9830: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA9834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA9838: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82EA983C: 4BFFE83D  bl 0x82ea8078
	ctx.lr = 0x82EA9840;
	sub_82EA8078(ctx, base);
	// 82EA9840: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9844: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA9848: 419A0030  beq cr6, 0x82ea9878
	if ctx.cr[6].eq {
	pc = 0x82EA9878; continue 'dispatch;
	}
	// 82EA984C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9850: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9854: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 82EA9858: 4BFFE3B1  bl 0x82ea7c08
	ctx.lr = 0x82EA985C;
	sub_82EA7C08(ctx, base);
	// 82EA985C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EA9860: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA9864: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA9868: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EA986C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EA9870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA9874: 4E800421  bctrl
	ctx.lr = 0x82EA9878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9878: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA987C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EA9880: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA9884: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA9888: 48000361  bl 0x82ea9be8
	ctx.lr = 0x82EA988C;
	sub_82EA9BE8(ctx, base);
	// 82EA988C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9890: 4BFFE381  bl 0x82ea7c10
	ctx.lr = 0x82EA9894;
	sub_82EA7C10(ctx, base);
	// 82EA9894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA9898: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA989C: 409AFFDC  bne cr6, 0x82ea9878
	if !ctx.cr[6].eq {
	pc = 0x82EA9878; continue 'dispatch;
	}
	// 82EA98A0: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82EA98A4: 419A0018  beq cr6, 0x82ea98bc
	if ctx.cr[6].eq {
	pc = 0x82EA98BC; continue 'dispatch;
	}
	// 82EA98A8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA98AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA98B0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA98B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA98B8: 4E800421  bctrl
	ctx.lr = 0x82EA98BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA98BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EA98C0: 482FE8EC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA98C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA98C8 size=8
    let mut pc: u32 = 0x82EA98C8;
    'dispatch: loop {
        match pc {
            0x82EA98C8 => {
    //   block [0x82EA98C8..0x82EA98D0)
	// 82EA98C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA98CC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA98D0 size=12
    let mut pc: u32 = 0x82EA98D0;
    'dispatch: loop {
        match pc {
            0x82EA98D0 => {
    //   block [0x82EA98D0..0x82EA98DC)
	// 82EA98D0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA98D4: 806B70F8  lwz r3, 0x70f8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28920 as u32) ) } as u64;
	// 82EA98D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA98E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA98E0 size=28
    let mut pc: u32 = 0x82EA98E0;
    'dispatch: loop {
        match pc {
            0x82EA98E0 => {
    //   block [0x82EA98E0..0x82EA98FC)
	// 82EA98E0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA98E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EA98E8: 806B70F4  lwz r3, 0x70f4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28916 as u32) ) } as u64;
	// 82EA98EC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA98F0: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA98F4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA98F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9900 size=272
    let mut pc: u32 = 0x82EA9900;
    'dispatch: loop {
        match pc {
            0x82EA9900 => {
    //   block [0x82EA9900..0x82EA9A10)
	// 82EA9900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9904: 482FE855  bl 0x831a8158
	ctx.lr = 0x82EA9908;
	sub_831A8130(ctx, base);
	// 82EA9908: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA990C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA9910: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA9914: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA9918: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82EA991C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82EA9920: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EA9924: 419A00E4  beq cr6, 0x82ea9a08
	if ctx.cr[6].eq {
	pc = 0x82EA9A08; continue 'dispatch;
	}
	// 82EA9928: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA992C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA9930: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EA9934: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA9938: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA993C: 4E800421  bctrl
	ctx.lr = 0x82EA9940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9940: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82EA9944: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA9948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA994C: 4BFFE72D  bl 0x82ea8078
	ctx.lr = 0x82EA9950;
	sub_82EA8078(ctx, base);
	// 82EA9950: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9954: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA9958: 419A002C  beq cr6, 0x82ea9984
	if ctx.cr[6].eq {
	pc = 0x82EA9984; continue 'dispatch;
	}
	// 82EA995C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9960: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA9964: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82EA9968: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA996C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9970: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA9974: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9978: 4E800421  bctrl
	ctx.lr = 0x82EA997C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA997C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EA9980: 482FE828  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 82EA9984: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA9988: 4BFFE6E1  bl 0x82ea8068
	ctx.lr = 0x82EA998C;
	sub_82EA8068(ctx, base);
	// 82EA998C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9990: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EA9994: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA9998: 806B6A30  lwz r3, 0x6a30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EA999C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA99A0: 812A003C  lwz r9, 0x3c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EA99A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA99A8: 4E800421  bctrl
	ctx.lr = 0x82EA99AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA99AC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82EA99B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA99B4: 419A000C  beq cr6, 0x82ea99c0
	if ctx.cr[6].eq {
	pc = 0x82EA99C0; continue 'dispatch;
	}
	// 82EA99B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA99BC: 48000010  b 0x82ea99cc
	pc = 0x82EA99CC; continue 'dispatch;
	// 82EA99C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA99C4: 4BFFE245  bl 0x82ea7c08
	ctx.lr = 0x82EA99C8;
	sub_82EA7C08(ctx, base);
	// 82EA99C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EA99CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA99D0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82EA99D4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EA99D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA99DC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82EA99E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA99E4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA99E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA99EC: 4E800421  bctrl
	ctx.lr = 0x82EA99F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA99F0: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82EA99F4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EA99F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA99FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA9A00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA9A04: 4BFFFDED  bl 0x82ea97f0
	ctx.lr = 0x82EA9A08;
	sub_82EA97F0(ctx, base);
	// 82EA9A08: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EA9A0C: 482FE79C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9A10 size=116
    let mut pc: u32 = 0x82EA9A10;
    'dispatch: loop {
        match pc {
            0x82EA9A10 => {
    //   block [0x82EA9A10..0x82EA9A84)
	// 82EA9A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA9A18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA9A1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA9A20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA9A24: 887F000D  lbz r3, 0xd(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 82EA9A28: 48004481  bl 0x82eadea8
	ctx.lr = 0x82EA9A2C;
	sub_82EADEA8(ctx, base);
	// 82EA9A2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA9A30: A14B0008  lhz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA9A34: 7D430734  extsh r3, r10
	ctx.r[3].s64 = ctx.r[10].s16 as i64;
	// 82EA9A38: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA9A3C: 41990034  bgt cr6, 0x82ea9a70
	if ctx.cr[6].gt {
	pc = 0x82EA9A70; continue 'dispatch;
	}
	// 82EA9A40: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9A44: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82EA9A48: 409A0024  bne cr6, 0x82ea9a6c
	if !ctx.cr[6].eq {
	pc = 0x82EA9A6C; continue 'dispatch;
	}
	// 82EA9A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9A50: 48004479  bl 0x82eadec8
	ctx.lr = 0x82EA9A54;
	sub_82EADEC8(ctx, base);
	// 82EA9A54: 4BFFE615  bl 0x82ea8068
	ctx.lr = 0x82EA9A58;
	sub_82EA8068(ctx, base);
	// 82EA9A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA9A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA9A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA9A64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA9A68: 4E800020  blr
	return;
	// 82EA9A6C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EA9A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA9A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA9A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA9A7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA9A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9A88 size=144
    let mut pc: u32 = 0x82EA9A88;
    'dispatch: loop {
        match pc {
            0x82EA9A88 => {
    //   block [0x82EA9A88..0x82EA9B18)
	// 82EA9A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9A8C: 482FE6D1  bl 0x831a815c
	ctx.lr = 0x82EA9A90;
	sub_831A8130(ctx, base);
	// 82EA9A90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA9A94: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EA9A98: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82EA9A9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA9AA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA9AA4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EA9AA8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82EA9AAC: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EA9AB0: 4BFFE5B9  bl 0x82ea8068
	ctx.lr = 0x82EA9AB4;
	sub_82EA8068(ctx, base);
	// 82EA9AB4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9AB8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EA9ABC: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9AC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA9AC4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA9AC8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9ACC: 4E800421  bctrl
	ctx.lr = 0x82EA9AD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9AD0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA9AD4: 40990028  ble cr6, 0x82ea9afc
	if !ctx.cr[6].gt {
	pc = 0x82EA9AFC; continue 'dispatch;
	}
	// 82EA9AD8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EA9ADC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EA9AE0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA9AE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA9AE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA9AEC: 4BFFFE15  bl 0x82ea9900
	ctx.lr = 0x82EA9AF0;
	sub_82EA9900(ctx, base);
	// 82EA9AF0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA9AF4: 7FFFD214  add r31, r31, r26
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 82EA9AF8: 4082FFE0  bne 0x82ea9ad8
	if !ctx.cr[0].eq {
	pc = 0x82EA9AD8; continue 'dispatch;
	}
	// 82EA9AFC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9B00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA9B04: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA9B08: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9B0C: 4E800421  bctrl
	ctx.lr = 0x82EA9B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9B10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA9B14: 482FE698  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9B18 size=208
    let mut pc: u32 = 0x82EA9B18;
    'dispatch: loop {
        match pc {
            0x82EA9B18 => {
    //   block [0x82EA9B18..0x82EA9BE8)
	// 82EA9B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9B1C: 482FE649  bl 0x831a8164
	ctx.lr = 0x82EA9B20;
	sub_831A8130(ctx, base);
	// 82EA9B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA9B24: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EA9B28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA9B2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA9B30: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EA9B34: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EA9B38: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EA9B3C: 409900A4  ble cr6, 0x82ea9be0
	if !ctx.cr[6].gt {
	pc = 0x82EA9BE0; continue 'dispatch;
	}
	// 82EA9B40: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 82EA9B44: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82EA9B48: 419A0034  beq cr6, 0x82ea9b7c
	if ctx.cr[6].eq {
	pc = 0x82EA9B7C; continue 'dispatch;
	}
	// 82EA9B4C: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 82EA9B50: 409A0090  bne cr6, 0x82ea9be0
	if !ctx.cr[6].eq {
	pc = 0x82EA9BE0; continue 'dispatch;
	}
	// 82EA9B54: 48004375  bl 0x82eadec8
	ctx.lr = 0x82EA9B58;
	sub_82EADEC8(ctx, base);
	// 82EA9B58: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA9B5C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82EA9B60: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EA9B64: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EA9B68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA9B6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA9B70: 4BFFFF19  bl 0x82ea9a88
	ctx.lr = 0x82EA9B74;
	sub_82EA9A88(ctx, base);
	// 82EA9B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA9B78: 482FE63C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EA9B7C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9B80: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA9B84: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9B88: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA9B8C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9B90: 4E800421  bctrl
	ctx.lr = 0x82EA9B94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9B94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA9B98: 48004331  bl 0x82eadec8
	ctx.lr = 0x82EA9B9C;
	sub_82EADEC8(ctx, base);
	// 82EA9B9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA9BA0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EA9BA4: 40990028  ble cr6, 0x82ea9bcc
	if !ctx.cr[6].gt {
	pc = 0x82EA9BCC; continue 'dispatch;
	}
	// 82EA9BA8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EA9BAC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9BB0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA9BB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EA9BB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA9BBC: 4BFFFD45  bl 0x82ea9900
	ctx.lr = 0x82EA9BC0;
	sub_82EA9900(ctx, base);
	// 82EA9BC0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EA9BC4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EA9BC8: 4082FFE0  bne 0x82ea9ba8
	if !ctx.cr[0].eq {
	pc = 0x82EA9BA8; continue 'dispatch;
	}
	// 82EA9BCC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9BD0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA9BD4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA9BD8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9BDC: 4E800421  bctrl
	ctx.lr = 0x82EA9BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA9BE4: 482FE5D0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9BE8 size=828
    let mut pc: u32 = 0x82EA9BE8;
    'dispatch: loop {
        match pc {
            0x82EA9BE8 => {
    //   block [0x82EA9BE8..0x82EA9CC4)
	// 82EA9BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9BEC: 482FE561  bl 0x831a814c
	ctx.lr = 0x82EA9BF0;
	sub_831A8130(ctx, base);
	// 82EA9BF0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA9BF4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82EA9BF8: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82EA9BFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA9C00: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82EA9C04: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82EA9C08: 419A0314  beq cr6, 0x82ea9f1c
	if ctx.cr[6].eq {
	pc = 0x82EA9F1C; continue 'dispatch;
	}
	// 82EA9C0C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82EA9C10: 409A000C  bne cr6, 0x82ea9c1c
	if !ctx.cr[6].eq {
	pc = 0x82EA9C1C; continue 'dispatch;
	}
	// 82EA9C14: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9C18: 832B70F8  lwz r25, 0x70f8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28920 as u32) ) } as u64;
	// 82EA9C1C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9C20: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82EA9C24: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EA9C28: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA9C2C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA9C30: 4E800421  bctrl
	ctx.lr = 0x82EA9C34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9C34: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EA9C38: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82EA9C3C: 4BFFE27D  bl 0x82ea7eb8
	ctx.lr = 0x82EA9C40;
	sub_82EA7EB8(ctx, base);
	// 82EA9C40: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82EA9C44: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82EA9C48: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82EA9C4C: 409902D0  ble cr6, 0x82ea9f1c
	if !ctx.cr[6].gt {
	pc = 0x82EA9F1C; continue 'dispatch;
	}
	// 82EA9C50: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EA9C54: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82EA9C58: 4BFFE269  bl 0x82ea7ec0
	ctx.lr = 0x82EA9C5C;
	sub_82EA7EC0(ctx, base);
	// 82EA9C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA9C60: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA9C64: 556ABA7E  srwi r10, r11, 9
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(9);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA9C68: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82EA9C6C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA9C70: 409A02A0  bne cr6, 0x82ea9f10
	if !ctx.cr[6].eq {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9C74: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA9C78: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 82EA9C7C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82EA9C80: 41990290  bgt cr6, 0x82ea9f10
	if ctx.cr[6].gt {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9C84: 3D8082EB  lis r12, -0x7d15
	ctx.r[12].s64 = -2098528256;
	// 82EA9C88: 398C9C9C  addi r12, r12, -0x6364
	ctx.r[12].s64 = ctx.r[12].s64 + -25444;
	// 82EA9C8C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82EA9C90: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82EA9C94: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82EA9C98: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82EA9EB0; continue 'dispatch;
		},
		1 => {
	pc = 0x82EA9F10; continue 'dispatch;
		},
		2 => {
	pc = 0x82EA9D50; continue 'dispatch;
		},
		3 => {
	pc = 0x82EA9F10; continue 'dispatch;
		},
		4 => {
	pc = 0x82EA9F10; continue 'dispatch;
		},
		5 => {
	pc = 0x82EA9F10; continue 'dispatch;
		},
		6 => {
	pc = 0x82EA9DB4; continue 'dispatch;
		},
		7 => {
	pc = 0x82EA9E28; continue 'dispatch;
		},
		8 => {
	pc = 0x82EA9EE0; continue 'dispatch;
		},
		9 => {
	pc = 0x82EA9CC4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82EA9C9C: 82EA9EB0  lwz r23, -0x6150(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24912 as u32) ) } as u64;
	// 82EA9CA0: 82EA9F10  lwz r23, -0x60f0(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24816 as u32) ) } as u64;
	// 82EA9CA4: 82EA9D50  lwz r23, -0x62b0(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25264 as u32) ) } as u64;
	// 82EA9CA8: 82EA9F10  lwz r23, -0x60f0(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24816 as u32) ) } as u64;
	// 82EA9CAC: 82EA9F10  lwz r23, -0x60f0(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24816 as u32) ) } as u64;
	// 82EA9CB0: 82EA9F10  lwz r23, -0x60f0(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24816 as u32) ) } as u64;
	// 82EA9CB4: 82EA9DB4  lwz r23, -0x624c(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25164 as u32) ) } as u64;
	// 82EA9CB8: 82EA9E28  lwz r23, -0x61d8(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25048 as u32) ) } as u64;
	// 82EA9CBC: 82EA9EE0  lwz r23, -0x6120(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24864 as u32) ) } as u64;
	// 82EA9CC0: 82EA9CC4  lwz r23, -0x633c(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25404 as u32) ) } as u64;
            }
            0x82EA9CC4 => {
    //   block [0x82EA9CC4..0x82EA9D50)
	// 82EA9CC4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA9CC8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EA9CCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA9CD0: 48006AB9  bl 0x82eb0788
	ctx.lr = 0x82EA9CD4;
	sub_82EB0788(ctx, base);
	// 82EA9CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9CD8: 48004209  bl 0x82eadee0
	ctx.lr = 0x82EA9CDC;
	sub_82EADEE0(ctx, base);
	// 82EA9CDC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA9CE0: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82EA9CE4: 4199002C  bgt cr6, 0x82ea9d10
	if ctx.cr[6].gt {
	pc = 0x82EA9D10; continue 'dispatch;
	}
	// 82EA9CE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA9CEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA9CF0: 48006BB9  bl 0x82eb08a8
	ctx.lr = 0x82EA9CF4;
	sub_82EB08A8(ctx, base);
	// 82EA9CF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA9CF8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EA9CFC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EA9D00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9D04: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9D08: 4BFFF9F9  bl 0x82ea9700
	ctx.lr = 0x82EA9D0C;
	sub_82EA9700(ctx, base);
	// 82EA9D0C: 48000204  b 0x82ea9f10
	pc = 0x82EA9F10; continue 'dispatch;
	// 82EA9D10: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EA9D14: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EA9D18: 409901F8  ble cr6, 0x82ea9f10
	if !ctx.cr[6].gt {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9D1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA9D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA9D24: 48006B85  bl 0x82eb08a8
	ctx.lr = 0x82EA9D28;
	sub_82EB08A8(ctx, base);
	// 82EA9D28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA9D2C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EA9D30: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EA9D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9D38: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9D3C: 4BFFF9C5  bl 0x82ea9700
	ctx.lr = 0x82EA9D40;
	sub_82EA9700(ctx, base);
	// 82EA9D40: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EA9D44: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EA9D48: 4198FFD4  blt cr6, 0x82ea9d1c
	if ctx.cr[6].lt {
	pc = 0x82EA9D1C; continue 'dispatch;
	}
	// 82EA9D4C: 480001C4  b 0x82ea9f10
	pc = 0x82EA9F10; continue 'dispatch;
            }
            0x82EA9D50 => {
    //   block [0x82EA9D50..0x82EA9DB4)
	// 82EA9D50: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EA9D54: 7FCBC214  add r30, r11, r24
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82EA9D58: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA9D5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA9D60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA9D64: 409A01AC  bne cr6, 0x82ea9f10
	if !ctx.cr[6].eq {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9D6C: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA9D70: 557B00BE  clrlwi r27, r11, 2
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA9D74: 4BFFFC9D  bl 0x82ea9a10
	ctx.lr = 0x82EA9D78;
	sub_82EA9A10(ctx, base);
	// 82EA9D78: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82EA9D7C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA9D80: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9D84: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82EA9D88: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EA9D8C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EA9D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9D94: 4BFFF9ED  bl 0x82ea9780
	ctx.lr = 0x82EA9D98;
	sub_82EA9780(ctx, base);
	// 82EA9D98: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EA9D9C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EA9DA0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9DA4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA9DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9DAC: 4BFFFD6D  bl 0x82ea9b18
	ctx.lr = 0x82EA9DB0;
	sub_82EA9B18(ctx, base);
	// 82EA9DB0: 48000160  b 0x82ea9f10
	pc = 0x82EA9F10; continue 'dispatch;
            }
            0x82EA9DB4 => {
    //   block [0x82EA9DB4..0x82EA9E28)
	// 82EA9DB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA9DB8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EA9DBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA9DC0: 480069C9  bl 0x82eb0788
	ctx.lr = 0x82EA9DC4;
	sub_82EB0788(ctx, base);
	// 82EA9DC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA9DC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA9DCC: 48006BCD  bl 0x82eb0998
	ctx.lr = 0x82EA9DD0;
	sub_82EB0998(ctx, base);
	// 82EA9DD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA9DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9DD8: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EA9DDC: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82EA9DE0: 4BFFFC31  bl 0x82ea9a10
	ctx.lr = 0x82EA9DE4;
	sub_82EA9A10(ctx, base);
	// 82EA9DE4: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EA9DE8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82EA9DEC: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA9DF0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82EA9DF4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EA9DF8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EA9DFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA9E00: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA9E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9E08: 4BFFF979  bl 0x82ea9780
	ctx.lr = 0x82EA9E0C;
	sub_82EA9780(ctx, base);
	// 82EA9E0C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EA9E10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EA9E14: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA9E18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA9E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9E20: 4BFFFCF9  bl 0x82ea9b18
	ctx.lr = 0x82EA9E24;
	sub_82EA9B18(ctx, base);
	// 82EA9E24: 480000EC  b 0x82ea9f10
	pc = 0x82EA9F10; continue 'dispatch;
            }
            0x82EA9E28 => {
    //   block [0x82EA9E28..0x82EA9EB0)
	// 82EA9E28: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA9E2C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EA9E30: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82EA9E34: 48006955  bl 0x82eb0788
	ctx.lr = 0x82EA9E38;
	sub_82EB0788(ctx, base);
	// 82EA9E38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA9E3C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82EA9E40: 48006B69  bl 0x82eb09a8
	ctx.lr = 0x82EA9E44;
	sub_82EB09A8(ctx, base);
	// 82EA9E44: 83C30008  lwz r30, 8(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA9E48: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9E4C: 83630004  lwz r27, 4(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA9E50: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA9E54: 409900BC  ble cr6, 0x82ea9f10
	if !ctx.cr[6].gt {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9E58: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EA9E5C: 419A00B4  beq cr6, 0x82ea9f10
	if ctx.cr[6].eq {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9E60: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EA9E64: 419A00AC  beq cr6, 0x82ea9f10
	if ctx.cr[6].eq {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9E68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA9E6C: 4BFFE1FD  bl 0x82ea8068
	ctx.lr = 0x82EA9E70;
	sub_82EA8068(ctx, base);
	// 82EA9E70: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82EA9E74: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EA9E78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA9E7C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA9E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9E84: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EA9E88: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82EA9E8C: 4BFFF8F5  bl 0x82ea9780
	ctx.lr = 0x82EA9E90;
	sub_82EA9780(ctx, base);
	// 82EA9E90: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82EA9E94: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82EA9E98: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EA9E9C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA9EA0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA9EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9EA8: 4BFFFBE1  bl 0x82ea9a88
	ctx.lr = 0x82EA9EAC;
	sub_82EA9A88(ctx, base);
	// 82EA9EAC: 48000064  b 0x82ea9f10
	pc = 0x82EA9F10; continue 'dispatch;
            }
            0x82EA9EB0 => {
    //   block [0x82EA9EB0..0x82EA9EE0)
	// 82EA9EB0: 897F000D  lbz r11, 0xd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 82EA9EB4: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82EA9EB8: 409A0058  bne cr6, 0x82ea9f10
	if !ctx.cr[6].eq {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9EBC: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EA9EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9EC4: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9EC8: 7FCBC02E  lwzx r30, r11, r24
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EA9ECC: 48003FFD  bl 0x82eadec8
	ctx.lr = 0x82EA9ED0;
	sub_82EADEC8(ctx, base);
	// 82EA9ED0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA9ED4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA9ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9EDC: 48000028  b 0x82ea9f04
	pc = 0x82EA9F04; continue 'dispatch;
            }
            0x82EA9EE0 => {
    //   block [0x82EA9EE0..0x82EA9F10)
	// 82EA9EE0: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EA9EE4: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82EA9EE8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9EEC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA9EF0: 419A0020  beq cr6, 0x82ea9f10
	if ctx.cr[6].eq {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9EF4: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA9EF8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EA9EFC: 419A0014  beq cr6, 0x82ea9f10
	if ctx.cr[6].eq {
	pc = 0x82EA9F10; continue 'dispatch;
	}
	// 82EA9F00: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9F04: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EA9F08: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EA9F0C: 4BFFF9F5  bl 0x82ea9900
	ctx.lr = 0x82EA9F10;
	sub_82EA9900(ctx, base);
	pc = 0x82EA9F10; continue 'dispatch;
            }
            0x82EA9F10 => {
    //   block [0x82EA9F10..0x82EA9F24)
	// 82EA9F10: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82EA9F14: 7F17A800  cmpw cr6, r23, r21
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[21].s32, &mut ctx.xer);
	// 82EA9F18: 4198FD38  blt cr6, 0x82ea9c50
	if ctx.cr[6].lt {
	pc = 0x82EA9C50; continue 'dispatch;
	}
	// 82EA9F1C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82EA9F20: 482FE27C  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9F28 size=100
    let mut pc: u32 = 0x82EA9F28;
    'dispatch: loop {
        match pc {
            0x82EA9F28 => {
    //   block [0x82EA9F28..0x82EA9F8C)
	// 82EA9F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9F2C: 482FE23D  bl 0x831a8168
	ctx.lr = 0x82EA9F30;
	sub_831A8130(ctx, base);
	// 82EA9F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA9F34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA9F38: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EA9F3C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EA9F40: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA9F44: 419A0040  beq cr6, 0x82ea9f84
	if ctx.cr[6].eq {
	pc = 0x82EA9F84; continue 'dispatch;
	}
	// 82EA9F48: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA9F4C: 409A000C  bne cr6, 0x82ea9f58
	if !ctx.cr[6].eq {
	pc = 0x82EA9F58; continue 'dispatch;
	}
	// 82EA9F50: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9F54: 83CB70F8  lwz r30, 0x70f8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28920 as u32) ) } as u64;
	// 82EA9F58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA9F5C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EA9F60: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA9F64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA9F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA9F6C: 4BFFFC7D  bl 0x82ea9be8
	ctx.lr = 0x82EA9F70;
	sub_82EA9BE8(ctx, base);
	// 82EA9F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA9F74: 4BFFDC9D  bl 0x82ea7c10
	ctx.lr = 0x82EA9F78;
	sub_82EA7C10(ctx, base);
	// 82EA9F78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA9F7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA9F80: 409AFFDC  bne cr6, 0x82ea9f5c
	if !ctx.cr[6].eq {
	pc = 0x82EA9F5C; continue 'dispatch;
	}
	// 82EA9F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA9F88: 482FE230  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA9F90 size=40
    let mut pc: u32 = 0x82EA9F90;
    'dispatch: loop {
        match pc {
            0x82EA9F90 => {
    //   block [0x82EA9F90..0x82EA9FB8)
	// 82EA9F90: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82EA9F94: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EA9F98: 409A000C  bne cr6, 0x82ea9fa4
	if !ctx.cr[6].eq {
	pc = 0x82EA9FA4; continue 'dispatch;
	}
	// 82EA9F9C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9FA0: 80EB70F8  lwz r7, 0x70f8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28920 as u32) ) } as u64;
	// 82EA9FA4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EA9FA8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EA9FAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EA9FB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA9FB4: 4BFFF83C  b 0x82ea97f0
	sub_82EA97F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA9FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA9FB8 size=108
    let mut pc: u32 = 0x82EA9FB8;
    'dispatch: loop {
        match pc {
            0x82EA9FB8 => {
    //   block [0x82EA9FB8..0x82EAA024)
	// 82EA9FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA9FBC: 482FE1B1  bl 0x831a816c
	ctx.lr = 0x82EA9FC0;
	sub_831A8130(ctx, base);
	// 82EA9FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA9FC4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA9FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA9FCC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA9FD0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA9FD4: 409A000C  bne cr6, 0x82ea9fe0
	if !ctx.cr[6].eq {
	pc = 0x82EA9FE0; continue 'dispatch;
	}
	// 82EA9FD8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9FDC: 83CB70F8  lwz r30, 0x70f8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28920 as u32) ) } as u64;
	// 82EA9FE0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA9FE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA9FE8: 806B70F4  lwz r3, 0x70f4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28916 as u32) ) } as u64;
	// 82EA9FEC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA9FF0: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA9FF4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA9FF8: 4E800421  bctrl
	ctx.lr = 0x82EA9FFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA9FFC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EAA000: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EAA004: 419A0018  beq cr6, 0x82eaa01c
	if ctx.cr[6].eq {
	pc = 0x82EAA01C; continue 'dispatch;
	}
	// 82EAA008: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82EAA00C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EAA010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAA014: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAA018: 4BFFF7D9  bl 0x82ea97f0
	ctx.lr = 0x82EAA01C;
	sub_82EA97F0(ctx, base);
	// 82EAA01C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAA020: 482FE19C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA028 size=204
    let mut pc: u32 = 0x82EAA028;
    'dispatch: loop {
        match pc {
            0x82EAA028 => {
    //   block [0x82EAA028..0x82EAA0F4)
	// 82EAA028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA02C: 482FE139  bl 0x831a8164
	ctx.lr = 0x82EAA030;
	sub_831A8130(ctx, base);
	// 82EAA030: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA034: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EAA038: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EAA03C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EAA040: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EAA044: 419A00A4  beq cr6, 0x82eaa0e8
	if ctx.cr[6].eq {
	pc = 0x82EAA0E8; continue 'dispatch;
	}
	// 82EAA048: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EAA04C: 409A000C  bne cr6, 0x82eaa058
	if !ctx.cr[6].eq {
	pc = 0x82EAA058; continue 'dispatch;
	}
	// 82EAA050: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EAA054: 83CB70F8  lwz r30, 0x70f8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28920 as u32) ) } as u64;
	// 82EAA058: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EAA05C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EAA060: 806B70F4  lwz r3, 0x70f4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28916 as u32) ) } as u64;
	// 82EAA064: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA068: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAA06C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EAA070: 4E800421  bctrl
	ctx.lr = 0x82EAA074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAA074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAA078: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EAA07C: 419A006C  beq cr6, 0x82eaa0e8
	if ctx.cr[6].eq {
	pc = 0x82EAA0E8; continue 'dispatch;
	}
	// 82EAA080: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA084: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAA088: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAA08C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAA090: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAA094: 4E800421  bctrl
	ctx.lr = 0x82EAA098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAA098: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAA09C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAA0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EAA0A4: 4BFFDFD5  bl 0x82ea8078
	ctx.lr = 0x82EAA0A8;
	sub_82EA8078(ctx, base);
	// 82EAA0A8: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA0AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EAA0B0: 419A0038  beq cr6, 0x82eaa0e8
	if ctx.cr[6].eq {
	pc = 0x82EAA0E8; continue 'dispatch;
	}
	// 82EAA0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAA0B8: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA0BC: 4BFFDB4D  bl 0x82ea7c08
	ctx.lr = 0x82EAA0C0;
	sub_82EA7C08(ctx, base);
	// 82EAA0C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EAA0C4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAA0C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EAA0CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EAA0D0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EAA0D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EAA0D8: 4E800421  bctrl
	ctx.lr = 0x82EAA0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAA0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAA0E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EAA0E4: 482FE0D0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EAA0E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAA0EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EAA0F0: 482FE0C4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA0F8 size=88
    let mut pc: u32 = 0x82EAA0F8;
    'dispatch: loop {
        match pc {
            0x82EAA0F8 => {
    //   block [0x82EAA0F8..0x82EAA150)
	// 82EAA0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA0FC: 482FE071  bl 0x831a816c
	ctx.lr = 0x82EAA100;
	sub_831A8130(ctx, base);
	// 82EAA100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA104: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EAA108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAA10C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EAA110: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EAA114: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAA118: 806B70F4  lwz r3, 0x70f4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28916 as u32) ) } as u64;
	// 82EAA11C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA120: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAA124: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EAA128: 4E800421  bctrl
	ctx.lr = 0x82EAA12C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAA12C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EAA130: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EAA134: 419A0014  beq cr6, 0x82eaa148
	if ctx.cr[6].eq {
	pc = 0x82EAA148; continue 'dispatch;
	}
	// 82EAA138: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EAA13C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EAA140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAA144: 4BFFFAA5  bl 0x82ea9be8
	ctx.lr = 0x82EAA148;
	sub_82EA9BE8(ctx, base);
	// 82EAA148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAA14C: 482FE070  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA150 size=88
    let mut pc: u32 = 0x82EAA150;
    'dispatch: loop {
        match pc {
            0x82EAA150 => {
    //   block [0x82EAA150..0x82EAA1A8)
	// 82EAA150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA154: 482FE019  bl 0x831a816c
	ctx.lr = 0x82EAA158;
	sub_831A8130(ctx, base);
	// 82EAA158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA15C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EAA160: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAA164: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EAA168: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EAA16C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAA170: 806B70F4  lwz r3, 0x70f4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28916 as u32) ) } as u64;
	// 82EAA174: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA178: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAA17C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EAA180: 4E800421  bctrl
	ctx.lr = 0x82EAA184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAA184: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EAA188: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EAA18C: 419A0014  beq cr6, 0x82eaa1a0
	if ctx.cr[6].eq {
	pc = 0x82EAA1A0; continue 'dispatch;
	}
	// 82EAA190: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EAA194: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EAA198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAA19C: 4BFFFD8D  bl 0x82ea9f28
	ctx.lr = 0x82EAA1A0;
	sub_82EA9F28(ctx, base);
	// 82EAA1A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAA1A4: 482FE018  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA1A8 size=12
    let mut pc: u32 = 0x82EAA1A8;
    'dispatch: loop {
        match pc {
            0x82EAA1A8 => {
    //   block [0x82EAA1A8..0x82EAA1B4)
	// 82EAA1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAA1AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EAA1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA1B8 size=64
    let mut pc: u32 = 0x82EAA1B8;
    'dispatch: loop {
        match pc {
            0x82EAA1B8 => {
    //   block [0x82EAA1B8..0x82EAA1F8)
	// 82EAA1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA1C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAA1C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA1C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAA1CC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA1D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAA1D4: 419A0010  beq cr6, 0x82eaa1e4
	if ctx.cr[6].eq {
	pc = 0x82EAA1E4; continue 'dispatch;
	}
	// 82EAA1D8: 4BD22849  bl 0x82bcca20
	ctx.lr = 0x82EAA1DC;
	sub_82BCCA20(ctx, base);
	// 82EAA1DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAA1E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EAA1E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA1F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAA1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA1F8 size=80
    let mut pc: u32 = 0x82EAA1F8;
    'dispatch: loop {
        match pc {
            0x82EAA1F8 => {
    //   block [0x82EAA1F8..0x82EAA248)
	// 82EAA1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAA204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA208: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAA20C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EAA210: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EAA214: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 82EAA218: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EAA21C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EAA220: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAA224: 4831A965  bl 0x831c4b88
	ctx.lr = 0x82EAA228;
	sub_831C4B88(ctx, base);
	// 82EAA228: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EAA22C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EAA230: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EAA234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAA244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA248 size=84
    let mut pc: u32 = 0x82EAA248;
    'dispatch: loop {
        match pc {
            0x82EAA248 => {
    //   block [0x82EAA248..0x82EAA29C)
	// 82EAA248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA254: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA258: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAA25C: 409A0018  bne cr6, 0x82eaa274
	if !ctx.cr[6].eq {
	pc = 0x82EAA274; continue 'dispatch;
	}
	// 82EAA260: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82EAA264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA270: 4E800020  blr
	return;
	// 82EAA274: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EAA278: 4BD23BE1  bl 0x82bcde58
	ctx.lr = 0x82EAA27C;
	sub_82BCDE58(ctx, base);
	// 82EAA27C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EAA280: 394BFEFD  addi r10, r11, -0x103
	ctx.r[10].s64 = ctx.r[11].s64 + -259;
	// 82EAA284: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82EAA288: 5523DFFE  rlwinm r3, r9, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82EAA28C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA2A0 size=36
    let mut pc: u32 = 0x82EAA2A0;
    'dispatch: loop {
        match pc {
            0x82EAA2A0 => {
    //   block [0x82EAA2A0..0x82EAA2C4)
	// 82EAA2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA2A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA2AC: 48319D75  bl 0x831c4020
	ctx.lr = 0x82EAA2B0;
	sub_831C4020(ctx, base);
	// 82EAA2B0: 78630020  clrldi r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82EAA2B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA2C8 size=8
    let mut pc: u32 = 0x82EAA2C8;
    'dispatch: loop {
        match pc {
            0x82EAA2C8 => {
    //   block [0x82EAA2C8..0x82EAA2D0)
	// 82EAA2C8: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82EAA2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA2D0 size=8
    let mut pc: u32 = 0x82EAA2D0;
    'dispatch: loop {
        match pc {
            0x82EAA2D0 => {
    //   block [0x82EAA2D0..0x82EAA2D8)
	// 82EAA2D0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA2D8 size=64
    let mut pc: u32 = 0x82EAA2D8;
    'dispatch: loop {
        match pc {
            0x82EAA2D8 => {
    //   block [0x82EAA2D8..0x82EAA318)
	// 82EAA2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA2E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAA2E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA2E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAA2EC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA2F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAA2F4: 419A0010  beq cr6, 0x82eaa304
	if ctx.cr[6].eq {
	pc = 0x82EAA304; continue 'dispatch;
	}
	// 82EAA2F8: 4BD22729  bl 0x82bcca20
	ctx.lr = 0x82EAA2FC;
	sub_82BCCA20(ctx, base);
	// 82EAA2FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAA300: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EAA304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAA314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA318 size=12
    let mut pc: u32 = 0x82EAA318;
    'dispatch: loop {
        match pc {
            0x82EAA318 => {
    //   block [0x82EAA318..0x82EAA324)
	// 82EAA318: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82EAA31C: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 82EAA320: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA324(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA324 size=8
    let mut pc: u32 = 0x82EAA324;
    'dispatch: loop {
        match pc {
            0x82EAA324 => {
    //   block [0x82EAA324..0x82EAA32C)
	// 82EAA324: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 82EAA328: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA32C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA32C size=12
    let mut pc: u32 = 0x82EAA32C;
    'dispatch: loop {
        match pc {
            0x82EAA32C => {
    //   block [0x82EAA32C..0x82EAA338)
	// 82EAA32C: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 82EAA330: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82EAA334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA338 size=12
    let mut pc: u32 = 0x82EAA338;
    'dispatch: loop {
        match pc {
            0x82EAA338 => {
    //   block [0x82EAA338..0x82EAA344)
	// 82EAA338: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82EAA33C: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA340: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA344 size=8
    let mut pc: u32 = 0x82EAA344;
    'dispatch: loop {
        match pc {
            0x82EAA344 => {
    //   block [0x82EAA344..0x82EAA34C)
	// 82EAA344: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA348: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA34C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA34C size=12
    let mut pc: u32 = 0x82EAA34C;
    'dispatch: loop {
        match pc {
            0x82EAA34C => {
    //   block [0x82EAA34C..0x82EAA358)
	// 82EAA34C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA350: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82EAA354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA358 size=4
    let mut pc: u32 = 0x82EAA358;
    'dispatch: loop {
        match pc {
            0x82EAA358 => {
    //   block [0x82EAA358..0x82EAA35C)
	// 82EAA358: 48303970  b 0x831adcc8
	sub_831ADCC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA360 size=68
    let mut pc: u32 = 0x82EAA360;
    'dispatch: loop {
        match pc {
            0x82EAA360 => {
    //   block [0x82EAA360..0x82EAA3A4)
	// 82EAA360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA368: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EAA36C: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EAA370: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EAA374: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EAA378: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EAA37C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA380: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EAA384: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 82EAA388: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAA38C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EAA390: 48303939  bl 0x831adcc8
	ctx.lr = 0x82EAA394;
	sub_831ADCC8(ctx, base);
	// 82EAA394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA3A8 size=72
    let mut pc: u32 = 0x82EAA3A8;
    'dispatch: loop {
        match pc {
            0x82EAA3A8 => {
    //   block [0x82EAA3A8..0x82EAA3F0)
	// 82EAA3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA3B0: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82EAA3B4: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EAA3B8: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EAA3BC: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EAA3C0: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EAA3C4: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EAA3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA3CC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EAA3D0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82EAA3D4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAA3D8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EAA3DC: 48301A25  bl 0x831abe00
	ctx.lr = 0x82EAA3E0;
	sub_831ABE00(ctx, base);
	// 82EAA3E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA3F0 size=24
    let mut pc: u32 = 0x82EAA3F0;
    'dispatch: loop {
        match pc {
            0x82EAA3F0 => {
    //   block [0x82EAA3F0..0x82EAA408)
	// 82EAA3F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EAA3F4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA3F8: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA3FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAA400: 7C695050  subf r3, r9, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82EAA404: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA408 size=20
    let mut pc: u32 = 0x82EAA408;
    'dispatch: loop {
        match pc {
            0x82EAA408 => {
    //   block [0x82EAA408..0x82EAA41C)
	// 82EAA408: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAA40C: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82EAA410: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EAA414: 419AFFE0  beq cr6, 0x82eaa3f4
	if ctx.cr[6].eq {
		sub_82EAA3F0(ctx, base);
		return;
	}
	// 82EAA418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA420 size=4
    let mut pc: u32 = 0x82EAA420;
    'dispatch: loop {
        match pc {
            0x82EAA420 => {
    //   block [0x82EAA420..0x82EAA424)
	// 82EAA420: 483029B0  b 0x831acdd0
	sub_831ACDD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA428 size=60
    let mut pc: u32 = 0x82EAA428;
    'dispatch: loop {
        match pc {
            0x82EAA428 => {
    //   block [0x82EAA428..0x82EAA464)
	// 82EAA428: 7CE41850  subf r7, r4, r3
	ctx.r[7].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82EAA42C: 7D4720AE  lbzx r10, r7, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82EAA430: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAA434: 409A0010  bne cr6, 0x82eaa444
	if !ctx.cr[6].eq {
	pc = 0x82EAA444; continue 'dispatch;
	}
	// 82EAA438: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA43C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAA440: 419A00B8  beq cr6, 0x82eaa4f8
	if ctx.cr[6].eq {
		sub_82EAA4F8(ctx, base);
		return;
	}
	// 82EAA444: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82EAA448: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA44C: 41980018  blt cr6, 0x82eaa464
	if ctx.cr[6].lt {
		sub_82EAA464(ctx, base);
		return;
	}
	// 82EAA450: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA454: 41990010  bgt cr6, 0x82eaa464
	if ctx.cr[6].gt {
		sub_82EAA464(ctx, base);
		return;
	}
	// 82EAA458: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA45C: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 82EAA460: 48000008  b 0x82eaa468
	sub_82EAA464(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA464(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA464 size=40
    let mut pc: u32 = 0x82EAA464;
    'dispatch: loop {
        match pc {
            0x82EAA464 => {
    //   block [0x82EAA464..0x82EAA48C)
	// 82EAA464: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82EAA468: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA46C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82EAA470: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA474: 41980018  blt cr6, 0x82eaa48c
	if ctx.cr[6].lt {
		sub_82EAA48C(ctx, base);
		return;
	}
	// 82EAA478: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA47C: 41990010  bgt cr6, 0x82eaa48c
	if ctx.cr[6].gt {
		sub_82EAA48C(ctx, base);
		return;
	}
	// 82EAA480: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA484: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA488: 48000008  b 0x82eaa490
	sub_82EAA48C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA48C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA48C size=80
    let mut pc: u32 = 0x82EAA48C;
    'dispatch: loop {
        match pc {
            0x82EAA48C => {
    //   block [0x82EAA48C..0x82EAA4DC)
	// 82EAA48C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82EAA490: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA494: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82EAA498: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAA49C: 41980064  blt cr6, 0x82eaa500
	if ctx.cr[6].lt {
		sub_82EAA500(ctx, base);
		return;
	}
	// 82EAA4A0: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82EAA4A4: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA4A8: 41980014  blt cr6, 0x82eaa4bc
	if ctx.cr[6].lt {
	pc = 0x82EAA4BC; continue 'dispatch;
	}
	// 82EAA4AC: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA4B0: 4199000C  bgt cr6, 0x82eaa4bc
	if ctx.cr[6].gt {
	pc = 0x82EAA4BC; continue 'dispatch;
	}
	// 82EAA4B4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA4B8: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82EAA4BC: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82EAA4C0: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA4C4: 41980018  blt cr6, 0x82eaa4dc
	if ctx.cr[6].lt {
		sub_82EAA4DC(ctx, base);
		return;
	}
	// 82EAA4C8: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA4CC: 41990010  bgt cr6, 0x82eaa4dc
	if ctx.cr[6].gt {
		sub_82EAA4DC(ctx, base);
		return;
	}
	// 82EAA4D0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA4D4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA4D8: 48000008  b 0x82eaa4e0
	sub_82EAA4DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA4DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA4DC size=28
    let mut pc: u32 = 0x82EAA4DC;
    'dispatch: loop {
        match pc {
            0x82EAA4DC => {
    //   block [0x82EAA4DC..0x82EAA4F8)
	// 82EAA4DC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82EAA4E0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA4E4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EAA4E8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAA4EC: 4199001C  bgt cr6, 0x82eaa508
	if ctx.cr[6].gt {
		sub_82EAA508(ctx, base);
		return;
	}
	// 82EAA4F0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82EAA4F4: 4BFFFF38  b 0x82eaa42c
	sub_82EAA428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA4F8 size=8
    let mut pc: u32 = 0x82EAA4F8;
    'dispatch: loop {
        match pc {
            0x82EAA4F8 => {
    //   block [0x82EAA4F8..0x82EAA500)
	// 82EAA4F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAA4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA500 size=8
    let mut pc: u32 = 0x82EAA500;
    'dispatch: loop {
        match pc {
            0x82EAA500 => {
    //   block [0x82EAA500..0x82EAA508)
	// 82EAA500: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EAA504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA508 size=8
    let mut pc: u32 = 0x82EAA508;
    'dispatch: loop {
        match pc {
            0x82EAA508 => {
    //   block [0x82EAA508..0x82EAA510)
	// 82EAA508: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EAA50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA510 size=76
    let mut pc: u32 = 0x82EAA510;
    'dispatch: loop {
        match pc {
            0x82EAA510 => {
    //   block [0x82EAA510..0x82EAA55C)
	// 82EAA510: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 82EAA514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EAA518: 7C841850  subf r4, r4, r3
	ctx.r[4].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82EAA51C: 7D4438AE  lbzx r10, r4, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82EAA520: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAA524: 409A0010  bne cr6, 0x82eaa534
	if !ctx.cr[6].eq {
	pc = 0x82EAA534; continue 'dispatch;
	}
	// 82EAA528: 89670000  lbz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA52C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAA530: 419A00D4  beq cr6, 0x82eaa604
	if ctx.cr[6].eq {
		sub_82EAA604(ctx, base);
		return;
	}
	// 82EAA534: 7F062800  cmpw cr6, r6, r5
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EAA538: 409800CC  bge cr6, 0x82eaa604
	if !ctx.cr[6].lt {
		sub_82EAA604(ctx, base);
		return;
	}
	// 82EAA53C: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82EAA540: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA544: 41980018  blt cr6, 0x82eaa55c
	if ctx.cr[6].lt {
		sub_82EAA55C(ctx, base);
		return;
	}
	// 82EAA548: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA54C: 41990010  bgt cr6, 0x82eaa55c
	if ctx.cr[6].gt {
		sub_82EAA55C(ctx, base);
		return;
	}
	// 82EAA550: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA554: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 82EAA558: 48000008  b 0x82eaa560
	sub_82EAA55C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA55C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA55C size=40
    let mut pc: u32 = 0x82EAA55C;
    'dispatch: loop {
        match pc {
            0x82EAA55C => {
    //   block [0x82EAA55C..0x82EAA584)
	// 82EAA55C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82EAA560: 89270000  lbz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA564: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82EAA568: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA56C: 41980018  blt cr6, 0x82eaa584
	if ctx.cr[6].lt {
		sub_82EAA584(ctx, base);
		return;
	}
	// 82EAA570: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA574: 41990010  bgt cr6, 0x82eaa584
	if ctx.cr[6].gt {
		sub_82EAA584(ctx, base);
		return;
	}
	// 82EAA578: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA57C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA580: 48000008  b 0x82eaa588
	sub_82EAA584(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA584 size=80
    let mut pc: u32 = 0x82EAA584;
    'dispatch: loop {
        match pc {
            0x82EAA584 => {
    //   block [0x82EAA584..0x82EAA5D4)
	// 82EAA584: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82EAA588: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA58C: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82EAA590: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAA594: 41980060  blt cr6, 0x82eaa5f4
	if ctx.cr[6].lt {
		sub_82EAA5F4(ctx, base);
		return;
	}
	// 82EAA598: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82EAA59C: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA5A0: 41980014  blt cr6, 0x82eaa5b4
	if ctx.cr[6].lt {
	pc = 0x82EAA5B4; continue 'dispatch;
	}
	// 82EAA5A4: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA5A8: 4199000C  bgt cr6, 0x82eaa5b4
	if ctx.cr[6].gt {
	pc = 0x82EAA5B4; continue 'dispatch;
	}
	// 82EAA5AC: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA5B0: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82EAA5B4: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82EAA5B8: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA5BC: 41980018  blt cr6, 0x82eaa5d4
	if ctx.cr[6].lt {
		sub_82EAA5D4(ctx, base);
		return;
	}
	// 82EAA5C0: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA5C4: 41990010  bgt cr6, 0x82eaa5d4
	if ctx.cr[6].gt {
		sub_82EAA5D4(ctx, base);
		return;
	}
	// 82EAA5C8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA5CC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA5D0: 48000008  b 0x82eaa5d8
	sub_82EAA5D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA5D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA5D4 size=32
    let mut pc: u32 = 0x82EAA5D4;
    'dispatch: loop {
        match pc {
            0x82EAA5D4 => {
    //   block [0x82EAA5D4..0x82EAA5F4)
	// 82EAA5D4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82EAA5D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAA5DC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EAA5E0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAA5E4: 41990018  bgt cr6, 0x82eaa5fc
	if ctx.cr[6].gt {
		sub_82EAA5FC(ctx, base);
		return;
	}
	// 82EAA5E8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82EAA5EC: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82EAA5F0: 4BFFFF2C  b 0x82eaa51c
	sub_82EAA510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA5F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA5F4 size=8
    let mut pc: u32 = 0x82EAA5F4;
    'dispatch: loop {
        match pc {
            0x82EAA5F4 => {
    //   block [0x82EAA5F4..0x82EAA5FC)
	// 82EAA5F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EAA5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA5FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA5FC size=8
    let mut pc: u32 = 0x82EAA5FC;
    'dispatch: loop {
        match pc {
            0x82EAA5FC => {
    //   block [0x82EAA5FC..0x82EAA604)
	// 82EAA5FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EAA600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA604(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA604 size=8
    let mut pc: u32 = 0x82EAA604;
    'dispatch: loop {
        match pc {
            0x82EAA604 => {
    //   block [0x82EAA604..0x82EAA60C)
	// 82EAA604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAA608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA610 size=28
    let mut pc: u32 = 0x82EAA610;
    'dispatch: loop {
        match pc {
            0x82EAA610 => {
    //   block [0x82EAA610..0x82EAA62C)
	// 82EAA610: 7D641850  subf r11, r4, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82EAA614: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA618: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAA61C: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 82EAA620: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82EAA624: 409AFFF0  bne cr6, 0x82eaa614
	if !ctx.cr[6].eq {
	pc = 0x82EAA614; continue 'dispatch;
	}
	// 82EAA628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA630 size=8
    let mut pc: u32 = 0x82EAA630;
    'dispatch: loop {
        match pc {
            0x82EAA630 => {
    //   block [0x82EAA630..0x82EAA638)
	// 82EAA630: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EAA634: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA638 size=8
    let mut pc: u32 = 0x82EAA638;
    'dispatch: loop {
        match pc {
            0x82EAA638 => {
    //   block [0x82EAA638..0x82EAA640)
	// 82EAA638: 48301EB8  b 0x831ac4f0
	sub_831AC4F0(ctx, base);
	return;
	// 82EAA63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA640 size=36
    let mut pc: u32 = 0x82EAA640;
    'dispatch: loop {
        match pc {
            0x82EAA640 => {
    //   block [0x82EAA640..0x82EAA664)
	// 82EAA640: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EAA644: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA648: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAA64C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAA650: 409AFFF4  bne cr6, 0x82eaa644
	if !ctx.cr[6].eq {
	pc = 0x82EAA644; continue 'dispatch;
	}
	// 82EAA654: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82EAA658: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EAA65C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAA660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA668 size=12
    let mut pc: u32 = 0x82EAA668;
    'dispatch: loop {
        match pc {
            0x82EAA668 => {
    //   block [0x82EAA668..0x82EAA674)
	// 82EAA668: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EAA66C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EAA670: 482FFEC0  b 0x831aa530
	sub_831AA530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA678 size=40
    let mut pc: u32 = 0x82EAA678;
    'dispatch: loop {
        match pc {
            0x82EAA678 => {
    //   block [0x82EAA678..0x82EAA6A0)
	// 82EAA678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA684: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EAA688: 48300061  bl 0x831aa6e8
	ctx.lr = 0x82EAA68C;
	sub_831AA6E8(ctx, base);
	// 82EAA68C: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EAA690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA6A0 size=4
    let mut pc: u32 = 0x82EAA6A0;
    'dispatch: loop {
        match pc {
            0x82EAA6A0 => {
    //   block [0x82EAA6A0..0x82EAA6A4)
	// 82EAA6A0: 48302240  b 0x831ac8e0
	sub_831AC8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA6A8 size=4
    let mut pc: u32 = 0x82EAA6A8;
    'dispatch: loop {
        match pc {
            0x82EAA6A8 => {
    //   block [0x82EAA6A8..0x82EAA6AC)
	// 82EAA6A8: 48302778  b 0x831ace20
	sub_831ACE20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA6B0 size=4
    let mut pc: u32 = 0x82EAA6B0;
    'dispatch: loop {
        match pc {
            0x82EAA6B0 => {
    //   block [0x82EAA6B0..0x82EAA6B4)
	// 82EAA6B0: 48303DE0  b 0x831ae490
	sub_831AE490(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA6B8 size=12
    let mut pc: u32 = 0x82EAA6B8;
    'dispatch: loop {
        match pc {
            0x82EAA6B8 => {
    //   block [0x82EAA6B8..0x82EAA6C4)
	// 82EAA6B8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA6BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAA6C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA6C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA6C4 size=60
    let mut pc: u32 = 0x82EAA6C4;
    'dispatch: loop {
        match pc {
            0x82EAA6C4 => {
    //   block [0x82EAA6C4..0x82EAA700)
	// 82EAA6C4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EAA6C8: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA6CC: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82EAA6D0: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82EAA6D4: 41980014  blt cr6, 0x82eaa6e8
	if ctx.cr[6].lt {
	pc = 0x82EAA6E8; continue 'dispatch;
	}
	// 82EAA6D8: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82EAA6DC: 4199000C  bgt cr6, 0x82eaa6e8
	if ctx.cr[6].gt {
	pc = 0x82EAA6E8; continue 'dispatch;
	}
	// 82EAA6E0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EAA6E4: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82EAA6E8: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82EAA6EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EAA6F0: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA6F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAA6F8: 409AFFD0  bne cr6, 0x82eaa6c8
	if !ctx.cr[6].eq {
	pc = 0x82EAA6C8; continue 'dispatch;
	}
	// 82EAA6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA700 size=12
    let mut pc: u32 = 0x82EAA700;
    'dispatch: loop {
        match pc {
            0x82EAA700 => {
    //   block [0x82EAA700..0x82EAA70C)
	// 82EAA700: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAA708: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA70C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA70C size=60
    let mut pc: u32 = 0x82EAA70C;
    'dispatch: loop {
        match pc {
            0x82EAA70C => {
    //   block [0x82EAA70C..0x82EAA748)
	// 82EAA70C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EAA710: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA714: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82EAA718: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 82EAA71C: 41980014  blt cr6, 0x82eaa730
	if ctx.cr[6].lt {
	pc = 0x82EAA730; continue 'dispatch;
	}
	// 82EAA720: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 82EAA724: 4199000C  bgt cr6, 0x82eaa730
	if ctx.cr[6].gt {
	pc = 0x82EAA730; continue 'dispatch;
	}
	// 82EAA728: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 82EAA72C: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82EAA730: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82EAA734: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EAA738: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA73C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAA740: 409AFFD0  bne cr6, 0x82eaa710
	if !ctx.cr[6].eq {
	pc = 0x82EAA710; continue 'dispatch;
	}
	// 82EAA744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA748 size=4
    let mut pc: u32 = 0x82EAA748;
    'dispatch: loop {
        match pc {
            0x82EAA748 => {
    //   block [0x82EAA748..0x82EAA74C)
	// 82EAA748: 482FDDC8  b 0x831a8510
	sub_831A8510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA750 size=4
    let mut pc: u32 = 0x82EAA750;
    'dispatch: loop {
        match pc {
            0x82EAA750 => {
    //   block [0x82EAA750..0x82EAA754)
	// 82EAA750: 48302C00  b 0x831ad350
	sub_831AD350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA758 size=4
    let mut pc: u32 = 0x82EAA758;
    'dispatch: loop {
        match pc {
            0x82EAA758 => {
    //   block [0x82EAA758..0x82EAA75C)
	// 82EAA758: 482FDA88  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA760 size=20
    let mut pc: u32 = 0x82EAA760;
    'dispatch: loop {
        match pc {
            0x82EAA760 => {
    //   block [0x82EAA760..0x82EAA774)
	// 82EAA760: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EAA764: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82EAA768: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAA76C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EAA770: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA774 size=20
    let mut pc: u32 = 0x82EAA774;
    'dispatch: loop {
        match pc {
            0x82EAA774 => {
    //   block [0x82EAA774..0x82EAA788)
	// 82EAA774: 7D2B2A14  add r9, r11, r5
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82EAA778: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA77C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA780: 7C674051  subf. r3, r7, r8
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EAA784: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAA788 size=20
    let mut pc: u32 = 0x82EAA788;
    'dispatch: loop {
        match pc {
            0x82EAA788 => {
    //   block [0x82EAA788..0x82EAA79C)
	// 82EAA788: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAA78C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EAA790: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EAA794: 409AFFE4  bne cr6, 0x82eaa778
	if !ctx.cr[6].eq {
		sub_82EAA774(ctx, base);
		return;
	}
	// 82EAA798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA7A0 size=124
    let mut pc: u32 = 0x82EAA7A0;
    'dispatch: loop {
        match pc {
            0x82EAA7A0 => {
    //   block [0x82EAA7A0..0x82EAA81C)
	// 82EAA7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAA7A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAA7AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA7B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAA7B4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAA7B8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA7BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAA7C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAA7C4: 409AFFF4  bne cr6, 0x82eaa7b8
	if !ctx.cr[6].eq {
	pc = 0x82EAA7B8; continue 'dispatch;
	}
	// 82EAA7C8: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82EAA7CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA7D0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAA7D4: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82EAA7D8: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAA7DC: 550B003E  slwi r11, r8, 0
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAA7E0: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82EAA7E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAA7E8: 4BFF6179  bl 0x82ea0960
	ctx.lr = 0x82EAA7EC;
	sub_82EA0960(ctx, base);
	// 82EAA7EC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAA7F0: 7D5F1850  subf r10, r31, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82EAA7F4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA7F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EAA7FC: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82EAA800: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAA804: 409AFFF0  bne cr6, 0x82eaa7f4
	if !ctx.cr[6].eq {
	pc = 0x82EAA7F4; continue 'dispatch;
	}
	// 82EAA808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAA80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAA810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAA814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAA818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA820 size=128
    let mut pc: u32 = 0x82EAA820;
    'dispatch: loop {
        match pc {
            0x82EAA820 => {
    //   block [0x82EAA820..0x82EAA8A0)
	// 82EAA820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA824: 482FD949  bl 0x831a816c
	ctx.lr = 0x82EAA828;
	sub_831A8130(ctx, base);
	// 82EAA828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA82C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EAA830: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EAA834: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA838: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAA83C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAA840: 409AFFF4  bne cr6, 0x82eaa834
	if !ctx.cr[6].eq {
	pc = 0x82EAA834; continue 'dispatch;
	}
	// 82EAA844: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EAA848: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EAA84C: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EAA850: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82EAA854: 40990008  ble cr6, 0x82eaa85c
	if !ctx.cr[6].gt {
	pc = 0x82EAA85C; continue 'dispatch;
	}
	// 82EAA858: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAA85C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA860: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAA864: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAA868: 389F0001  addi r4, r31, 1
	ctx.r[4].s64 = ctx.r[31].s64 + 1;
	// 82EAA86C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAA870: 4BFF60F1  bl 0x82ea0960
	ctx.lr = 0x82EAA874;
	sub_82EA0960(ctx, base);
	// 82EAA874: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAA878: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EAA87C: 419A0010  beq cr6, 0x82eaa88c
	if ctx.cr[6].eq {
	pc = 0x82EAA88C; continue 'dispatch;
	}
	// 82EAA880: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAA884: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EAA888: 48301C69  bl 0x831ac4f0
	ctx.lr = 0x82EAA88C;
	sub_831AC4F0(ctx, base);
	// 82EAA88C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAA890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAA894: 7D7EF9AE  stbx r11, r30, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 82EAA898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAA89C: 482FD920  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA8A0 size=212
    let mut pc: u32 = 0x82EAA8A0;
    'dispatch: loop {
        match pc {
            0x82EAA8A0 => {
    //   block [0x82EAA8A0..0x82EAA974)
	// 82EAA8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA8A4: 482FD8B1  bl 0x831a8154
	ctx.lr = 0x82EAA8A8;
	sub_831A8130(ctx, base);
	// 82EAA8A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA8AC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EAA8B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EAA8B4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EAA8B8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82EAA8BC: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82EAA8C0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82EAA8C4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA8C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAA8CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAA8D0: 409AFFF4  bne cr6, 0x82eaa8c4
	if !ctx.cr[6].eq {
	pc = 0x82EAA8C4; continue 'dispatch;
	}
	// 82EAA8D4: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82EAA8D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EAA8DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EAA8E0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EAA8E4: 5578003E  slwi r24, r11, 0
	ctx.r[24].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82EAA8E8: 48301FF9  bl 0x831ac8e0
	ctx.lr = 0x82EAA8EC;
	sub_831AC8E0(ctx, base);
	// 82EAA8EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAA8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAA8F4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EAA8F8: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EAA8FC: 419A006C  beq cr6, 0x82eaa968
	if ctx.cr[6].eq {
	pc = 0x82EAA968; continue 'dispatch;
	}
	// 82EAA900: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82EAA904: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAA908: 7FBAF050  subf r29, r26, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[26].s64;
	// 82EAA90C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAA910: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAA914: 9B3C0000  stb r25, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82EAA918: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EAA91C: 409A0010  bne cr6, 0x82eaa92c
	if !ctx.cr[6].eq {
	pc = 0x82EAA92C; continue 'dispatch;
	}
	// 82EAA920: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EAA924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAA928: 4BFFBF59  bl 0x82ea6880
	ctx.lr = 0x82EAA92C;
	sub_82EA6880(ctx, base);
	// 82EAA92C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAA930: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82EAA934: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA938: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EAA93C: 7FA9512E  stwx r29, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82EAA940: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAA944: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EAA948: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EAA94C: 419A001C  beq cr6, 0x82eaa968
	if ctx.cr[6].eq {
	pc = 0x82EAA968; continue 'dispatch;
	}
	// 82EAA950: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EAA954: 7C7EC214  add r3, r30, r24
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 82EAA958: 48301F89  bl 0x831ac8e0
	ctx.lr = 0x82EAA95C;
	sub_831AC8E0(ctx, base);
	// 82EAA95C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAA960: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EAA964: 409AFFA0  bne cr6, 0x82eaa904
	if !ctx.cr[6].eq {
	pc = 0x82EAA904; continue 'dispatch;
	}
	// 82EAA968: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EAA96C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EAA970: 482FD834  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAA978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAA978 size=344
    let mut pc: u32 = 0x82EAA978;
    'dispatch: loop {
        match pc {
            0x82EAA978 => {
    //   block [0x82EAA978..0x82EAAAD0)
	// 82EAA978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAA97C: 482FD7ED  bl 0x831a8168
	ctx.lr = 0x82EAA980;
	sub_831A8130(ctx, base);
	// 82EAA980: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82EAA984: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EAA988: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EAA98C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EAA990: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EAA994: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EAA998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAA99C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAA9A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EAA9A4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAA9A8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAA9AC: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 82EAA9B0: 4098002C  bge cr6, 0x82eaa9dc
	if !ctx.cr[6].lt {
	pc = 0x82EAA9DC; continue 'dispatch;
	}
	// 82EAA9B4: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 82EAA9B8: 40980024  bge cr6, 0x82eaa9dc
	if !ctx.cr[6].lt {
	pc = 0x82EAA9DC; continue 'dispatch;
	}
	// 82EAA9BC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAA9C0: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 82EAA9C4: 41990008  bgt cr6, 0x82eaa9cc
	if ctx.cr[6].gt {
	pc = 0x82EAA9CC; continue 'dispatch;
	}
	// 82EAA9C8: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 82EAA9CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAA9D0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAA9D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAA9D8: 4BFFBE21  bl 0x82ea67f8
	ctx.lr = 0x82EAA9DC;
	sub_82EA67F8(ctx, base);
	// 82EAA9DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EAA9E0: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82EAA9E4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAA9E8: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EAA9EC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAA9F0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EAA9F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EAA9F8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAA9FC: 557F00BE  clrlwi r31, r11, 2
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAAA00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAAA04: 483032C5  bl 0x831adcc8
	ctx.lr = 0x82EAAA08;
	sub_831ADCC8(ctx, base);
	// 82EAAA08: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EAAA0C: 41980048  blt cr6, 0x82eaaa54
	if ctx.cr[6].lt {
	pc = 0x82EAAA54; continue 'dispatch;
	}
	// 82EAAA10: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAAA14: 4198008C  blt cr6, 0x82eaaaa0
	if ctx.cr[6].lt {
	pc = 0x82EAAAA0; continue 'dispatch;
	}
	// 82EAAA18: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAAA1C: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82EAAA20: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAAA24: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAAA28: 40980024  bge cr6, 0x82eaaa4c
	if !ctx.cr[6].lt {
	pc = 0x82EAAA4C; continue 'dispatch;
	}
	// 82EAAA2C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAAA30: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAAA34: 41980008  blt cr6, 0x82eaaa3c
	if ctx.cr[6].lt {
	pc = 0x82EAAA3C; continue 'dispatch;
	}
	// 82EAAA38: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAAA3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAAA40: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAAA44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAAA48: 4BFFBDB1  bl 0x82ea67f8
	ctx.lr = 0x82EAAA4C;
	sub_82EA67F8(ctx, base);
	// 82EAAA4C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EAAA50: 4BFFFF9C  b 0x82eaa9ec
	pc = 0x82EAA9EC; continue 'dispatch;
	// 82EAAA54: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAAA58: 2F0B00FF  cmpwi cr6, r11, 0xff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 255, &mut ctx.xer);
	// 82EAAA5C: 41990008  bgt cr6, 0x82eaaa64
	if ctx.cr[6].gt {
	pc = 0x82EAAA64; continue 'dispatch;
	}
	// 82EAAA60: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 82EAAA64: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAAA68: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82EAAA6C: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAAA70: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAAA74: 4098FFD8  bge cr6, 0x82eaaa4c
	if !ctx.cr[6].lt {
	pc = 0x82EAAA4C; continue 'dispatch;
	}
	// 82EAAA78: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAAA7C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAAA80: 4198FFBC  blt cr6, 0x82eaaa3c
	if ctx.cr[6].lt {
	pc = 0x82EAAA3C; continue 'dispatch;
	}
	// 82EAAA84: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAAA88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAAA8C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAAA90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAAA94: 4BFFBD65  bl 0x82ea67f8
	ctx.lr = 0x82EAAA98;
	sub_82EA67F8(ctx, base);
	// 82EAAA98: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EAAA9C: 4BFFFF50  b 0x82eaa9ec
	pc = 0x82EAA9EC; continue 'dispatch;
	// 82EAAAA0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAAAA4: 38C30001  addi r6, r3, 1
	ctx.r[6].s64 = ctx.r[3].s64 + 1;
	// 82EAAAA8: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAAAAC: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82EAAAB0: 7F065000  cmpw cr6, r6, r10
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAAAB4: 41990014  bgt cr6, 0x82eaaac8
	if ctx.cr[6].gt {
	pc = 0x82EAAAC8; continue 'dispatch;
	}
	// 82EAAAB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EAAABC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EAAAC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAAAC4: 4BFFBE55  bl 0x82ea6918
	ctx.lr = 0x82EAAAC8;
	sub_82EA6918(ctx, base);
	// 82EAAAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAAACC: 482FD6EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAAD0 size=72
    let mut pc: u32 = 0x82EAAAD0;
    'dispatch: loop {
        match pc {
            0x82EAAAD0 => {
    //   block [0x82EAAAD0..0x82EAAB18)
	// 82EAAAD0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAAD4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82EAAAD8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAAADC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAAAE0: 40980030  bge cr6, 0x82eaab10
	if !ctx.cr[6].lt {
	pc = 0x82EAAB10; continue 'dispatch;
	}
	// 82EAAAE4: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82EAAAE8: 40980028  bge cr6, 0x82eaab10
	if !ctx.cr[6].lt {
	pc = 0x82EAAB10; continue 'dispatch;
	}
	// 82EAAAEC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAAF0: 7C880774  extsb r8, r4
	ctx.r[8].s64 = ctx.r[4].s8 as i64;
	// 82EAAAF4: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAAAF8: 7CE50774  extsb r5, r7
	ctx.r[5].s64 = ctx.r[7].s8 as i64;
	// 82EAAAFC: 7F054000  cmpw cr6, r5, r8
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EAAB00: 419A0018  beq cr6, 0x82eaab18
	if ctx.cr[6].eq {
		sub_82EAAB18(ctx, base);
		return;
	}
	// 82EAAB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAAB08: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAAB0C: 4198FFD8  blt cr6, 0x82eaaae4
	if ctx.cr[6].lt {
	pc = 0x82EAAAE4; continue 'dispatch;
	}
	// 82EAAB10: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EAAB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAB18 size=8
    let mut pc: u32 = 0x82EAAB18;
    'dispatch: loop {
        match pc {
            0x82EAAB18 => {
    //   block [0x82EAAB18..0x82EAAB20)
	// 82EAAB18: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EAAB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAB20 size=76
    let mut pc: u32 = 0x82EAAB20;
    'dispatch: loop {
        match pc {
            0x82EAAB20 => {
    //   block [0x82EAAB20..0x82EAAB6C)
	// 82EAAB20: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAB24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EAAB28: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAAB2C: 40990008  ble cr6, 0x82eaab34
	if !ctx.cr[6].gt {
	pc = 0x82EAAB34; continue 'dispatch;
	}
	// 82EAAB30: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82EAAB34: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 82EAAB38: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EAAB3C: 41980028  blt cr6, 0x82eaab64
	if ctx.cr[6].lt {
	pc = 0x82EAAB64; continue 'dispatch;
	}
	// 82EAAB40: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAB44: 7C890774  extsb r9, r4
	ctx.r[9].s64 = ctx.r[4].s8 as i64;
	// 82EAAB48: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAAB4C: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82EAAB50: 7F074800  cmpw cr6, r7, r9
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EAAB54: 419A0018  beq cr6, 0x82eaab6c
	if ctx.cr[6].eq {
		sub_82EAAB6C(ctx, base);
		return;
	}
	// 82EAAB58: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EAAB5C: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EAAB60: 4098FFE8  bge cr6, 0x82eaab48
	if !ctx.cr[6].lt {
	pc = 0x82EAAB48; continue 'dispatch;
	}
	// 82EAAB64: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EAAB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAB6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAB6C size=8
    let mut pc: u32 = 0x82EAAB6C;
    'dispatch: loop {
        match pc {
            0x82EAAB6C => {
    //   block [0x82EAAB6C..0x82EAAB74)
	// 82EAAB6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EAAB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAAB78 size=128
    let mut pc: u32 = 0x82EAAB78;
    'dispatch: loop {
        match pc {
            0x82EAAB78 => {
    //   block [0x82EAAB78..0x82EAABF8)
	// 82EAAB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAAB7C: 482FD5E9  bl 0x831a8164
	ctx.lr = 0x82EAAB80;
	sub_831A8130(ctx, base);
	// 82EAAB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAAB84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAAB88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EAAB8C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAB90: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAB94: 3B6AFFFF  addi r27, r10, -1
	ctx.r[27].s64 = ctx.r[10].s64 + -1;
	// 82EAAB98: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 82EAAB9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAABA0: 7D5CDA14  add r10, r28, r27
	ctx.r[10].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82EAABA4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAABA8: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 82EAABAC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAABB0: 40980024  bge cr6, 0x82eaabd4
	if !ctx.cr[6].lt {
	pc = 0x82EAABD4; continue 'dispatch;
	}
	// 82EAABB4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAABB8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAABBC: 41980008  blt cr6, 0x82eaabc4
	if ctx.cr[6].lt {
	pc = 0x82EAABC4; continue 'dispatch;
	}
	// 82EAABC0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EAABC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAABC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAABCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAABD0: 4BFFBC29  bl 0x82ea67f8
	ctx.lr = 0x82EAABD4;
	sub_82EA67F8(ctx, base);
	// 82EAABD4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EAABD8: 38BC0001  addi r5, r28, 1
	ctx.r[5].s64 = ctx.r[28].s64 + 1;
	// 82EAABDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAABE0: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAABE4: 7C6BDA14  add r3, r11, r27
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EAABE8: 482FD929  bl 0x831a8510
	ctx.lr = 0x82EAABEC;
	sub_831A8510(ctx, base);
	// 82EAABEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAABF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAABF4: 482FD5C0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAABF8 size=152
    let mut pc: u32 = 0x82EAABF8;
    'dispatch: loop {
        match pc {
            0x82EAABF8 => {
    //   block [0x82EAABF8..0x82EAAC90)
	// 82EAABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAABFC: 482FD569  bl 0x831a8164
	ctx.lr = 0x82EAAC00;
	sub_831A8130(ctx, base);
	// 82EAAC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAAC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAAC08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EAAC0C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82EAAC10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAC14: 3B6AFFFF  addi r27, r10, -1
	ctx.r[27].s64 = ctx.r[10].s64 + -1;
	// 82EAAC18: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAC1C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAAC20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAAC24: 409AFFF4  bne cr6, 0x82eaac18
	if !ctx.cr[6].eq {
	pc = 0x82EAAC18; continue 'dispatch;
	}
	// 82EAAC28: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82EAAC2C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAAC30: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82EAAC34: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAAC38: 553D003E  slwi r29, r9, 0
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EAAC3C: 7D5DDA14  add r10, r29, r27
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[27].u64;
	// 82EAAC40: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 82EAAC44: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAAC48: 40980024  bge cr6, 0x82eaac6c
	if !ctx.cr[6].lt {
	pc = 0x82EAAC6C; continue 'dispatch;
	}
	// 82EAAC4C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAAC50: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAAC54: 41980008  blt cr6, 0x82eaac5c
	if ctx.cr[6].lt {
	pc = 0x82EAAC5C; continue 'dispatch;
	}
	// 82EAAC58: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EAAC5C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAAC60: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAAC64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAAC68: 4BFFBB91  bl 0x82ea67f8
	ctx.lr = 0x82EAAC6C;
	sub_82EA67F8(ctx, base);
	// 82EAAC6C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EAAC70: 38BD0001  addi r5, r29, 1
	ctx.r[5].s64 = ctx.r[29].s64 + 1;
	// 82EAAC74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAC78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EAAC7C: 7C6BDA14  add r3, r11, r27
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EAAC80: 482FD891  bl 0x831a8510
	ctx.lr = 0x82EAAC84;
	sub_831A8510(ctx, base);
	// 82EAAC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAAC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAAC8C: 482FD528  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAC90 size=16
    let mut pc: u32 = 0x82EAAC90;
    'dispatch: loop {
        match pc {
            0x82EAAC90 => {
    //   block [0x82EAAC90..0x82EAACA0)
	// 82EAAC90: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAC94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAAC98: 352AFFFF  addic. r9, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EAAC9C: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAACA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAACA0 size=64
    let mut pc: u32 = 0x82EAACA0;
    'dispatch: loop {
        match pc {
            0x82EAACA0 => {
    //   block [0x82EAACA0..0x82EAACE0)
	// 82EAACA0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAACA4: 7D2858AE  lbzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAACA8: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82EAACAC: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 82EAACB0: 41980014  blt cr6, 0x82eaacc4
	if ctx.cr[6].lt {
	pc = 0x82EAACC4; continue 'dispatch;
	}
	// 82EAACB4: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 82EAACB8: 4199000C  bgt cr6, 0x82eaacc4
	if ctx.cr[6].gt {
	pc = 0x82EAACC4; continue 'dispatch;
	}
	// 82EAACBC: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 82EAACC0: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	// 82EAACC4: 7D2859AE  stbx r9, r8, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82EAACC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAACCC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAACD0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAACD4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAACD8: 4198FFC8  blt cr6, 0x82eaaca0
	if ctx.cr[6].lt {
	pc = 0x82EAACA0; continue 'dispatch;
	}
	// 82EAACDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAACE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAACE0 size=16
    let mut pc: u32 = 0x82EAACE0;
    'dispatch: loop {
        match pc {
            0x82EAACE0 => {
    //   block [0x82EAACE0..0x82EAACF0)
	// 82EAACE0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAACE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAACE8: 352AFFFF  addic. r9, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EAACEC: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAACF0 size=64
    let mut pc: u32 = 0x82EAACF0;
    'dispatch: loop {
        match pc {
            0x82EAACF0 => {
    //   block [0x82EAACF0..0x82EAAD30)
	// 82EAACF0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAACF4: 7D2858AE  lbzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAACF8: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82EAACFC: 2F0A0041  cmpwi cr6, r10, 0x41
	ctx.cr[6].compare_i32(ctx.r[10].s32, 65, &mut ctx.xer);
	// 82EAAD00: 41980014  blt cr6, 0x82eaad14
	if ctx.cr[6].lt {
	pc = 0x82EAAD14; continue 'dispatch;
	}
	// 82EAAD04: 2F0A005A  cmpwi cr6, r10, 0x5a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 90, &mut ctx.xer);
	// 82EAAD08: 4199000C  bgt cr6, 0x82eaad14
	if ctx.cr[6].gt {
	pc = 0x82EAAD14; continue 'dispatch;
	}
	// 82EAAD0C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82EAAD10: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	// 82EAAD14: 7D2859AE  stbx r9, r8, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82EAAD18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAAD1C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAD20: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAAD24: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAAD28: 4198FFC8  blt cr6, 0x82eaacf0
	if ctx.cr[6].lt {
	pc = 0x82EAACF0; continue 'dispatch;
	}
	// 82EAAD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAD30 size=80
    let mut pc: u32 = 0x82EAAD30;
    'dispatch: loop {
        match pc {
            0x82EAAD30 => {
    //   block [0x82EAAD30..0x82EAAD80)
	// 82EAAD30: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAD34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAAD38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAAD3C: 419A0038  beq cr6, 0x82eaad74
	if ctx.cr[6].eq {
	pc = 0x82EAAD74; continue 'dispatch;
	}
	// 82EAAD40: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAD44: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAAD48: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAAD4C: 40980034  bge cr6, 0x82eaad80
	if !ctx.cr[6].lt {
		sub_82EAAD80(ctx, base);
		return;
	}
	// 82EAAD50: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAD54: 7D0B28AE  lbzx r8, r11, r5
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82EAAD58: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAAD5C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EAAD60: 409A0020  bne cr6, 0x82eaad80
	if !ctx.cr[6].eq {
		sub_82EAAD80(ctx, base);
		return;
	}
	// 82EAAD64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAAD68: 7D2B28AE  lbzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82EAAD6C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EAAD70: 409AFFD8  bne cr6, 0x82eaad48
	if !ctx.cr[6].eq {
	pc = 0x82EAAD48; continue 'dispatch;
	}
	// 82EAAD74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAAD78: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAAD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAD80 size=12
    let mut pc: u32 = 0x82EAAD80;
    'dispatch: loop {
        match pc {
            0x82EAAD80 => {
    //   block [0x82EAAD80..0x82EAAD8C)
	// 82EAAD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAAD84: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAAD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAD90 size=36
    let mut pc: u32 = 0x82EAAD90;
    'dispatch: loop {
        match pc {
            0x82EAAD90 => {
    //   block [0x82EAAD90..0x82EAADB4)
	// 82EAAD90: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAD94: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAD98: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82EAAD9C: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 82EAADA0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EAADA4: 40980010  bge cr6, 0x82eaadb4
	if !ctx.cr[6].lt {
		sub_82EAADB4(ctx, base);
		return;
	}
	// 82EAADA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAADAC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAADB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAADB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAADB4 size=68
    let mut pc: u32 = 0x82EAADB4;
    'dispatch: loop {
        match pc {
            0x82EAADB4 => {
    //   block [0x82EAADB4..0x82EAADF8)
	// 82EAADB4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EAADB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAADBC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EAADC0: 4099002C  ble cr6, 0x82eaadec
	if !ctx.cr[6].gt {
	pc = 0x82EAADEC; continue 'dispatch;
	}
	// 82EAADC4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAADC8: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAADCC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EAADD0: 7D2758AE  lbzx r9, r7, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAADD4: 7CCA58AE  lbzx r6, r10, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAADD8: 7F064840  cmplw cr6, r6, r9
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EAADDC: 409AFFCC  bne cr6, 0x82eaada8
	if !ctx.cr[6].eq {
		sub_82EAAD90(ctx, base);
		return;
	}
	// 82EAADE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAADE4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EAADE8: 4198FFE8  blt cr6, 0x82eaadd0
	if ctx.cr[6].lt {
	pc = 0x82EAADD0; continue 'dispatch;
	}
	// 82EAADEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAADF0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAADF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAADF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAADF8 size=64
    let mut pc: u32 = 0x82EAADF8;
    'dispatch: loop {
        match pc {
            0x82EAADF8 => {
    //   block [0x82EAADF8..0x82EAAE38)
	// 82EAADF8: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82EAADFC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82EAAE00: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAE04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAAE08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAAE0C: 409AFFF4  bne cr6, 0x82eaae00
	if !ctx.cr[6].eq {
	pc = 0x82EAAE00; continue 'dispatch;
	}
	// 82EAAE10: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82EAAE14: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAE18: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82EAAE1C: 5507003E  slwi r7, r8, 0
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EAAE20: 7CC75050  subf r6, r7, r10
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82EAAE24: 3566FFFF  addic. r11, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAAE28: 40800010  bge 0x82eaae38
	if !ctx.cr[0].lt {
		sub_82EAAE38(ctx, base);
		return;
	}
	// 82EAAE2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAAE30: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAAE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAE38 size=76
    let mut pc: u32 = 0x82EAAE38;
    'dispatch: loop {
        match pc {
            0x82EAAE38 => {
    //   block [0x82EAAE38..0x82EAAE84)
	// 82EAAE38: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAE3C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EAAE40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAAE44: 419A0034  beq cr6, 0x82eaae78
	if ctx.cr[6].eq {
	pc = 0x82EAAE78; continue 'dispatch;
	}
	// 82EAAE48: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAE4C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82EAAE50: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAE54: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82EAAE58: 7F075000  cmpw cr6, r7, r10
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAAE5C: 409AFFD0  bne cr6, 0x82eaae2c
	if !ctx.cr[6].eq {
		sub_82EAADF8(ctx, base);
		return;
	}
	// 82EAAE60: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EAAE64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAAE68: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAE6C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EAAE70: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAAE74: 409AFFDC  bne cr6, 0x82eaae50
	if !ctx.cr[6].eq {
	pc = 0x82EAAE50; continue 'dispatch;
	}
	// 82EAAE78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAAE7C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAAE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAE88 size=20
    let mut pc: u32 = 0x82EAAE88;
    'dispatch: loop {
        match pc {
            0x82EAAE88 => {
    //   block [0x82EAAE88..0x82EAAE9C)
	// 82EAAE88: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAE8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAAE90: 352AFFFF  addic. r9, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EAAE94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAAE98: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAE9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAE9C size=44
    let mut pc: u32 = 0x82EAAE9C;
    'dispatch: loop {
        match pc {
            0x82EAAE9C => {
    //   block [0x82EAAE9C..0x82EAAEC8)
	// 82EAAE9C: 7CA80774  extsb r8, r5
	ctx.r[8].s64 = ctx.r[5].s8 as i64;
	// 82EAAEA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EAAEA4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAEA8: 7CAA58AE  lbzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAAEAC: 7CA50774  extsb r5, r5
	ctx.r[5].s64 = ctx.r[5].s8 as i64;
	// 82EAAEB0: 7F054000  cmpw cr6, r5, r8
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EAAEB4: 409A0014  bne cr6, 0x82eaaec8
	if !ctx.cr[6].eq {
		sub_82EAAEC8(ctx, base);
		return;
	}
	// 82EAAEB8: 7CCA59AE  stbx r6, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u8) };
	// 82EAAEBC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EAAEC0: 99230000  stb r9, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82EAAEC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAAEC8 size=24
    let mut pc: u32 = 0x82EAAEC8;
    'dispatch: loop {
        match pc {
            0x82EAAEC8 => {
    //   block [0x82EAAEC8..0x82EAAEE0)
	// 82EAAEC8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAECC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAAED0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAAED4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAAED8: 4198FFCC  blt cr6, 0x82eaaea4
	if ctx.cr[6].lt {
		sub_82EAAE9C(ctx, base);
		return;
	}
	// 82EAAEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAAEE0 size=232
    let mut pc: u32 = 0x82EAAEE0;
    'dispatch: loop {
        match pc {
            0x82EAAEE0 => {
    //   block [0x82EAAEE0..0x82EAAFC8)
	// 82EAAEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAAEE4: 482FD269  bl 0x831a814c
	ctx.lr = 0x82EAAEE8;
	sub_831A8130(ctx, base);
	// 82EAAEE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAAEEC: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82EAAEF0: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAEF4: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82EAAEF8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82EAAEFC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82EAAF00: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82EAAF04: 81570004  lwz r10, 4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAF08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EAAF0C: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAF10: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EAAF14: 3B4BFFFF  addi r26, r11, -1
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	// 82EAAF18: 3B0AFFFF  addi r24, r10, -1
	ctx.r[24].s64 = ctx.r[10].s64 + -1;
	// 82EAAF1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EAAF20: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EAAF24: 40990074  ble cr6, 0x82eaaf98
	if !ctx.cr[6].gt {
	pc = 0x82EAAF98; continue 'dispatch;
	}
	// 82EAAF28: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EAAF2C: 409A0010  bne cr6, 0x82eaaf3c
	if !ctx.cr[6].eq {
	pc = 0x82EAAF3C; continue 'dispatch;
	}
	// 82EAAF30: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAF34: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAF38: 48000020  b 0x82eaaf58
	pc = 0x82EAAF58; continue 'dispatch;
	// 82EAAF3C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAF40: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAAF44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EAAF48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAF4C: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EAAF50: 7D095050  subf r8, r9, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82EAAF54: 7FDA4050  subf r30, r26, r8
	ctx.r[30].s64 = ctx.r[8].s64 - ctx.r[26].s64;
	// 82EAAF58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EAAF5C: 7C9CB214  add r4, r28, r22
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	// 82EAAF60: 7C7BCA14  add r3, r27, r25
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 82EAAF64: 482FD5AD  bl 0x831a8510
	ctx.lr = 0x82EAAF68;
	sub_831A8510(ctx, base);
	// 82EAAF68: 7F7EDA14  add r27, r30, r27
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 82EAAF6C: 7D7ED214  add r11, r30, r26
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[26].u64;
	// 82EAAF70: 80970000  lwz r4, 0(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAF74: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82EAAF78: 7C7BCA14  add r3, r27, r25
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 82EAAF7C: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EAAF80: 482FD591  bl 0x831a8510
	ctx.lr = 0x82EAAF84;
	sub_831A8510(ctx, base);
	// 82EAAF84: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAF88: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EAAF8C: 7F78DA14  add r27, r24, r27
	ctx.r[27].u64 = ctx.r[24].u64 + ctx.r[27].u64;
	// 82EAAF90: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAAF94: 4198FF94  blt cr6, 0x82eaaf28
	if ctx.cr[6].lt {
	pc = 0x82EAAF28; continue 'dispatch;
	}
	// 82EAAF98: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAF9C: 7C9CB214  add r4, r28, r22
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	// 82EAAFA0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAFA4: 7C7BCA14  add r3, r27, r25
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 82EAAFA8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAAFAC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EAAFB0: 812AFFFC  lwz r9, -4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EAAFB4: 7D09A850  subf r8, r9, r21
	ctx.r[8].s64 = ctx.r[21].s64 - ctx.r[9].s64;
	// 82EAAFB8: 7CBA4050  subf r5, r26, r8
	ctx.r[5].s64 = ctx.r[8].s64 - ctx.r[26].s64;
	// 82EAAFBC: 482FD555  bl 0x831a8510
	ctx.lr = 0x82EAAFC0;
	sub_831A8510(ctx, base);
	// 82EAAFC0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EAAFC4: 482FD1D8  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAAFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAAFC8 size=128
    let mut pc: u32 = 0x82EAAFC8;
    'dispatch: loop {
        match pc {
            0x82EAAFC8 => {
    //   block [0x82EAAFC8..0x82EAB048)
	// 82EAAFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAAFCC: 482FD191  bl 0x831a815c
	ctx.lr = 0x82EAAFD0;
	sub_831A8130(ctx, base);
	// 82EAAFD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAAFD4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EAAFD8: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAAFDC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82EAAFE0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAAFE4: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAAFE8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EAAFEC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAFF0: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAAFF4: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82EAAFF8: 7C69402E  lwzx r3, r9, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EAAFFC: 3B8AFFFF  addi r28, r10, -1
	ctx.r[28].s64 = ctx.r[10].s64 + -1;
	// 82EAB000: 7D7CFA14  add r11, r28, r31
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82EAB004: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82EAB008: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EAB00C: 4BFF5725  bl 0x82ea0730
	ctx.lr = 0x82EAB010;
	sub_82EA0730(ctx, base);
	// 82EAB010: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAB014: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB018: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EAB01C: 482FD4F5  bl 0x831a8510
	ctx.lr = 0x82EAB020;
	sub_831A8510(ctx, base);
	// 82EAB020: 38BC0001  addi r5, r28, 1
	ctx.r[5].s64 = ctx.r[28].s64 + 1;
	// 82EAB024: 7C7DFA14  add r3, r29, r31
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82EAB028: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB02C: 482FD4E5  bl 0x831a8510
	ctx.lr = 0x82EAB030;
	sub_831A8510(ctx, base);
	// 82EAB030: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EAB034: 93DB0004  stw r30, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EAB038: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EAB03C: 93DB0008  stw r30, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EAB040: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EAB044: 482FD168  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB048 size=152
    let mut pc: u32 = 0x82EAB048;
    'dispatch: loop {
        match pc {
            0x82EAB048 => {
    //   block [0x82EAB048..0x82EAB0E0)
	// 82EAB048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB04C: 482FD111  bl 0x831a815c
	ctx.lr = 0x82EAB050;
	sub_831A8130(ctx, base);
	// 82EAB050: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB054: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82EAB058: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EAB05C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAB060: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82EAB064: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB068: 3B8AFFFF  addi r28, r10, -1
	ctx.r[28].s64 = ctx.r[10].s64 + -1;
	// 82EAB06C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB070: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAB074: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAB078: 409AFFF4  bne cr6, 0x82eab06c
	if !ctx.cr[6].eq {
	pc = 0x82EAB06C; continue 'dispatch;
	}
	// 82EAB07C: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82EAB080: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB084: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAB088: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82EAB08C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAB090: 551A003E  slwi r26, r8, 0
	ctx.r[26].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82EAB094: 7D7AE214  add r11, r26, r28
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[28].u64;
	// 82EAB098: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAB09C: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82EAB0A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EAB0A4: 4BFF568D  bl 0x82ea0730
	ctx.lr = 0x82EAB0A8;
	sub_82EA0730(ctx, base);
	// 82EAB0A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EAB0AC: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB0B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EAB0B4: 482FD45D  bl 0x831a8510
	ctx.lr = 0x82EAB0B8;
	sub_831A8510(ctx, base);
	// 82EAB0B8: 38BA0001  addi r5, r26, 1
	ctx.r[5].s64 = ctx.r[26].s64 + 1;
	// 82EAB0BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EAB0C0: 7C7DE214  add r3, r29, r28
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82EAB0C4: 482FD44D  bl 0x831a8510
	ctx.lr = 0x82EAB0C8;
	sub_831A8510(ctx, base);
	// 82EAB0C8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EAB0CC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EAB0D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAB0D4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EAB0D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EAB0DC: 482FD0D0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB0E0 size=200
    let mut pc: u32 = 0x82EAB0E0;
    'dispatch: loop {
        match pc {
            0x82EAB0E0 => {
    //   block [0x82EAB0E0..0x82EAB1A8)
	// 82EAB0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAB0E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAB0EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAB0F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB0F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB0F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAB0FC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAB100: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAB104: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAB108: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB10C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAB110: 4BFF5621  bl 0x82ea0730
	ctx.lr = 0x82EAB114;
	sub_82EA0730(ctx, base);
	// 82EAB114: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB118: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82EAB11C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAB120: 34E9FFFF  addic. r7, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EAB124: 40810048  ble 0x82eab16c
	if !ctx.cr[0].gt {
	pc = 0x82EAB16C; continue 'dispatch;
	}
	// 82EAB128: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB12C: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAB130: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82EAB134: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 82EAB138: 41980018  blt cr6, 0x82eab150
	if ctx.cr[6].lt {
	pc = 0x82EAB150; continue 'dispatch;
	}
	// 82EAB13C: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 82EAB140: 41990010  bgt cr6, 0x82eab150
	if ctx.cr[6].gt {
	pc = 0x82EAB150; continue 'dispatch;
	}
	// 82EAB144: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 82EAB148: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EAB14C: 48000008  b 0x82eab154
	pc = 0x82EAB154; continue 'dispatch;
	// 82EAB150: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82EAB154: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82EAB158: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAB15C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB160: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAB164: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAB168: 4198FFC0  blt cr6, 0x82eab128
	if ctx.cr[6].lt {
	pc = 0x82EAB128; continue 'dispatch;
	}
	// 82EAB16C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB174: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EAB178: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAB17C: 7D285A14  add r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82EAB180: 9949FFFF  stb r10, -1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 82EAB184: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB188: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EAB18C: 911E0008  stw r8, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EAB190: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAB194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAB198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAB19C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAB1A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAB1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB1A8 size=200
    let mut pc: u32 = 0x82EAB1A8;
    'dispatch: loop {
        match pc {
            0x82EAB1A8 => {
    //   block [0x82EAB1A8..0x82EAB270)
	// 82EAB1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAB1B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAB1B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAB1B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB1BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB1C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAB1C4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAB1C8: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAB1CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAB1D0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB1D4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAB1D8: 4BFF5559  bl 0x82ea0730
	ctx.lr = 0x82EAB1DC;
	sub_82EA0730(ctx, base);
	// 82EAB1DC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB1E0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82EAB1E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAB1E8: 34E9FFFF  addic. r7, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EAB1EC: 40810048  ble 0x82eab234
	if !ctx.cr[0].gt {
	pc = 0x82EAB234; continue 'dispatch;
	}
	// 82EAB1F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB1F4: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAB1F8: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82EAB1FC: 2F0A0041  cmpwi cr6, r10, 0x41
	ctx.cr[6].compare_i32(ctx.r[10].s32, 65, &mut ctx.xer);
	// 82EAB200: 41980018  blt cr6, 0x82eab218
	if ctx.cr[6].lt {
	pc = 0x82EAB218; continue 'dispatch;
	}
	// 82EAB204: 2F0A005A  cmpwi cr6, r10, 0x5a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 90, &mut ctx.xer);
	// 82EAB208: 41990010  bgt cr6, 0x82eab218
	if ctx.cr[6].gt {
	pc = 0x82EAB218; continue 'dispatch;
	}
	// 82EAB20C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82EAB210: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EAB214: 48000008  b 0x82eab21c
	pc = 0x82EAB21C; continue 'dispatch;
	// 82EAB218: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82EAB21C: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82EAB220: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAB224: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB228: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAB22C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAB230: 4198FFC0  blt cr6, 0x82eab1f0
	if ctx.cr[6].lt {
	pc = 0x82EAB1F0; continue 'dispatch;
	}
	// 82EAB234: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB23C: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EAB240: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAB244: 7D285A14  add r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82EAB248: 9949FFFF  stb r10, -1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 82EAB24C: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB250: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EAB254: 911E0008  stw r8, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EAB258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAB25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAB260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAB264: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAB268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAB26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB270 size=184
    let mut pc: u32 = 0x82EAB270;
    'dispatch: loop {
        match pc {
            0x82EAB270 => {
    //   block [0x82EAB270..0x82EAB328)
	// 82EAB270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB274: 482FCEED  bl 0x831a8160
	ctx.lr = 0x82EAB278;
	sub_831A8130(ctx, base);
	// 82EAB278: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB27C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB280: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAB284: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAB288: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EAB28C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAB290: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EAB294: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB298: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EAB29C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAB2A0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82EAB2A4: 4BFF548D  bl 0x82ea0730
	ctx.lr = 0x82EAB2A8;
	sub_82EA0730(ctx, base);
	// 82EAB2A8: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB2AC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB2B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAB2B4: 482FD25D  bl 0x831a8510
	ctx.lr = 0x82EAB2B8;
	sub_831A8510(ctx, base);
	// 82EAB2B8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB2BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAB2C0: 3509FFFF  addic. r8, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EAB2C4: 40810038  ble 0x82eab2fc
	if !ctx.cr[0].gt {
	pc = 0x82EAB2FC; continue 'dispatch;
	}
	// 82EAB2C8: 7FA90774  extsb r9, r29
	ctx.r[9].s64 = ctx.r[29].s8 as i64;
	// 82EAB2CC: 7D4BF0AE  lbzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EAB2D0: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 82EAB2D4: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EAB2D8: 409A0010  bne cr6, 0x82eab2e8
	if !ctx.cr[6].eq {
	pc = 0x82EAB2E8; continue 'dispatch;
	}
	// 82EAB2DC: 7F6BF1AE  stbx r27, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u8) };
	// 82EAB2E0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82EAB2E4: 419A0018  beq cr6, 0x82eab2fc
	if ctx.cr[6].eq {
	pc = 0x82EAB2FC; continue 'dispatch;
	}
	// 82EAB2E8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB2EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAB2F0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EAB2F4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAB2F8: 4198FFD4  blt cr6, 0x82eab2cc
	if ctx.cr[6].lt {
	pc = 0x82EAB2CC; continue 'dispatch;
	}
	// 82EAB2FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB304: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EAB308: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EAB30C: 7D3E5A14  add r9, r30, r11
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EAB310: 9949FFFF  stb r10, -1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 82EAB314: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB318: 911C0004  stw r8, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EAB31C: 911C0008  stw r8, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EAB320: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EAB324: 482FCE8C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB328 size=240
    let mut pc: u32 = 0x82EAB328;
    'dispatch: loop {
        match pc {
            0x82EAB328 => {
    //   block [0x82EAB328..0x82EAB418)
	// 82EAB328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB32C: 482FCE29  bl 0x831a8154
	ctx.lr = 0x82EAB330;
	sub_831A8130(ctx, base);
	// 82EAB330: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB334: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82EAB338: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82EAB33C: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82EAB340: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAB344: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 82EAB348: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EAB34C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EAB350: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EAB354: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EAB358: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82EAB35C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EAB360: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB364: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB368: 4BFFF539  bl 0x82eaa8a0
	ctx.lr = 0x82EAB36C;
	sub_82EAA8A0(ctx, base);
	// 82EAB36C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB370: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB374: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 82EAB378: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EAB37C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAB380: 7CC84850  subf r6, r8, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82EAB384: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB388: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB38C: 7D6639D6  mullw r11, r6, r7
	ctx.r[11].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82EAB390: 7C77C02E  lwzx r3, r23, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EAB394: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EAB398: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EAB39C: 4BFF5395  bl 0x82ea0730
	ctx.lr = 0x82EAB3A0;
	sub_82EA0730(ctx, base);
	// 82EAB3A0: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EAB3A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB3A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EAB3AC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EAB3B0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB3B4: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82EAB3B8: 419A0018  beq cr6, 0x82eab3d0
	if ctx.cr[6].eq {
	pc = 0x82EAB3D0; continue 'dispatch;
	}
	// 82EAB3BC: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82EAB3C0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EAB3C4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EAB3C8: 4BFFFB19  bl 0x82eaaee0
	ctx.lr = 0x82EAB3CC;
	sub_82EAAEE0(ctx, base);
	// 82EAB3CC: 48000008  b 0x82eab3d4
	pc = 0x82EAB3D4; continue 'dispatch;
	// 82EAB3D0: 482FD141  bl 0x831a8510
	ctx.lr = 0x82EAB3D4;
	sub_831A8510(ctx, base);
	// 82EAB3D4: 7D7CEA14  add r11, r28, r29
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 82EAB3D8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EAB3DC: 93BA0004  stw r29, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EAB3E0: 93BA0008  stw r29, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EAB3E4: 9B2BFFFF  stb r25, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[25].u8 ) };
	// 82EAB3E8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EAB3EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAB3F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAB3F4: 409A0018  bne cr6, 0x82eab40c
	if !ctx.cr[6].eq {
	pc = 0x82EAB40C; continue 'dispatch;
	}
	// 82EAB3F8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAB3FC: 7C77C02E  lwzx r3, r23, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EAB400: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EAB404: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAB408: 4BFF53A9  bl 0x82ea07b0
	ctx.lr = 0x82EAB40C;
	sub_82EA07B0(ctx, base);
	// 82EAB40C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EAB410: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82EAB414: 482FCD90  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB418 size=396
    let mut pc: u32 = 0x82EAB418;
    'dispatch: loop {
        match pc {
            0x82EAB418 => {
    //   block [0x82EAB418..0x82EAB5A4)
	// 82EAB418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB41C: 482FCD41  bl 0x831a815c
	ctx.lr = 0x82EAB420;
	sub_831A8130(ctx, base);
	// 82EAB420: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB424: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82EAB428: 3941006C  addi r10, r1, 0x6c
	ctx.r[10].s64 = ctx.r[1].s64 + 108;
	// 82EAB42C: 6169000C  ori r9, r11, 0xc
	ctx.r[9].u64 = ctx.r[11].u64 | 12;
	// 82EAB430: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EAB434: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82EAB438: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82EAB43C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAB440: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82EAB444: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EAB448: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82EAB44C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EAB450: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EAB454: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EAB458: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB45C: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB460: 4BFFF441  bl 0x82eaa8a0
	ctx.lr = 0x82EAB464;
	sub_82EAA8A0(ctx, base);
	// 82EAB464: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB468: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EAB46C: 419A00FC  beq cr6, 0x82eab568
	if ctx.cr[6].eq {
	pc = 0x82EAB568; continue 'dispatch;
	}
	// 82EAB470: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB474: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB478: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EAB47C: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EAB480: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB484: 7D4849D6  mullw r10, r8, r9
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82EAB488: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EAB48C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAB490: 40990024  ble cr6, 0x82eab4b4
	if !ctx.cr[6].gt {
	pc = 0x82EAB4B4; continue 'dispatch;
	}
	// 82EAB494: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB498: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAB49C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82EAB4A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EAB4A4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAB4A8: 4BFF5289  bl 0x82ea0730
	ctx.lr = 0x82EAB4AC;
	sub_82EA0730(ctx, base);
	// 82EAB4AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EAB4B0: 48000008  b 0x82eab4b8
	pc = 0x82EAB4B8; continue 'dispatch;
	// 82EAB4B4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB4B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB4BC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EAB4C0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EAB4C4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB4C8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EAB4CC: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82EAB4D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EAB4D4: 4BFFFA0D  bl 0x82eaaee0
	ctx.lr = 0x82EAB4D8;
	sub_82EAAEE0(ctx, base);
	// 82EAB4D8: 7D7DF214  add r11, r29, r30
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82EAB4DC: 9B4BFFFF  stb r26, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[26].u8 ) };
	// 82EAB4E0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB4E4: 7F1D2040  cmplw cr6, r29, r4
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EAB4E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB4EC: 419A0034  beq cr6, 0x82eab520
	if ctx.cr[6].eq {
	pc = 0x82EAB520; continue 'dispatch;
	}
	// 82EAB4F0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAB4F4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAB4F8: 409A001C  bne cr6, 0x82eab514
	if !ctx.cr[6].eq {
	pc = 0x82EAB514; continue 'dispatch;
	}
	// 82EAB4FC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB500: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAB504: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAB508: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAB50C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAB510: 4BFF52A1  bl 0x82ea07b0
	ctx.lr = 0x82EAB514;
	sub_82EA07B0(ctx, base);
	// 82EAB514: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EAB518: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EAB51C: 48000030  b 0x82eab54c
	pc = 0x82EAB54C; continue 'dispatch;
	// 82EAB520: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAB524: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAB528: 40980024  bge cr6, 0x82eab54c
	if !ctx.cr[6].lt {
	pc = 0x82EAB54C; continue 'dispatch;
	}
	// 82EAB52C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAB530: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAB534: 41980008  blt cr6, 0x82eab53c
	if ctx.cr[6].lt {
	pc = 0x82EAB53C; continue 'dispatch;
	}
	// 82EAB538: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EAB53C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAB540: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAB544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAB548: 4BFFB2B1  bl 0x82ea67f8
	ctx.lr = 0x82EAB54C;
	sub_82EA67F8(ctx, base);
	// 82EAB54C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EAB550: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EAB554: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EAB558: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAB55C: 99590000  stb r10, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EAB560: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EAB564: 48000014  b 0x82eab578
	pc = 0x82EAB578; continue 'dispatch;
	// 82EAB568: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EAB56C: 9B590000  stb r26, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 82EAB570: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAB574: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAB578: 409A0020  bne cr6, 0x82eab598
	if !ctx.cr[6].eq {
	pc = 0x82EAB598; continue 'dispatch;
	}
	// 82EAB57C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB580: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAB584: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAB588: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EAB58C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EAB590: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAB594: 4BFF521D  bl 0x82ea07b0
	ctx.lr = 0x82EAB598;
	sub_82EA07B0(ctx, base);
	// 82EAB598: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EAB59C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82EAB5A0: 482FCC0C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EAB5A8 size=28
    let mut pc: u32 = 0x82EAB5A8;
    'dispatch: loop {
        match pc {
            0x82EAB5A8 => {
    //   block [0x82EAB5A8..0x82EAB5C4)
	// 82EAB5A8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB5AC: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB5B0: 7D2A5810  subfc r9, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EAB5B4: 7D094910  subfe r8, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82EAB5B8: 550707FE  clrlwi r7, r8, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82EAB5BC: 98E30000  stb r7, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82EAB5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB5C8 size=108
    let mut pc: u32 = 0x82EAB5C8;
    'dispatch: loop {
        match pc {
            0x82EAB5C8 => {
    //   block [0x82EAB5C8..0x82EAB634)
	// 82EAB5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAB5D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAB5D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAB5D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAB5E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB5E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAB5E8: 419A0024  beq cr6, 0x82eab60c
	if ctx.cr[6].eq {
	pc = 0x82EAB60C; continue 'dispatch;
	}
	// 82EAB5EC: 3FC08332  lis r30, -0x7cce
	ctx.r[30].s64 = -2093875200;
	// 82EAB5F0: 817EBDC0  lwz r11, -0x4240(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EAB5F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EAB5F8: 4E800421  bctrl
	ctx.lr = 0x82EAB5FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB5FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB600: 815EBDC0  lwz r10, -0x4240(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EAB604: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAB608: 4E800421  bctrl
	ctx.lr = 0x82EAB60C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB60C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAB610: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EAB614: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EAB618: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EAB61C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAB620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAB624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAB628: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAB62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAB630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB638 size=372
    let mut pc: u32 = 0x82EAB638;
    'dispatch: loop {
        match pc {
            0x82EAB638 => {
    //   block [0x82EAB638..0x82EAB7AC)
	// 82EAB638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB63C: 482FCB1D  bl 0x831a8158
	ctx.lr = 0x82EAB640;
	sub_831A8130(ctx, base);
	// 82EAB640: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAB648: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB64C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EAB650: 40990154  ble cr6, 0x82eab7a4
	if !ctx.cr[6].gt {
	pc = 0x82EAB7A4; continue 'dispatch;
	}
	// 82EAB654: 3FC08332  lis r30, -0x7cce
	ctx.r[30].s64 = -2093875200;
	// 82EAB658: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAB65C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EAB660: 817EBDBC  lwz r11, -0x4244(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EAB664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EAB668: 4E800421  bctrl
	ctx.lr = 0x82EAB66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB66C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB670: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EAB674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB678: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EAB67C: 40990028  ble cr6, 0x82eab6a4
	if !ctx.cr[6].gt {
	pc = 0x82EAB6A4; continue 'dispatch;
	}
	// 82EAB680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAB684: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB688: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EAB68C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EAB690: 7D2BD12E  stwx r9, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[9].u32) };
	// 82EAB694: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EAB698: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB69C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EAB6A0: 4198FFE4  blt cr6, 0x82eab684
	if ctx.cr[6].lt {
	pc = 0x82EAB684; continue 'dispatch;
	}
	// 82EAB6A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB6A8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EAB6AC: 4099001C  ble cr6, 0x82eab6c8
	if !ctx.cr[6].gt {
	pc = 0x82EAB6C8; continue 'dispatch;
	}
	// 82EAB6B0: 3D4082EB  lis r10, -0x7d15
	ctx.r[10].s64 = -2098528256;
	// 82EAB6B4: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82EAB6B8: 38CAB5A8  addi r6, r10, -0x4a58
	ctx.r[6].s64 = ctx.r[10].s64 + -19032;
	// 82EAB6BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EAB6C0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EAB6C4: 480B99B5  bl 0x82f65078
	ctx.lr = 0x82EAB6C8;
	sub_82F65078(ctx, base);
	// 82EAB6C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB6CC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EAB6D0: 815EBDBC  lwz r10, -0x4244(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EAB6D4: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAB6D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAB6DC: 4E800421  bctrl
	ctx.lr = 0x82EAB6E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB6E0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB6E4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82EAB6E8: 811EBDBC  lwz r8, -0x4244(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EAB6EC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EAB6F0: 1C690054  mulli r3, r9, 0x54
	ctx.r[3].s64 = ctx.r[9].s64 * 84;
	// 82EAB6F4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EAB6F8: 4E800421  bctrl
	ctx.lr = 0x82EAB6FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB6FC: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB700: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82EAB704: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EAB708: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EAB70C: 4099005C  ble cr6, 0x82eab768
	if !ctx.cr[6].gt {
	pc = 0x82EAB768; continue 'dispatch;
	}
	// 82EAB710: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82EAB714: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82EAB718: 7F7AC850  subf r27, r26, r25
	ctx.r[27].s64 = ctx.r[25].s64 - ctx.r[26].s64;
	// 82EAB71C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB720: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 82EAB724: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EAB728: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB72C: 7D5BF12E  stwx r10, r27, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82EAB730: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB734: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB738: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB73C: 7CE84850  subf r7, r8, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82EAB740: 7CE61670  srawi r6, r7, 2
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[7].s32 >> 2) as i64;
	// 82EAB744: 1D660054  mulli r11, r6, 0x54
	ctx.r[11].s64 = ctx.r[6].s64 * 84;
	// 82EAB748: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EAB74C: 482FCDC5  bl 0x831a8510
	ctx.lr = 0x82EAB750;
	sub_831A8510(ctx, base);
	// 82EAB750: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB754: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82EAB758: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EAB75C: 3BBD0054  addi r29, r29, 0x54
	ctx.r[29].s64 = ctx.r[29].s64 + 84;
	// 82EAB760: 7F1C2800  cmpw cr6, r28, r5
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EAB764: 4198FFB8  blt cr6, 0x82eab71c
	if ctx.cr[6].lt {
	pc = 0x82EAB71C; continue 'dispatch;
	}
	// 82EAB768: 3FC08332  lis r30, -0x7cce
	ctx.r[30].s64 = -2093875200;
	// 82EAB76C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EAB770: 817EBDC0  lwz r11, -0x4240(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EAB774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EAB778: 4E800421  bctrl
	ctx.lr = 0x82EAB77C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB77C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB780: 815EBDC0  lwz r10, -0x4240(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EAB784: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAB788: 4E800421  bctrl
	ctx.lr = 0x82EAB78C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB78C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB790: 813EBDC0  lwz r9, -0x4240(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EAB794: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EAB798: 4E800421  bctrl
	ctx.lr = 0x82EAB79C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAB79C: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82EAB7A0: 931F0004  stw r24, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82EAB7A4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EAB7A8: 482FCA00  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB7B0 size=164
    let mut pc: u32 = 0x82EAB7B0;
    'dispatch: loop {
        match pc {
            0x82EAB7B0 => {
    //   block [0x82EAB7B0..0x82EAB854)
	// 82EAB7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAB7B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAB7BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAB7C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB7C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAB7C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EAB7CC: 4BFFFE6D  bl 0x82eab638
	ctx.lr = 0x82EAB7D0;
	sub_82EAB638(ctx, base);
	// 82EAB7D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAB7D4: 4BFFFE65  bl 0x82eab638
	ctx.lr = 0x82EAB7D8;
	sub_82EAB638(ctx, base);
	// 82EAB7D8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB7DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB7E0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EAB7E4: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB7E8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB7EC: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EAB7F0: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EAB7F4: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EAB7F8: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EAB7FC: 40980034  bge cr6, 0x82eab830
	if !ctx.cr[6].lt {
	pc = 0x82EAB830; continue 'dispatch;
	}
	// 82EAB800: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EAB804: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82EAB808: 40980028  bge cr6, 0x82eab830
	if !ctx.cr[6].lt {
	pc = 0x82EAB830; continue 'dispatch;
	}
	// 82EAB80C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB810: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB814: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EAB818: 409A0030  bne cr6, 0x82eab848
	if !ctx.cr[6].eq {
	pc = 0x82EAB848; continue 'dispatch;
	}
	// 82EAB81C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EAB820: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EAB824: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EAB828: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EAB82C: 4198FFD8  blt cr6, 0x82eab804
	if ctx.cr[6].lt {
	pc = 0x82EAB804; continue 'dispatch;
	}
	// 82EAB830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAB834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAB838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAB83C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAB840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAB844: 4E800020  blr
	return;
	// 82EAB848: 4098FFDC  bge cr6, 0x82eab824
	if !ctx.cr[6].lt {
	pc = 0x82EAB824; continue 'dispatch;
	}
	// 82EAB84C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EAB850: 4BFFFFD8  b 0x82eab828
	pc = 0x82EAB828; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAB858 size=32
    let mut pc: u32 = 0x82EAB858;
    'dispatch: loop {
        match pc {
            0x82EAB858 => {
    //   block [0x82EAB858..0x82EAB878)
	// 82EAB858: 54ABE8FE  srwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAB85C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EAB860: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EAB864: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82EAB868: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EAB86C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB870: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EAB874: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAB878 size=32
    let mut pc: u32 = 0x82EAB878;
    'dispatch: loop {
        match pc {
            0x82EAB878 => {
    //   block [0x82EAB878..0x82EAB898)
	// 82EAB878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB87C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EAB880: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB884: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB888: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82EAB88C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EAB890: 4082FFF0  bne 0x82eab880
	if !ctx.cr[0].eq {
	pc = 0x82EAB880; continue 'dispatch;
	}
	// 82EAB894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAB898 size=64
    let mut pc: u32 = 0x82EAB898;
    'dispatch: loop {
        match pc {
            0x82EAB898 => {
    //   block [0x82EAB898..0x82EAB8D8)
	// 82EAB898: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAB89C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAB8A0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAB8A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB8A8: 40990020  ble cr6, 0x82eab8c8
	if !ctx.cr[6].gt {
	pc = 0x82EAB8C8; continue 'dispatch;
	}
	// 82EAB8AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB8B0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EAB8B4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB8B8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB8BC: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82EAB8C0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EAB8C4: 4082FFF0  bne 0x82eab8b4
	if !ctx.cr[0].eq {
	pc = 0x82EAB8B4; continue 'dispatch;
	}
	// 82EAB8C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB8CC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAB8D0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EAB8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAB8D8 size=32
    let mut pc: u32 = 0x82EAB8D8;
    'dispatch: loop {
        match pc {
            0x82EAB8D8 => {
    //   block [0x82EAB8D8..0x82EAB8F8)
	// 82EAB8D8: 54ABE13E  srwi r11, r5, 4
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAB8DC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EAB8E0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EAB8E4: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82EAB8E8: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EAB8EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB8F0: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EAB8F4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAB8F8 size=32
    let mut pc: u32 = 0x82EAB8F8;
    'dispatch: loop {
        match pc {
            0x82EAB8F8 => {
    //   block [0x82EAB8F8..0x82EAB918)
	// 82EAB8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB8FC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EAB900: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB904: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB908: 7D2A412A  stdx r9, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 82EAB90C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EAB910: 4082FFF0  bne 0x82eab900
	if !ctx.cr[0].eq {
	pc = 0x82EAB900; continue 'dispatch;
	}
	// 82EAB914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAB918 size=32
    let mut pc: u32 = 0x82EAB918;
    'dispatch: loop {
        match pc {
            0x82EAB918 => {
    //   block [0x82EAB918..0x82EAB938)
	// 82EAB918: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EAB91C: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82EAB920: 40990010  ble cr6, 0x82eab930
	if !ctx.cr[6].gt {
	pc = 0x82EAB930; continue 'dispatch;
	}
	// 82EAB924: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAB928: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82EAB92C: 4198FFF8  blt cr6, 0x82eab924
	if ctx.cr[6].lt {
	pc = 0x82EAB924; continue 'dispatch;
	}
	// 82EAB930: 55632834  slwi r3, r11, 5
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAB934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB938 size=184
    let mut pc: u32 = 0x82EAB938;
    'dispatch: loop {
        match pc {
            0x82EAB938 => {
    //   block [0x82EAB938..0x82EAB9F0)
	// 82EAB938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAB940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAB944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAB948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAB94C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAB950: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAB954: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB958: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82EAB95C: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 82EAB960: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EAB964: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82EAB968: 40990010  ble cr6, 0x82eab978
	if !ctx.cr[6].gt {
	pc = 0x82EAB978; continue 'dispatch;
	}
	// 82EAB96C: 57FF083C  slwi r31, r31, 1
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EAB970: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAB974: 4198FFF8  blt cr6, 0x82eab96c
	if ctx.cr[6].lt {
	pc = 0x82EAB96C; continue 'dispatch;
	}
	// 82EAB978: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB97C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAB980: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EAB984: 57E41838  slwi r4, r31, 3
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EAB988: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAB98C: 4BFF4DA5  bl 0x82ea0730
	ctx.lr = 0x82EAB990;
	sub_82EA0730(ctx, base);
	// 82EAB990: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82EAB994: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EAB998: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EAB99C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EAB9A0: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAB9A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB9A8: 40990020  ble cr6, 0x82eab9c8
	if !ctx.cr[6].gt {
	pc = 0x82EAB9C8; continue 'dispatch;
	}
	// 82EAB9AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAB9B0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EAB9B4: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAB9B8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAB9BC: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82EAB9C0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EAB9C4: 4082FFF0  bne 0x82eab9b4
	if !ctx.cr[0].eq {
	pc = 0x82EAB9B4; continue 'dispatch;
	}
	// 82EAB9C8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAB9CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAB9D0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAB9D4: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EAB9D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAB9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAB9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAB9E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAB9E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAB9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAB9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAB9F0 size=128
    let mut pc: u32 = 0x82EAB9F0;
    'dispatch: loop {
        match pc {
            0x82EAB9F0 => {
    //   block [0x82EAB9F0..0x82EABA70)
	// 82EAB9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAB9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAB9F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAB9FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EABA00: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABA04: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EABA08: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EABA0C: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82EABA10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EABA14: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EABA18: 4BFF4D19  bl 0x82ea0730
	ctx.lr = 0x82EABA1C;
	sub_82EA0730(ctx, base);
	// 82EABA1C: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 82EABA20: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EABA24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EABA28: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EABA2C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82EABA30: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EABA34: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EABA38: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABA3C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EABA40: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82EABA44: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EABA48: 4082FFF0  bne 0x82eaba38
	if !ctx.cr[0].eq {
	pc = 0x82EABA38; continue 'dispatch;
	}
	// 82EABA4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EABA54: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EABA58: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EABA5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EABA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EABA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EABA68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EABA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABA70 size=16
    let mut pc: u32 = 0x82EABA70;
    'dispatch: loop {
        match pc {
            0x82EABA70 => {
    //   block [0x82EABA70..0x82EABA80)
	// 82EABA70: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABA74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EABA78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EABA7C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABA80 size=36
    let mut pc: u32 = 0x82EABA80;
    'dispatch: loop {
        match pc {
            0x82EABA80 => {
    //   block [0x82EABA80..0x82EABAA4)
	// 82EABA80: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABA84: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EABA88: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABA8C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EABA90: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EABA94: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABA98: 55051838  slwi r5, r8, 3
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EABA9C: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EABAA0: 4BFF4D10  b 0x82ea07b0
	sub_82EA07B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABAA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABAA4 size=4
    let mut pc: u32 = 0x82EABAA4;
    'dispatch: loop {
        match pc {
            0x82EABAA4 => {
    //   block [0x82EABAA4..0x82EABAA8)
	// 82EABAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EABAA8 size=212
    let mut pc: u32 = 0x82EABAA8;
    'dispatch: loop {
        match pc {
            0x82EABAA8 => {
    //   block [0x82EABAA8..0x82EABB7C)
	// 82EABAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EABAAC: 482FC6C1  bl 0x831a816c
	ctx.lr = 0x82EABAB0;
	sub_831A8130(ctx, base);
	// 82EABAB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EABAB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EABAB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EABABC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EABAC0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABAC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABAC8: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EABACC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EABAD0: 40990010  ble cr6, 0x82eabae0
	if !ctx.cr[6].gt {
	pc = 0x82EABAE0; continue 'dispatch;
	}
	// 82EABAD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EABAD8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EABADC: 48000325  bl 0x82eabe00
	ctx.lr = 0x82EABAE0;
	sub_82EABE00(ctx, base);
	// 82EABAE0: 3D609E37  lis r11, -0x61c9
	ctx.r[11].s64 = -1640562688;
	// 82EABAE4: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABAE8: 57CAE13E  srwi r10, r30, 4
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABAEC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABAF0: 616779B1  ori r7, r11, 0x79b1
	ctx.r[7].u64 = ctx.r[11].u64 | 31153;
	// 82EABAF4: 7CCA39D6  mullw r6, r10, r7
	ctx.r[6].s64 = (ctx.r[10].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82EABAF8: 7CCB4038  and r11, r6, r8
	ctx.r[11].u64 = ctx.r[6].u64 & ctx.r[8].u64;
	// 82EABAFC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABB00: 7CA9502E  lwzx r5, r9, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EABB04: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82EABB08: 419A002C  beq cr6, 0x82eabb34
	if ctx.cr[6].eq {
	pc = 0x82EABB34; continue 'dispatch;
	}
	// 82EABB0C: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EABB10: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EABB14: 419A0020  beq cr6, 0x82eabb34
	if ctx.cr[6].eq {
	pc = 0x82EABB34; continue 'dispatch;
	}
	// 82EABB18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EABB1C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABB20: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82EABB24: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABB28: 7CCA382E  lwzx r6, r10, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82EABB2C: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 82EABB30: 409AFFDC  bne cr6, 0x82eabb0c
	if !ctx.cr[6].eq {
	pc = 0x82EABB0C; continue 'dispatch;
	}
	// 82EABB34: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABB38: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABB3C: 7D0A482E  lwzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EABB40: 7CC8F050  subf r6, r8, r30
	ctx.r[6].s64 = ctx.r[30].s64 - ctx.r[8].s64;
	// 82EABB44: 7CC50034  cntlzw r5, r6
	ctx.r[5].u64 = if ctx.r[6].u32 == 0 { 32 } else { ctx.r[6].u32.leading_zeros() as u64 };
	// 82EABB48: 54A4DFFE  rlwinm r4, r5, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x0000001Fu64;
	// 82EABB4C: 68880001  xori r8, r4, 1
	ctx.r[8].u64 = ctx.r[4].u64 ^ 1;
	// 82EABB50: 7C683A14  add r3, r8, r7
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EABB54: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EABB58: 7FCA492E  stwx r30, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 82EABB5C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABB60: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABB64: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EABB68: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EABB6C: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EABB70: 7FA7492E  stwx r29, r7, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u32) };
	// 82EABB74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EABB78: 482FC644  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABB80 size=52
    let mut pc: u32 = 0x82EABB80;
    'dispatch: loop {
        match pc {
            0x82EABB80 => {
    //   block [0x82EABB80..0x82EABBB4)
	// 82EABB80: 3D009E37  lis r8, -0x61c9
	ctx.r[8].s64 = -1640562688;
	// 82EABB84: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABB88: 548BE13E  srwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EABB8C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABB90: 610779B1  ori r7, r8, 0x79b1
	ctx.r[7].u64 = ctx.r[8].u64 | 31153;
	// 82EABB94: 7CCB39D6  mullw r6, r11, r7
	ctx.r[6].s64 = (ctx.r[11].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82EABB98: 7CC35038  and r3, r6, r10
	ctx.r[3].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82EABB9C: 5465103A  slwi r5, r3, 2
	ctx.r[5].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EABBA0: 7D65482E  lwzx r11, r5, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EABBA4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82EABBA8: 419A0024  beq cr6, 0x82eabbcc
	if ctx.cr[6].eq {
		sub_82EABBB4(ctx, base);
		return;
	}
	// 82EABBAC: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EABBB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABBB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABBB4 size=32
    let mut pc: u32 = 0x82EABBB4;
    'dispatch: loop {
        match pc {
            0x82EABBB4 => {
    //   block [0x82EABBB4..0x82EABBD4)
	// 82EABBB4: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EABBB8: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82EABBBC: 5468103A  slwi r8, r3, 2
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EABBC0: 7D68482E  lwzx r11, r8, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EABBC4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82EABBC8: 409AFFE4  bne cr6, 0x82eabbac
	if !ctx.cr[6].eq {
		sub_82EABB80(ctx, base);
		return;
	}
	// 82EABBCC: 386A0001  addi r3, r10, 1
	ctx.r[3].s64 = ctx.r[10].s64 + 1;
	// 82EABBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABBD8 size=84
    let mut pc: u32 = 0x82EABBD8;
    'dispatch: loop {
        match pc {
            0x82EABBD8 => {
    //   block [0x82EABBD8..0x82EABC2C)
	// 82EABBD8: 3D409E37  lis r10, -0x61c9
	ctx.r[10].s64 = -1640562688;
	// 82EABBDC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABBE0: 548BE13E  srwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EABBE4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABBE8: 614779B1  ori r7, r10, 0x79b1
	ctx.r[7].u64 = ctx.r[10].u64 | 31153;
	// 82EABBEC: 7CCB39D6  mullw r6, r11, r7
	ctx.r[6].s64 = (ctx.r[11].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82EABBF0: 7CCB4838  and r11, r6, r9
	ctx.r[11].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 82EABBF4: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EABBF8: 7D43402E  lwzx r10, r3, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EABBFC: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82EABC00: 419A0024  beq cr6, 0x82eabc24
	if ctx.cr[6].eq {
	pc = 0x82EABC24; continue 'dispatch;
	}
	// 82EABC04: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EABC08: 419A0024  beq cr6, 0x82eabc2c
	if ctx.cr[6].eq {
		sub_82EABC2C(ctx, base);
		return;
	}
	// 82EABC0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EABC10: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82EABC14: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABC18: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EABC1C: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82EABC20: 409AFFE4  bne cr6, 0x82eabc04
	if !ctx.cr[6].eq {
	pc = 0x82EABC04; continue 'dispatch;
	}
	// 82EABC24: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82EABC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABC2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABC2C size=20
    let mut pc: u32 = 0x82EABC2C;
    'dispatch: loop {
        match pc {
            0x82EABC2C => {
    //   block [0x82EABC2C..0x82EABC40)
	// 82EABC2C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EABC30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EABC34: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABC38: 7C6A402E  lwzx r3, r10, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EABC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EABC40 size=124
    let mut pc: u32 = 0x82EABC40;
    'dispatch: loop {
        match pc {
            0x82EABC40 => {
    //   block [0x82EABC40..0x82EABCBC)
	// 82EABC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EABC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EABC48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EABC4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EABC50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EABC54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EABC58: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EABC5C: 4BFFFF25  bl 0x82eabb80
	ctx.lr = 0x82EABC60;
	sub_82EABB80(ctx, base);
	// 82EABC60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABC64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EABC68: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EABC6C: 40990008  ble cr6, 0x82eabc74
	if !ctx.cr[6].gt {
	pc = 0x82EABC74; continue 'dispatch;
	}
	// 82EABC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EABC74: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EABC78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EABC7C: 419A0024  beq cr6, 0x82eabca0
	if ctx.cr[6].eq {
	pc = 0x82EABCA0; continue 'dispatch;
	}
	// 82EABC80: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EABC84: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABC88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EABC8C: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EABC90: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EABC94: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EABC98: 90FE0000  stw r7, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EABC9C: 48000008  b 0x82eabca4
	pc = 0x82EABCA4; continue 'dispatch;
	// 82EABCA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EABCA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EABCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EABCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EABCB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EABCB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EABCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABCC0 size=320
    let mut pc: u32 = 0x82EABCC0;
    'dispatch: loop {
        match pc {
            0x82EABCC0 => {
    //   block [0x82EABCC0..0x82EABE00)
	// 82EABCC0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82EABCC4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EABCC8: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82EABCCC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABCD0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABCD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EABCD8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABCDC: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 82EABCE0: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EABCE4: 7CAA412E  stwx r5, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[5].u32) };
	// 82EABCE8: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABCEC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABCF0: 7CCA4A14  add r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EABCF4: 7CCB5038  and r11, r6, r10
	ctx.r[11].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82EABCF8: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EABCFC: 7D04382E  lwzx r8, r4, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82EABD00: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82EABD04: 419A0020  beq cr6, 0x82eabd24
	if ctx.cr[6].eq {
	pc = 0x82EABD24; continue 'dispatch;
	}
	// 82EABD08: 54E8003E  slwi r8, r7, 0
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EABD0C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EABD10: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82EABD14: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EABD18: 7C86402E  lwzx r4, r6, r8
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EABD1C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82EABD20: 409AFFEC  bne cr6, 0x82eabd0c
	if !ctx.cr[6].eq {
	pc = 0x82EABD0C; continue 'dispatch;
	}
	// 82EABD24: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82EABD28: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82EABD2C: 7D0B5038  and r11, r8, r10
	ctx.r[11].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 82EABD30: 7CDF5038  and r31, r6, r10
	ctx.r[31].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82EABD34: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EABD38: 7C87402E  lwzx r4, r7, r8
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EABD3C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82EABD40: 419A00B4  beq cr6, 0x82eabdf4
	if ctx.cr[6].eq {
	pc = 0x82EABDF4; continue 'dispatch;
	}
	// 82EABD44: 3CE09E37  lis r7, -0x61c9
	ctx.r[7].s64 = -1640562688;
	// 82EABD48: 60E479B1  ori r4, r7, 0x79b1
	ctx.r[4].u64 = ctx.r[7].u64 | 31153;
	// 82EABD4C: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABD50: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EABD54: 7CC7402E  lwzx r6, r7, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EABD58: 54DEE13E  srwi r30, r6, 4
	ctx.r[30].u32 = ctx.r[6].u32.wrapping_shr(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EABD5C: 7FDE21D6  mullw r30, r30, r4
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82EABD60: 7FCA5038  and r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 & ctx.r[10].u64;
	// 82EABD64: 4198000C  blt cr6, 0x82eabd70
	if ctx.cr[6].lt {
	pc = 0x82EABD70; continue 'dispatch;
	}
	// 82EABD68: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EABD6C: 41990068  bgt cr6, 0x82eabdd4
	if ctx.cr[6].gt {
	pc = 0x82EABDD4; continue 'dispatch;
	}
	// 82EABD70: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EABD74: 40980014  bge cr6, 0x82eabd88
	if !ctx.cr[6].lt {
	pc = 0x82EABD88; continue 'dispatch;
	}
	// 82EABD78: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EABD7C: 41990058  bgt cr6, 0x82eabdd4
	if ctx.cr[6].gt {
	pc = 0x82EABDD4; continue 'dispatch;
	}
	// 82EABD80: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EABD84: 40990050  ble cr6, 0x82eabdd4
	if !ctx.cr[6].gt {
	pc = 0x82EABDD4; continue 'dispatch;
	}
	// 82EABD88: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EABD8C: 4099000C  ble cr6, 0x82eabd98
	if !ctx.cr[6].gt {
	pc = 0x82EABD98; continue 'dispatch;
	}
	// 82EABD90: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EABD94: 41980040  blt cr6, 0x82eabdd4
	if ctx.cr[6].lt {
	pc = 0x82EABDD4; continue 'dispatch;
	}
	// 82EABD98: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABD9C: 7CCA392E  stwx r6, r10, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.r[6].u32) };
	// 82EABDA0: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABDA4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABDA8: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EABDAC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EABDB0: 39270001  addi r9, r7, 1
	ctx.r[9].s64 = ctx.r[7].s64 + 1;
	// 82EABDB4: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 82EABDB8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABDBC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EABDC0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82EABDC4: 7D4A302E  lwzx r10, r10, r6
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82EABDC8: 7D47312E  stwx r10, r7, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), ctx.r[10].u32) };
	// 82EABDCC: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABDD0: 7CA8392E  stwx r5, r8, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), ctx.r[5].u32) };
	// 82EABDD4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABDD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EABDDC: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABDE0: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82EABDE4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EABDE8: 7CC8382E  lwzx r6, r8, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82EABDEC: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 82EABDF0: 409AFF5C  bne cr6, 0x82eabd4c
	if !ctx.cr[6].eq {
	pc = 0x82EABD4C; continue 'dispatch;
	}
	// 82EABDF4: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EABDF8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EABDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EABE00 size=224
    let mut pc: u32 = 0x82EABE00;
    'dispatch: loop {
        match pc {
            0x82EABE00 => {
    //   block [0x82EABE00..0x82EABEE0)
	// 82EABE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EABE04: 482FC351  bl 0x831a8154
	ctx.lr = 0x82EABE08;
	sub_831A8130(ctx, base);
	// 82EABE08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EABE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EABE10: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABE14: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 82EABE18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EABE1C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EABE20: 57C41838  slwi r4, r30, 3
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EABE24: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABE28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABE2C: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABE30: 55570000  rlwinm r23, r10, 0, 0, 0
	ctx.r[23].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82EABE34: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82EABE38: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 82EABE3C: 4BFF48F5  bl 0x82ea0730
	ctx.lr = 0x82EABE40;
	sub_82EA0730(ctx, base);
	// 82EABE40: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EABE44: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EABE48: 40990024  ble cr6, 0x82eabe6c
	if !ctx.cr[6].gt {
	pc = 0x82EABE6C; continue 'dispatch;
	}
	// 82EABE4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EABE50: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EABE54: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EABE58: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABE5C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EABE60: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82EABE64: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EABE68: 4082FFF0  bne 0x82eabe58
	if !ctx.cr[0].eq {
	pc = 0x82EABE58; continue 'dispatch;
	}
	// 82EABE6C: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82EABE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EABE74: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EABE78: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EABE7C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EABE80: 4099003C  ble cr6, 0x82eabebc
	if !ctx.cr[6].gt {
	pc = 0x82EABEBC; continue 'dispatch;
	}
	// 82EABE84: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EABE88: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82EABE8C: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82EABE90: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82EABE94: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABE98: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82EABE9C: 419A0010  beq cr6, 0x82eabeac
	if ctx.cr[6].eq {
	pc = 0x82EABEAC; continue 'dispatch;
	}
	// 82EABEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EABEA4: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABEA8: 4BFFFC01  bl 0x82eabaa8
	ctx.lr = 0x82EABEAC;
	sub_82EABAA8(ctx, base);
	// 82EABEAC: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EABEB0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EABEB4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EABEB8: 4082FFDC  bne 0x82eabe94
	if !ctx.cr[0].eq {
	pc = 0x82EABE94; continue 'dispatch;
	}
	// 82EABEBC: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82EABEC0: 409A0018  bne cr6, 0x82eabed8
	if !ctx.cr[6].eq {
	pc = 0x82EABED8; continue 'dispatch;
	}
	// 82EABEC4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EABEC8: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82EABECC: 57651838  slwi r5, r27, 3
	ctx.r[5].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EABED0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EABED4: 4BFF48DD  bl 0x82ea07b0
	ctx.lr = 0x82EABED8;
	sub_82EA07B0(ctx, base);
	// 82EABED8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EABEDC: 482FC2C8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABEE0 size=16
    let mut pc: u32 = 0x82EABEE0;
    'dispatch: loop {
        match pc {
            0x82EABEE0 => {
    //   block [0x82EABEE0..0x82EABEF0)
	// 82EABEE0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABEE4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EABEE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EABEEC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABEF0 size=36
    let mut pc: u32 = 0x82EABEF0;
    'dispatch: loop {
        match pc {
            0x82EABEF0 => {
    //   block [0x82EABEF0..0x82EABF14)
	// 82EABEF0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABEF4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EABEF8: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABEFC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EABF00: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EABF04: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABF08: 55052036  slwi r5, r8, 4
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EABF0C: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EABF10: 4BFF48A0  b 0x82ea07b0
	sub_82EA07B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABF14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABF14 size=4
    let mut pc: u32 = 0x82EABF14;
    'dispatch: loop {
        match pc {
            0x82EABF14 => {
    //   block [0x82EABF14..0x82EABF18)
	// 82EABF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EABF18 size=220
    let mut pc: u32 = 0x82EABF18;
    'dispatch: loop {
        match pc {
            0x82EABF18 => {
    //   block [0x82EABF18..0x82EABFF4)
	// 82EABF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EABF1C: 482FC251  bl 0x831a816c
	ctx.lr = 0x82EABF20;
	sub_831A8130(ctx, base);
	// 82EABF20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EABF24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EABF28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EABF2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EABF30: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABF34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABF38: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EABF3C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EABF40: 40990010  ble cr6, 0x82eabf50
	if !ctx.cr[6].gt {
	pc = 0x82EABF50; continue 'dispatch;
	}
	// 82EABF44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EABF48: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EABF4C: 4800025D  bl 0x82eac1a8
	ctx.lr = 0x82EABF50;
	sub_82EAC1A8(ctx, base);
	// 82EABF50: 396079B1  li r11, 0x79b1
	ctx.r[11].s64 = 31153;
	// 82EABF54: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABF58: 7BC7E102  rldicl r7, r30, 0x3c, 4
	ctx.r[7].u64 = ctx.r[30].u64 & 0x000000000000000Fu64;
	// 82EABF5C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABF60: 65669E37  oris r6, r11, 0x9e37
	ctx.r[6].u64 = ctx.r[11].u64 | 2654404608;
	// 82EABF64: 7D4807B4  extsw r8, r10
	ctx.r[8].s64 = ctx.r[10].s32 as i64;
	// 82EABF68: 7CA731D2  mulld r5, r7, r6
	ctx.r[5].s64 = ctx.r[7].s64 * ctx.r[6].s64;
	// 82EABF6C: 7CAB4038  and r11, r5, r8
	ctx.r[11].u64 = ctx.r[5].u64 & ctx.r[8].u64;
	// 82EABF70: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABF74: 7C89502A  ldx r4, r9, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	// 82EABF78: 2F24FFFF  cmpdi cr6, r4, -1
	ctx.cr[6].compare_i64(ctx.r[4].s64, -1, &mut ctx.xer);
	// 82EABF7C: 419A002C  beq cr6, 0x82eabfa8
	if ctx.cr[6].eq {
	pc = 0x82EABFA8; continue 'dispatch;
	}
	// 82EABF80: 7D49502A  ldx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	// 82EABF84: 7F2AF040  cmpld cr6, r10, r30
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[30].u64, &mut ctx.xer);
	// 82EABF88: 419A0020  beq cr6, 0x82eabfa8
	if ctx.cr[6].eq {
	pc = 0x82EABFA8; continue 'dispatch;
	}
	// 82EABF8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EABF90: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABF94: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82EABF98: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABF9C: 7CCA382A  ldx r6, r10, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) };
	// 82EABFA0: 2F26FFFF  cmpdi cr6, r6, -1
	ctx.cr[6].compare_i64(ctx.r[6].s64, -1, &mut ctx.xer);
	// 82EABFA4: 409AFFDC  bne cr6, 0x82eabf80
	if !ctx.cr[6].eq {
	pc = 0x82EABF80; continue 'dispatch;
	}
	// 82EABFA8: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EABFAC: 5568003E  slwi r8, r11, 0
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EABFB0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EABFB4: 7D6A482A  ldx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82EABFB8: 7F2BF040  cmpld cr6, r11, r30
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[30].u64, &mut ctx.xer);
	// 82EABFBC: 409A0008  bne cr6, 0x82eabfc4
	if !ctx.cr[6].eq {
	pc = 0x82EABFC4; continue 'dispatch;
	}
	// 82EABFC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EABFC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EABFC8: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EABFCC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EABFD0: 7FCA492A  stdx r30, r10, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u64) };
	// 82EABFD4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EABFD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EABFDC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EABFE0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EABFE4: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EABFE8: 7FA8512A  stdx r29, r8, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u64) };
	// 82EABFEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EABFF0: 482FC1CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EABFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EABFF8 size=88
    let mut pc: u32 = 0x82EABFF8;
    'dispatch: loop {
        match pc {
            0x82EABFF8 => {
    //   block [0x82EABFF8..0x82EAC050)
	// 82EABFF8: 396079B1  li r11, 0x79b1
	ctx.r[11].s64 = 31153;
	// 82EABFFC: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC000: 788AE102  rldicl r10, r4, 0x3c, 4
	ctx.r[10].u64 = ctx.r[4].u64 & 0x000000000000000Fu64;
	// 82EAC004: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC008: 65669E37  oris r6, r11, 0x9e37
	ctx.r[6].u64 = ctx.r[11].u64 | 2654404608;
	// 82EAC00C: 7CE807B4  extsw r8, r7
	ctx.r[8].s64 = ctx.r[7].s32 as i64;
	// 82EAC010: 7CAA31D2  mulld r5, r10, r6
	ctx.r[5].s64 = ctx.r[10].s64 * ctx.r[6].s64;
	// 82EAC014: 7CAB4038  and r11, r5, r8
	ctx.r[11].u64 = ctx.r[5].u64 & ctx.r[8].u64;
	// 82EAC018: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAC01C: 7D43482A  ldx r10, r3, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) };
	// 82EAC020: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 82EAC024: 419A0024  beq cr6, 0x82eac048
	if ctx.cr[6].eq {
	pc = 0x82EAC048; continue 'dispatch;
	}
	// 82EAC028: 7F2A2040  cmpld cr6, r10, r4
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[4].u64, &mut ctx.xer);
	// 82EAC02C: 419A0024  beq cr6, 0x82eac050
	if ctx.cr[6].eq {
		sub_82EAC050(ctx, base);
		return;
	}
	// 82EAC030: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAC034: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82EAC038: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EAC03C: 7D4A482A  ldx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82EAC040: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 82EAC044: 409AFFE4  bne cr6, 0x82eac028
	if !ctx.cr[6].eq {
	pc = 0x82EAC028; continue 'dispatch;
	}
	// 82EAC048: 38670001  addi r3, r7, 1
	ctx.r[3].s64 = ctx.r[7].s64 + 1;
	// 82EAC04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAC050 size=8
    let mut pc: u32 = 0x82EAC050;
    'dispatch: loop {
        match pc {
            0x82EAC050 => {
    //   block [0x82EAC050..0x82EAC058)
	// 82EAC050: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAC054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAC058 size=332
    let mut pc: u32 = 0x82EAC058;
    'dispatch: loop {
        match pc {
            0x82EAC058 => {
    //   block [0x82EAC058..0x82EAC1A4)
	// 82EAC058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAC05C: 482FC111  bl 0x831a816c
	ctx.lr = 0x82EAC060;
	sub_831A8130(ctx, base);
	// 82EAC060: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAC064: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EAC068: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC06C: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82EAC070: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 82EAC074: 7C8907B4  extsw r9, r4
	ctx.r[9].s64 = ctx.r[4].s32 as i64;
	// 82EAC078: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EAC07C: 7FAA412A  stdx r29, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[29].u64) };
	// 82EAC080: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC084: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC088: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 82EAC08C: 7CAA4A14  add r5, r10, r9
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EAC090: 7CA85038  and r8, r5, r10
	ctx.r[8].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	// 82EAC094: 55041838  slwi r4, r8, 3
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EAC098: 7D64382A  ldx r11, r4, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) };
	// 82EAC09C: 2F2BFFFF  cmpdi cr6, r11, -1
	ctx.cr[6].compare_i64(ctx.r[11].s64, -1, &mut ctx.xer);
	// 82EAC0A0: 419A0020  beq cr6, 0x82eac0c0
	if ctx.cr[6].eq {
	pc = 0x82EAC0C0; continue 'dispatch;
	}
	// 82EAC0A4: 54EB003E  slwi r11, r7, 0
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC0A8: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82EAC0AC: 7D085038  and r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 82EAC0B0: 55061838  slwi r6, r8, 3
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EAC0B4: 7CA6582A  ldx r5, r6, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) };
	// 82EAC0B8: 2F25FFFF  cmpdi cr6, r5, -1
	ctx.cr[6].compare_i64(ctx.r[5].s64, -1, &mut ctx.xer);
	// 82EAC0BC: 409AFFEC  bne cr6, 0x82eac0a8
	if !ctx.cr[6].eq {
	pc = 0x82EAC0A8; continue 'dispatch;
	}
	// 82EAC0C0: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82EAC0C4: 38C80001  addi r6, r8, 1
	ctx.r[6].s64 = ctx.r[8].s64 + 1;
	// 82EAC0C8: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82EAC0CC: 7CDE5038  and r30, r6, r10
	ctx.r[30].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82EAC0D0: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EAC0D4: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EAC0D8: 7CA7402A  ldx r5, r7, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) };
	// 82EAC0DC: 2F25FFFF  cmpdi cr6, r5, -1
	ctx.cr[6].compare_i64(ctx.r[5].s64, -1, &mut ctx.xer);
	// 82EAC0E0: 419A00C0  beq cr6, 0x82eac1a0
	if ctx.cr[6].eq {
	pc = 0x82EAC1A0; continue 'dispatch;
	}
	// 82EAC0E4: 38E079B1  li r7, 0x79b1
	ctx.r[7].s64 = 31153;
	// 82EAC0E8: 64FF9E37  oris r31, r7, 0x9e37
	ctx.r[31].u64 = ctx.r[7].u64 | 2654404608;
	// 82EAC0EC: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC0F0: 7F2BF040  cmpld cr6, r11, r30
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[30].u64, &mut ctx.xer);
	// 82EAC0F4: 7CA6402A  ldx r5, r6, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) };
	// 82EAC0F8: 78A7E102  rldicl r7, r5, 0x3c, 4
	ctx.r[7].u64 = ctx.r[5].u64 & 0x000000000000000Fu64;
	// 82EAC0FC: 7CE7F9D2  mulld r7, r7, r31
	ctx.r[7].s64 = ctx.r[7].s64 * ctx.r[31].s64;
	// 82EAC100: 7CEA5038  and r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 82EAC104: 4198000C  blt cr6, 0x82eac110
	if ctx.cr[6].lt {
	pc = 0x82EAC110; continue 'dispatch;
	}
	// 82EAC108: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82EAC10C: 4199006C  bgt cr6, 0x82eac178
	if ctx.cr[6].gt {
	pc = 0x82EAC178; continue 'dispatch;
	}
	// 82EAC110: 7F2B4840  cmpld cr6, r11, r9
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82EAC114: 40980014  bge cr6, 0x82eac128
	if !ctx.cr[6].lt {
	pc = 0x82EAC128; continue 'dispatch;
	}
	// 82EAC118: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82EAC11C: 4199005C  bgt cr6, 0x82eac178
	if ctx.cr[6].gt {
	pc = 0x82EAC178; continue 'dispatch;
	}
	// 82EAC120: 7F2A5840  cmpld cr6, r10, r11
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[11].u64, &mut ctx.xer);
	// 82EAC124: 40990054  ble cr6, 0x82eac178
	if !ctx.cr[6].gt {
	pc = 0x82EAC178; continue 'dispatch;
	}
	// 82EAC128: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82EAC12C: 4099000C  ble cr6, 0x82eac138
	if !ctx.cr[6].gt {
	pc = 0x82EAC138; continue 'dispatch;
	}
	// 82EAC130: 7F2AF040  cmpld cr6, r10, r30
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[30].u64, &mut ctx.xer);
	// 82EAC134: 41980044  blt cr6, 0x82eac178
	if ctx.cr[6].lt {
	pc = 0x82EAC178; continue 'dispatch;
	}
	// 82EAC138: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EAC13C: 5527003E  slwi r7, r9, 0
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EAC140: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82EAC144: 7CAA312A  stdx r5, r10, r6
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32), ctx.r[5].u64) };
	// 82EAC148: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC14C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC150: 7CCA2214  add r6, r10, r4
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82EAC154: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EAC158: 38860001  addi r4, r6, 1
	ctx.r[4].s64 = ctx.r[6].s64 + 1;
	// 82EAC15C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EAC160: 54871838  slwi r7, r4, 3
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EAC164: 55461838  slwi r6, r10, 3
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EAC168: 7C87282A  ldx r4, r7, r5
	ctx.r[4].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[5].u32)) };
	// 82EAC16C: 7C86292A  stdx r4, r6, r5
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[5].u32), ctx.r[4].u64) };
	// 82EAC170: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC174: 7FA8512A  stdx r29, r8, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u64) };
	// 82EAC178: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC17C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EAC180: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC184: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82EAC188: 7D0B5038  and r11, r8, r10
	ctx.r[11].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 82EAC18C: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EAC190: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EAC194: 7CC8382A  ldx r6, r8, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	// 82EAC198: 2F26FFFF  cmpdi cr6, r6, -1
	ctx.cr[6].compare_i64(ctx.r[6].s64, -1, &mut ctx.xer);
	// 82EAC19C: 409AFF50  bne cr6, 0x82eac0ec
	if !ctx.cr[6].eq {
	pc = 0x82EAC0EC; continue 'dispatch;
	}
	// 82EAC1A0: 482FC01C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAC1A8 size=224
    let mut pc: u32 = 0x82EAC1A8;
    'dispatch: loop {
        match pc {
            0x82EAC1A8 => {
    //   block [0x82EAC1A8..0x82EAC288)
	// 82EAC1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAC1AC: 482FBFA9  bl 0x831a8154
	ctx.lr = 0x82EAC1B0;
	sub_831A8130(ctx, base);
	// 82EAC1B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAC1B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAC1B8: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC1BC: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 82EAC1C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EAC1C4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EAC1C8: 57C42036  slwi r4, r30, 4
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EAC1CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAC1D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC1D4: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC1D8: 55570000  rlwinm r23, r10, 0, 0, 0
	ctx.r[23].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAC1DC: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82EAC1E0: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 82EAC1E4: 4BFF454D  bl 0x82ea0730
	ctx.lr = 0x82EAC1E8;
	sub_82EA0730(ctx, base);
	// 82EAC1E8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EAC1EC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EAC1F0: 40990024  ble cr6, 0x82eac214
	if !ctx.cr[6].gt {
	pc = 0x82EAC214; continue 'dispatch;
	}
	// 82EAC1F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAC1F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EAC1FC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EAC200: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC204: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAC208: 7D2A412A  stdx r9, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 82EAC20C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EAC210: 4082FFF0  bne 0x82eac200
	if !ctx.cr[0].eq {
	pc = 0x82EAC200; continue 'dispatch;
	}
	// 82EAC214: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82EAC218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAC21C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EAC220: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EAC224: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EAC228: 4099003C  ble cr6, 0x82eac264
	if !ctx.cr[6].gt {
	pc = 0x82EAC264; continue 'dispatch;
	}
	// 82EAC22C: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC230: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82EAC234: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82EAC238: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82EAC23C: E89D0000  ld r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 82EAC240: 2F24FFFF  cmpdi cr6, r4, -1
	ctx.cr[6].compare_i64(ctx.r[4].s64, -1, &mut ctx.xer);
	// 82EAC244: 419A0010  beq cr6, 0x82eac254
	if ctx.cr[6].eq {
	pc = 0x82EAC254; continue 'dispatch;
	}
	// 82EAC248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAC24C: E8BE0000  ld r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82EAC250: 4BFFFCC9  bl 0x82eabf18
	ctx.lr = 0x82EAC254;
	sub_82EABF18(ctx, base);
	// 82EAC254: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EAC258: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82EAC25C: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82EAC260: 4082FFDC  bne 0x82eac23c
	if !ctx.cr[0].eq {
	pc = 0x82EAC23C; continue 'dispatch;
	}
	// 82EAC264: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82EAC268: 409A0018  bne cr6, 0x82eac280
	if !ctx.cr[6].eq {
	pc = 0x82EAC280; continue 'dispatch;
	}
	// 82EAC26C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAC270: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82EAC274: 57652036  slwi r5, r27, 4
	ctx.r[5].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EAC278: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EAC27C: 4BFF4535  bl 0x82ea07b0
	ctx.lr = 0x82EAC280;
	sub_82EA07B0(ctx, base);
	// 82EAC280: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EAC284: 482FBF20  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAC288 size=116
    let mut pc: u32 = 0x82EAC288;
    'dispatch: loop {
        match pc {
            0x82EAC288 => {
    //   block [0x82EAC288..0x82EAC2FC)
	// 82EAC288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAC28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAC290: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAC294: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAC298: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAC29C: 4BFFF8E5  bl 0x82eabb80
	ctx.lr = 0x82EAC2A0;
	sub_82EABB80(ctx, base);
	// 82EAC2A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC2A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EAC2A8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC2AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAC2B0: 40990008  ble cr6, 0x82eac2b8
	if !ctx.cr[6].gt {
	pc = 0x82EAC2B8; continue 'dispatch;
	}
	// 82EAC2B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAC2B8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAC2BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAC2C0: 419A0024  beq cr6, 0x82eac2e4
	if ctx.cr[6].eq {
	pc = 0x82EAC2E4; continue 'dispatch;
	}
	// 82EAC2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAC2C8: 4BFFF9F9  bl 0x82eabcc0
	ctx.lr = 0x82EAC2CC;
	sub_82EABCC0(ctx, base);
	// 82EAC2CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAC2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAC2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAC2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAC2DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAC2E0: 4E800020  blr
	return;
	// 82EAC2E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EAC2E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAC2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAC2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAC2F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAC2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAC300 size=40
    let mut pc: u32 = 0x82EAC300;
    'dispatch: loop {
        match pc {
            0x82EAC300 => {
    //   block [0x82EAC300..0x82EAC328)
	// 82EAC300: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EAC304: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EAC308: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82EAC30C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82EAC310: 40990010  ble cr6, 0x82eac320
	if !ctx.cr[6].gt {
	pc = 0x82EAC320; continue 'dispatch;
	}
	// 82EAC314: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC318: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAC31C: 4198FFF8  blt cr6, 0x82eac314
	if ctx.cr[6].lt {
	pc = 0x82EAC314; continue 'dispatch;
	}
	// 82EAC320: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC324: 4BFFFADC  b 0x82eabe00
	sub_82EABE00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAC328 size=412
    let mut pc: u32 = 0x82EAC328;
    'dispatch: loop {
        match pc {
            0x82EAC328 => {
    //   block [0x82EAC328..0x82EAC4C4)
	// 82EAC328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAC32C: 482FBE39  bl 0x831a8164
	ctx.lr = 0x82EAC330;
	sub_831A8130(ctx, base);
	// 82EAC330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAC334: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAC338: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EAC33C: 3B6B0860  addi r27, r11, 0x860
	ctx.r[27].s64 = ctx.r[11].s64 + 2144;
	// 82EAC340: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAC344: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EAC348: 4BFFE2F9  bl 0x82eaa640
	ctx.lr = 0x82EAC34C;
	sub_82EAA640(ctx, base);
	// 82EAC34C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EAC350: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EAC354: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EAC358: 4BFFE0C9  bl 0x82eaa420
	ctx.lr = 0x82EAC35C;
	sub_82EAA420(ctx, base);
	// 82EAC35C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EAC360: 419A0020  beq cr6, 0x82eac380
	if ctx.cr[6].eq {
	pc = 0x82EAC380; continue 'dispatch;
	}
	// 82EAC364: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82EAC368: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EAC36C: 388B2298  addi r4, r11, 0x2298
	ctx.r[4].s64 = ctx.r[11].s64 + 8856;
	// 82EAC370: 4BFFE331  bl 0x82eaa6a0
	ctx.lr = 0x82EAC374;
	sub_82EAA6A0(ctx, base);
	// 82EAC374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAC378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAC37C: 419A0008  beq cr6, 0x82eac384
	if ctx.cr[6].eq {
	pc = 0x82EAC384; continue 'dispatch;
	}
	// 82EAC380: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAC384: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAC388: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAC38C: 409A0094  bne cr6, 0x82eac420
	if !ctx.cr[6].eq {
	pc = 0x82EAC420; continue 'dispatch;
	}
	// 82EAC390: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EAC394: 7D7EE8AE  lbzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EAC398: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EAC39C: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 82EAC3A0: 419A000C  beq cr6, 0x82eac3ac
	if ctx.cr[6].eq {
	pc = 0x82EAC3AC; continue 'dispatch;
	}
	// 82EAC3A4: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 82EAC3A8: 409A000C  bne cr6, 0x82eac3b4
	if !ctx.cr[6].eq {
	pc = 0x82EAC3B4; continue 'dispatch;
	}
	// 82EAC3AC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EAC3B0: 4BFFFFE4  b 0x82eac394
	pc = 0x82EAC394; continue 'dispatch;
	// 82EAC3B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EAC3B8: 4BFFE289  bl 0x82eaa640
	ctx.lr = 0x82EAC3BC;
	sub_82EAA640(ctx, base);
	// 82EAC3BC: 7FBEEA14  add r29, r30, r29
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82EAC3C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EAC3C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EAC3C8: 4BFFE279  bl 0x82eaa640
	ctx.lr = 0x82EAC3CC;
	sub_82EAA640(ctx, base);
	// 82EAC3CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC3D0: 7D43E214  add r10, r3, r28
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 82EAC3D4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC3D8: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 82EAC3DC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAC3E0: 40980024  bge cr6, 0x82eac404
	if !ctx.cr[6].lt {
	pc = 0x82EAC404; continue 'dispatch;
	}
	// 82EAC3E4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC3E8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC3EC: 41980008  blt cr6, 0x82eac3f4
	if ctx.cr[6].lt {
	pc = 0x82EAC3F4; continue 'dispatch;
	}
	// 82EAC3F0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EAC3F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC3F8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC3FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAC400: 4BFFA3F9  bl 0x82ea67f8
	ctx.lr = 0x82EAC404;
	sub_82EA67F8(ctx, base);
	// 82EAC404: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EAC408: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EAC40C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC410: 4BFFE201  bl 0x82eaa610
	ctx.lr = 0x82EAC414;
	sub_82EAA610(ctx, base);
	// 82EAC414: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC418: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EAC41C: 48000048  b 0x82eac464
	pc = 0x82EAC464; continue 'dispatch;
	// 82EAC420: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EAC424: 4BFFE21D  bl 0x82eaa640
	ctx.lr = 0x82EAC428;
	sub_82EAA640(ctx, base);
	// 82EAC428: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC42C: 3BC30001  addi r30, r3, 1
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	// 82EAC430: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC434: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAC438: 40980024  bge cr6, 0x82eac45c
	if !ctx.cr[6].lt {
	pc = 0x82EAC45C; continue 'dispatch;
	}
	// 82EAC43C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC440: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC444: 41980008  blt cr6, 0x82eac44c
	if ctx.cr[6].lt {
	pc = 0x82EAC44C; continue 'dispatch;
	}
	// 82EAC448: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EAC44C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC450: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAC458: 4BFFA3A1  bl 0x82ea67f8
	ctx.lr = 0x82EAC45C;
	sub_82EA67F8(ctx, base);
	// 82EAC45C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC460: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EAC464: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EAC468: 4BFFE1A9  bl 0x82eaa610
	ctx.lr = 0x82EAC46C;
	sub_82EAA610(ctx, base);
	// 82EAC46C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC470: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAC474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EAC478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAC47C: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EAC480: 9907FFFF  stb r8, -1(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(-1 as u32), ctx.r[8].u8 ) };
	// 82EAC484: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAC488: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82EAC48C: 4099002C  ble cr6, 0x82eac4b8
	if !ctx.cr[6].gt {
	pc = 0x82EAC4B8; continue 'dispatch;
	}
	// 82EAC490: 3920005C  li r9, 0x5c
	ctx.r[9].s64 = 92;
	// 82EAC494: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC498: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAC49C: 2B08002F  cmplwi cr6, r8, 0x2f
	ctx.cr[6].compare_u32(ctx.r[8].u32, 47 as u32, &mut ctx.xer);
	// 82EAC4A0: 409A0008  bne cr6, 0x82eac4a8
	if !ctx.cr[6].eq {
	pc = 0x82EAC4A8; continue 'dispatch;
	}
	// 82EAC4A4: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82EAC4A8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAC4AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EAC4B0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAC4B4: 4198FFE0  blt cr6, 0x82eac494
	if ctx.cr[6].lt {
	pc = 0x82EAC494; continue 'dispatch;
	}
	// 82EAC4B8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAC4C0: 482FBCF4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAC4C8 size=1164
    let mut pc: u32 = 0x82EAC4C8;
    'dispatch: loop {
        match pc {
            0x82EAC4C8 => {
    //   block [0x82EAC4C8..0x82EAC954)
	// 82EAC4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAC4CC: 482FBC8D  bl 0x831a8158
	ctx.lr = 0x82EAC4D0;
	sub_831A8130(ctx, base);
	// 82EAC4D0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAC4D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EAC4D8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EAC4DC: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82EAC4E0: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82EAC4E4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82EAC4E8: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82EAC4EC: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 82EAC4F0: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82EAC4F4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EAC4F8: 419A0054  beq cr6, 0x82eac54c
	if ctx.cr[6].eq {
	pc = 0x82EAC54C; continue 'dispatch;
	}
	// 82EAC4FC: 4BFFE145  bl 0x82eaa640
	ctx.lr = 0x82EAC500;
	sub_82EAA640(ctx, base);
	// 82EAC500: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EAC504: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82EAC508: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC50C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAC510: 40980024  bge cr6, 0x82eac534
	if !ctx.cr[6].lt {
	pc = 0x82EAC534; continue 'dispatch;
	}
	// 82EAC514: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC518: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC51C: 41980008  blt cr6, 0x82eac524
	if ctx.cr[6].lt {
	pc = 0x82EAC524; continue 'dispatch;
	}
	// 82EAC520: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAC524: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC528: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC52C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EAC530: 4BFFA2C9  bl 0x82ea67f8
	ctx.lr = 0x82EAC534;
	sub_82EA67F8(ctx, base);
	// 82EAC534: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82EAC538: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAC53C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EAC540: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAC544: 4BFFE205  bl 0x82eaa748
	ctx.lr = 0x82EAC548;
	sub_82EAA748(ctx, base);
	// 82EAC548: 48000020  b 0x82eac568
	pc = 0x82EAC568; continue 'dispatch;
	// 82EAC54C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC550: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EAC554: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EAC558: 4BFFA2A1  bl 0x82ea67f8
	ctx.lr = 0x82EAC55C;
	sub_82EA67F8(ctx, base);
	// 82EAC55C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAC560: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 82EAC564: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82EAC568: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82EAC56C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82EAC570: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 82EAC574: 93610080  stw r27, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u32 ) };
	// 82EAC578: 3B8B043C  addi r28, r11, 0x43c
	ctx.r[28].s64 = ctx.r[11].s64 + 1084;
	// 82EAC57C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EAC580: 4BFFE0C1  bl 0x82eaa640
	ctx.lr = 0x82EAC584;
	sub_82EAA640(ctx, base);
	// 82EAC584: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EAC588: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82EAC58C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC590: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAC594: 40980024  bge cr6, 0x82eac5b8
	if !ctx.cr[6].lt {
	pc = 0x82EAC5B8; continue 'dispatch;
	}
	// 82EAC598: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC59C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC5A0: 41980008  blt cr6, 0x82eac5a8
	if ctx.cr[6].lt {
	pc = 0x82EAC5A8; continue 'dispatch;
	}
	// 82EAC5A4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAC5A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC5AC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC5B0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82EAC5B4: 4BFFA245  bl 0x82ea67f8
	ctx.lr = 0x82EAC5B8;
	sub_82EA67F8(ctx, base);
	// 82EAC5B8: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 82EAC5BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EAC5C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAC5C4: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EAC5C8: 4BFFE181  bl 0x82eaa748
	ctx.lr = 0x82EAC5CC;
	sub_82EAA748(ctx, base);
	// 82EAC5CC: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82EAC5D0: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82EAC5D4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAC5D8: 93610070  stw r27, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 82EAC5DC: 3BCBB234  addi r30, r11, -0x4dcc
	ctx.r[30].s64 = ctx.r[11].s64 + -19916;
	// 82EAC5E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAC5E4: 4BFFE05D  bl 0x82eaa640
	ctx.lr = 0x82EAC5E8;
	sub_82EAA640(ctx, base);
	// 82EAC5E8: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EAC5EC: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82EAC5F0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC5F4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAC5F8: 40980024  bge cr6, 0x82eac61c
	if !ctx.cr[6].lt {
	pc = 0x82EAC61C; continue 'dispatch;
	}
	// 82EAC5FC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC600: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC604: 41980008  blt cr6, 0x82eac60c
	if ctx.cr[6].lt {
	pc = 0x82EAC60C; continue 'dispatch;
	}
	// 82EAC608: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAC60C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC610: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC614: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82EAC618: 4BFFA1E1  bl 0x82ea67f8
	ctx.lr = 0x82EAC61C;
	sub_82EA67F8(ctx, base);
	// 82EAC61C: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82EAC620: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EAC624: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAC628: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EAC62C: 4BFFE11D  bl 0x82eaa748
	ctx.lr = 0x82EAC630;
	sub_82EAA748(ctx, base);
	// 82EAC630: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EAC634: 38C10078  addi r6, r1, 0x78
	ctx.r[6].s64 = ctx.r[1].s64 + 120;
	// 82EAC638: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82EAC63C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EAC640: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EAC644: 4BFFEDD5  bl 0x82eab418
	ctx.lr = 0x82EAC648;
	sub_82EAB418(ctx, base);
	// 82EAC648: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EAC64C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAC650: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAC654: 409A0020  bne cr6, 0x82eac674
	if !ctx.cr[6].eq {
	pc = 0x82EAC674; continue 'dispatch;
	}
	// 82EAC658: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC65C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAC660: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAC664: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EAC668: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC66C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAC670: 4BFF4141  bl 0x82ea07b0
	ctx.lr = 0x82EAC674;
	sub_82EA07B0(ctx, base);
	// 82EAC674: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EAC678: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAC67C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAC680: 409A0020  bne cr6, 0x82eac6a0
	if !ctx.cr[6].eq {
	pc = 0x82EAC6A0; continue 'dispatch;
	}
	// 82EAC684: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC688: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAC68C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAC690: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EAC694: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC698: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAC69C: 4BFF4115  bl 0x82ea07b0
	ctx.lr = 0x82EAC6A0;
	sub_82EA07B0(ctx, base);
	// 82EAC6A0: 93A10098  stw r29, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[29].u32 ) };
	// 82EAC6A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EAC6A8: 93A1009C  stw r29, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 82EAC6AC: 936100A0  stw r27, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u32 ) };
	// 82EAC6B0: 4BFFDF91  bl 0x82eaa640
	ctx.lr = 0x82EAC6B4;
	sub_82EAA640(ctx, base);
	// 82EAC6B4: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EAC6B8: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82EAC6BC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC6C0: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAC6C4: 40980024  bge cr6, 0x82eac6e8
	if !ctx.cr[6].lt {
	pc = 0x82EAC6E8; continue 'dispatch;
	}
	// 82EAC6C8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC6CC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC6D0: 41980008  blt cr6, 0x82eac6d8
	if ctx.cr[6].lt {
	pc = 0x82EAC6D8; continue 'dispatch;
	}
	// 82EAC6D4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAC6D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC6DC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC6E0: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82EAC6E4: 4BFFA115  bl 0x82ea67f8
	ctx.lr = 0x82EAC6E8;
	sub_82EA67F8(ctx, base);
	// 82EAC6E8: 93E1009C  stw r31, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[31].u32 ) };
	// 82EAC6EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EAC6F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAC6F4: 80610098  lwz r3, 0x98(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EAC6F8: 4BFFE051  bl 0x82eaa748
	ctx.lr = 0x82EAC6FC;
	sub_82EAA748(ctx, base);
	// 82EAC6FC: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82EAC700: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82EAC704: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAC708: 93610090  stw r27, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[27].u32 ) };
	// 82EAC70C: 3BCB0868  addi r30, r11, 0x868
	ctx.r[30].s64 = ctx.r[11].s64 + 2152;
	// 82EAC710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAC714: 4BFFDF2D  bl 0x82eaa640
	ctx.lr = 0x82EAC718;
	sub_82EAA640(ctx, base);
	// 82EAC718: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EAC71C: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82EAC720: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC724: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAC728: 40980024  bge cr6, 0x82eac74c
	if !ctx.cr[6].lt {
	pc = 0x82EAC74C; continue 'dispatch;
	}
	// 82EAC72C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC730: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC734: 41980008  blt cr6, 0x82eac73c
	if ctx.cr[6].lt {
	pc = 0x82EAC73C; continue 'dispatch;
	}
	// 82EAC738: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAC73C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC740: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC744: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82EAC748: 4BFFA0B1  bl 0x82ea67f8
	ctx.lr = 0x82EAC74C;
	sub_82EA67F8(ctx, base);
	// 82EAC74C: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82EAC750: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EAC754: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAC758: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82EAC75C: 4BFFDFED  bl 0x82eaa748
	ctx.lr = 0x82EAC760;
	sub_82EAA748(ctx, base);
	// 82EAC760: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EAC764: 38C10098  addi r6, r1, 0x98
	ctx.r[6].s64 = ctx.r[1].s64 + 152;
	// 82EAC768: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 82EAC76C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EAC770: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EAC774: 4BFFECA5  bl 0x82eab418
	ctx.lr = 0x82EAC778;
	sub_82EAB418(ctx, base);
	// 82EAC778: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC77C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAC780: 409A0014  bne cr6, 0x82eac794
	if !ctx.cr[6].eq {
	pc = 0x82EAC794; continue 'dispatch;
	}
	// 82EAC784: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EAC788: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAC78C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EAC790: 419A0008  beq cr6, 0x82eac798
	if ctx.cr[6].eq {
	pc = 0x82EAC798; continue 'dispatch;
	}
	// 82EAC794: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82EAC798: 81410090  lwz r10, 0x90(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EAC79C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82EAC7A0: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAC7A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAC7A8: 409A0020  bne cr6, 0x82eac7c8
	if !ctx.cr[6].eq {
	pc = 0x82EAC7C8; continue 'dispatch;
	}
	// 82EAC7AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC7B0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAC7B4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAC7B8: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82EAC7BC: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC7C0: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAC7C4: 4BFF3FED  bl 0x82ea07b0
	ctx.lr = 0x82EAC7C8;
	sub_82EA07B0(ctx, base);
	// 82EAC7C8: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EAC7CC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAC7D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAC7D4: 409A0020  bne cr6, 0x82eac7f4
	if !ctx.cr[6].eq {
	pc = 0x82EAC7F4; continue 'dispatch;
	}
	// 82EAC7D8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC7DC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAC7E0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAC7E4: 80810098  lwz r4, 0x98(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EAC7E8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC7EC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAC7F0: 4BFF3FC1  bl 0x82ea07b0
	ctx.lr = 0x82EAC7F4;
	sub_82EA07B0(ctx, base);
	// 82EAC7F4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAC7F8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EAC7FC: 38AB0864  addi r5, r11, 0x864
	ctx.r[5].s64 = ctx.r[11].s64 + 2148;
	// 82EAC800: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EAC804: 4BFFE52D  bl 0x82eaad30
	ctx.lr = 0x82EAC808;
	sub_82EAAD30(ctx, base);
	// 82EAC808: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC80C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAC810: 419A007C  beq cr6, 0x82eac88c
	if ctx.cr[6].eq {
	pc = 0x82EAC88C; continue 'dispatch;
	}
	// 82EAC814: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82EAC818: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EAC81C: 617EFFFF  ori r30, r11, 0xffff
	ctx.r[30].u64 = ctx.r[11].u64 | 65535;
	// 82EAC820: 396AFFFD  addi r11, r10, -3
	ctx.r[11].s64 = ctx.r[10].s64 + -3;
	// 82EAC824: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82EAC828: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EAC82C: 40980008  bge cr6, 0x82eac834
	if !ctx.cr[6].lt {
	pc = 0x82EAC834; continue 'dispatch;
	}
	// 82EAC830: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82EAC834: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAC838: 3BFE0001  addi r31, r30, 1
	ctx.r[31].s64 = ctx.r[30].s64 + 1;
	// 82EAC83C: 38830002  addi r4, r3, 2
	ctx.r[4].s64 = ctx.r[3].s64 + 2;
	// 82EAC840: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAC844: 4BFFDF0D  bl 0x82eaa750
	ctx.lr = 0x82EAC848;
	sub_82EAA750(ctx, base);
	// 82EAC848: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAC84C: 7FBE59AE  stbx r29, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u8) };
	// 82EAC850: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EAC854: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC858: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAC85C: 40980024  bge cr6, 0x82eac880
	if !ctx.cr[6].lt {
	pc = 0x82EAC880; continue 'dispatch;
	}
	// 82EAC860: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC864: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC868: 41980008  blt cr6, 0x82eac870
	if ctx.cr[6].lt {
	pc = 0x82EAC870; continue 'dispatch;
	}
	// 82EAC86C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAC870: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC874: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC878: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EAC87C: 4BFF9F7D  bl 0x82ea67f8
	ctx.lr = 0x82EAC880;
	sub_82EA67F8(ctx, base);
	// 82EAC880: 9B010050  stb r24, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u8 ) };
	// 82EAC884: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82EAC888: 48000008  b 0x82eac890
	pc = 0x82EAC890; continue 'dispatch;
	// 82EAC88C: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EAC890: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EAC894: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAC898: 419A0084  beq cr6, 0x82eac91c
	if ctx.cr[6].eq {
	pc = 0x82EAC91C; continue 'dispatch;
	}
	// 82EAC89C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAC8A0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC8A4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EAC8A8: 40980024  bge cr6, 0x82eac8cc
	if !ctx.cr[6].lt {
	pc = 0x82EAC8CC; continue 'dispatch;
	}
	// 82EAC8AC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EAC8B0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EAC8B4: 41980008  blt cr6, 0x82eac8bc
	if ctx.cr[6].lt {
	pc = 0x82EAC8BC; continue 'dispatch;
	}
	// 82EAC8B8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAC8BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EAC8C0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EAC8C4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EAC8C8: 4BFF9F31  bl 0x82ea67f8
	ctx.lr = 0x82EAC8CC;
	sub_82EA67F8(ctx, base);
	// 82EAC8CC: 93F90004  stw r31, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EAC8D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EAC8D4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAC8D8: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC8DC: 4BFFDE6D  bl 0x82eaa748
	ctx.lr = 0x82EAC8E0;
	sub_82EAA748(ctx, base);
	// 82EAC8E0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EAC8E4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAC8E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAC8EC: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC8F0: 409A0020  bne cr6, 0x82eac910
	if !ctx.cr[6].eq {
	pc = 0x82EAC910; continue 'dispatch;
	}
	// 82EAC8F4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC8F8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAC8FC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAC900: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAC904: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC908: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAC90C: 4BFF3EA5  bl 0x82ea07b0
	ctx.lr = 0x82EAC910;
	sub_82EA07B0(ctx, base);
	// 82EAC910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAC914: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82EAC918: 482FB890  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 82EAC91C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EAC920: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EAC924: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAC928: 409A0020  bne cr6, 0x82eac948
	if !ctx.cr[6].eq {
	pc = 0x82EAC948; continue 'dispatch;
	}
	// 82EAC92C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAC930: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EAC934: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EAC938: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EAC93C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EAC940: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EAC944: 4BFF3E6D  bl 0x82ea07b0
	ctx.lr = 0x82EAC948;
	sub_82EA07B0(ctx, base);
	// 82EAC948: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EAC94C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82EAC950: 482FB858  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAC958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EAC958 size=208
    let mut pc: u32 = 0x82EAC958;
    'dispatch: loop {
        match pc {
            0x82EAC958 => {
    //   block [0x82EAC958..0x82EACA28)
	// 82EAC958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAC95C: 482FB80D  bl 0x831a8168
	ctx.lr = 0x82EAC960;
	sub_831A8130(ctx, base);
	// 82EAC960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAC964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAC968: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EAC96C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAC970: 40980018  bge cr6, 0x82eac988
	if !ctx.cr[6].lt {
	pc = 0x82EAC988; continue 'dispatch;
	}
	// 82EAC974: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAC978: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EAC97C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EAC980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAC984: 482FB834  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EAC988: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAC98C: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EAC990: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EAC994: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EAC998: 40990024  ble cr6, 0x82eac9bc
	if !ctx.cr[6].gt {
	pc = 0x82EAC9BC; continue 'dispatch;
	}
	// 82EAC99C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAC9A0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EAC9A4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EAC9A8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EAC9AC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82EAC9B0: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EAC9B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAC9B8: 482FB800  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EAC9BC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EAC9C0: 40990060  ble cr6, 0x82eaca20
	if !ctx.cr[6].gt {
	pc = 0x82EACA20; continue 'dispatch;
	}
	// 82EAC9C4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EAC9C8: 7D694E70  srawi r9, r11, 9
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 9) as i64;
	// 82EAC9CC: 7D090194  addze r8, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82EAC9D0: 5507482C  slwi r7, r8, 9
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(9);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EAC9D4: 7F875851  subf. r28, r7, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EAC9D8: 23BC0200  subfic r29, r28, 0x200
	ctx.xer.ca = ctx.r[28].u32 <= 512 as u32;
	ctx.r[29].s64 = (512 as i64) - ctx.r[28].s64;
	// 82EAC9DC: 40820008  bne 0x82eac9e4
	if !ctx.cr[0].eq {
	pc = 0x82EAC9E4; continue 'dispatch;
	}
	// 82EAC9E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EAC9E4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAC9E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EAC9EC: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EAC9F0: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EAC9F4: 4BFFDD5D  bl 0x82eaa750
	ctx.lr = 0x82EAC9F8;
	sub_82EAA750(ctx, base);
	// 82EAC9F8: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 82EAC9FC: 7FCA4E70  srawi r10, r30, 9
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 9) as i64;
	// 82EACA00: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82EACA04: 5569DFFE  rlwinm r9, r11, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EACA08: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82EACA0C: 692A0001  xori r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u64 ^ 1;
	// 82EACA10: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EACA14: 5507482C  slwi r7, r8, 9
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(9);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EACA18: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82EACA1C: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82EACA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EACA24: 482FB794  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACA28 size=188
    let mut pc: u32 = 0x82EACA28;
    'dispatch: loop {
        match pc {
            0x82EACA28 => {
    //   block [0x82EACA28..0x82EACAE4)
	// 82EACA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACA2C: 482FB741  bl 0x831a816c
	ctx.lr = 0x82EACA30;
	sub_831A8130(ctx, base);
	// 82EACA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACA34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACA38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EACA3C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACA40: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACA44: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EACA48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EACA4C: 4E800421  bctrl
	ctx.lr = 0x82EACA50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACA50: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACA54: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EACA58: 409A0010  bne cr6, 0x82eaca68
	if !ctx.cr[6].eq {
	pc = 0x82EACA68; continue 'dispatch;
	}
	// 82EACA5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EACA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EACA64: 482FB758  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EACA68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACA6C: 4BFFFEED  bl 0x82eac958
	ctx.lr = 0x82EACA70;
	sub_82EAC958(ctx, base);
	// 82EACA70: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EACA74: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACA78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EACA7C: 7FCA5851  subf. r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EACA80: 40810048  ble 0x82eacac8
	if !ctx.cr[0].gt {
	pc = 0x82EACAC8; continue 'dispatch;
	}
	// 82EACA84: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACA88: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EACA8C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACA90: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EACA94: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EACA98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACA9C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACAA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EACAA4: 4E800421  bctrl
	ctx.lr = 0x82EACAA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACAA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACAAC: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82EACAB0: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EACAB4: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EACAB8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EACABC: 409A0018  bne cr6, 0x82eacad4
	if !ctx.cr[6].eq {
	pc = 0x82EACAD4; continue 'dispatch;
	}
	// 82EACAC0: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EACAC4: 4198FFC0  blt cr6, 0x82eaca84
	if ctx.cr[6].lt {
	pc = 0x82EACA84; continue 'dispatch;
	}
	// 82EACAC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EACACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EACAD0: 482FB6EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EACAD4: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82EACAD8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EACADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EACAE0: 482FB6DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACAE8 size=196
    let mut pc: u32 = 0x82EACAE8;
    'dispatch: loop {
        match pc {
            0x82EACAE8 => {
    //   block [0x82EACAE8..0x82EACBAC)
	// 82EACAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACAEC: 482FB679  bl 0x831a8164
	ctx.lr = 0x82EACAF0;
	sub_831A8130(ctx, base);
	// 82EACAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACAF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACAF8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EACAFC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EACB00: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82EACB04: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACB08: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACB0C: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EACB10: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EACB14: 4099005C  ble cr6, 0x82eacb70
	if !ctx.cr[6].gt {
	pc = 0x82EACB70; continue 'dispatch;
	}
	// 82EACB18: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EACB1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EACB20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EACB24: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EACB28: 4BFFDC21  bl 0x82eaa748
	ctx.lr = 0x82EACB2C;
	sub_82EAA748(ctx, base);
	// 82EACB2C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACB30: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACB34: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82EACB38: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EACB3C: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82EACB40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACB44: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EACB48: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EACB4C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EACB50: 4E800421  bctrl
	ctx.lr = 0x82EACB54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACB54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EACB58: 409A0048  bne cr6, 0x82eacba0
	if !ctx.cr[6].eq {
	pc = 0x82EACBA0; continue 'dispatch;
	}
	// 82EACB5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACB60: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACB64: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EACB68: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EACB6C: 4199FFAC  bgt cr6, 0x82eacb18
	if ctx.cr[6].gt {
	pc = 0x82EACB18; continue 'dispatch;
	}
	// 82EACB70: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EACB74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EACB78: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACB7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EACB80: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EACB84: 4BFFDBC5  bl 0x82eaa748
	ctx.lr = 0x82EACB88;
	sub_82EAA748(ctx, base);
	// 82EACB88: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACB8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EACB90: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EACB94: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EACB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EACB9C: 482FB618  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EACBA0: 7C7DD850  subf r3, r29, r27
	ctx.r[3].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 82EACBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EACBA8: 482FB60C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACBB0 size=132
    let mut pc: u32 = 0x82EACBB0;
    'dispatch: loop {
        match pc {
            0x82EACBB0 => {
    //   block [0x82EACBB0..0x82EACC34)
	// 82EACBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACBB4: 482FB5B9  bl 0x831a816c
	ctx.lr = 0x82EACBB8;
	sub_831A8130(ctx, base);
	// 82EACBB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACBBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACBC0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EACBC4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82EACBC8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACBCC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACBD0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EACBD4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EACBD8: 40990038  ble cr6, 0x82eacc10
	if !ctx.cr[6].gt {
	pc = 0x82EACC10; continue 'dispatch;
	}
	// 82EACBDC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACBE0: 7FCBF050  subf r30, r11, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82EACBE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACBE8: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EACBEC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EACBF0: 4E800421  bctrl
	ctx.lr = 0x82EACBF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACBF4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EACBF8: 409A0030  bne cr6, 0x82eacc28
	if !ctx.cr[6].eq {
	pc = 0x82EACC28; continue 'dispatch;
	}
	// 82EACBFC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACC00: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACC04: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EACC08: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EACC0C: 4199FFD0  bgt cr6, 0x82eacbdc
	if ctx.cr[6].gt {
	pc = 0x82EACBDC; continue 'dispatch;
	}
	// 82EACC10: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACC14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EACC18: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EACC1C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EACC20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EACC24: 482FB598  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EACC28: 7C7EE850  subf r3, r30, r29
	ctx.r[3].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82EACC2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EACC30: 482FB58C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACC38 size=108
    let mut pc: u32 = 0x82EACC38;
    'dispatch: loop {
        match pc {
            0x82EACC38 => {
    //   block [0x82EACC38..0x82EACCA4)
	// 82EACC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EACC40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EACC44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACC48: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACC4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACC50: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACC54: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EACC58: 409A002C  bne cr6, 0x82eacc84
	if !ctx.cr[6].eq {
	pc = 0x82EACC84; continue 'dispatch;
	}
	// 82EACC5C: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACC60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EACC64: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACC68: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EACC6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EACC70: 4E800421  bctrl
	ctx.lr = 0x82EACC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACC74: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EACC7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EACC80: 419A0008  beq cr6, 0x82eacc88
	if ctx.cr[6].eq {
	pc = 0x82EACC88; continue 'dispatch;
	}
	// 82EACC84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EACC88: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EACC8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACC90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EACC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACC9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EACCA8 size=24
    let mut pc: u32 = 0x82EACCA8;
    'dispatch: loop {
        match pc {
            0x82EACCA8 => {
    //   block [0x82EACCA8..0x82EACCC0)
	// 82EACCA8: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EACCAC: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EACCB0: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EACCB4: 69280001  xori r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 ^ 1;
	// 82EACCB8: 99030000  stb r8, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82EACCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EACCC0 size=28
    let mut pc: u32 = 0x82EACCC0;
    'dispatch: loop {
        match pc {
            0x82EACCC0 => {
    //   block [0x82EACCC0..0x82EACCDC)
	// 82EACCC0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACCC4: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EACCC8: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82EACCCC: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EACCD0: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EACCD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EACCD8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACCDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EACCDC size=8
    let mut pc: u32 = 0x82EACCDC;
    'dispatch: loop {
        match pc {
            0x82EACCDC => {
    //   block [0x82EACCDC..0x82EACCE4)
	// 82EACCDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EACCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EACCE8 size=24
    let mut pc: u32 = 0x82EACCE8;
    'dispatch: loop {
        match pc {
            0x82EACCE8 => {
    //   block [0x82EACCE8..0x82EACD00)
	// 82EACCE8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EACCEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EACCF0: 41980010  blt cr6, 0x82eacd00
	if ctx.cr[6].lt {
		sub_82EACD00(ctx, base);
		return;
	}
	// 82EACCF4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EACCF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EACCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EACD00 size=8
    let mut pc: u32 = 0x82EACD00;
    'dispatch: loop {
        match pc {
            0x82EACD00 => {
    //   block [0x82EACD00..0x82EACD08)
	// 82EACD00: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EACD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACD08 size=64
    let mut pc: u32 = 0x82EACD08;
    'dispatch: loop {
        match pc {
            0x82EACD08 => {
    //   block [0x82EACD08..0x82EACD48)
	// 82EACD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EACD10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EACD14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACD18: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACD1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACD20: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACD24: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EACD28: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EACD2C: 4E800421  bctrl
	ctx.lr = 0x82EACD30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACD34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EACD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACD40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EACD48 size=44
    let mut pc: u32 = 0x82EACD48;
    'dispatch: loop {
        match pc {
            0x82EACD48 => {
    //   block [0x82EACD48..0x82EACD74)
	// 82EACD48: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EACD4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EACD50: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82EACD54: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EACD58: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EACD5C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EACD60: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACD64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACD68: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EACD6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EACD70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACD78 size=108
    let mut pc: u32 = 0x82EACD78;
    'dispatch: loop {
        match pc {
            0x82EACD78 => {
    //   block [0x82EACD78..0x82EACDE4)
	// 82EACD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EACD80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EACD84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACD88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACD8C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACD90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACD94: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EACD98: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EACD9C: 4E800421  bctrl
	ctx.lr = 0x82EACDA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACDA0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EACDA4: 41980028  blt cr6, 0x82eacdcc
	if ctx.cr[6].lt {
	pc = 0x82EACDCC; continue 'dispatch;
	}
	// 82EACDA8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EACDAC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EACDB0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EACDB4: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EACDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EACDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACDC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACDC8: 4E800020  blr
	return;
	// 82EACDCC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EACDD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EACDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACDDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACDE8 size=128
    let mut pc: u32 = 0x82EACDE8;
    'dispatch: loop {
        match pc {
            0x82EACDE8 => {
    //   block [0x82EACDE8..0x82EACE68)
	// 82EACDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EACDF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EACDF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EACDF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACDFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACE00: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EACE04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACE08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EACE0C: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EACE10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EACE14: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EACE18: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82EACE1C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACE20: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACE24: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EACE28: 4E800421  bctrl
	ctx.lr = 0x82EACE2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACE2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EACE30: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EACE34: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EACE38: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82EACE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACE40: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EACE44: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EACE48: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EACE4C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EACE50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EACE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACE5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EACE60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EACE68 size=32
    let mut pc: u32 = 0x82EACE68;
    'dispatch: loop {
        match pc {
            0x82EACE68 => {
    //   block [0x82EACE68..0x82EACE88)
	// 82EACE68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACE6C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EACE70: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACE74: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EACE78: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACE7C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EACE80: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EACE84: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACE88 size=108
    let mut pc: u32 = 0x82EACE88;
    'dispatch: loop {
        match pc {
            0x82EACE88 => {
    //   block [0x82EACE88..0x82EACEF4)
	// 82EACE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EACE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EACE94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACE98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACE9C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EACEA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EACEA4: 392B0870  addi r9, r11, 0x870
	ctx.r[9].s64 = ctx.r[11].s64 + 2160;
	// 82EACEA8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82EACEAC: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82EACEB0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EACEB4: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EACEB8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EACEBC: 4BFFFF2D  bl 0x82eacde8
	ctx.lr = 0x82EACEC0;
	sub_82EACDE8(ctx, base);
	// 82EACEC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACEC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACEC8: A10B0004  lhz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EACECC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EACED0: 419A0010  beq cr6, 0x82eacee0
	if ctx.cr[6].eq {
	pc = 0x82EACEE0; continue 'dispatch;
	}
	// 82EACED4: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EACED8: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82EACEDC: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EACEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EACEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACEEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACEF8 size=156
    let mut pc: u32 = 0x82EACEF8;
    'dispatch: loop {
        match pc {
            0x82EACEF8 => {
    //   block [0x82EACEF8..0x82EACF94)
	// 82EACEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EACF00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EACF04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACF08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACF0C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EACF10: 394B0870  addi r10, r11, 0x870
	ctx.r[10].s64 = ctx.r[11].s64 + 2160;
	// 82EACF14: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EACF18: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EACF1C: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EACF20: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EACF24: 419A0030  beq cr6, 0x82eacf54
	if ctx.cr[6].eq {
	pc = 0x82EACF54; continue 'dispatch;
	}
	// 82EACF28: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EACF2C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EACF30: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EACF34: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EACF38: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EACF3C: 409A0018  bne cr6, 0x82eacf54
	if !ctx.cr[6].eq {
	pc = 0x82EACF54; continue 'dispatch;
	}
	// 82EACF40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACF44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EACF48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACF4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EACF50: 4E800421  bctrl
	ctx.lr = 0x82EACF54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACF54: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACF58: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EACF5C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EACF60: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EACF64: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACF68: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EACF6C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EACF70: 4E800421  bctrl
	ctx.lr = 0x82EACF74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EACF74: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82EACF78: 38C79EAC  addi r6, r7, -0x6154
	ctx.r[6].s64 = ctx.r[7].s64 + -24916;
	// 82EACF7C: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EACF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EACF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACF8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EACF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EACF98 size=100
    let mut pc: u32 = 0x82EACF98;
    'dispatch: loop {
        match pc {
            0x82EACF98 => {
    //   block [0x82EACF98..0x82EACFFC)
	// 82EACF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EACF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EACFA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EACFA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EACFA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EACFAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EACFB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EACFB4: 4BFFFF45  bl 0x82eacef8
	ctx.lr = 0x82EACFB8;
	sub_82EACEF8(ctx, base);
	// 82EACFB8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EACFBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EACFC0: 419A0020  beq cr6, 0x82eacfe0
	if ctx.cr[6].eq {
	pc = 0x82EACFE0; continue 'dispatch;
	}
	// 82EACFC4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EACFC8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EACFCC: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EACFD0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EACFD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EACFD8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EACFDC: 4BFF37D5  bl 0x82ea07b0
	ctx.lr = 0x82EACFE0;
	sub_82EA07B0(ctx, base);
	// 82EACFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EACFE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EACFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EACFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EACFF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EACFF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EACFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD000 size=72
    let mut pc: u32 = 0x82EAD000;
    'dispatch: loop {
        match pc {
            0x82EAD000 => {
    //   block [0x82EAD000..0x82EAD048)
	// 82EAD000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD00C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD014: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EAD018: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EAD01C: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD020: 48302079  bl 0x831af098
	ctx.lr = 0x82EAD024;
	sub_831AF098(ctx, base);
	// 82EAD024: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EAD028: 4199000C  bgt cr6, 0x82ead034
	if ctx.cr[6].gt {
	pc = 0x82EAD034; continue 'dispatch;
	}
	// 82EAD02C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAD030: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82EAD034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD048 size=12
    let mut pc: u32 = 0x82EAD048;
    'dispatch: loop {
        match pc {
            0x82EAD048 => {
    //   block [0x82EAD048..0x82EAD054)
	// 82EAD048: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD04C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAD050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD058 size=12
    let mut pc: u32 = 0x82EAD058;
    'dispatch: loop {
        match pc {
            0x82EAD058 => {
    //   block [0x82EAD058..0x82EAD064)
	// 82EAD058: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAD05C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAD060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD068 size=12
    let mut pc: u32 = 0x82EAD068;
    'dispatch: loop {
        match pc {
            0x82EAD068 => {
    //   block [0x82EAD068..0x82EAD074)
	// 82EAD068: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAD06C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAD070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD078 size=48
    let mut pc: u32 = 0x82EAD078;
    'dispatch: loop {
        match pc {
            0x82EAD078 => {
    //   block [0x82EAD078..0x82EAD0A8)
	// 82EAD078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD084: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD088: 48302101  bl 0x831af188
	ctx.lr = 0x82EAD08C;
	sub_831AF188(ctx, base);
	// 82EAD08C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EAD090: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EAD094: 69430001  xori r3, r10, 1
	ctx.r[3].u64 = ctx.r[10].u64 ^ 1;
	// 82EAD098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD0A8 size=8
    let mut pc: u32 = 0x82EAD0A8;
    'dispatch: loop {
        match pc {
            0x82EAD0A8 => {
    //   block [0x82EAD0A8..0x82EAD0B0)
	// 82EAD0A8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD0AC: 48304884  b 0x831b1930
	sub_831B1930(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD0B0 size=112
    let mut pc: u32 = 0x82EAD0B0;
    'dispatch: loop {
        match pc {
            0x82EAD0B0 => {
    //   block [0x82EAD0B0..0x82EAD120)
	// 82EAD0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD0B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD0BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD0C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD0C4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EAD0C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAD0CC: 392A08AC  addi r9, r10, 0x8ac
	ctx.r[9].s64 = ctx.r[10].s64 + 2220;
	// 82EAD0D0: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82EAD0D4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82EAD0D8: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82EAD0DC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EAD0E0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EAD0E4: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EAD0E8: 388708A4  addi r4, r7, 0x8a4
	ctx.r[4].s64 = ctx.r[7].s64 + 2212;
	// 82EAD0EC: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82EAD0F0: 48304561  bl 0x831b1650
	ctx.lr = 0x82EAD0F4;
	sub_831B1650(ctx, base);
	// 82EAD0F4: 7C660034  cntlzw r6, r3
	ctx.r[6].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EAD0F8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EAD0FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD100: 54C5DFFE  rlwinm r5, r6, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 82EAD104: 68A40001  xori r4, r5, 1
	ctx.r[4].u64 = ctx.r[5].u64 ^ 1;
	// 82EAD108: 989F0010  stb r4, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u8 ) };
	// 82EAD10C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD118: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD120 size=80
    let mut pc: u32 = 0x82EAD120;
    'dispatch: loop {
        match pc {
            0x82EAD120 => {
    //   block [0x82EAD120..0x82EAD170)
	// 82EAD120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD12C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD134: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAD138: 394B08AC  addi r10, r11, 0x8ac
	ctx.r[10].s64 = ctx.r[11].s64 + 2220;
	// 82EAD13C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD140: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAD144: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAD148: 419A0008  beq cr6, 0x82ead150
	if ctx.cr[6].eq {
	pc = 0x82EAD150; continue 'dispatch;
	}
	// 82EAD14C: 48302575  bl 0x831af6c0
	ctx.lr = 0x82EAD150;
	sub_831AF6C0(ctx, base);
	// 82EAD150: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EAD154: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EAD158: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAD15C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD170 size=72
    let mut pc: u32 = 0x82EAD170;
    'dispatch: loop {
        match pc {
            0x82EAD170 => {
    //   block [0x82EAD170..0x82EAD1B8)
	// 82EAD170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD17C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD184: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD188: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EAD18C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD190: 4E800421  bctrl
	ctx.lr = 0x82EAD194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD194: 39230001  addi r9, r3, 1
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	// 82EAD198: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EAD19C: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82EAD1A0: 5503DFFE  rlwinm r3, r8, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82EAD1A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD1B8 size=24
    let mut pc: u32 = 0x82EAD1B8;
    'dispatch: loop {
        match pc {
            0x82EAD1B8 => {
    //   block [0x82EAD1B8..0x82EAD1D0)
	// 82EAD1B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD1BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EAD1C0: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD1C4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EAD1C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD1CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD1D0 size=136
    let mut pc: u32 = 0x82EAD1D0;
    'dispatch: loop {
        match pc {
            0x82EAD1D0 => {
    //   block [0x82EAD1D0..0x82EAD258)
	// 82EAD1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD1D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAD1DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD1E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD1E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD1E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAD1EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EAD1F0: 394B08AC  addi r10, r11, 0x8ac
	ctx.r[10].s64 = ctx.r[11].s64 + 2220;
	// 82EAD1F4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD1F8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAD1FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAD200: 419A0008  beq cr6, 0x82ead208
	if ctx.cr[6].eq {
	pc = 0x82EAD208; continue 'dispatch;
	}
	// 82EAD204: 483024BD  bl 0x831af6c0
	ctx.lr = 0x82EAD208;
	sub_831AF6C0(ctx, base);
	// 82EAD208: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EAD20C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EAD210: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EAD214: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAD218: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EAD21C: 419A0020  beq cr6, 0x82ead23c
	if ctx.cr[6].eq {
	pc = 0x82EAD23C; continue 'dispatch;
	}
	// 82EAD220: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD224: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAD228: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EAD22C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAD230: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAD234: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAD238: 4BFF3579  bl 0x82ea07b0
	ctx.lr = 0x82EAD23C;
	sub_82EA07B0(ctx, base);
	// 82EAD23C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD24C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAD250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD258 size=140
    let mut pc: u32 = 0x82EAD258;
    'dispatch: loop {
        match pc {
            0x82EAD258 => {
    //   block [0x82EAD258..0x82EAD2E4)
	// 82EAD258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD25C: 482FAF11  bl 0x831a816c
	ctx.lr = 0x82EAD260;
	sub_831A8130(ctx, base);
	// 82EAD260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD264: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAD268: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD26C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD270: 419A0068  beq cr6, 0x82ead2d8
	if ctx.cr[6].eq {
	pc = 0x82EAD2D8; continue 'dispatch;
	}
	// 82EAD274: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD278: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EAD27C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EAD280: 40990038  ble cr6, 0x82ead2b8
	if !ctx.cr[6].gt {
	pc = 0x82EAD2B8; continue 'dispatch;
	}
	// 82EAD284: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD288: 7CBFE850  subf r5, r31, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82EAD28C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD290: 7C9F5A14  add r4, r31, r11
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82EAD294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD298: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD29C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD2A0: 4E800421  bctrl
	ctx.lr = 0x82EAD2A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD2A4: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EAD2A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EAD2AC: 419A0020  beq cr6, 0x82ead2cc
	if ctx.cr[6].eq {
	pc = 0x82EAD2CC; continue 'dispatch;
	}
	// 82EAD2B0: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EAD2B4: 4198FFD0  blt cr6, 0x82ead284
	if ctx.cr[6].lt {
	pc = 0x82EAD284; continue 'dispatch;
	}
	// 82EAD2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAD2BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD2C0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EAD2C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD2C8: 482FAEF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EAD2CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD2D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD2D4: 482FAEE8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EAD2D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAD2DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD2E0: 482FAEDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD2E8 size=184
    let mut pc: u32 = 0x82EAD2E8;
    'dispatch: loop {
        match pc {
            0x82EAD2E8 => {
    //   block [0x82EAD2E8..0x82EAD3A0)
	// 82EAD2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD2EC: 482FAE79  bl 0x831a8164
	ctx.lr = 0x82EAD2F0;
	sub_831A8130(ctx, base);
	// 82EAD2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD2F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD2F8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EAD2FC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EAD300: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82EAD304: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EAD308: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD30C: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EAD310: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAD314: 40990050  ble cr6, 0x82ead364
	if !ctx.cr[6].gt {
	pc = 0x82EAD364; continue 'dispatch;
	}
	// 82EAD318: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD31C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EAD320: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EAD324: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EAD328: 4BFFD421  bl 0x82eaa748
	ctx.lr = 0x82EAD32C;
	sub_82EAA748(ctx, base);
	// 82EAD32C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD330: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82EAD334: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82EAD338: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EAD33C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD340: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EAD344: 4BFFFF15  bl 0x82ead258
	ctx.lr = 0x82EAD348;
	sub_82EAD258(ctx, base);
	// 82EAD348: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAD34C: 409A0048  bne cr6, 0x82ead394
	if !ctx.cr[6].eq {
	pc = 0x82EAD394; continue 'dispatch;
	}
	// 82EAD350: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD354: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EAD358: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EAD35C: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EAD360: 4199FFB8  bgt cr6, 0x82ead318
	if ctx.cr[6].gt {
	pc = 0x82EAD318; continue 'dispatch;
	}
	// 82EAD364: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD368: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EAD36C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD370: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EAD374: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EAD378: 4BFFD3D1  bl 0x82eaa748
	ctx.lr = 0x82EAD37C;
	sub_82EAA748(ctx, base);
	// 82EAD37C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD380: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EAD384: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EAD388: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EAD38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAD390: 482FAE24  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EAD394: 7C7DD850  subf r3, r29, r27
	ctx.r[3].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 82EAD398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EAD39C: 482FAE18  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD3A0 size=76
    let mut pc: u32 = 0x82EAD3A0;
    'dispatch: loop {
        match pc {
            0x82EAD3A0 => {
    //   block [0x82EAD3A0..0x82EAD3EC)
	// 82EAD3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD3A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD3AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD3B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD3B4: 4BFFFEA5  bl 0x82ead258
	ctx.lr = 0x82EAD3B8;
	sub_82EAD258(ctx, base);
	// 82EAD3B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD3BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD3C0: 419A0018  beq cr6, 0x82ead3d8
	if ctx.cr[6].eq {
	pc = 0x82EAD3D8; continue 'dispatch;
	}
	// 82EAD3C4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAD3C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD3CC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EAD3D0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD3D4: 4E800421  bctrl
	ctx.lr = 0x82EAD3D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD3E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD3F0 size=192
    let mut pc: u32 = 0x82EAD3F0;
    'dispatch: loop {
        match pc {
            0x82EAD3F0 => {
    //   block [0x82EAD3F0..0x82EAD4B0)
	// 82EAD3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD3F4: 482FAD79  bl 0x831a816c
	ctx.lr = 0x82EAD3F8;
	sub_831A8130(ctx, base);
	// 82EAD3F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD3FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAD400: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EAD404: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EAD408: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD40C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD410: 419A002C  beq cr6, 0x82ead43c
	if ctx.cr[6].eq {
	pc = 0x82EAD43C; continue 'dispatch;
	}
	// 82EAD414: 4BFFFE45  bl 0x82ead258
	ctx.lr = 0x82EAD418;
	sub_82EAD258(ctx, base);
	// 82EAD418: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD41C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EAD420: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAD424: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD428: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EAD42C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD430: 4E800421  bctrl
	ctx.lr = 0x82EAD434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD434: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD438: 482FAD84  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EAD43C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EAD440: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 82EAD444: 41980028  blt cr6, 0x82ead46c
	if ctx.cr[6].lt {
	pc = 0x82EAD46C; continue 'dispatch;
	}
	// 82EAD448: 419A0018  beq cr6, 0x82ead460
	if ctx.cr[6].eq {
	pc = 0x82EAD460; continue 'dispatch;
	}
	// 82EAD44C: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 82EAD450: 40980020  bge cr6, 0x82ead470
	if !ctx.cr[6].lt {
	pc = 0x82EAD470; continue 'dispatch;
	}
	// 82EAD454: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD458: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82EAD45C: 48000014  b 0x82ead470
	pc = 0x82EAD470; continue 'dispatch;
	// 82EAD460: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD464: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EAD468: 48000008  b 0x82ead470
	pc = 0x82EAD470; continue 'dispatch;
	// 82EAD46C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EAD470: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAD474: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAD478: 40980018  bge cr6, 0x82ead490
	if !ctx.cr[6].lt {
	pc = 0x82EAD490; continue 'dispatch;
	}
	// 82EAD47C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAD480: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EAD484: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EAD488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD48C: 482FAD30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EAD490: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EAD494: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EAD498: 4099000C  ble cr6, 0x82ead4a4
	if !ctx.cr[6].gt {
	pc = 0x82EAD4A4; continue 'dispatch;
	}
	// 82EAD49C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EAD4A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EAD4A4: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EAD4A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD4AC: 482FAD10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD4B0 size=120
    let mut pc: u32 = 0x82EAD4B0;
    'dispatch: loop {
        match pc {
            0x82EAD4B0 => {
    //   block [0x82EAD4B0..0x82EAD528)
	// 82EAD4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD4B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD4BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD4C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD4C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD4C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD4CC: 419A003C  beq cr6, 0x82ead508
	if ctx.cr[6].eq {
	pc = 0x82EAD508; continue 'dispatch;
	}
	// 82EAD4D0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EAD4D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD4D8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EAD4DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD4E0: 4E800421  bctrl
	ctx.lr = 0x82EAD4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD4E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EAD4E8: 41980028  blt cr6, 0x82ead510
	if ctx.cr[6].lt {
	pc = 0x82EAD510; continue 'dispatch;
	}
	// 82EAD4EC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD4F0: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EAD4F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD504: 4E800020  blr
	return;
	// 82EAD508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAD50C: 4BFFFFE0  b 0x82ead4ec
	pc = 0x82EAD4EC; continue 'dispatch;
	// 82EAD510: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EAD514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD528 size=128
    let mut pc: u32 = 0x82EAD528;
    'dispatch: loop {
        match pc {
            0x82EAD528 => {
    //   block [0x82EAD528..0x82EAD5A8)
	// 82EAD528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD538: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD53C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAD540: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EAD544: 390B08E0  addi r8, r11, 0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2272;
	// 82EAD548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EAD54C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EAD550: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EAD554: 7CCB0774  extsb r11, r6
	ctx.r[11].s64 = ctx.r[6].s8 as i64;
	// 82EAD558: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EAD55C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EAD560: 3925FFFF  addi r9, r5, -1
	ctx.r[9].s64 = ctx.r[5].s64 + -1;
	// 82EAD564: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EAD568: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAD56C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EAD570: 409A0008  bne cr6, 0x82ead578
	if !ctx.cr[6].eq {
	pc = 0x82EAD578; continue 'dispatch;
	}
	// 82EAD574: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82EAD578: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EAD57C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EAD580: 995F0018  stb r10, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82EAD584: 419A000C  beq cr6, 0x82ead590
	if ctx.cr[6].eq {
	pc = 0x82EAD590; continue 'dispatch;
	}
	// 82EAD588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EAD58C: 4BFFD1CD  bl 0x82eaa758
	ctx.lr = 0x82EAD590;
	sub_82EAA758(ctx, base);
	// 82EAD590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD594: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD5A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD5A8 size=120
    let mut pc: u32 = 0x82EAD5A8;
    'dispatch: loop {
        match pc {
            0x82EAD5A8 => {
    //   block [0x82EAD5A8..0x82EAD620)
	// 82EAD5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD5B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD5B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD5B8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD5BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD5C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EAD5C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD5C8: 419A001C  beq cr6, 0x82ead5e4
	if ctx.cr[6].eq {
	pc = 0x82EAD5E4; continue 'dispatch;
	}
	// 82EAD5CC: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EAD5D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD5D4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD5D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD5DC: 4E800421  bctrl
	ctx.lr = 0x82EAD5E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD5E0: 48000020  b 0x82ead600
	pc = 0x82EAD600; continue 'dispatch;
	// 82EAD5E4: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EAD5E8: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EAD5EC: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EAD5F0: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82EAD5F4: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82EAD5F8: 68E60001  xori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 ^ 1;
	// 82EAD5FC: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82EAD600: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD608: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAD60C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD620 size=100
    let mut pc: u32 = 0x82EAD620;
    'dispatch: loop {
        match pc {
            0x82EAD620 => {
    //   block [0x82EAD620..0x82EAD684)
	// 82EAD620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD62C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD630: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EAD63C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD640: 419A001C  beq cr6, 0x82ead65c
	if ctx.cr[6].eq {
	pc = 0x82EAD65C; continue 'dispatch;
	}
	// 82EAD644: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EAD648: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD64C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EAD650: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD654: 4E800421  bctrl
	ctx.lr = 0x82EAD658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD658: 4800000C  b 0x82ead664
	pc = 0x82EAD664; continue 'dispatch;
	// 82EAD65C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAD660: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82EAD664: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD66C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAD670: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD67C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD688 size=176
    let mut pc: u32 = 0x82EAD688;
    'dispatch: loop {
        match pc {
            0x82EAD688 => {
    //   block [0x82EAD688..0x82EAD738)
	// 82EAD688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAD694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD698: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD69C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD6A0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EAD6A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAD6A8: 392A08E0  addi r9, r10, 0x8e0
	ctx.r[9].s64 = ctx.r[10].s64 + 2272;
	// 82EAD6AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EAD6B0: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82EAD6B4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82EAD6B8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EAD6BC: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EAD6C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD6C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD6C8: 419A001C  beq cr6, 0x82ead6e4
	if ctx.cr[6].eq {
	pc = 0x82EAD6E4; continue 'dispatch;
	}
	// 82EAD6CC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAD6D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EAD6D4: 419A0010  beq cr6, 0x82ead6e4
	if ctx.cr[6].eq {
	pc = 0x82EAD6E4; continue 'dispatch;
	}
	// 82EAD6D8: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EAD6DC: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82EAD6E0: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EAD6E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD6E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAD6EC: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EAD6F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EAD6F4: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82EAD6F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAD6FC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD700: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD704: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EAD708: 4E800421  bctrl
	ctx.lr = 0x82EAD70C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EAD710: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EAD714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD718: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82EAD71C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EAD720: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD72C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAD730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD738 size=208
    let mut pc: u32 = 0x82EAD738;
    'dispatch: loop {
        match pc {
            0x82EAD738 => {
    //   block [0x82EAD738..0x82EAD808)
	// 82EAD738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD74C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EAD750: 394B08E0  addi r10, r11, 0x8e0
	ctx.r[10].s64 = ctx.r[11].s64 + 2272;
	// 82EAD754: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAD758: 4BFFFB01  bl 0x82ead258
	ctx.lr = 0x82EAD75C;
	sub_82EAD258(ctx, base);
	// 82EAD75C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD760: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAD764: 419A0014  beq cr6, 0x82ead778
	if ctx.cr[6].eq {
	pc = 0x82EAD778; continue 'dispatch;
	}
	// 82EAD768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD76C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EAD770: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD774: 4E800421  bctrl
	ctx.lr = 0x82EAD778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD778: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD77C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAD780: 419A003C  beq cr6, 0x82ead7bc
	if ctx.cr[6].eq {
	pc = 0x82EAD7BC; continue 'dispatch;
	}
	// 82EAD784: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAD788: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD78C: 419A0030  beq cr6, 0x82ead7bc
	if ctx.cr[6].eq {
	pc = 0x82EAD7BC; continue 'dispatch;
	}
	// 82EAD790: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EAD794: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EAD798: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EAD79C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EAD7A0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EAD7A4: 409A0018  bne cr6, 0x82ead7bc
	if !ctx.cr[6].eq {
	pc = 0x82EAD7BC; continue 'dispatch;
	}
	// 82EAD7A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD7AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EAD7B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD7B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EAD7B8: 4E800421  bctrl
	ctx.lr = 0x82EAD7BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD7BC: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EAD7C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD7C4: 419A0024  beq cr6, 0x82ead7e8
	if ctx.cr[6].eq {
	pc = 0x82EAD7E8; continue 'dispatch;
	}
	// 82EAD7C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD7CC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAD7D0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD7D4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAD7D8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD7DC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAD7E0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EAD7E4: 4E800421  bctrl
	ctx.lr = 0x82EAD7E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EAD7E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EAD7EC: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EAD7F0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EAD7F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD808 size=100
    let mut pc: u32 = 0x82EAD808;
    'dispatch: loop {
        match pc {
            0x82EAD808 => {
    //   block [0x82EAD808..0x82EAD86C)
	// 82EAD808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAD814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD81C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD820: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EAD824: 4BFFFF15  bl 0x82ead738
	ctx.lr = 0x82EAD828;
	sub_82EAD738(ctx, base);
	// 82EAD828: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EAD82C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD830: 419A0020  beq cr6, 0x82ead850
	if ctx.cr[6].eq {
	pc = 0x82EAD850; continue 'dispatch;
	}
	// 82EAD834: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EAD838: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EAD83C: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EAD840: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EAD844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EAD848: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EAD84C: 4BFF2F65  bl 0x82ea07b0
	ctx.lr = 0x82EAD850;
	sub_82EA07B0(ctx, base);
	// 82EAD850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EAD854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD860: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAD864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD870 size=56
    let mut pc: u32 = 0x82EAD870;
    'dispatch: loop {
        match pc {
            0x82EAD870 => {
    //   block [0x82EAD870..0x82EAD8A8)
	// 82EAD870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD87C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD884: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EAD888: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EAD88C: 48303DC5  bl 0x831b1650
	ctx.lr = 0x82EAD890;
	sub_831B1650(ctx, base);
	// 82EAD890: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EAD894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD8A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD8A8 size=76
    let mut pc: u32 = 0x82EAD8A8;
    'dispatch: loop {
        match pc {
            0x82EAD8A8 => {
    //   block [0x82EAD8A8..0x82EAD8F4)
	// 82EAD8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD8B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD8B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD8B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD8BC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD8C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAD8C4: 419A0014  beq cr6, 0x82ead8d8
	if ctx.cr[6].eq {
	pc = 0x82EAD8D8; continue 'dispatch;
	}
	// 82EAD8C8: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD8CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD8D0: 419A0008  beq cr6, 0x82ead8d8
	if ctx.cr[6].eq {
	pc = 0x82EAD8D8; continue 'dispatch;
	}
	// 82EAD8D4: 48301DED  bl 0x831af6c0
	ctx.lr = 0x82EAD8D8;
	sub_831AF6C0(ctx, base);
	// 82EAD8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAD8DC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EAD8E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD8EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD8F8 size=132
    let mut pc: u32 = 0x82EAD8F8;
    'dispatch: loop {
        match pc {
            0x82EAD8F8 => {
    //   block [0x82EAD8F8..0x82EAD97C)
	// 82EAD8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EAD904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EAD908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD90C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EAD910: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EAD914: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD918: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82EAD91C: 419A0044  beq cr6, 0x82ead960
	if ctx.cr[6].eq {
	pc = 0x82EAD960; continue 'dispatch;
	}
	// 82EAD920: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EAD924: 48301BDD  bl 0x831af500
	ctx.lr = 0x82EAD928;
	sub_831AF500(ctx, base);
	// 82EAD928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EAD92C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EAD930: 41990028  bgt cr6, 0x82ead958
	if ctx.cr[6].gt {
	pc = 0x82EAD958; continue 'dispatch;
	}
	// 82EAD934: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD938: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAD93C: 419A0014  beq cr6, 0x82ead950
	if ctx.cr[6].eq {
	pc = 0x82EAD950; continue 'dispatch;
	}
	// 82EAD940: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EAD944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EAD948: 419A0008  beq cr6, 0x82ead950
	if ctx.cr[6].eq {
	pc = 0x82EAD950; continue 'dispatch;
	}
	// 82EAD94C: 48301D75  bl 0x831af6c0
	ctx.lr = 0x82EAD950;
	sub_831AF6C0(ctx, base);
	// 82EAD950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EAD954: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EAD958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EAD95C: 48000008  b 0x82ead964
	pc = 0x82EAD964; continue 'dispatch;
	// 82EAD960: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EAD964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EAD968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EAD974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EAD978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD980 size=12
    let mut pc: u32 = 0x82EAD980;
    'dispatch: loop {
        match pc {
            0x82EAD980 => {
    //   block [0x82EAD980..0x82EAD98C)
	// 82EAD980: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD984: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EAD988: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD98C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD98C size=8
    let mut pc: u32 = 0x82EAD98C;
    'dispatch: loop {
        match pc {
            0x82EAD98C => {
    //   block [0x82EAD98C..0x82EAD994)
	// 82EAD98C: 48302FF4  b 0x831b0980
	sub_831B0980(ctx, base);
	return;
	// 82EAD990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD998 size=24
    let mut pc: u32 = 0x82EAD998;
    'dispatch: loop {
        match pc {
            0x82EAD998 => {
    //   block [0x82EAD998..0x82EAD9B0)
	// 82EAD998: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD99C: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EAD9A0: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EAD9A4: 69280001  xori r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 ^ 1;
	// 82EAD9A8: 99030000  stb r8, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82EAD9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD9B0 size=12
    let mut pc: u32 = 0x82EAD9B0;
    'dispatch: loop {
        match pc {
            0x82EAD9B0 => {
    //   block [0x82EAD9B0..0x82EAD9BC)
	// 82EAD9B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAD9B4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAD9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EAD9C0 size=12
    let mut pc: u32 = 0x82EAD9C0;
    'dispatch: loop {
        match pc {
            0x82EAD9C0 => {
    //   block [0x82EAD9C0..0x82EAD9CC)
	// 82EAD9C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EAD9C4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EAD9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EAD9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EAD9D0 size=48
    let mut pc: u32 = 0x82EAD9D0;
    'dispatch: loop {
        match pc {
            0x82EAD9D0 => {
    //   block [0x82EAD9D0..0x82EADA00)
	// 82EAD9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EAD9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EAD9D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EAD9DC: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EAD9E0: 483017A9  bl 0x831af188
	ctx.lr = 0x82EAD9E4;
	sub_831AF188(ctx, base);
	// 82EAD9E4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EAD9E8: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EAD9EC: 69430001  xori r3, r10, 1
	ctx.r[3].u64 = ctx.r[10].u64 ^ 1;
	// 82EAD9F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EAD9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EAD9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EAD9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EADA00 size=8
    let mut pc: u32 = 0x82EADA00;
    'dispatch: loop {
        match pc {
            0x82EADA00 => {
    //   block [0x82EADA00..0x82EADA08)
	// 82EADA00: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EADA04: 48303F2C  b 0x831b1930
	sub_831B1930(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADA08 size=104
    let mut pc: u32 = 0x82EADA08;
    'dispatch: loop {
        match pc {
            0x82EADA08 => {
    //   block [0x82EADA08..0x82EADA70)
	// 82EADA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EADA14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADA18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADA1C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EADA20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EADA24: 392B0908  addi r9, r11, 0x908
	ctx.r[9].s64 = ctx.r[11].s64 + 2312;
	// 82EADA28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EADA2C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EADA30: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82EADA34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EADA38: 991F000C  stb r8, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 82EADA3C: 409A0010  bne cr6, 0x82eada4c
	if !ctx.cr[6].eq {
	pc = 0x82EADA4C; continue 'dispatch;
	}
	// 82EADA40: 483029B1  bl 0x831b03f0
	ctx.lr = 0x82EADA44;
	sub_831B03F0(ctx, base);
	// 82EADA44: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82EADA48: 4800000C  b 0x82eada54
	pc = 0x82EADA54; continue 'dispatch;
	// 82EADA4C: 483029A5  bl 0x831b03f0
	ctx.lr = 0x82EADA50;
	sub_831B03F0(ctx, base);
	// 82EADA50: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82EADA54: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EADA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADA5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EADA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADA68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EADA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADA70 size=104
    let mut pc: u32 = 0x82EADA70;
    'dispatch: loop {
        match pc {
            0x82EADA70 => {
    //   block [0x82EADA70..0x82EADAD8)
	// 82EADA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADA78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EADA7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADA80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADA84: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EADA88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EADA8C: 392A0908  addi r9, r10, 0x908
	ctx.r[9].s64 = ctx.r[10].s64 + 2312;
	// 82EADA90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EADA94: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82EADA98: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EADA9C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EADAA0: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EADAA4: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82EADAA8: 419A0018  beq cr6, 0x82eadac0
	if ctx.cr[6].eq {
	pc = 0x82EADAC0; continue 'dispatch;
	}
	// 82EADAAC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EADAB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EADAB4: 388B0930  addi r4, r11, 0x930
	ctx.r[4].s64 = ctx.r[11].s64 + 2352;
	// 82EADAB8: 48303B99  bl 0x831b1650
	ctx.lr = 0x82EADABC;
	sub_831B1650(ctx, base);
	// 82EADABC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EADAC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADAC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EADAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADAD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EADAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADAD8 size=100
    let mut pc: u32 = 0x82EADAD8;
    'dispatch: loop {
        match pc {
            0x82EADAD8 => {
    //   block [0x82EADAD8..0x82EADB3C)
	// 82EADAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADAE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EADAE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADAE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADAEC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EADAF0: 394B0908  addi r10, r11, 0x908
	ctx.r[10].s64 = ctx.r[11].s64 + 2312;
	// 82EADAF4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EADAF8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EADAFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EADB00: 419A0014  beq cr6, 0x82eadb14
	if ctx.cr[6].eq {
	pc = 0x82EADB14; continue 'dispatch;
	}
	// 82EADB04: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EADB08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EADB0C: 419A0008  beq cr6, 0x82eadb14
	if ctx.cr[6].eq {
	pc = 0x82EADB14; continue 'dispatch;
	}
	// 82EADB10: 48301BB1  bl 0x831af6c0
	ctx.lr = 0x82EADB14;
	sub_831AF6C0(ctx, base);
	// 82EADB14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EADB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EADB1C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EADB20: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EADB24: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EADB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EADB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADB34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EADB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADB40 size=156
    let mut pc: u32 = 0x82EADB40;
    'dispatch: loop {
        match pc {
            0x82EADB40 => {
    //   block [0x82EADB40..0x82EADBDC)
	// 82EADB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADB48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EADB4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EADB50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADB58: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EADB5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EADB60: 394B0908  addi r10, r11, 0x908
	ctx.r[10].s64 = ctx.r[11].s64 + 2312;
	// 82EADB64: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EADB68: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EADB6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EADB70: 419A0014  beq cr6, 0x82eadb84
	if ctx.cr[6].eq {
	pc = 0x82EADB84; continue 'dispatch;
	}
	// 82EADB74: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EADB78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EADB7C: 419A0008  beq cr6, 0x82eadb84
	if ctx.cr[6].eq {
	pc = 0x82EADB84; continue 'dispatch;
	}
	// 82EADB80: 48301B41  bl 0x831af6c0
	ctx.lr = 0x82EADB84;
	sub_831AF6C0(ctx, base);
	// 82EADB84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EADB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EADB8C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EADB90: 57C807FE  clrlwi r8, r30, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EADB94: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EADB98: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EADB9C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EADBA0: 419A0020  beq cr6, 0x82eadbc0
	if ctx.cr[6].eq {
	pc = 0x82EADBC0; continue 'dispatch;
	}
	// 82EADBA4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EADBA8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EADBAC: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EADBB0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EADBB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EADBB8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EADBBC: 4BFF2BF5  bl 0x82ea07b0
	ctx.lr = 0x82EADBC0;
	sub_82EA07B0(ctx, base);
	// 82EADBC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADBC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EADBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADBD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EADBD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EADBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADBE0 size=108
    let mut pc: u32 = 0x82EADBE0;
    'dispatch: loop {
        match pc {
            0x82EADBE0 => {
    //   block [0x82EADBE0..0x82EADC4C)
	// 82EADBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADBE4: 482FA585  bl 0x831a8168
	ctx.lr = 0x82EADBE8;
	sub_831A8130(ctx, base);
	// 82EADBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADBEC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EADBF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EADBF4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EADBF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EADBFC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EADC00: 40990034  ble cr6, 0x82eadc34
	if !ctx.cr[6].gt {
	pc = 0x82EADC34; continue 'dispatch;
	}
	// 82EADC04: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EADC08: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82EADC0C: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 82EADC10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EADC14: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EADC18: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EADC1C: 4E800421  bctrl
	ctx.lr = 0x82EADC20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EADC20: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EADC24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EADC28: 419A0018  beq cr6, 0x82eadc40
	if ctx.cr[6].eq {
	pc = 0x82EADC40; continue 'dispatch;
	}
	// 82EADC2C: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EADC30: 4198FFD4  blt cr6, 0x82eadc04
	if ctx.cr[6].lt {
	pc = 0x82EADC04; continue 'dispatch;
	}
	// 82EADC34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EADC38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EADC3C: 482FA57C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EADC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EADC48: 482FA570  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADC50 size=64
    let mut pc: u32 = 0x82EADC50;
    'dispatch: loop {
        match pc {
            0x82EADC50 => {
    //   block [0x82EADC50..0x82EADC90)
	// 82EADC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADC58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EADC5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADC60: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EADC64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADC68: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EADC6C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EADC70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EADC74: 4E800421  bctrl
	ctx.lr = 0x82EADC78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EADC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADC7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EADC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADC88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EADC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADC90 size=108
    let mut pc: u32 = 0x82EADC90;
    'dispatch: loop {
        match pc {
            0x82EADC90 => {
    //   block [0x82EADC90..0x82EADCFC)
	// 82EADC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADC94: 482FA4D5  bl 0x831a8168
	ctx.lr = 0x82EADC98;
	sub_831A8130(ctx, base);
	// 82EADC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADC9C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EADCA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EADCA4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EADCA8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EADCAC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EADCB0: 40990034  ble cr6, 0x82eadce4
	if !ctx.cr[6].gt {
	pc = 0x82EADCE4; continue 'dispatch;
	}
	// 82EADCB4: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EADCB8: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82EADCBC: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 82EADCC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EADCC4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EADCC8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EADCCC: 4E800421  bctrl
	ctx.lr = 0x82EADCD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EADCD0: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EADCD4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EADCD8: 419A0018  beq cr6, 0x82eadcf0
	if ctx.cr[6].eq {
	pc = 0x82EADCF0; continue 'dispatch;
	}
	// 82EADCDC: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EADCE0: 4198FFD4  blt cr6, 0x82eadcb4
	if ctx.cr[6].lt {
	pc = 0x82EADCB4; continue 'dispatch;
	}
	// 82EADCE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EADCE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EADCEC: 482FA4CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EADCF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADCF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EADCF8: 482FA4C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADD00 size=64
    let mut pc: u32 = 0x82EADD00;
    'dispatch: loop {
        match pc {
            0x82EADD00 => {
    //   block [0x82EADD00..0x82EADD40)
	// 82EADD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADD08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EADD0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADD10: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EADD14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADD18: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EADD1C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EADD20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EADD24: 4E800421  bctrl
	ctx.lr = 0x82EADD28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EADD28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADD2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EADD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADD38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EADD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADD40 size=68
    let mut pc: u32 = 0x82EADD40;
    'dispatch: loop {
        match pc {
            0x82EADD40 => {
    //   block [0x82EADD40..0x82EADD84)
	// 82EADD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADD48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADD4C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EADD50: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EADD54: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82EADD58: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82EADD5C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EADD60: 4BFF29D1  bl 0x82ea0730
	ctx.lr = 0x82EADD64;
	sub_82EA0730(ctx, base);
	// 82EADD64: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 82EADD68: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EADD6C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82EADD70: 48003451  bl 0x82eb11c0
	ctx.lr = 0x82EADD74;
	sub_82EB11C0(ctx, base);
	// 82EADD74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EADD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADD88 size=148
    let mut pc: u32 = 0x82EADD88;
    'dispatch: loop {
        match pc {
            0x82EADD88 => {
    //   block [0x82EADD88..0x82EADE1C)
	// 82EADD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADD8C: 482FA3E1  bl 0x831a816c
	ctx.lr = 0x82EADD90;
	sub_831A8130(ctx, base);
	// 82EADD90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADD94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADD98: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EADD9C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EADDA0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EADDA4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82EADDA8: 390B0994  addi r8, r11, 0x994
	ctx.r[8].s64 = ctx.r[11].s64 + 2452;
	// 82EADDAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EADDB0: B3BF0006  sth r29, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[29].u16 ) };
	// 82EADDB4: 3FC08338  lis r30, -0x7cc8
	ctx.r[30].s64 = -2093481984;
	// 82EADDB8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EADDBC: 38EA0938  addi r7, r10, 0x938
	ctx.r[7].s64 = ctx.r[10].s64 + 2360;
	// 82EADDC0: B3BF000E  sth r29, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[29].u16 ) };
	// 82EADDC4: 38C9096C  addi r6, r9, 0x96c
	ctx.r[6].s64 = ctx.r[9].s64 + 2412;
	// 82EADDC8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EADDCC: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82EADDD0: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82EADDD4: B3BF001A  sth r29, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[29].u16 ) };
	// 82EADDD8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EADDDC: 93FF0010  stw r31, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82EADDE0: 93FF001C  stw r31, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82EADDE4: 897E70F0  lbz r11, 0x70f0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28912 as u32) ) } as u64;
	// 82EADDE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EADDEC: 409A0024  bne cr6, 0x82eade10
	if !ctx.cr[6].eq {
	pc = 0x82EADE10; continue 'dispatch;
	}
	// 82EADDF0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82EADDF4: 814BC57C  lwz r10, -0x3a84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14980 as u32) ) } as u64;
	// 82EADDF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EADDFC: 419A0014  beq cr6, 0x82eade10
	if ctx.cr[6].eq {
	pc = 0x82EADE10; continue 'dispatch;
	}
	// 82EADE00: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EADE04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EADE08: 4E800421  bctrl
	ctx.lr = 0x82EADE0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EADE0C: 9BBE70F0  stb r29, 0x70f0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28912 as u32), ctx.r[29].u8 ) };
	// 82EADE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADE14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EADE18: 482FA3A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EADE20 size=104
    let mut pc: u32 = 0x82EADE20;
    'dispatch: loop {
        match pc {
            0x82EADE20 => {
    //   block [0x82EADE20..0x82EADE88)
	// 82EADE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EADE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EADE28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EADE2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EADE30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EADE34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EADE38: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82EADE3C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EADE40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EADE44: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EADE48: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EADE4C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EADE50: 419A0020  beq cr6, 0x82eade70
	if ctx.cr[6].eq {
	pc = 0x82EADE70; continue 'dispatch;
	}
	// 82EADE54: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EADE58: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EADE5C: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82EADE60: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EADE64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EADE68: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EADE6C: 4BFF2945  bl 0x82ea07b0
	ctx.lr = 0x82EADE70;
	sub_82EA07B0(ctx, base);
	// 82EADE70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EADE74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EADE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EADE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EADE80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EADE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EADE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EADE88 size=8
    let mut pc: u32 = 0x82EADE88;
    'dispatch: loop {
        match pc {
            0x82EADE88 => {
    //   block [0x82EADE88..0x82EADE90)
	// 82EADE88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EADE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


