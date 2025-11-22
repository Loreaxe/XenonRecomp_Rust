pub fn sub_82506BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82506BD8 size=1372
    let mut pc: u32 = 0x82506BD8;
    'dispatch: loop {
        match pc {
            0x82506BD8 => {
    //   block [0x82506BD8..0x82507134)
	// 82506BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82506BDC: 48CA155D  bl 0x831a8138
	ctx.lr = 0x82506BE0;
	sub_831A8130(ctx, base);
	// 82506BE0: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82506BE4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82506BE8: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82506BEC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82506BF0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82506BF4: 7E90A378  mr r16, r20
	ctx.r[16].u64 = ctx.r[20].u64;
	// 82506BF8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506BFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506C00: 419A050C  beq cr6, 0x8250710c
	if ctx.cr[6].eq {
	pc = 0x8250710C; continue 'dispatch;
	}
	// 82506C04: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82506C08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82506C0C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82506C10: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82506C14: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82506C18: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82506C1C: 3A3B0004  addi r17, r27, 4
	ctx.r[17].s64 = ctx.r[27].s64 + 4;
	// 82506C20: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 82506C24: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82506C28: 3A6B1630  addi r19, r11, 0x1630
	ctx.r[19].s64 = ctx.r[11].s64 + 5680;
	// 82506C2C: 3A4A162C  addi r18, r10, 0x162c
	ctx.r[18].s64 = ctx.r[10].s64 + 5676;
	// 82506C30: 3BA91554  addi r29, r9, 0x1554
	ctx.r[29].s64 = ctx.r[9].s64 + 5460;
	// 82506C34: 3B081624  addi r24, r8, 0x1624
	ctx.r[24].s64 = ctx.r[8].s64 + 5668;
	// 82506C38: 3AE7A078  addi r23, r7, -0x5f88
	ctx.r[23].s64 = ctx.r[7].s64 + -24456;
	// 82506C3C: 3AA6E984  addi r21, r6, -0x167c
	ctx.r[21].s64 = ctx.r[6].s64 + -5756;
	// 82506C40: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82506C44: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506C48: 480D2921  bl 0x825d9568
	ctx.lr = 0x82506C4C;
	sub_825D9568(ctx, base);
	// 82506C4C: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82506C50: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82506C54: 488ECDB5  bl 0x82df3a08
	ctx.lr = 0x82506C58;
	sub_82DF3A08(ctx, base);
	// 82506C58: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82506C5C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82506C60: 488EC6A9  bl 0x82df3308
	ctx.lr = 0x82506C64;
	sub_82DF3308(ctx, base);
	// 82506C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506C68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82506C6C: 488EC7BD  bl 0x82df3428
	ctx.lr = 0x82506C70;
	sub_82DF3428(ctx, base);
	// 82506C70: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506C74: 41820328  beq 0x82506f9c
	if ctx.cr[0].eq {
	pc = 0x82506F9C; continue 'dispatch;
	}
	// 82506C78: 560B063F  clrlwi. r11, r16, 0x18
	ctx.r[11].u64 = ctx.r[16].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506C7C: 40820010  bne 0x82506c8c
	if !ctx.cr[0].eq {
	pc = 0x82506C8C; continue 'dispatch;
	}
	// 82506C80: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506C84: 7ED0B378  mr r16, r22
	ctx.r[16].u64 = ctx.r[22].u64;
	// 82506C88: 9ACB0004  stb r22, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[22].u8 ) };
	// 82506C8C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82506C90: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506C94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82506C98: 480D27B9  bl 0x825d9450
	ctx.lr = 0x82506C9C;
	sub_825D9450(ctx, base);
	// 82506C9C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82506CA0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82506CA4: 488ECD65  bl 0x82df3a08
	ctx.lr = 0x82506CA8;
	sub_82DF3A08(ctx, base);
	// 82506CA8: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82506CAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82506CB0: 488EC659  bl 0x82df3308
	ctx.lr = 0x82506CB4;
	sub_82DF3308(ctx, base);
	// 82506CB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506CB8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82506CBC: 488EC76D  bl 0x82df3428
	ctx.lr = 0x82506CC0;
	sub_82DF3428(ctx, base);
	// 82506CC0: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506CC4: 418200D4  beq 0x82506d98
	if ctx.cr[0].eq {
	pc = 0x82506D98; continue 'dispatch;
	}
	// 82506CC8: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506CCC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506CD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82506CD4: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82506CD8: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 82506CDC: 419A0024  beq cr6, 0x82506d00
	if ctx.cr[6].eq {
	pc = 0x82506D00; continue 'dispatch;
	}
	// 82506CE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82506CE4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82506CE8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82506CEC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82506CF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82506CF4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82506CF8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82506CFC: 4082FFE8  bne 0x82506ce4
	if !ctx.cr[0].eq {
	pc = 0x82506CE4; continue 'dispatch;
	}
	// 82506D00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82506D04: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82506D08: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82506D0C: 4BFFEAFD  bl 0x82505808
	ctx.lr = 0x82506D10;
	sub_82505808(ctx, base);
	// 82506D10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82506D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82506D18: 38A0007E  li r5, 0x7e
	ctx.r[5].s64 = 126;
	// 82506D1C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82506D20: 4BDB96B9  bl 0x822c03d8
	ctx.lr = 0x82506D24;
	sub_822C03D8(ctx, base);
	// 82506D24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506D28: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82506D2C: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82506D30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506D34: 4BFF9A3D  bl 0x82500770
	ctx.lr = 0x82506D38;
	sub_82500770(ctx, base);
	// 82506D38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82506D3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506D40: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82506D44: 4BDB92BD  bl 0x822c0000
	ctx.lr = 0x82506D48;
	sub_822C0000(ctx, base);
	// 82506D48: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82506D4C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82506D50: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506D54: 817C7008  lwz r11, 0x7008(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28680 as u32) ) } as u64;
	// 82506D58: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82506D5C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82506D60: 409A000C  bne cr6, 0x82506d6c
	if !ctx.cr[6].eq {
	pc = 0x82506D6C; continue 'dispatch;
	}
	// 82506D64: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 82506D68: 48000010  b 0x82506d78
	pc = 0x82506D78; continue 'dispatch;
	// 82506D6C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82506D70: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82506D74: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82506D78: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82506D7C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82506D80: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82506D84: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506D88: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82506D8C: 486AC37D  bl 0x82bb3108
	ctx.lr = 0x82506D90;
	sub_82BB3108(ctx, base);
	// 82506D90: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82506D94: 480001F0  b 0x82506f84
	pc = 0x82506F84; continue 'dispatch;
	// 82506D98: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82506D9C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82506DA0: 488ECC69  bl 0x82df3a08
	ctx.lr = 0x82506DA4;
	sub_82DF3A08(ctx, base);
	// 82506DA4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82506DA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82506DAC: 488EC55D  bl 0x82df3308
	ctx.lr = 0x82506DB0;
	sub_82DF3308(ctx, base);
	// 82506DB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506DB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82506DB8: 488EC671  bl 0x82df3428
	ctx.lr = 0x82506DBC;
	sub_82DF3428(ctx, base);
	// 82506DBC: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506DC0: 418200D0  beq 0x82506e90
	if ctx.cr[0].eq {
	pc = 0x82506E90; continue 'dispatch;
	}
	// 82506DC4: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506DC8: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506DCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82506DD0: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82506DD4: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82506DD8: 419A0024  beq cr6, 0x82506dfc
	if ctx.cr[6].eq {
	pc = 0x82506DFC; continue 'dispatch;
	}
	// 82506DDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82506DE0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82506DE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82506DE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82506DEC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82506DF0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82506DF4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82506DF8: 4082FFE8  bne 0x82506de0
	if !ctx.cr[0].eq {
	pc = 0x82506DE0; continue 'dispatch;
	}
	// 82506DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82506E00: 38810098  addi r4, r1, 0x98
	ctx.r[4].s64 = ctx.r[1].s64 + 152;
	// 82506E04: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82506E08: 4BFFEB11  bl 0x82505918
	ctx.lr = 0x82506E0C;
	sub_82505918(ctx, base);
	// 82506E0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82506E10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82506E14: 38A00085  li r5, 0x85
	ctx.r[5].s64 = 133;
	// 82506E18: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82506E1C: 4BDB95BD  bl 0x822c03d8
	ctx.lr = 0x82506E20;
	sub_822C03D8(ctx, base);
	// 82506E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506E24: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82506E28: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82506E2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506E30: 4BFF9941  bl 0x82500770
	ctx.lr = 0x82506E34;
	sub_82500770(ctx, base);
	// 82506E34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82506E38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506E3C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82506E40: 4BDB91C1  bl 0x822c0000
	ctx.lr = 0x82506E44;
	sub_822C0000(ctx, base);
	// 82506E44: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82506E48: 92C90000  stw r22, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82506E4C: 817C7008  lwz r11, 0x7008(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28680 as u32) ) } as u64;
	// 82506E50: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82506E54: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82506E58: 409A000C  bne cr6, 0x82506e64
	if !ctx.cr[6].eq {
	pc = 0x82506E64; continue 'dispatch;
	}
	// 82506E5C: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 82506E60: 48000010  b 0x82506e70
	pc = 0x82506E70; continue 'dispatch;
	// 82506E64: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506E68: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82506E6C: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82506E70: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82506E74: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82506E78: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82506E7C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506E80: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82506E84: 486AC285  bl 0x82bb3108
	ctx.lr = 0x82506E88;
	sub_82BB3108(ctx, base);
	// 82506E88: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82506E8C: 480000F8  b 0x82506f84
	pc = 0x82506F84; continue 'dispatch;
	// 82506E90: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82506E94: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82506E98: 488ECB71  bl 0x82df3a08
	ctx.lr = 0x82506E9C;
	sub_82DF3A08(ctx, base);
	// 82506E9C: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82506EA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82506EA4: 488EC465  bl 0x82df3308
	ctx.lr = 0x82506EA8;
	sub_82DF3308(ctx, base);
	// 82506EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506EAC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82506EB0: 488EC579  bl 0x82df3428
	ctx.lr = 0x82506EB4;
	sub_82DF3428(ctx, base);
	// 82506EB4: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506EB8: 418200D8  beq 0x82506f90
	if ctx.cr[0].eq {
	pc = 0x82506F90; continue 'dispatch;
	}
	// 82506EBC: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506EC0: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506EC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82506EC8: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82506ECC: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82506ED0: 419A0024  beq cr6, 0x82506ef4
	if ctx.cr[6].eq {
	pc = 0x82506EF4; continue 'dispatch;
	}
	// 82506ED4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82506ED8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82506EDC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82506EE0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82506EE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82506EE8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82506EEC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82506EF0: 4082FFE8  bne 0x82506ed8
	if !ctx.cr[0].eq {
	pc = 0x82506ED8; continue 'dispatch;
	}
	// 82506EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82506EF8: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82506EFC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82506F00: 4BFFF231  bl 0x82506130
	ctx.lr = 0x82506F04;
	sub_82506130(ctx, base);
	// 82506F04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82506F08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82506F0C: 38A0008C  li r5, 0x8c
	ctx.r[5].s64 = 140;
	// 82506F10: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82506F14: 4BDB94C5  bl 0x822c03d8
	ctx.lr = 0x82506F18;
	sub_822C03D8(ctx, base);
	// 82506F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506F1C: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 82506F20: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82506F24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506F28: 4BFF9849  bl 0x82500770
	ctx.lr = 0x82506F2C;
	sub_82500770(ctx, base);
	// 82506F2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82506F30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506F34: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 82506F38: 4BDB90C9  bl 0x822c0000
	ctx.lr = 0x82506F3C;
	sub_822C0000(ctx, base);
	// 82506F3C: 81210078  lwz r9, 0x78(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82506F40: 92890000  stw r20, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82506F44: 817C7008  lwz r11, 0x7008(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28680 as u32) ) } as u64;
	// 82506F48: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506F4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82506F50: 409A000C  bne cr6, 0x82506f5c
	if !ctx.cr[6].eq {
	pc = 0x82506F5C; continue 'dispatch;
	}
	// 82506F54: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 82506F58: 48000010  b 0x82506f68
	pc = 0x82506F68; continue 'dispatch;
	// 82506F5C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82506F60: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82506F64: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82506F68: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82506F6C: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 82506F70: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82506F74: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506F78: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82506F7C: 486AC18D  bl 0x82bb3108
	ctx.lr = 0x82506F80;
	sub_82BB3108(ctx, base);
	// 82506F80: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82506F84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82506F88: 419A0008  beq cr6, 0x82506f90
	if ctx.cr[6].eq {
	pc = 0x82506F90; continue 'dispatch;
	}
	// 82506F8C: 4BDB9905  bl 0x822c0890
	ctx.lr = 0x82506F90;
	sub_822C0890(ctx, base);
	// 82506F90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82506F94: 488EC495  bl 0x82df3428
	ctx.lr = 0x82506F98;
	sub_82DF3428(ctx, base);
	// 82506F98: 4800012C  b 0x825070c4
	pc = 0x825070C4; continue 'dispatch;
	// 82506F9C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82506FA0: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506FA4: 480D29FD  bl 0x825d99a0
	ctx.lr = 0x82506FA8;
	sub_825D99A0(ctx, base);
	// 82506FA8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82506FAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82506FB0: 419A0104  beq cr6, 0x825070b4
	if ctx.cr[6].eq {
	pc = 0x825070B4; continue 'dispatch;
	}
	// 82506FB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82506FB8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82506FBC: 38A00095  li r5, 0x95
	ctx.r[5].s64 = 149;
	// 82506FC0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82506FC4: 4BDB9415  bl 0x822c03d8
	ctx.lr = 0x82506FC8;
	sub_822C03D8(ctx, base);
	// 82506FC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82506FCC: 41820010  beq 0x82506fdc
	if ctx.cr[0].eq {
	pc = 0x82506FDC; continue 'dispatch;
	}
	// 82506FD0: 4BFFCBF1  bl 0x82503bc0
	ctx.lr = 0x82506FD4;
	sub_82503BC0(ctx, base);
	// 82506FD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506FD8: 48000008  b 0x82506fe0
	pc = 0x82506FE0; continue 'dispatch;
	// 82506FDC: 7E9FA378  mr r31, r20
	ctx.r[31].u64 = ctx.r[20].u64;
	// 82506FE0: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82506FE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506FE8: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82506FEC: 4BFFF58D  bl 0x82506578
	ctx.lr = 0x82506FF0;
	sub_82506578(ctx, base);
	// 82506FF0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82506FF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82506FF8: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82506FFC: 4BDB9005  bl 0x822c0000
	ctx.lr = 0x82507000;
	sub_822C0000(ctx, base);
	// 82507000: 83C10080  lwz r30, 0x80(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82507004: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82507008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250700C: 488ECBC5  bl 0x82df3bd0
	ctx.lr = 0x82507010;
	sub_82DF3BD0(ctx, base);
	// 82507010: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507014: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82507018: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8250701C: 486AC0ED  bl 0x82bb3108
	ctx.lr = 0x82507020;
	sub_82BB3108(ctx, base);
	// 82507020: 83E10084  lwz r31, 0x84(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82507024: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507028: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 8250702C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82507030: 93E100AC  stw r31, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 82507034: 9A8B0004  stb r20, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[20].u8 ) };
	// 82507038: 419A0024  beq cr6, 0x8250705c
	if ctx.cr[6].eq {
	pc = 0x8250705C; continue 'dispatch;
	}
	// 8250703C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82507040: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82507044: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507048: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250704C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507050: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82507054: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507058: 4082FFE8  bne 0x82507040
	if !ctx.cr[0].eq {
	pc = 0x82507040; continue 'dispatch;
	}
	// 8250705C: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82507060: 81410088  lwz r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82507064: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507068: 916100B4  stw r11, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 8250706C: 914100B0  stw r10, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 82507070: 419A0024  beq cr6, 0x82507094
	if ctx.cr[6].eq {
	pc = 0x82507094; continue 'dispatch;
	}
	// 82507074: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82507078: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250707C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507080: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82507084: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507088: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250708C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507090: 4082FFE8  bne 0x82507078
	if !ctx.cr[0].eq {
	pc = 0x82507078; continue 'dispatch;
	}
	// 82507094: 38A100A8  addi r5, r1, 0xa8
	ctx.r[5].s64 = ctx.r[1].s64 + 168;
	// 82507098: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 8250709C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825070A0: 4BFFFB39  bl 0x82506bd8
	ctx.lr = 0x825070A4;
	sub_82506BD8(ctx, base);
	// 825070A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825070A8: 419A000C  beq cr6, 0x825070b4
	if ctx.cr[6].eq {
	pc = 0x825070B4; continue 'dispatch;
	}
	// 825070AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825070B0: 4BDB97E1  bl 0x822c0890
	ctx.lr = 0x825070B4;
	sub_822C0890(ctx, base);
	// 825070B4: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 825070B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825070BC: 419A0008  beq cr6, 0x825070c4
	if ctx.cr[6].eq {
	pc = 0x825070C4; continue 'dispatch;
	}
	// 825070C0: 4BDB97D1  bl 0x822c0890
	ctx.lr = 0x825070C4;
	sub_822C0890(ctx, base);
	// 825070C4: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 825070C8: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825070CC: 480D29ED  bl 0x825d9ab8
	ctx.lr = 0x825070D0;
	sub_825D9AB8(ctx, base);
	// 825070D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825070D4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 825070D8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825070DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825070E0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825070E4: 4BDBD37D  bl 0x822c4460
	ctx.lr = 0x825070E8;
	sub_822C4460(ctx, base);
	// 825070E8: 806100BC  lwz r3, 0xbc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 825070EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825070F0: 419A0008  beq cr6, 0x825070f8
	if ctx.cr[6].eq {
	pc = 0x825070F8; continue 'dispatch;
	}
	// 825070F4: 4BDB979D  bl 0x822c0890
	ctx.lr = 0x825070F8;
	sub_822C0890(ctx, base);
	// 825070F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825070FC: 488EC32D  bl 0x82df3428
	ctx.lr = 0x82507100;
	sub_82DF3428(ctx, base);
	// 82507100: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507104: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82507108: 409AFB38  bne cr6, 0x82506c40
	if !ctx.cr[6].eq {
	pc = 0x82506C40; continue 'dispatch;
	}
	// 8250710C: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507110: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507114: 419A0008  beq cr6, 0x8250711c
	if ctx.cr[6].eq {
	pc = 0x8250711C; continue 'dispatch;
	}
	// 82507118: 4BDB9779  bl 0x822c0890
	ctx.lr = 0x8250711C;
	sub_822C0890(ctx, base);
	// 8250711C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507120: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507124: 419A0008  beq cr6, 0x8250712c
	if ctx.cr[6].eq {
	pc = 0x8250712C; continue 'dispatch;
	}
	// 82507128: 4BDB9769  bl 0x822c0890
	ctx.lr = 0x8250712C;
	sub_822C0890(ctx, base);
	// 8250712C: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82507130: 48CA1058  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507138 size=704
    let mut pc: u32 = 0x82507138;
    'dispatch: loop {
        match pc {
            0x82507138 => {
    //   block [0x82507138..0x825073F8)
	// 82507138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250713C: 48CA1019  bl 0x831a8154
	ctx.lr = 0x82507140;
	sub_831A8130(ctx, base);
	// 82507140: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507144: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82507148: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8250714C: 3B0B1554  addi r24, r11, 0x1554
	ctx.r[24].s64 = ctx.r[11].s64 + 5460;
	// 82507150: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82507154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82507158: 38A0066D  li r5, 0x66d
	ctx.r[5].s64 = 1645;
	// 8250715C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82507160: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82507164: 4BDB9275  bl 0x822c03d8
	ctx.lr = 0x82507168;
	sub_822C03D8(ctx, base);
	// 82507168: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8250716C: 41820020  beq 0x8250718c
	if ctx.cr[0].eq {
	pc = 0x8250718C; continue 'dispatch;
	}
	// 82507170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507174: 488EBF7D  bl 0x82df30f0
	ctx.lr = 0x82507178;
	sub_82DF30F0(ctx, base);
	// 82507178: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8250717C: 933F0008  stw r25, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82507180: 933F000C  stw r25, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 82507184: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 82507188: 4800000C  b 0x82507194
	pc = 0x82507194; continue 'dispatch;
	// 8250718C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82507190: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82507194: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82507198: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250719C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 825071A0: 4BFFF491  bl 0x82506630
	ctx.lr = 0x825071A4;
	sub_82506630(ctx, base);
	// 825071A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825071A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825071AC: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 825071B0: 4BDB8E51  bl 0x822c0000
	ctx.lr = 0x825071B4;
	sub_822C0000(ctx, base);
	// 825071B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825071B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825071BC: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 825071C0: 3BCB1670  addi r30, r11, 0x1670
	ctx.r[30].s64 = ctx.r[11].s64 + 5744;
	// 825071C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825071C8: 480D2289  bl 0x825d9450
	ctx.lr = 0x825071CC;
	sub_825D9450(ctx, base);
	// 825071CC: 83E10070  lwz r31, 0x70(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825071D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825071D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825071D8: 488EC9F9  bl 0x82df3bd0
	ctx.lr = 0x825071DC;
	sub_82DF3BD0(ctx, base);
	// 825071DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825071E0: 488EC249  bl 0x82df3428
	ctx.lr = 0x825071E4;
	sub_82DF3428(ctx, base);
	// 825071E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825071E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825071EC: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 825071F0: 38AB1668  addi r5, r11, 0x1668
	ctx.r[5].s64 = ctx.r[11].s64 + 5736;
	// 825071F4: 480D29DD  bl 0x825d9bd0
	ctx.lr = 0x825071F8;
	sub_825D9BD0(ctx, base);
	// 825071F8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825071FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507200: 419A01A4  beq cr6, 0x825073a4
	if ctx.cr[6].eq {
	pc = 0x825073A4; continue 'dispatch;
	}
	// 82507204: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82507208: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250720C: 3B7F0004  addi r27, r31, 4
	ctx.r[27].s64 = ctx.r[31].s64 + 4;
	// 82507210: 3BAA1660  addi r29, r10, 0x1660
	ctx.r[29].s64 = ctx.r[10].s64 + 5728;
	// 82507214: 3B8B1654  addi r28, r11, 0x1654
	ctx.r[28].s64 = ctx.r[11].s64 + 5716;
	// 82507218: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8250721C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82507220: 38A00674  li r5, 0x674
	ctx.r[5].s64 = 1652;
	// 82507224: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82507228: 4BDB91B1  bl 0x822c03d8
	ctx.lr = 0x8250722C;
	sub_822C03D8(ctx, base);
	// 8250722C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82507230: 41820028  beq 0x82507258
	if ctx.cr[0].eq {
	pc = 0x82507258; continue 'dispatch;
	}
	// 82507234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507238: 488EBEB9  bl 0x82df30f0
	ctx.lr = 0x8250723C;
	sub_82DF30F0(ctx, base);
	// 8250723C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82507240: 488EBEB1  bl 0x82df30f0
	ctx.lr = 0x82507244;
	sub_82DF30F0(ctx, base);
	// 82507244: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82507248: 488EBEA9  bl 0x82df30f0
	ctx.lr = 0x8250724C;
	sub_82DF30F0(ctx, base);
	// 8250724C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82507250: 488EBEA1  bl 0x82df30f0
	ctx.lr = 0x82507254;
	sub_82DF30F0(ctx, base);
	// 82507254: 48000008  b 0x8250725c
	pc = 0x8250725C; continue 'dispatch;
	// 82507258: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8250725C: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82507260: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82507264: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82507268: 4BFFAC51  bl 0x82501eb8
	ctx.lr = 0x8250726C;
	sub_82501EB8(ctx, base);
	// 8250726C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82507270: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82507274: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82507278: 4BDB8D89  bl 0x822c0000
	ctx.lr = 0x8250727C;
	sub_822C0000(ctx, base);
	// 8250727C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82507280: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507284: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507288: 480D21C9  bl 0x825d9450
	ctx.lr = 0x8250728C;
	sub_825D9450(ctx, base);
	// 8250728C: 83E10068  lwz r31, 0x68(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82507290: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507298: 488EC939  bl 0x82df3bd0
	ctx.lr = 0x8250729C;
	sub_82DF3BD0(ctx, base);
	// 8250729C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825072A0: 488EC189  bl 0x82df3428
	ctx.lr = 0x825072A4;
	sub_82DF3428(ctx, base);
	// 825072A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825072A8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825072AC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825072B0: 480D2921  bl 0x825d9bd0
	ctx.lr = 0x825072B4;
	sub_825D9BD0(ctx, base);
	// 825072B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825072B8: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 825072BC: 480D2315  bl 0x825d95d0
	ctx.lr = 0x825072C0;
	sub_825D95D0(ctx, base);
	// 825072C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825072C4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825072C8: 488EC909  bl 0x82df3bd0
	ctx.lr = 0x825072CC;
	sub_82DF3BD0(ctx, base);
	// 825072CC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825072D0: 488EC159  bl 0x82df3428
	ctx.lr = 0x825072D4;
	sub_82DF3428(ctx, base);
	// 825072D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825072D8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825072DC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825072E0: 480D28F1  bl 0x825d9bd0
	ctx.lr = 0x825072E4;
	sub_825D9BD0(ctx, base);
	// 825072E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825072E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825072EC: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825072F0: 480D2161  bl 0x825d9450
	ctx.lr = 0x825072F4;
	sub_825D9450(ctx, base);
	// 825072F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825072F8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825072FC: 488EC8D5  bl 0x82df3bd0
	ctx.lr = 0x82507300;
	sub_82DF3BD0(ctx, base);
	// 82507300: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82507304: 488EC125  bl 0x82df3428
	ctx.lr = 0x82507308;
	sub_82DF3428(ctx, base);
	// 82507308: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250730C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82507310: 480D22C1  bl 0x825d95d0
	ctx.lr = 0x82507314;
	sub_825D95D0(ctx, base);
	// 82507314: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507318: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8250731C: 488EC8B5  bl 0x82df3bd0
	ctx.lr = 0x82507320;
	sub_82DF3BD0(ctx, base);
	// 82507320: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82507324: 488EC105  bl 0x82df3428
	ctx.lr = 0x82507328;
	sub_82DF3428(ctx, base);
	// 82507328: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8250732C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82507330: 486ABDD9  bl 0x82bb3108
	ctx.lr = 0x82507334;
	sub_82BB3108(ctx, base);
	// 82507334: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82507338: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250733C: 480D277D  bl 0x825d9ab8
	ctx.lr = 0x82507340;
	sub_825D9AB8(ctx, base);
	// 82507340: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507344: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82507348: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8250734C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507350: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82507354: 4BDBD10D  bl 0x822c4460
	ctx.lr = 0x82507358;
	sub_822C4460(ctx, base);
	// 82507358: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8250735C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507360: 419A0008  beq cr6, 0x82507368
	if ctx.cr[6].eq {
	pc = 0x82507368; continue 'dispatch;
	}
	// 82507364: 4BDB952D  bl 0x822c0890
	ctx.lr = 0x82507368;
	sub_822C0890(ctx, base);
	// 82507368: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8250736C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507370: 419A0008  beq cr6, 0x82507378
	if ctx.cr[6].eq {
	pc = 0x82507378; continue 'dispatch;
	}
	// 82507374: 4BDB951D  bl 0x822c0890
	ctx.lr = 0x82507378;
	sub_822C0890(ctx, base);
	// 82507378: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8250737C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507380: 419A0008  beq cr6, 0x82507388
	if ctx.cr[6].eq {
	pc = 0x82507388; continue 'dispatch;
	}
	// 82507384: 4BDB950D  bl 0x822c0890
	ctx.lr = 0x82507388;
	sub_822C0890(ctx, base);
	// 82507388: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8250738C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507390: 419A0008  beq cr6, 0x82507398
	if ctx.cr[6].eq {
	pc = 0x82507398; continue 'dispatch;
	}
	// 82507394: 4BDB94FD  bl 0x822c0890
	ctx.lr = 0x82507398;
	sub_822C0890(ctx, base);
	// 82507398: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250739C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825073A0: 409AFE78  bne cr6, 0x82507218
	if !ctx.cr[6].eq {
	pc = 0x82507218; continue 'dispatch;
	}
	// 825073A4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825073A8: 80770000  lwz r3, 0(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 825073AC: 486ABD5D  bl 0x82bb3108
	ctx.lr = 0x825073B0;
	sub_82BB3108(ctx, base);
	// 825073B0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825073B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825073B8: 419A0008  beq cr6, 0x825073c0
	if ctx.cr[6].eq {
	pc = 0x825073C0; continue 'dispatch;
	}
	// 825073BC: 4BDB94D5  bl 0x822c0890
	ctx.lr = 0x825073C0;
	sub_822C0890(ctx, base);
	// 825073C0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825073C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825073C8: 419A0008  beq cr6, 0x825073d0
	if ctx.cr[6].eq {
	pc = 0x825073D0; continue 'dispatch;
	}
	// 825073CC: 4BDB94C5  bl 0x822c0890
	ctx.lr = 0x825073D0;
	sub_822C0890(ctx, base);
	// 825073D0: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825073D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825073D8: 419A0008  beq cr6, 0x825073e0
	if ctx.cr[6].eq {
	pc = 0x825073E0; continue 'dispatch;
	}
	// 825073DC: 4BDB94B5  bl 0x822c0890
	ctx.lr = 0x825073E0;
	sub_822C0890(ctx, base);
	// 825073E0: 80770004  lwz r3, 4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 825073E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825073E8: 419A0008  beq cr6, 0x825073f0
	if ctx.cr[6].eq {
	pc = 0x825073F0; continue 'dispatch;
	}
	// 825073EC: 4BDB94A5  bl 0x822c0890
	ctx.lr = 0x825073F0;
	sub_822C0890(ctx, base);
	// 825073F0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 825073F4: 48CA0DB0  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825073F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825073F8 size=112
    let mut pc: u32 = 0x825073F8;
    'dispatch: loop {
        match pc {
            0x825073F8 => {
    //   block [0x825073F8..0x82507468)
	// 825073F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825073FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82507400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82507404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82507408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250740C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82507410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507414: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82507418: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250741C: 4BFFF15D  bl 0x82506578
	ctx.lr = 0x82507420;
	sub_82506578(ctx, base);
	// 82507420: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82507424: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82507428: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250742C: 4BDB8BD5  bl 0x822c0000
	ctx.lr = 0x82507430;
	sub_822C0000(ctx, base);
	// 82507430: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82507434: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82507438: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250743C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507444: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82507448: 419A0008  beq cr6, 0x82507450
	if ctx.cr[6].eq {
	pc = 0x82507450; continue 'dispatch;
	}
	// 8250744C: 4BDB9445  bl 0x822c0890
	ctx.lr = 0x82507450;
	sub_822C0890(ctx, base);
	// 82507450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82507454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82507458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250745C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82507460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82507464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82507468 size=872
    let mut pc: u32 = 0x82507468;
    'dispatch: loop {
        match pc {
            0x82507468 => {
    //   block [0x82507468..0x825077D0)
	// 82507468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250746C: 48CA0CED  bl 0x831a8158
	ctx.lr = 0x82507470;
	sub_831A8130(ctx, base);
	// 82507470: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507474: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82507478: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250747C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82507480: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82507484: 480D1A75  bl 0x825d8ef8
	ctx.lr = 0x82507488;
	sub_825D8EF8(ctx, base);
	// 82507488: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250748C: 488EBD25  bl 0x82df31b0
	ctx.lr = 0x82507490;
	sub_82DF31B0(ctx, base);
	// 82507490: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507494: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82507498: 488EC571  bl 0x82df3a08
	ctx.lr = 0x8250749C;
	sub_82DF3A08(ctx, base);
	// 8250749C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825074A0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825074A4: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825074A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825074AC: 4BF178BD  bl 0x8241ed68
	ctx.lr = 0x825074B0;
	sub_8241ED68(ctx, base);
	// 825074B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825074B4: 488EBF75  bl 0x82df3428
	ctx.lr = 0x825074B8;
	sub_82DF3428(ctx, base);
	// 825074B8: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825074BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825074C0: 419A01A4  beq cr6, 0x82507664
	if ctx.cr[6].eq {
	pc = 0x82507664; continue 'dispatch;
	}
	// 825074C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825074C8: 488F7319  bl 0x82dfe7e0
	ctx.lr = 0x825074CC;
	sub_82DFE7E0(ctx, base);
	// 825074CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825074D0: 83410054  lwz r26, 0x54(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825074D4: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825074D8: 3BCB1554  addi r30, r11, 0x1554
	ctx.r[30].s64 = ctx.r[11].s64 + 5460;
	// 825074DC: 40820058  bne 0x82507534
	if !ctx.cr[0].eq {
	pc = 0x82507534; continue 'dispatch;
	}
	// 825074E0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825074E4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 825074E8: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 825074EC: 419A0024  beq cr6, 0x82507510
	if ctx.cr[6].eq {
	pc = 0x82507510; continue 'dispatch;
	}
	// 825074F0: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 825074F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825074F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825074FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82507500: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507504: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82507508: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250750C: 4082FFE8  bne 0x825074f4
	if !ctx.cr[0].eq {
	pc = 0x825074F4; continue 'dispatch;
	}
	// 82507510: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82507514: 4BFE3575  bl 0x824eaa88
	ctx.lr = 0x82507518;
	sub_824EAA88(ctx, base);
	// 82507518: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250751C: 38C0003E  li r6, 0x3e
	ctx.r[6].s64 = 62;
	// 82507520: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507524: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82507528: 486B2C59  bl 0x82bba180
	ctx.lr = 0x8250752C;
	sub_82BBA180(ctx, base);
	// 8250752C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82507530: 488EA761  bl 0x82df1c90
	ctx.lr = 0x82507534;
	sub_82DF1C90(ctx, base);
	// 82507534: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82507538: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8250753C: 489933CD  bl 0x82e9a908
	ctx.lr = 0x82507540;
	sub_82E9A908(ctx, base);
	// 82507540: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507544: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82507548: 832B0000  lwz r25, 0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250754C: 488EBC65  bl 0x82df31b0
	ctx.lr = 0x82507550;
	sub_82DF31B0(ctx, base);
	// 82507550: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82507554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507558: 48B00BD1  bl 0x83008128
	ctx.lr = 0x8250755C;
	sub_83008128(ctx, base);
	// 8250755C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82507560: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82507564: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82507568: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250756C: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82507570: 480D1BB9  bl 0x825d9128
	ctx.lr = 0x82507574;
	sub_825D9128(ctx, base);
	// 82507574: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82507578: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250757C: 419A0008  beq cr6, 0x82507584
	if ctx.cr[6].eq {
	pc = 0x82507584; continue 'dispatch;
	}
	// 82507580: 4BDB9311  bl 0x822c0890
	ctx.lr = 0x82507584;
	sub_822C0890(ctx, base);
	// 82507584: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82507588: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8250758C: 419A0214  beq cr6, 0x825077a0
	if ctx.cr[6].eq {
	pc = 0x825077A0; continue 'dispatch;
	}
	// 82507590: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82507594: 480D1275  bl 0x825d8808
	ctx.lr = 0x82507598;
	sub_825D8808(ctx, base);
	// 82507598: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250759C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825075A0: 419A01C8  beq cr6, 0x82507768
	if ctx.cr[6].eq {
	pc = 0x82507768; continue 'dispatch;
	}
	// 825075A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825075A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825075AC: 38AB1684  addi r5, r11, 0x1684
	ctx.r[5].s64 = ctx.r[11].s64 + 5764;
	// 825075B0: 480D1EA1  bl 0x825d9450
	ctx.lr = 0x825075B4;
	sub_825D9450(ctx, base);
	// 825075B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825075B8: 488EBBF9  bl 0x82df31b0
	ctx.lr = 0x825075BC;
	sub_82DF31B0(ctx, base);
	// 825075BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825075C0: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825075C4: 4BFFB175  bl 0x82502738
	ctx.lr = 0x825075C8;
	sub_82502738(ctx, base);
	// 825075C8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825075CC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 825075D0: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 825075D4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825075D8: 4BFFC941  bl 0x82503f18
	ctx.lr = 0x825075DC;
	sub_82503F18(ctx, base);
	// 825075DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825075E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825075E4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825075E8: 4BFF9931  bl 0x82500f18
	ctx.lr = 0x825075EC;
	sub_82500F18(ctx, base);
	// 825075EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825075F0: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825075F4: 4BFFC7C5  bl 0x82503db8
	ctx.lr = 0x825075F8;
	sub_82503DB8(ctx, base);
	// 825075F8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825075FC: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82507600: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82507604: 816B7008  lwz r11, 0x7008(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28680 as u32) ) } as u64;
	// 82507608: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8250760C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82507610: 480D2391  bl 0x825d99a0
	ctx.lr = 0x82507614;
	sub_825D99A0(ctx, base);
	// 82507614: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507618: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250761C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82507620: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82507624: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82507628: 4BDBCE39  bl 0x822c4460
	ctx.lr = 0x8250762C;
	sub_822C4460(ctx, base);
	// 8250762C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82507630: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507634: 419A0008  beq cr6, 0x8250763c
	if ctx.cr[6].eq {
	pc = 0x8250763C; continue 'dispatch;
	}
	// 82507638: 4BDB9259  bl 0x822c0890
	ctx.lr = 0x8250763C;
	sub_822C0890(ctx, base);
	// 8250763C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82507640: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82507644: 38A0005B  li r5, 0x5b
	ctx.r[5].s64 = 91;
	// 82507648: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8250764C: 4BDB8D8D  bl 0x822c03d8
	ctx.lr = 0x82507650;
	sub_822C03D8(ctx, base);
	// 82507650: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82507654: 41820020  beq 0x82507674
	if ctx.cr[0].eq {
	pc = 0x82507674; continue 'dispatch;
	}
	// 82507658: 4BFFC569  bl 0x82503bc0
	ctx.lr = 0x8250765C;
	sub_82503BC0(ctx, base);
	// 8250765C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507660: 48000018  b 0x82507678
	pc = 0x82507678; continue 'dispatch;
	// 82507664: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82507668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250766C: 419A0154  beq cr6, 0x825077c0
	if ctx.cr[6].eq {
	pc = 0x825077C0; continue 'dispatch;
	}
	// 82507670: 4800014C  b 0x825077bc
	pc = 0x825077BC; continue 'dispatch;
	// 82507674: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82507678: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250767C: 3BEB7010  addi r31, r11, 0x7010
	ctx.r[31].s64 = ctx.r[11].s64 + 28688;
	// 82507680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507684: 4BFFFD75  bl 0x825073f8
	ctx.lr = 0x82507688;
	sub_825073F8(ctx, base);
	// 82507688: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250768C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507690: 388B167C  addi r4, r11, 0x167c
	ctx.r[4].s64 = ctx.r[11].s64 + 5756;
	// 82507694: 488EC1E5  bl 0x82df3878
	ctx.lr = 0x82507698;
	sub_82DF3878(ctx, base);
	// 82507698: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8250769C: 4BFFAA6D  bl 0x82502108
	ctx.lr = 0x825076A0;
	sub_82502108(ctx, base);
	// 825076A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825076A4: 488EBD85  bl 0x82df3428
	ctx.lr = 0x825076A8;
	sub_82DF3428(ctx, base);
	// 825076A8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825076AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825076B0: 419A00B8  beq cr6, 0x82507768
	if ctx.cr[6].eq {
	pc = 0x82507768; continue 'dispatch;
	}
	// 825076B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825076B8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825076BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825076C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825076C4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825076C8: 419A0024  beq cr6, 0x825076ec
	if ctx.cr[6].eq {
	pc = 0x825076EC; continue 'dispatch;
	}
	// 825076CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825076D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825076D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825076D8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825076DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825076E0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825076E4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825076E8: 4082FFE8  bne 0x825076d0
	if !ctx.cr[0].eq {
	pc = 0x825076D0; continue 'dispatch;
	}
	// 825076EC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825076F0: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825076F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825076F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825076FC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82507700: 419A0024  beq cr6, 0x82507724
	if ctx.cr[6].eq {
	pc = 0x82507724; continue 'dispatch;
	}
	// 82507704: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82507708: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250770C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507710: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82507714: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507718: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250771C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507720: 4082FFE8  bne 0x82507708
	if !ctx.cr[0].eq {
	pc = 0x82507708; continue 'dispatch;
	}
	// 82507724: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82507728: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8250772C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82507730: 4BFFF4A9  bl 0x82506bd8
	ctx.lr = 0x82507734;
	sub_82506BD8(ctx, base);
	// 82507734: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82507738: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250773C: 480D237D  bl 0x825d9ab8
	ctx.lr = 0x82507740;
	sub_825D9AB8(ctx, base);
	// 82507740: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507744: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82507748: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8250774C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507750: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82507754: 4BDBCD0D  bl 0x822c4460
	ctx.lr = 0x82507758;
	sub_822C4460(ctx, base);
	// 82507758: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8250775C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507760: 419A0008  beq cr6, 0x82507768
	if ctx.cr[6].eq {
	pc = 0x82507768; continue 'dispatch;
	}
	// 82507764: 4BDB912D  bl 0x822c0890
	ctx.lr = 0x82507768;
	sub_822C0890(ctx, base);
	// 82507768: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250776C: 488EBA45  bl 0x82df31b0
	ctx.lr = 0x82507770;
	sub_82DF31B0(ctx, base);
	// 82507770: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507774: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82507778: 488EC291  bl 0x82df3a08
	ctx.lr = 0x8250777C;
	sub_82DF3A08(ctx, base);
	// 8250777C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82507780: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507784: 488F9DF5  bl 0x82e01578
	ctx.lr = 0x82507788;
	sub_82E01578(ctx, base);
	// 82507788: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250778C: 488EBC9D  bl 0x82df3428
	ctx.lr = 0x82507790;
	sub_82DF3428(ctx, base);
	// 82507790: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82507794: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507798: 419A0008  beq cr6, 0x825077a0
	if ctx.cr[6].eq {
	pc = 0x825077A0; continue 'dispatch;
	}
	// 8250779C: 4BDB90F5  bl 0x822c0890
	ctx.lr = 0x825077A0;
	sub_822C0890(ctx, base);
	// 825077A0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825077A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825077A8: 419A0008  beq cr6, 0x825077b0
	if ctx.cr[6].eq {
	pc = 0x825077B0; continue 'dispatch;
	}
	// 825077AC: 4BDB90E5  bl 0x822c0890
	ctx.lr = 0x825077B0;
	sub_822C0890(ctx, base);
	// 825077B0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 825077B4: 419A000C  beq cr6, 0x825077c0
	if ctx.cr[6].eq {
	pc = 0x825077C0; continue 'dispatch;
	}
	// 825077B8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825077BC: 4BDB90D5  bl 0x822c0890
	ctx.lr = 0x825077C0;
	sub_822C0890(ctx, base);
	// 825077C0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825077C4: 480D17ED  bl 0x825d8fb0
	ctx.lr = 0x825077C8;
	sub_825D8FB0(ctx, base);
	// 825077C8: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 825077CC: 48CA09DC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825077D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825077D0 size=816
    let mut pc: u32 = 0x825077D0;
    'dispatch: loop {
        match pc {
            0x825077D0 => {
    //   block [0x825077D0..0x82507B00)
	// 825077D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825077D4: 48CA0979  bl 0x831a814c
	ctx.lr = 0x825077D8;
	sub_831A8130(ctx, base);
	// 825077D8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825077DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825077E0: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 825077E4: 3B0B1554  addi r24, r11, 0x1554
	ctx.r[24].s64 = ctx.r[11].s64 + 5460;
	// 825077E8: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 825077EC: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 825077F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825077F4: 38A000C9  li r5, 0xc9
	ctx.r[5].s64 = 201;
	// 825077F8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825077FC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82507800: 4BDB8BD9  bl 0x822c03d8
	ctx.lr = 0x82507804;
	sub_822C03D8(ctx, base);
	// 82507804: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82507808: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8250780C: 41820024  beq 0x82507830
	if ctx.cr[0].eq {
	pc = 0x82507830; continue 'dispatch;
	}
	// 82507810: 93430004  stw r26, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82507814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507818: 93430008  stw r26, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 8250781C: 9343000C  stw r26, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 82507820: 93430014  stw r26, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82507824: 93430018  stw r26, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 82507828: 9343001C  stw r26, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 8250782C: 48000008  b 0x82507834
	pc = 0x82507834; continue 'dispatch;
	// 82507830: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82507834: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82507838: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250783C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82507840: 4BFFF181  bl 0x825069c0
	ctx.lr = 0x82507844;
	sub_825069C0(ctx, base);
	// 82507844: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82507848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250784C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82507850: 4BDB87B1  bl 0x822c0000
	ctx.lr = 0x82507854;
	sub_822C0000(ctx, base);
	// 82507854: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82507858: 80960000  lwz r4, 0(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250785C: 480D2145  bl 0x825d99a0
	ctx.lr = 0x82507860;
	sub_825D99A0(ctx, base);
	// 82507860: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507864: 83210074  lwz r25, 0x74(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82507868: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8250786C: 419A0240  beq cr6, 0x82507aac
	if ctx.cr[6].eq {
	pc = 0x82507AAC; continue 'dispatch;
	}
	// 82507870: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82507874: 83810070  lwz r28, 0x70(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82507878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8250787C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82507880: 3BCB1698  addi r30, r11, 0x1698
	ctx.r[30].s64 = ctx.r[11].s64 + 5784;
	// 82507884: 3B6A168C  addi r27, r10, 0x168c
	ctx.r[27].s64 = ctx.r[10].s64 + 5772;
	// 82507888: 3BA9E984  addi r29, r9, -0x167c
	ctx.r[29].s64 = ctx.r[9].s64 + -5756;
	// 8250788C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507890: 480D1CD9  bl 0x825d9568
	ctx.lr = 0x82507894;
	sub_825D9568(ctx, base);
	// 82507894: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82507898: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250789C: 488EC16D  bl 0x82df3a08
	ctx.lr = 0x825078A0;
	sub_82DF3A08(ctx, base);
	// 825078A0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825078A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825078A8: 488EBA61  bl 0x82df3308
	ctx.lr = 0x825078AC;
	sub_82DF3308(ctx, base);
	// 825078AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825078B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825078B4: 488EBB75  bl 0x82df3428
	ctx.lr = 0x825078B8;
	sub_82DF3428(ctx, base);
	// 825078B8: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825078BC: 418200F0  beq 0x825079ac
	if ctx.cr[0].eq {
	pc = 0x825079AC; continue 'dispatch;
	}
	// 825078C0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825078C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825078C8: 38A000D4  li r5, 0xd4
	ctx.r[5].s64 = 212;
	// 825078CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825078D0: 4BDB8B09  bl 0x822c03d8
	ctx.lr = 0x825078D4;
	sub_822C03D8(ctx, base);
	// 825078D4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825078D8: 41820028  beq 0x82507900
	if ctx.cr[0].eq {
	pc = 0x82507900; continue 'dispatch;
	}
	// 825078DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825078E0: 488EB811  bl 0x82df30f0
	ctx.lr = 0x825078E4;
	sub_82DF30F0(ctx, base);
	// 825078E4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825078E8: 488EB809  bl 0x82df30f0
	ctx.lr = 0x825078EC;
	sub_82DF30F0(ctx, base);
	// 825078EC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825078F0: 488EB801  bl 0x82df30f0
	ctx.lr = 0x825078F4;
	sub_82DF30F0(ctx, base);
	// 825078F4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 825078F8: 488EB7F9  bl 0x82df30f0
	ctx.lr = 0x825078FC;
	sub_82DF30F0(ctx, base);
	// 825078FC: 48000008  b 0x82507904
	pc = 0x82507904; continue 'dispatch;
	// 82507900: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82507904: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82507908: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250790C: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 82507910: 4BFFA5A9  bl 0x82501eb8
	ctx.lr = 0x82507914;
	sub_82501EB8(ctx, base);
	// 82507914: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82507918: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250791C: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 82507920: 4BDB86E1  bl 0x822c0000
	ctx.lr = 0x82507924;
	sub_822C0000(ctx, base);
	// 82507924: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 82507928: 93210084  stw r25, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[25].u32 ) };
	// 8250792C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82507930: 419A0024  beq cr6, 0x82507954
	if ctx.cr[6].eq {
	pc = 0x82507954; continue 'dispatch;
	}
	// 82507934: 39790004  addi r11, r25, 4
	ctx.r[11].s64 = ctx.r[25].s64 + 4;
	// 82507938: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250793C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507940: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82507944: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507948: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250794C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507950: 4082FFE8  bne 0x82507938
	if !ctx.cr[0].eq {
	pc = 0x82507938; continue 'dispatch;
	}
	// 82507954: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82507958: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250795C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507960: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82507964: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82507968: 419A0024  beq cr6, 0x8250798c
	if ctx.cr[6].eq {
	pc = 0x8250798C; continue 'dispatch;
	}
	// 8250796C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82507970: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82507974: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507978: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250797C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507980: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82507984: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507988: 4082FFE8  bne 0x82507970
	if !ctx.cr[0].eq {
	pc = 0x82507970; continue 'dispatch;
	}
	// 8250798C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82507990: 38810088  addi r4, r1, 0x88
	ctx.r[4].s64 = ctx.r[1].s64 + 136;
	// 82507994: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82507998: 4BFFF7A1  bl 0x82507138
	ctx.lr = 0x8250799C;
	sub_82507138(ctx, base);
	// 8250799C: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 825079A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825079A4: 419A0008  beq cr6, 0x825079ac
	if ctx.cr[6].eq {
	pc = 0x825079AC; continue 'dispatch;
	}
	// 825079A8: 4BDB8EE9  bl 0x822c0890
	ctx.lr = 0x825079AC;
	sub_822C0890(ctx, base);
	// 825079AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825079B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825079B4: 488EC055  bl 0x82df3a08
	ctx.lr = 0x825079B8;
	sub_82DF3A08(ctx, base);
	// 825079B8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825079BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825079C0: 488EB949  bl 0x82df3308
	ctx.lr = 0x825079C4;
	sub_82DF3308(ctx, base);
	// 825079C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825079C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825079CC: 488EBA5D  bl 0x82df3428
	ctx.lr = 0x825079D0;
	sub_82DF3428(ctx, base);
	// 825079D0: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825079D4: 41820090  beq 0x82507a64
	if ctx.cr[0].eq {
	pc = 0x82507A64; continue 'dispatch;
	}
	// 825079D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825079DC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825079E0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825079E4: 480D21ED  bl 0x825d9bd0
	ctx.lr = 0x825079E8;
	sub_825D9BD0(ctx, base);
	// 825079E8: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825079EC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825079F0: 419A0064  beq cr6, 0x82507a54
	if ctx.cr[6].eq {
	pc = 0x82507A54; continue 'dispatch;
	}
	// 825079F4: 3BFC0010  addi r31, r28, 0x10
	ctx.r[31].s64 = ctx.r[28].s64 + 16;
	// 825079F8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825079FC: 480D1BD5  bl 0x825d95d0
	ctx.lr = 0x82507A00;
	sub_825D95D0(ctx, base);
	// 82507A00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507A08: 4BDE7321  bl 0x822eed28
	ctx.lr = 0x82507A0C;
	sub_822EED28(ctx, base);
	// 82507A0C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82507A10: 488EBA19  bl 0x82df3428
	ctx.lr = 0x82507A14;
	sub_82DF3428(ctx, base);
	// 82507A14: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82507A18: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82507A1C: 480D209D  bl 0x825d9ab8
	ctx.lr = 0x82507A20;
	sub_825D9AB8(ctx, base);
	// 82507A20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507A24: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82507A28: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82507A2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507A30: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82507A34: 4BDBCA2D  bl 0x822c4460
	ctx.lr = 0x82507A38;
	sub_822C4460(ctx, base);
	// 82507A38: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82507A3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507A40: 419A0008  beq cr6, 0x82507a48
	if ctx.cr[6].eq {
	pc = 0x82507A48; continue 'dispatch;
	}
	// 82507A44: 4BDB8E4D  bl 0x822c0890
	ctx.lr = 0x82507A48;
	sub_822C0890(ctx, base);
	// 82507A48: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82507A4C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82507A50: 409AFFA8  bne cr6, 0x825079f8
	if !ctx.cr[6].eq {
	pc = 0x825079F8; continue 'dispatch;
	}
	// 82507A54: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82507A58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507A5C: 419A0008  beq cr6, 0x82507a64
	if ctx.cr[6].eq {
	pc = 0x82507A64; continue 'dispatch;
	}
	// 82507A60: 4BDB8E31  bl 0x822c0890
	ctx.lr = 0x82507A64;
	sub_822C0890(ctx, base);
	// 82507A64: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82507A68: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507A6C: 480D204D  bl 0x825d9ab8
	ctx.lr = 0x82507A70;
	sub_825D9AB8(ctx, base);
	// 82507A70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507A74: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82507A78: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82507A7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507A80: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82507A84: 4BDBC9DD  bl 0x822c4460
	ctx.lr = 0x82507A88;
	sub_822C4460(ctx, base);
	// 82507A88: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82507A8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507A90: 419A0008  beq cr6, 0x82507a98
	if ctx.cr[6].eq {
	pc = 0x82507A98; continue 'dispatch;
	}
	// 82507A94: 4BDB8DFD  bl 0x822c0890
	ctx.lr = 0x82507A98;
	sub_822C0890(ctx, base);
	// 82507A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507A9C: 488EB98D  bl 0x82df3428
	ctx.lr = 0x82507AA0;
	sub_82DF3428(ctx, base);
	// 82507AA0: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507AA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82507AA8: 409AFDE4  bne cr6, 0x8250788c
	if !ctx.cr[6].eq {
	pc = 0x8250788C; continue 'dispatch;
	}
	// 82507AAC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82507AB0: 38770008  addi r3, r23, 8
	ctx.r[3].s64 = ctx.r[23].s64 + 8;
	// 82507AB4: 486AB655  bl 0x82bb3108
	ctx.lr = 0x82507AB8;
	sub_82BB3108(ctx, base);
	// 82507AB8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82507ABC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507AC0: 419A0008  beq cr6, 0x82507ac8
	if ctx.cr[6].eq {
	pc = 0x82507AC8; continue 'dispatch;
	}
	// 82507AC4: 4BDB8DCD  bl 0x822c0890
	ctx.lr = 0x82507AC8;
	sub_822C0890(ctx, base);
	// 82507AC8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82507ACC: 419A000C  beq cr6, 0x82507ad8
	if ctx.cr[6].eq {
	pc = 0x82507AD8; continue 'dispatch;
	}
	// 82507AD0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82507AD4: 4BDB8DBD  bl 0x822c0890
	ctx.lr = 0x82507AD8;
	sub_822C0890(ctx, base);
	// 82507AD8: 80760004  lwz r3, 4(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507ADC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507AE0: 419A0008  beq cr6, 0x82507ae8
	if ctx.cr[6].eq {
	pc = 0x82507AE8; continue 'dispatch;
	}
	// 82507AE4: 4BDB8DAD  bl 0x822c0890
	ctx.lr = 0x82507AE8;
	sub_822C0890(ctx, base);
	// 82507AE8: 80750004  lwz r3, 4(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507AEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507AF0: 419A0008  beq cr6, 0x82507af8
	if ctx.cr[6].eq {
	pc = 0x82507AF8; continue 'dispatch;
	}
	// 82507AF4: 4BDB8D9D  bl 0x822c0890
	ctx.lr = 0x82507AF8;
	sub_822C0890(ctx, base);
	// 82507AF8: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82507AFC: 48CA06A0  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507B00 size=692
    let mut pc: u32 = 0x82507B00;
    'dispatch: loop {
        match pc {
            0x82507B00 => {
    //   block [0x82507B00..0x82507DB4)
	// 82507B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82507B04: 48CA0659  bl 0x831a815c
	ctx.lr = 0x82507B08;
	sub_831A8130(ctx, base);
	// 82507B08: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507B0C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82507B10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507B14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82507B18: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82507B1C: 480D13DD  bl 0x825d8ef8
	ctx.lr = 0x82507B20;
	sub_825D8EF8(ctx, base);
	// 82507B20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82507B24: 488EB68D  bl 0x82df31b0
	ctx.lr = 0x82507B28;
	sub_82DF31B0(ctx, base);
	// 82507B28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507B2C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82507B30: 488EBED9  bl 0x82df3a08
	ctx.lr = 0x82507B34;
	sub_82DF3A08(ctx, base);
	// 82507B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82507B38: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82507B3C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507B40: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82507B44: 4BF17225  bl 0x8241ed68
	ctx.lr = 0x82507B48;
	sub_8241ED68(ctx, base);
	// 82507B48: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82507B4C: 488EB8DD  bl 0x82df3428
	ctx.lr = 0x82507B50;
	sub_82DF3428(ctx, base);
	// 82507B50: 83E10068  lwz r31, 0x68(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82507B54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82507B58: 419A024C  beq cr6, 0x82507da4
	if ctx.cr[6].eq {
	pc = 0x82507DA4; continue 'dispatch;
	}
	// 82507B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507B60: 488F6C81  bl 0x82dfe7e0
	ctx.lr = 0x82507B64;
	sub_82DFE7E0(ctx, base);
	// 82507B64: 8361006C  lwz r27, 0x6c(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82507B68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82507B6C: 4082005C  bne 0x82507bc8
	if !ctx.cr[0].eq {
	pc = 0x82507BC8; continue 'dispatch;
	}
	// 82507B70: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82507B74: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82507B78: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82507B7C: 419A0024  beq cr6, 0x82507ba0
	if ctx.cr[6].eq {
	pc = 0x82507BA0; continue 'dispatch;
	}
	// 82507B80: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 82507B84: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82507B88: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507B8C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82507B90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507B94: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82507B98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507B9C: 4082FFE8  bne 0x82507b84
	if !ctx.cr[0].eq {
	pc = 0x82507B84; continue 'dispatch;
	}
	// 82507BA0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82507BA4: 4BFE2EE5  bl 0x824eaa88
	ctx.lr = 0x82507BA8;
	sub_824EAA88(ctx, base);
	// 82507BA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82507BAC: 38C000A8  li r6, 0xa8
	ctx.r[6].s64 = 168;
	// 82507BB0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507BB4: 38AB1554  addi r5, r11, 0x1554
	ctx.r[5].s64 = ctx.r[11].s64 + 5460;
	// 82507BB8: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82507BBC: 486B25C5  bl 0x82bba180
	ctx.lr = 0x82507BC0;
	sub_82BBA180(ctx, base);
	// 82507BC0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82507BC4: 488EA0CD  bl 0x82df1c90
	ctx.lr = 0x82507BC8;
	sub_82DF1C90(ctx, base);
	// 82507BC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82507BCC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82507BD0: 48992D39  bl 0x82e9a908
	ctx.lr = 0x82507BD4;
	sub_82E9A908(ctx, base);
	// 82507BD4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507BD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82507BDC: 834B0000  lwz r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507BE0: 488EB5D1  bl 0x82df31b0
	ctx.lr = 0x82507BE4;
	sub_82DF31B0(ctx, base);
	// 82507BE4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82507BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507BEC: 48B0053D  bl 0x83008128
	ctx.lr = 0x82507BF0;
	sub_83008128(ctx, base);
	// 82507BF0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82507BF4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82507BF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82507BFC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82507C00: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82507C04: 480D1525  bl 0x825d9128
	ctx.lr = 0x82507C08;
	sub_825D9128(ctx, base);
	// 82507C08: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82507C0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507C10: 419A0008  beq cr6, 0x82507c18
	if ctx.cr[6].eq {
	pc = 0x82507C18; continue 'dispatch;
	}
	// 82507C14: 4BDB8C7D  bl 0x822c0890
	ctx.lr = 0x82507C18;
	sub_822C0890(ctx, base);
	// 82507C18: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82507C1C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82507C20: 419A0154  beq cr6, 0x82507d74
	if ctx.cr[6].eq {
	pc = 0x82507D74; continue 'dispatch;
	}
	// 82507C24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82507C28: 480D0BE1  bl 0x825d8808
	ctx.lr = 0x82507C2C;
	sub_825D8808(ctx, base);
	// 82507C2C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507C30: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82507C34: 419A0108  beq cr6, 0x82507d3c
	if ctx.cr[6].eq {
	pc = 0x82507D3C; continue 'dispatch;
	}
	// 82507C38: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82507C3C: 480D1D65  bl 0x825d99a0
	ctx.lr = 0x82507C40;
	sub_825D99A0(ctx, base);
	// 82507C40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507C44: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82507C48: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82507C4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507C50: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82507C54: 4BDBC80D  bl 0x822c4460
	ctx.lr = 0x82507C58;
	sub_822C4460(ctx, base);
	// 82507C58: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82507C5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507C60: 419A0008  beq cr6, 0x82507c68
	if ctx.cr[6].eq {
	pc = 0x82507C68; continue 'dispatch;
	}
	// 82507C64: 4BDB8C2D  bl 0x822c0890
	ctx.lr = 0x82507C68;
	sub_822C0890(ctx, base);
	// 82507C68: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507C6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507C70: 419A00CC  beq cr6, 0x82507d3c
	if ctx.cr[6].eq {
	pc = 0x82507D3C; continue 'dispatch;
	}
	// 82507C74: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82507C78: 3BEB7010  addi r31, r11, 0x7010
	ctx.r[31].s64 = ctx.r[11].s64 + 28688;
	// 82507C7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507C80: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82507C8C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82507C90: 419A0024  beq cr6, 0x82507cb4
	if ctx.cr[6].eq {
	pc = 0x82507CB4; continue 'dispatch;
	}
	// 82507C94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82507C98: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82507C9C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507CA0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82507CA4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507CA8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82507CAC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507CB0: 4082FFE8  bne 0x82507c98
	if !ctx.cr[0].eq {
	pc = 0x82507C98; continue 'dispatch;
	}
	// 82507CB4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82507CB8: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507CBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507CC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82507CC4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82507CC8: 419A0024  beq cr6, 0x82507cec
	if ctx.cr[6].eq {
	pc = 0x82507CEC; continue 'dispatch;
	}
	// 82507CCC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82507CD0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82507CD4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507CD8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82507CDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82507CE0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82507CE4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82507CE8: 4082FFE8  bne 0x82507cd0
	if !ctx.cr[0].eq {
	pc = 0x82507CD0; continue 'dispatch;
	}
	// 82507CEC: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82507CF0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82507CF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82507CF8: 4BFFFAD9  bl 0x825077d0
	ctx.lr = 0x82507CFC;
	sub_825077D0(ctx, base);
	// 82507CFC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82507D00: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507D04: 480D1DB5  bl 0x825d9ab8
	ctx.lr = 0x82507D08;
	sub_825D9AB8(ctx, base);
	// 82507D08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82507D0C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82507D10: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82507D14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507D18: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82507D1C: 4BDBC745  bl 0x822c4460
	ctx.lr = 0x82507D20;
	sub_822C4460(ctx, base);
	// 82507D20: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82507D24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507D28: 419A0008  beq cr6, 0x82507d30
	if ctx.cr[6].eq {
	pc = 0x82507D30; continue 'dispatch;
	}
	// 82507D2C: 4BDB8B65  bl 0x822c0890
	ctx.lr = 0x82507D30;
	sub_822C0890(ctx, base);
	// 82507D30: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82507D34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507D38: 409AFF44  bne cr6, 0x82507c7c
	if !ctx.cr[6].eq {
	pc = 0x82507C7C; continue 'dispatch;
	}
	// 82507D3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82507D40: 488EB471  bl 0x82df31b0
	ctx.lr = 0x82507D44;
	sub_82DF31B0(ctx, base);
	// 82507D44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507D48: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82507D4C: 488EBCBD  bl 0x82df3a08
	ctx.lr = 0x82507D50;
	sub_82DF3A08(ctx, base);
	// 82507D50: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82507D54: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507D58: 488F9821  bl 0x82e01578
	ctx.lr = 0x82507D5C;
	sub_82E01578(ctx, base);
	// 82507D5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82507D60: 488EB6C9  bl 0x82df3428
	ctx.lr = 0x82507D64;
	sub_82DF3428(ctx, base);
	// 82507D64: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82507D68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507D6C: 419A0008  beq cr6, 0x82507d74
	if ctx.cr[6].eq {
	pc = 0x82507D74; continue 'dispatch;
	}
	// 82507D70: 4BDB8B21  bl 0x822c0890
	ctx.lr = 0x82507D74;
	sub_822C0890(ctx, base);
	// 82507D74: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82507D78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507D7C: 419A0008  beq cr6, 0x82507d84
	if ctx.cr[6].eq {
	pc = 0x82507D84; continue 'dispatch;
	}
	// 82507D80: 4BDB8B11  bl 0x822c0890
	ctx.lr = 0x82507D84;
	sub_822C0890(ctx, base);
	// 82507D84: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82507D88: 419A000C  beq cr6, 0x82507d94
	if ctx.cr[6].eq {
	pc = 0x82507D94; continue 'dispatch;
	}
	// 82507D8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82507D90: 4BDB8B01  bl 0x822c0890
	ctx.lr = 0x82507D94;
	sub_822C0890(ctx, base);
	// 82507D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507D98: 480D1219  bl 0x825d8fb0
	ctx.lr = 0x82507D9C;
	sub_825D8FB0(ctx, base);
	// 82507D9C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82507DA0: 48CA040C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82507DA4: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82507DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82507DAC: 419AFFE8  beq cr6, 0x82507d94
	if ctx.cr[6].eq {
	pc = 0x82507D94; continue 'dispatch;
	}
	// 82507DB0: 4BFFFFE0  b 0x82507d90
	pc = 0x82507D90; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507DB8 size=120
    let mut pc: u32 = 0x82507DB8;
    'dispatch: loop {
        match pc {
            0x82507DB8 => {
    //   block [0x82507DB8..0x82507E30)
	// 82507DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82507DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82507DC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82507DC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82507DC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507DCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507DD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82507DD4: 48955AFD  bl 0x82e5d8d0
	ctx.lr = 0x82507DD8;
	sub_82E5D8D0(ctx, base);
	// 82507DD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82507DDC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82507DE0: 396B16A4  addi r11, r11, 0x16a4
	ctx.r[11].s64 = ctx.r[11].s64 + 5796;
	// 82507DE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82507DE8: 48951109  bl 0x82e58ef0
	ctx.lr = 0x82507DEC;
	sub_82E58EF0(ctx, base);
	// 82507DEC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82507DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82507DF4: 396B16C8  addi r11, r11, 0x16c8
	ctx.r[11].s64 = ctx.r[11].s64 + 5832;
	// 82507DF8: 394A16B4  addi r10, r10, 0x16b4
	ctx.r[10].s64 = ctx.r[10].s64 + 5812;
	// 82507DFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82507E00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82507E04: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82507E08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507E0C: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 82507E10: 48B4B501  bl 0x83053310
	ctx.lr = 0x82507E14;
	sub_83053310(ctx, base);
	// 82507E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507E18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82507E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82507E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82507E24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82507E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82507E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82507E30 size=12
    let mut pc: u32 = 0x82507E30;
    'dispatch: loop {
        match pc {
            0x82507E30 => {
    //   block [0x82507E30..0x82507E3C)
	// 82507E30: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82507E34: 3863FFA0  addi r3, r3, -0x60
	ctx.r[3].s64 = ctx.r[3].s64 + -96;
	// 82507E38: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507E3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82507E3C size=8
    let mut pc: u32 = 0x82507E3C;
    'dispatch: loop {
        match pc {
            0x82507E3C => {
    //   block [0x82507E3C..0x82507E44)
	// 82507E3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82507E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82507E48 size=8
    let mut pc: u32 = 0x82507E48;
    'dispatch: loop {
        match pc {
            0x82507E48 => {
    //   block [0x82507E48..0x82507E50)
	// 82507E48: 3863FFA0  addi r3, r3, -0x60
	ctx.r[3].s64 = ctx.r[3].s64 + -96;
	// 82507E4C: 4800041C  b 0x82508268
	sub_82508268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507E50 size=56
    let mut pc: u32 = 0x82507E50;
    'dispatch: loop {
        match pc {
            0x82507E50 => {
    //   block [0x82507E50..0x82507E88)
	// 82507E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82507E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82507E58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82507E5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507E60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82507E64: 808400F4  lwz r4, 0xf4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(244 as u32) ) } as u64;
	// 82507E68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507E6C: 488E9D8D  bl 0x82df1bf8
	ctx.lr = 0x82507E70;
	sub_82DF1BF8(ctx, base);
	// 82507E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82507E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82507E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82507E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82507E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507E88 size=140
    let mut pc: u32 = 0x82507E88;
    'dispatch: loop {
        match pc {
            0x82507E88 => {
    //   block [0x82507E88..0x82507F14)
	// 82507E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82507E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82507E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82507E94: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82507E98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507E9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507EA0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82507EA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507EA8: 48952679  bl 0x82e5a520
	ctx.lr = 0x82507EAC;
	sub_82E5A520(ctx, base);
	// 82507EAC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82507EB0: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 82507EB4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507EB8: 38CBBDFC  addi r6, r11, -0x4204
	ctx.r[6].s64 = ctx.r[11].s64 + -16900;
	// 82507EBC: 38AA4E38  addi r5, r10, 0x4e38
	ctx.r[5].s64 = ctx.r[10].s64 + 20024;
	// 82507EC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82507EC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82507EC8: 48CA2081  bl 0x831a9f48
	ctx.lr = 0x82507ECC;
	sub_831A9F48(ctx, base);
	// 82507ECC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82507ED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507ED4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507ED8: 419A000C  beq cr6, 0x82507ee4
	if ctx.cr[6].eq {
	pc = 0x82507EE4; continue 'dispatch;
	}
	// 82507EDC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82507EE0: 4BDB89B1  bl 0x822c0890
	ctx.lr = 0x82507EE4;
	sub_822C0890(ctx, base);
	// 82507EE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507EE8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82507EEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507EF0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82507EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82507EF8: 4E800421  bctrl
	ctx.lr = 0x82507EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82507EFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82507F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82507F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82507F08: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82507F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82507F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507F18 size=140
    let mut pc: u32 = 0x82507F18;
    'dispatch: loop {
        match pc {
            0x82507F18 => {
    //   block [0x82507F18..0x82507FA4)
	// 82507F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82507F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82507F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82507F24: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82507F28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507F2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507F30: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82507F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507F38: 489525E9  bl 0x82e5a520
	ctx.lr = 0x82507F3C;
	sub_82E5A520(ctx, base);
	// 82507F3C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82507F40: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 82507F44: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507F48: 38CBBDFC  addi r6, r11, -0x4204
	ctx.r[6].s64 = ctx.r[11].s64 + -16900;
	// 82507F4C: 38AA4E38  addi r5, r10, 0x4e38
	ctx.r[5].s64 = ctx.r[10].s64 + 20024;
	// 82507F50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82507F54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82507F58: 48CA1FF1  bl 0x831a9f48
	ctx.lr = 0x82507F5C;
	sub_831A9F48(ctx, base);
	// 82507F5C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82507F60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507F64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507F68: 419A000C  beq cr6, 0x82507f74
	if ctx.cr[6].eq {
	pc = 0x82507F74; continue 'dispatch;
	}
	// 82507F6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82507F70: 4BDB8921  bl 0x822c0890
	ctx.lr = 0x82507F74;
	sub_822C0890(ctx, base);
	// 82507F74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507F78: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82507F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82507F80: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82507F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82507F88: 4E800421  bctrl
	ctx.lr = 0x82507F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82507F8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82507F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82507F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82507F98: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82507F9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82507FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507FA8 size=148
    let mut pc: u32 = 0x82507FA8;
    'dispatch: loop {
        match pc {
            0x82507FA8 => {
    //   block [0x82507FA8..0x8250803C)
	// 82507FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82507FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82507FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82507FB4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82507FB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507FBC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82507FC0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82507FC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82507FC8: 48952559  bl 0x82e5a520
	ctx.lr = 0x82507FCC;
	sub_82E5A520(ctx, base);
	// 82507FCC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82507FD0: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 82507FD4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507FD8: 38CBBDFC  addi r6, r11, -0x4204
	ctx.r[6].s64 = ctx.r[11].s64 + -16900;
	// 82507FDC: 38AA4E38  addi r5, r10, 0x4e38
	ctx.r[5].s64 = ctx.r[10].s64 + 20024;
	// 82507FE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82507FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82507FE8: 48CA1F61  bl 0x831a9f48
	ctx.lr = 0x82507FEC;
	sub_831A9F48(ctx, base);
	// 82507FEC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82507FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507FF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507FF8: 419A000C  beq cr6, 0x82508004
	if ctx.cr[6].eq {
	pc = 0x82508004; continue 'dispatch;
	}
	// 82507FFC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82508000: 4BDB8891  bl 0x822c0890
	ctx.lr = 0x82508004;
	sub_822C0890(ctx, base);
	// 82508004: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82508008: 419A001C  beq cr6, 0x82508024
	if ctx.cr[6].eq {
	pc = 0x82508024; continue 'dispatch;
	}
	// 8250800C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508010: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82508014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508018: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8250801C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82508020: 4E800421  bctrl
	ctx.lr = 0x82508024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82508024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250802C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508030: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508034: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508040 size=80
    let mut pc: u32 = 0x82508040;
    'dispatch: loop {
        match pc {
            0x82508040 => {
    //   block [0x82508040..0x82508090)
	// 82508040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250804C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508050: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82508054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508058: 489524C9  bl 0x82e5a520
	ctx.lr = 0x8250805C;
	sub_82E5A520(ctx, base);
	// 8250805C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82508060: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508064: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508068: 419A000C  beq cr6, 0x82508074
	if ctx.cr[6].eq {
	pc = 0x82508074; continue 'dispatch;
	}
	// 8250806C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82508070: 4BDB8821  bl 0x822c0890
	ctx.lr = 0x82508074;
	sub_822C0890(ctx, base);
	// 82508074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508078: 480221B9  bl 0x8252a230
	ctx.lr = 0x8250807C;
	sub_8252A230(ctx, base);
	// 8250807C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250808C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508090 size=96
    let mut pc: u32 = 0x82508090;
    'dispatch: loop {
        match pc {
            0x82508090 => {
    //   block [0x82508090..0x825080F0)
	// 82508090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250809C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825080A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825080A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825080A8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825080AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825080B0: 48952471  bl 0x82e5a520
	ctx.lr = 0x825080B4;
	sub_82E5A520(ctx, base);
	// 825080B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825080B8: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825080BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825080C0: 419A000C  beq cr6, 0x825080cc
	if ctx.cr[6].eq {
	pc = 0x825080CC; continue 'dispatch;
	}
	// 825080C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825080C8: 4BDB87C9  bl 0x822c0890
	ctx.lr = 0x825080CC;
	sub_822C0890(ctx, base);
	// 825080CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825080D0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825080D4: 48022165  bl 0x8252a238
	ctx.lr = 0x825080D8;
	sub_8252A238(ctx, base);
	// 825080D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825080DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825080E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825080E4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825080E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825080EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825080F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825080F0 size=196
    let mut pc: u32 = 0x825080F0;
    'dispatch: loop {
        match pc {
            0x825080F0 => {
    //   block [0x825080F0..0x825081B4)
	// 825080F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825080F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825080F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825080FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508104: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82508108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250810C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82508110: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82508114: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82508118: 4BDB8821  bl 0x822c0938
	ctx.lr = 0x8250811C;
	sub_822C0938(ctx, base);
	// 8250811C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82508120: 41820028  beq 0x82508148
	if ctx.cr[0].eq {
	pc = 0x82508148; continue 'dispatch;
	}
	// 82508124: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82508128: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250812C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82508130: 392B16D8  addi r9, r11, 0x16d8
	ctx.r[9].s64 = ctx.r[11].s64 + 5848;
	// 82508134: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82508138: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250813C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82508140: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82508144: 48000008  b 0x8250814c
	pc = 0x8250814C; continue 'dispatch;
	// 82508148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250814C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82508150: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508154: 409A0044  bne cr6, 0x82508198
	if !ctx.cr[6].eq {
	pc = 0x82508198; continue 'dispatch;
	}
	// 82508158: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250815C: 419A001C  beq cr6, 0x82508178
	if ctx.cr[6].eq {
	pc = 0x82508178; continue 'dispatch;
	}
	// 82508160: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508164: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82508168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250816C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82508170: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82508174: 4E800421  bctrl
	ctx.lr = 0x82508178;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82508178: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250817C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82508180: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508184: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82508188: 816BBD58  lwz r11, -0x42a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17064 as u32) ) } as u64;
	// 8250818C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82508190: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82508194: 4BDB7E6D  bl 0x822c0000
	ctx.lr = 0x82508198;
	sub_822C0000(ctx, base);
	// 82508198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250819C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825081A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825081A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825081A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825081AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825081B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825081B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825081B8 size=176
    let mut pc: u32 = 0x825081B8;
    'dispatch: loop {
        match pc {
            0x825081B8 => {
    //   block [0x825081B8..0x82508268)
	// 825081B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825081BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825081C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825081C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825081C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825081CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825081D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825081D4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 825081D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825081DC: 394A16C8  addi r10, r10, 0x16c8
	ctx.r[10].s64 = ctx.r[10].s64 + 5832;
	// 825081E0: 392916B4  addi r9, r9, 0x16b4
	ctx.r[9].s64 = ctx.r[9].s64 + 5812;
	// 825081E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825081E8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825081EC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825081F0: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 825081F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825081F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825081FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82508200: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82508204: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508208: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250820C: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82508210: 3BDF0060  addi r30, r31, 0x60
	ctx.r[30].s64 = ctx.r[31].s64 + 96;
	// 82508214: 48956495  bl 0x82e5e6a8
	ctx.lr = 0x82508218;
	sub_82E5E6A8(ctx, base);
	// 82508218: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250821C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82508220: 419A0008  beq cr6, 0x82508228
	if ctx.cr[6].eq {
	pc = 0x82508228; continue 'dispatch;
	}
	// 82508224: 4BDB866D  bl 0x822c0890
	ctx.lr = 0x82508228;
	sub_822C0890(ctx, base);
	// 82508228: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250822C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82508230: 419A0008  beq cr6, 0x82508238
	if ctx.cr[6].eq {
	pc = 0x82508238; continue 'dispatch;
	}
	// 82508234: 4BDB865D  bl 0x822c0890
	ctx.lr = 0x82508238;
	sub_822C0890(ctx, base);
	// 82508238: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8250823C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82508240: 997F0090  stb r11, 0x90(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u8 ) };
	// 82508244: 48950B25  bl 0x82e58d68
	ctx.lr = 0x82508248;
	sub_82E58D68(ctx, base);
	// 82508248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250824C: 489555FD  bl 0x82e5d848
	ctx.lr = 0x82508250;
	sub_82E5D848(ctx, base);
	// 82508250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82508254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250825C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508268 size=76
    let mut pc: u32 = 0x82508268;
    'dispatch: loop {
        match pc {
            0x82508268 => {
    //   block [0x82508268..0x825082B4)
	// 82508268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250826C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82508274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250827C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508280: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82508284: 4BFFFF35  bl 0x825081b8
	ctx.lr = 0x82508288;
	sub_825081B8(ctx, base);
	// 82508288: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250828C: 4182000C  beq 0x82508298
	if ctx.cr[0].eq {
	pc = 0x82508298; continue 'dispatch;
	}
	// 82508290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508294: 488EA145  bl 0x82df23d8
	ctx.lr = 0x82508298;
	sub_82DF23D8(ctx, base);
	// 82508298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250829C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825082A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825082A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825082A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825082AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825082B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825082B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825082B8 size=264
    let mut pc: u32 = 0x825082B8;
    'dispatch: loop {
        match pc {
            0x825082B8 => {
    //   block [0x825082B8..0x825083C0)
	// 825082B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825082BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825082C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825082C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825082C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825082CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825082D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825082D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825082D8: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 825082DC: 38A0003F  li r5, 0x3f
	ctx.r[5].s64 = 63;
	// 825082E0: 38600230  li r3, 0x230
	ctx.r[3].s64 = 560;
	// 825082E4: 488EA105  bl 0x82df23e8
	ctx.lr = 0x825082E8;
	sub_82DF23E8(ctx, base);
	// 825082E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825082EC: 41820010  beq 0x825082fc
	if ctx.cr[0].eq {
	pc = 0x825082FC; continue 'dispatch;
	}
	// 825082F0: 48022829  bl 0x8252ab18
	ctx.lr = 0x825082F4;
	sub_8252AB18(ctx, base);
	// 825082F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825082F8: 48000008  b 0x82508300
	pc = 0x82508300; continue 'dispatch;
	// 825082FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82508300: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82508304: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508308: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250830C: 4BFFFDE5  bl 0x825080f0
	ctx.lr = 0x82508310;
	sub_825080F0(ctx, base);
	// 82508310: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82508314: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508318: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250831C: 4BDB7CE5  bl 0x822c0000
	ctx.lr = 0x82508320;
	sub_822C0000(ctx, base);
	// 82508320: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82508324: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82508328: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250832C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82508330: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82508334: 419A0024  beq cr6, 0x82508358
	if ctx.cr[6].eq {
	pc = 0x82508358; continue 'dispatch;
	}
	// 82508338: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8250833C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82508340: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82508344: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82508348: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250834C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82508350: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82508354: 4082FFE8  bne 0x8250833c
	if !ctx.cr[0].eq {
	pc = 0x8250833C; continue 'dispatch;
	}
	// 82508358: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8250835C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82508360: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82508364: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82508368: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250836C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82508370: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82508374: 48956335  bl 0x82e5e6a8
	ctx.lr = 0x82508378;
	sub_82E5E6A8(ctx, base);
	// 82508378: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8250837C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82508380: 419A0008  beq cr6, 0x82508388
	if ctx.cr[6].eq {
	pc = 0x82508388; continue 'dispatch;
	}
	// 82508384: 4BDB850D  bl 0x822c0890
	ctx.lr = 0x82508388;
	sub_822C0890(ctx, base);
	// 82508388: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250838C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82508390: 419A0008  beq cr6, 0x82508398
	if ctx.cr[6].eq {
	pc = 0x82508398; continue 'dispatch;
	}
	// 82508394: 4BDB84FD  bl 0x822c0890
	ctx.lr = 0x82508398;
	sub_822C0890(ctx, base);
	// 82508398: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250839C: 419A000C  beq cr6, 0x825083a8
	if ctx.cr[6].eq {
	pc = 0x825083A8; continue 'dispatch;
	}
	// 825083A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825083A4: 4BDB84ED  bl 0x822c0890
	ctx.lr = 0x825083A8;
	sub_822C0890(ctx, base);
	// 825083A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825083AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825083B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825083B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825083B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825083BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825083C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825083C0 size=136
    let mut pc: u32 = 0x825083C0;
    'dispatch: loop {
        match pc {
            0x825083C0 => {
    //   block [0x825083C0..0x82508448)
	// 825083C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825083C4: 48C9FDA9  bl 0x831a816c
	ctx.lr = 0x825083C8;
	sub_831A8130(ctx, base);
	// 825083C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825083CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825083D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825083D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825083D8: 806B1108  lwz r3, 0x1108(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4360 as u32) ) } as u64;
	// 825083DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825083E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825083E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825083E8: 4E800421  bctrl
	ctx.lr = 0x825083EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825083EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825083F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825083F4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825083F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825083FC: 4BDB86FD  bl 0x822c0af8
	ctx.lr = 0x82508400;
	sub_822C0AF8(ctx, base);
	// 82508400: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82508404: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82508408: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250840C: 4BDB7BF5  bl 0x822c0000
	ctx.lr = 0x82508410;
	sub_822C0000(ctx, base);
	// 82508410: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82508414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508418: 48954A61  bl 0x82e5ce78
	ctx.lr = 0x8250841C;
	sub_82E5CE78(ctx, base);
	// 8250841C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508420: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82508424: 48954C6D  bl 0x82e5d090
	ctx.lr = 0x82508428;
	sub_82E5D090(ctx, base);
	// 82508428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250842C: 4BFFFC15  bl 0x82508040
	ctx.lr = 0x82508430;
	sub_82508040(ctx, base);
	// 82508430: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82508434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82508438: 419A0008  beq cr6, 0x82508440
	if ctx.cr[6].eq {
	pc = 0x82508440; continue 'dispatch;
	}
	// 8250843C: 4BDB8455  bl 0x822c0890
	ctx.lr = 0x82508440;
	sub_822C0890(ctx, base);
	// 82508440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82508444: 48C9FD78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508448 size=88
    let mut pc: u32 = 0x82508448;
    'dispatch: loop {
        match pc {
            0x82508448 => {
    //   block [0x82508448..0x825084A0)
	// 82508448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82508454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250845C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508460: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82508464: 48008C8D  bl 0x825110f0
	ctx.lr = 0x82508468;
	sub_825110F0(ctx, base);
	// 82508468: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250846C: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82508470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82508474: 396B1734  addi r11, r11, 0x1734
	ctx.r[11].s64 = ctx.r[11].s64 + 5940;
	// 82508478: 394A1720  addi r10, r10, 0x1720
	ctx.r[10].s64 = ctx.r[10].s64 + 5920;
	// 8250847C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82508480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508484: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82508488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250848C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250849C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825084A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825084A0 size=12
    let mut pc: u32 = 0x825084A0;
    'dispatch: loop {
        match pc {
            0x825084A0 => {
    //   block [0x825084A0..0x825084AC)
	// 825084A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825084A4: 908B0054  stw r4, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 825084A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825084B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825084B0 size=12
    let mut pc: u32 = 0x825084B0;
    'dispatch: loop {
        match pc {
            0x825084B0 => {
    //   block [0x825084B0..0x825084BC)
	// 825084B0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825084B4: 806B0054  lwz r3, 0x54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825084B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825084C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825084C0 size=12
    let mut pc: u32 = 0x825084C0;
    'dispatch: loop {
        match pc {
            0x825084C0 => {
    //   block [0x825084C0..0x825084CC)
	// 825084C0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825084C4: 908B0058  stw r4, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 825084C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825084D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825084D0 size=12
    let mut pc: u32 = 0x825084D0;
    'dispatch: loop {
        match pc {
            0x825084D0 => {
    //   block [0x825084D0..0x825084DC)
	// 825084D0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825084D4: 806B0058  lwz r3, 0x58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825084D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825084E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825084E0 size=12
    let mut pc: u32 = 0x825084E0;
    'dispatch: loop {
        match pc {
            0x825084E0 => {
    //   block [0x825084E0..0x825084EC)
	// 825084E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825084E4: D02B005C  stfs f1, 0x5c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 825084E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825084F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825084F0 size=20
    let mut pc: u32 = 0x825084F0;
    'dispatch: loop {
        match pc {
            0x825084F0 => {
    //   block [0x825084F0..0x82508504)
	// 825084F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825084F4: C00B0060  lfs f0, 0x60(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825084F8: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 825084FC: D00B0060  stfs f0, 0x60(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82508500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82508508 size=12
    let mut pc: u32 = 0x82508508;
    'dispatch: loop {
        match pc {
            0x82508508 => {
    //   block [0x82508508..0x82508514)
	// 82508508: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250850C: D02B0060  stfs f1, 0x60(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82508510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82508518 size=12
    let mut pc: u32 = 0x82508518;
    'dispatch: loop {
        match pc {
            0x82508518 => {
    //   block [0x82508518..0x82508524)
	// 82508518: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250851C: C02B0060  lfs f1, 0x60(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82508520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508528 size=12
    let mut pc: u32 = 0x82508528;
    'dispatch: loop {
        match pc {
            0x82508528 => {
    //   block [0x82508528..0x82508534)
	// 82508528: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250852C: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82508530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508538 size=12
    let mut pc: u32 = 0x82508538;
    'dispatch: loop {
        match pc {
            0x82508538 => {
    //   block [0x82508538..0x82508544)
	// 82508538: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250853C: 386B009C  addi r3, r11, 0x9c
	ctx.r[3].s64 = ctx.r[11].s64 + 156;
	// 82508540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508548 size=12
    let mut pc: u32 = 0x82508548;
    'dispatch: loop {
        match pc {
            0x82508548 => {
    //   block [0x82508548..0x82508554)
	// 82508548: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250854C: 386B00AC  addi r3, r11, 0xac
	ctx.r[3].s64 = ctx.r[11].s64 + 172;
	// 82508550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508558 size=12
    let mut pc: u32 = 0x82508558;
    'dispatch: loop {
        match pc {
            0x82508558 => {
    //   block [0x82508558..0x82508564)
	// 82508558: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250855C: 386B00AC  addi r3, r11, 0xac
	ctx.r[3].s64 = ctx.r[11].s64 + 172;
	// 82508560: 488EB670  b 0x82df3bd0
	sub_82DF3BD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508568 size=12
    let mut pc: u32 = 0x82508568;
    'dispatch: loop {
        match pc {
            0x82508568 => {
    //   block [0x82508568..0x82508574)
	// 82508568: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250856C: 806B0090  lwz r3, 0x90(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82508570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508578 size=12
    let mut pc: u32 = 0x82508578;
    'dispatch: loop {
        match pc {
            0x82508578 => {
    //   block [0x82508578..0x82508584)
	// 82508578: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250857C: 908B00D0  stw r4, 0xd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[4].u32 ) };
	// 82508580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508588 size=12
    let mut pc: u32 = 0x82508588;
    'dispatch: loop {
        match pc {
            0x82508588 => {
    //   block [0x82508588..0x82508594)
	// 82508588: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250858C: 806B00D0  lwz r3, 0xd0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82508590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508598 size=12
    let mut pc: u32 = 0x82508598;
    'dispatch: loop {
        match pc {
            0x82508598 => {
    //   block [0x82508598..0x825085A4)
	// 82508598: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250859C: 806B00D4  lwz r3, 0xd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 825085A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825085A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825085A8 size=12
    let mut pc: u32 = 0x825085A8;
    'dispatch: loop {
        match pc {
            0x825085A8 => {
    //   block [0x825085A8..0x825085B4)
	// 825085A8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825085AC: 908B00DC  stw r4, 0xdc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), ctx.r[4].u32 ) };
	// 825085B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825085B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825085B8 size=12
    let mut pc: u32 = 0x825085B8;
    'dispatch: loop {
        match pc {
            0x825085B8 => {
    //   block [0x825085B8..0x825085C4)
	// 825085B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825085BC: 806B00DC  lwz r3, 0xdc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(220 as u32) ) } as u64;
	// 825085C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825085C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825085C8 size=12
    let mut pc: u32 = 0x825085C8;
    'dispatch: loop {
        match pc {
            0x825085C8 => {
    //   block [0x825085C8..0x825085D4)
	// 825085C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825085CC: 908B00E0  stw r4, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[4].u32 ) };
	// 825085D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825085D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825085D8 size=12
    let mut pc: u32 = 0x825085D8;
    'dispatch: loop {
        match pc {
            0x825085D8 => {
    //   block [0x825085D8..0x825085E4)
	// 825085D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825085DC: 806B00E0  lwz r3, 0xe0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(224 as u32) ) } as u64;
	// 825085E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825085E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825085E8 size=12
    let mut pc: u32 = 0x825085E8;
    'dispatch: loop {
        match pc {
            0x825085E8 => {
    //   block [0x825085E8..0x825085F4)
	// 825085E8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825085EC: 806B00E4  lwz r3, 0xe4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 825085F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825085F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825085F8 size=12
    let mut pc: u32 = 0x825085F8;
    'dispatch: loop {
        match pc {
            0x825085F8 => {
    //   block [0x825085F8..0x82508604)
	// 825085F8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825085FC: 806B00EC  lwz r3, 0xec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(236 as u32) ) } as u64;
	// 82508600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508608 size=12
    let mut pc: u32 = 0x82508608;
    'dispatch: loop {
        match pc {
            0x82508608 => {
    //   block [0x82508608..0x82508614)
	// 82508608: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250860C: 908B0104  stw r4, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[4].u32 ) };
	// 82508610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508618 size=12
    let mut pc: u32 = 0x82508618;
    'dispatch: loop {
        match pc {
            0x82508618 => {
    //   block [0x82508618..0x82508624)
	// 82508618: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250861C: 806B0104  lwz r3, 0x104(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82508620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508628 size=12
    let mut pc: u32 = 0x82508628;
    'dispatch: loop {
        match pc {
            0x82508628 => {
    //   block [0x82508628..0x82508634)
	// 82508628: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250862C: 908B0108  stw r4, 0x108(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[4].u32 ) };
	// 82508630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508638 size=12
    let mut pc: u32 = 0x82508638;
    'dispatch: loop {
        match pc {
            0x82508638 => {
    //   block [0x82508638..0x82508644)
	// 82508638: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250863C: 806B0108  lwz r3, 0x108(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(264 as u32) ) } as u64;
	// 82508640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508648 size=12
    let mut pc: u32 = 0x82508648;
    'dispatch: loop {
        match pc {
            0x82508648 => {
    //   block [0x82508648..0x82508654)
	// 82508648: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250864C: 806B00F8  lwz r3, 0xf8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 82508650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508658 size=8
    let mut pc: u32 = 0x82508658;
    'dispatch: loop {
        match pc {
            0x82508658 => {
    //   block [0x82508658..0x82508660)
	// 82508658: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8250865C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508660 size=8
    let mut pc: u32 = 0x82508660;
    'dispatch: loop {
        match pc {
            0x82508660 => {
    //   block [0x82508660..0x82508668)
	// 82508660: 2F040012  cmpwi cr6, r4, 0x12
	ctx.cr[6].compare_i32(ctx.r[4].s32, 18, &mut ctx.xer);
	// 82508664: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508668 size=20
    let mut pc: u32 = 0x82508668;
    'dispatch: loop {
        match pc {
            0x82508668 => {
    //   block [0x82508668..0x8250867C)
	// 82508668: 39640049  addi r11, r4, 0x49
	ctx.r[11].s64 = ctx.r[4].s64 + 73;
	// 8250866C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508670: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82508674: 7CAB512E  stwx r5, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 82508678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508680 size=36
    let mut pc: u32 = 0x82508680;
    'dispatch: loop {
        match pc {
            0x82508680 => {
    //   block [0x82508680..0x825086A4)
	// 82508680: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82508684: 40990020  ble cr6, 0x825086a4
	if !ctx.cr[6].gt {
		sub_825086A4(ctx, base);
		return;
	}
	// 82508688: 2F040012  cmpwi cr6, r4, 0x12
	ctx.cr[6].compare_i32(ctx.r[4].s32, 18, &mut ctx.xer);
	// 8250868C: 40980018  bge cr6, 0x825086a4
	if !ctx.cr[6].lt {
		sub_825086A4(ctx, base);
		return;
	}
	// 82508690: 39640049  addi r11, r4, 0x49
	ctx.r[11].s64 = ctx.r[4].s64 + 73;
	// 82508694: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508698: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250869C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825086A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825086A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825086A4 size=8
    let mut pc: u32 = 0x825086A4;
    'dispatch: loop {
        match pc {
            0x825086A4 => {
    //   block [0x825086A4..0x825086AC)
	// 825086A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825086A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825086B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825086B0 size=24
    let mut pc: u32 = 0x825086B0;
    'dispatch: loop {
        match pc {
            0x825086B0 => {
    //   block [0x825086B0..0x825086C8)
	// 825086B0: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825086B4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825086B8: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 825086BC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825086C0: 914B01D4  stw r10, 0x1d4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(468 as u32), ctx.r[10].u32 ) };
	// 825086C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825086C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825086C8 size=12
    let mut pc: u32 = 0x825086C8;
    'dispatch: loop {
        match pc {
            0x825086C8 => {
    //   block [0x825086C8..0x825086D4)
	// 825086C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825086CC: 386B01D0  addi r3, r11, 0x1d0
	ctx.r[3].s64 = ctx.r[11].s64 + 464;
	// 825086D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825086D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825086D8 size=12
    let mut pc: u32 = 0x825086D8;
    'dispatch: loop {
        match pc {
            0x825086D8 => {
    //   block [0x825086D8..0x825086E4)
	// 825086D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825086DC: 386B01D8  addi r3, r11, 0x1d8
	ctx.r[3].s64 = ctx.r[11].s64 + 472;
	// 825086E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825086E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825086E8 size=12
    let mut pc: u32 = 0x825086E8;
    'dispatch: loop {
        match pc {
            0x825086E8 => {
    //   block [0x825086E8..0x825086F4)
	// 825086E8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825086EC: 908B0220  stw r4, 0x220(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(544 as u32), ctx.r[4].u32 ) };
	// 825086F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825086F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825086F8 size=12
    let mut pc: u32 = 0x825086F8;
    'dispatch: loop {
        match pc {
            0x825086F8 => {
    //   block [0x825086F8..0x82508704)
	// 825086F8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825086FC: 806B0220  lwz r3, 0x220(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(544 as u32) ) } as u64;
	// 82508700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508708 size=12
    let mut pc: u32 = 0x82508708;
    'dispatch: loop {
        match pc {
            0x82508708 => {
    //   block [0x82508708..0x82508714)
	// 82508708: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250870C: 988B0228  stb r4, 0x228(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(552 as u32), ctx.r[4].u8 ) };
	// 82508710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508718 size=12
    let mut pc: u32 = 0x82508718;
    'dispatch: loop {
        match pc {
            0x82508718 => {
    //   block [0x82508718..0x82508724)
	// 82508718: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250871C: 886B0228  lbz r3, 0x228(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(552 as u32) ) } as u64;
	// 82508720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82508728 size=88
    let mut pc: u32 = 0x82508728;
    'dispatch: loop {
        match pc {
            0x82508728 => {
    //   block [0x82508728..0x82508780)
	// 82508728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82508734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250873C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508740: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82508744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508748: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250874C: 48006BE5  bl 0x8250f330
	ctx.lr = 0x82508750;
	sub_8250F330(ctx, base);
	// 82508750: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82508754: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82508758: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250875C: 4BFE11FD  bl 0x824e9958
	ctx.lr = 0x82508760;
	sub_824E9958(ctx, base);
	// 82508760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508764: 488E952D  bl 0x82df1c90
	ctx.lr = 0x82508768;
	sub_82DF1C90(ctx, base);
	// 82508768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250876C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508774: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250877C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508780 size=12
    let mut pc: u32 = 0x82508780;
    'dispatch: loop {
        match pc {
            0x82508780 => {
    //   block [0x82508780..0x8250878C)
	// 82508780: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508784: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82508788: 4895B100  b 0x82e63888
	sub_82E63888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508790 size=12
    let mut pc: u32 = 0x82508790;
    'dispatch: loop {
        match pc {
            0x82508790 => {
    //   block [0x82508790..0x8250879C)
	// 82508790: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508794: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82508798: 4895AF90  b 0x82e63728
	sub_82E63728(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825087A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825087A0 size=76
    let mut pc: u32 = 0x825087A0;
    'dispatch: loop {
        match pc {
            0x825087A0 => {
    //   block [0x825087A0..0x825087EC)
	// 825087A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825087A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825087A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825087AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825087B0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825087B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825087B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825087BC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825087C0: 48021A31  bl 0x8252a1f0
	ctx.lr = 0x825087C4;
	sub_8252A1F0(ctx, base);
	// 825087C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825087C8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825087CC: 4BFE0E25  bl 0x824e95f0
	ctx.lr = 0x825087D0;
	sub_824E95F0(ctx, base);
	// 825087D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825087D4: 488E94BD  bl 0x82df1c90
	ctx.lr = 0x825087D8;
	sub_82DF1C90(ctx, base);
	// 825087D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825087DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825087E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825087E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825087E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825087F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825087F0 size=92
    let mut pc: u32 = 0x825087F0;
    'dispatch: loop {
        match pc {
            0x825087F0 => {
    //   block [0x825087F0..0x8250884C)
	// 825087F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825087F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825087F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825087FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508804: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250880C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82508810: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82508814: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508818: 480219D9  bl 0x8252a1f0
	ctx.lr = 0x8250881C;
	sub_8252A1F0(ctx, base);
	// 8250881C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82508820: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508824: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508828: 4BFE0DD9  bl 0x824e9600
	ctx.lr = 0x8250882C;
	sub_824E9600(ctx, base);
	// 8250882C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508830: 488E9461  bl 0x82df1c90
	ctx.lr = 0x82508834;
	sub_82DF1C90(ctx, base);
	// 82508834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250883C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508850 size=108
    let mut pc: u32 = 0x82508850;
    'dispatch: loop {
        match pc {
            0x82508850 => {
    //   block [0x82508850..0x825088BC)
	// 82508850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250885C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508864: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508868: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250886C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82508870: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82508874: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508878: 48021979  bl 0x8252a1f0
	ctx.lr = 0x8250887C;
	sub_8252A1F0(ctx, base);
	// 8250887C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82508880: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508884: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508888: 4BFE0E49  bl 0x824e96d0
	ctx.lr = 0x8250888C;
	sub_824E96D0(ctx, base);
	// 8250888C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508890: 488E9401  bl 0x82df1c90
	ctx.lr = 0x82508894;
	sub_82DF1C90(ctx, base);
	// 82508894: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82508898: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250889C: 419A0008  beq cr6, 0x825088a4
	if ctx.cr[6].eq {
	pc = 0x825088A4; continue 'dispatch;
	}
	// 825088A0: 4BDB7FF1  bl 0x822c0890
	ctx.lr = 0x825088A4;
	sub_822C0890(ctx, base);
	// 825088A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825088A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825088AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825088B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825088B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825088B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825088C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825088C0 size=76
    let mut pc: u32 = 0x825088C0;
    'dispatch: loop {
        match pc {
            0x825088C0 => {
    //   block [0x825088C0..0x8250890C)
	// 825088C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825088C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825088C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825088CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825088D0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825088D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825088D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825088DC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825088E0: 48021911  bl 0x8252a1f0
	ctx.lr = 0x825088E4;
	sub_8252A1F0(ctx, base);
	// 825088E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825088E8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825088EC: 4BFE0E3D  bl 0x824e9728
	ctx.lr = 0x825088F0;
	sub_824E9728(ctx, base);
	// 825088F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825088F4: 488E939D  bl 0x82df1c90
	ctx.lr = 0x825088F8;
	sub_82DF1C90(ctx, base);
	// 825088F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825088FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508910 size=76
    let mut pc: u32 = 0x82508910;
    'dispatch: loop {
        match pc {
            0x82508910 => {
    //   block [0x82508910..0x8250895C)
	// 82508910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250891C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508920: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508924: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508928: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250892C: 480218C5  bl 0x8252a1f0
	ctx.lr = 0x82508930;
	sub_8252A1F0(ctx, base);
	// 82508930: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508934: 4BFE0E3D  bl 0x824e9770
	ctx.lr = 0x82508938;
	sub_824E9770(ctx, base);
	// 82508938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250893C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508940: 488E9351  bl 0x82df1c90
	ctx.lr = 0x82508944;
	sub_82DF1C90(ctx, base);
	// 82508944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508948: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250894C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508960 size=80
    let mut pc: u32 = 0x82508960;
    'dispatch: loop {
        match pc {
            0x82508960 => {
    //   block [0x82508960..0x825089B0)
	// 82508960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250896C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508970: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508978: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8250897C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508980: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 82508984: 409A0008  bne cr6, 0x8250898c
	if !ctx.cr[6].eq {
	pc = 0x8250898C; continue 'dispatch;
	}
	// 82508988: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250898C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82508990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508994: 488E9265  bl 0x82df1bf8
	ctx.lr = 0x82508998;
	sub_82DF1BF8(ctx, base);
	// 82508998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250899C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825089A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825089A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825089A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825089AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825089B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825089B0 size=80
    let mut pc: u32 = 0x825089B0;
    'dispatch: loop {
        match pc {
            0x825089B0 => {
    //   block [0x825089B0..0x82508A00)
	// 825089B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825089B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825089B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825089BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825089C0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825089C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825089C8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825089CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825089D0: 388B00CC  addi r4, r11, 0xcc
	ctx.r[4].s64 = ctx.r[11].s64 + 204;
	// 825089D4: 409A0008  bne cr6, 0x825089dc
	if !ctx.cr[6].eq {
	pc = 0x825089DC; continue 'dispatch;
	}
	// 825089D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825089DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825089E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825089E4: 488E9215  bl 0x82df1bf8
	ctx.lr = 0x825089E8;
	sub_82DF1BF8(ctx, base);
	// 825089E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825089EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825089F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825089F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825089F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825089FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508A00 size=20
    let mut pc: u32 = 0x82508A00;
    'dispatch: loop {
        match pc {
            0x82508A00 => {
    //   block [0x82508A00..0x82508A14)
	// 82508A00: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508A04: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82508A08: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82508A0C: 7CAB512E  stwx r5, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 82508A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508A18 size=52
    let mut pc: u32 = 0x82508A18;
    'dispatch: loop {
        match pc {
            0x82508A18 => {
    //   block [0x82508A18..0x82508A4C)
	// 82508A18: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508A1C: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82508A20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508A24: 419A0028  beq cr6, 0x82508a4c
	if ctx.cr[6].eq {
		sub_82508A4C(ctx, base);
		return;
	}
	// 82508A28: 812B003C  lwz r9, 0x3c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82508A2C: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82508A30: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82508A34: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82508A38: 40980014  bge cr6, 0x82508a4c
	if !ctx.cr[6].lt {
		sub_82508A4C(ctx, base);
		return;
	}
	// 82508A3C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82508A40: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82508A44: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82508A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508A4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508A4C size=8
    let mut pc: u32 = 0x82508A4C;
    'dispatch: loop {
        match pc {
            0x82508A4C => {
    //   block [0x82508A4C..0x82508A54)
	// 82508A4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82508A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508A58 size=24
    let mut pc: u32 = 0x82508A58;
    'dispatch: loop {
        match pc {
            0x82508A58 => {
    //   block [0x82508A58..0x82508A70)
	// 82508A58: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508A5C: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 82508A60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508A64: 409A000C  bne cr6, 0x82508a70
	if !ctx.cr[6].eq {
		sub_82508A70(ctx, base);
		return;
	}
	// 82508A68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82508A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508A70 size=16
    let mut pc: u32 = 0x82508A70;
    'dispatch: loop {
        match pc {
            0x82508A70 => {
    //   block [0x82508A70..0x82508A80)
	// 82508A70: 814A003C  lwz r10, 0x3c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82508A74: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82508A78: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82508A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508A80 size=76
    let mut pc: u32 = 0x82508A80;
    'dispatch: loop {
        match pc {
            0x82508A80 => {
    //   block [0x82508A80..0x82508ACC)
	// 82508A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508A88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508A8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508A90: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508A94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508A98: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508A9C: 48021755  bl 0x8252a1f0
	ctx.lr = 0x82508AA0;
	sub_8252A1F0(ctx, base);
	// 82508AA0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508AA4: 4BFE0DBD  bl 0x824e9860
	ctx.lr = 0x82508AA8;
	sub_824E9860(ctx, base);
	// 82508AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508AAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508AB0: 488E91E1  bl 0x82df1c90
	ctx.lr = 0x82508AB4;
	sub_82DF1C90(ctx, base);
	// 82508AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508AB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508AC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508AD0 size=76
    let mut pc: u32 = 0x82508AD0;
    'dispatch: loop {
        match pc {
            0x82508AD0 => {
    //   block [0x82508AD0..0x82508B1C)
	// 82508AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508AD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508ADC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508AE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508AE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508AE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82508AEC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508AF0: 48021701  bl 0x8252a1f0
	ctx.lr = 0x82508AF4;
	sub_8252A1F0(ctx, base);
	// 82508AF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508AF8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508AFC: 4BFE0E0D  bl 0x824e9908
	ctx.lr = 0x82508B00;
	sub_824E9908(ctx, base);
	// 82508B00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508B04: 488E918D  bl 0x82df1c90
	ctx.lr = 0x82508B08;
	sub_82DF1C90(ctx, base);
	// 82508B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508B20 size=12
    let mut pc: u32 = 0x82508B20;
    'dispatch: loop {
        match pc {
            0x82508B20 => {
    //   block [0x82508B20..0x82508B2C)
	// 82508B20: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508B24: 806B0044  lwz r3, 0x44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82508B28: 480A5B98  b 0x825ae6c0
	sub_825AE6C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508B30 size=12
    let mut pc: u32 = 0x82508B30;
    'dispatch: loop {
        match pc {
            0x82508B30 => {
    //   block [0x82508B30..0x82508B3C)
	// 82508B30: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508B34: 806B004C  lwz r3, 0x4c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82508B38: 480B70D0  b 0x825bfc08
	sub_825BFC08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508B40 size=12
    let mut pc: u32 = 0x82508B40;
    'dispatch: loop {
        match pc {
            0x82508B40 => {
    //   block [0x82508B40..0x82508B4C)
	// 82508B40: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508B44: 806B004C  lwz r3, 0x4c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82508B48: 480B71A8  b 0x825bfcf0
	sub_825BFCF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82508B50 size=56
    let mut pc: u32 = 0x82508B50;
    'dispatch: loop {
        match pc {
            0x82508B50 => {
    //   block [0x82508B50..0x82508B88)
	// 82508B50: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508B54: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 82508B58: C00B005C  lfs f0, 0x5c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82508B5C: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82508B60: D00B005C  stfs f0, 0x5c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82508B64: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508B68: C1AABE18  lfs f13, -0x41e8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16872 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82508B6C: C00B005C  lfs f0, 0x5c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82508B70: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82508B74: 41980008  blt cr6, 0x82508b7c
	if ctx.cr[6].lt {
	pc = 0x82508B7C; continue 'dispatch;
	}
	// 82508B78: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82508B7C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508B80: D00B005C  stfs f0, 0x5c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82508B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82508B88 size=24
    let mut pc: u32 = 0x82508B88;
    'dispatch: loop {
        match pc {
            0x82508B88 => {
    //   block [0x82508B88..0x82508BA0)
	// 82508B88: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508B8C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82508B90: C1AB005C  lfs f13, 0x5c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82508B94: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82508B98: FC2D036E  fsel f1, f13, f13, f0
	ctx.f[1].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82508B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508BA0 size=92
    let mut pc: u32 = 0x82508BA0;
    'dispatch: loop {
        match pc {
            0x82508BA0 => {
    //   block [0x82508BA0..0x82508BFC)
	// 82508BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82508BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508BB4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508BB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508BBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82508BC0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82508BC4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508BC8: 48021629  bl 0x8252a1f0
	ctx.lr = 0x82508BCC;
	sub_8252A1F0(ctx, base);
	// 82508BCC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82508BD0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508BD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508BD8: 4BFE0D51  bl 0x824e9928
	ctx.lr = 0x82508BDC;
	sub_824E9928(ctx, base);
	// 82508BDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508BE0: 488E90B1  bl 0x82df1c90
	ctx.lr = 0x82508BE4;
	sub_82DF1C90(ctx, base);
	// 82508BE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508BF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508BF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508C00 size=92
    let mut pc: u32 = 0x82508C00;
    'dispatch: loop {
        match pc {
            0x82508C00 => {
    //   block [0x82508C00..0x82508C5C)
	// 82508C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508C08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82508C0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508C14: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508C18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508C1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82508C20: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82508C24: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508C28: 480215C9  bl 0x8252a1f0
	ctx.lr = 0x82508C2C;
	sub_8252A1F0(ctx, base);
	// 82508C2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82508C30: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508C34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82508C38: 4BFE0D09  bl 0x824e9940
	ctx.lr = 0x82508C3C;
	sub_824E9940(ctx, base);
	// 82508C3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508C40: 488E9051  bl 0x82df1c90
	ctx.lr = 0x82508C44;
	sub_82DF1C90(ctx, base);
	// 82508C44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508C50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508C54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508C60 size=80
    let mut pc: u32 = 0x82508C60;
    'dispatch: loop {
        match pc {
            0x82508C60 => {
    //   block [0x82508C60..0x82508CB0)
	// 82508C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508C6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508C70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82508C74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508C78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508C7C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82508C80: 488EAD89  bl 0x82df3a08
	ctx.lr = 0x82508C84;
	sub_82DF3A08(ctx, base);
	// 82508C84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508C88: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82508C8C: 806B004C  lwz r3, 0x4c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82508C90: 480B96A1  bl 0x825c2330
	ctx.lr = 0x82508C94;
	sub_825C2330(ctx, base);
	// 82508C94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82508C98: 488EA791  bl 0x82df3428
	ctx.lr = 0x82508C9C;
	sub_82DF3428(ctx, base);
	// 82508C9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508CB0 size=12
    let mut pc: u32 = 0x82508CB0;
    'dispatch: loop {
        match pc {
            0x82508CB0 => {
    //   block [0x82508CB0..0x82508CBC)
	// 82508CB0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508CB4: 806B0044  lwz r3, 0x44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82508CB8: 480A8228  b 0x825b0ee0
	sub_825B0EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508CC0 size=12
    let mut pc: u32 = 0x82508CC0;
    'dispatch: loop {
        match pc {
            0x82508CC0 => {
    //   block [0x82508CC0..0x82508CCC)
	// 82508CC0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508CC4: 806B002C  lwz r3, 0x2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82508CC8: 4BDBE078  b 0x822c6d40
	sub_822C6D40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508CD0 size=20
    let mut pc: u32 = 0x82508CD0;
    'dispatch: loop {
        match pc {
            0x82508CD0 => {
    //   block [0x82508CD0..0x82508CE4)
	// 82508CD0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508CD4: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82508CD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508CDC: 386B00C0  addi r3, r11, 0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + 192;
	// 82508CE0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508CE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508CE4 size=8
    let mut pc: u32 = 0x82508CE4;
    'dispatch: loop {
        match pc {
            0x82508CE4 => {
    //   block [0x82508CE4..0x82508CEC)
	// 82508CE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82508CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508CF0 size=16
    let mut pc: u32 = 0x82508CF0;
    'dispatch: loop {
        match pc {
            0x82508CF0 => {
    //   block [0x82508CF0..0x82508D00)
	// 82508CF0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508CF4: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82508CF8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82508CFC: 48B002BC  b 0x83008fb8
	sub_83008FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508D00 size=12
    let mut pc: u32 = 0x82508D00;
    'dispatch: loop {
        match pc {
            0x82508D00 => {
    //   block [0x82508D00..0x82508D0C)
	// 82508D00: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D04: 806B00B0  lwz r3, 0xb0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82508D08: 48632440  b 0x82b3b148
	sub_82B3B148(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508D10 size=24
    let mut pc: u32 = 0x82508D10;
    'dispatch: loop {
        match pc {
            0x82508D10 => {
    //   block [0x82508D10..0x82508D28)
	// 82508D10: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D14: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508D18: 914B00B8  stw r10, 0xb8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 82508D1C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D20: 908B00BC  stw r4, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[4].u32 ) };
	// 82508D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82508D28 size=44
    let mut pc: u32 = 0x82508D28;
    'dispatch: loop {
        match pc {
            0x82508D28 => {
    //   block [0x82508D28..0x82508D54)
	// 82508D28: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82508D2C: 41820014  beq 0x82508d40
	if ctx.cr[0].eq {
	pc = 0x82508D40; continue 'dispatch;
	}
	// 82508D30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82508D34: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D38: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82508D3C: D00A005C  stfs f0, 0x5c(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82508D40: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82508D48: 814B01C0  lwz r10, 0x1c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508D4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508D50: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508D54 size=8
    let mut pc: u32 = 0x82508D54;
    'dispatch: loop {
        match pc {
            0x82508D54 => {
    //   block [0x82508D54..0x82508D5C)
	// 82508D54: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82508D58: 4BF5B8A8  b 0x82464600
	sub_82464600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508D5C size=4
    let mut pc: u32 = 0x82508D5C;
    'dispatch: loop {
        match pc {
            0x82508D5C => {
    //   block [0x82508D5C..0x82508D60)
	// 82508D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508D60 size=24
    let mut pc: u32 = 0x82508D60;
    'dispatch: loop {
        match pc {
            0x82508D60 => {
    //   block [0x82508D60..0x82508D78)
	// 82508D60: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D64: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508D68: 914B00D8  stw r10, 0xd8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(216 as u32), ctx.r[10].u32 ) };
	// 82508D6C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D70: 908B00D4  stw r4, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[4].u32 ) };
	// 82508D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508D78 size=24
    let mut pc: u32 = 0x82508D78;
    'dispatch: loop {
        match pc {
            0x82508D78 => {
    //   block [0x82508D78..0x82508D90)
	// 82508D78: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D7C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508D80: 914B00E8  stw r10, 0xe8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82508D84: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D88: 908B00E4  stw r4, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[4].u32 ) };
	// 82508D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508D90 size=24
    let mut pc: u32 = 0x82508D90;
    'dispatch: loop {
        match pc {
            0x82508D90 => {
    //   block [0x82508D90..0x82508DA8)
	// 82508D90: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508D94: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508D98: 914B00F0  stw r10, 0xf0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[10].u32 ) };
	// 82508D9C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508DA0: 908B00EC  stw r4, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[4].u32 ) };
	// 82508DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508DA8 size=24
    let mut pc: u32 = 0x82508DA8;
    'dispatch: loop {
        match pc {
            0x82508DA8 => {
    //   block [0x82508DA8..0x82508DC0)
	// 82508DA8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508DAC: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508DB0: 914B00F4  stw r10, 0xf4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 82508DB4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508DB8: 908B00F8  stw r4, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[4].u32 ) };
	// 82508DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508DC0 size=24
    let mut pc: u32 = 0x82508DC0;
    'dispatch: loop {
        match pc {
            0x82508DC0 => {
    //   block [0x82508DC0..0x82508DD8)
	// 82508DC0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508DC4: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508DC8: 914B00FC  stw r10, 0xfc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(252 as u32), ctx.r[10].u32 ) };
	// 82508DCC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508DD0: 908B0100  stw r4, 0x100(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(256 as u32), ctx.r[4].u32 ) };
	// 82508DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508DD8 size=104
    let mut pc: u32 = 0x82508DD8;
    'dispatch: loop {
        match pc {
            0x82508DD8 => {
    //   block [0x82508DD8..0x82508E40)
	// 82508DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508DDC: 48C9F389  bl 0x831a8164
	ctx.lr = 0x82508DE0;
	sub_831A8130(ctx, base);
	// 82508DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508DE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508DE8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82508DEC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82508DF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82508DF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82508DF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508DFC: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82508E00: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82508E04: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 82508E08: 806B01C0  lwz r3, 0x1c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508E0C: 4BF57DCD  bl 0x82460bd8
	ctx.lr = 0x82508E10;
	sub_82460BD8(ctx, base);
	// 82508E10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508E14: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82508E18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82508E1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82508E20: 806B01C0  lwz r3, 0x1c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508E24: 4BF5901D  bl 0x82461e40
	ctx.lr = 0x82508E28;
	sub_82461E40(ctx, base);
	// 82508E28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508E2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82508E30: 806B01C0  lwz r3, 0x1c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508E34: 4BF544DD  bl 0x8245d310
	ctx.lr = 0x82508E38;
	sub_8245D310(ctx, base);
	// 82508E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82508E3C: 48C9F378  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508E40 size=16
    let mut pc: u32 = 0x82508E40;
    'dispatch: loop {
        match pc {
            0x82508E40 => {
    //   block [0x82508E40..0x82508E50)
	// 82508E40: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508E44: 814B01C0  lwz r10, 0x1c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508E48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508E4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508E50 size=8
    let mut pc: u32 = 0x82508E50;
    'dispatch: loop {
        match pc {
            0x82508E50 => {
    //   block [0x82508E50..0x82508E58)
	// 82508E50: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82508E54: 4BF56494  b 0x8245f2e8
	sub_8245F2E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508E58 size=4
    let mut pc: u32 = 0x82508E58;
    'dispatch: loop {
        match pc {
            0x82508E58 => {
    //   block [0x82508E58..0x82508E5C)
	// 82508E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508E60 size=28
    let mut pc: u32 = 0x82508E60;
    'dispatch: loop {
        match pc {
            0x82508E60 => {
    //   block [0x82508E60..0x82508E7C)
	// 82508E60: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508E64: 814B01C8  lwz r10, 0x1c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(456 as u32) ) } as u64;
	// 82508E68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508E6C: 419A0010  beq cr6, 0x82508e7c
	if ctx.cr[6].eq {
		sub_82508E7C(ctx, base);
		return;
	}
	// 82508E70: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82508E74: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82508E78: 48B00140  b 0x83008fb8
	sub_83008FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508E7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508E7C size=8
    let mut pc: u32 = 0x82508E7C;
    'dispatch: loop {
        match pc {
            0x82508E7C => {
    //   block [0x82508E7C..0x82508E84)
	// 82508E7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82508E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508E88 size=12
    let mut pc: u32 = 0x82508E88;
    'dispatch: loop {
        match pc {
            0x82508E88 => {
    //   block [0x82508E88..0x82508E94)
	// 82508E88: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508E8C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82508E90: 4895A948  b 0x82e637d8
	sub_82E637D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508E98 size=16
    let mut pc: u32 = 0x82508E98;
    'dispatch: loop {
        match pc {
            0x82508E98 => {
    //   block [0x82508E98..0x82508EA8)
	// 82508E98: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508E9C: 814B01C0  lwz r10, 0x1c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508EA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508EA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508EA8 size=8
    let mut pc: u32 = 0x82508EA8;
    'dispatch: loop {
        match pc {
            0x82508EA8 => {
    //   block [0x82508EA8..0x82508EB0)
	// 82508EA8: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82508EAC: 4BF54474  b 0x8245d320
	sub_8245D320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508EB0 size=4
    let mut pc: u32 = 0x82508EB0;
    'dispatch: loop {
        match pc {
            0x82508EB0 => {
    //   block [0x82508EB0..0x82508EB4)
	// 82508EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508EB8 size=12
    let mut pc: u32 = 0x82508EB8;
    'dispatch: loop {
        match pc {
            0x82508EB8 => {
    //   block [0x82508EB8..0x82508EC4)
	// 82508EB8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508EBC: 806B01C0  lwz r3, 0x1c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508EC0: 4BF5A560  b 0x82463420
	sub_82463420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508EC8 size=12
    let mut pc: u32 = 0x82508EC8;
    'dispatch: loop {
        match pc {
            0x82508EC8 => {
    //   block [0x82508EC8..0x82508ED4)
	// 82508EC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508ECC: 806B01C0  lwz r3, 0x1c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508ED0: 4BF58BF0  b 0x82461ac0
	sub_82461AC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508ED8 size=12
    let mut pc: u32 = 0x82508ED8;
    'dispatch: loop {
        match pc {
            0x82508ED8 => {
    //   block [0x82508ED8..0x82508EE4)
	// 82508ED8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508EDC: 806B01C0  lwz r3, 0x1c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508EE0: 4BF59DB0  b 0x82462c90
	sub_82462C90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508EE8 size=16
    let mut pc: u32 = 0x82508EE8;
    'dispatch: loop {
        match pc {
            0x82508EE8 => {
    //   block [0x82508EE8..0x82508EF8)
	// 82508EE8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508EEC: 814B0224  lwz r10, 0x224(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(548 as u32) ) } as u64;
	// 82508EF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508EF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508EF8 size=8
    let mut pc: u32 = 0x82508EF8;
    'dispatch: loop {
        match pc {
            0x82508EF8 => {
    //   block [0x82508EF8..0x82508F00)
	// 82508EF8: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82508EFC: 486ABD7C  b 0x82bb4c78
	sub_82BB4C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508F00 size=4
    let mut pc: u32 = 0x82508F00;
    'dispatch: loop {
        match pc {
            0x82508F00 => {
    //   block [0x82508F00..0x82508F04)
	// 82508F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508F08 size=24
    let mut pc: u32 = 0x82508F08;
    'dispatch: loop {
        match pc {
            0x82508F08 => {
    //   block [0x82508F08..0x82508F20)
	// 82508F08: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508F0C: 814B01C0  lwz r10, 0x1c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508F10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508F14: 419A000C  beq cr6, 0x82508f20
	if ctx.cr[6].eq {
		sub_82508F20(ctx, base);
		return;
	}
	// 82508F18: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82508F1C: 4BF54454  b 0x8245d370
	sub_8245D370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508F20 size=8
    let mut pc: u32 = 0x82508F20;
    'dispatch: loop {
        match pc {
            0x82508F20 => {
    //   block [0x82508F20..0x82508F28)
	// 82508F20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82508F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508F28 size=24
    let mut pc: u32 = 0x82508F28;
    'dispatch: loop {
        match pc {
            0x82508F28 => {
    //   block [0x82508F28..0x82508F40)
	// 82508F28: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508F2C: 814B01C0  lwz r10, 0x1c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82508F30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508F34: 419A000C  beq cr6, 0x82508f40
	if ctx.cr[6].eq {
		sub_82508F40(ctx, base);
		return;
	}
	// 82508F38: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82508F3C: 4BF5442C  b 0x8245d368
	sub_8245D368(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508F40 size=8
    let mut pc: u32 = 0x82508F40;
    'dispatch: loop {
        match pc {
            0x82508F40 => {
    //   block [0x82508F40..0x82508F48)
	// 82508F40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82508F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82508F48 size=88
    let mut pc: u32 = 0x82508F48;
    'dispatch: loop {
        match pc {
            0x82508F48 => {
    //   block [0x82508F48..0x82508FA0)
	// 82508F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508F50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508F54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508F58: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82508F5C: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 82508F60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82508F64: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82508F68: 38CBC6CC  addi r6, r11, -0x3934
	ctx.r[6].s64 = ctx.r[11].s64 + -14644;
	// 82508F6C: 38AABE54  addi r5, r10, -0x41ac
	ctx.r[5].s64 = ctx.r[10].s64 + -16812;
	// 82508F70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82508F74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82508F78: 48CA0FD1  bl 0x831a9f48
	ctx.lr = 0x82508F7C;
	sub_831A9F48(ctx, base);
	// 82508F7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82508F80: 4182000C  beq 0x82508f8c
	if ctx.cr[0].eq {
	pc = 0x82508F8C; continue 'dispatch;
	}
	// 82508F84: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82508F88: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82508F8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82508F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508F98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508FA0 size=196
    let mut pc: u32 = 0x82508FA0;
    'dispatch: loop {
        match pc {
            0x82508FA0 => {
    //   block [0x82508FA0..0x82509064)
	// 82508FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82508FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508FB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508FB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82508FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82508FBC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82508FC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82508FC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82508FC8: 4BDB7971  bl 0x822c0938
	ctx.lr = 0x82508FCC;
	sub_822C0938(ctx, base);
	// 82508FCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82508FD0: 41820028  beq 0x82508ff8
	if ctx.cr[0].eq {
	pc = 0x82508FF8; continue 'dispatch;
	}
	// 82508FD4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82508FD8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82508FDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82508FE0: 392B1760  addi r9, r11, 0x1760
	ctx.r[9].s64 = ctx.r[11].s64 + 5984;
	// 82508FE4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82508FE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82508FEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82508FF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82508FF4: 48000008  b 0x82508ffc
	pc = 0x82508FFC; continue 'dispatch;
	// 82508FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82508FFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509000: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509004: 409A0044  bne cr6, 0x82509048
	if !ctx.cr[6].eq {
	pc = 0x82509048; continue 'dispatch;
	}
	// 82509008: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250900C: 419A001C  beq cr6, 0x82509028
	if ctx.cr[6].eq {
	pc = 0x82509028; continue 'dispatch;
	}
	// 82509010: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509014: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250901C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509020: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509024: 4E800421  bctrl
	ctx.lr = 0x82509028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509028: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250902C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509034: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509038: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250903C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509044: 4BDB6FBD  bl 0x822c0000
	ctx.lr = 0x82509048;
	sub_822C0000(ctx, base);
	// 82509048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250904C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509058: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250905C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509068 size=196
    let mut pc: u32 = 0x82509068;
    'dispatch: loop {
        match pc {
            0x82509068 => {
    //   block [0x82509068..0x8250912C)
	// 82509068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250907C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509084: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509088: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250908C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509090: 4BDB78A9  bl 0x822c0938
	ctx.lr = 0x82509094;
	sub_822C0938(ctx, base);
	// 82509094: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509098: 41820028  beq 0x825090c0
	if ctx.cr[0].eq {
	pc = 0x825090C0; continue 'dispatch;
	}
	// 8250909C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825090A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825090A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825090A8: 392B1788  addi r9, r11, 0x1788
	ctx.r[9].s64 = ctx.r[11].s64 + 6024;
	// 825090AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825090B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825090B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825090B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825090BC: 48000008  b 0x825090c4
	pc = 0x825090C4; continue 'dispatch;
	// 825090C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825090C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825090C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825090CC: 409A0044  bne cr6, 0x82509110
	if !ctx.cr[6].eq {
	pc = 0x82509110; continue 'dispatch;
	}
	// 825090D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825090D4: 419A001C  beq cr6, 0x825090f0
	if ctx.cr[6].eq {
	pc = 0x825090F0; continue 'dispatch;
	}
	// 825090D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825090DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825090E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825090E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825090E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825090EC: 4E800421  bctrl
	ctx.lr = 0x825090F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825090F0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825090F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825090F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825090FC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509100: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509104: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509108: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250910C: 4BDB6EF5  bl 0x822c0000
	ctx.lr = 0x82509110;
	sub_822C0000(ctx, base);
	// 82509110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250911C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509120: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509124: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509130 size=196
    let mut pc: u32 = 0x82509130;
    'dispatch: loop {
        match pc {
            0x82509130 => {
    //   block [0x82509130..0x825091F4)
	// 82509130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250913C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509144: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250914C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509150: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509154: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509158: 4BDB77E1  bl 0x822c0938
	ctx.lr = 0x8250915C;
	sub_822C0938(ctx, base);
	// 8250915C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509160: 41820028  beq 0x82509188
	if ctx.cr[0].eq {
	pc = 0x82509188; continue 'dispatch;
	}
	// 82509164: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509168: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250916C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509170: 392B179C  addi r9, r11, 0x179c
	ctx.r[9].s64 = ctx.r[11].s64 + 6044;
	// 82509174: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509178: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250917C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509180: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509184: 48000008  b 0x8250918c
	pc = 0x8250918C; continue 'dispatch;
	// 82509188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250918C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509190: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509194: 409A0044  bne cr6, 0x825091d8
	if !ctx.cr[6].eq {
	pc = 0x825091D8; continue 'dispatch;
	}
	// 82509198: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250919C: 419A001C  beq cr6, 0x825091b8
	if ctx.cr[6].eq {
	pc = 0x825091B8; continue 'dispatch;
	}
	// 825091A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825091A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825091A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825091AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825091B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825091B4: 4E800421  bctrl
	ctx.lr = 0x825091B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825091B8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825091BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825091C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825091C4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825091C8: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 825091CC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825091D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825091D4: 4BDB6E2D  bl 0x822c0000
	ctx.lr = 0x825091D8;
	sub_822C0000(ctx, base);
	// 825091D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825091DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825091E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825091E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825091E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825091EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825091F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825091F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825091F8 size=196
    let mut pc: u32 = 0x825091F8;
    'dispatch: loop {
        match pc {
            0x825091F8 => {
    //   block [0x825091F8..0x825092BC)
	// 825091F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825091FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250920C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509214: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509218: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250921C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509220: 4BDB7719  bl 0x822c0938
	ctx.lr = 0x82509224;
	sub_822C0938(ctx, base);
	// 82509224: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509228: 41820028  beq 0x82509250
	if ctx.cr[0].eq {
	pc = 0x82509250; continue 'dispatch;
	}
	// 8250922C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509230: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509238: 392B17B0  addi r9, r11, 0x17b0
	ctx.r[9].s64 = ctx.r[11].s64 + 6064;
	// 8250923C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509240: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509244: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509248: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250924C: 48000008  b 0x82509254
	pc = 0x82509254; continue 'dispatch;
	// 82509250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509254: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250925C: 409A0044  bne cr6, 0x825092a0
	if !ctx.cr[6].eq {
	pc = 0x825092A0; continue 'dispatch;
	}
	// 82509260: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509264: 419A001C  beq cr6, 0x82509280
	if ctx.cr[6].eq {
	pc = 0x82509280; continue 'dispatch;
	}
	// 82509268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250926C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509274: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250927C: 4E800421  bctrl
	ctx.lr = 0x82509280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509280: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509284: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250928C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509290: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509294: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250929C: 4BDB6D65  bl 0x822c0000
	ctx.lr = 0x825092A0;
	sub_822C0000(ctx, base);
	// 825092A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825092A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825092A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825092AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825092B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825092B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825092B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825092C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825092C0 size=196
    let mut pc: u32 = 0x825092C0;
    'dispatch: loop {
        match pc {
            0x825092C0 => {
    //   block [0x825092C0..0x82509384)
	// 825092C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825092C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825092C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825092CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825092D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825092D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825092D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825092DC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825092E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825092E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825092E8: 4BDB7651  bl 0x822c0938
	ctx.lr = 0x825092EC;
	sub_822C0938(ctx, base);
	// 825092EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825092F0: 41820028  beq 0x82509318
	if ctx.cr[0].eq {
	pc = 0x82509318; continue 'dispatch;
	}
	// 825092F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825092F8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825092FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509300: 392B17C4  addi r9, r11, 0x17c4
	ctx.r[9].s64 = ctx.r[11].s64 + 6084;
	// 82509304: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509308: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250930C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509310: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509314: 48000008  b 0x8250931c
	pc = 0x8250931C; continue 'dispatch;
	// 82509318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250931C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509324: 409A0044  bne cr6, 0x82509368
	if !ctx.cr[6].eq {
	pc = 0x82509368; continue 'dispatch;
	}
	// 82509328: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250932C: 419A001C  beq cr6, 0x82509348
	if ctx.cr[6].eq {
	pc = 0x82509348; continue 'dispatch;
	}
	// 82509330: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509334: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250933C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509344: 4E800421  bctrl
	ctx.lr = 0x82509348;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509348: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250934C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509354: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509358: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250935C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509360: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509364: 4BDB6C9D  bl 0x822c0000
	ctx.lr = 0x82509368;
	sub_822C0000(ctx, base);
	// 82509368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250936C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509378: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250937C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509388 size=196
    let mut pc: u32 = 0x82509388;
    'dispatch: loop {
        match pc {
            0x82509388 => {
    //   block [0x82509388..0x8250944C)
	// 82509388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250938C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250939C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825093A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825093A4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825093A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825093AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825093B0: 4BDB7589  bl 0x822c0938
	ctx.lr = 0x825093B4;
	sub_822C0938(ctx, base);
	// 825093B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825093B8: 41820028  beq 0x825093e0
	if ctx.cr[0].eq {
	pc = 0x825093E0; continue 'dispatch;
	}
	// 825093BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825093C0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825093C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825093C8: 392B17D8  addi r9, r11, 0x17d8
	ctx.r[9].s64 = ctx.r[11].s64 + 6104;
	// 825093CC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825093D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825093D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825093D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825093DC: 48000008  b 0x825093e4
	pc = 0x825093E4; continue 'dispatch;
	// 825093E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825093E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825093E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825093EC: 409A0044  bne cr6, 0x82509430
	if !ctx.cr[6].eq {
	pc = 0x82509430; continue 'dispatch;
	}
	// 825093F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825093F4: 419A001C  beq cr6, 0x82509410
	if ctx.cr[6].eq {
	pc = 0x82509410; continue 'dispatch;
	}
	// 825093F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825093FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509404: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250940C: 4E800421  bctrl
	ctx.lr = 0x82509410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509410: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509414: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509418: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250941C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509420: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509424: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250942C: 4BDB6BD5  bl 0x822c0000
	ctx.lr = 0x82509430;
	sub_822C0000(ctx, base);
	// 82509430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509434: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250943C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509450 size=196
    let mut pc: u32 = 0x82509450;
    'dispatch: loop {
        match pc {
            0x82509450 => {
    //   block [0x82509450..0x82509514)
	// 82509450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250945C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509464: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250946C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509470: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509474: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509478: 4BDB74C1  bl 0x822c0938
	ctx.lr = 0x8250947C;
	sub_822C0938(ctx, base);
	// 8250947C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509480: 41820028  beq 0x825094a8
	if ctx.cr[0].eq {
	pc = 0x825094A8; continue 'dispatch;
	}
	// 82509484: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509488: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250948C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509490: 392B17EC  addi r9, r11, 0x17ec
	ctx.r[9].s64 = ctx.r[11].s64 + 6124;
	// 82509494: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509498: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250949C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825094A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825094A4: 48000008  b 0x825094ac
	pc = 0x825094AC; continue 'dispatch;
	// 825094A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825094AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825094B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825094B4: 409A0044  bne cr6, 0x825094f8
	if !ctx.cr[6].eq {
	pc = 0x825094F8; continue 'dispatch;
	}
	// 825094B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825094BC: 419A001C  beq cr6, 0x825094d8
	if ctx.cr[6].eq {
	pc = 0x825094D8; continue 'dispatch;
	}
	// 825094C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825094C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825094C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825094CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825094D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825094D4: 4E800421  bctrl
	ctx.lr = 0x825094D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825094D8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825094DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825094E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825094E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825094E8: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 825094EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825094F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825094F4: 4BDB6B0D  bl 0x822c0000
	ctx.lr = 0x825094F8;
	sub_822C0000(ctx, base);
	// 825094F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825094FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509508: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250950C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509518 size=196
    let mut pc: u32 = 0x82509518;
    'dispatch: loop {
        match pc {
            0x82509518 => {
    //   block [0x82509518..0x825095DC)
	// 82509518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250951C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250952C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509534: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509538: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250953C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509540: 4BDB73F9  bl 0x822c0938
	ctx.lr = 0x82509544;
	sub_822C0938(ctx, base);
	// 82509544: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509548: 41820028  beq 0x82509570
	if ctx.cr[0].eq {
	pc = 0x82509570; continue 'dispatch;
	}
	// 8250954C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509550: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509554: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509558: 392B1800  addi r9, r11, 0x1800
	ctx.r[9].s64 = ctx.r[11].s64 + 6144;
	// 8250955C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509560: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509564: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509568: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250956C: 48000008  b 0x82509574
	pc = 0x82509574; continue 'dispatch;
	// 82509570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509574: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509578: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250957C: 409A0044  bne cr6, 0x825095c0
	if !ctx.cr[6].eq {
	pc = 0x825095C0; continue 'dispatch;
	}
	// 82509580: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509584: 419A001C  beq cr6, 0x825095a0
	if ctx.cr[6].eq {
	pc = 0x825095A0; continue 'dispatch;
	}
	// 82509588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250958C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509594: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250959C: 4E800421  bctrl
	ctx.lr = 0x825095A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825095A0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825095A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825095A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825095AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825095B0: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 825095B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825095B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825095BC: 4BDB6A45  bl 0x822c0000
	ctx.lr = 0x825095C0;
	sub_822C0000(ctx, base);
	// 825095C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825095C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825095C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825095CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825095D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825095D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825095D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825095E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825095E0 size=196
    let mut pc: u32 = 0x825095E0;
    'dispatch: loop {
        match pc {
            0x825095E0 => {
    //   block [0x825095E0..0x825096A4)
	// 825095E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825095E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825095E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825095EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825095F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825095F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825095F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825095FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509600: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509604: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509608: 4BDB7331  bl 0x822c0938
	ctx.lr = 0x8250960C;
	sub_822C0938(ctx, base);
	// 8250960C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509610: 41820028  beq 0x82509638
	if ctx.cr[0].eq {
	pc = 0x82509638; continue 'dispatch;
	}
	// 82509614: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509618: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250961C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509620: 392B1814  addi r9, r11, 0x1814
	ctx.r[9].s64 = ctx.r[11].s64 + 6164;
	// 82509624: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509628: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250962C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509630: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509634: 48000008  b 0x8250963c
	pc = 0x8250963C; continue 'dispatch;
	// 82509638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250963C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509640: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509644: 409A0044  bne cr6, 0x82509688
	if !ctx.cr[6].eq {
	pc = 0x82509688; continue 'dispatch;
	}
	// 82509648: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250964C: 419A001C  beq cr6, 0x82509668
	if ctx.cr[6].eq {
	pc = 0x82509668; continue 'dispatch;
	}
	// 82509650: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509654: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250965C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509664: 4E800421  bctrl
	ctx.lr = 0x82509668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509668: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250966C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509674: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509678: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250967C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509680: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509684: 4BDB697D  bl 0x822c0000
	ctx.lr = 0x82509688;
	sub_822C0000(ctx, base);
	// 82509688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250968C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509698: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250969C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825096A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825096A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825096A8 size=196
    let mut pc: u32 = 0x825096A8;
    'dispatch: loop {
        match pc {
            0x825096A8 => {
    //   block [0x825096A8..0x8250976C)
	// 825096A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825096AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825096B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825096B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825096B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825096BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825096C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825096C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825096C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825096CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825096D0: 4BDB7269  bl 0x822c0938
	ctx.lr = 0x825096D4;
	sub_822C0938(ctx, base);
	// 825096D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825096D8: 41820028  beq 0x82509700
	if ctx.cr[0].eq {
	pc = 0x82509700; continue 'dispatch;
	}
	// 825096DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825096E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825096E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825096E8: 392B1828  addi r9, r11, 0x1828
	ctx.r[9].s64 = ctx.r[11].s64 + 6184;
	// 825096EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825096F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825096F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825096F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825096FC: 48000008  b 0x82509704
	pc = 0x82509704; continue 'dispatch;
	// 82509700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509704: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250970C: 409A0044  bne cr6, 0x82509750
	if !ctx.cr[6].eq {
	pc = 0x82509750; continue 'dispatch;
	}
	// 82509710: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509714: 419A001C  beq cr6, 0x82509730
	if ctx.cr[6].eq {
	pc = 0x82509730; continue 'dispatch;
	}
	// 82509718: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250971C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509724: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250972C: 4E800421  bctrl
	ctx.lr = 0x82509730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509730: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509734: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250973C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509740: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509744: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250974C: 4BDB68B5  bl 0x822c0000
	ctx.lr = 0x82509750;
	sub_822C0000(ctx, base);
	// 82509750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250975C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509770 size=196
    let mut pc: u32 = 0x82509770;
    'dispatch: loop {
        match pc {
            0x82509770 => {
    //   block [0x82509770..0x82509834)
	// 82509770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250977C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509784: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250978C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509790: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509794: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509798: 4BDB71A1  bl 0x822c0938
	ctx.lr = 0x8250979C;
	sub_822C0938(ctx, base);
	// 8250979C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825097A0: 41820028  beq 0x825097c8
	if ctx.cr[0].eq {
	pc = 0x825097C8; continue 'dispatch;
	}
	// 825097A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825097A8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825097AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825097B0: 392B183C  addi r9, r11, 0x183c
	ctx.r[9].s64 = ctx.r[11].s64 + 6204;
	// 825097B4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825097B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825097BC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825097C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825097C4: 48000008  b 0x825097cc
	pc = 0x825097CC; continue 'dispatch;
	// 825097C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825097CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825097D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825097D4: 409A0044  bne cr6, 0x82509818
	if !ctx.cr[6].eq {
	pc = 0x82509818; continue 'dispatch;
	}
	// 825097D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825097DC: 419A001C  beq cr6, 0x825097f8
	if ctx.cr[6].eq {
	pc = 0x825097F8; continue 'dispatch;
	}
	// 825097E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825097E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825097E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825097EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825097F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825097F4: 4E800421  bctrl
	ctx.lr = 0x825097F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825097F8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825097FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509804: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509808: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250980C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509814: 4BDB67ED  bl 0x822c0000
	ctx.lr = 0x82509818;
	sub_822C0000(ctx, base);
	// 82509818: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250981C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509828: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250982C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509838 size=196
    let mut pc: u32 = 0x82509838;
    'dispatch: loop {
        match pc {
            0x82509838 => {
    //   block [0x82509838..0x825098FC)
	// 82509838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250984C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509854: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509858: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250985C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509860: 4BDB70D9  bl 0x822c0938
	ctx.lr = 0x82509864;
	sub_822C0938(ctx, base);
	// 82509864: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509868: 41820028  beq 0x82509890
	if ctx.cr[0].eq {
	pc = 0x82509890; continue 'dispatch;
	}
	// 8250986C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509870: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509874: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509878: 392B1850  addi r9, r11, 0x1850
	ctx.r[9].s64 = ctx.r[11].s64 + 6224;
	// 8250987C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509880: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509884: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509888: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250988C: 48000008  b 0x82509894
	pc = 0x82509894; continue 'dispatch;
	// 82509890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509894: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509898: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250989C: 409A0044  bne cr6, 0x825098e0
	if !ctx.cr[6].eq {
	pc = 0x825098E0; continue 'dispatch;
	}
	// 825098A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825098A4: 419A001C  beq cr6, 0x825098c0
	if ctx.cr[6].eq {
	pc = 0x825098C0; continue 'dispatch;
	}
	// 825098A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825098AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825098B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825098B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825098B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825098BC: 4E800421  bctrl
	ctx.lr = 0x825098C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825098C0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825098C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825098C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825098CC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825098D0: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 825098D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825098D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825098DC: 4BDB6725  bl 0x822c0000
	ctx.lr = 0x825098E0;
	sub_822C0000(ctx, base);
	// 825098E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825098E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825098E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825098EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825098F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825098F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825098F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509900 size=196
    let mut pc: u32 = 0x82509900;
    'dispatch: loop {
        match pc {
            0x82509900 => {
    //   block [0x82509900..0x825099C4)
	// 82509900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250990C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509914: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250991C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509920: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509924: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509928: 4BDB7011  bl 0x822c0938
	ctx.lr = 0x8250992C;
	sub_822C0938(ctx, base);
	// 8250992C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509930: 41820028  beq 0x82509958
	if ctx.cr[0].eq {
	pc = 0x82509958; continue 'dispatch;
	}
	// 82509934: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509938: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250993C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509940: 392B1864  addi r9, r11, 0x1864
	ctx.r[9].s64 = ctx.r[11].s64 + 6244;
	// 82509944: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509948: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250994C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509950: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509954: 48000008  b 0x8250995c
	pc = 0x8250995C; continue 'dispatch;
	// 82509958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250995C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509964: 409A0044  bne cr6, 0x825099a8
	if !ctx.cr[6].eq {
	pc = 0x825099A8; continue 'dispatch;
	}
	// 82509968: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250996C: 419A001C  beq cr6, 0x82509988
	if ctx.cr[6].eq {
	pc = 0x82509988; continue 'dispatch;
	}
	// 82509970: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509974: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250997C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509984: 4E800421  bctrl
	ctx.lr = 0x82509988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509988: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250998C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509990: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509994: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509998: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250999C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825099A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825099A4: 4BDB665D  bl 0x822c0000
	ctx.lr = 0x825099A8;
	sub_822C0000(ctx, base);
	// 825099A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825099AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825099B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825099B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825099B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825099BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825099C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825099C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825099C8 size=196
    let mut pc: u32 = 0x825099C8;
    'dispatch: loop {
        match pc {
            0x825099C8 => {
    //   block [0x825099C8..0x82509A8C)
	// 825099C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825099CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825099D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825099D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825099D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825099DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825099E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825099E4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825099E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825099EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825099F0: 4BDB6F49  bl 0x822c0938
	ctx.lr = 0x825099F4;
	sub_822C0938(ctx, base);
	// 825099F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825099F8: 41820028  beq 0x82509a20
	if ctx.cr[0].eq {
	pc = 0x82509A20; continue 'dispatch;
	}
	// 825099FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509A00: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509A04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509A08: 392B1878  addi r9, r11, 0x1878
	ctx.r[9].s64 = ctx.r[11].s64 + 6264;
	// 82509A0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509A10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509A14: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509A18: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509A1C: 48000008  b 0x82509a24
	pc = 0x82509A24; continue 'dispatch;
	// 82509A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509A24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509A28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509A2C: 409A0044  bne cr6, 0x82509a70
	if !ctx.cr[6].eq {
	pc = 0x82509A70; continue 'dispatch;
	}
	// 82509A30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509A34: 419A001C  beq cr6, 0x82509a50
	if ctx.cr[6].eq {
	pc = 0x82509A50; continue 'dispatch;
	}
	// 82509A38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509A3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509A40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509A44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509A48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509A4C: 4E800421  bctrl
	ctx.lr = 0x82509A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509A50: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509A54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509A58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509A5C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509A60: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509A64: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509A68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509A6C: 4BDB6595  bl 0x822c0000
	ctx.lr = 0x82509A70;
	sub_822C0000(ctx, base);
	// 82509A70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509A74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509A80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509A90 size=196
    let mut pc: u32 = 0x82509A90;
    'dispatch: loop {
        match pc {
            0x82509A90 => {
    //   block [0x82509A90..0x82509B54)
	// 82509A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509AA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509AAC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509AB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509AB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509AB8: 4BDB6E81  bl 0x822c0938
	ctx.lr = 0x82509ABC;
	sub_822C0938(ctx, base);
	// 82509ABC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509AC0: 41820028  beq 0x82509ae8
	if ctx.cr[0].eq {
	pc = 0x82509AE8; continue 'dispatch;
	}
	// 82509AC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509AC8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509ACC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509AD0: 392B188C  addi r9, r11, 0x188c
	ctx.r[9].s64 = ctx.r[11].s64 + 6284;
	// 82509AD4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509AD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509ADC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509AE0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509AE4: 48000008  b 0x82509aec
	pc = 0x82509AEC; continue 'dispatch;
	// 82509AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509AEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509AF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509AF4: 409A0044  bne cr6, 0x82509b38
	if !ctx.cr[6].eq {
	pc = 0x82509B38; continue 'dispatch;
	}
	// 82509AF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509AFC: 419A001C  beq cr6, 0x82509b18
	if ctx.cr[6].eq {
	pc = 0x82509B18; continue 'dispatch;
	}
	// 82509B00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509B04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509B0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509B10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509B14: 4E800421  bctrl
	ctx.lr = 0x82509B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509B18: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509B1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509B20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509B24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509B28: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509B2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509B30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509B34: 4BDB64CD  bl 0x822c0000
	ctx.lr = 0x82509B38;
	sub_822C0000(ctx, base);
	// 82509B38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509B3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509B48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509B4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509B58 size=196
    let mut pc: u32 = 0x82509B58;
    'dispatch: loop {
        match pc {
            0x82509B58 => {
    //   block [0x82509B58..0x82509C1C)
	// 82509B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509B60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509B64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509B68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509B6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509B74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509B78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509B7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509B80: 4BDB6DB9  bl 0x822c0938
	ctx.lr = 0x82509B84;
	sub_822C0938(ctx, base);
	// 82509B84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509B88: 41820028  beq 0x82509bb0
	if ctx.cr[0].eq {
	pc = 0x82509BB0; continue 'dispatch;
	}
	// 82509B8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509B90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509B94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509B98: 392B18B4  addi r9, r11, 0x18b4
	ctx.r[9].s64 = ctx.r[11].s64 + 6324;
	// 82509B9C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509BA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509BA4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509BA8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509BAC: 48000008  b 0x82509bb4
	pc = 0x82509BB4; continue 'dispatch;
	// 82509BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509BB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509BB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509BBC: 409A0044  bne cr6, 0x82509c00
	if !ctx.cr[6].eq {
	pc = 0x82509C00; continue 'dispatch;
	}
	// 82509BC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509BC4: 419A001C  beq cr6, 0x82509be0
	if ctx.cr[6].eq {
	pc = 0x82509BE0; continue 'dispatch;
	}
	// 82509BC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509BCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509BD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509BD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509BDC: 4E800421  bctrl
	ctx.lr = 0x82509BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509BE0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509BE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509BE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509BEC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509BF0: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509BF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509BF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509BFC: 4BDB6405  bl 0x822c0000
	ctx.lr = 0x82509C00;
	sub_822C0000(ctx, base);
	// 82509C00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509C04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509C10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509C14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509C20 size=196
    let mut pc: u32 = 0x82509C20;
    'dispatch: loop {
        match pc {
            0x82509C20 => {
    //   block [0x82509C20..0x82509CE4)
	// 82509C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509C34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509C3C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509C40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509C44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509C48: 4BDB6CF1  bl 0x822c0938
	ctx.lr = 0x82509C4C;
	sub_822C0938(ctx, base);
	// 82509C4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509C50: 41820028  beq 0x82509c78
	if ctx.cr[0].eq {
	pc = 0x82509C78; continue 'dispatch;
	}
	// 82509C54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509C58: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509C5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509C60: 392B18DC  addi r9, r11, 0x18dc
	ctx.r[9].s64 = ctx.r[11].s64 + 6364;
	// 82509C64: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509C68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509C6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509C70: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509C74: 48000008  b 0x82509c7c
	pc = 0x82509C7C; continue 'dispatch;
	// 82509C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509C7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509C84: 409A0044  bne cr6, 0x82509cc8
	if !ctx.cr[6].eq {
	pc = 0x82509CC8; continue 'dispatch;
	}
	// 82509C88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509C8C: 419A001C  beq cr6, 0x82509ca8
	if ctx.cr[6].eq {
	pc = 0x82509CA8; continue 'dispatch;
	}
	// 82509C90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509C94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509C9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509CA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509CA4: 4E800421  bctrl
	ctx.lr = 0x82509CA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509CA8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509CAC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509CB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509CB4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509CB8: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509CBC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509CC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509CC4: 4BDB633D  bl 0x822c0000
	ctx.lr = 0x82509CC8;
	sub_822C0000(ctx, base);
	// 82509CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509CE8 size=196
    let mut pc: u32 = 0x82509CE8;
    'dispatch: loop {
        match pc {
            0x82509CE8 => {
    //   block [0x82509CE8..0x82509DAC)
	// 82509CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509CFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509D04: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509D08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509D0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509D10: 4BDB6C29  bl 0x822c0938
	ctx.lr = 0x82509D14;
	sub_822C0938(ctx, base);
	// 82509D14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509D18: 41820028  beq 0x82509d40
	if ctx.cr[0].eq {
	pc = 0x82509D40; continue 'dispatch;
	}
	// 82509D1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509D20: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509D24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509D28: 392B18F0  addi r9, r11, 0x18f0
	ctx.r[9].s64 = ctx.r[11].s64 + 6384;
	// 82509D2C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509D30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509D34: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509D38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509D3C: 48000008  b 0x82509d44
	pc = 0x82509D44; continue 'dispatch;
	// 82509D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509D44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509D48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509D4C: 409A0044  bne cr6, 0x82509d90
	if !ctx.cr[6].eq {
	pc = 0x82509D90; continue 'dispatch;
	}
	// 82509D50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509D54: 419A001C  beq cr6, 0x82509d70
	if ctx.cr[6].eq {
	pc = 0x82509D70; continue 'dispatch;
	}
	// 82509D58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509D5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509D64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509D68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509D6C: 4E800421  bctrl
	ctx.lr = 0x82509D70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509D70: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509D74: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509D78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509D7C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509D80: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509D84: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509D88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509D8C: 4BDB6275  bl 0x822c0000
	ctx.lr = 0x82509D90;
	sub_822C0000(ctx, base);
	// 82509D90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509D94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509DA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509DA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509DB0 size=196
    let mut pc: u32 = 0x82509DB0;
    'dispatch: loop {
        match pc {
            0x82509DB0 => {
    //   block [0x82509DB0..0x82509E74)
	// 82509DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509DB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509DBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509DC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509DC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509DCC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509DD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509DD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509DD8: 4BDB6B61  bl 0x822c0938
	ctx.lr = 0x82509DDC;
	sub_822C0938(ctx, base);
	// 82509DDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509DE0: 41820028  beq 0x82509e08
	if ctx.cr[0].eq {
	pc = 0x82509E08; continue 'dispatch;
	}
	// 82509DE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509DE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509DEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509DF0: 392B1904  addi r9, r11, 0x1904
	ctx.r[9].s64 = ctx.r[11].s64 + 6404;
	// 82509DF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509DF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509DFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509E00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509E04: 48000008  b 0x82509e0c
	pc = 0x82509E0C; continue 'dispatch;
	// 82509E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509E0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509E10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509E14: 409A0044  bne cr6, 0x82509e58
	if !ctx.cr[6].eq {
	pc = 0x82509E58; continue 'dispatch;
	}
	// 82509E18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509E1C: 419A001C  beq cr6, 0x82509e38
	if ctx.cr[6].eq {
	pc = 0x82509E38; continue 'dispatch;
	}
	// 82509E20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509E24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509E2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509E30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509E34: 4E800421  bctrl
	ctx.lr = 0x82509E38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509E38: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509E3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509E40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509E44: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509E48: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509E4C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509E50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509E54: 4BDB61AD  bl 0x822c0000
	ctx.lr = 0x82509E58;
	sub_822C0000(ctx, base);
	// 82509E58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509E5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509E68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509E78 size=196
    let mut pc: u32 = 0x82509E78;
    'dispatch: loop {
        match pc {
            0x82509E78 => {
    //   block [0x82509E78..0x82509F3C)
	// 82509E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509E80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509E84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509E88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509E8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509E94: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509E98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509E9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509EA0: 4BDB6A99  bl 0x822c0938
	ctx.lr = 0x82509EA4;
	sub_822C0938(ctx, base);
	// 82509EA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509EA8: 41820028  beq 0x82509ed0
	if ctx.cr[0].eq {
	pc = 0x82509ED0; continue 'dispatch;
	}
	// 82509EAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509EB0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509EB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509EB8: 392B1918  addi r9, r11, 0x1918
	ctx.r[9].s64 = ctx.r[11].s64 + 6424;
	// 82509EBC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509EC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509EC4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509EC8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509ECC: 48000008  b 0x82509ed4
	pc = 0x82509ED4; continue 'dispatch;
	// 82509ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509ED4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509ED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509EDC: 409A0044  bne cr6, 0x82509f20
	if !ctx.cr[6].eq {
	pc = 0x82509F20; continue 'dispatch;
	}
	// 82509EE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509EE4: 419A001C  beq cr6, 0x82509f00
	if ctx.cr[6].eq {
	pc = 0x82509F00; continue 'dispatch;
	}
	// 82509EE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509EEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509EF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509EFC: 4E800421  bctrl
	ctx.lr = 0x82509F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509F00: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509F04: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509F08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509F0C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509F10: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509F14: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509F18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509F1C: 4BDB60E5  bl 0x822c0000
	ctx.lr = 0x82509F20;
	sub_822C0000(ctx, base);
	// 82509F20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509F24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509F30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509F40 size=196
    let mut pc: u32 = 0x82509F40;
    'dispatch: loop {
        match pc {
            0x82509F40 => {
    //   block [0x82509F40..0x8250A004)
	// 82509F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509F48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509F4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509F54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509F5C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82509F60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509F64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509F68: 4BDB69D1  bl 0x822c0938
	ctx.lr = 0x82509F6C;
	sub_822C0938(ctx, base);
	// 82509F6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82509F70: 41820028  beq 0x82509f98
	if ctx.cr[0].eq {
	pc = 0x82509F98; continue 'dispatch;
	}
	// 82509F74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82509F78: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82509F7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82509F80: 392B192C  addi r9, r11, 0x192c
	ctx.r[9].s64 = ctx.r[11].s64 + 6444;
	// 82509F84: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82509F88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82509F8C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509F90: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82509F94: 48000008  b 0x82509f9c
	pc = 0x82509F9C; continue 'dispatch;
	// 82509F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82509F9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509FA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509FA4: 409A0044  bne cr6, 0x82509fe8
	if !ctx.cr[6].eq {
	pc = 0x82509FE8; continue 'dispatch;
	}
	// 82509FA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82509FAC: 419A001C  beq cr6, 0x82509fc8
	if ctx.cr[6].eq {
	pc = 0x82509FC8; continue 'dispatch;
	}
	// 82509FB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509FB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509FB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509FBC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509FC4: 4E800421  bctrl
	ctx.lr = 0x82509FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509FC8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82509FCC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82509FD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82509FD4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82509FD8: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 82509FDC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82509FE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82509FE4: 4BDB601D  bl 0x822c0000
	ctx.lr = 0x82509FE8;
	sub_822C0000(ctx, base);
	// 82509FE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509FEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509FF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509FFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A008 size=196
    let mut pc: u32 = 0x8250A008;
    'dispatch: loop {
        match pc {
            0x8250A008 => {
    //   block [0x8250A008..0x8250A0CC)
	// 8250A008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A010: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A014: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A018: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A01C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250A020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A024: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250A028: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A02C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A030: 4BDB6909  bl 0x822c0938
	ctx.lr = 0x8250A034;
	sub_822C0938(ctx, base);
	// 8250A034: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250A038: 41820028  beq 0x8250a060
	if ctx.cr[0].eq {
	pc = 0x8250A060; continue 'dispatch;
	}
	// 8250A03C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250A040: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250A044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250A048: 392B1940  addi r9, r11, 0x1940
	ctx.r[9].s64 = ctx.r[11].s64 + 6464;
	// 8250A04C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250A050: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250A054: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250A058: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250A05C: 48000008  b 0x8250a064
	pc = 0x8250A064; continue 'dispatch;
	// 8250A060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A064: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A06C: 409A0044  bne cr6, 0x8250a0b0
	if !ctx.cr[6].eq {
	pc = 0x8250A0B0; continue 'dispatch;
	}
	// 8250A070: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250A074: 419A001C  beq cr6, 0x8250a090
	if ctx.cr[6].eq {
	pc = 0x8250A090; continue 'dispatch;
	}
	// 8250A078: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A07C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250A080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A084: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A088: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250A08C: 4E800421  bctrl
	ctx.lr = 0x8250A090;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250A090: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250A094: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250A098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250A09C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250A0A0: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250A0A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250A0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250A0AC: 4BDB5F55  bl 0x822c0000
	ctx.lr = 0x8250A0B0;
	sub_822C0000(ctx, base);
	// 8250A0B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250A0B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A0C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A0C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A0D0 size=196
    let mut pc: u32 = 0x8250A0D0;
    'dispatch: loop {
        match pc {
            0x8250A0D0 => {
    //   block [0x8250A0D0..0x8250A194)
	// 8250A0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A0D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A0DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A0E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A0E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250A0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A0EC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250A0F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A0F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A0F8: 4BDB6841  bl 0x822c0938
	ctx.lr = 0x8250A0FC;
	sub_822C0938(ctx, base);
	// 8250A0FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250A100: 41820028  beq 0x8250a128
	if ctx.cr[0].eq {
	pc = 0x8250A128; continue 'dispatch;
	}
	// 8250A104: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250A108: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250A10C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250A110: 392B1954  addi r9, r11, 0x1954
	ctx.r[9].s64 = ctx.r[11].s64 + 6484;
	// 8250A114: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250A118: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250A11C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250A120: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250A124: 48000008  b 0x8250a12c
	pc = 0x8250A12C; continue 'dispatch;
	// 8250A128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A12C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A130: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A134: 409A0044  bne cr6, 0x8250a178
	if !ctx.cr[6].eq {
	pc = 0x8250A178; continue 'dispatch;
	}
	// 8250A138: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250A13C: 419A001C  beq cr6, 0x8250a158
	if ctx.cr[6].eq {
	pc = 0x8250A158; continue 'dispatch;
	}
	// 8250A140: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A144: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250A148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A14C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A150: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250A154: 4E800421  bctrl
	ctx.lr = 0x8250A158;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250A158: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250A15C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250A160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250A164: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250A168: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250A16C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250A170: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250A174: 4BDB5E8D  bl 0x822c0000
	ctx.lr = 0x8250A178;
	sub_822C0000(ctx, base);
	// 8250A178: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250A17C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A188: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A18C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A198 size=172
    let mut pc: u32 = 0x8250A198;
    'dispatch: loop {
        match pc {
            0x8250A198 => {
    //   block [0x8250A198..0x8250A244)
	// 8250A198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A1AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250A1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A1B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250A1B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A1BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A1C0: 4BDB6779  bl 0x822c0938
	ctx.lr = 0x8250A1C4;
	sub_822C0938(ctx, base);
	// 8250A1C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250A1C8: 41820028  beq 0x8250a1f0
	if ctx.cr[0].eq {
	pc = 0x8250A1F0; continue 'dispatch;
	}
	// 8250A1CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250A1D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250A1D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250A1D8: 392B1968  addi r9, r11, 0x1968
	ctx.r[9].s64 = ctx.r[11].s64 + 6504;
	// 8250A1DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250A1E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250A1E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250A1E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250A1EC: 48000008  b 0x8250a1f4
	pc = 0x8250A1F4; continue 'dispatch;
	// 8250A1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A1F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A1F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A1FC: 409A002C  bne cr6, 0x8250a228
	if !ctx.cr[6].eq {
	pc = 0x8250A228; continue 'dispatch;
	}
	// 8250A200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A204: 4BDB6065  bl 0x822c0268
	ctx.lr = 0x8250A208;
	sub_822C0268(ctx, base);
	// 8250A208: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250A20C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250A210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250A214: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250A218: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250A21C: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250A220: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250A224: 4BDB5DDD  bl 0x822c0000
	ctx.lr = 0x8250A228;
	sub_822C0000(ctx, base);
	// 8250A228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250A22C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A238: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A248 size=196
    let mut pc: u32 = 0x8250A248;
    'dispatch: loop {
        match pc {
            0x8250A248 => {
    //   block [0x8250A248..0x8250A30C)
	// 8250A248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A25C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250A260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A264: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250A268: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A26C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A270: 4BDB66C9  bl 0x822c0938
	ctx.lr = 0x8250A274;
	sub_822C0938(ctx, base);
	// 8250A274: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250A278: 41820028  beq 0x8250a2a0
	if ctx.cr[0].eq {
	pc = 0x8250A2A0; continue 'dispatch;
	}
	// 8250A27C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250A280: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250A284: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250A288: 392B197C  addi r9, r11, 0x197c
	ctx.r[9].s64 = ctx.r[11].s64 + 6524;
	// 8250A28C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250A290: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250A294: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250A298: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250A29C: 48000008  b 0x8250a2a4
	pc = 0x8250A2A4; continue 'dispatch;
	// 8250A2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A2A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250A2A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A2AC: 409A0044  bne cr6, 0x8250a2f0
	if !ctx.cr[6].eq {
	pc = 0x8250A2F0; continue 'dispatch;
	}
	// 8250A2B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250A2B4: 419A001C  beq cr6, 0x8250a2d0
	if ctx.cr[6].eq {
	pc = 0x8250A2D0; continue 'dispatch;
	}
	// 8250A2B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A2BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250A2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A2C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A2C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250A2CC: 4E800421  bctrl
	ctx.lr = 0x8250A2D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250A2D0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250A2D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250A2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250A2DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250A2E0: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250A2E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250A2E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250A2EC: 4BDB5D15  bl 0x822c0000
	ctx.lr = 0x8250A2F0;
	sub_822C0000(ctx, base);
	// 8250A2F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250A2F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A300: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A310 size=148
    let mut pc: u32 = 0x8250A310;
    'dispatch: loop {
        match pc {
            0x8250A310 => {
    //   block [0x8250A310..0x8250A3A4)
	// 8250A310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A31C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A324: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8250A328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A32C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8250A330: 48005041  bl 0x8250f370
	ctx.lr = 0x8250A334;
	sub_8250F370(ctx, base);
	// 8250A334: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250A338: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250A33C: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 8250A340: 488E96C9  bl 0x82df3a08
	ctx.lr = 0x8250A344;
	sub_82DF3A08(ctx, base);
	// 8250A344: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A348: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250A34C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250A350: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250A354: 48959535  bl 0x82e63888
	ctx.lr = 0x8250A358;
	sub_82E63888(ctx, base);
	// 8250A358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250A35C: 488E90CD  bl 0x82df3428
	ctx.lr = 0x8250A360;
	sub_82DF3428(ctx, base);
	// 8250A360: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250A364: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250A368: 808BE268  lwz r4, -0x1d98(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7576 as u32) ) } as u64;
	// 8250A36C: 488E969D  bl 0x82df3a08
	ctx.lr = 0x8250A370;
	sub_82DF3A08(ctx, base);
	// 8250A370: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A374: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250A378: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8250A37C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250A380: 48959509  bl 0x82e63888
	ctx.lr = 0x8250A384;
	sub_82E63888(ctx, base);
	// 8250A384: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250A388: 488E90A1  bl 0x82df3428
	ctx.lr = 0x8250A38C;
	sub_82DF3428(ctx, base);
	// 8250A38C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A39C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A3A8 size=84
    let mut pc: u32 = 0x8250A3A8;
    'dispatch: loop {
        match pc {
            0x8250A3A8 => {
    //   block [0x8250A3A8..0x8250A3FC)
	// 8250A3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A3B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A3B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A3B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A3BC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A3C0: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8250A3C4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8250A3C8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250A3CC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A3D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250A3D4: 4BDBA08D  bl 0x822c4460
	ctx.lr = 0x8250A3D8;
	sub_822C4460(ctx, base);
	// 8250A3D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250A3DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250A3E0: 419A0008  beq cr6, 0x8250a3e8
	if ctx.cr[6].eq {
	pc = 0x8250A3E8; continue 'dispatch;
	}
	// 8250A3E4: 4BDB64AD  bl 0x822c0890
	ctx.lr = 0x8250A3E8;
	sub_822C0890(ctx, base);
	// 8250A3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A3F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A400 size=168
    let mut pc: u32 = 0x8250A400;
    'dispatch: loop {
        match pc {
            0x8250A400 => {
    //   block [0x8250A400..0x8250A4A8)
	// 8250A400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A40C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A414: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A418: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250A41C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250A420: 488F7F61  bl 0x82e02380
	ctx.lr = 0x8250A424;
	sub_82E02380(ctx, base);
	// 8250A424: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A428: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A42C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250A430: 388B0018  addi r4, r11, 0x18
	ctx.r[4].s64 = ctx.r[11].s64 + 24;
	// 8250A434: 488F7D95  bl 0x82e021c8
	ctx.lr = 0x8250A438;
	sub_82E021C8(ctx, base);
	// 8250A438: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A43C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250A440: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8250A444: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8250A448: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250A44C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250A450: 4BDBA011  bl 0x822c4460
	ctx.lr = 0x8250A454;
	sub_822C4460(ctx, base);
	// 8250A454: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A458: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250A45C: 488F496D  bl 0x82dfedc8
	ctx.lr = 0x8250A460;
	sub_82DFEDC8(ctx, base);
	// 8250A460: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250A464: 480C3F65  bl 0x825ce3c8
	ctx.lr = 0x8250A468;
	sub_825CE3C8(ctx, base);
	// 8250A468: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250A46C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250A470: 488F44B1  bl 0x82dfe920
	ctx.lr = 0x8250A474;
	sub_82DFE920(ctx, base);
	// 8250A474: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A478: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250A47C: 3BEB0018  addi r31, r11, 0x18
	ctx.r[31].s64 = ctx.r[11].s64 + 24;
	// 8250A480: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250A484: 419A0008  beq cr6, 0x8250a48c
	if ctx.cr[6].eq {
	pc = 0x8250A48C; continue 'dispatch;
	}
	// 8250A488: 4BDB6409  bl 0x822c0890
	ctx.lr = 0x8250A48C;
	sub_822C0890(ctx, base);
	// 8250A48C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A49C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A4A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A4A8 size=56
    let mut pc: u32 = 0x8250A4A8;
    'dispatch: loop {
        match pc {
            0x8250A4A8 => {
    //   block [0x8250A4A8..0x8250A4E0)
	// 8250A4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A4B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A4B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A4B8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A4BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A4C0: 808B0044  lwz r4, 0x44(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250A4C4: 480A6515  bl 0x825b09d8
	ctx.lr = 0x8250A4C8;
	sub_825B09D8(ctx, base);
	// 8250A4C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A4CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A4D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A4E0 size=56
    let mut pc: u32 = 0x8250A4E0;
    'dispatch: loop {
        match pc {
            0x8250A4E0 => {
    //   block [0x8250A4E0..0x8250A518)
	// 8250A4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A4E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A4EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A4F0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A4F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A4F8: 808B0044  lwz r4, 0x44(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250A4FC: 480A66C5  bl 0x825b0bc0
	ctx.lr = 0x8250A500;
	sub_825B0BC0(ctx, base);
	// 8250A500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A518 size=28
    let mut pc: u32 = 0x8250A518;
    'dispatch: loop {
        match pc {
            0x8250A518 => {
    //   block [0x8250A518..0x8250A534)
	// 8250A518: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A51C: 814B0074  lwz r10, 0x74(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 8250A520: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250A524: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 8250A528: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A52C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250A530: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A534 size=36
    let mut pc: u32 = 0x8250A534;
    'dispatch: loop {
        match pc {
            0x8250A534 => {
    //   block [0x8250A534..0x8250A558)
	// 8250A534: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250A538: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250A53C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250A540: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250A544: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250A548: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250A54C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250A550: 4082FFE8  bne 0x8250a538
	if !ctx.cr[0].eq {
	pc = 0x8250A538; continue 'dispatch;
	}
	// 8250A554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A558 size=112
    let mut pc: u32 = 0x8250A558;
    'dispatch: loop {
        match pc {
            0x8250A558 => {
    //   block [0x8250A558..0x8250A5C8)
	// 8250A558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A56C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250A570: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A574: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8250A578: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A57C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A580: 396B0084  addi r11, r11, 0x84
	ctx.r[11].s64 = ctx.r[11].s64 + 132;
	// 8250A584: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250A588: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250A58C: 4BDB9ED5  bl 0x822c4460
	ctx.lr = 0x8250A590;
	sub_822C4460(ctx, base);
	// 8250A590: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A594: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 8250A598: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 8250A59C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A5A0: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8250A5A4: 48AFEA15  bl 0x83008fb8
	ctx.lr = 0x8250A5A8;
	sub_83008FB8(ctx, base);
	// 8250A5A8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A5AC: 906B0090  stw r3, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 8250A5B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A5BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A5C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A5C8 size=80
    let mut pc: u32 = 0x8250A5C8;
    'dispatch: loop {
        match pc {
            0x8250A5C8 => {
    //   block [0x8250A5C8..0x8250A618)
	// 8250A5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A5D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A5D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A5D8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A5E0: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8250A5E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A5E8: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8250A5EC: 409A0008  bne cr6, 0x8250a5f4
	if !ctx.cr[6].eq {
	pc = 0x8250A5F4; continue 'dispatch;
	}
	// 8250A5F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A5F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A5FC: 488E75FD  bl 0x82df1bf8
	ctx.lr = 0x8250A600;
	sub_82DF1BF8(ctx, base);
	// 8250A600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A618 size=32
    let mut pc: u32 = 0x8250A618;
    'dispatch: loop {
        match pc {
            0x8250A618 => {
    //   block [0x8250A618..0x8250A638)
	// 8250A618: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8250A61C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A620: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8250A624: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A628: 396A00A4  addi r11, r10, 0xa4
	ctx.r[11].s64 = ctx.r[10].s64 + 164;
	// 8250A62C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250A630: 912A00A4  stw r9, 0xa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(164 as u32), ctx.r[9].u32 ) };
	// 8250A634: 4BDB9E2C  b 0x822c4460
	sub_822C4460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A638 size=28
    let mut pc: u32 = 0x8250A638;
    'dispatch: loop {
        match pc {
            0x8250A638 => {
    //   block [0x8250A638..0x8250A654)
	// 8250A638: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A63C: 814B00A4  lwz r10, 0xa4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8250A640: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250A644: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8250A648: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A64C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250A650: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A654 size=36
    let mut pc: u32 = 0x8250A654;
    'dispatch: loop {
        match pc {
            0x8250A654 => {
    //   block [0x8250A654..0x8250A678)
	// 8250A654: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250A658: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250A65C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250A660: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250A664: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250A668: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250A66C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250A670: 4082FFE8  bne 0x8250a658
	if !ctx.cr[0].eq {
	pc = 0x8250A658; continue 'dispatch;
	}
	// 8250A674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A678 size=80
    let mut pc: u32 = 0x8250A678;
    'dispatch: loop {
        match pc {
            0x8250A678 => {
    //   block [0x8250A678..0x8250A6C8)
	// 8250A678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A688: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A68C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A690: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 8250A694: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A698: 388B00CC  addi r4, r11, 0xcc
	ctx.r[4].s64 = ctx.r[11].s64 + 204;
	// 8250A69C: 409A0008  bne cr6, 0x8250a6a4
	if !ctx.cr[6].eq {
	pc = 0x8250A6A4; continue 'dispatch;
	}
	// 8250A6A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A6A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A6AC: 488E754D  bl 0x82df1bf8
	ctx.lr = 0x8250A6B0;
	sub_82DF1BF8(ctx, base);
	// 8250A6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A6B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A6C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A6C8 size=80
    let mut pc: u32 = 0x8250A6C8;
    'dispatch: loop {
        match pc {
            0x8250A6C8 => {
    //   block [0x8250A6C8..0x8250A718)
	// 8250A6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A6D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A6D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A6D8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A6DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A6E0: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 8250A6E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A6E8: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8250A6EC: 409A0008  bne cr6, 0x8250a6f4
	if !ctx.cr[6].eq {
	pc = 0x8250A6F4; continue 'dispatch;
	}
	// 8250A6F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A6FC: 488E74FD  bl 0x82df1bf8
	ctx.lr = 0x8250A700;
	sub_82DF1BF8(ctx, base);
	// 8250A700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A718 size=80
    let mut pc: u32 = 0x8250A718;
    'dispatch: loop {
        match pc {
            0x8250A718 => {
    //   block [0x8250A718..0x8250A768)
	// 8250A718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A728: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A72C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A730: 816B00D8  lwz r11, 0xd8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 8250A734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A738: 388B00CC  addi r4, r11, 0xcc
	ctx.r[4].s64 = ctx.r[11].s64 + 204;
	// 8250A73C: 409A0008  bne cr6, 0x8250a744
	if !ctx.cr[6].eq {
	pc = 0x8250A744; continue 'dispatch;
	}
	// 8250A740: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A74C: 488E74AD  bl 0x82df1bf8
	ctx.lr = 0x8250A750;
	sub_82DF1BF8(ctx, base);
	// 8250A750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A768 size=80
    let mut pc: u32 = 0x8250A768;
    'dispatch: loop {
        match pc {
            0x8250A768 => {
    //   block [0x8250A768..0x8250A7B8)
	// 8250A768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A778: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A77C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A780: 816B00F4  lwz r11, 0xf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 8250A784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A788: 388B00CC  addi r4, r11, 0xcc
	ctx.r[4].s64 = ctx.r[11].s64 + 204;
	// 8250A78C: 409A0008  bne cr6, 0x8250a794
	if !ctx.cr[6].eq {
	pc = 0x8250A794; continue 'dispatch;
	}
	// 8250A790: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A79C: 488E745D  bl 0x82df1bf8
	ctx.lr = 0x8250A7A0;
	sub_82DF1BF8(ctx, base);
	// 8250A7A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A7A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A7B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A7B8 size=80
    let mut pc: u32 = 0x8250A7B8;
    'dispatch: loop {
        match pc {
            0x8250A7B8 => {
    //   block [0x8250A7B8..0x8250A808)
	// 8250A7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A7C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A7C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A7C8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A7CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A7D0: 816B010C  lwz r11, 0x10c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(268 as u32) ) } as u64;
	// 8250A7D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A7D8: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8250A7DC: 409A0008  bne cr6, 0x8250a7e4
	if !ctx.cr[6].eq {
	pc = 0x8250A7E4; continue 'dispatch;
	}
	// 8250A7E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A7E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A7E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A7EC: 488E740D  bl 0x82df1bf8
	ctx.lr = 0x8250A7F0;
	sub_82DF1BF8(ctx, base);
	// 8250A7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A7F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A808 size=80
    let mut pc: u32 = 0x8250A808;
    'dispatch: loop {
        match pc {
            0x8250A808 => {
    //   block [0x8250A808..0x8250A858)
	// 8250A808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A818: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A81C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A820: 816B00FC  lwz r11, 0xfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(252 as u32) ) } as u64;
	// 8250A824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A828: 388B00CC  addi r4, r11, 0xcc
	ctx.r[4].s64 = ctx.r[11].s64 + 204;
	// 8250A82C: 409A0008  bne cr6, 0x8250a834
	if !ctx.cr[6].eq {
	pc = 0x8250A834; continue 'dispatch;
	}
	// 8250A830: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A83C: 488E73BD  bl 0x82df1bf8
	ctx.lr = 0x8250A840;
	sub_82DF1BF8(ctx, base);
	// 8250A840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A858 size=84
    let mut pc: u32 = 0x8250A858;
    'dispatch: loop {
        match pc {
            0x8250A858 => {
    //   block [0x8250A858..0x8250A8AC)
	// 8250A858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A868: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8250A86C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A870: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8250A874: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A878: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A87C: 396A0190  addi r11, r10, 0x190
	ctx.r[11].s64 = ctx.r[10].s64 + 400;
	// 8250A880: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250A884: 912A0190  stw r9, 0x190(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(400 as u32), ctx.r[9].u32 ) };
	// 8250A888: 4BDB9BD9  bl 0x822c4460
	ctx.lr = 0x8250A88C;
	sub_822C4460(ctx, base);
	// 8250A88C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A890: 814B0190  lwz r10, 0x190(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(400 as u32) ) } as u64;
	// 8250A894: 914B0198  stw r10, 0x198(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(408 as u32), ctx.r[10].u32 ) };
	// 8250A898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A8A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A8B0 size=80
    let mut pc: u32 = 0x8250A8B0;
    'dispatch: loop {
        match pc {
            0x8250A8B0 => {
    //   block [0x8250A8B0..0x8250A900)
	// 8250A8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A8B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A8BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A8C0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A8C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A8C8: 816B0174  lwz r11, 0x174(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(372 as u32) ) } as u64;
	// 8250A8CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A8D0: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8250A8D4: 409A0008  bne cr6, 0x8250a8dc
	if !ctx.cr[6].eq {
	pc = 0x8250A8DC; continue 'dispatch;
	}
	// 8250A8D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A8E4: 488E7315  bl 0x82df1bf8
	ctx.lr = 0x8250A8E8;
	sub_82DF1BF8(ctx, base);
	// 8250A8E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A8EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A8F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A900 size=80
    let mut pc: u32 = 0x8250A900;
    'dispatch: loop {
        match pc {
            0x8250A900 => {
    //   block [0x8250A900..0x8250A950)
	// 8250A900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A90C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A910: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A918: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8250A91C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A920: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8250A924: 409A0008  bne cr6, 0x8250a92c
	if !ctx.cr[6].eq {
	pc = 0x8250A92C; continue 'dispatch;
	}
	// 8250A928: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A92C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A934: 488E72C5  bl 0x82df1bf8
	ctx.lr = 0x8250A938;
	sub_82DF1BF8(ctx, base);
	// 8250A938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A93C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A950 size=80
    let mut pc: u32 = 0x8250A950;
    'dispatch: loop {
        match pc {
            0x8250A950 => {
    //   block [0x8250A950..0x8250A9A0)
	// 8250A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A95C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A960: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250A968: 816B0198  lwz r11, 0x198(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(408 as u32) ) } as u64;
	// 8250A96C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250A970: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8250A974: 409A0008  bne cr6, 0x8250a97c
	if !ctx.cr[6].eq {
	pc = 0x8250A97C; continue 'dispatch;
	}
	// 8250A978: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250A97C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250A980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A984: 488E7275  bl 0x82df1bf8
	ctx.lr = 0x8250A988;
	sub_82DF1BF8(ctx, base);
	// 8250A988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A98C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A9A0 size=100
    let mut pc: u32 = 0x8250A9A0;
    'dispatch: loop {
        match pc {
            0x8250A9A0 => {
    //   block [0x8250A9A0..0x8250AA04)
	// 8250A9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A9A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250A9AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A9B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A9B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A9B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250A9BC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A9C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250A9C4: 419A0028  beq cr6, 0x8250a9ec
	if ctx.cr[6].eq {
	pc = 0x8250A9EC; continue 'dispatch;
	}
	// 8250A9C8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A9CC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8250A9D0: 396B019C  addi r11, r11, 0x19c
	ctx.r[11].s64 = ctx.r[11].s64 + 412;
	// 8250A9D4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250A9D8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250A9DC: 4BDB9A85  bl 0x822c4460
	ctx.lr = 0x8250A9E0;
	sub_822C4460(ctx, base);
	// 8250A9E0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250A9E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A9E8: 914B01A4  stw r10, 0x1a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(420 as u32), ctx.r[10].u32 ) };
	// 8250A9EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250A9F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A9F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A9F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A9FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AA08 size=60
    let mut pc: u32 = 0x8250AA08;
    'dispatch: loop {
        match pc {
            0x8250AA08 => {
    //   block [0x8250AA08..0x8250AA44)
	// 8250AA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AA14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AA18: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250AA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250AA20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AA24: 808B01A4  lwz r4, 0x1a4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8250AA28: 488E71D1  bl 0x82df1bf8
	ctx.lr = 0x8250AA2C;
	sub_82DF1BF8(ctx, base);
	// 8250AA2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250AA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AA3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AA48 size=104
    let mut pc: u32 = 0x8250AA48;
    'dispatch: loop {
        match pc {
            0x8250AA48 => {
    //   block [0x8250AA48..0x8250AAB0)
	// 8250AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AA50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AA54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AA58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AA5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250AA60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250AA64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250AA68: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8250AA6C: 4BFFFB5D  bl 0x8250a5c8
	ctx.lr = 0x8250AA70;
	sub_8250A5C8(ctx, base);
	// 8250AA70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250AA74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250AA78: 386BFF40  addi r3, r11, -0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + -192;
	// 8250AA7C: 409A0008  bne cr6, 0x8250aa84
	if !ctx.cr[6].eq {
	pc = 0x8250AA84; continue 'dispatch;
	}
	// 8250AA80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250AA84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250AA88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250AA8C: 482A13FD  bl 0x827abe88
	ctx.lr = 0x8250AA90;
	sub_827ABE88(ctx, base);
	// 8250AA90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250AA94: 488E71FD  bl 0x82df1c90
	ctx.lr = 0x8250AA98;
	sub_82DF1C90(ctx, base);
	// 8250AA98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250AA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AAA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AAA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AAB0 size=188
    let mut pc: u32 = 0x8250AAB0;
    'dispatch: loop {
        match pc {
            0x8250AAB0 => {
    //   block [0x8250AAB0..0x8250AB6C)
	// 8250AAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AAB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AABC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AAC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AAC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250AAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250AACC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250AAD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250AAD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250AAD8: 4BDB5E61  bl 0x822c0938
	ctx.lr = 0x8250AADC;
	sub_822C0938(ctx, base);
	// 8250AADC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250AAE0: 41820028  beq 0x8250ab08
	if ctx.cr[0].eq {
	pc = 0x8250AB08; continue 'dispatch;
	}
	// 8250AAE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250AAE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250AAEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250AAF0: 392B18C8  addi r9, r11, 0x18c8
	ctx.r[9].s64 = ctx.r[11].s64 + 6344;
	// 8250AAF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250AAF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250AAFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250AB00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250AB04: 48000008  b 0x8250ab0c
	pc = 0x8250AB0C; continue 'dispatch;
	// 8250AB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250AB0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250AB10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250AB14: 409A003C  bne cr6, 0x8250ab50
	if !ctx.cr[6].eq {
	pc = 0x8250AB50; continue 'dispatch;
	}
	// 8250AB18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250AB1C: 419A0014  beq cr6, 0x8250ab30
	if ctx.cr[6].eq {
	pc = 0x8250AB30; continue 'dispatch;
	}
	// 8250AB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AB24: 48917E7D  bl 0x82e229a0
	ctx.lr = 0x8250AB28;
	sub_82E229A0(ctx, base);
	// 8250AB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AB2C: 4BDB573D  bl 0x822c0268
	ctx.lr = 0x8250AB30;
	sub_822C0268(ctx, base);
	// 8250AB30: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250AB34: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250AB38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250AB3C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250AB40: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250AB44: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250AB48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250AB4C: 4BDB54B5  bl 0x822c0000
	ctx.lr = 0x8250AB50;
	sub_822C0000(ctx, base);
	// 8250AB50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250AB54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250AB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AB60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AB64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AB70 size=64
    let mut pc: u32 = 0x8250AB70;
    'dispatch: loop {
        match pc {
            0x8250AB70 => {
    //   block [0x8250AB70..0x8250ABB0)
	// 8250AB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AB78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AB7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AB80: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250AB84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250AB88: 419A0014  beq cr6, 0x8250ab9c
	if ctx.cr[6].eq {
	pc = 0x8250AB9C; continue 'dispatch;
	}
	// 8250AB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AB90: 48917E11  bl 0x82e229a0
	ctx.lr = 0x8250AB94;
	sub_82E229A0(ctx, base);
	// 8250AB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AB98: 4BDB56D1  bl 0x822c0268
	ctx.lr = 0x8250AB9C;
	sub_822C0268(ctx, base);
	// 8250AB9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250ABA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250ABA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250ABA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250ABAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250ABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250ABB0 size=160
    let mut pc: u32 = 0x8250ABB0;
    'dispatch: loop {
        match pc {
            0x8250ABB0 => {
    //   block [0x8250ABB0..0x8250AC50)
	// 8250ABB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250ABB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250ABB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250ABBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250ABC0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8250ABC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250ABC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250ABCC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250ABD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250ABD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250ABD8: 388B0670  addi r4, r11, 0x670
	ctx.r[4].s64 = ctx.r[11].s64 + 1648;
	// 8250ABDC: 488E8E2D  bl 0x82df3a08
	ctx.lr = 0x8250ABE0;
	sub_82DF3A08(ctx, base);
	// 8250ABE0: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8250ABE4: 488E84AD  bl 0x82df3090
	ctx.lr = 0x8250ABE8;
	sub_82DF3090(ctx, base);
	// 8250ABE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250ABEC: 488E7D5D  bl 0x82df2948
	ctx.lr = 0x8250ABF0;
	sub_82DF2948(ctx, base);
	// 8250ABF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250ABF4: 488E8835  bl 0x82df3428
	ctx.lr = 0x8250ABF8;
	sub_82DF3428(ctx, base);
	// 8250ABF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250ABFC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250AC00: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250AC04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8250AC08: 48000024  b 0x8250ac2c
	pc = 0x8250AC2C; continue 'dispatch;
	// 8250AC0C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250AC10: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250AC14: 4801D3ED  bl 0x82528000
	ctx.lr = 0x8250AC18;
	sub_82528000(ctx, base);
	// 8250AC18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250AC1C: 4BE96A6D  bl 0x823a1688
	ctx.lr = 0x8250AC20;
	sub_823A1688(ctx, base);
	// 8250AC20: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250AC24: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AC28: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250AC2C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250AC30: 409AFFDC  bne cr6, 0x8250ac0c
	if !ctx.cr[6].eq {
	pc = 0x8250AC0C; continue 'dispatch;
	}
	// 8250AC34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250AC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AC40: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8250AC44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AC48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AC50 size=136
    let mut pc: u32 = 0x8250AC50;
    'dispatch: loop {
        match pc {
            0x8250AC50 => {
    //   block [0x8250AC50..0x8250ACD8)
	// 8250AC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AC58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AC5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AC60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AC64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250AC68: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250AC6C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250AC70: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8250AC74: 48000020  b 0x8250ac94
	pc = 0x8250AC94; continue 'dispatch;
	// 8250AC78: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250AC7C: 4801D3A5  bl 0x82528020
	ctx.lr = 0x8250AC80;
	sub_82528020(ctx, base);
	// 8250AC80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250AC84: 4BE96A05  bl 0x823a1688
	ctx.lr = 0x8250AC88;
	sub_823A1688(ctx, base);
	// 8250AC88: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250AC8C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AC90: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250AC94: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250AC98: 409AFFE0  bne cr6, 0x8250ac78
	if !ctx.cr[6].eq {
	pc = 0x8250AC78; continue 'dispatch;
	}
	// 8250AC9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250ACA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250ACA4: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 8250ACA8: 488E8D61  bl 0x82df3a08
	ctx.lr = 0x8250ACAC;
	sub_82DF3A08(ctx, base);
	// 8250ACAC: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8250ACB0: 488E83E1  bl 0x82df3090
	ctx.lr = 0x8250ACB4;
	sub_82DF3090(ctx, base);
	// 8250ACB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250ACB8: 488E7C91  bl 0x82df2948
	ctx.lr = 0x8250ACBC;
	sub_82DF2948(ctx, base);
	// 8250ACBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250ACC0: 488E8769  bl 0x82df3428
	ctx.lr = 0x8250ACC4;
	sub_82DF3428(ctx, base);
	// 8250ACC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250ACC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250ACCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250ACD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250ACD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250ACD8 size=8
    let mut pc: u32 = 0x8250ACD8;
    'dispatch: loop {
        match pc {
            0x8250ACD8 => {
    //   block [0x8250ACD8..0x8250ACE0)
	// 8250ACD8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8250ACDC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250ACE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250ACE0 size=24
    let mut pc: u32 = 0x8250ACE0;
    'dispatch: loop {
        match pc {
            0x8250ACE0 => {
    //   block [0x8250ACE0..0x8250ACF8)
	// 8250ACE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250ACE4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250ACE8: 812B01B4  lwz r9, 0x1b4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(436 as u32) ) } as u64;
	// 8250ACEC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250ACF0: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 8250ACF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250ACF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250ACF8 size=120
    let mut pc: u32 = 0x8250ACF8;
    'dispatch: loop {
        match pc {
            0x8250ACF8 => {
    //   block [0x8250ACF8..0x8250AD70)
	// 8250ACF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250ACFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AD00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AD04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AD08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250AD0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AD10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250AD14: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250AD18: 38A009F1  li r5, 0x9f1
	ctx.r[5].s64 = 2545;
	// 8250AD1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250AD20: 488E76C9  bl 0x82df23e8
	ctx.lr = 0x8250AD24;
	sub_82DF23E8(ctx, base);
	// 8250AD24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250AD28: 4182000C  beq 0x8250ad34
	if ctx.cr[0].eq {
	pc = 0x8250AD34; continue 'dispatch;
	}
	// 8250AD2C: 486A49FD  bl 0x82baf728
	ctx.lr = 0x8250AD30;
	sub_82BAF728(ctx, base);
	// 8250AD30: 48000008  b 0x8250ad38
	pc = 0x8250AD38; continue 'dispatch;
	// 8250AD34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250AD38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250AD3C: 83EB0224  lwz r31, 0x224(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(548 as u32) ) } as u64;
	// 8250AD40: 906B0224  stw r3, 0x224(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(548 as u32), ctx.r[3].u32 ) };
	// 8250AD44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250AD48: 419A0014  beq cr6, 0x8250ad5c
	if ctx.cr[6].eq {
	pc = 0x8250AD5C; continue 'dispatch;
	}
	// 8250AD4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AD50: 486A4A31  bl 0x82baf780
	ctx.lr = 0x8250AD54;
	sub_82BAF780(ctx, base);
	// 8250AD54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AD58: 488E7681  bl 0x82df23d8
	ctx.lr = 0x8250AD5C;
	sub_82DF23D8(ctx, base);
	// 8250AD5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250AD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AD70 size=112
    let mut pc: u32 = 0x8250AD70;
    'dispatch: loop {
        match pc {
            0x8250AD70 => {
    //   block [0x8250AD70..0x8250ADE0)
	// 8250AD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AD78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AD7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AD80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AD84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250AD88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AD8C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250AD90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AD94: 4BFFE5F5  bl 0x82509388
	ctx.lr = 0x8250AD98;
	sub_82509388(ctx, base);
	// 8250AD98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250AD9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250ADA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250ADA4: 4BDB525D  bl 0x822c0000
	ctx.lr = 0x8250ADA8;
	sub_822C0000(ctx, base);
	// 8250ADA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250ADAC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250ADB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250ADB4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250ADB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250ADBC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250ADC0: 419A0008  beq cr6, 0x8250adc8
	if ctx.cr[6].eq {
	pc = 0x8250ADC8; continue 'dispatch;
	}
	// 8250ADC4: 4BDB5ACD  bl 0x822c0890
	ctx.lr = 0x8250ADC8;
	sub_822C0890(ctx, base);
	// 8250ADC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250ADCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250ADD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250ADD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250ADD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250ADDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250ADE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250ADE0 size=112
    let mut pc: u32 = 0x8250ADE0;
    'dispatch: loop {
        match pc {
            0x8250ADE0 => {
    //   block [0x8250ADE0..0x8250AE50)
	// 8250ADE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250ADE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250ADE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250ADEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250ADF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250ADF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250ADF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250ADFC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250AE00: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AE04: 4BFFE64D  bl 0x82509450
	ctx.lr = 0x8250AE08;
	sub_82509450(ctx, base);
	// 8250AE08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250AE0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250AE10: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AE14: 4BDB51ED  bl 0x822c0000
	ctx.lr = 0x8250AE18;
	sub_822C0000(ctx, base);
	// 8250AE18: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AE1C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250AE20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250AE24: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250AE28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250AE2C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250AE30: 419A0008  beq cr6, 0x8250ae38
	if ctx.cr[6].eq {
	pc = 0x8250AE38; continue 'dispatch;
	}
	// 8250AE34: 4BDB5A5D  bl 0x822c0890
	ctx.lr = 0x8250AE38;
	sub_822C0890(ctx, base);
	// 8250AE38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250AE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AE44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AE48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AE50 size=112
    let mut pc: u32 = 0x8250AE50;
    'dispatch: loop {
        match pc {
            0x8250AE50 => {
    //   block [0x8250AE50..0x8250AEC0)
	// 8250AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AE58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AE5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AE60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AE64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250AE68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AE6C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250AE70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AE74: 4BFFE6A5  bl 0x82509518
	ctx.lr = 0x8250AE78;
	sub_82509518(ctx, base);
	// 8250AE78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250AE7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250AE80: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AE84: 4BDB517D  bl 0x822c0000
	ctx.lr = 0x8250AE88;
	sub_822C0000(ctx, base);
	// 8250AE88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AE8C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250AE90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250AE94: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250AE98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250AE9C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250AEA0: 419A0008  beq cr6, 0x8250aea8
	if ctx.cr[6].eq {
	pc = 0x8250AEA8; continue 'dispatch;
	}
	// 8250AEA4: 4BDB59ED  bl 0x822c0890
	ctx.lr = 0x8250AEA8;
	sub_822C0890(ctx, base);
	// 8250AEA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250AEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AEB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AEB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AEC0 size=112
    let mut pc: u32 = 0x8250AEC0;
    'dispatch: loop {
        match pc {
            0x8250AEC0 => {
    //   block [0x8250AEC0..0x8250AF30)
	// 8250AEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AEC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AED4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250AED8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AEDC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250AEE0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AEE4: 4BFFE6FD  bl 0x825095e0
	ctx.lr = 0x8250AEE8;
	sub_825095E0(ctx, base);
	// 8250AEE8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250AEEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250AEF0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AEF4: 4BDB510D  bl 0x822c0000
	ctx.lr = 0x8250AEF8;
	sub_822C0000(ctx, base);
	// 8250AEF8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AEFC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250AF00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250AF04: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250AF08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250AF0C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250AF10: 419A0008  beq cr6, 0x8250af18
	if ctx.cr[6].eq {
	pc = 0x8250AF18; continue 'dispatch;
	}
	// 8250AF14: 4BDB597D  bl 0x822c0890
	ctx.lr = 0x8250AF18;
	sub_822C0890(ctx, base);
	// 8250AF18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250AF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AF24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AF28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AF30 size=112
    let mut pc: u32 = 0x8250AF30;
    'dispatch: loop {
        match pc {
            0x8250AF30 => {
    //   block [0x8250AF30..0x8250AFA0)
	// 8250AF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AF38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AF3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AF40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AF44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250AF48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AF4C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250AF50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AF54: 4BFFE755  bl 0x825096a8
	ctx.lr = 0x8250AF58;
	sub_825096A8(ctx, base);
	// 8250AF58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250AF5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250AF60: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AF64: 4BDB509D  bl 0x822c0000
	ctx.lr = 0x8250AF68;
	sub_822C0000(ctx, base);
	// 8250AF68: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AF6C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250AF70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250AF74: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250AF78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250AF7C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250AF80: 419A0008  beq cr6, 0x8250af88
	if ctx.cr[6].eq {
	pc = 0x8250AF88; continue 'dispatch;
	}
	// 8250AF84: 4BDB590D  bl 0x822c0890
	ctx.lr = 0x8250AF88;
	sub_822C0890(ctx, base);
	// 8250AF88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250AF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AF94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AF98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250AF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AFA0 size=112
    let mut pc: u32 = 0x8250AFA0;
    'dispatch: loop {
        match pc {
            0x8250AFA0 => {
    //   block [0x8250AFA0..0x8250B010)
	// 8250AFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AFA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AFAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AFB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AFB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250AFB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250AFBC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250AFC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AFC4: 4BFFE7AD  bl 0x82509770
	ctx.lr = 0x8250AFC8;
	sub_82509770(ctx, base);
	// 8250AFC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250AFCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250AFD0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250AFD4: 4BDB502D  bl 0x822c0000
	ctx.lr = 0x8250AFD8;
	sub_822C0000(ctx, base);
	// 8250AFD8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AFDC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250AFE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250AFE4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250AFE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250AFEC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250AFF0: 419A0008  beq cr6, 0x8250aff8
	if ctx.cr[6].eq {
	pc = 0x8250AFF8; continue 'dispatch;
	}
	// 8250AFF4: 4BDB589D  bl 0x822c0890
	ctx.lr = 0x8250AFF8;
	sub_822C0890(ctx, base);
	// 8250AFF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250AFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B010 size=112
    let mut pc: u32 = 0x8250B010;
    'dispatch: loop {
        match pc {
            0x8250B010 => {
    //   block [0x8250B010..0x8250B080)
	// 8250B010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B01C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B024: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B028: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B02C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B030: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B034: 4BFFE805  bl 0x82509838
	ctx.lr = 0x8250B038;
	sub_82509838(ctx, base);
	// 8250B038: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B03C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B040: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B044: 4BDB4FBD  bl 0x822c0000
	ctx.lr = 0x8250B048;
	sub_822C0000(ctx, base);
	// 8250B048: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B04C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B050: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B054: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B058: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B05C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B060: 419A0008  beq cr6, 0x8250b068
	if ctx.cr[6].eq {
	pc = 0x8250B068; continue 'dispatch;
	}
	// 8250B064: 4BDB582D  bl 0x822c0890
	ctx.lr = 0x8250B068;
	sub_822C0890(ctx, base);
	// 8250B068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B074: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B080 size=112
    let mut pc: u32 = 0x8250B080;
    'dispatch: loop {
        match pc {
            0x8250B080 => {
    //   block [0x8250B080..0x8250B0F0)
	// 8250B080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B088: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B08C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B094: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B09C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B0A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B0A4: 4BFFE85D  bl 0x82509900
	ctx.lr = 0x8250B0A8;
	sub_82509900(ctx, base);
	// 8250B0A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B0AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B0B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B0B4: 4BDB4F4D  bl 0x822c0000
	ctx.lr = 0x8250B0B8;
	sub_822C0000(ctx, base);
	// 8250B0B8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B0BC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B0C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B0C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B0C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B0CC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B0D0: 419A0008  beq cr6, 0x8250b0d8
	if ctx.cr[6].eq {
	pc = 0x8250B0D8; continue 'dispatch;
	}
	// 8250B0D4: 4BDB57BD  bl 0x822c0890
	ctx.lr = 0x8250B0D8;
	sub_822C0890(ctx, base);
	// 8250B0D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B0E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B0E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B0F0 size=112
    let mut pc: u32 = 0x8250B0F0;
    'dispatch: loop {
        match pc {
            0x8250B0F0 => {
    //   block [0x8250B0F0..0x8250B160)
	// 8250B0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B0F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B0FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B104: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B10C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B110: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B114: 4BFFE8B5  bl 0x825099c8
	ctx.lr = 0x8250B118;
	sub_825099C8(ctx, base);
	// 8250B118: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B11C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B120: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B124: 4BDB4EDD  bl 0x822c0000
	ctx.lr = 0x8250B128;
	sub_822C0000(ctx, base);
	// 8250B128: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B12C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B130: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B134: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B138: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B13C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B140: 419A0008  beq cr6, 0x8250b148
	if ctx.cr[6].eq {
	pc = 0x8250B148; continue 'dispatch;
	}
	// 8250B144: 4BDB574D  bl 0x822c0890
	ctx.lr = 0x8250B148;
	sub_822C0890(ctx, base);
	// 8250B148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B154: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B160 size=112
    let mut pc: u32 = 0x8250B160;
    'dispatch: loop {
        match pc {
            0x8250B160 => {
    //   block [0x8250B160..0x8250B1D0)
	// 8250B160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B16C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B174: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B17C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B180: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B184: 4BFFE90D  bl 0x82509a90
	ctx.lr = 0x8250B188;
	sub_82509A90(ctx, base);
	// 8250B188: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B18C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B190: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B194: 4BDB4E6D  bl 0x822c0000
	ctx.lr = 0x8250B198;
	sub_822C0000(ctx, base);
	// 8250B198: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B19C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B1A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B1A4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B1A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B1AC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B1B0: 419A0008  beq cr6, 0x8250b1b8
	if ctx.cr[6].eq {
	pc = 0x8250B1B8; continue 'dispatch;
	}
	// 8250B1B4: 4BDB56DD  bl 0x822c0890
	ctx.lr = 0x8250B1B8;
	sub_822C0890(ctx, base);
	// 8250B1B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B1C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B1C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B1D0 size=112
    let mut pc: u32 = 0x8250B1D0;
    'dispatch: loop {
        match pc {
            0x8250B1D0 => {
    //   block [0x8250B1D0..0x8250B240)
	// 8250B1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B1D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B1DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B1E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B1E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B1E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B1EC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B1F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B1F4: 4BFFE965  bl 0x82509b58
	ctx.lr = 0x8250B1F8;
	sub_82509B58(ctx, base);
	// 8250B1F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B1FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B200: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B204: 4BDB4DFD  bl 0x822c0000
	ctx.lr = 0x8250B208;
	sub_822C0000(ctx, base);
	// 8250B208: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B20C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B210: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B214: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B218: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B21C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B220: 419A0008  beq cr6, 0x8250b228
	if ctx.cr[6].eq {
	pc = 0x8250B228; continue 'dispatch;
	}
	// 8250B224: 4BDB566D  bl 0x822c0890
	ctx.lr = 0x8250B228;
	sub_822C0890(ctx, base);
	// 8250B228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B234: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B240 size=112
    let mut pc: u32 = 0x8250B240;
    'dispatch: loop {
        match pc {
            0x8250B240 => {
    //   block [0x8250B240..0x8250B2B0)
	// 8250B240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B24C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B254: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B25C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B260: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B264: 4BFFE9BD  bl 0x82509c20
	ctx.lr = 0x8250B268;
	sub_82509C20(ctx, base);
	// 8250B268: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B26C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B270: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B274: 4BDB4D8D  bl 0x822c0000
	ctx.lr = 0x8250B278;
	sub_822C0000(ctx, base);
	// 8250B278: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B27C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B280: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B284: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B288: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B28C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B290: 419A0008  beq cr6, 0x8250b298
	if ctx.cr[6].eq {
	pc = 0x8250B298; continue 'dispatch;
	}
	// 8250B294: 4BDB55FD  bl 0x822c0890
	ctx.lr = 0x8250B298;
	sub_822C0890(ctx, base);
	// 8250B298: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B2A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B2A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B2B0 size=112
    let mut pc: u32 = 0x8250B2B0;
    'dispatch: loop {
        match pc {
            0x8250B2B0 => {
    //   block [0x8250B2B0..0x8250B320)
	// 8250B2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B2B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B2BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B2C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B2C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B2CC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B2D0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B2D4: 4BFFEA15  bl 0x82509ce8
	ctx.lr = 0x8250B2D8;
	sub_82509CE8(ctx, base);
	// 8250B2D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B2DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B2E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B2E4: 4BDB4D1D  bl 0x822c0000
	ctx.lr = 0x8250B2E8;
	sub_822C0000(ctx, base);
	// 8250B2E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B2EC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B2F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B2F4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B2F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B2FC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B300: 419A0008  beq cr6, 0x8250b308
	if ctx.cr[6].eq {
	pc = 0x8250B308; continue 'dispatch;
	}
	// 8250B304: 4BDB558D  bl 0x822c0890
	ctx.lr = 0x8250B308;
	sub_822C0890(ctx, base);
	// 8250B308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B314: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B318: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B320 size=112
    let mut pc: u32 = 0x8250B320;
    'dispatch: loop {
        match pc {
            0x8250B320 => {
    //   block [0x8250B320..0x8250B390)
	// 8250B320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B32C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B334: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B33C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B340: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B344: 4BFFEA6D  bl 0x82509db0
	ctx.lr = 0x8250B348;
	sub_82509DB0(ctx, base);
	// 8250B348: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B34C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B350: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B354: 4BDB4CAD  bl 0x822c0000
	ctx.lr = 0x8250B358;
	sub_822C0000(ctx, base);
	// 8250B358: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B35C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B360: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B364: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B36C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B370: 419A0008  beq cr6, 0x8250b378
	if ctx.cr[6].eq {
	pc = 0x8250B378; continue 'dispatch;
	}
	// 8250B374: 4BDB551D  bl 0x822c0890
	ctx.lr = 0x8250B378;
	sub_822C0890(ctx, base);
	// 8250B378: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B384: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B390 size=112
    let mut pc: u32 = 0x8250B390;
    'dispatch: loop {
        match pc {
            0x8250B390 => {
    //   block [0x8250B390..0x8250B400)
	// 8250B390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B39C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B3A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B3A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B3A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B3AC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B3B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B3B4: 4BFFEAC5  bl 0x82509e78
	ctx.lr = 0x8250B3B8;
	sub_82509E78(ctx, base);
	// 8250B3B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B3BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B3C0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B3C4: 4BDB4C3D  bl 0x822c0000
	ctx.lr = 0x8250B3C8;
	sub_822C0000(ctx, base);
	// 8250B3C8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B3CC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B3D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B3D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B3D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B3DC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B3E0: 419A0008  beq cr6, 0x8250b3e8
	if ctx.cr[6].eq {
	pc = 0x8250B3E8; continue 'dispatch;
	}
	// 8250B3E4: 4BDB54AD  bl 0x822c0890
	ctx.lr = 0x8250B3E8;
	sub_822C0890(ctx, base);
	// 8250B3E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B3F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B400 size=112
    let mut pc: u32 = 0x8250B400;
    'dispatch: loop {
        match pc {
            0x8250B400 => {
    //   block [0x8250B400..0x8250B470)
	// 8250B400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B40C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B414: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B418: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B41C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B420: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B424: 4BFFEB1D  bl 0x82509f40
	ctx.lr = 0x8250B428;
	sub_82509F40(ctx, base);
	// 8250B428: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B42C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B430: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B434: 4BDB4BCD  bl 0x822c0000
	ctx.lr = 0x8250B438;
	sub_822C0000(ctx, base);
	// 8250B438: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B43C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B440: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B444: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B448: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B44C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B450: 419A0008  beq cr6, 0x8250b458
	if ctx.cr[6].eq {
	pc = 0x8250B458; continue 'dispatch;
	}
	// 8250B454: 4BDB543D  bl 0x822c0890
	ctx.lr = 0x8250B458;
	sub_822C0890(ctx, base);
	// 8250B458: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B464: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B468: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B470 size=112
    let mut pc: u32 = 0x8250B470;
    'dispatch: loop {
        match pc {
            0x8250B470 => {
    //   block [0x8250B470..0x8250B4E0)
	// 8250B470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B478: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B47C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B484: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B48C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B490: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B494: 4BFFEB75  bl 0x8250a008
	ctx.lr = 0x8250B498;
	sub_8250A008(ctx, base);
	// 8250B498: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B49C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B4A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B4A4: 4BDB4B5D  bl 0x822c0000
	ctx.lr = 0x8250B4A8;
	sub_822C0000(ctx, base);
	// 8250B4A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B4AC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B4B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B4B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B4B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B4BC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B4C0: 419A0008  beq cr6, 0x8250b4c8
	if ctx.cr[6].eq {
	pc = 0x8250B4C8; continue 'dispatch;
	}
	// 8250B4C4: 4BDB53CD  bl 0x822c0890
	ctx.lr = 0x8250B4C8;
	sub_822C0890(ctx, base);
	// 8250B4C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B4D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B4D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B4E0 size=112
    let mut pc: u32 = 0x8250B4E0;
    'dispatch: loop {
        match pc {
            0x8250B4E0 => {
    //   block [0x8250B4E0..0x8250B550)
	// 8250B4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B4E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B4EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B4F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B4F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B4F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B4FC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B500: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B504: 4BFFEBCD  bl 0x8250a0d0
	ctx.lr = 0x8250B508;
	sub_8250A0D0(ctx, base);
	// 8250B508: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B50C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B510: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B514: 4BDB4AED  bl 0x822c0000
	ctx.lr = 0x8250B518;
	sub_822C0000(ctx, base);
	// 8250B518: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B51C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B520: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B524: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B528: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B52C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B530: 419A0008  beq cr6, 0x8250b538
	if ctx.cr[6].eq {
	pc = 0x8250B538; continue 'dispatch;
	}
	// 8250B534: 4BDB535D  bl 0x822c0890
	ctx.lr = 0x8250B538;
	sub_822C0890(ctx, base);
	// 8250B538: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B544: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B550 size=112
    let mut pc: u32 = 0x8250B550;
    'dispatch: loop {
        match pc {
            0x8250B550 => {
    //   block [0x8250B550..0x8250B5C0)
	// 8250B550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B55C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B564: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B56C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250B570: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B574: 4BFFECD5  bl 0x8250a248
	ctx.lr = 0x8250B578;
	sub_8250A248(ctx, base);
	// 8250B578: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250B57C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250B580: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250B584: 4BDB4A7D  bl 0x822c0000
	ctx.lr = 0x8250B588;
	sub_822C0000(ctx, base);
	// 8250B588: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B58C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250B590: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250B594: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B598: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B59C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250B5A0: 419A0008  beq cr6, 0x8250b5a8
	if ctx.cr[6].eq {
	pc = 0x8250B5A8; continue 'dispatch;
	}
	// 8250B5A4: 4BDB52ED  bl 0x822c0890
	ctx.lr = 0x8250B5A8;
	sub_822C0890(ctx, base);
	// 8250B5A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B5B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B5B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250B5C0 size=140
    let mut pc: u32 = 0x8250B5C0;
    'dispatch: loop {
        match pc {
            0x8250B5C0 => {
    //   block [0x8250B5C0..0x8250B64C)
	// 8250B5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B5C4: 48C9CBA9  bl 0x831a816c
	ctx.lr = 0x8250B5C8;
	sub_831A8130(ctx, base);
	// 8250B5C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B5CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B5D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250B5D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250B5D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250B5DC: 388B19D8  addi r4, r11, 0x19d8
	ctx.r[4].s64 = ctx.r[11].s64 + 6616;
	// 8250B5E0: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 8250B5E4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8250B5E8: 4BDB4DF1  bl 0x822c03d8
	ctx.lr = 0x8250B5EC;
	sub_822C03D8(ctx, base);
	// 8250B5EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250B5F0: 41820028  beq 0x8250b618
	if ctx.cr[0].eq {
	pc = 0x8250B618; continue 'dispatch;
	}
	// 8250B5F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B5F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8250B5FC: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250B600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B604: 394A1990  addi r10, r10, 0x1990
	ctx.r[10].s64 = ctx.r[10].s64 + 6544;
	// 8250B608: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8250B60C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250B610: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250B614: 48000008  b 0x8250b61c
	pc = 0x8250B61C; continue 'dispatch;
	// 8250B618: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250B61C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8250B620: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8250B624: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B62C: 4BFFEB6D  bl 0x8250a198
	ctx.lr = 0x8250B630;
	sub_8250A198(ctx, base);
	// 8250B630: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250B634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B638: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B63C: 4BDB49C5  bl 0x822c0000
	ctx.lr = 0x8250B640;
	sub_822C0000(ctx, base);
	// 8250B640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250B644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B648: 48C9CB74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B650 size=124
    let mut pc: u32 = 0x8250B650;
    'dispatch: loop {
        match pc {
            0x8250B650 => {
    //   block [0x8250B650..0x8250B6CC)
	// 8250B650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250B65C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B664: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B66C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250B670: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B674: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 8250B678: 482EB151  bl 0x827f67c8
	ctx.lr = 0x8250B67C;
	sub_827F67C8(ctx, base);
	// 8250B67C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B680: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B684: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B688: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250B68C: 419A0014  beq cr6, 0x8250b6a0
	if ctx.cr[6].eq {
	pc = 0x8250B6A0; continue 'dispatch;
	}
	// 8250B690: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B694: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B698: 388B0094  addi r4, r11, 0x94
	ctx.r[4].s64 = ctx.r[11].s64 + 148;
	// 8250B69C: 409A0008  bne cr6, 0x8250b6a4
	if !ctx.cr[6].eq {
	pc = 0x8250B6A4; continue 'dispatch;
	}
	// 8250B6A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250B6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250B6A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250B6AC: 488E654D  bl 0x82df1bf8
	ctx.lr = 0x8250B6B0;
	sub_82DF1BF8(ctx, base);
	// 8250B6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250B6B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250B6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B6C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250B6C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B6D0 size=2488
    let mut pc: u32 = 0x8250B6D0;
    'dispatch: loop {
        match pc {
            0x8250B6D0 => {
    //   block [0x8250B6D0..0x8250C088)
	// 8250B6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B6D4: 48C9CA89  bl 0x831a815c
	ctx.lr = 0x8250B6D8;
	sub_831A8130(ctx, base);
	// 8250B6D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B6DC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8250B6E0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8250B6E4: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 8250B6E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250B6EC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 8250B6F0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8250B6F4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8250B6F8: 93610074  stw r27, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
	// 8250B6FC: 4BFE7ADD  bl 0x824f31d8
	ctx.lr = 0x8250B700;
	sub_824F31D8(ctx, base);
	// 8250B700: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250B704: 4182097C  beq 0x8250c080
	if ctx.cr[0].eq {
	pc = 0x8250C080; continue 'dispatch;
	}
	// 8250B708: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B70C: 93610078  stw r27, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[27].u32 ) };
	// 8250B710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B714: 9361007C  stw r27, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[27].u32 ) };
	// 8250B718: 388B1AE0  addi r4, r11, 0x1ae0
	ctx.r[4].s64 = ctx.r[11].s64 + 6880;
	// 8250B71C: 488E88AD  bl 0x82df3fc8
	ctx.lr = 0x8250B720;
	sub_82DF3FC8(ctx, base);
	// 8250B720: 3FA08212  lis r29, -0x7dee
	ctx.r[29].s64 = -2112749568;
	// 8250B724: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250B728: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250B72C: 419A0044  beq cr6, 0x8250b770
	if ctx.cr[6].eq {
	pc = 0x8250B770; continue 'dispatch;
	}
	// 8250B730: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250B738: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250B73C: 38A0061E  li r5, 0x61e
	ctx.r[5].s64 = 1566;
	// 8250B740: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250B744: 488E6CA5  bl 0x82df23e8
	ctx.lr = 0x8250B748;
	sub_82DF23E8(ctx, base);
	// 8250B748: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250B74C: 41820014  beq 0x8250b760
	if ctx.cr[0].eq {
	pc = 0x8250B760; continue 'dispatch;
	}
	// 8250B750: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250B754: 48632D15  bl 0x82b3e468
	ctx.lr = 0x8250B758;
	sub_82B3E468(ctx, base);
	// 8250B758: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B75C: 48000008  b 0x8250b764
	pc = 0x8250B764; continue 'dispatch;
	// 8250B760: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250B764: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250B768: 4BFFFAD9  bl 0x8250b240
	ctx.lr = 0x8250B76C;
	sub_8250B240(ctx, base);
	// 8250B76C: 480008A8  b 0x8250c014
	pc = 0x8250C014; continue 'dispatch;
	// 8250B770: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B778: 388B1AD8  addi r4, r11, 0x1ad8
	ctx.r[4].s64 = ctx.r[11].s64 + 6872;
	// 8250B77C: 488E884D  bl 0x82df3fc8
	ctx.lr = 0x8250B780;
	sub_82DF3FC8(ctx, base);
	// 8250B780: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250B784: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250B788: 419A009C  beq cr6, 0x8250b824
	if ctx.cr[6].eq {
	pc = 0x8250B824; continue 'dispatch;
	}
	// 8250B78C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B790: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250B794: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250B798: 38A00620  li r5, 0x620
	ctx.r[5].s64 = 1568;
	// 8250B79C: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250B7A0: 488E6C49  bl 0x82df23e8
	ctx.lr = 0x8250B7A4;
	sub_82DF23E8(ctx, base);
	// 8250B7A4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250B7A8: 4182004C  beq 0x8250b7f4
	if ctx.cr[0].eq {
	pc = 0x8250B7F4; continue 'dispatch;
	}
	// 8250B7AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250B7B0: 488E7A01  bl 0x82df31b0
	ctx.lr = 0x8250B7B4;
	sub_82DF31B0(ctx, base);
	// 8250B7B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B7B8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250B7BC: 488E824D  bl 0x82df3a08
	ctx.lr = 0x8250B7C0;
	sub_82DF3A08(ctx, base);
	// 8250B7C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B7C4: 488E79ED  bl 0x82df31b0
	ctx.lr = 0x8250B7C8;
	sub_82DF31B0(ctx, base);
	// 8250B7C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B7CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250B7D0: 488E8239  bl 0x82df3a08
	ctx.lr = 0x8250B7D4;
	sub_82DF3A08(ctx, base);
	// 8250B7D4: 38C10064  addi r6, r1, 0x64
	ctx.r[6].s64 = ctx.r[1].s64 + 100;
	// 8250B7D8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250B7DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250B7E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250B7E4: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8250B7E8: 48639B59  bl 0x82b45340
	ctx.lr = 0x8250B7EC;
	sub_82B45340(ctx, base);
	// 8250B7EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B7F0: 48000008  b 0x8250b7f8
	pc = 0x8250B7F8; continue 'dispatch;
	// 8250B7F4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250B7F8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250B7FC: 4BFFFAB5  bl 0x8250b2b0
	ctx.lr = 0x8250B800;
	sub_8250B2B0(ctx, base);
	// 8250B800: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250B804: 41820010  beq 0x8250b814
	if ctx.cr[0].eq {
	pc = 0x8250B814; continue 'dispatch;
	}
	// 8250B808: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250B80C: 57FF07FA  rlwinm r31, r31, 0, 0x1f, 0x1d
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250B810: 488E7C19  bl 0x82df3428
	ctx.lr = 0x8250B814;
	sub_82DF3428(ctx, base);
	// 8250B814: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250B818: 418207FC  beq 0x8250c014
	if ctx.cr[0].eq {
	pc = 0x8250C014; continue 'dispatch;
	}
	// 8250B81C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250B820: 480007F0  b 0x8250c010
	pc = 0x8250C010; continue 'dispatch;
	// 8250B824: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B82C: 388B1AD0  addi r4, r11, 0x1ad0
	ctx.r[4].s64 = ctx.r[11].s64 + 6864;
	// 8250B830: 488E8799  bl 0x82df3fc8
	ctx.lr = 0x8250B834;
	sub_82DF3FC8(ctx, base);
	// 8250B834: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250B838: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250B83C: 419A00EC  beq cr6, 0x8250b928
	if ctx.cr[6].eq {
	pc = 0x8250B928; continue 'dispatch;
	}
	// 8250B840: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250B848: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250B84C: 38A00622  li r5, 0x622
	ctx.r[5].s64 = 1570;
	// 8250B850: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250B854: 488E6B95  bl 0x82df23e8
	ctx.lr = 0x8250B858;
	sub_82DF23E8(ctx, base);
	// 8250B858: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250B85C: 41820074  beq 0x8250b8d0
	if ctx.cr[0].eq {
	pc = 0x8250B8D0; continue 'dispatch;
	}
	// 8250B860: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B864: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250B868: 388B1AC4  addi r4, r11, 0x1ac4
	ctx.r[4].s64 = ctx.r[11].s64 + 6852;
	// 8250B86C: 488E819D  bl 0x82df3a08
	ctx.lr = 0x8250B870;
	sub_82DF3A08(ctx, base);
	// 8250B870: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B874: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250B878: 388B1AB4  addi r4, r11, 0x1ab4
	ctx.r[4].s64 = ctx.r[11].s64 + 6836;
	// 8250B87C: 488E818D  bl 0x82df3a08
	ctx.lr = 0x8250B880;
	sub_82DF3A08(ctx, base);
	// 8250B880: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250B884: 488E792D  bl 0x82df31b0
	ctx.lr = 0x8250B888;
	sub_82DF31B0(ctx, base);
	// 8250B888: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B88C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250B890: 488E8179  bl 0x82df3a08
	ctx.lr = 0x8250B894;
	sub_82DF3A08(ctx, base);
	// 8250B894: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B898: 488E7919  bl 0x82df31b0
	ctx.lr = 0x8250B89C;
	sub_82DF31B0(ctx, base);
	// 8250B89C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B8A0: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250B8A4: 488E8165  bl 0x82df3a08
	ctx.lr = 0x8250B8A8;
	sub_82DF3A08(ctx, base);
	// 8250B8A8: 3901006C  addi r8, r1, 0x6c
	ctx.r[8].s64 = ctx.r[1].s64 + 108;
	// 8250B8AC: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 8250B8B0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8250B8B4: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 8250B8B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250B8BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250B8C0: 3BE0003C  li r31, 0x3c
	ctx.r[31].s64 = 60;
	// 8250B8C4: 48638D15  bl 0x82b445d8
	ctx.lr = 0x8250B8C8;
	sub_82B445D8(ctx, base);
	// 8250B8C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B8CC: 48000008  b 0x8250b8d4
	pc = 0x8250B8D4; continue 'dispatch;
	// 8250B8D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250B8D4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250B8D8: 4BFFFA49  bl 0x8250b320
	ctx.lr = 0x8250B8DC;
	sub_8250B320(ctx, base);
	// 8250B8DC: 57EB06B5  rlwinm. r11, r31, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250B8E0: 41820010  beq 0x8250b8f0
	if ctx.cr[0].eq {
	pc = 0x8250B8F0; continue 'dispatch;
	}
	// 8250B8E4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250B8E8: 57FF06F2  rlwinm r31, r31, 0, 0x1b, 0x19
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250B8EC: 488E7B3D  bl 0x82df3428
	ctx.lr = 0x8250B8F0;
	sub_82DF3428(ctx, base);
	// 8250B8F0: 57EB06F7  rlwinm. r11, r31, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250B8F4: 41820010  beq 0x8250b904
	if ctx.cr[0].eq {
	pc = 0x8250B904; continue 'dispatch;
	}
	// 8250B8F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250B8FC: 57FF0734  rlwinm r31, r31, 0, 0x1c, 0x1a
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250B900: 488E7B29  bl 0x82df3428
	ctx.lr = 0x8250B904;
	sub_82DF3428(ctx, base);
	// 8250B904: 57EB0739  rlwinm. r11, r31, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250B908: 41820010  beq 0x8250b918
	if ctx.cr[0].eq {
	pc = 0x8250B918; continue 'dispatch;
	}
	// 8250B90C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250B910: 57FF0776  rlwinm r31, r31, 0, 0x1d, 0x1b
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250B914: 488E7B15  bl 0x82df3428
	ctx.lr = 0x8250B918;
	sub_82DF3428(ctx, base);
	// 8250B918: 57EB077B  rlwinm. r11, r31, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250B91C: 418206F8  beq 0x8250c014
	if ctx.cr[0].eq {
	pc = 0x8250C014; continue 'dispatch;
	}
	// 8250B920: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250B924: 480006EC  b 0x8250c010
	pc = 0x8250C010; continue 'dispatch;
	// 8250B928: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B92C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B930: 388B1AA4  addi r4, r11, 0x1aa4
	ctx.r[4].s64 = ctx.r[11].s64 + 6820;
	// 8250B934: 488E8695  bl 0x82df3fc8
	ctx.lr = 0x8250B938;
	sub_82DF3FC8(ctx, base);
	// 8250B938: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250B93C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250B940: 419A013C  beq cr6, 0x8250ba7c
	if ctx.cr[6].eq {
	pc = 0x8250BA7C; continue 'dispatch;
	}
	// 8250B944: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B948: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250B94C: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250B950: 38A00625  li r5, 0x625
	ctx.r[5].s64 = 1573;
	// 8250B954: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250B958: 488E6A91  bl 0x82df23e8
	ctx.lr = 0x8250B95C;
	sub_82DF23E8(ctx, base);
	// 8250B95C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250B960: 4182009C  beq 0x8250b9fc
	if ctx.cr[0].eq {
	pc = 0x8250B9FC; continue 'dispatch;
	}
	// 8250B964: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B968: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250B96C: 388B1A98  addi r4, r11, 0x1a98
	ctx.r[4].s64 = ctx.r[11].s64 + 6808;
	// 8250B970: 488E8099  bl 0x82df3a08
	ctx.lr = 0x8250B974;
	sub_82DF3A08(ctx, base);
	// 8250B974: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B978: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250B97C: 388B1A88  addi r4, r11, 0x1a88
	ctx.r[4].s64 = ctx.r[11].s64 + 6792;
	// 8250B980: 488E8089  bl 0x82df3a08
	ctx.lr = 0x8250B984;
	sub_82DF3A08(ctx, base);
	// 8250B984: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B988: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250B98C: 388B1A7C  addi r4, r11, 0x1a7c
	ctx.r[4].s64 = ctx.r[11].s64 + 6780;
	// 8250B990: 488E8079  bl 0x82df3a08
	ctx.lr = 0x8250B994;
	sub_82DF3A08(ctx, base);
	// 8250B994: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250B998: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250B99C: 388B1A68  addi r4, r11, 0x1a68
	ctx.r[4].s64 = ctx.r[11].s64 + 6760;
	// 8250B9A0: 488E8069  bl 0x82df3a08
	ctx.lr = 0x8250B9A4;
	sub_82DF3A08(ctx, base);
	// 8250B9A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250B9A8: 488E7809  bl 0x82df31b0
	ctx.lr = 0x8250B9AC;
	sub_82DF31B0(ctx, base);
	// 8250B9AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B9B0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250B9B4: 488E8055  bl 0x82df3a08
	ctx.lr = 0x8250B9B8;
	sub_82DF3A08(ctx, base);
	// 8250B9B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B9BC: 488E77F5  bl 0x82df31b0
	ctx.lr = 0x8250B9C0;
	sub_82DF31B0(ctx, base);
	// 8250B9C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B9C4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250B9C8: 488E8041  bl 0x82df3a08
	ctx.lr = 0x8250B9CC;
	sub_82DF3A08(ctx, base);
	// 8250B9CC: 39410074  addi r10, r1, 0x74
	ctx.r[10].s64 = ctx.r[1].s64 + 116;
	// 8250B9D0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8250B9D4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8250B9D8: 38E10064  addi r7, r1, 0x64
	ctx.r[7].s64 = ctx.r[1].s64 + 100;
	// 8250B9DC: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 8250B9E0: 38A1006C  addi r5, r1, 0x6c
	ctx.r[5].s64 = ctx.r[1].s64 + 108;
	// 8250B9E4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250B9E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250B9EC: 3BE00FC0  li r31, 0xfc0
	ctx.r[31].s64 = 4032;
	// 8250B9F0: 48636DE9  bl 0x82b427d8
	ctx.lr = 0x8250B9F4;
	sub_82B427D8(ctx, base);
	// 8250B9F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B9F8: 48000008  b 0x8250ba00
	pc = 0x8250BA00; continue 'dispatch;
	// 8250B9FC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250BA00: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250BA04: 4BFFF98D  bl 0x8250b390
	ctx.lr = 0x8250BA08;
	sub_8250B390(ctx, base);
	// 8250BA08: 57EB0529  rlwinm. r11, r31, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BA0C: 41820010  beq 0x8250ba1c
	if ctx.cr[0].eq {
	pc = 0x8250BA1C; continue 'dispatch;
	}
	// 8250BA10: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BA14: 57FF0566  rlwinm r31, r31, 0, 0x15, 0x13
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BA18: 488E7A11  bl 0x82df3428
	ctx.lr = 0x8250BA1C;
	sub_82DF3428(ctx, base);
	// 8250BA1C: 57EB056B  rlwinm. r11, r31, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BA20: 41820010  beq 0x8250ba30
	if ctx.cr[0].eq {
	pc = 0x8250BA30; continue 'dispatch;
	}
	// 8250BA24: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BA28: 57FF05A8  rlwinm r31, r31, 0, 0x16, 0x14
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BA2C: 488E79FD  bl 0x82df3428
	ctx.lr = 0x8250BA30;
	sub_82DF3428(ctx, base);
	// 8250BA30: 57EB05AD  rlwinm. r11, r31, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BA34: 41820010  beq 0x8250ba44
	if ctx.cr[0].eq {
	pc = 0x8250BA44; continue 'dispatch;
	}
	// 8250BA38: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BA3C: 57FF05EA  rlwinm r31, r31, 0, 0x17, 0x15
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BA40: 488E79E9  bl 0x82df3428
	ctx.lr = 0x8250BA44;
	sub_82DF3428(ctx, base);
	// 8250BA44: 57EB05EF  rlwinm. r11, r31, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BA48: 41820010  beq 0x8250ba58
	if ctx.cr[0].eq {
	pc = 0x8250BA58; continue 'dispatch;
	}
	// 8250BA4C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250BA50: 57FF062C  rlwinm r31, r31, 0, 0x18, 0x16
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BA54: 488E79D5  bl 0x82df3428
	ctx.lr = 0x8250BA58;
	sub_82DF3428(ctx, base);
	// 8250BA58: 57EB0631  rlwinm. r11, r31, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BA5C: 41820010  beq 0x8250ba6c
	if ctx.cr[0].eq {
	pc = 0x8250BA6C; continue 'dispatch;
	}
	// 8250BA60: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BA64: 57FF066E  rlwinm r31, r31, 0, 0x19, 0x17
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BA68: 488E79C1  bl 0x82df3428
	ctx.lr = 0x8250BA6C;
	sub_82DF3428(ctx, base);
	// 8250BA6C: 57EB0673  rlwinm. r11, r31, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BA70: 418205A4  beq 0x8250c014
	if ctx.cr[0].eq {
	pc = 0x8250C014; continue 'dispatch;
	}
	// 8250BA74: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BA78: 48000598  b 0x8250c010
	pc = 0x8250C010; continue 'dispatch;
	// 8250BA7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BA80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BA84: 388B1A60  addi r4, r11, 0x1a60
	ctx.r[4].s64 = ctx.r[11].s64 + 6752;
	// 8250BA88: 488E8541  bl 0x82df3fc8
	ctx.lr = 0x8250BA8C;
	sub_82DF3FC8(ctx, base);
	// 8250BA8C: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250BA90: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250BA94: 419A0068  beq cr6, 0x8250bafc
	if ctx.cr[6].eq {
	pc = 0x8250BAFC; continue 'dispatch;
	}
	// 8250BA98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250BAA0: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250BAA4: 38A00627  li r5, 0x627
	ctx.r[5].s64 = 1575;
	// 8250BAA8: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250BAAC: 488E693D  bl 0x82df23e8
	ctx.lr = 0x8250BAB0;
	sub_82DF23E8(ctx, base);
	// 8250BAB0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250BAB4: 41820034  beq 0x8250bae8
	if ctx.cr[0].eq {
	pc = 0x8250BAE8; continue 'dispatch;
	}
	// 8250BAB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BABC: 488E76F5  bl 0x82df31b0
	ctx.lr = 0x8250BAC0;
	sub_82DF31B0(ctx, base);
	// 8250BAC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BAC4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BAC8: 488E7F41  bl 0x82df3a08
	ctx.lr = 0x8250BACC;
	sub_82DF3A08(ctx, base);
	// 8250BACC: 38A10074  addi r5, r1, 0x74
	ctx.r[5].s64 = ctx.r[1].s64 + 116;
	// 8250BAD0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250BAD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250BAD8: 3BE01000  li r31, 0x1000
	ctx.r[31].s64 = 4096;
	// 8250BADC: 48635E35  bl 0x82b41910
	ctx.lr = 0x8250BAE0;
	sub_82B41910(ctx, base);
	// 8250BAE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BAE4: 48000008  b 0x8250baec
	pc = 0x8250BAEC; continue 'dispatch;
	// 8250BAE8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250BAEC: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250BAF0: 4BFFF911  bl 0x8250b400
	ctx.lr = 0x8250BAF4;
	sub_8250B400(ctx, base);
	// 8250BAF4: 57EB04E7  rlwinm. r11, r31, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BAF8: 4BFFFF78  b 0x8250ba70
	pc = 0x8250BA70; continue 'dispatch;
	// 8250BAFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BB00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BB04: 388B1A58  addi r4, r11, 0x1a58
	ctx.r[4].s64 = ctx.r[11].s64 + 6744;
	// 8250BB08: 488E84C1  bl 0x82df3fc8
	ctx.lr = 0x8250BB0C;
	sub_82DF3FC8(ctx, base);
	// 8250BB0C: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250BB10: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250BB14: 419A0134  beq cr6, 0x8250bc48
	if ctx.cr[6].eq {
	pc = 0x8250BC48; continue 'dispatch;
	}
	// 8250BB18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BB1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250BB20: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250BB24: 38A00629  li r5, 0x629
	ctx.r[5].s64 = 1577;
	// 8250BB28: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250BB2C: 488E68BD  bl 0x82df23e8
	ctx.lr = 0x8250BB30;
	sub_82DF23E8(ctx, base);
	// 8250BB30: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250BB34: 4182009C  beq 0x8250bbd0
	if ctx.cr[0].eq {
	pc = 0x8250BBD0; continue 'dispatch;
	}
	// 8250BB38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BB3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250BB40: 388B1A98  addi r4, r11, 0x1a98
	ctx.r[4].s64 = ctx.r[11].s64 + 6808;
	// 8250BB44: 488E7EC5  bl 0x82df3a08
	ctx.lr = 0x8250BB48;
	sub_82DF3A08(ctx, base);
	// 8250BB48: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BB4C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BB50: 388B1A88  addi r4, r11, 0x1a88
	ctx.r[4].s64 = ctx.r[11].s64 + 6792;
	// 8250BB54: 488E7EB5  bl 0x82df3a08
	ctx.lr = 0x8250BB58;
	sub_82DF3A08(ctx, base);
	// 8250BB58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BB5C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BB60: 388B1A7C  addi r4, r11, 0x1a7c
	ctx.r[4].s64 = ctx.r[11].s64 + 6780;
	// 8250BB64: 488E7EA5  bl 0x82df3a08
	ctx.lr = 0x8250BB68;
	sub_82DF3A08(ctx, base);
	// 8250BB68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BB6C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BB70: 388B1A68  addi r4, r11, 0x1a68
	ctx.r[4].s64 = ctx.r[11].s64 + 6760;
	// 8250BB74: 488E7E95  bl 0x82df3a08
	ctx.lr = 0x8250BB78;
	sub_82DF3A08(ctx, base);
	// 8250BB78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BB7C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BB80: 388B1A48  addi r4, r11, 0x1a48
	ctx.r[4].s64 = ctx.r[11].s64 + 6728;
	// 8250BB84: 488E7E85  bl 0x82df3a08
	ctx.lr = 0x8250BB88;
	sub_82DF3A08(ctx, base);
	// 8250BB88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BB8C: 488E7625  bl 0x82df31b0
	ctx.lr = 0x8250BB90;
	sub_82DF31B0(ctx, base);
	// 8250BB90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BB94: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BB98: 488E7E71  bl 0x82df3a08
	ctx.lr = 0x8250BB9C;
	sub_82DF3A08(ctx, base);
	// 8250BB9C: 3FE00007  lis r31, 7
	ctx.r[31].s64 = 458752;
	// 8250BBA0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8250BBA4: 39210064  addi r9, r1, 0x64
	ctx.r[9].s64 = ctx.r[1].s64 + 100;
	// 8250BBA8: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 8250BBAC: 38E1006C  addi r7, r1, 0x6c
	ctx.r[7].s64 = ctx.r[1].s64 + 108;
	// 8250BBB0: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8250BBB4: 38A10074  addi r5, r1, 0x74
	ctx.r[5].s64 = ctx.r[1].s64 + 116;
	// 8250BBB8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250BBBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250BBC0: 63FFE000  ori r31, r31, 0xe000
	ctx.r[31].u64 = ctx.r[31].u64 | 57344;
	// 8250BBC4: 486363ED  bl 0x82b41fb0
	ctx.lr = 0x8250BBC8;
	sub_82B41FB0(ctx, base);
	// 8250BBC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BBCC: 48000008  b 0x8250bbd4
	pc = 0x8250BBD4; continue 'dispatch;
	// 8250BBD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250BBD4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250BBD8: 4BFFF899  bl 0x8250b470
	ctx.lr = 0x8250BBDC;
	sub_8250B470(ctx, base);
	// 8250BBDC: 57EB035B  rlwinm. r11, r31, 0, 0xd, 0xd
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BBE0: 41820010  beq 0x8250bbf0
	if ctx.cr[0].eq {
	pc = 0x8250BBF0; continue 'dispatch;
	}
	// 8250BBE4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BBE8: 57FF0398  rlwinm r31, r31, 0, 0xe, 0xc
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BBEC: 488E783D  bl 0x82df3428
	ctx.lr = 0x8250BBF0;
	sub_82DF3428(ctx, base);
	// 8250BBF0: 57EB039D  rlwinm. r11, r31, 0, 0xe, 0xe
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BBF4: 41820010  beq 0x8250bc04
	if ctx.cr[0].eq {
	pc = 0x8250BC04; continue 'dispatch;
	}
	// 8250BBF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BBFC: 57FF03DA  rlwinm r31, r31, 0, 0xf, 0xd
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BC00: 488E7829  bl 0x82df3428
	ctx.lr = 0x8250BC04;
	sub_82DF3428(ctx, base);
	// 8250BC04: 57EB03DF  rlwinm. r11, r31, 0, 0xf, 0xf
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BC08: 41820010  beq 0x8250bc18
	if ctx.cr[0].eq {
	pc = 0x8250BC18; continue 'dispatch;
	}
	// 8250BC0C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BC10: 57FF041C  rlwinm r31, r31, 0, 0x10, 0xe
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BC14: 488E7815  bl 0x82df3428
	ctx.lr = 0x8250BC18;
	sub_82DF3428(ctx, base);
	// 8250BC18: 57EB0421  rlwinm. r11, r31, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BC1C: 41820010  beq 0x8250bc2c
	if ctx.cr[0].eq {
	pc = 0x8250BC2C; continue 'dispatch;
	}
	// 8250BC20: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BC24: 57FF045E  rlwinm r31, r31, 0, 0x11, 0xf
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BC28: 488E7801  bl 0x82df3428
	ctx.lr = 0x8250BC2C;
	sub_82DF3428(ctx, base);
	// 8250BC2C: 57EB0463  rlwinm. r11, r31, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BC30: 41820010  beq 0x8250bc40
	if ctx.cr[0].eq {
	pc = 0x8250BC40; continue 'dispatch;
	}
	// 8250BC34: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BC38: 57FF04A0  rlwinm r31, r31, 0, 0x12, 0x10
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BC3C: 488E77ED  bl 0x82df3428
	ctx.lr = 0x8250BC40;
	sub_82DF3428(ctx, base);
	// 8250BC40: 57EB04A5  rlwinm. r11, r31, 0, 0x12, 0x12
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BC44: 480003C4  b 0x8250c008
	pc = 0x8250C008; continue 'dispatch;
	// 8250BC48: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BC4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BC50: 388B1A40  addi r4, r11, 0x1a40
	ctx.r[4].s64 = ctx.r[11].s64 + 6720;
	// 8250BC54: 488E8375  bl 0x82df3fc8
	ctx.lr = 0x8250BC58;
	sub_82DF3FC8(ctx, base);
	// 8250BC58: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250BC5C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250BC60: 419A013C  beq cr6, 0x8250bd9c
	if ctx.cr[6].eq {
	pc = 0x8250BD9C; continue 'dispatch;
	}
	// 8250BC64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BC68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250BC6C: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250BC70: 38A0062B  li r5, 0x62b
	ctx.r[5].s64 = 1579;
	// 8250BC74: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250BC78: 488E6771  bl 0x82df23e8
	ctx.lr = 0x8250BC7C;
	sub_82DF23E8(ctx, base);
	// 8250BC7C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250BC80: 418200A4  beq 0x8250bd24
	if ctx.cr[0].eq {
	pc = 0x8250BD24; continue 'dispatch;
	}
	// 8250BC84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BC88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250BC8C: 388B1A98  addi r4, r11, 0x1a98
	ctx.r[4].s64 = ctx.r[11].s64 + 6808;
	// 8250BC90: 488E7D79  bl 0x82df3a08
	ctx.lr = 0x8250BC94;
	sub_82DF3A08(ctx, base);
	// 8250BC94: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BC98: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BC9C: 388B1A88  addi r4, r11, 0x1a88
	ctx.r[4].s64 = ctx.r[11].s64 + 6792;
	// 8250BCA0: 488E7D69  bl 0x82df3a08
	ctx.lr = 0x8250BCA4;
	sub_82DF3A08(ctx, base);
	// 8250BCA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BCA8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BCAC: 388B1A7C  addi r4, r11, 0x1a7c
	ctx.r[4].s64 = ctx.r[11].s64 + 6780;
	// 8250BCB0: 488E7D59  bl 0x82df3a08
	ctx.lr = 0x8250BCB4;
	sub_82DF3A08(ctx, base);
	// 8250BCB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BCB8: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BCBC: 388B1A28  addi r4, r11, 0x1a28
	ctx.r[4].s64 = ctx.r[11].s64 + 6696;
	// 8250BCC0: 488E7D49  bl 0x82df3a08
	ctx.lr = 0x8250BCC4;
	sub_82DF3A08(ctx, base);
	// 8250BCC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250BCC8: 488E74E9  bl 0x82df31b0
	ctx.lr = 0x8250BCCC;
	sub_82DF31B0(ctx, base);
	// 8250BCCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BCD0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BCD4: 488E7D35  bl 0x82df3a08
	ctx.lr = 0x8250BCD8;
	sub_82DF3A08(ctx, base);
	// 8250BCD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BCDC: 488E74D5  bl 0x82df31b0
	ctx.lr = 0x8250BCE0;
	sub_82DF31B0(ctx, base);
	// 8250BCE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BCE4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BCE8: 488E7D21  bl 0x82df3a08
	ctx.lr = 0x8250BCEC;
	sub_82DF3A08(ctx, base);
	// 8250BCEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8250BCF0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8250BCF4: 39210064  addi r9, r1, 0x64
	ctx.r[9].s64 = ctx.r[1].s64 + 100;
	// 8250BCF8: 99610057  stb r11, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[11].u8 ) };
	// 8250BCFC: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 8250BD00: 38E1006C  addi r7, r1, 0x6c
	ctx.r[7].s64 = ctx.r[1].s64 + 108;
	// 8250BD04: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8250BD08: 38A10074  addi r5, r1, 0x74
	ctx.r[5].s64 = ctx.r[1].s64 + 116;
	// 8250BD0C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250BD10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250BD14: 3FE001F8  lis r31, 0x1f8
	ctx.r[31].s64 = 33030144;
	// 8250BD18: 486391B9  bl 0x82b44ed0
	ctx.lr = 0x8250BD1C;
	sub_82B44ED0(ctx, base);
	// 8250BD1C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BD20: 48000008  b 0x8250bd28
	pc = 0x8250BD28; continue 'dispatch;
	// 8250BD24: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250BD28: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250BD2C: 4BFFF7B5  bl 0x8250b4e0
	ctx.lr = 0x8250BD30;
	sub_8250B4E0(ctx, base);
	// 8250BD30: 57EB01CF  rlwinm. r11, r31, 0, 7, 7
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BD34: 41820010  beq 0x8250bd44
	if ctx.cr[0].eq {
	pc = 0x8250BD44; continue 'dispatch;
	}
	// 8250BD38: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BD3C: 57FF020C  rlwinm r31, r31, 0, 8, 6
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BD40: 488E76E9  bl 0x82df3428
	ctx.lr = 0x8250BD44;
	sub_82DF3428(ctx, base);
	// 8250BD44: 57EB0211  rlwinm. r11, r31, 0, 8, 8
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BD48: 41820010  beq 0x8250bd58
	if ctx.cr[0].eq {
	pc = 0x8250BD58; continue 'dispatch;
	}
	// 8250BD4C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BD50: 57FF024E  rlwinm r31, r31, 0, 9, 7
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BD54: 488E76D5  bl 0x82df3428
	ctx.lr = 0x8250BD58;
	sub_82DF3428(ctx, base);
	// 8250BD58: 57EB0253  rlwinm. r11, r31, 0, 9, 9
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BD5C: 41820010  beq 0x8250bd6c
	if ctx.cr[0].eq {
	pc = 0x8250BD6C; continue 'dispatch;
	}
	// 8250BD60: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BD64: 57FF0290  rlwinm r31, r31, 0, 0xa, 8
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BD68: 488E76C1  bl 0x82df3428
	ctx.lr = 0x8250BD6C;
	sub_82DF3428(ctx, base);
	// 8250BD6C: 57EB0295  rlwinm. r11, r31, 0, 0xa, 0xa
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BD70: 41820010  beq 0x8250bd80
	if ctx.cr[0].eq {
	pc = 0x8250BD80; continue 'dispatch;
	}
	// 8250BD74: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BD78: 57FF02D2  rlwinm r31, r31, 0, 0xb, 9
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BD7C: 488E76AD  bl 0x82df3428
	ctx.lr = 0x8250BD80;
	sub_82DF3428(ctx, base);
	// 8250BD80: 57EB02D7  rlwinm. r11, r31, 0, 0xb, 0xb
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BD84: 41820010  beq 0x8250bd94
	if ctx.cr[0].eq {
	pc = 0x8250BD94; continue 'dispatch;
	}
	// 8250BD88: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BD8C: 57FF0314  rlwinm r31, r31, 0, 0xc, 0xa
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BD90: 488E7699  bl 0x82df3428
	ctx.lr = 0x8250BD94;
	sub_82DF3428(ctx, base);
	// 8250BD94: 57EB0319  rlwinm. r11, r31, 0, 0xc, 0xc
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BD98: 48000270  b 0x8250c008
	pc = 0x8250C008; continue 'dispatch;
	// 8250BD9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BDA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BDA4: 388B1A14  addi r4, r11, 0x1a14
	ctx.r[4].s64 = ctx.r[11].s64 + 6676;
	// 8250BDA8: 488E8221  bl 0x82df3fc8
	ctx.lr = 0x8250BDAC;
	sub_82DF3FC8(ctx, base);
	// 8250BDAC: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 8250BDB0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250BDB4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250BDB8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BDBC: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 8250BDC0: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250BDC4: 419A0124  beq cr6, 0x8250bee8
	if ctx.cr[6].eq {
	pc = 0x8250BEE8; continue 'dispatch;
	}
	// 8250BDC8: 38A0062D  li r5, 0x62d
	ctx.r[5].s64 = 1581;
	// 8250BDCC: 488E661D  bl 0x82df23e8
	ctx.lr = 0x8250BDD0;
	sub_82DF23E8(ctx, base);
	// 8250BDD0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250BDD4: 4182009C  beq 0x8250be70
	if ctx.cr[0].eq {
	pc = 0x8250BE70; continue 'dispatch;
	}
	// 8250BDD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BDDC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250BDE0: 388B1A98  addi r4, r11, 0x1a98
	ctx.r[4].s64 = ctx.r[11].s64 + 6808;
	// 8250BDE4: 488E7C25  bl 0x82df3a08
	ctx.lr = 0x8250BDE8;
	sub_82DF3A08(ctx, base);
	// 8250BDE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BDEC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BDF0: 388B1A88  addi r4, r11, 0x1a88
	ctx.r[4].s64 = ctx.r[11].s64 + 6792;
	// 8250BDF4: 488E7C15  bl 0x82df3a08
	ctx.lr = 0x8250BDF8;
	sub_82DF3A08(ctx, base);
	// 8250BDF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BDFC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BE00: 388B1A7C  addi r4, r11, 0x1a7c
	ctx.r[4].s64 = ctx.r[11].s64 + 6780;
	// 8250BE04: 488E7C05  bl 0x82df3a08
	ctx.lr = 0x8250BE08;
	sub_82DF3A08(ctx, base);
	// 8250BE08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BE0C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BE10: 388B1A68  addi r4, r11, 0x1a68
	ctx.r[4].s64 = ctx.r[11].s64 + 6760;
	// 8250BE14: 488E7BF5  bl 0x82df3a08
	ctx.lr = 0x8250BE18;
	sub_82DF3A08(ctx, base);
	// 8250BE18: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250BE1C: 488E7395  bl 0x82df31b0
	ctx.lr = 0x8250BE20;
	sub_82DF31B0(ctx, base);
	// 8250BE20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BE24: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BE28: 488E7BE1  bl 0x82df3a08
	ctx.lr = 0x8250BE2C;
	sub_82DF3A08(ctx, base);
	// 8250BE2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BE30: 488E7381  bl 0x82df31b0
	ctx.lr = 0x8250BE34;
	sub_82DF31B0(ctx, base);
	// 8250BE34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BE38: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BE3C: 488E7BCD  bl 0x82df3a08
	ctx.lr = 0x8250BE40;
	sub_82DF3A08(ctx, base);
	// 8250BE40: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8250BE44: 39210064  addi r9, r1, 0x64
	ctx.r[9].s64 = ctx.r[1].s64 + 100;
	// 8250BE48: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 8250BE4C: 38E1006C  addi r7, r1, 0x6c
	ctx.r[7].s64 = ctx.r[1].s64 + 108;
	// 8250BE50: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8250BE54: 38A10074  addi r5, r1, 0x74
	ctx.r[5].s64 = ctx.r[1].s64 + 116;
	// 8250BE58: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250BE5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250BE60: 3FE07E00  lis r31, 0x7e00
	ctx.r[31].s64 = 2113929216;
	// 8250BE64: 48636975  bl 0x82b427d8
	ctx.lr = 0x8250BE68;
	sub_82B427D8(ctx, base);
	// 8250BE68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BE6C: 48000008  b 0x8250be74
	pc = 0x8250BE74; continue 'dispatch;
	// 8250BE70: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250BE74: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250BE78: 4BFFF519  bl 0x8250b390
	ctx.lr = 0x8250BE7C;
	sub_8250B390(ctx, base);
	// 8250BE7C: 57EB0043  rlwinm. r11, r31, 0, 1, 1
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BE80: 41820010  beq 0x8250be90
	if ctx.cr[0].eq {
	pc = 0x8250BE90; continue 'dispatch;
	}
	// 8250BE84: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BE88: 57FF0080  rlwinm r31, r31, 0, 2, 0
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BE8C: 488E759D  bl 0x82df3428
	ctx.lr = 0x8250BE90;
	sub_82DF3428(ctx, base);
	// 8250BE90: 57EB0085  rlwinm. r11, r31, 0, 2, 2
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BE94: 41820010  beq 0x8250bea4
	if ctx.cr[0].eq {
	pc = 0x8250BEA4; continue 'dispatch;
	}
	// 8250BE98: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BE9C: 57FF00C2  rlwinm r31, r31, 0, 3, 1
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BEA0: 488E7589  bl 0x82df3428
	ctx.lr = 0x8250BEA4;
	sub_82DF3428(ctx, base);
	// 8250BEA4: 57EB00C7  rlwinm. r11, r31, 0, 3, 3
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BEA8: 41820010  beq 0x8250beb8
	if ctx.cr[0].eq {
	pc = 0x8250BEB8; continue 'dispatch;
	}
	// 8250BEAC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BEB0: 57FF0104  rlwinm r31, r31, 0, 4, 2
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BEB4: 488E7575  bl 0x82df3428
	ctx.lr = 0x8250BEB8;
	sub_82DF3428(ctx, base);
	// 8250BEB8: 57EB0109  rlwinm. r11, r31, 0, 4, 4
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BEBC: 41820010  beq 0x8250becc
	if ctx.cr[0].eq {
	pc = 0x8250BECC; continue 'dispatch;
	}
	// 8250BEC0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BEC4: 57FF0146  rlwinm r31, r31, 0, 5, 3
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BEC8: 488E7561  bl 0x82df3428
	ctx.lr = 0x8250BECC;
	sub_82DF3428(ctx, base);
	// 8250BECC: 57EB014B  rlwinm. r11, r31, 0, 5, 5
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BED0: 41820010  beq 0x8250bee0
	if ctx.cr[0].eq {
	pc = 0x8250BEE0; continue 'dispatch;
	}
	// 8250BED4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BED8: 57FF0188  rlwinm r31, r31, 0, 6, 4
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BEDC: 488E754D  bl 0x82df3428
	ctx.lr = 0x8250BEE0;
	sub_82DF3428(ctx, base);
	// 8250BEE0: 57EB018D  rlwinm. r11, r31, 0, 6, 6
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BEE4: 48000124  b 0x8250c008
	pc = 0x8250C008; continue 'dispatch;
	// 8250BEE8: 38A0062F  li r5, 0x62f
	ctx.r[5].s64 = 1583;
	// 8250BEEC: 488E64FD  bl 0x82df23e8
	ctx.lr = 0x8250BEF0;
	sub_82DF23E8(ctx, base);
	// 8250BEF0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250BEF4: 418200A4  beq 0x8250bf98
	if ctx.cr[0].eq {
	pc = 0x8250BF98; continue 'dispatch;
	}
	// 8250BEF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BEFC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250BF00: 388B1A98  addi r4, r11, 0x1a98
	ctx.r[4].s64 = ctx.r[11].s64 + 6808;
	// 8250BF04: 488E7B05  bl 0x82df3a08
	ctx.lr = 0x8250BF08;
	sub_82DF3A08(ctx, base);
	// 8250BF08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BF0C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250BF10: 388B1A88  addi r4, r11, 0x1a88
	ctx.r[4].s64 = ctx.r[11].s64 + 6792;
	// 8250BF14: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 8250BF18: 488E7AF1  bl 0x82df3a08
	ctx.lr = 0x8250BF1C;
	sub_82DF3A08(ctx, base);
	// 8250BF1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BF20: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BF24: 388B1A7C  addi r4, r11, 0x1a7c
	ctx.r[4].s64 = ctx.r[11].s64 + 6780;
	// 8250BF28: 488E7AE1  bl 0x82df3a08
	ctx.lr = 0x8250BF2C;
	sub_82DF3A08(ctx, base);
	// 8250BF2C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250BF30: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BF34: 388B1A68  addi r4, r11, 0x1a68
	ctx.r[4].s64 = ctx.r[11].s64 + 6760;
	// 8250BF38: 488E7AD1  bl 0x82df3a08
	ctx.lr = 0x8250BF3C;
	sub_82DF3A08(ctx, base);
	// 8250BF3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250BF40: 488E7271  bl 0x82df31b0
	ctx.lr = 0x8250BF44;
	sub_82DF31B0(ctx, base);
	// 8250BF44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BF48: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BF4C: 488E7ABD  bl 0x82df3a08
	ctx.lr = 0x8250BF50;
	sub_82DF3A08(ctx, base);
	// 8250BF50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250BF54: 488E725D  bl 0x82df31b0
	ctx.lr = 0x8250BF58;
	sub_82DF31B0(ctx, base);
	// 8250BF58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BF5C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BF60: 488E7AA9  bl 0x82df3a08
	ctx.lr = 0x8250BF64;
	sub_82DF3A08(ctx, base);
	// 8250BF64: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8250BF68: 39210064  addi r9, r1, 0x64
	ctx.r[9].s64 = ctx.r[1].s64 + 100;
	// 8250BF6C: 9B610057  stb r27, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[27].u8 ) };
	// 8250BF70: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 8250BF74: 38E1006C  addi r7, r1, 0x6c
	ctx.r[7].s64 = ctx.r[1].s64 + 108;
	// 8250BF78: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8250BF7C: 38A10074  addi r5, r1, 0x74
	ctx.r[5].s64 = ctx.r[1].s64 + 116;
	// 8250BF80: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250BF84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250BF88: 3B60001F  li r27, 0x1f
	ctx.r[27].s64 = 31;
	// 8250BF8C: 48638F45  bl 0x82b44ed0
	ctx.lr = 0x8250BF90;
	sub_82B44ED0(ctx, base);
	// 8250BF90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250BF94: 48000008  b 0x8250bf9c
	pc = 0x8250BF9C; continue 'dispatch;
	// 8250BF98: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250BF9C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250BFA0: 4BFFF541  bl 0x8250b4e0
	ctx.lr = 0x8250BFA4;
	sub_8250B4E0(ctx, base);
	// 8250BFA4: 576B06F7  rlwinm. r11, r27, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BFA8: 41820010  beq 0x8250bfb8
	if ctx.cr[0].eq {
	pc = 0x8250BFB8; continue 'dispatch;
	}
	// 8250BFAC: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250BFB0: 577B0734  rlwinm r27, r27, 0, 0x1c, 0x1a
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BFB4: 488E7475  bl 0x82df3428
	ctx.lr = 0x8250BFB8;
	sub_82DF3428(ctx, base);
	// 8250BFB8: 576B0739  rlwinm. r11, r27, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BFBC: 41820010  beq 0x8250bfcc
	if ctx.cr[0].eq {
	pc = 0x8250BFCC; continue 'dispatch;
	}
	// 8250BFC0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250BFC4: 577B0776  rlwinm r27, r27, 0, 0x1d, 0x1b
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BFC8: 488E7461  bl 0x82df3428
	ctx.lr = 0x8250BFCC;
	sub_82DF3428(ctx, base);
	// 8250BFCC: 576B077B  rlwinm. r11, r27, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BFD0: 41820010  beq 0x8250bfe0
	if ctx.cr[0].eq {
	pc = 0x8250BFE0; continue 'dispatch;
	}
	// 8250BFD4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250BFD8: 577B07B8  rlwinm r27, r27, 0, 0x1e, 0x1c
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BFDC: 488E744D  bl 0x82df3428
	ctx.lr = 0x8250BFE0;
	sub_82DF3428(ctx, base);
	// 8250BFE0: 576B07BD  rlwinm. r11, r27, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BFE4: 41820010  beq 0x8250bff4
	if ctx.cr[0].eq {
	pc = 0x8250BFF4; continue 'dispatch;
	}
	// 8250BFE8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250BFEC: 577B07FA  rlwinm r27, r27, 0, 0x1f, 0x1d
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 8250BFF0: 488E7439  bl 0x82df3428
	ctx.lr = 0x8250BFF4;
	sub_82DF3428(ctx, base);
	// 8250BFF4: 576B07FF  clrlwi. r11, r27, 0x1f
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BFF8: 4182000C  beq 0x8250c004
	if ctx.cr[0].eq {
	pc = 0x8250C004; continue 'dispatch;
	}
	// 8250BFFC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250C000: 488E7429  bl 0x82df3428
	ctx.lr = 0x8250C004;
	sub_82DF3428(ctx, base);
	// 8250C004: 57EB0001  rlwinm. r11, r31, 0, 0, 0
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250C008: 4182000C  beq 0x8250c014
	if ctx.cr[0].eq {
	pc = 0x8250C014; continue 'dispatch;
	}
	// 8250C00C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250C010: 488E7419  bl 0x82df3428
	ctx.lr = 0x8250C014;
	sub_82DF3428(ctx, base);
	// 8250C014: 83E1007C  lwz r31, 0x7c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8250C018: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8250C01C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250C020: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 8250C024: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8250C028: 419A0024  beq cr6, 0x8250c04c
	if ctx.cr[6].eq {
	pc = 0x8250C04C; continue 'dispatch;
	}
	// 8250C02C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8250C030: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250C034: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250C038: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250C03C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250C040: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250C044: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250C048: 4082FFE8  bne 0x8250c030
	if !ctx.cr[0].eq {
	pc = 0x8250C030; continue 'dispatch;
	}
	// 8250C04C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C050: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8250C054: 806B00B0  lwz r3, 0xb0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8250C058: 4862FB21  bl 0x82b3bb78
	ctx.lr = 0x8250C05C;
	sub_82B3BB78(ctx, base);
	// 8250C05C: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250C060: 41820010  beq 0x8250c070
	if ctx.cr[0].eq {
	pc = 0x8250C070; continue 'dispatch;
	}
	// 8250C064: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C068: 806B00B0  lwz r3, 0xb0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8250C06C: 4862EF8D  bl 0x82b3aff8
	ctx.lr = 0x8250C070;
	sub_82B3AFF8(ctx, base);
	// 8250C070: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250C074: 419A000C  beq cr6, 0x8250c080
	if ctx.cr[6].eq {
	pc = 0x8250C080; continue 'dispatch;
	}
	// 8250C078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C07C: 4BDB4815  bl 0x822c0890
	ctx.lr = 0x8250C080;
	sub_822C0890(ctx, base);
	// 8250C080: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8250C084: 48C9C128  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250C088 size=488
    let mut pc: u32 = 0x8250C088;
    'dispatch: loop {
        match pc {
            0x8250C088 => {
    //   block [0x8250C088..0x8250C270)
	// 8250C088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C08C: 48C9C0D5  bl 0x831a8160
	ctx.lr = 0x8250C090;
	sub_831A8130(ctx, base);
	// 8250C090: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C094: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8250C098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250C09C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8250C0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C0A4: 388B059C  addi r4, r11, 0x59c
	ctx.r[4].s64 = ctx.r[11].s64 + 1436;
	// 8250C0A8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8250C0AC: 488E795D  bl 0x82df3a08
	ctx.lr = 0x8250C0B0;
	sub_82DF3A08(ctx, base);
	// 8250C0B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250C0B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250C0B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250C0BC: 4BFFF595  bl 0x8250b650
	ctx.lr = 0x8250C0C0;
	sub_8250B650(ctx, base);
	// 8250C0C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C0C4: 488E7365  bl 0x82df3428
	ctx.lr = 0x8250C0C8;
	sub_82DF3428(ctx, base);
	// 8250C0C8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250C0CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C0D0: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8250C0D4: 409A0008  bne cr6, 0x8250c0dc
	if !ctx.cr[6].eq {
	pc = 0x8250C0DC; continue 'dispatch;
	}
	// 8250C0D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250C0DC: 4801C5E5  bl 0x825286c0
	ctx.lr = 0x8250C0E0;
	sub_825286C0(ctx, base);
	// 8250C0E0: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8250C0E4: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 8250C0E8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250C0EC: 3BC00030  li r30, 0x30
	ctx.r[30].s64 = 48;
	// 8250C0F0: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8250C0F4: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 8250C0F8: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 8250C0FC: 390100B0  addi r8, r1, 0xb0
	ctx.r[8].s64 = ctx.r[1].s64 + 176;
	// 8250C100: 13DC1C07  vcmpneb. (lvlx128) v30, v28, v3
	tmp.u32 = ctx.r[28].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8250C104: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 8250C108: 13BD1C07  vcmpneb. (lvlx128) v29, v29, v3
	tmp.u32 = ctx.r[29].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8250C10C: 139E1C07  vcmpneb. (lvlx128) v28, v30, v3
	tmp.u32 = ctx.r[30].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8250C110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C270 size=104
    let mut pc: u32 = 0x8250C270;
    'dispatch: loop {
        match pc {
            0x8250C270 => {
    //   block [0x8250C270..0x8250C2D8)
	// 8250C270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250C278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250C27C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C280: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250C284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250C288: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250C28C: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250C290: 38A008C9  li r5, 0x8c9
	ctx.r[5].s64 = 2249;
	// 8250C294: 38600280  li r3, 0x280
	ctx.r[3].s64 = 640;
	// 8250C298: 488E6151  bl 0x82df23e8
	ctx.lr = 0x8250C29C;
	sub_82DF23E8(ctx, base);
	// 8250C29C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250C2A0: 41820014  beq 0x8250c2b4
	if ctx.cr[0].eq {
	pc = 0x8250C2B4; continue 'dispatch;
	}
	// 8250C2A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250C2A8: 4BF533E9  bl 0x8245f690
	ctx.lr = 0x8250C2AC;
	sub_8245F690(ctx, base);
	// 8250C2AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250C2B0: 48000008  b 0x8250c2b8
	pc = 0x8250C2B8; continue 'dispatch;
	// 8250C2B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250C2B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C2BC: 386B01C0  addi r3, r11, 0x1c0
	ctx.r[3].s64 = ctx.r[11].s64 + 448;
	// 8250C2C0: 4BFFF291  bl 0x8250b550
	ctx.lr = 0x8250C2C4;
	sub_8250B550(ctx, base);
	// 8250C2C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250C2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250C2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250C2D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C2D8 size=112
    let mut pc: u32 = 0x8250C2D8;
    'dispatch: loop {
        match pc {
            0x8250C2D8 => {
    //   block [0x8250C2D8..0x8250C348)
	// 8250C2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250C2E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250C2E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250C2E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C2EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250C2F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250C2F4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250C2F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250C2FC: 4BFFE7B5  bl 0x8250aab0
	ctx.lr = 0x8250C300;
	sub_8250AAB0(ctx, base);
	// 8250C300: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250C304: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250C308: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250C30C: 4BDB3CF5  bl 0x822c0000
	ctx.lr = 0x8250C310;
	sub_822C0000(ctx, base);
	// 8250C310: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250C314: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250C318: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250C31C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C320: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250C324: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250C328: 419A0008  beq cr6, 0x8250c330
	if ctx.cr[6].eq {
	pc = 0x8250C330; continue 'dispatch;
	}
	// 8250C32C: 4BDB4565  bl 0x822c0890
	ctx.lr = 0x8250C330;
	sub_822C0890(ctx, base);
	// 8250C330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250C334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250C338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250C33C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250C340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C348 size=232
    let mut pc: u32 = 0x8250C348;
    'dispatch: loop {
        match pc {
            0x8250C348 => {
    //   block [0x8250C348..0x8250C430)
	// 8250C348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250C350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250C354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250C358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250C360: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250C364: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250C368: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C36C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C370: 4891E779  bl 0x82e2aae8
	ctx.lr = 0x8250C374;
	sub_82E2AAE8(ctx, base);
	// 8250C374: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8250C378: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250C37C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8250C380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C384: 4892046D  bl 0x82e2c7f0
	ctx.lr = 0x8250C388;
	sub_82E2C7F0(ctx, base);
	// 8250C388: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250C38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250C390: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250C394: 38A005E1  li r5, 0x5e1
	ctx.r[5].s64 = 1505;
	// 8250C398: 3860011C  li r3, 0x11c
	ctx.r[3].s64 = 284;
	// 8250C39C: 4BDB403D  bl 0x822c03d8
	ctx.lr = 0x8250C3A0;
	sub_822C03D8(ctx, base);
	// 8250C3A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250C3A4: 4182004C  beq 0x8250c3f0
	if ctx.cr[0].eq {
	pc = 0x8250C3F0; continue 'dispatch;
	}
	// 8250C3A8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250C3AC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250C3B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C3B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8250C3B8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8250C3BC: 419A0024  beq cr6, 0x8250c3e0
	if ctx.cr[6].eq {
	pc = 0x8250C3E0; continue 'dispatch;
	}
	// 8250C3C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250C3C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250C3C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250C3CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250C3D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250C3D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250C3D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250C3DC: 4082FFE8  bne 0x8250c3c4
	if !ctx.cr[0].eq {
	pc = 0x8250C3C4; continue 'dispatch;
	}
	// 8250C3E0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8250C3E4: 48916AA5  bl 0x82e22e88
	ctx.lr = 0x8250C3E8;
	sub_82E22E88(ctx, base);
	// 8250C3E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250C3EC: 48000008  b 0x8250c3f4
	pc = 0x8250C3F4; continue 'dispatch;
	// 8250C3F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250C3F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C3F8: 386B0094  addi r3, r11, 0x94
	ctx.r[3].s64 = ctx.r[11].s64 + 148;
	// 8250C3FC: 4BFFFEDD  bl 0x8250c2d8
	ctx.lr = 0x8250C400;
	sub_8250C2D8(ctx, base);
	// 8250C400: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250C404: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250C408: 419A0008  beq cr6, 0x8250c410
	if ctx.cr[6].eq {
	pc = 0x8250C410; continue 'dispatch;
	}
	// 8250C40C: 4BDB4485  bl 0x822c0890
	ctx.lr = 0x8250C410;
	sub_822C0890(ctx, base);
	// 8250C410: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250C414: 4891E6ED  bl 0x82e2ab00
	ctx.lr = 0x8250C418;
	sub_82E2AB00(ctx, base);
	// 8250C418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250C41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250C420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250C424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250C428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C430 size=168
    let mut pc: u32 = 0x8250C430;
    'dispatch: loop {
        match pc {
            0x8250C430 => {
    //   block [0x8250C430..0x8250C4D8)
	// 8250C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250C438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250C43C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250C440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C444: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250C448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250C44C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C450: 816B0094  lwz r11, 0x94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 8250C454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C458: 409A0028  bne cr6, 0x8250c480
	if !ctx.cr[6].eq {
	pc = 0x8250C480; continue 'dispatch;
	}
	// 8250C45C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250C460: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C464: 388B1AEC  addi r4, r11, 0x1aec
	ctx.r[4].s64 = ctx.r[11].s64 + 6892;
	// 8250C468: 488E75A1  bl 0x82df3a08
	ctx.lr = 0x8250C46C;
	sub_82DF3A08(ctx, base);
	// 8250C46C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250C470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250C474: 4BFFFED5  bl 0x8250c348
	ctx.lr = 0x8250C478;
	sub_8250C348(ctx, base);
	// 8250C478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C47C: 488E6FAD  bl 0x82df3428
	ctx.lr = 0x8250C480;
	sub_82DF3428(ctx, base);
	// 8250C480: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C484: 814B0094  lwz r10, 0x94(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 8250C488: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250C48C: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8250C490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C494: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250C498: 419A0024  beq cr6, 0x8250c4bc
	if ctx.cr[6].eq {
	pc = 0x8250C4BC; continue 'dispatch;
	}
	// 8250C49C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250C4A0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250C4A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250C4A8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250C4AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250C4B0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250C4B4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250C4B8: 4082FFE8  bne 0x8250c4a0
	if !ctx.cr[0].eq {
	pc = 0x8250C4A0; continue 'dispatch;
	}
	// 8250C4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C4C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250C4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250C4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250C4CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250C4D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C4D8 size=48
    let mut pc: u32 = 0x8250C4D8;
    'dispatch: loop {
        match pc {
            0x8250C4D8 => {
    //   block [0x8250C4D8..0x8250C508)
	// 8250C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250C4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C4E4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C4E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C4EC: 80A40000  lwz r5, 0(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C4F0: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8250C4F4: 481B8C75  bl 0x826c5168
	ctx.lr = 0x8250C4F8;
	sub_826C5168(ctx, base);
	// 8250C4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250C4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250C500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250C504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C508 size=240
    let mut pc: u32 = 0x8250C508;
    'dispatch: loop {
        match pc {
            0x8250C508 => {
    //   block [0x8250C508..0x8250C5F8)
	// 8250C508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C50C: 48C9BC55  bl 0x831a8160
	ctx.lr = 0x8250C510;
	sub_831A8130(ctx, base);
	// 8250C510: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C514: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250C518: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8250C51C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8250C520: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C524: 834B0024  lwz r26, 0x24(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250C528: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C52C: 814B01B4  lwz r10, 0x1b4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(436 as u32) ) } as u64;
	// 8250C530: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250C534: 419A007C  beq cr6, 0x8250c5b0
	if ctx.cr[6].eq {
	pc = 0x8250C5B0; continue 'dispatch;
	}
	// 8250C538: 812B01B8  lwz r9, 0x1b8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8250C53C: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 8250C540: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 8250C544: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250C548: 40980068  bge cr6, 0x8250c5b0
	if !ctx.cr[6].lt {
	pc = 0x8250C5B0; continue 'dispatch;
	}
	// 8250C54C: 83AB01B4  lwz r29, 0x1b4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(436 as u32) ) } as u64;
	// 8250C550: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8250C554: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8250C558: 419A004C  beq cr6, 0x8250c5a4
	if ctx.cr[6].eq {
	pc = 0x8250C5A4; continue 'dispatch;
	}
	// 8250C55C: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C560: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250C564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C568: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C56C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C574: 4E800421  bctrl
	ctx.lr = 0x8250C578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C578: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8250C57C: 4894CFB5  bl 0x82e59530
	ctx.lr = 0x8250C580;
	sub_82E59530(ctx, base);
	// 8250C580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C584: 4800420D  bl 0x82510790
	ctx.lr = 0x8250C588;
	sub_82510790(ctx, base);
	// 8250C588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C58C: 4800433D  bl 0x825108c8
	ctx.lr = 0x8250C590;
	sub_825108C8(ctx, base);
	// 8250C590: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C598: 7CBDE02E  lwzx r5, r29, r28
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8250C59C: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8250C5A0: 481B8BC9  bl 0x826c5168
	ctx.lr = 0x8250C5A4;
	sub_826C5168(ctx, base);
	// 8250C5A4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8250C5A8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8250C5AC: 4BFFFF7C  b 0x8250c528
	pc = 0x8250C528; continue 'dispatch;
	// 8250C5B0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C5B4: 390B01B0  addi r8, r11, 0x1b0
	ctx.r[8].s64 = ctx.r[11].s64 + 432;
	// 8250C5B8: 812B01B4  lwz r9, 0x1b4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(436 as u32) ) } as u64;
	// 8250C5BC: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8250C5C0: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C5C4: 419A002C  beq cr6, 0x8250c5f0
	if ctx.cr[6].eq {
	pc = 0x8250C5F0; continue 'dispatch;
	}
	// 8250C5C8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8250C5CC: 7F0B5840  cmplw cr6, r11, r11
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C5D0: 419A001C  beq cr6, 0x8250c5ec
	if ctx.cr[6].eq {
	pc = 0x8250C5EC; continue 'dispatch;
	}
	// 8250C5D4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C5D8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250C5DC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C5E0: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8250C5E4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8250C5E8: 409AFFEC  bne cr6, 0x8250c5d4
	if !ctx.cr[6].eq {
	pc = 0x8250C5D4; continue 'dispatch;
	}
	// 8250C5EC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8250C5F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250C5F4: 48C9BBBC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C5F8 size=1044
    let mut pc: u32 = 0x8250C5F8;
    'dispatch: loop {
        match pc {
            0x8250C5F8 => {
    //   block [0x8250C5F8..0x8250CA0C)
	// 8250C5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C5FC: 48C9BB5D  bl 0x831a8158
	ctx.lr = 0x8250C600;
	sub_831A8130(ctx, base);
	// 8250C600: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C604: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8250C608: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8250C60C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8250C610: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8250C614: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C61C: 419A0048  beq cr6, 0x8250c664
	if ctx.cr[6].eq {
	pc = 0x8250C664; continue 'dispatch;
	}
	// 8250C620: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250C624: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C628: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8250C62C: 4BDB929D  bl 0x822c58c8
	ctx.lr = 0x8250C630;
	sub_822C58C8(ctx, base);
	// 8250C630: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250C634: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250C638: 4BDBD879  bl 0x822c9eb0
	ctx.lr = 0x8250C63C;
	sub_822C9EB0(ctx, base);
	// 8250C63C: 4BDB7C75  bl 0x822c42b0
	ctx.lr = 0x8250C640;
	sub_822C42B0(ctx, base);
	// 8250C640: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250C644: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250C648: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8250C64C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8250C650: 4BDB8E21  bl 0x822c5470
	ctx.lr = 0x8250C654;
	sub_822C5470(ctx, base);
	// 8250C654: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250C658: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250C65C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250C660: 4BDB8681  bl 0x822c4ce0
	ctx.lr = 0x8250C664;
	sub_822C4CE0(ctx, base);
	// 8250C664: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8250C668: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 8250C66C: 4BE9501D  bl 0x823a1688
	ctx.lr = 0x8250C670;
	sub_823A1688(ctx, base);
	// 8250C670: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C674: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C678: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250C67C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8250C680: 419A000C  beq cr6, 0x8250c68c
	if ctx.cr[6].eq {
	pc = 0x8250C68C; continue 'dispatch;
	}
	// 8250C684: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C688: 48000028  b 0x8250c6b0
	pc = 0x8250C6B0; continue 'dispatch;
	// 8250C68C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C690: 894A0019  lbz r10, 0x19(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C694: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250C698: 419A000C  beq cr6, 0x8250c6a4
	if ctx.cr[6].eq {
	pc = 0x8250C6A4; continue 'dispatch;
	}
	// 8250C69C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8250C6A0: 48000010  b 0x8250c6b0
	pc = 0x8250C6B0; continue 'dispatch;
	// 8250C6A4: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C6A8: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250C6AC: 409A00DC  bne cr6, 0x8250c788
	if !ctx.cr[6].eq {
	pc = 0x8250C788; continue 'dispatch;
	}
	// 8250C6B0: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C6B4: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C6B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C6BC: 409A0008  bne cr6, 0x8250c6c4
	if !ctx.cr[6].eq {
	pc = 0x8250C6C4; continue 'dispatch;
	}
	// 8250C6C0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8250C6C4: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C6C8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C6CC: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250C6D0: 409A000C  bne cr6, 0x8250c6dc
	if !ctx.cr[6].eq {
	pc = 0x8250C6DC; continue 'dispatch;
	}
	// 8250C6D4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8250C6D8: 4800001C  b 0x8250c6f4
	pc = 0x8250C6F4; continue 'dispatch;
	// 8250C6DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C6E0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250C6E4: 409A000C  bne cr6, 0x8250c6f0
	if !ctx.cr[6].eq {
	pc = 0x8250C6F0; continue 'dispatch;
	}
	// 8250C6E8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8250C6EC: 48000008  b 0x8250c6f4
	pc = 0x8250C6F4; continue 'dispatch;
	// 8250C6F0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8250C6F4: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C6F8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C6FC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250C700: 409A003C  bne cr6, 0x8250c73c
	if !ctx.cr[6].eq {
	pc = 0x8250C73C; continue 'dispatch;
	}
	// 8250C704: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C70C: 419A000C  beq cr6, 0x8250c718
	if ctx.cr[6].eq {
	pc = 0x8250C718; continue 'dispatch;
	}
	// 8250C710: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8250C714: 48000024  b 0x8250c738
	pc = 0x8250C738; continue 'dispatch;
	// 8250C718: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C71C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8250C720: 4800000C  b 0x8250c72c
	pc = 0x8250C72C; continue 'dispatch;
	// 8250C724: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8250C728: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C72C: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C730: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8250C734: 419AFFF0  beq cr6, 0x8250c724
	if ctx.cr[6].eq {
	pc = 0x8250C724; continue 'dispatch;
	}
	// 8250C738: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250C73C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C740: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C744: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250C748: 409A00D4  bne cr6, 0x8250c81c
	if !ctx.cr[6].eq {
	pc = 0x8250C81C; continue 'dispatch;
	}
	// 8250C74C: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C754: 419A000C  beq cr6, 0x8250c760
	if ctx.cr[6].eq {
	pc = 0x8250C760; continue 'dispatch;
	}
	// 8250C758: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8250C75C: 48000024  b 0x8250c780
	pc = 0x8250C780; continue 'dispatch;
	// 8250C760: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C764: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8250C768: 4800000C  b 0x8250c774
	pc = 0x8250C774; continue 'dispatch;
	// 8250C76C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8250C770: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C774: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C778: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8250C77C: 419AFFF0  beq cr6, 0x8250c76c
	if ctx.cr[6].eq {
	pc = 0x8250C76C; continue 'dispatch;
	}
	// 8250C780: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250C784: 48000098  b 0x8250c81c
	pc = 0x8250C81C; continue 'dispatch;
	// 8250C788: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8250C78C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C790: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250C794: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C798: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C79C: 409A000C  bne cr6, 0x8250c7a8
	if !ctx.cr[6].eq {
	pc = 0x8250C7A8; continue 'dispatch;
	}
	// 8250C7A0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8250C7A4: 4800002C  b 0x8250c7d0
	pc = 0x8250C7D0; continue 'dispatch;
	// 8250C7A8: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C7AC: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C7B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C7B4: 409A0008  bne cr6, 0x8250c7bc
	if !ctx.cr[6].eq {
	pc = 0x8250C7BC; continue 'dispatch;
	}
	// 8250C7B8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8250C7BC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8250C7C0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C7C4: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250C7C8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C7CC: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8250C7D0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C7D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C7D8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250C7DC: 409A000C  bne cr6, 0x8250c7e8
	if !ctx.cr[6].eq {
	pc = 0x8250C7E8; continue 'dispatch;
	}
	// 8250C7E0: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8250C7E4: 48000020  b 0x8250c804
	pc = 0x8250C804; continue 'dispatch;
	// 8250C7E8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C7EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C7F0: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250C7F4: 409A000C  bne cr6, 0x8250c800
	if !ctx.cr[6].eq {
	pc = 0x8250C800; continue 'dispatch;
	}
	// 8250C7F8: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8250C7FC: 48000008  b 0x8250c804
	pc = 0x8250C804; continue 'dispatch;
	// 8250C800: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8250C804: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C808: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250C80C: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C810: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C814: 99790018  stb r11, 0x18(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 8250C818: 995B0018  stb r10, 0x18(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 8250C81C: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C820: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8250C824: 409A0198  bne cr6, 0x8250c9bc
	if !ctx.cr[6].eq {
	pc = 0x8250C9BC; continue 'dispatch;
	}
	// 8250C828: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C82C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8250C830: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C834: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C838: 419A0180  beq cr6, 0x8250c9b8
	if ctx.cr[6].eq {
	pc = 0x8250C9B8; continue 'dispatch;
	}
	// 8250C83C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250C840: 897C0018  lbz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C844: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8250C848: 409A0170  bne cr6, 0x8250c9b8
	if !ctx.cr[6].eq {
	pc = 0x8250C9B8; continue 'dispatch;
	}
	// 8250C84C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C850: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C854: 409A00A8  bne cr6, 0x8250c8fc
	if !ctx.cr[6].eq {
	pc = 0x8250C8FC; continue 'dispatch;
	}
	// 8250C858: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C85C: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C860: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250C864: 409A001C  bne cr6, 0x8250c880
	if !ctx.cr[6].eq {
	pc = 0x8250C880; continue 'dispatch;
	}
	// 8250C868: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C86C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250C870: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 8250C874: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250C878: 4BF376F1  bl 0x82443f68
	ctx.lr = 0x8250C87C;
	sub_82443F68(ctx, base);
	// 8250C87C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C880: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C884: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250C888: 409A00C8  bne cr6, 0x8250c950
	if !ctx.cr[6].eq {
	pc = 0x8250C950; continue 'dispatch;
	}
	// 8250C88C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C890: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C894: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250C898: 409A0014  bne cr6, 0x8250c8ac
	if !ctx.cr[6].eq {
	pc = 0x8250C8AC; continue 'dispatch;
	}
	// 8250C89C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C8A0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C8A4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250C8A8: 419A00A4  beq cr6, 0x8250c94c
	if ctx.cr[6].eq {
	pc = 0x8250C94C; continue 'dispatch;
	}
	// 8250C8AC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C8B0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C8B4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250C8B8: 409A0020  bne cr6, 0x8250c8d8
	if !ctx.cr[6].eq {
	pc = 0x8250C8D8; continue 'dispatch;
	}
	// 8250C8BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C8C0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8250C8C4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250C8C8: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C8CC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 8250C8D0: 4BF37631  bl 0x82443f00
	ctx.lr = 0x8250C8D4;
	sub_82443F00(ctx, base);
	// 8250C8D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C8D8: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C8DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250C8E0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250C8E4: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 8250C8E8: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C8EC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C8F0: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C8F4: 4BF37675  bl 0x82443f68
	ctx.lr = 0x8250C8F8;
	sub_82443F68(ctx, base);
	// 8250C8F8: 480000C0  b 0x8250c9b8
	pc = 0x8250C9B8; continue 'dispatch;
	// 8250C8FC: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C900: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250C904: 409A001C  bne cr6, 0x8250c920
	if !ctx.cr[6].eq {
	pc = 0x8250C920; continue 'dispatch;
	}
	// 8250C908: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C90C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250C910: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 8250C914: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250C918: 4BF375E9  bl 0x82443f00
	ctx.lr = 0x8250C91C;
	sub_82443F00(ctx, base);
	// 8250C91C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C920: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8250C924: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250C928: 409A0028  bne cr6, 0x8250c950
	if !ctx.cr[6].eq {
	pc = 0x8250C950; continue 'dispatch;
	}
	// 8250C92C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C930: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C934: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250C938: 409A0034  bne cr6, 0x8250c96c
	if !ctx.cr[6].eq {
	pc = 0x8250C96C; continue 'dispatch;
	}
	// 8250C93C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C940: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C944: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250C948: 409A0024  bne cr6, 0x8250c96c
	if !ctx.cr[6].eq {
	pc = 0x8250C96C; continue 'dispatch;
	}
	// 8250C94C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 8250C950: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C954: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 8250C958: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C95C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C960: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C964: 409AFEDC  bne cr6, 0x8250c840
	if !ctx.cr[6].eq {
	pc = 0x8250C840; continue 'dispatch;
	}
	// 8250C968: 48000050  b 0x8250c9b8
	pc = 0x8250C9B8; continue 'dispatch;
	// 8250C96C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C970: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C974: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250C978: 409A0020  bne cr6, 0x8250c998
	if !ctx.cr[6].eq {
	pc = 0x8250C998; continue 'dispatch;
	}
	// 8250C97C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C980: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8250C984: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250C988: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C98C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 8250C990: 4BF375D9  bl 0x82443f68
	ctx.lr = 0x8250C994;
	sub_82443F68(ctx, base);
	// 8250C994: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C998: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C99C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250C9A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250C9A4: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 8250C9A8: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C9AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C9B0: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C9B4: 4BF3754D  bl 0x82443f00
	ctx.lr = 0x8250C9B8;
	sub_82443F00(ctx, base);
	// 8250C9B8: 9BDC0018  stb r30, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8250C9BC: 807B0014  lwz r3, 0x14(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C9C0: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 8250C9C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250C9C8: 419A0008  beq cr6, 0x8250c9d0
	if ctx.cr[6].eq {
	pc = 0x8250C9D0; continue 'dispatch;
	}
	// 8250C9CC: 4BDB3EC5  bl 0x822c0890
	ctx.lr = 0x8250C9D0;
	sub_822C0890(ctx, base);
	// 8250C9D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C9D4: 488E6A55  bl 0x82df3428
	ctx.lr = 0x8250C9D8;
	sub_82DF3428(ctx, base);
	// 8250C9D8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250C9DC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250C9E0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8250C9E4: 488E57A5  bl 0x82df2188
	ctx.lr = 0x8250C9E8;
	sub_82DF2188(ctx, base);
	// 8250C9E8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C9EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250C9F0: 419A000C  beq cr6, 0x8250c9fc
	if ctx.cr[6].eq {
	pc = 0x8250C9FC; continue 'dispatch;
	}
	// 8250C9F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250C9F8: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250C9FC: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8250CA00: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8250CA04: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8250CA08: 48C9B7A0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250CA10 size=132
    let mut pc: u32 = 0x8250CA10;
    'dispatch: loop {
        match pc {
            0x8250CA10 => {
    //   block [0x8250CA10..0x8250CA94)
	// 8250CA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CA14: 48C9B755  bl 0x831a8168
	ctx.lr = 0x8250CA18;
	sub_831A8130(ctx, base);
	// 8250CA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CA1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250CA20: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8250CA24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250CA28: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8250CA2C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CA30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CA34: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250CA38: 409A0044  bne cr6, 0x8250ca7c
	if !ctx.cr[6].eq {
	pc = 0x8250CA7C; continue 'dispatch;
	}
	// 8250CA3C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250CA40: 409A003C  bne cr6, 0x8250ca7c
	if !ctx.cr[6].eq {
	pc = 0x8250CA7C; continue 'dispatch;
	}
	// 8250CA44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250CA48: 48679F21  bl 0x82b86968
	ctx.lr = 0x8250CA4C;
	sub_82B86968(ctx, base);
	// 8250CA4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CA50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CA54: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250CA58: 48000030  b 0x8250ca88
	pc = 0x8250CA88; continue 'dispatch;
	// 8250CA5C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8250CA60: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8250CA64: 4BE94C25  bl 0x823a1688
	ctx.lr = 0x8250CA68;
	sub_823A1688(ctx, base);
	// 8250CA68: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8250CA6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CA70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250CA74: 4BFFFB85  bl 0x8250c5f8
	ctx.lr = 0x8250CA78;
	sub_8250C5F8(ctx, base);
	// 8250CA78: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8250CA7C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8250CA80: 409AFFDC  bne cr6, 0x8250ca5c
	if !ctx.cr[6].eq {
	pc = 0x8250CA5C; continue 'dispatch;
	}
	// 8250CA84: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8250CA88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250CA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250CA90: 48C9B728  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250CA98 size=1028
    let mut pc: u32 = 0x8250CA98;
    'dispatch: loop {
        match pc {
            0x8250CA98 => {
    //   block [0x8250CA98..0x8250CE9C)
	// 8250CA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CA9C: 48C9B6BD  bl 0x831a8158
	ctx.lr = 0x8250CAA0;
	sub_831A8130(ctx, base);
	// 8250CAA0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CAA4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8250CAA8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8250CAAC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8250CAB0: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8250CAB4: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CABC: 419A0048  beq cr6, 0x8250cb04
	if ctx.cr[6].eq {
	pc = 0x8250CB04; continue 'dispatch;
	}
	// 8250CAC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250CAC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250CAC8: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8250CACC: 4BDB8DFD  bl 0x822c58c8
	ctx.lr = 0x8250CAD0;
	sub_822C58C8(ctx, base);
	// 8250CAD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250CAD4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250CAD8: 4BDBD3D9  bl 0x822c9eb0
	ctx.lr = 0x8250CADC;
	sub_822C9EB0(ctx, base);
	// 8250CADC: 4BDB77D5  bl 0x822c42b0
	ctx.lr = 0x8250CAE0;
	sub_822C42B0(ctx, base);
	// 8250CAE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250CAE4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250CAE8: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8250CAEC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8250CAF0: 4BDB8981  bl 0x822c5470
	ctx.lr = 0x8250CAF4;
	sub_822C5470(ctx, base);
	// 8250CAF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250CAF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250CAFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250CB00: 4BDB81E1  bl 0x822c4ce0
	ctx.lr = 0x8250CB04;
	sub_822C4CE0(ctx, base);
	// 8250CB04: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8250CB08: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 8250CB0C: 4834DEAD  bl 0x8285a9b8
	ctx.lr = 0x8250CB10;
	sub_8285A9B8(ctx, base);
	// 8250CB10: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CB14: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CB18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250CB1C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8250CB20: 419A000C  beq cr6, 0x8250cb2c
	if ctx.cr[6].eq {
	pc = 0x8250CB2C; continue 'dispatch;
	}
	// 8250CB24: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CB28: 48000028  b 0x8250cb50
	pc = 0x8250CB50; continue 'dispatch;
	// 8250CB2C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CB30: 894A001D  lbz r10, 0x1d(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CB34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250CB38: 419A000C  beq cr6, 0x8250cb44
	if ctx.cr[6].eq {
	pc = 0x8250CB44; continue 'dispatch;
	}
	// 8250CB3C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8250CB40: 48000010  b 0x8250cb50
	pc = 0x8250CB50; continue 'dispatch;
	// 8250CB44: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CB48: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250CB4C: 409A00DC  bne cr6, 0x8250cc28
	if !ctx.cr[6].eq {
	pc = 0x8250CC28; continue 'dispatch;
	}
	// 8250CB50: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CB54: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CB58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CB5C: 409A0008  bne cr6, 0x8250cb64
	if !ctx.cr[6].eq {
	pc = 0x8250CB64; continue 'dispatch;
	}
	// 8250CB60: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8250CB64: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CB68: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CB6C: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250CB70: 409A000C  bne cr6, 0x8250cb7c
	if !ctx.cr[6].eq {
	pc = 0x8250CB7C; continue 'dispatch;
	}
	// 8250CB74: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8250CB78: 4800001C  b 0x8250cb94
	pc = 0x8250CB94; continue 'dispatch;
	// 8250CB7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CB80: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250CB84: 409A000C  bne cr6, 0x8250cb90
	if !ctx.cr[6].eq {
	pc = 0x8250CB90; continue 'dispatch;
	}
	// 8250CB88: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8250CB8C: 48000008  b 0x8250cb94
	pc = 0x8250CB94; continue 'dispatch;
	// 8250CB90: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8250CB94: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CB98: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CB9C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250CBA0: 409A003C  bne cr6, 0x8250cbdc
	if !ctx.cr[6].eq {
	pc = 0x8250CBDC; continue 'dispatch;
	}
	// 8250CBA4: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CBA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CBAC: 419A000C  beq cr6, 0x8250cbb8
	if ctx.cr[6].eq {
	pc = 0x8250CBB8; continue 'dispatch;
	}
	// 8250CBB0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8250CBB4: 48000024  b 0x8250cbd8
	pc = 0x8250CBD8; continue 'dispatch;
	// 8250CBB8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CBBC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8250CBC0: 4800000C  b 0x8250cbcc
	pc = 0x8250CBCC; continue 'dispatch;
	// 8250CBC4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8250CBC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CBCC: 890B001D  lbz r8, 0x1d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CBD0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8250CBD4: 419AFFF0  beq cr6, 0x8250cbc4
	if ctx.cr[6].eq {
	pc = 0x8250CBC4; continue 'dispatch;
	}
	// 8250CBD8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250CBDC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CBE0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CBE4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250CBE8: 409A00D4  bne cr6, 0x8250ccbc
	if !ctx.cr[6].eq {
	pc = 0x8250CCBC; continue 'dispatch;
	}
	// 8250CBEC: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CBF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CBF4: 419A000C  beq cr6, 0x8250cc00
	if ctx.cr[6].eq {
	pc = 0x8250CC00; continue 'dispatch;
	}
	// 8250CBF8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8250CBFC: 48000024  b 0x8250cc20
	pc = 0x8250CC20; continue 'dispatch;
	// 8250CC00: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CC04: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8250CC08: 4800000C  b 0x8250cc14
	pc = 0x8250CC14; continue 'dispatch;
	// 8250CC0C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8250CC10: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CC14: 890B001D  lbz r8, 0x1d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CC18: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8250CC1C: 419AFFF0  beq cr6, 0x8250cc0c
	if ctx.cr[6].eq {
	pc = 0x8250CC0C; continue 'dispatch;
	}
	// 8250CC20: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250CC24: 48000098  b 0x8250ccbc
	pc = 0x8250CCBC; continue 'dispatch;
	// 8250CC28: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8250CC2C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CC30: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250CC34: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CC38: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250CC3C: 409A000C  bne cr6, 0x8250cc48
	if !ctx.cr[6].eq {
	pc = 0x8250CC48; continue 'dispatch;
	}
	// 8250CC40: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8250CC44: 4800002C  b 0x8250cc70
	pc = 0x8250CC70; continue 'dispatch;
	// 8250CC48: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CC4C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CC50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CC54: 409A0008  bne cr6, 0x8250cc5c
	if !ctx.cr[6].eq {
	pc = 0x8250CC5C; continue 'dispatch;
	}
	// 8250CC58: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8250CC5C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8250CC60: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CC64: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250CC68: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CC6C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8250CC70: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CC74: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CC78: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250CC7C: 409A000C  bne cr6, 0x8250cc88
	if !ctx.cr[6].eq {
	pc = 0x8250CC88; continue 'dispatch;
	}
	// 8250CC80: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8250CC84: 48000020  b 0x8250cca4
	pc = 0x8250CCA4; continue 'dispatch;
	// 8250CC88: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CC8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CC90: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8250CC94: 409A000C  bne cr6, 0x8250cca0
	if !ctx.cr[6].eq {
	pc = 0x8250CCA0; continue 'dispatch;
	}
	// 8250CC98: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8250CC9C: 48000008  b 0x8250cca4
	pc = 0x8250CCA4; continue 'dispatch;
	// 8250CCA0: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8250CCA4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CCA8: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250CCAC: 897B001C  lbz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CCB0: 8959001C  lbz r10, 0x1c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CCB4: 9979001C  stb r11, 0x1c(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8250CCB8: 995B001C  stb r10, 0x1c(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 8250CCBC: 897B001C  lbz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CCC0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8250CCC4: 409A0198  bne cr6, 0x8250ce5c
	if !ctx.cr[6].eq {
	pc = 0x8250CE5C; continue 'dispatch;
	}
	// 8250CCC8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CCCC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8250CCD0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CCD4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250CCD8: 419A0180  beq cr6, 0x8250ce58
	if ctx.cr[6].eq {
	pc = 0x8250CE58; continue 'dispatch;
	}
	// 8250CCDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250CCE0: 897C001C  lbz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CCE4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8250CCE8: 409A0170  bne cr6, 0x8250ce58
	if !ctx.cr[6].eq {
	pc = 0x8250CE58; continue 'dispatch;
	}
	// 8250CCEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CCF0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250CCF4: 409A00A8  bne cr6, 0x8250cd9c
	if !ctx.cr[6].eq {
	pc = 0x8250CD9C; continue 'dispatch;
	}
	// 8250CCF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CCFC: 894B001C  lbz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CD00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250CD04: 409A001C  bne cr6, 0x8250cd20
	if !ctx.cr[6].eq {
	pc = 0x8250CD20; continue 'dispatch;
	}
	// 8250CD08: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CD0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CD10: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 8250CD14: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250CD18: 48022871  bl 0x8252f588
	ctx.lr = 0x8250CD1C;
	sub_8252F588(ctx, base);
	// 8250CD1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CD20: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CD24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250CD28: 409A00C8  bne cr6, 0x8250cdf0
	if !ctx.cr[6].eq {
	pc = 0x8250CDF0; continue 'dispatch;
	}
	// 8250CD2C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CD30: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CD34: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250CD38: 409A0014  bne cr6, 0x8250cd4c
	if !ctx.cr[6].eq {
	pc = 0x8250CD4C; continue 'dispatch;
	}
	// 8250CD3C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CD40: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CD44: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250CD48: 419A00A4  beq cr6, 0x8250cdec
	if ctx.cr[6].eq {
	pc = 0x8250CDEC; continue 'dispatch;
	}
	// 8250CD4C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CD50: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CD54: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250CD58: 409A0020  bne cr6, 0x8250cd78
	if !ctx.cr[6].eq {
	pc = 0x8250CD78; continue 'dispatch;
	}
	// 8250CD5C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CD60: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8250CD64: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250CD68: 9BCA001C  stb r30, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CD6C: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 8250CD70: 48022881  bl 0x8252f5f0
	ctx.lr = 0x8250CD74;
	sub_8252F5F0(ctx, base);
	// 8250CD74: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CD78: 895F001C  lbz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CD7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CD80: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250CD84: 994B001C  stb r10, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 8250CD88: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CD8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CD90: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CD94: 480227F5  bl 0x8252f588
	ctx.lr = 0x8250CD98;
	sub_8252F588(ctx, base);
	// 8250CD98: 480000C0  b 0x8250ce58
	pc = 0x8250CE58; continue 'dispatch;
	// 8250CD9C: 894B001C  lbz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CDA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250CDA4: 409A001C  bne cr6, 0x8250cdc0
	if !ctx.cr[6].eq {
	pc = 0x8250CDC0; continue 'dispatch;
	}
	// 8250CDA8: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CDAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CDB0: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 8250CDB4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250CDB8: 48022839  bl 0x8252f5f0
	ctx.lr = 0x8250CDBC;
	sub_8252F5F0(ctx, base);
	// 8250CDBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CDC0: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 8250CDC4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250CDC8: 409A0028  bne cr6, 0x8250cdf0
	if !ctx.cr[6].eq {
	pc = 0x8250CDF0; continue 'dispatch;
	}
	// 8250CDCC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CDD0: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CDD4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250CDD8: 409A0034  bne cr6, 0x8250ce0c
	if !ctx.cr[6].eq {
	pc = 0x8250CE0C; continue 'dispatch;
	}
	// 8250CDDC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CDE0: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CDE4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250CDE8: 409A0024  bne cr6, 0x8250ce0c
	if !ctx.cr[6].eq {
	pc = 0x8250CE0C; continue 'dispatch;
	}
	// 8250CDEC: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 8250CDF0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CDF4: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 8250CDF8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CDFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CE00: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250CE04: 409AFEDC  bne cr6, 0x8250cce0
	if !ctx.cr[6].eq {
	pc = 0x8250CCE0; continue 'dispatch;
	}
	// 8250CE08: 48000050  b 0x8250ce58
	pc = 0x8250CE58; continue 'dispatch;
	// 8250CE0C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CE10: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CE14: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8250CE18: 409A0020  bne cr6, 0x8250ce38
	if !ctx.cr[6].eq {
	pc = 0x8250CE38; continue 'dispatch;
	}
	// 8250CE1C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CE20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8250CE24: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250CE28: 9BCA001C  stb r30, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CE2C: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 8250CE30: 48022759  bl 0x8252f588
	ctx.lr = 0x8250CE34;
	sub_8252F588(ctx, base);
	// 8250CE34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CE38: 895F001C  lbz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CE3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CE40: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250CE44: 994B001C  stb r10, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 8250CE48: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CE4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CE50: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CE54: 4802279D  bl 0x8252f5f0
	ctx.lr = 0x8250CE58;
	sub_8252F5F0(ctx, base);
	// 8250CE58: 9BDC001C  stb r30, 0x1c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8250CE5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250CE60: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8250CE64: 482CFF3D  bl 0x827dcda0
	ctx.lr = 0x8250CE68;
	sub_827DCDA0(ctx, base);
	// 8250CE68: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250CE6C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250CE70: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8250CE74: 488E5315  bl 0x82df2188
	ctx.lr = 0x8250CE78;
	sub_82DF2188(ctx, base);
	// 8250CE78: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CE7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CE80: 419A000C  beq cr6, 0x8250ce8c
	if ctx.cr[6].eq {
	pc = 0x8250CE8C; continue 'dispatch;
	}
	// 8250CE84: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250CE88: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250CE8C: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8250CE90: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8250CE94: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8250CE98: 48C9B310  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250CEA0 size=364
    let mut pc: u32 = 0x8250CEA0;
    'dispatch: loop {
        match pc {
            0x8250CEA0 => {
    //   block [0x8250CEA0..0x8250D00C)
	// 8250CEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CEA4: 48C9B2C5  bl 0x831a8168
	ctx.lr = 0x8250CEA8;
	sub_831A8130(ctx, base);
	// 8250CEA8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CEAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250CEB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250CEB4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8250CEB8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8250CEBC: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CEC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250CEC4: 418200F0  beq 0x8250cfb4
	if ctx.cr[0].eq {
	pc = 0x8250CFB4; continue 'dispatch;
	}
	// 8250CEC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250CECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250CED0: 388B19D8  addi r4, r11, 0x19d8
	ctx.r[4].s64 = ctx.r[11].s64 + 6616;
	// 8250CED4: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8250CED8: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8250CEDC: 4BDB34FD  bl 0x822c03d8
	ctx.lr = 0x8250CEE0;
	sub_822C03D8(ctx, base);
	// 8250CEE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250CEE4: 41820024  beq 0x8250cf08
	if ctx.cr[0].eq {
	pc = 0x8250CF08; continue 'dispatch;
	}
	// 8250CEE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250CEEC: C01C0000  lfs f0, 0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250CEF0: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8250CEF4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8250CEF8: 396B1990  addi r11, r11, 0x1990
	ctx.r[11].s64 = ctx.r[11].s64 + 6544;
	// 8250CEFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250CF00: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250CF04: 48000008  b 0x8250cf0c
	pc = 0x8250CF0C; continue 'dispatch;
	// 8250CF08: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250CF0C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8250CF10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CF14: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250CF18: 4BFFD281  bl 0x8250a198
	ctx.lr = 0x8250CF1C;
	sub_8250A198(ctx, base);
	// 8250CF1C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250CF20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CF24: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250CF28: 4BDB30D9  bl 0x822c0000
	ctx.lr = 0x8250CF2C;
	sub_822C0000(ctx, base);
	// 8250CF2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250CF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250CF34: 488E6CCD  bl 0x82df3c00
	ctx.lr = 0x8250CF38;
	sub_82DF3C00(ctx, base);
	// 8250CF38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250CF3C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8250CF40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250CF44: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250CF48: 4857E8C9  bl 0x82a8b810
	ctx.lr = 0x8250CF4C;
	sub_82A8B810(ctx, base);
	// 8250CF4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250CF50: 488E64D9  bl 0x82df3428
	ctx.lr = 0x8250CF54;
	sub_82DF3428(ctx, base);
	// 8250CF54: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250CF58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250CF5C: 419A0008  beq cr6, 0x8250cf64
	if ctx.cr[6].eq {
	pc = 0x8250CF64; continue 'dispatch;
	}
	// 8250CF60: 4BDB3931  bl 0x822c0890
	ctx.lr = 0x8250CF64;
	sub_822C0890(ctx, base);
	// 8250CF64: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8250CF68: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250CF6C: 48678845  bl 0x82b857b0
	ctx.lr = 0x8250CF70;
	sub_82B857B0(ctx, base);
	// 8250CF70: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250CF74: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8250CF78: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250CF7C: 48679A45  bl 0x82b869c0
	ctx.lr = 0x8250CF80;
	sub_82B869C0(ctx, base);
	// 8250CF80: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8250CF84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250CF88: 419A0008  beq cr6, 0x8250cf90
	if ctx.cr[6].eq {
	pc = 0x8250CF90; continue 'dispatch;
	}
	// 8250CF8C: 4BDB3905  bl 0x822c0890
	ctx.lr = 0x8250CF90;
	sub_822C0890(ctx, base);
	// 8250CF90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250CF94: 488E6495  bl 0x82df3428
	ctx.lr = 0x8250CF98;
	sub_82DF3428(ctx, base);
	// 8250CF98: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8250CF9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250CFA0: 419A0008  beq cr6, 0x8250cfa8
	if ctx.cr[6].eq {
	pc = 0x8250CFA8; continue 'dispatch;
	}
	// 8250CFA4: 4BDB38ED  bl 0x822c0890
	ctx.lr = 0x8250CFA8;
	sub_822C0890(ctx, base);
	// 8250CFA8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250CFAC: 488E647D  bl 0x82df3428
	ctx.lr = 0x8250CFB0;
	sub_82DF3428(ctx, base);
	// 8250CFB0: 48000054  b 0x8250d004
	pc = 0x8250D004; continue 'dispatch;
	// 8250CFB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8250CFB8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8250CFBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250CFC0: 482E9809  bl 0x827f67c8
	ctx.lr = 0x8250CFC4;
	sub_827F67C8(ctx, base);
	// 8250CFC4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250CFC8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CFCC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250CFD0: 419A0034  beq cr6, 0x8250d004
	if ctx.cr[6].eq {
	pc = 0x8250D004; continue 'dispatch;
	}
	// 8250CFD4: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250CFD8: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 8250CFDC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250CFE0: 38CAC6CC  addi r6, r10, -0x3934
	ctx.r[6].s64 = ctx.r[10].s64 + -14644;
	// 8250CFE4: 38ABBE54  addi r5, r11, -0x41ac
	ctx.r[5].s64 = ctx.r[11].s64 + -16812;
	// 8250CFE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8250CFEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250CFF0: 48C9CF59  bl 0x831a9f48
	ctx.lr = 0x8250CFF4;
	sub_831A9F48(ctx, base);
	// 8250CFF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250CFF8: 4182000C  beq 0x8250d004
	if ctx.cr[0].eq {
	pc = 0x8250D004; continue 'dispatch;
	}
	// 8250CFFC: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250D000: D01C0000  stfs f0, 0(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8250D004: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8250D008: 48C9B1B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D010 size=172
    let mut pc: u32 = 0x8250D010;
    'dispatch: loop {
        match pc {
            0x8250D010 => {
    //   block [0x8250D010..0x8250D0BC)
	// 8250D010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D014: 48C9B151  bl 0x831a8164
	ctx.lr = 0x8250D018;
	sub_831A8130(ctx, base);
	// 8250D018: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D01C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D020: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8250D024: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250D028: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8250D02C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8250D030: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D034: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250D038: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8250D03C: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8250D040: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250D044: 484C0A15  bl 0x829cda58
	ctx.lr = 0x8250D048;
	sub_829CDA58(ctx, base);
	// 8250D048: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250D04C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D050: 48002479  bl 0x8250f4c8
	ctx.lr = 0x8250D054;
	sub_8250F4C8(ctx, base);
	// 8250D054: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250D058: 488E4C39  bl 0x82df1c90
	ctx.lr = 0x8250D05C;
	sub_82DF1C90(ctx, base);
	// 8250D05C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8250D060: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250D064: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D068: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250D06C: 48002EC5  bl 0x8250ff30
	ctx.lr = 0x8250D070;
	sub_8250FF30(ctx, base);
	// 8250D070: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8250D074: 419A0010  beq cr6, 0x8250d084
	if ctx.cr[6].eq {
	pc = 0x8250D084; continue 'dispatch;
	}
	// 8250D078: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D07C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250D080: 48003C21  bl 0x82510ca0
	ctx.lr = 0x8250D084;
	sub_82510CA0(ctx, base);
	// 8250D084: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D088: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8250D08C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250D090: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250D094: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D098: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250D09C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D0A0: 4E800421  bctrl
	ctx.lr = 0x8250D0A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D0A4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D0A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D0AC: 419A0008  beq cr6, 0x8250d0b4
	if ctx.cr[6].eq {
	pc = 0x8250D0B4; continue 'dispatch;
	}
	// 8250D0B0: 4BDB37E1  bl 0x822c0890
	ctx.lr = 0x8250D0B4;
	sub_822C0890(ctx, base);
	// 8250D0B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250D0B8: 48C9B0FC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D0C0 size=160
    let mut pc: u32 = 0x8250D0C0;
    'dispatch: loop {
        match pc {
            0x8250D0C0 => {
    //   block [0x8250D0C0..0x8250D160)
	// 8250D0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D0C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D0CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D0D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D0D4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8250D0D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250D0DC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8250D0E0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8250D0E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D0E8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D0EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250D0F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250D0F4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250D0F8: 419A0024  beq cr6, 0x8250d11c
	if ctx.cr[6].eq {
	pc = 0x8250D11C; continue 'dispatch;
	}
	// 8250D0FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250D100: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D104: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D108: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D10C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D110: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D114: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D118: 4082FFE8  bne 0x8250d100
	if !ctx.cr[0].eq {
	pc = 0x8250D100; continue 'dispatch;
	}
	// 8250D11C: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 8250D120: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250D124: 4BFFFEED  bl 0x8250d010
	ctx.lr = 0x8250D128;
	sub_8250D010(ctx, base);
	// 8250D128: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D12C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8250D130: 48AFBE89  bl 0x83008fb8
	ctx.lr = 0x8250D134;
	sub_83008FB8(ctx, base);
	// 8250D134: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8250D138: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D13C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D140: 419A0008  beq cr6, 0x8250d148
	if ctx.cr[6].eq {
	pc = 0x8250D148; continue 'dispatch;
	}
	// 8250D144: 4BDB374D  bl 0x822c0890
	ctx.lr = 0x8250D148;
	sub_822C0890(ctx, base);
	// 8250D148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D154: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250D158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D160 size=100
    let mut pc: u32 = 0x8250D160;
    'dispatch: loop {
        match pc {
            0x8250D160 => {
    //   block [0x8250D160..0x8250D1C4)
	// 8250D160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D16C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D174: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250D178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D17C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250D180: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250D184: 388B1AF8  addi r4, r11, 0x1af8
	ctx.r[4].s64 = ctx.r[11].s64 + 6904;
	// 8250D188: 488E6881  bl 0x82df3a08
	ctx.lr = 0x8250D18C;
	sub_82DF3A08(ctx, base);
	// 8250D18C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D190: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8250D194: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250D198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250D19C: 38AB0060  addi r5, r11, 0x60
	ctx.r[5].s64 = ctx.r[11].s64 + 96;
	// 8250D1A0: 4BFFFD01  bl 0x8250cea0
	ctx.lr = 0x8250D1A4;
	sub_8250CEA0(ctx, base);
	// 8250D1A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250D1A8: 488E6281  bl 0x82df3428
	ctx.lr = 0x8250D1AC;
	sub_82DF3428(ctx, base);
	// 8250D1AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D1B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250D1BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D1C8 size=152
    let mut pc: u32 = 0x8250D1C8;
    'dispatch: loop {
        match pc {
            0x8250D1C8 => {
    //   block [0x8250D1C8..0x8250D260)
	// 8250D1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D1D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D1D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D1D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D1DC: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250D1E0: 40820028  bne 0x8250d208
	if !ctx.cr[0].eq {
	pc = 0x8250D208; continue 'dispatch;
	}
	// 8250D1E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D1E8: 814B0114  lwz r10, 0x114(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8250D1EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250D1F0: 419A0018  beq cr6, 0x8250d208
	if ctx.cr[6].eq {
	pc = 0x8250D208; continue 'dispatch;
	}
	// 8250D1F4: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8250D1F8: 482A53B1  bl 0x827b25a8
	ctx.lr = 0x8250D1FC;
	sub_827B25A8(ctx, base);
	// 8250D1FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D200: 808B0114  lwz r4, 0x114(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8250D204: 4800003C  b 0x8250d240
	pc = 0x8250D240; continue 'dispatch;
	// 8250D208: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D20C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250D210: 396B0114  addi r11, r11, 0x114
	ctx.r[11].s64 = ctx.r[11].s64 + 276;
	// 8250D214: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250D218: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D21C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D220: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250D224: 419A0008  beq cr6, 0x8250d22c
	if ctx.cr[6].eq {
	pc = 0x8250D22C; continue 'dispatch;
	}
	// 8250D228: 4BDB3669  bl 0x822c0890
	ctx.lr = 0x8250D22C;
	sub_822C0890(ctx, base);
	// 8250D22C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D230: 806B011C  lwz r3, 0x11c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(284 as u32) ) } as u64;
	// 8250D234: 482A5375  bl 0x827b25a8
	ctx.lr = 0x8250D238;
	sub_827B25A8(ctx, base);
	// 8250D238: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D23C: 808B011C  lwz r4, 0x11c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(284 as u32) ) } as u64;
	// 8250D240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D244: 4BFFFF1D  bl 0x8250d160
	ctx.lr = 0x8250D248;
	sub_8250D160(ctx, base);
	// 8250D248: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8250D24C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250D250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D260 size=132
    let mut pc: u32 = 0x8250D260;
    'dispatch: loop {
        match pc {
            0x8250D260 => {
    //   block [0x8250D260..0x8250D2E4)
	// 8250D260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D264: 48C9AF05  bl 0x831a8168
	ctx.lr = 0x8250D268;
	sub_831A8130(ctx, base);
	// 8250D268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D26C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250D270: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8250D274: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250D278: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8250D27C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D280: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D284: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250D288: 409A0044  bne cr6, 0x8250d2cc
	if !ctx.cr[6].eq {
	pc = 0x8250D2CC; continue 'dispatch;
	}
	// 8250D28C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250D290: 409A003C  bne cr6, 0x8250d2cc
	if !ctx.cr[6].eq {
	pc = 0x8250D2CC; continue 'dispatch;
	}
	// 8250D294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D298: 4BDD85B1  bl 0x822e5848
	ctx.lr = 0x8250D29C;
	sub_822E5848(ctx, base);
	// 8250D29C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D2A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D2A4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250D2A8: 48000030  b 0x8250d2d8
	pc = 0x8250D2D8; continue 'dispatch;
	// 8250D2AC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8250D2B0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8250D2B4: 4834D705  bl 0x8285a9b8
	ctx.lr = 0x8250D2B8;
	sub_8285A9B8(ctx, base);
	// 8250D2B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8250D2BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D2C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250D2C4: 4BFFF7D5  bl 0x8250ca98
	ctx.lr = 0x8250D2C8;
	sub_8250CA98(ctx, base);
	// 8250D2C8: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8250D2CC: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8250D2D0: 409AFFDC  bne cr6, 0x8250d2ac
	if !ctx.cr[6].eq {
	pc = 0x8250D2AC; continue 'dispatch;
	}
	// 8250D2D4: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8250D2D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250D2DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250D2E0: 48C9AED8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D2E8 size=540
    let mut pc: u32 = 0x8250D2E8;
    'dispatch: loop {
        match pc {
            0x8250D2E8 => {
    //   block [0x8250D2E8..0x8250D504)
	// 8250D2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D2F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D2F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D2F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D2FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D300: 83DF0224  lwz r30, 0x224(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(548 as u32) ) } as u64;
	// 8250D304: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8250D308: 419A0014  beq cr6, 0x8250d31c
	if ctx.cr[6].eq {
	pc = 0x8250D31C; continue 'dispatch;
	}
	// 8250D30C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250D310: 486A2471  bl 0x82baf780
	ctx.lr = 0x8250D314;
	sub_82BAF780(ctx, base);
	// 8250D314: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250D318: 488E50C1  bl 0x82df23d8
	ctx.lr = 0x8250D31C;
	sub_82DF23D8(ctx, base);
	// 8250D31C: 807F01CC  lwz r3, 0x1cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 8250D320: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D324: 419A0008  beq cr6, 0x8250d32c
	if ctx.cr[6].eq {
	pc = 0x8250D32C; continue 'dispatch;
	}
	// 8250D328: 4BDB3569  bl 0x822c0890
	ctx.lr = 0x8250D32C;
	sub_822C0890(ctx, base);
	// 8250D32C: 807F01C4  lwz r3, 0x1c4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(452 as u32) ) } as u64;
	// 8250D330: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D334: 419A0008  beq cr6, 0x8250d33c
	if ctx.cr[6].eq {
	pc = 0x8250D33C; continue 'dispatch;
	}
	// 8250D338: 4BDB3559  bl 0x822c0890
	ctx.lr = 0x8250D33C;
	sub_822C0890(ctx, base);
	// 8250D33C: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 8250D340: 4BF5CCC1  bl 0x8246a000
	ctx.lr = 0x8250D344;
	sub_8246A000(ctx, base);
	// 8250D344: 807F01AC  lwz r3, 0x1ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(428 as u32) ) } as u64;
	// 8250D348: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D34C: 419A0008  beq cr6, 0x8250d354
	if ctx.cr[6].eq {
	pc = 0x8250D354; continue 'dispatch;
	}
	// 8250D350: 4BDB3541  bl 0x822c0890
	ctx.lr = 0x8250D354;
	sub_822C0890(ctx, base);
	// 8250D354: 807F01A0  lwz r3, 0x1a0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(416 as u32) ) } as u64;
	// 8250D358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D35C: 419A0008  beq cr6, 0x8250d364
	if ctx.cr[6].eq {
	pc = 0x8250D364; continue 'dispatch;
	}
	// 8250D360: 4BDB3531  bl 0x822c0890
	ctx.lr = 0x8250D364;
	sub_822C0890(ctx, base);
	// 8250D364: 807F0194  lwz r3, 0x194(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(404 as u32) ) } as u64;
	// 8250D368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D36C: 419A0008  beq cr6, 0x8250d374
	if ctx.cr[6].eq {
	pc = 0x8250D374; continue 'dispatch;
	}
	// 8250D370: 4BDB3521  bl 0x822c0890
	ctx.lr = 0x8250D374;
	sub_822C0890(ctx, base);
	// 8250D374: 807F0188  lwz r3, 0x188(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) } as u64;
	// 8250D378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D37C: 419A0008  beq cr6, 0x8250d384
	if ctx.cr[6].eq {
	pc = 0x8250D384; continue 'dispatch;
	}
	// 8250D380: 4BDB3511  bl 0x822c0890
	ctx.lr = 0x8250D384;
	sub_822C0890(ctx, base);
	// 8250D384: 807F017C  lwz r3, 0x17c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(380 as u32) ) } as u64;
	// 8250D388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D38C: 419A0008  beq cr6, 0x8250d394
	if ctx.cr[6].eq {
	pc = 0x8250D394; continue 'dispatch;
	}
	// 8250D390: 4BDB3501  bl 0x822c0890
	ctx.lr = 0x8250D394;
	sub_822C0890(ctx, base);
	// 8250D394: 807F0170  lwz r3, 0x170(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8250D398: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D39C: 419A0008  beq cr6, 0x8250d3a4
	if ctx.cr[6].eq {
	pc = 0x8250D3A4; continue 'dispatch;
	}
	// 8250D3A0: 4BDB34F1  bl 0x822c0890
	ctx.lr = 0x8250D3A4;
	sub_822C0890(ctx, base);
	// 8250D3A4: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 8250D3A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D3AC: 419A0008  beq cr6, 0x8250d3b4
	if ctx.cr[6].eq {
	pc = 0x8250D3B4; continue 'dispatch;
	}
	// 8250D3B0: 4BDB34E1  bl 0x822c0890
	ctx.lr = 0x8250D3B4;
	sub_822C0890(ctx, base);
	// 8250D3B4: 807F0118  lwz r3, 0x118(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 8250D3B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D3BC: 419A0008  beq cr6, 0x8250d3c4
	if ctx.cr[6].eq {
	pc = 0x8250D3C4; continue 'dispatch;
	}
	// 8250D3C0: 4BDB34D1  bl 0x822c0890
	ctx.lr = 0x8250D3C4;
	sub_822C0890(ctx, base);
	// 8250D3C4: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8250D3C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D3CC: 419A0008  beq cr6, 0x8250d3d4
	if ctx.cr[6].eq {
	pc = 0x8250D3D4; continue 'dispatch;
	}
	// 8250D3D0: 4BDB34C1  bl 0x822c0890
	ctx.lr = 0x8250D3D4;
	sub_822C0890(ctx, base);
	// 8250D3D4: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8250D3D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D3DC: 419A0008  beq cr6, 0x8250d3e4
	if ctx.cr[6].eq {
	pc = 0x8250D3E4; continue 'dispatch;
	}
	// 8250D3E0: 4BDB34B1  bl 0x822c0890
	ctx.lr = 0x8250D3E4;
	sub_822C0890(ctx, base);
	// 8250D3E4: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 8250D3E8: 488E6041  bl 0x82df3428
	ctx.lr = 0x8250D3EC;
	sub_82DF3428(ctx, base);
	// 8250D3EC: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8250D3F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D3F4: 419A0008  beq cr6, 0x8250d3fc
	if ctx.cr[6].eq {
	pc = 0x8250D3FC; continue 'dispatch;
	}
	// 8250D3F8: 4BDB3499  bl 0x822c0890
	ctx.lr = 0x8250D3FC;
	sub_822C0890(ctx, base);
	// 8250D3FC: 807F00A0  lwz r3, 0xa0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8250D400: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D404: 419A0008  beq cr6, 0x8250d40c
	if ctx.cr[6].eq {
	pc = 0x8250D40C; continue 'dispatch;
	}
	// 8250D408: 4BDB3489  bl 0x822c0890
	ctx.lr = 0x8250D40C;
	sub_822C0890(ctx, base);
	// 8250D40C: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8250D410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D414: 419A0008  beq cr6, 0x8250d41c
	if ctx.cr[6].eq {
	pc = 0x8250D41C; continue 'dispatch;
	}
	// 8250D418: 4BDB3479  bl 0x822c0890
	ctx.lr = 0x8250D41C;
	sub_822C0890(ctx, base);
	// 8250D41C: 807F0088  lwz r3, 0x88(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8250D420: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D424: 419A0008  beq cr6, 0x8250d42c
	if ctx.cr[6].eq {
	pc = 0x8250D42C; continue 'dispatch;
	}
	// 8250D428: 4BDB3469  bl 0x822c0890
	ctx.lr = 0x8250D42C;
	sub_822C0890(ctx, base);
	// 8250D42C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8250D430: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D434: 419A0008  beq cr6, 0x8250d43c
	if ctx.cr[6].eq {
	pc = 0x8250D43C; continue 'dispatch;
	}
	// 8250D438: 4BDB3459  bl 0x822c0890
	ctx.lr = 0x8250D43C;
	sub_822C0890(ctx, base);
	// 8250D43C: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8250D440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D444: 419A0008  beq cr6, 0x8250d44c
	if ctx.cr[6].eq {
	pc = 0x8250D44C; continue 'dispatch;
	}
	// 8250D448: 4BDB3449  bl 0x822c0890
	ctx.lr = 0x8250D44C;
	sub_822C0890(ctx, base);
	// 8250D44C: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8250D450: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D454: 419A0008  beq cr6, 0x8250d45c
	if ctx.cr[6].eq {
	pc = 0x8250D45C; continue 'dispatch;
	}
	// 8250D458: 4BDB3439  bl 0x822c0890
	ctx.lr = 0x8250D45C;
	sub_822C0890(ctx, base);
	// 8250D45C: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8250D460: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D464: 419A0008  beq cr6, 0x8250d46c
	if ctx.cr[6].eq {
	pc = 0x8250D46C; continue 'dispatch;
	}
	// 8250D468: 4BDB3429  bl 0x822c0890
	ctx.lr = 0x8250D46C;
	sub_822C0890(ctx, base);
	// 8250D46C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250D470: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D474: 419A0008  beq cr6, 0x8250d47c
	if ctx.cr[6].eq {
	pc = 0x8250D47C; continue 'dispatch;
	}
	// 8250D478: 4BDB3419  bl 0x822c0890
	ctx.lr = 0x8250D47C;
	sub_822C0890(ctx, base);
	// 8250D47C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250D480: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D484: 419A0008  beq cr6, 0x8250d48c
	if ctx.cr[6].eq {
	pc = 0x8250D48C; continue 'dispatch;
	}
	// 8250D488: 4BDB3409  bl 0x822c0890
	ctx.lr = 0x8250D48C;
	sub_822C0890(ctx, base);
	// 8250D48C: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8250D490: 4BF5CB71  bl 0x8246a000
	ctx.lr = 0x8250D494;
	sub_8246A000(ctx, base);
	// 8250D494: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250D498: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D49C: 419A0008  beq cr6, 0x8250d4a4
	if ctx.cr[6].eq {
	pc = 0x8250D4A4; continue 'dispatch;
	}
	// 8250D4A0: 4BDB33F1  bl 0x822c0890
	ctx.lr = 0x8250D4A4;
	sub_822C0890(ctx, base);
	// 8250D4A4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8250D4A8: 482F9011  bl 0x828064b8
	ctx.lr = 0x8250D4AC;
	sub_828064B8(ctx, base);
	// 8250D4AC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250D4B0: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250D4B4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8250D4B8: 488E4CD1  bl 0x82df2188
	ctx.lr = 0x8250D4BC;
	sub_82DF2188(ctx, base);
	// 8250D4BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250D4C0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8250D4C4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250D4C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D4CC: 419A0008  beq cr6, 0x8250d4d4
	if ctx.cr[6].eq {
	pc = 0x8250D4D4; continue 'dispatch;
	}
	// 8250D4D0: 4BDB33C1  bl 0x822c0890
	ctx.lr = 0x8250D4D4;
	sub_822C0890(ctx, base);
	// 8250D4D4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8250D4D8: 480A2C39  bl 0x825b0110
	ctx.lr = 0x8250D4DC;
	sub_825B0110(ctx, base);
	// 8250D4DC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D4E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D4E4: 419A0008  beq cr6, 0x8250d4ec
	if ctx.cr[6].eq {
	pc = 0x8250D4EC; continue 'dispatch;
	}
	// 8250D4E8: 4BDB33A9  bl 0x822c0890
	ctx.lr = 0x8250D4EC;
	sub_822C0890(ctx, base);
	// 8250D4EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D4F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250D4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D508 size=132
    let mut pc: u32 = 0x8250D508;
    'dispatch: loop {
        match pc {
            0x8250D508 => {
    //   block [0x8250D508..0x8250D58C)
	// 8250D508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D514: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D518: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D51C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8250D520: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D524: 3BE50004  addi r31, r5, 4
	ctx.r[31].s64 = ctx.r[5].s64 + 4;
	// 8250D528: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250D52C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250D530: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250D534: 419A0024  beq cr6, 0x8250d558
	if ctx.cr[6].eq {
	pc = 0x8250D558; continue 'dispatch;
	}
	// 8250D538: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250D53C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D540: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D544: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D548: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D54C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D550: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D554: 4082FFE8  bne 0x8250d53c
	if !ctx.cr[0].eq {
	pc = 0x8250D53C; continue 'dispatch;
	}
	// 8250D558: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D55C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250D560: 38CB0018  addi r6, r11, 0x18
	ctx.r[6].s64 = ctx.r[11].s64 + 24;
	// 8250D564: 4BFFFAAD  bl 0x8250d010
	ctx.lr = 0x8250D568;
	sub_8250D010(ctx, base);
	// 8250D568: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D56C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D570: 419A0008  beq cr6, 0x8250d578
	if ctx.cr[6].eq {
	pc = 0x8250D578; continue 'dispatch;
	}
	// 8250D574: 4BDB331D  bl 0x822c0890
	ctx.lr = 0x8250D578;
	sub_822C0890(ctx, base);
	// 8250D578: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D57C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D590 size=132
    let mut pc: u32 = 0x8250D590;
    'dispatch: loop {
        match pc {
            0x8250D590 => {
    //   block [0x8250D590..0x8250D614)
	// 8250D590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D59C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D5A0: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D5A4: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8250D5A8: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D5AC: 3BE60004  addi r31, r6, 4
	ctx.r[31].s64 = ctx.r[6].s64 + 4;
	// 8250D5B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250D5B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250D5B8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250D5BC: 419A0024  beq cr6, 0x8250d5e0
	if ctx.cr[6].eq {
	pc = 0x8250D5E0; continue 'dispatch;
	}
	// 8250D5C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250D5C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D5C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D5CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D5D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D5D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D5D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D5DC: 4082FFE8  bne 0x8250d5c4
	if !ctx.cr[0].eq {
	pc = 0x8250D5C4; continue 'dispatch;
	}
	// 8250D5E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D5E4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8250D5E8: 38EB0018  addi r7, r11, 0x18
	ctx.r[7].s64 = ctx.r[11].s64 + 24;
	// 8250D5EC: 4BFFFAD5  bl 0x8250d0c0
	ctx.lr = 0x8250D5F0;
	sub_8250D0C0(ctx, base);
	// 8250D5F0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D5F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D5F8: 419A0008  beq cr6, 0x8250d600
	if ctx.cr[6].eq {
	pc = 0x8250D600; continue 'dispatch;
	}
	// 8250D5FC: 4BDB3295  bl 0x822c0890
	ctx.lr = 0x8250D600;
	sub_822C0890(ctx, base);
	// 8250D600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D60C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D618 size=1004
    let mut pc: u32 = 0x8250D618;
    'dispatch: loop {
        match pc {
            0x8250D618 => {
    //   block [0x8250D618..0x8250DA04)
	// 8250D618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D61C: 48C9AB41  bl 0x831a815c
	ctx.lr = 0x8250D620;
	sub_831A8130(ctx, base);
	// 8250D620: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D624: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8250D628: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8250D62C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8250D630: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8250D634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250D638: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D63C: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 8250D640: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250D644: 482E9185  bl 0x827f67c8
	ctx.lr = 0x8250D648;
	sub_827F67C8(ctx, base);
	// 8250D648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D64C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8250D650: 409A03AC  bne cr6, 0x8250d9fc
	if !ctx.cr[6].eq {
	pc = 0x8250D9FC; continue 'dispatch;
	}
	// 8250D654: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250D658: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250D65C: 3BCB199C  addi r30, r11, 0x199c
	ctx.r[30].s64 = ctx.r[11].s64 + 6556;
	// 8250D660: 38A002E5  li r5, 0x2e5
	ctx.r[5].s64 = 741;
	// 8250D664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250D668: 3860009C  li r3, 0x9c
	ctx.r[3].s64 = 156;
	// 8250D66C: 488E4D7D  bl 0x82df23e8
	ctx.lr = 0x8250D670;
	sub_82DF23E8(ctx, base);
	// 8250D670: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250D674: 41820014  beq 0x8250d688
	if ctx.cr[0].eq {
	pc = 0x8250D688; continue 'dispatch;
	}
	// 8250D678: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250D67C: 4801C0ED  bl 0x82529768
	ctx.lr = 0x8250D680;
	sub_82529768(ctx, base);
	// 8250D680: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D684: 48000008  b 0x8250d68c
	pc = 0x8250D68C; continue 'dispatch;
	// 8250D688: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250D68C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8250D690: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D694: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250D698: 4BFFBA99  bl 0x82509130
	ctx.lr = 0x8250D69C;
	sub_82509130(ctx, base);
	// 8250D69C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250D6A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D6A4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250D6A8: 4BDB2959  bl 0x822c0000
	ctx.lr = 0x8250D6AC;
	sub_822C0000(ctx, base);
	// 8250D6AC: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250D6B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8250D6B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250D6B8: 4BFFB0E9  bl 0x825087a0
	ctx.lr = 0x8250D6BC;
	sub_825087A0(ctx, base);
	// 8250D6BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250D6C0: 4834D261  bl 0x8285a920
	ctx.lr = 0x8250D6C4;
	sub_8285A920(ctx, base);
	// 8250D6C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D6C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8250D6CC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D6D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250D6D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250D6D8: 419A0024  beq cr6, 0x8250d6fc
	if ctx.cr[6].eq {
	pc = 0x8250D6FC; continue 'dispatch;
	}
	// 8250D6DC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250D6E0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D6E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D6E8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D6EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D6F0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D6F4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D6F8: 4082FFE8  bne 0x8250d6e0
	if !ctx.cr[0].eq {
	pc = 0x8250D6E0; continue 'dispatch;
	}
	// 8250D6FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250D700: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8250D704: 4801A8BD  bl 0x82527fc0
	ctx.lr = 0x8250D708;
	sub_82527FC0(ctx, base);
	// 8250D708: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250D70C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8250D710: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250D714: 4BFFB13D  bl 0x82508850
	ctx.lr = 0x8250D718;
	sub_82508850(ctx, base);
	// 8250D718: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250D71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250D720: 38A002EB  li r5, 0x2eb
	ctx.r[5].s64 = 747;
	// 8250D724: 386003A0  li r3, 0x3a0
	ctx.r[3].s64 = 928;
	// 8250D728: 488E4CC1  bl 0x82df23e8
	ctx.lr = 0x8250D72C;
	sub_82DF23E8(ctx, base);
	// 8250D72C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250D730: 41820010  beq 0x8250d740
	if ctx.cr[0].eq {
	pc = 0x8250D740; continue 'dispatch;
	}
	// 8250D734: 4BF5A7ED  bl 0x82467f20
	ctx.lr = 0x8250D738;
	sub_82467F20(ctx, base);
	// 8250D738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D73C: 48000008  b 0x8250d744
	pc = 0x8250D744; continue 'dispatch;
	// 8250D740: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250D744: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8250D748: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D74C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250D750: 4BFFBAA9  bl 0x825091f8
	ctx.lr = 0x8250D754;
	sub_825091F8(ctx, base);
	// 8250D754: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250D758: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D75C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250D760: 4BDB28A1  bl 0x822c0000
	ctx.lr = 0x8250D764;
	sub_822C0000(ctx, base);
	// 8250D764: 83810054  lwz r28, 0x54(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250D768: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250D76C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8250D770: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8250D774: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8250D778: 419A0024  beq cr6, 0x8250d79c
	if ctx.cr[6].eq {
	pc = 0x8250D79C; continue 'dispatch;
	}
	// 8250D77C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8250D780: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D784: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D788: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D78C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D790: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D794: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D798: 4082FFE8  bne 0x8250d780
	if !ctx.cr[0].eq {
	pc = 0x8250D780; continue 'dispatch;
	}
	// 8250D79C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8250D7A0: 389D0094  addi r4, r29, 0x94
	ctx.r[4].s64 = ctx.r[29].s64 + 148;
	// 8250D7A4: 409A0008  bne cr6, 0x8250d7ac
	if !ctx.cr[6].eq {
	pc = 0x8250D7AC; continue 'dispatch;
	}
	// 8250D7A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250D7AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250D7B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250D7B4: 488E4445  bl 0x82df1bf8
	ctx.lr = 0x8250D7B8;
	sub_82DF1BF8(ctx, base);
	// 8250D7B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250D7BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250D7C0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8250D7C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8250D7C8: 4BFFFD41  bl 0x8250d508
	ctx.lr = 0x8250D7CC;
	sub_8250D508(ctx, base);
	// 8250D7CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250D7D0: 488E44C1  bl 0x82df1c90
	ctx.lr = 0x8250D7D4;
	sub_82DF1C90(ctx, base);
	// 8250D7D4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8250D7D8: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8250D7DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8250D7E0: 419A0024  beq cr6, 0x8250d804
	if ctx.cr[6].eq {
	pc = 0x8250D804; continue 'dispatch;
	}
	// 8250D7E4: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8250D7E8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D7EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D7F0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D7F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D7F8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D7FC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D800: 4082FFE8  bne 0x8250d7e8
	if !ctx.cr[0].eq {
	pc = 0x8250D7E8; continue 'dispatch;
	}
	// 8250D804: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250D808: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250D80C: 4801B055  bl 0x82528860
	ctx.lr = 0x8250D810;
	sub_82528860(ctx, base);
	// 8250D810: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250D814: 4182010C  beq 0x8250d920
	if ctx.cr[0].eq {
	pc = 0x8250D920; continue 'dispatch;
	}
	// 8250D818: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250D81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250D820: 38A002F2  li r5, 0x2f2
	ctx.r[5].s64 = 754;
	// 8250D824: 386003A0  li r3, 0x3a0
	ctx.r[3].s64 = 928;
	// 8250D828: 488E4BC1  bl 0x82df23e8
	ctx.lr = 0x8250D82C;
	sub_82DF23E8(ctx, base);
	// 8250D82C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250D830: 41820010  beq 0x8250d840
	if ctx.cr[0].eq {
	pc = 0x8250D840; continue 'dispatch;
	}
	// 8250D834: 4BF5A6ED  bl 0x82467f20
	ctx.lr = 0x8250D838;
	sub_82467F20(ctx, base);
	// 8250D838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D83C: 48000008  b 0x8250d844
	pc = 0x8250D844; continue 'dispatch;
	// 8250D840: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250D844: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8250D848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D84C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250D850: 4BFFB9A9  bl 0x825091f8
	ctx.lr = 0x8250D854;
	sub_825091F8(ctx, base);
	// 8250D854: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250D858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250D85C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250D860: 4BDB27A1  bl 0x822c0000
	ctx.lr = 0x8250D864;
	sub_822C0000(ctx, base);
	// 8250D864: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250D868: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250D86C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250D870: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8250D874: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250D878: 419A0024  beq cr6, 0x8250d89c
	if ctx.cr[6].eq {
	pc = 0x8250D89C; continue 'dispatch;
	}
	// 8250D87C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8250D880: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D884: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D888: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D88C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D890: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D894: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D898: 4082FFE8  bne 0x8250d880
	if !ctx.cr[0].eq {
	pc = 0x8250D880; continue 'dispatch;
	}
	// 8250D89C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8250D8A0: 389D0094  addi r4, r29, 0x94
	ctx.r[4].s64 = ctx.r[29].s64 + 148;
	// 8250D8A4: 409A0008  bne cr6, 0x8250d8ac
	if !ctx.cr[6].eq {
	pc = 0x8250D8AC; continue 'dispatch;
	}
	// 8250D8A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250D8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250D8B0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250D8B4: 488E4345  bl 0x82df1bf8
	ctx.lr = 0x8250D8B8;
	sub_82DF1BF8(ctx, base);
	// 8250D8B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250D8BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250D8C0: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8250D8C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8250D8C8: 4BFFFC41  bl 0x8250d508
	ctx.lr = 0x8250D8CC;
	sub_8250D508(ctx, base);
	// 8250D8CC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250D8D0: 488E43C1  bl 0x82df1c90
	ctx.lr = 0x8250D8D4;
	sub_82DF1C90(ctx, base);
	// 8250D8D4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250D8D8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8250D8DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250D8E0: 419A0024  beq cr6, 0x8250d904
	if ctx.cr[6].eq {
	pc = 0x8250D904; continue 'dispatch;
	}
	// 8250D8E4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8250D8E8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D8EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D8F0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D8F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D8F8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D8FC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D900: 4082FFE8  bne 0x8250d8e8
	if !ctx.cr[0].eq {
	pc = 0x8250D8E8; continue 'dispatch;
	}
	// 8250D904: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250D908: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250D90C: 4801B045  bl 0x82528950
	ctx.lr = 0x8250D910;
	sub_82528950(ctx, base);
	// 8250D910: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250D914: 419A000C  beq cr6, 0x8250d920
	if ctx.cr[6].eq {
	pc = 0x8250D920; continue 'dispatch;
	}
	// 8250D918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D91C: 4BDB2F75  bl 0x822c0890
	ctx.lr = 0x8250D920;
	sub_822C0890(ctx, base);
	// 8250D920: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250D924: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8250D928: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250D92C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8250D930: 419A0024  beq cr6, 0x8250d954
	if ctx.cr[6].eq {
	pc = 0x8250D954; continue 'dispatch;
	}
	// 8250D934: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8250D938: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250D93C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D940: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250D944: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250D948: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250D94C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250D950: 4082FFE8  bne 0x8250d938
	if !ctx.cr[0].eq {
	pc = 0x8250D938; continue 'dispatch;
	}
	// 8250D954: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250D958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250D95C: 488E62A5  bl 0x82df3c00
	ctx.lr = 0x8250D960;
	sub_82DF3C00(ctx, base);
	// 8250D960: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D964: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8250D968: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250D96C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8250D970: 4857DEA1  bl 0x82a8b810
	ctx.lr = 0x8250D974;
	sub_82A8B810(ctx, base);
	// 8250D974: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250D978: 488E5AB1  bl 0x82df3428
	ctx.lr = 0x8250D97C;
	sub_82DF3428(ctx, base);
	// 8250D97C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250D980: 419A000C  beq cr6, 0x8250d98c
	if ctx.cr[6].eq {
	pc = 0x8250D98C; continue 'dispatch;
	}
	// 8250D984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D988: 4BDB2F09  bl 0x822c0890
	ctx.lr = 0x8250D98C;
	sub_822C0890(ctx, base);
	// 8250D98C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8250D990: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250D994: 48677E1D  bl 0x82b857b0
	ctx.lr = 0x8250D998;
	sub_82B857B0(ctx, base);
	// 8250D998: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250D99C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8250D9A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250D9A4: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 8250D9A8: 48679019  bl 0x82b869c0
	ctx.lr = 0x8250D9AC;
	sub_82B869C0(ctx, base);
	// 8250D9AC: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8250D9B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D9B4: 419A0008  beq cr6, 0x8250d9bc
	if ctx.cr[6].eq {
	pc = 0x8250D9BC; continue 'dispatch;
	}
	// 8250D9B8: 4BDB2ED9  bl 0x822c0890
	ctx.lr = 0x8250D9BC;
	sub_822C0890(ctx, base);
	// 8250D9BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250D9C0: 488E5A69  bl 0x82df3428
	ctx.lr = 0x8250D9C4;
	sub_82DF3428(ctx, base);
	// 8250D9C4: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8250D9C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D9CC: 419A0008  beq cr6, 0x8250d9d4
	if ctx.cr[6].eq {
	pc = 0x8250D9D4; continue 'dispatch;
	}
	// 8250D9D0: 4BDB2EC1  bl 0x822c0890
	ctx.lr = 0x8250D9D4;
	sub_822C0890(ctx, base);
	// 8250D9D4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8250D9D8: 488E5A51  bl 0x82df3428
	ctx.lr = 0x8250D9DC;
	sub_82DF3428(ctx, base);
	// 8250D9DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8250D9E0: 419A000C  beq cr6, 0x8250d9ec
	if ctx.cr[6].eq {
	pc = 0x8250D9EC; continue 'dispatch;
	}
	// 8250D9E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250D9E8: 4BDB2EA9  bl 0x822c0890
	ctx.lr = 0x8250D9EC;
	sub_822C0890(ctx, base);
	// 8250D9EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250D9F0: 419A000C  beq cr6, 0x8250d9fc
	if ctx.cr[6].eq {
	pc = 0x8250D9FC; continue 'dispatch;
	}
	// 8250D9F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D9F8: 4BDB2E99  bl 0x822c0890
	ctx.lr = 0x8250D9FC;
	sub_822C0890(ctx, base);
	// 8250D9FC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8250DA00: 48C9A7AC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DA08 size=92
    let mut pc: u32 = 0x8250DA08;
    'dispatch: loop {
        match pc {
            0x8250DA08 => {
    //   block [0x8250DA08..0x8250DA64)
	// 8250DA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250DA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250DA14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DA18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250DA20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DA24: 386B0034  addi r3, r11, 0x34
	ctx.r[3].s64 = ctx.r[11].s64 + 52;
	// 8250DA28: 4BF84201  bl 0x82491c28
	ctx.lr = 0x8250DA2C;
	sub_82491C28(ctx, base);
	// 8250DA2C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DA30: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250DA34: 814A003C  lwz r10, 0x3c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 8250DA38: 48000010  b 0x8250da48
	pc = 0x8250DA48; continue 'dispatch;
	// 8250DA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250DA40: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250DA44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250DA48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8250DA4C: 409AFFF0  bne cr6, 0x8250da3c
	if !ctx.cr[6].eq {
	pc = 0x8250DA3C; continue 'dispatch;
	}
	// 8250DA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250DA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250DA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250DA5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250DA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DA68 size=228
    let mut pc: u32 = 0x8250DA68;
    'dispatch: loop {
        match pc {
            0x8250DA68 => {
    //   block [0x8250DA68..0x8250DB4C)
	// 8250DA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DA6C: 48C9A701  bl 0x831a816c
	ctx.lr = 0x8250DA70;
	sub_831A8130(ctx, base);
	// 8250DA70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DA74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DA78: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8250DA7C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8250DA80: 48471AD1  bl 0x8297f550
	ctx.lr = 0x8250DA84;
	sub_8297F550(ctx, base);
	// 8250DA84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250DA88: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250DA8C: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8250DA90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250DA94: 4BFFB82D  bl 0x825092c0
	ctx.lr = 0x8250DA98;
	sub_825092C0(ctx, base);
	// 8250DA98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250DA9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250DAA0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250DAA4: 4BDB255D  bl 0x822c0000
	ctx.lr = 0x8250DAA8;
	sub_822C0000(ctx, base);
	// 8250DAA8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8250DAAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250DAB0: 388B059C  addi r4, r11, 0x59c
	ctx.r[4].s64 = ctx.r[11].s64 + 1436;
	// 8250DAB4: 488E5F55  bl 0x82df3a08
	ctx.lr = 0x8250DAB8;
	sub_82DF3A08(ctx, base);
	// 8250DAB8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250DABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250DAC0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250DAC4: 4BFFDB8D  bl 0x8250b650
	ctx.lr = 0x8250DAC8;
	sub_8250B650(ctx, base);
	// 8250DAC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250DACC: 488E595D  bl 0x82df3428
	ctx.lr = 0x8250DAD0;
	sub_82DF3428(ctx, base);
	// 8250DAD0: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250DAD4: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250DAD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8250DADC: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8250DAE0: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8250DAE4: 419A0024  beq cr6, 0x8250db08
	if ctx.cr[6].eq {
	pc = 0x8250DB08; continue 'dispatch;
	}
	// 8250DAE8: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8250DAEC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250DAF0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250DAF4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250DAF8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250DAFC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250DB00: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250DB04: 4082FFE8  bne 0x8250daec
	if !ctx.cr[0].eq {
	pc = 0x8250DAEC; continue 'dispatch;
	}
	// 8250DB08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8250DB10: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8250DB14: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250DB18: 388B0110  addi r4, r11, 0x110
	ctx.r[4].s64 = ctx.r[11].s64 + 272;
	// 8250DB1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250DB20: 4BFFFA71  bl 0x8250d590
	ctx.lr = 0x8250DB24;
	sub_8250D590(ctx, base);
	// 8250DB24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DB28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250DB2C: 93AB010C  stw r29, 0x10c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(268 as u32), ctx.r[29].u32 ) };
	// 8250DB30: 488E4161  bl 0x82df1c90
	ctx.lr = 0x8250DB34;
	sub_82DF1C90(ctx, base);
	// 8250DB34: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8250DB38: 419A000C  beq cr6, 0x8250db44
	if ctx.cr[6].eq {
	pc = 0x8250DB44; continue 'dispatch;
	}
	// 8250DB3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DB40: 4BDB2D51  bl 0x822c0890
	ctx.lr = 0x8250DB44;
	sub_822C0890(ctx, base);
	// 8250DB44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250DB48: 48C9A674  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DB50 size=84
    let mut pc: u32 = 0x8250DB50;
    'dispatch: loop {
        match pc {
            0x8250DB50 => {
    //   block [0x8250DB50..0x8250DBA4)
	// 8250DB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250DB58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250DB5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DB60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DB64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DB68: 386B01B0  addi r3, r11, 0x1b0
	ctx.r[3].s64 = ctx.r[11].s64 + 432;
	// 8250DB6C: 4BFA766D  bl 0x824b51d8
	ctx.lr = 0x8250DB70;
	sub_824B51D8(ctx, base);
	// 8250DB70: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DB74: 816A01B4  lwz r11, 0x1b4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(436 as u32) ) } as u64;
	// 8250DB78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250DB7C: 419A0010  beq cr6, 0x8250db8c
	if ctx.cr[6].eq {
	pc = 0x8250DB8C; continue 'dispatch;
	}
	// 8250DB80: 814A01B8  lwz r10, 0x1b8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(440 as u32) ) } as u64;
	// 8250DB84: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8250DB88: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 8250DB8C: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 8250DB90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250DB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250DB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250DB9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250DBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DBA8 size=88
    let mut pc: u32 = 0x8250DBA8;
    'dispatch: loop {
        match pc {
            0x8250DBA8 => {
    //   block [0x8250DBA8..0x8250DC00)
	// 8250DBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250DBB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250DBB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DBB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DBBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250DBC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250DBC4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DBC8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DBCC: 4BFFF695  bl 0x8250d260
	ctx.lr = 0x8250DBD0;
	sub_8250D260(ctx, base);
	// 8250DBD0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250DBD4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DBD8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8250DBDC: 488E45AD  bl 0x82df2188
	ctx.lr = 0x8250DBE0;
	sub_82DF2188(ctx, base);
	// 8250DBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250DBE4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250DBE8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250DBEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250DBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250DBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250DBF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250DBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250DC00 size=416
    let mut pc: u32 = 0x8250DC00;
    'dispatch: loop {
        match pc {
            0x8250DC00 => {
    //   block [0x8250DC00..0x8250DDA0)
	// 8250DC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250DC08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250DC0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250DC10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DC14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DC18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250DC1C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8250DC20: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8250DC24: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DC28: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8250DC2C: 4828DFCD  bl 0x8279bbf8
	ctx.lr = 0x8250DC30;
	sub_8279BBF8(ctx, base);
	// 8250DC30: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8250DC34: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8250DC38: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8250DC3C: 485F8055  bl 0x82b05c90
	ctx.lr = 0x8250DC40;
	sub_82B05C90(ctx, base);
	// 8250DC40: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8250DC44: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8250DC48: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8250DC4C: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 8250DC50: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 8250DC54: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 8250DC58: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 8250DC5C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8250DC60: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 8250DC64: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250DC68: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 8250DC6C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 8250DC70: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 8250DC74: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250DC78: D01F005C  stfs f0, 0x5c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8250DC7C: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8250DC80: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8250DC84: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8250DC88: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8250DC8C: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8250DC90: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8250DC94: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 8250DC98: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 8250DC9C: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 8250DCA0: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 8250DCA4: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 8250DCA8: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8250DCAC: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 8250DCB0: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 8250DCB4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8250DCB8: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 8250DCBC: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 8250DCC0: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 8250DCC4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8250DCC8: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 8250DCCC: 488E5425  bl 0x82df30f0
	ctx.lr = 0x8250DCD0;
	sub_82DF30F0(ctx, base);
	// 8250DCD0: 93DF00B0  stw r30, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 8250DCD4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8250DCD8: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 8250DCDC: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 8250DCE0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8250DCE4: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 8250DCE8: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 8250DCEC: 93DF00D8  stw r30, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 8250DCF0: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 8250DCF4: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 8250DCF8: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 8250DCFC: 93DF00FC  stw r30, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 8250DD00: 93DF010C  stw r30, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[30].u32 ) };
	// 8250DD04: 93DF0114  stw r30, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[30].u32 ) };
	// 8250DD08: 93DF0118  stw r30, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u32 ) };
	// 8250DD0C: 93DF011C  stw r30, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[30].u32 ) };
	// 8250DD10: 93DF0120  stw r30, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[30].u32 ) };
	// 8250DD14: 93DF016C  stw r30, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[30].u32 ) };
	// 8250DD18: 93DF0170  stw r30, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[30].u32 ) };
	// 8250DD1C: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 8250DD20: 93DF0178  stw r30, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[30].u32 ) };
	// 8250DD24: 93DF017C  stw r30, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[30].u32 ) };
	// 8250DD28: 93DF0180  stw r30, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[30].u32 ) };
	// 8250DD2C: 93DF0184  stw r30, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[30].u32 ) };
	// 8250DD30: 93DF0188  stw r30, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[30].u32 ) };
	// 8250DD34: 93DF018C  stw r30, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[30].u32 ) };
	// 8250DD38: 93DF0190  stw r30, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[30].u32 ) };
	// 8250DD3C: 93DF0194  stw r30, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[30].u32 ) };
	// 8250DD40: 93DF0198  stw r30, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[30].u32 ) };
	// 8250DD44: 93DF019C  stw r30, 0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(412 as u32), ctx.r[30].u32 ) };
	// 8250DD48: 93DF01A0  stw r30, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[30].u32 ) };
	// 8250DD4C: 93DF01A4  stw r30, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[30].u32 ) };
	// 8250DD50: 93DF01A8  stw r30, 0x1a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[30].u32 ) };
	// 8250DD54: 93DF01AC  stw r30, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[30].u32 ) };
	// 8250DD58: 93DF01B4  stw r30, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[30].u32 ) };
	// 8250DD5C: 387F01D8  addi r3, r31, 0x1d8
	ctx.r[3].s64 = ctx.r[31].s64 + 472;
	// 8250DD60: 93DF01B8  stw r30, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[30].u32 ) };
	// 8250DD64: 93DF01BC  stw r30, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[30].u32 ) };
	// 8250DD68: 93DF01C0  stw r30, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[30].u32 ) };
	// 8250DD6C: 93DF01C4  stw r30, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[30].u32 ) };
	// 8250DD70: 93DF01C8  stw r30, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[30].u32 ) };
	// 8250DD74: 93DF01CC  stw r30, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 8250DD78: 4BF4E711  bl 0x8245c488
	ctx.lr = 0x8250DD7C;
	sub_8245C488(ctx, base);
	// 8250DD7C: 93DF0224  stw r30, 0x224(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(548 as u32), ctx.r[30].u32 ) };
	// 8250DD80: 9BDF0228  stb r30, 0x228(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[30].u8 ) };
	// 8250DD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250DD88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250DD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250DD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250DD94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250DD98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250DD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DDA0 size=128
    let mut pc: u32 = 0x8250DDA0;
    'dispatch: loop {
        match pc {
            0x8250DDA0 => {
    //   block [0x8250DDA0..0x8250DE20)
	// 8250DDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250DDA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250DDAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250DDB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DDB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DDB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250DDBC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8250DDC0: 488E4719  bl 0x82df24d8
	ctx.lr = 0x8250DDC4;
	sub_82DF24D8(ctx, base);
	// 8250DDC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250DDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8250DDCC: 396B1B08  addi r11, r11, 0x1b08
	ctx.r[11].s64 = ctx.r[11].s64 + 6920;
	// 8250DDD0: 388A199C  addi r4, r10, 0x199c
	ctx.r[4].s64 = ctx.r[10].s64 + 6556;
	// 8250DDD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250DDD8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250DDDC: 38A00134  li r5, 0x134
	ctx.r[5].s64 = 308;
	// 8250DDE0: 3860022C  li r3, 0x22c
	ctx.r[3].s64 = 556;
	// 8250DDE4: 488E4605  bl 0x82df23e8
	ctx.lr = 0x8250DDE8;
	sub_82DF23E8(ctx, base);
	// 8250DDE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250DDEC: 41820010  beq 0x8250ddfc
	if ctx.cr[0].eq {
	pc = 0x8250DDFC; continue 'dispatch;
	}
	// 8250DDF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250DDF4: 4BFFFE0D  bl 0x8250dc00
	ctx.lr = 0x8250DDF8;
	sub_8250DC00(ctx, base);
	// 8250DDF8: 48000008  b 0x8250de00
	pc = 0x8250DE00; continue 'dispatch;
	// 8250DDFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250DE00: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8250DE04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250DE08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250DE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250DE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250DE14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250DE18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250DE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DE20 size=516
    let mut pc: u32 = 0x8250DE20;
    'dispatch: loop {
        match pc {
            0x8250DE20 => {
    //   block [0x8250DE20..0x8250E024)
	// 8250DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DE24: 48C9A349  bl 0x831a816c
	ctx.lr = 0x8250DE28;
	sub_831A8130(ctx, base);
	// 8250DE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DE2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DE30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8250DE34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250DE38: 394A1B08  addi r10, r10, 0x1b08
	ctx.r[10].s64 = ctx.r[10].s64 + 6920;
	// 8250DE3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DE40: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250DE44: 396B01C0  addi r11, r11, 0x1c0
	ctx.r[11].s64 = ctx.r[11].s64 + 448;
	// 8250DE48: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250DE4C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DE50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250DE54: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DE58: 419A0008  beq cr6, 0x8250de60
	if ctx.cr[6].eq {
	pc = 0x8250DE60; continue 'dispatch;
	}
	// 8250DE5C: 4BDB2A35  bl 0x822c0890
	ctx.lr = 0x8250DE60;
	sub_822C0890(ctx, base);
	// 8250DE60: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8250DE64: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8250DE68: 916A7018  stw r11, 0x7018(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28696 as u32), ctx.r[11].u32 ) };
	// 8250DE6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DE70: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250DE74: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DE78: 48000030  b 0x8250dea8
	pc = 0x8250DEA8; continue 'dispatch;
	// 8250DE7C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DE80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250DE84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DE88: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250DE8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DE90: 4E800421  bctrl
	ctx.lr = 0x8250DE94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DE94: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DE98: 48002A31  bl 0x825108c8
	ctx.lr = 0x8250DE9C;
	sub_825108C8(ctx, base);
	// 8250DE9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DEA0: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DEA4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250DEA8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250DEAC: 409AFFD0  bne cr6, 0x8250de7c
	if !ctx.cr[6].eq {
	pc = 0x8250DE7C; continue 'dispatch;
	}
	// 8250DEB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DEB4: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8250DEB8: 482F8601  bl 0x828064b8
	ctx.lr = 0x8250DEBC;
	sub_828064B8(ctx, base);
	// 8250DEBC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DEC0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250DEC4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DEC8: 4801C329  bl 0x8252a1f0
	ctx.lr = 0x8250DECC;
	sub_8252A1F0(ctx, base);
	// 8250DECC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DED0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250DED4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8250DED8: 4BFDDCB9  bl 0x824ebb90
	ctx.lr = 0x8250DEDC;
	sub_824EBB90(ctx, base);
	// 8250DEDC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250DEE0: 488E3DB1  bl 0x82df1c90
	ctx.lr = 0x8250DEE4;
	sub_82DF1C90(ctx, base);
	// 8250DEE4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DEE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250DEEC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250DEF0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DEF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250DEF8: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DEFC: 419A0008  beq cr6, 0x8250df04
	if ctx.cr[6].eq {
	pc = 0x8250DF04; continue 'dispatch;
	}
	// 8250DF00: 4BDB2991  bl 0x822c0890
	ctx.lr = 0x8250DF04;
	sub_822C0890(ctx, base);
	// 8250DF04: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DF08: 396B0084  addi r11, r11, 0x84
	ctx.r[11].s64 = ctx.r[11].s64 + 132;
	// 8250DF0C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250DF10: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DF14: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DF18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250DF1C: 419A0008  beq cr6, 0x8250df24
	if ctx.cr[6].eq {
	pc = 0x8250DF24; continue 'dispatch;
	}
	// 8250DF20: 4BDB2971  bl 0x822c0890
	ctx.lr = 0x8250DF24;
	sub_822C0890(ctx, base);
	// 8250DF24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DF28: 83AB0010  lwz r29, 0x10(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250DF2C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DF30: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8250DF34: 48000024  b 0x8250df58
	pc = 0x8250DF58; continue 'dispatch;
	// 8250DF38: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250DF3C: 4801A085  bl 0x82527fc0
	ctx.lr = 0x8250DF40;
	sub_82527FC0(ctx, base);
	// 8250DF40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250DF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250DF48: 4BFFA979  bl 0x825088c0
	ctx.lr = 0x8250DF4C;
	sub_825088C0(ctx, base);
	// 8250DF4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250DF50: 4BE93739  bl 0x823a1688
	ctx.lr = 0x8250DF54;
	sub_823A1688(ctx, base);
	// 8250DF54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250DF58: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8250DF5C: 409AFFDC  bne cr6, 0x8250df38
	if !ctx.cr[6].eq {
	pc = 0x8250DF38; continue 'dispatch;
	}
	// 8250DF60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DF64: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8250DF68: 48678A01  bl 0x82b86968
	ctx.lr = 0x8250DF6C;
	sub_82B86968(ctx, base);
	// 8250DF6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DF70: 396B0094  addi r11, r11, 0x94
	ctx.r[11].s64 = ctx.r[11].s64 + 148;
	// 8250DF74: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250DF78: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DF7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250DF80: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DF84: 419A0008  beq cr6, 0x8250df8c
	if ctx.cr[6].eq {
	pc = 0x8250DF8C; continue 'dispatch;
	}
	// 8250DF88: 4BDB2909  bl 0x822c0890
	ctx.lr = 0x8250DF8C;
	sub_822C0890(ctx, base);
	// 8250DF8C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DF90: 396B004C  addi r11, r11, 0x4c
	ctx.r[11].s64 = ctx.r[11].s64 + 76;
	// 8250DF94: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250DF98: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DF9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250DFA0: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DFA4: 419A0008  beq cr6, 0x8250dfac
	if ctx.cr[6].eq {
	pc = 0x8250DFAC; continue 'dispatch;
	}
	// 8250DFA8: 4BDB28E9  bl 0x822c0890
	ctx.lr = 0x8250DFAC;
	sub_822C0890(ctx, base);
	// 8250DFAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DFB0: 396B009C  addi r11, r11, 0x9c
	ctx.r[11].s64 = ctx.r[11].s64 + 156;
	// 8250DFB4: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250DFB8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DFBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250DFC0: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DFC4: 419A0008  beq cr6, 0x8250dfcc
	if ctx.cr[6].eq {
	pc = 0x8250DFCC; continue 'dispatch;
	}
	// 8250DFC8: 4BDB28C9  bl 0x822c0890
	ctx.lr = 0x8250DFCC;
	sub_822C0890(ctx, base);
	// 8250DFCC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DFD0: 806B00B0  lwz r3, 0xb0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8250DFD4: 4862D075  bl 0x82b3b048
	ctx.lr = 0x8250DFD8;
	sub_82B3B048(ctx, base);
	// 8250DFD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DFDC: 396B00B0  addi r11, r11, 0xb0
	ctx.r[11].s64 = ctx.r[11].s64 + 176;
	// 8250DFE0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250DFE4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DFE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250DFEC: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8250DFF0: 419A0008  beq cr6, 0x8250dff8
	if ctx.cr[6].eq {
	pc = 0x8250DFF8; continue 'dispatch;
	}
	// 8250DFF4: 4BDB289D  bl 0x822c0890
	ctx.lr = 0x8250DFF8;
	sub_822C0890(ctx, base);
	// 8250DFF8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DFFC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8250E000: 419A0014  beq cr6, 0x8250e014
	if ctx.cr[6].eq {
	pc = 0x8250E014; continue 'dispatch;
	}
	// 8250E004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E008: 4BFFF2E1  bl 0x8250d2e8
	ctx.lr = 0x8250E00C;
	sub_8250D2E8(ctx, base);
	// 8250E00C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E010: 488E43C9  bl 0x82df23d8
	ctx.lr = 0x8250E014;
	sub_82DF23D8(ctx, base);
	// 8250E014: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8250E018: 488E4539  bl 0x82df2550
	ctx.lr = 0x8250E01C;
	sub_82DF2550(ctx, base);
	// 8250E01C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250E020: 48C9A19C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E028 size=76
    let mut pc: u32 = 0x8250E028;
    'dispatch: loop {
        match pc {
            0x8250E028 => {
    //   block [0x8250E028..0x8250E074)
	// 8250E028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250E034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E03C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E040: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250E044: 4BFFFDDD  bl 0x8250de20
	ctx.lr = 0x8250E048;
	sub_8250DE20(ctx, base);
	// 8250E048: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250E04C: 4182000C  beq 0x8250e058
	if ctx.cr[0].eq {
	pc = 0x8250E058; continue 'dispatch;
	}
	// 8250E050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E054: 4BDB2215  bl 0x822c0268
	ctx.lr = 0x8250E058;
	sub_822C0268(ctx, base);
	// 8250E058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250E060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E068: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250E06C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E078 size=188
    let mut pc: u32 = 0x8250E078;
    'dispatch: loop {
        match pc {
            0x8250E078 => {
    //   block [0x8250E078..0x8250E134)
	// 8250E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E080: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250E084: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E08C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250E090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E094: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250E098: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250E09C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E0A0: 4BDB2899  bl 0x822c0938
	ctx.lr = 0x8250E0A4;
	sub_822C0938(ctx, base);
	// 8250E0A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250E0A8: 41820028  beq 0x8250e0d0
	if ctx.cr[0].eq {
	pc = 0x8250E0D0; continue 'dispatch;
	}
	// 8250E0AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250E0B0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250E0B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250E0B8: 392B18A0  addi r9, r11, 0x18a0
	ctx.r[9].s64 = ctx.r[11].s64 + 6304;
	// 8250E0BC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250E0C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250E0C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250E0C8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250E0CC: 48000008  b 0x8250e0d4
	pc = 0x8250E0D4; continue 'dispatch;
	// 8250E0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E0D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E0D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E0DC: 409A003C  bne cr6, 0x8250e118
	if !ctx.cr[6].eq {
	pc = 0x8250E118; continue 'dispatch;
	}
	// 8250E0E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250E0E4: 419A0014  beq cr6, 0x8250e0f8
	if ctx.cr[6].eq {
	pc = 0x8250E0F8; continue 'dispatch;
	}
	// 8250E0E8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8250E0EC: 480A2025  bl 0x825b0110
	ctx.lr = 0x8250E0F0;
	sub_825B0110(ctx, base);
	// 8250E0F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E0F4: 4BDB2175  bl 0x822c0268
	ctx.lr = 0x8250E0F8;
	sub_822C0268(ctx, base);
	// 8250E0F8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E0FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250E100: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E104: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250E108: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250E10C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250E110: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250E114: 4BDB1EED  bl 0x822c0000
	ctx.lr = 0x8250E118;
	sub_822C0000(ctx, base);
	// 8250E118: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E11C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250E120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250E12C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E138 size=64
    let mut pc: u32 = 0x8250E138;
    'dispatch: loop {
        match pc {
            0x8250E138 => {
    //   block [0x8250E138..0x8250E178)
	// 8250E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E148: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E14C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250E150: 419A0014  beq cr6, 0x8250e164
	if ctx.cr[6].eq {
	pc = 0x8250E164; continue 'dispatch;
	}
	// 8250E154: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8250E158: 480A1FB9  bl 0x825b0110
	ctx.lr = 0x8250E15C;
	sub_825B0110(ctx, base);
	// 8250E15C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E160: 4BDB2109  bl 0x822c0268
	ctx.lr = 0x8250E164;
	sub_822C0268(ctx, base);
	// 8250E164: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250E168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E170: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E178 size=112
    let mut pc: u32 = 0x8250E178;
    'dispatch: loop {
        match pc {
            0x8250E178 => {
    //   block [0x8250E178..0x8250E1E8)
	// 8250E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250E184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E18C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250E190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E194: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8250E198: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250E19C: 4BFFFEDD  bl 0x8250e078
	ctx.lr = 0x8250E1A0;
	sub_8250E078(ctx, base);
	// 8250E1A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250E1A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250E1A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8250E1AC: 4BDB1E55  bl 0x822c0000
	ctx.lr = 0x8250E1B0;
	sub_822C0000(ctx, base);
	// 8250E1B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250E1B4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250E1B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E1BC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E1C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250E1C4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250E1C8: 419A0008  beq cr6, 0x8250e1d0
	if ctx.cr[6].eq {
	pc = 0x8250E1D0; continue 'dispatch;
	}
	// 8250E1CC: 4BDB26C5  bl 0x822c0890
	ctx.lr = 0x8250E1D0;
	sub_822C0890(ctx, base);
	// 8250E1D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250E1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E1DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250E1E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E1E8 size=188
    let mut pc: u32 = 0x8250E1E8;
    'dispatch: loop {
        match pc {
            0x8250E1E8 => {
    //   block [0x8250E1E8..0x8250E2A4)
	// 8250E1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E1F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250E1F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E1F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E1FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250E200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E204: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250E208: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250E20C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E210: 4BDB2729  bl 0x822c0938
	ctx.lr = 0x8250E214;
	sub_822C0938(ctx, base);
	// 8250E214: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250E218: 41820028  beq 0x8250e240
	if ctx.cr[0].eq {
	pc = 0x8250E240; continue 'dispatch;
	}
	// 8250E21C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250E220: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250E224: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250E228: 392B1774  addi r9, r11, 0x1774
	ctx.r[9].s64 = ctx.r[11].s64 + 6004;
	// 8250E22C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250E230: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250E234: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250E238: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250E23C: 48000008  b 0x8250e244
	pc = 0x8250E244; continue 'dispatch;
	// 8250E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E244: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E24C: 409A003C  bne cr6, 0x8250e288
	if !ctx.cr[6].eq {
	pc = 0x8250E288; continue 'dispatch;
	}
	// 8250E250: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250E254: 419A0014  beq cr6, 0x8250e268
	if ctx.cr[6].eq {
	pc = 0x8250E268; continue 'dispatch;
	}
	// 8250E258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E25C: 4BFFF94D  bl 0x8250dba8
	ctx.lr = 0x8250E260;
	sub_8250DBA8(ctx, base);
	// 8250E260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E264: 4BDB2005  bl 0x822c0268
	ctx.lr = 0x8250E268;
	sub_822C0268(ctx, base);
	// 8250E268: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E26C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250E270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E274: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250E278: 816BBE1C  lwz r11, -0x41e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16868 as u32) ) } as u64;
	// 8250E27C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250E280: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250E284: 4BDB1D7D  bl 0x822c0000
	ctx.lr = 0x8250E288;
	sub_822C0000(ctx, base);
	// 8250E288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E28C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250E290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250E29C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E2A8 size=64
    let mut pc: u32 = 0x8250E2A8;
    'dispatch: loop {
        match pc {
            0x8250E2A8 => {
    //   block [0x8250E2A8..0x8250E2E8)
	// 8250E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E2B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E2B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E2B8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E2BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250E2C0: 419A0014  beq cr6, 0x8250e2d4
	if ctx.cr[6].eq {
	pc = 0x8250E2D4; continue 'dispatch;
	}
	// 8250E2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E2C8: 4BFFF8E1  bl 0x8250dba8
	ctx.lr = 0x8250E2CC;
	sub_8250DBA8(ctx, base);
	// 8250E2CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E2D0: 4BDB1F99  bl 0x822c0268
	ctx.lr = 0x8250E2D4;
	sub_822C0268(ctx, base);
	// 8250E2D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250E2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E2E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E2E8 size=140
    let mut pc: u32 = 0x8250E2E8;
    'dispatch: loop {
        match pc {
            0x8250E2E8 => {
    //   block [0x8250E2E8..0x8250E374)
	// 8250E2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E2F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250E2F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E2F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E2FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250E300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E308: 388B199C  addi r4, r11, 0x199c
	ctx.r[4].s64 = ctx.r[11].s64 + 6556;
	// 8250E30C: 38A00690  li r5, 0x690
	ctx.r[5].s64 = 1680;
	// 8250E310: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250E314: 4BDB20C5  bl 0x822c03d8
	ctx.lr = 0x8250E318;
	sub_822C03D8(ctx, base);
	// 8250E318: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8250E31C: 41820014  beq 0x8250e330
	if ctx.cr[0].eq {
	pc = 0x8250E330; continue 'dispatch;
	}
	// 8250E320: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8250E324: 4828D8D5  bl 0x8279bbf8
	ctx.lr = 0x8250E328;
	sub_8279BBF8(ctx, base);
	// 8250E328: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250E32C: 48000008  b 0x8250e334
	pc = 0x8250E334; continue 'dispatch;
	// 8250E330: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250E334: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E338: 386B0114  addi r3, r11, 0x114
	ctx.r[3].s64 = ctx.r[11].s64 + 276;
	// 8250E33C: 4BFFFE3D  bl 0x8250e178
	ctx.lr = 0x8250E340;
	sub_8250E178(ctx, base);
	// 8250E340: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E344: 806B0114  lwz r3, 0x114(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8250E348: 482A4251  bl 0x827b2598
	ctx.lr = 0x8250E34C;
	sub_827B2598(ctx, base);
	// 8250E34C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E354: 808B0114  lwz r4, 0x114(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8250E358: 4BFFEE09  bl 0x8250d160
	ctx.lr = 0x8250E35C;
	sub_8250D160(ctx, base);
	// 8250E35C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250E360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E368: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250E36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250E378 size=3972
    let mut pc: u32 = 0x8250E378;
    'dispatch: loop {
        match pc {
            0x8250E378 => {
    //   block [0x8250E378..0x8250F2FC)
	// 8250E378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E37C: 48C99DDD  bl 0x831a8158
	ctx.lr = 0x8250E380;
	sub_831A8130(ctx, base);
	// 8250E380: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E384: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8250E388: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8250E38C: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 8250E390: 3CE08332  lis r7, -0x7cce
	ctx.r[7].s64 = -2093875200;
	// 8250E394: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250E398: C00A964C  lfs f0, -0x69b4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250E39C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E3A0: C1A9BBE8  lfs f13, -0x4418(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17432 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250E3A4: 3B4B199C  addi r26, r11, 0x199c
	ctx.r[26].s64 = ctx.r[11].s64 + 6556;
	// 8250E3A8: D008A558  stfs f0, -0x5aa8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-23208 as u32), tmp.u32 ) };
	// 8250E3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E3B0: D1A7A55C  stfs f13, -0x5aa4(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-23204 as u32), tmp.u32 ) };
	// 8250E3B4: 38A00145  li r5, 0x145
	ctx.r[5].s64 = 325;
	// 8250E3B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250E3BC: 386001D0  li r3, 0x1d0
	ctx.r[3].s64 = 464;
	// 8250E3C0: 488E4029  bl 0x82df23e8
	ctx.lr = 0x8250E3C4;
	sub_82DF23E8(ctx, base);
	// 8250E3C4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8250E3C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250E3CC: 41820010  beq 0x8250e3dc
	if ctx.cr[0].eq {
	pc = 0x8250E3DC; continue 'dispatch;
	}
	// 8250E3D0: 480A31A9  bl 0x825b1578
	ctx.lr = 0x8250E3D4;
	sub_825B1578(ctx, base);
	// 8250E3D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250E3D8: 48000008  b 0x8250e3e0
	pc = 0x8250E3E0; continue 'dispatch;
	// 8250E3DC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250E3E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E3E4: 386B0044  addi r3, r11, 0x44
	ctx.r[3].s64 = ctx.r[11].s64 + 68;
	// 8250E3E8: 4BFFC989  bl 0x8250ad70
	ctx.lr = 0x8250E3EC;
	sub_8250AD70(ctx, base);
	// 8250E3EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E3F0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8250E3F4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E3F8: 4801BDF9  bl 0x8252a1f0
	ctx.lr = 0x8250E3FC;
	sub_8252A1F0(ctx, base);
	// 8250E3FC: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 8250E400: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8250E404: 4BFDE1A5  bl 0x824ec5a8
	ctx.lr = 0x8250E408;
	sub_824EC5A8(ctx, base);
	// 8250E408: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E40C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E410: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 8250E414: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250E418: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250E41C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250E420: 4BDB6041  bl 0x822c4460
	ctx.lr = 0x8250E424;
	sub_822C4460(ctx, base);
	// 8250E424: 806100BC  lwz r3, 0xbc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250E428: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250E42C: 419A0008  beq cr6, 0x8250e434
	if ctx.cr[6].eq {
	pc = 0x8250E434; continue 'dispatch;
	}
	// 8250E430: 4BDB2461  bl 0x822c0890
	ctx.lr = 0x8250E434;
	sub_822C0890(ctx, base);
	// 8250E434: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E438: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8250E43C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8250E440: 4BFDB1D1  bl 0x824e9610
	ctx.lr = 0x8250E444;
	sub_824E9610(ctx, base);
	// 8250E444: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E448: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250E44C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E450: 409A002C  bne cr6, 0x8250e47c
	if !ctx.cr[6].eq {
	pc = 0x8250E47C; continue 'dispatch;
	}
	// 8250E454: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8250E458: 4BFDA801  bl 0x824e8c58
	ctx.lr = 0x8250E45C;
	sub_824E8C58(ctx, base);
	// 8250E45C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8250E460: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E464: 388A0004  addi r4, r10, 4
	ctx.r[4].s64 = ctx.r[10].s64 + 4;
	// 8250E468: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8250E46C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E470: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250E474: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250E478: 4BDB5FE9  bl 0x822c4460
	ctx.lr = 0x8250E47C;
	sub_822C4460(ctx, base);
	// 8250E47C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E480: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8250E484: 808BE254  lwz r4, -0x1dac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7596 as u32) ) } as u64;
	// 8250E488: 488E5581  bl 0x82df3a08
	ctx.lr = 0x8250E48C;
	sub_82DF3A08(ctx, base);
	// 8250E48C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E490: 38810088  addi r4, r1, 0x88
	ctx.r[4].s64 = ctx.r[1].s64 + 136;
	// 8250E494: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E498: 48956709  bl 0x82e64ba0
	ctx.lr = 0x8250E49C;
	sub_82E64BA0(ctx, base);
	// 8250E49C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8250E4A0: 488E4F89  bl 0x82df3428
	ctx.lr = 0x8250E4A4;
	sub_82DF3428(ctx, base);
	// 8250E4A4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E4A8: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250E4AC: 808BE25C  lwz r4, -0x1da4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7588 as u32) ) } as u64;
	// 8250E4B0: 488E5559  bl 0x82df3a08
	ctx.lr = 0x8250E4B4;
	sub_82DF3A08(ctx, base);
	// 8250E4B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E4B8: 3881006C  addi r4, r1, 0x6c
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	// 8250E4BC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E4C0: 489566E1  bl 0x82e64ba0
	ctx.lr = 0x8250E4C4;
	sub_82E64BA0(ctx, base);
	// 8250E4C4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8250E4C8: 488E4F61  bl 0x82df3428
	ctx.lr = 0x8250E4CC;
	sub_82DF3428(ctx, base);
	// 8250E4CC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E4D0: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250E4D4: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 8250E4D8: 488E5531  bl 0x82df3a08
	ctx.lr = 0x8250E4DC;
	sub_82DF3A08(ctx, base);
	// 8250E4DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E4E0: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 8250E4E4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E4E8: 489566B9  bl 0x82e64ba0
	ctx.lr = 0x8250E4EC;
	sub_82E64BA0(ctx, base);
	// 8250E4EC: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8250E4F0: 488E4F39  bl 0x82df3428
	ctx.lr = 0x8250E4F4;
	sub_82DF3428(ctx, base);
	// 8250E4F4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E4F8: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 8250E4FC: 808BE258  lwz r4, -0x1da8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7592 as u32) ) } as u64;
	// 8250E500: 488E5509  bl 0x82df3a08
	ctx.lr = 0x8250E504;
	sub_82DF3A08(ctx, base);
	// 8250E504: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E508: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 8250E50C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E510: 48956691  bl 0x82e64ba0
	ctx.lr = 0x8250E514;
	sub_82E64BA0(ctx, base);
	// 8250E514: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 8250E518: 488E4F11  bl 0x82df3428
	ctx.lr = 0x8250E51C;
	sub_82DF3428(ctx, base);
	// 8250E51C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E520: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250E524: 808BE260  lwz r4, -0x1da0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7584 as u32) ) } as u64;
	// 8250E528: 488E54E1  bl 0x82df3a08
	ctx.lr = 0x8250E52C;
	sub_82DF3A08(ctx, base);
	// 8250E52C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E530: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8250E534: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E538: 48956669  bl 0x82e64ba0
	ctx.lr = 0x8250E53C;
	sub_82E64BA0(ctx, base);
	// 8250E53C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250E540: 488E4EE9  bl 0x82df3428
	ctx.lr = 0x8250E544;
	sub_82DF3428(ctx, base);
	// 8250E544: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E548: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8250E54C: 808BE264  lwz r4, -0x1d9c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7580 as u32) ) } as u64;
	// 8250E550: 488E54B9  bl 0x82df3a08
	ctx.lr = 0x8250E554;
	sub_82DF3A08(ctx, base);
	// 8250E554: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E558: 38810098  addi r4, r1, 0x98
	ctx.r[4].s64 = ctx.r[1].s64 + 152;
	// 8250E55C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E560: 48956641  bl 0x82e64ba0
	ctx.lr = 0x8250E564;
	sub_82E64BA0(ctx, base);
	// 8250E564: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8250E568: 488E4EC1  bl 0x82df3428
	ctx.lr = 0x8250E56C;
	sub_82DF3428(ctx, base);
	// 8250E56C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E570: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250E574: 808BE268  lwz r4, -0x1d98(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7576 as u32) ) } as u64;
	// 8250E578: 488E5491  bl 0x82df3a08
	ctx.lr = 0x8250E57C;
	sub_82DF3A08(ctx, base);
	// 8250E57C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E580: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8250E584: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E588: 48956619  bl 0x82e64ba0
	ctx.lr = 0x8250E58C;
	sub_82E64BA0(ctx, base);
	// 8250E58C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8250E590: 488E4E99  bl 0x82df3428
	ctx.lr = 0x8250E594;
	sub_82DF3428(ctx, base);
	// 8250E594: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E598: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 8250E59C: 808BE26C  lwz r4, -0x1d94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7572 as u32) ) } as u64;
	// 8250E5A0: 488E5469  bl 0x82df3a08
	ctx.lr = 0x8250E5A4;
	sub_82DF3A08(ctx, base);
	// 8250E5A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E5A8: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 8250E5AC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E5B0: 489565F1  bl 0x82e64ba0
	ctx.lr = 0x8250E5B4;
	sub_82E64BA0(ctx, base);
	// 8250E5B4: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 8250E5B8: 488E4E71  bl 0x82df3428
	ctx.lr = 0x8250E5BC;
	sub_82DF3428(ctx, base);
	// 8250E5BC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E5C0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250E5C4: 808BE278  lwz r4, -0x1d88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7560 as u32) ) } as u64;
	// 8250E5C8: 488E5441  bl 0x82df3a08
	ctx.lr = 0x8250E5CC;
	sub_82DF3A08(ctx, base);
	// 8250E5CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E5D0: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8250E5D4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E5D8: 489565C9  bl 0x82e64ba0
	ctx.lr = 0x8250E5DC;
	sub_82E64BA0(ctx, base);
	// 8250E5DC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8250E5E0: 488E4E49  bl 0x82df3428
	ctx.lr = 0x8250E5E4;
	sub_82DF3428(ctx, base);
	// 8250E5E4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250E5E8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8250E5EC: 808BE27C  lwz r4, -0x1d84(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7556 as u32) ) } as u64;
	// 8250E5F0: 488E5419  bl 0x82df3a08
	ctx.lr = 0x8250E5F4;
	sub_82DF3A08(ctx, base);
	// 8250E5F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E5F8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8250E5FC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E600: 489565A1  bl 0x82e64ba0
	ctx.lr = 0x8250E604;
	sub_82E64BA0(ctx, base);
	// 8250E604: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8250E608: 488E4E21  bl 0x82df3428
	ctx.lr = 0x8250E60C;
	sub_82DF3428(ctx, base);
	// 8250E60C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250E610: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E614: 38A0016C  li r5, 0x16c
	ctx.r[5].s64 = 364;
	// 8250E618: 386000C4  li r3, 0xc4
	ctx.r[3].s64 = 196;
	// 8250E61C: 488E3DCD  bl 0x82df23e8
	ctx.lr = 0x8250E620;
	sub_82DF23E8(ctx, base);
	// 8250E620: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250E624: 41820014  beq 0x8250e638
	if ctx.cr[0].eq {
	pc = 0x8250E638; continue 'dispatch;
	}
	// 8250E628: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250E62C: 4BFF9E1D  bl 0x82508448
	ctx.lr = 0x8250E630;
	sub_82508448(ctx, base);
	// 8250E630: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250E634: 48000008  b 0x8250e63c
	pc = 0x8250E63C; continue 'dispatch;
	// 8250E638: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 8250E63C: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8250E640: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250E644: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250E648: 4BFFA959  bl 0x82508fa0
	ctx.lr = 0x8250E64C;
	sub_82508FA0(ctx, base);
	// 8250E64C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250E650: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250E654: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250E658: 4BDB19A9  bl 0x822c0000
	ctx.lr = 0x8250E65C;
	sub_822C0000(ctx, base);
	// 8250E65C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8250E660: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E664: 3B6B059C  addi r27, r11, 0x59c
	ctx.r[27].s64 = ctx.r[11].s64 + 1436;
	// 8250E668: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250E66C: 488E539D  bl 0x82df3a08
	ctx.lr = 0x8250E670;
	sub_82DF3A08(ctx, base);
	// 8250E670: 83810064  lwz r28, 0x64(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8250E674: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250E678: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8250E67C: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8250E680: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250E684: 419A0024  beq cr6, 0x8250e6a8
	if ctx.cr[6].eq {
	pc = 0x8250E6A8; continue 'dispatch;
	}
	// 8250E688: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8250E68C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250E690: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E694: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250E698: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250E69C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250E6A0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E6A4: 4082FFE8  bne 0x8250e68c
	if !ctx.cr[0].eq {
	pc = 0x8250E68C; continue 'dispatch;
	}
	// 8250E6A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250E6AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250E6B0: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8250E6B4: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250E6B8: 4BFFCF99  bl 0x8250b650
	ctx.lr = 0x8250E6BC;
	sub_8250B650(ctx, base);
	// 8250E6BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250E6C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E6C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250E6C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E6CC: 4BFFEE3D  bl 0x8250d508
	ctx.lr = 0x8250E6D0;
	sub_8250D508(ctx, base);
	// 8250E6D0: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8250E6D4: 488E35BD  bl 0x82df1c90
	ctx.lr = 0x8250E6D8;
	sub_82DF1C90(ctx, base);
	// 8250E6D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E6DC: 488E4D4D  bl 0x82df3428
	ctx.lr = 0x8250E6E0;
	sub_82DF3428(ctx, base);
	// 8250E6E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250E6E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E6E8: 3BAB1B0C  addi r29, r11, 0x1b0c
	ctx.r[29].s64 = ctx.r[11].s64 + 6924;
	// 8250E6EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250E6F0: 488E5319  bl 0x82df3a08
	ctx.lr = 0x8250E6F4;
	sub_82DF3A08(ctx, base);
	// 8250E6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250E6F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250E6FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E700: 4BFFEF19  bl 0x8250d618
	ctx.lr = 0x8250E704;
	sub_8250D618(ctx, base);
	// 8250E704: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E708: 488E4D21  bl 0x82df3428
	ctx.lr = 0x8250E70C;
	sub_82DF3428(ctx, base);
	// 8250E70C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250E710: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E714: 38A00173  li r5, 0x173
	ctx.r[5].s64 = 371;
	// 8250E718: 38600160  li r3, 0x160
	ctx.r[3].s64 = 352;
	// 8250E71C: 488E3CCD  bl 0x82df23e8
	ctx.lr = 0x8250E720;
	sub_82DF23E8(ctx, base);
	// 8250E720: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8250E724: 41820020  beq 0x8250e744
	if ctx.cr[0].eq {
	pc = 0x8250E744; continue 'dispatch;
	}
	// 8250E728: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8250E72C: 4BFDA52D  bl 0x824e8c58
	ctx.lr = 0x8250E730;
	sub_824E8C58(ctx, base);
	// 8250E730: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250E734: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E738: 4BDBA1D1  bl 0x822c8908
	ctx.lr = 0x8250E73C;
	sub_822C8908(ctx, base);
	// 8250E73C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250E740: 48000008  b 0x8250e748
	pc = 0x8250E748; continue 'dispatch;
	// 8250E744: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250E748: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E74C: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 8250E750: 4BFFC691  bl 0x8250ade0
	ctx.lr = 0x8250E754;
	sub_8250ADE0(ctx, base);
	// 8250E754: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E758: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 8250E75C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E760: 419A0010  beq cr6, 0x8250e770
	if ctx.cr[6].eq {
	pc = 0x8250E770; continue 'dispatch;
	}
	// 8250E764: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 8250E768: 916100A0  stw r11, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8250E76C: 48000008  b 0x8250e774
	pc = 0x8250E774; continue 'dispatch;
	// 8250E770: 932100A0  stw r25, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[25].u32 ) };
	// 8250E774: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250E778: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E77C: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 8250E780: 419A0024  beq cr6, 0x8250e7a4
	if ctx.cr[6].eq {
	pc = 0x8250E7A4; continue 'dispatch;
	}
	// 8250E784: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250E788: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250E78C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E790: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250E794: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250E798: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250E79C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E7A0: 4082FFE8  bne 0x8250e788
	if !ctx.cr[0].eq {
	pc = 0x8250E788; continue 'dispatch;
	}
	// 8250E7A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250E7A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E7AC: 488E525D  bl 0x82df3a08
	ctx.lr = 0x8250E7B0;
	sub_82DF3A08(ctx, base);
	// 8250E7B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250E7B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250E7B8: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 8250E7BC: 4BFFCE95  bl 0x8250b650
	ctx.lr = 0x8250E7C0;
	sub_8250B650(ctx, base);
	// 8250E7C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E7C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E7C8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8250E7CC: 409A0008  bne cr6, 0x8250e7d4
	if !ctx.cr[6].eq {
	pc = 0x8250E7D4; continue 'dispatch;
	}
	// 8250E7D0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8250E7D4: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 8250E7D8: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 8250E7DC: 809E70B0  lwz r4, 0x70b0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28848 as u32) ) } as u64;
	// 8250E7E0: 4801B3A9  bl 0x82529b88
	ctx.lr = 0x8250E7E4;
	sub_82529B88(ctx, base);
	// 8250E7E4: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 8250E7E8: 488E34A9  bl 0x82df1c90
	ctx.lr = 0x8250E7EC;
	sub_82DF1C90(ctx, base);
	// 8250E7EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E7F0: 488E4C39  bl 0x82df3428
	ctx.lr = 0x8250E7F4;
	sub_82DF3428(ctx, base);
	// 8250E7F4: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8250E7F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250E7FC: 419A0008  beq cr6, 0x8250e804
	if ctx.cr[6].eq {
	pc = 0x8250E804; continue 'dispatch;
	}
	// 8250E800: 4BDB2091  bl 0x822c0890
	ctx.lr = 0x8250E804;
	sub_822C0890(ctx, base);
	// 8250E804: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250E808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E80C: 488E51FD  bl 0x82df3a08
	ctx.lr = 0x8250E810;
	sub_82DF3A08(ctx, base);
	// 8250E810: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E814: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250E818: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250E81C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250E820: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8250E824: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250E828: 419A0024  beq cr6, 0x8250e84c
	if ctx.cr[6].eq {
	pc = 0x8250E84C; continue 'dispatch;
	}
	// 8250E82C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250E830: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250E834: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E838: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250E83C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250E840: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250E844: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E848: 4082FFE8  bne 0x8250e830
	if !ctx.cr[0].eq {
	pc = 0x8250E830; continue 'dispatch;
	}
	// 8250E84C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250E850: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250E854: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 8250E858: 3B010058  addi r24, r1, 0x58
	ctx.r[24].s64 = ctx.r[1].s64 + 88;
	// 8250E85C: 4BFFCDF5  bl 0x8250b650
	ctx.lr = 0x8250E860;
	sub_8250B650(ctx, base);
	// 8250E860: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250E864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E868: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8250E86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E870: 4BFFEC99  bl 0x8250d508
	ctx.lr = 0x8250E874;
	sub_8250D508(ctx, base);
	// 8250E874: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 8250E878: 488E3419  bl 0x82df1c90
	ctx.lr = 0x8250E87C;
	sub_82DF1C90(ctx, base);
	// 8250E87C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E880: 488E4BA9  bl 0x82df3428
	ctx.lr = 0x8250E884;
	sub_82DF3428(ctx, base);
	// 8250E884: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250E888: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E88C: 38A0017C  li r5, 0x17c
	ctx.r[5].s64 = 380;
	// 8250E890: 38600158  li r3, 0x158
	ctx.r[3].s64 = 344;
	// 8250E894: 488E3B55  bl 0x82df23e8
	ctx.lr = 0x8250E898;
	sub_82DF23E8(ctx, base);
	// 8250E898: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250E89C: 41820010  beq 0x8250e8ac
	if ctx.cr[0].eq {
	pc = 0x8250E8AC; continue 'dispatch;
	}
	// 8250E8A0: 480B0FF9  bl 0x825bf898
	ctx.lr = 0x8250E8A4;
	sub_825BF898(ctx, base);
	// 8250E8A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250E8A8: 48000008  b 0x8250e8b0
	pc = 0x8250E8B0; continue 'dispatch;
	// 8250E8AC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250E8B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E8B4: 386B004C  addi r3, r11, 0x4c
	ctx.r[3].s64 = ctx.r[11].s64 + 76;
	// 8250E8B8: 4BFFC599  bl 0x8250ae50
	ctx.lr = 0x8250E8BC;
	sub_8250AE50(ctx, base);
	// 8250E8BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250E8C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E8C4: 488E5145  bl 0x82df3a08
	ctx.lr = 0x8250E8C8;
	sub_82DF3A08(ctx, base);
	// 8250E8C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E8CC: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250E8D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250E8D4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250E8D8: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8250E8DC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250E8E0: 419A0024  beq cr6, 0x8250e904
	if ctx.cr[6].eq {
	pc = 0x8250E904; continue 'dispatch;
	}
	// 8250E8E4: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250E8E8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250E8EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E8F0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250E8F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250E8F8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250E8FC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E900: 4082FFE8  bne 0x8250e8e8
	if !ctx.cr[0].eq {
	pc = 0x8250E8E8; continue 'dispatch;
	}
	// 8250E904: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250E908: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250E90C: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 8250E910: 3B010058  addi r24, r1, 0x58
	ctx.r[24].s64 = ctx.r[1].s64 + 88;
	// 8250E914: 4BFFCD3D  bl 0x8250b650
	ctx.lr = 0x8250E918;
	sub_8250B650(ctx, base);
	// 8250E918: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250E91C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E920: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8250E924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250E928: 4BFFEBE1  bl 0x8250d508
	ctx.lr = 0x8250E92C;
	sub_8250D508(ctx, base);
	// 8250E92C: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 8250E930: 488E3361  bl 0x82df1c90
	ctx.lr = 0x8250E934;
	sub_82DF1C90(ctx, base);
	// 8250E934: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E938: 488E4AF1  bl 0x82df3428
	ctx.lr = 0x8250E93C;
	sub_82DF3428(ctx, base);
	// 8250E93C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E940: 816A004C  lwz r11, 0x4c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8250E944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E948: 419A0010  beq cr6, 0x8250e958
	if ctx.cr[6].eq {
	pc = 0x8250E958; continue 'dispatch;
	}
	// 8250E94C: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 8250E950: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8250E954: 48000008  b 0x8250e95c
	pc = 0x8250E95C; continue 'dispatch;
	// 8250E958: 932100B0  stw r25, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[25].u32 ) };
	// 8250E95C: 816A0050  lwz r11, 0x50(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250E960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E964: 916100B4  stw r11, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 8250E968: 419A0024  beq cr6, 0x8250e98c
	if ctx.cr[6].eq {
	pc = 0x8250E98C; continue 'dispatch;
	}
	// 8250E96C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250E970: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250E974: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E978: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250E97C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250E980: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250E984: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250E988: 4082FFE8  bne 0x8250e970
	if !ctx.cr[0].eq {
	pc = 0x8250E970; continue 'dispatch;
	}
	// 8250E98C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250E990: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E994: 488E5075  bl 0x82df3a08
	ctx.lr = 0x8250E998;
	sub_82DF3A08(ctx, base);
	// 8250E998: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250E99C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250E9A0: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8250E9A4: 4BFFCCAD  bl 0x8250b650
	ctx.lr = 0x8250E9A8;
	sub_8250B650(ctx, base);
	// 8250E9A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E9AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E9B0: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8250E9B4: 409A0008  bne cr6, 0x8250e9bc
	if !ctx.cr[6].eq {
	pc = 0x8250E9BC; continue 'dispatch;
	}
	// 8250E9B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8250E9BC: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 8250E9C0: 809E70B0  lwz r4, 0x70b0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28848 as u32) ) } as u64;
	// 8250E9C4: 4801B1C5  bl 0x82529b88
	ctx.lr = 0x8250E9C8;
	sub_82529B88(ctx, base);
	// 8250E9C8: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8250E9CC: 488E32C5  bl 0x82df1c90
	ctx.lr = 0x8250E9D0;
	sub_82DF1C90(ctx, base);
	// 8250E9D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E9D4: 488E4A55  bl 0x82df3428
	ctx.lr = 0x8250E9D8;
	sub_82DF3428(ctx, base);
	// 8250E9D8: 806100B4  lwz r3, 0xb4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8250E9DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250E9E0: 419A0008  beq cr6, 0x8250e9e8
	if ctx.cr[6].eq {
	pc = 0x8250E9E8; continue 'dispatch;
	}
	// 8250E9E4: 4BDB1EAD  bl 0x822c0890
	ctx.lr = 0x8250E9E8;
	sub_822C0890(ctx, base);
	// 8250E9E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E9EC: 806B004C  lwz r3, 0x4c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8250E9F0: 480B0971  bl 0x825bf360
	ctx.lr = 0x8250E9F4;
	sub_825BF360(ctx, base);
	// 8250E9F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250E9F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E9FC: 488E500D  bl 0x82df3a08
	ctx.lr = 0x8250EA00;
	sub_82DF3A08(ctx, base);
	// 8250EA00: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EA04: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250EA08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250EA0C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250EA10: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250EA14: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250EA18: 419A0024  beq cr6, 0x8250ea3c
	if ctx.cr[6].eq {
	pc = 0x8250EA3C; continue 'dispatch;
	}
	// 8250EA1C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250EA20: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250EA24: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EA28: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250EA2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250EA30: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250EA34: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EA38: 4082FFE8  bne 0x8250ea20
	if !ctx.cr[0].eq {
	pc = 0x8250EA20; continue 'dispatch;
	}
	// 8250EA3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250EA40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250EA44: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 8250EA48: 3B010058  addi r24, r1, 0x58
	ctx.r[24].s64 = ctx.r[1].s64 + 88;
	// 8250EA4C: 4BFFCC05  bl 0x8250b650
	ctx.lr = 0x8250EA50;
	sub_8250B650(ctx, base);
	// 8250EA50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EA54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250EA58: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8250EA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EA60: 4BFFEAA9  bl 0x8250d508
	ctx.lr = 0x8250EA64;
	sub_8250D508(ctx, base);
	// 8250EA64: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 8250EA68: 488E3229  bl 0x82df1c90
	ctx.lr = 0x8250EA6C;
	sub_82DF1C90(ctx, base);
	// 8250EA6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EA70: 488E49B9  bl 0x82df3428
	ctx.lr = 0x8250EA74;
	sub_82DF3428(ctx, base);
	// 8250EA74: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EA78: 816A0044  lwz r11, 0x44(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250EA7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250EA80: 419A0010  beq cr6, 0x8250ea90
	if ctx.cr[6].eq {
	pc = 0x8250EA90; continue 'dispatch;
	}
	// 8250EA84: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 8250EA88: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 8250EA8C: 48000008  b 0x8250ea94
	pc = 0x8250EA94; continue 'dispatch;
	// 8250EA90: 932100A8  stw r25, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[25].u32 ) };
	// 8250EA94: 816A0048  lwz r11, 0x48(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250EA98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250EA9C: 916100AC  stw r11, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 8250EAA0: 419A0024  beq cr6, 0x8250eac4
	if ctx.cr[6].eq {
	pc = 0x8250EAC4; continue 'dispatch;
	}
	// 8250EAA4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250EAA8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250EAAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EAB0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250EAB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250EAB8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250EABC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EAC0: 4082FFE8  bne 0x8250eaa8
	if !ctx.cr[0].eq {
	pc = 0x8250EAA8; continue 'dispatch;
	}
	// 8250EAC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250EAC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EACC: 488E4F3D  bl 0x82df3a08
	ctx.lr = 0x8250EAD0;
	sub_82DF3A08(ctx, base);
	// 8250EAD0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250EAD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250EAD8: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 8250EADC: 4BFFCB75  bl 0x8250b650
	ctx.lr = 0x8250EAE0;
	sub_8250B650(ctx, base);
	// 8250EAE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250EAE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250EAE8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8250EAEC: 409A0008  bne cr6, 0x8250eaf4
	if !ctx.cr[6].eq {
	pc = 0x8250EAF4; continue 'dispatch;
	}
	// 8250EAF0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8250EAF4: 38A100A8  addi r5, r1, 0xa8
	ctx.r[5].s64 = ctx.r[1].s64 + 168;
	// 8250EAF8: 809E70B0  lwz r4, 0x70b0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28848 as u32) ) } as u64;
	// 8250EAFC: 4801B08D  bl 0x82529b88
	ctx.lr = 0x8250EB00;
	sub_82529B88(ctx, base);
	// 8250EB00: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 8250EB04: 488E318D  bl 0x82df1c90
	ctx.lr = 0x8250EB08;
	sub_82DF1C90(ctx, base);
	// 8250EB08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EB0C: 488E491D  bl 0x82df3428
	ctx.lr = 0x8250EB10;
	sub_82DF3428(ctx, base);
	// 8250EB10: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 8250EB14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250EB18: 419A0008  beq cr6, 0x8250eb20
	if ctx.cr[6].eq {
	pc = 0x8250EB20; continue 'dispatch;
	}
	// 8250EB1C: 4BDB1D75  bl 0x822c0890
	ctx.lr = 0x8250EB20;
	sub_822C0890(ctx, base);
	// 8250EB20: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250EB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EB28: 38A00188  li r5, 0x188
	ctx.r[5].s64 = 392;
	// 8250EB2C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8250EB30: 4BDB18A9  bl 0x822c03d8
	ctx.lr = 0x8250EB34;
	sub_822C03D8(ctx, base);
	// 8250EB34: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8250EB38: 41820010  beq 0x8250eb48
	if ctx.cr[0].eq {
	pc = 0x8250EB48; continue 'dispatch;
	}
	// 8250EB3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250EB40: 482D89D9  bl 0x827e7518
	ctx.lr = 0x8250EB44;
	sub_827E7518(ctx, base);
	// 8250EB44: 48000008  b 0x8250eb4c
	pc = 0x8250EB4C; continue 'dispatch;
	// 8250EB48: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 8250EB4C: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8250EB50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250EB54: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250EB58: 4BFFF691  bl 0x8250e1e8
	ctx.lr = 0x8250EB5C;
	sub_8250E1E8(ctx, base);
	// 8250EB5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250EB60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250EB64: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8250EB68: 4BDB1499  bl 0x822c0000
	ctx.lr = 0x8250EB6C;
	sub_822C0000(ctx, base);
	// 8250EB6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EB70: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250EB74: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 8250EB78: 396B0074  addi r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 + 116;
	// 8250EB7C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8250EB80: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250EB84: 4BDB58DD  bl 0x822c4460
	ctx.lr = 0x8250EB88;
	sub_822C4460(ctx, base);
	// 8250EB88: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8250EB8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250EB90: 419A0008  beq cr6, 0x8250eb98
	if ctx.cr[6].eq {
	pc = 0x8250EB98; continue 'dispatch;
	}
	// 8250EB94: 4BDB1CFD  bl 0x822c0890
	ctx.lr = 0x8250EB98;
	sub_822C0890(ctx, base);
	// 8250EB98: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250EB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EBA0: 38A0018E  li r5, 0x18e
	ctx.r[5].s64 = 398;
	// 8250EBA4: 386000C8  li r3, 0xc8
	ctx.r[3].s64 = 200;
	// 8250EBA8: 488E3841  bl 0x82df23e8
	ctx.lr = 0x8250EBAC;
	sub_82DF23E8(ctx, base);
	// 8250EBAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250EBB0: 41820010  beq 0x8250ebc0
	if ctx.cr[0].eq {
	pc = 0x8250EBC0; continue 'dispatch;
	}
	// 8250EBB4: 4862DEB5  bl 0x82b3ca68
	ctx.lr = 0x8250EBB8;
	sub_82B3CA68(ctx, base);
	// 8250EBB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EBBC: 48000008  b 0x8250ebc4
	pc = 0x8250EBC4; continue 'dispatch;
	// 8250EBC0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250EBC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EBC8: 386B00B0  addi r3, r11, 0xb0
	ctx.r[3].s64 = ctx.r[11].s64 + 176;
	// 8250EBCC: 4BFFC2F5  bl 0x8250aec0
	ctx.lr = 0x8250EBD0;
	sub_8250AEC0(ctx, base);
	// 8250EBD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250EBD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EBD8: 488E4E31  bl 0x82df3a08
	ctx.lr = 0x8250EBDC;
	sub_82DF3A08(ctx, base);
	// 8250EBDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EBE0: 814B00B4  lwz r10, 0xb4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 8250EBE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250EBE8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250EBEC: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8250EBF0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250EBF4: 419A0024  beq cr6, 0x8250ec18
	if ctx.cr[6].eq {
	pc = 0x8250EC18; continue 'dispatch;
	}
	// 8250EBF8: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250EBFC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250EC00: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EC04: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250EC08: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250EC0C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250EC10: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EC14: 4082FFE8  bne 0x8250ebfc
	if !ctx.cr[0].eq {
	pc = 0x8250EBFC; continue 'dispatch;
	}
	// 8250EC18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250EC1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250EC20: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 8250EC24: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250EC28: 4BFFCA29  bl 0x8250b650
	ctx.lr = 0x8250EC2C;
	sub_8250B650(ctx, base);
	// 8250EC2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EC30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250EC34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250EC38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EC3C: 4BFFE8CD  bl 0x8250d508
	ctx.lr = 0x8250EC40;
	sub_8250D508(ctx, base);
	// 8250EC40: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 8250EC44: 488E304D  bl 0x82df1c90
	ctx.lr = 0x8250EC48;
	sub_82DF1C90(ctx, base);
	// 8250EC48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EC4C: 488E47DD  bl 0x82df3428
	ctx.lr = 0x8250EC50;
	sub_82DF3428(ctx, base);
	// 8250EC50: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250EC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EC58: 38A00192  li r5, 0x192
	ctx.r[5].s64 = 402;
	// 8250EC5C: 386000DC  li r3, 0xdc
	ctx.r[3].s64 = 220;
	// 8250EC60: 488E3789  bl 0x82df23e8
	ctx.lr = 0x8250EC64;
	sub_82DF23E8(ctx, base);
	// 8250EC64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250EC68: 41820010  beq 0x8250ec78
	if ctx.cr[0].eq {
	pc = 0x8250EC78; continue 'dispatch;
	}
	// 8250EC6C: 48669C1D  bl 0x82b78888
	ctx.lr = 0x8250EC70;
	sub_82B78888(ctx, base);
	// 8250EC70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EC74: 48000008  b 0x8250ec7c
	pc = 0x8250EC7C; continue 'dispatch;
	// 8250EC78: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250EC7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EC80: 386B00C0  addi r3, r11, 0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + 192;
	// 8250EC84: 4BFFC2AD  bl 0x8250af30
	ctx.lr = 0x8250EC88;
	sub_8250AF30(ctx, base);
	// 8250EC88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EC8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250EC90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EC94: 814B00C0  lwz r10, 0xc0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8250EC98: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 8250EC9C: 488E4D6D  bl 0x82df3a08
	ctx.lr = 0x8250ECA0;
	sub_82DF3A08(ctx, base);
	// 8250ECA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250ECA4: 814B00C4  lwz r10, 0xc4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 8250ECA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250ECAC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250ECB0: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8250ECB4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250ECB8: 419A0024  beq cr6, 0x8250ecdc
	if ctx.cr[6].eq {
	pc = 0x8250ECDC; continue 'dispatch;
	}
	// 8250ECBC: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250ECC0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250ECC4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250ECC8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250ECCC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250ECD0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250ECD4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250ECD8: 4082FFE8  bne 0x8250ecc0
	if !ctx.cr[0].eq {
	pc = 0x8250ECC0; continue 'dispatch;
	}
	// 8250ECDC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250ECE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250ECE4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8250ECE8: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250ECEC: 4BFFC965  bl 0x8250b650
	ctx.lr = 0x8250ECF0;
	sub_8250B650(ctx, base);
	// 8250ECF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250ECF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250ECF8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250ECFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250ED00: 4BFFE809  bl 0x8250d508
	ctx.lr = 0x8250ED04;
	sub_8250D508(ctx, base);
	// 8250ED04: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8250ED08: 488E2F89  bl 0x82df1c90
	ctx.lr = 0x8250ED0C;
	sub_82DF1C90(ctx, base);
	// 8250ED0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250ED10: 488E4719  bl 0x82df3428
	ctx.lr = 0x8250ED14;
	sub_82DF3428(ctx, base);
	// 8250ED14: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8250ED18: 419A000C  beq cr6, 0x8250ed24
	if ctx.cr[6].eq {
	pc = 0x8250ED24; continue 'dispatch;
	}
	// 8250ED1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250ED20: 4BDB1B71  bl 0x822c0890
	ctx.lr = 0x8250ED24;
	sub_822C0890(ctx, base);
	// 8250ED24: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8250ED28: 488E2F69  bl 0x82df1c90
	ctx.lr = 0x8250ED2C;
	sub_82DF1C90(ctx, base);
	// 8250ED2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250ED30: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8250ED34: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250ED38: 4801B4B9  bl 0x8252a1f0
	ctx.lr = 0x8250ED3C;
	sub_8252A1F0(ctx, base);
	// 8250ED3C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250ED40: 4BFD9F39  bl 0x824e8c78
	ctx.lr = 0x8250ED44;
	sub_824E8C78(ctx, base);
	// 8250ED44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250ED48: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250ED4C: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8250ED50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250ED54: 916A00CC  stw r11, 0xcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 8250ED58: 488E2F39  bl 0x82df1c90
	ctx.lr = 0x8250ED5C;
	sub_82DF1C90(ctx, base);
	// 8250ED5C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250ED60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250ED64: 38A0019E  li r5, 0x19e
	ctx.r[5].s64 = 414;
	// 8250ED68: 38600290  li r3, 0x290
	ctx.r[3].s64 = 656;
	// 8250ED6C: 488E367D  bl 0x82df23e8
	ctx.lr = 0x8250ED70;
	sub_82DF23E8(ctx, base);
	// 8250ED70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250ED74: 41820010  beq 0x8250ed84
	if ctx.cr[0].eq {
	pc = 0x8250ED84; continue 'dispatch;
	}
	// 8250ED78: 48080921  bl 0x8258f698
	ctx.lr = 0x8250ED7C;
	sub_8258F698(ctx, base);
	// 8250ED7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250ED80: 48000008  b 0x8250ed88
	pc = 0x8250ED88; continue 'dispatch;
	// 8250ED84: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250ED88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250ED8C: 386B016C  addi r3, r11, 0x16c
	ctx.r[3].s64 = ctx.r[11].s64 + 364;
	// 8250ED90: 4BFFC211  bl 0x8250afa0
	ctx.lr = 0x8250ED94;
	sub_8250AFA0(ctx, base);
	// 8250ED94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250ED98: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250ED9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EDA0: 814B016C  lwz r10, 0x16c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(364 as u32) ) } as u64;
	// 8250EDA4: 914B0174  stw r10, 0x174(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(372 as u32), ctx.r[10].u32 ) };
	// 8250EDA8: 488E4C61  bl 0x82df3a08
	ctx.lr = 0x8250EDAC;
	sub_82DF3A08(ctx, base);
	// 8250EDAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EDB0: 814B0170  lwz r10, 0x170(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(368 as u32) ) } as u64;
	// 8250EDB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250EDB8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250EDBC: 816B016C  lwz r11, 0x16c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(364 as u32) ) } as u64;
	// 8250EDC0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250EDC4: 419A0024  beq cr6, 0x8250ede8
	if ctx.cr[6].eq {
	pc = 0x8250EDE8; continue 'dispatch;
	}
	// 8250EDC8: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250EDCC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250EDD0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EDD4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250EDD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250EDDC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250EDE0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EDE4: 4082FFE8  bne 0x8250edcc
	if !ctx.cr[0].eq {
	pc = 0x8250EDCC; continue 'dispatch;
	}
	// 8250EDE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250EDEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250EDF0: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 8250EDF4: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250EDF8: 4BFFC859  bl 0x8250b650
	ctx.lr = 0x8250EDFC;
	sub_8250B650(ctx, base);
	// 8250EDFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EE00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250EE04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250EE08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EE0C: 4BFFE6FD  bl 0x8250d508
	ctx.lr = 0x8250EE10;
	sub_8250D508(ctx, base);
	// 8250EE10: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 8250EE14: 488E2E7D  bl 0x82df1c90
	ctx.lr = 0x8250EE18;
	sub_82DF1C90(ctx, base);
	// 8250EE18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EE1C: 488E460D  bl 0x82df3428
	ctx.lr = 0x8250EE20;
	sub_82DF3428(ctx, base);
	// 8250EE20: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250EE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EE28: 38A001A5  li r5, 0x1a5
	ctx.r[5].s64 = 421;
	// 8250EE2C: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 8250EE30: 488E35B9  bl 0x82df23e8
	ctx.lr = 0x8250EE34;
	sub_82DF23E8(ctx, base);
	// 8250EE34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250EE38: 41820010  beq 0x8250ee48
	if ctx.cr[0].eq {
	pc = 0x8250EE48; continue 'dispatch;
	}
	// 8250EE3C: 4800BEA5  bl 0x8251ace0
	ctx.lr = 0x8250EE40;
	sub_8251ACE0(ctx, base);
	// 8250EE40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EE44: 48000008  b 0x8250ee4c
	pc = 0x8250EE4C; continue 'dispatch;
	// 8250EE48: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250EE4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EE50: 386B0178  addi r3, r11, 0x178
	ctx.r[3].s64 = ctx.r[11].s64 + 376;
	// 8250EE54: 4BFFC1BD  bl 0x8250b010
	ctx.lr = 0x8250EE58;
	sub_8250B010(ctx, base);
	// 8250EE58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EE5C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250EE60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EE64: 814B0178  lwz r10, 0x178(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(376 as u32) ) } as u64;
	// 8250EE68: 914B0180  stw r10, 0x180(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(384 as u32), ctx.r[10].u32 ) };
	// 8250EE6C: 488E4B9D  bl 0x82df3a08
	ctx.lr = 0x8250EE70;
	sub_82DF3A08(ctx, base);
	// 8250EE70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EE74: 814B017C  lwz r10, 0x17c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(380 as u32) ) } as u64;
	// 8250EE78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250EE7C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250EE80: 816B0178  lwz r11, 0x178(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(376 as u32) ) } as u64;
	// 8250EE84: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250EE88: 419A0024  beq cr6, 0x8250eeac
	if ctx.cr[6].eq {
	pc = 0x8250EEAC; continue 'dispatch;
	}
	// 8250EE8C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250EE90: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250EE94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EE98: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250EE9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250EEA0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250EEA4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EEA8: 4082FFE8  bne 0x8250ee90
	if !ctx.cr[0].eq {
	pc = 0x8250EE90; continue 'dispatch;
	}
	// 8250EEAC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250EEB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250EEB4: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8250EEB8: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250EEBC: 4BFFC795  bl 0x8250b650
	ctx.lr = 0x8250EEC0;
	sub_8250B650(ctx, base);
	// 8250EEC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EEC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250EEC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250EECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EED0: 4BFFE639  bl 0x8250d508
	ctx.lr = 0x8250EED4;
	sub_8250D508(ctx, base);
	// 8250EED4: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8250EED8: 488E2DB9  bl 0x82df1c90
	ctx.lr = 0x8250EEDC;
	sub_82DF1C90(ctx, base);
	// 8250EEDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EEE0: 488E4549  bl 0x82df3428
	ctx.lr = 0x8250EEE4;
	sub_82DF3428(ctx, base);
	// 8250EEE4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250EEE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EEEC: 38A001AC  li r5, 0x1ac
	ctx.r[5].s64 = 428;
	// 8250EEF0: 38600108  li r3, 0x108
	ctx.r[3].s64 = 264;
	// 8250EEF4: 488E34F5  bl 0x82df23e8
	ctx.lr = 0x8250EEF8;
	sub_82DF23E8(ctx, base);
	// 8250EEF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250EEFC: 41820010  beq 0x8250ef0c
	if ctx.cr[0].eq {
	pc = 0x8250EF0C; continue 'dispatch;
	}
	// 8250EF00: 4800B8E1  bl 0x8251a7e0
	ctx.lr = 0x8250EF04;
	sub_8251A7E0(ctx, base);
	// 8250EF04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EF08: 48000008  b 0x8250ef10
	pc = 0x8250EF10; continue 'dispatch;
	// 8250EF0C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250EF10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EF14: 386B0184  addi r3, r11, 0x184
	ctx.r[3].s64 = ctx.r[11].s64 + 388;
	// 8250EF18: 4BFFC169  bl 0x8250b080
	ctx.lr = 0x8250EF1C;
	sub_8250B080(ctx, base);
	// 8250EF1C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250EF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EF24: 488E4AE5  bl 0x82df3a08
	ctx.lr = 0x8250EF28;
	sub_82DF3A08(ctx, base);
	// 8250EF28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EF2C: 814B0188  lwz r10, 0x188(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(392 as u32) ) } as u64;
	// 8250EF30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250EF34: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250EF38: 816B0184  lwz r11, 0x184(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(388 as u32) ) } as u64;
	// 8250EF3C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250EF40: 419A0024  beq cr6, 0x8250ef64
	if ctx.cr[6].eq {
	pc = 0x8250EF64; continue 'dispatch;
	}
	// 8250EF44: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250EF48: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250EF4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EF50: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250EF54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250EF58: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250EF5C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250EF60: 4082FFE8  bne 0x8250ef48
	if !ctx.cr[0].eq {
	pc = 0x8250EF48; continue 'dispatch;
	}
	// 8250EF64: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250EF68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250EF6C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8250EF70: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250EF74: 4BFFC6DD  bl 0x8250b650
	ctx.lr = 0x8250EF78;
	sub_8250B650(ctx, base);
	// 8250EF78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EF7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250EF80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250EF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EF88: 4BFFE581  bl 0x8250d508
	ctx.lr = 0x8250EF8C;
	sub_8250D508(ctx, base);
	// 8250EF8C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8250EF90: 488E2D01  bl 0x82df1c90
	ctx.lr = 0x8250EF94;
	sub_82DF1C90(ctx, base);
	// 8250EF94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EF98: 488E4491  bl 0x82df3428
	ctx.lr = 0x8250EF9C;
	sub_82DF3428(ctx, base);
	// 8250EF9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EFA0: 816B0184  lwz r11, 0x184(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(388 as u32) ) } as u64;
	// 8250EFA4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8250EFA8: 48AFA011  bl 0x83008fb8
	ctx.lr = 0x8250EFAC;
	sub_83008FB8(ctx, base);
	// 8250EFAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EFB0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250EFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250EFB8: 38A001B3  li r5, 0x1b3
	ctx.r[5].s64 = 435;
	// 8250EFBC: 906B018C  stw r3, 0x18c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(396 as u32), ctx.r[3].u32 ) };
	// 8250EFC0: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 8250EFC4: 488E3425  bl 0x82df23e8
	ctx.lr = 0x8250EFC8;
	sub_82DF23E8(ctx, base);
	// 8250EFC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250EFCC: 41820010  beq 0x8250efdc
	if ctx.cr[0].eq {
	pc = 0x8250EFDC; continue 'dispatch;
	}
	// 8250EFD0: 4864F751  bl 0x82b5e720
	ctx.lr = 0x8250EFD4;
	sub_82B5E720(ctx, base);
	// 8250EFD4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250EFD8: 48000008  b 0x8250efe0
	pc = 0x8250EFE0; continue 'dispatch;
	// 8250EFDC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250EFE0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EFE4: 386B01A8  addi r3, r11, 0x1a8
	ctx.r[3].s64 = ctx.r[11].s64 + 424;
	// 8250EFE8: 4BFFC109  bl 0x8250b0f0
	ctx.lr = 0x8250EFEC;
	sub_8250B0F0(ctx, base);
	// 8250EFEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250EFF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250EFF4: 488E4A15  bl 0x82df3a08
	ctx.lr = 0x8250EFF8;
	sub_82DF3A08(ctx, base);
	// 8250EFF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EFFC: 814B01AC  lwz r10, 0x1ac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(428 as u32) ) } as u64;
	// 8250F000: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250F004: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250F008: 816B01A8  lwz r11, 0x1a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(424 as u32) ) } as u64;
	// 8250F00C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250F010: 419A0024  beq cr6, 0x8250f034
	if ctx.cr[6].eq {
	pc = 0x8250F034; continue 'dispatch;
	}
	// 8250F014: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250F018: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250F01C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F020: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250F024: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250F028: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250F02C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F030: 4082FFE8  bne 0x8250f018
	if !ctx.cr[0].eq {
	pc = 0x8250F018; continue 'dispatch;
	}
	// 8250F034: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250F038: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250F03C: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8250F040: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250F044: 4BFFC60D  bl 0x8250b650
	ctx.lr = 0x8250F048;
	sub_8250B650(ctx, base);
	// 8250F048: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250F04C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F050: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250F054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F058: 4BFFE4B1  bl 0x8250d508
	ctx.lr = 0x8250F05C;
	sub_8250D508(ctx, base);
	// 8250F05C: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8250F060: 488E2C31  bl 0x82df1c90
	ctx.lr = 0x8250F064;
	sub_82DF1C90(ctx, base);
	// 8250F064: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F068: 488E43C1  bl 0x82df3428
	ctx.lr = 0x8250F06C;
	sub_82DF3428(ctx, base);
	// 8250F06C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250F070: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F074: 38A001BB  li r5, 0x1bb
	ctx.r[5].s64 = 443;
	// 8250F078: 386001B0  li r3, 0x1b0
	ctx.r[3].s64 = 432;
	// 8250F07C: 488E336D  bl 0x82df23e8
	ctx.lr = 0x8250F080;
	sub_82DF23E8(ctx, base);
	// 8250F080: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250F084: 41820010  beq 0x8250f094
	if ctx.cr[0].eq {
	pc = 0x8250F094; continue 'dispatch;
	}
	// 8250F088: 48084269  bl 0x825932f0
	ctx.lr = 0x8250F08C;
	sub_825932F0(ctx, base);
	// 8250F08C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250F090: 48000008  b 0x8250f098
	pc = 0x8250F098; continue 'dispatch;
	// 8250F094: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250F098: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F09C: 386B009C  addi r3, r11, 0x9c
	ctx.r[3].s64 = ctx.r[11].s64 + 156;
	// 8250F0A0: 4BFFC0C1  bl 0x8250b160
	ctx.lr = 0x8250F0A4;
	sub_8250B160(ctx, base);
	// 8250F0A4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250F0A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F0AC: 488E495D  bl 0x82df3a08
	ctx.lr = 0x8250F0B0;
	sub_82DF3A08(ctx, base);
	// 8250F0B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F0B4: 814B00A0  lwz r10, 0xa0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8250F0B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250F0BC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250F0C0: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8250F0C4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250F0C8: 419A0024  beq cr6, 0x8250f0ec
	if ctx.cr[6].eq {
	pc = 0x8250F0EC; continue 'dispatch;
	}
	// 8250F0CC: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250F0D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250F0D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F0D8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250F0DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250F0E0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250F0E4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F0E8: 4082FFE8  bne 0x8250f0d0
	if !ctx.cr[0].eq {
	pc = 0x8250F0D0; continue 'dispatch;
	}
	// 8250F0EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250F0F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250F0F4: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 8250F0F8: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250F0FC: 4BFFC555  bl 0x8250b650
	ctx.lr = 0x8250F100;
	sub_8250B650(ctx, base);
	// 8250F100: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250F104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F108: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250F10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F110: 4BFFE3F9  bl 0x8250d508
	ctx.lr = 0x8250F114;
	sub_8250D508(ctx, base);
	// 8250F114: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 8250F118: 488E2B79  bl 0x82df1c90
	ctx.lr = 0x8250F11C;
	sub_82DF1C90(ctx, base);
	// 8250F11C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F120: 488E4309  bl 0x82df3428
	ctx.lr = 0x8250F124;
	sub_82DF3428(ctx, base);
	// 8250F124: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250F128: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F12C: 38A001C3  li r5, 0x1c3
	ctx.r[5].s64 = 451;
	// 8250F130: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250F134: 4BDB12A5  bl 0x822c03d8
	ctx.lr = 0x8250F138;
	sub_822C03D8(ctx, base);
	// 8250F138: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8250F13C: 41820014  beq 0x8250f150
	if ctx.cr[0].eq {
	pc = 0x8250F150; continue 'dispatch;
	}
	// 8250F140: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8250F144: 4828CAB5  bl 0x8279bbf8
	ctx.lr = 0x8250F148;
	sub_8279BBF8(ctx, base);
	// 8250F148: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250F14C: 48000008  b 0x8250f154
	pc = 0x8250F154; continue 'dispatch;
	// 8250F150: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250F154: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F158: 386B011C  addi r3, r11, 0x11c
	ctx.r[3].s64 = ctx.r[11].s64 + 284;
	// 8250F15C: 4BFFF01D  bl 0x8250e178
	ctx.lr = 0x8250F160;
	sub_8250E178(ctx, base);
	// 8250F160: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F164: 806B011C  lwz r3, 0x11c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(284 as u32) ) } as u64;
	// 8250F168: 482A3431  bl 0x827b2598
	ctx.lr = 0x8250F16C;
	sub_827B2598(ctx, base);
	// 8250F16C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F174: 808B011C  lwz r4, 0x11c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(284 as u32) ) } as u64;
	// 8250F178: 4BFFDFE9  bl 0x8250d160
	ctx.lr = 0x8250F17C;
	sub_8250D160(ctx, base);
	// 8250F17C: 39600124  li r11, 0x124
	ctx.r[11].s64 = 292;
	// 8250F180: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F184: 7F2A592E  stwx r25, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[25].u32) };
	// 8250F188: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250F18C: 2F0B016C  cmpwi cr6, r11, 0x16c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 364, &mut ctx.xer);
	// 8250F190: 4198FFF0  blt cr6, 0x8250f180
	if ctx.cr[6].lt {
	pc = 0x8250F180; continue 'dispatch;
	}
	// 8250F194: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250F198: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F19C: 38A001D6  li r5, 0x1d6
	ctx.r[5].s64 = 470;
	// 8250F1A0: 38600158  li r3, 0x158
	ctx.r[3].s64 = 344;
	// 8250F1A4: 488E3245  bl 0x82df23e8
	ctx.lr = 0x8250F1A8;
	sub_82DF23E8(ctx, base);
	// 8250F1A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250F1AC: 41820010  beq 0x8250f1bc
	if ctx.cr[0].eq {
	pc = 0x8250F1BC; continue 'dispatch;
	}
	// 8250F1B0: 480883F9  bl 0x825975a8
	ctx.lr = 0x8250F1B4;
	sub_825975A8(ctx, base);
	// 8250F1B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250F1B8: 48000008  b 0x8250f1c0
	pc = 0x8250F1C0; continue 'dispatch;
	// 8250F1BC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8250F1C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F1C4: 386B01C8  addi r3, r11, 0x1c8
	ctx.r[3].s64 = ctx.r[11].s64 + 456;
	// 8250F1C8: 4BFFC009  bl 0x8250b1d0
	ctx.lr = 0x8250F1CC;
	sub_8250B1D0(ctx, base);
	// 8250F1CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250F1D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F1D4: 488E4835  bl 0x82df3a08
	ctx.lr = 0x8250F1D8;
	sub_82DF3A08(ctx, base);
	// 8250F1D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F1DC: 814B01CC  lwz r10, 0x1cc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(460 as u32) ) } as u64;
	// 8250F1E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250F1E4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8250F1E8: 816B01C8  lwz r11, 0x1c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(456 as u32) ) } as u64;
	// 8250F1EC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250F1F0: 419A0024  beq cr6, 0x8250f214
	if ctx.cr[6].eq {
	pc = 0x8250F214; continue 'dispatch;
	}
	// 8250F1F4: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8250F1F8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250F1FC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F200: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250F204: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250F208: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250F20C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F210: 4082FFE8  bne 0x8250f1f8
	if !ctx.cr[0].eq {
	pc = 0x8250F1F8; continue 'dispatch;
	}
	// 8250F214: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250F218: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250F21C: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 8250F220: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250F224: 4BFFC42D  bl 0x8250b650
	ctx.lr = 0x8250F228;
	sub_8250B650(ctx, base);
	// 8250F228: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250F22C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F230: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250F234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F238: 4BFFE2D1  bl 0x8250d508
	ctx.lr = 0x8250F23C;
	sub_8250D508(ctx, base);
	// 8250F23C: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 8250F240: 488E2A51  bl 0x82df1c90
	ctx.lr = 0x8250F244;
	sub_82DF1C90(ctx, base);
	// 8250F244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F248: 488E41E1  bl 0x82df3428
	ctx.lr = 0x8250F24C;
	sub_82DF3428(ctx, base);
	// 8250F24C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250F250: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F254: 38A001DF  li r5, 0x1df
	ctx.r[5].s64 = 479;
	// 8250F258: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 8250F25C: 488E318D  bl 0x82df23e8
	ctx.lr = 0x8250F260;
	sub_82DF23E8(ctx, base);
	// 8250F260: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250F264: 41820010  beq 0x8250f274
	if ctx.cr[0].eq {
	pc = 0x8250F274; continue 'dispatch;
	}
	// 8250F268: 4840A699  bl 0x82919900
	ctx.lr = 0x8250F26C;
	sub_82919900(ctx, base);
	// 8250F26C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250F270: 48000008  b 0x8250f278
	pc = 0x8250F278; continue 'dispatch;
	// 8250F274: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 8250F278: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250F27C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F280: 488E4789  bl 0x82df3a08
	ctx.lr = 0x8250F284;
	sub_82DF3A08(ctx, base);
	// 8250F284: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8250F288: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250F28C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250F290: 4BFF9DD9  bl 0x82509068
	ctx.lr = 0x8250F294;
	sub_82509068(ctx, base);
	// 8250F294: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250F298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250F29C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8250F2A0: 4BDB0D61  bl 0x822c0000
	ctx.lr = 0x8250F2A4;
	sub_822C0000(ctx, base);
	// 8250F2A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250F2A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250F2AC: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 8250F2B0: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8250F2B4: 4BFFC39D  bl 0x8250b650
	ctx.lr = 0x8250F2B8;
	sub_8250B650(ctx, base);
	// 8250F2B8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8250F2BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8250F2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F2C4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250F2C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8250F2CC: 4BFFE2C5  bl 0x8250d590
	ctx.lr = 0x8250F2D0;
	sub_8250D590(ctx, base);
	// 8250F2D0: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 8250F2D4: 488E29BD  bl 0x82df1c90
	ctx.lr = 0x8250F2D8;
	sub_82DF1C90(ctx, base);
	// 8250F2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F2DC: 488E414D  bl 0x82df3428
	ctx.lr = 0x8250F2E0;
	sub_82DF3428(ctx, base);
	// 8250F2E0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F2E4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8250F2E8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8250F2EC: 91690160  stw r11, 0x160(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 8250F2F0: 93EA7018  stw r31, 0x7018(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28696 as u32), ctx.r[31].u32 ) };
	// 8250F2F4: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 8250F2F8: 48C98EB0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F300 size=12
    let mut pc: u32 = 0x8250F300;
    'dispatch: loop {
        match pc {
            0x8250F300 => {
    //   block [0x8250F300..0x8250F30C)
	// 8250F300: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250F304: 986B701C  stb r3, 0x701c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28700 as u32), ctx.r[3].u8 ) };
	// 8250F308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F310 size=12
    let mut pc: u32 = 0x8250F310;
    'dispatch: loop {
        match pc {
            0x8250F310 => {
    //   block [0x8250F310..0x8250F31C)
	// 8250F310: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250F314: 886B701C  lbz r3, 0x701c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28700 as u32) ) } as u64;
	// 8250F318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F320 size=12
    let mut pc: u32 = 0x8250F320;
    'dispatch: loop {
        match pc {
            0x8250F320 => {
    //   block [0x8250F320..0x8250F32C)
	// 8250F320: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250F324: 986B701D  stb r3, 0x701d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28701 as u32), ctx.r[3].u8 ) };
	// 8250F328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F330 size=48
    let mut pc: u32 = 0x8250F330;
    'dispatch: loop {
        match pc {
            0x8250F330 => {
    //   block [0x8250F330..0x8250F360)
	// 8250F330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F33C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F340: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F344: 4BFDB745  bl 0x824eaa88
	ctx.lr = 0x8250F348;
	sub_824EAA88(ctx, base);
	// 8250F348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F34C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F360 size=12
    let mut pc: u32 = 0x8250F360;
    'dispatch: loop {
        match pc {
            0x8250F360 => {
    //   block [0x8250F360..0x8250F36C)
	// 8250F360: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F364: 886B0008  lbz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F370 size=68
    let mut pc: u32 = 0x8250F370;
    'dispatch: loop {
        match pc {
            0x8250F370 => {
    //   block [0x8250F370..0x8250F3B4)
	// 8250F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250F37C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F388: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250F38C: 48947085  bl 0x82e56410
	ctx.lr = 0x8250F390;
	sub_82E56410(ctx, base);
	// 8250F390: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250F394: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8250F398: 489472D9  bl 0x82e56670
	ctx.lr = 0x8250F39C;
	sub_82E56670(ctx, base);
	// 8250F39C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F3A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250F3AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F3B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F3B8 size=12
    let mut pc: u32 = 0x8250F3B8;
    'dispatch: loop {
        match pc {
            0x8250F3B8 => {
    //   block [0x8250F3B8..0x8250F3C4)
	// 8250F3B8: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F3BC: F88B0040  std r4, 0x40(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[4].u64 ) };
	// 8250F3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F3C8 size=12
    let mut pc: u32 = 0x8250F3C8;
    'dispatch: loop {
        match pc {
            0x8250F3C8 => {
    //   block [0x8250F3C8..0x8250F3D4)
	// 8250F3C8: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F3CC: E86B0040  ld r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 8250F3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F3D8 size=12
    let mut pc: u32 = 0x8250F3D8;
    'dispatch: loop {
        match pc {
            0x8250F3D8 => {
    //   block [0x8250F3D8..0x8250F3E4)
	// 8250F3D8: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F3DC: 886B0050  lbz r3, 0x50(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250F3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F3E8 size=24
    let mut pc: u32 = 0x8250F3E8;
    'dispatch: loop {
        match pc {
            0x8250F3E8 => {
    //   block [0x8250F3E8..0x8250F400)
	// 8250F3E8: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F3EC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250F3F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250F3F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250F3F8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8250F3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F400 size=196
    let mut pc: u32 = 0x8250F400;
    'dispatch: loop {
        match pc {
            0x8250F400 => {
    //   block [0x8250F400..0x8250F4C4)
	// 8250F400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250F40C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250F418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250F41C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8250F420: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250F424: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250F428: 4BDB1511  bl 0x822c0938
	ctx.lr = 0x8250F42C;
	sub_822C0938(ctx, base);
	// 8250F42C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250F430: 41820028  beq 0x8250f458
	if ctx.cr[0].eq {
	pc = 0x8250F458; continue 'dispatch;
	}
	// 8250F434: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250F438: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250F43C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250F440: 392B1B2C  addi r9, r11, 0x1b2c
	ctx.r[9].s64 = ctx.r[11].s64 + 6956;
	// 8250F444: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250F448: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250F44C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250F450: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250F454: 48000008  b 0x8250f45c
	pc = 0x8250F45C; continue 'dispatch;
	// 8250F458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250F45C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250F460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F464: 409A0044  bne cr6, 0x8250f4a8
	if !ctx.cr[6].eq {
	pc = 0x8250F4A8; continue 'dispatch;
	}
	// 8250F468: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250F46C: 419A001C  beq cr6, 0x8250f488
	if ctx.cr[6].eq {
	pc = 0x8250F488; continue 'dispatch;
	}
	// 8250F470: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F474: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250F478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F47C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250F484: 4E800421  bctrl
	ctx.lr = 0x8250F488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250F488: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250F48C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250F490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F494: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8250F498: 816BC71C  lwz r11, -0x38e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14564 as u32) ) } as u64;
	// 8250F49C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8250F4A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250F4A4: 4BDB0B5D  bl 0x822c0000
	ctx.lr = 0x8250F4A8;
	sub_822C0000(ctx, base);
	// 8250F4A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250F4AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F4B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F4B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F4B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250F4BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F4C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F4C8 size=80
    let mut pc: u32 = 0x8250F4C8;
    'dispatch: loop {
        match pc {
            0x8250F4C8 => {
    //   block [0x8250F4C8..0x8250F518)
	// 8250F4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F4D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F4D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F4D8: 816400BC  lwz r11, 0xbc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F4DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F4E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F4E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F4E8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8250F4EC: 409A0008  bne cr6, 0x8250f4f4
	if !ctx.cr[6].eq {
	pc = 0x8250F4F4; continue 'dispatch;
	}
	// 8250F4F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250F4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250F4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F4FC: 488E26FD  bl 0x82df1bf8
	ctx.lr = 0x8250F500;
	sub_82DF1BF8(ctx, base);
	// 8250F500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F518 size=80
    let mut pc: u32 = 0x8250F518;
    'dispatch: loop {
        match pc {
            0x8250F518 => {
    //   block [0x8250F518..0x8250F568)
	// 8250F518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F520: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F524: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F528: 816400BC  lwz r11, 0xbc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F52C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F530: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F534: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F538: 388B0094  addi r4, r11, 0x94
	ctx.r[4].s64 = ctx.r[11].s64 + 148;
	// 8250F53C: 409A0008  bne cr6, 0x8250f544
	if !ctx.cr[6].eq {
	pc = 0x8250F544; continue 'dispatch;
	}
	// 8250F540: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250F544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250F548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F54C: 488E26AD  bl 0x82df1bf8
	ctx.lr = 0x8250F550;
	sub_82DF1BF8(ctx, base);
	// 8250F550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F568 size=88
    let mut pc: u32 = 0x8250F568;
    'dispatch: loop {
        match pc {
            0x8250F568 => {
    //   block [0x8250F568..0x8250F5C0)
	// 8250F568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F570: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F574: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F57C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F580: 4BFFFF49  bl 0x8250f4c8
	ctx.lr = 0x8250F584;
	sub_8250F4C8(ctx, base);
	// 8250F584: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F588: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F58C: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8250F590: 409A0008  bne cr6, 0x8250f598
	if !ctx.cr[6].eq {
	pc = 0x8250F598; continue 'dispatch;
	}
	// 8250F594: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250F598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F59C: 4BFFB02D  bl 0x8250a5c8
	ctx.lr = 0x8250F5A0;
	sub_8250A5C8(ctx, base);
	// 8250F5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F5A4: 488E26ED  bl 0x82df1c90
	ctx.lr = 0x8250F5A8;
	sub_82DF1C90(ctx, base);
	// 8250F5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F5AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F5B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


