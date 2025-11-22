pub fn sub_82EA16D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA16D0 size=228
    let mut pc: u32 = 0x82EA16D0;
    'dispatch: loop {
        match pc {
            0x82EA16D0 => {
    //   block [0x82EA16D0..0x82EA17B4)
	// 82EA16D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA16D4: 48306A89  bl 0x831a815c
	ctx.lr = 0x82EA16D8;
	sub_831A8130(ctx, base);
	// 82EA16D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA16DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA16E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA16E4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82EA16E8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EA16EC: 480009C5  bl 0x82ea20b0
	ctx.lr = 0x82EA16F0;
	sub_82EA20B0(ctx, base);
	// 82EA16F0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA16F4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EA16F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA16FC: 83FC0038  lwz r31, 0x38(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1700: 4099009C  ble cr6, 0x82ea179c
	if !ctx.cr[6].gt {
	pc = 0x82EA179C; continue 'dispatch;
	}
	// 82EA1704: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82EA1708: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EA170C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA1710: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA1714: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA1718: 7F6BFA14  add r27, r11, r31
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA171C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1720: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82EA1724: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EA1728: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA172C: 409A000C  bne cr6, 0x82ea1738
	if !ctx.cr[6].eq {
	pc = 0x82EA1738; continue 'dispatch;
	}
	// 82EA1730: 48000D69  bl 0x82ea2498
	ctx.lr = 0x82EA1734;
	sub_82EA2498(ctx, base);
	// 82EA1734: 48000008  b 0x82ea173c
	pc = 0x82EA173C; continue 'dispatch;
	// 82EA1738: 48000CA9  bl 0x82ea23e0
	ctx.lr = 0x82EA173C;
	sub_82EA23E0(ctx, base);
	// 82EA173C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1740: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA1744: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1748: 41820040  beq 0x82ea1788
	if ctx.cr[0].eq {
	pc = 0x82EA1788; continue 'dispatch;
	}
	// 82EA174C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1750: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1754: 419A0014  beq cr6, 0x82ea1768
	if ctx.cr[6].eq {
	pc = 0x82EA1768; continue 'dispatch;
	}
	// 82EA1758: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA175C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA1760: 807C0054  lwz r3, 0x54(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1764: 4800001C  b 0x82ea1780
	pc = 0x82EA1780; continue 'dispatch;
	// 82EA1768: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA176C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1770: 419A0018  beq cr6, 0x82ea1788
	if ctx.cr[6].eq {
	pc = 0x82EA1788; continue 'dispatch;
	}
	// 82EA1774: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1778: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA177C: 807C0058  lwz r3, 0x58(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA1780: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1784: 48000EDD  bl 0x82ea2660
	ctx.lr = 0x82EA1788;
	sub_82EA2660(ctx, base);
	// 82EA1788: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA178C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82EA1790: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EA1794: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA1798: 4198FF84  blt cr6, 0x82ea171c
	if ctx.cr[6].lt {
	pc = 0x82EA171C; continue 'dispatch;
	}
	// 82EA179C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA17A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA17A4: F97C0020  std r11, 0x20(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA17A8: 483A11B5  bl 0x8324295c
	ctx.lr = 0x82EA17AC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA17AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA17B0: 483069FC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA17B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA17B8 size=1160
    let mut pc: u32 = 0x82EA17B8;
    'dispatch: loop {
        match pc {
            0x82EA17B8 => {
    //   block [0x82EA17B8..0x82EA1C40)
	// 82EA17B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA17BC: 4830698D  bl 0x831a8148
	ctx.lr = 0x82EA17C0;
	sub_831A8130(ctx, base);
	// 82EA17C0: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA17C4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA17C8: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82EA17CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA17D0: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82EA17D4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EA17D8: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82EA17DC: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA17E0: 7D164378  mr r22, r8
	ctx.r[22].u64 = ctx.r[8].u64;
	// 82EA17E4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA17E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA17EC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA17F0: 40980020  bge cr6, 0x82ea1810
	if !ctx.cr[6].lt {
	pc = 0x82EA1810; continue 'dispatch;
	}
	// 82EA17F4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA17F8: 3909E924  addi r8, r9, -0x16dc
	ctx.r[8].s64 = ctx.r[9].s64 + -5852;
	// 82EA17FC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA1800: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EA1804: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EA1808: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA180C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA1810: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA1814: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA1818: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82EA181C: 3A8BBADC  addi r20, r11, -0x4524
	ctx.r[20].s64 = ctx.r[11].s64 + -17700;
	// 82EA1820: 3AEAE914  addi r23, r10, -0x16ec
	ctx.r[23].s64 = ctx.r[10].s64 + -5868;
	// 82EA1824: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1828: 48000889  bl 0x82ea20b0
	ctx.lr = 0x82EA182C;
	sub_82EA20B0(ctx, base);
	// 82EA182C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EA1830: 83FD0038  lwz r31, 0x38(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1834: 419A0074  beq cr6, 0x82ea18a8
	if ctx.cr[6].eq {
	pc = 0x82EA18A8; continue 'dispatch;
	}
	// 82EA1838: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EA183C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EA1840: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EA1844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA1848: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA184C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA1850: 4E800421  bctrl
	ctx.lr = 0x82EA1854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA1854: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA1858: 409A0038  bne cr6, 0x82ea1890
	if !ctx.cr[6].eq {
	pc = 0x82EA1890; continue 'dispatch;
	}
	// 82EA185C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1860: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EA1864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA1868: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA186C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1870: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA1874: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA1878: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA187C: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA1880: 409A000C  bne cr6, 0x82ea188c
	if !ctx.cr[6].eq {
	pc = 0x82EA188C; continue 'dispatch;
	}
	// 82EA1884: 48000C15  bl 0x82ea2498
	ctx.lr = 0x82EA1888;
	sub_82EA2498(ctx, base);
	// 82EA1888: 48000008  b 0x82ea1890
	pc = 0x82EA1890; continue 'dispatch;
	// 82EA188C: 48000B55  bl 0x82ea23e0
	ctx.lr = 0x82EA1890;
	sub_82EA23E0(ctx, base);
	// 82EA1890: 2F18FFFF  cmpwi cr6, r24, -1
	ctx.cr[6].compare_i32(ctx.r[24].s32, -1, &mut ctx.xer);
	// 82EA1894: 419A0274  beq cr6, 0x82ea1b08
	if ctx.cr[6].eq {
	pc = 0x82EA1B08; continue 'dispatch;
	}
	// 82EA1898: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA189C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EA18A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA18A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA18A8: 3978000F  addi r11, r24, 0xf
	ctx.r[11].s64 = ctx.r[24].s64 + 15;
	// 82EA18AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA18B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA18B4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EA18B8: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA18BC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EA18C0: 40990038  ble cr6, 0x82ea18f8
	if !ctx.cr[6].gt {
	pc = 0x82EA18F8; continue 'dispatch;
	}
	// 82EA18C4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EA18C8: 7D6850AE  lbzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA18CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA18D0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA18D4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EA18D8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA18DC: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA18E0: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA18E4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA18E8: 409A015C  bne cr6, 0x82ea1a44
	if !ctx.cr[6].eq {
	pc = 0x82EA1A44; continue 'dispatch;
	}
	// 82EA18EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA18F0: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA18F4: 4198FFD4  blt cr6, 0x82ea18c8
	if ctx.cr[6].lt {
	pc = 0x82EA18C8; continue 'dispatch;
	}
	// 82EA18F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA18FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1900: 409A0048  bne cr6, 0x82ea1948
	if !ctx.cr[6].eq {
	pc = 0x82EA1948; continue 'dispatch;
	}
	// 82EA1904: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1908: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA190C: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1910: 40820038  bne 0x82ea1948
	if !ctx.cr[0].eq {
	pc = 0x82EA1948; continue 'dispatch;
	}
	// 82EA1914: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA1918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA191C: 419A02D8  beq cr6, 0x82ea1bf4
	if ctx.cr[6].eq {
	pc = 0x82EA1BF4; continue 'dispatch;
	}
	// 82EA1920: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1928: 40990018  ble cr6, 0x82ea1940
	if !ctx.cr[6].gt {
	pc = 0x82EA1940; continue 'dispatch;
	}
	// 82EA192C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1930: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1934: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA1938: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA193C: 48000D25  bl 0x82ea2660
	ctx.lr = 0x82EA1940;
	sub_82EA2660(ctx, base);
	// 82EA1940: 2F180002  cmpwi cr6, r24, 2
	ctx.cr[6].compare_i32(ctx.r[24].s32, 2, &mut ctx.xer);
	// 82EA1944: 419A02BC  beq cr6, 0x82ea1c00
	if ctx.cr[6].eq {
	pc = 0x82EA1C00; continue 'dispatch;
	}
	// 82EA1948: 2F160001  cmpwi cr6, r22, 1
	ctx.cr[6].compare_i32(ctx.r[22].s32, 1, &mut ctx.xer);
	// 82EA194C: 419A0208  beq cr6, 0x82ea1b54
	if ctx.cr[6].eq {
	pc = 0x82EA1B54; continue 'dispatch;
	}
	// 82EA1950: 2F180002  cmpwi cr6, r24, 2
	ctx.cr[6].compare_i32(ctx.r[24].s32, 2, &mut ctx.xer);
	// 82EA1954: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1958: 419A0078  beq cr6, 0x82ea19d0
	if ctx.cr[6].eq {
	pc = 0x82EA19D0; continue 'dispatch;
	}
	// 82EA195C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1960: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1964: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA1968: FB3D0020  std r25, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[25].u64 ) };
	// 82EA196C: 483A0FF1  bl 0x8324295c
	ctx.lr = 0x82EA1970;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1970: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1974: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1978: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA197C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1980: 40980018  bge cr6, 0x82ea1998
	if !ctx.cr[6].lt {
	pc = 0x82EA1998; continue 'dispatch;
	}
	// 82EA1984: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82EA1988: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA198C: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1990: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1994: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1998: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA199C: 48000C95  bl 0x82ea2630
	ctx.lr = 0x82EA19A0;
	sub_82EA2630(ctx, base);
	// 82EA19A0: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA19A4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA19A8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA19AC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA19B0: 4098FE74  bge cr6, 0x82ea1824
	if !ctx.cr[6].lt {
	pc = 0x82EA1824; continue 'dispatch;
	}
	// 82EA19B4: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA19B8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA19BC: 5528003E  slwi r8, r9, 0
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA19C0: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA19C4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EA19C8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA19CC: 4BFFFE58  b 0x82ea1824
	pc = 0x82EA1824; continue 'dispatch;
	// 82EA19D0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA19D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA19D8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA19DC: FB3D0020  std r25, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[25].u64 ) };
	// 82EA19E0: 483A0F7D  bl 0x8324295c
	ctx.lr = 0x82EA19E4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA19E4: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA19E8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA19EC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA19F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA19F4: 40980018  bge cr6, 0x82ea1a0c
	if !ctx.cr[6].lt {
	pc = 0x82EA1A0C; continue 'dispatch;
	}
	// 82EA19F8: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82EA19FC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1A00: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1A04: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1A08: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1A0C: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1A10: 48000C21  bl 0x82ea2630
	ctx.lr = 0x82EA1A14;
	sub_82EA2630(ctx, base);
	// 82EA1A14: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1A18: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1A1C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1A20: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1A24: 4098FE00  bge cr6, 0x82ea1824
	if !ctx.cr[6].lt {
	pc = 0x82EA1824; continue 'dispatch;
	}
	// 82EA1A28: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1A2C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1A30: 5528003E  slwi r8, r9, 0
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA1A34: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1A38: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EA1A3C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1A40: 4BFFFDE4  b 0x82ea1824
	pc = 0x82EA1824; continue 'dispatch;
	// 82EA1A44: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA1A48: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 82EA1A4C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1A50: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EA1A54: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA1A58: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EA1A5C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA1A60: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EA1A64: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EA1A68: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EA1A6C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EA1A70: 4200FFF0  bdnz 0x82ea1a60
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EA1A60; continue 'dispatch;
	}
	// 82EA1A74: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA1A78: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1A7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1A80: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA1A84: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EA1A88: 409A000C  bne cr6, 0x82ea1a94
	if !ctx.cr[6].eq {
	pc = 0x82EA1A94; continue 'dispatch;
	}
	// 82EA1A8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA1A90: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA1A94: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA1A98: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82EA1A9C: 38A100E0  addi r5, r1, 0xe0
	ctx.r[5].s64 = ctx.r[1].s64 + 224;
	// 82EA1AA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1AA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA1AA8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EA1AAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1AB0: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EA1AB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA1AB8: 4E800421  bctrl
	ctx.lr = 0x82EA1ABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA1ABC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82EA1AC0: 409A0010  bne cr6, 0x82ea1ad0
	if !ctx.cr[6].eq {
	pc = 0x82EA1AD0; continue 'dispatch;
	}
	// 82EA1AC4: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 82EA1AC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA1ACC: 480009CD  bl 0x82ea2498
	ctx.lr = 0x82EA1AD0;
	sub_82EA2498(ctx, base);
	// 82EA1AD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1AD4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA1AD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1ADC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA1AE0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1AE4: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1AE8: 418200CC  beq 0x82ea1bb4
	if ctx.cr[0].eq {
	pc = 0x82EA1BB4; continue 'dispatch;
	}
	// 82EA1AEC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1AF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1AF4: 419A00A0  beq cr6, 0x82ea1b94
	if ctx.cr[6].eq {
	pc = 0x82EA1B94; continue 'dispatch;
	}
	// 82EA1AF8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1AFC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA1B00: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1B04: 480000A8  b 0x82ea1bac
	pc = 0x82EA1BAC; continue 'dispatch;
	// 82EA1B08: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1B0C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA1B10: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1B14: 41820040  beq 0x82ea1b54
	if ctx.cr[0].eq {
	pc = 0x82EA1B54; continue 'dispatch;
	}
	// 82EA1B18: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1B1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1B20: 419A0014  beq cr6, 0x82ea1b34
	if ctx.cr[6].eq {
	pc = 0x82EA1B34; continue 'dispatch;
	}
	// 82EA1B24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1B28: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA1B2C: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1B30: 4800001C  b 0x82ea1b4c
	pc = 0x82EA1B4C; continue 'dispatch;
	// 82EA1B34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1B38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1B3C: 419A0018  beq cr6, 0x82ea1b54
	if ctx.cr[6].eq {
	pc = 0x82EA1B54; continue 'dispatch;
	}
	// 82EA1B40: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1B44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA1B48: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA1B4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1B50: 48000B11  bl 0x82ea2660
	ctx.lr = 0x82EA1B54;
	sub_82EA2660(ctx, base);
	// 82EA1B54: FB3D0020  std r25, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[25].u64 ) };
	// 82EA1B58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1B5C: 483A0E01  bl 0x8324295c
	ctx.lr = 0x82EA1B60;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1B60: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1B64: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1B68: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1B6C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1B70: 40980018  bge cr6, 0x82ea1b88
	if !ctx.cr[6].lt {
	pc = 0x82EA1B88; continue 'dispatch;
	}
	// 82EA1B74: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1B78: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1B7C: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1B80: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1B84: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1B88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA1B8C: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82EA1B90: 48306608  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 82EA1B94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1B98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1B9C: 419A0018  beq cr6, 0x82ea1bb4
	if ctx.cr[6].eq {
	pc = 0x82EA1BB4; continue 'dispatch;
	}
	// 82EA1BA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1BA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA1BA8: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA1BAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1BB0: 48000AB1  bl 0x82ea2660
	ctx.lr = 0x82EA1BB4;
	sub_82EA2660(ctx, base);
	// 82EA1BB4: FB3D0020  std r25, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[25].u64 ) };
	// 82EA1BB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1BBC: 483A0DA1  bl 0x8324295c
	ctx.lr = 0x82EA1BC0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1BC0: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1BC4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1BC8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1BCC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1BD0: 40980018  bge cr6, 0x82ea1be8
	if !ctx.cr[6].lt {
	pc = 0x82EA1BE8; continue 'dispatch;
	}
	// 82EA1BD4: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1BD8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1BDC: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1BE0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1BE4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1BE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA1BEC: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82EA1BF0: 483065A8  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 82EA1BF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA1BF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1BFC: 4BFFF665  bl 0x82ea1260
	ctx.lr = 0x82EA1C00;
	sub_82EA1260(ctx, base);
	// 82EA1C00: FB3D0020  std r25, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[25].u64 ) };
	// 82EA1C04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1C08: 483A0D55  bl 0x8324295c
	ctx.lr = 0x82EA1C0C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1C0C: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1C10: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1C14: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1C18: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1C1C: 40980018  bge cr6, 0x82ea1c34
	if !ctx.cr[6].lt {
	pc = 0x82EA1C34; continue 'dispatch;
	}
	// 82EA1C20: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1C24: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1C28: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1C2C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1C30: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1C34: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82EA1C38: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82EA1C3C: 4830655C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA1C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA1C40 size=1020
    let mut pc: u32 = 0x82EA1C40;
    'dispatch: loop {
        match pc {
            0x82EA1C40 => {
    //   block [0x82EA1C40..0x82EA203C)
	// 82EA1C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA1C44: 483064F9  bl 0x831a813c
	ctx.lr = 0x82EA1C48;
	sub_831A8130(ctx, base);
	// 82EA1C48: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA1C4C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1C50: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82EA1C54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA1C58: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82EA1C5C: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82EA1C60: 7CD33378  mr r19, r6
	ctx.r[19].u64 = ctx.r[6].u64;
	// 82EA1C64: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1C68: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EA1C6C: 7D114378  mr r17, r8
	ctx.r[17].u64 = ctx.r[8].u64;
	// 82EA1C70: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1C74: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1C78: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1C7C: 40980020  bge cr6, 0x82ea1c9c
	if !ctx.cr[6].lt {
	pc = 0x82EA1C9C; continue 'dispatch;
	}
	// 82EA1C80: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA1C84: 3909E924  addi r8, r9, -0x16dc
	ctx.r[8].s64 = ctx.r[9].s64 + -5852;
	// 82EA1C88: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA1C8C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EA1C90: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EA1C94: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1C98: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA1C9C: 3973000F  addi r11, r19, 0xf
	ctx.r[11].s64 = ctx.r[19].s64 + 15;
	// 82EA1CA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EA1CA4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA1CA8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82EA1CAC: 7EABEA14  add r21, r11, r29
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EA1CB0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA1CB4: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82EA1CB8: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82EA1CBC: 3AEBE914  addi r23, r11, -0x16ec
	ctx.r[23].s64 = ctx.r[11].s64 + -5868;
	// 82EA1CC0: 3A8ABADC  addi r20, r10, -0x4524
	ctx.r[20].s64 = ctx.r[10].s64 + -17700;
	// 82EA1CC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1CC8: 480003E9  bl 0x82ea20b0
	ctx.lr = 0x82EA1CCC;
	sub_82EA20B0(ctx, base);
	// 82EA1CCC: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82EA1CD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA1CD4: 83DD0038  lwz r30, 0x38(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1CD8: 419A003C  beq cr6, 0x82ea1d14
	if ctx.cr[6].eq {
	pc = 0x82EA1D14; continue 'dispatch;
	}
	// 82EA1CDC: 39780001  addi r11, r24, 1
	ctx.r[11].s64 = ctx.r[24].s64 + 1;
	// 82EA1CE0: 2F120000  cmpwi cr6, r18, 0
	ctx.cr[6].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82EA1CE4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA1CE8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EA1CEC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA1CF0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA1CF4: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA1CF8: 409A000C  bne cr6, 0x82ea1d04
	if !ctx.cr[6].eq {
	pc = 0x82EA1D04; continue 'dispatch;
	}
	// 82EA1CFC: 4800079D  bl 0x82ea2498
	ctx.lr = 0x82EA1D00;
	sub_82EA2498(ctx, base);
	// 82EA1D00: 48000008  b 0x82ea1d08
	pc = 0x82EA1D08; continue 'dispatch;
	// 82EA1D04: 480006DD  bl 0x82ea23e0
	ctx.lr = 0x82EA1D08;
	sub_82EA23E0(ctx, base);
	// 82EA1D08: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1D0C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1D10: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA1D14: 88F50000  lbz r7, 0(r21)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1D18: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 82EA1D1C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EA1D20: 40990038  ble cr6, 0x82ea1d58
	if !ctx.cr[6].gt {
	pc = 0x82EA1D58; continue 'dispatch;
	}
	// 82EA1D24: 39150001  addi r8, r21, 1
	ctx.r[8].s64 = ctx.r[21].s64 + 1;
	// 82EA1D28: 7D6848AE  lbzx r11, r8, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EA1D2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1D30: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA1D34: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA1D38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA1D3C: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA1D40: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA1D44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA1D48: 409A00C4  bne cr6, 0x82ea1e0c
	if !ctx.cr[6].eq {
	pc = 0x82EA1E0C; continue 'dispatch;
	}
	// 82EA1D4C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EA1D50: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA1D54: 4198FFD4  blt cr6, 0x82ea1d28
	if ctx.cr[6].lt {
	pc = 0x82EA1D28; continue 'dispatch;
	}
	// 82EA1D58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA1D5C: 7EDFB378  mr r31, r22
	ctx.r[31].u64 = ctx.r[22].u64;
	// 82EA1D60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1D64: 419A01B8  beq cr6, 0x82ea1f1c
	if ctx.cr[6].eq {
	pc = 0x82EA1F1C; continue 'dispatch;
	}
	// 82EA1D68: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1D6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1D70: 409A0014  bne cr6, 0x82ea1d84
	if !ctx.cr[6].eq {
	pc = 0x82EA1D84; continue 'dispatch;
	}
	// 82EA1D74: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1D78: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA1D7C: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1D80: 41820228  beq 0x82ea1fa8
	if ctx.cr[0].eq {
	pc = 0x82EA1FA8; continue 'dispatch;
	}
	// 82EA1D84: 2F110001  cmpwi cr6, r17, 1
	ctx.cr[6].compare_i32(ctx.r[17].s32, 1, &mut ctx.xer);
	// 82EA1D88: 419A026C  beq cr6, 0x82ea1ff4
	if ctx.cr[6].eq {
	pc = 0x82EA1FF4; continue 'dispatch;
	}
	// 82EA1D8C: 2F130002  cmpwi cr6, r19, 2
	ctx.cr[6].compare_i32(ctx.r[19].s32, 2, &mut ctx.xer);
	// 82EA1D90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1D94: 419A0114  beq cr6, 0x82ea1ea8
	if ctx.cr[6].eq {
	pc = 0x82EA1EA8; continue 'dispatch;
	}
	// 82EA1D98: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1D9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1DA0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA1DA4: FB5D0020  std r26, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[26].u64 ) };
	// 82EA1DA8: 483A0BB5  bl 0x8324295c
	ctx.lr = 0x82EA1DAC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1DAC: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1DB0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1DB4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1DB8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1DBC: 40980018  bge cr6, 0x82ea1dd4
	if !ctx.cr[6].lt {
	pc = 0x82EA1DD4; continue 'dispatch;
	}
	// 82EA1DC0: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82EA1DC4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1DC8: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1DCC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1DD0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1DD4: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA1DD8: 48000859  bl 0x82ea2630
	ctx.lr = 0x82EA1DDC;
	sub_82EA2630(ctx, base);
	// 82EA1DDC: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1DE0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1DE4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1DE8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1DEC: 4098FED8  bge cr6, 0x82ea1cc4
	if !ctx.cr[6].lt {
	pc = 0x82EA1CC4; continue 'dispatch;
	}
	// 82EA1DF0: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1DF4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1DF8: 5528003E  slwi r8, r9, 0
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA1DFC: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1E00: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EA1E04: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1E08: 4BFFFEBC  b 0x82ea1cc4
	pc = 0x82EA1CC4; continue 'dispatch;
	// 82EA1E0C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA1E10: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EA1E14: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1E18: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EA1E1C: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA1E20: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EA1E24: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA1E28: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EA1E2C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EA1E30: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EA1E34: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EA1E38: 4200FFF0  bdnz 0x82ea1e28
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EA1E28; continue 'dispatch;
	}
	// 82EA1E3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA1E40: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1E44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1E48: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA1E4C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EA1E50: 409A0008  bne cr6, 0x82ea1e58
	if !ctx.cr[6].eq {
	pc = 0x82EA1E58; continue 'dispatch;
	}
	// 82EA1E54: 92DF0008  stw r22, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82EA1E58: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA1E5C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82EA1E60: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EA1E64: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1E68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA1E6C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EA1E70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1E74: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EA1E78: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA1E7C: 4E800421  bctrl
	ctx.lr = 0x82EA1E80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA1E80: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82EA1E84: 409A0010  bne cr6, 0x82ea1e94
	if !ctx.cr[6].eq {
	pc = 0x82EA1E94; continue 'dispatch;
	}
	// 82EA1E88: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA1E8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA1E90: 48000609  bl 0x82ea2498
	ctx.lr = 0x82EA1E94;
	sub_82EA2498(ctx, base);
	// 82EA1E94: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA1E98: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82EA1E9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA1EA0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA1EA4: 4BFFFEB8  b 0x82ea1d5c
	pc = 0x82EA1D5C; continue 'dispatch;
	// 82EA1EA8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1EAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA1EB0: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA1EB4: FB5D0020  std r26, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[26].u64 ) };
	// 82EA1EB8: 483A0AA5  bl 0x8324295c
	ctx.lr = 0x82EA1EBC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1EBC: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1EC0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1EC4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1EC8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1ECC: 40980018  bge cr6, 0x82ea1ee4
	if !ctx.cr[6].lt {
	pc = 0x82EA1EE4; continue 'dispatch;
	}
	// 82EA1ED0: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82EA1ED4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1ED8: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1EDC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1EE0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1EE4: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1EE8: 48000749  bl 0x82ea2630
	ctx.lr = 0x82EA1EEC;
	sub_82EA2630(ctx, base);
	// 82EA1EEC: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1EF0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1EF4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1EF8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1EFC: 4098FDC8  bge cr6, 0x82ea1cc4
	if !ctx.cr[6].lt {
	pc = 0x82EA1CC4; continue 'dispatch;
	}
	// 82EA1F00: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1F04: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1F08: 5528003E  slwi r8, r9, 0
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA1F0C: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1F10: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EA1F14: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1F18: 4BFFFDAC  b 0x82ea1cc4
	pc = 0x82EA1CC4; continue 'dispatch;
	// 82EA1F1C: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA1F20: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA1F24: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1F28: 41820040  beq 0x82ea1f68
	if ctx.cr[0].eq {
	pc = 0x82EA1F68; continue 'dispatch;
	}
	// 82EA1F2C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1F30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1F34: 419A0014  beq cr6, 0x82ea1f48
	if ctx.cr[6].eq {
	pc = 0x82EA1F48; continue 'dispatch;
	}
	// 82EA1F38: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1F3C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA1F40: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA1F44: 4800001C  b 0x82ea1f60
	pc = 0x82EA1F60; continue 'dispatch;
	// 82EA1F48: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1F4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA1F50: 419A0018  beq cr6, 0x82ea1f68
	if ctx.cr[6].eq {
	pc = 0x82EA1F68; continue 'dispatch;
	}
	// 82EA1F54: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA1F58: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA1F5C: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA1F60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA1F64: 480006FD  bl 0x82ea2660
	ctx.lr = 0x82EA1F68;
	sub_82EA2660(ctx, base);
	// 82EA1F68: FB5D0020  std r26, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[26].u64 ) };
	// 82EA1F6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1F70: 483A09ED  bl 0x8324295c
	ctx.lr = 0x82EA1F74;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1F74: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1F78: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1F7C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1F80: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1F84: 40980018  bge cr6, 0x82ea1f9c
	if !ctx.cr[6].lt {
	pc = 0x82EA1F9C; continue 'dispatch;
	}
	// 82EA1F88: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1F8C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1F90: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1F94: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1F98: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1F9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA1FA0: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82EA1FA4: 483061E8  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
	// 82EA1FA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA1FAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1FB0: 4BFFF2B1  bl 0x82ea1260
	ctx.lr = 0x82EA1FB4;
	sub_82EA1260(ctx, base);
	// 82EA1FB4: FB5D0020  std r26, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[26].u64 ) };
	// 82EA1FB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1FBC: 483A09A1  bl 0x8324295c
	ctx.lr = 0x82EA1FC0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA1FC0: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA1FC4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA1FC8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA1FCC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA1FD0: 40980018  bge cr6, 0x82ea1fe8
	if !ctx.cr[6].lt {
	pc = 0x82EA1FE8; continue 'dispatch;
	}
	// 82EA1FD4: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82EA1FD8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EA1FDC: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EA1FE0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EA1FE4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA1FE8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82EA1FEC: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82EA1FF0: 4830619C  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
	// 82EA1FF4: FB5D0020  std r26, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[26].u64 ) };
	// 82EA1FF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA1FFC: 483A0961  bl 0x8324295c
	ctx.lr = 0x82EA2000;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA2000: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA2004: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2008: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA200C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA2010: 40980020  bge cr6, 0x82ea2030
	if !ctx.cr[6].lt {
	pc = 0x82EA2030; continue 'dispatch;
	}
	// 82EA2014: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA2018: 3909E934  addi r8, r9, -0x16cc
	ctx.r[8].s64 = ctx.r[9].s64 + -5836;
	// 82EA201C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA2020: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EA2024: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EA2028: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA202C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA2030: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA2034: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82EA2038: 48306154  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2040 size=88
    let mut pc: u32 = 0x82EA2040;
    'dispatch: loop {
        match pc {
            0x82EA2040 => {
    //   block [0x82EA2040..0x82EA2098)
	// 82EA2040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA204C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2050: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA2054: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EA2058: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA205C: 419A000C  beq cr6, 0x82ea2068
	if ctx.cr[6].eq {
	pc = 0x82EA2068; continue 'dispatch;
	}
	// 82EA2060: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA2064: 480004E5  bl 0x82ea2548
	ctx.lr = 0x82EA2068;
	sub_82EA2548(ctx, base);
	// 82EA2068: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA206C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA2070: 419A000C  beq cr6, 0x82ea207c
	if ctx.cr[6].eq {
	pc = 0x82EA207C; continue 'dispatch;
	}
	// 82EA2074: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA2078: 4BFFF139  bl 0x82ea11b0
	ctx.lr = 0x82EA207C;
	sub_82EA11B0(ctx, base);
	// 82EA207C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EA2080: 480000F9  bl 0x82ea2178
	ctx.lr = 0x82EA2084;
	sub_82EA2178(ctx, base);
	// 82EA2084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA2088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA208C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA2090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA2094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2098 size=20
    let mut pc: u32 = 0x82EA2098;
    'dispatch: loop {
        match pc {
            0x82EA2098 => {
    //   block [0x82EA2098..0x82EA20AC)
	// 82EA2098: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82EA209C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EA20A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EA20A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EA20A8: 4BFFF710  b 0x82ea17b8
	sub_82EA17B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA20B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA20B0 size=196
    let mut pc: u32 = 0x82EA20B0;
    'dispatch: loop {
        match pc {
            0x82EA20B0 => {
    //   block [0x82EA20B0..0x82EA2174)
	// 82EA20B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA20B4: 483060B9  bl 0x831a816c
	ctx.lr = 0x82EA20B8;
	sub_831A8130(ctx, base);
	// 82EA20B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA20BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA20C0: 483A0D7D  bl 0x83242e3c
	ctx.lr = 0x82EA20C4;
	// extern call 0x83242E3C → crate::xboxkrnl::RtlTryEnterCriticalSection
	crate::xboxkrnl::RtlTryEnterCriticalSection(ctx, base);
	// 82EA20C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA20C8: 409A009C  bne cr6, 0x82ea2164
	if !ctx.cr[6].eq {
	pc = 0x82EA2164; continue 'dispatch;
	}
	// 82EA20CC: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA20D0: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82EA20D4: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA20D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA20DC: 419A0080  beq cr6, 0x82ea215c
	if ctx.cr[6].eq {
	pc = 0x82EA215C; continue 'dispatch;
	}
	// 82EA20E0: 3BE00018  li r31, 0x18
	ctx.r[31].s64 = 24;
	// 82EA20E4: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA20E8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA20EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA20F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA20F4: 40980020  bge cr6, 0x82ea2114
	if !ctx.cr[6].lt {
	pc = 0x82EA2114; continue 'dispatch;
	}
	// 82EA20F8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA20FC: 3909E904  addi r8, r9, -0x16fc
	ctx.r[8].s64 = ctx.r[9].s64 + -5884;
	// 82EA2100: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA2104: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EA2108: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EA210C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA2110: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA2114: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA2118: 483A0855  bl 0x8324296c
	ctx.lr = 0x82EA211C;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EA211C: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA2120: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2124: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA2128: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA212C: 40980038  bge cr6, 0x82ea2164
	if !ctx.cr[6].lt {
	pc = 0x82EA2164; continue 'dispatch;
	}
	// 82EA2130: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82EA2134: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82EA2138: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA213C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EA2140: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EA2144: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EA2148: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA214C: 48008155  bl 0x82eaa2a0
	ctx.lr = 0x82EA2150;
	sub_82EAA2A0(ctx, base);
	// 82EA2150: F87D0020  std r3, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[3].u64 ) };
	// 82EA2154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA2158: 48306064  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EA215C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA2160: 483A080D  bl 0x8324296c
	ctx.lr = 0x82EA2164;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EA2164: 4800813D  bl 0x82eaa2a0
	ctx.lr = 0x82EA2168;
	sub_82EAA2A0(ctx, base);
	// 82EA2168: F87D0020  std r3, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[3].u64 ) };
	// 82EA216C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA2170: 4830604C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2178 size=124
    let mut pc: u32 = 0x82EA2178;
    'dispatch: loop {
        match pc {
            0x82EA2178 => {
    //   block [0x82EA2178..0x82EA21F4)
	// 82EA2178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA217C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA2184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA2188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA218C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA2190: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA2198: 419A0044  beq cr6, 0x82ea21dc
	if ctx.cr[6].eq {
	pc = 0x82EA21DC; continue 'dispatch;
	}
	// 82EA219C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA21A0: 3BCB70B0  addi r30, r11, 0x70b0
	ctx.r[30].s64 = ctx.r[11].s64 + 28848;
	// 82EA21A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA21A8: 4BFFFF09  bl 0x82ea20b0
	ctx.lr = 0x82EA21AC;
	sub_82EA20B0(ctx, base);
	// 82EA21AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA21B0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA21B4: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82EA21B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA21BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA21C0: 419A000C  beq cr6, 0x82ea21cc
	if ctx.cr[6].eq {
	pc = 0x82EA21CC; continue 'dispatch;
	}
	// 82EA21C4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA21C8: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82EA21CC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA21D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA21D4: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA21D8: 483A0785  bl 0x8324295c
	ctx.lr = 0x82EA21DC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA21DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA21E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA21E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA21E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA21EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA21F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA21F8 size=144
    let mut pc: u32 = 0x82EA21F8;
    'dispatch: loop {
        match pc {
            0x82EA21F8 => {
    //   block [0x82EA21F8..0x82EA2288)
	// 82EA21F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA21FC: 48305F6D  bl 0x831a8168
	ctx.lr = 0x82EA2200;
	sub_831A8130(ctx, base);
	// 82EA2200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2204: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA2208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA220C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA2210: 3BDD0028  addi r30, r29, 0x28
	ctx.r[30].s64 = ctx.r[29].s64 + 40;
	// 82EA2214: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EA2218: 939D0028  stw r28, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82EA221C: 917D002C  stw r11, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EA2220: 917D0030  stw r11, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EA2224: 419A0044  beq cr6, 0x82ea2268
	if ctx.cr[6].eq {
	pc = 0x82EA2268; continue 'dispatch;
	}
	// 82EA2228: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA222C: 3BEB70B0  addi r31, r11, 0x70b0
	ctx.r[31].s64 = ctx.r[11].s64 + 28848;
	// 82EA2230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA2234: 4BFFFE7D  bl 0x82ea20b0
	ctx.lr = 0x82EA2238;
	sub_82EA20B0(ctx, base);
	// 82EA2238: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EA223C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA2240: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA2244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA2248: 419A0008  beq cr6, 0x82ea2250
	if ctx.cr[6].eq {
	pc = 0x82EA2250; continue 'dispatch;
	}
	// 82EA224C: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82EA2250: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA2254: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EA2258: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82EA225C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA2260: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA2264: 483A06F9  bl 0x8324295c
	ctx.lr = 0x82EA2268;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA2268: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA226C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA2270: 483A0BDD  bl 0x83242e4c
	ctx.lr = 0x82EA2274;
	// extern call 0x83242E4C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82EA2274: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA2278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA227C: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA2280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA2284: 48305F34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2288 size=240
    let mut pc: u32 = 0x82EA2288;
    'dispatch: loop {
        match pc {
            0x82EA2288 => {
    //   block [0x82EA2288..0x82EA2378)
	// 82EA2288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA228C: 48305ED1  bl 0x831a815c
	ctx.lr = 0x82EA2290;
	sub_831A8130(ctx, base);
	// 82EA2290: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA2298: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EA229C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA22A0: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82EA22A4: 409800CC  bge cr6, 0x82ea2370
	if !ctx.cr[6].lt {
	pc = 0x82EA2370; continue 'dispatch;
	}
	// 82EA22A8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA22AC: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82EA22B0: 41980008  blt cr6, 0x82ea22b8
	if ctx.cr[6].lt {
	pc = 0x82EA22B8; continue 'dispatch;
	}
	// 82EA22B4: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82EA22B8: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA22BC: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 82EA22C0: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EA22C4: 57643830  slwi r4, r27, 7
	ctx.r[4].u32 = ctx.r[27].u32.wrapping_shl(7);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EA22C8: 7C79D02E  lwzx r3, r25, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82EA22CC: 4BFFE465  bl 0x82ea0730
	ctx.lr = 0x82EA22D0;
	sub_82EA0730(ctx, base);
	// 82EA22D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA22D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA22D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA22DC: 419A006C  beq cr6, 0x82ea2348
	if ctx.cr[6].eq {
	pc = 0x82EA2348; continue 'dispatch;
	}
	// 82EA22E0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA22E4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA22E8: 419A0050  beq cr6, 0x82ea2338
	if ctx.cr[6].eq {
	pc = 0x82EA2338; continue 'dispatch;
	}
	// 82EA22EC: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA22F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA22F4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA22F8: 41990030  bgt cr6, 0x82ea2328
	if ctx.cr[6].gt {
	pc = 0x82EA2328; continue 'dispatch;
	}
	// 82EA22FC: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2300: 55693830  slwi r9, r11, 7
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA2304: 7CEB4050  subf r7, r11, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82EA2308: 7C895214  add r4, r9, r10
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EA230C: 54FE3830  slwi r30, r7, 7
	ctx.r[30].u32 = ctx.r[7].u32.wrapping_shl(7);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EA2310: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA2314: 48008435  bl 0x82eaa748
	ctx.lr = 0x82EA2318;
	sub_82EAA748(ctx, base);
	// 82EA2318: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA231C: 57A53830  slwi r5, r29, 7
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA2320: 7C7EE214  add r3, r30, r28
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82EA2324: 48000010  b 0x82ea2334
	pc = 0x82EA2334; continue 'dispatch;
	// 82EA2328: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA232C: 55253830  slwi r5, r9, 7
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA2330: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA2334: 48008415  bl 0x82eaa748
	ctx.lr = 0x82EA2338;
	sub_82EAA748(ctx, base);
	// 82EA2338: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA233C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA2340: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EA2344: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA2348: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA234C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA2350: 419A0018  beq cr6, 0x82ea2368
	if ctx.cr[6].eq {
	pc = 0x82EA2368; continue 'dispatch;
	}
	// 82EA2354: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA2358: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA235C: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA2360: 7C79D02E  lwzx r3, r25, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82EA2364: 4BFFE44D  bl 0x82ea07b0
	ctx.lr = 0x82EA2368;
	sub_82EA07B0(ctx, base);
	// 82EA2368: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EA236C: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82EA2370: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA2374: 48305E38  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2378 size=104
    let mut pc: u32 = 0x82EA2378;
    'dispatch: loop {
        match pc {
            0x82EA2378 => {
    //   block [0x82EA2378..0x82EA23E0)
	// 82EA2378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA237C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA2384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA2388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA238C: 3BE30050  addi r31, r3, 0x50
	ctx.r[31].s64 = ctx.r[3].s64 + 80;
	// 82EA2390: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82EA2394: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 82EA2398: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA239C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA23A0: 419A0020  beq cr6, 0x82ea23c0
	if ctx.cr[6].eq {
	pc = 0x82EA23C0; continue 'dispatch;
	}
	// 82EA23A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA23A8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA23AC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA23B0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA23B4: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA23B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA23BC: 4BFFE3F5  bl 0x82ea07b0
	ctx.lr = 0x82EA23C0;
	sub_82EA07B0(ctx, base);
	// 82EA23C0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA23C4: 4080FFD0  bge 0x82ea2394
	if !ctx.cr[0].lt {
	pc = 0x82EA2394; continue 'dispatch;
	}
	// 82EA23C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA23CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA23D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA23D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA23D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA23DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA23E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA23E0 size=184
    let mut pc: u32 = 0x82EA23E0;
    'dispatch: loop {
        match pc {
            0x82EA23E0 => {
    //   block [0x82EA23E0..0x82EA2498)
	// 82EA23E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA23E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA23E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA23EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA23F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA23F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA23F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA23FC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA2400: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2404: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA2408: 41980018  blt cr6, 0x82ea2420
	if ctx.cr[6].lt {
	pc = 0x82EA2420; continue 'dispatch;
	}
	// 82EA240C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA2410: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EA2414: 419A0008  beq cr6, 0x82ea241c
	if ctx.cr[6].eq {
	pc = 0x82EA241C; continue 'dispatch;
	}
	// 82EA2418: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EA241C: 4BFFFE6D  bl 0x82ea2288
	ctx.lr = 0x82EA2420;
	sub_82EA2288(ctx, base);
	// 82EA2420: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2424: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA2428: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA242C: 409A000C  bne cr6, 0x82ea2438
	if !ctx.cr[6].eq {
	pc = 0x82EA2438; continue 'dispatch;
	}
	// 82EA2430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA2434: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA2438: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA243C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EA2440: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2444: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EA2448: 554A3830  slwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA244C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82EA2450: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA2454: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EA2458: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EA245C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EA2460: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EA2464: 4200FFF0  bdnz 0x82ea2454
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EA2454; continue 'dispatch;
	}
	// 82EA2468: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA246C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA2470: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA2474: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EA2478: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EA247C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82EA2480: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA2484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA248C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA2490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA2494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2498 size=176
    let mut pc: u32 = 0x82EA2498;
    'dispatch: loop {
        match pc {
            0x82EA2498 => {
    //   block [0x82EA2498..0x82EA2548)
	// 82EA2498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA249C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA24A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA24A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA24A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA24AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA24B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA24B4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA24B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA24BC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA24C0: 41980018  blt cr6, 0x82ea24d8
	if ctx.cr[6].lt {
	pc = 0x82EA24D8; continue 'dispatch;
	}
	// 82EA24C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA24C8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EA24CC: 419A0008  beq cr6, 0x82ea24d4
	if ctx.cr[6].eq {
	pc = 0x82EA24D4; continue 'dispatch;
	}
	// 82EA24D0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EA24D4: 4BFFFDB5  bl 0x82ea2288
	ctx.lr = 0x82EA24D8;
	sub_82EA2288(ctx, base);
	// 82EA24D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA24DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA24E0: 409A000C  bne cr6, 0x82ea24ec
	if !ctx.cr[6].eq {
	pc = 0x82EA24EC; continue 'dispatch;
	}
	// 82EA24E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA24E8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA24EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA24F0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82EA24F4: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA24F8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EA24FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EA2500: 55683830  slwi r8, r11, 7
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA2504: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA2508: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EA250C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA2510: E92A0000  ld r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82EA2514: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EA2518: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EA251C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EA2520: 4200FFF0  bdnz 0x82ea2510
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EA2510; continue 'dispatch;
	}
	// 82EA2524: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA2528: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA252C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EA2530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA2534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA253C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA2540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA2544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2548 size=160
    let mut pc: u32 = 0x82EA2548;
    'dispatch: loop {
        match pc {
            0x82EA2548 => {
    //   block [0x82EA2548..0x82EA25E8)
	// 82EA2548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA254C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA2554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA2558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA255C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA2560: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA2564: 4BFFFE15  bl 0x82ea2378
	ctx.lr = 0x82EA2568;
	sub_82EA2378(ctx, base);
	// 82EA2568: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA256C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA2570: 419A005C  beq cr6, 0x82ea25cc
	if ctx.cr[6].eq {
	pc = 0x82EA25CC; continue 'dispatch;
	}
	// 82EA2574: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA2578: 419A0054  beq cr6, 0x82ea25cc
	if ctx.cr[6].eq {
	pc = 0x82EA25CC; continue 'dispatch;
	}
	// 82EA257C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2580: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA2584: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA2588: 812B006C  lwz r9, 0x6c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EA258C: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA2590: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EA2594: 4198001C  blt cr6, 0x82ea25b0
	if ctx.cr[6].lt {
	pc = 0x82EA25B0; continue 'dispatch;
	}
	// 82EA2598: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82EA259C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA25A0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82EA25A4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA25A8: 4BFFDAB9  bl 0x82ea0060
	ctx.lr = 0x82EA25AC;
	sub_82EA0060(ctx, base);
	// 82EA25AC: 48000020  b 0x82ea25cc
	pc = 0x82EA25CC; continue 'dispatch;
	// 82EA25B0: 812B006C  lwz r9, 0x6c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EA25B4: 394B0068  addi r10, r11, 0x68
	ctx.r[10].s64 = ctx.r[11].s64 + 104;
	// 82EA25B8: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EA25BC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EA25C0: 912B006C  stw r9, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82EA25C4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA25C8: 93EB0068  stw r31, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82EA25CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA25D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA25D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA25D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA25DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA25E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA25E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA25E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA25E8 size=60
    let mut pc: u32 = 0x82EA25E8;
    'dispatch: loop {
        match pc {
            0x82EA25E8 => {
    //   block [0x82EA25E8..0x82EA2624)
	// 82EA25E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA25EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA25F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA25F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA25F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA25FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EA2600: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA2604: 48322BCD  bl 0x831c51d0
	ctx.lr = 0x82EA2608;
	sub_831C51D0(ctx, base);
	// 82EA2608: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EA260C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA2610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA2614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA261C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA2620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2628 size=8
    let mut pc: u32 = 0x82EA2628;
    'dispatch: loop {
        match pc {
            0x82EA2628 => {
    //   block [0x82EA2628..0x82EA2630)
	// 82EA2628: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA262C: 4BD2A3F4  b 0x82bcca20
	sub_82BCCA20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2630 size=12
    let mut pc: u32 = 0x82EA2630;
    'dispatch: loop {
        match pc {
            0x82EA2630 => {
    //   block [0x82EA2630..0x82EA263C)
	// 82EA2630: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2634: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EA2638: 4BD2B6C8  b 0x82bcdd00
	sub_82BCDD00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2640 size=12
    let mut pc: u32 = 0x82EA2640;
    'dispatch: loop {
        match pc {
            0x82EA2640 => {
    //   block [0x82EA2640..0x82EA264C)
	// 82EA2640: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EA2648: 48322C20  b 0x831c5268
	sub_831C5268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2650 size=12
    let mut pc: u32 = 0x82EA2650;
    'dispatch: loop {
        match pc {
            0x82EA2650 => {
    //   block [0x82EA2650..0x82EA265C)
	// 82EA2650: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2654: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EA2658: 4BD2B6A8  b 0x82bcdd00
	sub_82BCDD00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2660 size=12
    let mut pc: u32 = 0x82EA2660;
    'dispatch: loop {
        match pc {
            0x82EA2660 => {
    //   block [0x82EA2660..0x82EA266C)
	// 82EA2660: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EA2668: 48322C00  b 0x831c5268
	sub_831C5268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2670 size=4
    let mut pc: u32 = 0x82EA2670;
    'dispatch: loop {
        match pc {
            0x82EA2670 => {
    //   block [0x82EA2670..0x82EA2674)
	// 82EA2670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2678 size=16
    let mut pc: u32 = 0x82EA2678;
    'dispatch: loop {
        match pc {
            0x82EA2678 => {
    //   block [0x82EA2678..0x82EA2688)
	// 82EA2678: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA267C: 394BE94C  addi r10, r11, -0x16b4
	ctx.r[10].s64 = ctx.r[11].s64 + -5812;
	// 82EA2680: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA2684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2688 size=4
    let mut pc: u32 = 0x82EA2688;
    'dispatch: loop {
        match pc {
            0x82EA2688 => {
    //   block [0x82EA2688..0x82EA268C)
	// 82EA2688: 4830EFE8  b 0x831b1670
	sub_831B1670(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2690 size=4
    let mut pc: u32 = 0x82EA2690;
    'dispatch: loop {
        match pc {
            0x82EA2690 => {
    //   block [0x82EA2690..0x82EA2694)
	// 82EA2690: 4830EFC8  b 0x831b1658
	sub_831B1658(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2698 size=136
    let mut pc: u32 = 0x82EA2698;
    'dispatch: loop {
        match pc {
            0x82EA2698 => {
    //   block [0x82EA2698..0x82EA2720)
	// 82EA2698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA269C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA26A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA26A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA26A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA26AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA26B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA26B4: 419A0010  beq cr6, 0x82ea26c4
	if ctx.cr[6].eq {
	pc = 0x82EA26C4; continue 'dispatch;
	}
	// 82EA26B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA26BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA26C0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EA26C4: 3FC08338  lis r30, -0x7cc8
	ctx.r[30].s64 = -2093481984;
	// 82EA26C8: 807E6A30  lwz r3, 0x6a30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EA26CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA26D0: 419A0034  beq cr6, 0x82ea2704
	if ctx.cr[6].eq {
	pc = 0x82EA2704; continue 'dispatch;
	}
	// 82EA26D4: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA26D8: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82EA26DC: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82EA26E0: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA26E4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EA26E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA26EC: 409A0018  bne cr6, 0x82ea2704
	if !ctx.cr[6].eq {
	pc = 0x82EA2704; continue 'dispatch;
	}
	// 82EA26F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA26F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA26F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA26FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA2700: 4E800421  bctrl
	ctx.lr = 0x82EA2704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2704: 93FE6A30  stw r31, 0x6a30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(27184 as u32), ctx.r[31].u32 ) };
	// 82EA2708: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA270C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA2714: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA2718: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA271C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2720 size=24
    let mut pc: u32 = 0x82EA2720;
    'dispatch: loop {
        match pc {
            0x82EA2720 => {
    //   block [0x82EA2720..0x82EA2738)
	// 82EA2720: 2F040010  cmpwi cr6, r4, 0x10
	ctx.cr[6].compare_i32(ctx.r[4].s32, 16, &mut ctx.xer);
	// 82EA2724: 40990014  ble cr6, 0x82ea2738
	if !ctx.cr[6].gt {
		sub_82EA2738(ctx, base);
		return;
	}
	// 82EA2728: 3964000F  addi r11, r4, 0xf
	ctx.r[11].s64 = ctx.r[4].s64 + 15;
	// 82EA272C: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA2730: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EA2734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2738 size=8
    let mut pc: u32 = 0x82EA2738;
    'dispatch: loop {
        match pc {
            0x82EA2738 => {
    //   block [0x82EA2738..0x82EA2740)
	// 82EA2738: 38640008  addi r3, r4, 8
	ctx.r[3].s64 = ctx.r[4].s64 + 8;
	// 82EA273C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2740 size=12
    let mut pc: u32 = 0x82EA2740;
    'dispatch: loop {
        match pc {
            0x82EA2740 => {
    //   block [0x82EA2740..0x82EA274C)
	// 82EA2740: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA2744: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA2748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2750 size=48
    let mut pc: u32 = 0x82EA2750;
    'dispatch: loop {
        match pc {
            0x82EA2750 => {
    //   block [0x82EA2750..0x82EA2780)
	// 82EA2750: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA2754: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82EA2758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA275C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EA2760: 38EAE94C  addi r7, r10, -0x16b4
	ctx.r[7].s64 = ctx.r[10].s64 + -5812;
	// 82EA2764: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA2768: 6126FFFF  ori r6, r9, 0xffff
	ctx.r[6].u64 = ctx.r[9].u64 | 65535;
	// 82EA276C: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82EA2770: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EA2774: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA2778: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82EA277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2780 size=84
    let mut pc: u32 = 0x82EA2780;
    'dispatch: loop {
        match pc {
            0x82EA2780 => {
    //   block [0x82EA2780..0x82EA27D4)
	// 82EA2780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA278C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA2794: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA2798: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82EA279C: 392BE94C  addi r9, r11, -0x16b4
	ctx.r[9].s64 = ctx.r[11].s64 + -5812;
	// 82EA27A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA27A4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA27A8: 419A0018  beq cr6, 0x82ea27c0
	if ctx.cr[6].eq {
	pc = 0x82EA27C0; continue 'dispatch;
	}
	// 82EA27AC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82EA27B0: 814BBDC0  lwz r10, -0x4240(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EA27B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA27B8: 4E800421  bctrl
	ctx.lr = 0x82EA27BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA27BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA27C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA27C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA27C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA27CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA27D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA27D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA27D8 size=4
    let mut pc: u32 = 0x82EA27D8;
    'dispatch: loop {
        match pc {
            0x82EA27D8 => {
    //   block [0x82EA27D8..0x82EA27DC)
	// 82EA27D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA27E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA27E0 size=40
    let mut pc: u32 = 0x82EA27E0;
    'dispatch: loop {
        match pc {
            0x82EA27E0 => {
    //   block [0x82EA27E0..0x82EA2808)
	// 82EA27E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA27E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA27E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA27EC: 4BFFDABD  bl 0x82ea02a8
	ctx.lr = 0x82EA27F0;
	sub_82EA02A8(ctx, base);
	// 82EA27F0: 48003F81  bl 0x82ea6770
	ctx.lr = 0x82EA27F4;
	sub_82EA6770(ctx, base);
	// 82EA27F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA27F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA27FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA2804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2808 size=88
    let mut pc: u32 = 0x82EA2808;
    'dispatch: loop {
        match pc {
            0x82EA2808 => {
    //   block [0x82EA2808..0x82EA2860)
	// 82EA2808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA280C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA2814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2818: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA281C: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82EA2820: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA2824: 48003D8D  bl 0x82ea65b0
	ctx.lr = 0x82EA2828;
	sub_82EA65B0(ctx, base);
	// 82EA2828: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA282C: 7C7F502E  lwzx r3, r31, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA2830: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2834: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA2838: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA283C: 4E800421  bctrl
	ctx.lr = 0x82EA2840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2840: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA2844: 4BFFDA65  bl 0x82ea02a8
	ctx.lr = 0x82EA2848;
	sub_82EA02A8(ctx, base);
	// 82EA2848: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA284C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA2850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA2858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA285C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2860 size=16
    let mut pc: u32 = 0x82EA2860;
    'dispatch: loop {
        match pc {
            0x82EA2860 => {
    //   block [0x82EA2860..0x82EA2870)
	// 82EA2860: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA2864: 896B6A38  lbz r11, 0x6a38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(27192 as u32) ) } as u64;
	// 82EA2868: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA286C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2870 size=80
    let mut pc: u32 = 0x82EA2870;
    'dispatch: loop {
        match pc {
            0x82EA2870 => {
    //   block [0x82EA2870..0x82EA28C0)
	// 82EA2870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA287C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2880: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA2884: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82EA2888: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EA288C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA2890: 4BFFDEA1  bl 0x82ea0730
	ctx.lr = 0x82EA2894;
	sub_82EA0730(ctx, base);
	// 82EA2894: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA2898: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82EA289C: 38E9E9E4  addi r7, r9, -0x161c
	ctx.r[7].s64 = ctx.r[9].s64 + -5660;
	// 82EA28A0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EA28A4: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82EA28A8: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EA28AC: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82EA28B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA28B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA28B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA28BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA28C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA28C0 size=364
    let mut pc: u32 = 0x82EA28C0;
    'dispatch: loop {
        match pc {
            0x82EA28C0 => {
    //   block [0x82EA28C0..0x82EA2A2C)
	// 82EA28C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA28C4: 483058A5  bl 0x831a8168
	ctx.lr = 0x82EA28C8;
	sub_831A8130(ctx, base);
	// 82EA28C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA28CC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA28D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EA28D4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EA28D8: 3BCB70E8  addi r30, r11, 0x70e8
	ctx.r[30].s64 = ctx.r[11].s64 + 28904;
	// 82EA28DC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82EA28E0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82EA28E4: 83EB70E8  lwz r31, 0x70e8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28904 as u32) ) } as u64;
	// 82EA28E8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82EA28EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA28F0: 419A00B8  beq cr6, 0x82ea29a8
	if ctx.cr[6].eq {
	pc = 0x82EA29A8; continue 'dispatch;
	}
	// 82EA28F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA28F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA28FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA2900: 409A0020  bne cr6, 0x82ea2920
	if !ctx.cr[6].eq {
	pc = 0x82EA2920; continue 'dispatch;
	}
	// 82EA2904: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA290C: 4E800421  bctrl
	ctx.lr = 0x82EA2910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2910: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA2914: 419A00C8  beq cr6, 0x82ea29dc
	if ctx.cr[6].eq {
	pc = 0x82EA29DC; continue 'dispatch;
	}
	// 82EA2918: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA291C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EA2920: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EA2924: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2928: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA292C: 409AFFC8  bne cr6, 0x82ea28f4
	if !ctx.cr[6].eq {
	pc = 0x82EA28F4; continue 'dispatch;
	}
	// 82EA2930: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA2934: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA2938: 419A0070  beq cr6, 0x82ea29a8
	if ctx.cr[6].eq {
	pc = 0x82EA29A8; continue 'dispatch;
	}
	// 82EA293C: 378BFFFF  addic. r28, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EA2940: 4180005C  blt 0x82ea299c
	if ctx.cr[0].lt {
	pc = 0x82EA299C; continue 'dispatch;
	}
	// 82EA2944: 579D103A  slwi r29, r28, 2
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EA2948: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA294C: 7FFD582E  lwzx r31, r29, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA2950: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2954: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA2958: 4E800421  bctrl
	ctx.lr = 0x82EA295C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA295C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA2960: 419A0030  beq cr6, 0x82ea2990
	if ctx.cr[6].eq {
	pc = 0x82EA2990; continue 'dispatch;
	}
	// 82EA2964: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA2968: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EA296C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EA2970: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EA2974: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA2978: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA297C: 3969FFFF  addi r11, r9, -1
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	// 82EA2980: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA2984: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EA2988: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA298C: 7CFD512E  stwx r7, r29, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u32) };
	// 82EA2990: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EA2994: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 82EA2998: 4080FFB0  bge 0x82ea2948
	if !ctx.cr[0].lt {
	pc = 0x82EA2948; continue 'dispatch;
	}
	// 82EA299C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA29A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA29A4: 409AFF98  bne cr6, 0x82ea293c
	if !ctx.cr[6].eq {
	pc = 0x82EA293C; continue 'dispatch;
	}
	// 82EA29A8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA29AC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA29B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA29B4: 409A0020  bne cr6, 0x82ea29d4
	if !ctx.cr[6].eq {
	pc = 0x82EA29D4; continue 'dispatch;
	}
	// 82EA29B8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA29BC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA29C0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA29C4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA29C8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA29CC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA29D0: 4BFFDDE1  bl 0x82ea07b0
	ctx.lr = 0x82EA29D4;
	sub_82EA07B0(ctx, base);
	// 82EA29D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA29D8: 483057E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA29DC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA29E0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA29E4: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA29E8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EA29EC: 409A0010  bne cr6, 0x82ea29fc
	if !ctx.cr[6].eq {
	pc = 0x82EA29FC; continue 'dispatch;
	}
	// 82EA29F0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EA29F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA29F8: 48003E89  bl 0x82ea6880
	ctx.lr = 0x82EA29FC;
	sub_82EA6880(ctx, base);
	// 82EA29FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA2A00: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA2A04: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA2A08: 7FE9512E  stwx r31, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82EA2A0C: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA2A10: 38E80001  addi r7, r8, 1
	ctx.r[7].s64 = ctx.r[8].s64 + 1;
	// 82EA2A14: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82EA2A18: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2A1C: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2A20: 93A60004  stw r29, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EA2A24: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EA2A28: 4BFFFF00  b 0x82ea2928
	pc = 0x82EA2928; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2A30 size=292
    let mut pc: u32 = 0x82EA2A30;
    'dispatch: loop {
        match pc {
            0x82EA2A30 => {
    //   block [0x82EA2A30..0x82EA2B54)
	// 82EA2A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2A34: 48305739  bl 0x831a816c
	ctx.lr = 0x82EA2A38;
	sub_831A8130(ctx, base);
	// 82EA2A38: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2A3C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82EA2A40: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EA2A44: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EA2A48: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 82EA2A4C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EA2A50: 83EA70E8  lwz r31, 0x70e8(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28904 as u32) ) } as u64;
	// 82EA2A54: 61270080  ori r7, r9, 0x80
	ctx.r[7].u64 = ctx.r[9].u64 | 128;
	// 82EA2A58: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82EA2A5C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EA2A60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA2A64: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82EA2A68: 419A0048  beq cr6, 0x82ea2ab0
	if ctx.cr[6].eq {
	pc = 0x82EA2AB0; continue 'dispatch;
	}
	// 82EA2A6C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA2A70: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA2A74: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EA2A78: 409A0014  bne cr6, 0x82ea2a8c
	if !ctx.cr[6].eq {
	pc = 0x82EA2A8C; continue 'dispatch;
	}
	// 82EA2A7C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EA2A80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA2A84: 48003DFD  bl 0x82ea6880
	ctx.lr = 0x82EA2A88;
	sub_82EA6880(ctx, base);
	// 82EA2A88: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA2A8C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA2A90: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA2A94: 7FE9512E  stwx r31, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82EA2A98: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA2A9C: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 82EA2AA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EA2AA4: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2AA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA2AAC: 409AFFC0  bne cr6, 0x82ea2a6c
	if !ctx.cr[6].eq {
	pc = 0x82EA2A6C; continue 'dispatch;
	}
	// 82EA2AB0: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA2AB4: 4180006C  blt 0x82ea2b20
	if ctx.cr[0].lt {
	pc = 0x82EA2B20; continue 'dispatch;
	}
	// 82EA2AB8: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EA2ABC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA2AC0: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA2AC4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA2AC8: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2ACC: A1030004  lhz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2AD0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EA2AD4: 419A0030  beq cr6, 0x82ea2b04
	if ctx.cr[6].eq {
	pc = 0x82EA2B04; continue 'dispatch;
	}
	// 82EA2AD8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA2ADC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EA2AE0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EA2AE4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA2AE8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA2AEC: 409A0018  bne cr6, 0x82ea2b04
	if !ctx.cr[6].eq {
	pc = 0x82EA2B04; continue 'dispatch;
	}
	// 82EA2AF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2AF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA2AF8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2AFC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA2B00: 4E800421  bctrl
	ctx.lr = 0x82EA2B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2B04: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA2B08: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA2B0C: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA2B10: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82EA2B14: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA2B18: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EA2B1C: 4080FFA0  bge 0x82ea2abc
	if !ctx.cr[0].lt {
	pc = 0x82EA2ABC; continue 'dispatch;
	}
	// 82EA2B20: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA2B24: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA2B28: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA2B2C: 409A0020  bne cr6, 0x82ea2b4c
	if !ctx.cr[6].eq {
	pc = 0x82EA2B4C; continue 'dispatch;
	}
	// 82EA2B30: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2B34: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA2B38: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA2B3C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA2B40: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA2B44: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA2B48: 4BFFDC69  bl 0x82ea07b0
	ctx.lr = 0x82EA2B4C;
	sub_82EA07B0(ctx, base);
	// 82EA2B4C: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82EA2B50: 4830566C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2B58 size=312
    let mut pc: u32 = 0x82EA2B58;
    'dispatch: loop {
        match pc {
            0x82EA2B58 => {
    //   block [0x82EA2B58..0x82EA2C90)
	// 82EA2B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2B5C: 48305609  bl 0x831a8164
	ctx.lr = 0x82EA2B60;
	sub_831A8130(ctx, base);
	// 82EA2B60: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2B64: 3FA08338  lis r29, -0x7cc8
	ctx.r[29].s64 = -2093481984;
	// 82EA2B68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA2B6C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EA2B70: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EA2B74: 897D6A38  lbz r11, 0x6a38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(27192 as u32) ) } as u64;
	// 82EA2B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA2B7C: 409A00A8  bne cr6, 0x82ea2c24
	if !ctx.cr[6].eq {
	pc = 0x82EA2C24; continue 'dispatch;
	}
	// 82EA2B80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA2B84: 419A00DC  beq cr6, 0x82ea2c60
	if ctx.cr[6].eq {
	pc = 0x82EA2C60; continue 'dispatch;
	}
	// 82EA2B88: 4BFFFB11  bl 0x82ea2698
	ctx.lr = 0x82EA2B8C;
	sub_82EA2698(ctx, base);
	// 82EA2B8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA2B90: 419A00A0  beq cr6, 0x82ea2c30
	if ctx.cr[6].eq {
	pc = 0x82EA2C30; continue 'dispatch;
	}
	// 82EA2B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA2B98: 4BFFD711  bl 0x82ea02a8
	ctx.lr = 0x82EA2B9C;
	sub_82EA02A8(ctx, base);
	// 82EA2B9C: 48003BD5  bl 0x82ea6770
	ctx.lr = 0x82EA2BA0;
	sub_82EA6770(ctx, base);
	// 82EA2BA0: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2BA4: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82EA2BA8: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82EA2BAC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EA2BB0: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA2BB4: 4BFFDB7D  bl 0x82ea0730
	ctx.lr = 0x82EA2BB8;
	sub_82EA0730(ctx, base);
	// 82EA2BB8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA2BBC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82EA2BC0: 392BE9C8  addi r9, r11, -0x1638
	ctx.r[9].s64 = ctx.r[11].s64 + -5688;
	// 82EA2BC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EA2BC8: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82EA2BCC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA2BD0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82EA2BD4: 48000275  bl 0x82ea2e48
	ctx.lr = 0x82EA2BD8;
	sub_82EA2E48(ctx, base);
	// 82EA2BD8: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82EA2BDC: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82EA2BE0: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA2BE4: 4BFFDB4D  bl 0x82ea0730
	ctx.lr = 0x82EA2BE8;
	sub_82EA0730(ctx, base);
	// 82EA2BE8: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 82EA2BEC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EA2BF0: B0E30004  sth r7, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82EA2BF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA2BF8: 48000551  bl 0x82ea3148
	ctx.lr = 0x82EA2BFC;
	sub_82EA3148(ctx, base);
	// 82EA2BFC: 480001C5  bl 0x82ea2dc0
	ctx.lr = 0x82EA2C00;
	sub_82EA2DC0(ctx, base);
	// 82EA2C00: 4BFFFCC1  bl 0x82ea28c0
	ctx.lr = 0x82EA2C04;
	sub_82EA28C0(ctx, base);
	// 82EA2C04: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 82EA2C08: 80666A34  lwz r3, 0x6a34(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(27188 as u32) ) } as u64;
	// 82EA2C0C: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2C10: 8085000C  lwz r4, 0xc(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA2C14: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82EA2C18: 4E800421  bctrl
	ctx.lr = 0x82EA2C1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2C1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA2C20: 997D6A38  stb r11, 0x6a38(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(27192 as u32), ctx.r[11].u8 ) };
	// 82EA2C24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA2C28: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82EA2C2C: 48305588  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EA2C30: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA2C34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA2C38: 388BEB48  addi r4, r11, -0x14b8
	ctx.r[4].s64 = ctx.r[11].s64 + -5304;
	// 82EA2C3C: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 82EA2C40: 483058D1  bl 0x831a8510
	ctx.lr = 0x82EA2C44;
	sub_831A8510(ctx, base);
	// 82EA2C44: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA2C48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA2C4C: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 82EA2C50: 4E800421  bctrl
	ctx.lr = 0x82EA2C54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2C54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA2C58: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82EA2C5C: 48305558  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EA2C60: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA2C64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA2C68: 388BEA68  addi r4, r11, -0x1598
	ctx.r[4].s64 = ctx.r[11].s64 + -5528;
	// 82EA2C6C: 38A000DC  li r5, 0xdc
	ctx.r[5].s64 = 220;
	// 82EA2C70: 483058A1  bl 0x831a8510
	ctx.lr = 0x82EA2C74;
	sub_831A8510(ctx, base);
	// 82EA2C74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA2C78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA2C7C: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 82EA2C80: 4E800421  bctrl
	ctx.lr = 0x82EA2C84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2C84: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA2C88: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82EA2C8C: 48305528  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2C90 size=300
    let mut pc: u32 = 0x82EA2C90;
    'dispatch: loop {
        match pc {
            0x82EA2C90 => {
    //   block [0x82EA2C90..0x82EA2DBC)
	// 82EA2C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2C94: 483054D5  bl 0x831a8168
	ctx.lr = 0x82EA2C98;
	sub_831A8130(ctx, base);
	// 82EA2C98: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2C9C: 3F808338  lis r28, -0x7cc8
	ctx.r[28].s64 = -2093481984;
	// 82EA2CA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EA2CA4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82EA2CA8: 897C6A38  lbz r11, 0x6a38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(27192 as u32) ) } as u64;
	// 82EA2CAC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82EA2CB0: 409A0100  bne cr6, 0x82ea2db0
	if !ctx.cr[6].eq {
	pc = 0x82EA2DB0; continue 'dispatch;
	}
	// 82EA2CB4: 4BFFFD7D  bl 0x82ea2a30
	ctx.lr = 0x82EA2CB8;
	sub_82EA2A30(ctx, base);
	// 82EA2CB8: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82EA2CBC: 897F70F0  lbz r11, 0x70f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28912 as u32) ) } as u64;
	// 82EA2CC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA2CC4: 419A0028  beq cr6, 0x82ea2cec
	if ctx.cr[6].eq {
	pc = 0x82EA2CEC; continue 'dispatch;
	}
	// 82EA2CC8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82EA2CCC: 814BC580  lwz r10, -0x3a80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14976 as u32) ) } as u64;
	// 82EA2CD0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA2CD4: 419A0018  beq cr6, 0x82ea2cec
	if ctx.cr[6].eq {
	pc = 0x82EA2CEC; continue 'dispatch;
	}
	// 82EA2CD8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA2CDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA2CE0: 4E800421  bctrl
	ctx.lr = 0x82EA2CE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2CE4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EA2CE8: 997F70F0  stb r11, 0x70f0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28912 as u32), ctx.r[11].u8 ) };
	// 82EA2CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA2CF0: 480000D1  bl 0x82ea2dc0
	ctx.lr = 0x82EA2CF4;
	sub_82EA2DC0(ctx, base);
	// 82EA2CF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA2CF8: 48000151  bl 0x82ea2e48
	ctx.lr = 0x82EA2CFC;
	sub_82EA2E48(ctx, base);
	// 82EA2CFC: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2D00: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82EA2D04: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA2D08: 480038A9  bl 0x82ea65b0
	ctx.lr = 0x82EA2D0C;
	sub_82EA65B0(ctx, base);
	// 82EA2D0C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA2D10: 7C7F502E  lwzx r3, r31, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA2D14: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2D18: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA2D1C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA2D20: 4E800421  bctrl
	ctx.lr = 0x82EA2D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2D24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA2D28: 4BFFD581  bl 0x82ea02a8
	ctx.lr = 0x82EA2D2C;
	sub_82EA02A8(ctx, base);
	// 82EA2D2C: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82EA2D30: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EA2D34: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2D38: 80C70040  lwz r6, 0x40(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EA2D3C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82EA2D40: 4E800421  bctrl
	ctx.lr = 0x82EA2D44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2D44: 5465063E  clrlwi r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EA2D48: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EA2D4C: 419A0054  beq cr6, 0x82ea2da0
	if ctx.cr[6].eq {
	pc = 0x82EA2DA0; continue 'dispatch;
	}
	// 82EA2D50: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EA2D54: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA2D58: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82EA2D5C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82EA2D60: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82EA2D64: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82EA2D68: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82EA2D6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA2D70: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82EA2D74: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2D78: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2D7C: 812A0080  lwz r9, 0x80(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EA2D80: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA2D84: 4E800421  bctrl
	ctx.lr = 0x82EA2D88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2D88: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EA2D8C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EA2D90: 41990008  bgt cr6, 0x82ea2d98
	if ctx.cr[6].gt {
	pc = 0x82EA2D98; continue 'dispatch;
	}
	// 82EA2D94: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82EA2D98: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EA2D9C: 4800882D  bl 0x82eab5c8
	ctx.lr = 0x82EA2DA0;
	sub_82EAB5C8(ctx, base);
	// 82EA2DA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA2DA4: 4BFFF8F5  bl 0x82ea2698
	ctx.lr = 0x82EA2DA8;
	sub_82EA2698(ctx, base);
	// 82EA2DA8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EA2DAC: 997C6A38  stb r11, 0x6a38(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(27192 as u32), ctx.r[11].u8 ) };
	// 82EA2DB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA2DB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA2DB8: 48305400  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2DC0 size=132
    let mut pc: u32 = 0x82EA2DC0;
    'dispatch: loop {
        match pc {
            0x82EA2DC0 => {
    //   block [0x82EA2DC0..0x82EA2E44)
	// 82EA2DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA2DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA2DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2DD4: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82EA2DD8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA2DDC: 807F6E5C  lwz r3, 0x6e5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82EA2DE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA2DE4: 419A0044  beq cr6, 0x82ea2e28
	if ctx.cr[6].eq {
	pc = 0x82EA2E28; continue 'dispatch;
	}
	// 82EA2DE8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2DEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA2DF0: 419A0038  beq cr6, 0x82ea2e28
	if ctx.cr[6].eq {
	pc = 0x82EA2E28; continue 'dispatch;
	}
	// 82EA2DF4: A1430006  lhz r10, 6(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA2DF8: 39630006  addi r11, r3, 6
	ctx.r[11].s64 = ctx.r[3].s64 + 6;
	// 82EA2DFC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EA2E00: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82EA2E04: 5527043E  clrlwi r7, r9, 0x10
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82EA2E08: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA2E0C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EA2E10: 409A0018  bne cr6, 0x82ea2e28
	if !ctx.cr[6].eq {
	pc = 0x82EA2E28; continue 'dispatch;
	}
	// 82EA2E14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2E18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA2E1C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2E20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA2E24: 4E800421  bctrl
	ctx.lr = 0x82EA2E28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2E28: 93DF6E5C  stw r30, 0x6e5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28252 as u32), ctx.r[30].u32 ) };
	// 82EA2E2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA2E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA2E38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA2E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA2E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2E48 size=132
    let mut pc: u32 = 0x82EA2E48;
    'dispatch: loop {
        match pc {
            0x82EA2E48 => {
    //   block [0x82EA2E48..0x82EA2ECC)
	// 82EA2E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA2E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA2E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA2E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2E5C: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82EA2E60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA2E64: 807F70EC  lwz r3, 0x70ec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28908 as u32) ) } as u64;
	// 82EA2E68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA2E6C: 419A0044  beq cr6, 0x82ea2eb0
	if ctx.cr[6].eq {
	pc = 0x82EA2EB0; continue 'dispatch;
	}
	// 82EA2E70: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2E74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA2E78: 419A0038  beq cr6, 0x82ea2eb0
	if ctx.cr[6].eq {
	pc = 0x82EA2EB0; continue 'dispatch;
	}
	// 82EA2E7C: A1430006  lhz r10, 6(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA2E80: 39630006  addi r11, r3, 6
	ctx.r[11].s64 = ctx.r[3].s64 + 6;
	// 82EA2E84: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EA2E88: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82EA2E8C: 5527043E  clrlwi r7, r9, 0x10
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82EA2E90: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA2E94: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EA2E98: 409A0018  bne cr6, 0x82ea2eb0
	if !ctx.cr[6].eq {
	pc = 0x82EA2EB0; continue 'dispatch;
	}
	// 82EA2E9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2EA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA2EA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2EA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA2EAC: 4E800421  bctrl
	ctx.lr = 0x82EA2EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2EB0: 93DF70EC  stw r30, 0x70ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28908 as u32), ctx.r[30].u32 ) };
	// 82EA2EB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA2EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA2EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA2EC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA2EC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA2EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA2ED0 size=24
    let mut pc: u32 = 0x82EA2ED0;
    'dispatch: loop {
        match pc {
            0x82EA2ED0 => {
    //   block [0x82EA2ED0..0x82EA2EE8)
	// 82EA2ED0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82EA2ED4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EA2ED8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EA2EDC: 814BC4B0  lwz r10, -0x3b50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15184 as u32) ) } as u64;
	// 82EA2EE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA2EE4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA2EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA2EE8 size=344
    let mut pc: u32 = 0x82EA2EE8;
    'dispatch: loop {
        match pc {
            0x82EA2EE8 => {
    //   block [0x82EA2EE8..0x82EA3040)
	// 82EA2EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA2EEC: 4830527D  bl 0x831a8168
	ctx.lr = 0x82EA2EF0;
	sub_831A8130(ctx, base);
	// 82EA2EF0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA2EF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA2EF8: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2EFC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EA2F00: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EA2F04: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82EA2F08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82EA2F0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA2F10: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82EA2F14: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82EA2F18: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82EA2F1C: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA2F20: 4BFFD811  bl 0x82ea0730
	ctx.lr = 0x82EA2F24;
	sub_82EA0730(ctx, base);
	// 82EA2F24: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA2F28: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82EA2F2C: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82EA2F30: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EA2F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA2F38: B11C0004  sth r8, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82EA2F3C: 80E9C4B4  lwz r7, -0x3b4c(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15180 as u32) ) } as u64;
	// 82EA2F40: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82EA2F44: 4E800421  bctrl
	ctx.lr = 0x82EA2F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2F48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EA2F4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA2F50: 4800A161  bl 0x82ead0b0
	ctx.lr = 0x82EA2F54;
	sub_82EAD0B0(ctx, base);
	// 82EA2F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA2F58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA2F5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA2F60: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2F64: 80A60018  lwz r5, 0x18(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA2F68: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82EA2F6C: 4E800421  bctrl
	ctx.lr = 0x82EA2F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2F70: 88830000  lbz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2F74: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA2F78: 409A0098  bne cr6, 0x82ea3010
	if !ctx.cr[6].eq {
	pc = 0x82EA3010; continue 'dispatch;
	}
	// 82EA2F7C: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82EA2F80: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA2F84: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82EA2F88: 4BFFD7A9  bl 0x82ea0730
	ctx.lr = 0x82EA2F8C;
	sub_82EA0730(ctx, base);
	// 82EA2F8C: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82EA2F90: 38A01000  li r5, 0x1000
	ctx.r[5].s64 = 4096;
	// 82EA2F94: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82EA2F98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA2F9C: 48009EED  bl 0x82eace88
	ctx.lr = 0x82EA2FA0;
	sub_82EACE88(ctx, base);
	// 82EA2FA0: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA2FA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA2FA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA2FAC: 419A0034  beq cr6, 0x82ea2fe0
	if ctx.cr[6].eq {
	pc = 0x82EA2FE0; continue 'dispatch;
	}
	// 82EA2FB0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA2FB4: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EA2FB8: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EA2FBC: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA2FC0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA2FC4: 409A001C  bne cr6, 0x82ea2fe0
	if !ctx.cr[6].eq {
	pc = 0x82EA2FE0; continue 'dispatch;
	}
	// 82EA2FC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2FCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA2FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA2FD4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA2FD8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA2FDC: 4E800421  bctrl
	ctx.lr = 0x82EA2FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA2FE0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EA2FE4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA2FE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA2FEC: 409A0018  bne cr6, 0x82ea3004
	if !ctx.cr[6].eq {
	pc = 0x82EA3004; continue 'dispatch;
	}
	// 82EA2FF0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA2FF4: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA2FF8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA2FFC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA3000: 4BFFD7B1  bl 0x82ea07b0
	ctx.lr = 0x82EA3004;
	sub_82EA07B0(ctx, base);
	// 82EA3004: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA3008: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA300C: 483051AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA3010: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EA3014: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA3018: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA301C: 409A0018  bne cr6, 0x82ea3034
	if !ctx.cr[6].eq {
	pc = 0x82EA3034; continue 'dispatch;
	}
	// 82EA3020: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA3024: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA3028: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA302C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA3030: 4BFFD781  bl 0x82ea07b0
	ctx.lr = 0x82EA3034;
	sub_82EA07B0(ctx, base);
	// 82EA3034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA3038: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA303C: 4830517C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3040 size=264
    let mut pc: u32 = 0x82EA3040;
    'dispatch: loop {
        match pc {
            0x82EA3040 => {
    //   block [0x82EA3040..0x82EA3148)
	// 82EA3040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3044: 48305125  bl 0x831a8168
	ctx.lr = 0x82EA3048;
	sub_831A8130(ctx, base);
	// 82EA3048: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA304C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA3050: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3054: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EA3058: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EA305C: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82EA3060: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EA3064: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA3068: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82EA306C: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82EA3070: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EA3074: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA3078: 4BFFD6B9  bl 0x82ea0730
	ctx.lr = 0x82EA307C;
	sub_82EA0730(ctx, base);
	// 82EA307C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA3080: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82EA3084: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EA3088: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA308C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA3090: B11C0004  sth r8, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82EA3094: 80E9C4B4  lwz r7, -0x3b4c(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15180 as u32) ) } as u64;
	// 82EA3098: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82EA309C: 4E800421  bctrl
	ctx.lr = 0x82EA30A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA30A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EA30A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA30A8: 4800A9C9  bl 0x82eada70
	ctx.lr = 0x82EA30AC;
	sub_82EADA70(ctx, base);
	// 82EA30AC: 7CDDF02E  lwzx r6, r29, r30
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA30B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA30B4: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82EA30B8: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82EA30BC: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82EA30C0: 4BFFD671  bl 0x82ea0730
	ctx.lr = 0x82EA30C4;
	sub_82EA0730(ctx, base);
	// 82EA30C4: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82EA30C8: 38A01000  li r5, 0x1000
	ctx.r[5].s64 = 4096;
	// 82EA30CC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82EA30D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA30D4: 4800A5B5  bl 0x82ead688
	ctx.lr = 0x82EA30D8;
	sub_82EAD688(ctx, base);
	// 82EA30D8: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA30DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA30E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA30E4: 419A0034  beq cr6, 0x82ea3118
	if ctx.cr[6].eq {
	pc = 0x82EA3118; continue 'dispatch;
	}
	// 82EA30E8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA30EC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EA30F0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EA30F4: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA30F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA30FC: 409A001C  bne cr6, 0x82ea3118
	if !ctx.cr[6].eq {
	pc = 0x82EA3118; continue 'dispatch;
	}
	// 82EA3100: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3104: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA3108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA310C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3110: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3114: 4E800421  bctrl
	ctx.lr = 0x82EA3118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3118: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EA311C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA3120: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA3124: 409A0018  bne cr6, 0x82ea313c
	if !ctx.cr[6].eq {
	pc = 0x82EA313C; continue 'dispatch;
	}
	// 82EA3128: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA312C: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA3130: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA3134: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA3138: 4BFFD679  bl 0x82ea07b0
	ctx.lr = 0x82EA313C;
	sub_82EA07B0(ctx, base);
	// 82EA313C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA3140: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA3144: 48305074  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3148 size=92
    let mut pc: u32 = 0x82EA3148;
    'dispatch: loop {
        match pc {
            0x82EA3148 => {
    //   block [0x82EA3148..0x82EA31A4)
	// 82EA3148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA314C: 48305021  bl 0x831a816c
	ctx.lr = 0x82EA3150;
	sub_831A8130(ctx, base);
	// 82EA3150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3158: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA315C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA3160: 392BE9F8  addi r9, r11, -0x1608
	ctx.r[9].s64 = ctx.r[11].s64 + -5640;
	// 82EA3164: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EA3168: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EA316C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA3170: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA3174: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA3178: 48008879  bl 0x82eab9f0
	ctx.lr = 0x82EA317C;
	sub_82EAB9F0(ctx, base);
	// 82EA317C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA3180: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82EA3184: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EA3188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA318C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EA3190: 911F001C  stw r8, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82EA3194: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EA3198: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82EA319C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA31A0: 4830501C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA31A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA31A8 size=364
    let mut pc: u32 = 0x82EA31A8;
    'dispatch: loop {
        match pc {
            0x82EA31A8 => {
    //   block [0x82EA31A8..0x82EA3314)
	// 82EA31A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA31AC: 48304FA5  bl 0x831a8150
	ctx.lr = 0x82EA31B0;
	sub_831A8130(ctx, base);
	// 82EA31B0: 9421FCB0  stwu r1, -0x350(r1)
	ea = ctx.r[1].u32.wrapping_add(-848 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA31B4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA31B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA31BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA31C0: 388BEA48  addi r4, r11, -0x15b8
	ctx.r[4].s64 = ctx.r[11].s64 + -5560;
	// 82EA31C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82EA31C8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EA31CC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EA31D0: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EA31D4: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82EA31D8: 480071D1  bl 0x82eaa3a8
	ctx.lr = 0x82EA31DC;
	sub_82EAA3A8(ctx, base);
	// 82EA31DC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EA31E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EA31E4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82EA31E8: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82EA31EC: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82EA31F0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82EA31F4: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA31F8: 4800A331  bl 0x82ead528
	ctx.lr = 0x82EA31FC;
	sub_82EAD528(ctx, base);
	// 82EA31FC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82EA3200: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA3204: 48002A85  bl 0x82ea5c88
	ctx.lr = 0x82EA3208;
	sub_82EA5C88(ctx, base);
	// 82EA3208: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82EA320C: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82EA3210: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82EA3214: 3CA08212  lis r5, -0x7dee
	ctx.r[5].s64 = -2112749568;
	// 82EA3218: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA321C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA3220: 3B88EA44  addi r28, r8, -0x15bc
	ctx.r[28].s64 = ctx.r[8].s64 + -5564;
	// 82EA3224: 3B27EA3C  addi r25, r7, -0x15c4
	ctx.r[25].s64 = ctx.r[7].s64 + -5572;
	// 82EA3228: 3B06EA38  addi r24, r6, -0x15c8
	ctx.r[24].s64 = ctx.r[6].s64 + -5576;
	// 82EA322C: 3AE5EA30  addi r23, r5, -0x15d0
	ctx.r[23].s64 = ctx.r[5].s64 + -5584;
	// 82EA3230: 3AC10070  addi r22, r1, 0x70
	ctx.r[22].s64 = ctx.r[1].s64 + 112;
	// 82EA3234: 480025A5  bl 0x82ea57d8
	ctx.lr = 0x82EA3238;
	sub_82EA57D8(ctx, base);
	// 82EA3238: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82EA323C: 4800254D  bl 0x82ea5788
	ctx.lr = 0x82EA3240;
	sub_82EA5788(ctx, base);
	// 82EA3240: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA3244: 480026E5  bl 0x82ea5928
	ctx.lr = 0x82EA3248;
	sub_82EA5928(ctx, base);
	// 82EA3248: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EA324C: 4800258D  bl 0x82ea57d8
	ctx.lr = 0x82EA3250;
	sub_82EA57D8(ctx, base);
	// 82EA3250: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82EA3254: 48002585  bl 0x82ea57d8
	ctx.lr = 0x82EA3258;
	sub_82EA57D8(ctx, base);
	// 82EA3258: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EA325C: 4800257D  bl 0x82ea57d8
	ctx.lr = 0x82EA3260;
	sub_82EA57D8(ctx, base);
	// 82EA3260: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA3264: 48002575  bl 0x82ea57d8
	ctx.lr = 0x82EA3268;
	sub_82EA57D8(ctx, base);
	// 82EA3268: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EA326C: 4800256D  bl 0x82ea57d8
	ctx.lr = 0x82EA3270;
	sub_82EA57D8(ctx, base);
	// 82EA3270: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA3274: 48002565  bl 0x82ea57d8
	ctx.lr = 0x82EA3278;
	sub_82EA57D8(ctx, base);
	// 82EA3278: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA327C: 4800255D  bl 0x82ea57d8
	ctx.lr = 0x82EA3280;
	sub_82EA57D8(ctx, base);
	// 82EA3280: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA3284: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA3288: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82EA328C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA3290: 4E800421  bctrl
	ctx.lr = 0x82EA3294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3294: 7F4A0774  extsb r10, r26
	ctx.r[10].s64 = ctx.r[26].s8 as i64;
	// 82EA3298: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA329C: 419A0060  beq cr6, 0x82ea32fc
	if ctx.cr[6].eq {
	pc = 0x82EA32FC; continue 'dispatch;
	}
	// 82EA32A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EA32A4: 4B42BF9D  bl 0x822cf240
	ctx.lr = 0x82EA32A8;
	sub_822CF240(ctx, base);
	// 82EA32A8: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82EA32AC: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82EA32B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EA32B4: 4B92BF25  bl 0x827cf1d8
	ctx.lr = 0x82EA32B8;
	sub_827CF1D8(ctx, base);
	// 82EA32B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA32BC: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82EA32C0: 40990034  ble cr6, 0x82ea32f4
	if !ctx.cr[6].gt {
	pc = 0x82EA32F4; continue 'dispatch;
	}
	// 82EA32C4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA32C8: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA32CC: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA32D0: 386BEA1C  addi r3, r11, -0x15e4
	ctx.r[3].s64 = ctx.r[11].s64 + -5604;
	// 82EA32D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA32D8: 4E800421  bctrl
	ctx.lr = 0x82EA32DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA32DC: 38BEFFFE  addi r5, r30, -2
	ctx.r[5].s64 = ctx.r[30].s64 + -2;
	// 82EA32E0: 388100A8  addi r4, r1, 0xa8
	ctx.r[4].s64 = ctx.r[1].s64 + 168;
	// 82EA32E4: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA32E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EA32EC: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA32F0: 4B42BEF1  bl 0x822cf1e0
	ctx.lr = 0x82EA32F4;
	sub_822CF1E0(ctx, base);
	// 82EA32F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EA32F8: 4B42BF61  bl 0x822cf258
	ctx.lr = 0x82EA32FC;
	sub_822CF258(ctx, base);
	// 82EA32FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA3300: 48002B19  bl 0x82ea5e18
	ctx.lr = 0x82EA3304;
	sub_82EA5E18(ctx, base);
	// 82EA3304: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82EA3308: 4800A431  bl 0x82ead738
	ctx.lr = 0x82EA330C;
	sub_82EAD738(ctx, base);
	// 82EA330C: 38210350  addi r1, r1, 0x350
	ctx.r[1].s64 = ctx.r[1].s64 + 848;
	// 82EA3310: 48304E90  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3318 size=20
    let mut pc: u32 = 0x82EA3318;
    'dispatch: loop {
        match pc {
            0x82EA3318 => {
    //   block [0x82EA3318..0x82EA332C)
	// 82EA3318: 7CAB0774  extsb r11, r5
	ctx.r[11].s64 = ctx.r[5].s8 as i64;
	// 82EA331C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82EA3320: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA3324: 419A0008  beq cr6, 0x82ea332c
	if ctx.cr[6].eq {
		sub_82EA332C(ctx, base);
		return;
	}
	// 82EA3328: 48008F60  b 0x82eac288
	sub_82EAC288(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA332C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA332C size=8
    let mut pc: u32 = 0x82EA332C;
    'dispatch: loop {
        match pc {
            0x82EA332C => {
    //   block [0x82EA332C..0x82EA3334)
	// 82EA332C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EA3330: 48008778  b 0x82eabaa8
	sub_82EABAA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3338 size=76
    let mut pc: u32 = 0x82EA3338;
    'dispatch: loop {
        match pc {
            0x82EA3338 => {
    //   block [0x82EA3338..0x82EA3384)
	// 82EA3338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA333C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA3340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA3344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3348: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA334C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3350: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EA3354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EA3358: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EA335C: 4800887D  bl 0x82eabbd8
	ctx.lr = 0x82EA3360;
	sub_82EABBD8(ctx, base);
	// 82EA3360: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EA3364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA3368: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EA336C: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EA3370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA3374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA3378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA337C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA3380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3388 size=8
    let mut pc: u32 = 0x82EA3388;
    'dispatch: loop {
        match pc {
            0x82EA3388 => {
    //   block [0x82EA3388..0x82EA3390)
	// 82EA3388: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82EA338C: 4800850C  b 0x82eab898
	sub_82EAB898(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3390 size=292
    let mut pc: u32 = 0x82EA3390;
    'dispatch: loop {
        match pc {
            0x82EA3390 => {
    //   block [0x82EA3390..0x82EA344C)
	// 82EA3390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3394: 48304DCD  bl 0x831a8160
	ctx.lr = 0x82EA3398;
	sub_831A8130(ctx, base);
	// 82EA3398: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA339C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA33A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA33A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA33A8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EA33AC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EA33B0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EA33B4: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EA33B8: 409A0024  bne cr6, 0x82ea33dc
	if !ctx.cr[6].eq {
	pc = 0x82EA33DC; continue 'dispatch;
	}
	// 82EA33BC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA33C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA33C4: 419A0018  beq cr6, 0x82ea33dc
	if ctx.cr[6].eq {
	pc = 0x82EA33DC; continue 'dispatch;
	}
	// 82EA33C8: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA33CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA33D0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA33D4: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EA33D8: 83C9FFFC  lwz r30, -4(r9)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EA33DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA33E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA33E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA33E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA33EC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA33F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA33F4: 4E800421  bctrl
	ctx.lr = 0x82EA33F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA33F8: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA33FC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA3400: 409A0010  bne cr6, 0x82ea3410
	if !ctx.cr[6].eq {
	pc = 0x82EA3410; continue 'dispatch;
	}
	// 82EA3404: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA3408: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA340C: 48304DA4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82EA3410: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA3414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EA3418: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82EA341C: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 82EA3420: 4199005C  bgt cr6, 0x82ea347c
	if ctx.cr[6].gt {
	pc = 0x82EA347C; continue 'dispatch;
	}
	// 82EA3424: 3D8082EA  lis r12, -0x7d16
	ctx.r[12].s64 = -2098593792;
	// 82EA3428: 398C343C  addi r12, r12, 0x343c
	ctx.r[12].s64 = ctx.r[12].s64 + 13372;
	// 82EA342C: 57A0103A  slwi r0, r29, 2
	ctx.r[0].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82EA3430: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82EA3434: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82EA3438: 4E800420  bctr
	match ctx.r[29].u64 {
		0 => {
	pc = 0x82EA344C; continue 'dispatch;
		},
		1 => {
	pc = 0x82EA3458; continue 'dispatch;
		},
		2 => {
	pc = 0x82EA3464; continue 'dispatch;
		},
		3 => {
	pc = 0x82EA3470; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82EA343C: 82EA344C  lwz r23, 0x344c(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13388 as u32) ) } as u64;
	// 82EA3440: 82EA3458  lwz r23, 0x3458(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13400 as u32) ) } as u64;
	// 82EA3444: 82EA3464  lwz r23, 0x3464(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13412 as u32) ) } as u64;
	// 82EA3448: 82EA3470  lwz r23, 0x3470(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13424 as u32) ) } as u64;
            }
            0x82EA344C => {
    //   block [0x82EA344C..0x82EA3458)
	// 82EA344C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA3450: 388BEA60  addi r4, r11, -0x15a0
	ctx.r[4].s64 = ctx.r[11].s64 + -5536;
	// 82EA3454: 48000028  b 0x82ea347c
	pc = 0x82EA347C; continue 'dispatch;
            }
            0x82EA3458 => {
    //   block [0x82EA3458..0x82EA3464)
	// 82EA3458: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA345C: 388BEA58  addi r4, r11, -0x15a8
	ctx.r[4].s64 = ctx.r[11].s64 + -5544;
	// 82EA3460: 4800001C  b 0x82ea347c
	pc = 0x82EA347C; continue 'dispatch;
            }
            0x82EA3464 => {
    //   block [0x82EA3464..0x82EA3470)
	// 82EA3464: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82EA3468: 388BFF5C  addi r4, r11, -0xa4
	ctx.r[4].s64 = ctx.r[11].s64 + -164;
	// 82EA346C: 4800000C  b 0x82ea3478
	pc = 0x82EA3478; continue 'dispatch;
            }
            0x82EA3470 => {
    //   block [0x82EA3470..0x82EA34B4)
	// 82EA3470: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA3474: 388BEA50  addi r4, r11, -0x15b0
	ctx.r[4].s64 = ctx.r[11].s64 + -5552;
	// 82EA3478: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EA347C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EA3480: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EA3484: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA3488: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA348C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA3490: 4BFFFD19  bl 0x82ea31a8
	ctx.lr = 0x82EA3494;
	sub_82EA31A8(ctx, base);
	// 82EA3494: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82EA3498: 419A0010  beq cr6, 0x82ea34a8
	if ctx.cr[6].eq {
	pc = 0x82EA34A8; continue 'dispatch;
	}
	// 82EA349C: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 82EA34A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA34A4: 409A0008  bne cr6, 0x82ea34ac
	if !ctx.cr[6].eq {
	pc = 0x82EA34AC; continue 'dispatch;
	}
	// 82EA34A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA34AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA34B0: 48304D00  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA34B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA34B8 size=112
    let mut pc: u32 = 0x82EA34B8;
    'dispatch: loop {
        match pc {
            0x82EA34B8 => {
    //   block [0x82EA34B8..0x82EA3528)
	// 82EA34B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA34BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA34C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA34C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA34C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA34CC: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA34D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA34D4: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA34D8: 3BE30014  addi r31, r3, 0x14
	ctx.r[31].s64 = ctx.r[3].s64 + 20;
	// 82EA34DC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA34E0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EA34E4: 409A0010  bne cr6, 0x82ea34f4
	if !ctx.cr[6].eq {
	pc = 0x82EA34F4; continue 'dispatch;
	}
	// 82EA34E8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EA34EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA34F0: 48003391  bl 0x82ea6880
	ctx.lr = 0x82EA34F4;
	sub_82EA6880(ctx, base);
	// 82EA34F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA34F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA34FC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA3500: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82EA3504: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA3508: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EA350C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EA3510: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA3514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA3518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA351C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA3520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA3524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3528 size=20
    let mut pc: u32 = 0x82EA3528;
    'dispatch: loop {
        match pc {
            0x82EA3528 => {
    //   block [0x82EA3528..0x82EA353C)
	// 82EA3528: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA352C: 39630014  addi r11, r3, 0x14
	ctx.r[11].s64 = ctx.r[3].s64 + 20;
	// 82EA3530: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82EA3534: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EA3538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3540 size=160
    let mut pc: u32 = 0x82EA3540;
    'dispatch: loop {
        match pc {
            0x82EA3540 => {
    //   block [0x82EA3540..0x82EA35E0)
	// 82EA3540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA3548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA354C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA3550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3558: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA355C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA3560: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA3564: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA3568: 409A0020  bne cr6, 0x82ea3588
	if !ctx.cr[6].eq {
	pc = 0x82EA3588; continue 'dispatch;
	}
	// 82EA356C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3570: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA3574: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA3578: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA357C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA3580: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA3584: 4BFFD22D  bl 0x82ea07b0
	ctx.lr = 0x82EA3588;
	sub_82EA07B0(ctx, base);
	// 82EA3588: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EA358C: 480084E5  bl 0x82eaba70
	ctx.lr = 0x82EA3590;
	sub_82EABA70(ctx, base);
	// 82EA3590: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA3594: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA3598: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EA359C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA35A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA35A4: 419A0020  beq cr6, 0x82ea35c4
	if ctx.cr[6].eq {
	pc = 0x82EA35C4; continue 'dispatch;
	}
	// 82EA35A8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA35AC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA35B0: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82EA35B4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA35B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA35BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA35C0: 4BFFD1F1  bl 0x82ea07b0
	ctx.lr = 0x82EA35C4;
	sub_82EA07B0(ctx, base);
	// 82EA35C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA35C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA35CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA35D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA35D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA35D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA35DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA35E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA35E0 size=44
    let mut pc: u32 = 0x82EA35E0;
    'dispatch: loop {
        match pc {
            0x82EA35E0 => {
    //   block [0x82EA35E0..0x82EA360C)
	// 82EA35E0: 2F042000  cmpwi cr6, r4, 0x2000
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8192, &mut ctx.xer);
	// 82EA35E4: 4199004C  bgt cr6, 0x82ea3630
	if ctx.cr[6].gt {
		sub_82EA3630(ctx, base);
		return;
	}
	// 82EA35E8: 2F040200  cmpwi cr6, r4, 0x200
	ctx.cr[6].compare_i32(ctx.r[4].s32, 512, &mut ctx.xer);
	// 82EA35EC: 41990020  bgt cr6, 0x82ea360c
	if ctx.cr[6].gt {
		sub_82EA360C(ctx, base);
		return;
	}
	// 82EA35F0: 7D641A14  add r11, r4, r3
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 82EA35F4: 894B0AF0  lbz r10, 0xaf0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82EA35F8: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82EA35FC: 396B02AB  addi r11, r11, 0x2ab
	ctx.r[11].s64 = ctx.r[11].s64 + 683;
	// 82EA3600: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA3604: 7C6A182E  lwzx r3, r10, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EA3608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA360C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA360C size=36
    let mut pc: u32 = 0x82EA360C;
    'dispatch: loop {
        match pc {
            0x82EA360C => {
    //   block [0x82EA360C..0x82EA3630)
	// 82EA360C: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EA3610: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82EA3614: 394B033D  addi r10, r11, 0x33d
	ctx.r[10].s64 = ctx.r[11].s64 + 829;
	// 82EA3618: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA361C: 7D69182E  lwzx r11, r9, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EA3620: 396B02AB  addi r11, r11, 0x2ab
	ctx.r[11].s64 = ctx.r[11].s64 + 683;
	// 82EA3624: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA3628: 7C6A182E  lwzx r3, r10, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EA362C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3630 size=4
    let mut pc: u32 = 0x82EA3630;
    'dispatch: loop {
        match pc {
            0x82EA3630 => {
    //   block [0x82EA3630..0x82EA3634)
	// 82EA3630: 4BFFF0F0  b 0x82ea2720
	sub_82EA2720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3638 size=8
    let mut pc: u32 = 0x82EA3638;
    'dispatch: loop {
        match pc {
            0x82EA3638 => {
    //   block [0x82EA3638..0x82EA3640)
	// 82EA3638: 80630D74  lwz r3, 0xd74(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA363C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3640 size=72
    let mut pc: u32 = 0x82EA3640;
    'dispatch: loop {
        match pc {
            0x82EA3640 => {
    //   block [0x82EA3640..0x82EA3688)
	// 82EA3640: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EA3644: 39630D18  addi r11, r3, 0xd18
	ctx.r[11].s64 = ctx.r[3].s64 + 3352;
	// 82EA3648: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82EA364C: 812BFD54  lwz r9, -0x2ac(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-684 as u32) ) } as u64;
	// 82EA3650: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA3654: 419A0010  beq cr6, 0x82ea3664
	if ctx.cr[6].eq {
	pc = 0x82EA3664; continue 'dispatch;
	}
	// 82EA3658: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA365C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA3660: 409AFFF8  bne cr6, 0x82ea3658
	if !ctx.cr[6].eq {
	pc = 0x82EA3658; continue 'dispatch;
	}
	// 82EA3664: 812BFD98  lwz r9, -0x268(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-616 as u32) ) } as u64;
	// 82EA3668: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA366C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3670: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA3674: 7D2939D6  mullw r9, r9, r7
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82EA3678: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82EA367C: 4082FFD0  bne 0x82ea364c
	if !ctx.cr[0].eq {
	pc = 0x82EA364C; continue 'dispatch;
	}
	// 82EA3680: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82EA3684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3688 size=104
    let mut pc: u32 = 0x82EA3688;
    'dispatch: loop {
        match pc {
            0x82EA3688 => {
    //   block [0x82EA3688..0x82EA36F0)
	// 82EA3688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA3690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3694: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82EA3698: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA369C: 409A001C  bne cr6, 0x82ea36b8
	if !ctx.cr[6].eq {
	pc = 0x82EA36B8; continue 'dispatch;
	}
	// 82EA36A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA36A4: 91660D78  stw r11, 0xd78(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(3448 as u32), ctx.r[11].u32 ) };
	// 82EA36A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA36AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA36B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA36B4: 4E800020  blr
	return;
	// 82EA36B8: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82EA36BC: 4BFFFF85  bl 0x82ea3640
	ctx.lr = 0x82EA36C0;
	sub_82EA3640(ctx, base);
	// 82EA36C0: 81660D5C  lwz r11, 0xd5c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA36C4: 7D435A14  add r10, r3, r11
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82EA36C8: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA36CC: 41980014  blt cr6, 0x82ea36e0
	if ctx.cr[6].lt {
	pc = 0x82EA36E0; continue 'dispatch;
	}
	// 82EA36D0: 81660D7C  lwz r11, 0xd7c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3452 as u32) ) } as u64;
	// 82EA36D4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EA36D8: 41980008  blt cr6, 0x82ea36e0
	if ctx.cr[6].lt {
	pc = 0x82EA36E0; continue 'dispatch;
	}
	// 82EA36DC: 90860D78  stw r4, 0xd78(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(3448 as u32), ctx.r[4].u32 ) };
	// 82EA36E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA36E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA36E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA36EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA36F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA36F0 size=20
    let mut pc: u32 = 0x82EA36F0;
    'dispatch: loop {
        match pc {
            0x82EA36F0 => {
    //   block [0x82EA36F0..0x82EA3704)
	// 82EA36F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA36F4: 409A0010  bne cr6, 0x82ea3704
	if !ctx.cr[6].eq {
		sub_82EA3704(ctx, base);
		return;
	}
	// 82EA36F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA36FC: 91630D7C  stw r11, 0xd7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3452 as u32), ctx.r[11].u32 ) };
	// 82EA3700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3704 size=20
    let mut pc: u32 = 0x82EA3704;
    'dispatch: loop {
        match pc {
            0x82EA3704 => {
    //   block [0x82EA3704..0x82EA3718)
	// 82EA3704: 81630D78  lwz r11, 0xd78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82EA3708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA370C: 419A000C  beq cr6, 0x82ea3718
	if ctx.cr[6].eq {
		sub_82EA3718(ctx, base);
		return;
	}
	// 82EA3710: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EA3714: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3718 size=8
    let mut pc: u32 = 0x82EA3718;
    'dispatch: loop {
        match pc {
            0x82EA3718 => {
    //   block [0x82EA3718..0x82EA3720)
	// 82EA3718: 90830D7C  stw r4, 0xd7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3452 as u32), ctx.r[4].u32 ) };
	// 82EA371C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3720 size=8
    let mut pc: u32 = 0x82EA3720;
    'dispatch: loop {
        match pc {
            0x82EA3720 => {
    //   block [0x82EA3720..0x82EA3728)
	// 82EA3720: 80630D78  lwz r3, 0xd78(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82EA3724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3728 size=8
    let mut pc: u32 = 0x82EA3728;
    'dispatch: loop {
        match pc {
            0x82EA3728 => {
    //   block [0x82EA3728..0x82EA3730)
	// 82EA3728: 80630D7C  lwz r3, 0xd7c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3452 as u32) ) } as u64;
	// 82EA372C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3730 size=16
    let mut pc: u32 = 0x82EA3730;
    'dispatch: loop {
        match pc {
            0x82EA3730 => {
    //   block [0x82EA3730..0x82EA3740)
	// 82EA3730: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA3734: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EA3738: 388BEBC0  addi r4, r11, -0x1440
	ctx.r[4].s64 = ctx.r[11].s64 + -5184;
	// 82EA373C: 480023F4  b 0x82ea5b30
	sub_82EA5B30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3740 size=4
    let mut pc: u32 = 0x82EA3740;
    'dispatch: loop {
        match pc {
            0x82EA3740 => {
    //   block [0x82EA3740..0x82EA3744)
	// 82EA3740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3748 size=44
    let mut pc: u32 = 0x82EA3748;
    'dispatch: loop {
        match pc {
            0x82EA3748 => {
    //   block [0x82EA3748..0x82EA3774)
	// 82EA3748: 39650345  addi r11, r5, 0x345
	ctx.r[11].s64 = ctx.r[5].s64 + 837;
	// 82EA374C: 3925029A  addi r9, r5, 0x29a
	ctx.r[9].s64 = ctx.r[5].s64 + 666;
	// 82EA3750: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA3754: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA3758: 7D2A182E  lwzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EA375C: 7D0B182E  lwzx r8, r11, r3
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EA3760: 38E9FFFF  addi r7, r9, -1
	ctx.r[7].s64 = ctx.r[9].s64 + -1;
	// 82EA3764: 7CEA192E  stwx r7, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[7].u32) };
	// 82EA3768: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA376C: 7C8B192E  stwx r4, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u32) };
	// 82EA3770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3778 size=4
    let mut pc: u32 = 0x82EA3778;
    'dispatch: loop {
        match pc {
            0x82EA3778 => {
    //   block [0x82EA3778..0x82EA377C)
	// 82EA3778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3780 size=4
    let mut pc: u32 = 0x82EA3780;
    'dispatch: loop {
        match pc {
            0x82EA3780 => {
    //   block [0x82EA3780..0x82EA3784)
	// 82EA3780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA3788 size=8
    let mut pc: u32 = 0x82EA3788;
    'dispatch: loop {
        match pc {
            0x82EA3788 => {
    //   block [0x82EA3788..0x82EA3790)
	// 82EA3788: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA378C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3790 size=100
    let mut pc: u32 = 0x82EA3790;
    'dispatch: loop {
        match pc {
            0x82EA3790 => {
    //   block [0x82EA3790..0x82EA37F4)
	// 82EA3790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3794: 483049D1  bl 0x831a8164
	ctx.lr = 0x82EA3798;
	sub_831A8130(ctx, base);
	// 82EA3798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA379C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA37A0: 39602100  li r11, 0x2100
	ctx.r[11].s64 = 8448;
	// 82EA37A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EA37A8: 7FA45BD7  divw. r29, r4, r11
	ctx.r[29].s32 = ctx.r[4].s32 / ctx.r[11].s32;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EA37AC: 83DC0A1C  lwz r30, 0xa1c(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA37B0: 40810038  ble 0x82ea37e8
	if !ctx.cr[0].gt {
	pc = 0x82EA37E8; continue 'dispatch;
	}
	// 82EA37B4: 3F608332  lis r27, -0x7cce
	ctx.r[27].s64 = -2093875200;
	// 82EA37B8: 817BBDBC  lwz r11, -0x4244(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA37BC: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA37C0: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA37C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA37C8: 4E800421  bctrl
	ctx.lr = 0x82EA37CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA37CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA37D0: 419A0018  beq cr6, 0x82ea37e8
	if ctx.cr[6].eq {
	pc = 0x82EA37E8; continue 'dispatch;
	}
	// 82EA37D4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EA37D8: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EA37DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA37E0: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EA37E4: 4198FFD4  blt cr6, 0x82ea37b8
	if ctx.cr[6].lt {
	pc = 0x82EA37B8; continue 'dispatch;
	}
	// 82EA37E8: 93DC0A1C  stw r30, 0xa1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2588 as u32), ctx.r[30].u32 ) };
	// 82EA37EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA37F0: 483049C4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA37F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA37F8 size=20
    let mut pc: u32 = 0x82EA37F8;
    'dispatch: loop {
        match pc {
            0x82EA37F8 => {
    //   block [0x82EA37F8..0x82EA380C)
	// 82EA37F8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82EA37FC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82EA3800: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA3804: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82EA3808: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA380C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA380C size=32
    let mut pc: u32 = 0x82EA380C;
    'dispatch: loop {
        match pc {
            0x82EA380C => {
    //   block [0x82EA380C..0x82EA382C)
	// 82EA380C: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82EA3810: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82EA3814: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA3818: 7CE8482E  lwzx r7, r8, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EA381C: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EA3820: 4098000C  bge cr6, 0x82ea382c
	if !ctx.cr[6].lt {
		sub_82EA382C(ctx, base);
		return;
	}
	// 82EA3824: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82EA3828: 48000008  b 0x82ea3830
	sub_82EA382C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA382C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA382C size=20
    let mut pc: u32 = 0x82EA382C;
    'dispatch: loop {
        match pc {
            0x82EA382C => {
    //   block [0x82EA382C..0x82EA3840)
	// 82EA382C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA3830: 7D635050  subf r11, r3, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 82EA3834: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EA3838: 4199FFD4  bgt cr6, 0x82ea380c
	if ctx.cr[6].gt {
		sub_82EA380C(ctx, base);
		return;
	}
	// 82EA383C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3840 size=76
    let mut pc: u32 = 0x82EA3840;
    'dispatch: loop {
        match pc {
            0x82EA3840 => {
    //   block [0x82EA3840..0x82EA388C)
	// 82EA3840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA3848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA384C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3850: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3854: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3858: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EA385C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3860: 4E800421  bctrl
	ctx.lr = 0x82EA3864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3864: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA386C: 81090034  lwz r8, 0x34(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA3870: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA3874: 4E800421  bctrl
	ctx.lr = 0x82EA3878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA387C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA3880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA3884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA3888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3890 size=60
    let mut pc: u32 = 0x82EA3890;
    'dispatch: loop {
        match pc {
            0x82EA3890 => {
    //   block [0x82EA3890..0x82EA38CC)
	// 82EA3890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3894: 483048D9  bl 0x831a816c
	ctx.lr = 0x82EA3898;
	sub_831A8130(ctx, base);
	// 82EA3898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA389C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA38A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA38A4: 3BDF0A30  addi r30, r31, 0xa30
	ctx.r[30].s64 = ctx.r[31].s64 + 2608;
	// 82EA38A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA38AC: 4BFFE805  bl 0x82ea20b0
	ctx.lr = 0x82EA38B0;
	sub_82EA20B0(ctx, base);
	// 82EA38B0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA38B4: 93BF0D74  stw r29, 0xd74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3444 as u32), ctx.r[29].u32 ) };
	// 82EA38B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA38BC: F97F0A50  std r11, 0xa50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2640 as u32), ctx.r[11].u64 ) };
	// 82EA38C0: 4839F09D  bl 0x8324295c
	ctx.lr = 0x82EA38C4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA38C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA38C8: 483048F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA38D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EA38D0 size=192
    let mut pc: u32 = 0x82EA38D0;
    'dispatch: loop {
        match pc {
            0x82EA38D0 => {
    //   block [0x82EA38D0..0x82EA3990)
	// 82EA38D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA38D4: 4830488D  bl 0x831a8160
	ctx.lr = 0x82EA38D8;
	sub_831A8130(ctx, base);
	// 82EA38D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA38DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA38E0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EA38E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EA38E8: 83BF0D7C  lwz r29, 0xd7c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3452 as u32) ) } as u64;
	// 82EA38EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA38F0: 409A0024  bne cr6, 0x82ea3914
	if !ctx.cr[6].eq {
	pc = 0x82EA3914; continue 'dispatch;
	}
	// 82EA38F4: 83BF0D78  lwz r29, 0xd78(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82EA38F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA38FC: 409A0018  bne cr6, 0x82ea3914
	if !ctx.cr[6].eq {
	pc = 0x82EA3914; continue 'dispatch;
	}
	// 82EA3900: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA3904: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EA3908: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA390C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA3910: 483048A0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82EA3914: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EA3918: 409A0008  bne cr6, 0x82ea3920
	if !ctx.cr[6].eq {
	pc = 0x82EA3920; continue 'dispatch;
	}
	// 82EA391C: 3B601000  li r27, 0x1000
	ctx.r[27].s64 = 4096;
	// 82EA3920: 817F0D68  lwz r11, 0xd68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3432 as u32) ) } as u64;
	// 82EA3924: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA3928: 815F0D5C  lwz r10, 0xd5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA392C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82EA3930: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA3934: 7D0BDA14  add r8, r11, r27
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EA3938: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82EA393C: 4198FFC4  blt cr6, 0x82ea3900
	if ctx.cr[6].lt {
	pc = 0x82EA3900; continue 'dispatch;
	}
	// 82EA3940: 3BDF0A30  addi r30, r31, 0xa30
	ctx.r[30].s64 = ctx.r[31].s64 + 2608;
	// 82EA3944: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA3948: 4BFFE769  bl 0x82ea20b0
	ctx.lr = 0x82EA394C;
	sub_82EA20B0(ctx, base);
	// 82EA394C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA3950: 4BFFFCF1  bl 0x82ea3640
	ctx.lr = 0x82EA3954;
	sub_82EA3640(ctx, base);
	// 82EA3954: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA3958: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA395C: F97F0A50  std r11, 0xa50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2640 as u32), ctx.r[11].u64 ) };
	// 82EA3960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA3964: 4839EFF9  bl 0x8324295c
	ctx.lr = 0x82EA3968;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA3968: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA396C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EA3970: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EA3974: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EA3978: 7D3D5010  subfc r9, r29, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[29].u32;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 82EA397C: 7D094910  subfe r8, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82EA3980: 550707FE  clrlwi r7, r8, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82EA3984: 98FA0000  stb r7, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82EA3988: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA398C: 48304824  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3990 size=116
    let mut pc: u32 = 0x82EA3990;
    'dispatch: loop {
        match pc {
            0x82EA3990 => {
    //   block [0x82EA3990..0x82EA3A04)
	// 82EA3990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3994: 483047D9  bl 0x831a816c
	ctx.lr = 0x82EA3998;
	sub_831A8130(ctx, base);
	// 82EA3998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA399C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA39A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA39A4: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 82EA39A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA39AC: 4BFFE705  bl 0x82ea20b0
	ctx.lr = 0x82EA39B0;
	sub_82EA20B0(ctx, base);
	// 82EA39B0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA39B4: 409A0010  bne cr6, 0x82ea39c4
	if !ctx.cr[6].eq {
	pc = 0x82EA39C4; continue 'dispatch;
	}
	// 82EA39B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA39BC: 917F0D78  stw r11, 0xd78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3448 as u32), ctx.r[11].u32 ) };
	// 82EA39C0: 4800002C  b 0x82ea39ec
	pc = 0x82EA39EC; continue 'dispatch;
	// 82EA39C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA39C8: 4BFFFC79  bl 0x82ea3640
	ctx.lr = 0x82EA39CC;
	sub_82EA3640(ctx, base);
	// 82EA39CC: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA39D0: 7D435A14  add r10, r3, r11
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82EA39D4: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA39D8: 41980014  blt cr6, 0x82ea39ec
	if ctx.cr[6].lt {
	pc = 0x82EA39EC; continue 'dispatch;
	}
	// 82EA39DC: 817F0D7C  lwz r11, 0xd7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3452 as u32) ) } as u64;
	// 82EA39E0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EA39E4: 41980008  blt cr6, 0x82ea39ec
	if ctx.cr[6].lt {
	pc = 0x82EA39EC; continue 'dispatch;
	}
	// 82EA39E8: 93DF0D78  stw r30, 0xd78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3448 as u32), ctx.r[30].u32 ) };
	// 82EA39EC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA39F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA39F4: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA39F8: 4839EF65  bl 0x8324295c
	ctx.lr = 0x82EA39FC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA39FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA3A00: 483047BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3A08 size=100
    let mut pc: u32 = 0x82EA3A08;
    'dispatch: loop {
        match pc {
            0x82EA3A08 => {
    //   block [0x82EA3A08..0x82EA3A6C)
	// 82EA3A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3A0C: 48304761  bl 0x831a816c
	ctx.lr = 0x82EA3A10;
	sub_831A8130(ctx, base);
	// 82EA3A10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3A18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA3A1C: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 82EA3A20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA3A24: 4BFFE68D  bl 0x82ea20b0
	ctx.lr = 0x82EA3A28;
	sub_82EA20B0(ctx, base);
	// 82EA3A28: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA3A2C: 409A0010  bne cr6, 0x82ea3a3c
	if !ctx.cr[6].eq {
	pc = 0x82EA3A3C; continue 'dispatch;
	}
	// 82EA3A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA3A34: 917F0D7C  stw r11, 0xd7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3452 as u32), ctx.r[11].u32 ) };
	// 82EA3A38: 4800001C  b 0x82ea3a54
	pc = 0x82EA3A54; continue 'dispatch;
	// 82EA3A3C: 817F0D78  lwz r11, 0xd78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82EA3A40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA3A44: 419A000C  beq cr6, 0x82ea3a50
	if ctx.cr[6].eq {
	pc = 0x82EA3A50; continue 'dispatch;
	}
	// 82EA3A48: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EA3A4C: 41990008  bgt cr6, 0x82ea3a54
	if ctx.cr[6].gt {
	pc = 0x82EA3A54; continue 'dispatch;
	}
	// 82EA3A50: 93DF0D7C  stw r30, 0xd7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3452 as u32), ctx.r[30].u32 ) };
	// 82EA3A54: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA3A58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA3A5C: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA3A60: 4839EEFD  bl 0x8324295c
	ctx.lr = 0x82EA3A64;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA3A64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA3A68: 48304754  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3A70 size=196
    let mut pc: u32 = 0x82EA3A70;
    'dispatch: loop {
        match pc {
            0x82EA3A70 => {
    //   block [0x82EA3A70..0x82EA3B34)
	// 82EA3A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3A74: 483046F9  bl 0x831a816c
	ctx.lr = 0x82EA3A78;
	sub_831A8130(ctx, base);
	// 82EA3A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3A7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3A80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA3A84: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 82EA3A88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA3A8C: 4BFFE625  bl 0x82ea20b0
	ctx.lr = 0x82EA3A90;
	sub_82EA20B0(ctx, base);
	// 82EA3A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA3A94: 4BFFFBAD  bl 0x82ea3640
	ctx.lr = 0x82EA3A98;
	sub_82EA3640(ctx, base);
	// 82EA3A98: 817F0D64  lwz r11, 0xd64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA3A9C: 813F0D6C  lwz r9, 0xd6c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3436 as u32) ) } as u64;
	// 82EA3AA0: 811F0D68  lwz r8, 0xd68(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3432 as u32) ) } as u64;
	// 82EA3AA4: 815F0D5C  lwz r10, 0xd5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA3AA8: 7D2959D6  mullw r9, r9, r11
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82EA3AAC: 7CE859D6  mullw r7, r8, r11
	ctx.r[7].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82EA3AB0: 7D633850  subf r11, r3, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[3].s64;
	// 82EA3AB4: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EA3AB8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EA3ABC: 7CCA1A14  add r6, r10, r3
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82EA3AC0: 90DE0000  stw r6, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EA3AC4: 815F0D5C  lwz r10, 0xd5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA3AC8: 7CAA1A14  add r5, r10, r3
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82EA3ACC: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA3AD0: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA3AD4: 809F0D78  lwz r4, 0xd78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82EA3AD8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA3ADC: 409A0014  bne cr6, 0x82ea3af0
	if !ctx.cr[6].eq {
	pc = 0x82EA3AF0; continue 'dispatch;
	}
	// 82EA3AE0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA3AE4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA3AE8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EA3AEC: 48000030  b 0x82ea3b1c
	pc = 0x82EA3B1C; continue 'dispatch;
	// 82EA3AF0: 817F0D78  lwz r11, 0xd78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82EA3AF4: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82EA3AF8: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EA3AFC: 813F0D78  lwz r9, 0xd78(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82EA3B00: 811F0D5C  lwz r8, 0xd5c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA3B04: 80FF0D64  lwz r7, 0xd64(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA3B08: 80DF0D68  lwz r6, 0xd68(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3432 as u32) ) } as u64;
	// 82EA3B0C: 7CA731D6  mullw r5, r7, r6
	ctx.r[5].s64 = (ctx.r[7].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82EA3B10: 7C854850  subf r4, r5, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[5].s64;
	// 82EA3B14: 7C682050  subf r3, r8, r4
	ctx.r[3].s64 = ctx.r[4].s64 - ctx.r[8].s64;
	// 82EA3B18: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82EA3B1C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA3B20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA3B24: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA3B28: 4839EE35  bl 0x8324295c
	ctx.lr = 0x82EA3B2C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA3B2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA3B30: 4830468C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3B38 size=1116
    let mut pc: u32 = 0x82EA3B38;
    'dispatch: loop {
        match pc {
            0x82EA3B38 => {
    //   block [0x82EA3B38..0x82EA3F94)
	// 82EA3B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3B3C: 48304619  bl 0x831a8154
	ctx.lr = 0x82EA3B40;
	sub_831A8130(ctx, base);
	// 82EA3B40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3B48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA3B4C: 3AFF0A30  addi r23, r31, 0xa30
	ctx.r[23].s64 = ctx.r[31].s64 + 2608;
	// 82EA3B50: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82EA3B54: 4BFFE55D  bl 0x82ea20b0
	ctx.lr = 0x82EA3B58;
	sub_82EA20B0(ctx, base);
	// 82EA3B58: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82EA3B5C: 41990370  bgt cr6, 0x82ea3ecc
	if ctx.cr[6].gt {
	pc = 0x82EA3ECC; continue 'dispatch;
	}
	// 82EA3B60: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82EA3B64: 41990014  bgt cr6, 0x82ea3b78
	if ctx.cr[6].gt {
	pc = 0x82EA3B78; continue 'dispatch;
	}
	// 82EA3B68: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82EA3B6C: 894B0AF0  lbz r10, 0xaf0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82EA3B70: 7D580774  extsb r24, r10
	ctx.r[24].s64 = ctx.r[10].s8 as i64;
	// 82EA3B74: 48000018  b 0x82ea3b8c
	pc = 0x82EA3B8C; continue 'dispatch;
	// 82EA3B78: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82EA3B7C: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82EA3B80: 394B033D  addi r10, r11, 0x33d
	ctx.r[10].s64 = ctx.r[11].s64 + 829;
	// 82EA3B84: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA3B88: 7F09F82E  lwzx r24, r9, r31
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3B8C: 39780345  addi r11, r24, 0x345
	ctx.r[11].s64 = ctx.r[24].s64 + 837;
	// 82EA3B90: 395802AB  addi r10, r24, 0x2ab
	ctx.r[10].s64 = ctx.r[24].s64 + 683;
	// 82EA3B94: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82EA3B98: 3938029A  addi r9, r24, 0x29a
	ctx.r[9].s64 = ctx.r[24].s64 + 666;
	// 82EA3B9C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA3BA0: 5539103A  slwi r25, r9, 2
	ctx.r[25].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82EA3BA4: 7D3AF82E  lwzx r9, r26, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3BA8: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82EA3BAC: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3BB0: 7D1AF92E  stwx r8, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u32) };
	// 82EA3BB4: 813F0D70  lwz r9, 0xd70(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 82EA3BB8: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA3BBC: 90FF0D70  stw r7, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[7].u32 ) };
	// 82EA3BC0: 7FB9F82E  lwzx r29, r25, r31
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3BC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA3BC8: 419A0010  beq cr6, 0x82ea3bd8
	if ctx.cr[6].eq {
	pc = 0x82EA3BD8; continue 'dispatch;
	}
	// 82EA3BCC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3BD0: 7D79F92E  stwx r11, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82EA3BD4: 480003A4  b 0x82ea3f78
	pc = 0x82EA3F78; continue 'dispatch;
	// 82EA3BD8: 7FCBF82E  lwzx r30, r11, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3BDC: 2F18000D  cmpwi cr6, r24, 0xd
	ctx.cr[6].compare_i32(ctx.r[24].s32, 13, &mut ctx.xer);
	// 82EA3BE0: 409801C8  bge cr6, 0x82ea3da8
	if !ctx.cr[6].lt {
	pc = 0x82EA3DA8; continue 'dispatch;
	}
	// 82EA3BE4: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA3BE8: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82EA3BEC: 815F0A24  lwz r10, 0xa24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA3BF0: 7D3E5A14  add r9, r30, r11
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EA3BF4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA3BF8: 409900D4  ble cr6, 0x82ea3ccc
	if !ctx.cr[6].gt {
	pc = 0x82EA3CCC; continue 'dispatch;
	}
	// 82EA3BFC: 817F0A1C  lwz r11, 0xa1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA3C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA3C04: 419A0010  beq cr6, 0x82ea3c14
	if ctx.cr[6].eq {
	pc = 0x82EA3C14; continue 'dispatch;
	}
	// 82EA3C08: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3C0C: 915F0A1C  stw r10, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[10].u32 ) };
	// 82EA3C10: 48000090  b 0x82ea3ca0
	pc = 0x82EA3CA0; continue 'dispatch;
	// 82EA3C14: 3F608332  lis r27, -0x7cce
	ctx.r[27].s64 = -2093875200;
	// 82EA3C18: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA3C1C: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA3C20: 817BBDBC  lwz r11, -0x4244(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA3C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA3C28: 4E800421  bctrl
	ctx.lr = 0x82EA3C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3C2C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA3C30: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA3C34: 409A0068  bne cr6, 0x82ea3c9c
	if !ctx.cr[6].eq {
	pc = 0x82EA3C9C; continue 'dispatch;
	}
	// 82EA3C38: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA3C3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA3C40: 419A0058  beq cr6, 0x82ea3c98
	if ctx.cr[6].eq {
	pc = 0x82EA3C98; continue 'dispatch;
	}
	// 82EA3C44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3C48: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA3C4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA3C50: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA3C54: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3C58: 4E800421  bctrl
	ctx.lr = 0x82EA3C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3C5C: 813BBDBC  lwz r9, -0x4244(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA3C60: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA3C64: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA3C68: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA3C6C: 4E800421  bctrl
	ctx.lr = 0x82EA3C70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3C70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA3C74: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA3C78: 409A0024  bne cr6, 0x82ea3c9c
	if !ctx.cr[6].eq {
	pc = 0x82EA3C9C; continue 'dispatch;
	}
	// 82EA3C7C: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA3C80: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA3C84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA3C88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3C8C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA3C90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3C94: 4E800421  bctrl
	ctx.lr = 0x82EA3C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3C98: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA3C9C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EA3CA0: 813F0A18  lwz r9, 0xa18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA3CA4: 394B0100  addi r10, r11, 0x100
	ctx.r[10].s64 = ctx.r[11].s64 + 256;
	// 82EA3CA8: 390A2000  addi r8, r10, 0x2000
	ctx.r[8].s64 = ctx.r[10].s64 + 8192;
	// 82EA3CAC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA3CB0: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA3CB4: 38E90001  addi r7, r9, 1
	ctx.r[7].s64 = ctx.r[9].s64 + 1;
	// 82EA3CB8: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	// 82EA3CBC: 915F0A20  stw r10, 0xa20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2592 as u32), ctx.r[10].u32 ) };
	// 82EA3CC0: 915F0A28  stw r10, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[10].u32 ) };
	// 82EA3CC4: 911F0A24  stw r8, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[8].u32 ) };
	// 82EA3CC8: 90FF0D64  stw r7, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[7].u32 ) };
	// 82EA3CCC: 83BF0A28  lwz r29, 0xa28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA3CD0: 7D7DF214  add r11, r29, r30
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82EA3CD4: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 82EA3CD8: 7CFAF82E  lwzx r7, r26, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3CDC: 2F1C0200  cmpwi cr6, r28, 0x200
	ctx.cr[6].compare_i32(ctx.r[28].s32, 512, &mut ctx.xer);
	// 82EA3CE0: 41980010  blt cr6, 0x82ea3cf0
	if ctx.cr[6].lt {
	pc = 0x82EA3CF0; continue 'dispatch;
	}
	// 82EA3CE4: 578A063E  clrlwi r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82EA3CE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA3CEC: 419A00B4  beq cr6, 0x82ea3da0
	if ctx.cr[6].eq {
	pc = 0x82EA3DA0; continue 'dispatch;
	}
	// 82EA3CF0: 815F0A24  lwz r10, 0xa24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA3CF4: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA3CF8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA3CFC: 40980030  bge cr6, 0x82ea3d2c
	if !ctx.cr[6].lt {
	pc = 0x82EA3D2C; continue 'dispatch;
	}
	// 82EA3D00: 7D5AF82E  lwzx r10, r26, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3D04: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82EA3D08: 7D39F82E  lwzx r9, r25, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3D0C: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 82EA3D10: 7D1AF92E  stwx r8, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u32) };
	// 82EA3D14: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA3D18: 7D79F92E  stwx r11, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82EA3D1C: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA3D20: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EA3D24: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 82EA3D28: 4BFFFFB4  b 0x82ea3cdc
	pc = 0x82EA3CDC; continue 'dispatch;
	// 82EA3D2C: 3518FFFF  addic. r8, r24, -1
	ctx.xer.ca = (ctx.r[24].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[24].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EA3D30: 40810070  ble 0x82ea3da0
	if !ctx.cr[0].gt {
	pc = 0x82EA3DA0; continue 'dispatch;
	}
	// 82EA3D34: 39680345  addi r11, r8, 0x345
	ctx.r[11].s64 = ctx.r[8].s64 + 837;
	// 82EA3D38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA3D3C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA3D40: 814BFD98  lwz r10, -0x268(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-616 as u32) ) } as u64;
	// 82EA3D44: 813F0A28  lwz r9, 0xa28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA3D48: 80DF0A24  lwz r6, 0xa24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA3D4C: 7CAA4A14  add r5, r10, r9
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA3D50: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82EA3D54: 40980040  bge cr6, 0x82ea3d94
	if !ctx.cr[6].lt {
	pc = 0x82EA3D94; continue 'dispatch;
	}
	// 82EA3D58: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3D5C: 80DF0A28  lwz r6, 0xa28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA3D60: 80ABFD54  lwz r5, -0x2ac(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-684 as u32) ) } as u64;
	// 82EA3D64: 3889FFFF  addi r4, r9, -1
	ctx.r[4].s64 = ctx.r[9].s64 + -1;
	// 82EA3D68: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EA3D6C: 90A60000  stw r5, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EA3D70: 90CBFD54  stw r6, -0x2ac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-684 as u32), ctx.r[6].u32 ) };
	// 82EA3D74: 813F0A28  lwz r9, 0xa28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA3D78: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA3D7C: 5469003E  slwi r9, r3, 0
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA3D80: 907F0A28  stw r3, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[3].u32 ) };
	// 82EA3D84: 80DF0A24  lwz r6, 0xa24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA3D88: 7CAA4A14  add r5, r10, r9
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA3D8C: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82EA3D90: 4198FFC8  blt cr6, 0x82ea3d58
	if ctx.cr[6].lt {
	pc = 0x82EA3D58; continue 'dispatch;
	}
	// 82EA3D94: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EA3D98: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82EA3D9C: 4181FFA4  bgt 0x82ea3d40
	if ctx.cr[0].gt {
	pc = 0x82EA3D40; continue 'dispatch;
	}
	// 82EA3DA0: 7CFAF92E  stwx r7, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u32) };
	// 82EA3DA4: 480001D4  b 0x82ea3f78
	pc = 0x82EA3F78; continue 'dispatch;
	// 82EA3DA8: 817F0A1C  lwz r11, 0xa1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA3DAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA3DB0: 419A0010  beq cr6, 0x82ea3dc0
	if ctx.cr[6].eq {
	pc = 0x82EA3DC0; continue 'dispatch;
	}
	// 82EA3DB4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3DB8: 915F0A1C  stw r10, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[10].u32 ) };
	// 82EA3DBC: 48000090  b 0x82ea3e4c
	pc = 0x82EA3E4C; continue 'dispatch;
	// 82EA3DC0: 3F808332  lis r28, -0x7cce
	ctx.r[28].s64 = -2093875200;
	// 82EA3DC4: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA3DC8: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA3DCC: 817CBDBC  lwz r11, -0x4244(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA3DD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA3DD4: 4E800421  bctrl
	ctx.lr = 0x82EA3DD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3DD8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA3DDC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA3DE0: 409A0068  bne cr6, 0x82ea3e48
	if !ctx.cr[6].eq {
	pc = 0x82EA3E48; continue 'dispatch;
	}
	// 82EA3DE4: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA3DE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA3DEC: 419A0058  beq cr6, 0x82ea3e44
	if ctx.cr[6].eq {
	pc = 0x82EA3E44; continue 'dispatch;
	}
	// 82EA3DF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3DF4: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA3DF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA3DFC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA3E00: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3E04: 4E800421  bctrl
	ctx.lr = 0x82EA3E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3E08: 813CBDBC  lwz r9, -0x4244(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA3E0C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA3E10: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA3E14: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA3E18: 4E800421  bctrl
	ctx.lr = 0x82EA3E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3E1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA3E20: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA3E24: 409A0024  bne cr6, 0x82ea3e48
	if !ctx.cr[6].eq {
	pc = 0x82EA3E48; continue 'dispatch;
	}
	// 82EA3E28: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA3E2C: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA3E30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA3E34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3E38: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA3E3C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3E40: 4E800421  bctrl
	ctx.lr = 0x82EA3E44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3E44: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA3E48: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EA3E4C: 815F0A18  lwz r10, 0xa18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA3E50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA3E54: 419A0018  beq cr6, 0x82ea3e6c
	if ctx.cr[6].eq {
	pc = 0x82EA3E6C; continue 'dispatch;
	}
	// 82EA3E58: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3E5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA3E60: 813F0A18  lwz r9, 0xa18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA3E64: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA3E68: 48000010  b 0x82ea3e78
	pc = 0x82EA3E78; continue 'dispatch;
	// 82EA3E6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA3E70: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA3E74: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	// 82EA3E78: 815F0D64  lwz r10, 0xd64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA3E7C: 3BAB0100  addi r29, r11, 0x100
	ctx.r[29].s64 = ctx.r[11].s64 + 256;
	// 82EA3E80: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82EA3E84: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA3E88: 7D7DF214  add r11, r29, r30
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82EA3E8C: 915F0D64  stw r10, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[10].u32 ) };
	// 82EA3E90: 7D1AF82E  lwzx r8, r26, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3E94: 40980030  bge cr6, 0x82ea3ec4
	if !ctx.cr[6].lt {
	pc = 0x82EA3EC4; continue 'dispatch;
	}
	// 82EA3E98: 7D5D5850  subf r10, r29, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EA3E9C: 7D3AF82E  lwzx r9, r26, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3EA0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82EA3EA4: 7CF9F82E  lwzx r7, r25, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3EA8: 38C9FFFF  addi r6, r9, -1
	ctx.r[6].s64 = ctx.r[9].s64 + -1;
	// 82EA3EAC: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82EA3EB0: 7CDAF92E  stwx r6, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u32) };
	// 82EA3EB4: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EA3EB8: 7D79F92E  stwx r11, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82EA3EBC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA3EC0: 4198FFDC  blt cr6, 0x82ea3e9c
	if ctx.cr[6].lt {
	pc = 0x82EA3E9C; continue 'dispatch;
	}
	// 82EA3EC4: 7D1AF92E  stwx r8, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u32) };
	// 82EA3EC8: 480000B0  b 0x82ea3f78
	pc = 0x82EA3F78; continue 'dispatch;
	// 82EA3ECC: 815F0D58  lwz r10, 0xd58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3416 as u32) ) } as u64;
	// 82EA3ED0: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA3ED4: 813F0D60  lwz r9, 0xd60(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3424 as u32) ) } as u64;
	// 82EA3ED8: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82EA3EDC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA3EE0: 911F0D58  stw r8, 0xd58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3416 as u32), ctx.r[8].u32 ) };
	// 82EA3EE4: 917F0D5C  stw r11, 0xd5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3420 as u32), ctx.r[11].u32 ) };
	// 82EA3EE8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EA3EEC: 40990008  ble cr6, 0x82ea3ef4
	if !ctx.cr[6].gt {
	pc = 0x82EA3EF4; continue 'dispatch;
	}
	// 82EA3EF0: 917F0D60  stw r11, 0xd60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3424 as u32), ctx.r[11].u32 ) };
	// 82EA3EF4: 3F808332  lis r28, -0x7cce
	ctx.r[28].s64 = -2093875200;
	// 82EA3EF8: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA3EFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA3F00: 817CBDBC  lwz r11, -0x4244(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA3F04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA3F08: 4E800421  bctrl
	ctx.lr = 0x82EA3F0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3F0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA3F10: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA3F14: 409A0064  bne cr6, 0x82ea3f78
	if !ctx.cr[6].eq {
	pc = 0x82EA3F78; continue 'dispatch;
	}
	// 82EA3F18: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA3F1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA3F20: 419A0058  beq cr6, 0x82ea3f78
	if ctx.cr[6].eq {
	pc = 0x82EA3F78; continue 'dispatch;
	}
	// 82EA3F24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3F28: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA3F2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA3F30: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA3F34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3F38: 4E800421  bctrl
	ctx.lr = 0x82EA3F3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3F3C: 813CBDBC  lwz r9, -0x4244(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA3F40: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA3F44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA3F48: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA3F4C: 4E800421  bctrl
	ctx.lr = 0x82EA3F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3F50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA3F54: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA3F58: 409A0020  bne cr6, 0x82ea3f78
	if !ctx.cr[6].eq {
	pc = 0x82EA3F78; continue 'dispatch;
	}
	// 82EA3F5C: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA3F60: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA3F64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA3F68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA3F6C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA3F70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA3F74: 4E800421  bctrl
	ctx.lr = 0x82EA3F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA3F78: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA3F7C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82EA3F80: F9770020  std r11, 0x20(r23)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[23].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA3F84: 4839E9D9  bl 0x8324295c
	ctx.lr = 0x82EA3F88;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA3F88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA3F8C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EA3F90: 48304214  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA3F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA3F98 size=1020
    let mut pc: u32 = 0x82EA3F98;
    'dispatch: loop {
        match pc {
            0x82EA3F98 => {
    //   block [0x82EA3F98..0x82EA4394)
	// 82EA3F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA3F9C: 483041AD  bl 0x831a8148
	ctx.lr = 0x82EA3FA0;
	sub_831A8130(ctx, base);
	// 82EA3FA0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA3FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA3FA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA3FAC: 3A9F0A30  addi r20, r31, 0xa30
	ctx.r[20].s64 = ctx.r[31].s64 + 2608;
	// 82EA3FB0: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82EA3FB4: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82EA3FB8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EA3FBC: 4BFFE0F5  bl 0x82ea20b0
	ctx.lr = 0x82EA3FC0;
	sub_82EA20B0(ctx, base);
	// 82EA3FC0: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82EA3FC4: 41990014  bgt cr6, 0x82ea3fd8
	if ctx.cr[6].gt {
	pc = 0x82EA3FD8; continue 'dispatch;
	}
	// 82EA3FC8: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82EA3FCC: 894B0AF0  lbz r10, 0xaf0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82EA3FD0: 7D5A0774  extsb r26, r10
	ctx.r[26].s64 = ctx.r[10].s8 as i64;
	// 82EA3FD4: 48000018  b 0x82ea3fec
	pc = 0x82EA3FEC; continue 'dispatch;
	// 82EA3FD8: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82EA3FDC: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82EA3FE0: 394B033D  addi r10, r11, 0x33d
	ctx.r[10].s64 = ctx.r[11].s64 + 829;
	// 82EA3FE4: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA3FE8: 7F49F82E  lwzx r26, r9, r31
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3FEC: 397A029A  addi r11, r26, 0x29a
	ctx.r[11].s64 = ctx.r[26].s64 + 666;
	// 82EA3FF0: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82EA3FF4: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82EA3FF8: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA3FFC: 4099034C  ble cr6, 0x82ea4348
	if !ctx.cr[6].gt {
	pc = 0x82EA4348; continue 'dispatch;
	}
	// 82EA4000: 7FB8EB78  mr r24, r29
	ctx.r[24].u64 = ctx.r[29].u64;
	// 82EA4004: 7ED5B378  mr r21, r22
	ctx.r[21].u64 = ctx.r[22].u64;
	// 82EA4008: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82EA400C: 3F208332  lis r25, -0x7cce
	ctx.r[25].s64 = -2093875200;
	// 82EA4010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA4014: 419A0010  beq cr6, 0x82ea4024
	if ctx.cr[6].eq {
	pc = 0x82EA4024; continue 'dispatch;
	}
	// 82EA4018: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA401C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4020: 4800031C  b 0x82ea433c
	pc = 0x82EA433C; continue 'dispatch;
	// 82EA4024: 2F1A000D  cmpwi cr6, r26, 0xd
	ctx.cr[6].compare_i32(ctx.r[26].s32, 13, &mut ctx.xer);
	// 82EA4028: 7EFBF92E  stwx r23, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[23].u32) };
	// 82EA402C: 395A02AB  addi r10, r26, 0x2ab
	ctx.r[10].s64 = ctx.r[26].s64 + 683;
	// 82EA4030: 409801D8  bge cr6, 0x82ea4208
	if !ctx.cr[6].lt {
	pc = 0x82EA4208; continue 'dispatch;
	}
	// 82EA4034: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA4038: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA403C: 813F0A24  lwz r9, 0xa24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA4040: 7FC8F82E  lwzx r30, r8, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4044: 7CFE5A14  add r7, r30, r11
	ctx.r[7].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EA4048: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82EA404C: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA4050: 409900D0  ble cr6, 0x82ea4120
	if !ctx.cr[6].gt {
	pc = 0x82EA4120; continue 'dispatch;
	}
	// 82EA4054: 817F0A1C  lwz r11, 0xa1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA4058: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA405C: 419A0010  beq cr6, 0x82ea406c
	if ctx.cr[6].eq {
	pc = 0x82EA406C; continue 'dispatch;
	}
	// 82EA4060: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4064: 915F0A1C  stw r10, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[10].u32 ) };
	// 82EA4068: 4800008C  b 0x82ea40f4
	pc = 0x82EA40F4; continue 'dispatch;
	// 82EA406C: 8179BDBC  lwz r11, -0x4244(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA4070: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA4074: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA4078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA407C: 4E800421  bctrl
	ctx.lr = 0x82EA4080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4080: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA4084: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA4088: 409A0068  bne cr6, 0x82ea40f0
	if !ctx.cr[6].eq {
	pc = 0x82EA40F0; continue 'dispatch;
	}
	// 82EA408C: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA4090: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA4094: 419A0058  beq cr6, 0x82ea40ec
	if ctx.cr[6].eq {
	pc = 0x82EA40EC; continue 'dispatch;
	}
	// 82EA4098: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA409C: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA40A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA40A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA40A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA40AC: 4E800421  bctrl
	ctx.lr = 0x82EA40B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA40B0: 8139BDBC  lwz r9, -0x4244(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA40B4: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA40B8: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA40BC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA40C0: 4E800421  bctrl
	ctx.lr = 0x82EA40C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA40C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA40C8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA40CC: 409A0024  bne cr6, 0x82ea40f0
	if !ctx.cr[6].eq {
	pc = 0x82EA40F0; continue 'dispatch;
	}
	// 82EA40D0: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA40D4: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA40D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA40DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA40E0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA40E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA40E8: 4E800421  bctrl
	ctx.lr = 0x82EA40EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA40EC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA40F0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EA40F4: 813F0A18  lwz r9, 0xa18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA40F8: 394B0100  addi r10, r11, 0x100
	ctx.r[10].s64 = ctx.r[11].s64 + 256;
	// 82EA40FC: 390A2000  addi r8, r10, 0x2000
	ctx.r[8].s64 = ctx.r[10].s64 + 8192;
	// 82EA4100: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA4104: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA4108: 38E90001  addi r7, r9, 1
	ctx.r[7].s64 = ctx.r[9].s64 + 1;
	// 82EA410C: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	// 82EA4110: 915F0A20  stw r10, 0xa20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2592 as u32), ctx.r[10].u32 ) };
	// 82EA4114: 915F0A28  stw r10, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[10].u32 ) };
	// 82EA4118: 911F0A24  stw r8, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[8].u32 ) };
	// 82EA411C: 90FF0D64  stw r7, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[7].u32 ) };
	// 82EA4120: 397A0345  addi r11, r26, 0x345
	ctx.r[11].s64 = ctx.r[26].s64 + 837;
	// 82EA4124: 80DF0A28  lwz r6, 0xa28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA4128: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA412C: 7D66F214  add r11, r6, r30
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[30].u64;
	// 82EA4130: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 82EA4134: 7CA8F82E  lwzx r5, r8, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4138: 2F1C0200  cmpwi cr6, r28, 0x200
	ctx.cr[6].compare_i32(ctx.r[28].s32, 512, &mut ctx.xer);
	// 82EA413C: 41980010  blt cr6, 0x82ea414c
	if ctx.cr[6].lt {
	pc = 0x82EA414C; continue 'dispatch;
	}
	// 82EA4140: 578A063E  clrlwi r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82EA4144: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA4148: 419A00B4  beq cr6, 0x82ea41fc
	if ctx.cr[6].eq {
	pc = 0x82EA41FC; continue 'dispatch;
	}
	// 82EA414C: 815F0A24  lwz r10, 0xa24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA4150: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA4154: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA4158: 40980030  bge cr6, 0x82ea4188
	if !ctx.cr[6].lt {
	pc = 0x82EA4188; continue 'dispatch;
	}
	// 82EA415C: 7D48F82E  lwzx r10, r8, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4160: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82EA4164: 7D3BF82E  lwzx r9, r27, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4168: 38EAFFFF  addi r7, r10, -1
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	// 82EA416C: 7CE8F92E  stwx r7, r8, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u32) };
	// 82EA4170: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA4174: 7D7BF92E  stwx r11, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82EA4178: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA417C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EA4180: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 82EA4184: 4BFFFFB4  b 0x82ea4138
	pc = 0x82EA4138; continue 'dispatch;
	// 82EA4188: 34FAFFFF  addic. r7, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EA418C: 40810070  ble 0x82ea41fc
	if !ctx.cr[0].gt {
	pc = 0x82EA41FC; continue 'dispatch;
	}
	// 82EA4190: 39670345  addi r11, r7, 0x345
	ctx.r[11].s64 = ctx.r[7].s64 + 837;
	// 82EA4194: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA4198: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA419C: 814BFD98  lwz r10, -0x268(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-616 as u32) ) } as u64;
	// 82EA41A0: 813F0A28  lwz r9, 0xa28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA41A4: 809F0A24  lwz r4, 0xa24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA41A8: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA41AC: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EA41B0: 40980040  bge cr6, 0x82ea41f0
	if !ctx.cr[6].lt {
	pc = 0x82EA41F0; continue 'dispatch;
	}
	// 82EA41B4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA41B8: 809F0A28  lwz r4, 0xa28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA41BC: 806BFD54  lwz r3, -0x2ac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-684 as u32) ) } as u64;
	// 82EA41C0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82EA41C4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA41C8: 90640000  stw r3, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EA41CC: 908BFD54  stw r4, -0x2ac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-684 as u32), ctx.r[4].u32 ) };
	// 82EA41D0: 813F0A28  lwz r9, 0xa28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82EA41D4: 7C8A4A14  add r4, r10, r9
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA41D8: 5489003E  slwi r9, r4, 0
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA41DC: 909F0A28  stw r4, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[4].u32 ) };
	// 82EA41E0: 807F0A24  lwz r3, 0xa24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82EA41E4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA41E8: 7F091840  cmplw cr6, r9, r3
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EA41EC: 4198FFC8  blt cr6, 0x82ea41b4
	if ctx.cr[6].lt {
	pc = 0x82EA41B4; continue 'dispatch;
	}
	// 82EA41F0: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EA41F4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82EA41F8: 4181FFA4  bgt 0x82ea419c
	if ctx.cr[0].gt {
	pc = 0x82EA419C; continue 'dispatch;
	}
	// 82EA41FC: 7CA8F92E  stwx r5, r8, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u32) };
	// 82EA4200: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82EA4204: 48000130  b 0x82ea4334
	pc = 0x82EA4334; continue 'dispatch;
	// 82EA4208: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA420C: 817F0A1C  lwz r11, 0xa1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA4210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA4214: 7FC9F82E  lwzx r30, r9, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4218: 419A0010  beq cr6, 0x82ea4228
	if ctx.cr[6].eq {
	pc = 0x82EA4228; continue 'dispatch;
	}
	// 82EA421C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4220: 915F0A1C  stw r10, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[10].u32 ) };
	// 82EA4224: 4800008C  b 0x82ea42b0
	pc = 0x82EA42B0; continue 'dispatch;
	// 82EA4228: 8179BDBC  lwz r11, -0x4244(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA422C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA4230: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA4234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA4238: 4E800421  bctrl
	ctx.lr = 0x82EA423C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA423C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA4240: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA4244: 409A0068  bne cr6, 0x82ea42ac
	if !ctx.cr[6].eq {
	pc = 0x82EA42AC; continue 'dispatch;
	}
	// 82EA4248: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA424C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA4250: 419A0058  beq cr6, 0x82ea42a8
	if ctx.cr[6].eq {
	pc = 0x82EA42A8; continue 'dispatch;
	}
	// 82EA4254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4258: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA425C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA4260: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA4264: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA4268: 4E800421  bctrl
	ctx.lr = 0x82EA426C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA426C: 8139BDBC  lwz r9, -0x4244(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 82EA4270: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82EA4274: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82EA4278: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA427C: 4E800421  bctrl
	ctx.lr = 0x82EA4280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4280: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA4284: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA4288: 409A0024  bne cr6, 0x82ea42ac
	if !ctx.cr[6].eq {
	pc = 0x82EA42AC; continue 'dispatch;
	}
	// 82EA428C: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82EA4290: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82EA4294: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA4298: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA429C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA42A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA42A4: 4E800421  bctrl
	ctx.lr = 0x82EA42A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA42A8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA42AC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EA42B0: 815F0A18  lwz r10, 0xa18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA42B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA42B8: 419A0018  beq cr6, 0x82ea42d0
	if ctx.cr[6].eq {
	pc = 0x82EA42D0; continue 'dispatch;
	}
	// 82EA42BC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA42C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA42C4: 813F0A18  lwz r9, 0xa18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA42C8: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA42CC: 4800000C  b 0x82ea42d8
	pc = 0x82EA42D8; continue 'dispatch;
	// 82EA42D0: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82EA42D4: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	// 82EA42D8: 395A0345  addi r10, r26, 0x345
	ctx.r[10].s64 = ctx.r[26].s64 + 837;
	// 82EA42DC: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA42E0: 38EB0100  addi r7, r11, 0x100
	ctx.r[7].s64 = ctx.r[11].s64 + 256;
	// 82EA42E4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA42E8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EA42EC: 7D67F214  add r11, r7, r30
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82EA42F0: 913F0D64  stw r9, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[9].u32 ) };
	// 82EA42F4: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82EA42F8: 7CCAF82E  lwzx r6, r10, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA42FC: 40980030  bge cr6, 0x82ea432c
	if !ctx.cr[6].lt {
	pc = 0x82EA432C; continue 'dispatch;
	}
	// 82EA4300: 7D275850  subf r9, r7, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82EA4304: 7D0AF82E  lwzx r8, r10, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4308: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82EA430C: 7CBBF82E  lwzx r5, r27, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4310: 3888FFFF  addi r4, r8, -1
	ctx.r[4].s64 = ctx.r[8].s64 + -1;
	// 82EA4314: 2F092000  cmpwi cr6, r9, 0x2000
	ctx.cr[6].compare_i32(ctx.r[9].s32, 8192, &mut ctx.xer);
	// 82EA4318: 7C8AF92E  stwx r4, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[4].u32) };
	// 82EA431C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EA4320: 7D7BF92E  stwx r11, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82EA4324: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA4328: 4198FFDC  blt cr6, 0x82ea4304
	if ctx.cr[6].lt {
	pc = 0x82EA4304; continue 'dispatch;
	}
	// 82EA432C: 7CCAF92E  stwx r6, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u32) };
	// 82EA4330: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82EA4334: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA4338: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA433C: 36B5FFFF  addic. r21, r21, -1
	ctx.xer.ca = (ctx.r[21].u32 > (!(-1 as u32)));
	ctx.r[21].s64 = ctx.r[21].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82EA4340: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 82EA4344: 4082FCCC  bne 0x82ea4010
	if !ctx.cr[0].eq {
	pc = 0x82EA4010; continue 'dispatch;
	}
	// 82EA4348: 395A0345  addi r10, r26, 0x345
	ctx.r[10].s64 = ctx.r[26].s64 + 837;
	// 82EA434C: 7D7BF92E  stwx r11, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82EA4350: 393A02AB  addi r9, r26, 0x2ab
	ctx.r[9].s64 = ctx.r[26].s64 + 683;
	// 82EA4354: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA4358: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA435C: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82EA4360: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82EA4364: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4368: 7CC8F82E  lwzx r6, r8, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA436C: 7CA9B214  add r5, r9, r22
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[22].u64;
	// 82EA4370: 7D46B1D6  mullw r10, r6, r22
	ctx.r[10].s64 = (ctx.r[6].s32 as i64) * (ctx.r[22].s32 as i64);
	// 82EA4374: 7CABF92E  stwx r5, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u32) };
	// 82EA4378: 817F0D70  lwz r11, 0xd70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 82EA437C: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EA4380: 909F0D70  stw r4, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[4].u32 ) };
	// 82EA4384: F8F40020  std r7, 0x20(r20)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[20].u32.wrapping_add(32 as u32), ctx.r[7].u64 ) };
	// 82EA4388: 4839E5D5  bl 0x8324295c
	ctx.lr = 0x82EA438C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA438C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82EA4390: 48303E08  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4398 size=220
    let mut pc: u32 = 0x82EA4398;
    'dispatch: loop {
        match pc {
            0x82EA4398 => {
    //   block [0x82EA4398..0x82EA4474)
	// 82EA4398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA439C: 48303DCD  bl 0x831a8168
	ctx.lr = 0x82EA43A0;
	sub_831A8130(ctx, base);
	// 82EA43A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA43A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA43A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA43AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA43B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EA43B4: 419A00B8  beq cr6, 0x82ea446c
	if ctx.cr[6].eq {
	pc = 0x82EA446C; continue 'dispatch;
	}
	// 82EA43B8: 3B9F0A30  addi r28, r31, 0xa30
	ctx.r[28].s64 = ctx.r[31].s64 + 2608;
	// 82EA43BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA43C0: 4BFFDCF1  bl 0x82ea20b0
	ctx.lr = 0x82EA43C4;
	sub_82EA20B0(ctx, base);
	// 82EA43C4: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82EA43C8: 41990074  bgt cr6, 0x82ea443c
	if ctx.cr[6].gt {
	pc = 0x82EA443C; continue 'dispatch;
	}
	// 82EA43CC: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82EA43D0: 41990014  bgt cr6, 0x82ea43e4
	if ctx.cr[6].gt {
	pc = 0x82EA43E4; continue 'dispatch;
	}
	// 82EA43D4: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82EA43D8: 894B0AF0  lbz r10, 0xaf0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82EA43DC: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82EA43E0: 48000018  b 0x82ea43f8
	pc = 0x82EA43F8; continue 'dispatch;
	// 82EA43E4: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82EA43E8: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82EA43EC: 394B033D  addi r10, r11, 0x33d
	ctx.r[10].s64 = ctx.r[11].s64 + 829;
	// 82EA43F0: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA43F4: 7D69F82E  lwzx r11, r9, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA43F8: 394B02AB  addi r10, r11, 0x2ab
	ctx.r[10].s64 = ctx.r[11].s64 + 683;
	// 82EA43FC: 813F0D70  lwz r9, 0xd70(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 82EA4400: 390B029A  addi r8, r11, 0x29a
	ctx.r[8].s64 = ctx.r[11].s64 + 666;
	// 82EA4404: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA4408: 38CB0345  addi r6, r11, 0x345
	ctx.r[6].s64 = ctx.r[11].s64 + 837;
	// 82EA440C: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA4410: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA4414: 7CA7F82E  lwzx r5, r7, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4418: 7C854850  subf r4, r5, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[5].s64;
	// 82EA441C: 909F0D70  stw r4, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[4].u32 ) };
	// 82EA4420: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4424: 7D2AF82E  lwzx r9, r10, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4428: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82EA442C: 7D2AF92E  stwx r9, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	// 82EA4430: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EA4434: 7FABF92E  stwx r29, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82EA4438: 48000024  b 0x82ea445c
	pc = 0x82EA445C; continue 'dispatch;
	// 82EA443C: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82EA4440: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82EA4444: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA4448: 7D3E5850  subf r9, r30, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82EA444C: 913F0D5C  stw r9, 0xd5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3420 as u32), ctx.r[9].u32 ) };
	// 82EA4450: 810ABDC0  lwz r8, -0x4240(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EA4454: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA4458: 4E800421  bctrl
	ctx.lr = 0x82EA445C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA445C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA4460: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA4464: F97C0020  std r11, 0x20(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA4468: 4839E4F5  bl 0x8324295c
	ctx.lr = 0x82EA446C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA446C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA4470: 48303D48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4478 size=216
    let mut pc: u32 = 0x82EA4478;
    'dispatch: loop {
        match pc {
            0x82EA4478 => {
    //   block [0x82EA4478..0x82EA4550)
	// 82EA4478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA447C: 48303CE9  bl 0x831a8164
	ctx.lr = 0x82EA4480;
	sub_831A8130(ctx, base);
	// 82EA4480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA4484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA4488: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA448C: 3B7F0A30  addi r27, r31, 0xa30
	ctx.r[27].s64 = ctx.r[31].s64 + 2608;
	// 82EA4490: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA4494: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EA4498: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EA449C: 4BFFDC15  bl 0x82ea20b0
	ctx.lr = 0x82EA44A0;
	sub_82EA20B0(ctx, base);
	// 82EA44A0: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82EA44A4: 41990014  bgt cr6, 0x82ea44b8
	if ctx.cr[6].gt {
	pc = 0x82EA44B8; continue 'dispatch;
	}
	// 82EA44A8: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82EA44AC: 894B0AF0  lbz r10, 0xaf0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82EA44B0: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 82EA44B4: 48000018  b 0x82ea44cc
	pc = 0x82EA44CC; continue 'dispatch;
	// 82EA44B8: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82EA44BC: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82EA44C0: 394B033D  addi r10, r11, 0x33d
	ctx.r[10].s64 = ctx.r[11].s64 + 829;
	// 82EA44C4: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA44C8: 7D09F82E  lwzx r8, r9, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA44CC: 396802AB  addi r11, r8, 0x2ab
	ctx.r[11].s64 = ctx.r[8].s64 + 683;
	// 82EA44D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EA44D4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA44D8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EA44DC: 7CAAF82E  lwzx r5, r10, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA44E0: 4099004C  ble cr6, 0x82ea452c
	if !ctx.cr[6].gt {
	pc = 0x82EA452C; continue 'dispatch;
	}
	// 82EA44E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA44E8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA44EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA44F0: 419A0030  beq cr6, 0x82ea4520
	if ctx.cr[6].eq {
	pc = 0x82EA4520; continue 'dispatch;
	}
	// 82EA44F4: 39480345  addi r10, r8, 0x345
	ctx.r[10].s64 = ctx.r[8].s64 + 837;
	// 82EA44F8: 38E8029A  addi r7, r8, 0x29a
	ctx.r[7].s64 = ctx.r[8].s64 + 666;
	// 82EA44FC: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA4500: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA4504: 7CC62A14  add r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82EA4508: 7CE9F82E  lwzx r7, r9, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA450C: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA4510: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82EA4514: 7CE9F92E  stwx r7, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u32) };
	// 82EA4518: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EA451C: 7D6AF92E  stwx r11, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82EA4520: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EA4524: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82EA4528: 4082FFC0  bne 0x82ea44e8
	if !ctx.cr[0].eq {
	pc = 0x82EA44E8; continue 'dispatch;
	}
	// 82EA452C: 817F0D70  lwz r11, 0xd70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 82EA4530: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EA4534: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EA4538: 7D265850  subf r9, r6, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82EA453C: 913F0D70  stw r9, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[9].u32 ) };
	// 82EA4540: F95B0020  std r10, 0x20(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82EA4544: 4839E419  bl 0x8324295c
	ctx.lr = 0x82EA4548;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA4548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA454C: 48303C68  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4550 size=148
    let mut pc: u32 = 0x82EA4550;
    'dispatch: loop {
        match pc {
            0x82EA4550 => {
    //   block [0x82EA4550..0x82EA45E4)
	// 82EA4550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA4554: 48303C11  bl 0x831a8164
	ctx.lr = 0x82EA4558;
	sub_831A8130(ctx, base);
	// 82EA4558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA455C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA4560: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA4564: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 82EA4568: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EA456C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA4570: 4BFFDB41  bl 0x82ea20b0
	ctx.lr = 0x82EA4574;
	sub_82EA20B0(ctx, base);
	// 82EA4574: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA4578: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EA457C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA4580: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EA4584: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA4588: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82EA458C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EA4590: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA4594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA4598: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA459C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA45A0: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 82EA45A4: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82EA45A8: 993E0018  stb r9, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 82EA45AC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA45B0: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA45B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA45B8: 4E800421  bctrl
	ctx.lr = 0x82EA45BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA45BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA45C0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EA45C4: 907E0020  stw r3, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82EA45C8: 937E0024  stw r27, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 82EA45CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA45D0: 995E0028  stb r10, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82EA45D4: F93F0A50  std r9, 0xa50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2640 as u32), ctx.r[9].u64 ) };
	// 82EA45D8: 4839E385  bl 0x8324295c
	ctx.lr = 0x82EA45DC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA45DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA45E0: 48303BD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA45E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA45E8 size=144
    let mut pc: u32 = 0x82EA45E8;
    'dispatch: loop {
        match pc {
            0x82EA45E8 => {
    //   block [0x82EA45E8..0x82EA4678)
	// 82EA45E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA45EC: 48303B7D  bl 0x831a8168
	ctx.lr = 0x82EA45F0;
	sub_831A8130(ctx, base);
	// 82EA45F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA45F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA45F8: 3B9F0A30  addi r28, r31, 0xa30
	ctx.r[28].s64 = ctx.r[31].s64 + 2608;
	// 82EA45FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA4600: 4BFFDAB1  bl 0x82ea20b0
	ctx.lr = 0x82EA4604;
	sub_82EA20B0(ctx, base);
	// 82EA4604: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA4608: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EA460C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA4610: 40990048  ble cr6, 0x82ea4658
	if !ctx.cr[6].gt {
	pc = 0x82EA4658; continue 'dispatch;
	}
	// 82EA4614: 3BDF0024  addi r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 + 36;
	// 82EA4618: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA461C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA4620: 409A0024  bne cr6, 0x82ea4644
	if !ctx.cr[6].eq {
	pc = 0x82EA4644; continue 'dispatch;
	}
	// 82EA4624: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA462C: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4630: 80BEFFF8  lwz r5, -8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA4634: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EA4638: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA463C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA4640: 4E800421  bctrl
	ctx.lr = 0x82EA4644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4644: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA4648: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EA464C: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 82EA4650: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA4654: 4198FFC4  blt cr6, 0x82ea4618
	if ctx.cr[6].lt {
	pc = 0x82EA4618; continue 'dispatch;
	}
	// 82EA4658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA465C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EA4660: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EA4664: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA4668: F95C0020  std r10, 0x20(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82EA466C: 4839E2F1  bl 0x8324295c
	ctx.lr = 0x82EA4670;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA4670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA4674: 48303B44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4678 size=584
    let mut pc: u32 = 0x82EA4678;
    'dispatch: loop {
        match pc {
            0x82EA4678 => {
    //   block [0x82EA4678..0x82EA48C0)
	// 82EA4678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA467C: 48303AE1  bl 0x831a815c
	ctx.lr = 0x82EA4680;
	sub_831A8130(ctx, base);
	// 82EA4680: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA4684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA4688: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA468C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EA4690: 2F1F2000  cmpwi cr6, r31, 0x2000
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8192, &mut ctx.xer);
	// 82EA4694: 4098001C  bge cr6, 0x82ea46b0
	if !ctx.cr[6].lt {
	pc = 0x82EA46B0; continue 'dispatch;
	}
	// 82EA4698: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA469C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA46A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA46A4: 4BFFC08D  bl 0x82ea0730
	ctx.lr = 0x82EA46A8;
	sub_82EA0730(ctx, base);
	// 82EA46A8: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82EA46AC: 48303B00  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82EA46B0: 3B5E0A30  addi r26, r30, 0xa30
	ctx.r[26].s64 = ctx.r[30].s64 + 2608;
	// 82EA46B4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EA46B8: 4BFFD9F9  bl 0x82ea20b0
	ctx.lr = 0x82EA46BC;
	sub_82EA20B0(ctx, base);
	// 82EA46BC: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA46C0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EA46C4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EA46C8: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EA46CC: 4099005C  ble cr6, 0x82ea4728
	if !ctx.cr[6].gt {
	pc = 0x82EA4728; continue 'dispatch;
	}
	// 82EA46D0: 54E8003E  slwi r8, r7, 0
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA46D4: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 82EA46D8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA46DC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82EA46E0: 409A0028  bne cr6, 0x82ea4708
	if !ctx.cr[6].eq {
	pc = 0x82EA4708; continue 'dispatch;
	}
	// 82EA46E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA46E8: 7F0AF800  cmpw cr6, r10, r31
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EA46EC: 4198001C  blt cr6, 0x82ea4708
	if ctx.cr[6].lt {
	pc = 0x82EA4708; continue 'dispatch;
	}
	// 82EA46F0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA46F4: 419A0010  beq cr6, 0x82ea4704
	if ctx.cr[6].eq {
	pc = 0x82EA4704; continue 'dispatch;
	}
	// 82EA46F8: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA46FC: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82EA4700: 40980008  bge cr6, 0x82ea4708
	if !ctx.cr[6].lt {
	pc = 0x82EA4708; continue 'dispatch;
	}
	// 82EA4704: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82EA4708: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EA470C: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82EA4710: 4082FFC8  bne 0x82ea46d8
	if !ctx.cr[0].eq {
	pc = 0x82EA46D8; continue 'dispatch;
	}
	// 82EA4714: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EA4718: 419A0010  beq cr6, 0x82ea4728
	if ctx.cr[6].eq {
	pc = 0x82EA4728; continue 'dispatch;
	}
	// 82EA471C: 83E90008  lwz r31, 8(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA4720: 9B690000  stb r27, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82EA4724: 48000180  b 0x82ea48a4
	pc = 0x82EA48A4; continue 'dispatch;
	// 82EA4728: 357F0040  addic. r11, r31, 0x40
	ctx.xer.ca = (ctx.r[31].u32 > (!(64 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA472C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA4730: 40810010  ble 0x82ea4740
	if !ctx.cr[0].gt {
	pc = 0x82EA4740; continue 'dispatch;
	}
	// 82EA4734: 7D6B0E71  srawi. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA4738: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA473C: 4181FFF8  bgt 0x82ea4734
	if ctx.cr[0].gt {
	pc = 0x82EA4734; continue 'dispatch;
	}
	// 82EA4740: 3BAAFFC0  addi r29, r10, -0x40
	ctx.r[29].s64 = ctx.r[10].s64 + -64;
	// 82EA4744: 2F070080  cmpwi cr6, r7, 0x80
	ctx.cr[6].compare_i32(ctx.r[7].s32, 128, &mut ctx.xer);
	// 82EA4748: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82EA474C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82EA4750: 409800C8  bge cr6, 0x82ea4818
	if !ctx.cr[6].lt {
	pc = 0x82EA4818; continue 'dispatch;
	}
	// 82EA4754: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA4758: 48001C39  bl 0x82ea6390
	ctx.lr = 0x82EA475C;
	sub_82EA6390(ctx, base);
	// 82EA475C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA4760: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA4764: 388BECDC  addi r4, r11, -0x1324
	ctx.r[4].s64 = ctx.r[11].s64 + -4900;
	// 82EA4768: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA476C: 3B2AEC9C  addi r25, r10, -0x1364
	ctx.r[25].s64 = ctx.r[10].s64 + -4964;
	// 82EA4770: 48001069  bl 0x82ea57d8
	ctx.lr = 0x82EA4774;
	sub_82EA57D8(ctx, base);
	// 82EA4774: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA4778: 480011B1  bl 0x82ea5928
	ctx.lr = 0x82EA477C;
	sub_82EA5928(ctx, base);
	// 82EA477C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EA4780: 48001059  bl 0x82ea57d8
	ctx.lr = 0x82EA4784;
	sub_82EA57D8(ctx, base);
	// 82EA4784: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82EA4788: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82EA478C: 3CA0AF55  lis r5, -0x50ab
	ctx.r[5].s64 = -1353383936;
	// 82EA4790: 390003B6  li r8, 0x3b6
	ctx.r[8].s64 = 950;
	// 82EA4794: 38E7EC74  addi r7, r7, -0x138c
	ctx.r[7].s64 = ctx.r[7].s64 + -5004;
	// 82EA4798: 80696E5C  lwz r3, 0x6e5c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82EA479C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82EA47A0: 60A5ADDE  ori r5, r5, 0xadde
	ctx.r[5].u64 = ctx.r[5].u64 | 44510;
	// 82EA47A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA47A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA47AC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA47B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA47B4: 4E800421  bctrl
	ctx.lr = 0x82EA47B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA47B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA47BC: 4800165D  bl 0x82ea5e18
	ctx.lr = 0x82EA47C0;
	sub_82EA5E18(ctx, base);
	// 82EA47C0: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA47C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA47C8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA47CC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EA47D0: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA47D4: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EA47D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA47DC: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA47E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA47E4: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA47E8: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82EA47EC: 9B7F0018  stb r27, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82EA47F0: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82EA47F4: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA47F8: 80C7000C  lwz r6, 0xc(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA47FC: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82EA4800: 4E800421  bctrl
	ctx.lr = 0x82EA4804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4804: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82EA4808: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82EA480C: 9B7F0028  stb r27, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82EA4810: 83FF0020  lwz r31, 0x20(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA4814: 48000090  b 0x82ea48a4
	pc = 0x82EA48A4; continue 'dispatch;
	// 82EA4818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA481C: 48001B75  bl 0x82ea6390
	ctx.lr = 0x82EA4820;
	sub_82EA6390(ctx, base);
	// 82EA4820: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA4824: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA4828: 388BECDC  addi r4, r11, -0x1324
	ctx.r[4].s64 = ctx.r[11].s64 + -4900;
	// 82EA482C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA4830: 3BAAEC18  addi r29, r10, -0x13e8
	ctx.r[29].s64 = ctx.r[10].s64 + -5096;
	// 82EA4834: 48000FA5  bl 0x82ea57d8
	ctx.lr = 0x82EA4838;
	sub_82EA57D8(ctx, base);
	// 82EA4838: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA483C: 480010ED  bl 0x82ea5928
	ctx.lr = 0x82EA4840;
	sub_82EA5928(ctx, base);
	// 82EA4840: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA4844: 48000F95  bl 0x82ea57d8
	ctx.lr = 0x82EA4848;
	sub_82EA57D8(ctx, base);
	// 82EA4848: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82EA484C: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82EA4850: 3CA0AF55  lis r5, -0x50ab
	ctx.r[5].s64 = -1353383936;
	// 82EA4854: 390003C3  li r8, 0x3c3
	ctx.r[8].s64 = 963;
	// 82EA4858: 38E7EC74  addi r7, r7, -0x138c
	ctx.r[7].s64 = ctx.r[7].s64 + -5004;
	// 82EA485C: 80696E5C  lwz r3, 0x6e5c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82EA4860: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82EA4864: 60A5ADDE  ori r5, r5, 0xadde
	ctx.r[5].u64 = ctx.r[5].u64 | 44510;
	// 82EA4868: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA486C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4870: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA4874: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA4878: 4E800421  bctrl
	ctx.lr = 0x82EA487C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA487C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA4880: 48001599  bl 0x82ea5e18
	ctx.lr = 0x82EA4884;
	sub_82EA5E18(ctx, base);
	// 82EA4884: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4888: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA488C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA4890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4894: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA4898: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA489C: 4E800421  bctrl
	ctx.lr = 0x82EA48A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA48A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA48A4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA48A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EA48AC: F97A0020  std r11, 0x20(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA48B0: 4839E0AD  bl 0x8324295c
	ctx.lr = 0x82EA48B4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA48B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA48B8: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82EA48BC: 483038F0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA48C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA48C0 size=300
    let mut pc: u32 = 0x82EA48C0;
    'dispatch: loop {
        match pc {
            0x82EA48C0 => {
    //   block [0x82EA48C0..0x82EA49EC)
	// 82EA48C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA48C4: 483038A1  bl 0x831a8164
	ctx.lr = 0x82EA48C8;
	sub_831A8130(ctx, base);
	// 82EA48C8: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA48CC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EA48D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA48D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA48D8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EA48DC: 2F1C2000  cmpwi cr6, r28, 0x2000
	ctx.cr[6].compare_i32(ctx.r[28].s32, 8192, &mut ctx.xer);
	// 82EA48E0: 4098001C  bge cr6, 0x82ea48fc
	if !ctx.cr[6].lt {
	pc = 0x82EA48FC; continue 'dispatch;
	}
	// 82EA48E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA48E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA48EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA48F0: 4BFFBEC1  bl 0x82ea07b0
	ctx.lr = 0x82EA48F4;
	sub_82EA07B0(ctx, base);
	// 82EA48F4: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82EA48F8: 483038BC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EA48FC: 3BDF0A30  addi r30, r31, 0xa30
	ctx.r[30].s64 = ctx.r[31].s64 + 2608;
	// 82EA4900: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4904: 4BFFD7AD  bl 0x82ea20b0
	ctx.lr = 0x82EA4908;
	sub_82EA20B0(ctx, base);
	// 82EA4908: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA490C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA4910: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA4914: 40990024  ble cr6, 0x82ea4938
	if !ctx.cr[6].gt {
	pc = 0x82EA4938; continue 'dispatch;
	}
	// 82EA4918: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82EA491C: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA4920: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82EA4924: 419A00A8  beq cr6, 0x82ea49cc
	if ctx.cr[6].eq {
	pc = 0x82EA49CC; continue 'dispatch;
	}
	// 82EA4928: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA492C: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82EA4930: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EA4934: 4198FFE8  blt cr6, 0x82ea491c
	if ctx.cr[6].lt {
	pc = 0x82EA491C; continue 'dispatch;
	}
	// 82EA4938: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82EA493C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EA4940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA4944: 48001A4D  bl 0x82ea6390
	ctx.lr = 0x82EA4948;
	sub_82EA6390(ctx, base);
	// 82EA4948: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA494C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA4950: 388BECF8  addi r4, r11, -0x1308
	ctx.r[4].s64 = ctx.r[11].s64 + -4872;
	// 82EA4954: 48000E85  bl 0x82ea57d8
	ctx.lr = 0x82EA4958;
	sub_82EA57D8(ctx, base);
	// 82EA4958: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82EA495C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA4960: 3CA0AF55  lis r5, -0x50ab
	ctx.r[5].s64 = -1353383936;
	// 82EA4964: 390003DF  li r8, 0x3df
	ctx.r[8].s64 = 991;
	// 82EA4968: 38E9EC74  addi r7, r9, -0x138c
	ctx.r[7].s64 = ctx.r[9].s64 + -5004;
	// 82EA496C: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82EA4970: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EA4974: 60A5ADDF  ori r5, r5, 0xaddf
	ctx.r[5].u64 = ctx.r[5].u64 | 44511;
	// 82EA4978: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA497C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4980: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA4984: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA4988: 4E800421  bctrl
	ctx.lr = 0x82EA498C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA498C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA4990: 48001489  bl 0x82ea5e18
	ctx.lr = 0x82EA4994;
	sub_82EA5E18(ctx, base);
	// 82EA4994: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4998: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EA499C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EA49A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA49A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA49A8: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA49AC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EA49B0: 4E800421  bctrl
	ctx.lr = 0x82EA49B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA49B4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82EA49B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA49BC: F8FE0020  std r7, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[7].u64 ) };
	// 82EA49C0: 4839DF9D  bl 0x8324295c
	ctx.lr = 0x82EA49C4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA49C4: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82EA49C8: 483037EC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EA49CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA49D0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EA49D4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EA49D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA49DC: F93E0020  std r9, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[9].u64 ) };
	// 82EA49E0: 4839DF7D  bl 0x8324295c
	ctx.lr = 0x82EA49E4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA49E4: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82EA49E8: 483037CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA49F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA49F0 size=120
    let mut pc: u32 = 0x82EA49F0;
    'dispatch: loop {
        match pc {
            0x82EA49F0 => {
    //   block [0x82EA49F0..0x82EA4A68)
	// 82EA49F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA49F4: 48303771  bl 0x831a8164
	ctx.lr = 0x82EA49F8;
	sub_831A8130(ctx, base);
	// 82EA49F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA49FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA4A00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA4A04: 3BDF0A30  addi r30, r31, 0xa30
	ctx.r[30].s64 = ctx.r[31].s64 + 2608;
	// 82EA4A08: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EA4A0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4A10: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EA4A14: 4BFFD69D  bl 0x82ea20b0
	ctx.lr = 0x82EA4A18;
	sub_82EA20B0(ctx, base);
	// 82EA4A18: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA4A1C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EA4A20: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA4A24: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EA4A28: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA4A2C: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82EA4A30: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82EA4A34: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA4A38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4A3C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA4A40: 394B0018  addi r10, r11, 0x18
	ctx.r[10].s64 = ctx.r[11].s64 + 24;
	// 82EA4A44: 992B0018  stb r9, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 82EA4A48: 938B001C  stw r28, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82EA4A4C: 93AB0020  stw r29, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82EA4A50: 936B0024  stw r27, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 82EA4A54: 992B0028  stb r9, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 82EA4A58: F8DF0A50  std r6, 0xa50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2640 as u32), ctx.r[6].u64 ) };
	// 82EA4A5C: 4839DF01  bl 0x8324295c
	ctx.lr = 0x82EA4A60;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA4A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA4A64: 48303750  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4A68 size=108
    let mut pc: u32 = 0x82EA4A68;
    'dispatch: loop {
        match pc {
            0x82EA4A68 => {
    //   block [0x82EA4A68..0x82EA4AD4)
	// 82EA4A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA4A6C: 48303701  bl 0x831a816c
	ctx.lr = 0x82EA4A70;
	sub_831A8130(ctx, base);
	// 82EA4A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA4A74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA4A78: 3BBE0A30  addi r29, r30, 0xa30
	ctx.r[29].s64 = ctx.r[30].s64 + 2608;
	// 82EA4A7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA4A80: 4BFFD631  bl 0x82ea20b0
	ctx.lr = 0x82EA4A84;
	sub_82EA20B0(ctx, base);
	// 82EA4A84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA4A88: 83FE0A1C  lwz r31, 0xa1c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA4A8C: 917E0A1C  stw r11, 0xa1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(2588 as u32), ctx.r[11].u32 ) };
	// 82EA4A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA4A94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA4A98: 419A0024  beq cr6, 0x82ea4abc
	if ctx.cr[6].eq {
	pc = 0x82EA4ABC; continue 'dispatch;
	}
	// 82EA4A9C: 3FC08332  lis r30, -0x7cce
	ctx.r[30].s64 = -2093875200;
	// 82EA4AA0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4AA4: 817EBDC0  lwz r11, -0x4240(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EA4AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA4AAC: 4E800421  bctrl
	ctx.lr = 0x82EA4AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA4AB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA4AB8: 409AFFE8  bne cr6, 0x82ea4aa0
	if !ctx.cr[6].eq {
	pc = 0x82EA4AA0; continue 'dispatch;
	}
	// 82EA4ABC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA4AC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA4AC4: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA4AC8: 4839DE95  bl 0x8324295c
	ctx.lr = 0x82EA4ACC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA4ACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA4AD0: 483036EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4AD8 size=860
    let mut pc: u32 = 0x82EA4AD8;
    'dispatch: loop {
        match pc {
            0x82EA4AD8 => {
    //   block [0x82EA4AD8..0x82EA4E34)
	// 82EA4AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA4ADC: 48303689  bl 0x831a8164
	ctx.lr = 0x82EA4AE0;
	sub_831A8130(ctx, base);
	// 82EA4AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA4AE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA4AE8: 4BFFDC69  bl 0x82ea2750
	ctx.lr = 0x82EA4AEC;
	sub_82EA2750(ctx, base);
	// 82EA4AEC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA4AF0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EA4AF4: 394BED24  addi r10, r11, -0x12dc
	ctx.r[10].s64 = ctx.r[11].s64 + -4828;
	// 82EA4AF8: 39200FA0  li r9, 0xfa0
	ctx.r[9].s64 = 4000;
	// 82EA4AFC: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA4B00: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA4B04: 913C0A58  stw r9, 0xa58(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2648 as u32), ctx.r[9].u32 ) };
	// 82EA4B08: 3BFC0A30  addi r31, r28, 0xa30
	ctx.r[31].s64 = ctx.r[28].s64 + 2608;
	// 82EA4B0C: 937C0A5C  stw r27, 0xa5c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2652 as u32), ctx.r[27].u32 ) };
	// 82EA4B10: 3BCB70B0  addi r30, r11, 0x70b0
	ctx.r[30].s64 = ctx.r[11].s64 + 28848;
	// 82EA4B14: 937C0A60  stw r27, 0xa60(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2656 as u32), ctx.r[27].u32 ) };
	// 82EA4B18: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 82EA4B1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4B20: 4BFFD591  bl 0x82ea20b0
	ctx.lr = 0x82EA4B24;
	sub_82EA20B0(ctx, base);
	// 82EA4B24: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EA4B28: 917C0A60  stw r11, 0xa60(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2656 as u32), ctx.r[11].u32 ) };
	// 82EA4B2C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA4B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA4B34: 419A0008  beq cr6, 0x82ea4b3c
	if ctx.cr[6].eq {
	pc = 0x82EA4B3C; continue 'dispatch;
	}
	// 82EA4B38: 93EB002C  stw r31, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 82EA4B3C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA4B40: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EA4B44: 93FE0030  stw r31, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82EA4B48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4B4C: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA4B50: 4839DE0D  bl 0x8324295c
	ctx.lr = 0x82EA4B54;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA4B54: 38800FA0  li r4, 0xfa0
	ctx.r[4].s64 = 4000;
	// 82EA4B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA4B5C: 4839E2F1  bl 0x83242e4c
	ctx.lr = 0x82EA4B60;
	// extern call 0x83242E4C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82EA4B60: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA4B64: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82EA4B68: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA4B6C: 397C0D54  addi r11, r28, 0xd54
	ctx.r[11].s64 = ctx.r[28].s64 + 3412;
	// 82EA4B70: 937C0A20  stw r27, 0xa20(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2592 as u32), ctx.r[27].u32 ) };
	// 82EA4B74: 937C0A24  stw r27, 0xa24(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2596 as u32), ctx.r[27].u32 ) };
	// 82EA4B78: 937C0A28  stw r27, 0xa28(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2600 as u32), ctx.r[27].u32 ) };
	// 82EA4B7C: 937C0A1C  stw r27, 0xa1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2588 as u32), ctx.r[27].u32 ) };
	// 82EA4B80: 937C0A18  stw r27, 0xa18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2584 as u32), ctx.r[27].u32 ) };
	// 82EA4B84: 937C0D74  stw r27, 0xd74(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3444 as u32), ctx.r[27].u32 ) };
	// 82EA4B88: 937C0D7C  stw r27, 0xd7c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3452 as u32), ctx.r[27].u32 ) };
	// 82EA4B8C: 937C0D78  stw r27, 0xd78(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3448 as u32), ctx.r[27].u32 ) };
	// 82EA4B90: 936BFD54  stw r27, -0x2ac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-684 as u32), ctx.r[27].u32 ) };
	// 82EA4B94: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA4B98: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EA4B9C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82EA4BA0: 4080FFF0  bge 0x82ea4b90
	if !ctx.cr[0].lt {
	pc = 0x82EA4B90; continue 'dispatch;
	}
	// 82EA4BA4: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82EA4BA8: 393C0AF0  addi r9, r28, 0xaf0
	ctx.r[9].s64 = ctx.r[28].s64 + 2800;
	// 82EA4BAC: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82EA4BB0: 4199000C  bgt cr6, 0x82ea4bbc
	if ctx.cr[6].gt {
	pc = 0x82EA4BBC; continue 'dispatch;
	}
	// 82EA4BB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA4BB8: 480000FC  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4BBC: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82EA4BC0: 4199000C  bgt cr6, 0x82ea4bcc
	if ctx.cr[6].gt {
	pc = 0x82EA4BCC; continue 'dispatch;
	}
	// 82EA4BC4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EA4BC8: 480000EC  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4BCC: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82EA4BD0: 4199000C  bgt cr6, 0x82ea4bdc
	if ctx.cr[6].gt {
	pc = 0x82EA4BDC; continue 'dispatch;
	}
	// 82EA4BD4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82EA4BD8: 480000DC  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4BDC: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 82EA4BE0: 4199000C  bgt cr6, 0x82ea4bec
	if ctx.cr[6].gt {
	pc = 0x82EA4BEC; continue 'dispatch;
	}
	// 82EA4BE4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EA4BE8: 480000CC  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4BEC: 2F0A0040  cmpwi cr6, r10, 0x40
	ctx.cr[6].compare_i32(ctx.r[10].s32, 64, &mut ctx.xer);
	// 82EA4BF0: 4199000C  bgt cr6, 0x82ea4bfc
	if ctx.cr[6].gt {
	pc = 0x82EA4BFC; continue 'dispatch;
	}
	// 82EA4BF4: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82EA4BF8: 480000BC  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4BFC: 2F0A0060  cmpwi cr6, r10, 0x60
	ctx.cr[6].compare_i32(ctx.r[10].s32, 96, &mut ctx.xer);
	// 82EA4C00: 4199000C  bgt cr6, 0x82ea4c0c
	if ctx.cr[6].gt {
	pc = 0x82EA4C0C; continue 'dispatch;
	}
	// 82EA4C04: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82EA4C08: 480000AC  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C0C: 2F0A0080  cmpwi cr6, r10, 0x80
	ctx.cr[6].compare_i32(ctx.r[10].s32, 128, &mut ctx.xer);
	// 82EA4C10: 4199000C  bgt cr6, 0x82ea4c1c
	if ctx.cr[6].gt {
	pc = 0x82EA4C1C; continue 'dispatch;
	}
	// 82EA4C14: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82EA4C18: 4800009C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C1C: 2F0A00A0  cmpwi cr6, r10, 0xa0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 160, &mut ctx.xer);
	// 82EA4C20: 4199000C  bgt cr6, 0x82ea4c2c
	if ctx.cr[6].gt {
	pc = 0x82EA4C2C; continue 'dispatch;
	}
	// 82EA4C24: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82EA4C28: 4800008C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C2C: 2F0A00C0  cmpwi cr6, r10, 0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 192, &mut ctx.xer);
	// 82EA4C30: 4199000C  bgt cr6, 0x82ea4c3c
	if ctx.cr[6].gt {
	pc = 0x82EA4C3C; continue 'dispatch;
	}
	// 82EA4C34: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82EA4C38: 4800007C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C3C: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 82EA4C40: 4199000C  bgt cr6, 0x82ea4c4c
	if ctx.cr[6].gt {
	pc = 0x82EA4C4C; continue 'dispatch;
	}
	// 82EA4C44: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82EA4C48: 4800006C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C4C: 2F0A0140  cmpwi cr6, r10, 0x140
	ctx.cr[6].compare_i32(ctx.r[10].s32, 320, &mut ctx.xer);
	// 82EA4C50: 4199000C  bgt cr6, 0x82ea4c5c
	if ctx.cr[6].gt {
	pc = 0x82EA4C5C; continue 'dispatch;
	}
	// 82EA4C54: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82EA4C58: 4800005C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C5C: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 82EA4C60: 4199000C  bgt cr6, 0x82ea4c6c
	if ctx.cr[6].gt {
	pc = 0x82EA4C6C; continue 'dispatch;
	}
	// 82EA4C64: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82EA4C68: 4800004C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C6C: 2F0A0400  cmpwi cr6, r10, 0x400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1024, &mut ctx.xer);
	// 82EA4C70: 4199000C  bgt cr6, 0x82ea4c7c
	if ctx.cr[6].gt {
	pc = 0x82EA4C7C; continue 'dispatch;
	}
	// 82EA4C74: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82EA4C78: 4800003C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C7C: 2F0A0800  cmpwi cr6, r10, 0x800
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2048, &mut ctx.xer);
	// 82EA4C80: 4199000C  bgt cr6, 0x82ea4c8c
	if ctx.cr[6].gt {
	pc = 0x82EA4C8C; continue 'dispatch;
	}
	// 82EA4C84: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 82EA4C88: 4800002C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C8C: 2F0A1000  cmpwi cr6, r10, 0x1000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4096, &mut ctx.xer);
	// 82EA4C90: 4199000C  bgt cr6, 0x82ea4c9c
	if ctx.cr[6].gt {
	pc = 0x82EA4C9C; continue 'dispatch;
	}
	// 82EA4C94: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82EA4C98: 4800001C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4C9C: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82EA4CA0: 4199000C  bgt cr6, 0x82ea4cac
	if ctx.cr[6].gt {
	pc = 0x82EA4CAC; continue 'dispatch;
	}
	// 82EA4CA4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82EA4CA8: 4800000C  b 0x82ea4cb4
	pc = 0x82EA4CB4; continue 'dispatch;
	// 82EA4CAC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA4CB0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA4CB4: 390B02AB  addi r8, r11, 0x2ab
	ctx.r[8].s64 = ctx.r[11].s64 + 683;
	// 82EA4CB8: 7D6951AE  stbx r11, r9, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82EA4CBC: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EA4CC0: 7D46E12E  stwx r10, r6, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82EA4CC4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA4CC8: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 82EA4CCC: 4099FEE0  ble cr6, 0x82ea4bac
	if !ctx.cr[6].gt {
	pc = 0x82EA4BAC; continue 'dispatch;
	}
	// 82EA4CD0: 39400400  li r10, 0x400
	ctx.r[10].s64 = 1024;
	// 82EA4CD4: 393C0CF4  addi r9, r28, 0xcf4
	ctx.r[9].s64 = ctx.r[28].s64 + 3316;
	// 82EA4CD8: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82EA4CDC: 4199000C  bgt cr6, 0x82ea4ce8
	if ctx.cr[6].gt {
	pc = 0x82EA4CE8; continue 'dispatch;
	}
	// 82EA4CE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA4CE4: 480000FC  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4CE8: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82EA4CEC: 4199000C  bgt cr6, 0x82ea4cf8
	if ctx.cr[6].gt {
	pc = 0x82EA4CF8; continue 'dispatch;
	}
	// 82EA4CF0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EA4CF4: 480000EC  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4CF8: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82EA4CFC: 4199000C  bgt cr6, 0x82ea4d08
	if ctx.cr[6].gt {
	pc = 0x82EA4D08; continue 'dispatch;
	}
	// 82EA4D00: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82EA4D04: 480000DC  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D08: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 82EA4D0C: 4199000C  bgt cr6, 0x82ea4d18
	if ctx.cr[6].gt {
	pc = 0x82EA4D18; continue 'dispatch;
	}
	// 82EA4D10: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EA4D14: 480000CC  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D18: 2F0A0040  cmpwi cr6, r10, 0x40
	ctx.cr[6].compare_i32(ctx.r[10].s32, 64, &mut ctx.xer);
	// 82EA4D1C: 4199000C  bgt cr6, 0x82ea4d28
	if ctx.cr[6].gt {
	pc = 0x82EA4D28; continue 'dispatch;
	}
	// 82EA4D20: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82EA4D24: 480000BC  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D28: 2F0A0060  cmpwi cr6, r10, 0x60
	ctx.cr[6].compare_i32(ctx.r[10].s32, 96, &mut ctx.xer);
	// 82EA4D2C: 4199000C  bgt cr6, 0x82ea4d38
	if ctx.cr[6].gt {
	pc = 0x82EA4D38; continue 'dispatch;
	}
	// 82EA4D30: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82EA4D34: 480000AC  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D38: 2F0A0080  cmpwi cr6, r10, 0x80
	ctx.cr[6].compare_i32(ctx.r[10].s32, 128, &mut ctx.xer);
	// 82EA4D3C: 4199000C  bgt cr6, 0x82ea4d48
	if ctx.cr[6].gt {
	pc = 0x82EA4D48; continue 'dispatch;
	}
	// 82EA4D40: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82EA4D44: 4800009C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D48: 2F0A00A0  cmpwi cr6, r10, 0xa0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 160, &mut ctx.xer);
	// 82EA4D4C: 4199000C  bgt cr6, 0x82ea4d58
	if ctx.cr[6].gt {
	pc = 0x82EA4D58; continue 'dispatch;
	}
	// 82EA4D50: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82EA4D54: 4800008C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D58: 2F0A00C0  cmpwi cr6, r10, 0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 192, &mut ctx.xer);
	// 82EA4D5C: 4199000C  bgt cr6, 0x82ea4d68
	if ctx.cr[6].gt {
	pc = 0x82EA4D68; continue 'dispatch;
	}
	// 82EA4D60: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82EA4D64: 4800007C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D68: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 82EA4D6C: 4199000C  bgt cr6, 0x82ea4d78
	if ctx.cr[6].gt {
	pc = 0x82EA4D78; continue 'dispatch;
	}
	// 82EA4D70: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82EA4D74: 4800006C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D78: 2F0A0140  cmpwi cr6, r10, 0x140
	ctx.cr[6].compare_i32(ctx.r[10].s32, 320, &mut ctx.xer);
	// 82EA4D7C: 4199000C  bgt cr6, 0x82ea4d88
	if ctx.cr[6].gt {
	pc = 0x82EA4D88; continue 'dispatch;
	}
	// 82EA4D80: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82EA4D84: 4800005C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D88: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 82EA4D8C: 4199000C  bgt cr6, 0x82ea4d98
	if ctx.cr[6].gt {
	pc = 0x82EA4D98; continue 'dispatch;
	}
	// 82EA4D90: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82EA4D94: 4800004C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4D98: 2F0A0400  cmpwi cr6, r10, 0x400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1024, &mut ctx.xer);
	// 82EA4D9C: 4199000C  bgt cr6, 0x82ea4da8
	if ctx.cr[6].gt {
	pc = 0x82EA4DA8; continue 'dispatch;
	}
	// 82EA4DA0: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82EA4DA4: 4800003C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4DA8: 2F0A0800  cmpwi cr6, r10, 0x800
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2048, &mut ctx.xer);
	// 82EA4DAC: 4199000C  bgt cr6, 0x82ea4db8
	if ctx.cr[6].gt {
	pc = 0x82EA4DB8; continue 'dispatch;
	}
	// 82EA4DB0: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 82EA4DB4: 4800002C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4DB8: 2F0A1000  cmpwi cr6, r10, 0x1000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4096, &mut ctx.xer);
	// 82EA4DBC: 4199000C  bgt cr6, 0x82ea4dc8
	if ctx.cr[6].gt {
	pc = 0x82EA4DC8; continue 'dispatch;
	}
	// 82EA4DC0: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82EA4DC4: 4800001C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4DC8: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82EA4DCC: 4199000C  bgt cr6, 0x82ea4dd8
	if ctx.cr[6].gt {
	pc = 0x82EA4DD8; continue 'dispatch;
	}
	// 82EA4DD0: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82EA4DD4: 4800000C  b 0x82ea4de0
	pc = 0x82EA4DE0; continue 'dispatch;
	// 82EA4DD8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EA4DDC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA4DE0: 390B02AB  addi r8, r11, 0x2ab
	ctx.r[8].s64 = ctx.r[11].s64 + 683;
	// 82EA4DE4: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA4DE8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82EA4DEC: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA4DF0: 7D47E12E  stwx r10, r7, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82EA4DF4: 394A0400  addi r10, r10, 0x400
	ctx.r[10].s64 = ctx.r[10].s64 + 1024;
	// 82EA4DF8: 2F0A2400  cmpwi cr6, r10, 0x2400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 9216, &mut ctx.xer);
	// 82EA4DFC: 4198FEDC  blt cr6, 0x82ea4cd8
	if ctx.cr[6].lt {
	pc = 0x82EA4CD8; continue 'dispatch;
	}
	// 82EA4E00: 39602000  li r11, 0x2000
	ctx.r[11].s64 = 8192;
	// 82EA4E04: 937C0D58  stw r27, 0xd58(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3416 as u32), ctx.r[27].u32 ) };
	// 82EA4E08: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 82EA4E0C: 937C0D5C  stw r27, 0xd5c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3420 as u32), ctx.r[27].u32 ) };
	// 82EA4E10: 937C0D60  stw r27, 0xd60(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3424 as u32), ctx.r[27].u32 ) };
	// 82EA4E14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA4E18: 937C0D64  stw r27, 0xd64(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3428 as u32), ctx.r[27].u32 ) };
	// 82EA4E1C: 917C0D68  stw r11, 0xd68(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3432 as u32), ctx.r[11].u32 ) };
	// 82EA4E20: 915C0D6C  stw r10, 0xd6c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3436 as u32), ctx.r[10].u32 ) };
	// 82EA4E24: 937C0D70  stw r27, 0xd70(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3440 as u32), ctx.r[27].u32 ) };
	// 82EA4E28: 937C0014  stw r27, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82EA4E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA4E30: 48303384  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4E38 size=212
    let mut pc: u32 = 0x82EA4E38;
    'dispatch: loop {
        match pc {
            0x82EA4E38 => {
    //   block [0x82EA4E38..0x82EA4F0C)
	// 82EA4E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA4E3C: 48303329  bl 0x831a8164
	ctx.lr = 0x82EA4E40;
	sub_831A8130(ctx, base);
	// 82EA4E40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA4E44: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EA4E48: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA4E4C: 394BED24  addi r10, r11, -0x12dc
	ctx.r[10].s64 = ctx.r[11].s64 + -4828;
	// 82EA4E50: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA4E54: 4BFFF795  bl 0x82ea45e8
	ctx.lr = 0x82EA4E58;
	sub_82EA45E8(ctx, base);
	// 82EA4E58: 813B0A18  lwz r9, 0xa18(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA4E5C: 811B0A1C  lwz r8, 0xa1c(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA4E60: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82EA4E64: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 82EA4E68: 3FA08332  lis r29, -0x7cce
	ctx.r[29].s64 = -2093875200;
	// 82EA4E6C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82EA4E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82EA4E74: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4E78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA4E7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA4E80: 419A0020  beq cr6, 0x82ea4ea0
	if ctx.cr[6].eq {
	pc = 0x82EA4EA0; continue 'dispatch;
	}
	// 82EA4E84: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4E88: 817DBDC0  lwz r11, -0x4240(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EA4E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA4E90: 4E800421  bctrl
	ctx.lr = 0x82EA4E94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4E94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA4E98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EA4E9C: 409AFFE8  bne cr6, 0x82ea4e84
	if !ctx.cr[6].eq {
	pc = 0x82EA4E84; continue 'dispatch;
	}
	// 82EA4EA0: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EA4EA4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EA4EA8: 4082FFCC  bne 0x82ea4e74
	if !ctx.cr[0].eq {
	pc = 0x82EA4E74; continue 'dispatch;
	}
	// 82EA4EAC: 817B0A5C  lwz r11, 0xa5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2652 as u32) ) } as u64;
	// 82EA4EB0: 3BFB0A58  addi r31, r27, 0xa58
	ctx.r[31].s64 = ctx.r[27].s64 + 2648;
	// 82EA4EB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA4EB8: 419A0044  beq cr6, 0x82ea4efc
	if ctx.cr[6].eq {
	pc = 0x82EA4EFC; continue 'dispatch;
	}
	// 82EA4EBC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA4EC0: 3BCB70B0  addi r30, r11, 0x70b0
	ctx.r[30].s64 = ctx.r[11].s64 + 28848;
	// 82EA4EC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4EC8: 4BFFD1E9  bl 0x82ea20b0
	ctx.lr = 0x82EA4ECC;
	sub_82EA20B0(ctx, base);
	// 82EA4ECC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA4ED0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA4ED4: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82EA4ED8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA4EDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA4EE0: 419A000C  beq cr6, 0x82ea4eec
	if ctx.cr[6].eq {
	pc = 0x82EA4EEC; continue 'dispatch;
	}
	// 82EA4EE4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA4EE8: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82EA4EEC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA4EF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA4EF4: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA4EF8: 4839DA65  bl 0x8324295c
	ctx.lr = 0x82EA4EFC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA4EFC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EA4F00: 4BFFD779  bl 0x82ea2678
	ctx.lr = 0x82EA4F04;
	sub_82EA2678(ctx, base);
	// 82EA4F04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA4F08: 483032AC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA4F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA4F10 size=760
    let mut pc: u32 = 0x82EA4F10;
    'dispatch: loop {
        match pc {
            0x82EA4F10 => {
    //   block [0x82EA4F10..0x82EA5208)
	// 82EA4F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA4F14: 48303241  bl 0x831a8154
	ctx.lr = 0x82EA4F18;
	sub_831A8130(ctx, base);
	// 82EA4F18: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA4F1C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EA4F20: 3AFB0A30  addi r23, r27, 0xa30
	ctx.r[23].s64 = ctx.r[27].s64 + 2608;
	// 82EA4F24: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82EA4F28: 4BFFD189  bl 0x82ea20b0
	ctx.lr = 0x82EA4F2C;
	sub_82EA20B0(ctx, base);
	// 82EA4F2C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4F30: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 82EA4F34: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EA4F38: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA4F3C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EA4F40: 83BB0D64  lwz r29, 0xd64(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA4F44: 391D0004  addi r8, r29, 4
	ctx.r[8].s64 = ctx.r[29].s64 + 4;
	// 82EA4F48: 551F1036  rlwinm r31, r8, 2, 0, 0x1b
	ctx.r[31].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA4F4C: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA4F50: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA4F54: 41990010  bgt cr6, 0x82ea4f64
	if ctx.cr[6].gt {
	pc = 0x82EA4F64; continue 'dispatch;
	}
	// 82EA4F58: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EA4F5C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82EA4F60: 4800001C  b 0x82ea4f7c
	pc = 0x82EA4F7C; continue 'dispatch;
	// 82EA4F64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4F68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA4F6C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA4F70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA4F74: 4E800421  bctrl
	ctx.lr = 0x82EA4F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4F78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA4F7C: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EA4F80: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA4F84: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EA4F88: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA4F8C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA4F90: 41990010  bgt cr6, 0x82ea4fa0
	if ctx.cr[6].gt {
	pc = 0x82EA4FA0; continue 'dispatch;
	}
	// 82EA4F94: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EA4F98: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82EA4F9C: 4800001C  b 0x82ea4fb8
	pc = 0x82EA4FB8; continue 'dispatch;
	// 82EA4FA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA4FA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA4FA8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA4FAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA4FB0: 4E800421  bctrl
	ctx.lr = 0x82EA4FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA4FB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA4FB8: 3B5DFFFF  addi r26, r29, -1
	ctx.r[26].s64 = ctx.r[29].s64 + -1;
	// 82EA4FBC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EA4FC0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82EA4FC4: 41980024  blt cr6, 0x82ea4fe8
	if ctx.cr[6].lt {
	pc = 0x82EA4FE8; continue 'dispatch;
	}
	// 82EA4FC8: 355A0001  addic. r10, r26, 1
	ctx.xer.ca = (ctx.r[26].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[26].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA4FCC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EA4FD0: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82EA4FD4: 41820014  beq 0x82ea4fe8
	if ctx.cr[0].eq {
	pc = 0x82EA4FE8; continue 'dispatch;
	}
	// 82EA4FD8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA4FDC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA4FE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA4FE4: 4200FFF8  bdnz 0x82ea4fdc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EA4FDC; continue 'dispatch;
	}
	// 82EA4FE8: 817B0A18  lwz r11, 0xa18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA4FEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA4FF0: 419A001C  beq cr6, 0x82ea500c
	if ctx.cr[6].eq {
	pc = 0x82EA500C; continue 'dispatch;
	}
	// 82EA4FF4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82EA4FF8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA4FFC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EA5000: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5004: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA5008: 409AFFF0  bne cr6, 0x82ea4ff8
	if !ctx.cr[6].eq {
	pc = 0x82EA4FF8; continue 'dispatch;
	}
	// 82EA500C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA5010: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82EA5014: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82EA5018: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA501C: 40990014  ble cr6, 0x82ea5030
	if !ctx.cr[6].gt {
	pc = 0x82EA5030; continue 'dispatch;
	}
	// 82EA5020: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EA5024: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA5028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA502C: 480001DD  bl 0x82ea5208
	ctx.lr = 0x82EA5030;
	sub_82EA5208(ctx, base);
	// 82EA5030: 38DB0A6C  addi r6, r27, 0xa6c
	ctx.r[6].s64 = ctx.r[27].s64 + 2668;
	// 82EA5034: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EA5038: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82EA503C: 81050000  lwz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5040: 80E50044  lwz r7, 0x44(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EA5044: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EA5048: 419A0060  beq cr6, 0x82ea50a8
	if ctx.cr[6].eq {
	pc = 0x82EA50A8; continue 'dispatch;
	}
	// 82EA504C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82EA5050: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82EA5054: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82EA5058: 40990034  ble cr6, 0x82ea508c
	if !ctx.cr[6].gt {
	pc = 0x82EA508C; continue 'dispatch;
	}
	// 82EA505C: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EA5060: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82EA5064: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EA5068: 7C63F02E  lwzx r3, r3, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA506C: 7F081840  cmplw cr6, r8, r3
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EA5070: 4098000C  bge cr6, 0x82ea507c
	if !ctx.cr[6].lt {
	pc = 0x82EA507C; continue 'dispatch;
	}
	// 82EA5074: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82EA5078: 48000008  b 0x82ea5080
	pc = 0x82EA5080; continue 'dispatch;
	// 82EA507C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82EA5080: 7D6A4850  subf r11, r10, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82EA5084: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EA5088: 4199FFD4  bgt cr6, 0x82ea505c
	if ctx.cr[6].gt {
	pc = 0x82EA505C; continue 'dispatch;
	}
	// 82EA508C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA5090: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA5094: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82EA5098: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82EA509C: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA50A0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EA50A4: 409AFFA8  bne cr6, 0x82ea504c
	if !ctx.cr[6].eq {
	pc = 0x82EA504C; continue 'dispatch;
	}
	// 82EA50A8: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EA50AC: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 82EA50B0: 4082FF8C  bne 0x82ea503c
	if !ctx.cr[0].eq {
	pc = 0x82EA503C; continue 'dispatch;
	}
	// 82EA50B4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82EA50B8: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA50BC: 93860000  stw r28, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EA50C0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EA50C4: 419A0070  beq cr6, 0x82ea5134
	if ctx.cr[6].eq {
	pc = 0x82EA5134; continue 'dispatch;
	}
	// 82EA50C8: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA50CC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82EA50D0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82EA50D4: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82EA50D8: 40990034  ble cr6, 0x82ea510c
	if !ctx.cr[6].gt {
	pc = 0x82EA510C; continue 'dispatch;
	}
	// 82EA50DC: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EA50E0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82EA50E4: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EA50E8: 7C64F02E  lwzx r3, r4, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA50EC: 7F081840  cmplw cr6, r8, r3
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EA50F0: 4098000C  bge cr6, 0x82ea50fc
	if !ctx.cr[6].lt {
	pc = 0x82EA50FC; continue 'dispatch;
	}
	// 82EA50F4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82EA50F8: 48000008  b 0x82ea5100
	pc = 0x82EA5100; continue 'dispatch;
	// 82EA50FC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82EA5100: 7D6A4850  subf r11, r10, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82EA5104: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EA5108: 4199FFD4  bgt cr6, 0x82ea50dc
	if ctx.cr[6].gt {
	pc = 0x82EA50DC; continue 'dispatch;
	}
	// 82EA510C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA5110: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EA5114: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82EA5118: 419A0010  beq cr6, 0x82ea5128
	if ctx.cr[6].eq {
	pc = 0x82EA5128; continue 'dispatch;
	}
	// 82EA511C: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5120: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA5124: 91060000  stw r8, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA5128: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82EA512C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EA5130: 409AFF98  bne cr6, 0x82ea50c8
	if !ctx.cr[6].eq {
	pc = 0x82EA50C8; continue 'dispatch;
	}
	// 82EA5134: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EA5138: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82EA513C: 4082FF7C  bne 0x82ea50b8
	if !ctx.cr[0].eq {
	pc = 0x82EA50B8; continue 'dispatch;
	}
	// 82EA5140: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EA5144: 939B0A18  stw r28, 0xa18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2584 as u32), ctx.r[28].u32 ) };
	// 82EA5148: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82EA514C: 41980054  blt cr6, 0x82ea51a0
	if ctx.cr[6].lt {
	pc = 0x82EA51A0; continue 'dispatch;
	}
	// 82EA5150: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA5154: 7CFFF050  subf r7, r31, r30
	ctx.r[7].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82EA5158: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EA515C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5160: 7D67502E  lwzx r11, r7, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA5164: 2F092000  cmpwi cr6, r9, 0x2000
	ctx.cr[6].compare_i32(ctx.r[9].s32, 8192, &mut ctx.xer);
	// 82EA5168: 409A0020  bne cr6, 0x82ea5188
	if !ctx.cr[6].eq {
	pc = 0x82EA5188; continue 'dispatch;
	}
	// 82EA516C: 813B0A1C  lwz r9, 0xa1c(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82EA5170: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA5174: 917B0A1C  stw r11, 0xa1c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2588 as u32), ctx.r[11].u32 ) };
	// 82EA5178: 813B0D64  lwz r9, 0xd64(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82EA517C: 38C9FFFF  addi r6, r9, -1
	ctx.r[6].s64 = ctx.r[9].s64 + -1;
	// 82EA5180: 90DB0D64  stw r6, 0xd64(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(3428 as u32), ctx.r[6].u32 ) };
	// 82EA5184: 48000010  b 0x82ea5194
	pc = 0x82EA5194; continue 'dispatch;
	// 82EA5188: 813B0A18  lwz r9, 0xa18(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82EA518C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA5190: 917B0A18  stw r11, 0xa18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	// 82EA5194: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EA5198: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82EA519C: 4080FFC0  bge 0x82ea515c
	if !ctx.cr[0].lt {
	pc = 0x82EA515C; continue 'dispatch;
	}
	// 82EA51A0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EA51A4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82EA51A8: F9770020  std r11, 0x20(r23)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[23].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EA51AC: 4839D7B1  bl 0x8324295c
	ctx.lr = 0x82EA51B0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EA51B0: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EA51B4: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EA51B8: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82EA51BC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA51C0: 409A0018  bne cr6, 0x82ea51d8
	if !ctx.cr[6].eq {
	pc = 0x82EA51D8; continue 'dispatch;
	}
	// 82EA51C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA51C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA51CC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA51D0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA51D4: 4E800421  bctrl
	ctx.lr = 0x82EA51D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA51D8: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EA51DC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EA51E0: 93C30020  stw r30, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EA51E4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EA51E8: 409A0018  bne cr6, 0x82ea5200
	if !ctx.cr[6].eq {
	pc = 0x82EA5200; continue 'dispatch;
	}
	// 82EA51EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA51F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA51F4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA51F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA51FC: 4E800421  bctrl
	ctx.lr = 0x82EA5200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5200: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EA5204: 48302FA0  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5208 size=224
    let mut pc: u32 = 0x82EA5208;
    'dispatch: loop {
        match pc {
            0x82EA5208 => {
    //   block [0x82EA5208..0x82EA52E8)
	// 82EA5208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA520C: 48302F5D  bl 0x831a8168
	ctx.lr = 0x82EA5210;
	sub_831A8130(ctx, base);
	// 82EA5210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5214: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5218: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA521C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EA5220: 7D64EA14  add r11, r4, r29
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 82EA5224: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EA5228: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82EA522C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA5230: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA5234: 7D29F02E  lwzx r9, r9, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA5238: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA523C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA5240: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5244: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA5248: 40980018  bge cr6, 0x82ea5260
	if !ctx.cr[6].lt {
	pc = 0x82EA5260; continue 'dispatch;
	}
	// 82EA524C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA5250: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EA5254: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5258: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA525C: 4198FFF0  blt cr6, 0x82ea524c
	if ctx.cr[6].lt {
	pc = 0x82EA524C; continue 'dispatch;
	}
	// 82EA5260: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA5264: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA5268: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA526C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA5270: 40980018  bge cr6, 0x82ea5288
	if !ctx.cr[6].lt {
	pc = 0x82EA5288; continue 'dispatch;
	}
	// 82EA5274: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82EA5278: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82EA527C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5280: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EA5284: 4198FFF0  blt cr6, 0x82ea5274
	if ctx.cr[6].lt {
	pc = 0x82EA5274; continue 'dispatch;
	}
	// 82EA5288: 7F05F800  cmpw cr6, r5, r31
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EA528C: 41980030  blt cr6, 0x82ea52bc
	if ctx.cr[6].lt {
	pc = 0x82EA52BC; continue 'dispatch;
	}
	// 82EA5290: 419A001C  beq cr6, 0x82ea52ac
	if ctx.cr[6].eq {
	pc = 0x82EA52AC; continue 'dispatch;
	}
	// 82EA5294: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA5298: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA529C: 7D0AF02E  lwzx r8, r10, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA52A0: 7CEBF02E  lwzx r7, r11, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EA52A4: 7D0BF12E  stwx r8, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[8].u32) };
	// 82EA52A8: 7CEAF12E  stwx r7, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[7].u32) };
	// 82EA52AC: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82EA52B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EA52B4: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EA52B8: 4099FF80  ble cr6, 0x82ea5238
	if !ctx.cr[6].gt {
	pc = 0x82EA5238; continue 'dispatch;
	}
	// 82EA52BC: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EA52C0: 40980010  bge cr6, 0x82ea52d0
	if !ctx.cr[6].lt {
	pc = 0x82EA52D0; continue 'dispatch;
	}
	// 82EA52C4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA52C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA52CC: 4BFFFF3D  bl 0x82ea5208
	ctx.lr = 0x82EA52D0;
	sub_82EA5208(ctx, base);
	// 82EA52D0: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EA52D4: 4098000C  bge cr6, 0x82ea52e0
	if !ctx.cr[6].lt {
	pc = 0x82EA52E0; continue 'dispatch;
	}
	// 82EA52D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA52DC: 4BFFFF44  b 0x82ea5220
	pc = 0x82EA5220; continue 'dispatch;
	// 82EA52E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA52E4: 48302ED4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA52E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA52E8 size=92
    let mut pc: u32 = 0x82EA52E8;
    'dispatch: loop {
        match pc {
            0x82EA52E8 => {
    //   block [0x82EA52E8..0x82EA5344)
	// 82EA52E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA52EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA52F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA52F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA52F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA52FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5300: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA5304: 4BFFFB35  bl 0x82ea4e38
	ctx.lr = 0x82EA5308;
	sub_82EA4E38(ctx, base);
	// 82EA5308: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA530C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5310: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA5314: 419A0018  beq cr6, 0x82ea532c
	if ctx.cr[6].eq {
	pc = 0x82EA532C; continue 'dispatch;
	}
	// 82EA5318: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82EA531C: 814BBDC0  lwz r10, -0x4240(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 82EA5320: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5324: 4E800421  bctrl
	ctx.lr = 0x82EA5328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA532C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA5330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5338: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA533C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5348 size=4
    let mut pc: u32 = 0x82EA5348;
    'dispatch: loop {
        match pc {
            0x82EA5348 => {
    //   block [0x82EA5348..0x82EA534C)
	// 82EA5348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5350 size=4
    let mut pc: u32 = 0x82EA5350;
    'dispatch: loop {
        match pc {
            0x82EA5350 => {
    //   block [0x82EA5350..0x82EA5354)
	// 82EA5350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5358 size=4
    let mut pc: u32 = 0x82EA5358;
    'dispatch: loop {
        match pc {
            0x82EA5358 => {
    //   block [0x82EA5358..0x82EA535C)
	// 82EA5358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5360 size=4
    let mut pc: u32 = 0x82EA5360;
    'dispatch: loop {
        match pc {
            0x82EA5360 => {
    //   block [0x82EA5360..0x82EA5364)
	// 82EA5360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5368 size=4
    let mut pc: u32 = 0x82EA5368;
    'dispatch: loop {
        match pc {
            0x82EA5368 => {
    //   block [0x82EA5368..0x82EA536C)
	// 82EA5368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5370 size=4
    let mut pc: u32 = 0x82EA5370;
    'dispatch: loop {
        match pc {
            0x82EA5370 => {
    //   block [0x82EA5370..0x82EA5374)
	// 82EA5370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5378 size=4
    let mut pc: u32 = 0x82EA5378;
    'dispatch: loop {
        match pc {
            0x82EA5378 => {
    //   block [0x82EA5378..0x82EA537C)
	// 82EA5378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5380 size=4
    let mut pc: u32 = 0x82EA5380;
    'dispatch: loop {
        match pc {
            0x82EA5380 => {
    //   block [0x82EA5380..0x82EA5384)
	// 82EA5380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5388 size=8
    let mut pc: u32 = 0x82EA5388;
    'dispatch: loop {
        match pc {
            0x82EA5388 => {
    //   block [0x82EA5388..0x82EA5390)
	// 82EA5388: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA538C: 48000004  b 0x82ea5390
	sub_82EA5390(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5390 size=192
    let mut pc: u32 = 0x82EA5390;
    'dispatch: loop {
        match pc {
            0x82EA5390 => {
    //   block [0x82EA5390..0x82EA5450)
	// 82EA5390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA539C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA53A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA53A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA53A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA53AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA53B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA53B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA53B8: 409A0020  bne cr6, 0x82ea53d8
	if !ctx.cr[6].eq {
	pc = 0x82EA53D8; continue 'dispatch;
	}
	// 82EA53BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA53C0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA53C4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA53C8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA53CC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA53D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA53D4: 4BFFB3DD  bl 0x82ea07b0
	ctx.lr = 0x82EA53D8;
	sub_82EA07B0(ctx, base);
	// 82EA53D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA53DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA53E0: 419A0054  beq cr6, 0x82ea5434
	if ctx.cr[6].eq {
	pc = 0x82EA5434; continue 'dispatch;
	}
	// 82EA53E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA53E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA53EC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA53F0: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EA53F4: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA53F8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EA53FC: 4198001C  blt cr6, 0x82ea5418
	if ctx.cr[6].lt {
	pc = 0x82EA5418; continue 'dispatch;
	}
	// 82EA5400: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 82EA5404: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA5408: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82EA540C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA5410: 4BFFAC51  bl 0x82ea0060
	ctx.lr = 0x82EA5414;
	sub_82EA0060(ctx, base);
	// 82EA5414: 48000020  b 0x82ea5434
	pc = 0x82EA5434; continue 'dispatch;
	// 82EA5418: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EA541C: 394B0048  addi r10, r11, 0x48
	ctx.r[10].s64 = ctx.r[11].s64 + 72;
	// 82EA5420: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EA5424: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EA5428: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82EA542C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA5430: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82EA5434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA543C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA5448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA544C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5450 size=4
    let mut pc: u32 = 0x82EA5450;
    'dispatch: loop {
        match pc {
            0x82EA5450 => {
    //   block [0x82EA5450..0x82EA5454)
	// 82EA5450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5458 size=4
    let mut pc: u32 = 0x82EA5458;
    'dispatch: loop {
        match pc {
            0x82EA5458 => {
    //   block [0x82EA5458..0x82EA545C)
	// 82EA5458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5460 size=4
    let mut pc: u32 = 0x82EA5460;
    'dispatch: loop {
        match pc {
            0x82EA5460 => {
    //   block [0x82EA5460..0x82EA5464)
	// 82EA5460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5468 size=4
    let mut pc: u32 = 0x82EA5468;
    'dispatch: loop {
        match pc {
            0x82EA5468 => {
    //   block [0x82EA5468..0x82EA546C)
	// 82EA5468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5470 size=4
    let mut pc: u32 = 0x82EA5470;
    'dispatch: loop {
        match pc {
            0x82EA5470 => {
    //   block [0x82EA5470..0x82EA5474)
	// 82EA5470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5478 size=4
    let mut pc: u32 = 0x82EA5478;
    'dispatch: loop {
        match pc {
            0x82EA5478 => {
    //   block [0x82EA5478..0x82EA547C)
	// 82EA5478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5480 size=4
    let mut pc: u32 = 0x82EA5480;
    'dispatch: loop {
        match pc {
            0x82EA5480 => {
    //   block [0x82EA5480..0x82EA5484)
	// 82EA5480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5488 size=4
    let mut pc: u32 = 0x82EA5488;
    'dispatch: loop {
        match pc {
            0x82EA5488 => {
    //   block [0x82EA5488..0x82EA548C)
	// 82EA5488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5490 size=8
    let mut pc: u32 = 0x82EA5490;
    'dispatch: loop {
        match pc {
            0x82EA5490 => {
    //   block [0x82EA5490..0x82EA5498)
	// 82EA5490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA5494: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5498 size=16
    let mut pc: u32 = 0x82EA5498;
    'dispatch: loop {
        match pc {
            0x82EA5498 => {
    //   block [0x82EA5498..0x82EA54A8)
	// 82EA5498: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA549C: 656A8000  oris r10, r11, 0x8000
	ctx.r[10].u64 = ctx.r[11].u64 | 2147483648;
	// 82EA54A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EA54A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA54A8 size=8
    let mut pc: u32 = 0x82EA54A8;
    'dispatch: loop {
        match pc {
            0x82EA54A8 => {
    //   block [0x82EA54A8..0x82EA54B0)
	// 82EA54A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA54AC: 48000004  b 0x82ea54b0
	sub_82EA54B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA54B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA54B0 size=204
    let mut pc: u32 = 0x82EA54B0;
    'dispatch: loop {
        match pc {
            0x82EA54B0 => {
    //   block [0x82EA54B0..0x82EA557C)
	// 82EA54B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA54B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA54B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA54BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA54C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA54C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA54C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA54CC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA54D0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA54D4: 419A0030  beq cr6, 0x82ea5504
	if ctx.cr[6].eq {
	pc = 0x82EA5504; continue 'dispatch;
	}
	// 82EA54D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA54DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA54E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA54E4: 409A0020  bne cr6, 0x82ea5504
	if !ctx.cr[6].eq {
	pc = 0x82EA5504; continue 'dispatch;
	}
	// 82EA54E8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA54EC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA54F0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA54F4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA54F8: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EA54FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA5500: 4BFFB2B1  bl 0x82ea07b0
	ctx.lr = 0x82EA5504;
	sub_82EA07B0(ctx, base);
	// 82EA5504: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA5508: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA550C: 419A0054  beq cr6, 0x82ea5560
	if ctx.cr[6].eq {
	pc = 0x82EA5560; continue 'dispatch;
	}
	// 82EA5510: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5514: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA5518: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA551C: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EA5520: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA5524: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EA5528: 4198001C  blt cr6, 0x82ea5544
	if ctx.cr[6].lt {
	pc = 0x82EA5544; continue 'dispatch;
	}
	// 82EA552C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA5530: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA5534: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82EA5538: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA553C: 4BFFAB25  bl 0x82ea0060
	ctx.lr = 0x82EA5540;
	sub_82EA0060(ctx, base);
	// 82EA5540: 48000020  b 0x82ea5560
	pc = 0x82EA5560; continue 'dispatch;
	// 82EA5544: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EA5548: 394B0048  addi r10, r11, 0x48
	ctx.r[10].s64 = ctx.r[11].s64 + 72;
	// 82EA554C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EA5550: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EA5554: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82EA5558: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA555C: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82EA5560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA5568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA556C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA5574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5580 size=4
    let mut pc: u32 = 0x82EA5580;
    'dispatch: loop {
        match pc {
            0x82EA5580 => {
    //   block [0x82EA5580..0x82EA5584)
	// 82EA5580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5588 size=4
    let mut pc: u32 = 0x82EA5588;
    'dispatch: loop {
        match pc {
            0x82EA5588 => {
    //   block [0x82EA5588..0x82EA558C)
	// 82EA5588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5590 size=8
    let mut pc: u32 = 0x82EA5590;
    'dispatch: loop {
        match pc {
            0x82EA5590 => {
    //   block [0x82EA5590..0x82EA5598)
	// 82EA5590: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA5594: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5598 size=24
    let mut pc: u32 = 0x82EA5598;
    'dispatch: loop {
        match pc {
            0x82EA5598 => {
    //   block [0x82EA5598..0x82EA55B0)
	// 82EA5598: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA559C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA55A0: 392B9EB4  addi r9, r11, -0x614c
	ctx.r[9].s64 = ctx.r[11].s64 + -24908;
	// 82EA55A4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EA55A8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA55AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA55B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA55B0 size=20
    let mut pc: u32 = 0x82EA55B0;
    'dispatch: loop {
        match pc {
            0x82EA55B0 => {
    //   block [0x82EA55B0..0x82EA55C4)
	// 82EA55B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA55B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA55B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA55BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA55C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA55C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA55C8 size=12
    let mut pc: u32 = 0x82EA55C8;
    'dispatch: loop {
        match pc {
            0x82EA55C8 => {
    //   block [0x82EA55C8..0x82EA55D4)
	// 82EA55C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA55CC: 386B9EB4  addi r3, r11, -0x614c
	ctx.r[3].s64 = ctx.r[11].s64 + -24908;
	// 82EA55D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA55D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA55D8 size=112
    let mut pc: u32 = 0x82EA55D8;
    'dispatch: loop {
        match pc {
            0x82EA55D8 => {
    //   block [0x82EA55D8..0x82EA5648)
	// 82EA55D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA55DC: 48302B91  bl 0x831a816c
	ctx.lr = 0x82EA55E0;
	sub_831A8130(ctx, base);
	// 82EA55E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA55E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA55E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA55EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA55F0: 419A0030  beq cr6, 0x82ea5620
	if ctx.cr[6].eq {
	pc = 0x82EA5620; continue 'dispatch;
	}
	// 82EA55F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA55F8: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA55FC: 48005045  bl 0x82eaa640
	ctx.lr = 0x82EA5600;
	sub_82EAA640(ctx, base);
	// 82EA5600: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA5604: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5608: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA560C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA5614: 4E800421  bctrl
	ctx.lr = 0x82EA5618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5618: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA561C: 48302BA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EA5620: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5624: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA5628: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82EA562C: 388AD4A8  addi r4, r10, -0x2b58
	ctx.r[4].s64 = ctx.r[10].s64 + -11096;
	// 82EA5630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5634: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5638: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA563C: 4E800421  bctrl
	ctx.lr = 0x82EA5640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA5644: 48302B78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5648 size=64
    let mut pc: u32 = 0x82EA5648;
    'dispatch: loop {
        match pc {
            0x82EA5648 => {
    //   block [0x82EA5648..0x82EA5688)
	// 82EA5648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA564C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5658: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA565C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5660: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5664: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA5668: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA566C: 4E800421  bctrl
	ctx.lr = 0x82EA5670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5674: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA5678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA567C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5688 size=100
    let mut pc: u32 = 0x82EA5688;
    'dispatch: loop {
        match pc {
            0x82EA5688 => {
    //   block [0x82EA5688..0x82EA56EC)
	// 82EA5688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA568C: 48302AE1  bl 0x831a816c
	ctx.lr = 0x82EA5690;
	sub_831A8130(ctx, base);
	// 82EA5690: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA5694: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA5698: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA569C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82EA56A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA56A4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82EA56A8: 38ABD230  addi r5, r11, -0x2dd0
	ctx.r[5].s64 = ctx.r[11].s64 + -11728;
	// 82EA56AC: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA56B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA56B4: 48004CAD  bl 0x82eaa360
	ctx.lr = 0x82EA56B8;
	sub_82EAA360(ctx, base);
	// 82EA56B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA56BC: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA56C0: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA56C4: 48004F7D  bl 0x82eaa640
	ctx.lr = 0x82EA56C8;
	sub_82EAA640(ctx, base);
	// 82EA56C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA56CC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA56D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA56D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA56D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA56DC: 4E800421  bctrl
	ctx.lr = 0x82EA56E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA56E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA56E4: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA56E8: 48302AD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA56F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA56F0 size=152
    let mut pc: u32 = 0x82EA56F0;
    'dispatch: loop {
        match pc {
            0x82EA56F0 => {
    //   block [0x82EA56F0..0x82EA5788)
	// 82EA56F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA56F4: 48302A75  bl 0x831a8168
	ctx.lr = 0x82EA56F8;
	sub_831A8130(ctx, base);
	// 82EA56F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA56FC: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82EA5700: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA5704: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA5708: 419A0010  beq cr6, 0x82ea5718
	if ctx.cr[6].eq {
	pc = 0x82EA5718; continue 'dispatch;
	}
	// 82EA570C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA5710: 3BCB9FFC  addi r30, r11, -0x6004
	ctx.r[30].s64 = ctx.r[11].s64 + -24580;
	// 82EA5714: 4800000C  b 0x82ea5720
	pc = 0x82EA5720; continue 'dispatch;
	// 82EA5718: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA571C: 3BCB9FF4  addi r30, r11, -0x600c
	ctx.r[30].s64 = ctx.r[11].s64 + -24588;
	// 82EA5720: 83FC0008  lwz r31, 8(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5724: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA5728: 419A0034  beq cr6, 0x82ea575c
	if ctx.cr[6].eq {
	pc = 0x82EA575C; continue 'dispatch;
	}
	// 82EA572C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA5730: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5734: 48004F0D  bl 0x82eaa640
	ctx.lr = 0x82EA5738;
	sub_82EAA640(ctx, base);
	// 82EA5738: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA573C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5740: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA5744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5748: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA574C: 4E800421  bctrl
	ctx.lr = 0x82EA5750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5750: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA5754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA5758: 48302A60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA575C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5760: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA5764: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82EA5768: 388AD4A8  addi r4, r10, -0x2b58
	ctx.r[4].s64 = ctx.r[10].s64 + -11096;
	// 82EA576C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5770: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5774: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA5778: 4E800421  bctrl
	ctx.lr = 0x82EA577C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA577C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EA5780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA5784: 48302A34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5788 size=76
    let mut pc: u32 = 0x82EA5788;
    'dispatch: loop {
        match pc {
            0x82EA5788 => {
    //   block [0x82EA5788..0x82EA57D4)
	// 82EA5788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA578C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA579C: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82EA57A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EA57A4: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 82EA57A8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA57AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA57B0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA57B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA57B8: 4E800421  bctrl
	ctx.lr = 0x82EA57BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA57BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA57C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA57C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA57C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA57CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA57D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA57D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA57D8 size=124
    let mut pc: u32 = 0x82EA57D8;
    'dispatch: loop {
        match pc {
            0x82EA57D8 => {
    //   block [0x82EA57D8..0x82EA5854)
	// 82EA57D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA57DC: 4830298D  bl 0x831a8168
	ctx.lr = 0x82EA57E0;
	sub_831A8130(ctx, base);
	// 82EA57E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA57E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA57E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA57EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EA57F0: 83FD0008  lwz r31, 8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA57F4: 419A0034  beq cr6, 0x82ea5828
	if ctx.cr[6].eq {
	pc = 0x82EA5828; continue 'dispatch;
	}
	// 82EA57F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA57FC: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5800: 48004E41  bl 0x82eaa640
	ctx.lr = 0x82EA5804;
	sub_82EAA640(ctx, base);
	// 82EA5804: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA5808: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA580C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA5810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA5818: 4E800421  bctrl
	ctx.lr = 0x82EA581C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA581C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA5820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA5824: 48302994  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA5828: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA582C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA5830: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82EA5834: 388AD4A8  addi r4, r10, -0x2b58
	ctx.r[4].s64 = ctx.r[10].s64 + -11096;
	// 82EA5838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA583C: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5840: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA5844: 4E800421  bctrl
	ctx.lr = 0x82EA5848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5848: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA584C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA5850: 48302968  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5858 size=100
    let mut pc: u32 = 0x82EA5858;
    'dispatch: loop {
        match pc {
            0x82EA5858 => {
    //   block [0x82EA5858..0x82EA58BC)
	// 82EA5858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA585C: 48302911  bl 0x831a816c
	ctx.lr = 0x82EA5860;
	sub_831A8130(ctx, base);
	// 82EA5860: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA5864: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA5868: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA586C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5870: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5874: 7C860734  extsh r6, r4
	ctx.r[6].s64 = ctx.r[4].s16 as i64;
	// 82EA5878: 38ABFDFC  addi r5, r11, -0x204
	ctx.r[5].s64 = ctx.r[11].s64 + -516;
	// 82EA587C: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA5880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5884: 48004ADD  bl 0x82eaa360
	ctx.lr = 0x82EA5888;
	sub_82EAA360(ctx, base);
	// 82EA5888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA588C: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5890: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5894: 48004DAD  bl 0x82eaa640
	ctx.lr = 0x82EA5898;
	sub_82EAA640(ctx, base);
	// 82EA5898: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA589C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA58A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA58A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA58A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA58AC: 4E800421  bctrl
	ctx.lr = 0x82EA58B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA58B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA58B4: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA58B8: 48302904  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA58C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA58C0 size=100
    let mut pc: u32 = 0x82EA58C0;
    'dispatch: loop {
        match pc {
            0x82EA58C0 => {
    //   block [0x82EA58C0..0x82EA5924)
	// 82EA58C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA58C4: 483028A9  bl 0x831a816c
	ctx.lr = 0x82EA58C8;
	sub_831A8130(ctx, base);
	// 82EA58C8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA58CC: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA58D0: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA58D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82EA58D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA58DC: 5486043E  clrlwi r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82EA58E0: 38ABD248  addi r5, r11, -0x2db8
	ctx.r[5].s64 = ctx.r[11].s64 + -11704;
	// 82EA58E4: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA58E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA58EC: 48004A75  bl 0x82eaa360
	ctx.lr = 0x82EA58F0;
	sub_82EAA360(ctx, base);
	// 82EA58F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA58F4: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA58F8: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA58FC: 48004D45  bl 0x82eaa640
	ctx.lr = 0x82EA5900;
	sub_82EAA640(ctx, base);
	// 82EA5900: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA5904: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5908: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA590C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA5910: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5914: 4E800421  bctrl
	ctx.lr = 0x82EA5918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5918: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA591C: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA5920: 4830289C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5928 size=100
    let mut pc: u32 = 0x82EA5928;
    'dispatch: loop {
        match pc {
            0x82EA5928 => {
    //   block [0x82EA5928..0x82EA598C)
	// 82EA5928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA592C: 48302841  bl 0x831a816c
	ctx.lr = 0x82EA5930;
	sub_831A8130(ctx, base);
	// 82EA5930: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA5934: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA5938: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA593C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5940: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5944: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82EA5948: 38ABFDFC  addi r5, r11, -0x204
	ctx.r[5].s64 = ctx.r[11].s64 + -516;
	// 82EA594C: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA5950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5954: 48004A0D  bl 0x82eaa360
	ctx.lr = 0x82EA5958;
	sub_82EAA360(ctx, base);
	// 82EA5958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA595C: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5960: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5964: 48004CDD  bl 0x82eaa640
	ctx.lr = 0x82EA5968;
	sub_82EAA640(ctx, base);
	// 82EA5968: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA596C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5970: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA5974: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA5978: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA597C: 4E800421  bctrl
	ctx.lr = 0x82EA5980;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA5984: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA5988: 48302834  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5990 size=100
    let mut pc: u32 = 0x82EA5990;
    'dispatch: loop {
        match pc {
            0x82EA5990 => {
    //   block [0x82EA5990..0x82EA59F4)
	// 82EA5990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5994: 483027D9  bl 0x831a816c
	ctx.lr = 0x82EA5998;
	sub_831A8130(ctx, base);
	// 82EA5998: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA599C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA59A0: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA59A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82EA59A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA59AC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82EA59B0: 38ABD248  addi r5, r11, -0x2db8
	ctx.r[5].s64 = ctx.r[11].s64 + -11704;
	// 82EA59B4: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA59B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA59BC: 480049A5  bl 0x82eaa360
	ctx.lr = 0x82EA59C0;
	sub_82EAA360(ctx, base);
	// 82EA59C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA59C4: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA59C8: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA59CC: 48004C75  bl 0x82eaa640
	ctx.lr = 0x82EA59D0;
	sub_82EAA640(ctx, base);
	// 82EA59D0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA59D4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA59D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA59DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA59E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA59E4: 4E800421  bctrl
	ctx.lr = 0x82EA59E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA59E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA59EC: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA59F0: 483027CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA59F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA59F8 size=104
    let mut pc: u32 = 0x82EA59F8;
    'dispatch: loop {
        match pc {
            0x82EA59F8 => {
    //   block [0x82EA59F8..0x82EA5A60)
	// 82EA59F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA59FC: 48302771  bl 0x831a816c
	ctx.lr = 0x82EA5A00;
	sub_831A8130(ctx, base);
	// 82EA5A00: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA5A04: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA5A08: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5A0C: D8210028  stfd f1, 0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.f[1].u64 ) };
	// 82EA5A10: E8C10028  ld r6, 0x28(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	// 82EA5A14: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82EA5A18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5A1C: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA5A20: 38AB768C  addi r5, r11, 0x768c
	ctx.r[5].s64 = ctx.r[11].s64 + 30348;
	// 82EA5A24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5A28: 48004939  bl 0x82eaa360
	ctx.lr = 0x82EA5A2C;
	sub_82EAA360(ctx, base);
	// 82EA5A2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5A30: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5A34: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5A38: 48004C09  bl 0x82eaa640
	ctx.lr = 0x82EA5A3C;
	sub_82EAA640(ctx, base);
	// 82EA5A3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA5A40: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5A44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA5A48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA5A4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5A50: 4E800421  bctrl
	ctx.lr = 0x82EA5A54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5A54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA5A58: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA5A5C: 48302760  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5A60 size=100
    let mut pc: u32 = 0x82EA5A60;
    'dispatch: loop {
        match pc {
            0x82EA5A60 => {
    //   block [0x82EA5A60..0x82EA5AC4)
	// 82EA5A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5A64: 48302709  bl 0x831a816c
	ctx.lr = 0x82EA5A68;
	sub_831A8130(ctx, base);
	// 82EA5A68: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA5A6C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA5A70: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5A74: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5A78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5A7C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82EA5A80: 38ABFE00  addi r5, r11, -0x200
	ctx.r[5].s64 = ctx.r[11].s64 + -512;
	// 82EA5A84: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA5A88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5A8C: 480048D5  bl 0x82eaa360
	ctx.lr = 0x82EA5A90;
	sub_82EAA360(ctx, base);
	// 82EA5A90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5A94: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5A98: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5A9C: 48004BA5  bl 0x82eaa640
	ctx.lr = 0x82EA5AA0;
	sub_82EAA640(ctx, base);
	// 82EA5AA0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA5AA4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5AA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA5AAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA5AB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5AB4: 4E800421  bctrl
	ctx.lr = 0x82EA5AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5AB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA5ABC: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA5AC0: 483026FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5AC8 size=100
    let mut pc: u32 = 0x82EA5AC8;
    'dispatch: loop {
        match pc {
            0x82EA5AC8 => {
    //   block [0x82EA5AC8..0x82EA5B2C)
	// 82EA5AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5ACC: 483026A1  bl 0x831a816c
	ctx.lr = 0x82EA5AD0;
	sub_831A8130(ctx, base);
	// 82EA5AD0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA5AD4: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA5AD8: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5ADC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5AE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5AE4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82EA5AE8: 38ABFE08  addi r5, r11, -0x1f8
	ctx.r[5].s64 = ctx.r[11].s64 + -504;
	// 82EA5AEC: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA5AF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5AF4: 4800486D  bl 0x82eaa360
	ctx.lr = 0x82EA5AF8;
	sub_82EAA360(ctx, base);
	// 82EA5AF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA5AFC: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5B00: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5B04: 48004B3D  bl 0x82eaa640
	ctx.lr = 0x82EA5B08;
	sub_82EAA640(ctx, base);
	// 82EA5B08: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA5B0C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5B10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EA5B14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA5B18: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5B1C: 4E800421  bctrl
	ctx.lr = 0x82EA5B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5B20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA5B24: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA5B28: 48302694  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5B30 size=152
    let mut pc: u32 = 0x82EA5B30;
    'dispatch: loop {
        match pc {
            0x82EA5B30 => {
    //   block [0x82EA5B30..0x82EA5BC8)
	// 82EA5B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5B38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA5B3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5B40: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82EA5B44: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EA5B48: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EA5B4C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EA5B50: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EA5B54: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EA5B58: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EA5B5C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EA5B60: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5B64: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA5B68: 394127C0  addi r10, r1, 0x27c0
	ctx.r[10].s64 = ctx.r[1].s64 + 10176;
	// 82EA5B6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5B70: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EA5B74: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82EA5B78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA5B7C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA5B80: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA5B84: 480047D5  bl 0x82eaa358
	ctx.lr = 0x82EA5B88;
	sub_82EAA358(ctx, base);
	// 82EA5B88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EA5B8C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5B90: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5B94: 48004AAD  bl 0x82eaa640
	ctx.lr = 0x82EA5B98;
	sub_82EAA640(ctx, base);
	// 82EA5B98: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA5B9C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5BA0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EA5BA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA5BA8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA5BAC: 4E800421  bctrl
	ctx.lr = 0x82EA5BB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5BB0: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82EA5BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5BBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA5BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5BC8 size=20
    let mut pc: u32 = 0x82EA5BC8;
    'dispatch: loop {
        match pc {
            0x82EA5BC8 => {
    //   block [0x82EA5BC8..0x82EA5BDC)
	// 82EA5BC8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5BCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5BD0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA5BD4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5BD8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5BE0 size=20
    let mut pc: u32 = 0x82EA5BE0;
    'dispatch: loop {
        match pc {
            0x82EA5BE0 => {
    //   block [0x82EA5BE0..0x82EA5BF4)
	// 82EA5BE0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5BE8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5BEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5BF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5BF8 size=76
    let mut pc: u32 = 0x82EA5BF8;
    'dispatch: loop {
        match pc {
            0x82EA5BF8 => {
    //   block [0x82EA5BF8..0x82EA5C44)
	// 82EA5BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5C04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5C08: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82EA5C0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5C10: 388B3EFC  addi r4, r11, 0x3efc
	ctx.r[4].s64 = ctx.r[11].s64 + 16124;
	// 82EA5C14: 4BFFFBC5  bl 0x82ea57d8
	ctx.lr = 0x82EA5C18;
	sub_82EA57D8(ctx, base);
	// 82EA5C18: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5C1C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5C20: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA5C24: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EA5C28: 4E800421  bctrl
	ctx.lr = 0x82EA5C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA5C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5C3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5C48 size=64
    let mut pc: u32 = 0x82EA5C48;
    'dispatch: loop {
        match pc {
            0x82EA5C48 => {
    //   block [0x82EA5C48..0x82EA5C88)
	// 82EA5C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5C50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5C54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5C58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5C5C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5C60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5C64: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA5C68: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5C6C: 4E800421  bctrl
	ctx.lr = 0x82EA5C70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5C74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA5C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5C80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5C88 size=32
    let mut pc: u32 = 0x82EA5C88;
    'dispatch: loop {
        match pc {
            0x82EA5C88 => {
    //   block [0x82EA5C88..0x82EA5CA8)
	// 82EA5C88: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5C8C: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82EA5C90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA5C94: 392BFE40  addi r9, r11, -0x1c0
	ctx.r[9].s64 = ctx.r[11].s64 + -448;
	// 82EA5C98: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EA5C9C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EA5CA0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA5CA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5CA8 size=12
    let mut pc: u32 = 0x82EA5CA8;
    'dispatch: loop {
        match pc {
            0x82EA5CA8 => {
    //   block [0x82EA5CA8..0x82EA5CB4)
	// 82EA5CA8: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5CAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA5CB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5CB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5CB4 size=16
    let mut pc: u32 = 0x82EA5CB4;
    'dispatch: loop {
        match pc {
            0x82EA5CB4 => {
    //   block [0x82EA5CB4..0x82EA5CC4)
	// 82EA5CB4: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA5CB8: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EA5CBC: B1440006  sth r10, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EA5CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5CC8 size=92
    let mut pc: u32 = 0x82EA5CC8;
    'dispatch: loop {
        match pc {
            0x82EA5CC8 => {
    //   block [0x82EA5CC8..0x82EA5D24)
	// 82EA5CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5CDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5CE0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82EA5CE4: 392BFE40  addi r9, r11, -0x1c0
	ctx.r[9].s64 = ctx.r[11].s64 + -448;
	// 82EA5CE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EA5CEC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA5CF0: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82EA5CF4: 806A70EC  lwz r3, 0x70ec(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28908 as u32) ) } as u64;
	// 82EA5CF8: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5CFC: 80C70010  lwz r6, 0x10(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5D00: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82EA5D04: 4E800421  bctrl
	ctx.lr = 0x82EA5D08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5D08: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EA5D0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA5D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5D1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5D28 size=112
    let mut pc: u32 = 0x82EA5D28;
    'dispatch: loop {
        match pc {
            0x82EA5D28 => {
    //   block [0x82EA5D28..0x82EA5D98)
	// 82EA5D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5D2C: 4830243D  bl 0x831a8168
	ctx.lr = 0x82EA5D30;
	sub_831A8130(ctx, base);
	// 82EA5D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5D38: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5D3C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5D40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EA5D44: 390BFE40  addi r8, r11, -0x1c0
	ctx.r[8].s64 = ctx.r[11].s64 + -448;
	// 82EA5D48: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82EA5D4C: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA5D50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA5D54: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA5D58: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA5D5C: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82EA5D60: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82EA5D64: 7C67502E  lwzx r3, r7, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA5D68: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EA5D6C: 4BFFA9C5  bl 0x82ea0730
	ctx.lr = 0x82EA5D70;
	sub_82EA0730(ctx, base);
	// 82EA5D70: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 82EA5D74: B0A30004  sth r5, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 82EA5D78: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EA5D7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EA5D80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA5D84: 480077A5  bl 0x82ead528
	ctx.lr = 0x82EA5D88;
	sub_82EAD528(ctx, base);
	// 82EA5D88: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EA5D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5D90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA5D94: 48302424  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5D98 size=128
    let mut pc: u32 = 0x82EA5D98;
    'dispatch: loop {
        match pc {
            0x82EA5D98 => {
    //   block [0x82EA5D98..0x82EA5E18)
	// 82EA5D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5D9C: 483023CD  bl 0x831a8168
	ctx.lr = 0x82EA5DA0;
	sub_831A8130(ctx, base);
	// 82EA5DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5DA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5DA8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5DAC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5DB0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82EA5DB4: 392BFE40  addi r9, r11, -0x1c0
	ctx.r[9].s64 = ctx.r[11].s64 + -448;
	// 82EA5DB8: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82EA5DBC: B3FE0006  sth r31, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[31].u16 ) };
	// 82EA5DC0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA5DC4: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA5DC8: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82EA5DCC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82EA5DD0: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA5DD4: 4BFFA95D  bl 0x82ea0730
	ctx.lr = 0x82EA5DD8;
	sub_82EA0730(ctx, base);
	// 82EA5DD8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA5DDC: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82EA5DE0: 93BC0008  stw r29, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EA5DE4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82EA5DE8: B3FC0006  sth r31, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[31].u16 ) };
	// 82EA5DEC: 38A7FE14  addi r5, r7, -0x1ec
	ctx.r[5].s64 = ctx.r[7].s64 + -492;
	// 82EA5DF0: B0DC0004  sth r6, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82EA5DF4: 90BC0000  stw r5, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EA5DF8: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5DFC: 909C000C  stw r4, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82EA5E00: 93FC0010  stw r31, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82EA5E04: 4800017D  bl 0x82ea5f80
	ctx.lr = 0x82EA5E08;
	sub_82EA5F80(ctx, base);
	// 82EA5E08: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82EA5E0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA5E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA5E14: 483023A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5E18 size=132
    let mut pc: u32 = 0x82EA5E18;
    'dispatch: loop {
        match pc {
            0x82EA5E18 => {
    //   block [0x82EA5E18..0x82EA5E9C)
	// 82EA5E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5E20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5E24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5E28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5E2C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA5E30: 394BFE40  addi r10, r11, -0x1c0
	ctx.r[10].s64 = ctx.r[11].s64 + -448;
	// 82EA5E34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5E38: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA5E3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA5E40: 419A003C  beq cr6, 0x82ea5e7c
	if ctx.cr[6].eq {
	pc = 0x82EA5E7C; continue 'dispatch;
	}
	// 82EA5E44: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5E48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA5E4C: 419A0030  beq cr6, 0x82ea5e7c
	if ctx.cr[6].eq {
	pc = 0x82EA5E7C; continue 'dispatch;
	}
	// 82EA5E50: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA5E54: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EA5E58: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EA5E5C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA5E60: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA5E64: 409A0018  bne cr6, 0x82ea5e7c
	if !ctx.cr[6].eq {
	pc = 0x82EA5E7C; continue 'dispatch;
	}
	// 82EA5E68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5E6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA5E70: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5E74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5E78: 4E800421  bctrl
	ctx.lr = 0x82EA5E7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5E7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA5E80: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EA5E84: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA5E88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA5E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5E94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5EA0 size=140
    let mut pc: u32 = 0x82EA5EA0;
    'dispatch: loop {
        match pc {
            0x82EA5EA0 => {
    //   block [0x82EA5EA0..0x82EA5F2C)
	// 82EA5EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5EA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA5EAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5EB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA5EB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA5EBC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5EC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA5EC4: 419A0010  beq cr6, 0x82ea5ed4
	if ctx.cr[6].eq {
	pc = 0x82EA5ED4; continue 'dispatch;
	}
	// 82EA5EC8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA5ECC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EA5ED0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EA5ED4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5ED8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5EDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA5EE0: 419A0030  beq cr6, 0x82ea5f10
	if ctx.cr[6].eq {
	pc = 0x82EA5F10; continue 'dispatch;
	}
	// 82EA5EE4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA5EE8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EA5EEC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EA5EF0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA5EF4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA5EF8: 409A0018  bne cr6, 0x82ea5f10
	if !ctx.cr[6].eq {
	pc = 0x82EA5F10; continue 'dispatch;
	}
	// 82EA5EFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5F00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA5F04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5F08: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5F0C: 4E800421  bctrl
	ctx.lr = 0x82EA5F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5F10: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82EA5F14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA5F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5F20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA5F24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5F30 size=76
    let mut pc: u32 = 0x82EA5F30;
    'dispatch: loop {
        match pc {
            0x82EA5F30 => {
    //   block [0x82EA5F30..0x82EA5F7C)
	// 82EA5F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5F44: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5F48: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5F4C: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82EA5F50: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5F54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5F58: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA5F5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA5F60: 4E800421  bctrl
	ctx.lr = 0x82EA5F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA5F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA5F68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA5F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5F74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA5F80 size=116
    let mut pc: u32 = 0x82EA5F80;
    'dispatch: loop {
        match pc {
            0x82EA5F80 => {
    //   block [0x82EA5F80..0x82EA5FF4)
	// 82EA5F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA5F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA5F88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA5F8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA5F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA5F94: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5F98: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5F9C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5FA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA5FA4: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA5FA8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA5FAC: 40980020  bge cr6, 0x82ea5fcc
	if !ctx.cr[6].lt {
	pc = 0x82EA5FCC; continue 'dispatch;
	}
	// 82EA5FB0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA5FB4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EA5FB8: 40980008  bge cr6, 0x82ea5fc0
	if !ctx.cr[6].lt {
	pc = 0x82EA5FC0; continue 'dispatch;
	}
	// 82EA5FBC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EA5FC0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EA5FC4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EA5FC8: 48000831  bl 0x82ea67f8
	ctx.lr = 0x82EA5FCC;
	sub_82EA67F8(ctx, base);
	// 82EA5FCC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA5FD4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA5FD8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA5FDC: 7D4941AE  stbx r10, r9, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82EA5FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA5FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA5FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA5FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA5FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA5FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA5FF8 size=20
    let mut pc: u32 = 0x82EA5FF8;
    'dispatch: loop {
        match pc {
            0x82EA5FF8 => {
    //   block [0x82EA5FF8..0x82EA600C)
	// 82EA5FF8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA5FFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA6000: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA6004: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EA6008: 4BFFFF78  b 0x82ea5f80
	sub_82EA5F80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6010 size=212
    let mut pc: u32 = 0x82EA6010;
    'dispatch: loop {
        match pc {
            0x82EA6010 => {
    //   block [0x82EA6010..0x82EA60E4)
	// 82EA6010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6014: 48302155  bl 0x831a8168
	ctx.lr = 0x82EA6018;
	sub_831A8130(ctx, base);
	// 82EA6018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA601C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6020: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA6024: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA6028: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA602C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA6030: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA6034: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EA6038: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EA603C: 40990058  ble cr6, 0x82ea6094
	if !ctx.cr[6].gt {
	pc = 0x82EA6094; continue 'dispatch;
	}
	// 82EA6040: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EA6044: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6048: 7FABF214  add r29, r11, r30
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA604C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA6050: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82EA6054: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA6058: 40980020  bge cr6, 0x82ea6078
	if !ctx.cr[6].lt {
	pc = 0x82EA6078; continue 'dispatch;
	}
	// 82EA605C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA6060: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EA6064: 40980008  bge cr6, 0x82ea606c
	if !ctx.cr[6].lt {
	pc = 0x82EA606C; continue 'dispatch;
	}
	// 82EA6068: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EA606C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EA6070: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EA6074: 48000785  bl 0x82ea67f8
	ctx.lr = 0x82EA6078;
	sub_82EA67F8(ctx, base);
	// 82EA6078: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA607C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA6080: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EA6084: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6088: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA608C: 7D48E9AE  stbx r10, r8, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u8) };
	// 82EA6090: 48000020  b 0x82ea60b0
	pc = 0x82EA60B0; continue 'dispatch;
	// 82EA6094: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6098: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA609C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA60A0: 40990010  ble cr6, 0x82ea60b0
	if !ctx.cr[6].gt {
	pc = 0x82EA60B0; continue 'dispatch;
	}
	// 82EA60A4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA60A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EA60AC: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82EA60B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA60B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EA60B8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA60BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA60C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA60C4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA60C8: 48004681  bl 0x82eaa748
	ctx.lr = 0x82EA60CC;
	sub_82EAA748(ctx, base);
	// 82EA60CC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA60D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA60D4: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EA60D8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EA60DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA60E0: 483020D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA60E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA60E8 size=232
    let mut pc: u32 = 0x82EA60E8;
    'dispatch: loop {
        match pc {
            0x82EA60E8 => {
    //   block [0x82EA60E8..0x82EA61D0)
	// 82EA60E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA60EC: 48302079  bl 0x831a8164
	ctx.lr = 0x82EA60F0;
	sub_831A8130(ctx, base);
	// 82EA60F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA60F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EA60F8: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82EA60FC: 839B000C  lwz r28, 0xc(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA6100: 41980028  blt cr6, 0x82ea6128
	if ctx.cr[6].lt {
	pc = 0x82EA6128; continue 'dispatch;
	}
	// 82EA6104: 419A001C  beq cr6, 0x82ea6120
	if ctx.cr[6].eq {
	pc = 0x82EA6120; continue 'dispatch;
	}
	// 82EA6108: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 82EA610C: 40980020  bge cr6, 0x82ea612c
	if !ctx.cr[6].lt {
	pc = 0x82EA612C; continue 'dispatch;
	}
	// 82EA6110: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6114: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA6118: 7F845050  subf r28, r4, r10
	ctx.r[28].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 82EA611C: 48000010  b 0x82ea612c
	pc = 0x82EA612C; continue 'dispatch;
	// 82EA6120: 7F9C2214  add r28, r28, r4
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[4].u64;
	// 82EA6124: 48000008  b 0x82ea612c
	pc = 0x82EA612C; continue 'dispatch;
	// 82EA6128: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA612C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EA6130: 41980094  blt cr6, 0x82ea61c4
	if ctx.cr[6].lt {
	pc = 0x82EA61C4; continue 'dispatch;
	}
	// 82EA6134: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6138: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA613C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA6140: 40990074  ble cr6, 0x82ea61b4
	if !ctx.cr[6].gt {
	pc = 0x82EA61B4; continue 'dispatch;
	}
	// 82EA6144: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EA6148: 3BFC0001  addi r31, r28, 1
	ctx.r[31].s64 = ctx.r[28].s64 + 1;
	// 82EA614C: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EA6150: 40990058  ble cr6, 0x82ea61a8
	if !ctx.cr[6].gt {
	pc = 0x82EA61A8; continue 'dispatch;
	}
	// 82EA6154: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6158: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA615C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EA6160: 40980024  bge cr6, 0x82ea6184
	if !ctx.cr[6].lt {
	pc = 0x82EA6184; continue 'dispatch;
	}
	// 82EA6164: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA6168: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA616C: 41980008  blt cr6, 0x82ea6174
	if ctx.cr[6].lt {
	pc = 0x82EA6174; continue 'dispatch;
	}
	// 82EA6170: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EA6174: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EA6178: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EA617C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA6180: 48000679  bl 0x82ea67f8
	ctx.lr = 0x82EA6184;
	sub_82EA67F8(ctx, base);
	// 82EA6184: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EA6188: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EA618C: 4098001C  bge cr6, 0x82ea61a8
	if !ctx.cr[6].lt {
	pc = 0x82EA61A8; continue 'dispatch;
	}
	// 82EA6190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA6194: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6198: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 82EA619C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA61A0: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EA61A4: 4198FFF0  blt cr6, 0x82ea6194
	if ctx.cr[6].lt {
	pc = 0x82EA6194; continue 'dispatch;
	}
	// 82EA61A8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EA61AC: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA61B0: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82EA61B4: 939B000C  stw r28, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82EA61B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA61BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA61C0: 48301FF4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EA61C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA61C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA61CC: 48301FE8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA61D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA61D0 size=192
    let mut pc: u32 = 0x82EA61D0;
    'dispatch: loop {
        match pc {
            0x82EA61D0 => {
    //   block [0x82EA61D0..0x82EA6290)
	// 82EA61D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA61D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA61D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA61DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA61E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA61E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA61E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA61EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA61F0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA61F4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA61F8: 409A0020  bne cr6, 0x82ea6218
	if !ctx.cr[6].eq {
	pc = 0x82EA6218; continue 'dispatch;
	}
	// 82EA61FC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6200: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA6204: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA6208: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA620C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA6210: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA6214: 4BFFA59D  bl 0x82ea07b0
	ctx.lr = 0x82EA6218;
	sub_82EA07B0(ctx, base);
	// 82EA6218: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA621C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA6220: 419A0054  beq cr6, 0x82ea6274
	if ctx.cr[6].eq {
	pc = 0x82EA6274; continue 'dispatch;
	}
	// 82EA6224: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6228: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA622C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA6230: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EA6234: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA6238: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EA623C: 4198001C  blt cr6, 0x82ea6258
	if ctx.cr[6].lt {
	pc = 0x82EA6258; continue 'dispatch;
	}
	// 82EA6240: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA6244: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EA6248: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82EA624C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EA6250: 4BFF9E11  bl 0x82ea0060
	ctx.lr = 0x82EA6254;
	sub_82EA0060(ctx, base);
	// 82EA6254: 48000020  b 0x82ea6274
	pc = 0x82EA6274; continue 'dispatch;
	// 82EA6258: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EA625C: 394B0048  addi r10, r11, 0x48
	ctx.r[10].s64 = ctx.r[11].s64 + 72;
	// 82EA6260: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EA6264: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EA6268: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82EA626C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA6270: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82EA6274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA6278: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA627C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA6280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA6284: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA6288: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA628C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6290 size=100
    let mut pc: u32 = 0x82EA6290;
    'dispatch: loop {
        match pc {
            0x82EA6290 => {
    //   block [0x82EA6290..0x82EA62F4)
	// 82EA6290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA6298: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA629C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA62A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA62A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA62A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA62AC: 4BFFFB6D  bl 0x82ea5e18
	ctx.lr = 0x82EA62B0;
	sub_82EA5E18(ctx, base);
	// 82EA62B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA62B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA62B8: 419A0020  beq cr6, 0x82ea62d8
	if ctx.cr[6].eq {
	pc = 0x82EA62D8; continue 'dispatch;
	}
	// 82EA62BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA62C0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA62C4: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EA62C8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA62CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA62D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA62D4: 4BFFA4DD  bl 0x82ea07b0
	ctx.lr = 0x82EA62D8;
	sub_82EA07B0(ctx, base);
	// 82EA62D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA62DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA62E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA62E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA62E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA62EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA62F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA62F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA62F8 size=152
    let mut pc: u32 = 0x82EA62F8;
    'dispatch: loop {
        match pc {
            0x82EA62F8 => {
    //   block [0x82EA62F8..0x82EA6390)
	// 82EA62F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA62FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA6300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA6304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA6308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA630C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6310: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA6314: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA6318: 392BFE14  addi r9, r11, -0x1ec
	ctx.r[9].s64 = ctx.r[11].s64 + -492;
	// 82EA631C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA6320: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA6324: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA6328: 409A0018  bne cr6, 0x82ea6340
	if !ctx.cr[6].eq {
	pc = 0x82EA6340; continue 'dispatch;
	}
	// 82EA632C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6330: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA6334: 419A000C  beq cr6, 0x82ea6340
	if ctx.cr[6].eq {
	pc = 0x82EA6340; continue 'dispatch;
	}
	// 82EA6338: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EA633C: 4BFFFE95  bl 0x82ea61d0
	ctx.lr = 0x82EA6340;
	sub_82EA61D0(ctx, base);
	// 82EA6340: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA6344: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EA6348: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EA634C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA6350: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA6354: 419A0020  beq cr6, 0x82ea6374
	if ctx.cr[6].eq {
	pc = 0x82EA6374; continue 'dispatch;
	}
	// 82EA6358: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA635C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA6360: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82EA6364: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA6368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA636C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA6370: 4BFFA441  bl 0x82ea07b0
	ctx.lr = 0x82EA6374;
	sub_82EA07B0(ctx, base);
	// 82EA6374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA6378: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA637C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA6380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA6384: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA6388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA638C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6390 size=136
    let mut pc: u32 = 0x82EA6390;
    'dispatch: loop {
        match pc {
            0x82EA6390 => {
    //   block [0x82EA6390..0x82EA6418)
	// 82EA6390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6394: 48301DD9  bl 0x831a816c
	ctx.lr = 0x82EA6398;
	sub_831A8130(ctx, base);
	// 82EA6398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA639C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA63A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EA63A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA63A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA63AC: 4BFFF8DD  bl 0x82ea5c88
	ctx.lr = 0x82EA63B0;
	sub_82EA5C88(ctx, base);
	// 82EA63B0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA63B4: 7D5EEA14  add r10, r30, r29
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82EA63B8: 392BFE50  addi r9, r11, -0x1b0
	ctx.r[9].s64 = ctx.r[11].s64 + -432;
	// 82EA63BC: 346AFFE0  addic. r3, r10, -0x20
	ctx.xer.ca = (ctx.r[10].u32 > (!(-32 as u32)));
	ctx.r[3].s64 = ctx.r[10].s64 + -32;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA63C0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EA63C4: 41820028  beq 0x82ea63ec
	if ctx.cr[0].eq {
	pc = 0x82EA63EC; continue 'dispatch;
	}
	// 82EA63C8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA63CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA63D0: 38BDFFE0  addi r5, r29, -0x20
	ctx.r[5].s64 = ctx.r[29].s64 + -32;
	// 82EA63D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA63D8: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82EA63DC: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA63E0: 48007149  bl 0x82ead528
	ctx.lr = 0x82EA63E4;
	sub_82EAD528(ctx, base);
	// 82EA63E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA63E8: 48000008  b 0x82ea63f0
	pc = 0x82EA63F0; continue 'dispatch;
	// 82EA63EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA63F0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA63F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA63F8: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA63FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA6400: 419A0010  beq cr6, 0x82ea6410
	if ctx.cr[6].eq {
	pc = 0x82EA6410; continue 'dispatch;
	}
	// 82EA6404: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA6408: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82EA640C: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EA6410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA6414: 48301DA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6418 size=264
    let mut pc: u32 = 0x82EA6418;
    'dispatch: loop {
        match pc {
            0x82EA6418 => {
    //   block [0x82EA6418..0x82EA6520)
	// 82EA6418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA641C: 48301D35  bl 0x831a8150
	ctx.lr = 0x82EA6420;
	sub_831A8130(ctx, base);
	// 82EA6420: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA6424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6428: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA642C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82EA6430: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EA6434: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA6438: 4BFFFF59  bl 0x82ea6390
	ctx.lr = 0x82EA643C;
	sub_82EA6390(ctx, base);
	// 82EA643C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA6440: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA6444: 388BFF58  addi r4, r11, -0xa8
	ctx.r[4].s64 = ctx.r[11].s64 + -168;
	// 82EA6448: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA644C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82EA6450: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82EA6454: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82EA6458: 3CA08212  lis r5, -0x7dee
	ctx.r[5].s64 = -2112749568;
	// 82EA645C: 3FA08212  lis r29, -0x7dee
	ctx.r[29].s64 = -2112749568;
	// 82EA6460: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA6464: 3B8AFF20  addi r28, r10, -0xe0
	ctx.r[28].s64 = ctx.r[10].s64 + -224;
	// 82EA6468: 3B69FF04  addi r27, r9, -0xfc
	ctx.r[27].s64 = ctx.r[9].s64 + -252;
	// 82EA646C: 3B48FED0  addi r26, r8, -0x130
	ctx.r[26].s64 = ctx.r[8].s64 + -304;
	// 82EA6470: 3B27FE98  addi r25, r7, -0x168
	ctx.r[25].s64 = ctx.r[7].s64 + -360;
	// 82EA6474: 3B06FE94  addi r24, r6, -0x16c
	ctx.r[24].s64 = ctx.r[6].s64 + -364;
	// 82EA6478: 3AE5FE8C  addi r23, r5, -0x174
	ctx.r[23].s64 = ctx.r[5].s64 + -372;
	// 82EA647C: 3BBDFE78  addi r29, r29, -0x188
	ctx.r[29].s64 = ctx.r[29].s64 + -392;
	// 82EA6480: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EA6484: 7D760734  extsh r22, r11
	ctx.r[22].s64 = ctx.r[11].s16 as i64;
	// 82EA6488: 4BFFF351  bl 0x82ea57d8
	ctx.lr = 0x82EA648C;
	sub_82EA57D8(ctx, base);
	// 82EA648C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA6490: 4BFFF1F9  bl 0x82ea5688
	ctx.lr = 0x82EA6494;
	sub_82EA5688(ctx, base);
	// 82EA6494: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA6498: 4BFFF341  bl 0x82ea57d8
	ctx.lr = 0x82EA649C;
	sub_82EA57D8(ctx, base);
	// 82EA649C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82EA64A0: 4BFFF489  bl 0x82ea5928
	ctx.lr = 0x82EA64A4;
	sub_82EA5928(ctx, base);
	// 82EA64A4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EA64A8: 4BFFF331  bl 0x82ea57d8
	ctx.lr = 0x82EA64AC;
	sub_82EA57D8(ctx, base);
	// 82EA64AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA64B0: 4BFFF329  bl 0x82ea57d8
	ctx.lr = 0x82EA64B4;
	sub_82EA57D8(ctx, base);
	// 82EA64B4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EA64B8: 4BFFF321  bl 0x82ea57d8
	ctx.lr = 0x82EA64BC;
	sub_82EA57D8(ctx, base);
	// 82EA64BC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EA64C0: 4BFFF319  bl 0x82ea57d8
	ctx.lr = 0x82EA64C4;
	sub_82EA57D8(ctx, base);
	// 82EA64C4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EA64C8: 4BFFF311  bl 0x82ea57d8
	ctx.lr = 0x82EA64CC;
	sub_82EA57D8(ctx, base);
	// 82EA64CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EA64D0: 4BFFF309  bl 0x82ea57d8
	ctx.lr = 0x82EA64D4;
	sub_82EA57D8(ctx, base);
	// 82EA64D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA64D8: 4BFFF301  bl 0x82ea57d8
	ctx.lr = 0x82EA64DC;
	sub_82EA57D8(ctx, base);
	// 82EA64DC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82EA64E0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA64E4: 3CA02C66  lis r5, 0x2c66
	ctx.r[5].s64 = 744882176;
	// 82EA64E8: 3900001E  li r8, 0x1e
	ctx.r[8].s64 = 30;
	// 82EA64EC: 38E9FE5C  addi r7, r9, -0x1a4
	ctx.r[7].s64 = ctx.r[9].s64 + -420;
	// 82EA64F0: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82EA64F4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EA64F8: 60A5F2D8  ori r5, r5, 0xf2d8
	ctx.r[5].u64 = ctx.r[5].u64 | 62168;
	// 82EA64FC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82EA6500: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6504: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA6508: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EA650C: 4E800421  bctrl
	ctx.lr = 0x82EA6510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA6510: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA6514: 4BFFF905  bl 0x82ea5e18
	ctx.lr = 0x82EA6518;
	sub_82EA5E18(ctx, base);
	// 82EA6518: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82EA651C: 48301C84  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6520 size=128
    let mut pc: u32 = 0x82EA6520;
    'dispatch: loop {
        match pc {
            0x82EA6520 => {
    //   block [0x82EA6520..0x82EA65A0)
	// 82EA6520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA6528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA652C: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA6530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6534: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82EA6538: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EA653C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA6540: 4BFFFE51  bl 0x82ea6390
	ctx.lr = 0x82EA6544;
	sub_82EA6390(ctx, base);
	// 82EA6544: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EA6548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA654C: 4BFFF28D  bl 0x82ea57d8
	ctx.lr = 0x82EA6550;
	sub_82EA57D8(ctx, base);
	// 82EA6550: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EA6554: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA6558: 3CA02636  lis r5, 0x2636
	ctx.r[5].s64 = 641073152;
	// 82EA655C: 39000025  li r8, 0x25
	ctx.r[8].s64 = 37;
	// 82EA6560: 38EAFE5C  addi r7, r10, -0x1a4
	ctx.r[7].s64 = ctx.r[10].s64 + -420;
	// 82EA6564: 806B6E5C  lwz r3, 0x6e5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82EA6568: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EA656C: 60A5FE25  ori r5, r5, 0xfe25
	ctx.r[5].u64 = ctx.r[5].u64 | 65061;
	// 82EA6570: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82EA6574: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6578: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA657C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA6580: 4E800421  bctrl
	ctx.lr = 0x82EA6584;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA6584: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA6588: 4BFFF891  bl 0x82ea5e18
	ctx.lr = 0x82EA658C;
	sub_82EA5E18(ctx, base);
	// 82EA658C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82EA6590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA6594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA6598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA659C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA65A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA65A0 size=12
    let mut pc: u32 = 0x82EA65A0;
    'dispatch: loop {
        match pc {
            0x82EA65A0 => {
    //   block [0x82EA65A0..0x82EA65AC)
	// 82EA65A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA65A4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EA65A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA65B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA65B0 size=168
    let mut pc: u32 = 0x82EA65B0;
    'dispatch: loop {
        match pc {
            0x82EA65B0 => {
    //   block [0x82EA65B0..0x82EA6658)
	// 82EA65B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA65B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA65B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA65BC: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA65C0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EA65C4: 419A0020  beq cr6, 0x82ea65e4
	if ctx.cr[6].eq {
	pc = 0x82EA65E4; continue 'dispatch;
	}
	// 82EA65C8: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA65CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA65D0: 419A0014  beq cr6, 0x82ea65e4
	if ctx.cr[6].eq {
	pc = 0x82EA65E4; continue 'dispatch;
	}
	// 82EA65D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA65D8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA65DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA65E0: 4BFFA3D9  bl 0x82ea09b8
	ctx.lr = 0x82EA65E4;
	sub_82EA09B8(ctx, base);
	// 82EA65E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA65E8: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82EA65EC: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA65F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EA65F4: 419A0054  beq cr6, 0x82ea6648
	if ctx.cr[6].eq {
	pc = 0x82EA6648; continue 'dispatch;
	}
	// 82EA65F8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA65FC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA6600: 81230054  lwz r9, 0x54(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA6604: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EA6608: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EA660C: 41980020  blt cr6, 0x82ea662c
	if ctx.cr[6].lt {
	pc = 0x82EA662C; continue 'dispatch;
	}
	// 82EA6610: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 82EA6614: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82EA6618: 4BFF9A49  bl 0x82ea0060
	ctx.lr = 0x82EA661C;
	sub_82EA0060(ctx, base);
	// 82EA661C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA6620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA6624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA6628: 4E800020  blr
	return;
	// 82EA662C: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA6630: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82EA6634: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA6638: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA663C: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82EA6640: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA6644: 90A30050  stw r5, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82EA6648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA664C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA6650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA6654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6658 size=184
    let mut pc: u32 = 0x82EA6658;
    'dispatch: loop {
        match pc {
            0x82EA6658 => {
    //   block [0x82EA6658..0x82EA6710)
	// 82EA6658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA6660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA6664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA6668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA666C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6670: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA6674: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6678: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA667C: 7D445850  subf r10, r4, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82EA6680: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EA6684: 419A0074  beq cr6, 0x82ea66f8
	if ctx.cr[6].eq {
	pc = 0x82EA66F8; continue 'dispatch;
	}
	// 82EA6688: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA668C: 40990064  ble cr6, 0x82ea66f0
	if !ctx.cr[6].gt {
	pc = 0x82EA66F0; continue 'dispatch;
	}
	// 82EA6690: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EA6694: 419A0020  beq cr6, 0x82ea66b4
	if ctx.cr[6].eq {
	pc = 0x82EA66B4; continue 'dispatch;
	}
	// 82EA6698: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA669C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA66A0: 419A0014  beq cr6, 0x82ea66b4
	if ctx.cr[6].eq {
	pc = 0x82EA66B4; continue 'dispatch;
	}
	// 82EA66A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA66A8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA66AC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA66B0: 4BFFA309  bl 0x82ea09b8
	ctx.lr = 0x82EA66B4;
	sub_82EA09B8(ctx, base);
	// 82EA66B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA66B8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA66BC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA66C0: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82EA66C4: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 82EA66C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EA66CC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA66D0: 4BFFA291  bl 0x82ea0960
	ctx.lr = 0x82EA66D4;
	sub_82EA0960(ctx, base);
	// 82EA66D4: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82EA66D8: 390BFFE0  addi r8, r11, -0x20
	ctx.r[8].s64 = ctx.r[11].s64 + -32;
	// 82EA66DC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EA66E0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EA66E4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA66E8: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82EA66EC: 4800000C  b 0x82ea66f8
	pc = 0x82EA66F8; continue 'dispatch;
	// 82EA66F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA66F4: 4BFFFEBD  bl 0x82ea65b0
	ctx.lr = 0x82EA66F8;
	sub_82EA65B0(ctx, base);
	// 82EA66F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA66FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA6700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA6704: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EA6708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA670C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6710 size=96
    let mut pc: u32 = 0x82EA6710;
    'dispatch: loop {
        match pc {
            0x82EA6710 => {
    //   block [0x82EA6710..0x82EA6770)
	// 82EA6710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6714: 48301A59  bl 0x831a816c
	ctx.lr = 0x82EA6718;
	sub_831A8130(ctx, base);
	// 82EA6718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA671C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA6724: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EA6728: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA672C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA6730: 419A0018  beq cr6, 0x82ea6748
	if ctx.cr[6].eq {
	pc = 0x82EA6748; continue 'dispatch;
	}
	// 82EA6734: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6738: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA673C: 7D2A5851  subf. r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EA6740: 41820008  beq 0x82ea6748
	if ctx.cr[0].eq {
	pc = 0x82EA6748; continue 'dispatch;
	}
	// 82EA6744: 4BFFFE6D  bl 0x82ea65b0
	ctx.lr = 0x82EA6748;
	sub_82EA65B0(ctx, base);
	// 82EA6748: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82EA674C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA6750: 392BFFE0  addi r9, r11, -0x20
	ctx.r[9].s64 = ctx.r[11].s64 + -32;
	// 82EA6754: 995F0010  stb r10, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82EA6758: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EA675C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EA6760: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EA6764: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82EA6768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA676C: 48301A50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6770 size=132
    let mut pc: u32 = 0x82EA6770;
    'dispatch: loop {
        match pc {
            0x82EA6770 => {
    //   block [0x82EA6770..0x82EA67F4)
	// 82EA6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA6778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA677C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA6780: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6784: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82EA6788: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA678C: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA6790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA6794: 419A001C  beq cr6, 0x82ea67b0
	if ctx.cr[6].eq {
	pc = 0x82EA67B0; continue 'dispatch;
	}
	// 82EA6798: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA679C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EA67A0: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82EA67A4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA67A8: 91230050  stw r9, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82EA67AC: 48000014  b 0x82ea67c0
	pc = 0x82EA67C0; continue 'dispatch;
	// 82EA67B0: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 82EA67B4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82EA67B8: 4BFF9EA9  bl 0x82ea0660
	ctx.lr = 0x82EA67BC;
	sub_82EA0660(ctx, base);
	// 82EA67BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA67C0: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82EA67C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA67C8: 7D7F492E  stwx r11, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82EA67CC: 994B0010  stb r10, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82EA67D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA67D4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EA67D8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EA67DC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EA67E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA67E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA67E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA67EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA67F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA67F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA67F8 size=136
    let mut pc: u32 = 0x82EA67F8;
    'dispatch: loop {
        match pc {
            0x82EA67F8 => {
    //   block [0x82EA67F8..0x82EA6880)
	// 82EA67F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA67FC: 48301965  bl 0x831a8160
	ctx.lr = 0x82EA6800;
	sub_831A8130(ctx, base);
	// 82EA6800: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA6804: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6808: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82EA680C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA6810: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EA6814: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EA6818: 7C9BF1D6  mullw r4, r27, r30
	ctx.r[4].s64 = (ctx.r[27].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82EA681C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6820: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA6824: 4BFF9F0D  bl 0x82ea0730
	ctx.lr = 0x82EA6828;
	sub_82EA0730(ctx, base);
	// 82EA6828: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA682C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6830: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EA6834: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82EA6838: 48301CD9  bl 0x831a8510
	ctx.lr = 0x82EA683C;
	sub_831A8510(ctx, base);
	// 82EA683C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6840: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6844: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA6848: 409A001C  bne cr6, 0x82ea6864
	if !ctx.cr[6].eq {
	pc = 0x82EA6864; continue 'dispatch;
	}
	// 82EA684C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA6850: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6854: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA6858: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA685C: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82EA6860: 4BFF9F51  bl 0x82ea07b0
	ctx.lr = 0x82EA6864;
	sub_82EA07B0(ctx, base);
	// 82EA6864: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6868: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82EA686C: 556A0042  rlwinm r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6870: 7D49DB78  or r9, r10, r27
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[27].u64;
	// 82EA6874: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EA6878: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA687C: 48301934  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6880 size=152
    let mut pc: u32 = 0x82EA6880;
    'dispatch: loop {
        match pc {
            0x82EA6880 => {
    //   block [0x82EA6880..0x82EA6918)
	// 82EA6880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6884: 483018DD  bl 0x831a8160
	ctx.lr = 0x82EA6888;
	sub_831A8130(ctx, base);
	// 82EA6888: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA688C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6890: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA6894: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA6898: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA689C: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82EA68A0: 409A0008  bne cr6, 0x82ea68a8
	if !ctx.cr[6].eq {
	pc = 0x82EA68A8; continue 'dispatch;
	}
	// 82EA68A4: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82EA68A8: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA68AC: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82EA68B0: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EA68B4: 7C9AF1D6  mullw r4, r26, r30
	ctx.r[4].s64 = (ctx.r[26].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82EA68B8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA68BC: 4BFF9E75  bl 0x82ea0730
	ctx.lr = 0x82EA68C0;
	sub_82EA0730(ctx, base);
	// 82EA68C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA68C4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA68C8: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82EA68CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EA68D0: 48301C41  bl 0x831a8510
	ctx.lr = 0x82EA68D4;
	sub_831A8510(ctx, base);
	// 82EA68D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA68D8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA68DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EA68E0: 409A001C  bne cr6, 0x82ea68fc
	if !ctx.cr[6].eq {
	pc = 0x82EA68FC; continue 'dispatch;
	}
	// 82EA68E4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA68E8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA68EC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA68F0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EA68F4: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82EA68F8: 4BFF9EB9  bl 0x82ea07b0
	ctx.lr = 0x82EA68FC;
	sub_82EA07B0(ctx, base);
	// 82EA68FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6900: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EA6904: 556A0042  rlwinm r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6908: 7D49D378  or r9, r10, r26
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[26].u64;
	// 82EA690C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EA6910: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EA6914: 4830189C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6918 size=176
    let mut pc: u32 = 0x82EA6918;
    'dispatch: loop {
        match pc {
            0x82EA6918 => {
    //   block [0x82EA6918..0x82EA69C8)
	// 82EA6918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA691C: 48301849  bl 0x831a8164
	ctx.lr = 0x82EA6920;
	sub_831A8130(ctx, base);
	// 82EA6920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA6924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA6928: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA692C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EA6930: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EA6934: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EA6938: 419A001C  beq cr6, 0x82ea6954
	if ctx.cr[6].eq {
	pc = 0x82EA6954; continue 'dispatch;
	}
	// 82EA693C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA6940: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EA6944: 40980010  bge cr6, 0x82ea6954
	if !ctx.cr[6].lt {
	pc = 0x82EA6954; continue 'dispatch;
	}
	// 82EA6948: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA694C: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82EA6950: 48000020  b 0x82ea6970
	pc = 0x82EA6970; continue 'dispatch;
	// 82EA6954: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6958: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA695C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EA6960: 7C9DE1D6  mullw r4, r29, r28
	ctx.r[4].s64 = (ctx.r[29].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82EA6964: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA6968: 4BFF9DC9  bl 0x82ea0730
	ctx.lr = 0x82EA696C;
	sub_82EA0730(ctx, base);
	// 82EA696C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA6970: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA6974: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA6978: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA697C: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82EA6980: 48003DC9  bl 0x82eaa748
	ctx.lr = 0x82EA6984;
	sub_82EAA748(ctx, base);
	// 82EA6984: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA6988: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA698C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6990: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EA6994: 550700BE  clrlwi r7, r8, 2
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	// 82EA6998: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EA699C: 7CA7E9D6  mullw r5, r7, r29
	ctx.r[5].s64 = (ctx.r[7].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82EA69A0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EA69A4: 4BFF9E0D  bl 0x82ea07b0
	ctx.lr = 0x82EA69A8;
	sub_82EA07B0(ctx, base);
	// 82EA69A8: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA69AC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EA69B0: 54C50042  rlwinm r5, r6, 0, 1, 1
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA69B4: 7CA4DB78  or r4, r5, r27
	ctx.r[4].u64 = ctx.r[5].u64 | ctx.r[27].u64;
	// 82EA69B8: 7C83E378  or r3, r4, r28
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[28].u64;
	// 82EA69BC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EA69C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA69C4: 483017F0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA69C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA69C8 size=64
    let mut pc: u32 = 0x82EA69C8;
    'dispatch: loop {
        match pc {
            0x82EA69C8 => {
    //   block [0x82EA69C8..0x82EA6A08)
	// 82EA69C8: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82EA69CC: 3D203E39  lis r9, 0x3e39
	ctx.r[9].s64 = 1043922944;
	// 82EA69D0: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82EA69D4: 6127B193  ori r7, r9, 0xb193
	ctx.r[7].u64 = ctx.r[9].u64 | 45459;
	// 82EA69D8: 816AC3F0  lwz r11, -0x3c10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15376 as u32) ) } as u64;
	// 82EA69DC: 7CCB39D6  mullw r6, r11, r7
	ctx.r[6].s64 = (ctx.r[11].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82EA69E0: C008DFB4  lfs f0, -0x204c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA69E4: 20A63039  subfic r5, r6, 0x3039
	ctx.xer.ca = ctx.r[6].u32 <= 12345 as u32;
	ctx.r[5].s64 = (12345 as i64) - ctx.r[6].s64;
	// 82EA69E8: 54AB007E  clrlwi r11, r5, 1
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x7FFFFFFFu64;
	// 82EA69EC: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82EA69F0: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EA69F4: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82EA69F8: 916AC3F0  stw r11, -0x3c10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15376 as u32), ctx.r[11].u32 ) };
	// 82EA69FC: FD606018  frsp f11, f12
	ctx.f[11].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82EA6A00: EC2B0032  fmuls f1, f11, f0
	ctx.f[1].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82EA6A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6A08 size=144
    let mut pc: u32 = 0x82EA6A08;
    'dispatch: loop {
        match pc {
            0x82EA6A08 => {
    //   block [0x82EA6A08..0x82EA6A98)
	// 82EA6A08: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA6A0C: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA6A10: 554B4E3E  rlwinm r11, r10, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x007FFFFFu64;
	// 82EA6A14: 5549007E  clrlwi r9, r10, 1
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EA6A18: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 82EA6A1C: 38A9FFFF  addi r5, r9, -1
	ctx.r[5].s64 = ctx.r[9].s64 + -1;
	// 82EA6A20: 212B0017  subfic r9, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[9].s64 = (23 as i64) - ctx.r[11].s64;
	// 82EA6A24: 3D00FF80  lis r8, -0x80
	ctx.r[8].s64 = -8388608;
	// 82EA6A28: 7D24FE70  srawi r4, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 82EA6A2C: 7CA3FE70  srawi r3, r5, 0x1f
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[5].s32 >> 31) as i64;
	// 82EA6A30: 7C8720F8  nor r7, r4, r4
	ctx.r[7].u64 = !(ctx.r[4].u64 | ctx.r[4].u64);
	// 82EA6A34: 7D4A1878  andc r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[3].u64;
	// 82EA6A38: 54E406FE  clrlwi r4, r7, 0x1b
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82EA6A3C: 7CE54838  and r5, r7, r9
	ctx.r[5].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82EA6A40: 54840776  rlwinm r4, r4, 0, 0x1d, 0x1b
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6A44: 3CC00080  lis r6, 0x80
	ctx.r[6].s64 = 8388608;
	// 82EA6A48: 7C652050  subf r3, r5, r4
	ctx.r[3].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82EA6A4C: 7D091E30  sraw r9, r8, r3
	tmp.u32 = ctx.r[3].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[8].s32 >> tmp.u32) as i64;
	// 82EA6A50: 7D253B38  orc r5, r9, r7
	ctx.r[5].u64 = ctx.r[9].u64 | ~ctx.r[7].u64;
	// 82EA6A54: 7D49FE70  srawi r9, r10, 0x1f
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 31) as i64;
	// 82EA6A58: 7D472878  andc r7, r10, r5
	ctx.r[7].u64 = ctx.r[10].u64 & !ctx.r[5].u64;
	// 82EA6A5C: 7D68FE70  srawi r8, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 82EA6A60: 7CC45E30  sraw r4, r6, r11
	tmp.u32 = ctx.r[11].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[6].s32 >> tmp.u32) as i64;
	// 82EA6A64: 3867FFFF  addi r3, r7, -1
	ctx.r[3].s64 = ctx.r[7].s64 + -1;
	// 82EA6A68: 7D0B4838  and r11, r8, r9
	ctx.r[11].u64 = ctx.r[8].u64 & ctx.r[9].u64;
	// 82EA6A6C: 7C871878  andc r7, r4, r3
	ctx.r[7].u64 = ctx.r[4].u64 & !ctx.r[3].u64;
	// 82EA6A70: 55660010  rlwinm r6, r11, 0, 0, 8
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6A74: 7CEB4838  and r11, r7, r9
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82EA6A78: 54C60080  rlwinm r6, r6, 0, 2, 0
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6A7C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA6A80: 7C834078  andc r3, r4, r8
	ctx.r[3].u64 = ctx.r[4].u64 & !ctx.r[8].u64;
	// 82EA6A84: 7C6B2838  and r11, r3, r5
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[5].u64;
	// 82EA6A88: 7D6A3378  or r10, r11, r6
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 82EA6A8C: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 82EA6A90: C021FFF0  lfs f1, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EA6A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6A98 size=188
    let mut pc: u32 = 0x82EA6A98;
    'dispatch: loop {
        match pc {
            0x82EA6A98 => {
    //   block [0x82EA6A98..0x82EA6B54)
	// 82EA6A98: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EA6A9C: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA6AA0: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA6AA4: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 82EA6AA8: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 82EA6AAC: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 82EA6AB0: 390BFFE9  addi r8, r11, -0x17
	ctx.r[8].s64 = ctx.r[11].s64 + -23;
	// 82EA6AB4: 38AAFFFF  addi r5, r10, -1
	ctx.r[5].s64 = ctx.r[10].s64 + -1;
	// 82EA6AB8: 3888FFFF  addi r4, r8, -1
	ctx.r[4].s64 = ctx.r[8].s64 + -1;
	// 82EA6ABC: 3CE0FF80  lis r7, -0x80
	ctx.r[7].s64 = -8388608;
	// 82EA6AC0: 7C8AFE70  srawi r10, r4, 0x1f
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[4].s32 >> 31) as i64;
	// 82EA6AC4: 206B0017  subfic r3, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[3].s64 = (23 as i64) - ctx.r[11].s64;
	// 82EA6AC8: 554406FE  clrlwi r4, r10, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EA6ACC: 7C635038  and r3, r3, r10
	ctx.r[3].u64 = ctx.r[3].u64 & ctx.r[10].u64;
	// 82EA6AD0: 54840776  rlwinm r4, r4, 0, 0x1d, 0x1b
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6AD4: 7D5F50F8  nor r31, r10, r10
	ctx.r[31].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82EA6AD8: 7C832050  subf r4, r3, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 82EA6ADC: 7CA5FE70  srawi r5, r5, 0x1f
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 31) as i64;
	// 82EA6AE0: 7CEA2630  sraw r10, r7, r4
	tmp.u32 = ctx.r[4].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[7].s32 >> tmp.u32) as i64;
	// 82EA6AE4: 7D252878  andc r5, r9, r5
	ctx.r[5].u64 = ctx.r[9].u64 & !ctx.r[5].u64;
	// 82EA6AE8: 7D44FB78  or r4, r10, r31
	ctx.r[4].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 82EA6AEC: 3CC00080  lis r6, 0x80
	ctx.r[6].s64 = 8388608;
	// 82EA6AF0: 7CA72078  andc r7, r5, r4
	ctx.r[7].u64 = ctx.r[5].u64 & !ctx.r[4].u64;
	// 82EA6AF4: 7CAAFE70  srawi r10, r5, 0x1f
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[5].s32 >> 31) as i64;
	// 82EA6AF8: 7D69FE70  srawi r9, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 82EA6AFC: 7CCB5E30  sraw r11, r6, r11
	tmp.u32 = ctx.r[11].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[6].s32 >> tmp.u32) as i64;
	// 82EA6B00: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82EA6B04: 7D2648F8  nor r6, r9, r9
	ctx.r[6].u64 = !(ctx.r[9].u64 | ctx.r[9].u64);
	// 82EA6B08: 7D673878  andc r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 & !ctx.r[7].u64;
	// 82EA6B0C: 7CCB2838  and r11, r6, r5
	ctx.r[11].u64 = ctx.r[6].u64 & ctx.r[5].u64;
	// 82EA6B10: 7CE75038  and r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 82EA6B14: 7D255038  and r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82EA6B18: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EA6B1C: 7CA93378  or r9, r5, r6
	ctx.r[9].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 82EA6B20: 7FE84038  and r8, r31, r8
	ctx.r[8].u64 = ctx.r[31].u64 & ctx.r[8].u64;
	// 82EA6B24: 512B0210  rlwimi r11, r9, 0, 8, 8
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x0000000000800000) | (ctx.r[11].u64 & 0xFFFFFFFFFF7FFFFF);
	// 82EA6B28: 5567023E  clrlwi r7, r11, 8
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 82EA6B2C: 7CE62038  and r6, r7, r4
	ctx.r[6].u64 = ctx.r[7].u64 & ctx.r[4].u64;
	// 82EA6B30: 54CB083C  slwi r11, r6, 1
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA6B34: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82EA6B38: 7CA45378  or r4, r5, r10
	ctx.r[4].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 82EA6B3C: 7D662050  subf r11, r6, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[6].s64;
	// 82EA6B40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA6B44: 7D6A1E30  sraw r10, r11, r3
	tmp.u32 = ctx.r[3].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> tmp.u32) as i64;
	// 82EA6B48: 7D434030  slw r3, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[10].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82EA6B4C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EA6B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6B58 size=144
    let mut pc: u32 = 0x82EA6B58;
    'dispatch: loop {
        match pc {
            0x82EA6B58 => {
    //   block [0x82EA6B58..0x82EA6BE8)
	// 82EA6B58: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA6B5C: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA6B60: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 82EA6B64: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 82EA6B68: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 82EA6B6C: 390BFFE9  addi r8, r11, -0x17
	ctx.r[8].s64 = ctx.r[11].s64 + -23;
	// 82EA6B70: 38AAFFFF  addi r5, r10, -1
	ctx.r[5].s64 = ctx.r[10].s64 + -1;
	// 82EA6B74: 3888FFFF  addi r4, r8, -1
	ctx.r[4].s64 = ctx.r[8].s64 + -1;
	// 82EA6B78: 3CE0FF80  lis r7, -0x80
	ctx.r[7].s64 = -8388608;
	// 82EA6B7C: 7C8AFE70  srawi r10, r4, 0x1f
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[4].s32 >> 31) as i64;
	// 82EA6B80: 206B0017  subfic r3, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[3].s64 = (23 as i64) - ctx.r[11].s64;
	// 82EA6B84: 7CA5FE70  srawi r5, r5, 0x1f
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 31) as i64;
	// 82EA6B88: 7C645038  and r4, r3, r10
	ctx.r[4].u64 = ctx.r[3].u64 & ctx.r[10].u64;
	// 82EA6B8C: 554306FE  clrlwi r3, r10, 0x1b
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EA6B90: 7D292878  andc r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[5].u64;
	// 82EA6B94: 54630776  rlwinm r3, r3, 0, 0x1d, 0x1b
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6B98: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EA6B9C: 7CA41850  subf r5, r4, r3
	ctx.r[5].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82EA6BA0: 7D4350F8  nor r3, r10, r10
	ctx.r[3].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82EA6BA4: 7CEA2E30  sraw r10, r7, r5
	tmp.u32 = ctx.r[5].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[7].s32 >> tmp.u32) as i64;
	// 82EA6BA8: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82EA6BAC: 7D451B78  or r5, r10, r3
	ctx.r[5].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 82EA6BB0: 50C7B810  rlwimi r7, r6, 0x17, 0, 8
	ctx.r[7].u64 = (((ctx.r[6].u32).rotate_left(23) as u64) & 0x00000000FF800000) | (ctx.r[7].u64 & 0xFFFFFFFF007FFFFF);
	// 82EA6BB4: 7D2AFE70  srawi r10, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 82EA6BB8: 7CE92838  and r9, r7, r5
	ctx.r[9].u64 = ctx.r[7].u64 & ctx.r[5].u64;
	// 82EA6BBC: 7D67FE70  srawi r7, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 82EA6BC0: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA6BC4: 7C664038  and r6, r3, r8
	ctx.r[6].u64 = ctx.r[3].u64 & ctx.r[8].u64;
	// 82EA6BC8: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82EA6BCC: 7CA35378  or r3, r5, r10
	ctx.r[3].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 82EA6BD0: 7D691850  subf r11, r9, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[9].s64;
	// 82EA6BD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EA6BD8: 7D6A3878  andc r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 & !ctx.r[7].u64;
	// 82EA6BDC: 7D492630  sraw r9, r10, r4
	tmp.u32 = ctx.r[4].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> tmp.u32) as i64;
	// 82EA6BE0: 7D233030  slw r3, r9, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[9].u32) << ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 82EA6BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6BE8 size=68
    let mut pc: u32 = 0x82EA6BE8;
    'dispatch: loop {
        match pc {
            0x82EA6BE8 => {
    //   block [0x82EA6BE8..0x82EA6C2C)
	// 82EA6BE8: FC000A10  fabs f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82EA6BEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA6BF0: FDA01210  fabs f13, f2
	ctx.f[13].u64 = ctx.f[2].u64 & !0x8000_0000_0000_0000u64;
	// 82EA6BF4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EA6BF8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EA6BFC: C18BC3C8  lfs f12, -0x3c38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA6C00: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82EA6C04: 41990028  bgt cr6, 0x82ea6c2c
	if ctx.cr[6].gt {
		sub_82EA6C2C(ctx, base);
		return;
	}
	// 82EA6C08: ED6D602A  fadds f11, f13, f12
	ctx.f[11].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 82EA6C0C: C1AAFF80  lfs f13, -0x80(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-128 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA6C10: C189FF7C  lfs f12, -0x84(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-132 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA6C14: ED405824  fdivs f10, f0, f11
	ctx.f[10].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 82EA6C18: ED2A02B2  fmuls f9, f10, f10
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[10].f64) as f32) as f64);
	// 82EA6C1C: ED09537C  fnmsubs f8, f9, f13, f10
	ctx.f[8].f64 = -(((ctx.f[9].f64 * ctx.f[13].f64 - ctx.f[10].f64) as f32) as f64);
	// 82EA6C20: ECE902B2  fmuls f7, f9, f10
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[10].f64) as f32) as f64);
	// 82EA6C24: EC07433C  fnmsubs f0, f7, f12, f8
	ctx.f[0].f64 = -(((ctx.f[7].f64 * ctx.f[12].f64 - ctx.f[8].f64) as f32) as f64);
	// 82EA6C28: 48000030  b 0x82ea6c58
	sub_82EA6C2C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6C2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6C2C size=88
    let mut pc: u32 = 0x82EA6C2C;
    'dispatch: loop {
        match pc {
            0x82EA6C2C => {
    //   block [0x82EA6C2C..0x82EA6C84)
	// 82EA6C2C: ED40602A  fadds f10, f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[10].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 82EA6C30: C00AFF80  lfs f0, -0x80(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA6C34: C189FF7C  lfs f12, -0x84(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-132 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA6C38: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82EA6C3C: C168D96C  lfs f11, -0x2694(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-9876 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EA6C40: ED2D5024  fdivs f9, f13, f10
	ctx.f[9].f64 = ((ctx.f[13].f64 / ctx.f[10].f64) as f32) as f64;
	// 82EA6C44: ED090272  fmuls f8, f9, f9
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[9].f64) as f32) as f64);
	// 82EA6C48: ECE8483C  fnmsubs f7, f8, f0, f9
	ctx.f[7].f64 = -(((ctx.f[8].f64 * ctx.f[0].f64 - ctx.f[9].f64) as f32) as f64);
	// 82EA6C4C: ECC80272  fmuls f6, f8, f9
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 82EA6C50: ECA63B3C  fnmsubs f5, f6, f12, f7
	ctx.f[5].f64 = -(((ctx.f[6].f64 * ctx.f[12].f64 - ctx.f[7].f64) as f32) as f64);
	// 82EA6C54: EC0B2828  fsubs f0, f11, f5
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[5].f64) as f32) as f64);
	// 82EA6C58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA6C5C: C18B08A4  lfs f12, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA6C60: FF026000  fcmpu cr6, f2, f12
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[12].f64);
	// 82EA6C64: 40980010  bge cr6, 0x82ea6c74
	if !ctx.cr[6].lt {
	pc = 0x82EA6C74; continue 'dispatch;
	}
	// 82EA6C68: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82EA6C6C: C1AB7590  lfs f13, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA6C70: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82EA6C74: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 82EA6C78: 4098000C  bge cr6, 0x82ea6c84
	if !ctx.cr[6].lt {
		sub_82EA6C84(ctx, base);
		return;
	}
	// 82EA6C7C: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82EA6C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6C84 size=8
    let mut pc: u32 = 0x82EA6C84;
    'dispatch: loop {
        match pc {
            0x82EA6C84 => {
    //   block [0x82EA6C84..0x82EA6C8C)
	// 82EA6C84: FC200090  fmr f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82EA6C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6C90 size=72
    let mut pc: u32 = 0x82EA6C90;
    'dispatch: loop {
        match pc {
            0x82EA6C90 => {
    //   block [0x82EA6C90..0x82EA6CD8)
	// 82EA6C90: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA6C94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA6C98: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EA6C9C: 40990030  ble cr6, 0x82ea6ccc
	if !ctx.cr[6].gt {
	pc = 0x82EA6CCC; continue 'dispatch;
	}
	// 82EA6CA0: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82EA6CA4: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA6CA8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA6CAC: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA6CB0: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6CB4: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA6CB8: 419A0020  beq cr6, 0x82ea6cd8
	if ctx.cr[6].eq {
		sub_82EA6CD8(ctx, base);
		return;
	}
	// 82EA6CBC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA6CC0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA6CC4: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EA6CC8: 4198FFDC  blt cr6, 0x82ea6ca4
	if ctx.cr[6].lt {
	pc = 0x82EA6CA4; continue 'dispatch;
	}
	// 82EA6CCC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA6CD0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA6CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6CD8 size=12
    let mut pc: u32 = 0x82EA6CD8;
    'dispatch: loop {
        match pc {
            0x82EA6CD8 => {
    //   block [0x82EA6CD8..0x82EA6CE4)
	// 82EA6CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA6CDC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA6CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6CE8 size=64
    let mut pc: u32 = 0x82EA6CE8;
    'dispatch: loop {
        match pc {
            0x82EA6CE8 => {
    //   block [0x82EA6CE8..0x82EA6D28)
	// 82EA6CE8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA6CEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA6CF0: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82EA6CF4: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA6CF8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA6CFC: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA6D00: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6D04: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA6D08: 419A0020  beq cr6, 0x82ea6d28
	if ctx.cr[6].eq {
		sub_82EA6D28(ctx, base);
		return;
	}
	// 82EA6D0C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA6D10: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA6D14: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82EA6D18: 4198FFDC  blt cr6, 0x82ea6cf4
	if ctx.cr[6].lt {
	pc = 0x82EA6CF4; continue 'dispatch;
	}
	// 82EA6D1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA6D20: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA6D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6D28 size=12
    let mut pc: u32 = 0x82EA6D28;
    'dispatch: loop {
        match pc {
            0x82EA6D28 => {
    //   block [0x82EA6D28..0x82EA6D34)
	// 82EA6D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA6D2C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA6D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA6D38 size=64
    let mut pc: u32 = 0x82EA6D38;
    'dispatch: loop {
        match pc {
            0x82EA6D38 => {
    //   block [0x82EA6D38..0x82EA6D78)
	// 82EA6D38: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EA6D3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA6D40: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82EA6D44: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA6D48: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82EA6D4C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EA6D50: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EA6D54: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EA6D58: 419A0020  beq cr6, 0x82ea6d78
	if ctx.cr[6].eq {
		sub_82EA6D78(ctx, base);
		return;
	}
	// 82EA6D5C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EA6D60: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA6D64: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82EA6D68: 4198FFDC  blt cr6, 0x82ea6d44
	if ctx.cr[6].lt {
	pc = 0x82EA6D44; continue 'dispatch;
	}
	// 82EA6D6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA6D70: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA6D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6D78 size=12
    let mut pc: u32 = 0x82EA6D78;
    'dispatch: loop {
        match pc {
            0x82EA6D78 => {
    //   block [0x82EA6D78..0x82EA6D84)
	// 82EA6D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA6D7C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA6D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6D88 size=112
    let mut pc: u32 = 0x82EA6D88;
    'dispatch: loop {
        match pc {
            0x82EA6D88 => {
    //   block [0x82EA6D88..0x82EA6DF8)
	// 82EA6D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA6D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA6D94: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82EA6D98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA6D9C: 4BFFFF4D  bl 0x82ea6ce8
	ctx.lr = 0x82EA6DA0;
	sub_82EA6CE8(ctx, base);
	// 82EA6DA0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA6DA8: 419A0034  beq cr6, 0x82ea6ddc
	if ctx.cr[6].eq {
	pc = 0x82EA6DDC; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA6DF8 size=112
    let mut pc: u32 = 0x82EA6DF8;
    'dispatch: loop {
        match pc {
            0x82EA6DF8 => {
    //   block [0x82EA6DF8..0x82EA6E68)
	// 82EA6DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA6DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA6E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA6E04: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82EA6E08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EA6E0C: 4BFFFF2D  bl 0x82ea6d38
	ctx.lr = 0x82EA6E10;
	sub_82EA6D38(ctx, base);
	// 82EA6E10: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA6E14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA6E18: 419A0034  beq cr6, 0x82ea6e4c
	if ctx.cr[6].eq {
	pc = 0x82EA6E4C; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6E68 size=68
    let mut pc: u32 = 0x82EA6E68;
    'dispatch: loop {
        match pc {
            0x82EA6E68 => {
    //   block [0x82EA6E68..0x82EA6EAC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6EB0 size=80
    let mut pc: u32 = 0x82EA6EB0;
    'dispatch: loop {
        match pc {
            0x82EA6EB0 => {
    //   block [0x82EA6EB0..0x82EA6F00)
	// 82EA6EB0: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6F00 size=56
    let mut pc: u32 = 0x82EA6F00;
    'dispatch: loop {
        match pc {
            0x82EA6F00 => {
    //   block [0x82EA6F00..0x82EA6F38)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6F38 size=56
    let mut pc: u32 = 0x82EA6F38;
    'dispatch: loop {
        match pc {
            0x82EA6F38 => {
    //   block [0x82EA6F38..0x82EA6F70)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6F70 size=116
    let mut pc: u32 = 0x82EA6F70;
    'dispatch: loop {
        match pc {
            0x82EA6F70 => {
    //   block [0x82EA6F70..0x82EA6FE4)
	// 82EA6F70: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA6FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA6FE8 size=140
    let mut pc: u32 = 0x82EA6FE8;
    'dispatch: loop {
        match pc {
            0x82EA6FE8 => {
    //   block [0x82EA6FE8..0x82EA7074)
	// 82EA6FE8: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EA7078 size=100
    let mut pc: u32 = 0x82EA7078;
    'dispatch: loop {
        match pc {
            0x82EA7078 => {
    //   block [0x82EA7078..0x82EA70DC)
	// 82EA7078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA707C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA7080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA7084: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82EA7088: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA708C: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA7090: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EA7094: 41980030  blt cr6, 0x82ea70c4
	if ctx.cr[6].lt {
	pc = 0x82EA70C4; continue 'dispatch;
	}
	// 82EA7098: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA709C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA70A0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82EA70A4: 4199000C  bgt cr6, 0x82ea70b0
	if ctx.cr[6].gt {
	pc = 0x82EA70B0; continue 'dispatch;
	}
	// 82EA70A8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82EA70AC: C00B7590  lfs f0, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA70B0: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82EA70B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA70B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA70BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA70C0: 4E800020  blr
	return;
	// 82EA70C4: 483041E5  bl 0x831ab2a8
	ctx.lr = 0x82EA70C8;
	sub_831AB2A8(ctx, base);
	// 82EA70C8: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EA70CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA70D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA70D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA70D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA70E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EA70E0 size=108
    let mut pc: u32 = 0x82EA70E0;
    'dispatch: loop {
        match pc {
            0x82EA70E0 => {
    //   block [0x82EA70E0..0x82EA714C)
	// 82EA70E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA70E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA70E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA70EC: C003000C  lfs f0, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA70F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA70F4: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82EA70F8: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA70FC: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82EA7100: FD800A10  fabs f12, f1
	ctx.f[12].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82EA7104: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 82EA7108: 41980020  blt cr6, 0x82ea7128
	if ctx.cr[6].lt {
	pc = 0x82EA7128; continue 'dispatch;
	}
	// 82EA710C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA7110: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA7114: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82EA7118: 41990018  bgt cr6, 0x82ea7130
	if ctx.cr[6].gt {
	pc = 0x82EA7130; continue 'dispatch;
	}
	// 82EA711C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82EA7120: C1AB7590  lfs f13, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA7124: 4800000C  b 0x82ea7130
	pc = 0x82EA7130; continue 'dispatch;
	// 82EA7128: 48304181  bl 0x831ab2a8
	ctx.lr = 0x82EA712C;
	sub_831AB2A8(ctx, base);
	// 82EA712C: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EA7130: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA7134: C00B9524  lfs f0, -0x6adc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA7138: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82EA713C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EA7140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EA7144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EA7148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EA7150 size=128
    let mut pc: u32 = 0x82EA7150;
    'dispatch: loop {
        match pc {
            0x82EA7150 => {
    //   block [0x82EA7150..0x82EA71D0)
	// 82EA7150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA7154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA7158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA715C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA7160: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EA7164: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA7168: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EA716C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA7170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA7174: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA7178: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82EA717C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EA7180: 48301C49  bl 0x831a8dc8
	ctx.lr = 0x82EA7184;
	sub_831A8DC8(ctx, base);
	// 82EA7184: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EA7188: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EA718C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82EA7190: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA71D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA71D0 size=156
    let mut pc: u32 = 0x82EA71D0;
    'dispatch: loop {
        match pc {
            0x82EA71D0 => {
    //   block [0x82EA71D0..0x82EA726C)
	// 82EA71D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA71D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA71D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA71DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA71E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA71E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA71E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EA71EC: 4BFFFB4D  bl 0x82ea6d38
	ctx.lr = 0x82EA71F0;
	sub_82EA6D38(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA7270 size=476
    let mut pc: u32 = 0x82EA7270;
    'dispatch: loop {
        match pc {
            0x82EA7270 => {
    //   block [0x82EA7270..0x82EA744C)
	// 82EA7270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA7274: 48300EF5  bl 0x831a8168
	ctx.lr = 0x82EA7278;
	sub_831A8130(ctx, base);
	// 82EA7278: C1A40014  lfs f13, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA727C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA7280: C1840000  lfs f12, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA7284: EC0C682A  fadds f0, f12, f13
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 82EA7288: C1440028  lfs f10, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EA728C: C16B08A4  lfs f11, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EA7290: EC00502A  fadds f0, f0, f10
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64;
	// 82EA7294: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82EA7298: 40990074  ble cr6, 0x82ea730c
	if !ctx.cr[6].gt {
	pc = 0x82EA730C; continue 'dispatch;
	}
	// 82EA729C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EA72A0: C1A40018  lfs f13, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA72A4: C1840024  lfs f12, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA72A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EA72AC: ED4D6028  fsubs f10, f13, f12
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82EA72B0: C1640020  lfs f11, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EA72B4: C1240008  lfs f9, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EA72B8: 3921FFC0  addi r9, r1, -0x40
	ctx.r[9].s64 = ctx.r[1].s64 + -64;
	// 82EA72BC: C1040004  lfs f8, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82EA72C0: ECEB4828  fsubs f7, f11, f9
	ctx.f[7].f64 = (((ctx.f[11].f64 - ctx.f[9].f64) as f32) as f64);
	// 82EA72C4: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA72C8: ECC0682A  fadds f6, f0, f13
	ctx.f[6].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82EA72CC: C00A9450  lfs f0, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA72D0: C0A40010  lfs f5, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82EA72D4: EC882828  fsubs f4, f8, f5
	ctx.f[4].f64 = (((ctx.f[8].f64 - ctx.f[5].f64) as f32) as f64);
	// 82EA72D8: EC60302C  fsqrts f3, f6
	ctx.f[3].f64 = ((ctx.f[6].f64).sqrt() as f32) as f64;
	// 82EA72DC: EC401824  fdivs f2, f0, f3
	ctx.f[2].f64 = ((ctx.f[0].f64 / ctx.f[3].f64) as f32) as f64;
	// 82EA72E0: EC230032  fmuls f1, f3, f0
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 82EA72E4: D021FFCC  stfs f1, -0x34(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 82EA72E8: EC0A00B2  fmuls f0, f10, f2
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[2].f64) as f32) as f64);
	// 82EA72EC: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82EA72F0: EDA700B2  fmuls f13, f7, f2
	ctx.f[13].f64 = (((ctx.f[7].f64 * ctx.f[2].f64) as f32) as f64);
	// 82EA72F4: D1A1FFC4  stfs f13, -0x3c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82EA72F8: ED8400B2  fmuls f12, f4, f2
	ctx.f[12].f64 = (((ctx.f[4].f64 * ctx.f[2].f64) as f32) as f64);
	// 82EA72FC: D181FFC8  stfs f12, -0x38(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7450 size=4
    let mut pc: u32 = 0x82EA7450;
    'dispatch: loop {
        match pc {
            0x82EA7450 => {
    //   block [0x82EA7450..0x82EA7454)
	// 82EA7450: 4BFFFE20  b 0x82ea7270
	sub_82EA7270(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA7458 size=104
    let mut pc: u32 = 0x82EA7458;
    'dispatch: loop {
        match pc {
            0x82EA7458 => {
    //   block [0x82EA7458..0x82EA74C0)
	// 82EA7458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA7460: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA7464: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA7468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA746C: 4BFFFE05  bl 0x82ea7270
	ctx.lr = 0x82EA7470;
	sub_82EA7270(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA74C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA74C0 size=340
    let mut pc: u32 = 0x82EA74C0;
    'dispatch: loop {
        match pc {
            0x82EA74C0 => {
    //   block [0x82EA74C0..0x82EA7614)
	// 82EA74C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA74C4: 48300CA9  bl 0x831a816c
	ctx.lr = 0x82EA74C8;
	sub_831A8130(ctx, base);
	// 82EA74C8: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82EA74CC: 483015AD  bl 0x831a8a78
	ctx.lr = 0x82EA74D0;
	sub_831A8A40(ctx, base);
	// 82EA74D0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA74D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EA74D8: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 82EA74DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EA74E0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EA74E4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EA74E8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EA7618 size=220
    let mut pc: u32 = 0x82EA7618;
    'dispatch: loop {
        match pc {
            0x82EA7618 => {
    //   block [0x82EA7618..0x82EA76F4)
	// 82EA7618: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EA761C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EA7620: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EA7624: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82EA7628: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82EA762C: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EA7630: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA7634: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82EA7638: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82EA763C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EA7640: 40980010  bge cr6, 0x82ea7650
	if !ctx.cr[6].lt {
	pc = 0x82EA7650; continue 'dispatch;
	}
	// 82EA7644: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA7648: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82EA764C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EA7650: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82EA7654: 40980008  bge cr6, 0x82ea765c
	if !ctx.cr[6].lt {
	pc = 0x82EA765C; continue 'dispatch;
	}
	// 82EA7658: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82EA765C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82EA7660: 38E1FFE0  addi r7, r1, -0x20
	ctx.r[7].s64 = ctx.r[1].s64 + -32;
	// 82EA7664: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA7668: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EA766C: 38A1FFE0  addi r5, r1, -0x20
	ctx.r[5].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA76F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA76F8 size=484
    let mut pc: u32 = 0x82EA76F8;
    'dispatch: loop {
        match pc {
            0x82EA76F8 => {
    //   block [0x82EA76F8..0x82EA78DC)
	// 82EA76F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA76FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EA7700: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EA7704: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EA7708: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EA770C: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA78E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA78E0 size=368
    let mut pc: u32 = 0x82EA78E0;
    'dispatch: loop {
        match pc {
            0x82EA78E0 => {
    //   block [0x82EA78E0..0x82EA7A50)
	// 82EA78E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA78E4: 48300885  bl 0x831a8168
	ctx.lr = 0x82EA78E8;
	sub_831A8130(ctx, base);
	// 82EA78E8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82EA78EC: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7A50 size=44
    let mut pc: u32 = 0x82EA7A50;
    'dispatch: loop {
        match pc {
            0x82EA7A50 => {
    //   block [0x82EA7A50..0x82EA7A7C)
	// 82EA7A50: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EA7A54: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7A58: 3D404358  lis r10, 0x4358
	ctx.r[10].s64 = 1129840640;
	// 82EA7A5C: 38CB0028  addi r6, r11, 0x28
	ctx.r[6].s64 = ctx.r[11].s64 + 40;
	// 82EA7A60: 6148CCCD  ori r8, r10, 0xcccd
	ctx.r[8].u64 = ctx.r[10].u64 | 52429;
	// 82EA7A64: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82EA7A68: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EA7A6C: 39660400  addi r11, r6, 0x400
	ctx.r[11].s64 = ctx.r[6].s64 + 1024;
	// 82EA7A70: 4099000C  ble cr6, 0x82ea7a7c
	if !ctx.cr[6].gt {
		sub_82EA7A7C(ctx, base);
		return;
	}
	// 82EA7A74: 39460200  addi r10, r6, 0x200
	ctx.r[10].s64 = ctx.r[6].s64 + 512;
	// 82EA7A78: 48000008  b 0x82ea7a80
	sub_82EA7A7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7A7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7A7C size=36
    let mut pc: u32 = 0x82EA7A7C;
    'dispatch: loop {
        match pc {
            0x82EA7A7C => {
    //   block [0x82EA7A7C..0x82EA7AA0)
	// 82EA7A7C: 39660200  addi r11, r6, 0x200
	ctx.r[11].s64 = ctx.r[6].s64 + 512;
	// 82EA7A80: 5567F87E  srwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA7A84: 5548F87E  srwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA7A88: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82EA7A8C: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7A90: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA7A94: 4099000C  ble cr6, 0x82ea7aa0
	if !ctx.cr[6].gt {
		sub_82EA7AA0(ctx, base);
		return;
	}
	// 82EA7A98: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EA7A9C: 48000008  b 0x82ea7aa4
	sub_82EA7AA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7AA0 size=36
    let mut pc: u32 = 0x82EA7AA0;
    'dispatch: loop {
        match pc {
            0x82EA7AA0 => {
    //   block [0x82EA7AA0..0x82EA7AC4)
	// 82EA7AA0: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82EA7AA4: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA7AA8: 5568F87E  srwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA7AAC: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EA7AB0: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7AB4: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA7AB8: 4099000C  ble cr6, 0x82ea7ac4
	if !ctx.cr[6].gt {
		sub_82EA7AC4(ctx, base);
		return;
	}
	// 82EA7ABC: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EA7AC0: 48000008  b 0x82ea7ac8
	sub_82EA7AC4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7AC4 size=36
    let mut pc: u32 = 0x82EA7AC4;
    'dispatch: loop {
        match pc {
            0x82EA7AC4 => {
    //   block [0x82EA7AC4..0x82EA7AE8)
	// 82EA7AC4: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82EA7AC8: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA7ACC: 5568F87E  srwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA7AD0: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EA7AD4: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7AD8: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA7ADC: 4099000C  ble cr6, 0x82ea7ae8
	if !ctx.cr[6].gt {
		sub_82EA7AE8(ctx, base);
		return;
	}
	// 82EA7AE0: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EA7AE4: 48000008  b 0x82ea7aec
	sub_82EA7AE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7AE8 size=36
    let mut pc: u32 = 0x82EA7AE8;
    'dispatch: loop {
        match pc {
            0x82EA7AE8 => {
    //   block [0x82EA7AE8..0x82EA7B0C)
	// 82EA7AE8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82EA7AEC: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA7AF0: 5568F87E  srwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA7AF4: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EA7AF8: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7AFC: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA7B00: 4099000C  ble cr6, 0x82ea7b0c
	if !ctx.cr[6].gt {
		sub_82EA7B0C(ctx, base);
		return;
	}
	// 82EA7B04: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EA7B08: 48000008  b 0x82ea7b10
	sub_82EA7B0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7B0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7B0C size=36
    let mut pc: u32 = 0x82EA7B0C;
    'dispatch: loop {
        match pc {
            0x82EA7B0C => {
    //   block [0x82EA7B0C..0x82EA7B30)
	// 82EA7B0C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82EA7B10: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA7B14: 5568F87E  srwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA7B18: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EA7B1C: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7B20: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA7B24: 4099000C  ble cr6, 0x82ea7b30
	if !ctx.cr[6].gt {
		sub_82EA7B30(ctx, base);
		return;
	}
	// 82EA7B28: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EA7B2C: 48000008  b 0x82ea7b34
	sub_82EA7B30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7B30 size=36
    let mut pc: u32 = 0x82EA7B30;
    'dispatch: loop {
        match pc {
            0x82EA7B30 => {
    //   block [0x82EA7B30..0x82EA7B54)
	// 82EA7B30: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82EA7B34: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EA7B38: 5568F87E  srwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EA7B3C: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EA7B40: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7B44: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EA7B48: 4099000C  ble cr6, 0x82ea7b54
	if !ctx.cr[6].gt {
		sub_82EA7B54(ctx, base);
		return;
	}
	// 82EA7B4C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EA7B50: 48000008  b 0x82ea7b58
	sub_82EA7B54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7B54 size=48
    let mut pc: u32 = 0x82EA7B54;
    'dispatch: loop {
        match pc {
            0x82EA7B54 => {
    //   block [0x82EA7B54..0x82EA7B84)
	// 82EA7B54: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82EA7B58: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EA7B5C: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7B60: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA7B64: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7B68: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EA7B6C: 40990008  ble cr6, 0x82ea7b74
	if !ctx.cr[6].gt {
	pc = 0x82EA7B74; continue 'dispatch;
	}
	// 82EA7B70: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EA7B74: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82EA7B78: 7D6A1670  srawi r10, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EA7B7C: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EA7B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA7B88 size=128
    let mut pc: u32 = 0x82EA7B88;
    'dispatch: loop {
        match pc {
            0x82EA7B88 => {
    //   block [0x82EA7B88..0x82EA7C08)
	// 82EA7B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA7B8C: 483005DD  bl 0x831a8168
	ctx.lr = 0x82EA7B90;
	sub_831A8130(ctx, base);
	// 82EA7B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA7B94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA7B98: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA7B9C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EA7BA0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7BA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA7BA8: 40990034  ble cr6, 0x82ea7bdc
	if !ctx.cr[6].gt {
	pc = 0x82EA7BDC; continue 'dispatch;
	}
	// 82EA7BAC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EA7BB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7BB4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA7BB8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EA7BBC: 48002835  bl 0x82eaa3f0
	ctx.lr = 0x82EA7BC0;
	sub_82EAA3F0(ctx, base);
	// 82EA7BC0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA7BC4: 419A0024  beq cr6, 0x82ea7be8
	if ctx.cr[6].eq {
	pc = 0x82EA7BE8; continue 'dispatch;
	}
	// 82EA7BC8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7BCC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EA7BD0: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82EA7BD4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EA7BD8: 4198FFD8  blt cr6, 0x82ea7bb0
	if ctx.cr[6].lt {
	pc = 0x82EA7BB0; continue 'dispatch;
	}
	// 82EA7BDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA7BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA7BE4: 483005D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA7BE8: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7BEC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7BF0: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82EA7BF4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7BF8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA7BFC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82EA7C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA7C04: 483005B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C08 size=8
    let mut pc: u32 = 0x82EA7C08;
    'dispatch: loop {
        match pc {
            0x82EA7C08 => {
    //   block [0x82EA7C08..0x82EA7C10)
	// 82EA7C08: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C10 size=8
    let mut pc: u32 = 0x82EA7C10;
    'dispatch: loop {
        match pc {
            0x82EA7C10 => {
    //   block [0x82EA7C10..0x82EA7C18)
	// 82EA7C10: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C18 size=8
    let mut pc: u32 = 0x82EA7C18;
    'dispatch: loop {
        match pc {
            0x82EA7C18 => {
    //   block [0x82EA7C18..0x82EA7C20)
	// 82EA7C18: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C20 size=16
    let mut pc: u32 = 0x82EA7C20;
    'dispatch: loop {
        match pc {
            0x82EA7C20 => {
    //   block [0x82EA7C20..0x82EA7C30)
	// 82EA7C20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EA7C24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA7C28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7C2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C30 size=20
    let mut pc: u32 = 0x82EA7C30;
    'dispatch: loop {
        match pc {
            0x82EA7C30 => {
    //   block [0x82EA7C30..0x82EA7C44)
	// 82EA7C30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7C34: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82EA7C38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7C3C: 409AFFF4  bne cr6, 0x82ea7c30
	if !ctx.cr[6].eq {
	pc = 0x82EA7C30; continue 'dispatch;
	}
	// 82EA7C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C48 size=40
    let mut pc: u32 = 0x82EA7C48;
    'dispatch: loop {
        match pc {
            0x82EA7C48 => {
    //   block [0x82EA7C48..0x82EA7C70)
	// 82EA7C48: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EA7C4C: 419A0018  beq cr6, 0x82ea7c64
	if ctx.cr[6].eq {
	pc = 0x82EA7C64; continue 'dispatch;
	}
	// 82EA7C50: 7F052040  cmplw cr6, r5, r4
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EA7C54: 419A001C  beq cr6, 0x82ea7c70
	if ctx.cr[6].eq {
		sub_82EA7C70(ctx, base);
		return;
	}
	// 82EA7C58: 80A50004  lwz r5, 4(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7C5C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EA7C60: 409AFFF0  bne cr6, 0x82ea7c50
	if !ctx.cr[6].eq {
	pc = 0x82EA7C50; continue 'dispatch;
	}
	// 82EA7C64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EA7C68: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA7C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C70 size=12
    let mut pc: u32 = 0x82EA7C70;
    'dispatch: loop {
        match pc {
            0x82EA7C70 => {
    //   block [0x82EA7C70..0x82EA7C7C)
	// 82EA7C70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EA7C74: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EA7C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C80 size=16
    let mut pc: u32 = 0x82EA7C80;
    'dispatch: loop {
        match pc {
            0x82EA7C80 => {
    //   block [0x82EA7C80..0x82EA7C90)
	// 82EA7C80: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7C84: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA7C88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7C8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7C90 size=24
    let mut pc: u32 = 0x82EA7C90;
    'dispatch: loop {
        match pc {
            0x82EA7C90 => {
    //   block [0x82EA7C90..0x82EA7CA8)
	// 82EA7C90: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA7C94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7C98: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82EA7C9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7CA0: 409AFFF0  bne cr6, 0x82ea7c90
	if !ctx.cr[6].eq {
	pc = 0x82EA7C90; continue 'dispatch;
	}
	// 82EA7CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7CA8 size=8
    let mut pc: u32 = 0x82EA7CA8;
    'dispatch: loop {
        match pc {
            0x82EA7CA8 => {
    //   block [0x82EA7CA8..0x82EA7CB0)
	// 82EA7CA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA7CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7CB0 size=8
    let mut pc: u32 = 0x82EA7CB0;
    'dispatch: loop {
        match pc {
            0x82EA7CB0 => {
    //   block [0x82EA7CB0..0x82EA7CB8)
	// 82EA7CB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA7CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7CB8 size=8
    let mut pc: u32 = 0x82EA7CB8;
    'dispatch: loop {
        match pc {
            0x82EA7CB8 => {
    //   block [0x82EA7CB8..0x82EA7CC0)
	// 82EA7CB8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA7CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7CC0 size=16
    let mut pc: u32 = 0x82EA7CC0;
    'dispatch: loop {
        match pc {
            0x82EA7CC0 => {
    //   block [0x82EA7CC0..0x82EA7CD0)
	// 82EA7CC0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7CC4: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7CCC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7CD0 size=24
    let mut pc: u32 = 0x82EA7CD0;
    'dispatch: loop {
        match pc {
            0x82EA7CD0 => {
    //   block [0x82EA7CD0..0x82EA7CE8)
	// 82EA7CD0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7CD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7CD8: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82EA7CDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7CE0: 409AFFF0  bne cr6, 0x82ea7cd0
	if !ctx.cr[6].eq {
	pc = 0x82EA7CD0; continue 'dispatch;
	}
	// 82EA7CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7CE8 size=76
    let mut pc: u32 = 0x82EA7CE8;
    'dispatch: loop {
        match pc {
            0x82EA7CE8 => {
    //   block [0x82EA7CE8..0x82EA7D34)
	// 82EA7CE8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7CEC: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7CF4: 419A0018  beq cr6, 0x82ea7d0c
	if ctx.cr[6].eq {
	pc = 0x82EA7D0C; continue 'dispatch;
	}
	// 82EA7CF8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7CFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7D00: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA7D04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7D08: 409AFFF0  bne cr6, 0x82ea7cf8
	if !ctx.cr[6].eq {
	pc = 0x82EA7CF8; continue 'dispatch;
	}
	// 82EA7D0C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EA7D10: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82EA7D14: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7D18: 7D695A15  add. r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA7D1C: 40800018  bge 0x82ea7d34
	if !ctx.cr[0].lt {
		sub_82EA7D34(ctx, base);
		return;
	}
	// 82EA7D20: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7D24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA7D28: 409AFFEC  bne cr6, 0x82ea7d14
	if !ctx.cr[6].eq {
	pc = 0x82EA7D14; continue 'dispatch;
	}
	// 82EA7D2C: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA7D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7D34 size=24
    let mut pc: u32 = 0x82EA7D34;
    'dispatch: loop {
        match pc {
            0x82EA7D34 => {
    //   block [0x82EA7D34..0x82EA7D4C)
	// 82EA7D34: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA7D38: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA7D3C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EA7D40: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7D44: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA7D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA7D50 size=196
    let mut pc: u32 = 0x82EA7D50;
    'dispatch: loop {
        match pc {
            0x82EA7D50 => {
    //   block [0x82EA7D50..0x82EA7E14)
	// 82EA7D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA7D54: 48300415  bl 0x831a8168
	ctx.lr = 0x82EA7D58;
	sub_831A8130(ctx, base);
	// 82EA7D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA7D5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA7D60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA7D64: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EA7D68: 4BFFFF59  bl 0x82ea7cc0
	ctx.lr = 0x82EA7D6C;
	sub_82EA7CC0(ctx, base);
	// 82EA7D6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA7D70: 40990074  ble cr6, 0x82ea7de4
	if !ctx.cr[6].gt {
	pc = 0x82EA7DE4; continue 'dispatch;
	}
	// 82EA7D74: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7D78: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7D80: 419A0018  beq cr6, 0x82ea7d98
	if ctx.cr[6].eq {
	pc = 0x82EA7D98; continue 'dispatch;
	}
	// 82EA7D84: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7D88: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7D8C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA7D90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7D94: 409AFFF0  bne cr6, 0x82ea7d84
	if !ctx.cr[6].eq {
	pc = 0x82EA7D84; continue 'dispatch;
	}
	// 82EA7D98: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82EA7D9C: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82EA7DA0: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7DA4: 7D695A15  add. r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA7DA8: 40800048  bge 0x82ea7df0
	if !ctx.cr[0].lt {
	pc = 0x82EA7DF0; continue 'dispatch;
	}
	// 82EA7DAC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7DB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA7DB4: 409AFFEC  bne cr6, 0x82ea7da0
	if !ctx.cr[6].eq {
	pc = 0x82EA7DA0; continue 'dispatch;
	}
	// 82EA7DB8: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA7DBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA7DC0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7DC4: 4800262D  bl 0x82eaa3f0
	ctx.lr = 0x82EA7DC8;
	sub_82EAA3F0(ctx, base);
	// 82EA7DC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA7DCC: 419A003C  beq cr6, 0x82ea7e08
	if ctx.cr[6].eq {
	pc = 0x82EA7E08; continue 'dispatch;
	}
	// 82EA7DD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA7DD4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EA7DD8: 4BFFFEE9  bl 0x82ea7cc0
	ctx.lr = 0x82EA7DDC;
	sub_82EA7CC0(ctx, base);
	// 82EA7DDC: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82EA7DE0: 4198FF94  blt cr6, 0x82ea7d74
	if ctx.cr[6].lt {
	pc = 0x82EA7D74; continue 'dispatch;
	}
	// 82EA7DE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA7DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA7DEC: 483003CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA7DF0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA7DF4: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA7DF8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EA7DFC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7E00: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA7E04: 4BFFFFB8  b 0x82ea7dbc
	pc = 0x82EA7DBC; continue 'dispatch;
	// 82EA7E08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA7E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA7E10: 483003A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7E18 size=8
    let mut pc: u32 = 0x82EA7E18;
    'dispatch: loop {
        match pc {
            0x82EA7E18 => {
    //   block [0x82EA7E18..0x82EA7E20)
	// 82EA7E18: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EA7E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7E20 size=16
    let mut pc: u32 = 0x82EA7E20;
    'dispatch: loop {
        match pc {
            0x82EA7E20 => {
    //   block [0x82EA7E20..0x82EA7E30)
	// 82EA7E20: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7E24: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7E28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7E2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7E30 size=24
    let mut pc: u32 = 0x82EA7E30;
    'dispatch: loop {
        match pc {
            0x82EA7E30 => {
    //   block [0x82EA7E30..0x82EA7E48)
	// 82EA7E30: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7E34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7E38: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82EA7E3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7E40: 409AFFF0  bne cr6, 0x82ea7e30
	if !ctx.cr[6].eq {
	pc = 0x82EA7E30; continue 'dispatch;
	}
	// 82EA7E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7E48 size=76
    let mut pc: u32 = 0x82EA7E48;
    'dispatch: loop {
        match pc {
            0x82EA7E48 => {
    //   block [0x82EA7E48..0x82EA7E94)
	// 82EA7E48: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7E4C: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7E50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7E54: 419A0018  beq cr6, 0x82ea7e6c
	if ctx.cr[6].eq {
	pc = 0x82EA7E6C; continue 'dispatch;
	}
	// 82EA7E58: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7E5C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7E60: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA7E64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7E68: 409AFFF0  bne cr6, 0x82ea7e58
	if !ctx.cr[6].eq {
	pc = 0x82EA7E58; continue 'dispatch;
	}
	// 82EA7E6C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EA7E70: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82EA7E74: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7E78: 7D695A15  add. r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA7E7C: 40800018  bge 0x82ea7e94
	if !ctx.cr[0].lt {
		sub_82EA7E94(ctx, base);
		return;
	}
	// 82EA7E80: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7E84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA7E88: 409AFFEC  bne cr6, 0x82ea7e74
	if !ctx.cr[6].eq {
	pc = 0x82EA7E74; continue 'dispatch;
	}
	// 82EA7E8C: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA7E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7E94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7E94 size=24
    let mut pc: u32 = 0x82EA7E94;
    'dispatch: loop {
        match pc {
            0x82EA7E94 => {
    //   block [0x82EA7E94..0x82EA7EAC)
	// 82EA7E94: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA7E98: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA7E9C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EA7EA0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7EA4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA7EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7EB0 size=4
    let mut pc: u32 = 0x82EA7EB0;
    'dispatch: loop {
        match pc {
            0x82EA7EB0 => {
    //   block [0x82EA7EB0..0x82EA7EB4)
	// 82EA7EB0: 4BFFFF98  b 0x82ea7e48
	sub_82EA7E48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7EB8 size=8
    let mut pc: u32 = 0x82EA7EB8;
    'dispatch: loop {
        match pc {
            0x82EA7EB8 => {
    //   block [0x82EA7EB8..0x82EA7EC0)
	// 82EA7EB8: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA7EC0 size=24
    let mut pc: u32 = 0x82EA7EC0;
    'dispatch: loop {
        match pc {
            0x82EA7EC0 => {
    //   block [0x82EA7EC0..0x82EA7ED8)
	// 82EA7EC0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7EC4: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA7EC8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82EA7ECC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7ED0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA7ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA7ED8 size=196
    let mut pc: u32 = 0x82EA7ED8;
    'dispatch: loop {
        match pc {
            0x82EA7ED8 => {
    //   block [0x82EA7ED8..0x82EA7F9C)
	// 82EA7ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA7EDC: 4830028D  bl 0x831a8168
	ctx.lr = 0x82EA7EE0;
	sub_831A8130(ctx, base);
	// 82EA7EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA7EE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA7EE8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EA7EEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EA7EF0: 4BFFFF31  bl 0x82ea7e20
	ctx.lr = 0x82EA7EF4;
	sub_82EA7E20(ctx, base);
	// 82EA7EF4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA7EF8: 40990074  ble cr6, 0x82ea7f6c
	if !ctx.cr[6].gt {
	pc = 0x82EA7F6C; continue 'dispatch;
	}
	// 82EA7EFC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7F00: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7F04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7F08: 419A0018  beq cr6, 0x82ea7f20
	if ctx.cr[6].eq {
	pc = 0x82EA7F20; continue 'dispatch;
	}
	// 82EA7F0C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7F10: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7F14: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA7F18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7F1C: 409AFFF0  bne cr6, 0x82ea7f0c
	if !ctx.cr[6].eq {
	pc = 0x82EA7F0C; continue 'dispatch;
	}
	// 82EA7F20: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82EA7F24: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82EA7F28: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7F2C: 7D695A15  add. r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA7F30: 40800048  bge 0x82ea7f78
	if !ctx.cr[0].lt {
	pc = 0x82EA7F78; continue 'dispatch;
	}
	// 82EA7F34: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7F38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA7F3C: 409AFFEC  bne cr6, 0x82ea7f28
	if !ctx.cr[6].eq {
	pc = 0x82EA7F28; continue 'dispatch;
	}
	// 82EA7F40: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA7F44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EA7F48: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA7F4C: 480024A5  bl 0x82eaa3f0
	ctx.lr = 0x82EA7F50;
	sub_82EAA3F0(ctx, base);
	// 82EA7F50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA7F54: 419A003C  beq cr6, 0x82ea7f90
	if ctx.cr[6].eq {
	pc = 0x82EA7F90; continue 'dispatch;
	}
	// 82EA7F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA7F5C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EA7F60: 4BFFFEC1  bl 0x82ea7e20
	ctx.lr = 0x82EA7F64;
	sub_82EA7E20(ctx, base);
	// 82EA7F64: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82EA7F68: 4198FF94  blt cr6, 0x82ea7efc
	if ctx.cr[6].lt {
	pc = 0x82EA7EFC; continue 'dispatch;
	}
	// 82EA7F6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA7F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA7F74: 48300244  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA7F78: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA7F7C: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA7F80: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EA7F84: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA7F88: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA7F8C: 4BFFFFB8  b 0x82ea7f44
	pc = 0x82EA7F44; continue 'dispatch;
	// 82EA7F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA7F94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA7F98: 48300220  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA7FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA7FA0 size=196
    let mut pc: u32 = 0x82EA7FA0;
    'dispatch: loop {
        match pc {
            0x82EA7FA0 => {
    //   block [0x82EA7FA0..0x82EA8064)
	// 82EA7FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA7FA4: 483001C9  bl 0x831a816c
	ctx.lr = 0x82EA7FA8;
	sub_831A8130(ctx, base);
	// 82EA7FA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA7FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EA7FB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EA7FB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EA7FB8: 4BFFFE69  bl 0x82ea7e20
	ctx.lr = 0x82EA7FBC;
	sub_82EA7E20(ctx, base);
	// 82EA7FBC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA7FC0: 40990074  ble cr6, 0x82ea8034
	if !ctx.cr[6].gt {
	pc = 0x82EA8034; continue 'dispatch;
	}
	// 82EA7FC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7FC8: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7FCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7FD0: 419A0018  beq cr6, 0x82ea7fe8
	if ctx.cr[6].eq {
	pc = 0x82EA7FE8; continue 'dispatch;
	}
	// 82EA7FD4: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7FD8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA7FDC: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EA7FE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA7FE4: 409AFFF0  bne cr6, 0x82ea7fd4
	if !ctx.cr[6].eq {
	pc = 0x82EA7FD4; continue 'dispatch;
	}
	// 82EA7FE8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82EA7FEC: 7D69F050  subf r11, r9, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	// 82EA7FF0: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA7FF4: 7D695A15  add. r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA7FF8: 40800048  bge 0x82ea8040
	if !ctx.cr[0].lt {
	pc = 0x82EA8040; continue 'dispatch;
	}
	// 82EA7FFC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA8000: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA8004: 409AFFEC  bne cr6, 0x82ea7ff0
	if !ctx.cr[6].eq {
	pc = 0x82EA7FF0; continue 'dispatch;
	}
	// 82EA8008: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA800C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EA8010: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8014: 480023DD  bl 0x82eaa3f0
	ctx.lr = 0x82EA8018;
	sub_82EAA3F0(ctx, base);
	// 82EA8018: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EA801C: 419A003C  beq cr6, 0x82ea8058
	if ctx.cr[6].eq {
	pc = 0x82EA8058; continue 'dispatch;
	}
	// 82EA8020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA8024: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EA8028: 4BFFFDF9  bl 0x82ea7e20
	ctx.lr = 0x82EA802C;
	sub_82EA7E20(ctx, base);
	// 82EA802C: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82EA8030: 4198FF94  blt cr6, 0x82ea7fc4
	if ctx.cr[6].lt {
	pc = 0x82EA7FC4; continue 'dispatch;
	}
	// 82EA8034: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EA8038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA803C: 48300180  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EA8040: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA8044: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA8048: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EA804C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA8050: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA8054: 4BFFFFB8  b 0x82ea800c
	pc = 0x82EA800C; continue 'dispatch;
	// 82EA8058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA805C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EA8060: 4830015C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8068 size=8
    let mut pc: u32 = 0x82EA8068;
    'dispatch: loop {
        match pc {
            0x82EA8068 => {
    //   block [0x82EA8068..0x82EA8070)
	// 82EA8068: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EA806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8070 size=8
    let mut pc: u32 = 0x82EA8070;
    'dispatch: loop {
        match pc {
            0x82EA8070 => {
    //   block [0x82EA8070..0x82EA8078)
	// 82EA8070: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82EA8074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8078 size=52
    let mut pc: u32 = 0x82EA8078;
    'dispatch: loop {
        match pc {
            0x82EA8078 => {
    //   block [0x82EA8078..0x82EA80AC)
	// 82EA8078: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA807C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA8080: 419A0014  beq cr6, 0x82ea8094
	if ctx.cr[6].eq {
	pc = 0x82EA8094; continue 'dispatch;
	}
	// 82EA8084: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EA8088: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA808C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA8090: 409AFFF4  bne cr6, 0x82ea8084
	if !ctx.cr[6].eq {
	pc = 0x82EA8084; continue 'dispatch;
	}
	// 82EA8094: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA8098: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EA809C: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EA80A0: 69280001  xori r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 ^ 1;
	// 82EA80A4: 99030000  stb r8, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82EA80A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA80B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA80B0 size=8
    let mut pc: u32 = 0x82EA80B0;
    'dispatch: loop {
        match pc {
            0x82EA80B0 => {
    //   block [0x82EA80B0..0x82EA80B8)
	// 82EA80B0: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EA80B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA80B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA80B8 size=76
    let mut pc: u32 = 0x82EA80B8;
    'dispatch: loop {
        match pc {
            0x82EA80B8 => {
    //   block [0x82EA80B8..0x82EA8104)
	// 82EA80B8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EA80BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA80C0: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA80C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA80C8: 419A0018  beq cr6, 0x82ea80e0
	if ctx.cr[6].eq {
	pc = 0x82EA80E0; continue 'dispatch;
	}
	// 82EA80CC: 810B001C  lwz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA80D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA80D4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82EA80D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EA80DC: 409AFFF0  bne cr6, 0x82ea80cc
	if !ctx.cr[6].eq {
	pc = 0x82EA80CC; continue 'dispatch;
	}
	// 82EA80E0: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82EA80E4: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EA80E8: 7D695A15  add. r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EA80EC: 40800018  bge 0x82ea8104
	if !ctx.cr[0].lt {
		sub_82EA8104(ctx, base);
		return;
	}
	// 82EA80F0: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EA80F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EA80F8: 409AFFEC  bne cr6, 0x82ea80e4
	if !ctx.cr[6].eq {
	pc = 0x82EA80E4; continue 'dispatch;
	}
	// 82EA80FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EA8100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8104 size=68
    let mut pc: u32 = 0x82EA8104;
    'dispatch: loop {
        match pc {
            0x82EA8104 => {
    //   block [0x82EA8104..0x82EA8148)
	// 82EA8104: 810A0020  lwz r8, 0x20(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EA8108: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EA810C: 419AFFF0  beq cr6, 0x82ea80fc
	if ctx.cr[6].eq {
		sub_82EA80B8(ctx, base);
		return;
	}
	// 82EA8110: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA8114: 7CE9402E  lwzx r7, r9, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EA8118: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EA811C: 4198FFE0  blt cr6, 0x82ea80fc
	if ctx.cr[6].lt {
		sub_82EA80B8(ctx, base);
		return;
	}
	// 82EA8120: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EA8124: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EA8128: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EA812C: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EA8130: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA8134: 54EB1838  slwi r11, r7, 3
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA8138: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EA813C: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA8140: 90A60000  stw r5, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EA8144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8148 size=88
    let mut pc: u32 = 0x82EA8148;
    'dispatch: loop {
        match pc {
            0x82EA8148 => {
    //   block [0x82EA8148..0x82EA81A0)
	// 82EA8148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA814C: 48300021  bl 0x831a816c
	ctx.lr = 0x82EA8150;
	sub_831A8130(ctx, base);
	// 82EA8150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8154: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EA8158: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EA815C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82EA8160: 4BFFFF59  bl 0x82ea80b8
	ctx.lr = 0x82EA8164;
	sub_82EA80B8(ctx, base);
	// 82EA8164: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EA8168: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EA816C: 409A0028  bne cr6, 0x82ea8194
	if !ctx.cr[6].eq {
	pc = 0x82EA8194; continue 'dispatch;
	}
	// 82EA8170: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EA8174: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA8178: 48005D99  bl 0x82eadf10
	ctx.lr = 0x82EA817C;
	sub_82EADF10(ctx, base);
	// 82EA817C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EA8180: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA8184: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA8188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA818C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EA8190: 4E800421  bctrl
	ctx.lr = 0x82EA8194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EA8194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA8198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA819C: 48300020  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA81A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA81A0 size=16
    let mut pc: u32 = 0x82EA81A0;
    'dispatch: loop {
        match pc {
            0x82EA81A0 => {
    //   block [0x82EA81A0..0x82EA81B0)
	// 82EA81A0: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EA81A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EA81A8: 419A0008  beq cr6, 0x82ea81b0
	if ctx.cr[6].eq {
		sub_82EA81B0(ctx, base);
		return;
	}
	// 82EA81AC: 4BFFF9DC  b 0x82ea7b88
	sub_82EA7B88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA81B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA81B0 size=8
    let mut pc: u32 = 0x82EA81B0;
    'dispatch: loop {
        match pc {
            0x82EA81B0 => {
    //   block [0x82EA81B0..0x82EA81B8)
	// 82EA81B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EA81B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA81B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA81B8 size=8
    let mut pc: u32 = 0x82EA81B8;
    'dispatch: loop {
        match pc {
            0x82EA81B8 => {
    //   block [0x82EA81B8..0x82EA81C0)
	// 82EA81B8: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82EA81BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA81C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA81C0 size=8
    let mut pc: u32 = 0x82EA81C0;
    'dispatch: loop {
        match pc {
            0x82EA81C0 => {
    //   block [0x82EA81C0..0x82EA81C8)
	// 82EA81C0: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82EA81C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA81C8 size=84
    let mut pc: u32 = 0x82EA81C8;
    'dispatch: loop {
        match pc {
            0x82EA81C8 => {
    //   block [0x82EA81C8..0x82EA821C)
	// 82EA81C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA81CC: 482FFF9D  bl 0x831a8168
	ctx.lr = 0x82EA81D0;
	sub_831A8130(ctx, base);
	// 82EA81D0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA81D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EA81D8: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EA81DC: 83C10064  lwz r30, 0x64(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EA81E0: 83A1006C  lwz r29, 0x6c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EA81E4: 83810074  lwz r28, 0x74(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EA81E8: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EA81EC: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EA81F0: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82EA81F4: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82EA81F8: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82EA81FC: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EA8200: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EA8204: 93E3001C  stw r31, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82EA8208: 93C30020  stw r30, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EA820C: 93A30024  stw r29, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82EA8210: 93830028  stw r28, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82EA8214: 90E3002C  stw r7, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82EA8218: 482FFFA0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EA8220 size=24
    let mut pc: u32 = 0x82EA8220;
    'dispatch: loop {
        match pc {
            0x82EA8220 => {
    //   block [0x82EA8220..0x82EA8238)
	// 82EA8220: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA8224: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EA8228: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82EA822C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EA8230: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EA8234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EA8238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EA8238 size=176
    let mut pc: u32 = 0x82EA8238;
    'dispatch: loop {
        match pc {
            0x82EA8238 => {
    //   block [0x82EA8238..0x82EA82E8)
	// 82EA8238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EA823C: 482FFF2D  bl 0x831a8168
	ctx.lr = 0x82EA8240;
	sub_831A8130(ctx, base);
	// 82EA8240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EA8244: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EA8248: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82EA824C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EA8250: 4BFFFE69  bl 0x82ea80b8
	ctx.lr = 0x82EA8254;
	sub_82EA80B8(ctx, base);
	// 82EA8254: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EA8258: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EA825C: 409A0030  bne cr6, 0x82ea828c
	if !ctx.cr[6].eq {
	pc = 0x82EA828C; continue 'dispatch;
	}
	// 82EA8260: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EA8264: 88BE000C  lbz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EA8268: 2B050014  cmplwi cr6, r5, 0x14
	ctx.cr[6].compare_u32(ctx.r[5].u32, 20 as u32, &mut ctx.xer);
	// 82EA826C: 409A002C  bne cr6, 0x82ea8298
	if !ctx.cr[6].eq {
	pc = 0x82EA8298; continue 'dispatch;
	}
	// 82EA8270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA8274: 48005C5D  bl 0x82eaded0
	ctx.lr = 0x82EA8278;
	sub_82EADED0(ctx, base);
	// 82EA8278: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA827C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EA8280: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EA8284: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EA8288: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EA828C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA8290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA8294: 482FFF24  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA8298: 2B050018  cmplwi cr6, r5, 0x18
	ctx.cr[6].compare_u32(ctx.r[5].u32, 24 as u32, &mut ctx.xer);
	// 82EA829C: 409A0038  bne cr6, 0x82ea82d4
	if !ctx.cr[6].eq {
	pc = 0x82EA82D4; continue 'dispatch;
	}
	// 82EA82A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA82A4: 48005C35  bl 0x82eaded8
	ctx.lr = 0x82EA82A8;
	sub_82EADED8(ctx, base);
	// 82EA82A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EA82AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EA82B0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA82B4: 48005F25  bl 0x82eae1d8
	ctx.lr = 0x82EA82B8;
	sub_82EAE1D8(ctx, base);
	// 82EA82B8: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82EA82BC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82EA82C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EA82C4: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82EA82C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EA82CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA82D0: 482FFEE8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EA82D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EA82D8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EA82DC: 480002E5  bl 0x82ea85c0
	ctx.lr = 0x82EA82E0;
	sub_82EA85C0(ctx, base);
	// 82EA82E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EA82E4: 482FFED4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


