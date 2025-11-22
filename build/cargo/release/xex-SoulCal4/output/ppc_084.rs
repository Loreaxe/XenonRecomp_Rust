pub fn sub_825A2BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A2BF0 size=228
    let mut pc: u32 = 0x825A2BF0;
    'dispatch: loop {
        match pc {
            0x825A2BF0 => {
    //   block [0x825A2BF0..0x825A2CD4)
	// 825A2BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A2BF4: 4BF924BD  bl 0x825350b0
	ctx.lr = 0x825A2BF8;
	sub_82535080(ctx, base);
	// 825A2BF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A2BFC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A2C00: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A2C04: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825A2C08: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 825A2C0C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 825A2C10: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A2C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A2C18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A2C1C: 409A0048  bne cr6, 0x825a2c64
	if !ctx.cr[6].eq {
	pc = 0x825A2C64; continue 'dispatch;
	}
	// 825A2C20: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825A2C24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A2C28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A2C2C: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 825A2C30: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 825A2C34: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 825A2C38: 897C0001  lbz r11, 1(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(1 as u32) ) } as u64;
	// 825A2C3C: 9BE10064  stb r31, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u8 ) };
	// 825A2C40: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 825A2C44: 99610065  stb r11, 0x65(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(101 as u32), ctx.r[11].u8 ) };
	// 825A2C48: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 825A2C4C: 616BBB80  ori r11, r11, 0xbb80
	ctx.r[11].u64 = ctx.r[11].u64 | 48000;
	// 825A2C50: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825A2C54: 4BFE1A95  bl 0x825846e8
	ctx.lr = 0x825A2C58;
	sub_825846E8(ctx, base);
	// 825A2C58: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A2C5C: 41980070  blt cr6, 0x825a2ccc
	if ctx.cr[6].lt {
	pc = 0x825A2CCC; continue 'dispatch;
	}
	// 825A2C60: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A2C64: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A2C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A2C6C: 419A0054  beq cr6, 0x825a2cc0
	if ctx.cr[6].eq {
	pc = 0x825A2CC0; continue 'dispatch;
	}
	// 825A2C70: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2C74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A2C78: 419A0048  beq cr6, 0x825a2cc0
	if ctx.cr[6].eq {
	pc = 0x825A2CC0; continue 'dispatch;
	}
	// 825A2C7C: 3F60829A  lis r27, -0x7d66
	ctx.r[27].s64 = -2103836672;
	// 825A2C80: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A2C84: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A2C88: 817B3920  lwz r11, 0x3920(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(14624 as u32) ) } as u64;
	// 825A2C8C: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 825A2C90: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2C94: 4BFE913D  bl 0x8258bdd0
	ctx.lr = 0x825A2C98;
	sub_8258BDD0(ctx, base);
	// 825A2C98: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A2C9C: 41980030  blt cr6, 0x825a2ccc
	if ctx.cr[6].lt {
	pc = 0x825A2CCC; continue 'dispatch;
	}
	// 825A2CA0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A2CA4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A2CA8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A2CAC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825A2CB0: 7FAAEA14  add r29, r10, r29
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 825A2CB4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2CB8: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A2CBC: 4198FFC4  blt cr6, 0x825a2c80
	if ctx.cr[6].lt {
	pc = 0x825A2C80; continue 'dispatch;
	}
	// 825A2CC0: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A2CC4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 825A2CC8: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A2CCC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825A2CD0: 4BF92430  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A2CD8 size=200
    let mut pc: u32 = 0x825A2CD8;
    'dispatch: loop {
        match pc {
            0x825A2CD8 => {
    //   block [0x825A2CD8..0x825A2DA0)
	// 825A2CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A2CDC: 4BF923DD  bl 0x825350b8
	ctx.lr = 0x825A2CE0;
	sub_82535080(ctx, base);
	// 825A2CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A2CE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A2CE8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A2CEC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825A2CF0: 396B6E10  addi r11, r11, 0x6e10
	ctx.r[11].s64 = ctx.r[11].s64 + 28176;
	// 825A2CF4: 895F003C  lbz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2CF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A2CFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A2D00: 419A004C  beq cr6, 0x825a2d4c
	if ctx.cr[6].eq {
	pc = 0x825A2D4C; continue 'dispatch;
	}
	// 825A2D04: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 825A2D08: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A2D0C: 57BE1838  slwi r30, r29, 3
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825A2D10: 7D7E502E  lwzx r11, r30, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A2D14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A2D18: 419A0020  beq cr6, 0x825a2d38
	if ctx.cr[6].eq {
	pc = 0x825A2D38; continue 'dispatch;
	}
	// 825A2D1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2D20: 7C7E502E  lwzx r3, r30, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A2D24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A2D28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A2D2C: 4E800421  bctrl
	ctx.lr = 0x825A2D30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A2D30: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A2D34: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 825A2D38: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 825A2D3C: 895F003C  lbz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2D40: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A2D44: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A2D48: 4198FFC0  blt cr6, 0x825a2d08
	if ctx.cr[6].lt {
	pc = 0x825A2D08; continue 'dispatch;
	}
	// 825A2D4C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 825A2D50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A2D54: 419A000C  beq cr6, 0x825a2d60
	if ctx.cr[6].eq {
	pc = 0x825A2D60; continue 'dispatch;
	}
	// 825A2D58: 4BFE1651  bl 0x825843a8
	ctx.lr = 0x825A2D5C;
	sub_825843A8(ctx, base);
	// 825A2D5C: 939F0020  stw r28, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 825A2D60: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A2D64: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A2D68: 396B67C8  addi r11, r11, 0x67c8
	ctx.r[11].s64 = ctx.r[11].s64 + 26568;
	// 825A2D6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A2D70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A2D74: 419A0018  beq cr6, 0x825a2d8c
	if ctx.cr[6].eq {
	pc = 0x825A2D8C; continue 'dispatch;
	}
	// 825A2D78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2D7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A2D80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A2D84: 4E800421  bctrl
	ctx.lr = 0x825A2D88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A2D88: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825A2D8C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A2D90: 396B62E8  addi r11, r11, 0x62e8
	ctx.r[11].s64 = ctx.r[11].s64 + 25320;
	// 825A2D94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A2D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A2D9C: 4BF9236C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A2DA0 size=152
    let mut pc: u32 = 0x825A2DA0;
    'dispatch: loop {
        match pc {
            0x825A2DA0 => {
    //   block [0x825A2DA0..0x825A2E38)
	// 825A2DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A2DA4: 4BF92319  bl 0x825350bc
	ctx.lr = 0x825A2DA8;
	sub_82535080(ctx, base);
	// 825A2DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A2DAC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A2DB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A2DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A2DB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A2DBC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 825A2DC0: 816B3920  lwz r11, 0x3920(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14624 as u32) ) } as u64;
	// 825A2DC4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825A2DC8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 825A2DCC: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2DD0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A2DD4: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A2DD8: 4BFE9019  bl 0x8258bdf0
	ctx.lr = 0x825A2DDC;
	sub_8258BDF0(ctx, base);
	// 825A2DDC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A2DE0: 41980050  blt cr6, 0x825a2e30
	if ctx.cr[6].lt {
	pc = 0x825A2E30; continue 'dispatch;
	}
	// 825A2DE4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2DE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A2DEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2DF0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A2DF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A2DF8: 4E800421  bctrl
	ctx.lr = 0x825A2DFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A2DFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A2E00: 41980030  blt cr6, 0x825a2e30
	if ctx.cr[6].lt {
	pc = 0x825A2E30; continue 'dispatch;
	}
	// 825A2E04: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A2E08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A2E0C: A1610052  lhz r11, 0x52(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 825A2E10: 9BBF0004  stb r29, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 825A2E14: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 825A2E18: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 825A2E1C: 993F0006  stb r9, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u8 ) };
	// 825A2E20: A15E003E  lhz r10, 0x3e(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(62 as u32) ) } as u64;
	// 825A2E24: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A2E28: 40990008  ble cr6, 0x825a2e30
	if !ctx.cr[6].gt {
	pc = 0x825A2E30; continue 'dispatch;
	}
	// 825A2E2C: B17E003E  sth r11, 0x3e(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(62 as u32), ctx.r[11].u16 ) };
	// 825A2E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A2E34: 4BF922D8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2E38 size=36
    let mut pc: u32 = 0x825A2E38;
    'dispatch: loop {
        match pc {
            0x825A2E38 => {
    //   block [0x825A2E38..0x825A2E5C)
	// 825A2E38: 8943003C  lbz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2E3C: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 825A2E40: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 825A2E44: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825A2E48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A2E4C: 41980010  blt cr6, 0x825a2e5c
	if ctx.cr[6].lt {
		sub_825A2E5C(ctx, base);
		return;
	}
	// 825A2E50: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 825A2E54: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 825A2E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2E5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2E5C size=36
    let mut pc: u32 = 0x825A2E5C;
    'dispatch: loop {
        match pc {
            0x825A2E5C => {
    //   block [0x825A2E5C..0x825A2E80)
	// 825A2E5C: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A2E60: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A2E64: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825A2E68: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 825A2E6C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A2E70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2E74: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A2E78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A2E7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2E80 size=36
    let mut pc: u32 = 0x825A2E80;
    'dispatch: loop {
        match pc {
            0x825A2E80 => {
    //   block [0x825A2E80..0x825A2EA4)
	// 825A2E80: 8943003C  lbz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2E84: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 825A2E88: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 825A2E8C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825A2E90: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A2E94: 41980010  blt cr6, 0x825a2ea4
	if ctx.cr[6].lt {
		sub_825A2EA4(ctx, base);
		return;
	}
	// 825A2E98: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 825A2E9C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 825A2EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2EA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2EA4 size=36
    let mut pc: u32 = 0x825A2EA4;
    'dispatch: loop {
        match pc {
            0x825A2EA4 => {
    //   block [0x825A2EA4..0x825A2EC8)
	// 825A2EA4: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A2EA8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A2EAC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825A2EB0: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 825A2EB4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A2EB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2EBC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A2EC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A2EC4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2EC8 size=28
    let mut pc: u32 = 0x825A2EC8;
    'dispatch: loop {
        match pc {
            0x825A2EC8 => {
    //   block [0x825A2EC8..0x825A2EE4)
	// 825A2EC8: 8943003C  lbz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2ECC: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 825A2ED0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A2ED4: 41980010  blt cr6, 0x825a2ee4
	if ctx.cr[6].lt {
		sub_825A2EE4(ctx, base);
		return;
	}
	// 825A2ED8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 825A2EDC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 825A2EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2EE4 size=28
    let mut pc: u32 = 0x825A2EE4;
    'dispatch: loop {
        match pc {
            0x825A2EE4 => {
    //   block [0x825A2EE4..0x825A2F00)
	// 825A2EE4: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A2EE8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A2EEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A2EF0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A2EF4: 896B0005  lbz r11, 5(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 825A2EF8: 99650000  stb r11, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A2EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2F00 size=28
    let mut pc: u32 = 0x825A2F00;
    'dispatch: loop {
        match pc {
            0x825A2F00 => {
    //   block [0x825A2F00..0x825A2F1C)
	// 825A2F00: 8943003C  lbz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A2F04: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 825A2F08: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A2F0C: 41980010  blt cr6, 0x825a2f1c
	if ctx.cr[6].lt {
		sub_825A2F1C(ctx, base);
		return;
	}
	// 825A2F10: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 825A2F14: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 825A2F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2F1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A2F1C size=24
    let mut pc: u32 = 0x825A2F1C;
    'dispatch: loop {
        match pc {
            0x825A2F1C => {
    //   block [0x825A2F1C..0x825A2F34)
	// 825A2F1C: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A2F20: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A2F24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A2F28: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A2F2C: 98AB0005  stb r5, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[5].u8 ) };
	// 825A2F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A2F38 size=156
    let mut pc: u32 = 0x825A2F38;
    'dispatch: loop {
        match pc {
            0x825A2F38 => {
    //   block [0x825A2F38..0x825A2FD4)
	// 825A2F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A2F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A2F40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A2F44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A2F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A2F4C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A2F50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A2F54: 396B67C8  addi r11, r11, 0x67c8
	ctx.r[11].s64 = ctx.r[11].s64 + 26568;
	// 825A2F58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A2F5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A2F60: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A2F64: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 825A2F68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A2F6C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A2F70: 419A0018  beq cr6, 0x825a2f88
	if ctx.cr[6].eq {
	pc = 0x825A2F88; continue 'dispatch;
	}
	// 825A2F74: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2F78: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A2F7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A2F80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A2F84: 4E800421  bctrl
	ctx.lr = 0x825A2F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A2F88: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 825A2F8C: 9BDF000C  stb r30, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u8 ) };
	// 825A2F90: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 825A2F94: 39296E10  addi r9, r9, 0x6e10
	ctx.r[9].s64 = ctx.r[9].s64 + 28176;
	// 825A2F98: 395F0018  addi r10, r31, 0x18
	ctx.r[10].s64 = ctx.r[31].s64 + 24;
	// 825A2F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825A2FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A2FA4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A2FA8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A2FAC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A2FB0: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A2FB4: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A2FB8: 911F0028  stw r8, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 825A2FBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A2FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A2FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A2FC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A2FCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A2FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A2FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A2FD8 size=92
    let mut pc: u32 = 0x825A2FD8;
    'dispatch: loop {
        match pc {
            0x825A2FD8 => {
    //   block [0x825A2FD8..0x825A3034)
	// 825A2FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A2FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A2FE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A2FE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A2FE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A2FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A2FF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A2FF4: 4BFFFCE5  bl 0x825a2cd8
	ctx.lr = 0x825A2FF8;
	sub_825A2CD8(ctx, base);
	// 825A2FF8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 825A2FFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3000: 419A0018  beq cr6, 0x825a3018
	if ctx.cr[6].eq {
	pc = 0x825A3018; continue 'dispatch;
	}
	// 825A3004: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A3008: 3CA06182  lis r5, 0x6182
	ctx.r[5].s64 = 1635909632;
	// 825A300C: 386B38C8  addi r3, r11, 0x38c8
	ctx.r[3].s64 = ctx.r[11].s64 + 14536;
	// 825A3010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A3014: 4BFE18ED  bl 0x82584900
	ctx.lr = 0x825A3018;
	sub_82584900(ctx, base);
	// 825A3018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A301C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3028: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A302C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3038 size=116
    let mut pc: u32 = 0x825A3038;
    'dispatch: loop {
        match pc {
            0x825A3038 => {
    //   block [0x825A3038..0x825A30AC)
	// 825A3038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A303C: 4BF92081  bl 0x825350bc
	ctx.lr = 0x825A3040;
	sub_82535080(ctx, base);
	// 825A3040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3044: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3048: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A304C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A3050: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A3054: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3058: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A305C: 419A0048  beq cr6, 0x825a30a4
	if ctx.cr[6].eq {
	pc = 0x825A30A4; continue 'dispatch;
	}
	// 825A3060: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A3064: 41980040  blt cr6, 0x825a30a4
	if ctx.cr[6].lt {
	pc = 0x825A30A4; continue 'dispatch;
	}
	// 825A3068: 54DF063E  clrlwi r31, r6, 0x18
	ctx.r[31].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 825A306C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3070: 815D0024  lwz r10, 0x24(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A3074: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A3078: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825A307C: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A3080: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A3084: 7CA8482E  lwzx r5, r8, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825A3088: 4BFFFD19  bl 0x825a2da0
	ctx.lr = 0x825A308C;
	sub_825A2DA0(ctx, base);
	// 825A308C: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 825A3090: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3094: 5566063E  clrlwi r6, r11, 0x18
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A3098: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 825A309C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A30A0: 4198FFC0  blt cr6, 0x825a3060
	if ctx.cr[6].lt {
	pc = 0x825A3060; continue 'dispatch;
	}
	// 825A30A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A30A8: 4BF92064  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A30B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A30B0 size=288
    let mut pc: u32 = 0x825A30B0;
    'dispatch: loop {
        match pc {
            0x825A30B0 => {
    //   block [0x825A30B0..0x825A31D0)
	// 825A30B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A30B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A30B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A30BC: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 825A30C0: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 825A30C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A30C8: 409A0018  bne cr6, 0x825a30e0
	if !ctx.cr[6].eq {
	pc = 0x825A30E0; continue 'dispatch;
	}
	// 825A30CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A30D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A30D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A30D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A30DC: 4E800020  blr
	return;
	// 825A30E0: 89640006  lbz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 825A30E4: 556A077C  rlwinm r10, r11, 0, 0x1d, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A30E8: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 825A30EC: 409A0080  bne cr6, 0x825a316c
	if !ctx.cr[6].eq {
	pc = 0x825A316C; continue 'dispatch;
	}
	// 825A30F0: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A30F4: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A30F8: 892D010C  lbz r9, 0x10c(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[13].u32.wrapping_add(268 as u32) ) } as u64;
	// 825A30FC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 825A3100: 810B3920  lwz r8, 0x3920(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14624 as u32) ) } as u64;
	// 825A3104: 419A0030  beq cr6, 0x825a3134
	if ctx.cr[6].eq {
	pc = 0x825A3134; continue 'dispatch;
	}
	// 825A3108: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A310C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3110: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 825A3114: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 825A3118: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A311C: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 825A3120: 419A0028  beq cr6, 0x825a3148
	if ctx.cr[6].eq {
	pc = 0x825A3148; continue 'dispatch;
	}
	// 825A3124: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A3128: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825A312C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 825A3130: 4198FFE8  blt cr6, 0x825a3118
	if ctx.cr[6].lt {
	pc = 0x825A3118; continue 'dispatch;
	}
	// 825A3134: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A3138: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 825A313C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A3140: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3144: 48000060  b 0x825a31a4
	pc = 0x825A31A4; continue 'dispatch;
	// 825A3148: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A314C: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A3150: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 825A3154: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A3158: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 825A315C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A3160: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 825A3164: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3168: 4800003C  b 0x825a31a4
	pc = 0x825A31A4; continue 'dispatch;
	// 825A316C: 556A07BE  clrlwi r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 825A3170: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3174: 419A0010  beq cr6, 0x825a3184
	if ctx.cr[6].eq {
	pc = 0x825A3184; continue 'dispatch;
	}
	// 825A3178: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A317C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3180: 48000024  b 0x825a31a4
	pc = 0x825A31A4; continue 'dispatch;
	// 825A3184: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A3188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A318C: 419A0010  beq cr6, 0x825a319c
	if ctx.cr[6].eq {
	pc = 0x825A319C; continue 'dispatch;
	}
	// 825A3190: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3198: 4800000C  b 0x825a31a4
	pc = 0x825A31A4; continue 'dispatch;
	// 825A319C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A31A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A31A4: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A31A8: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825A31AC: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 825A31B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A31B4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A31B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A31BC: 4E800421  bctrl
	ctx.lr = 0x825A31C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A31C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A31C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A31C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A31CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A31D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A31D0 size=532
    let mut pc: u32 = 0x825A31D0;
    'dispatch: loop {
        match pc {
            0x825A31D0 => {
    //   block [0x825A31D0..0x825A33E4)
	// 825A31D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A31D4: 4BF91EE5  bl 0x825350b8
	ctx.lr = 0x825A31D8;
	sub_82535080(ctx, base);
	// 825A31D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A31DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A31E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A31E4: 4BD4E1ED  bl 0x822f13d0
	ctx.lr = 0x825A31E8;
	sub_822F13D0(ctx, base);
	// 825A31E8: 4816AC65  bl 0x8270de4c
	ctx.lr = 0x825A31EC;
	// extern call 0x8270DE4C → crate::xboxkrnl::KeRaiseIrqlToDpcLevel
	crate::xboxkrnl::KeRaiseIrqlToDpcLevel(ctx, base);
	// 825A31EC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A31F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A31F4: 3BEB38B0  addi r31, r11, 0x38b0
	ctx.r[31].s64 = ctx.r[11].s64 + 14512;
	// 825A31F8: 7DBE6B78  mr r30, r13
	ctx.r[30].u64 = ctx.r[13].u64;
	// 825A31FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3200: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3204: 419A0010  beq cr6, 0x825a3214
	if ctx.cr[6].eq {
	pc = 0x825A3214; continue 'dispatch;
	}
	// 825A3208: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A320C: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A3210: 419A0018  beq cr6, 0x825a3228
	if ctx.cr[6].eq {
	pc = 0x825A3228; continue 'dispatch;
	}
	// 825A3214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3218: 4816A675  bl 0x8270d88c
	ctx.lr = 0x825A321C;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 825A321C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825A3220: 9BBF000C  stb r29, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 825A3224: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3228: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A322C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3230: 897C003D  lbz r11, 0x3d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 825A3234: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 825A3238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A323C: 419A009C  beq cr6, 0x825a32d8
	if ctx.cr[6].eq {
	pc = 0x825A32D8; continue 'dispatch;
	}
	// 825A3240: 4816AC0D  bl 0x8270de4c
	ctx.lr = 0x825A3244;
	// extern call 0x8270DE4C → crate::xboxkrnl::KeRaiseIrqlToDpcLevel
	crate::xboxkrnl::KeRaiseIrqlToDpcLevel(ctx, base);
	// 825A3244: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3248: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A324C: 7DBE6B78  mr r30, r13
	ctx.r[30].u64 = ctx.r[13].u64;
	// 825A3250: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3254: 419A0010  beq cr6, 0x825a3264
	if ctx.cr[6].eq {
	pc = 0x825A3264; continue 'dispatch;
	}
	// 825A3258: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A325C: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A3260: 419A0018  beq cr6, 0x825a3278
	if ctx.cr[6].eq {
	pc = 0x825A3278; continue 'dispatch;
	}
	// 825A3264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3268: 4816A625  bl 0x8270d88c
	ctx.lr = 0x825A326C;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 825A326C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825A3270: 9BBF000C  stb r29, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 825A3274: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3278: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A327C: 7DAA6B78  mr r10, r13
	ctx.r[10].u64 = ctx.r[13].u64;
	// 825A3280: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3284: 897C003D  lbz r11, 0x3d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 825A3288: 716B00FB  andi. r11, r11, 0xfb
	ctx.r[11].u64 = ctx.r[11].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A328C: 997C003D  stb r11, 0x3d(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 825A3290: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3294: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3298: 419A00F0  beq cr6, 0x825a3388
	if ctx.cr[6].eq {
	pc = 0x825A3388; continue 'dispatch;
	}
	// 825A329C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A32A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A32A4: 409A00E4  bne cr6, 0x825a3388
	if !ctx.cr[6].eq {
	pc = 0x825A3388; continue 'dispatch;
	}
	// 825A32A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A32AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A32B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A32B4: 409A00D4  bne cr6, 0x825a3388
	if !ctx.cr[6].eq {
	pc = 0x825A3388; continue 'dispatch;
	}
	// 825A32B8: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A32BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A32C0: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A32C4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A32C8: 4816A5B5  bl 0x8270d87c
	ctx.lr = 0x825A32CC;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A32CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A32D0: 4816AB8D  bl 0x8270de5c
	ctx.lr = 0x825A32D4;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A32D4: 480000B0  b 0x825a3384
	pc = 0x825A3384; continue 'dispatch;
	// 825A32D8: 4816AB75  bl 0x8270de4c
	ctx.lr = 0x825A32DC;
	// extern call 0x8270DE4C → crate::xboxkrnl::KeRaiseIrqlToDpcLevel
	crate::xboxkrnl::KeRaiseIrqlToDpcLevel(ctx, base);
	// 825A32DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A32E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A32E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A32E8: 7DBE6B78  mr r30, r13
	ctx.r[30].u64 = ctx.r[13].u64;
	// 825A32EC: 419A0010  beq cr6, 0x825a32fc
	if ctx.cr[6].eq {
	pc = 0x825A32FC; continue 'dispatch;
	}
	// 825A32F0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A32F4: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A32F8: 419A0018  beq cr6, 0x825a3310
	if ctx.cr[6].eq {
	pc = 0x825A3310; continue 'dispatch;
	}
	// 825A32FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3300: 4816A58D  bl 0x8270d88c
	ctx.lr = 0x825A3304;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 825A3304: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825A3308: 9BBF000C  stb r29, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 825A330C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3310: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A3314: 7DAA6B78  mr r10, r13
	ctx.r[10].u64 = ctx.r[13].u64;
	// 825A3318: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A331C: 897C003D  lbz r11, 0x3d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 825A3320: 716B00BE  andi. r11, r11, 0xbe
	ctx.r[11].u64 = ctx.r[11].u64 & 190;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3324: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825A3328: 997C003D  stb r11, 0x3d(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 825A332C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3330: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3334: 419A003C  beq cr6, 0x825a3370
	if ctx.cr[6].eq {
	pc = 0x825A3370; continue 'dispatch;
	}
	// 825A3338: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A333C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A3340: 409A0030  bne cr6, 0x825a3370
	if !ctx.cr[6].eq {
	pc = 0x825A3370; continue 'dispatch;
	}
	// 825A3344: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A3348: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A334C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3350: 409A0020  bne cr6, 0x825a3370
	if !ctx.cr[6].eq {
	pc = 0x825A3370; continue 'dispatch;
	}
	// 825A3354: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A3358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A335C: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A3360: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A3364: 4816A519  bl 0x8270d87c
	ctx.lr = 0x825A3368;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A3368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A336C: 4816AAF1  bl 0x8270de5c
	ctx.lr = 0x825A3370;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A3370: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3374: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A3378: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 825A337C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3380: 4E800421  bctrl
	ctx.lr = 0x825A3384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3384: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3388: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A338C: 7DAA6B78  mr r10, r13
	ctx.r[10].u64 = ctx.r[13].u64;
	// 825A3390: 419A0040  beq cr6, 0x825a33d0
	if ctx.cr[6].eq {
	pc = 0x825A33D0; continue 'dispatch;
	}
	// 825A3394: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3398: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A339C: 409A0034  bne cr6, 0x825a33d0
	if !ctx.cr[6].eq {
	pc = 0x825A33D0; continue 'dispatch;
	}
	// 825A33A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A33A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A33A8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A33AC: 409A0024  bne cr6, 0x825a33d0
	if !ctx.cr[6].eq {
	pc = 0x825A33D0; continue 'dispatch;
	}
	// 825A33B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A33B4: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A33B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A33BC: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A33C0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A33C4: 4816A4B9  bl 0x8270d87c
	ctx.lr = 0x825A33C8;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A33C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A33CC: 4816AA91  bl 0x8270de5c
	ctx.lr = 0x825A33D0;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A33D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A33D4: 4BD4DFFD  bl 0x822f13d0
	ctx.lr = 0x825A33D8;
	sub_822F13D0(ctx, base);
	// 825A33D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A33DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A33E0: 4BF91D28  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A33E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A33E8 size=928
    let mut pc: u32 = 0x825A33E8;
    'dispatch: loop {
        match pc {
            0x825A33E8 => {
    //   block [0x825A33E8..0x825A3788)
	// 825A33E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A33EC: 4BF91CCD  bl 0x825350b8
	ctx.lr = 0x825A33F0;
	sub_82535080(ctx, base);
	// 825A33F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A33F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A33F8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825A33FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A3400: 4BD4DFD1  bl 0x822f13d0
	ctx.lr = 0x825A3404;
	sub_822F13D0(ctx, base);
	// 825A3404: 57EB07FE  clrlwi r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	// 825A3408: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A340C: 419A01CC  beq cr6, 0x825a35d8
	if ctx.cr[6].eq {
	pc = 0x825A35D8; continue 'dispatch;
	}
	// 825A3410: 4816AA3D  bl 0x8270de4c
	ctx.lr = 0x825A3414;
	// extern call 0x8270DE4C → crate::xboxkrnl::KeRaiseIrqlToDpcLevel
	crate::xboxkrnl::KeRaiseIrqlToDpcLevel(ctx, base);
	// 825A3414: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A3418: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A341C: 3BEB38B0  addi r31, r11, 0x38b0
	ctx.r[31].s64 = ctx.r[11].s64 + 14512;
	// 825A3420: 7DBE6B78  mr r30, r13
	ctx.r[30].u64 = ctx.r[13].u64;
	// 825A3424: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3428: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A342C: 419A0010  beq cr6, 0x825a343c
	if ctx.cr[6].eq {
	pc = 0x825A343C; continue 'dispatch;
	}
	// 825A3430: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3434: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A3438: 419A0018  beq cr6, 0x825a3450
	if ctx.cr[6].eq {
	pc = 0x825A3450; continue 'dispatch;
	}
	// 825A343C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3440: 4816A44D  bl 0x8270d88c
	ctx.lr = 0x825A3444;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 825A3444: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825A3448: 9BBF000C  stb r29, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 825A344C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3450: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 825A3454: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A3458: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A345C: 806A3920  lwz r3, 0x3920(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(14624 as u32) ) } as u64;
	// 825A3460: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3464: 4BFE633D  bl 0x825897a0
	ctx.lr = 0x825A3468;
	sub_825897A0(ctx, base);
	// 825A3468: 897C003D  lbz r11, 0x3d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 825A346C: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 825A3470: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3474: 409A0060  bne cr6, 0x825a34d4
	if !ctx.cr[6].eq {
	pc = 0x825A34D4; continue 'dispatch;
	}
	// 825A3478: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A347C: 7DA96B78  mr r9, r13
	ctx.r[9].u64 = ctx.r[13].u64;
	// 825A3480: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825A3484: 419A02F0  beq cr6, 0x825a3774
	if ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A3488: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A348C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A3490: 409A02E4  bne cr6, 0x825a3774
	if !ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A3494: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 825A3498: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A349C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A34A0: 409A02D4  bne cr6, 0x825a3774
	if !ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A34A4: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A34A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A34AC: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A34B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A34B4: 4816A3C9  bl 0x8270d87c
	ctx.lr = 0x825A34B8;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A34B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A34BC: 4816A9A1  bl 0x8270de5c
	ctx.lr = 0x825A34C0;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A34C0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825A34C4: 4BD4DF0D  bl 0x822f13d0
	ctx.lr = 0x825A34C8;
	sub_822F13D0(ctx, base);
	// 825A34C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A34CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A34D0: 4BF91C38  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 825A34D4: 4816A979  bl 0x8270de4c
	ctx.lr = 0x825A34D8;
	// extern call 0x8270DE4C → crate::xboxkrnl::KeRaiseIrqlToDpcLevel
	crate::xboxkrnl::KeRaiseIrqlToDpcLevel(ctx, base);
	// 825A34D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A34DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A34E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A34E4: 7DBE6B78  mr r30, r13
	ctx.r[30].u64 = ctx.r[13].u64;
	// 825A34E8: 419A0010  beq cr6, 0x825a34f8
	if ctx.cr[6].eq {
	pc = 0x825A34F8; continue 'dispatch;
	}
	// 825A34EC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A34F0: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A34F4: 419A0018  beq cr6, 0x825a350c
	if ctx.cr[6].eq {
	pc = 0x825A350C; continue 'dispatch;
	}
	// 825A34F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A34FC: 4816A391  bl 0x8270d88c
	ctx.lr = 0x825A3500;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 825A3500: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825A3504: 9BBF000C  stb r29, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 825A3508: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A350C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A3510: 7DAA6B78  mr r10, r13
	ctx.r[10].u64 = ctx.r[13].u64;
	// 825A3514: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3518: 897C003D  lbz r11, 0x3d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 825A351C: 716B00BA  andi. r11, r11, 0xba
	ctx.r[11].u64 = ctx.r[11].u64 & 186;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3520: 997C003D  stb r11, 0x3d(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 825A3524: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3528: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A352C: 419A003C  beq cr6, 0x825a3568
	if ctx.cr[6].eq {
	pc = 0x825A3568; continue 'dispatch;
	}
	// 825A3530: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3534: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A3538: 409A0030  bne cr6, 0x825a3568
	if !ctx.cr[6].eq {
	pc = 0x825A3568; continue 'dispatch;
	}
	// 825A353C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A3540: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3544: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3548: 409A0020  bne cr6, 0x825a3568
	if !ctx.cr[6].eq {
	pc = 0x825A3568; continue 'dispatch;
	}
	// 825A354C: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A3550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3554: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A3558: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A355C: 4816A321  bl 0x8270d87c
	ctx.lr = 0x825A3560;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A3560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3564: 4816A8F9  bl 0x8270de5c
	ctx.lr = 0x825A3568;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A3568: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A356C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A3570: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3578: 4E800421  bctrl
	ctx.lr = 0x825A357C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A357C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3580: 7DAA6B78  mr r10, r13
	ctx.r[10].u64 = ctx.r[13].u64;
	// 825A3584: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3588: 419A01EC  beq cr6, 0x825a3774
	if ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A358C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3590: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A3594: 409A01E0  bne cr6, 0x825a3774
	if !ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A3598: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A359C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A35A0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A35A4: 409A01D0  bne cr6, 0x825a3774
	if !ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A35A8: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A35AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A35B0: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A35B4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A35B8: 4816A2C5  bl 0x8270d87c
	ctx.lr = 0x825A35BC;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A35BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A35C0: 4816A89D  bl 0x8270de5c
	ctx.lr = 0x825A35C4;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A35C4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825A35C8: 4BD4DE09  bl 0x822f13d0
	ctx.lr = 0x825A35CC;
	sub_822F13D0(ctx, base);
	// 825A35CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A35D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A35D4: 4BF91B34  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 825A35D8: 4816A875  bl 0x8270de4c
	ctx.lr = 0x825A35DC;
	// extern call 0x8270DE4C → crate::xboxkrnl::KeRaiseIrqlToDpcLevel
	crate::xboxkrnl::KeRaiseIrqlToDpcLevel(ctx, base);
	// 825A35DC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A35E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A35E4: 3BEB38B0  addi r31, r11, 0x38b0
	ctx.r[31].s64 = ctx.r[11].s64 + 14512;
	// 825A35E8: 7DBE6B78  mr r30, r13
	ctx.r[30].u64 = ctx.r[13].u64;
	// 825A35EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A35F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A35F4: 419A0010  beq cr6, 0x825a3604
	if ctx.cr[6].eq {
	pc = 0x825A3604; continue 'dispatch;
	}
	// 825A35F8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A35FC: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A3600: 419A0024  beq cr6, 0x825a3624
	if ctx.cr[6].eq {
	pc = 0x825A3624; continue 'dispatch;
	}
	// 825A3604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3608: 4816A285  bl 0x8270d88c
	ctx.lr = 0x825A360C;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 825A360C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 825A3610: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 825A3614: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A3618: 9BDF000C  stb r30, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u8 ) };
	// 825A361C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3620: 48000008  b 0x825a3628
	pc = 0x825A3628; continue 'dispatch;
	// 825A3624: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A3628: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A362C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3630: 893C003D  lbz r9, 0x3d(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 825A3634: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 825A3638: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825A363C: 409A0054  bne cr6, 0x825a3690
	if !ctx.cr[6].eq {
	pc = 0x825A3690; continue 'dispatch;
	}
	// 825A3640: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3644: 7DA96B78  mr r9, r13
	ctx.r[9].u64 = ctx.r[13].u64;
	// 825A3648: 419A012C  beq cr6, 0x825a3774
	if ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A364C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A3650: 409A0124  bne cr6, 0x825a3774
	if !ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A3654: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A3658: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A365C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3660: 409A0114  bne cr6, 0x825a3774
	if !ctx.cr[6].eq {
	pc = 0x825A3774; continue 'dispatch;
	}
	// 825A3664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3668: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A366C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A3670: 4816A20D  bl 0x8270d87c
	ctx.lr = 0x825A3674;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A3674: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3678: 4816A7E5  bl 0x8270de5c
	ctx.lr = 0x825A367C;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A367C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825A3680: 4BD4DD51  bl 0x822f13d0
	ctx.lr = 0x825A3684;
	sub_822F13D0(ctx, base);
	// 825A3684: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A3688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A368C: 4BF91A7C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 825A3690: 4816A7BD  bl 0x8270de4c
	ctx.lr = 0x825A3694;
	// extern call 0x8270DE4C → crate::xboxkrnl::KeRaiseIrqlToDpcLevel
	crate::xboxkrnl::KeRaiseIrqlToDpcLevel(ctx, base);
	// 825A3694: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3698: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A369C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A36A0: 7DBE6B78  mr r30, r13
	ctx.r[30].u64 = ctx.r[13].u64;
	// 825A36A4: 419A0010  beq cr6, 0x825a36b4
	if ctx.cr[6].eq {
	pc = 0x825A36B4; continue 'dispatch;
	}
	// 825A36A8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A36AC: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A36B0: 419A0018  beq cr6, 0x825a36c8
	if ctx.cr[6].eq {
	pc = 0x825A36C8; continue 'dispatch;
	}
	// 825A36B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A36B8: 4816A1D5  bl 0x8270d88c
	ctx.lr = 0x825A36BC;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 825A36BC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825A36C0: 9BBF000C  stb r29, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 825A36C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A36C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A36CC: 7DAA6B78  mr r10, r13
	ctx.r[10].u64 = ctx.r[13].u64;
	// 825A36D0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A36D4: 897C003D  lbz r11, 0x3d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 825A36D8: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A36DC: 997C003D  stb r11, 0x3d(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 825A36E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A36E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A36E8: 419A0040  beq cr6, 0x825a3728
	if ctx.cr[6].eq {
	pc = 0x825A3728; continue 'dispatch;
	}
	// 825A36EC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A36F0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A36F4: 409A0034  bne cr6, 0x825a3728
	if !ctx.cr[6].eq {
	pc = 0x825A3728; continue 'dispatch;
	}
	// 825A36F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A36FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3700: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3704: 409A0024  bne cr6, 0x825a3728
	if !ctx.cr[6].eq {
	pc = 0x825A3728; continue 'dispatch;
	}
	// 825A3708: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A370C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3710: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A3714: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A3718: 4816A165  bl 0x8270d87c
	ctx.lr = 0x825A371C;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A371C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3720: 4816A73D  bl 0x8270de5c
	ctx.lr = 0x825A3724;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A3724: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3728: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A372C: 7DAA6B78  mr r10, r13
	ctx.r[10].u64 = ctx.r[13].u64;
	// 825A3730: 419A003C  beq cr6, 0x825a376c
	if ctx.cr[6].eq {
	pc = 0x825A376C; continue 'dispatch;
	}
	// 825A3734: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3738: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A373C: 409A0030  bne cr6, 0x825a376c
	if !ctx.cr[6].eq {
	pc = 0x825A376C; continue 'dispatch;
	}
	// 825A3740: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A3744: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3748: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A374C: 409A0020  bne cr6, 0x825a376c
	if !ctx.cr[6].eq {
	pc = 0x825A376C; continue 'dispatch;
	}
	// 825A3750: 8BDF000C  lbz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A3754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3758: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A375C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A3760: 4816A11D  bl 0x8270d87c
	ctx.lr = 0x825A3764;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 825A3764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3768: 4816A6F5  bl 0x8270de5c
	ctx.lr = 0x825A376C;
	// extern call 0x8270DE5C → crate::xboxkrnl::KfLowerIrql
	crate::xboxkrnl::KfLowerIrql(ctx, base);
	// 825A376C: A17C003E  lhz r11, 0x3e(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(62 as u32) ) } as u64;
	// 825A3770: B17C0040  sth r11, 0x40(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u16 ) };
	// 825A3774: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825A3778: 4BD4DC59  bl 0x822f13d0
	ctx.lr = 0x825A377C;
	sub_822F13D0(ctx, base);
	// 825A377C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A3780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A3784: 4BF91984  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3788 size=312
    let mut pc: u32 = 0x825A3788;
    'dispatch: loop {
        match pc {
            0x825A3788 => {
    //   block [0x825A3788..0x825A38C0)
	// 825A3788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A378C: 4BF9192D  bl 0x825350b8
	ctx.lr = 0x825A3790;
	sub_82535080(ctx, base);
	// 825A3790: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3794: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A379C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A37A0: 395F0034  addi r10, r31, 0x34
	ctx.r[10].s64 = ctx.r[31].s64 + 52;
	// 825A37A4: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 825A37A8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A37AC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825A37B0: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A37B4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825A37B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A37BC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A37C0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A37C4: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A37C8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A37CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A37D0: 419A000C  beq cr6, 0x825a37dc
	if ctx.cr[6].eq {
	pc = 0x825A37DC; continue 'dispatch;
	}
	// 825A37D4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A37D8: 48000008  b 0x825a37e0
	pc = 0x825A37E0; continue 'dispatch;
	// 825A37DC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 825A37E0: 997F003C  stb r11, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 825A37E4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A37E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A37EC: 419A001C  beq cr6, 0x825a3808
	if ctx.cr[6].eq {
	pc = 0x825A3808; continue 'dispatch;
	}
	// 825A37F0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A37F4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A37F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A37FC: 419A0054  beq cr6, 0x825a3850
	if ctx.cr[6].eq {
	pc = 0x825A3850; continue 'dispatch;
	}
	// 825A3800: 4BFE0B89  bl 0x82584388
	ctx.lr = 0x825A3804;
	sub_82584388(ctx, base);
	// 825A3804: 4800004C  b 0x825a3850
	pc = 0x825A3850; continue 'dispatch;
	// 825A3808: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A380C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3810: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 825A3814: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3818: FBAB0000  std r29, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 825A381C: FBAB0008  std r29, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 825A3820: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 825A3824: 897E0001  lbz r11, 1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 825A3828: 9BA10054  stb r29, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u8 ) };
	// 825A382C: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 825A3830: 99610055  stb r11, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[11].u8 ) };
	// 825A3834: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 825A3838: 616BBB80  ori r11, r11, 0xbb80
	ctx.r[11].u64 = ctx.r[11].u64 | 48000;
	// 825A383C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825A3840: 4BFE0F01  bl 0x82584740
	ctx.lr = 0x825A3844;
	sub_82584740(ctx, base);
	// 825A3844: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A3848: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825A384C: 41980068  blt cr6, 0x825a38b4
	if ctx.cr[6].lt {
	pc = 0x825A38B4; continue 'dispatch;
	}
	// 825A3850: 897F003C  lbz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A3854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3858: 419A002C  beq cr6, 0x825a3884
	if ctx.cr[6].eq {
	pc = 0x825A3884; continue 'dispatch;
	}
	// 825A385C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3860: 55641838  slwi r4, r11, 3
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 825A3864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3868: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A386C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3870: 4E800421  bctrl
	ctx.lr = 0x825A3874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3874: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A3878: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 825A387C: 419A0028  beq cr6, 0x825a38a4
	if ctx.cr[6].eq {
	pc = 0x825A38A4; continue 'dispatch;
	}
	// 825A3880: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 825A3884: 897F003C  lbz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A3888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A388C: 419A0028  beq cr6, 0x825a38b4
	if ctx.cr[6].eq {
	pc = 0x825A38B4; continue 'dispatch;
	}
	// 825A3890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3894: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3898: 4BFFF7A1  bl 0x825a3038
	ctx.lr = 0x825A389C;
	sub_825A3038(ctx, base);
	// 825A389C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A38A0: 4BF91868  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 825A38A4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 825A38A8: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 825A38AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A38B0: 4BF91858  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 825A38B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A38B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A38BC: 4BF9184C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A38C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A38C0 size=108
    let mut pc: u32 = 0x825A38C0;
    'dispatch: loop {
        match pc {
            0x825A38C0 => {
    //   block [0x825A38C0..0x825A392C)
	// 825A38C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A38C4: 4BF917F9  bl 0x825350bc
	ctx.lr = 0x825A38C8;
	sub_82535080(ctx, base);
	// 825A38C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A38CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A38D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825A38D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A38D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A38DC: 895F003C  lbz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A38E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A38E4: 419A0040  beq cr6, 0x825a3924
	if ctx.cr[6].eq {
	pc = 0x825A3924; continue 'dispatch;
	}
	// 825A38E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A38EC: 41980038  blt cr6, 0x825a3924
	if ctx.cr[6].lt {
	pc = 0x825A3924; continue 'dispatch;
	}
	// 825A38F0: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A38F4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A38F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825A38FC: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A3900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3904: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A3908: 4BFFF7A9  bl 0x825a30b0
	ctx.lr = 0x825A390C;
	sub_825A30B0(ctx, base);
	// 825A390C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 825A3910: 895F003C  lbz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A3914: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A3918: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 825A391C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A3920: 4198FFC8  blt cr6, 0x825a38e8
	if ctx.cr[6].lt {
	pc = 0x825A38E8; continue 'dispatch;
	}
	// 825A3924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3928: 4BF917E4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3930 size=128
    let mut pc: u32 = 0x825A3930;
    'dispatch: loop {
        match pc {
            0x825A3930 => {
    //   block [0x825A3930..0x825A39B0)
	// 825A3930: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3934: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 825A3938: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 825A393C: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825A3940: C18B6E7C  lfs f12, 0x6e7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28284 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3944: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3948: C1696E84  lfs f11, 0x6e84(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28292 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A394C: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 825A3950: D1830034  stfs f12, 0x34(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A3954: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A3958: D1630048  stfs f11, 0x48(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825A395C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A3960: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 825A3964: C00B6E80  lfs f0, 0x6e80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3968: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 825A396C: C1A96E88  lfs f13, 0x6e88(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28296 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A3970: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825A3974: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A3978: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 825A397C: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A3980: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 825A3984: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825A3988: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825A398C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A3990: D1A3004C  stfs f13, 0x4c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825A3994: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 825A3998: D1A30050  stfs f13, 0x50(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A399C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A39A0: 91230024  stw r9, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 825A39A4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825A39A8: 9123002C  stw r9, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[9].u32 ) };
	// 825A39AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A39B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A39B0 size=360
    let mut pc: u32 = 0x825A39B0;
    'dispatch: loop {
        match pc {
            0x825A39B0 => {
    //   block [0x825A39B0..0x825A3B18)
	// 825A39B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A39B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A39B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A39BC: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 825A39C0: 4BF92621  bl 0x82535fe0
	ctx.lr = 0x825A39C4;
	sub_82535FB0(ctx, base);
	// 825A39C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A39C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A39CC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A39D0: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A39D4: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A39D8: EFA0682A  fadds f29, f0, f13
	ctx.f[29].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825A39DC: C34B6E80  lfs f26, 0x6e80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28288 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 825A39E0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A39E4: C00B6E84  lfs f0, 0x6e84(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A39E8: FF1DD000  fcmpu cr6, f29, f26
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[26].f64);
	// 825A39EC: 4098000C  bge cr6, 0x825a39f8
	if !ctx.cr[6].lt {
	pc = 0x825A39F8; continue 'dispatch;
	}
	// 825A39F0: EF8DE824  fdivs f28, f13, f29
	ctx.f[28].f64 = ((ctx.f[13].f64 / ctx.f[29].f64) as f32) as f64;
	// 825A39F4: 48000008  b 0x825a39fc
	pc = 0x825A39FC; continue 'dispatch;
	// 825A39F8: FF800090  fmr f28, f0
	ctx.f[28].f64 = ctx.f[0].f64;
	// 825A39FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3A00: ED80E028  fsubs f12, f0, f28
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[28].f64) as f32) as f64);
	// 825A3A04: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A3A08: C80B7B18  lfd f0, 0x7b18(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(31512 as u32) ) };
	// 825A3A0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3A10: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 825A3A14: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A3A18: ED8C0772  fmuls f12, f12, f29
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[29].f64) as f32) as f64);
	// 825A3A1C: CBEB2260  lfd f31, 0x2260(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8800 as u32) ) };
	// 825A3A20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A3A24: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825A3A28: FDA06824  fdiv f13, f0, f13
	ctx.f[13].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 825A3A2C: C00B8E30  lfs f0, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3A30: EC4C0032  fmuls f2, f12, f0
	ctx.f[2].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3A34: FF606818  frsp f27, f13
	ctx.f[27].f64 = (ctx.f[13].f64 as f32) as f64;
	// 825A3A38: 4BF8FD39  bl 0x82533770
	ctx.lr = 0x825A3A3C;
	sub_82533770(ctx, base);
	// 825A3A3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3A40: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A3A44: CBCB20E0  lfd f30, 0x20e0(r11)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8416 as u32) ) };
	// 825A3A48: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825A3A4C: 4BF8FD25  bl 0x82533770
	ctx.lr = 0x825A3A50;
	sub_82533770(ctx, base);
	// 825A3A50: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 825A3A54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A3A58: EDBC0772  fmuls f13, f28, f29
	ctx.f[13].f64 = (((ctx.f[28].f64 * ctx.f[29].f64) as f32) as f64);
	// 825A3A5C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825A3A60: FFE00018  frsp f31, f0
	ctx.f[31].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A3A64: C00B2954  lfs f0, 0x2954(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3A68: EC4D0032  fmuls f2, f13, f0
	ctx.f[2].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3A6C: 4BF8FD05  bl 0x82533770
	ctx.lr = 0x825A3A70;
	sub_82533770(ctx, base);
	// 825A3A70: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A3A74: D03F0018  stfs f1, 0x18(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3A78: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825A3A7C: 4BF8FCF5  bl 0x82533770
	ctx.lr = 0x825A3A80;
	sub_82533770(ctx, base);
	// 825A3A80: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3A84: C00B2988  lfs f0, 0x2988(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10632 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3A88: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 825A3A8C: FC000818  frsp f0, f1
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A3A90: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3A94: 40980064  bge cr6, 0x825a3af8
	if !ctx.cr[6].lt {
	pc = 0x825A3AF8; continue 'dispatch;
	}
	// 825A3A98: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 825A3A9C: 4BF9288D  bl 0x82536328
	ctx.lr = 0x825A3AA0;
	sub_82536328(ctx, base);
	// 825A3AA0: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A3AA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3AA8: ED7F07F2  fmuls f11, f31, f31
	ctx.f[11].f64 = (((ctx.f[31].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A3AAC: C98B2020  lfd f12, 0x2020(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8224 as u32) ) };
	// 825A3AB0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3AB4: ED200032  fmuls f9, f0, f0
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3AB8: C9AB2000  lfd f13, 0x2000(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8192 as u32) ) };
	// 825A3ABC: ED0007F2  fmuls f8, f0, f31
	ctx.f[8].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A3AC0: FC0D0028  fsub f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 825A3AC4: FD4DF828  fsub f10, f13, f31
	ctx.f[10].f64 = ctx.f[13].f64 - ctx.f[31].f64;
	// 825A3AC8: FD2D4828  fsub f9, f13, f9
	ctx.f[9].f64 = ctx.f[13].f64 - ctx.f[9].f64;
	// 825A3ACC: FDAD4028  fsub f13, f13, f8
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[8].f64;
	// 825A3AD0: FC0007F2  fmul f0, f0, f31
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 825A3AD4: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 825A3AD8: FD6902F2  fmul f11, f9, f11
	ctx.f[11].f64 = ctx.f[9].f64 * ctx.f[11].f64;
	// 825A3ADC: FC005B38  fmsub f0, f0, f12, f11
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[12].f64 - ctx.f[11].f64;
	// 825A3AE0: FC00002C  fsqrt f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64).sqrt();
	// 825A3AE4: FC0D0028  fsub f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 825A3AE8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A3AEC: EC005024  fdivs f0, f0, f10
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[10].f64) as f32) as f64;
	// 825A3AF0: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A3AF4: 48000008  b 0x825a3afc
	pc = 0x825A3AFC; continue 'dispatch;
	// 825A3AF8: D35F001C  stfs f26, 0x1c(r31)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A3AFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A3B00: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 825A3B04: 4BF92529  bl 0x8253602c
	ctx.lr = 0x825A3B08;
	sub_82535FFC(ctx, base);
	// 825A3B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3B10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3B18 size=100
    let mut pc: u32 = 0x825A3B18;
    'dispatch: loop {
        match pc {
            0x825A3B18 => {
    //   block [0x825A3B18..0x825A3B7C)
	// 825A3B18: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825A3B1C: 39400800  li r10, 0x800
	ctx.r[10].s64 = 2048;
	// 825A3B20: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A3B28: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A3B2C: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3B34: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A3B38: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B3C: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A3B40: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B44: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3B48: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B4C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3B50: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B54: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3B58: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B5C: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3B60: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B64: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3B68: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825A3B6C: 409AFFB4  bne cr6, 0x825a3b20
	if !ctx.cr[6].eq {
	pc = 0x825A3B20; continue 'dispatch;
	}
	// 825A3B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3B74: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3B80 size=100
    let mut pc: u32 = 0x825A3B80;
    'dispatch: loop {
        match pc {
            0x825A3B80 => {
    //   block [0x825A3B80..0x825A3BE4)
	// 825A3B80: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825A3B84: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 825A3B88: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B8C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A3B90: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A3B94: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3B9C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A3BA0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3BA4: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A3BA8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3BAC: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3BB0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3BB4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3BB8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3BBC: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3BC0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3BC4: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3BC8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3BCC: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3BD0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825A3BD4: 409AFFB4  bne cr6, 0x825a3b88
	if !ctx.cr[6].eq {
	pc = 0x825A3B88; continue 'dispatch;
	}
	// 825A3BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3BDC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3BE8 size=100
    let mut pc: u32 = 0x825A3BE8;
    'dispatch: loop {
        match pc {
            0x825A3BE8 => {
    //   block [0x825A3BE8..0x825A3C4C)
	// 825A3BE8: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825A3BEC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A3BF0: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3BF4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A3BF8: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A3BFC: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3C04: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A3C08: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C0C: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A3C10: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C14: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3C18: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C1C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3C20: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C24: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3C28: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C2C: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3C30: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C34: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3C38: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825A3C3C: 409AFFB4  bne cr6, 0x825a3bf0
	if !ctx.cr[6].eq {
	pc = 0x825A3BF0; continue 'dispatch;
	}
	// 825A3C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3C44: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3C50 size=100
    let mut pc: u32 = 0x825A3C50;
    'dispatch: loop {
        match pc {
            0x825A3C50 => {
    //   block [0x825A3C50..0x825A3CB4)
	// 825A3C50: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825A3C54: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 825A3C58: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C5C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A3C60: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A3C64: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3C6C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A3C70: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C74: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A3C78: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C7C: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3C80: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C84: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3C88: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C8C: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3C90: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C94: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3C98: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3C9C: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3CA0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825A3CA4: 409AFFB4  bne cr6, 0x825a3c58
	if !ctx.cr[6].eq {
	pc = 0x825A3C58; continue 'dispatch;
	}
	// 825A3CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3CAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3CB8 size=100
    let mut pc: u32 = 0x825A3CB8;
    'dispatch: loop {
        match pc {
            0x825A3CB8 => {
    //   block [0x825A3CB8..0x825A3D1C)
	// 825A3CB8: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825A3CBC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 825A3CC0: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3CC4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A3CC8: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A3CCC: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3CD0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3CD4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A3CD8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3CDC: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A3CE0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3CE4: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3CE8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3CEC: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3CF0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3CF4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3CF8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3CFC: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3D00: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D04: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3D08: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825A3D0C: 409AFFB4  bne cr6, 0x825a3cc0
	if !ctx.cr[6].eq {
	pc = 0x825A3CC0; continue 'dispatch;
	}
	// 825A3D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3D14: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3D20 size=100
    let mut pc: u32 = 0x825A3D20;
    'dispatch: loop {
        match pc {
            0x825A3D20 => {
    //   block [0x825A3D20..0x825A3D84)
	// 825A3D20: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825A3D24: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 825A3D28: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D2C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A3D30: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A3D34: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3D3C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A3D40: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D44: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A3D48: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D4C: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3D50: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D54: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3D58: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D5C: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3D60: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D64: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3D68: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D6C: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3D70: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825A3D74: 409AFFB4  bne cr6, 0x825a3d28
	if !ctx.cr[6].eq {
	pc = 0x825A3D28; continue 'dispatch;
	}
	// 825A3D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3D7C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3D88 size=100
    let mut pc: u32 = 0x825A3D88;
    'dispatch: loop {
        match pc {
            0x825A3D88 => {
    //   block [0x825A3D88..0x825A3DEC)
	// 825A3D88: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825A3D8C: 39400200  li r10, 0x200
	ctx.r[10].s64 = 512;
	// 825A3D90: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3D94: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A3D98: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A3D9C: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3DA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3DA4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A3DA8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3DAC: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A3DB0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3DB4: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3DB8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3DBC: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3DC0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3DC4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3DC8: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3DCC: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3DD0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3DD4: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A3DD8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825A3DDC: 409AFFB4  bne cr6, 0x825a3d90
	if !ctx.cr[6].eq {
	pc = 0x825A3D90; continue 'dispatch;
	}
	// 825A3DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3DE4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3DF0 size=8
    let mut pc: u32 = 0x825A3DF0;
    'dispatch: loop {
        match pc {
            0x825A3DF0 => {
    //   block [0x825A3DF0..0x825A3DF8)
	// 825A3DF0: 90830064  stw r4, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[4].u32 ) };
	// 825A3DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3DF8 size=20
    let mut pc: u32 = 0x825A3DF8;
    'dispatch: loop {
        match pc {
            0x825A3DF8 => {
    //   block [0x825A3DF8..0x825A3E0C)
	// 825A3DF8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3DFC: C00B20B0  lfs f0, 0x20b0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8368 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3E00: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3E04: D003005C  stfs f0, 0x5c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 825A3E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A3E10 size=3632
    let mut pc: u32 = 0x825A3E10;
    'dispatch: loop {
        match pc {
            0x825A3E10 => {
    //   block [0x825A3E10..0x825A4C40)
	// 825A3E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3E14: 4BF9126D  bl 0x82535080
	ctx.lr = 0x825A3E18;
	sub_82535080(ctx, base);
	// 825A3E18: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 825A3E1C: 4BF92195  bl 0x82535fb0
	ctx.lr = 0x825A3E20;
	sub_82535FB0(ctx, base);
	// 825A3E20: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A3E28: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 825A3E2C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A3E30: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825A3E34: 4BF90D1D  bl 0x82534b50
	ctx.lr = 0x825A3E38;
	sub_82534B50(ctx, base);
	// 825A3E38: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825A3E3C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3E40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3E44: C00A2238  lfs f0, 0x2238(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3E48: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825A3E4C: 409A000C  bne cr6, 0x825a3e58
	if !ctx.cr[6].eq {
	pc = 0x825A3E58; continue 'dispatch;
	}
	// 825A3E50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A3E54: 48000034  b 0x825a3e88
	pc = 0x825A3E88; continue 'dispatch;
	// 825A3E58: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825A3E5C: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A3E60: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A3E64: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 825A3E68: C9810078  lfd f12, 0x78(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 825A3E6C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 825A3E70: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 825A3E74: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A3E78: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3E7C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A3E80: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A3E84: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A3E88: 48001E91  bl 0x825a5d18
	ctx.lr = 0x825A3E8C;
	sub_825A5D18(ctx, base);
	// 825A3E8C: 2B030FFF  cmplwi cr6, r3, 0xfff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 4095 as u32, &mut ctx.xer);
	// 825A3E90: 40990008  ble cr6, 0x825a3e98
	if !ctx.cr[6].gt {
	pc = 0x825A3E98; continue 'dispatch;
	}
	// 825A3E94: 38600FFF  li r3, 0xfff
	ctx.r[3].s64 = 4095;
	// 825A3E98: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A3E9C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 825A3EA0: 616B9F08  ori r11, r11, 0x9f08
	ctx.r[11].u64 = ctx.r[11].u64 | 40712;
	// 825A3EA4: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825A3EA8: 3D000004  lis r8, 4
	ctx.r[8].s64 = 262144;
	// 825A3EAC: 614AEFC4  ori r10, r10, 0xefc4
	ctx.r[10].u64 = ctx.r[10].u64 | 61380;
	// 825A3EB0: 6129A060  ori r9, r9, 0xa060
	ctx.r[9].u64 = ctx.r[9].u64 | 41056;
	// 825A3EB4: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 825A3EB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A3EBC: 61085100  ori r8, r8, 0x5100
	ctx.r[8].u64 = ctx.r[8].u64 | 20736;
	// 825A3EC0: 7C7F512E  stwx r3, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 825A3EC4: 7C7F492E  stwx r3, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[3].u32) };
	// 825A3EC8: C14B7580  lfs f10, 0x7580(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30080 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A3ECC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3ED0: 7C7F412E  stwx r3, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[3].u32) };
	// 825A3ED4: C01D0050  lfs f0, 0x50(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3ED8: 3BCB6E90  addi r30, r11, 0x6e90
	ctx.r[30].s64 = ctx.r[11].s64 + 28304;
	// 825A3EDC: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 825A3EE0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3EE4: C1AB6E84  lfs f13, 0x6e84(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A3EE8: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A3EEC: 4198001C  blt cr6, 0x825a3f08
	if ctx.cr[6].lt {
	pc = 0x825A3F08; continue 'dispatch;
	}
	// 825A3EF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3EF4: C18B20B0  lfs f12, 0x20b0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8368 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3EF8: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A3EFC: C19F0058  lfs f12, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3F00: ED800332  fmuls f12, f0, f12
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A3F04: 48000028  b 0x825a3f2c
	pc = 0x825A3F2C; continue 'dispatch;
	// 825A3F08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A3F0C: C19E0004  lfs f12, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3F10: C13F0058  lfs f9, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A3F14: C16B76F4  lfs f11, 0x76f4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A3F18: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3F1C: ED805B3A  fmadds f12, f0, f12, f11
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 825A3F20: C00B20B0  lfs f0, 0x20b0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8368 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3F24: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3F28: ED800272  fmuls f12, f0, f9
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 825A3F2C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3F30: C00B7B74  lfs f0, 0x7b74(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31604 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3F34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A3F38: EF2C0032  fmuls f25, f12, f0
	ctx.f[25].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3F3C: C00B8E28  lfs f0, -0x71d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29144 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3F40: FF190000  fcmpu cr6, f25, f0
	ctx.cr[6].compare_f64(ctx.f[25].f64, ctx.f[0].f64);
	// 825A3F44: 40980008  bge cr6, 0x825a3f4c
	if !ctx.cr[6].lt {
	pc = 0x825A3F4C; continue 'dispatch;
	}
	// 825A3F48: FF200090  fmr f25, f0
	ctx.f[25].f64 = ctx.f[0].f64;
	// 825A3F4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3F50: C18B2068  lfs f12, 0x2068(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3F54: EFD90332  fmuls f30, f25, f12
	ctx.f[30].f64 = (((ctx.f[25].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A3F58: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 825A3F5C: 4098000C  bge cr6, 0x825a3f68
	if !ctx.cr[6].lt {
	pc = 0x825A3F68; continue 'dispatch;
	}
	// 825A3F60: FFC00090  fmr f30, f0
	ctx.f[30].f64 = ctx.f[0].f64;
	// 825A3F64: 48000010  b 0x825a3f74
	pc = 0x825A3F74; continue 'dispatch;
	// 825A3F68: FF1E6800  fcmpu cr6, f30, f13
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[13].f64);
	// 825A3F6C: 40990008  ble cr6, 0x825a3f74
	if !ctx.cr[6].gt {
	pc = 0x825A3F74; continue 'dispatch;
	}
	// 825A3F70: FFC06890  fmr f30, f13
	ctx.f[30].f64 = ctx.f[13].f64;
	// 825A3F74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3F78: C01D004C  lfs f0, 0x4c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3F7C: C18B2428  lfs f12, 0x2428(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9256 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3F80: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 825A3F84: 41980024  blt cr6, 0x825a3fa8
	if ctx.cr[6].lt {
	pc = 0x825A3FA8; continue 'dispatch;
	}
	// 825A3F88: ED806028  fsubs f12, f0, f12
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 825A3F8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3F90: FFA06890  fmr f29, f13
	ctx.f[29].f64 = ctx.f[13].f64;
	// 825A3F94: FF806890  fmr f28, f13
	ctx.f[28].f64 = ctx.f[13].f64;
	// 825A3F98: FF606890  fmr f27, f13
	ctx.f[27].f64 = ctx.f[13].f64;
	// 825A3F9C: C00B24D0  lfs f0, 0x24d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9424 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3FA0: EFEC0032  fmuls f31, f12, f0
	ctx.f[31].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3FA4: 48000070  b 0x825a4014
	pc = 0x825A4014; continue 'dispatch;
	// 825A3FA8: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 825A3FAC: 41980028  blt cr6, 0x825a3fd4
	if ctx.cr[6].lt {
	pc = 0x825A3FD4; continue 'dispatch;
	}
	// 825A3FB0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3FB4: ED805028  fsubs f12, f0, f10
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 825A3FB8: FF806890  fmr f28, f13
	ctx.f[28].f64 = ctx.f[13].f64;
	// 825A3FBC: FF606890  fmr f27, f13
	ctx.f[27].f64 = ctx.f[13].f64;
	// 825A3FC0: C3EB6E80  lfs f31, 0x6e80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28288 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A3FC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3FC8: C00B24D0  lfs f0, 0x24d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9424 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3FCC: EFAC0032  fmuls f29, f12, f0
	ctx.f[29].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A3FD0: 48000044  b 0x825a4014
	pc = 0x825A4014; continue 'dispatch;
	// 825A3FD4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3FD8: C18B2108  lfs f12, 0x2108(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8456 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3FDC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A3FE0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 825A3FE4: C3EB6E80  lfs f31, 0x6e80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28288 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A3FE8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A3FEC: FFA0F890  fmr f29, f31
	ctx.f[29].f64 = ctx.f[31].f64;
	// 825A3FF0: 41980018  blt cr6, 0x825a4008
	if ctx.cr[6].lt {
	pc = 0x825A4008; continue 'dispatch;
	}
	// 825A3FF4: ED806028  fsubs f12, f0, f12
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 825A3FF8: C00B24D0  lfs f0, 0x24d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9424 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3FFC: FF606890  fmr f27, f13
	ctx.f[27].f64 = ctx.f[13].f64;
	// 825A4000: EF8C0032  fmuls f28, f12, f0
	ctx.f[28].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4004: 48000010  b 0x825a4014
	pc = 0x825A4014; continue 'dispatch;
	// 825A4008: C1AB24D0  lfs f13, 0x24d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9424 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A400C: FF80F890  fmr f28, f31
	ctx.f[28].f64 = ctx.f[31].f64;
	// 825A4010: EF600372  fmuls f27, f0, f13
	ctx.f[27].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A4014: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4018: C00B7B70  lfs f0, 0x7b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31600 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A401C: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A4020: EC1E0032  fmuls f0, f30, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4024: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4028: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A402C: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4030: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A4034: 48001CE5  bl 0x825a5d18
	ctx.lr = 0x825A4038;
	sub_825A5D18(ctx, base);
	// 825A4038: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A403C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 825A4040: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825A4044: 614A00F0  ori r10, r10, 0xf0
	ctx.r[10].u64 = ctx.r[10].u64 | 240;
	// 825A4048: 612900FC  ori r9, r9, 0xfc
	ctx.r[9].u64 = ctx.r[9].u64 | 252;
	// 825A404C: C00B7B6C  lfs f0, 0x7b6c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31596 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4050: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A4054: EC1E0032  fmuls f0, f30, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4058: 7C7F512E  stwx r3, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 825A405C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4060: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A4064: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 825A4068: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A406C: 7D7F492E  stwx r11, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 825A4070: 48001CA9  bl 0x825a5d18
	ctx.lr = 0x825A4074;
	sub_825A5D18(ctx, base);
	// 825A4074: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4078: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 825A407C: 616B0B34  ori r11, r11, 0xb34
	ctx.r[11].u64 = ctx.r[11].u64 | 2868;
	// 825A4080: 3903FFFF  addi r8, r3, -1
	ctx.r[8].s64 = ctx.r[3].s64 + -1;
	// 825A4084: 614A0B40  ori r10, r10, 0xb40
	ctx.r[10].u64 = ctx.r[10].u64 | 2880;
	// 825A4088: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 825A408C: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 825A4090: 7D1F512E  stwx r8, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 825A4094: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4098: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 825A409C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 825A40A0: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A40A4: 556B072A  rlwinm r11, r11, 0, 0x1c, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A40A8: 396B03FC  addi r11, r11, 0x3fc
	ctx.r[11].s64 = ctx.r[11].s64 + 1020;
	// 825A40AC: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825A40B0: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 825A40B4: C8010078  lfd f0, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 825A40B8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A40BC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A40C0: EC0007B2  fmuls f0, f0, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 825A40C4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A40C8: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 825A40CC: 83610058  lwz r27, 0x58(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A40D0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825A40D4: 48001C45  bl 0x825a5d18
	ctx.lr = 0x825A40D8;
	sub_825A5D18(ctx, base);
	// 825A40D8: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 825A40DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A40E0: 614A3790  ori r10, r10, 0x3790
	ctx.r[10].u64 = ctx.r[10].u64 | 14224;
	// 825A40E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A40E8: 7D7F512E  stwx r11, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 825A40EC: 48001C2D  bl 0x825a5d18
	ctx.lr = 0x825A40F0;
	sub_825A5D18(ctx, base);
	// 825A40F0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A40F4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 825A40F8: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825A40FC: 614A47B8  ori r10, r10, 0x47b8
	ctx.r[10].u64 = ctx.r[10].u64 | 18360;
	// 825A4100: 612947C4  ori r9, r9, 0x47c4
	ctx.r[9].u64 = ctx.r[9].u64 | 18372;
	// 825A4104: C00B7B68  lfs f0, 0x7b68(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31592 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4108: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A410C: EC1E0032  fmuls f0, f30, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4110: 7C7F512E  stwx r3, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 825A4114: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4118: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A411C: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 825A4120: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4124: 7D7F492E  stwx r11, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 825A4128: 48001BF1  bl 0x825a5d18
	ctx.lr = 0x825A412C;
	sub_825A5D18(ctx, base);
	// 825A412C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A4130: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 825A4134: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825A4138: 614A51FC  ori r10, r10, 0x51fc
	ctx.r[10].u64 = ctx.r[10].u64 | 20988;
	// 825A413C: 61295208  ori r9, r9, 0x5208
	ctx.r[9].u64 = ctx.r[9].u64 | 21000;
	// 825A4140: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 825A4144: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825A4148: 7D7F512E  stwx r11, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 825A414C: 7D1F492E  stwx r8, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 825A4150: 48001BC9  bl 0x825a5d18
	ctx.lr = 0x825A4154;
	sub_825A5D18(ctx, base);
	// 825A4154: EF5F07B2  fmuls f26, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[26].f64 = (((ctx.f[31].f64 * ctx.f[30].f64) as f32) as f64);
	// 825A4158: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A415C: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A4160: 616B7E58  ori r11, r11, 0x7e58
	ctx.r[11].u64 = ctx.r[11].u64 | 32344;
	// 825A4164: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 825A4168: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A416C: C00B7B64  lfs f0, 0x7b64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31588 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4170: EC1A0032  fmuls f0, f26, f0
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4174: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4178: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A417C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4180: 48001B99  bl 0x825a5d18
	ctx.lr = 0x825A4184;
	sub_825A5D18(ctx, base);
	// 825A4184: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A4188: 409A0008  bne cr6, 0x825a4190
	if !ctx.cr[6].eq {
	pc = 0x825A4190; continue 'dispatch;
	}
	// 825A418C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4190: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A4194: 3DFF0003  addis r15, r31, 3
	ctx.r[15].s64 = ctx.r[31].s64 + 196608;
	// 825A4198: 3DDF0003  addis r14, r31, 3
	ctx.r[14].s64 = ctx.r[31].s64 + 196608;
	// 825A419C: 39EF8E78  addi r15, r15, -0x7188
	ctx.r[15].s64 = ctx.r[15].s64 + -29064;
	// 825A41A0: 39CE9690  addi r14, r14, -0x6970
	ctx.r[14].s64 = ctx.r[14].s64 + -26992;
	// 825A41A4: C00B2CD8  lfs f0, 0x2cd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11480 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A41A8: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A41AC: EC1E0032  fmuls f0, f30, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A41B0: 906F0000  stw r3, 0(r15)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[15].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A41B4: 91E10064  stw r15, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[15].u32 ) };
	// 825A41B8: 91C10068  stw r14, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[14].u32 ) };
	// 825A41BC: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A41C0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A41C4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A41C8: 48001B51  bl 0x825a5d18
	ctx.lr = 0x825A41CC;
	sub_825A5D18(ctx, base);
	// 825A41CC: EDBF0672  fmuls f13, f31, f25
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[25].f64) as f32) as f64);
	// 825A41D0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A41D4: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A41D8: 906E0000  stw r3, 0(r14)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[14].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A41DC: C00B7B60  lfs f0, 0x7b60(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31584 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A41E0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A41E4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A41E8: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A41EC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A41F0: 48001B29  bl 0x825a5d18
	ctx.lr = 0x825A41F4;
	sub_825A5D18(ctx, base);
	// 825A41F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A41F8: 409A0008  bne cr6, 0x825a4200
	if !ctx.cr[6].eq {
	pc = 0x825A4200; continue 'dispatch;
	}
	// 825A41FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4200: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4204: 3E1F0003  addis r16, r31, 3
	ctx.r[16].s64 = ctx.r[31].s64 + 196608;
	// 825A4208: 3A109EFC  addi r16, r16, -0x6104
	ctx.r[16].s64 = ctx.r[16].s64 + -24836;
	// 825A420C: C00B7B5C  lfs f0, 0x7b5c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4210: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A4214: EC1A0032  fmuls f0, f26, f0
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4218: 90700000  stw r3, 0(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A421C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4220: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A4224: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4228: 48001AF1  bl 0x825a5d18
	ctx.lr = 0x825A422C;
	sub_825A5D18(ctx, base);
	// 825A422C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A4230: 409A0008  bne cr6, 0x825a4238
	if !ctx.cr[6].eq {
	pc = 0x825A4238; continue 'dispatch;
	}
	// 825A4234: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4238: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A423C: 3E5F0003  addis r18, r31, 3
	ctx.r[18].s64 = ctx.r[31].s64 + 196608;
	// 825A4240: 3E3F0003  addis r17, r31, 3
	ctx.r[17].s64 = ctx.r[31].s64 + 196608;
	// 825A4244: 3A52DF34  addi r18, r18, -0x20cc
	ctx.r[18].s64 = ctx.r[18].s64 + -8396;
	// 825A4248: 3A31E74C  addi r17, r17, -0x18b4
	ctx.r[17].s64 = ctx.r[17].s64 + -6324;
	// 825A424C: C00B7B58  lfs f0, 0x7b58(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31576 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4250: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A4254: EC1E0032  fmuls f0, f30, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4258: 90720000  stw r3, 0(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A425C: 9241006C  stw r18, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[18].u32 ) };
	// 825A4260: 92210070  stw r17, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[17].u32 ) };
	// 825A4264: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4268: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A426C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4270: 48001AA9  bl 0x825a5d18
	ctx.lr = 0x825A4274;
	sub_825A5D18(ctx, base);
	// 825A4274: EDBD0672  fmuls f13, f29, f25
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[29].f64 * ctx.f[25].f64) as f32) as f64);
	// 825A4278: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A427C: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A4280: 90710000  stw r3, 0(r17)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A4284: C00B7B54  lfs f0, 0x7b54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4288: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A428C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4290: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A4294: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4298: 48001A81  bl 0x825a5d18
	ctx.lr = 0x825A429C;
	sub_825A5D18(ctx, base);
	// 825A429C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A42A0: 409A0008  bne cr6, 0x825a42a8
	if !ctx.cr[6].eq {
	pc = 0x825A42A8; continue 'dispatch;
	}
	// 825A42A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A42A8: EF9C07B2  fmuls f28, f28, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[30].f64) as f32) as f64);
	// 825A42AC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A42B0: 3E7F0003  addis r19, r31, 3
	ctx.r[19].s64 = ctx.r[31].s64 + 196608;
	// 825A42B4: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A42B8: 3A73EFB8  addi r19, r19, -0x1048
	ctx.r[19].s64 = ctx.r[19].s64 + -4168;
	// 825A42BC: C00B7B50  lfs f0, 0x7b50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A42C0: 90730000  stw r3, 0(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A42C4: EC1C0032  fmuls f0, f28, f0
	ctx.f[0].f64 = (((ctx.f[28].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A42C8: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A42CC: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A42D0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A42D4: 48001A45  bl 0x825a5d18
	ctx.lr = 0x825A42D8;
	sub_825A5D18(ctx, base);
	// 825A42D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A42DC: 409A0008  bne cr6, 0x825a42e4
	if !ctx.cr[6].eq {
	pc = 0x825A42E4; continue 'dispatch;
	}
	// 825A42E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A42E4: EFFD07B2  fmuls f31, f29, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[29].f64 * ctx.f[30].f64) as f32) as f64);
	// 825A42E8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A42EC: 3E9F0003  addis r20, r31, 3
	ctx.r[20].s64 = ctx.r[31].s64 + 196608;
	// 825A42F0: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A42F4: 3A942FF0  addi r20, r20, 0x2ff0
	ctx.r[20].s64 = ctx.r[20].s64 + 12272;
	// 825A42F8: C00B7B4C  lfs f0, 0x7b4c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31564 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A42FC: 90740000  stw r3, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A4300: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4304: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4308: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A430C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4310: 48001A09  bl 0x825a5d18
	ctx.lr = 0x825A4314;
	sub_825A5D18(ctx, base);
	// 825A4314: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A4318: 409A0008  bne cr6, 0x825a4320
	if !ctx.cr[6].eq {
	pc = 0x825A4320; continue 'dispatch;
	}
	// 825A431C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4320: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4324: 3EDF0003  addis r22, r31, 3
	ctx.r[22].s64 = ctx.r[31].s64 + 196608;
	// 825A4328: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 825A432C: 3AD65008  addi r22, r22, 0x5008
	ctx.r[22].s64 = ctx.r[22].s64 + 20488;
	// 825A4330: 3EBF0003  addis r21, r31, 3
	ctx.r[21].s64 = ctx.r[31].s64 + 196608;
	// 825A4334: C00B7B48  lfs f0, 0x7b48(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31560 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4338: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A433C: EC1E0032  fmuls f0, f30, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4340: 3AB57024  addi r21, r21, 0x7024
	ctx.r[21].s64 = ctx.r[21].s64 + 28708;
	// 825A4344: 91560000  stw r10, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A4348: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A434C: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A4350: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4354: 480019C5  bl 0x825a5d18
	ctx.lr = 0x825A4358;
	sub_825A5D18(ctx, base);
	// 825A4358: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A435C: 90750000  stw r3, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A4360: C00B7B44  lfs f0, 0x7b44(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31556 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4364: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A4368: EC1A0032  fmuls f0, f26, f0
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A436C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4370: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A4374: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4378: 480019A1  bl 0x825a5d18
	ctx.lr = 0x825A437C;
	sub_825A5D18(ctx, base);
	// 825A437C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A4380: 409A0008  bne cr6, 0x825a4388
	if !ctx.cr[6].eq {
	pc = 0x825A4388; continue 'dispatch;
	}
	// 825A4384: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4388: EDBB0672  fmuls f13, f27, f25
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[27].f64 * ctx.f[25].f64) as f32) as f64);
	// 825A438C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4390: 3EFF0004  addis r23, r31, 4
	ctx.r[23].s64 = ctx.r[31].s64 + 262144;
	// 825A4394: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A4398: 3AF7903C  addi r23, r23, -0x6fc4
	ctx.r[23].s64 = ctx.r[23].s64 + -28612;
	// 825A439C: C00B7B40  lfs f0, 0x7b40(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31552 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A43A0: 90770000  stw r3, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A43A4: 92E10074  stw r23, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[23].u32 ) };
	// 825A43A8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A43AC: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A43B0: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A43B4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A43B8: 48001961  bl 0x825a5d18
	ctx.lr = 0x825A43BC;
	sub_825A5D18(ctx, base);
	// 825A43BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A43C0: 409A0008  bne cr6, 0x825a43c8
	if !ctx.cr[6].eq {
	pc = 0x825A43C8; continue 'dispatch;
	}
	// 825A43C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A43C8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A43CC: 3F1F0004  addis r24, r31, 4
	ctx.r[24].s64 = ctx.r[31].s64 + 262144;
	// 825A43D0: 3B18A054  addi r24, r24, -0x5fac
	ctx.r[24].s64 = ctx.r[24].s64 + -24492;
	// 825A43D4: C00B7B3C  lfs f0, 0x7b3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31548 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A43D8: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A43DC: EC1C0032  fmuls f0, f28, f0
	ctx.f[0].f64 = (((ctx.f[28].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A43E0: 90780000  stw r3, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A43E4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A43E8: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A43EC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A43F0: 48001929  bl 0x825a5d18
	ctx.lr = 0x825A43F4;
	sub_825A5D18(ctx, base);
	// 825A43F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A43F8: 409A0008  bne cr6, 0x825a4400
	if !ctx.cr[6].eq {
	pc = 0x825A4400; continue 'dispatch;
	}
	// 825A43FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4400: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4404: 3F3F0004  addis r25, r31, 4
	ctx.r[25].s64 = ctx.r[31].s64 + 262144;
	// 825A4408: 3B39E090  addi r25, r25, -0x1f70
	ctx.r[25].s64 = ctx.r[25].s64 + -8048;
	// 825A440C: C00B7B38  lfs f0, 0x7b38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4410: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A4414: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4418: 90790000  stw r3, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A441C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4420: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A4424: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4428: 480018F1  bl 0x825a5d18
	ctx.lr = 0x825A442C;
	sub_825A5D18(ctx, base);
	// 825A442C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A4430: 409A0008  bne cr6, 0x825a4438
	if !ctx.cr[6].eq {
	pc = 0x825A4438; continue 'dispatch;
	}
	// 825A4434: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4438: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A443C: 3F5F0004  addis r26, r31, 4
	ctx.r[26].s64 = ctx.r[31].s64 + 262144;
	// 825A4440: 3B5A00A8  addi r26, r26, 0xa8
	ctx.r[26].s64 = ctx.r[26].s64 + 168;
	// 825A4444: C00B7B34  lfs f0, 0x7b34(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31540 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4448: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A444C: EC1A0032  fmuls f0, f26, f0
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4450: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A4454: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4458: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A445C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A4460: 480018B9  bl 0x825a5d18
	ctx.lr = 0x825A4464;
	sub_825A5D18(ctx, base);
	// 825A4464: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A4468: 409A0008  bne cr6, 0x825a4470
	if !ctx.cr[6].eq {
	pc = 0x825A4470; continue 'dispatch;
	}
	// 825A446C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A4470: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A4474: 3F9F0004  addis r28, r31, 4
	ctx.r[28].s64 = ctx.r[31].s64 + 262144;
	// 825A4478: 3F7F0004  addis r27, r31, 4
	ctx.r[27].s64 = ctx.r[31].s64 + 262144;
	// 825A447C: 3B9C20C4  addi r28, r28, 0x20c4
	ctx.r[28].s64 = ctx.r[28].s64 + 8388;
	// 825A4480: 3B7B40DC  addi r27, r27, 0x40dc
	ctx.r[27].s64 = ctx.r[27].s64 + 16604;
	// 825A4484: C00B3050  lfs f0, 0x3050(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12368 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4488: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A448C: EC1A0032  fmuls f0, f26, f0
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4490: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A4494: 93610078  stw r27, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[27].u32 ) };
	// 825A4498: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A449C: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A44A0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A44A4: 48001875  bl 0x825a5d18
	ctx.lr = 0x825A44A8;
	sub_825A5D18(ctx, base);
	// 825A44A8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A44AC: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A44B0: C00B7B30  lfs f0, 0x7b30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31536 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A44B4: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 825A44B8: EC190032  fmuls f0, f25, f0
	ctx.f[0].f64 = (((ctx.f[25].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A44BC: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A44C0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A44C4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A44C8: 48001851  bl 0x825a5d18
	ctx.lr = 0x825A44CC;
	sub_825A5D18(ctx, base);
	// 825A44CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A44D0: 409A0008  bne cr6, 0x825a44d8
	if !ctx.cr[6].eq {
	pc = 0x825A44D8; continue 'dispatch;
	}
	// 825A44D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A44D8: 3D400004  lis r10, 4
	ctx.r[10].s64 = 262144;
	// 825A44DC: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A44E0: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A44E4: C01F0058  lfs f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A44E8: 615950F4  ori r25, r10, 0x50f4
	ctx.r[25].u64 = ctx.r[10].u64 | 20724;
	// 825A44EC: 81520000  lwz r10, 0(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A44F0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825A44F4: 80FB0000  lwz r7, 0(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A44F8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A44FC: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4500: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A4504: 81500000  lwz r10, 0(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4508: 839A0000  lwz r28, 0(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A450C: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 825A4510: 80950000  lwz r4, 0(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4514: 80B60000  lwz r5, 0(r22)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4518: CBC92260  lfd f30, 0x2260(r9)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8800 as u32) ) };
	// 825A451C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 825A4520: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4524: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 825A4528: 80CE0000  lwz r6, 0(r14)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A452C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825A4530: 80F70000  lwz r7, 0(r23)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4534: 810F0000  lwz r8, 0(r15)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4538: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A453C: 81340000  lwz r9, 0(r20)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4540: 81530000  lwz r10, 0(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4544: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 825A4548: 7C7FC92E  stwx r3, r31, r25
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[25].u32), ctx.r[3].u32) };
	// 825A454C: C1BD0048  lfs f13, 0x48(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4550: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825A4554: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 825A4558: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 825A455C: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 825A4560: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 825A4564: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825A4568: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A456C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A4570: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A4574: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825A4578: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825A457C: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825A4580: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 825A4584: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4588: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 825A458C: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 825A4590: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 825A4594: C9AB7B28  lfd f13, 0x7b28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(31528 as u32) ) };
	// 825A4598: FC4D0024  fdiv f2, f13, f0
	ctx.f[2].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 825A459C: 4BF8F1D5  bl 0x82533770
	ctx.lr = 0x825A45A0;
	sub_82533770(ctx, base);
	// 825A45A0: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825A45A4: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A45A8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A45AC: 813D000C  lwz r9, 0xc(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A45B0: 38FE0378  addi r7, r30, 0x378
	ctx.r[7].s64 = ctx.r[30].s64 + 888;
	// 825A45B4: 38DE00C0  addi r6, r30, 0xc0
	ctx.r[6].s64 = ctx.r[30].s64 + 192;
	// 825A45B8: 3B9E0178  addi r28, r30, 0x178
	ctx.r[28].s64 = ctx.r[30].s64 + 376;
	// 825A45BC: C9A820E0  lfd f13, 0x20e0(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8416 as u32) ) };
	// 825A45C0: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825A45C4: FDA10372  fmul f13, f1, f13
	ctx.f[13].f64 = ctx.f[1].f64 * ctx.f[13].f64;
	// 825A45C8: C00B7B20  lfs f0, 0x7b20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31520 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A45CC: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A45D0: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A45D4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825A45D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A45DC: 38BE01F8  addi r5, r30, 0x1f8
	ctx.r[5].s64 = ctx.r[30].s64 + 504;
	// 825A45E0: 389E0278  addi r4, r30, 0x278
	ctx.r[4].s64 = ctx.r[30].s64 + 632;
	// 825A45E4: 7F8B3C2E  lfsx f28, r11, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 825A45E8: 387E02F8  addi r3, r30, 0x2f8
	ctx.r[3].s64 = ctx.r[30].s64 + 760;
	// 825A45EC: 7D2B342E  lfsx f9, r11, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A45F0: 7D0BE42E  lfsx f8, r11, r28
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A45F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A45F8: 7F2A2C2E  lfsx f25, r10, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 825A45FC: ED60682C  fsqrts f11, f13
	ctx.f[11].f64 = ((ctx.f[13].f64).sqrt() as f32) as f64;
	// 825A4600: C1A8BFFC  lfs f13, -0x4004(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4604: ED4B0372  fmuls f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A4608: 7F09242E  lfsx f24, r9, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 825A460C: 7EEA1C2E  lfsx f23, r10, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 825A4610: C1AB270C  lfs f13, 0x270c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9996 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4614: EEDC0372  fmuls f22, f28, f13
	ctx.f[22].f64 = (((ctx.f[28].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A4618: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A461C: 3AFE0278  addi r23, r30, 0x278
	ctx.r[23].s64 = ctx.r[30].s64 + 632;
	// 825A4620: 835D0018  lwz r26, 0x18(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A4624: C18B8E24  lfs f12, -0x71dc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29148 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A4628: 3B1E01F8  addi r24, r30, 0x1f8
	ctx.r[24].s64 = ctx.r[30].s64 + 504;
	// 825A462C: 3ADE02F8  addi r22, r30, 0x2f8
	ctx.r[22].s64 = ctx.r[30].s64 + 760;
	// 825A4630: ED8B603C  fnmsubs f12, f11, f0, f12
	ctx.f[12].f64 = -(((ctx.f[11].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 825A4634: 39FE0C48  addi r15, r30, 0xc48
	ctx.r[15].s64 = ctx.r[30].s64 + 3144;
	// 825A4638: 833D001C  lwz r25, 0x1c(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A463C: 811D0020  lwz r8, 0x20(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 825A4640: 39DE0C48  addi r14, r30, 0xc48
	ctx.r[14].s64 = ctx.r[30].s64 + 3144;
	// 825A4644: 7E6ABC2E  lfsx f19, r10, r23
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 825A4648: 574A103A  slwi r10, r26, 2
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A464C: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A4650: 7E89C42E  lfsx f20, r9, r24
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[24].u32)) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 825A4654: 7E49B42E  lfsx f18, r9, r22
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[22].u32)) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 825A4658: 5729103A  slwi r9, r25, 2
	ctx.r[9].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825A465C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A4660: 80FD0028  lwz r7, 0x28(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A4664: 387E0178  addi r3, r30, 0x178
	ctx.r[3].s64 = ctx.r[30].s64 + 376;
	// 825A4668: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A466C: 7C0A7C2E  lfsx f0, r10, r15
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[15].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4670: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A4674: 3B9E0378  addi r28, r30, 0x378
	ctx.r[28].s64 = ctx.r[30].s64 + 888;
	// 825A4678: 80DD002C  lwz r6, 0x2c(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A467C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 825A4680: 7FE9742E  lfsx f31, r9, r14
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[14].u32)) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A4684: 54E82036  slwi r8, r7, 4
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825A4688: EEA90332  fmuls f21, f9, f12
	ctx.f[21].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A468C: 3B7E00C0  addi r27, r30, 0xc0
	ctx.r[27].s64 = ctx.r[30].s64 + 192;
	// 825A4690: 7D2B1C2E  lfsx f9, r11, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A4694: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825A4698: EF490332  fmuls f26, f9, f12
	ctx.f[26].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A469C: 7D474050  subf r10, r7, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 825A46A0: EFA80332  fmuls f29, f8, f12
	ctx.f[29].f64 = (((ctx.f[8].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A46A4: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 825A46A8: 7F6BE42E  lfsx f27, r11, r28
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 825A46AC: 7D0A3214  add r8, r10, r6
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 825A46B0: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A46B4: 7D2BDC2E  lfsx f9, r11, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A46B8: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A46BC: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A46C0: FCE05050  fneg f7, f10
	ctx.f[7].u64 = ctx.f[10].u64 ^ 0x8000_0000_0000_0000u64;
	// 825A46C4: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825A46C8: EE090332  fmuls f16, f9, f12
	ctx.f[16].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A46CC: 3ABE03F8  addi r21, r30, 0x3f8
	ctx.r[21].s64 = ctx.r[30].s64 + 1016;
	// 825A46D0: EDFB0372  fmuls f15, f27, f13
	ctx.f[15].f64 = (((ctx.f[27].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A46D4: 3A9E013C  addi r20, r30, 0x13c
	ctx.r[20].s64 = ctx.r[30].s64 + 316;
	// 825A46D8: 3A7E0600  addi r19, r30, 0x600
	ctx.r[19].s64 = ctx.r[30].s64 + 1536;
	// 825A46DC: 3A5E0808  addi r18, r30, 0x808
	ctx.r[18].s64 = ctx.r[30].s64 + 2056;
	// 825A46E0: EDDA0372  fmuls f14, f26, f13
	ctx.f[14].f64 = (((ctx.f[26].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A46E4: 3A3E0A28  addi r17, r30, 0xa28
	ctx.r[17].s64 = ctx.r[30].s64 + 2600;
	// 825A46E8: EE3D0372  fmuls f17, f29, f13
	ctx.f[17].f64 = (((ctx.f[29].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A46EC: 3A1E0084  addi r16, r30, 0x84
	ctx.r[16].s64 = ctx.r[30].s64 + 132;
	// 825A46F0: 7CCBAC2E  lfsx f6, r11, r21
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[21].u32)) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 825A46F4: 7CAAA42E  lfsx f5, r10, r20
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[20].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 825A46F8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 825A46FC: 7C8B9C2E  lfsx f4, r11, r19
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[19].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 825A4700: 7C69942E  lfsx f3, r9, r18
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[18].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 825A4704: 7C498C2E  lfsx f2, r9, r17
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[17].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825A4708: 7C2A842E  lfsx f1, r10, r16
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[16].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825A470C: 409A000C  bne cr6, 0x825a4718
	if !ctx.cr[6].eq {
	pc = 0x825A4718; continue 'dispatch;
	}
	// 825A4710: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A4714: 48000038  b 0x825a474c
	pc = 0x825A474C; continue 'dispatch;
	// 825A4718: 78AB0020  clrldi r11, r5, 0x20
	ctx.r[11].u64 = ctx.r[5].u64 & 0x00000000FFFFFFFFu64;
	// 825A471C: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4720: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 825A4724: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825A4728: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825A472C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 825A4730: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 825A4734: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A4738: C1810060  lfs f12, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A473C: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A4740: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 825A4744: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A4748: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A474C: 397F0068  addi r11, r31, 0x68
	ctx.r[11].s64 = ctx.r[31].s64 + 104;
	// 825A4750: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A4754: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A4758: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825A475C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825A4760: 409A0010  bne cr6, 0x825a4770
	if !ctx.cr[6].eq {
	pc = 0x825A4770; continue 'dispatch;
	}
	// 825A4764: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4768: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A476C: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4770: 3D7F0001  addis r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 65536;
	// 825A4774: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 825A4778: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A477C: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A4780: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825A4784: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825A4788: 409A0010  bne cr6, 0x825a4798
	if !ctx.cr[6].eq {
	pc = 0x825A4798; continue 'dispatch;
	}
	// 825A478C: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4790: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A4794: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4798: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A479C: C19E0000  lfs f12, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A47A0: 3CA00002  lis r5, 2
	ctx.r[5].s64 = 131072;
	// 825A47A4: C13E0004  lfs f9, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A47A8: 60C600F4  ori r6, r6, 0xf4
	ctx.r[6].u64 = ctx.r[6].u64 | 244;
	// 825A47AC: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825A47B0: 60A50100  ori r5, r5, 0x100
	ctx.r[5].u64 = ctx.r[5].u64 | 256;
	// 825A47B4: C11E0014  lfs f8, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A47B8: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 825A47BC: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A47C0: 6084010C  ori r4, r4, 0x10c
	ctx.r[4].u64 = ctx.r[4].u64 | 268;
	// 825A47C4: 7D9F352E  stfsx f12, r31, r6
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 825A47C8: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A47CC: 7D3F2D2E  stfsx f9, r31, r5
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 825A47D0: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 825A47D4: 60DA3794  ori r26, r6, 0x3794
	ctx.r[26].u64 = ctx.r[6].u64 | 14228;
	// 825A47D8: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A47DC: 7F3F252E  stfsx f25, r31, r4
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 825A47E0: 3F800002  lis r28, 2
	ctx.r[28].s64 = 131072;
	// 825A47E4: 60C537A0  ori r5, r6, 0x37a0
	ctx.r[5].u64 = ctx.r[6].u64 | 14240;
	// 825A47E8: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A47EC: 3F600002  lis r27, 2
	ctx.r[27].s64 = 131072;
	// 825A47F0: 60C400E8  ori r4, r6, 0xe8
	ctx.r[4].u64 = ctx.r[6].u64 | 232;
	// 825A47F4: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A47F8: 3D5F0002  addis r10, r31, 2
	ctx.r[10].s64 = ctx.r[31].s64 + 131072;
	// 825A47FC: 60D900EC  ori r25, r6, 0xec
	ctx.r[25].u64 = ctx.r[6].u64 | 236;
	// 825A4800: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A4804: 396B091C  addi r11, r11, 0x91c
	ctx.r[11].s64 = ctx.r[11].s64 + 2332;
	// 825A4808: 60D847BC  ori r24, r6, 0x47bc
	ctx.r[24].u64 = ctx.r[6].u64 | 18364;
	// 825A480C: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A4810: 3D3F0002  addis r9, r31, 2
	ctx.r[9].s64 = ctx.r[31].s64 + 131072;
	// 825A4814: 60D747C8  ori r23, r6, 0x47c8
	ctx.r[23].u64 = ctx.r[6].u64 | 18376;
	// 825A4818: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A481C: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4820: 60630B38  ori r3, r3, 0xb38
	ctx.r[3].u64 = ctx.r[3].u64 | 2872;
	// 825A4824: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4828: 60D647D4  ori r22, r6, 0x47d4
	ctx.r[22].u64 = ctx.r[6].u64 | 18388;
	// 825A482C: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A4830: 639C0B44  ori r28, r28, 0xb44
	ctx.r[28].u64 = ctx.r[28].u64 | 2884;
	// 825A4834: 60D55200  ori r21, r6, 0x5200
	ctx.r[21].u64 = ctx.r[6].u64 | 20992;
	// 825A4838: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A483C: 7D9F1D2E  stfsx f12, r31, r3
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 825A4840: 637B0B50  ori r27, r27, 0xb50
	ctx.r[27].u64 = ctx.r[27].u64 | 2896;
	// 825A4844: 60D4520C  ori r20, r6, 0x520c
	ctx.r[20].u64 = ctx.r[6].u64 | 21004;
	// 825A4848: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A484C: 7D1FE52E  stfsx f8, r31, r28
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 825A4850: 394A2B60  addi r10, r10, 0x2b60
	ctx.r[10].s64 = ctx.r[10].s64 + 11104;
	// 825A4854: 3D1F0002  addis r8, r31, 2
	ctx.r[8].s64 = ctx.r[31].s64 + 131072;
	// 825A4858: 39292F78  addi r9, r9, 0x2f78
	ctx.r[9].s64 = ctx.r[9].s64 + 12152;
	// 825A485C: 7EDFDD2E  stfsx f22, r31, r27
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32), tmp.u32) };
	// 825A4860: 60D35218  ori r19, r6, 0x5218
	ctx.r[19].u64 = ctx.r[6].u64 | 21016;
	// 825A4864: 3CFF0002  addis r7, r31, 2
	ctx.r[7].s64 = ctx.r[31].s64 + 131072;
	// 825A4868: 39084FE4  addi r8, r8, 0x4fe4
	ctx.r[8].s64 = ctx.r[8].s64 + 20452;
	// 825A486C: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4870: 3CDF0002  addis r6, r31, 2
	ctx.r[6].s64 = ctx.r[31].s64 + 131072;
	// 825A4874: D1AA0004  stfs f13, 4(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4878: 38E77228  addi r7, r7, 0x7228
	ctx.r[7].s64 = ctx.r[7].s64 + 29224;
	// 825A487C: D0090008  stfs f0, 8(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4880: 38C67640  addi r6, r6, 0x7640
	ctx.r[6].s64 = ctx.r[6].s64 + 30272;
	// 825A4884: D1A90004  stfs f13, 4(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4888: 7D9FC52E  stfsx f12, r31, r24
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[24].u32), tmp.u32) };
	// 825A488C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4890: 7EBFD52E  stfsx f21, r31, r26
	tmp.f32 = (ctx.f[21].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[26].u32), tmp.u32) };
	// 825A4894: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 825A4898: 7FBF2D2E  stfsx f29, r31, r5
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 825A489C: 616B7E5C  ori r11, r11, 0x7e5c
	ctx.r[11].u64 = ctx.r[11].u64 | 32348;
	// 825A48A0: 7F1F252E  stfsx f24, r31, r4
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 825A48A4: 614A7E68  ori r10, r10, 0x7e68
	ctx.r[10].u64 = ctx.r[10].u64 | 32360;
	// 825A48A8: 7EFFCD2E  stfsx f23, r31, r25
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[25].u32), tmp.u32) };
	// 825A48AC: 7D3FBD2E  stfsx f9, r31, r23
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[23].u32), tmp.u32) };
	// 825A48B0: 7E9FB52E  stfsx f20, r31, r22
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[22].u32), tmp.u32) };
	// 825A48B4: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A48B8: D1A80004  stfs f13, 4(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A48BC: 7D9FAD2E  stfsx f12, r31, r21
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[21].u32), tmp.u32) };
	// 825A48C0: FD80F850  fneg f12, f31
	ctx.f[12].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 825A48C4: 7D1FA52E  stfsx f8, r31, r20
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[20].u32), tmp.u32) };
	// 825A48C8: 7DFF9D2E  stfsx f15, r31, r19
	tmp.f32 = (ctx.f[15].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[19].u32), tmp.u32) };
	// 825A48CC: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A48D0: D1A70004  stfs f13, 4(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A48D4: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A48D8: D1A60004  stfs f13, 4(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A48DC: 7E1F5D2E  stfsx f16, r31, r11
	tmp.f32 = (ctx.f[16].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 825A48E0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A48E4: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825A48E8: 7F5F552E  stfsx f26, r31, r10
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 825A48EC: 61659EC8  ori r5, r11, 0x9ec8
	ctx.r[5].u64 = ctx.r[11].u64 | 40648;
	// 825A48F0: 612947B0  ori r9, r9, 0x47b0
	ctx.r[9].u64 = ctx.r[9].u64 | 18352;
	// 825A48F4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A48F8: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 825A48FC: 616A9ECC  ori r10, r11, 0x9ecc
	ctx.r[10].u64 = ctx.r[11].u64 | 40652;
	// 825A4900: 610847B4  ori r8, r8, 0x47b4
	ctx.r[8].u64 = ctx.r[8].u64 | 18356;
	// 825A4904: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4908: 7E7F4D2E  stfsx f19, r31, r9
	tmp.f32 = (ctx.f[19].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 825A490C: 3CE00002  lis r7, 2
	ctx.r[7].s64 = 131072;
	// 825A4910: 61699EE8  ori r9, r11, 0x9ee8
	ctx.r[9].u64 = ctx.r[11].u64 | 40680;
	// 825A4914: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4918: 7E5F452E  stfsx f18, r31, r8
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 825A491C: 3CC00002  lis r6, 2
	ctx.r[6].s64 = 131072;
	// 825A4920: 61689EEC  ori r8, r11, 0x9eec
	ctx.r[8].u64 = ctx.r[11].u64 | 40684;
	// 825A4924: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4928: 60E79EAC  ori r7, r7, 0x9eac
	ctx.r[7].u64 = ctx.r[7].u64 | 40620;
	// 825A492C: 6164DF20  ori r4, r11, 0xdf20
	ctx.r[4].u64 = ctx.r[11].u64 | 57120;
	// 825A4930: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4934: 60C69EC4  ori r6, r6, 0x9ec4
	ctx.r[6].u64 = ctx.r[6].u64 | 40644;
	// 825A4938: 6163EF68  ori r3, r11, 0xef68
	ctx.r[3].u64 = ctx.r[11].u64 | 61288;
	// 825A493C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4940: 617CEF80  ori r28, r11, 0xef80
	ctx.r[28].u64 = ctx.r[11].u64 | 61312;
	// 825A4944: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4948: 617BEF84  ori r27, r11, 0xef84
	ctx.r[27].u64 = ctx.r[11].u64 | 61316;
	// 825A494C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4950: 617AEF88  ori r26, r11, 0xef88
	ctx.r[26].u64 = ctx.r[11].u64 | 61320;
	// 825A4954: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4958: 6179EFA4  ori r25, r11, 0xefa4
	ctx.r[25].u64 = ctx.r[11].u64 | 61348;
	// 825A495C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4960: 6178EFA8  ori r24, r11, 0xefa8
	ctx.r[24].u64 = ctx.r[11].u64 | 61352;
	// 825A4964: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 825A4968: 61772FDC  ori r23, r11, 0x2fdc
	ctx.r[23].u64 = ctx.r[11].u64 | 12252;
	// 825A496C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825A4970: D3EB0008  stfs f31, 8(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4974: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4978: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825A497C: D3EB0008  stfs f31, 8(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4980: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4984: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A4988: 7C5F4D2E  stfsx f2, r31, r9
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 825A498C: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825A4990: 7C3F452E  stfsx f1, r31, r8
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 825A4994: 7CDF3D2E  stfsx f6, r31, r7
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 825A4998: 6129E080  ori r9, r9, 0xe080
	ctx.r[9].u64 = ctx.r[9].u64 | 57472;
	// 825A499C: 7C7F552E  stfsx f3, r31, r10
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 825A49A0: 7CBF352E  stfsx f5, r31, r6
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 825A49A4: 7C9F2D2E  stfsx f4, r31, r5
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 825A49A8: 7D7F252E  stfsx f11, r31, r4
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 825A49AC: D3EB0008  stfs f31, 8(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A49B0: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A49B4: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825A49B8: D3EB0008  stfs f31, 8(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A49BC: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A49C0: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 825A49C4: 7C3FC52E  stfsx f1, r31, r24
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[24].u32), tmp.u32) };
	// 825A49C8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 825A49CC: 616AE07C  ori r10, r11, 0xe07c
	ctx.r[10].u64 = ctx.r[11].u64 | 57468;
	// 825A49D0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825A49D4: 7CDF1D2E  stfsx f6, r31, r3
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 825A49D8: 7CBFE52E  stfsx f5, r31, r28
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 825A49DC: 7C9FDD2E  stfsx f4, r31, r27
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32), tmp.u32) };
	// 825A49E0: 7C7FD52E  stfsx f3, r31, r26
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[26].u32), tmp.u32) };
	// 825A49E4: 7C5FCD2E  stfsx f2, r31, r25
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[25].u32), tmp.u32) };
	// 825A49E8: 7D7FBD2E  stfsx f11, r31, r23
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[23].u32), tmp.u32) };
	// 825A49EC: D3EB0008  stfs f31, 8(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A49F0: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A49F4: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 825A49F8: 7D5F552E  stfsx f10, r31, r10
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 825A49FC: 6168911C  ori r8, r11, 0x911c
	ctx.r[8].u64 = ctx.r[11].u64 | 37148;
	// 825A4A00: 7D5F4D2E  stfsx f10, r31, r9
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 825A4A04: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 825A4A08: 61679120  ori r7, r11, 0x9120
	ctx.r[7].u64 = ctx.r[11].u64 | 37152;
	// 825A4A0C: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825A4A10: D3EB0008  stfs f31, 8(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4A14: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4A18: 7CFF452E  stfsx f7, r31, r8
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 825A4A1C: 7CFF3D2E  stfsx f7, r31, r7
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 825A4A20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A4A24: C01D0044  lfs f0, 0x44(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4A28: CBAB2410  lfd f29, 0x2410(r11)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(9232 as u32) ) };
	// 825A4A2C: FC400772  fmul f2, f0, f29
	ctx.f[2].f64 = ctx.f[0].f64 * ctx.f[29].f64;
	// 825A4A30: 4BF8ED41  bl 0x82533770
	ctx.lr = 0x825A4A34;
	sub_82533770(ctx, base);
	// 825A4A34: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 825A4A38: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4A3C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 825A4A40: 616B9F0C  ori r11, r11, 0x9f0c
	ctx.r[11].u64 = ctx.r[11].u64 | 40716;
	// 825A4A44: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4A48: EC000732  fmuls f0, f0, f28
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[28].f64) as f32) as f64);
	// 825A4A4C: 7C1F5D2E  stfsx f0, r31, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 825A4A50: C01D0044  lfs f0, 0x44(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4A54: FC400772  fmul f2, f0, f29
	ctx.f[2].f64 = ctx.f[0].f64 * ctx.f[29].f64;
	// 825A4A58: 4BF8ED19  bl 0x82533770
	ctx.lr = 0x825A4A5C;
	sub_82533770(ctx, base);
	// 825A4A5C: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 825A4A60: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4A64: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 825A4A68: 616BEFC8  ori r11, r11, 0xefc8
	ctx.r[11].u64 = ctx.r[11].u64 | 61384;
	// 825A4A6C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4A70: EC0006F2  fmuls f0, f0, f27
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[27].f64) as f32) as f64);
	// 825A4A74: 7C1F5D2E  stfsx f0, r31, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 825A4A78: C01D0044  lfs f0, 0x44(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4A7C: FC400772  fmul f2, f0, f29
	ctx.f[2].f64 = ctx.f[0].f64 * ctx.f[29].f64;
	// 825A4A80: 4BF8ECF1  bl 0x82533770
	ctx.lr = 0x825A4A84;
	sub_82533770(ctx, base);
	// 825A4A84: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 825A4A88: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 825A4A8C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 825A4A90: 616BA064  ori r11, r11, 0xa064
	ctx.r[11].u64 = ctx.r[11].u64 | 41060;
	// 825A4A94: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4A98: EC000472  fmuls f0, f0, f17
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[17].f64) as f32) as f64);
	// 825A4A9C: 7C1F5D2E  stfsx f0, r31, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 825A4AA0: C01D0044  lfs f0, 0x44(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4AA4: FC400772  fmul f2, f0, f29
	ctx.f[2].f64 = ctx.f[0].f64 * ctx.f[29].f64;
	// 825A4AA8: 4BF8ECC9  bl 0x82533770
	ctx.lr = 0x825A4AAC;
	sub_82533770(ctx, base);
	// 825A4AAC: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 825A4AB0: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 825A4AB4: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 825A4AB8: 616B5104  ori r11, r11, 0x5104
	ctx.r[11].u64 = ctx.r[11].u64 | 20740;
	// 825A4ABC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4AC0: EC0003B2  fmuls f0, f0, f14
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[14].f64) as f32) as f64);
	// 825A4AC4: 7C1F5D2E  stfsx f0, r31, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 825A4AC8: C01D0040  lfs f0, 0x40(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4ACC: FC400772  fmul f2, f0, f29
	ctx.f[2].f64 = ctx.f[0].f64 * ctx.f[29].f64;
	// 825A4AD0: 4BF8ECA1  bl 0x82533770
	ctx.lr = 0x825A4AD4;
	sub_82533770(ctx, base);
	// 825A4AD4: C01E0014  lfs f0, 0x14(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4AD8: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A4ADC: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4AE0: 3D5F0005  addis r10, r31, 5
	ctx.r[10].s64 = ctx.r[31].s64 + 327680;
	// 825A4AE4: 396B9130  addi r11, r11, -0x6ed0
	ctx.r[11].s64 = ctx.r[11].s64 + -28368;
	// 825A4AE8: FDA00818  frsp f13, f1
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A4AEC: D1BF0060  stfs f13, 0x60(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825A4AF0: 3D3F0005  addis r9, r31, 5
	ctx.r[9].s64 = ctx.r[31].s64 + 327680;
	// 825A4AF4: 394A9948  addi r10, r10, -0x66b8
	ctx.r[10].s64 = ctx.r[10].s64 + -26296;
	// 825A4AF8: 3929A160  addi r9, r9, -0x5ea0
	ctx.r[9].s64 = ctx.r[9].s64 + -24224;
	// 825A4AFC: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 825A4B00: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4B04: 3BDE00A8  addi r30, r30, 0xa8
	ctx.r[30].s64 = ctx.r[30].s64 + 168;
	// 825A4B08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B0C: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825A4B10: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4B14: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A4B18: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4B1C: D1AA0004  stfs f13, 4(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4B20: 396BA978  addi r11, r11, -0x5688
	ctx.r[11].s64 = ctx.r[11].s64 + -22152;
	// 825A4B24: D0090008  stfs f0, 8(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4B28: D1A90004  stfs f13, 4(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4B2C: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4B30: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4B34: C01D0034  lfs f0, 0x34(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4B38: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4B3C: 4BFFEE75  bl 0x825a39b0
	ctx.lr = 0x825A4B40;
	sub_825A39B0(ctx, base);
	// 825A4B40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B44: C01D0038  lfs f0, 0x38(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4B48: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4B4C: 4BFFEE65  bl 0x825a39b0
	ctx.lr = 0x825A4B50;
	sub_825A39B0(ctx, base);
	// 825A4B50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B54: C01D003C  lfs f0, 0x3c(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4B58: D01E000C  stfs f0, 0xc(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A4B5C: 4BFFEE55  bl 0x825a39b0
	ctx.lr = 0x825A4B60;
	sub_825A39B0(ctx, base);
	// 825A4B60: C01D0034  lfs f0, 0x34(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4B64: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 825A4B68: 3BDE00C8  addi r30, r30, 0xc8
	ctx.r[30].s64 = ctx.r[30].s64 + 200;
	// 825A4B6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B70: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4B74: 4BFFEE3D  bl 0x825a39b0
	ctx.lr = 0x825A4B78;
	sub_825A39B0(ctx, base);
	// 825A4B78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B7C: C01D0038  lfs f0, 0x38(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4B80: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4B84: 4BFFEE2D  bl 0x825a39b0
	ctx.lr = 0x825A4B88;
	sub_825A39B0(ctx, base);
	// 825A4B88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B8C: C01D003C  lfs f0, 0x3c(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4B90: D01E000C  stfs f0, 0xc(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A4B94: 4BFFEE1D  bl 0x825a39b0
	ctx.lr = 0x825A4B98;
	sub_825A39B0(ctx, base);
	// 825A4B98: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A4B9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4BA0: 409A000C  bne cr6, 0x825a4bac
	if !ctx.cr[6].eq {
	pc = 0x825A4BAC; continue 'dispatch;
	}
	// 825A4BA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A4BA8: 48000038  b 0x825a4be0
	pc = 0x825A4BE0; continue 'dispatch;
	// 825A4BAC: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825A4BB0: C01F0058  lfs f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4BB4: 39410078  addi r10, r1, 0x78
	ctx.r[10].s64 = ctx.r[1].s64 + 120;
	// 825A4BB8: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 825A4BBC: C9A10078  lfd f13, 0x78(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 825A4BC0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 825A4BC4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 825A4BC8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A4BCC: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4BD0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A4BD4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4BD8: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A4BDC: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825A4BE0: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A4BE4: 396BB190  addi r11, r11, -0x4e70
	ctx.r[11].s64 = ctx.r[11].s64 + -20080;
	// 825A4BE8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A4BEC: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A4BF0: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825A4BF4: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825A4BF8: 409A0010  bne cr6, 0x825a4c08
	if !ctx.cr[6].eq {
	pc = 0x825A4C08; continue 'dispatch;
	}
	// 825A4BFC: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4C00: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A4C04: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4C08: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A4C0C: 396BB5B0  addi r11, r11, -0x4a50
	ctx.r[11].s64 = ctx.r[11].s64 + -19024;
	// 825A4C10: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A4C14: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A4C18: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825A4C1C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825A4C20: 409A0010  bne cr6, 0x825a4c30
	if !ctx.cr[6].eq {
	pc = 0x825A4C30; continue 'dispatch;
	}
	// 825A4C24: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4C28: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A4C2C: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4C30: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 825A4C34: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 825A4C38: 4BF913C5  bl 0x82535ffc
	ctx.lr = 0x825A4C3C;
	sub_82535FFC(ctx, base);
	// 825A4C3C: 4BF90494  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A4C40 size=3020
    let mut pc: u32 = 0x825A4C40;
    'dispatch: loop {
        match pc {
            0x825A4C40 => {
    //   block [0x825A4C40..0x825A580C)
	// 825A4C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4C44: 4BF90455  bl 0x82535098
	ctx.lr = 0x825A4C48;
	sub_82535080(ctx, base);
	// 825A4C48: 3981FF98  addi r12, r1, -0x68
	ctx.r[12].s64 = ctx.r[1].s64 + -104;
	// 825A4C4C: 4BF91371  bl 0x82535fbc
	ctx.lr = 0x825A4C50;
	sub_82535FB0(ctx, base);
	// 825A4C50: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4C54: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4C58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4C5C: 3F208209  lis r25, -0x7df7
	ctx.r[25].s64 = -2113339392;
	// 825A4C60: 3FC08209  lis r30, -0x7df7
	ctx.r[30].s64 = -2113339392;
	// 825A4C64: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 825A4C68: C18B6E7C  lfs f12, 0x6e7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28284 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A4C6C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4C70: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825A4C74: 392B68EC  addi r9, r11, 0x68ec
	ctx.r[9].s64 = ctx.r[11].s64 + 26860;
	// 825A4C78: C1396E84  lfs f9, 0x6e84(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28292 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A4C7C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4C80: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4C84: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825A4C88: FD404890  fmr f10, f9
	ctx.f[10].f64 = ctx.f[9].f64;
	// 825A4C8C: 3F008209  lis r24, -0x7df7
	ctx.r[24].s64 = -2113339392;
	// 825A4C90: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A4C94: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A4C98: C1AB6E88  lfs f13, 0x6e88(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28296 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4C9C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 825A4CA0: D19F0038  stfs f12, 0x38(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A4CA4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A4CA8: C1786E8C  lfs f11, 0x6e8c(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28300 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A4CAC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A4CB0: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A4CB4: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 825A4CB8: CBA92008  lfd f29, 0x2008(r9)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8200 as u32) ) };
	// 825A4CBC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A4CC0: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825A4CC4: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 825A4CC8: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825A4CCC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 825A4CD0: D01F0048  stfs f0, 0x48(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825A4CD4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A4CD8: D13F004C  stfs f9, 0x4c(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825A4CDC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825A4CE0: CB892260  lfd f28, 0x2260(r9)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8800 as u32) ) };
	// 825A4CE4: 911F0028  stw r8, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 825A4CE8: D1BF0050  stfs f13, 0x50(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A4CEC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825A4CF0: D1BF0054  stfs f13, 0x54(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825A4CF4: 911F0030  stw r8, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 825A4CF8: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 825A4CFC: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 825A4D00: D17F0058  stfs f11, 0x58(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 825A4D04: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 825A4D08: D15F005C  stfs f10, 0x5c(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 825A4D0C: 4BF8EA65  bl 0x82533770
	ctx.lr = 0x825A4D10;
	sub_82533770(ctx, base);
	// 825A4D10: 397F0068  addi r11, r31, 0x68
	ctx.r[11].s64 = ctx.r[31].s64 + 104;
	// 825A4D14: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A4D18: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A4D1C: D19F0060  stfs f12, 0x60(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825A4D20: 3B800028  li r28, 0x28
	ctx.r[28].s64 = 40;
	// 825A4D24: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4D28: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A4D2C: C1B86E8C  lfs f13, 0x6e8c(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28300 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4D30: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 825A4D34: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A4D38: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4D3C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825A4D40: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A4D44: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 825A4D48: 938B0010  stw r28, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 825A4D4C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4D50: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4D54: 4BFFEDC5  bl 0x825a3b18
	ctx.lr = 0x825A4D58;
	sub_825A3B18(ctx, base);
	// 825A4D58: 3D7F0001  addis r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 65536;
	// 825A4D5C: C1BE6E80  lfs f13, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4D60: C0186E8C  lfs f0, 0x6e8c(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4D64: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 825A4D68: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A4D6C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A4D70: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825A4D74: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4D78: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825A4D7C: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A4D80: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 825A4D84: 938B0010  stw r28, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 825A4D88: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4D8C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4D90: 4BFFED89  bl 0x825a3b18
	ctx.lr = 0x825A4D94;
	sub_825A3B18(ctx, base);
	// 825A4D94: C1BF0058  lfs f13, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4D98: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 825A4D9C: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4DA0: 3F40820D  lis r26, -0x7df3
	ctx.r[26].s64 = -2113077248;
	// 825A4DA4: 386300A8  addi r3, r3, 0xa8
	ctx.r[3].s64 = ctx.r[3].s64 + 168;
	// 825A4DA8: C19ABFFC  lfs f12, -0x4004(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A4DAC: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A4DB0: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A4DB4: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4DB8: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4DBC: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A4DC0: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A4DC4: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A4DC8: 4BFFEBE9  bl 0x825a39b0
	ctx.lr = 0x825A4DCC;
	sub_825A39B0(ctx, base);
	// 825A4DCC: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 825A4DD0: C1BF0058  lfs f13, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4DD4: C19ABFFC  lfs f12, -0x4004(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A4DD8: 386300C8  addi r3, r3, 0xc8
	ctx.r[3].s64 = ctx.r[3].s64 + 200;
	// 825A4DDC: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4DE0: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A4DE4: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A4DE8: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4DEC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4DF0: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A4DF4: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A4DF8: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A4DFC: 4BFFEBB5  bl 0x825a39b0
	ctx.lr = 0x825A4E00;
	sub_825A39B0(ctx, base);
	// 825A4E00: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A4E04: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4E08: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825A4E0C: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 825A4E10: 3B8B6E90  addi r28, r11, 0x6e90
	ctx.r[28].s64 = ctx.r[11].s64 + 28304;
	// 825A4E14: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 825A4E18: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4E1C: 612900EC  ori r9, r9, 0xec
	ctx.r[9].u64 = ctx.r[9].u64 | 236;
	// 825A4E20: 616A00E8  ori r10, r11, 0xe8
	ctx.r[10].u64 = ctx.r[11].u64 | 232;
	// 825A4E24: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A4E28: 3A8001FD  li r20, 0x1fd
	ctx.r[20].s64 = 509;
	// 825A4E2C: C35C002C  lfs f26, 0x2c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 825A4E30: 396B00F0  addi r11, r11, 0xf0
	ctx.r[11].s64 = ctx.r[11].s64 + 240;
	// 825A4E34: C33C0030  lfs f25, 0x30(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 825A4E38: 3AA001FC  li r21, 0x1fc
	ctx.r[21].s64 = 508;
	// 825A4E3C: 7F3F4D2E  stfsx f25, r31, r9
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 825A4E40: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 825A4E44: 7F5F552E  stfsx f26, r31, r10
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 825A4E48: 386B0024  addi r3, r11, 0x24
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	// 825A4E4C: C3DC0000  lfs f30, 0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825A4E50: C31C0004  lfs f24, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 825A4E54: C2FC0008  lfs f23, 8(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 825A4E58: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 825A4E5C: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4E60: 92AB000C  stw r21, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[21].u32 ) };
	// 825A4E64: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4E68: 936B0018  stw r27, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 825A4E6C: D30B0010  stfs f24, 0x10(r11)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A4E70: D18B0014  stfs f12, 0x14(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A4E74: D2EB001C  stfs f23, 0x1c(r11)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A4E78: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A4E7C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4E80: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4E84: 4BFFECFD  bl 0x825a3b80
	ctx.lr = 0x825A4E88;
	sub_825A3B80(ctx, base);
	// 825A4E88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A4E8C: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4E90: 39400053  li r10, 0x53
	ctx.r[10].s64 = 83;
	// 825A4E94: C2BC000C  lfs f21, 0xc(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 825A4E98: C2CB2708  lfs f22, 0x2708(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9992 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 825A4E9C: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A4EA0: 396B091C  addi r11, r11, 0x91c
	ctx.r[11].s64 = ctx.r[11].s64 + 2332;
	// 825A4EA4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A4EA8: D2CB0004  stfs f22, 4(r11)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4EAC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A4EB0: D2AB0008  stfs f21, 8(r11)
	tmp.f32 = (ctx.f[21].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4EB4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A4EB8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4EBC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4EC0: 4BFFED29  bl 0x825a3be8
	ctx.lr = 0x825A4EC4;
	sub_825A3BE8(ctx, base);
	// 825A4EC4: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A4EC8: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4ECC: C29C0014  lfs f20, 0x14(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 825A4ED0: 392007F7  li r9, 0x7f7
	ctx.r[9].s64 = 2039;
	// 825A4ED4: 396B0B34  addi r11, r11, 0xb34
	ctx.r[11].s64 = ctx.r[11].s64 + 2868;
	// 825A4ED8: 390007F6  li r8, 0x7f6
	ctx.r[8].s64 = 2038;
	// 825A4EDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A4EE0: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4EE4: 3AC00258  li r22, 0x258
	ctx.r[22].s64 = 600;
	// 825A4EE8: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4EEC: 386B0024  addi r3, r11, 0x24
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	// 825A4EF0: D28B0010  stfs f20, 0x10(r11)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A4EF4: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A4EF8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A4EFC: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A4F00: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 825A4F04: C26A2954  lfs f19, 0x2954(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10580 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 825A4F08: D26B001C  stfs f19, 0x1c(r11)
	tmp.f32 = (ctx.f[19].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A4F0C: 92CB0018  stw r22, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[22].u32 ) };
	// 825A4F10: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4F14: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4F18: 4BFFED39  bl 0x825a3c50
	ctx.lr = 0x825A4F1C;
	sub_825A3C50(ctx, base);
	// 825A4F1C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A4F20: 394000D3  li r10, 0xd3
	ctx.r[10].s64 = 211;
	// 825A4F24: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4F28: C37ABFFC  lfs f27, -0x4004(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 825A4F2C: C3EB2048  lfs f31, 0x2048(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8264 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A4F30: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A4F34: 396B2B60  addi r11, r11, 0x2b60
	ctx.r[11].s64 = ctx.r[11].s64 + 11104;
	// 825A4F38: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A4F3C: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4F40: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A4F44: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4F48: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A4F4C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4F50: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4F54: 4BFFED65  bl 0x825a3cb8
	ctx.lr = 0x825A4F58;
	sub_825A3CB8(ctx, base);
	// 825A4F58: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A4F5C: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4F60: 39400137  li r10, 0x137
	ctx.r[10].s64 = 311;
	// 825A4F64: 396B2F78  addi r11, r11, 0x2f78
	ctx.r[11].s64 = ctx.r[11].s64 + 12152;
	// 825A4F68: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A4F6C: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4F70: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A4F74: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4F78: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A4F7C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4F80: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4F84: 4BFFEBFD  bl 0x825a3b80
	ctx.lr = 0x825A4F88;
	sub_825A3B80(ctx, base);
	// 825A4F88: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A4F8C: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4F90: 3AE003FC  li r23, 0x3fc
	ctx.r[23].s64 = 1020;
	// 825A4F94: C25C0024  lfs f18, 0x24(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 825A4F98: 396B3790  addi r11, r11, 0x3790
	ctx.r[11].s64 = ctx.r[11].s64 + 14224;
	// 825A4F9C: C23C0028  lfs f17, 0x28(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) };
	ctx.f[17].f64 = (tmp.f32 as f64);
	// 825A4FA0: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 825A4FA4: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A4FA8: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 825A4FAC: D24B0004  stfs f18, 4(r11)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4FB0: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 825A4FB4: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A4FB8: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 825A4FBC: D22B0010  stfs f17, 0x10(r11)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A4FC0: D18B0014  stfs f12, 0x14(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A4FC4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A4FC8: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A4FCC: 4BFFED55  bl 0x825a3d20
	ctx.lr = 0x825A4FD0;
	sub_825A3D20(ctx, base);
	// 825A4FD0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 825A4FD4: C1BE6E80  lfs f13, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4FD8: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825A4FDC: FD806890  fmr f12, f13
	ctx.f[12].f64 = ctx.f[13].f64;
	// 825A4FE0: 616A47B0  ori r10, r11, 0x47b0
	ctx.r[10].u64 = ctx.r[11].u64 | 18352;
	// 825A4FE4: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825A4FE8: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A4FEC: 612947B4  ori r9, r9, 0x47b4
	ctx.r[9].u64 = ctx.r[9].u64 | 18356;
	// 825A4FF0: 396B47B8  addi r11, r11, 0x47b8
	ctx.r[11].s64 = ctx.r[11].s64 + 18360;
	// 825A4FF4: 7F5F552E  stfsx f26, r31, r10
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 825A4FF8: 386B0024  addi r3, r11, 0x24
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	// 825A4FFC: 7F3F4D2E  stfsx f25, r31, r9
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 825A5000: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5004: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 825A5008: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A500C: 92AB000C  stw r21, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[21].u32 ) };
	// 825A5010: D30B0010  stfs f24, 0x10(r11)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5014: 936B0018  stw r27, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 825A5018: D18B0014  stfs f12, 0x14(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A501C: D2EB001C  stfs f23, 0x1c(r11)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A5020: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A5024: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5028: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A502C: 4BFFEB55  bl 0x825a3b80
	ctx.lr = 0x825A5030;
	sub_825A3B80(ctx, base);
	// 825A5030: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A5034: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5038: 39400061  li r10, 0x61
	ctx.r[10].s64 = 97;
	// 825A503C: 396B4FE4  addi r11, r11, 0x4fe4
	ctx.r[11].s64 = ctx.r[11].s64 + 20452;
	// 825A5040: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5044: D2CB0004  stfs f22, 4(r11)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5048: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A504C: D2AB0008  stfs f21, 8(r11)
	tmp.f32 = (ctx.f[21].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5050: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5054: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5058: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A505C: 4BFFEB8D  bl 0x825a3be8
	ctx.lr = 0x825A5060;
	sub_825A3BE8(ctx, base);
	// 825A5060: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A5064: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5068: 39400511  li r10, 0x511
	ctx.r[10].s64 = 1297;
	// 825A506C: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 825A5070: 396B51FC  addi r11, r11, 0x51fc
	ctx.r[11].s64 = ctx.r[11].s64 + 20988;
	// 825A5074: 39200510  li r9, 0x510
	ctx.r[9].s64 = 1296;
	// 825A5078: 386B0024  addi r3, r11, 0x24
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	// 825A507C: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5080: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A5084: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5088: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 825A508C: D28B0010  stfs f20, 0x10(r11)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5090: 92CB0018  stw r22, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[22].u32 ) };
	// 825A5094: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5098: D26B001C  stfs f19, 0x1c(r11)
	tmp.f32 = (ctx.f[19].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A509C: D1AB0020  stfs f13, 0x20(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A50A0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A50A4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A50A8: 4BFFEBA9  bl 0x825a3c50
	ctx.lr = 0x825A50AC;
	sub_825A3C50(ctx, base);
	// 825A50AC: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A50B0: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A50B4: 394000DF  li r10, 0xdf
	ctx.r[10].s64 = 223;
	// 825A50B8: 396B7228  addi r11, r11, 0x7228
	ctx.r[11].s64 = ctx.r[11].s64 + 29224;
	// 825A50BC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A50C0: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A50C4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A50C8: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A50CC: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A50D0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A50D4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A50D8: 4BFFEBE1  bl 0x825a3cb8
	ctx.lr = 0x825A50DC;
	sub_825A3CB8(ctx, base);
	// 825A50DC: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A50E0: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A50E4: 39400125  li r10, 0x125
	ctx.r[10].s64 = 293;
	// 825A50E8: 396B7640  addi r11, r11, 0x7640
	ctx.r[11].s64 = ctx.r[11].s64 + 30272;
	// 825A50EC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A50F0: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A50F4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A50F8: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A50FC: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5100: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5104: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5108: 4BFFEA79  bl 0x825a3b80
	ctx.lr = 0x825A510C;
	sub_825A3B80(ctx, base);
	// 825A510C: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 825A5110: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5114: 396B7E58  addi r11, r11, 0x7e58
	ctx.r[11].s64 = ctx.r[11].s64 + 32344;
	// 825A5118: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A511C: D24B0004  stfs f18, 4(r11)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5120: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 825A5124: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5128: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 825A512C: D22B0010  stfs f17, 0x10(r11)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5130: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5134: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5138: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A513C: 4BFFEBE5  bl 0x825a3d20
	ctx.lr = 0x825A5140;
	sub_825A3D20(ctx, base);
	// 825A5140: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5144: C01C004C  lfs f0, 0x4c(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5148: 39400199  li r10, 0x199
	ctx.r[10].s64 = 409;
	// 825A514C: C1BE6E80  lfs f13, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5150: C3CB7B7C  lfs f30, 0x7b7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31612 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825A5154: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A5158: 396B8E78  addi r11, r11, -0x7188
	ctx.r[11].s64 = ctx.r[11].s64 + -29064;
	// 825A515C: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5160: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A5164: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5168: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A516C: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5170: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825A5174: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5178: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A517C: 4BFFEA05  bl 0x825a3b80
	ctx.lr = 0x825A5180;
	sub_825A3B80(ctx, base);
	// 825A5180: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A5184: 39400101  li r10, 0x101
	ctx.r[10].s64 = 257;
	// 825A5188: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A518C: 396B9690  addi r11, r11, -0x6970
	ctx.r[11].s64 = ctx.r[11].s64 + -26992;
	// 825A5190: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5194: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5198: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A519C: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A51A0: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A51A4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A51A8: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A51AC: 4BFFE9D5  bl 0x825a3b80
	ctx.lr = 0x825A51B0;
	sub_825A3B80(ctx, base);
	// 825A51B0: 3D3F0003  addis r9, r31, 3
	ctx.r[9].s64 = ctx.r[31].s64 + 196608;
	// 825A51B4: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A51B8: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A51BC: C1BC0034  lfs f13, 0x34(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A51C0: 39299EA8  addi r9, r9, -0x6158
	ctx.r[9].s64 = ctx.r[9].s64 + -24920;
	// 825A51C4: C19C0038  lfs f12, 0x38(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A51C8: 3D5F0003  addis r10, r31, 3
	ctx.r[10].s64 = ctx.r[31].s64 + 196608;
	// 825A51CC: C17C003C  lfs f11, 0x3c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A51D0: 396B9EC0  addi r11, r11, -0x6140
	ctx.r[11].s64 = ctx.r[11].s64 + -24896;
	// 825A51D4: C15C0040  lfs f10, 0x40(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A51D8: 394A9EE0  addi r10, r10, -0x6120
	ctx.r[10].s64 = ctx.r[10].s64 + -24864;
	// 825A51DC: C13C0044  lfs f9, 0x44(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A51E0: C11C0048  lfs f8, 0x48(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A51E4: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 825A51E8: D1A90004  stfs f13, 4(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A51EC: 93690000  stw r27, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A51F0: D0090008  stfs f0, 8(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A51F4: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 825A51F8: D009000C  stfs f0, 0xc(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A51FC: D0090010  stfs f0, 0x10(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5200: D0090014  stfs f0, 0x14(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5204: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A5208: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A520C: D16B0008  stfs f11, 8(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5210: D14B000C  stfs f10, 0xc(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5214: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5218: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A521C: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A5220: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A5224: 936A0000  stw r27, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A5228: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A522C: D12A0008  stfs f9, 8(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5230: D10A000C  stfs f8, 0xc(r10)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5234: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5238: D00A0014  stfs f0, 0x14(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A523C: D00A0018  stfs f0, 0x18(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A5240: 4BF8E531  bl 0x82533770
	ctx.lr = 0x825A5244;
	sub_82533770(ctx, base);
	// 825A5244: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A5248: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A524C: 39400D68  li r10, 0xd68
	ctx.r[10].s64 = 3432;
	// 825A5250: C01C0054  lfs f0, 0x54(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5254: 396B9EFC  addi r11, r11, -0x6104
	ctx.r[11].s64 = ctx.r[11].s64 + -24836;
	// 825A5258: C1996E84  lfs f12, 0x6e84(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28292 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A525C: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A5260: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5264: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A5268: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A526C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A5270: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5274: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5278: D1AB0010  stfs f13, 0x10(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A527C: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5280: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5284: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5288: 4BFFEB01  bl 0x825a3d88
	ctx.lr = 0x825A528C;
	sub_825A3D88(ctx, base);
	// 825A528C: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A5290: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5294: 3D5F0003  addis r10, r31, 3
	ctx.r[10].s64 = ctx.r[31].s64 + 196608;
	// 825A5298: FDA0A090  fmr f13, f20
	ctx.f[13].f64 = ctx.f[20].f64;
	// 825A529C: 396BDF1C  addi r11, r11, -0x20e4
	ctx.r[11].s64 = ctx.r[11].s64 + -8420;
	// 825A52A0: 394ADF34  addi r10, r10, -0x20cc
	ctx.r[10].s64 = ctx.r[10].s64 + -8396;
	// 825A52A4: 3920017F  li r9, 0x17f
	ctx.r[9].s64 = 383;
	// 825A52A8: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 825A52AC: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A52B0: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A52B4: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A52B8: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A52BC: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A52C0: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A52C4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A52C8: C19C004C  lfs f12, 0x4c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(76 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A52CC: D3CA0004  stfs f30, 4(r10)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A52D0: D18A0008  stfs f12, 8(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A52D4: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A52D8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A52DC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A52E0: 4BFFE8A1  bl 0x825a3b80
	ctx.lr = 0x825A52E4;
	sub_825A3B80(ctx, base);
	// 825A52E4: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A52E8: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A52EC: 394000E9  li r10, 0xe9
	ctx.r[10].s64 = 233;
	// 825A52F0: FDA0D890  fmr f13, f27
	ctx.f[13].f64 = ctx.f[27].f64;
	// 825A52F4: 396BE74C  addi r11, r11, -0x18b4
	ctx.r[11].s64 = ctx.r[11].s64 + -6324;
	// 825A52F8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A52FC: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5300: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A5304: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5308: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A530C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5310: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5314: 4BFFE86D  bl 0x825a3b80
	ctx.lr = 0x825A5318;
	sub_825A3B80(ctx, base);
	// 825A5318: 3D5F0003  addis r10, r31, 3
	ctx.r[10].s64 = ctx.r[31].s64 + 196608;
	// 825A531C: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5320: C1BC0034  lfs f13, 0x34(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5324: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 825A5328: 394AEF64  addi r10, r10, -0x109c
	ctx.r[10].s64 = ctx.r[10].s64 + -4252;
	// 825A532C: C17C0040  lfs f11, 0x40(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A5330: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A5334: C15C0044  lfs f10, 0x44(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A5338: C13C0048  lfs f9, 0x48(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A533C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 825A5340: 396BEF7C  addi r11, r11, -0x1084
	ctx.r[11].s64 = ctx.r[11].s64 + -4228;
	// 825A5344: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 825A5348: D1AA0004  stfs f13, 4(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A534C: 936A0000  stw r27, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A5350: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5354: D18A0010  stfs f12, 0x10(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5358: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A535C: D00A0014  stfs f0, 0x14(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5360: 3D5F0003  addis r10, r31, 3
	ctx.r[10].s64 = ctx.r[31].s64 + 196608;
	// 825A5364: C1BC0038  lfs f13, 0x38(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5368: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A536C: 394AEF9C  addi r10, r10, -0x1064
	ctx.r[10].s64 = ctx.r[10].s64 + -4196;
	// 825A5370: C19C003C  lfs f12, 0x3c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A5374: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5378: D18B0008  stfs f12, 8(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A537C: D16B000C  stfs f11, 0xc(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5380: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5384: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5388: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A538C: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A5390: 936A0000  stw r27, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A5394: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5398: D14A0008  stfs f10, 8(r10)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A539C: D12A000C  stfs f9, 0xc(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A53A0: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A53A4: D00A0014  stfs f0, 0x14(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A53A8: D00A0018  stfs f0, 0x18(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A53AC: 4BF8E3C5  bl 0x82533770
	ctx.lr = 0x825A53B0;
	sub_82533770(ctx, base);
	// 825A53B0: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A53B4: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A53B8: 39400EEC  li r10, 0xeec
	ctx.r[10].s64 = 3820;
	// 825A53BC: C19C0054  lfs f12, 0x54(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A53C0: 396BEFB8  addi r11, r11, -0x1048
	ctx.r[11].s64 = ctx.r[11].s64 + -4168;
	// 825A53C4: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A53C8: C1B96E84  lfs f13, 0x6e84(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A53CC: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A53D0: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A53D4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A53D8: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A53DC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A53E0: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A53E4: ED8B0332  fmuls f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A53E8: D18B0010  stfs f12, 0x10(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A53EC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A53F0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A53F4: 4BFFE995  bl 0x825a3d88
	ctx.lr = 0x825A53F8;
	sub_825A3D88(ctx, base);
	// 825A53F8: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A53FC: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5400: FDA0A090  fmr f13, f20
	ctx.f[13].f64 = ctx.f[20].f64;
	// 825A5404: 396B2FD8  addi r11, r11, 0x2fd8
	ctx.r[11].s64 = ctx.r[11].s64 + 12248;
	// 825A5408: C37C004C  lfs f27, 0x4c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(76 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 825A540C: 394005E7  li r10, 0x5e7
	ctx.r[10].s64 = 1511;
	// 825A5410: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5414: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A5418: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A541C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5420: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5424: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5428: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A542C: 396B2FF0  addi r11, r11, 0x2ff0
	ctx.r[11].s64 = ctx.r[11].s64 + 12272;
	// 825A5430: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5434: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5438: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A543C: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5440: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5444: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5448: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A544C: 4BFFE805  bl 0x825a3c50
	ctx.lr = 0x825A5450;
	sub_825A3C50(ctx, base);
	// 825A5450: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A5454: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5458: 39400425  li r10, 0x425
	ctx.r[10].s64 = 1061;
	// 825A545C: 396B5008  addi r11, r11, 0x5008
	ctx.r[11].s64 = ctx.r[11].s64 + 20488;
	// 825A5460: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 825A5464: D36B0004  stfs f27, 4(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5468: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A546C: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5470: D3CB000C  stfs f30, 0xc(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5474: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5478: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A547C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5480: 4BFFE7D1  bl 0x825a3c50
	ctx.lr = 0x825A5484;
	sub_825A3C50(ctx, base);
	// 825A5484: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 825A5488: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A548C: 39400355  li r10, 0x355
	ctx.r[10].s64 = 853;
	// 825A5490: 396B7024  addi r11, r11, 0x7024
	ctx.r[11].s64 = ctx.r[11].s64 + 28708;
	// 825A5494: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5498: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A549C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A54A0: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A54A4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A54A8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A54AC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A54B0: 4BFFE7A1  bl 0x825a3c50
	ctx.lr = 0x825A54B4;
	sub_825A3C50(ctx, base);
	// 825A54B4: 3D7F0004  addis r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 262144;
	// 825A54B8: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A54BC: 3940021D  li r10, 0x21d
	ctx.r[10].s64 = 541;
	// 825A54C0: C1BABFFC  lfs f13, -0x4004(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A54C4: 396B903C  addi r11, r11, -0x6fc4
	ctx.r[11].s64 = ctx.r[11].s64 + -28612;
	// 825A54C8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A54CC: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A54D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A54D4: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A54D8: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A54DC: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A54E0: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A54E4: 4BFFE83D  bl 0x825a3d20
	ctx.lr = 0x825A54E8;
	sub_825A3D20(ctx, base);
	// 825A54E8: FC40E890  fmr f2, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[29].f64;
	// 825A54EC: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 825A54F0: 4BF8E281  bl 0x82533770
	ctx.lr = 0x825A54F4;
	sub_82533770(ctx, base);
	// 825A54F4: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A54F8: 3D7F0004  addis r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 262144;
	// 825A54FC: C01C0054  lfs f0, 0x54(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5500: 394005E6  li r10, 0x5e6
	ctx.r[10].s64 = 1510;
	// 825A5504: 396BA054  addi r11, r11, -0x5fac
	ctx.r[11].s64 = ctx.r[11].s64 + -24492;
	// 825A5508: C1B96E84  lfs f13, 0x6e84(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A550C: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A5510: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5514: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A5518: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A551C: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5520: FD600090  fmr f11, f0
	ctx.f[11].f64 = ctx.f[0].f64;
	// 825A5524: D18B0010  stfs f12, 0x10(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5528: D16B0008  stfs f11, 8(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A552C: FD400090  fmr f10, f0
	ctx.f[10].f64 = ctx.f[0].f64;
	// 825A5530: D14B0014  stfs f10, 0x14(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5534: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A5538: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A553C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5540: 4BFFE849  bl 0x825a3d88
	ctx.lr = 0x825A5544;
	sub_825A3D88(ctx, base);
	// 825A5544: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825A5548: 3D7F0004  addis r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 262144;
	// 825A554C: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5550: 39200679  li r9, 0x679
	ctx.r[9].s64 = 1657;
	// 825A5554: 396BE074  addi r11, r11, -0x1f8c
	ctx.r[11].s64 = ctx.r[11].s64 + -8076;
	// 825A5558: C1AA8E28  lfs f13, -0x71d8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A555C: 3D5F0004  addis r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 262144;
	// 825A5560: 394AE090  addi r10, r10, -0x1f70
	ctx.r[10].s64 = ctx.r[10].s64 + -8048;
	// 825A5564: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5568: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A556C: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 825A5570: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5574: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5578: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A557C: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5580: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A5584: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A5588: D3CA0004  stfs f30, 4(r10)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A558C: D36A0008  stfs f27, 8(r10)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5590: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5594: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5598: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A559C: 4BFFE6B5  bl 0x825a3c50
	ctx.lr = 0x825A55A0;
	sub_825A3C50(ctx, base);
	// 825A55A0: 3D7F0004  addis r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 262144;
	// 825A55A4: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A55A8: 3940044F  li r10, 0x44f
	ctx.r[10].s64 = 1103;
	// 825A55AC: 396B00A8  addi r11, r11, 0xa8
	ctx.r[11].s64 = ctx.r[11].s64 + 168;
	// 825A55B0: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 825A55B4: D36B0004  stfs f27, 4(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A55B8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A55BC: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A55C0: D3CB000C  stfs f30, 0xc(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A55C4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A55C8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A55CC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A55D0: 4BFFE681  bl 0x825a3c50
	ctx.lr = 0x825A55D4;
	sub_825A3C50(ctx, base);
	// 825A55D4: 3D7F0004  addis r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 262144;
	// 825A55D8: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A55DC: 39400377  li r10, 0x377
	ctx.r[10].s64 = 887;
	// 825A55E0: 396B20C4  addi r11, r11, 0x20c4
	ctx.r[11].s64 = ctx.r[11].s64 + 8388;
	// 825A55E4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A55E8: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A55EC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A55F0: D36B0008  stfs f27, 8(r11)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A55F4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A55F8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A55FC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5600: 4BFFE651  bl 0x825a3c50
	ctx.lr = 0x825A5604;
	sub_825A3C50(ctx, base);
	// 825A5604: 3D7F0004  addis r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 262144;
	// 825A5608: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A560C: 394001EB  li r10, 0x1eb
	ctx.r[10].s64 = 491;
	// 825A5610: C1BABFFC  lfs f13, -0x4004(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5614: 396B40DC  addi r11, r11, 0x40dc
	ctx.r[11].s64 = ctx.r[11].s64 + 16604;
	// 825A5618: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A561C: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5620: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A5624: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5628: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A562C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5630: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5634: 4BFFE6ED  bl 0x825a3d20
	ctx.lr = 0x825A5638;
	sub_825A3D20(ctx, base);
	// 825A5638: FC40E890  fmr f2, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[29].f64;
	// 825A563C: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 825A5640: 4BF8E131  bl 0x82533770
	ctx.lr = 0x825A5644;
	sub_82533770(ctx, base);
	// 825A5644: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A5648: 3D7F0004  addis r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 262144;
	// 825A564C: 3940059E  li r10, 0x59e
	ctx.r[10].s64 = 1438;
	// 825A5650: C19C0054  lfs f12, 0x54(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A5654: 396B50F4  addi r11, r11, 0x50f4
	ctx.r[11].s64 = ctx.r[11].s64 + 20724;
	// 825A5658: C1B96E84  lfs f13, 0x6e84(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A565C: C01E6E80  lfs f0, 0x6e80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5660: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A5664: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5668: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A566C: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5670: ED8B0332  fmuls f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A5674: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5678: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A567C: D18B0010  stfs f12, 0x10(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5680: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5684: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5688: 4BFFE701  bl 0x825a3d88
	ctx.lr = 0x825A568C;
	sub_825A3D88(ctx, base);
	// 825A568C: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A5690: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5694: 3D5F0005  addis r10, r31, 5
	ctx.r[10].s64 = ctx.r[31].s64 + 327680;
	// 825A5698: C1BC0078  lfs f13, 0x78(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A569C: 396B9114  addi r11, r11, -0x6eec
	ctx.r[11].s64 = ctx.r[11].s64 + -28396;
	// 825A56A0: C3DC007C  lfs f30, 0x7c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(124 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825A56A4: 394A9130  addi r10, r10, -0x6ed0
	ctx.r[10].s64 = ctx.r[10].s64 + -28368;
	// 825A56A8: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 825A56AC: 39000083  li r8, 0x83
	ctx.r[8].s64 = 131;
	// 825A56B0: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 825A56B4: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A56B8: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A56BC: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A56C0: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A56C4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A56C8: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A56CC: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A56D0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825A56D4: C3E97B78  lfs f31, 0x7b78(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(31608 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A56D8: D3EA0004  stfs f31, 4(r10)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A56DC: D3CA0008  stfs f30, 8(r10)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A56E0: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A56E4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A56E8: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A56EC: 4BFFE495  bl 0x825a3b80
	ctx.lr = 0x825A56F0;
	sub_825A3B80(ctx, base);
	// 825A56F0: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A56F4: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A56F8: 39400071  li r10, 0x71
	ctx.r[10].s64 = 113;
	// 825A56FC: 396B9948  addi r11, r11, -0x66b8
	ctx.r[11].s64 = ctx.r[11].s64 + -26296;
	// 825A5700: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5704: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5708: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A570C: D3CB0008  stfs f30, 8(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5710: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5714: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5718: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A571C: 4BFFE465  bl 0x825a3b80
	ctx.lr = 0x825A5720;
	sub_825A3B80(ctx, base);
	// 825A5720: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A5724: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5728: 3940006B  li r10, 0x6b
	ctx.r[10].s64 = 107;
	// 825A572C: 396BA160  addi r11, r11, -0x5ea0
	ctx.r[11].s64 = ctx.r[11].s64 + -24224;
	// 825A5730: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5734: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5738: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A573C: D3CB0008  stfs f30, 8(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5740: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5744: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5748: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A574C: 4BFFE435  bl 0x825a3b80
	ctx.lr = 0x825A5750;
	sub_825A3B80(ctx, base);
	// 825A5750: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A5754: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5758: 3940007F  li r10, 0x7f
	ctx.r[10].s64 = 127;
	// 825A575C: 396BA978  addi r11, r11, -0x5688
	ctx.r[11].s64 = ctx.r[11].s64 + -22152;
	// 825A5760: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825A5764: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A5768: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A576C: D3CB0008  stfs f30, 8(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A5770: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5774: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A5778: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A577C: 4BFFE405  bl 0x825a3b80
	ctx.lr = 0x825A5780;
	sub_825A3B80(ctx, base);
	// 825A5780: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A5784: C01E6E80  lfs f0, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5788: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 825A578C: C1B86E8C  lfs f13, 0x6e8c(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28300 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5790: 396BB190  addi r11, r11, -0x4e70
	ctx.r[11].s64 = ctx.r[11].s64 + -20080;
	// 825A5794: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A5798: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A579C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825A57A0: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A57A4: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 825A57A8: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A57AC: 938B0010  stw r28, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 825A57B0: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A57B4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A57B8: 4BFFE501  bl 0x825a3cb8
	ctx.lr = 0x825A57BC;
	sub_825A3CB8(ctx, base);
	// 825A57BC: 3D7F0005  addis r11, r31, 5
	ctx.r[11].s64 = ctx.r[31].s64 + 327680;
	// 825A57C0: C1BE6E80  lfs f13, 0x6e80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28288 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A57C4: 396BB5B0  addi r11, r11, -0x4a50
	ctx.r[11].s64 = ctx.r[11].s64 + -19024;
	// 825A57C8: C0186E8C  lfs f0, 0x6e8c(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A57CC: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A57D0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A57D4: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825A57D8: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A57DC: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825A57E0: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A57E4: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 825A57E8: 938B0010  stw r28, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 825A57EC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A57F0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A57F4: 4BFFE4C5  bl 0x825a3cb8
	ctx.lr = 0x825A57F8;
	sub_825A3CB8(ctx, base);
	// 825A57F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A57FC: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 825A5800: 3981FF98  addi r12, r1, -0x68
	ctx.r[12].s64 = ctx.r[1].s64 + -104;
	// 825A5804: 4BF90805  bl 0x82536008
	ctx.lr = 0x825A5808;
	sub_82535FFC(ctx, base);
	// 825A5808: 4BF8F8E0  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A5810 size=724
    let mut pc: u32 = 0x825A5810;
    'dispatch: loop {
        match pc {
            0x825A5810 => {
    //   block [0x825A5810..0x825A5AE4)
	// 825A5810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A581C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5820: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 825A5824: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825A5828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A582C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5830: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825A5834: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A5838: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825A583C: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825A5840: 816B6E78  lwz r11, 0x6e78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28280 as u32) ) } as u64;
	// 825A5844: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825A5848: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A584C: 814B6E70  lwz r10, 0x6e70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28272 as u32) ) } as u64;
	// 825A5850: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A5854: 816B6E70  lwz r11, 0x6e70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28272 as u32) ) } as u64;
	// 825A5858: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825A585C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5860: 814B6E74  lwz r10, 0x6e74(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28276 as u32) ) } as u64;
	// 825A5864: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825A5868: 816B6E74  lwz r11, 0x6e74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28276 as u32) ) } as u64;
	// 825A586C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A5870: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5874: C00B6E88  lfs f0, 0x6e88(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5878: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A587C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5880: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 825A5884: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825A5888: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A588C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A5890: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A5894: C3EB20B0  lfs f31, 0x20b0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8368 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A5898: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A589C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A58A0: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A58A4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A58A8: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 825A58AC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 825A58B0: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 825A58B4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825A58B8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A58BC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A58C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A58C4: C3CB1850  lfs f30, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825A58C8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 825A58CC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A58D0: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A58D4: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A58D8: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A58DC: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 825A58E0: 4198008C  blt cr6, 0x825a596c
	if ctx.cr[6].lt {
	pc = 0x825A596C; continue 'dispatch;
	}
	// 825A58E4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825A58E8: C03E0010  lfs f1, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825A58EC: 4BF8D86D  bl 0x82533158
	ctx.lr = 0x825A58F0;
	sub_82533158(ctx, base);
	// 825A58F0: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A58F4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A58F8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 825A58FC: C80B7B88  lfd f0, 0x7b88(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(31624 as u32) ) };
	// 825A5900: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 825A5904: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825A5908: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A590C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A5910: 2F0BFFF8  cmpwi cr6, r11, -8
	ctx.cr[6].compare_i32(ctx.r[11].s32, -8, &mut ctx.xer);
	// 825A5914: 40980020  bge cr6, 0x825a5934
	if !ctx.cr[6].lt {
	pc = 0x825A5934; continue 'dispatch;
	}
	// 825A5918: 3960FFF8  li r11, -8
	ctx.r[11].s64 = -8;
	// 825A591C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825A5920: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A5924: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5928: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A592C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A5930: 48000098  b 0x825a59c8
	pc = 0x825A59C8; continue 'dispatch;
	// 825A5934: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5938: 4098001C  bge cr6, 0x825a5954
	if !ctx.cr[6].lt {
	pc = 0x825A5954; continue 'dispatch;
	}
	// 825A593C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825A5940: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A5944: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5948: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A594C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A5950: 48000078  b 0x825a59c8
	pc = 0x825A59C8; continue 'dispatch;
	// 825A5954: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 825A5958: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A595C: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5960: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5964: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825A5968: 48000060  b 0x825a59c8
	pc = 0x825A59C8; continue 'dispatch;
	// 825A596C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A5970: C03E0010  lfs f1, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825A5974: 4BF8D7E5  bl 0x82533158
	ctx.lr = 0x825A5978;
	sub_82533158(ctx, base);
	// 825A5978: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825A597C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5980: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 825A5984: C80B7B80  lfd f0, 0x7b80(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(31616 as u32) ) };
	// 825A5988: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 825A598C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825A5990: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A5994: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A5998: 2F0BFFF8  cmpwi cr6, r11, -8
	ctx.cr[6].compare_i32(ctx.r[11].s32, -8, &mut ctx.xer);
	// 825A599C: 40980010  bge cr6, 0x825a59ac
	if !ctx.cr[6].lt {
	pc = 0x825A59AC; continue 'dispatch;
	}
	// 825A59A0: 3960FFF8  li r11, -8
	ctx.r[11].s64 = -8;
	// 825A59A4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825A59A8: 48000018  b 0x825a59c0
	pc = 0x825A59C0; continue 'dispatch;
	// 825A59AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A59B0: 4098000C  bge cr6, 0x825a59bc
	if !ctx.cr[6].lt {
	pc = 0x825A59BC; continue 'dispatch;
	}
	// 825A59B4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825A59B8: 48000008  b 0x825a59c0
	pc = 0x825A59C0; continue 'dispatch;
	// 825A59BC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 825A59C0: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825A59C4: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A59C8: D01F0048  stfs f0, 0x48(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825A59CC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A59D0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 825A59D4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825A59D8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A59DC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A59E0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A59E4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A59E8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A59EC: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825A59F0: 814B6E68  lwz r10, 0x6e68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28264 as u32) ) } as u64;
	// 825A59F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A59F8: C01E0018  lfs f0, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A59FC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 825A5A00: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A5A04: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 825A5A08: C18B204C  lfs f12, 0x204c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A5A0C: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A5A10: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 825A5A14: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825A5A18: 41980008  blt cr6, 0x825a5a20
	if ctx.cr[6].lt {
	pc = 0x825A5A20; continue 'dispatch;
	}
	// 825A5A1C: EC0DF028  fsubs f0, f13, f30
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[30].f64) as f32) as f64);
	// 825A5A20: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 825A5A24: 41990008  bgt cr6, 0x825a5a2c
	if ctx.cr[6].gt {
	pc = 0x825A5A2C; continue 'dispatch;
	}
	// 825A5A28: FC00F090  fmr f0, f30
	ctx.f[0].f64 = ctx.f[30].f64;
	// 825A5A2C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A5A30: 7C00FFAE  stfiwx f0, 0, r31
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32, tmp.u32) };
	// 825A5A34: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A5A38: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 825A5A3C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825A5A40: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A5A44: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A5A48: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5A4C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A5A50: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A5A54: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825A5A58: 816B6E6C  lwz r11, 0x6e6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28268 as u32) ) } as u64;
	// 825A5A5C: C01E0020  lfs f0, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5A60: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A5A64: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825A5A68: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A5A6C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 825A5A70: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 825A5A74: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825A5A78: 41980008  blt cr6, 0x825a5a80
	if ctx.cr[6].lt {
	pc = 0x825A5A80; continue 'dispatch;
	}
	// 825A5A7C: EC0DF028  fsubs f0, f13, f30
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[30].f64) as f32) as f64);
	// 825A5A80: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825A5A84: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A5A88: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825A5A8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5A90: C1BE0024  lfs f13, 0x24(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5A94: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 825A5A98: C00B9070  lfs f0, -0x6f90(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28560 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5A9C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A5AA0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A5AA4: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 825A5AA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A5AAC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A5AB0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 825A5AB4: C01E0028  lfs f0, 0x28(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5AB8: D01F004C  stfs f0, 0x4c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825A5ABC: C01E002C  lfs f0, 0x2c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5AC0: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A5AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A5AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5AD0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825A5AD4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825A5AD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A5ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A5AE8 size=344
    let mut pc: u32 = 0x825A5AE8;
    'dispatch: loop {
        match pc {
            0x825A5AE8 => {
    //   block [0x825A5AE8..0x825A5B54)
	// 825A5AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5AF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5AF4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5AF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5AFC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825A5B00: 2B04000C  cmplwi cr6, r4, 0xc
	ctx.cr[6].compare_u32(ctx.r[4].u32, 12 as u32, &mut ctx.xer);
	// 825A5B04: 419900F0  bgt cr6, 0x825a5bf4
	if ctx.cr[6].gt {
	pc = 0x825A5BF4; continue 'dispatch;
	}
	// 825A5B08: 3D80825A  lis r12, -0x7da6
	ctx.r[12].s64 = -2108030976;
	// 825A5B0C: 398C5B20  addi r12, r12, 0x5b20
	ctx.r[12].s64 = ctx.r[12].s64 + 23328;
	// 825A5B10: 5480103A  slwi r0, r4, 2
	ctx.r[0].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 825A5B14: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 825A5B18: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 825A5B1C: 4E800420  bctr
	match ctx.r[4].u64 {
		0 => {
	pc = 0x825A5B54; continue 'dispatch;
		},
		1 => {
	pc = 0x825A5B68; continue 'dispatch;
		},
		2 => {
	pc = 0x825A5B74; continue 'dispatch;
		},
		3 => {
	pc = 0x825A5B80; continue 'dispatch;
		},
		4 => {
	pc = 0x825A5B8C; continue 'dispatch;
		},
		5 => {
	pc = 0x825A5B98; continue 'dispatch;
		},
		6 => {
	pc = 0x825A5BA4; continue 'dispatch;
		},
		7 => {
	pc = 0x825A5BB0; continue 'dispatch;
		},
		8 => {
	pc = 0x825A5BBC; continue 'dispatch;
		},
		9 => {
	pc = 0x825A5BC8; continue 'dispatch;
		},
		10 => {
	pc = 0x825A5BD4; continue 'dispatch;
		},
		11 => {
	pc = 0x825A5BE0; continue 'dispatch;
		},
		12 => {
	pc = 0x825A5BEC; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 825A5B20: 825A5B54  lwz r18, 0x5b54(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23380 as u32) ) } as u64;
	// 825A5B24: 825A5B68  lwz r18, 0x5b68(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23400 as u32) ) } as u64;
	// 825A5B28: 825A5B74  lwz r18, 0x5b74(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23412 as u32) ) } as u64;
	// 825A5B2C: 825A5B80  lwz r18, 0x5b80(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23424 as u32) ) } as u64;
	// 825A5B30: 825A5B8C  lwz r18, 0x5b8c(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23436 as u32) ) } as u64;
	// 825A5B34: 825A5B98  lwz r18, 0x5b98(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23448 as u32) ) } as u64;
	// 825A5B38: 825A5BA4  lwz r18, 0x5ba4(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23460 as u32) ) } as u64;
	// 825A5B3C: 825A5BB0  lwz r18, 0x5bb0(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23472 as u32) ) } as u64;
	// 825A5B40: 825A5BBC  lwz r18, 0x5bbc(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23484 as u32) ) } as u64;
	// 825A5B44: 825A5BC8  lwz r18, 0x5bc8(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23496 as u32) ) } as u64;
	// 825A5B48: 825A5BD4  lwz r18, 0x5bd4(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23508 as u32) ) } as u64;
	// 825A5B4C: 825A5BE0  lwz r18, 0x5be0(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23520 as u32) ) } as u64;
	// 825A5B50: 825A5BEC  lwz r18, 0x5bec(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(23532 as u32) ) } as u64;
            }
            0x825A5B54 => {
    //   block [0x825A5B54..0x825A5B68)
	// 825A5B54: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A5B58: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 825A5B5C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A5B60: 4BF8EFF1  bl 0x82534b50
	ctx.lr = 0x825A5B64;
	sub_82534B50(ctx, base);
	// 825A5B64: 48000090  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5B68 => {
    //   block [0x825A5B68..0x825A5B74)
	// 825A5B68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5B6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A5B70: 48000084  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5B74 => {
    //   block [0x825A5B74..0x825A5B80)
	// 825A5B74: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5B78: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A5B7C: 48000078  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5B80 => {
    //   block [0x825A5B80..0x825A5B8C)
	// 825A5B80: C00B0008  lfs f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5B84: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5B88: 4800006C  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5B8C => {
    //   block [0x825A5B8C..0x825A5B98)
	// 825A5B8C: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5B90: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5B94: 48000060  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5B98 => {
    //   block [0x825A5B98..0x825A5BA4)
	// 825A5B98: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5B9C: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5BA0: 48000054  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5BA4 => {
    //   block [0x825A5BA4..0x825A5BB0)
	// 825A5BA4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A5BA8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A5BAC: 48000048  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5BB0 => {
    //   block [0x825A5BB0..0x825A5BBC)
	// 825A5BB0: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5BB4: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A5BB8: 4800003C  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5BBC => {
    //   block [0x825A5BBC..0x825A5BC8)
	// 825A5BBC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A5BC0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A5BC4: 48000030  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5BC8 => {
    //   block [0x825A5BC8..0x825A5BD4)
	// 825A5BC8: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5BCC: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A5BD0: 48000024  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5BD4 => {
    //   block [0x825A5BD4..0x825A5BE0)
	// 825A5BD4: C00B0024  lfs f0, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5BD8: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A5BDC: 48000018  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5BE0 => {
    //   block [0x825A5BE0..0x825A5BEC)
	// 825A5BE0: C00B0028  lfs f0, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5BE4: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A5BE8: 4800000C  b 0x825a5bf4
	pc = 0x825A5BF4; continue 'dispatch;
            }
            0x825A5BEC => {
    //   block [0x825A5BEC..0x825A5C40)
	// 825A5BEC: C00B002C  lfs f0, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5BF0: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A5BF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A5BF8: 4BFFDD39  bl 0x825a3930
	ctx.lr = 0x825A5BFC;
	sub_825A3930(ctx, base);
	// 825A5BFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5C00: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A5C04: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 825A5C08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5C0C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5C10: C02B0058  lfs f1, 0x58(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825A5C14: 4BFFFBFD  bl 0x825a5810
	ctx.lr = 0x825A5C18;
	sub_825A5810(ctx, base);
	// 825A5C18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5C1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A5C20: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5C24: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5C28: 4BFFE1E9  bl 0x825a3e10
	ctx.lr = 0x825A5C2C;
	sub_825A3E10(ctx, base);
	// 825A5C2C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825A5C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5C38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A5C40 size=216
    let mut pc: u32 = 0x825A5C40;
    'dispatch: loop {
        match pc {
            0x825A5C40 => {
    //   block [0x825A5C40..0x825A5D18)
	// 825A5C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A5C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5C54: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A5C58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5C5C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A5C60: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825A5C64: 419A0018  beq cr6, 0x825a5c7c
	if ctx.cr[6].eq {
	pc = 0x825A5C7C; continue 'dispatch;
	}
	// 825A5C68: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5C6C: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 825A5C70: 396B7B90  addi r11, r11, 0x7b90
	ctx.r[11].s64 = ctx.r[11].s64 + 31632;
	// 825A5C74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A5C78: 4BFFEFC9  bl 0x825a4c40
	ctx.lr = 0x825A5C7C;
	sub_825A4C40(ctx, base);
	// 825A5C7C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A5C80: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5C84: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 825A5C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5C8C: 394B68D4  addi r10, r11, 0x68d4
	ctx.r[10].s64 = ctx.r[11].s64 + 26836;
	// 825A5C90: 3960D8F0  li r11, -0x2710
	ctx.r[11].s64 = -10000;
	// 825A5C94: C1A91FF8  lfs f13, 0x1ff8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A5C98: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 825A5C9C: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5CA0: C1891850  lfs f12, 0x1850(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A5CA4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A5CA8: 7D48F92E  stwx r10, r8, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 825A5CAC: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A5CB0: D19F0010  stfs f12, 0x10(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A5CB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A5CB8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825A5CBC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A5CC0: C169BFFC  lfs f11, -0x4004(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A5CC4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A5CC8: D17F0014  stfs f11, 0x14(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A5CCC: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A5CD0: C149209C  lfs f10, 0x209c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8348 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A5CD4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A5CD8: D15F001C  stfs f10, 0x1c(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A5CDC: C12924D0  lfs f9, 0x24d0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9424 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A5CE0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A5CE4: D13F0024  stfs f9, 0x24(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A5CE8: C0092094  lfs f0, 0x2094(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A5CEC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825A5CF0: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A5CF4: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A5CF8: C1092338  lfs f8, 0x2338(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9016 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A5CFC: D11F0030  stfs f8, 0x30(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A5D00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A5D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5D0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A5D10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5D18 size=16
    let mut pc: u32 = 0x825A5D18;
    'dispatch: loop {
        match pc {
            0x825A5D18 => {
    //   block [0x825A5D18..0x825A5D28)
	// 825A5D18: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 825A5D1C: 4199000C  bgt cr6, 0x825a5d28
	if ctx.cr[6].gt {
		sub_825A5D28(ctx, base);
		return;
	}
	// 825A5D20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A5D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5D28 size=16
    let mut pc: u32 = 0x825A5D28;
    'dispatch: loop {
        match pc {
            0x825A5D28 => {
    //   block [0x825A5D28..0x825A5D38)
	// 825A5D28: 2B0326F5  cmplwi cr6, r3, 0x26f5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 9973 as u32, &mut ctx.xer);
	// 825A5D2C: 4198000C  blt cr6, 0x825a5d38
	if ctx.cr[6].lt {
		sub_825A5D38(ctx, base);
		return;
	}
	// 825A5D30: 386026F5  li r3, 0x26f5
	ctx.r[3].s64 = 9973;
	// 825A5D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5D38 size=60
    let mut pc: u32 = 0x825A5D38;
    'dispatch: loop {
        match pc {
            0x825A5D38 => {
    //   block [0x825A5D38..0x825A5D74)
	// 825A5D38: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 825A5D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825A5D40: 390004CD  li r8, 0x4cd
	ctx.r[8].s64 = 1229;
	// 825A5D44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A5D48: 38CA7B98  addi r6, r10, 0x7b98
	ctx.r[6].s64 = ctx.r[10].s64 + 31640;
	// 825A5D4C: 7D484A14  add r10, r8, r9
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 825A5D50: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 825A5D54: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A5D58: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A5D5C: 7D4A302E  lwzx r10, r10, r6
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 825A5D60: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 825A5D64: 419A001C  beq cr6, 0x825a5d80
	if ctx.cr[6].eq {
		sub_825A5D74(ctx, base);
		return;
	}
	// 825A5D68: 4099000C  ble cr6, 0x825a5d74
	if !ctx.cr[6].gt {
		sub_825A5D74(ctx, base);
		return;
	}
	// 825A5D6C: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 825A5D70: 48000008  b 0x825a5d78
	sub_825A5D74(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5D74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5D74 size=24
    let mut pc: u32 = 0x825A5D74;
    'dispatch: loop {
        match pc {
            0x825A5D74 => {
    //   block [0x825A5D74..0x825A5D8C)
	// 825A5D74: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 825A5D78: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 825A5D7C: 409AFFD0  bne cr6, 0x825a5d4c
	if !ctx.cr[6].eq {
		sub_825A5D38(ctx, base);
		return;
	}
	// 825A5D80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A5D84: 7C6B302E  lwzx r3, r11, r6
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 825A5D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5D90 size=20
    let mut pc: u32 = 0x825A5D90;
    'dispatch: loop {
        match pc {
            0x825A5D90 => {
    //   block [0x825A5D90..0x825A5DA4)
	// 825A5D90: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A5D94: 814B3964  lwz r10, 0x3964(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14692 as u32) ) } as u64;
	// 825A5D98: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A5D9C: 914B3964  stw r10, 0x3964(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(14692 as u32), ctx.r[10].u32 ) };
	// 825A5DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5DA8 size=20
    let mut pc: u32 = 0x825A5DA8;
    'dispatch: loop {
        match pc {
            0x825A5DA8 => {
    //   block [0x825A5DA8..0x825A5DBC)
	// 825A5DA8: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A5DAC: 814B3964  lwz r10, 0x3964(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14692 as u32) ) } as u64;
	// 825A5DB0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825A5DB4: 914B3964  stw r10, 0x3964(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(14692 as u32), ctx.r[10].u32 ) };
	// 825A5DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5DC0 size=160
    let mut pc: u32 = 0x825A5DC0;
    'dispatch: loop {
        match pc {
            0x825A5DC0 => {
    //   block [0x825A5DC0..0x825A5E60)
	// 825A5DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5DC4: 4BF8F2F1  bl 0x825350b4
	ctx.lr = 0x825A5DC8;
	sub_82535080(ctx, base);
	// 825A5DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5DCC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A5DD0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A5DD4: 3BCB3968  addi r30, r11, 0x3968
	ctx.r[30].s64 = ctx.r[11].s64 + 14696;
	// 825A5DD8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A5DDC: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5DE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A5DE4: 409A000C  bne cr6, 0x825a5df0
	if !ctx.cr[6].eq {
	pc = 0x825A5DF0; continue 'dispatch;
	}
	// 825A5DE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 825A5DEC: 4800006C  b 0x825a5e58
	pc = 0x825A5E58; continue 'dispatch;
	// 825A5DF0: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5DF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A5DF8: 419A004C  beq cr6, 0x825a5e44
	if ctx.cr[6].eq {
	pc = 0x825A5E44; continue 'dispatch;
	}
	// 825A5DFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5E00: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A5E04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5E08: 40990030  ble cr6, 0x825a5e38
	if !ctx.cr[6].gt {
	pc = 0x825A5E38; continue 'dispatch;
	}
	// 825A5E0C: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 825A5E10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A5E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5E18: 4BF8D379  bl 0x82533190
	ctx.lr = 0x825A5E1C;
	sub_82533190(ctx, base);
	// 825A5E1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825A5E20: 4182001C  beq 0x825a5e3c
	if ctx.cr[0].eq {
	pc = 0x825A5E3C; continue 'dispatch;
	}
	// 825A5E24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5E28: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 825A5E2C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 825A5E30: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825A5E34: 4198FFD8  blt cr6, 0x825a5e0c
	if ctx.cr[6].lt {
	pc = 0x825A5E0C; continue 'dispatch;
	}
	// 825A5E38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A5E3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A5E40: 409A000C  bne cr6, 0x825a5e4c
	if !ctx.cr[6].eq {
	pc = 0x825A5E4C; continue 'dispatch;
	}
	// 825A5E44: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 825A5E48: 48000010  b 0x825a5e58
	pc = 0x825A5E58; continue 'dispatch;
	// 825A5E4C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A5E50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A5E54: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A5E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A5E5C: 4BF8F2A8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5E60 size=232
    let mut pc: u32 = 0x825A5E60;
    'dispatch: loop {
        match pc {
            0x825A5E60 => {
    //   block [0x825A5E60..0x825A5F48)
	// 825A5E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5E64: 4BF8F24D  bl 0x825350b0
	ctx.lr = 0x825A5E68;
	sub_82535080(ctx, base);
	// 825A5E68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5E6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5E70: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825A5E74: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825A5E78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A5E7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5E80: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A5E84: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 825A5E88: 4BEC76A9  bl 0x8246d530
	ctx.lr = 0x825A5E8C;
	sub_8246D530(ctx, base);
	// 825A5E8C: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 825A5E90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5E94: 409A00AC  bne cr6, 0x825a5f40
	if !ctx.cr[6].eq {
	pc = 0x825A5F40; continue 'dispatch;
	}
	// 825A5E98: 7C65DB78  or r5, r3, r27
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[27].u64;
	// 825A5E9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5EA0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A5EA4: 4BEC755D  bl 0x8246d400
	ctx.lr = 0x825A5EA8;
	sub_8246D400(ctx, base);
	// 825A5EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5EAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A5EB0: 4BEC0C59  bl 0x82466b08
	ctx.lr = 0x825A5EB4;
	sub_82466B08(ctx, base);
	// 825A5EB4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A5EB8: 40990050  ble cr6, 0x825a5f08
	if !ctx.cr[6].gt {
	pc = 0x825A5F08; continue 'dispatch;
	}
	// 825A5EBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A5EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5EC4: 4BEC0C4D  bl 0x82466b10
	ctx.lr = 0x825A5EC8;
	sub_82466B10(ctx, base);
	// 825A5EC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A5ECC: 4BEC5315  bl 0x8246b1e0
	ctx.lr = 0x825A5ED0;
	sub_8246B1E0(ctx, base);
	// 825A5ED0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A5ED4: 419A0014  beq cr6, 0x825a5ee8
	if ctx.cr[6].eq {
	pc = 0x825A5EE8; continue 'dispatch;
	}
	// 825A5ED8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825A5EDC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 825A5EE0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A5EE4: 4BFFFF7D  bl 0x825a5e60
	ctx.lr = 0x825A5EE8;
	sub_825A5E60(ctx, base);
	// 825A5EE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5EEC: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 825A5EF0: 4E800421  bctrl
	ctx.lr = 0x825A5EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A5EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5EF8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A5EFC: 4BEC0C0D  bl 0x82466b08
	ctx.lr = 0x825A5F00;
	sub_82466B08(ctx, base);
	// 825A5F00: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 825A5F04: 4198FFB8  blt cr6, 0x825a5ebc
	if ctx.cr[6].lt {
	pc = 0x825A5EBC; continue 'dispatch;
	}
	// 825A5F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5F0C: 4BEC095D  bl 0x82466868
	ctx.lr = 0x825A5F10;
	sub_82466868(ctx, base);
	// 825A5F10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A5F14: 419A002C  beq cr6, 0x825a5f40
	if ctx.cr[6].eq {
	pc = 0x825A5F40; continue 'dispatch;
	}
	// 825A5F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5F1C: 4BEC094D  bl 0x82466868
	ctx.lr = 0x825A5F20;
	sub_82466868(ctx, base);
	// 825A5F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A5F28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A5F2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5F30: 4BEC7601  bl 0x8246d530
	ctx.lr = 0x825A5F34;
	sub_8246D530(ctx, base);
	// 825A5F34: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 825A5F38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5F3C: 419AFF5C  beq cr6, 0x825a5e98
	if ctx.cr[6].eq {
	pc = 0x825A5E98; continue 'dispatch;
	}
	// 825A5F40: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A5F44: 4BF8F1BC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5F48 size=12
    let mut pc: u32 = 0x825A5F48;
    'dispatch: loop {
        match pc {
            0x825A5F48 => {
    //   block [0x825A5F48..0x825A5F54)
	// 825A5F48: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A5F4C: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 825A5F50: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5F54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5F54 size=12
    let mut pc: u32 = 0x825A5F54;
    'dispatch: loop {
        match pc {
            0x825A5F54 => {
    //   block [0x825A5F54..0x825A5F60)
	// 825A5F54: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 825A5F58: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 825A5F5C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A5F60 size=20
    let mut pc: u32 = 0x825A5F60;
    'dispatch: loop {
        match pc {
            0x825A5F60 => {
    //   block [0x825A5F60..0x825A5F74)
	// 825A5F60: 3960001D  li r11, 0x1d
	ctx.r[11].s64 = 29;
	// 825A5F64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A5F68: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A5F6C: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 825A5F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5F78 size=344
    let mut pc: u32 = 0x825A5F78;
    'dispatch: loop {
        match pc {
            0x825A5F78 => {
    //   block [0x825A5F78..0x825A60D0)
	// 825A5F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5F7C: 4BF8F129  bl 0x825350a4
	ctx.lr = 0x825A5F80;
	sub_82535080(ctx, base);
	// 825A5F80: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5F84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A5F88: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825A5F8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A5F90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A5F94: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825A5F98: 4BEC7599  bl 0x8246d530
	ctx.lr = 0x825A5F9C;
	sub_8246D530(ctx, base);
	// 825A5F9C: 546B07FE  clrlwi r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 825A5FA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5FA4: 409A0124  bne cr6, 0x825a60c8
	if !ctx.cr[6].eq {
	pc = 0x825A60C8; continue 'dispatch;
	}
	// 825A5FA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5FAC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 825A5FB0: 3AEB73C0  addi r23, r11, 0x73c0
	ctx.r[23].s64 = ctx.r[11].s64 + 29632;
	// 825A5FB4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A5FB8: 3B0B2F64  addi r24, r11, 0x2f64
	ctx.r[24].s64 = ctx.r[11].s64 + 12132;
	// 825A5FBC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825A5FC0: 3B6B3A78  addi r27, r11, 0x3a78
	ctx.r[27].s64 = ctx.r[11].s64 + 14968;
	// 825A5FC4: 60650001  ori r5, r3, 1
	ctx.r[5].u64 = ctx.r[3].u64 | 1;
	// 825A5FC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A5FCC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825A5FD0: 4BEC7431  bl 0x8246d400
	ctx.lr = 0x825A5FD4;
	sub_8246D400(ctx, base);
	// 825A5FD4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825A5FD8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825A5FDC: 4BEC0B4D  bl 0x82466b28
	ctx.lr = 0x825A5FE0;
	sub_82466B28(ctx, base);
	// 825A5FE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A5FE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A5FE8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 825A5FEC: A38B0012  lhz r28, 0x12(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 825A5FF0: 4BEC0879  bl 0x82466868
	ctx.lr = 0x825A5FF4;
	sub_82466868(ctx, base);
	// 825A5FF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A5FF8: 419A0020  beq cr6, 0x825a6018
	if ctx.cr[6].eq {
	pc = 0x825A6018; continue 'dispatch;
	}
	// 825A5FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6000: 7F3FE12E  stwx r25, r31, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32), ctx.r[25].u32) };
	// 825A6004: 4BEC0865  bl 0x82466868
	ctx.lr = 0x825A6008;
	sub_82466868(ctx, base);
	// 825A6008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A600C: 4BEC085D  bl 0x82466868
	ctx.lr = 0x825A6010;
	sub_82466868(ctx, base);
	// 825A6010: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A6014: 409AFFE8  bne cr6, 0x825a5ffc
	if !ctx.cr[6].eq {
	pc = 0x825A5FFC; continue 'dispatch;
	}
	// 825A6018: 7D7FE0AE  lbzx r11, r31, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825A601C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6020: 419A0020  beq cr6, 0x825a6040
	if ctx.cr[6].eq {
	pc = 0x825A6040; continue 'dispatch;
	}
	// 825A6024: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 825A6028: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825A602C: 4BEC0AFD  bl 0x82466b28
	ctx.lr = 0x825A6030;
	sub_82466B28(ctx, base);
	// 825A6030: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 825A6034: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 825A6038: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A603C: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 825A6040: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6044: 7F3FE12E  stwx r25, r31, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32), ctx.r[25].u32) };
	// 825A6048: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 825A604C: 4BEC0ABD  bl 0x82466b08
	ctx.lr = 0x825A6050;
	sub_82466B08(ctx, base);
	// 825A6050: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A6054: 4099003C  ble cr6, 0x825a6090
	if !ctx.cr[6].gt {
	pc = 0x825A6090; continue 'dispatch;
	}
	// 825A6058: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A605C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6060: 4BEC0AB1  bl 0x82466b10
	ctx.lr = 0x825A6064;
	sub_82466B10(ctx, base);
	// 825A6064: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A606C: 419A0010  beq cr6, 0x825a607c
	if ctx.cr[6].eq {
	pc = 0x825A607C; continue 'dispatch;
	}
	// 825A6070: 4BEC5169  bl 0x8246b1d8
	ctx.lr = 0x825A6074;
	sub_8246B1D8(ctx, base);
	// 825A6074: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825A6078: 4BFFFF01  bl 0x825a5f78
	ctx.lr = 0x825A607C;
	sub_825A5F78(ctx, base);
	// 825A607C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6080: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 825A6084: 4BEC0A85  bl 0x82466b08
	ctx.lr = 0x825A6088;
	sub_82466B08(ctx, base);
	// 825A6088: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 825A608C: 4198FFCC  blt cr6, 0x825a6058
	if ctx.cr[6].lt {
	pc = 0x825A6058; continue 'dispatch;
	}
	// 825A6090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6094: 4BEC07D5  bl 0x82466868
	ctx.lr = 0x825A6098;
	sub_82466868(ctx, base);
	// 825A6098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A609C: 419A002C  beq cr6, 0x825a60c8
	if ctx.cr[6].eq {
	pc = 0x825A60C8; continue 'dispatch;
	}
	// 825A60A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A60A4: 4BEC07C5  bl 0x82466868
	ctx.lr = 0x825A60A8;
	sub_82466868(ctx, base);
	// 825A60A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A60AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A60B0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825A60B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A60B8: 4BEC7479  bl 0x8246d530
	ctx.lr = 0x825A60BC;
	sub_8246D530(ctx, base);
	// 825A60BC: 546B07FE  clrlwi r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 825A60C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A60C4: 419AFF00  beq cr6, 0x825a5fc4
	if ctx.cr[6].eq {
	pc = 0x825A5FC4; continue 'dispatch;
	}
	// 825A60C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A60CC: 4BF8F028  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A60D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A60D0 size=176
    let mut pc: u32 = 0x825A60D0;
    'dispatch: loop {
        match pc {
            0x825A60D0 => {
    //   block [0x825A60D0..0x825A6180)
	// 825A60D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A60D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A60D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A60DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A60E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A60E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A60E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A60EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A60F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A60F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A60F8: 4BEC7439  bl 0x8246d530
	ctx.lr = 0x825A60FC;
	sub_8246D530(ctx, base);
	// 825A60FC: 546B077A  rlwinm r11, r3, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 825A6100: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6104: 409A0064  bne cr6, 0x825a6168
	if !ctx.cr[6].eq {
	pc = 0x825A6168; continue 'dispatch;
	}
	// 825A6108: 60650004  ori r5, r3, 4
	ctx.r[5].u64 = ctx.r[3].u64 | 4;
	// 825A610C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A6110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6114: 4BEC72ED  bl 0x8246d400
	ctx.lr = 0x825A6118;
	sub_8246D400(ctx, base);
	// 825A6118: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825A611C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6120: 3BCBBEB4  addi r30, r11, -0x414c
	ctx.r[30].s64 = ctx.r[11].s64 + -16716;
	// 825A6124: 4BEC0735  bl 0x82466858
	ctx.lr = 0x825A6128;
	sub_82466858(ctx, base);
	// 825A6128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A612C: 4BEC3EA5  bl 0x82469fd0
	ctx.lr = 0x825A6130;
	sub_82469FD0(ctx, base);
	// 825A6130: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A6134: 409A0034  bne cr6, 0x825a6168
	if !ctx.cr[6].eq {
	pc = 0x825A6168; continue 'dispatch;
	}
	// 825A6138: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 825A613C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6140: 4BEC09D1  bl 0x82466b10
	ctx.lr = 0x825A6144;
	sub_82466B10(ctx, base);
	// 825A6144: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825A6148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A614C: 396B8DC4  addi r11, r11, -0x723c
	ctx.r[11].s64 = ctx.r[11].s64 + -29244;
	// 825A6150: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825A6154: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825A6158: 4BEC0941  bl 0x82466a98
	ctx.lr = 0x825A615C;
	sub_82466A98(ctx, base);
	// 825A615C: A1430012  lhz r10, 0x12(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 825A6160: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 825A6164: 7D6AF9AE  stbx r11, r10, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 825A6168: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6174: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A617C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6180 size=60
    let mut pc: u32 = 0x825A6180;
    'dispatch: loop {
        match pc {
            0x825A6180 => {
    //   block [0x825A6180..0x825A61BC)
	// 825A6180: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A6184: 2B0B0013  cmplwi cr6, r11, 0x13
	ctx.cr[6].compare_u32(ctx.r[11].u32, 19 as u32, &mut ctx.xer);
	// 825A6188: 409A0020  bne cr6, 0x825a61a8
	if !ctx.cr[6].eq {
	pc = 0x825A61A8; continue 'dispatch;
	}
	// 825A618C: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 825A6190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A6194: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A6198: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 825A619C: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A61A0: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 825A61A4: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 825A61A8: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A61AC: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 825A61B0: 419A000C  beq cr6, 0x825a61bc
	if ctx.cr[6].eq {
		sub_825A61BC(ctx, base);
		return;
	}
	// 825A61B4: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 825A61B8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A61BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A61BC size=88
    let mut pc: u32 = 0x825A61BC;
    'dispatch: loop {
        match pc {
            0x825A61BC => {
    //   block [0x825A61BC..0x825A6214)
	// 825A61BC: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A61C0: 556B0738  rlwinm r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A61C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A61C8: 419A0018  beq cr6, 0x825a61e0
	if ctx.cr[6].eq {
	pc = 0x825A61E0; continue 'dispatch;
	}
	// 825A61CC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 825A61D0: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 825A61D4: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A61D8: 716BFFF7  andi. r11, r11, 0xfff7
	ctx.r[11].u64 = ctx.r[11].u64 & 65527;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A61DC: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 825A61E0: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A61E4: 556B06F6  rlwinm r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A61E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A61EC: 419A0018  beq cr6, 0x825a6204
	if ctx.cr[6].eq {
	pc = 0x825A6204; continue 'dispatch;
	}
	// 825A61F0: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 825A61F4: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 825A61F8: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A61FC: 716BFFEF  andi. r11, r11, 0xffef
	ctx.r[11].u64 = ctx.r[11].u64 & 65519;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6200: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 825A6204: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A6208: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A620C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6210: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6214(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6214 size=24
    let mut pc: u32 = 0x825A6214;
    'dispatch: loop {
        match pc {
            0x825A6214 => {
    //   block [0x825A6214..0x825A622C)
	// 825A6214: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 825A6218: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 825A621C: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A6220: 716BFFDF  andi. r11, r11, 0xffdf
	ctx.r[11].u64 = ctx.r[11].u64 & 65503;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6224: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 825A6228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6230 size=124
    let mut pc: u32 = 0x825A6230;
    'dispatch: loop {
        match pc {
            0x825A6230 => {
    //   block [0x825A6230..0x825A62AC)
	// 825A6230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6234: 4BF8EE89  bl 0x825350bc
	ctx.lr = 0x825A6238;
	sub_82535080(ctx, base);
	// 825A6238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A623C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A6240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6244: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6248: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 825A624C: 409A0018  bne cr6, 0x825a6264
	if !ctx.cr[6].eq {
	pc = 0x825A6264; continue 'dispatch;
	}
	// 825A6250: 4BFFFD29  bl 0x825a5f78
	ctx.lr = 0x825A6254;
	sub_825A5F78(ctx, base);
	// 825A6254: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A6258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A625C: 4BFFFE75  bl 0x825a60d0
	ctx.lr = 0x825A6260;
	sub_825A60D0(ctx, base);
	// 825A6260: 4800000C  b 0x825a626c
	pc = 0x825A626C; continue 'dispatch;
	// 825A6264: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 825A6268: 4098001C  bge cr6, 0x825a6284
	if !ctx.cr[6].lt {
	pc = 0x825A6284; continue 'dispatch;
	}
	// 825A626C: 3D60825A  lis r11, -0x7da6
	ctx.r[11].s64 = -2108030976;
	// 825A6270: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 825A6274: 38CB5F48  addi r6, r11, 0x5f48
	ctx.r[6].s64 = ctx.r[11].s64 + 24392;
	// 825A6278: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A627C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6280: 4BFFFBE1  bl 0x825a5e60
	ctx.lr = 0x825A6284;
	sub_825A5E60(ctx, base);
	// 825A6284: 2F1D0005  cmpwi cr6, r29, 5
	ctx.cr[6].compare_i32(ctx.r[29].s32, 5, &mut ctx.xer);
	// 825A6288: 4098001C  bge cr6, 0x825a62a4
	if !ctx.cr[6].lt {
	pc = 0x825A62A4; continue 'dispatch;
	}
	// 825A628C: 3D60825A  lis r11, -0x7da6
	ctx.r[11].s64 = -2108030976;
	// 825A6290: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 825A6294: 38CB6180  addi r6, r11, 0x6180
	ctx.r[6].s64 = ctx.r[11].s64 + 24960;
	// 825A6298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A629C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A62A0: 4BFFFBC1  bl 0x825a5e60
	ctx.lr = 0x825A62A4;
	sub_825A5E60(ctx, base);
	// 825A62A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A62A8: 4BF8EE64  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A62B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A62B0 size=192
    let mut pc: u32 = 0x825A62B0;
    'dispatch: loop {
        match pc {
            0x825A62B0 => {
    //   block [0x825A62B0..0x825A6370)
	// 825A62B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A62B4: 4BF8EE05  bl 0x825350b8
	ctx.lr = 0x825A62B8;
	sub_82535080(ctx, base);
	// 825A62B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A62BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A62C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A62C4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825A62C8: 4BEC7079  bl 0x8246d340
	ctx.lr = 0x825A62CC;
	sub_8246D340(ctx, base);
	// 825A62CC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A62D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A62D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A62D8: 419A0088  beq cr6, 0x825a6360
	if ctx.cr[6].eq {
	pc = 0x825A6360; continue 'dispatch;
	}
	// 825A62DC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 825A62E0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A62E4: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 825A62E8: 409A0020  bne cr6, 0x825a6308
	if !ctx.cr[6].eq {
	pc = 0x825A6308; continue 'dispatch;
	}
	// 825A62EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A62F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A62F4: 4BFFFC85  bl 0x825a5f78
	ctx.lr = 0x825A62F8;
	sub_825A5F78(ctx, base);
	// 825A62F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A62FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6300: 4BFFFDD1  bl 0x825a60d0
	ctx.lr = 0x825A6304;
	sub_825A60D0(ctx, base);
	// 825A6304: 4800000C  b 0x825a6310
	pc = 0x825A6310; continue 'dispatch;
	// 825A6308: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 825A630C: 4098001C  bge cr6, 0x825a6328
	if !ctx.cr[6].lt {
	pc = 0x825A6328; continue 'dispatch;
	}
	// 825A6310: 3D60825A  lis r11, -0x7da6
	ctx.r[11].s64 = -2108030976;
	// 825A6314: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 825A6318: 38CB5F48  addi r6, r11, 0x5f48
	ctx.r[6].s64 = ctx.r[11].s64 + 24392;
	// 825A631C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A6320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6324: 4BFFFB3D  bl 0x825a5e60
	ctx.lr = 0x825A6328;
	sub_825A5E60(ctx, base);
	// 825A6328: 2F1C0005  cmpwi cr6, r28, 5
	ctx.cr[6].compare_i32(ctx.r[28].s32, 5, &mut ctx.xer);
	// 825A632C: 4098001C  bge cr6, 0x825a6348
	if !ctx.cr[6].lt {
	pc = 0x825A6348; continue 'dispatch;
	}
	// 825A6330: 3D60825A  lis r11, -0x7da6
	ctx.r[11].s64 = -2108030976;
	// 825A6334: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 825A6338: 38CB6180  addi r6, r11, 0x6180
	ctx.r[6].s64 = ctx.r[11].s64 + 24960;
	// 825A633C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A6340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6344: 4BFFFB1D  bl 0x825a5e60
	ctx.lr = 0x825A6348;
	sub_825A5E60(ctx, base);
	// 825A6348: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A634C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6350: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 825A6354: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6358: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A635C: 409AFF84  bne cr6, 0x825a62e0
	if !ctx.cr[6].eq {
	pc = 0x825A62E0; continue 'dispatch;
	}
	// 825A6360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6364: 4BEC7065  bl 0x8246d3c8
	ctx.lr = 0x825A6368;
	sub_8246D3C8(ctx, base);
	// 825A6368: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A636C: 4BF8ED9C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6370 size=112
    let mut pc: u32 = 0x825A6370;
    'dispatch: loop {
        match pc {
            0x825A6370 => {
    //   block [0x825A6370..0x825A63E0)
	// 825A6370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A637C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A6384: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 825A6388: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A638C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825A6390: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6394: 394A9904  addi r10, r10, -0x66fc
	ctx.r[10].s64 = ctx.r[10].s64 + -26364;
	// 825A6398: 39299724  addi r9, r9, -0x68dc
	ctx.r[9].s64 = ctx.r[9].s64 + -26844;
	// 825A639C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825A63A0: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 825A63A4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 825A63A8: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A63AC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A63B0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 825A63B4: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 825A63B8: 480006A1  bl 0x825a6a58
	ctx.lr = 0x825A63BC;
	sub_825A6A58(ctx, base);
	// 825A63BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825A63C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A63C4: 396B96B4  addi r11, r11, -0x694c
	ctx.r[11].s64 = ctx.r[11].s64 + -26956;
	// 825A63C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A63CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A63D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A63D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A63D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A63DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A63E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A63E0 size=64
    let mut pc: u32 = 0x825A63E0;
    'dispatch: loop {
        match pc {
            0x825A63E0 => {
    //   block [0x825A63E0..0x825A6420)
	// 825A63E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A63E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A63E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A63EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A63F0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A63F4: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 825A63F8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 825A63FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825A6400: 4BEBDC39  bl 0x82464038
	ctx.lr = 0x825A6404;
	sub_82464038(ctx, base);
	// 825A6404: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 825A6408: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825A640C: 4BFFFF65  bl 0x825a6370
	ctx.lr = 0x825A6410;
	sub_825A6370(ctx, base);
	// 825A6410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A641C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6420 size=16
    let mut pc: u32 = 0x825A6420;
    'dispatch: loop {
        match pc {
            0x825A6420 => {
    //   block [0x825A6420..0x825A6430)
	// 825A6420: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825A6424: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 825A6428: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A642C: 48000344  b 0x825a6770
	sub_825A6770(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6430 size=112
    let mut pc: u32 = 0x825A6430;
    'dispatch: loop {
        match pc {
            0x825A6430 => {
    //   block [0x825A6430..0x825A64A0)
	// 825A6430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A643C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6444: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6448: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825A644C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A6450: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 825A6454: 4800069D  bl 0x825a6af0
	ctx.lr = 0x825A6458;
	sub_825A6AF0(ctx, base);
	// 825A6458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A645C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A6460: 419A0024  beq cr6, 0x825a6484
	if ctx.cr[6].eq {
	pc = 0x825A6484; continue 'dispatch;
	}
	// 825A6464: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6468: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A646C: 419A0010  beq cr6, 0x825a647c
	if ctx.cr[6].eq {
	pc = 0x825A647C; continue 'dispatch;
	}
	// 825A6470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6478: 4E800421  bctrl
	ctx.lr = 0x825A647C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A647C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6480: 48000008  b 0x825a6488
	pc = 0x825A6488; continue 'dispatch;
	// 825A6484: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A6488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A648C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A649C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A64A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A64A0 size=104
    let mut pc: u32 = 0x825A64A0;
    'dispatch: loop {
        match pc {
            0x825A64A0 => {
    //   block [0x825A64A0..0x825A6508)
	// 825A64A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A64A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A64A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A64AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A64B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A64B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A64B8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825A64BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A64C0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 825A64C4: 4800062D  bl 0x825a6af0
	ctx.lr = 0x825A64C8;
	sub_825A6AF0(ctx, base);
	// 825A64C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A64CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A64D0: 419A001C  beq cr6, 0x825a64ec
	if ctx.cr[6].eq {
	pc = 0x825A64EC; continue 'dispatch;
	}
	// 825A64D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A64D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A64DC: 419A0010  beq cr6, 0x825a64ec
	if ctx.cr[6].eq {
	pc = 0x825A64EC; continue 'dispatch;
	}
	// 825A64E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A64E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A64E8: 4E800421  bctrl
	ctx.lr = 0x825A64EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A64EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A64F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A64F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A64F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A64FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6508 size=160
    let mut pc: u32 = 0x825A6508;
    'dispatch: loop {
        match pc {
            0x825A6508 => {
    //   block [0x825A6508..0x825A65A8)
	// 825A6508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A650C: 4BF8EBAD  bl 0x825350b8
	ctx.lr = 0x825A6510;
	sub_82535080(ctx, base);
	// 825A6510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6514: 3BC40008  addi r30, r4, 8
	ctx.r[30].s64 = ctx.r[4].s64 + 8;
	// 825A6518: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A651C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6520: 48000179  bl 0x825a6698
	ctx.lr = 0x825A6524;
	sub_825A6698(ctx, base);
	// 825A6524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6528: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A652C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6530: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A6534: 4800021D  bl 0x825a6750
	ctx.lr = 0x825A6538;
	sub_825A6750(ctx, base);
	// 825A6538: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A653C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6540: 419A0060  beq cr6, 0x825a65a0
	if ctx.cr[6].eq {
	pc = 0x825A65A0; continue 'dispatch;
	}
	// 825A6544: 3B9D0008  addi r28, r29, 8
	ctx.r[28].s64 = ctx.r[29].s64 + 8;
	// 825A6548: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A654C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6550: 480001A1  bl 0x825a66f0
	ctx.lr = 0x825A6554;
	sub_825A66F0(ctx, base);
	// 825A6554: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A6558: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A655C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6560: 48000171  bl 0x825a66d0
	ctx.lr = 0x825A6564;
	sub_825A66D0(ctx, base);
	// 825A6564: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A6568: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825A656C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A6570: 48000201  bl 0x825a6770
	ctx.lr = 0x825A6574;
	sub_825A6770(ctx, base);
	// 825A6574: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A6578: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A657C: 48000195  bl 0x825a6710
	ctx.lr = 0x825A6580;
	sub_825A6710(ctx, base);
	// 825A6580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6584: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A6588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A658C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A6590: 480001C1  bl 0x825a6750
	ctx.lr = 0x825A6594;
	sub_825A6750(ctx, base);
	// 825A6594: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A6598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A659C: 409AFFAC  bne cr6, 0x825a6548
	if !ctx.cr[6].eq {
	pc = 0x825A6548; continue 'dispatch;
	}
	// 825A65A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A65A4: 4BF8EB64  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A65A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A65A8 size=116
    let mut pc: u32 = 0x825A65A8;
    'dispatch: loop {
        match pc {
            0x825A65A8 => {
    //   block [0x825A65A8..0x825A661C)
	// 825A65A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A65AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A65B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A65B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A65B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A65BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A65C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A65C4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A65C8: 480004F9  bl 0x825a6ac0
	ctx.lr = 0x825A65CC;
	sub_825A6AC0(ctx, base);
	// 825A65CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A65D0: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 825A65D4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 825A65D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A65DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A65E0: 419A0020  beq cr6, 0x825a6600
	if ctx.cr[6].eq {
	pc = 0x825A6600; continue 'dispatch;
	}
	// 825A65E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A65E8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A65EC: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 825A65F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A65F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A65F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825A65FC: 4BEBDABD  bl 0x824640b8
	ctx.lr = 0x825A6600;
	sub_824640B8(ctx, base);
	// 825A6600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6604: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A660C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6610: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6620 size=120
    let mut pc: u32 = 0x825A6620;
    'dispatch: loop {
        match pc {
            0x825A6620 => {
    //   block [0x825A6620..0x825A6698)
	// 825A6620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A662C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6630: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6634: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A6638: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 825A663C: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 825A6640: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825A6644: 4BEBD9F5  bl 0x82464038
	ctx.lr = 0x825A6648;
	sub_82464038(ctx, base);
	// 825A6648: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 825A664C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A6650: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825A6654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6658: 394A96E4  addi r10, r10, -0x691c
	ctx.r[10].s64 = ctx.r[10].s64 + -26908;
	// 825A665C: 39000018  li r8, 0x18
	ctx.r[8].s64 = 24;
	// 825A6660: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825A6664: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 825A6668: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A666C: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A6670: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A6674: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 825A6678: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 825A667C: 480003DD  bl 0x825a6a58
	ctx.lr = 0x825A6680;
	sub_825A6A58(ctx, base);
	// 825A6680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6684: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A668C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6698 size=20
    let mut pc: u32 = 0x825A6698;
    'dispatch: loop {
        match pc {
            0x825A6698 => {
    //   block [0x825A6698..0x825A66AC)
	// 825A6698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A669C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A66A0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A66A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825A66A8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A66AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A66AC size=16
    let mut pc: u32 = 0x825A66AC;
    'dispatch: loop {
        match pc {
            0x825A66AC => {
    //   block [0x825A66AC..0x825A66BC)
	// 825A66AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A66B0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A66B4: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 825A66B8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A66BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A66BC size=20
    let mut pc: u32 = 0x825A66BC;
    'dispatch: loop {
        match pc {
            0x825A66BC => {
    //   block [0x825A66BC..0x825A66D0)
	// 825A66BC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 825A66C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A66C4: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825A66C8: 4099FFE8  ble cr6, 0x825a66b0
	if !ctx.cr[6].gt {
		sub_825A66AC(ctx, base);
		return;
	}
	// 825A66CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A66D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A66D0 size=28
    let mut pc: u32 = 0x825A66D0;
    'dispatch: loop {
        match pc {
            0x825A66D0 => {
    //   block [0x825A66D0..0x825A66EC)
	// 825A66D0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A66D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A66D8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825A66DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A66E0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A66E4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A66E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A66F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A66F0 size=32
    let mut pc: u32 = 0x825A66F0;
    'dispatch: loop {
        match pc {
            0x825A66F0 => {
    //   block [0x825A66F0..0x825A6710)
	// 825A66F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A66F4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A66F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A66FC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6700: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825A6704: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6708: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A670C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6710 size=20
    let mut pc: u32 = 0x825A6710;
    'dispatch: loop {
        match pc {
            0x825A6710 => {
    //   block [0x825A6710..0x825A6724)
	// 825A6710: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A6714: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 825A6718: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A671C: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825A6720: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6724 size=24
    let mut pc: u32 = 0x825A6724;
    'dispatch: loop {
        match pc {
            0x825A6724 => {
    //   block [0x825A6724..0x825A673C)
	// 825A6724: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6728: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A672C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A6730: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6734: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 825A6738: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A673C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A673C size=20
    let mut pc: u32 = 0x825A673C;
    'dispatch: loop {
        match pc {
            0x825A673C => {
    //   block [0x825A673C..0x825A6750)
	// 825A673C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 825A6740: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A6744: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825A6748: 4099FFE8  ble cr6, 0x825a6730
	if !ctx.cr[6].gt {
		sub_825A6724(ctx, base);
		return;
	}
	// 825A674C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6750 size=28
    let mut pc: u32 = 0x825A6750;
    'dispatch: loop {
        match pc {
            0x825A6750 => {
    //   block [0x825A6750..0x825A676C)
	// 825A6750: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6754: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825A6758: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A675C: 40990008  ble cr6, 0x825a6764
	if !ctx.cr[6].gt {
	pc = 0x825A6764; continue 'dispatch;
	}
	// 825A6760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A6764: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A6768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6770 size=316
    let mut pc: u32 = 0x825A6770;
    'dispatch: loop {
        match pc {
            0x825A6770 => {
    //   block [0x825A6770..0x825A68AC)
	// 825A6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6774: 4BF8E941  bl 0x825350b4
	ctx.lr = 0x825A6778;
	sub_82535080(ctx, base);
	// 825A6778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A677C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825A6780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6784: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825A6788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A678C: 895C0000  lbz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6790: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 825A6794: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825A6798: 419A002C  beq cr6, 0x825a67c4
	if ctx.cr[6].eq {
	pc = 0x825A67C4; continue 'dispatch;
	}
	// 825A679C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 825A67A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A67A4: 55672834  slwi r7, r11, 5
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 825A67A8: 7D6B3850  subf r11, r11, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 825A67AC: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A67B0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 825A67B4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 825A67B8: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 825A67BC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825A67C0: 409AFFE0  bne cr6, 0x825a67a0
	if !ctx.cr[6].eq {
	pc = 0x825A67A0; continue 'dispatch;
	}
	// 825A67C4: 557D007E  clrlwi r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 825A67C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A67CC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A67D0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A67D4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825A67D8: 40990014  ble cr6, 0x825a67ec
	if !ctx.cr[6].gt {
	pc = 0x825A67EC; continue 'dispatch;
	}
	// 825A67DC: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 825A67E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A67E4: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 825A67E8: 48000339  bl 0x825a6b20
	ctx.lr = 0x825A67EC;
	sub_825A6B20(ctx, base);
	// 825A67EC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A67F0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A67F4: 7D5EE838  and r30, r10, r29
	ctx.r[30].u64 = ctx.r[10].u64 & ctx.r[29].u64;
	// 825A67F8: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A67FC: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825A6800: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 825A6804: 419A0054  beq cr6, 0x825a6858
	if ctx.cr[6].eq {
	pc = 0x825A6858; continue 'dispatch;
	}
	// 825A6808: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A680C: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825A6810: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825A6814: 409A0024  bne cr6, 0x825a6838
	if !ctx.cr[6].eq {
	pc = 0x825A6838; continue 'dispatch;
	}
	// 825A6818: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 825A681C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A6820: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A6824: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6828: 7C8B482E  lwzx r4, r11, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825A682C: 4BEC37A5  bl 0x82469fd0
	ctx.lr = 0x825A6830;
	sub_82469FD0(ctx, base);
	// 825A6830: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A6834: 419A0030  beq cr6, 0x825a6864
	if ctx.cr[6].eq {
	pc = 0x825A6864; continue 'dispatch;
	}
	// 825A6838: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A683C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 825A6840: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6844: 7D7E5038  and r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 825A6848: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A684C: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825A6850: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 825A6854: 409AFFB4  bne cr6, 0x825a6808
	if !ctx.cr[6].eq {
	pc = 0x825A6808; continue 'dispatch;
	}
	// 825A6858: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A685C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A6860: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A6864: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6868: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A686C: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 825A6870: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6874: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6878: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825A687C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A6880: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6884: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 825A6888: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A688C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6890: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A6894: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6898: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825A689C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A68A0: 7F6B512E  stwx r27, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u32) };
	// 825A68A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A68A8: 4BF8E85C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A68B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A68B0 size=212
    let mut pc: u32 = 0x825A68B0;
    'dispatch: loop {
        match pc {
            0x825A68B0 => {
    //   block [0x825A68B0..0x825A6984)
	// 825A68B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A68B4: 4BF8E805  bl 0x825350b8
	ctx.lr = 0x825A68B8;
	sub_82535080(ctx, base);
	// 825A68B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A68BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825A68C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A68C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A68C8: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A68CC: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 825A68D0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825A68D4: 419A002C  beq cr6, 0x825a6900
	if ctx.cr[6].eq {
	pc = 0x825A6900; continue 'dispatch;
	}
	// 825A68D8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 825A68DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A68E0: 55472834  slwi r7, r10, 5
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 825A68E4: 7D4A3850  subf r10, r10, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 825A68E8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A68EC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 825A68F0: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 825A68F4: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 825A68F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825A68FC: 409AFFE0  bne cr6, 0x825a68dc
	if !ctx.cr[6].eq {
	pc = 0x825A68DC; continue 'dispatch;
	}
	// 825A6900: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6904: 555D007E  clrlwi r29, r10, 1
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 825A6908: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A690C: 7D7FE838  and r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 & ctx.r[29].u64;
	// 825A6910: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825A6914: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A6918: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 825A691C: 419A004C  beq cr6, 0x825a6968
	if ctx.cr[6].eq {
	pc = 0x825A6968; continue 'dispatch;
	}
	// 825A6920: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825A6924: 409A0024  bne cr6, 0x825a6948
	if !ctx.cr[6].eq {
	pc = 0x825A6948; continue 'dispatch;
	}
	// 825A6928: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A692C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A6930: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A6934: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6938: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A693C: 4BEC3695  bl 0x82469fd0
	ctx.lr = 0x825A6940;
	sub_82469FD0(ctx, base);
	// 825A6940: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825A6944: 419A0034  beq cr6, 0x825a6978
	if ctx.cr[6].eq {
	pc = 0x825A6978; continue 'dispatch;
	}
	// 825A6948: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A694C: 393F0001  addi r9, r31, 1
	ctx.r[9].s64 = ctx.r[31].s64 + 1;
	// 825A6950: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6954: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 825A6958: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825A695C: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A6960: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 825A6964: 409AFFBC  bne cr6, 0x825a6920
	if !ctx.cr[6].eq {
	pc = 0x825A6920; continue 'dispatch;
	}
	// 825A6968: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A696C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 825A6970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A6974: 4BF8E794  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 825A6978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A697C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A6980: 4BF8E788  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6988 size=136
    let mut pc: u32 = 0x825A6988;
    'dispatch: loop {
        match pc {
            0x825A6988 => {
    //   block [0x825A6988..0x825A6A10)
	// 825A6988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A698C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A6994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A699C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A69A0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A69A4: 4BFFFF0D  bl 0x825a68b0
	ctx.lr = 0x825A69A8;
	sub_825A68B0(ctx, base);
	// 825A69A8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A69AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A69B0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825A69B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A69B8: 40990008  ble cr6, 0x825a69c0
	if !ctx.cr[6].gt {
	pc = 0x825A69C0; continue 'dispatch;
	}
	// 825A69BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A69C0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 825A69C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825A69C8: 419A002C  beq cr6, 0x825a69f4
	if ctx.cr[6].eq {
	pc = 0x825A69F4; continue 'dispatch;
	}
	// 825A69CC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A69D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A69D4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A69D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A69DC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A69E0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825A69E4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A69E8: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825A69EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A69F0: 48000008  b 0x825a69f8
	pc = 0x825A69F8; continue 'dispatch;
	// 825A69F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A69F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A69FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6A04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6A08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6A10 size=68
    let mut pc: u32 = 0x825A6A10;
    'dispatch: loop {
        match pc {
            0x825A6A10 => {
    //   block [0x825A6A10..0x825A6A54)
	// 825A6A10: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A6A18: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 825A6A1C: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6A20: 4081002C  ble 0x825a6a4c
	if !ctx.cr[0].gt {
	pc = 0x825A6A4C; continue 'dispatch;
	}
	// 825A6A24: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 825A6A28: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 825A6A2C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6A30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A6A34: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 825A6A38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A6A3C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6A40: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 825A6A44: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825A6A48: 4198FFE4  blt cr6, 0x825a6a2c
	if ctx.cr[6].lt {
	pc = 0x825A6A2C; continue 'dispatch;
	}
	// 825A6A4C: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 825A6A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6A58 size=100
    let mut pc: u32 = 0x825A6A58;
    'dispatch: loop {
        match pc {
            0x825A6A58 => {
    //   block [0x825A6A58..0x825A6ABC)
	// 825A6A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6A60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6A64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6A68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6A6C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A6A70: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 825A6A74: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 825A6A78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6A7C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825A6A80: 4BEBD5B9  bl 0x82464038
	ctx.lr = 0x825A6A84;
	sub_82464038(ctx, base);
	// 825A6A84: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 825A6A88: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A6A8C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 825A6A90: 4BEC38A9  bl 0x8246a338
	ctx.lr = 0x825A6A94;
	sub_8246A338(ctx, base);
	// 825A6A94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A6A98: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 825A6A9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6AA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A6AA4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A6AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6AC0 size=44
    let mut pc: u32 = 0x825A6AC0;
    'dispatch: loop {
        match pc {
            0x825A6AC0 => {
    //   block [0x825A6AC0..0x825A6AEC)
	// 825A6AC0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6AC4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A6AC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6ACC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825A6AD0: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6AD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A6AD8: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825A6ADC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A6AE0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A6AE4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825A6AE8: 4BEBD5D0  b 0x824640b8
	sub_824640B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6AF0 size=48
    let mut pc: u32 = 0x825A6AF0;
    'dispatch: loop {
        match pc {
            0x825A6AF0 => {
    //   block [0x825A6AF0..0x825A6B20)
	// 825A6AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6AFC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825A6B00: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A6B04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A6B08: 4BFFFE81  bl 0x825a6988
	ctx.lr = 0x825A6B0C;
	sub_825A6988(ctx, base);
	// 825A6B0C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A6B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6B20 size=216
    let mut pc: u32 = 0x825A6B20;
    'dispatch: loop {
        match pc {
            0x825A6B20 => {
    //   block [0x825A6B20..0x825A6BF8)
	// 825A6B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6B24: 4BF8E585  bl 0x825350a8
	ctx.lr = 0x825A6B28;
	sub_82535080(ctx, base);
	// 825A6B28: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6B30: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6B34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6B38: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 825A6B3C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 825A6B40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6B44: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6B48: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 825A6B4C: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 825A6B50: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6B54: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825A6B58: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 825A6B5C: 4BEBD4DD  bl 0x82464038
	ctx.lr = 0x825A6B60;
	sub_82464038(ctx, base);
	// 825A6B60: 57C5103A  slwi r5, r30, 2
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825A6B64: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 825A6B68: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A6B6C: 4BEC37CD  bl 0x8246a338
	ctx.lr = 0x825A6B70;
	sub_8246A338(ctx, base);
	// 825A6B70: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 825A6B74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A6B78: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A6B7C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825A6B80: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A6B84: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A6B88: 4099004C  ble cr6, 0x825a6bd4
	if !ctx.cr[6].gt {
	pc = 0x825A6BD4; continue 'dispatch;
	}
	// 825A6B8C: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6B90: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 825A6B94: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 825A6B98: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6B9C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 825A6BA0: 419A0020  beq cr6, 0x825a6bc0
	if ctx.cr[6].eq {
	pc = 0x825A6BC0; continue 'dispatch;
	}
	// 825A6BA4: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6BA8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6BAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6BB0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825A6BB4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6BB8: 7CABD82E  lwzx r5, r11, r27
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 825A6BBC: 4BFFFBB5  bl 0x825a6770
	ctx.lr = 0x825A6BC0;
	sub_825A6770(ctx, base);
	// 825A6BC0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A6BC4: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 825A6BC8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 825A6BCC: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825A6BD0: 4198FFC8  blt cr6, 0x825a6b98
	if ctx.cr[6].lt {
	pc = 0x825A6B98; continue 'dispatch;
	}
	// 825A6BD4: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A6BD8: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 825A6BDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825A6BE0: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 825A6BE4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825A6BE8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825A6BEC: 4BEBD4CD  bl 0x824640b8
	ctx.lr = 0x825A6BF0;
	sub_824640B8(ctx, base);
	// 825A6BF0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A6BF4: 4BF8E504  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A6BF8 size=64
    let mut pc: u32 = 0x825A6BF8;
    'dispatch: loop {
        match pc {
            0x825A6BF8 => {
    //   block [0x825A6BF8..0x825A6C38)
	// 825A6BF8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A6BFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A6C00: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 825A6C04: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6C08: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825A6C0C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825A6C10: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 825A6C14: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A6C18: 419A0020  beq cr6, 0x825a6c38
	if ctx.cr[6].eq {
		sub_825A6C38(ctx, base);
		return;
	}
	// 825A6C1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A6C20: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A6C24: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 825A6C28: 4198FFDC  blt cr6, 0x825a6c04
	if ctx.cr[6].lt {
	pc = 0x825A6C04; continue 'dispatch;
	}
	// 825A6C2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A6C30: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A6C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6C38 size=12
    let mut pc: u32 = 0x825A6C38;
    'dispatch: loop {
        match pc {
            0x825A6C38 => {
    //   block [0x825A6C38..0x825A6C44)
	// 825A6C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A6C3C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A6C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6C48 size=12
    let mut pc: u32 = 0x825A6C48;
    'dispatch: loop {
        match pc {
            0x825A6C48 => {
    //   block [0x825A6C48..0x825A6C54)
	// 825A6C48: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 825A6C4C: 386B2690  addi r3, r11, 0x2690
	ctx.r[3].s64 = ctx.r[11].s64 + 9872;
	// 825A6C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A6C58 size=52
    let mut pc: u32 = 0x825A6C58;
    'dispatch: loop {
        match pc {
            0x825A6C58 => {
    //   block [0x825A6C58..0x825A6C8C)
	// 825A6C58: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6C5C: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6C60: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A6C64: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A6C68: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6C6C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6C70: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A6C74: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A6C78: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6C7C: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6C80: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A6C84: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A6C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A6C90 size=96
    let mut pc: u32 = 0x825A6C90;
    'dispatch: loop {
        match pc {
            0x825A6C90 => {
    //   block [0x825A6C90..0x825A6CF0)
	// 825A6C90: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A6C94: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6C98: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A6C9C: C1A40014  lfs f13, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6CA0: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A6CA4: C1A40028  lfs f13, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6CA8: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6CAC: D1A30028  stfs f13, 0x28(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A6CB0: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A6CB4: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A6CB8: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A6CBC: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6CC0: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A6CC4: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6CC8: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A6CCC: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6CD0: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A6CD4: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6CD8: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A6CDC: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6CE0: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A6CE4: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6CE8: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A6CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6CF0 size=60
    let mut pc: u32 = 0x825A6CF0;
    'dispatch: loop {
        match pc {
            0x825A6CF0 => {
    //   block [0x825A6CF0..0x825A6D2C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6D30 size=80
    let mut pc: u32 = 0x825A6D30;
    'dispatch: loop {
        match pc {
            0x825A6D30 => {
    //   block [0x825A6D30..0x825A6D80)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6D80 size=236
    let mut pc: u32 = 0x825A6D80;
    'dispatch: loop {
        match pc {
            0x825A6D80 => {
    //   block [0x825A6D80..0x825A6E6C)
	// 825A6D80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A6E70 size=140
    let mut pc: u32 = 0x825A6E70;
    'dispatch: loop {
        match pc {
            0x825A6E70 => {
    //   block [0x825A6E70..0x825A6EFC)
	// 825A6E70: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A6E74: C1A40004  lfs f13, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6E78: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A6E7C: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 825A6E80: D1A1FFD8  stfs f13, -0x28(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 825A6E84: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 825A6E88: D181FFD4  stfs f12, -0x2c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 825A6E8C: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6E90: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 825A6E94: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 825A6E98: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 825A6E9C: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 825A6EA0: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 825A6EA4: D001FFEC  stfs f0, -0x14(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 825A6EA8: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 825A6EAC: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6F00 size=284
    let mut pc: u32 = 0x825A6F00;
    'dispatch: loop {
        match pc {
            0x825A6F00 => {
    //   block [0x825A6F00..0x825A701C)
	// 825A6F00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A6F04: EC010072  fmuls f0, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 825A6F08: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 825A6F0C: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 825A6F10: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A701C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A701C size=8
    let mut pc: u32 = 0x825A701C;
    'dispatch: loop {
        match pc {
            0x825A701C => {
    //   block [0x825A701C..0x825A7024)
	// 825A701C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A7020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7028 size=200
    let mut pc: u32 = 0x825A7028;
    'dispatch: loop {
        match pc {
            0x825A7028 => {
    //   block [0x825A7028..0x825A70F0)
	// 825A7028: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A70F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A70F0 size=68
    let mut pc: u32 = 0x825A70F0;
    'dispatch: loop {
        match pc {
            0x825A70F0 => {
    //   block [0x825A70F0..0x825A7134)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7138 size=68
    let mut pc: u32 = 0x825A7138;
    'dispatch: loop {
        match pc {
            0x825A7138 => {
    //   block [0x825A7138..0x825A717C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7180 size=60
    let mut pc: u32 = 0x825A7180;
    'dispatch: loop {
        match pc {
            0x825A7180 => {
    //   block [0x825A7180..0x825A71BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A71C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A71C0 size=84
    let mut pc: u32 = 0x825A71C0;
    'dispatch: loop {
        match pc {
            0x825A71C0 => {
    //   block [0x825A71C0..0x825A7214)
	// 825A71C0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7218 size=4
    let mut pc: u32 = 0x825A7218;
    'dispatch: loop {
        match pc {
            0x825A7218 => {
    //   block [0x825A7218..0x825A721C)
	// 825A7218: 4BFFFE10  b 0x825a7028
	sub_825A7028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7220 size=84
    let mut pc: u32 = 0x825A7220;
    'dispatch: loop {
        match pc {
            0x825A7220 => {
    //   block [0x825A7220..0x825A7274)
	// 825A7220: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7278 size=180
    let mut pc: u32 = 0x825A7278;
    'dispatch: loop {
        match pc {
            0x825A7278 => {
    //   block [0x825A7278..0x825A732C)
	// 825A7278: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A727C: C0050000  lfs f0, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7280: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 825A7284: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A7288: C0050014  lfs f0, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A728C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 825A7290: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 825A7294: C1A50028  lfs f13, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7298: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A729C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 825A72A0: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 825A72A4: D001FFEC  stfs f0, -0x14(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 825A72A8: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A72AC: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A72B0: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825A72B4: C0050020  lfs f0, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A72B8: D001FFD8  stfs f0, -0x28(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 825A72BC: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A72C0: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825A72C4: C0050024  lfs f0, 0x24(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A72C8: D1A1FFF8  stfs f13, -8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 825A72CC: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 825A72D0: C1A50010  lfs f13, 0x10(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7330 size=92
    let mut pc: u32 = 0x825A7330;
    'dispatch: loop {
        match pc {
            0x825A7330 => {
    //   block [0x825A7330..0x825A738C)
	// 825A7330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7338: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A733C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825A7340: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825A7344: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 825A7348: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A734C: 4BFFFED5  bl 0x825a7220
	ctx.lr = 0x825A7350;
	sub_825A7220(ctx, base);
	// 825A7350: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825A7354: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 825A7358: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7390 size=68
    let mut pc: u32 = 0x825A7390;
    'dispatch: loop {
        match pc {
            0x825A7390 => {
    //   block [0x825A7390..0x825A73D4)
	// 825A7390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7398: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A739C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 825A73A0: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 825A73A4: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 825A73A8: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 825A73AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A73B0: 4BFFFEC9  bl 0x825a7278
	ctx.lr = 0x825A73B4;
	sub_825A7278(ctx, base);
	// 825A73B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A73B8: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 825A73BC: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 825A73C0: 4BFFFE61  bl 0x825a7220
	ctx.lr = 0x825A73C4;
	sub_825A7220(ctx, base);
	// 825A73C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A73C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A73CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A73D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A73D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A73D8 size=64
    let mut pc: u32 = 0x825A73D8;
    'dispatch: loop {
        match pc {
            0x825A73D8 => {
    //   block [0x825A73D8..0x825A7418)
	// 825A73D8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A73DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A73E0: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 825A73E4: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A73E8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825A73EC: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825A73F0: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 825A73F4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A73F8: 419A0020  beq cr6, 0x825a7418
	if ctx.cr[6].eq {
		sub_825A7418(ctx, base);
		return;
	}
	// 825A73FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A7400: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A7404: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 825A7408: 4198FFDC  blt cr6, 0x825a73e4
	if ctx.cr[6].lt {
	pc = 0x825A73E4; continue 'dispatch;
	}
	// 825A740C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A7410: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A7414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7418 size=12
    let mut pc: u32 = 0x825A7418;
    'dispatch: loop {
        match pc {
            0x825A7418 => {
    //   block [0x825A7418..0x825A7424)
	// 825A7418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A741C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A7420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7428 size=100
    let mut pc: u32 = 0x825A7428;
    'dispatch: loop {
        match pc {
            0x825A7428 => {
    //   block [0x825A7428..0x825A748C)
	// 825A7428: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A742C: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7430: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A7434: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A7438: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A743C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7440: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A7444: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A7448: C0030030  lfs f0, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A744C: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7450: D1A30030  stfs f13, 0x30(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A7454: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A7458: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A745C: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7460: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A7464: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A7468: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A746C: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7470: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A7474: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A7478: C0030038  lfs f0, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A747C: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7480: D1A30038  stfs f13, 0x38(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A7484: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A7488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7490 size=132
    let mut pc: u32 = 0x825A7490;
    'dispatch: loop {
        match pc {
            0x825A7490 => {
    //   block [0x825A7490..0x825A7514)
	// 825A7490: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7494: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A7498: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A749C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A74A0: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74A4: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A74A8: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74AC: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A74B0: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74B4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A74B8: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74BC: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A74C0: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74C4: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A74C8: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74CC: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A74D0: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74D4: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A74D8: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74DC: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A74E0: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74E4: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A74E8: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74EC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A74F0: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74F4: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A74F8: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A74FC: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A7500: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7504: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A7508: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A750C: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A7510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7518 size=80
    let mut pc: u32 = 0x825A7518;
    'dispatch: loop {
        match pc {
            0x825A7518 => {
    //   block [0x825A7518..0x825A7568)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7568 size=300
    let mut pc: u32 = 0x825A7568;
    'dispatch: loop {
        match pc {
            0x825A7568 => {
    //   block [0x825A7568..0x825A7694)
	// 825A7568: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7698 size=704
    let mut pc: u32 = 0x825A7698;
    'dispatch: loop {
        match pc {
            0x825A7698 => {
    //   block [0x825A7698..0x825A7958)
	// 825A7698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A76A0: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 825A76A4: 4BF8E90D  bl 0x82535fb0
	ctx.lr = 0x825A76A8;
	sub_82535FB0(ctx, base);
	// 825A76A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A76AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A76B0: 38C1FF00  addi r6, r1, -0x100
	ctx.r[6].s64 = ctx.r[1].s64 + -256;
	// 825A76B4: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 825A76B8: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 825A76BC: C16B0004  lfs f11, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A76C0: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 825A76C4: C12B0020  lfs f9, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A76C8: C10B001C  lfs f8, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A76CC: EF6902F2  fmuls f27, f9, f11
	ctx.f[27].f64 = (((ctx.f[9].f64 * ctx.f[11].f64) as f32) as f64);
	// 825A76D0: C06B0038  lfs f3, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 825A76D4: EF430232  fmuls f26, f3, f8
	ctx.f[26].f64 = (((ctx.f[3].f64 * ctx.f[8].f64) as f32) as f64);
	// 825A76D8: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A76DC: C3EB000C  lfs f31, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A76E0: EF090032  fmuls f24, f9, f0
	ctx.f[24].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A76E4: C3AB0030  lfs f29, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 825A76E8: EEE307F2  fmuls f23, f3, f31
	ctx.f[23].f64 = (((ctx.f[3].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A76EC: C0AB0028  lfs f5, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 825A76F0: EEDD02F2  fmuls f22, f29, f11
	ctx.f[22].f64 = (((ctx.f[29].f64 * ctx.f[11].f64) as f32) as f64);
	// 825A76F4: EEA50232  fmuls f21, f5, f8
	ctx.f[21].f64 = (((ctx.f[5].f64 * ctx.f[8].f64) as f32) as f64);
	// 825A76F8: C14B0024  lfs f10, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A76FC: EE9D0032  fmuls f20, f29, f0
	ctx.f[20].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A7700: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7704: C18B0010  lfs f12, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A7708: EE6507F2  fmuls f19, f5, f31
	ctx.f[19].f64 = (((ctx.f[5].f64 * ctx.f[31].f64) as f32) as f64);
	// 825A770C: C0EB0018  lfs f7, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 825A7710: EE1D02B2  fmuls f16, f29, f10
	ctx.f[16].f64 = (((ctx.f[29].f64 * ctx.f[10].f64) as f32) as f64);
	// 825A7714: C0CB002C  lfs f6, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 825A7718: EF8B0332  fmuls f28, f11, f12
	ctx.f[28].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A771C: C08B003C  lfs f4, 0x3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 825A7720: EF2301B2  fmuls f25, f3, f6
	ctx.f[25].f64 = (((ctx.f[3].f64 * ctx.f[6].f64) as f32) as f64);
	// 825A7724: EF6ADB78  fmsubs f27, f10, f13, f27
	ctx.f[27].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 - ctx.f[27].f64) as f32) as f64);
	// 825A7728: C04B0008  lfs f2, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825A772C: EF44D1F8  fmsubs f26, f4, f7, f26
	ctx.f[26].f64 = (((ctx.f[4].f64 * ctx.f[7].f64 - ctx.f[26].f64) as f32) as f64);
	// 825A7730: C3CB0034  lfs f30, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825A7734: EE4AC338  fmsubs f18, f10, f12, f24
	ctx.f[18].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 - ctx.f[24].f64) as f32) as f64);
	// 825A7738: D241FF18  stfs f18, -0xe8(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-232 as u32), tmp.u32 ) };
	// 825A773C: EF04B8B8  fmsubs f24, f4, f2, f23
	ctx.f[24].f64 = (((ctx.f[4].f64 * ctx.f[2].f64 - ctx.f[23].f64) as f32) as f64);
	// 825A7740: EEFEB378  fmsubs f23, f30, f13, f22
	ctx.f[23].f64 = (((ctx.f[30].f64 * ctx.f[13].f64 - ctx.f[22].f64) as f32) as f64);
	// 825A7744: D2E1FF10  stfs f23, -0xf0(r1)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-240 as u32), tmp.u32 ) };
	// 825A7748: EEC6A9F8  fmsubs f22, f6, f7, f21
	ctx.f[22].f64 = (((ctx.f[6].f64 * ctx.f[7].f64 - ctx.f[21].f64) as f32) as f64);
	// 825A774C: D2C1FF08  stfs f22, -0xf8(r1)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-248 as u32), tmp.u32 ) };
	// 825A7750: EEBEA338  fmsubs f21, f30, f12, f20
	ctx.f[21].f64 = (((ctx.f[30].f64 * ctx.f[12].f64 - ctx.f[20].f64) as f32) as f64);
	// 825A7754: D2A1FF14  stfs f21, -0xec(r1)
	tmp.f32 = (ctx.f[21].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-236 as u32), tmp.u32 ) };
	// 825A7758: EE8698B8  fmsubs f20, f6, f2, f19
	ctx.f[20].f64 = (((ctx.f[6].f64 * ctx.f[2].f64 - ctx.f[19].f64) as f32) as f64);
	// 825A775C: D281FF04  stfs f20, -0xfc(r1)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-252 as u32), tmp.u32 ) };
	// 825A7760: EE7E8278  fmsubs f19, f30, f9, f16
	ctx.f[19].f64 = (((ctx.f[30].f64 * ctx.f[9].f64 - ctx.f[16].f64) as f32) as f64);
	// 825A7764: D261FF0C  stfs f19, -0xf4(r1)
	tmp.f32 = (ctx.f[19].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-244 as u32), tmp.u32 ) };
	// 825A7768: EF8DE038  fmsubs f28, f13, f0, f28
	ctx.f[28].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[28].f64) as f32) as f64);
	// 825A776C: EF24C978  fmsubs f25, f4, f5, f25
	ctx.f[25].f64 = (((ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[25].f64) as f32) as f64);
	// 825A7770: EE1A06F2  fmuls f16, f26, f27
	ctx.f[16].f64 = (((ctx.f[26].f64 * ctx.f[27].f64) as f32) as f64);
	// 825A7774: EE3F01F2  fmuls f17, f31, f7
	ctx.f[17].f64 = (((ctx.f[31].f64 * ctx.f[7].f64) as f32) as f64);
	// 825A7778: EE198738  fmsubs f16, f25, f28, f16
	ctx.f[16].f64 = (((ctx.f[25].f64 * ctx.f[28].f64 - ctx.f[16].f64) as f32) as f64);
	// 825A777C: EE228A38  fmsubs f17, f2, f8, f17
	ctx.f[17].f64 = (((ctx.f[2].f64 * ctx.f[8].f64 - ctx.f[17].f64) as f32) as f64);
	// 825A7780: D221FF1C  stfs f17, -0xe4(r1)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-228 as u32), tmp.u32 ) };
	// 825A7784: EE5884BA  fmadds f18, f24, f18, f16
	ctx.f[18].f64 = (((ctx.f[24].f64 * ctx.f[18].f64 + ctx.f[16].f64) as f32) as f64);
	// 825A7788: EE5695FA  fmadds f18, f22, f23, f18
	ctx.f[18].f64 = (((ctx.f[22].f64 * ctx.f[23].f64 + ctx.f[18].f64) as f32) as f64);
	// 825A778C: EE54957C  fnmsubs f18, f20, f21, f18
	ctx.f[18].f64 = -(((ctx.f[20].f64 * ctx.f[21].f64 - ctx.f[18].f64) as f32) as f64);
	// 825A7790: EE5194FA  fmadds f18, f17, f19, f18
	ctx.f[18].f64 = (((ctx.f[17].f64 * ctx.f[19].f64 + ctx.f[18].f64) as f32) as f64);
	// 825A7794: C22A1850  lfs f17, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[17].f64 = (tmp.f32 as f64);
	// 825A7798: EE319024  fdivs f17, f17, f18
	ctx.f[17].f64 = ((ctx.f[17].f64 / ctx.f[18].f64) as f32) as f64;
	// 825A779C: D221FF00  stfs f17, -0x100(r1)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-256 as u32), tmp.u32 ) };
	// 825A77A0: FE409210  fabs f18, f18
	ctx.f[18].u64 = ctx.f[18].u64 & !0x8000_0000_0000_0000u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7958 size=92
    let mut pc: u32 = 0x825A7958;
    'dispatch: loop {
        match pc {
            0x825A7958 => {
    //   block [0x825A7958..0x825A79B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A79B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A79B8 size=92
    let mut pc: u32 = 0x825A79B8;
    'dispatch: loop {
        match pc {
            0x825A79B8 => {
    //   block [0x825A79B8..0x825A7A14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7A18 size=80
    let mut pc: u32 = 0x825A7A18;
    'dispatch: loop {
        match pc {
            0x825A7A18 => {
    //   block [0x825A7A18..0x825A7A68)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7A68 size=160
    let mut pc: u32 = 0x825A7A68;
    'dispatch: loop {
        match pc {
            0x825A7A68 => {
    //   block [0x825A7A68..0x825A7B08)
	// 825A7A68: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A7A6C: C003000C  lfs f0, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7A70: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825A7A74: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 825A7A78: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7A7C: 119F038C  vspltisw v12, -1
	for i in 0..4 {
		ctx.v[12].u32[i] = 4294967295;
	}
	// 825A7A80: D1A1FFE4  stfs f13, -0x1c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 825A7A84: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7A88: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7A8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A7A90: D1A1FFE8  stfs f13, -0x18(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 825A7A94: C1A3003C  lfs f13, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7A98: D1A1FFEC  stfs f13, -0x14(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 825A7A9C: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 825A7AA0: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7AA4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825A7AA8: D001FFD4  stfs f0, -0x2c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 825A7AAC: 396B0E30  addi r11, r11, 0xe30
	ctx.r[11].s64 = ctx.r[11].s64 + 3632;
	// 825A7AB0: D001FFD8  stfs f0, -0x28(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 825A7AB4: D1A1FFDC  stfs f13, -0x24(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 825A7AB8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7B08 size=36
    let mut pc: u32 = 0x825A7B08;
    'dispatch: loop {
        match pc {
            0x825A7B08 => {
    //   block [0x825A7B08..0x825A7B2C)
	// 825A7B08: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A7B0C: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7B10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A7B14: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A7B18: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A7B1C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A7B20: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7B24: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A7B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7B30 size=164
    let mut pc: u32 = 0x825A7B30;
    'dispatch: loop {
        match pc {
            0x825A7B30 => {
    //   block [0x825A7B30..0x825A7BD4)
	// 825A7B30: C0040008  lfs f0, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7B34: C1430020  lfs f10, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A7B38: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A7B3C: C1A4000C  lfs f13, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A7B40: C1230030  lfs f9, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A7B44: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A7B48: C1030010  lfs f8, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A7B4C: C1640000  lfs f11, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A7B50: C0E30000  lfs f7, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 825A7B54: ED49537A  fmadds f10, f9, f13, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7B58: ED48533A  fmadds f10, f8, f12, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7B5C: ED4752FA  fmadds f10, f7, f11, f10
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7B60: D1450000  stfs f10, 0(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A7B64: C1430024  lfs f10, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A7B68: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A7B6C: C1230034  lfs f9, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A7B70: C1030014  lfs f8, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A7B74: C0E30004  lfs f7, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 825A7B78: ED49537A  fmadds f10, f9, f13, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7B7C: ED48533A  fmadds f10, f8, f12, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7B80: ED4752FA  fmadds f10, f7, f11, f10
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7B84: D1450004  stfs f10, 4(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A7B88: C1430028  lfs f10, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A7B8C: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A7B90: C1230038  lfs f9, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A7B94: C1030018  lfs f8, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A7B98: C0E30008  lfs f7, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 825A7B9C: ED49537A  fmadds f10, f9, f13, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7BA0: ED48533A  fmadds f10, f8, f12, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7BA4: ED4752FA  fmadds f10, f7, f11, f10
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 825A7BA8: D1450008  stfs f10, 8(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A7BAC: C143002C  lfs f10, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A7BB0: EC0A0032  fmuls f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A7BB4: C143003C  lfs f10, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A7BB8: C123001C  lfs f9, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825A7BBC: C103000C  lfs f8, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825A7BC0: EC0A037A  fmadds f0, f10, f13, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 825A7BC4: EC09033A  fmadds f0, f9, f12, f0
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 825A7BC8: EC0802FA  fmadds f0, f8, f11, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64);
	// 825A7BCC: D005000C  stfs f0, 0xc(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A7BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7BD8 size=132
    let mut pc: u32 = 0x825A7BD8;
    'dispatch: loop {
        match pc {
            0x825A7BD8 => {
    //   block [0x825A7BD8..0x825A7C5C)
	// 825A7BD8: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7BDC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A7BE0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7BE4: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A7BE8: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7BEC: D0040020  stfs f0, 0x20(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A7BF0: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7BF4: D0040030  stfs f0, 0x30(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A7BF8: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7BFC: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A7C00: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C04: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A7C08: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C0C: D0040024  stfs f0, 0x24(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A7C10: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C14: D0040034  stfs f0, 0x34(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A7C18: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C1C: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A7C20: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C24: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A7C28: C0030028  lfs f0, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C2C: D0040028  stfs f0, 0x28(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A7C30: C003002C  lfs f0, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C34: D0040038  stfs f0, 0x38(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A7C38: C0030030  lfs f0, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C3C: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A7C40: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C44: D004001C  stfs f0, 0x1c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A7C48: C0030038  lfs f0, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C4C: D004002C  stfs f0, 0x2c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A7C50: C003003C  lfs f0, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C54: D004003C  stfs f0, 0x3c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A7C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7C60 size=132
    let mut pc: u32 = 0x825A7C60;
    'dispatch: loop {
        match pc {
            0x825A7C60 => {
    //   block [0x825A7C60..0x825A7CE4)
	// 825A7C60: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C64: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A7C68: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C6C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A7C70: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C74: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A7C78: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C7C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A7C80: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C84: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A7C88: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C8C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A7C90: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C94: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A7C98: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7C9C: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A7CA0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CA4: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A7CA8: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CAC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A7CB0: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CB4: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A7CB8: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CBC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A7CC0: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CC4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A7CC8: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CCC: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A7CD0: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CD4: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A7CD8: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CDC: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A7CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7CE8 size=132
    let mut pc: u32 = 0x825A7CE8;
    'dispatch: loop {
        match pc {
            0x825A7CE8 => {
    //   block [0x825A7CE8..0x825A7D6C)
	// 825A7CE8: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CEC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A7CF0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CF4: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A7CF8: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7CFC: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A7D00: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D04: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A7D08: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D0C: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A7D10: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D14: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A7D18: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D1C: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A7D20: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D24: D004001C  stfs f0, 0x1c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A7D28: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D2C: D0040020  stfs f0, 0x20(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A7D30: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D34: D0040024  stfs f0, 0x24(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A7D38: C0030028  lfs f0, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D3C: D0040028  stfs f0, 0x28(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A7D40: C003002C  lfs f0, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D44: D004002C  stfs f0, 0x2c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A7D48: C0030030  lfs f0, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D4C: D0040030  stfs f0, 0x30(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A7D50: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D54: D0040034  stfs f0, 0x34(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A7D58: C0030038  lfs f0, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D5C: D0040038  stfs f0, 0x38(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A7D60: C003003C  lfs f0, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D64: D004003C  stfs f0, 0x3c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A7D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A7D70 size=132
    let mut pc: u32 = 0x825A7D70;
    'dispatch: loop {
        match pc {
            0x825A7D70 => {
    //   block [0x825A7D70..0x825A7DF4)
	// 825A7D70: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D74: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A7D78: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D7C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A7D80: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D84: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A7D88: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D8C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A7D90: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D94: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A7D98: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7D9C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A7DA0: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DA4: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825A7DA8: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DAC: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825A7DB0: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DB4: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825A7DB8: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DBC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825A7DC0: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DC4: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825A7DC8: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DCC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825A7DD0: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DD4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825A7DD8: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DDC: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825A7DE0: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DE4: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825A7DE8: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A7DEC: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825A7DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7DF8 size=80
    let mut pc: u32 = 0x825A7DF8;
    'dispatch: loop {
        match pc {
            0x825A7DF8 => {
    //   block [0x825A7DF8..0x825A7E48)
	// 825A7DF8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7E48 size=244
    let mut pc: u32 = 0x825A7E48;
    'dispatch: loop {
        match pc {
            0x825A7E48 => {
    //   block [0x825A7E48..0x825A7F3C)
	// 825A7E48: 3901FFC0  addi r8, r1, -0x40
	ctx.r[8].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7F40 size=128
    let mut pc: u32 = 0x825A7F40;
    'dispatch: loop {
        match pc {
            0x825A7F40 => {
    //   block [0x825A7F40..0x825A7FC0)
	// 825A7F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7F48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7F4C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7F50: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7FC0 size=116
    let mut pc: u32 = 0x825A7FC0;
    'dispatch: loop {
        match pc {
            0x825A7FC0 => {
    //   block [0x825A7FC0..0x825A8034)
	// 825A7FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7FC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7FCC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7FD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7FD4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825A7FD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A7FDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A7FE0: 4BFFFE69  bl 0x825a7e48
	ctx.lr = 0x825A7FE4;
	sub_825A7E48(ctx, base);
	// 825A7FE4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 825A7FE8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825A7FEC: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 825A7FF0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8038 size=172
    let mut pc: u32 = 0x825A8038;
    'dispatch: loop {
        match pc {
            0x825A8038 => {
    //   block [0x825A8038..0x825A80E4)
	// 825A8038: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A80E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A80E8 size=180
    let mut pc: u32 = 0x825A80E8;
    'dispatch: loop {
        match pc {
            0x825A80E8 => {
    //   block [0x825A80E8..0x825A819C)
	// 825A80E8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A81A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A81A0 size=40
    let mut pc: u32 = 0x825A81A0;
    'dispatch: loop {
        match pc {
            0x825A81A0 => {
    //   block [0x825A81A0..0x825A81C8)
	// 825A81A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A81A4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A81A8: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 825A81AC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 825A81B0: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 825A81B4: 55482834  slwi r8, r10, 5
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825A81B8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A81BC: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 825A81C0: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 825A81C4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A81C8 size=32
    let mut pc: u32 = 0x825A81C8;
    'dispatch: loop {
        match pc {
            0x825A81C8 => {
    //   block [0x825A81C8..0x825A81E8)
	// 825A81C8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 825A81CC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 825A81D0: 7D0B5830  slw r11, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 825A81D4: 7D0A482E  lwzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825A81D8: 409A0010  bne cr6, 0x825a81e8
	if !ctx.cr[6].eq {
		sub_825A81E8(ctx, base);
		return;
	}
	// 825A81DC: 7D0B5878  andc r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 & !ctx.r[11].u64;
	// 825A81E0: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 825A81E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A81E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A81E8 size=12
    let mut pc: u32 = 0x825A81E8;
    'dispatch: loop {
        match pc {
            0x825A81E8 => {
    //   block [0x825A81E8..0x825A81F4)
	// 825A81E8: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 825A81EC: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 825A81F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A81F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A81F8 size=588
    let mut pc: u32 = 0x825A81F8;
    'dispatch: loop {
        match pc {
            0x825A81F8 => {
    //   block [0x825A81F8..0x825A8444)
	// 825A81F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A81FC: 4BF8CEB1  bl 0x825350ac
	ctx.lr = 0x825A8200;
	sub_82535080(ctx, base);
	// 825A8200: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8204: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 825A8208: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A820C: 3979001F  addi r11, r25, 0x1f
	ctx.r[11].s64 = ctx.r[25].s64 + 31;
	// 825A8210: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 825A8214: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 825A8218: 7D7D2E70  srawi r29, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 825A821C: 409A0054  bne cr6, 0x825a8270
	if !ctx.cr[6].eq {
	pc = 0x825A8270; continue 'dispatch;
	}
	// 825A8220: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8224: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A8228: 419A01D8  beq cr6, 0x825a8400
	if ctx.cr[6].eq {
	pc = 0x825A8400; continue 'dispatch;
	}
	// 825A822C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8230: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A8234: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A8238: 409A0020  bne cr6, 0x825a8258
	if !ctx.cr[6].eq {
	pc = 0x825A8258; continue 'dispatch;
	}
	// 825A823C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8240: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A8244: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8248: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825A824C: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825A8250: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825A8254: 4BEBBE65  bl 0x824640b8
	ctx.lr = 0x825A8258;
	sub_824640B8(ctx, base);
	// 825A8258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A825C: 933F0008  stw r25, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 825A8260: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825A8264: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A8268: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A826C: 4BF8CE90  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 825A8270: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8278: 409A0068  bne cr6, 0x825a82e0
	if !ctx.cr[6].eq {
	pc = 0x825A82E0; continue 'dispatch;
	}
	// 825A827C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8280: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825A8284: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 825A8288: 57A4103A  slwi r4, r29, 2
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 825A828C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825A8290: 4BEBBDA9  bl 0x82464038
	ctx.lr = 0x825A8294;
	sub_82464038(ctx, base);
	// 825A8294: 217A0000  subfic r11, r26, 0
	ctx.xer.ca = ctx.r[26].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[26].s64;
	// 825A8298: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825A829C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825A82A0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825A82A4: 7D2B5910  subfe r9, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 825A82A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A82AC: 40990154  ble cr6, 0x825a8400
	if !ctx.cr[6].gt {
	pc = 0x825A8400; continue 'dispatch;
	}
	// 825A82B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A82B4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A82B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825A82BC: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 825A82C0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825A82C4: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A82C8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 825A82CC: 4198FFE8  blt cr6, 0x825a82b4
	if ctx.cr[6].lt {
	pc = 0x825A82B4; continue 'dispatch;
	}
	// 825A82D0: 933F0008  stw r25, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 825A82D4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825A82D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A82DC: 4BF8CE20  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 825A82E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A82E4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825A82E8: 409900B4  ble cr6, 0x825a839c
	if !ctx.cr[6].gt {
	pc = 0x825A839C; continue 'dispatch;
	}
	// 825A82EC: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A82F0: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 825A82F4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 825A82F8: 57A4103A  slwi r4, r29, 2
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 825A82FC: 7C7EE02E  lwzx r3, r30, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825A8300: 4BEBBD39  bl 0x82464038
	ctx.lr = 0x825A8304;
	sub_82464038(ctx, base);
	// 825A8304: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8308: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825A830C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8310: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825A8314: 4BEC2015  bl 0x8246a328
	ctx.lr = 0x825A8318;
	sub_8246A328(ctx, base);
	// 825A8318: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A831C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A8320: 419A0028  beq cr6, 0x825a8348
	if ctx.cr[6].eq {
	pc = 0x825A8348; continue 'dispatch;
	}
	// 825A8324: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8328: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A832C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A8330: 409A0018  bne cr6, 0x825a8348
	if !ctx.cr[6].eq {
	pc = 0x825A8348; continue 'dispatch;
	}
	// 825A8334: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8338: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825A833C: 7C7EE02E  lwzx r3, r30, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825A8340: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825A8344: 4BEBBD75  bl 0x824640b8
	ctx.lr = 0x825A8348;
	sub_824640B8(ctx, base);
	// 825A8348: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825A834C: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A8350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8354: 4BFFFE4D  bl 0x825a81a0
	ctx.lr = 0x825A8358;
	sub_825A81A0(ctx, base);
	// 825A8358: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A835C: 215A0000  subfic r10, r26, 0
	ctx.xer.ca = ctx.r[26].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[26].s64;
	// 825A8360: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825A8364: 7D2A5110  subfe r9, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 825A8368: 40980098  bge cr6, 0x825a8400
	if !ctx.cr[6].lt {
	pc = 0x825A8400; continue 'dispatch;
	}
	// 825A836C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A8370: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 825A8374: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8378: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825A837C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8380: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 825A8384: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825A8388: 409AFFEC  bne cr6, 0x825a8374
	if !ctx.cr[6].eq {
	pc = 0x825A8374; continue 'dispatch;
	}
	// 825A838C: 933F0008  stw r25, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 825A8390: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825A8394: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A8398: 4BF8CD64  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 825A839C: 40980074  bge cr6, 0x825a8410
	if !ctx.cr[6].lt {
	pc = 0x825A8410; continue 'dispatch;
	}
	// 825A83A0: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A83A4: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 825A83A8: 57BE103A  slwi r30, r29, 2
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825A83AC: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 825A83B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A83B4: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 825A83B8: 4BEBBC81  bl 0x82464038
	ctx.lr = 0x825A83BC;
	sub_82464038(ctx, base);
	// 825A83BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A83C0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A83C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825A83C8: 4BEC1F61  bl 0x8246a328
	ctx.lr = 0x825A83CC;
	sub_8246A328(ctx, base);
	// 825A83CC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A83D0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A83D4: 419A0028  beq cr6, 0x825a83fc
	if ctx.cr[6].eq {
	pc = 0x825A83FC; continue 'dispatch;
	}
	// 825A83D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A83DC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A83E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A83E4: 409A0018  bne cr6, 0x825a83fc
	if !ctx.cr[6].eq {
	pc = 0x825A83FC; continue 'dispatch;
	}
	// 825A83E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A83EC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825A83F0: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 825A83F4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825A83F8: 4BEBBCC1  bl 0x824640b8
	ctx.lr = 0x825A83FC;
	sub_824640B8(ctx, base);
	// 825A83FC: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 825A8400: 933F0008  stw r25, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 825A8404: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825A8408: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A840C: 4BF8CCF0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 825A8410: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8414: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 825A8418: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 825A841C: 40980010  bge cr6, 0x825a842c
	if !ctx.cr[6].lt {
	pc = 0x825A842C; continue 'dispatch;
	}
	// 825A8420: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825A8424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8428: 4BFFFD79  bl 0x825a81a0
	ctx.lr = 0x825A842C;
	sub_825A81A0(ctx, base);
	// 825A842C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8430: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A8434: 7D6BCB78  or r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[25].u64;
	// 825A8438: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A843C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A8440: 4BF8CCBC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A8448 size=132
    let mut pc: u32 = 0x825A8448;
    'dispatch: loop {
        match pc {
            0x825A8448 => {
    //   block [0x825A8448..0x825A84CC)
	// 825A8448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A8454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8458: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825A845C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8460: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A8464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8468: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A846C: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A8470: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A8474: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825A8478: 4BF8DDE1  bl 0x82536258
	ctx.lr = 0x825A847C;
	sub_82536258(ctx, base);
	// 825A847C: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 825A8480: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A84D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A84D0 size=156
    let mut pc: u32 = 0x825A84D0;
    'dispatch: loop {
        match pc {
            0x825A84D0 => {
    //   block [0x825A84D0..0x825A856C)
	// 825A84D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A84D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A84D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A84DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A84E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A84E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A84E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A84EC: 480034ED  bl 0x825ab9d8
	ctx.lr = 0x825A84F0;
	sub_825AB9D8(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A8570 size=476
    let mut pc: u32 = 0x825A8570;
    'dispatch: loop {
        match pc {
            0x825A8570 => {
    //   block [0x825A8570..0x825A874C)
	// 825A8570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8574: 4BF8CB45  bl 0x825350b8
	ctx.lr = 0x825A8578;
	sub_82535080(ctx, base);
	// 825A8578: C1A40014  lfs f13, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A857C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A8580: C1840000  lfs f12, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A8584: EC0C682A  fadds f0, f12, f13
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 825A8588: C1440028  lfs f10, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825A858C: C16B1FF8  lfs f11, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A8590: EC00502A  fadds f0, f0, f10
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64;
	// 825A8594: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 825A8598: 40990074  ble cr6, 0x825a860c
	if !ctx.cr[6].gt {
	pc = 0x825A860C; continue 'dispatch;
	}
	// 825A859C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A85A0: C1840024  lfs f12, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A85A4: C1640020  lfs f11, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825A85A8: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A85AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A85B0: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825A85B4: C1A40018  lfs f13, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A85B8: ED4D6028  fsubs f10, f13, f12
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 825A85BC: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A85C0: ED6B6828  fsubs f11, f11, f13
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 825A85C4: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A85C8: C1A40010  lfs f13, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A85CC: ED2C6828  fsubs f9, f12, f13
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 825A85D0: C18BBFFC  lfs f12, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A85D4: EDA0002C  fsqrts f13, f0
	ctx.f[13].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 825A85D8: EC0C6824  fdivs f0, f12, f13
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 825A85DC: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 825A85E0: D1A1FFCC  stfs f13, -0x34(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 825A85E4: EDAA0032  fmuls f13, f10, f0
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A85E8: D1A1FFC0  stfs f13, -0x40(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 825A85EC: EDAB0032  fmuls f13, f11, f0
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A85F0: D1A1FFC4  stfs f13, -0x3c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 825A85F4: EC090032  fmuls f0, f9, f0
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 825A85F8: D001FFC8  stfs f0, -0x38(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 825A85FC: 3961FFC0  addi r11, r1, -0x40
	ctx.r[11].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8750 size=4
    let mut pc: u32 = 0x825A8750;
    'dispatch: loop {
        match pc {
            0x825A8750 => {
    //   block [0x825A8750..0x825A8754)
	// 825A8750: 4BFFFE20  b 0x825a8570
	sub_825A8570(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8758 size=92
    let mut pc: u32 = 0x825A8758;
    'dispatch: loop {
        match pc {
            0x825A8758 => {
    //   block [0x825A8758..0x825A87B4)
	// 825A8758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8764: 4BFFFE0D  bl 0x825a8570
	ctx.lr = 0x825A8768;
	sub_825A8570(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A87B8 size=344
    let mut pc: u32 = 0x825A87B8;
    'dispatch: loop {
        match pc {
            0x825A87B8 => {
    //   block [0x825A87B8..0x825A8910)
	// 825A87B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A87BC: 4BF8C901  bl 0x825350bc
	ctx.lr = 0x825A87C0;
	sub_82535080(ctx, base);
	// 825A87C0: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 825A87C4: 4BF8D825  bl 0x82535fe8
	ctx.lr = 0x825A87C8;
	sub_82535FB0(ctx, base);
	// 825A87C8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A87CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A87D0: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 825A87D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A87D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A87DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A8910 size=220
    let mut pc: u32 = 0x825A8910;
    'dispatch: loop {
        match pc {
            0x825A8910 => {
    //   block [0x825A8910..0x825A89EC)
	// 825A8910: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A8914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825A8918: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A891C: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 825A8920: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 825A8924: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A8928: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A892C: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 825A8930: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825A8934: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825A8938: 40980010  bge cr6, 0x825a8948
	if !ctx.cr[6].lt {
	pc = 0x825A8948; continue 'dispatch;
	}
	// 825A893C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A8940: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825A8944: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825A8948: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 825A894C: 40980008  bge cr6, 0x825a8954
	if !ctx.cr[6].lt {
	pc = 0x825A8954; continue 'dispatch;
	}
	// 825A8950: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 825A8954: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 825A8958: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A895C: 38E1FFE0  addi r7, r1, -0x20
	ctx.r[7].s64 = ctx.r[1].s64 + -32;
	// 825A8960: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A8964: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A89F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A89F0 size=484
    let mut pc: u32 = 0x825A89F0;
    'dispatch: loop {
        match pc {
            0x825A89F0 => {
    //   block [0x825A89F0..0x825A8BD4)
	// 825A89F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A89F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A89F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A89FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8A00: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825A8A04: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8BD8 size=368
    let mut pc: u32 = 0x825A8BD8;
    'dispatch: loop {
        match pc {
            0x825A8BD8 => {
    //   block [0x825A8BD8..0x825A8D48)
	// 825A8BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8BDC: 4BF8C4DD  bl 0x825350b8
	ctx.lr = 0x825A8BE0;
	sub_82535080(ctx, base);
	// 825A8BE0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 825A8BE4: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8D48 size=136
    let mut pc: u32 = 0x825A8D48;
    'dispatch: loop {
        match pc {
            0x825A8D48 => {
    //   block [0x825A8D48..0x825A8DD0)
	// 825A8D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8D4C: 4BF8C365  bl 0x825350b0
	ctx.lr = 0x825A8D50;
	sub_82535080(ctx, base);
	// 825A8D50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8D54: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8D58: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 825A8D5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A8D60: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A8D64: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 825A8D68: 7C9BF1D6  mullw r4, r27, r30
	ctx.r[4].s64 = (ctx.r[27].s32 as i64) * (ctx.r[30].s32 as i64);
	// 825A8D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8D70: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825A8D74: 4BEBB2C5  bl 0x82464038
	ctx.lr = 0x825A8D78;
	sub_82464038(ctx, base);
	// 825A8D78: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8D7C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825A8D80: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8D84: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 825A8D88: 4BF8BDC9  bl 0x82534b50
	ctx.lr = 0x825A8D8C;
	sub_82534B50(ctx, base);
	// 825A8D8C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825A8D90: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A8D94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A8D98: 409A001C  bne cr6, 0x825a8db4
	if !ctx.cr[6].eq {
	pc = 0x825A8DB4; continue 'dispatch;
	}
	// 825A8D9C: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 825A8DA0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8DA4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825A8DA8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825A8DAC: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 825A8DB0: 4BEBB309  bl 0x824640b8
	ctx.lr = 0x825A8DB4;
	sub_824640B8(ctx, base);
	// 825A8DB4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825A8DB8: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 825A8DBC: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A8DC0: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 825A8DC4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 825A8DC8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A8DCC: 4BF8C334  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8DD0 size=152
    let mut pc: u32 = 0x825A8DD0;
    'dispatch: loop {
        match pc {
            0x825A8DD0 => {
    //   block [0x825A8DD0..0x825A8E68)
	// 825A8DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8DD4: 4BF8C2DD  bl 0x825350b0
	ctx.lr = 0x825A8DD8;
	sub_82535080(ctx, base);
	// 825A8DD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8DDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8DE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A8DE4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8DE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8DEC: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 825A8DF0: 409A0008  bne cr6, 0x825a8df8
	if !ctx.cr[6].eq {
	pc = 0x825A8DF8; continue 'dispatch;
	}
	// 825A8DF4: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 825A8DF8: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8DFC: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 825A8E00: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 825A8E04: 7C9AF1D6  mullw r4, r26, r30
	ctx.r[4].s64 = (ctx.r[26].s32 as i64) * (ctx.r[30].s32 as i64);
	// 825A8E08: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825A8E0C: 4BEBB22D  bl 0x82464038
	ctx.lr = 0x825A8E10;
	sub_82464038(ctx, base);
	// 825A8E10: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8E14: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8E18: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 825A8E1C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825A8E20: 4BF8BD31  bl 0x82534b50
	ctx.lr = 0x825A8E24;
	sub_82534B50(ctx, base);
	// 825A8E24: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825A8E28: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A8E2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A8E30: 409A001C  bne cr6, 0x825a8e4c
	if !ctx.cr[6].eq {
	pc = 0x825A8E4C; continue 'dispatch;
	}
	// 825A8E34: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 825A8E38: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8E3C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825A8E40: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825A8E44: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 825A8E48: 4BEBB271  bl 0x824640b8
	ctx.lr = 0x825A8E4C;
	sub_824640B8(ctx, base);
	// 825A8E4C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825A8E50: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 825A8E54: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A8E58: 7D6BD378  or r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[26].u64;
	// 825A8E5C: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 825A8E60: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A8E64: 4BF8C29C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8E68 size=8
    let mut pc: u32 = 0x825A8E68;
    'dispatch: loop {
        match pc {
            0x825A8E68 => {
    //   block [0x825A8E68..0x825A8E70)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8E70 size=8
    let mut pc: u32 = 0x825A8E70;
    'dispatch: loop {
        match pc {
            0x825A8E70 => {
    //   block [0x825A8E70..0x825A8E78)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8E78 size=80
    let mut pc: u32 = 0x825A8E78;
    'dispatch: loop {
        match pc {
            0x825A8E78 => {
    //   block [0x825A8E78..0x825A8EC8)
	// 825A8E78: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8EC8 size=80
    let mut pc: u32 = 0x825A8EC8;
    'dispatch: loop {
        match pc {
            0x825A8EC8 => {
    //   block [0x825A8EC8..0x825A8F18)
	// 825A8EC8: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A8F18 size=64
    let mut pc: u32 = 0x825A8F18;
    'dispatch: loop {
        match pc {
            0x825A8F18 => {
    //   block [0x825A8F18..0x825A8F58)
	// 825A8F18: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A8F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A8F20: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 825A8F24: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A8F28: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825A8F2C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825A8F30: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 825A8F34: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A8F38: 419A0020  beq cr6, 0x825a8f58
	if ctx.cr[6].eq {
		sub_825A8F58(ctx, base);
		return;
	}
	// 825A8F3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A8F40: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A8F44: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 825A8F48: 4198FFDC  blt cr6, 0x825a8f24
	if ctx.cr[6].lt {
	pc = 0x825A8F24; continue 'dispatch;
	}
	// 825A8F4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A8F50: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A8F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8F58 size=12
    let mut pc: u32 = 0x825A8F58;
    'dispatch: loop {
        match pc {
            0x825A8F58 => {
    //   block [0x825A8F58..0x825A8F64)
	// 825A8F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A8F5C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825A8F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A8F68 size=176
    let mut pc: u32 = 0x825A8F68;
    'dispatch: loop {
        match pc {
            0x825A8F68 => {
    //   block [0x825A8F68..0x825A9018)
	// 825A8F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8F6C: 4BF8C151  bl 0x825350bc
	ctx.lr = 0x825A8F70;
	sub_82535080(ctx, base);
	// 825A8F70: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 825A8F74: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8F78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A8F7C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825A8F80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8F84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A8F88: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A8F8C: 4BFFDDF5  bl 0x825a6d80
	ctx.lr = 0x825A8F90;
	sub_825A6D80(ctx, base);
	// 825A8F90: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8F98: 419A0068  beq cr6, 0x825a9000
	if ctx.cr[6].eq {
	pc = 0x825A9000; continue 'dispatch;
	}
	// 825A8F9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825A8FA0: D3E10060  stfs f31, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825A8FA4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 825A8FA8: 119F038C  vspltisw v12, -1
	for i in 0..4 {
		ctx.v[12].u32[i] = 4294967295;
	}
	// 825A8FAC: 396B0E30  addi r11, r11, 0xe30
	ctx.r[11].s64 = ctx.r[11].s64 + 3632;
	// 825A8FB0: 392000E0  li r9, 0xe0
	ctx.r[9].s64 = 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9018 size=128
    let mut pc: u32 = 0x825A9018;
    'dispatch: loop {
        match pc {
            0x825A9018 => {
    //   block [0x825A9018..0x825A9098)
	// 825A9018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A901C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A9024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A902C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A9034: 4BFFDC5D  bl 0x825a6c90
	ctx.lr = 0x825A9038;
	sub_825A6C90(ctx, base);
	// 825A9038: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9098 size=108
    let mut pc: u32 = 0x825A9098;
    'dispatch: loop {
        match pc {
            0x825A9098 => {
    //   block [0x825A9098..0x825A9104)
	// 825A9098: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9108 size=80
    let mut pc: u32 = 0x825A9108;
    'dispatch: loop {
        match pc {
            0x825A9108 => {
    //   block [0x825A9108..0x825A9158)
	// 825A9108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A910C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A9114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9118: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A911C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9120: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A9124: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A9128: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A912C: 48005015  bl 0x825ae140
	ctx.lr = 0x825A9130;
	sub_825AE140(ctx, base);
	// 825A9130: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A9134: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A9138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A913C: 4BFFFF5D  bl 0x825a9098
	ctx.lr = 0x825A9140;
	sub_825A9098(ctx, base);
	// 825A9140: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825A9144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A914C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A9150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9158 size=156
    let mut pc: u32 = 0x825A9158;
    'dispatch: loop {
        match pc {
            0x825A9158 => {
    //   block [0x825A9158..0x825A91F4)
	// 825A9158: 38E40010  addi r7, r4, 0x10
	ctx.r[7].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A91F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A91F8 size=100
    let mut pc: u32 = 0x825A91F8;
    'dispatch: loop {
        match pc {
            0x825A91F8 => {
    //   block [0x825A91F8..0x825A925C)
	// 825A91F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A91FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9200: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9204: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9260 size=80
    let mut pc: u32 = 0x825A9260;
    'dispatch: loop {
        match pc {
            0x825A9260 => {
    //   block [0x825A9260..0x825A92B0)
	// 825A9260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A926C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9270: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9278: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A927C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825A9280: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9284: 4BFFFD95  bl 0x825a9018
	ctx.lr = 0x825A9288;
	sub_825A9018(ctx, base);
	// 825A9288: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A928C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9294: 4BFFFE05  bl 0x825a9098
	ctx.lr = 0x825A9298;
	sub_825A9098(ctx, base);
	// 825A9298: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825A929C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A92A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A92A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A92A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A92AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A92B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A92B0 size=80
    let mut pc: u32 = 0x825A92B0;
    'dispatch: loop {
        match pc {
            0x825A92B0 => {
    //   block [0x825A92B0..0x825A9300)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9300 size=24
    let mut pc: u32 = 0x825A9300;
    'dispatch: loop {
        match pc {
            0x825A9300 => {
    //   block [0x825A9300..0x825A9318)
	// 825A9300: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825A9304: 396BFA60  addi r11, r11, -0x5a0
	ctx.r[11].s64 = ctx.r[11].s64 + -1440;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A9318 size=12
    let mut pc: u32 = 0x825A9318;
    'dispatch: loop {
        match pc {
            0x825A9318 => {
    //   block [0x825A9318..0x825A9324)
	// 825A9318: D025004C  stfs f1, 0x4c(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825A931C: D045005C  stfs f2, 0x5c(r5)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 825A9320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9328 size=128
    let mut pc: u32 = 0x825A9328;
    'dispatch: loop {
        match pc {
            0x825A9328 => {
    //   block [0x825A9328..0x825A93A8)
	// 825A9328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A932C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9330: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9334: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9338: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A93A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A93A8 size=448
    let mut pc: u32 = 0x825A93A8;
    'dispatch: loop {
        match pc {
            0x825A93A8 => {
    //   block [0x825A93A8..0x825A9568)
	// 825A93A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A93AC: 4BF8BD0D  bl 0x825350b8
	ctx.lr = 0x825A93B0;
	sub_82535080(ctx, base);
	// 825A93B0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A93B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A93B8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 825A93BC: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 825A93C0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 825A93C4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825A93C8: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A93CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A9568 size=452
    let mut pc: u32 = 0x825A9568;
    'dispatch: loop {
        match pc {
            0x825A9568 => {
    //   block [0x825A9568..0x825A972C)
	// 825A9568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A956C: 4BF8BB4D  bl 0x825350b8
	ctx.lr = 0x825A9570;
	sub_82535080(ctx, base);
	// 825A9570: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9578: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 825A957C: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 825A9580: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 825A9584: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825A9588: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A958C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A9730 size=432
    let mut pc: u32 = 0x825A9730;
    'dispatch: loop {
        match pc {
            0x825A9730 => {
    //   block [0x825A9730..0x825A98E0)
	// 825A9730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9734: 4BF8B985  bl 0x825350b8
	ctx.lr = 0x825A9738;
	sub_82535080(ctx, base);
	// 825A9738: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A973C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9740: D0210054  stfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825A9744: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 825A9748: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 825A974C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 825A9750: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825A9754: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A98E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A98E0 size=508
    let mut pc: u32 = 0x825A98E0;
    'dispatch: loop {
        match pc {
            0x825A98E0 => {
    //   block [0x825A98E0..0x825A9ADC)
	// 825A98E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A98E4: 4BF8B7D5  bl 0x825350b8
	ctx.lr = 0x825A98E8;
	sub_82535080(ctx, base);
	// 825A98E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A98EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A98F0: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 825A98F4: 3BFE0040  addi r31, r30, 0x40
	ctx.r[31].s64 = ctx.r[30].s64 + 64;
	// 825A98F8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825A98FC: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 825A9900: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825A9904: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 825A9908: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A990C: 391F001C  addi r8, r31, 0x1c
	ctx.r[8].s64 = ctx.r[31].s64 + 28;
	// 825A9910: EC010028  fsubs f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A9AE0 size=496
    let mut pc: u32 = 0x825A9AE0;
    'dispatch: loop {
        match pc {
            0x825A9AE0 => {
    //   block [0x825A9AE0..0x825A9CD0)
	// 825A9AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9AE4: 4BF8B5D9  bl 0x825350bc
	ctx.lr = 0x825A9AE8;
	sub_82535080(ctx, base);
	// 825A9AE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9AEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A9AF0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 825A9AF4: 3BFE0040  addi r31, r30, 0x40
	ctx.r[31].s64 = ctx.r[30].s64 + 64;
	// 825A9AF8: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 825A9AFC: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 825A9B00: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 825A9B04: 38FF000C  addi r7, r31, 0xc
	ctx.r[7].s64 = ctx.r[31].s64 + 12;
	// 825A9B08: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A9B0C: 38DF001C  addi r6, r31, 0x1c
	ctx.r[6].s64 = ctx.r[31].s64 + 28;
	// 825A9B10: ED810028  fsubs f12, f1, f0
	ctx.f[12].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 825A9B14: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9CD0 size=172
    let mut pc: u32 = 0x825A9CD0;
    'dispatch: loop {
        match pc {
            0x825A9CD0 => {
    //   block [0x825A9CD0..0x825A9D7C)
	// 825A9CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9CD4: 4BF8B3E1  bl 0x825350b4
	ctx.lr = 0x825A9CD8;
	sub_82535080(ctx, base);
	// 825A9CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9CDC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A9CE0: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 825A9CE4: 397E0090  addi r11, r30, 0x90
	ctx.r[11].s64 = ctx.r[30].s64 + 144;
	// 825A9CE8: 3BFE0040  addi r31, r30, 0x40
	ctx.r[31].s64 = ctx.r[30].s64 + 64;
	// 825A9CEC: 3B800030  li r28, 0x30
	ctx.r[28].s64 = 48;
	// 825A9CF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A9CF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9D80 size=212
    let mut pc: u32 = 0x825A9D80;
    'dispatch: loop {
        match pc {
            0x825A9D80 => {
    //   block [0x825A9D80..0x825A9E54)
	// 825A9D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9D84: 4BF8B339  bl 0x825350bc
	ctx.lr = 0x825A9D88;
	sub_82535080(ctx, base);
	// 825A9D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9D8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825A9D90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A9D94: 397D0090  addi r11, r29, 0x90
	ctx.r[11].s64 = ctx.r[29].s64 + 144;
	// 825A9D98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9D9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9DA0: 3BFD0040  addi r31, r29, 0x40
	ctx.r[31].s64 = ctx.r[29].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9E58 size=132
    let mut pc: u32 = 0x825A9E58;
    'dispatch: loop {
        match pc {
            0x825A9E58 => {
    //   block [0x825A9E58..0x825A9EDC)
	// 825A9E58: 39440090  addi r10, r4, 0x90
	ctx.r[10].s64 = ctx.r[4].s64 + 144;
	// 825A9E5C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825A9E60: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825A9E64: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 825A9E68: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9EE0 size=16
    let mut pc: u32 = 0x825A9EE0;
    'dispatch: loop {
        match pc {
            0x825A9EE0 => {
    //   block [0x825A9EE0..0x825A9EF0)
	// 825A9EE0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825A9EE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9EE8: 38650030  addi r3, r5, 0x30
	ctx.r[3].s64 = ctx.r[5].s64 + 48;
	// 825A9EEC: 4BFFFDE4  b 0x825a9cd0
	sub_825A9CD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9EF0 size=504
    let mut pc: u32 = 0x825A9EF0;
    'dispatch: loop {
        match pc {
            0x825A9EF0 => {
    //   block [0x825A9EF0..0x825AA0E8)
	// 825A9EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9EF4: 4BF8B1C1  bl 0x825350b4
	ctx.lr = 0x825A9EF8;
	sub_82535080(ctx, base);
	// 825A9EF8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 825A9EFC: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA0E8 size=4
    let mut pc: u32 = 0x825AA0E8;
    'dispatch: loop {
        match pc {
            0x825AA0E8 => {
    //   block [0x825AA0E8..0x825AA0EC)
	// 825AA0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA0F0 size=4
    let mut pc: u32 = 0x825AA0F0;
    'dispatch: loop {
        match pc {
            0x825AA0F0 => {
    //   block [0x825AA0F0..0x825AA0F4)
	// 825AA0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA0F8 size=12
    let mut pc: u32 = 0x825AA0F8;
    'dispatch: loop {
        match pc {
            0x825AA0F8 => {
    //   block [0x825AA0F8..0x825AA104)
	// 825AA0F8: 3960FFD1  li r11, -0x2f
	ctx.r[11].s64 = -47;
	// 825AA0FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA108 size=12
    let mut pc: u32 = 0x825AA108;
    'dispatch: loop {
        match pc {
            0x825AA108 => {
    //   block [0x825AA108..0x825AA114)
	// 825AA108: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA10C: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 825AA110: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA114 size=20
    let mut pc: u32 = 0x825AA114;
    'dispatch: loop {
        match pc {
            0x825AA114 => {
    //   block [0x825AA114..0x825AA128)
	// 825AA114: 3960FFF1  li r11, -0xf
	ctx.r[11].s64 = -15;
	// 825AA118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AA11C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA120: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 825AA124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA128 size=28
    let mut pc: u32 = 0x825AA128;
    'dispatch: loop {
        match pc {
            0x825AA128 => {
    //   block [0x825AA128..0x825AA144)
	// 825AA128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA12C: 3940FFD1  li r10, -0x2f
	ctx.r[10].s64 = -47;
	// 825AA130: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825AA134: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825AA138: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825AA13C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 825AA140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA148 size=360
    let mut pc: u32 = 0x825AA148;
    'dispatch: loop {
        match pc {
            0x825AA148 => {
    //   block [0x825AA148..0x825AA248)
	// 825AA148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA14C: 4BF8AF71  bl 0x825350bc
	ctx.lr = 0x825AA150;
	sub_82535080(ctx, base);
	// 825AA150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA158: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825AA15C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA160: 396B003F  addi r11, r11, 0x3f
	ctx.r[11].s64 = ctx.r[11].s64 + 63;
	// 825AA164: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 825AA168: 41990108  bgt cr6, 0x825aa270
	if ctx.cr[6].gt {
	pc = 0x825AA270; continue 'dispatch;
	}
	// 825AA16C: 3D80825B  lis r12, -0x7da5
	ctx.r[12].s64 = -2107965440;
	// 825AA170: 398CA184  addi r12, r12, -0x5e7c
	ctx.r[12].s64 = ctx.r[12].s64 + -24188;
	// 825AA174: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 825AA178: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 825AA17C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 825AA180: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x825AA248; continue 'dispatch;
		},
		1 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		2 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		3 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		4 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		5 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		6 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		7 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		8 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		9 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		10 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		11 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		12 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		13 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		14 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		15 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		16 => {
	pc = 0x825AA2A8; continue 'dispatch;
		},
		17 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		18 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		19 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		20 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		21 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		22 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		23 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		24 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		25 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		26 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		27 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		28 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		29 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		30 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		31 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		32 => {
	pc = 0x825AA248; continue 'dispatch;
		},
		33 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		34 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		35 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		36 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		37 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		38 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		39 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		40 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		41 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		42 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		43 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		44 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		45 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		46 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		47 => {
	pc = 0x825AA270; continue 'dispatch;
		},
		48 => {
	pc = 0x825AA248; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 825AA184: 825AA248  lwz r18, -0x5db8(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23992 as u32) ) } as u64;
	// 825AA188: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA18C: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA190: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA194: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA198: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA19C: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1A0: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1A4: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1A8: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1AC: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1B0: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1B4: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1B8: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1BC: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1C0: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1C4: 825AA2A8  lwz r18, -0x5d58(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 825AA1C8: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1CC: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1D0: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1D4: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1D8: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1DC: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1E0: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1E4: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1E8: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1EC: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1F0: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1F4: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1F8: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA1FC: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA200: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA204: 825AA248  lwz r18, -0x5db8(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23992 as u32) ) } as u64;
	// 825AA208: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA20C: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA210: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA214: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA218: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA21C: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA220: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA224: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA228: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA22C: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA230: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA234: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA238: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA23C: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA240: 825AA270  lwz r18, -0x5d90(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23952 as u32) ) } as u64;
	// 825AA244: 825AA248  lwz r18, -0x5db8(r26)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23992 as u32) ) } as u64;
            }
            0x825AA248 => {
    //   block [0x825AA248..0x825AA270)
	// 825AA248: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825AA24C: 3BCB3C70  addi r30, r11, 0x3c70
	ctx.r[30].s64 = ctx.r[11].s64 + 15472;
	// 825AA250: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA254: 4BEBBF25  bl 0x82466178
	ctx.lr = 0x825AA258;
	sub_82466178(ctx, base);
	// 825AA258: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 825AA25C: 3960FFE1  li r11, -0x1f
	ctx.r[11].s64 = -31;
	// 825AA260: 419A0008  beq cr6, 0x825aa268
	if ctx.cr[6].eq {
	pc = 0x825AA268; continue 'dispatch;
	}
	// 825AA264: 3960FFC1  li r11, -0x3f
	ctx.r[11].s64 = -63;
	// 825AA268: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA26C: 48000014  b 0x825aa280
	pc = 0x825AA280; continue 'dispatch;
            }
            0x825AA270 => {
    //   block [0x825AA270..0x825AA2A8)
	// 825AA270: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825AA274: 3BCB3C70  addi r30, r11, 0x3c70
	ctx.r[30].s64 = ctx.r[11].s64 + 15472;
	// 825AA278: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA27C: 4BEBBEFD  bl 0x82466178
	ctx.lr = 0x825AA280;
	sub_82466178(ctx, base);
	// 825AA280: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AA284: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 825AA288: A13F0006  lhz r9, 6(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825AA28C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AA290: 5529083E  rotlwi r9, r9, 1
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(1)) as u64;
	// 825AA294: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825AA298: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 825AA29C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA2A0: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 825AA2A4: 48162FC9  bl 0x8270d26c
	ctx.lr = 0x825AA2A8;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x825AA2A8; continue 'dispatch;
            }
            0x825AA2A8 => {
    //   block [0x825AA2A8..0x825AA2B0)
	// 825AA2A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA2AC: 4BF8AE60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA2B0 size=148
    let mut pc: u32 = 0x825AA2B0;
    'dispatch: loop {
        match pc {
            0x825AA2B0 => {
    //   block [0x825AA2B0..0x825AA344)
	// 825AA2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA2B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA2BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA2C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA2C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA2CC: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 825AA2D0: 419A005C  beq cr6, 0x825aa32c
	if ctx.cr[6].eq {
	pc = 0x825AA32C; continue 'dispatch;
	}
	// 825AA2D4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825AA2D8: 83CB3C70  lwz r30, 0x3c70(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15472 as u32) ) } as u64;
	// 825AA2DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA2E0: 4BEBBE99  bl 0x82466178
	ctx.lr = 0x825AA2E4;
	sub_82466178(ctx, base);
	// 825AA2E4: 4BEBFB9D  bl 0x82469e80
	ctx.lr = 0x825AA2E8;
	sub_82469E80(ctx, base);
	// 825AA2E8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825AA2EC: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AA2F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825AA2F4: 5568083E  rotlwi r8, r11, 1
	ctx.r[8].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 825AA2F8: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA2FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AA300: 512807DE  rlwimi r8, r9, 0, 0x1f, 0xf
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF0001) | (ctx.r[8].u64 & 0x000000000000FFFE);
	// 825AA304: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AA308: 2F07FFF1  cmpwi cr6, r7, -0xf
	ctx.cr[6].compare_i32(ctx.r[7].s32, -15, &mut ctx.xer);
	// 825AA30C: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 825AA310: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 825AA314: 409A0008  bne cr6, 0x825aa31c
	if !ctx.cr[6].eq {
	pc = 0x825AA31C; continue 'dispatch;
	}
	// 825AA318: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA31C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AA320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA324: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AA328: 48162F45  bl 0x8270d26c
	ctx.lr = 0x825AA32C;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AA32C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA338: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA33C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA348 size=128
    let mut pc: u32 = 0x825AA348;
    'dispatch: loop {
        match pc {
            0x825AA348 => {
    //   block [0x825AA348..0x825AA3C8)
	// 825AA348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA35C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AA360: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA364: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 825AA368: 419A0038  beq cr6, 0x825aa3a0
	if ctx.cr[6].eq {
	pc = 0x825AA3A0; continue 'dispatch;
	}
	// 825AA36C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825AA370: 83EB3C70  lwz r31, 0x3c70(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15472 as u32) ) } as u64;
	// 825AA374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA378: 4BEBBE01  bl 0x82466178
	ctx.lr = 0x825AA37C;
	sub_82466178(ctx, base);
	// 825AA37C: 4BEBFB05  bl 0x82469e80
	ctx.lr = 0x825AA380;
	sub_82469E80(ctx, base);
	// 825AA380: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA384: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AA388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA38C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AA390: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AA394: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AA398: 409A0010  bne cr6, 0x825aa3a8
	if !ctx.cr[6].eq {
	pc = 0x825AA3A8; continue 'dispatch;
	}
	// 825AA39C: 48162ED1  bl 0x8270d26c
	ctx.lr = 0x825AA3A0;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AA3A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825AA3A4: 4800000C  b 0x825aa3b0
	pc = 0x825AA3B0; continue 'dispatch;
	// 825AA3A8: 48162EC5  bl 0x8270d26c
	ctx.lr = 0x825AA3AC;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AA3AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AA3B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA3BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA3C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA3C8 size=136
    let mut pc: u32 = 0x825AA3C8;
    'dispatch: loop {
        match pc {
            0x825AA3C8 => {
    //   block [0x825AA3C8..0x825AA450)
	// 825AA3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA3D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA3D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA3D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA3E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA3E4: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 825AA3E8: 419A0050  beq cr6, 0x825aa438
	if ctx.cr[6].eq {
	pc = 0x825AA438; continue 'dispatch;
	}
	// 825AA3EC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825AA3F0: 83CB3C70  lwz r30, 0x3c70(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15472 as u32) ) } as u64;
	// 825AA3F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA3F8: 4BEBBD81  bl 0x82466178
	ctx.lr = 0x825AA3FC;
	sub_82466178(ctx, base);
	// 825AA3FC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AA400: A15F0006  lhz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825AA404: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AA408: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825AA40C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 825AA410: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AA414: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 825AA418: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825AA41C: 409A000C  bne cr6, 0x825aa428
	if !ctx.cr[6].eq {
	pc = 0x825AA428; continue 'dispatch;
	}
	// 825AA420: 3960FFF1  li r11, -0xf
	ctx.r[11].s64 = -15;
	// 825AA424: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA428: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AA42C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA430: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AA434: 48162E39  bl 0x8270d26c
	ctx.lr = 0x825AA438;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AA438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA450 size=136
    let mut pc: u32 = 0x825AA450;
    'dispatch: loop {
        match pc {
            0x825AA450 => {
    //   block [0x825AA450..0x825AA4D8)
	// 825AA450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA45C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA46C: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 825AA470: 419A0050  beq cr6, 0x825aa4c0
	if ctx.cr[6].eq {
	pc = 0x825AA4C0; continue 'dispatch;
	}
	// 825AA474: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825AA478: 83CB3C70  lwz r30, 0x3c70(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15472 as u32) ) } as u64;
	// 825AA47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA480: 4BEBBCF9  bl 0x82466178
	ctx.lr = 0x825AA484;
	sub_82466178(ctx, base);
	// 825AA484: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AA488: A15F0006  lhz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 825AA48C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AA490: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825AA494: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 825AA498: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AA49C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 825AA4A0: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825AA4A4: 409A000C  bne cr6, 0x825aa4b0
	if !ctx.cr[6].eq {
	pc = 0x825AA4B0; continue 'dispatch;
	}
	// 825AA4A8: 3960FFF1  li r11, -0xf
	ctx.r[11].s64 = -15;
	// 825AA4AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA4B0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AA4B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA4B8: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AA4BC: 48162DB1  bl 0x8270d26c
	ctx.lr = 0x825AA4C0;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AA4C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA4CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA4D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA4D8 size=224
    let mut pc: u32 = 0x825AA4D8;
    'dispatch: loop {
        match pc {
            0x825AA4D8 => {
    //   block [0x825AA4D8..0x825AA5B8)
	// 825AA4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA4DC: 4BF8ABE1  bl 0x825350bc
	ctx.lr = 0x825AA4E0;
	sub_82535080(ctx, base);
	// 825AA4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA4E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA4E8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825AA4EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AA4F0: 83A30060  lwz r29, 0x60(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 825AA4F4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825AA4F8: 419A001C  beq cr6, 0x825aa514
	if ctx.cr[6].eq {
	pc = 0x825AA514; continue 'dispatch;
	}
	// 825AA4FC: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AA500: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AA504: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825AA508: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA50C: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825AA510: 48000010  b 0x825aa520
	pc = 0x825AA520; continue 'dispatch;
	// 825AA514: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 825AA518: 4BEB9929  bl 0x82463e40
	ctx.lr = 0x825AA51C;
	sub_82463E40(ctx, base);
	// 825AA51C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AA520: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825AA524: 419A0080  beq cr6, 0x825aa5a4
	if ctx.cr[6].eq {
	pc = 0x825AA5A4; continue 'dispatch;
	}
	// 825AA528: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825AA52C: 3BFD0028  addi r31, r29, 0x28
	ctx.r[31].s64 = ctx.r[29].s64 + 40;
	// 825AA530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AA534: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 825AA538: 3BCB9198  addi r30, r11, -0x6e68
	ctx.r[30].s64 = ctx.r[11].s64 + -28264;
	// 825AA53C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA540: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825AA544: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AA548: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AA54C: 4BEBBC2D  bl 0x82466178
	ctx.lr = 0x825AA550;
	sub_82466178(ctx, base);
	// 825AA550: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 825AA554: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AA558: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AA55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA560: 419A0008  beq cr6, 0x825aa568
	if ctx.cr[6].eq {
	pc = 0x825AA568; continue 'dispatch;
	}
	// 825AA564: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 825AA568: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AA56C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 825AA570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA574: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 825AA578: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AA57C: 48162CF1  bl 0x8270d26c
	ctx.lr = 0x825AA580;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AA580: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 825AA584: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AA588: 48163795  bl 0x8270dd1c
	ctx.lr = 0x825AA58C;
	// extern call 0x8270DD1C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 825AA58C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AA590: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AA594: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825AA598: 93AB3C70  stw r29, 0x3c70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(15472 as u32), ctx.r[29].u32 ) };
	// 825AA59C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA5A0: 4BF8AB6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 825AA5A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 825AA5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AA5AC: 916A3C70  stw r11, 0x3c70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(15472 as u32), ctx.r[11].u32 ) };
	// 825AA5B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA5B4: 4BF8AB58  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA5B8 size=68
    let mut pc: u32 = 0x825AA5B8;
    'dispatch: loop {
        match pc {
            0x825AA5B8 => {
    //   block [0x825AA5B8..0x825AA5FC)
	// 825AA5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA5C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA5C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA5C8: 3FE0829A  lis r31, -0x7d66
	ctx.r[31].s64 = -2103836672;
	// 825AA5CC: 807F3C70  lwz r3, 0x3c70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(15472 as u32) ) } as u64;
	// 825AA5D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AA5D4: 419A0014  beq cr6, 0x825aa5e8
	if ctx.cr[6].eq {
	pc = 0x825AA5E8; continue 'dispatch;
	}
	// 825AA5D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AA5DC: 4BEF07B5  bl 0x8249ad90
	ctx.lr = 0x825AA5E0;
	sub_8249AD90(ctx, base);
	// 825AA5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AA5E4: 917F3C70  stw r11, 0x3c70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(15472 as u32), ctx.r[11].u32 ) };
	// 825AA5E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AA5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA5F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA600 size=76
    let mut pc: u32 = 0x825AA600;
    'dispatch: loop {
        match pc {
            0x825AA600 => {
    //   block [0x825AA600..0x825AA64C)
	// 825AA600: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA650 size=292
    let mut pc: u32 = 0x825AA650;
    'dispatch: loop {
        match pc {
            0x825AA650 => {
    //   block [0x825AA650..0x825AA774)
	// 825AA650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA654: 4BF8AA65  bl 0x825350b8
	ctx.lr = 0x825AA658;
	sub_82535080(ctx, base);
	// 825AA658: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA778 size=92
    let mut pc: u32 = 0x825AA778;
    'dispatch: loop {
        match pc {
            0x825AA778 => {
    //   block [0x825AA778..0x825AA7D4)
	// 825AA778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA77C: 4BF8A941  bl 0x825350bc
	ctx.lr = 0x825AA780;
	sub_82535080(ctx, base);
	// 825AA780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA788: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AA78C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825AA790: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825AA794: 4BFFFE6D  bl 0x825aa600
	ctx.lr = 0x825AA798;
	sub_825AA600(ctx, base);
	// 825AA798: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AA79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA7A0: 48000E61  bl 0x825ab600
	ctx.lr = 0x825AA7A4;
	sub_825AB600(ctx, base);
	// 825AA7A4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 825AA7A8: 397F0090  addi r11, r31, 0x90
	ctx.r[11].s64 = ctx.r[31].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AA7D8 size=152
    let mut pc: u32 = 0x825AA7D8;
    'dispatch: loop {
        match pc {
            0x825AA7D8 => {
    //   block [0x825AA7D8..0x825AA870)
	// 825AA7D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AA7DC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 825AA7E0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825AA7E4: 394A9F50  addi r10, r10, -0x60b0
	ctx.r[10].s64 = ctx.r[10].s64 + -24752;
	// 825AA7E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825AA7EC: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AA7F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AA7F4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825AA7F8: C169BFFC  lfs f11, -0x4004(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825AA7FC: C1AB2934  lfs f13, 0x2934(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10548 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825AA800: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825AA804: C14B2718  lfs f10, 0x2718(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10008 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825AA808: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 825AA80C: FC005090  fmr f0, f10
	ctx.f[0].f64 = ctx.f[10].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AA870 size=76
    let mut pc: u32 = 0x825AA870;
    'dispatch: loop {
        match pc {
            0x825AA870 => {
    //   block [0x825AA870..0x825AA8BC)
	// 825AA870: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AA874: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AA878: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AA87C: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AA880: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA8BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AA8BC size=76
    let mut pc: u32 = 0x825AA8BC;
    'dispatch: loop {
        match pc {
            0x825AA8BC => {
    //   block [0x825AA8BC..0x825AA908)
	// 825AA8BC: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AA8C0: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AA8C4: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AA8C8: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AA8CC: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AA908 size=76
    let mut pc: u32 = 0x825AA908;
    'dispatch: loop {
        match pc {
            0x825AA908 => {
    //   block [0x825AA908..0x825AA954)
	// 825AA908: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AA90C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AA910: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AA914: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AA918: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AA954 size=76
    let mut pc: u32 = 0x825AA954;
    'dispatch: loop {
        match pc {
            0x825AA954 => {
    //   block [0x825AA954..0x825AA9A0)
	// 825AA954: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AA958: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AA95C: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AA960: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AA964: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AA9A0 size=76
    let mut pc: u32 = 0x825AA9A0;
    'dispatch: loop {
        match pc {
            0x825AA9A0 => {
    //   block [0x825AA9A0..0x825AA9EC)
	// 825AA9A0: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AA9A4: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AA9A8: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AA9AC: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AA9B0: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA9EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AA9EC size=76
    let mut pc: u32 = 0x825AA9EC;
    'dispatch: loop {
        match pc {
            0x825AA9EC => {
    //   block [0x825AA9EC..0x825AAA38)
	// 825AA9EC: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AA9F0: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AA9F4: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AA9F8: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AA9FC: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AAA38 size=76
    let mut pc: u32 = 0x825AAA38;
    'dispatch: loop {
        match pc {
            0x825AAA38 => {
    //   block [0x825AAA38..0x825AAA84)
	// 825AAA38: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAA3C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAA40: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AAA44: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AAA48: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAA84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AAA84 size=92
    let mut pc: u32 = 0x825AAA84;
    'dispatch: loop {
        match pc {
            0x825AAA84 => {
    //   block [0x825AAA84..0x825AAAE0)
	// 825AAA84: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAA88: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 825AAA8C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825AAA90: 409AFD98  bne cr6, 0x825aa828
	if !ctx.cr[6].eq {
		sub_825AA7D8(ctx, base);
		return;
	}
	// 825AAA94: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 825AAA98: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAA9C: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AAAA0: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AAAA4: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAAE0 size=28
    let mut pc: u32 = 0x825AAAE0;
    'dispatch: loop {
        match pc {
            0x825AAAE0 => {
    //   block [0x825AAAE0..0x825AAAFC)
	// 825AAAE0: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAAE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AAAE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AAAEC: 409AFFAC  bne cr6, 0x825aaa98
	if !ctx.cr[6].eq {
		sub_825AAA84(ctx, base);
		return;
	}
	// 825AAAF0: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAAF4: EC2052F8  fmsubs f1, f0, f11, f10
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 - ctx.f[10].f64) as f32) as f64);
	// 825AAAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AAB00 size=152
    let mut pc: u32 = 0x825AAB00;
    'dispatch: loop {
        match pc {
            0x825AAB00 => {
    //   block [0x825AAB00..0x825AAB98)
	// 825AAB00: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AAB04: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825AAB08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825AAB0C: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AAB10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AAB14: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825AAB18: C169BFFC  lfs f11, -0x4004(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825AAB1C: C1AB2934  lfs f13, 0x2934(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10548 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825AAB20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825AAB24: C14B2718  lfs f10, 0x2718(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10008 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825AAB28: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 825AAB2C: FC005090  fmr f0, f10
	ctx.f[0].f64 = ctx.f[10].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AAB98 size=88
    let mut pc: u32 = 0x825AAB98;
    'dispatch: loop {
        match pc {
            0x825AAB98 => {
    //   block [0x825AAB98..0x825AABF0)
	// 825AAB98: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAB9C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AABA0: 3961FFC4  addi r11, r1, -0x3c
	ctx.r[11].s64 = ctx.r[1].s64 + -60;
	// 825AABA4: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AABA8: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AABAC: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AABF0 size=88
    let mut pc: u32 = 0x825AABF0;
    'dispatch: loop {
        match pc {
            0x825AABF0 => {
    //   block [0x825AABF0..0x825AAC48)
	// 825AABF0: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AABF4: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AABF8: 3961FFC8  addi r11, r1, -0x38
	ctx.r[11].s64 = ctx.r[1].s64 + -56;
	// 825AABFC: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AAC00: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AAC04: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AAC48 size=88
    let mut pc: u32 = 0x825AAC48;
    'dispatch: loop {
        match pc {
            0x825AAC48 => {
    //   block [0x825AAC48..0x825AACA0)
	// 825AAC48: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAC4C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAC50: 3961FFCC  addi r11, r1, -0x34
	ctx.r[11].s64 = ctx.r[1].s64 + -52;
	// 825AAC54: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AAC58: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AAC5C: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AACA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AACA0 size=88
    let mut pc: u32 = 0x825AACA0;
    'dispatch: loop {
        match pc {
            0x825AACA0 => {
    //   block [0x825AACA0..0x825AACF8)
	// 825AACA0: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AACA4: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AACA8: 3961FFD0  addi r11, r1, -0x30
	ctx.r[11].s64 = ctx.r[1].s64 + -48;
	// 825AACAC: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AACB0: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AACB4: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AACF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AACF8 size=88
    let mut pc: u32 = 0x825AACF8;
    'dispatch: loop {
        match pc {
            0x825AACF8 => {
    //   block [0x825AACF8..0x825AAD50)
	// 825AACF8: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AACFC: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAD00: 3961FFD4  addi r11, r1, -0x2c
	ctx.r[11].s64 = ctx.r[1].s64 + -44;
	// 825AAD04: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AAD08: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AAD0C: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AAD50 size=88
    let mut pc: u32 = 0x825AAD50;
    'dispatch: loop {
        match pc {
            0x825AAD50 => {
    //   block [0x825AAD50..0x825AADA8)
	// 825AAD50: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAD54: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAD58: 3961FFD8  addi r11, r1, -0x28
	ctx.r[11].s64 = ctx.r[1].s64 + -40;
	// 825AAD5C: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AAD60: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AAD64: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AADA8 size=88
    let mut pc: u32 = 0x825AADA8;
    'dispatch: loop {
        match pc {
            0x825AADA8 => {
    //   block [0x825AADA8..0x825AAE00)
	// 825AADA8: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AADAC: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AADB0: 3961FFDC  addi r11, r1, -0x24
	ctx.r[11].s64 = ctx.r[1].s64 + -36;
	// 825AADB4: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AADB8: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AADBC: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AAE00 size=104
    let mut pc: u32 = 0x825AAE00;
    'dispatch: loop {
        match pc {
            0x825AAE00 => {
    //   block [0x825AAE00..0x825AAE68)
	// 825AAE00: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAE04: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825AAE08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AAE0C: 409AFD38  bne cr6, 0x825aab44
	if !ctx.cr[6].eq {
		sub_825AAB00(ctx, base);
		return;
	}
	// 825AAE10: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 825AAE14: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAE18: 3941FFDC  addi r10, r1, -0x24
	ctx.r[10].s64 = ctx.r[1].s64 + -36;
	// 825AAE1C: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AAE20: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 825AAE24: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAE68 size=28
    let mut pc: u32 = 0x825AAE68;
    'dispatch: loop {
        match pc {
            0x825AAE68 => {
    //   block [0x825AAE68..0x825AAE84)
	// 825AAE68: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825AAE6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AAE70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AAE74: 409AFFA0  bne cr6, 0x825aae14
	if !ctx.cr[6].eq {
		sub_825AAE00(ctx, base);
		return;
	}
	// 825AAE78: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825AAE7C: EC2052F8  fmsubs f1, f0, f11, f10
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 - ctx.f[10].f64) as f32) as f64);
	// 825AAE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAE88 size=80
    let mut pc: u32 = 0x825AAE88;
    'dispatch: loop {
        match pc {
            0x825AAE88 => {
    //   block [0x825AAE88..0x825AAED8)
	// 825AAE88: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAED8 size=20
    let mut pc: u32 = 0x825AAED8;
    'dispatch: loop {
        match pc {
            0x825AAED8 => {
    //   block [0x825AAED8..0x825AAEEC)
	// 825AAED8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825AAEDC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825AAEE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AAEE4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825AAEE8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAEEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAEEC size=40
    let mut pc: u32 = 0x825AAEEC;
    'dispatch: loop {
        match pc {
            0x825AAEEC => {
    //   block [0x825AAEEC..0x825AAF14)
	// 825AAEEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AAEF0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 825AAEF4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAEF8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AAEFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AAF00: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAF04: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 825AAF08: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AAF0C: 409AFFE8  bne cr6, 0x825aaef4
	if !ctx.cr[6].eq {
	pc = 0x825AAEF4; continue 'dispatch;
	}
	// 825AAF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAF18 size=224
    let mut pc: u32 = 0x825AAF18;
    'dispatch: loop {
        match pc {
            0x825AAF18 => {
    //   block [0x825AAF18..0x825AAFF8)
	// 825AAF18: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 825AAF1C: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825AAF20: 40990010  ble cr6, 0x825aaf30
	if !ctx.cr[6].gt {
	pc = 0x825AAF30; continue 'dispatch;
	}
	// 825AAF24: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825AAF28: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 825AAF2C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825AAF30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAF34: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAF38: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 825AAF3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAF40: 7CC8582E  lwzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAF44: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 825AAF48: 41980048  blt cr6, 0x825aaf90
	if ctx.cr[6].lt {
	pc = 0x825AAF90; continue 'dispatch;
	}
	// 825AAF4C: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 825AAF50: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 825AAF54: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAF58: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAF5C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825AAF60: 4098FFF0  bge cr6, 0x825aaf50
	if !ctx.cr[6].lt {
	pc = 0x825AAF50; continue 'dispatch;
	}
	// 825AAF64: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 825AAF68: 41980028  blt cr6, 0x825aaf90
	if ctx.cr[6].lt {
	pc = 0x825AAF90; continue 'dispatch;
	}
	// 825AAF6C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAF70: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAF74: 7C88592E  stwx r4, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 825AAF78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAF7C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAF80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAF84: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAF88: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825AAF8C: 4098FFE0  bge cr6, 0x825aaf6c
	if !ctx.cr[6].lt {
	pc = 0x825AAF6C; continue 'dispatch;
	}
	// 825AAF90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAF94: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAF98: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 825AAF9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAFA0: 7CC8582E  lwzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAFA4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 825AAFA8: 41980048  blt cr6, 0x825aaff0
	if ctx.cr[6].lt {
	pc = 0x825AAFF0; continue 'dispatch;
	}
	// 825AAFAC: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 825AAFB0: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 825AAFB4: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAFB8: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAFBC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825AAFC0: 4098FFF0  bge cr6, 0x825aafb0
	if !ctx.cr[6].lt {
	pc = 0x825AAFB0; continue 'dispatch;
	}
	// 825AAFC4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 825AAFC8: 41980028  blt cr6, 0x825aaff0
	if ctx.cr[6].lt {
	pc = 0x825AAFF0; continue 'dispatch;
	}
	// 825AAFCC: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAFD0: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAFD4: 7CE8592E  stwx r7, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u32) };
	// 825AAFD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAFDC: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AAFE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAFE4: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AAFE8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825AAFEC: 4098FFE0  bge cr6, 0x825aafcc
	if !ctx.cr[6].lt {
	pc = 0x825AAFCC; continue 'dispatch;
	}
	// 825AAFF0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825AAFF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAFF8 size=60
    let mut pc: u32 = 0x825AAFF8;
    'dispatch: loop {
        match pc {
            0x825AAFF8 => {
    //   block [0x825AAFF8..0x825AB034)
	// 825AAFF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAFFC: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AB000: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825AB004: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB008: 7CC8582E  lwzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AB00C: 40980028  bge cr6, 0x825ab034
	if !ctx.cr[6].lt {
		sub_825AB034(ctx, base);
		return;
	}
	// 825AB010: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825AB014: 54C6003E  slwi r6, r6, 0
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 825AB018: 7CEA582E  lwzx r7, r10, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AB01C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 825AB020: 7CE8592E  stwx r7, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u32) };
	// 825AB024: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB028: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB02C: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 825AB030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB034 size=32
    let mut pc: u32 = 0x825AB034;
    'dispatch: loop {
        match pc {
            0x825AB034 => {
    //   block [0x825AB034..0x825AB054)
	// 825AB034: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825AB038: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AB03C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 825AB040: 7CE9592E  stwx r7, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u32) };
	// 825AB044: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB048: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB04C: 7D4B412E  stwx r10, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 825AB050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB058 size=32
    let mut pc: u32 = 0x825AB058;
    'dispatch: loop {
        match pc {
            0x825AB058 => {
    //   block [0x825AB058..0x825AB078)
	// 825AB058: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB05C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB060: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825AB064: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB068: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825AB06C: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 825AB070: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825AB074: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB078 size=88
    let mut pc: u32 = 0x825AB078;
    'dispatch: loop {
        match pc {
            0x825AB078 => {
    //   block [0x825AB078..0x825AB0D0)
	// 825AB078: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB07C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825AB080: 41980040  blt cr6, 0x825ab0c0
	if ctx.cr[6].lt {
	pc = 0x825AB0C0; continue 'dispatch;
	}
	// 825AB084: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825AB088: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825AB08C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825AB090: 41980030  blt cr6, 0x825ab0c0
	if ctx.cr[6].lt {
	pc = 0x825AB0C0; continue 'dispatch;
	}
	// 825AB094: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB098: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825AB09C: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825AB0A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825AB0A4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB0A8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB0AC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825AB0B0: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB0B4: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825AB0B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825AB0BC: 4098FFD8  bge cr6, 0x825ab094
	if !ctx.cr[6].lt {
	pc = 0x825AB094; continue 'dispatch;
	}
	// 825AB0C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AB0C4: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825AB0C8: 409AFFB0  bne cr6, 0x825ab078
	if !ctx.cr[6].eq {
	pc = 0x825AB078; continue 'dispatch;
	}
	// 825AB0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB0D0 size=192
    let mut pc: u32 = 0x825AB0D0;
    'dispatch: loop {
        match pc {
            0x825AB0D0 => {
    //   block [0x825AB0D0..0x825AB190)
	// 825AB0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB0D4: 4BF89FDD  bl 0x825350b0
	ctx.lr = 0x825AB0D8;
	sub_82535080(ctx, base);
	// 825AB0D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB0DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AB0E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AB0E4: 4BFFFF75  bl 0x825ab058
	ctx.lr = 0x825AB0E8;
	sub_825AB058(ctx, base);
	// 825AB0E8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB0EC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 825AB0F0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 825AB0F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AB0F8: 40990090  ble cr6, 0x825ab188
	if !ctx.cr[6].gt {
	pc = 0x825AB188; continue 'dispatch;
	}
	// 825AB0FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AB100: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB104: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB108: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AB10C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825AB110: 40980058  bge cr6, 0x825ab168
	if !ctx.cr[6].lt {
	pc = 0x825AB168; continue 'dispatch;
	}
	// 825AB114: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB118: 7F8A00D0  neg r28, r10
	ctx.r[28].s64 = -ctx.r[10].s64;
	// 825AB11C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB120: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825AB124: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825AB128: 409A0010  bne cr6, 0x825ab138
	if !ctx.cr[6].eq {
	pc = 0x825AB138; continue 'dispatch;
	}
	// 825AB12C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825AB130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB134: 4BEC321D  bl 0x8246e350
	ctx.lr = 0x825AB138;
	sub_8246E350(ctx, base);
	// 825AB138: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB13C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB140: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AB144: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 825AB148: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB14C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AB150: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825AB154: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB158: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB15C: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 825AB160: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 825AB164: 48000010  b 0x825ab174
	pc = 0x825AB174; continue 'dispatch;
	// 825AB168: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825AB16C: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AB170: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 825AB174: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB178: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 825AB17C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 825AB180: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825AB184: 4198FF7C  blt cr6, 0x825ab100
	if ctx.cr[6].lt {
	pc = 0x825AB100; continue 'dispatch;
	}
	// 825AB188: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AB18C: 4BF89F74  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB190 size=372
    let mut pc: u32 = 0x825AB190;
    'dispatch: loop {
        match pc {
            0x825AB190 => {
    //   block [0x825AB190..0x825AB304)
	// 825AB190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB194: 4BF89F1D  bl 0x825350b0
	ctx.lr = 0x825AB198;
	sub_82535080(ctx, base);
	// 825AB198: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB19C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AB1A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AB1A4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825AB1A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825AB1AC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB1B0: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB1B4: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 825AB1B8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB1BC: 40990034  ble cr6, 0x825ab1f0
	if !ctx.cr[6].gt {
	pc = 0x825AB1F0; continue 'dispatch;
	}
	// 825AB1C0: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 825AB1C4: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB1C8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825AB1CC: 4099000C  ble cr6, 0x825ab1d8
	if !ctx.cr[6].gt {
	pc = 0x825AB1D8; continue 'dispatch;
	}
	// 825AB1D0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 825AB1D4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 825AB1D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AB1DC: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 825AB1E0: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 825AB1E4: 4198FFE0  blt cr6, 0x825ab1c4
	if ctx.cr[6].lt {
	pc = 0x825AB1C4; continue 'dispatch;
	}
	// 825AB1E8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825AB1EC: 409A0010  bne cr6, 0x825ab1fc
	if !ctx.cr[6].eq {
	pc = 0x825AB1FC; continue 'dispatch;
	}
	// 825AB1F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AB1F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AB1F8: 4BF89F08  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 825AB1FC: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB200: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 825AB204: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825AB208: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825AB20C: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825AB210: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AB214: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 825AB218: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825AB21C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AB220: 41990010  bgt cr6, 0x825ab230
	if ctx.cr[6].gt {
	pc = 0x825AB230; continue 'dispatch;
	}
	// 825AB224: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825AB228: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825AB22C: 48000018  b 0x825ab244
	pc = 0x825AB244; continue 'dispatch;
	// 825AB230: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB234: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB238: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB23C: 4E800421  bctrl
	ctx.lr = 0x825AB240;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB240: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AB244: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB248: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825AB24C: 4099001C  ble cr6, 0x825ab268
	if !ctx.cr[6].gt {
	pc = 0x825AB268; continue 'dispatch;
	}
	// 825AB250: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 825AB254: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AB258: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AB25C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AB260: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 825AB264: 4198FFF0  blt cr6, 0x825ab254
	if ctx.cr[6].lt {
	pc = 0x825AB254; continue 'dispatch;
	}
	// 825AB268: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825AB26C: 93840000  stw r28, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825AB270: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825AB274: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB278: 7D2A212E  stwx r9, r10, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u32) };
	// 825AB27C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB280: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB284: 7D0A482E  lwzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825AB288: 7CEA492E  stwx r7, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[7].u32) };
	// 825AB28C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB290: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AB294: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB298: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825AB29C: 40990038  ble cr6, 0x825ab2d4
	if !ctx.cr[6].gt {
	pc = 0x825AB2D4; continue 'dispatch;
	}
	// 825AB2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AB2A4: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB2A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AB2AC: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB2B0: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 825AB2B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AB2B8: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB2BC: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AB2C0: 7D08202E  lwzx r8, r8, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 825AB2C4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AB2C8: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB2CC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825AB2D0: 4198FFD4  blt cr6, 0x825ab2a4
	if ctx.cr[6].lt {
	pc = 0x825AB2A4; continue 'dispatch;
	}
	// 825AB2D4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825AB2D8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825AB2DC: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 825AB2E0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB2E4: 409A0014  bne cr6, 0x825ab2f8
	if !ctx.cr[6].eq {
	pc = 0x825AB2F8; continue 'dispatch;
	}
	// 825AB2E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB2EC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AB2F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB2F4: 4E800421  bctrl
	ctx.lr = 0x825AB2F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB2F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AB2FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AB300: 4BF89E00  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB308 size=420
    let mut pc: u32 = 0x825AB308;
    'dispatch: loop {
        match pc {
            0x825AB308 => {
    //   block [0x825AB308..0x825AB4AC)
	// 825AB308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB30C: 4BF89DA5  bl 0x825350b0
	ctx.lr = 0x825AB310;
	sub_82535080(ctx, base);
	// 825AB310: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB314: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB318: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825AB31C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825AB320: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825AB324: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB328: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825AB32C: 4099003C  ble cr6, 0x825ab368
	if !ctx.cr[6].gt {
	pc = 0x825AB368; continue 'dispatch;
	}
	// 825AB330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AB334: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB338: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AB33C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB340: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB344: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 825AB348: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AB34C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB350: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 825AB354: 7D07402E  lwzx r8, r7, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 825AB358: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AB35C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB360: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825AB364: 4198FFD0  blt cr6, 0x825ab334
	if ctx.cr[6].lt {
	pc = 0x825AB334; continue 'dispatch;
	}
	// 825AB368: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB36C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 825AB370: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 825AB374: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825AB378: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825AB37C: 83E30020  lwz r31, 0x20(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AB380: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 825AB384: 7D7F2214  add r11, r31, r4
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 825AB388: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AB38C: 4199000C  bgt cr6, 0x825ab398
	if ctx.cr[6].gt {
	pc = 0x825AB398; continue 'dispatch;
	}
	// 825AB390: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825AB394: 48000018  b 0x825ab3ac
	pc = 0x825AB3AC; continue 'dispatch;
	// 825AB398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB39C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB3A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB3A4: 4E800421  bctrl
	ctx.lr = 0x825AB3A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB3A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AB3AC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825AB3B0: 40990024  ble cr6, 0x825ab3d4
	if !ctx.cr[6].gt {
	pc = 0x825AB3D4; continue 'dispatch;
	}
	// 825AB3B4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 825AB3B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AB3BC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 825AB3C0: 419A0014  beq cr6, 0x825ab3d4
	if ctx.cr[6].eq {
	pc = 0x825AB3D4; continue 'dispatch;
	}
	// 825AB3C4: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 825AB3C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825AB3CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AB3D0: 4200FFF8  bdnz 0x825ab3c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825AB3C8; continue 'dispatch;
	}
	// 825AB3D4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AB3DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AB3E0: 4099003C  ble cr6, 0x825ab41c
	if !ctx.cr[6].gt {
	pc = 0x825AB41C; continue 'dispatch;
	}
	// 825AB3E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB3E8: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB3EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AB3F0: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB3F4: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AB3F8: 7D0B402E  lwzx r8, r11, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 825AB3FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AB400: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825AB404: 7CE9F82E  lwzx r7, r9, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 825AB408: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 825AB40C: 7D09F92E  stwx r8, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u32) };
	// 825AB410: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB414: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825AB418: 4198FFD0  blt cr6, 0x825ab3e8
	if ctx.cr[6].lt {
	pc = 0x825AB3E8; continue 'dispatch;
	}
	// 825AB41C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB420: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825AB424: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 825AB428: 40980024  bge cr6, 0x825ab44c
	if !ctx.cr[6].lt {
	pc = 0x825AB44C; continue 'dispatch;
	}
	// 825AB42C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AB430: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825AB434: 41980008  blt cr6, 0x825ab43c
	if ctx.cr[6].lt {
	pc = 0x825AB43C; continue 'dispatch;
	}
	// 825AB438: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 825AB43C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825AB440: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825AB444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AB448: 4BEC2E81  bl 0x8246e2c8
	ctx.lr = 0x825AB44C;
	sub_8246E2C8(ctx, base);
	// 825AB44C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825AB450: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 825AB454: 40990028  ble cr6, 0x825ab47c
	if !ctx.cr[6].gt {
	pc = 0x825AB47C; continue 'dispatch;
	}
	// 825AB458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB45C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 825AB460: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 825AB464: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825AB468: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB46C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB470: 7D2B412E  stwx r9, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 825AB474: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AB478: 409AFFE8  bne cr6, 0x825ab460
	if !ctx.cr[6].eq {
	pc = 0x825AB460; continue 'dispatch;
	}
	// 825AB47C: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825AB480: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825AB484: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 825AB488: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB48C: 409A0018  bne cr6, 0x825ab4a4
	if !ctx.cr[6].eq {
	pc = 0x825AB4A4; continue 'dispatch;
	}
	// 825AB490: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB494: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB498: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AB49C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB4A0: 4E800421  bctrl
	ctx.lr = 0x825AB4A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB4A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AB4A8: 4BF89C58  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB4B0 size=336
    let mut pc: u32 = 0x825AB4B0;
    'dispatch: loop {
        match pc {
            0x825AB4B0 => {
    //   block [0x825AB4B0..0x825AB600)
	// 825AB4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB4B4: 4BF89BFD  bl 0x825350b0
	ctx.lr = 0x825AB4B8;
	sub_82535080(ctx, base);
	// 825AB4B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB4BC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825AB4C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AB4C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AB4C8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB4CC: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB4D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825AB4D4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 825AB4D8: 40980024  bge cr6, 0x825ab4fc
	if !ctx.cr[6].lt {
	pc = 0x825AB4FC; continue 'dispatch;
	}
	// 825AB4DC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AB4E0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825AB4E4: 41980008  blt cr6, 0x825ab4ec
	if ctx.cr[6].lt {
	pc = 0x825AB4EC; continue 'dispatch;
	}
	// 825AB4E8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 825AB4EC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825AB4F0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825AB4F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AB4F8: 4BEC2DD1  bl 0x8246e2c8
	ctx.lr = 0x825AB4FC;
	sub_8246E2C8(ctx, base);
	// 825AB4FC: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 825AB500: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB504: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825AB508: 409900F0  ble cr6, 0x825ab5f8
	if !ctx.cr[6].gt {
	pc = 0x825AB5F8; continue 'dispatch;
	}
	// 825AB50C: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB510: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 825AB514: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825AB518: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825AB51C: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825AB520: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AB524: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 825AB528: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825AB52C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AB530: 41990010  bgt cr6, 0x825ab540
	if ctx.cr[6].gt {
	pc = 0x825AB540; continue 'dispatch;
	}
	// 825AB534: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825AB538: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825AB53C: 48000018  b 0x825ab554
	pc = 0x825AB554; continue 'dispatch;
	// 825AB540: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB544: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB54C: 4E800421  bctrl
	ctx.lr = 0x825AB550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB550: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AB554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825AB558: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825AB55C: 4099002C  ble cr6, 0x825ab588
	if !ctx.cr[6].gt {
	pc = 0x825AB588; continue 'dispatch;
	}
	// 825AB560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB564: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 825AB568: 7D2B212E  stwx r9, r11, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u32) };
	// 825AB56C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825AB570: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB574: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB578: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AB57C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AB580: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 825AB584: 409AFFE4  bne cr6, 0x825ab568
	if !ctx.cr[6].eq {
	pc = 0x825AB568; continue 'dispatch;
	}
	// 825AB588: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB58C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB590: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB594: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB598: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825AB59C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB5A0: 40990034  ble cr6, 0x825ab5d4
	if !ctx.cr[6].gt {
	pc = 0x825AB5D4; continue 'dispatch;
	}
	// 825AB5A4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB5A8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AB5AC: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825AB5B0: 7D28202E  lwzx r9, r8, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 825AB5B4: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 825AB5B8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825AB5BC: 7CC8212E  stwx r6, r8, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), ctx.r[6].u32) };
	// 825AB5C0: 7D69392E  stwx r11, r9, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32), ctx.r[11].u32) };
	// 825AB5C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AB5C8: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB5CC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825AB5D0: 4198FFD4  blt cr6, 0x825ab5a4
	if ctx.cr[6].lt {
	pc = 0x825AB5A4; continue 'dispatch;
	}
	// 825AB5D4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825AB5D8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825AB5DC: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 825AB5E0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB5E4: 409A0014  bne cr6, 0x825ab5f8
	if !ctx.cr[6].eq {
	pc = 0x825AB5F8; continue 'dispatch;
	}
	// 825AB5E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB5EC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AB5F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB5F4: 4E800421  bctrl
	ctx.lr = 0x825AB5F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB5F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AB5FC: 4BF89B04  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB600 size=152
    let mut pc: u32 = 0x825AB600;
    'dispatch: loop {
        match pc {
            0x825AB600 => {
    //   block [0x825AB600..0x825AB698)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB698 size=200
    let mut pc: u32 = 0x825AB698;
    'dispatch: loop {
        match pc {
            0x825AB698 => {
    //   block [0x825AB698..0x825AB760)
	// 825AB698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB6A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB6A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB6A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AB6AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB6B0: 4BFFCD99  bl 0x825a8448
	ctx.lr = 0x825AB6B4;
	sub_825A8448(ctx, base);
	// 825AB6B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825AB6B8: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 825AB6BC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB760 size=56
    let mut pc: u32 = 0x825AB760;
    'dispatch: loop {
        match pc {
            0x825AB760 => {
    //   block [0x825AB760..0x825AB798)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB798 size=200
    let mut pc: u32 = 0x825AB798;
    'dispatch: loop {
        match pc {
            0x825AB798 => {
    //   block [0x825AB798..0x825AB860)
	// 825AB798: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 825AB79C: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB860 size=204
    let mut pc: u32 = 0x825AB860;
    'dispatch: loop {
        match pc {
            0x825AB860 => {
    //   block [0x825AB860..0x825AB92C)
	// 825AB860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB868: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB86C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB870: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AB874: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB878: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB87C: 4BFFCEDD  bl 0x825a8758
	ctx.lr = 0x825AB880;
	sub_825A8758(ctx, base);
	// 825AB880: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825AB884: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 825AB888: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AB930 size=72
    let mut pc: u32 = 0x825AB930;
    'dispatch: loop {
        match pc {
            0x825AB930 => {
    //   block [0x825AB930..0x825AB978)
	// 825AB930: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825AB934: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AB938: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825AB93C: 40990030  ble cr6, 0x825ab96c
	if !ctx.cr[6].gt {
	pc = 0x825AB96C; continue 'dispatch;
	}
	// 825AB940: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 825AB944: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AB948: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825AB94C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825AB950: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 825AB954: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AB958: 419A0020  beq cr6, 0x825ab978
	if ctx.cr[6].eq {
		sub_825AB978(ctx, base);
		return;
	}
	// 825AB95C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AB960: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AB964: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 825AB968: 4198FFDC  blt cr6, 0x825ab944
	if ctx.cr[6].lt {
	pc = 0x825AB944; continue 'dispatch;
	}
	// 825AB96C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825AB970: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825AB974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB978 size=12
    let mut pc: u32 = 0x825AB978;
    'dispatch: loop {
        match pc {
            0x825AB978 => {
    //   block [0x825AB978..0x825AB984)
	// 825AB978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB97C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825AB980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AB988 size=64
    let mut pc: u32 = 0x825AB988;
    'dispatch: loop {
        match pc {
            0x825AB988 => {
    //   block [0x825AB988..0x825AB9C8)
	// 825AB988: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825AB98C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AB990: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 825AB994: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AB998: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825AB99C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825AB9A0: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 825AB9A4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AB9A8: 419A0020  beq cr6, 0x825ab9c8
	if ctx.cr[6].eq {
		sub_825AB9C8(ctx, base);
		return;
	}
	// 825AB9AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AB9B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AB9B4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 825AB9B8: 4198FFDC  blt cr6, 0x825ab994
	if ctx.cr[6].lt {
	pc = 0x825AB994; continue 'dispatch;
	}
	// 825AB9BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825AB9C0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825AB9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB9C8 size=12
    let mut pc: u32 = 0x825AB9C8;
    'dispatch: loop {
        match pc {
            0x825AB9C8 => {
    //   block [0x825AB9C8..0x825AB9D4)
	// 825AB9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AB9CC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825AB9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AB9D8 size=64
    let mut pc: u32 = 0x825AB9D8;
    'dispatch: loop {
        match pc {
            0x825AB9D8 => {
    //   block [0x825AB9D8..0x825ABA18)
	// 825AB9D8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825AB9DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AB9E0: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 825AB9E4: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AB9E8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825AB9EC: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825AB9F0: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 825AB9F4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AB9F8: 419A0020  beq cr6, 0x825aba18
	if ctx.cr[6].eq {
		sub_825ABA18(ctx, base);
		return;
	}
	// 825AB9FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825ABA00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825ABA04: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 825ABA08: 4198FFDC  blt cr6, 0x825ab9e4
	if ctx.cr[6].lt {
	pc = 0x825AB9E4; continue 'dispatch;
	}
	// 825ABA0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825ABA10: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825ABA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABA18 size=12
    let mut pc: u32 = 0x825ABA18;
    'dispatch: loop {
        match pc {
            0x825ABA18 => {
    //   block [0x825ABA18..0x825ABA24)
	// 825ABA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ABA1C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825ABA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABA28 size=112
    let mut pc: u32 = 0x825ABA28;
    'dispatch: loop {
        match pc {
            0x825ABA28 => {
    //   block [0x825ABA28..0x825ABA98)
	// 825ABA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ABA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABA34: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 825ABA38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABA3C: 4BFFFF4D  bl 0x825ab988
	ctx.lr = 0x825ABA40;
	sub_825AB988(ctx, base);
	// 825ABA40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABA44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABA48: 419A0034  beq cr6, 0x825aba7c
	if ctx.cr[6].eq {
	pc = 0x825ABA7C; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABA98 size=112
    let mut pc: u32 = 0x825ABA98;
    'dispatch: loop {
        match pc {
            0x825ABA98 => {
    //   block [0x825ABA98..0x825ABB08)
	// 825ABA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ABAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABAA4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 825ABAA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABAAC: 4BFFFF2D  bl 0x825ab9d8
	ctx.lr = 0x825ABAB0;
	sub_825AB9D8(ctx, base);
	// 825ABAB0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABAB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABAB8: 419A0034  beq cr6, 0x825abaec
	if ctx.cr[6].eq {
	pc = 0x825ABAEC; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABB08 size=68
    let mut pc: u32 = 0x825ABB08;
    'dispatch: loop {
        match pc {
            0x825ABB08 => {
    //   block [0x825ABB08..0x825ABB4C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABB50 size=80
    let mut pc: u32 = 0x825ABB50;
    'dispatch: loop {
        match pc {
            0x825ABB50 => {
    //   block [0x825ABB50..0x825ABBA0)
	// 825ABB50: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABBA0 size=56
    let mut pc: u32 = 0x825ABBA0;
    'dispatch: loop {
        match pc {
            0x825ABBA0 => {
    //   block [0x825ABBA0..0x825ABBD8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABBD8 size=56
    let mut pc: u32 = 0x825ABBD8;
    'dispatch: loop {
        match pc {
            0x825ABBD8 => {
    //   block [0x825ABBD8..0x825ABC10)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABC10 size=116
    let mut pc: u32 = 0x825ABC10;
    'dispatch: loop {
        match pc {
            0x825ABC10 => {
    //   block [0x825ABC10..0x825ABC84)
	// 825ABC10: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABC88 size=144
    let mut pc: u32 = 0x825ABC88;
    'dispatch: loop {
        match pc {
            0x825ABC88 => {
    //   block [0x825ABC88..0x825ABD18)
	// 825ABC88: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ABD18 size=64
    let mut pc: u32 = 0x825ABD18;
    'dispatch: loop {
        match pc {
            0x825ABD18 => {
    //   block [0x825ABD18..0x825ABD58)
	// 825ABD18: 3D408283  lis r10, -0x7d7d
	ctx.r[10].s64 = -2105344000;
	// 825ABD1C: 3D603E39  lis r11, 0x3e39
	ctx.r[11].s64 = 1043922944;
	// 825ABD20: 6169B193  ori r9, r11, 0xb193
	ctx.r[9].u64 = ctx.r[11].u64 | 45459;
	// 825ABD24: 816A26F0  lwz r11, 0x26f0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9968 as u32) ) } as u64;
	// 825ABD28: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 825ABD2C: 216B3039  subfic r11, r11, 0x3039
	ctx.xer.ca = ctx.r[11].u32 <= 12345 as u32;
	ctx.r[11].s64 = (12345 as i64) - ctx.r[11].s64;
	// 825ABD30: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 825ABD34: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 825ABD38: 916A26F0  stw r11, 0x26f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9968 as u32), ctx.r[11].u32 ) };
	// 825ABD3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825ABD40: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ABD44: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825ABD48: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825ABD4C: C00B2A78  lfs f0, 0x2a78(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10872 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ABD50: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ABD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ABD58 size=136
    let mut pc: u32 = 0x825ABD58;
    'dispatch: loop {
        match pc {
            0x825ABD58 => {
    //   block [0x825ABD58..0x825ABDE0)
	// 825ABD58: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825ABD5C: 3CE0FF80  lis r7, -0x80
	ctx.r[7].s64 = -8388608;
	// 825ABD60: 3CC00080  lis r6, 0x80
	ctx.r[6].s64 = 8388608;
	// 825ABD64: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825ABD68: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 825ABD6C: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 825ABD70: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 825ABD74: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825ABD78: 210B0017  subfic r8, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[8].s64 = (23 as i64) - ctx.r[11].s64;
	// 825ABD7C: 7D05FE70  srawi r5, r8, 0x1f
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[8].s32 >> 31) as i64;
	// 825ABD80: 7D44FE70  srawi r4, r10, 0x1f
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 31) as i64;
	// 825ABD84: 7CAA28F8  nor r10, r5, r5
	ctx.r[10].u64 = !(ctx.r[5].u64 | ctx.r[5].u64);
	// 825ABD88: 7D292078  andc r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[4].u64;
	// 825ABD8C: 7D484038  and r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 & ctx.r[8].u64;
	// 825ABD90: 71450017  andi. r5, r10, 0x17
	ctx.r[5].u64 = ctx.r[10].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825ABD94: 7D082850  subf r8, r8, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[8].s64;
	// 825ABD98: 7CE84630  sraw r8, r7, r8
	tmp.u32 = ctx.r[8].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[7].s32 >> tmp.u32) as i64;
	// 825ABD9C: 7D0A5338  orc r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ~ctx.r[10].u64;
	// 825ABDA0: 7D28FE70  srawi r8, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 825ABDA4: 7D67FE70  srawi r7, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 825ABDA8: 7CC65E30  sraw r6, r6, r11
	tmp.u32 = ctx.r[11].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> tmp.u32) as i64;
	// 825ABDAC: 7D2B5078  andc r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 825ABDB0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825ABDB4: 7CCB5878  andc r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 & !ctx.r[11].u64;
	// 825ABDB8: 7CE64038  and r6, r7, r8
	ctx.r[6].u64 = ctx.r[7].u64 & ctx.r[8].u64;
	// 825ABDBC: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 825ABDC0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825ABDC4: 7D6B3878  andc r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[7].u64;
	// 825ABDC8: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 825ABDCC: 74C8BF80  andis. r8, r6, 0xbf80
	ctx.r[8].u64 = ctx.r[6].u64 & 3212836864;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825ABDD0: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 825ABDD4: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 825ABDD8: C021FFF0  lfs f1, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825ABDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ABDE0 size=184
    let mut pc: u32 = 0x825ABDE0;
    'dispatch: loop {
        match pc {
            0x825ABDE0 => {
    //   block [0x825ABDE0..0x825ABE98)
	// 825ABDE0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825ABDE4: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825ABDE8: 3C80FF80  lis r4, -0x80
	ctx.r[4].s64 = -8388608;
	// 825ABDEC: 3C600080  lis r3, 0x80
	ctx.r[3].s64 = 8388608;
	// 825ABDF0: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825ABDF4: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 825ABDF8: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 825ABDFC: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 825ABE00: 38EAFFFF  addi r7, r10, -1
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	// 825ABE04: 390BFFE9  addi r8, r11, -0x17
	ctx.r[8].s64 = ctx.r[11].s64 + -23;
	// 825ABE08: 3948FFFF  addi r10, r8, -1
	ctx.r[10].s64 = ctx.r[8].s64 + -1;
	// 825ABE0C: 7D4AFE70  srawi r10, r10, 0x1f
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 31) as i64;
	// 825ABE10: 20CB0017  subfic r6, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[6].s64 = (23 as i64) - ctx.r[11].s64;
	// 825ABE14: 7CE7FE70  srawi r7, r7, 0x1f
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 31) as i64;
	// 825ABE18: 7D293878  andc r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[7].u64;
	// 825ABE1C: 7CC75038  and r7, r6, r10
	ctx.r[7].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 825ABE20: 7D4650F8  nor r6, r10, r10
	ctx.r[6].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 825ABE24: 71450017  andi. r5, r10, 0x17
	ctx.r[5].u64 = ctx.r[10].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825ABE28: 7D472850  subf r10, r7, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 825ABE2C: 7CC54038  and r5, r6, r8
	ctx.r[5].u64 = ctx.r[6].u64 & ctx.r[8].u64;
	// 825ABE30: 7C845630  sraw r4, r4, r10
	tmp.u32 = ctx.r[10].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[4].s32 >> tmp.u32) as i64;
	// 825ABE34: 7D2AFE70  srawi r10, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 825ABE38: 7D68FE70  srawi r8, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 825ABE3C: 7C863378  or r6, r4, r6
	ctx.r[6].u64 = ctx.r[4].u64 | ctx.r[6].u64;
	// 825ABE40: 7D1F5038  and r31, r8, r10
	ctx.r[31].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 825ABE44: 7D243078  andc r4, r9, r6
	ctx.r[4].u64 = ctx.r[9].u64 & !ctx.r[6].u64;
	// 825ABE48: 7C635E30  sraw r3, r3, r11
	tmp.u32 = ctx.r[11].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> tmp.u32) as i64;
	// 825ABE4C: 7D0B40F8  nor r11, r8, r8
	ctx.r[11].u64 = !(ctx.r[8].u64 | ctx.r[8].u64);
	// 825ABE50: 3904FFFF  addi r8, r4, -1
	ctx.r[8].s64 = ctx.r[4].s64 + -1;
	// 825ABE54: 7FE45B78  or r4, r31, r11
	ctx.r[4].u64 = ctx.r[31].u64 | ctx.r[11].u64;
	// 825ABE58: 7C684078  andc r8, r3, r8
	ctx.r[8].u64 = ctx.r[3].u64 & !ctx.r[8].u64;
	// 825ABE5C: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 825ABE60: 7D085038  and r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 825ABE64: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 825ABE68: 508B0210  rlwimi r11, r4, 0, 8, 8
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x0000000000800000) | (ctx.r[11].u64 & 0xFFFFFFFFFF7FFFFF);
	// 825ABE6C: 556B023E  clrlwi r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 825ABE70: 7D6B3038  and r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[6].u64;
	// 825ABE74: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825ABE78: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 825ABE7C: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 825ABE80: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825ABE84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825ABE88: 7D6B3E30  sraw r11, r11, r7
	tmp.u32 = ctx.r[7].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> tmp.u32) as i64;
	// 825ABE8C: 7D632830  slw r3, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[11].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 825ABE90: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825ABE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ABE98 size=140
    let mut pc: u32 = 0x825ABE98;
    'dispatch: loop {
        match pc {
            0x825ABE98 => {
    //   block [0x825ABE98..0x825ABF24)
	// 825ABE98: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825ABE9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825ABEA0: 3CA0FF80  lis r5, -0x80
	ctx.r[5].s64 = -8388608;
	// 825ABEA4: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825ABEA8: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 825ABEAC: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 825ABEB0: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 825ABEB4: 38EAFFFF  addi r7, r10, -1
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	// 825ABEB8: 390BFFE9  addi r8, r11, -0x17
	ctx.r[8].s64 = ctx.r[11].s64 + -23;
	// 825ABEBC: 3948FFFF  addi r10, r8, -1
	ctx.r[10].s64 = ctx.r[8].s64 + -1;
	// 825ABEC0: 7D4AFE70  srawi r10, r10, 0x1f
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 31) as i64;
	// 825ABEC4: 20CB0017  subfic r6, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[6].s64 = (23 as i64) - ctx.r[11].s64;
	// 825ABEC8: 7CE7FE70  srawi r7, r7, 0x1f
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 31) as i64;
	// 825ABECC: 7D293878  andc r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[7].u64;
	// 825ABED0: 7CC75038  and r7, r6, r10
	ctx.r[7].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 825ABED4: 7D4650F8  nor r6, r10, r10
	ctx.r[6].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 825ABED8: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 825ABEDC: 5083B810  rlwimi r3, r4, 0x17, 0, 8
	ctx.r[3].u64 = (((ctx.r[4].u32).rotate_left(23) as u64) & 0x00000000FF800000) | (ctx.r[3].u64 & 0xFFFFFFFF007FFFFF);
	// 825ABEE0: 714A0017  andi. r10, r10, 0x17
	ctx.r[10].u64 = ctx.r[10].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825ABEE4: 7C875050  subf r4, r7, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 825ABEE8: 7CCA4038  and r10, r6, r8
	ctx.r[10].u64 = ctx.r[6].u64 & ctx.r[8].u64;
	// 825ABEEC: 7CA82630  sraw r8, r5, r4
	tmp.u32 = ctx.r[4].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[5].s32 >> tmp.u32) as i64;
	// 825ABEF0: 7D29FE70  srawi r9, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 825ABEF4: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 825ABEF8: 7D66FE70  srawi r6, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 825ABEFC: 7C6B4038  and r11, r3, r8
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[8].u64;
	// 825ABF00: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825ABF04: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 825ABF08: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 825ABF0C: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 825ABF10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825ABF14: 7D6B3078  andc r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[6].u64;
	// 825ABF18: 7D6B3E30  sraw r11, r11, r7
	tmp.u32 = ctx.r[7].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> tmp.u32) as i64;
	// 825ABF1C: 7D635030  slw r3, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 825ABF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ABF28 size=68
    let mut pc: u32 = 0x825ABF28;
    'dispatch: loop {
        match pc {
            0x825ABF28 => {
    //   block [0x825ABF28..0x825ABF6C)
	// 825ABF28: FC000A10  fabs f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 825ABF2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825ABF30: FDA01210  fabs f13, f2
	ctx.f[13].u64 = ctx.f[2].u64 & !0x8000_0000_0000_0000u64;
	// 825ABF34: C18B2280  lfs f12, 0x2280(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8832 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825ABF38: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825ABF3C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825ABF40: 4199002C  bgt cr6, 0x825abf6c
	if ctx.cr[6].gt {
		sub_825ABF6C(ctx, base);
		return;
	}
	// 825ABF44: EDAD602A  fadds f13, f13, f12
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 825ABF48: C18BA014  lfs f12, -0x5fec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24556 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825ABF4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825ABF50: C16BA010  lfs f11, -0x5ff0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24560 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825ABF54: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 825ABF58: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ABF5C: ED4D0032  fmuls f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ABF60: EC0D033C  fnmsubs f0, f13, f12, f0
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 825ABF64: EC0A02FC  fnmsubs f0, f10, f11, f0
	ctx.f[0].f64 = -(((ctx.f[10].f64 * ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 825ABF68: 48000034  b 0x825abf9c
	sub_825ABF6C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABF6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ABF6C size=92
    let mut pc: u32 = 0x825ABF6C;
    'dispatch: loop {
        match pc {
            0x825ABF6C => {
    //   block [0x825ABF6C..0x825ABFC8)
	// 825ABF6C: EC00602A  fadds f0, f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 825ABF70: C18BA014  lfs f12, -0x5fec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24556 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825ABF74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825ABF78: C16BA010  lfs f11, -0x5ff0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24560 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825ABF7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825ABF80: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 825ABF84: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ABF88: ED4D0032  fmuls f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ABF8C: EC0D033C  fnmsubs f0, f13, f12, f0
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 825ABF90: EDAA02FC  fnmsubs f13, f10, f11, f0
	ctx.f[13].f64 = -(((ctx.f[10].f64 * ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 825ABF94: C00BE358  lfs f0, -0x1ca8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ABF98: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 825ABF9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825ABFA0: C18B1FF8  lfs f12, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825ABFA4: FF026000  fcmpu cr6, f2, f12
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[12].f64);
	// 825ABFA8: 40980010  bge cr6, 0x825abfb8
	if !ctx.cr[6].lt {
	pc = 0x825ABFB8; continue 'dispatch;
	}
	// 825ABFAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825ABFB0: C1AB2930  lfs f13, 0x2930(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ABFB4: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 825ABFB8: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 825ABFBC: 4098000C  bge cr6, 0x825abfc8
	if !ctx.cr[6].lt {
		sub_825ABFC8(ctx, base);
		return;
	}
	// 825ABFC0: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825ABFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABFC8 size=8
    let mut pc: u32 = 0x825ABFC8;
    'dispatch: loop {
        match pc {
            0x825ABFC8 => {
    //   block [0x825ABFC8..0x825ABFD0)
	// 825ABFC8: FC200090  fmr f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[0].f64;
	// 825ABFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABFD0 size=152
    let mut pc: u32 = 0x825ABFD0;
    'dispatch: loop {
        match pc {
            0x825ABFD0 => {
    //   block [0x825ABFD0..0x825AC068)
	// 825ABFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ABFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825ABFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ABFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABFE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ABFE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825ABFEC: 480010BD  bl 0x825ad0a8
	ctx.lr = 0x825ABFF0;
	sub_825AD0A8(ctx, base);
	// 825ABFF0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 825ABFF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABFF8: 419A0054  beq cr6, 0x825ac04c
	if ctx.cr[6].eq {
	pc = 0x825AC04C; continue 'dispatch;
	}
	// 825ABFFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AC000: 419A004C  beq cr6, 0x825ac04c
	if ctx.cr[6].eq {
	pc = 0x825AC04C; continue 'dispatch;
	}
	// 825AC004: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC008: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825AC00C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AC010: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825AC014: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825AC018: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825AC01C: 41980018  blt cr6, 0x825ac034
	if ctx.cr[6].lt {
	pc = 0x825AC034; continue 'dispatch;
	}
	// 825AC020: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AC024: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AC028: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825AC02C: 4BEB7EED  bl 0x82463f18
	ctx.lr = 0x825AC030;
	sub_82463F18(ctx, base);
	// 825AC030: 4800001C  b 0x825ac04c
	pc = 0x825AC04C; continue 'dispatch;
	// 825AC034: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825AC038: 812B0040  lwz r9, 0x40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825AC03C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AC040: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 825AC044: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AC048: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 825AC04C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC050: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC05C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC068 size=12
    let mut pc: u32 = 0x825AC068;
    'dispatch: loop {
        match pc {
            0x825AC068 => {
    //   block [0x825AC068..0x825AC074)
	// 825AC068: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AC06C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AC070: 481611FC  b 0x8270d26c
	sub_8270D26C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC078 size=120
    let mut pc: u32 = 0x825AC078;
    'dispatch: loop {
        match pc {
            0x825AC078 => {
    //   block [0x825AC078..0x825AC0F0)
	// 825AC078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC07C: 4BF89041  bl 0x825350bc
	ctx.lr = 0x825AC080;
	sub_82535080(ctx, base);
	// 825AC080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC084: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AC088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AC08C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AC090: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AC094: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825AC098: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825AC09C: 4099001C  ble cr6, 0x825ac0b8
	if !ctx.cr[6].gt {
	pc = 0x825AC0B8; continue 'dispatch;
	}
	// 825AC0A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AC0A4: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AC0A8: 48001039  bl 0x825ad0e0
	ctx.lr = 0x825AC0AC;
	sub_825AD0E0(ctx, base);
	// 825AC0AC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 825AC0B0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825AC0B4: 4199FFEC  bgt cr6, 0x825ac0a0
	if ctx.cr[6].gt {
	pc = 0x825AC0A0; continue 'dispatch;
	}
	// 825AC0B8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AC0BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC0C0: 419A0028  beq cr6, 0x825ac0e8
	if ctx.cr[6].eq {
	pc = 0x825AC0E8; continue 'dispatch;
	}
	// 825AC0C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AC0C8: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AC0CC: 48001015  bl 0x825ad0e0
	ctx.lr = 0x825AC0D0;
	sub_825AD0E0(ctx, base);
	// 825AC0D0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AC0D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AC0D8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AC0DC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AC0E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC0E4: 409AFFE0  bne cr6, 0x825ac0c4
	if !ctx.cr[6].eq {
	pc = 0x825AC0C4; continue 'dispatch;
	}
	// 825AC0E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC0EC: 4BF89020  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC0F0 size=56
    let mut pc: u32 = 0x825AC0F0;
    'dispatch: loop {
        match pc {
            0x825AC0F0 => {
    //   block [0x825AC0F0..0x825AC128)
	// 825AC0F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AC0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825AC0F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825AC0FC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825AC100: 388B003C  addi r4, r11, 0x3c
	ctx.r[4].s64 = ctx.r[11].s64 + 60;
	// 825AC104: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825AC108: 990B003D  stb r8, 0x3d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(61 as u32), ctx.r[8].u8 ) };
	// 825AC10C: 386B0044  addi r3, r11, 0x44
	ctx.r[3].s64 = ctx.r[11].s64 + 68;
	// 825AC110: 992B003E  stb r9, 0x3e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(62 as u32), ctx.r[9].u8 ) };
	// 825AC114: 994B003C  stb r10, 0x3c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u8 ) };
	// 825AC118: 992B0041  stb r9, 0x41(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(65 as u32), ctx.r[9].u8 ) };
	// 825AC11C: 990B0042  stb r8, 0x42(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(66 as u32), ctx.r[8].u8 ) };
	// 825AC120: 994B0040  stb r10, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 825AC124: 4BEBE204  b 0x8246a328
	sub_8246A328(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC128 size=48
    let mut pc: u32 = 0x825AC128;
    'dispatch: loop {
        match pc {
            0x825AC128 => {
    //   block [0x825AC128..0x825AC158)
	// 825AC128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC130: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC134: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC138: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC13C: 4BEBA03D  bl 0x82466178
	ctx.lr = 0x825AC140;
	sub_82466178(ctx, base);
	// 825AC140: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AC148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC158 size=16
    let mut pc: u32 = 0x825AC158;
    'dispatch: loop {
        match pc {
            0x825AC158 => {
    //   block [0x825AC158..0x825AC168)
	// 825AC158: 81650038  lwz r11, 0x38(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC15C: 81450024  lwz r10, 0x24(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 825AC160: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC164: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC168 size=32
    let mut pc: u32 = 0x825AC168;
    'dispatch: loop {
        match pc {
            0x825AC168 => {
    //   block [0x825AC168..0x825AC188)
	// 825AC168: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AC16C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC170: 419A0018  beq cr6, 0x825ac188
	if ctx.cr[6].eq {
		sub_825AC188(ctx, base);
		return;
	}
	// 825AC174: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AC178: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AC17C: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AC180: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AC184: 48000F5C  b 0x825ad0e0
	sub_825AD0E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC188 size=12
    let mut pc: u32 = 0x825AC188;
    'dispatch: loop {
        match pc {
            0x825AC188 => {
    //   block [0x825AC188..0x825AC194)
	// 825AC188: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AC18C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC190: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC194 size=20
    let mut pc: u32 = 0x825AC194;
    'dispatch: loop {
        match pc {
            0x825AC194 => {
    //   block [0x825AC194..0x825AC1A8)
	// 825AC194: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AC198: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AC19C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825AC1A0: 80630058  lwz r3, 0x58(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AC1A4: 48000F3C  b 0x825ad0e0
	sub_825AD0E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC1A8 size=4
    let mut pc: u32 = 0x825AC1A8;
    'dispatch: loop {
        match pc {
            0x825AC1A8 => {
    //   block [0x825AC1A8..0x825AC1AC)
	// 825AC1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC1B0 size=104
    let mut pc: u32 = 0x825AC1B0;
    'dispatch: loop {
        match pc {
            0x825AC1B0 => {
    //   block [0x825AC1B0..0x825AC218)
	// 825AC1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC1B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AC1BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC1C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC1C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC1C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AC1CC: 4BEB9FAD  bl 0x82466178
	ctx.lr = 0x825AC1D0;
	sub_82466178(ctx, base);
	// 825AC1D0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC1D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AC1D8: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 825AC1DC: 4BF1957D  bl 0x824c5758
	ctx.lr = 0x825AC1E0;
	sub_824C5758(ctx, base);
	// 825AC1E0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC1E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AC1E8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 825AC1EC: 4BF1956D  bl 0x824c5758
	ctx.lr = 0x825AC1F0;
	sub_824C5758(ctx, base);
	// 825AC1F0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AC1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC1F8: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AC1FC: 48161071  bl 0x8270d26c
	ctx.lr = 0x825AC200;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AC200: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC20C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC210: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC218 size=524
    let mut pc: u32 = 0x825AC218;
    'dispatch: loop {
        match pc {
            0x825AC218 => {
    //   block [0x825AC218..0x825AC424)
	// 825AC218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC21C: 4BF88E8D  bl 0x825350a8
	ctx.lr = 0x825AC220;
	sub_82535080(ctx, base);
	// 825AC220: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC228: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825AC22C: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 825AC230: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AC234: 3B8B9198  addi r28, r11, -0x6e68
	ctx.r[28].s64 = ctx.r[11].s64 + -28264;
	// 825AC238: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825AC23C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AC240: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 825AC244: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 825AC248: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825AC24C: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 825AC250: 93DD0008  stw r30, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825AC254: 4BEB9F25  bl 0x82466178
	ctx.lr = 0x825AC258;
	sub_82466178(ctx, base);
	// 825AC258: 817C0030  lwz r11, 0x30(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 825AC25C: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AC260: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825AC264: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AC268: 419A0008  beq cr6, 0x825ac270
	if ctx.cr[6].eq {
	pc = 0x825AC270; continue 'dispatch;
	}
	// 825AC26C: 93EB002C  stw r31, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 825AC270: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 825AC274: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 825AC278: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AC27C: 93FC0030  stw r31, 0x30(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 825AC280: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 825AC284: F97C0020  std r11, 0x20(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825AC288: 48160FE5  bl 0x8270d26c
	ctx.lr = 0x825AC28C;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AC28C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825AC290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC294: 48161A89  bl 0x8270dd1c
	ctx.lr = 0x825AC298;
	// extern call 0x8270DD1C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 825AC298: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC29C: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 825AC2A0: FB7F0020  std r27, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u64 ) };
	// 825AC2A4: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825AC2A8: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 825AC2AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AC2B0: 419A001C  beq cr6, 0x825ac2cc
	if ctx.cr[6].eq {
	pc = 0x825AC2CC; continue 'dispatch;
	}
	// 825AC2B4: 8143006C  lwz r10, 0x6c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 825AC2B8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825AC2BC: 9143006C  stw r10, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 825AC2C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC2C4: 91430068  stw r10, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825AC2C8: 48000010  b 0x825ac2d8
	pc = 0x825AC2D8; continue 'dispatch;
	// 825AC2CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 825AC2D0: 4BEB7B71  bl 0x82463e40
	ctx.lr = 0x825AC2D4;
	sub_82463E40(ctx, base);
	// 825AC2D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AC2D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AC2DC: 419A0044  beq cr6, 0x825ac320
	if ctx.cr[6].eq {
	pc = 0x825AC320; continue 'dispatch;
	}
	// 825AC2E0: 93CB0014  stw r30, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 825AC2E4: 93CB0018  stw r30, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 825AC2E8: 93CB001C  stw r30, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 825AC2EC: 93CB0020  stw r30, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 825AC2F0: 93CB0024  stw r30, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 825AC2F4: 93CB0028  stw r30, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 825AC2F8: 93CB002C  stw r30, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 825AC2FC: 93CB0030  stw r30, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 825AC300: 93CB0034  stw r30, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 825AC304: 93CB0038  stw r30, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 825AC308: 93CB003C  stw r30, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 825AC30C: 93CB0040  stw r30, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 825AC310: 93CB0044  stw r30, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 825AC314: 93CB0048  stw r30, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 825AC318: 93CB004C  stw r30, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 825AC31C: 48000008  b 0x825ac324
	pc = 0x825AC324; continue 'dispatch;
	// 825AC320: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 825AC324: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825AC328: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825AC32C: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825AC330: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AC334: 419A0020  beq cr6, 0x825ac354
	if ctx.cr[6].eq {
	pc = 0x825AC354; continue 'dispatch;
	}
	// 825AC338: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825AC33C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 825AC340: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 825AC344: 912B0044  stw r9, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 825AC348: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC34C: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 825AC350: 48000010  b 0x825ac360
	pc = 0x825AC360; continue 'dispatch;
	// 825AC354: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AC358: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825AC35C: 4BEB7AE5  bl 0x82463e40
	ctx.lr = 0x825AC360;
	sub_82463E40(ctx, base);
	// 825AC360: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AC364: 419A0014  beq cr6, 0x825ac378
	if ctx.cr[6].eq {
	pc = 0x825AC378; continue 'dispatch;
	}
	// 825AC368: 38A003E8  li r5, 0x3e8
	ctx.r[5].s64 = 1000;
	// 825AC36C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825AC370: 48000CF9  bl 0x825ad068
	ctx.lr = 0x825AC374;
	sub_825AD068(ctx, base);
	// 825AC374: 48000008  b 0x825ac37c
	pc = 0x825AC37C; continue 'dispatch;
	// 825AC378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AC37C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC380: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 825AC384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC388: 935F0058  stw r26, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 825AC38C: 933F005C  stw r25, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 825AC390: 931F0060  stw r24, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 825AC394: 9BCB0010  stb r30, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 825AC398: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC39C: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 825AC3A0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC3A4: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 825AC3A8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC3AC: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 825AC3B0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC3B4: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825AC3B8: 4BEB9DC1  bl 0x82466178
	ctx.lr = 0x825AC3BC;
	sub_82466178(ctx, base);
	// 825AC3BC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC3C0: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 825AC3C4: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 825AC3C8: 4BF19391  bl 0x824c5758
	ctx.lr = 0x825AC3CC;
	sub_824C5758(ctx, base);
	// 825AC3CC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AC3D0: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 825AC3D4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 825AC3D8: 4BF19381  bl 0x824c5758
	ctx.lr = 0x825AC3DC;
	sub_824C5758(ctx, base);
	// 825AC3DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC3E0: FB7F0020  std r27, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u64 ) };
	// 825AC3E4: 48160E89  bl 0x8270d26c
	ctx.lr = 0x825AC3E8;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825AC3E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AC3EC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 825AC3F0: 9BDF003D  stb r30, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[30].u8 ) };
	// 825AC3F4: 389F003C  addi r4, r31, 0x3c
	ctx.r[4].s64 = ctx.r[31].s64 + 60;
	// 825AC3F8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825AC3FC: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 825AC400: 995F003E  stb r10, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[10].u8 ) };
	// 825AC404: 997F003C  stb r11, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 825AC408: 995F0041  stb r10, 0x41(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(65 as u32), ctx.r[10].u8 ) };
	// 825AC40C: 9BDF0042  stb r30, 0x42(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(66 as u32), ctx.r[30].u8 ) };
	// 825AC410: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 825AC414: 4BEBDF15  bl 0x8246a328
	ctx.lr = 0x825AC418;
	sub_8246A328(ctx, base);
	// 825AC418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC41C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AC420: 4BF88CD8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


